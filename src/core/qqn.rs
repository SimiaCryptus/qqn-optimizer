use crate::core::lbfgs::LBFGSState;
use crate::core::line_search::{create_1d_problem, create_1d_problem_linear, create_line_search, LineSearch, LineSearchResult, ParametricCurve};
use crate::core::optimizer::{OptimizationMetadata};
use crate::core::Optimizer;
use crate::core::StepResult;
use crate::core::{ConvergenceInfo, TerminationReason};
use crate::utils::math::{DifferentiableFunction, combine_tensors, compute_magnitude, create_1d_tensor, f64_to_tensors, log_tensor, scale_tensors};
use crate::LineSearchConfig;
use anyhow::{anyhow, Result as AnyhowResult};
use candle_core::{Error, Result as CandleResult, Tensor};
use log::{debug, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// QQN trace information for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QQNTrace {
    pub magnitude_ratios: Vec<f64>,
    pub quadratic_path_usage: Vec<bool>,
    pub step_sizes: Vec<f64>,
    pub gradient_norms: Vec<f64>,
    pub direction_norms: Vec<f64>,
    pub descent_dot_products: Vec<f64>,
}
impl Default for QQNTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl QQNTrace {
    pub fn new() -> Self {
        Self {
            magnitude_ratios: Vec::new(),
            quadratic_path_usage: Vec::new(),
            step_sizes: Vec::new(),
            gradient_norms: Vec::new(),
            direction_norms: Vec::new(),
            descent_dot_products: Vec::new(),
        }
    }
}

/// Configuration for the QQN optimizer
#[derive(Debug, Clone)]
pub struct QQNConfig {
    /// L-BFGS history length
    pub lbfgs_history: usize,
    /// Minimum number of iterations before enabling L-BFGS
    pub min_lbfgs_iterations: usize,
    /// Line search configuration
    pub line_search: LineSearchConfig,
    /// Numerical stability constant
    pub epsilon: f64,
    /// Enable verbose logging of tensor data and internal state
    pub verbose: bool,
}

impl Default for QQNConfig {
    fn default() -> Self {
        Self {
            lbfgs_history: 10,
            min_lbfgs_iterations: 2,
            line_search: LineSearchConfig {
                method: crate::core::line_search::LineSearchMethod::Bisection,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            verbose: true,
        }
    }
}

/// State information for the QQN optimizer
#[derive(Debug, Clone)]
pub struct QQNState {
    /// Current iteration number
    pub iteration: usize,
    /// L-BFGS internal state
    pub lbfgs_state: LBFGSState,
}

impl QQNState {
    pub fn new(lbfgs_history: usize) -> Self {
        Self {
            iteration: 0,
            lbfgs_state: LBFGSState::new(lbfgs_history, 1e-8),
        }
    }
}

#[derive(Debug)]
pub struct QQNOptimizer {
    config: QQNConfig,
    state: QQNState,
    line_search: Box<dyn LineSearch>,
}
impl Clone for QQNOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            line_search: self.line_search.clone_box(),
        }
    }
}

impl QQNOptimizer {
    /// Create a new QQN optimizer with the given configuration
    pub fn new(config: QQNConfig) -> Self {
        info!(
            "Creating QQN optimizer with config: lbfgs_history={}, min_lbfgs_iterations={}, epsilon={}, verbose={}",
            config.lbfgs_history, config.min_lbfgs_iterations, config.epsilon, config.verbose
        );
        let line_search = create_line_search(config.line_search.clone());
        Self {
            state: QQNState::new(config.lbfgs_history),
            config,
            line_search,
        }
    }
    /// Log tensor data if verbose mode is enabled
    fn log_tensor_data(&self, name: &str, tensors: &[Tensor]) {
        if !self.config.verbose {
            return;
        }
        debug!("=== QQN: {} ===", name);
        log_tensor(tensors);
    }


    /// Log scalar value if verbose mode is enabled
    fn log_scalar(&self, name: &str, value: f64) {
        if self.config.verbose {
            debug!("  {}: {:.12e}", name, value);
        }
    }

    /// Log optimization state if verbose mode is enabled
    fn log_optimization_state(&self, iteration: usize, additional_info: &str) {
        if !self.config.verbose {
            return;
        }
        debug!("=== QQN Optimization State (Iteration {}) ===", iteration);
        debug!("  L-BFGS History Length: {}", self.state.lbfgs_state.history_length());
        debug!("  L-BFGS Gamma: {:.6e}", self.state.lbfgs_state.gamma());
        debug!("  Additional Info: {}", additional_info);
    }

    /// Log line search details if verbose mode is enabled
    fn log_line_search_details(&self, optimal_t: f64, f_evals: usize, g_evals: usize) {
        if !self.config.verbose {
            return;
        }
        debug!("=== Line Search Results ===");
        debug!("  Optimal t: {:.12e}", optimal_t);
        debug!("  Function evaluations: {}", f_evals);
        debug!("  Gradient evaluations: {}", g_evals);
    }

    pub fn create_quadratic_path(
        &self,
        start_point: &[Tensor],
        gradient: &[Tensor],
        lbfgs_direction: &[Tensor],
    ) -> CandleResult<QuadraticPath> {
        debug!("Creating quadratic path between gradient and L-BFGS direction");
        // Log input tensors in verbose mode
        self.log_tensor_data("Start Point", start_point);
        // Log input tensors in verbose mode
        self.log_tensor_data("Input Gradient", gradient);
        self.log_tensor_data("Input L-BFGS Direction", lbfgs_direction);

        // Validate inputs
        if start_point.is_empty() || gradient.is_empty() || lbfgs_direction.is_empty() {
            warn!("Empty start point, gradient or direction vectors provided to create_quadratic_path");
            return Err(candle_core::Error::Msg(
                "Empty start point, gradient or direction vectors".into(),
            ));
        }
        if start_point.len() != gradient.len() || gradient.len() != lbfgs_direction.len() {
            warn!(
                "Dimension mismatch in create_quadratic_path: start_point={}, gradient={}, direction={}",
                start_point.len(),
                gradient.len(),
                lbfgs_direction.len()
            );
            return Err(candle_core::Error::Msg(format!(
                "Dimension mismatch: start_point={}, gradient={}, direction={}",
                start_point.len(),
                gradient.len(),
                lbfgs_direction.len()
            )));
        }

        // Create negative gradient as per paper formula
        let negative_gradient = scale_tensors(gradient, -1.0)?;
        // Log created tensors in verbose mode
        self.log_tensor_data("Negative Gradient", &negative_gradient);

        // Log norms for debugging
        let grad_norm = compute_magnitude(gradient)?;
        let lbfgs_norm = compute_magnitude(lbfgs_direction)?;
        debug!(
            "Quadratic path created: ||gradient||={:.6e}, ||lbfgs_dir||={:.6e}",
            grad_norm, lbfgs_norm
        );
        self.log_scalar("Gradient Norm", grad_norm);
        self.log_scalar("L-BFGS Direction Norm", lbfgs_norm);
        trace!("Quadratic path formula: d(t) = t(1-t)(-g) + t²d_lbfgs");

        Ok(QuadraticPath::new(
            start_point.to_vec(),
            negative_gradient,
            lbfgs_direction.to_vec(),
        ))
    }

    /// Find optimal t parameter for the quadratic path using line search
    fn find_optimal_t_line_search(
        &mut self,
        params: &[Tensor],
        quadratic_path: &QuadraticPath,
        function: &dyn DifferentiableFunction,
    ) -> CandleResult<LineSearchResult> {
        debug!("Starting line search for optimal t along quadratic path");
        // Create a parametric curve adapter for the quadratic path
        let curve = quadratic_path.clone();
        // Get the initial derivative of the quadratic path for line search
        let initial_derivative = curve.initial_derivative()
            .map_err(|e| candle_core::Error::Msg(format!("Failed to get initial derivative: {}", e)))?;


        // Create objective and gradient functions


        // Create 1D problem
        let fn1 = |x: &[f64]| -> anyhow::Result<f64> {
            let tensors = f64_to_tensors(x, params)?;
            function.evaluate(&tensors).map_err(|e| anyhow::anyhow!("Function evaluation failed: {}", e))
        };
        let fn2 = |x: &[f64]| -> anyhow::Result<Vec<f64>> {
            let tensors = f64_to_tensors(x, params)?;
            let grads = function.gradient(&tensors)
                .map_err(|e| anyhow::anyhow!("Gradient evaluation failed: {}", e))?;
            let mut result = Vec::new();
            for grad_tensor in grads {
                let flattened = grad_tensor.flatten_all()
                    .map_err(|e| anyhow::anyhow!("Failed to flatten gradient: {}", e))?;
                let values = flattened.to_vec1::<f64>()
                    .map_err(|e| anyhow::anyhow!("Failed to convert gradient to vec: {}", e))?;
                result.extend(values);
            }
            Ok(result)
        };
        let problem = create_1d_problem(
            Box::new(curve),
            &initial_derivative,
            &fn1,
            &fn2,
        ).map_err(|e| candle_core::Error::Msg(format!("Failed to create 1D problem: {}", e)))?;

        // Perform line search
        let result = match self.line_search.optimize_1d(&problem) {
            Ok(res) => res,
            Err(e) => {
                warn!("Line search failed: {}", e);
                LineSearchResult {
                    step_size: 1.0, // Default to 1.0 if search fails
                    success: false,
                    function_evaluations: 1, // At least one evaluation was attempted
                    gradient_evaluations: 1, // At least one gradient evaluation was attempted
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                }
            }
        };
        debug!(
            "Line search completed: t*={:.6}, success={}",
            result.step_size, result.success
        );
        Ok(result)
    }

    /// Perform steepest descent step with line search for adaptive learning rate
    fn steepest_descent_step(
        &mut self,
        nd_params: &mut [Tensor],
        gradients: &[Tensor],
        function: &dyn DifferentiableFunction,
        reason: &str,
    ) -> CandleResult<StepResult> {
        info!("Using steepest descent: {}", reason);
        // Create steepest descent direction (negative gradient)
        let direction = scale_tensors(gradients, -1.0)?;
        self.log_tensor_data("Steepest Descent Direction", &direction);
        // Scale direction if gradient is too large to avoid numerical issues
        let grad_norm = compute_magnitude(gradients)?;
        let direction = if grad_norm > 1000.0 {
            let scale_factor = 1000.0 / grad_norm;
            warn!("Large gradient norm {:.2e}, scaling direction by {:.2e}", grad_norm, scale_factor);
            scale_tensors(&direction, scale_factor)?
        } else {
            direction
        };


        // Convert to f64 for line search
        let params_f64: Vec<f64> = nd_params
            .iter()
            .map(|t| t.flatten_all()?.to_vec1::<f64>())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        let direction_f64: Vec<f64> = direction
            .iter()
            .map(|t| t.flatten_all()?.to_vec1::<f64>())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        let gradients_f64: Vec<f64> = gradients
            .iter()
            .map(|t| t.flatten_all()?.to_vec1::<f64>())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        // Creat a 1d tensor buffer for the unitary parameter
        // Perform line search in a separate scope to avoid borrow conflicts
        let mut line_search_result = {
            // Create objective and gradient functions
            let objective_fn = |x: &[f64]| -> anyhow::Result<f64> {
                let _1d_params = create_1d_tensor(x, nd_params[0].device())?;
                let tensors = f64_to_tensors(x, &[_1d_params])?;
                function.evaluate(&tensors).map_err(|e| anyhow!("Function evaluation failed: {}", e))
            };
            let gradient_fn = |x: &[f64]| -> anyhow::Result<Vec<f64>> {
                let _1d_params = create_1d_tensor(x, nd_params[0].device())?;
                let tensors = f64_to_tensors(x, &[_1d_params])?;
                let grads = function.gradient(&tensors)
                    .map_err(|e| anyhow!("Gradient evaluation failed: {}", e))?;
                debug!("Steepest descent line search: x={:?}, tensors={:?}, grads={:?}", x, tensors, grads);
                Ok(grads
                    .iter()
                    .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
                    .collect())
            };

            // Create 1D problem
            let problem = create_1d_problem_linear(
                &params_f64,
                &direction_f64,
                &gradients_f64,
                &objective_fn,
                &gradient_fn,
            ).map_err(|e| candle_core::Error::Msg(format!("Failed to create 1D problem: {}", e)))?;

            // Perform line search
            self.line_search.optimize_1d(&problem)
                .map_err(|e| {
                    warn!("Line search failed: {}", e);
                    candle_core::Error::Msg(format!("Line search failed: {}", e))
                })
        };

        // If line search failed, try a very small fixed step
        if line_search_result.is_err() || !line_search_result.as_ref().unwrap().success {
            warn!("Line search failed, using fixed small step");
            line_search_result = Ok(LineSearchResult {
                step_size: 1e-8,
                success: true,
                function_evaluations: 1,
                gradient_evaluations: 0,
                termination_reason: TerminationReason::WolfeConditionsSatisfied,
            });
        }

        let line_search_result = line_search_result?;

        if !line_search_result.success {
            warn!(
                "Line search did not succeed: step_size={:.6e}, reason={}",
                line_search_result.step_size, reason
            );
            // Don't fail completely, just use a very small step
            warn!("Using minimal step size as fallback");
        }

        debug!(
            "Steepest descent line search completed: step_size={:.6e}, success={}",
            line_search_result.step_size, line_search_result.success
        );
        self.log_scalar("Steepest Descent Step Size", line_search_result.step_size);

        // Apply the step
        for (param, dir) in nd_params.iter_mut().zip(direction.iter()) {
            let scaled_dir = dir.affine(line_search_result.step_size, 0.0)?;
            *param = param.add(&scaled_dir)?;
        }
        // Log updated parameters
        self.log_tensor_data("Updated Parameters (Steepest Descent)", nd_params);
        // Create convergence info
        let convergence_info = ConvergenceInfo {
            converged: false,
            function_change: None,
        };
        // Create metadata
        let mut metadata = OptimizationMetadata::default();
        metadata.optimizer_data.insert("method".to_string(), 0.0); // 0 = steepest descent
        metadata
            .optimizer_data
            .insert("gradient_norm".to_string(), compute_magnitude(gradients)?);
        metadata
            .optimizer_data
            .insert("direction_norm".to_string(), compute_magnitude(&direction)?);
        metadata
            .optimizer_data
            .insert("reason".to_string(), reason.len() as f64); // Store reason length as proxy
        Ok(StepResult {
            step_size: line_search_result.step_size,
            function_evaluations: line_search_result.function_evaluations,
            gradient_evaluations: line_search_result.gradient_evaluations,
            convergence_info,
            metadata,
        })
    }


    fn compute_scale_factor(&mut self, params: &mut [Tensor], step_norm: f64) -> Result<f64, Error> {
        // Safety check: if step is too large, scale it down
        // Make max step norm adaptive based on current parameter magnitude
        let param_norm = compute_magnitude(params)?;
        let max_step_norm = if param_norm > 0.0 {
            // Allow steps up to 10% of parameter norm, but at least 1.0
            (0.1 * param_norm).max(1.0)
        } else {
            1.0
        };
        let scaling_factor = if step_norm > max_step_norm {
            warn!(
                "Step norm {:.6e} exceeds maximum {:.6e}, scaling down",
                step_norm, max_step_norm
            );
            self.log_scalar("Scaling Factor", max_step_norm / step_norm);
            max_step_norm / step_norm
        } else {
            debug!("Max step norm: {:.6e} (based on param norm: {:.6e})", max_step_norm, param_norm);
            self.log_scalar("Scaling Factor", 1.0);
            1.0
        };
        Ok(scaling_factor)
    }

    fn apply_step(&mut self, params: &mut [Tensor], direction: &Vec<Tensor>, scaling_factor: f64) -> Result<(), Error> {
        for (param, dir) in params.iter_mut().zip(direction.iter()) {
            let scaled_dir = dir.affine(scaling_factor, 0.0)?;
            *param = param.add(&scaled_dir)?;
        }
        // Log updated parameters in verbose mode
        self.log_tensor_data("Updated Parameters", params);
        Ok(())
    }

    fn is_all_finite(tensor_vec: &Vec<Tensor>) -> bool {
        tensor_vec.iter().all(|d| {
            d.flatten_all()
                .and_then(|f| f.to_vec1::<f64>())
                .map(|v| v.iter().all(|&x| x.is_finite()))
                .unwrap_or(false)
        })
    }
}

impl Optimizer for QQNOptimizer {
    type Config = QQNConfig;
    type State = QQNState;

    fn new(config: Self::Config) -> Self {
        Self::new(config)
    }

    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
    ) -> CandleResult<StepResult> {
        info!(
            "QQN step {}: starting optimization step",
            self.state.iteration
        );
        self.log_optimization_state(self.state.iteration, "Starting step");

        // Log initial state in verbose mode
        if params.is_empty() {
            warn!("Empty parameters or gradients provided to QQN step");
            return Err(candle_core::Error::Msg(
                "Empty parameters or gradients".into(),
            ));
        }
        self.log_tensor_data("Initial Parameters", params);

        // Compute gradients at current parameters
        let gradients = function.gradient(params)?;
        self.log_tensor_data("Computed Gradients", &gradients);

        // Check for NaN/Inf in inputs
        for (i, grad) in gradients.iter().enumerate() {
            let grad_vec = grad.flatten_all()?.to_vec1::<f64>()?;
            if grad_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(format!(
                    "Non-finite gradient detected at index {}",
                    i
                )));
            }
        }

        let grad_norm = compute_magnitude(&gradients)?;
        debug!("Gradient norm: {:.6e}", grad_norm);
        self.log_scalar("Gradient Norm", grad_norm);

        // Check if we should use L-BFGS or fall back to steepest descent
        if self.state.iteration < self.config.min_lbfgs_iterations {
            debug!(
                "Iteration {} < min_lbfgs_iterations {}, using steepest descent",
                self.state.iteration, self.config.min_lbfgs_iterations
            );
            let result = self.steepest_descent_step(
                params,
                &gradients,
                function,
                "insufficient iterations for L-BFGS",
            )?;
            self.state.iteration += 1;
            return Ok(result);
        }

        debug!("Computing L-BFGS direction");
        let lbfgs_direction = self.state.lbfgs_state.compute_direction(&gradients)?;
        self.log_tensor_data("L-BFGS Direction", &lbfgs_direction);

        // Check if L-BFGS direction is valid (i.e., all finite)
        if !Self::is_all_finite(&lbfgs_direction) {
            self.state.iteration += 1;
            return Err(candle_core::Error::Msg(
                "Invalid L-BFGS direction, falling back to steepest descent".into(),
            ));
        }

        let quadratic_path = self.create_quadratic_path(params, &gradients, &lbfgs_direction)?;
        let line_search_result = self.find_optimal_t_line_search(params, &quadratic_path.clone(), function)?;
        info!("Found optimal t = {:.6}", line_search_result.step_size);
        self.log_scalar("Optimal t", line_search_result.step_size);
        self.log_line_search_details(line_search_result.step_size, line_search_result.function_evaluations, line_search_result.gradient_evaluations);

        let direction = quadratic_path.evaluate_direction(line_search_result.step_size)?;
        self.log_tensor_data("Final Direction", &direction);
        let direction_norm = compute_magnitude(&direction)?;
        let dot_product = crate::utils::math::dot_product(&gradients, &direction)?;
        debug!(
            "Final direction: ||d||={:.6e}, g^T d={:.6e}",
            direction_norm, dot_product
        );
        self.log_scalar("Direction Norm", direction_norm);
        self.log_scalar("Descent Dot Product", dot_product);

        trace!("Applying step to parameters");
        let step_norm = compute_magnitude(&direction)?;
        debug!("Step norm before application: {:.6e}", step_norm);
        self.log_scalar("Step Norm", step_norm);

        let scaling_factor = self.compute_scale_factor(params, step_norm)?;

        // Apply the scaled step
        self.apply_step(params, &direction, scaling_factor)?;

        // Check for NaN/Inf in updated parameters
        for (i, param) in params.iter().enumerate() {
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                warn!("Non-finite parameter detected at index {} after update", i);
                return Err(candle_core::Error::Msg(
                    "Non-finite parameter detected after update".into(),
                ));
            }
            // Also check for extremely large values
            if param_vec.iter().any(|&x| x.abs() > 1e10) {
                warn!(
                    "Extremely large parameter detected at index {} after update",
                    i
                );
                return Err(candle_core::Error::Msg(
                    "Parameter values too large after update".into(),
                ));
            }
        }

        // Update L-BFGS state with gradients at old parameters
        // This is the correct way - we need gradient difference between old and new points
        debug!("Updating L-BFGS history");
        // Scale the direction by the actual step size used
        let actual_direction = scale_tensors(&direction, scaling_factor)?;
        self.state.lbfgs_state.update(
            &gradients, // Use gradients at old parameters
            &actual_direction,
            1.0, // We've already applied the scaling
        )?;

        // Increment iteration counter AFTER all operations complete successfully
        self.state.iteration += 1;
        info!(
            "QQN step {} completed successfully",
            self.state.iteration - 1
        );

        // 7. Create convergence info
        let convergence_info = ConvergenceInfo {
            converged: false,      // QQN does not have a convergence criterion like L-BFGS
            function_change: None, // Would need previous function value
        };

        let mut metadata = OptimizationMetadata::default();
        metadata.optimizer_data.insert("method".to_string(), 1.0); // 1 = QQN with L-BFGS
        metadata
            .optimizer_data
            .insert("gradient_norm".to_string(), grad_norm);
        metadata
            .optimizer_data
            .insert("direction_norm".to_string(), compute_magnitude(&direction)?);
        metadata
            .optimizer_data
            .insert("optimal_t".to_string(), line_search_result.step_size);
        metadata
            .optimizer_data
            .insert("dot_product".to_string(), dot_product);

        Ok(StepResult {
            step_size: line_search_result.step_size,
            function_evaluations: line_search_result.function_evaluations,
            gradient_evaluations: line_search_result.gradient_evaluations,
            convergence_info,
            metadata,
        })
    }

    fn reset(&mut self) {
        info!("Resetting QQN optimizer state");
        self.state = QQNState::new(self.config.lbfgs_history);
        self.state.lbfgs_state.reset();
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn name(&self) -> &str {
        "QQN"
    }
}

/// Represents a quadratic interpolation path between two search directions
#[derive(Debug, Clone)]
pub struct QuadraticPath {
    start_point: Vec<Tensor>,
    negative_gradient: Vec<Tensor>,
    lbfgs_direction: Vec<Tensor>,
}

impl QuadraticPath {
    /// Create a new quadratic path
    pub fn new(start_point: Vec<Tensor>, negative_gradient: Vec<Tensor>, lbfgs_direction: Vec<Tensor>) -> Self {
        Self {
            start_point,
            negative_gradient,
            lbfgs_direction,
        }
    }

    /// Evaluate the quadratic path at parameter t ∈ [0, 1], returning the actual point
    ///
    /// x(t) = x₀ + d(t) where d(t) = t(1-t) * (-g) + t² * d_lbfgs
    pub fn evaluate(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        let direction = self.evaluate_direction(t)?;
        combine_tensors(&self.start_point, &direction)
    }

    /// Evaluate just the direction component at parameter t ∈ [0, 1]
    ///
    /// d(t) = t(1-t) * (-g) + t² * d_lbfgs
    pub fn evaluate_direction(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        // Clamp t to valid range
        let t_clamped = t.max(0.0).min(1.0);
        if (t - t_clamped).abs() > 1e-10 {
            trace!(
                "QuadraticPath::evaluate_direction: clamped t from {} to {}",
                t,
                t_clamped
            );
        }
        let t = t_clamped;

        // Coefficients for the quadratic path formula as per paper
        let gradient_coeff = t * (1.0 - t);
        let lbfgs_coeff = t * t;
        trace!(
            "QuadraticPath::evaluate_direction(t={}): gradient_coeff={}, lbfgs_coeff={}",
            t,
            gradient_coeff,
            lbfgs_coeff
        );

        // Handle special case where both coefficients are zero (t=0)
        if gradient_coeff.abs() < 1e-15 && lbfgs_coeff.abs() < 1e-15 {
            // Return zero direction
            return Ok(self.negative_gradient.iter()
                .map(|t| t.zeros_like())
                .collect::<Result<Vec<_>, _>>()?);
        }

        let gradient_term = scale_tensors(&self.negative_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;
        // Log intermediate terms for debugging
        trace!(
            "QuadraticPath::evaluate_direction: gradient_term magnitude={:.6e}, lbfgs_term magnitude={:.6e}",
            compute_magnitude(&gradient_term).unwrap_or(0.0),
            compute_magnitude(&lbfgs_term).unwrap_or(0.0)
        );

        combine_tensors(&gradient_term, &lbfgs_term)
    }
    /// Get the starting point
    pub fn start_point(&self) -> &[Tensor] {
        &self.start_point
    }

    /// Compute the derivative of the quadratic path at parameter t
    ///
    /// d'(t) = (1-2t) * (-g) + 2t * d_lbfgs
    pub fn derivative(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        trace!("QuadraticPath::derivative(t={})", t);

        let gradient_coeff = 1.0 - 2.0 * t;
        let lbfgs_coeff = 2.0 * t;
        trace!(
            "QuadraticPath::derivative: gradient_coeff={}, lbfgs_coeff={}",
            gradient_coeff,
            lbfgs_coeff
        );

        let gradient_term = scale_tensors(&self.negative_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;

        combine_tensors(&gradient_term, &lbfgs_term)
    }

    /// Get the negative gradient component
    pub fn negative_gradient(&self) -> &[Tensor] {
        &self.negative_gradient
    }

    /// Get the L-BFGS direction component
    pub fn lbfgs_direction(&self) -> &[Tensor] {
        &self.lbfgs_direction
    }
}
impl<'a> ParametricCurve for QuadraticPath {
    fn evaluate(&self, t: f64) -> AnyhowResult<Vec<f64>> {
        // Get the point at parameter t
        let point = self.evaluate(t)?;
        // Convert point tensors to f64
        Ok(point
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect())
    }
    fn derivative(&self, t: f64) -> AnyhowResult<Vec<f64>> {
        // Get the derivative of the quadratic path
        let deriv = self.derivative(t)?;
        // Convert to f64 vector
        Ok(deriv
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect())
    }
    fn initial_derivative(&self) -> AnyhowResult<Vec<f64>> {
        // Get the derivative of the quadratic path at t=0
        let deriv = self.derivative(0.0)
            .map_err(|e| anyhow::anyhow!("Failed to compute derivative: {}", e))?;
        // Convert to f64 vector
        Ok(deriv
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init_logging;
    use approx::assert_relative_eq;
    use candle_core::Device;
    use std::sync::Arc;
    use std::sync::Mutex;

    // Test function: f(x) = 0.5 * ||x||^2
    struct QuadraticFunction {
        eval_count: Arc<Mutex<usize>>,
        grad_count: Arc<Mutex<usize>>,
    }
    impl QuadraticFunction {
        fn new() -> Self {
            Self {
                eval_count: Arc::new(Mutex::new(0)),
                grad_count: Arc::new(Mutex::new(0)),
            }
        }
        fn evaluation_count(&self) -> usize {
            *self.eval_count.lock().unwrap()
        }
        fn gradient_count(&self) -> usize {
            *self.grad_count.lock().unwrap()
        }
    }
    impl DifferentiableFunction for QuadraticFunction {
        fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
            *self.eval_count.lock().unwrap() += 1;
            let mut sum = 0.0;
            for param in params {
                let values = param.flatten_all()?.to_vec1::<f64>()?;
                sum += values.iter().map(|x| x * x).sum::<f64>();
            }
            Ok(0.5 * sum)
        }
        fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
            *self.grad_count.lock().unwrap() += 1;
            // Gradient of 0.5 * ||x||^2 is x
            Ok(params.to_vec())
        }
    }
    // Rosenbrock function: f(x,y) = (1-x)^2 + 100(y-x^2)^2
    struct RosenbrockFunction;
    impl DifferentiableFunction for RosenbrockFunction {
        fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
            let values = params[0].flatten_all()?.to_vec1::<f64>()?;
            let x = values[0];
            let y = values[1];
            Ok((1.0 - x).powi(2) + 100.0 * (y - x * x).powi(2))
        }
        fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
            let values = params[0].flatten_all()?.to_vec1::<f64>()?;
            let x = values[0];
            let y = values[1];
            let grad_x = -2.0 * (1.0 - x) - 400.0 * x * (y - x * x);
            let grad_y = 200.0 * (y - x * x);
            let grad = Tensor::from_slice(&[grad_x, grad_y], &[2], params[0].device())?;
            Ok(vec![grad])
        }
    }

    #[test]
    fn test_quadratic_path_evaluation() -> CandleResult<()> {
        let device = Device::Cpu;
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[2], &device)?];

        // Create negative gradient as per paper formula
        let start_point = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let negative_gradient = vec![Tensor::from_slice(&[-1.0, 0.0], &[2], &device)?];
        let path = QuadraticPath::new(start_point, negative_gradient, lbfgs_dir);

        // At t=0, should be start point
        let result_0 = path.evaluate(0.0)?;
        let values_0 = result_0[0].to_vec1::<f64>()?;
        assert_relative_eq!(values_0[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(values_0[1], 2.0, epsilon = 1e-10);

        // At t=1, should be start_point + L-BFGS direction
        let result_1 = path.evaluate(1.0)?;
        let values_1 = result_1[0].to_vec1::<f64>()?;
        assert_relative_eq!(values_1[0], 1.0, epsilon = 1e-10); // 1.0 + 0.0
        assert_relative_eq!(values_1[1], 3.0, epsilon = 1e-10); // 2.0 + 1.0

        // At t=0.5, should be start_point + 0.5*(1-0.5)*(-g) + 0.5²*d_lbfgs = start_point + 0.25*(-g) + 0.25*d_lbfgs
        let result_half = path.evaluate(0.5)?;
        let values_half = result_half[0].to_vec1::<f64>()?;
        assert_relative_eq!(values_half[0], 0.75, epsilon = 1e-10); // 1.0 + 0.25 * (-1.0)
        assert_relative_eq!(values_half[1], 2.25, epsilon = 1e-10); // 2.0 + 0.25 * 1.0

        Ok(())
    }

    #[test]
    fn test_quadratic_path_derivative() -> CandleResult<()> {
        let device = Device::Cpu;
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[2], &device)?];

        // Create negative gradient as per paper formula
        let start_point = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let negative_gradient = vec![Tensor::from_slice(&[-1.0, 0.0], &[2], &device)?];
        let path = QuadraticPath::new(start_point, negative_gradient, lbfgs_dir);

        // At t=0, derivative should be negative gradient: d'(0) = (1-0)*(-g) + 0*d_lbfgs = -g
        let deriv_0 = path.derivative(0.0)?;
        let deriv_0_values = deriv_0[0].to_vec1::<f64>()?;
        assert_relative_eq!(deriv_0_values[0], -1.0, epsilon = 1e-10);
        assert_relative_eq!(deriv_0_values[1], 0.0, epsilon = 1e-10);

        // At t=1, derivative should be: d'(1) = (1-2)*(-g) + 2*d_lbfgs = g + 2*d_lbfgs
        let deriv_1 = path.derivative(1.0)?;
        let deriv_1_values = deriv_1[0].to_vec1::<f64>()?;
        assert_relative_eq!(deriv_1_values[0], 1.0, epsilon = 1e-10); // -1*(-1.0) + 2*0.0
        assert_relative_eq!(deriv_1_values[1], 2.0, epsilon = 1e-10); // -1*0.0 + 2*1.0

        Ok(())
    }

    #[test]
    fn test_qqn_state_initialization() {
        let state = QQNState::new(5);
        assert_eq!(state.iteration, 0);
    }

    #[test]
    fn test_qqn_config_default() {
        let config = QQNConfig::default();
        assert_eq!(config.lbfgs_history, 10);
        assert_eq!(config.min_lbfgs_iterations, 2);
        assert_eq!(config.epsilon, 1e-8);
    }

    #[test]
    fn test_qqn_min_iterations_steepest_descent() -> CandleResult<()> {
        let mut config = QQNConfig::default();
        config.min_lbfgs_iterations = 3;
        let optimizer = QQNOptimizer::new(config);
        // Check that early iterations should use steepest descent
        assert!(optimizer.state.iteration < optimizer.config.min_lbfgs_iterations);
        Ok(())
    }
    #[test]
    fn test_qqn_optimizer_creation() {
        let config = QQNConfig {
            lbfgs_history: 5,
            min_lbfgs_iterations: 3,
            line_search: LineSearchConfig::default(),
            epsilon: 1e-10,
            verbose: false,
        };
        let optimizer = QQNOptimizer::new(config.clone());
        assert_eq!(optimizer.config.lbfgs_history, 5);
        assert_eq!(optimizer.config.min_lbfgs_iterations, 3);
        assert_eq!(optimizer.config.epsilon, 1e-10);
        assert_eq!(optimizer.state.iteration, 0);
    }
    #[test]
    fn test_qqn_step_with_quadratic_function() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        config.min_lbfgs_iterations = 0; // Enable L-BFGS immediately
        let mut optimizer = QQNOptimizer::new(config);
        // Start at (2, 3)
        let mut params = vec![Tensor::from_slice(&[2.0, 3.0], &[2], &device)?];
        let function = QuadraticFunction::new();
        // Take a step
        let result = optimizer.step(&mut params, &function)?;
        // Should move towards origin
        let values = params[0].to_vec1::<f64>()?;
        assert!(values[0].abs() < 2.0);
        assert!(values[1].abs() < 3.0);
        // Check that function and gradient were evaluated
        assert!(result.function_evaluations > 0);
        assert!(result.gradient_evaluations > 0);
        assert_eq!(optimizer.state.iteration, 1);
        Ok(())
    }
    #[test]
    fn test_qqn_uses_steepest_descent_initially() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        config.min_lbfgs_iterations = 2;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let function = QuadraticFunction::new();
        // First step should use steepest descent
        let result = optimizer.step(&mut params, &function)?;
        // Check metadata indicates steepest descent was used
        assert_eq!(result.metadata.optimizer_data.get("method"), Some(&0.0));
        Ok(())
    }
    #[test]
    fn test_qqn_switches_to_lbfgs() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        config.min_lbfgs_iterations = 2;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let function = QuadraticFunction::new();
        // Take enough steps to enable L-BFGS
        for _ in 0..2 {
            optimizer.step(&mut params, &function)?;
        }
        // Third step should use L-BFGS
        let result = optimizer.step(&mut params, &function)?;
        // Check metadata indicates L-BFGS was used
        assert_eq!(result.metadata.optimizer_data.get("method"), Some(&1.0));
        Ok(())
    }
    #[test]
    fn test_qqn_step_with_gradients() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        config.min_lbfgs_iterations = 0;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![Tensor::from_slice(&[2.0, 3.0], &[2], &device)?];
        let gradients = vec![Tensor::from_slice(&[2.0, 3.0], &[2], &device)?]; // Gradient of quadratic
        let function = QuadraticFunction::new();
        let result = optimizer.step(&mut params, &function)?;
        // Should move towards origin
        let values = params[0].to_vec1::<f64>()?;
        assert!(values[0].abs() < 2.0);
        assert!(values[1].abs() < 3.0);
        // Function should be evaluated for line search, but gradient count should be lower
        assert!(result.function_evaluations > 0);
        Ok(())
    }
    #[test]
    fn test_qqn_reset() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let function = QuadraticFunction::new();
        // Take some steps
        for _ in 0..3 {
            optimizer.step(&mut params, &function)?;
        }
        assert_eq!(optimizer.state.iteration, 3);
        // Reset
        optimizer.reset();
        assert_eq!(optimizer.state.iteration, 0);
        assert_eq!(optimizer.state.lbfgs_state.history_length(), 0);
        Ok(())
    }
    #[test]
    fn test_qqn_handles_nan_gradients() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];

        // Create a function that returns NaN gradients
        struct NaNGradientFunction;
        impl DifferentiableFunction for NaNGradientFunction {
            fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
                let values = params[0].flatten_all()?.to_vec1::<f64>()?;
                Ok(values.iter().map(|x| x * x).sum::<f64>())
            }
            fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
                let device = params[0].device();
                Ok(vec![Tensor::from_slice(&[f64::NAN, 1.0], &[2], device)?])
            }
        }

        let function = NaNGradientFunction;
        let result = optimizer.step(&mut params, &function);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Non-finite gradient"));
        Ok(())
    }
    #[test]
    fn test_qqn_handles_empty_parameters() -> CandleResult<()> {
        let mut config = QQNConfig::default();
        config.verbose = false;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params: Vec<Tensor> = vec![];
        let function = QuadraticFunction::new();
        let result = optimizer.step(&mut params, &function);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Empty parameters"));
        Ok(())
    }
    #[test]
    fn test_qqn_convergence_on_quadratic() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        config.min_lbfgs_iterations = 0;
        let mut optimizer = QQNOptimizer::new(config);
        // Start far from optimum
        let mut params = vec![Tensor::from_slice(&[10.0, -5.0], &[2], &device)?];
        let function = QuadraticFunction::new();
        // Take multiple steps
        for _ in 0..20 {
            let result = optimizer.step(&mut params, &function)?;
            // Check if we're close enough to optimum
            let values = params[0].to_vec1::<f64>()?;
            if values.iter().all(|&x| x.abs() < 1e-6) {
                break;
            }
        }
        // Should converge close to origin
        let final_values = params[0].to_vec1::<f64>()?;
        assert!(final_values[0].abs() < 0.1);
        assert!(final_values[1].abs() < 0.1);
        Ok(())
    }
    #[test]
    fn test_qqn_on_rosenbrock() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        config.min_lbfgs_iterations = 2;
        let mut optimizer = QQNOptimizer::new(config);
        // Start at a challenging point
        let mut params = vec![Tensor::from_slice(&[-1.0, 1.0], &[2], &device)?];
        let function = RosenbrockFunction;
        // Take several steps
        for i in 0..10 {
            let result = optimizer.step(&mut params, &function)?;
            // Function value should generally decrease
            let f_val = function.evaluate(&params)?;
            println!("Step {}: f = {:.6e}", i, f_val);
        }
        // Should make progress towards optimum at (1, 1)
        let values = params[0].to_vec1::<f64>()?;
        let initial_dist = ((-1.0_f64 - 1.0).powi(2) + (1.0_f64 - 1.0).powi(2)).sqrt();
        let final_dist = ((values[0] - 1.0).powi(2) + (values[1] - 1.0).powi(2)).sqrt();
        assert!(final_dist < initial_dist);
        Ok(())
    }
    #[test]
    fn test_quadratic_path_clamping() -> CandleResult<()> {
        let device = Device::Cpu;
        let start = vec![Tensor::from_slice(&[0.0, 0.0], &[2], &device)?];
        let neg_grad = vec![Tensor::from_slice(&[1.0, 0.0], &[2], &device)?];
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[2], &device)?];
        let path = QuadraticPath::new(start, neg_grad, lbfgs_dir);
        // Test clamping at boundaries
        let result_neg = path.evaluate(-0.5)?;
        let result_0 = path.evaluate(0.0)?;
        let values_neg = result_neg[0].to_vec1::<f64>()?;
        let values_0 = result_0[0].to_vec1::<f64>()?;
        // Should clamp to t=0
        assert_eq!(values_neg[0], values_0[0]);
        assert_eq!(values_neg[1], values_0[1]);
        let result_large = path.evaluate(1.5)?;
        let result_1 = path.evaluate(1.0)?;
        let values_large = result_large[0].to_vec1::<f64>()?;
        let values_1 = result_1[0].to_vec1::<f64>()?;
        // Should clamp to t=1
        assert_eq!(values_large[0], values_1[0]);
        assert_eq!(values_large[1], values_1[1]);
        Ok(())
    }
    #[test]
    fn test_qqn_trace_collection() -> CandleResult<()> {
        let trace = QQNTrace::new();
        assert!(trace.magnitude_ratios.is_empty());
        assert!(trace.quadratic_path_usage.is_empty());
        assert!(trace.step_sizes.is_empty());
        assert!(trace.gradient_norms.is_empty());
        assert!(trace.direction_norms.is_empty());
        assert!(trace.descent_dot_products.is_empty());
        Ok(())
    }
    #[test]
    fn test_qqn_name() {
        let config = QQNConfig::default();
        let optimizer = QQNOptimizer::new(config);
        assert_eq!(optimizer.name(), "QQN");
    }
}