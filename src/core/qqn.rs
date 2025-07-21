use crate::core::lbfgs::LBFGSState;
use crate::core::line_search::{
    create_1d_problem, create_1d_problem_linear, create_line_search, LineSearch, LineSearchResult,
    ParametricCurve,
};
use crate::core::optimizer::OptimizationMetadata;
use crate::core::Optimizer;
use crate::core::StepResult;
use crate::core::{ConvergenceInfo, TerminationReason};
use crate::utils::math::{
    combine_tensors, compute_magnitude, create_1d_tensor, log_tensor, scale_tensors,
    DifferentiableFunction,
};
use crate::LineSearchConfig;
use anyhow::{anyhow, Result as AnyhowResult};
use candle_core::{Device, Error, Result as CandleResult, Tensor};
use log::{debug, error, info, trace, warn};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use crate::utils::dot_product;

/// QQN trace information for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QQNTrace {
    pub magnitude_ratios: Vec<f64>,
    pub quadratic_path_usage: Vec<bool>,
    pub step_sizes: Vec<f64>,
    pub gradient_norms: Vec<f64>,
    pub direction_norms: Vec<f64>,
    pub descent_dot_products: Vec<f64>,
    pub function_values: Vec<f64>,
    pub iteration_times: Vec<f64>,
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
            function_values: Vec::new(),
            iteration_times: Vec::new(),
        }
    }
    /// Add a new trace entry
    pub fn add_entry(
        &mut self,
        magnitude_ratio: f64,
        used_quadratic: bool,
        step_size: f64,
        gradient_norm: f64,
        direction_norm: f64,
        descent_dot_product: f64,
        function_value: f64,
        iteration_time: f64,
    ) {
        self.magnitude_ratios.push(magnitude_ratio);
        self.quadratic_path_usage.push(used_quadratic);
        self.step_sizes.push(step_size);
        self.gradient_norms.push(gradient_norm);
        self.direction_norms.push(direction_norm);
        self.descent_dot_products.push(descent_dot_product);
        self.function_values.push(function_value);
        self.iteration_times.push(iteration_time);
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
            min_lbfgs_iterations: 1,
            line_search: LineSearchConfig {
                method: crate::core::line_search::LineSearchMethod::Bisection,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-6,
            verbose: false,
        }
    }
}
impl QQNConfig {
    /// Create a strict configuration with conservative settings for robust convergence
    /// - Larger L-BFGS history for better approximation
    /// - More steepest descent iterations before enabling L-BFGS
    /// - Tighter numerical stability constant
    /// - More conservative line search settings
    pub fn strict() -> Self {
        Self {
            lbfgs_history: 20,
            min_lbfgs_iterations: 5,  // More steepest descent iterations
            line_search: LineSearchConfig {
                method: crate::core::line_search::LineSearchMethod::Bisection,
                max_iterations: 50,
                c1: 1e-4,
                c2: 0.9,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            verbose: false,
        }
    }
    /// Create a lax configuration with aggressive settings for faster convergence
    /// - Smaller L-BFGS history for computational efficiency
    /// - Fewer steepest descent iterations before enabling L-BFGS
    /// - Looser numerical stability constant
    /// - More aggressive line search settings
    pub fn lax() -> Self {
        Self {
            lbfgs_history: 5,
            min_lbfgs_iterations: 1,
            line_search: LineSearchConfig {
                method: crate::core::line_search::LineSearchMethod::Bisection,
                max_iterations: 20,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-4,
            verbose: false,
        }
    }
    /// Create a configuration with verbose logging enabled
    pub fn verbose() -> Self {
        Self {
            verbose: true,
            ..Self::default()
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
            lbfgs_state: LBFGSState::new_with_options(lbfgs_history, 1e-8, true), // Disable checks for QQN
        }
    }
}

#[derive(Debug)]
pub struct QQNOptimizer {
    config: QQNConfig,
    pub state: QQNState,
    line_search: Box<dyn LineSearch>,
    trace: Option<QQNTrace>,
}
impl Clone for QQNOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            line_search: self.line_search.clone_box(),
            trace: None,
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
            trace: None,
        }
    }
    /// Enable trace collection
    pub fn enable_trace(&mut self) {
        self.trace = Some(QQNTrace::new());
    }
    /// Get the collected trace
    pub fn get_trace(&self) -> Option<&QQNTrace> {
        self.trace.as_ref()
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
            debug!("  {}: {:.3e}", name, value);
        }
    }

    /// Log optimization state if verbose mode is enabled
    fn log_optimization_state(&self, iteration: usize, additional_info: &str) {
        if !self.config.verbose {
            return;
        }
        debug!("=== QQN Optimization State (Iteration {}) ===", iteration);
        debug!(
            "  L-BFGS History Length: {}",
            self.state.lbfgs_state.history_length()
        );
        debug!("  L-BFGS Gamma: {:.6e}", self.state.lbfgs_state.gamma());
        debug!("  Additional Info: {}", additional_info);
    }

    /// Log line search details if verbose mode is enabled
    fn log_line_search_details(&self, optimal_t: f64) {
        if !self.config.verbose {
            return;
        }
        debug!("=== Line Search Results ===");
        debug!("  Optimal t: {:.3e}", optimal_t);
    }

    pub fn create_quadratic_path(
        &self,
        start_point: &[Tensor],
        gradient: &[Tensor],
        lbfgs_direction: &[Tensor],
        function: Arc<dyn DifferentiableFunction + Send + Sync>,
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
            return Err(Error::Msg(
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
            return Err(Error::Msg(format!(
                "Dimension mismatch: start_point={}, gradient={}, direction={}",
                start_point.len(),
                gradient.len(),
                lbfgs_direction.len()
            )));
        }
        // Check for valid tensors
        for (i, tensor) in start_point.iter().enumerate() {
            if tensor.elem_count() == 0 {
                return Err(Error::Msg(format!(
                    "Empty tensor at index {} in start_point",
                    i
                )));
            }
        }

        // Create negative gradient
        let negative_gradient = gradient.iter()
            .map(|g| g.neg())
            .collect::<CandleResult<Vec<_>>>()?;

        // Log created tensors in verbose mode
        self.log_tensor_data("Negative Gradient", &negative_gradient);

        // Log norms for debugging
        let grad_norm = compute_magnitude(&negative_gradient)?;
        let lbfgs_norm = compute_magnitude(lbfgs_direction)?;
        debug!(
            "Quadratic path created: ||gradient||={:.3e}, ||lbfgs_dir||={:.3e}",
            grad_norm, lbfgs_norm
        );
        self.log_scalar("Gradient Norm", grad_norm);
        self.log_scalar("L-BFGS Direction Norm", lbfgs_norm);
        trace!("Quadratic path formula: d(t) = t(1-t)(-g) + t²d_lbfgs");

        Ok(QuadraticPath::new(
            start_point.to_vec(),
            negative_gradient,
            lbfgs_direction.to_vec(),
            Arc::new(Mutex::new(self.state.lbfgs_state.clone())),
            function,
        ))
    }

    /// Find optimal t parameter for the quadratic path using line search
    fn find_optimal_t_line_search(
        &mut self,
        quadratic_path: QuadraticPath,
    ) -> CandleResult<LineSearchResult> {
        debug!("Starting line search for optimal t along quadratic path");
        // Check if we have a valid descent direction at a small t > 0
        // At t=0, the quadratic path direction is zero, so we check at a small positive t
        let test_t = 1e-8;
        let test_direction = quadratic_path.evaluate_direction(test_t)?;
        let test_gradient = quadratic_path.negative_gradient();
        
        // The descent condition should check if moving along the path decreases the function
        // We need to check the directional derivative at the starting point
        let descent_check = test_gradient.iter()
            .zip(test_direction.iter())
            .map(|(g, d)| {
                // g is already the negative gradient, so positive dot product means descent
                let dot = dot_product(&[g.clone()], &[d.clone()])?;
                Ok(dot)
            })
            .collect::<CandleResult<Vec<_>>>()?
            .into_iter()
            .sum::<f64>();
            
        if descent_check <= 0.0 {
            debug!("Quadratic path does not provide a descent direction (dot product = {:.6e})", descent_check);
            return Ok(LineSearchResult {
                step_size: 0.0, // Signal to use steepest descent
                success: false,
                termination_reason: TerminationReason::InvalidDirection,
            });
        }

        let quadratic_path_clone = quadratic_path.clone();
        let value_fn = move |x: &[f64]| -> anyhow::Result<f64> {
            let tensors = [create_1d_tensor(x, &Device::Cpu)?].to_vec();
            quadratic_path_clone
                .function
                .evaluate(&tensors)
                .map_err(|e| anyhow::anyhow!("Function evaluation failed: {}", e))
        };
        let quadratic_path_clone2 = quadratic_path.clone();
        let gradient_fn = move |x: &[f64]| -> anyhow::Result<Vec<f64>> {
            let tensors = [create_1d_tensor(x, &Device::Cpu)?].to_vec();
            let grads = quadratic_path_clone2.function.gradient(&tensors)
                .map_err(|e| anyhow::anyhow!("Gradient evaluation failed: {}", e))?;
            let mut result = Vec::new();
            for grad_tensor in grads {
                let flattened = grad_tensor.flatten_all()
                    .map_err(|e| anyhow::anyhow!("Failed to flatten gradient: {}", e))?;
                let values: Vec<f64> = flattened.to_vec1::<f64>()
                    .map_err(|e| anyhow::anyhow!("Failed to convert gradient to vec: {}", e))?;
                result.extend(values);
            }
            Ok(result)
        };
        let problem = create_1d_problem(Box::new(quadratic_path), Arc::new(value_fn), Arc::new(gradient_fn))
            .map_err(|e| Error::Msg(format!("Failed to create 1D problem: {}", e)));
        if problem.is_err() {
            warn!("Failed to create 1D problem for line search: {}", problem.as_ref().err().unwrap());
            return Err(Error::Msg(format!(
                "Failed to create 1D problem for line search: {}",
                problem.as_ref().err().unwrap()
            )));
        }
        // Perform line search
        let result = self.line_search.optimize_1d(&problem?).unwrap_or_else(|e| {
            warn!("Line search failed: {}", e);
            LineSearchResult {
                step_size: 1.0, // Default to 1.0 if search fails
                success: false,
                termination_reason: TerminationReason::WolfeConditionsSatisfied,
            }
        });
        debug!(
            "Line search completed: t*={:.3e}, success={}",
            result.step_size, result.success
        );
        Ok(result)
    }

    /// Perform steepest descent step with line search for adaptive learning rate
    fn steepest_descent_step(
        &mut self,
        nd_params: &mut [Tensor],
        gradients: &[Tensor],
        function: Arc<dyn DifferentiableFunction + Send + Sync>,
        reason: &str,
    ) -> CandleResult<StepResult> {
        info!("Using steepest descent: {}", reason);
        // Evaluate function at current parameters to check for increasing steps
        let initial_function_value = function.evaluate(nd_params)?;
        debug!(
            "Initial function value (steepest descent): {:.6e}",
            initial_function_value
        );

        // Create steepest descent direction (negative gradient)
        let direction = scale_tensors(gradients, -1.0)?;
        self.log_tensor_data("Steepest Descent Direction", &direction);
        // Scale direction if gradient is too large to avoid numerical issues
        let grad_norm = compute_magnitude(gradients)?;
        let direction = if grad_norm > 100.0 {
            // Use a more aggressive scaling for very large gradients
            let scale_factor = 10.0 / grad_norm;
            warn!(
                "Large gradient norm {:.3e}, scaling direction by {:.3e}",
                grad_norm, scale_factor
            );
            scale_tensors(&direction, scale_factor)?
        } else if grad_norm < 1e-6 {
            warn!(
                "Very small gradient norm {:.3e}, convergence likely achieved",
                grad_norm
            );
            return Ok(StepResult {
                step_size: 0.0,
                convergence_info: ConvergenceInfo {
                    converged: true,
                    function_change: Some(0.0),
                },
                metadata: OptimizationMetadata::default(),
            });
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

        // Collect the shapes and device info we need before the closures
        let param_shapes: Vec<_> = nd_params.iter().map(|p| p.shape().clone()).collect();
        let param_device = nd_params[0].device().clone();
        
        // Perform line search in a separate scope to avoid borrow conflicts
        let line_search_result = {
            // Create objective and gradient functions
            let function_clone = function.clone();
            let param_shapes_clone = param_shapes.clone();
            let param_device_clone = param_device.clone();
            let objective_fn = move |x: &[f64]| -> anyhow::Result<f64> {
                let mut tensors = Vec::new();
                let mut idx = 0;
                for shape in &param_shapes_clone {
                    let size = shape.elem_count();
                    let slice = &x[idx..idx + size];
                    let tensor = Tensor::from_slice(slice, shape.dims(), &param_device_clone)
                        .map_err(|e| anyhow!("Failed to create tensor: {}", e))?;
                    tensors.push(tensor);
                    idx += size;
                }
                function_clone
                    .evaluate(&tensors)
                    .map_err(|e| anyhow!("Function evaluation failed: {}", e))
            };
            let function_clone = function.clone();
            let param_shapes_clone = param_shapes.clone();
            let param_device_clone = param_device.clone();
            let gradient_fn = move |x: &[f64]| -> anyhow::Result<Vec<f64>> {
                // Reconstruct the full parameter tensors from the flattened vector
                
                let mut tensors = Vec::new();
                let mut idx = 0;
                for shape in &param_shapes_clone {
                    let size = shape.elem_count();
                    let slice = &x[idx..idx + size];
                    let tensor = Tensor::from_slice(slice, shape.dims(), &param_device_clone)
                        .map_err(|e| anyhow!("Failed to create tensor: {}", e))?;
                    tensors.push(tensor);
                    idx += size;
                }
                let grads = function_clone
                    .gradient(&tensors)
                    .map_err(|e| anyhow!("Gradient evaluation failed: {}", e))?;
                Ok(grads
                    .iter()
                    .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
                    .collect())
            };

            // Create 1D problem
            let problem =
               create_1d_problem_linear(&params_f64, &direction_f64, Arc::new(objective_fn), Arc::new(gradient_fn))
                    .map_err(|e| Error::Msg(format!("Failed to create 1D problem: {}", e)))?;

            // Perform line search
            self.line_search.optimize_1d(&problem).map_err(|e| {
                warn!("Line search failed: {}", e);
                Error::Msg(format!("Line search failed: {}", e))
            })
        };

        if line_search_result.is_err() || !line_search_result.as_ref().unwrap().success {
            warn!("Line search failed, fatal error!");
            return Err(Error::Msg(
                "Line search failed, cannot proceed with steepest descent".into(),
            ));
        }

        let line_search_result = line_search_result?;

        if !line_search_result.success {
            warn!(
                "Line search did not succeed: step_size={:.3e}, reason={}",
                line_search_result.step_size, reason
            );
            // Don't fail completely, just use a very small step
            warn!("Using minimal step size as fallback");
        }

        debug!(
            "Steepest descent line search completed: step_size={:.3e}, success={}",
            line_search_result.step_size, line_search_result.success
        );
        self.log_scalar("Steepest Descent Step Size", line_search_result.step_size);
        // Save old parameters before updating
        let old_params = nd_params.to_vec();


        // Apply the step
        for (param, dir) in nd_params.iter_mut().zip(direction.iter()) {
            *param = (param.clone() + (dir * line_search_result.step_size)?)?;
        }

        // FATAL ERROR CHECK: Verify that the steepest descent step decreased the function value
        let final_function_value = function.evaluate(nd_params)?;
        debug!(
            "Final function value (steepest descent): {:.6e}",
            final_function_value
        );
        if final_function_value > initial_function_value {
            let increase = final_function_value - initial_function_value;
            error!(
                "FATAL ERROR: Steepest descent step increased function value by {:.6e} (from {:.6e} to {:.6e}). This should never happen!",
                increase, initial_function_value, final_function_value
            );
            return Err(Error::Msg(format!(
                "FATAL ERROR: Steepest descent step increased function value by {:.6e} (from {:.6e} to {:.6e}). This violates the descent property and should never happen.",
                increase, initial_function_value, final_function_value
            )));
        }
        let function_decrease = initial_function_value - final_function_value;
        debug!(
            "Function decreased by (steepest descent): {:.6e}",
            function_decrease
        );
        self.log_scalar("Function Decrease (Steepest Descent)", function_decrease);
        
        // Update L-BFGS state with the new gradient at the updated position
        let new_gradient = function.gradient(nd_params)?;
        // Only update if we made meaningful progress
        if line_search_result.step_size > 1e-10 {
            self.state.lbfgs_state.update(&old_params, nd_params, &new_gradient)?;
        } else {
            debug!("Step size too small ({:.3e}), skipping L-BFGS update", line_search_result.step_size);
        }
        
        // Create convergence info
        let convergence_info = ConvergenceInfo {
            converged: false,
            function_change: Some(function_decrease),
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
        metadata
            .optimizer_data
            .insert("function_decrease".to_string(), function_decrease);
        metadata
            .optimizer_data
            .insert("initial_function_value".to_string(), initial_function_value);
        metadata
            .optimizer_data
            .insert("final_function_value".to_string(), final_function_value);
        Ok(StepResult {
            step_size: line_search_result.step_size,
            convergence_info,
            metadata,
        })
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
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,
        params: &mut [Tensor],
        function: Arc<dyn DifferentiableFunction + Send + Sync>,
    ) -> CandleResult<StepResult> {
        let step_start = Instant::now();
        info!(
            "QQN step {}: starting optimization step",
            self.state.iteration
        );
        self.log_optimization_state(self.state.iteration, "Starting step");

        // Log initial state in verbose mode
        if params.is_empty() {
            warn!("Empty parameters or gradients provided to QQN step");
            return Err(Error::Msg("Empty parameters or gradients".into()));
        }
        self.log_tensor_data("Initial Parameters", params);
        // Evaluate function at current parameters to check for increasing steps
        let initial_function_value = function.evaluate(params)?;
        debug!("Initial function value: {:.6e}", initial_function_value);

        // Compute gradients at current parameters
        let gradients = function.gradient(params)?;
        self.log_tensor_data("Computed Gradients", &gradients);

        // Check for NaN/Inf in inputs
        for (i, grad) in gradients.iter().enumerate() {
            let grad_vec = grad.flatten_all()?.to_vec1::<f64>()?;
            if grad_vec.iter().any(|&x| !x.is_finite()) {
                return Err(Error::Msg(format!(
                    "Non-finite gradient detected at index {}",
                    i
                )));
            }
        }

        let grad_norm = compute_magnitude(&gradients)?;
        debug!("Gradient norm: {:.3e}", grad_norm);
        self.log_scalar("Gradient Norm", grad_norm);
        if grad_norm < self.config.epsilon {
            info!(
                "QQN converged: gradient norm {:.3e} < epsilon {:.3e}",
                grad_norm, self.config.epsilon
            );
            self.state.iteration += 1;
            let convergence_info = ConvergenceInfo {
                converged: true,
                function_change: None,
            };
            let mut metadata = OptimizationMetadata::default();
            metadata.optimizer_data.insert("method".to_string(), -1.0); // -1 = converged
            metadata
                .optimizer_data
                .insert("gradient_norm".to_string(), grad_norm);
            return Ok(StepResult {
                step_size: 0.0,
                convergence_info,
                metadata,
            });
        }
    // Additional convergence check for very small function changes
    if self.state.iteration > 10 {
        // Check if we're making very small progress
        if let Some(trace) = &self.trace {
            if trace.function_values.len() > 5 {
                let recent_values = &trace.function_values[trace.function_values.len() - 5..];
                let max_change = recent_values.windows(2)
                    .map(|w| (w[0] - w[1]).abs())
                    .fold(0.0, f64::max);
                if max_change < 1e-10 && grad_norm < 1e-3 {
                    info!(
                        "QQN converged: function change {:.3e} < 1e-10 and gradient norm {:.3e} < 1e-3",
                        max_change, grad_norm
                    );
                    self.state.iteration += 1;
                    return Ok(StepResult {
                        step_size: 0.0,
                        convergence_info: ConvergenceInfo {
                            converged: true,
                            function_change: Some(max_change),
                        },
                        metadata: OptimizationMetadata::default(),
                    });
                }
            }
        }
    }

        // Check if we should use L-BFGS or fall back to steepest descent
        if self.state.iteration < self.config.min_lbfgs_iterations {
            debug!(
                "Iteration {} < min_lbfgs_iterations {}, using steepest descent",
                self.state.iteration, self.config.min_lbfgs_iterations
            );
            let result = self.steepest_descent_step(
                params,
                &gradients,
                function.clone(),
                "insufficient iterations for L-BFGS",
            )?;
            self.state.iteration += 1;
            // Update L-BFGS state even during steepest descent to build history
            let new_gradient = function.gradient(params)?;
            self.state.lbfgs_state.update(&params, params, &new_gradient)?;
            return Ok(result);
        }

        debug!("Computing L-BFGS direction");
        let lbfgs_direction = self
            .state
            .lbfgs_state
            .compute_direction(&gradients)?;
        self.log_tensor_data("L-BFGS Direction", &lbfgs_direction);

        // Check if L-BFGS direction is valid (i.e., all finite)
        if !Self::is_all_finite(&lbfgs_direction) {
            warn!("L-BFGS direction contains non-finite values");
            let result = self.steepest_descent_step(
                params,
                &gradients,
                function.clone(),
                "invalid L-BFGS direction",
            )?;
            self.state.iteration += 1;
            return Ok(result);
        }

        debug!("L-BFGS direction computed successfully: {:?}->{:?}", params, lbfgs_direction);
        // Verify that L-BFGS direction is a descent direction
        let direction_dot_gradient = dot_product(&gradients, &lbfgs_direction)?;
        if direction_dot_gradient >= 0.0 {
            warn!("L-BFGS direction is not a descent direction (dot product = {:.6e}), falling back to steepest descent", direction_dot_gradient);
            let result = self.steepest_descent_step(
                params,
                &gradients,
                function.clone(),
                "L-BFGS direction not descent",
            )?;
            self.state.iteration += 1;
            return Ok(result);
        }
        
        let quadratic_path =
            self.create_quadratic_path(params, &gradients, &lbfgs_direction, function.clone())?;
        let line_search_result = self.find_optimal_t_line_search(quadratic_path.clone());
        if line_search_result.is_err() {
            warn!("Line search failed: {}", line_search_result.as_ref().err().unwrap());
            let result = self.steepest_descent_step(
                params,
                &gradients,
                function.clone(),
                "line search failure",
            )?;
            self.state.iteration += 1;
            return Ok(result);
        }
        let line_search_result = line_search_result?;
        // If line search returned step_size = 0, fall back to steepest descent
        if line_search_result.step_size == 0.0 && !line_search_result.success {
            debug!("Line search indicated invalid direction, falling back to steepest descent");
            let result = self.steepest_descent_step(
                params,
                &gradients,
                function.clone(),
                "invalid quadratic path direction",
            )?;
            self.state.iteration += 1;
            return Ok(result);
        }
        // If line search returned very small step size, check if we're at a local minimum
        if line_search_result.step_size < 1e-10 {
            debug!("Line search returned very small step size {:.3e}, checking convergence", line_search_result.step_size);
            if grad_norm < 1e-3 {
                info!("Converged with small gradient norm {:.3e}", grad_norm);
                self.state.iteration += 1;
                return Ok(StepResult {
                    step_size: line_search_result.step_size,
                    convergence_info: ConvergenceInfo {
                        converged: true,
                        function_change: Some(0.0),
                    },
                    metadata: OptimizationMetadata::default(),
                });
            }
        }
        
        info!("Found optimal t = {:.3e}", line_search_result.step_size);
        self.log_scalar("Optimal t", line_search_result.step_size);
        self.log_line_search_details(line_search_result.step_size);
        let position = quadratic_path.evaluate(line_search_result.step_size)?;
        

        self.log_tensor_data("Final position", &position);
        let old_params = params.to_vec();
        for (param, x) in params.iter_mut().zip(position.iter()) {
            *param = x.clone();
        }
        // Calculate function decrease before L-BFGS update
        let final_function_value = function.evaluate(params)?;
        debug!("Final function value: {:.6e}", final_function_value);
        let function_decrease = initial_function_value - final_function_value;
        
        debug!("Updating L-BFGS history");
        let old_params_before_update = old_params.clone();
        // Update L-BFGS state with the new position and gradient
        let new_gradient = function.gradient(params)?;
        // Only update if we made meaningful progress
        if line_search_result.step_size > 1e-10 && function_decrease > 1e-12 {
            self.state.lbfgs_state.update(&old_params_before_update, params, &new_gradient)?;
        } else {
            debug!("Insufficient progress for L-BFGS update: step_size={:.3e}, function_decrease={:.3e}", 
                   line_search_result.step_size, function_decrease);
        }

        // FATAL ERROR CHECK: Verify that the step decreased the function value
        if final_function_value > initial_function_value {
            let increase = final_function_value - initial_function_value;
            error!(
                "FATAL ERROR: QQN step increased function value by {:.6e} (from {:.6e} to {:.6e}). This should never happen!",
                increase, initial_function_value, final_function_value
            );
            return Err(Error::Msg(format!(
                "FATAL ERROR: QQN step increased function value by {:.6e} (from {:.6e} to {:.6e}). This violates the descent property and should never happen.",
                increase, initial_function_value, final_function_value
            )));
        }

        debug!("Function decreased by: {:.6e}", function_decrease);
        self.log_scalar("Function Decrease", function_decrease);

        // Check for NaN/Inf in updated parameters
        for (i, param) in params.iter().enumerate() {
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                warn!("Non-finite parameter detected at index {} after update", i);
                return Err(Error::Msg(
                    "Non-finite parameter detected after update".into(),
                ));
            }
            // Also check for extremely large values
            if param_vec.iter().any(|&x| x.abs() > 1e10) {
                warn!(
                    "Extremely large parameter detected at index {} after update",
                    i
                );
                return Err(Error::Msg("Parameter values too large after update".into()));
            }
        }

        // Increment iteration counter AFTER all operations complete successfully
        self.state.iteration += 1;
        info!(
            "QQN step {} completed successfully",
            self.state.iteration - 1
        );
        // Update trace if enabled
        if let Some(trace) = &mut self.trace {
            let step_time = step_start.elapsed().as_secs_f64();
            trace.add_entry(
                1.0, // magnitude ratio (not computed in this implementation)
                true, // used quadratic path
                line_search_result.step_size,
                grad_norm,
                compute_magnitude(&lbfgs_direction).unwrap_or(0.0),
                0.0, // descent dot product (not computed)
                final_function_value,
                step_time,
            );
        }

        // 7. Create convergence info
        let convergence_info = ConvergenceInfo {
            converged: false, // QQN does not have a convergence criterion like L-BFGS
            function_change: Some(function_decrease),
        };

        let mut metadata = OptimizationMetadata::default();
        metadata.optimizer_data.insert("method".to_string(), 1.0); // 1 = QQN with L-BFGS
        metadata
            .optimizer_data
            .insert("gradient_norm".to_string(), grad_norm);
        metadata
            .optimizer_data
            .insert("optimal_t".to_string(), line_search_result.step_size);
        metadata
            .optimizer_data
            .insert("function_decrease".to_string(), function_decrease);
        metadata
            .optimizer_data
            .insert("initial_function_value".to_string(), initial_function_value);
        metadata
            .optimizer_data
            .insert("final_function_value".to_string(), final_function_value);

        Ok(StepResult {
            step_size: line_search_result.step_size,
            convergence_info,
            metadata,
        })
    }

    fn reset(&mut self) {
        info!("Resetting QQN optimizer state");
        self.state = QQNState::new(self.config.lbfgs_history);
        self.state.lbfgs_state.reset();
    }

    fn name(&self) -> &str {
        "QQN"
    }
    fn iteration(&self) -> usize {
        self.state.iteration
    }
}
/// Wrapper to make DifferentiableFunction compatible with Arc<dyn ... + Send + Sync>
// Remove the FunctionWrapper struct entirely since we'll change the approach

/// Represents a quadratic interpolation path between two search directions
#[derive(Clone)]
pub struct QuadraticPath {
    start_point: Vec<Tensor>,
    negative_gradient: Vec<Tensor>,
    lbfgs_direction: Vec<Tensor>,
    position_cache: Arc<Mutex<HashMap<OrderedFloat<f64>, Vec<f64>>>>,
    gradient_cache: Arc<Mutex<HashMap<OrderedFloat<f64>, Vec<f64>>>>,
    lbfgs_state: Arc<Mutex<LBFGSState>>,
    function: Arc<dyn DifferentiableFunction + Send + Sync>,
    cache_hits: Arc<AtomicUsize>,
    cache_misses: Arc<AtomicUsize>,
}

impl std::fmt::Debug for QuadraticPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuadraticPath")
            .field("start_point", &self.start_point)
            .field("negative_gradient", &self.negative_gradient)
            .field("lbfgs_direction", &self.lbfgs_direction)
            .field("position_cache", &"<cached positions>")
            .field("gradient_cache", &"<cached gradients>")
            .field("lbfgs_state", &"<lbfgs state>")
            .field("function", &"<function>")
            .finish()
    }
}

impl QuadraticPath {
    /// Create a new quadratic path
    pub fn new(
        start_point: Vec<Tensor>,
        negative_gradient: Vec<Tensor>,
        lbfgs_direction: Vec<Tensor>,
        lbfgs_state: Arc<Mutex<LBFGSState>>,
        function: Arc<dyn DifferentiableFunction + Send + Sync>,
    ) -> Self {
        let start_point = start_point
            .iter()
            .map(|t| t.clone().to_device(&Device::Cpu).unwrap())
            .collect::<Vec<_>>();
        Self {
            start_point,
            negative_gradient,
            lbfgs_direction,
            position_cache: Arc::new(Mutex::new(HashMap::new())),
            gradient_cache: Arc::new(Mutex::new(HashMap::new())),
            lbfgs_state,
            function,
            cache_hits: Arc::new(AtomicUsize::new(0)),
            cache_misses: Arc::new(AtomicUsize::new(0)),
        }
    }
    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (
            self.cache_hits.load(Ordering::Relaxed),
            self.cache_misses.load(Ordering::Relaxed),
        )
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
            return Ok(self
                .negative_gradient
                .iter()
                .map(|t| t.zeros_like())
                .collect::<Result<Vec<_>, _>>()?);
        }

        let gradient_term = scale_tensors(&self.negative_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;
        // Log intermediate terms for debugging
        trace!(
            "QuadraticPath::evaluate_direction: gradient_term magnitude={:.3e}, lbfgs_term magnitude={:.3e}",
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

    /// Check if we have both position and gradient cached for the same t, and update L-BFGS if so
    fn maybe_update_lbfgs(&self, t: f64) -> CandleResult<()> {
        let key = OrderedFloat(t);
        let position_cache = self.position_cache.lock().unwrap();
        let gradient_cache = self.gradient_cache.lock().unwrap();
        if let (Some(position_f64), Some(gradient_f64)) = (position_cache.get(&key), gradient_cache.get(&key))
        {
            // We have both position and gradient for this t, update L-BFGS
            trace!("Updating L-BFGS state for t={}", t);
            // Convert f64 vectors back to tensors
            let device = self.start_point[0].device();
            let mut position_tensors = Vec::new();
            let mut gradient_tensors = Vec::new();
            // Reconstruct tensors from cached f64 values
            let mut pos_idx = 0;
            let mut grad_idx = 0;
            for (start_tensor, _) in self.start_point.iter().zip(self.negative_gradient.iter()) {
                let shape = start_tensor.shape();
                let size = shape.elem_count();
                // Extract position slice
                let pos_slice = &position_f64[pos_idx..pos_idx + size];
                let pos_tensor = Tensor::from_slice(pos_slice, shape.dims(), device)?;
                position_tensors.push(pos_tensor);
                pos_idx += size;
                // Extract gradient slice
                let grad_slice = &gradient_f64[grad_idx..grad_idx + size];
                let grad_tensor = Tensor::from_slice(grad_slice, shape.dims(), device)?;
                gradient_tensors.push(grad_tensor);
                grad_idx += size;
            }
            // Update L-BFGS state
            if let Ok(mut lbfgs_state) = self.lbfgs_state.try_lock() {
                if let Err(e) =
                    lbfgs_state.update(&self.start_point, &position_tensors, &gradient_tensors)
                {
                    warn!("Failed to update L-BFGS state: {}", e);
                }
            }
        }
        Ok(())
    }
}
impl<'a> ParametricCurve for QuadraticPath {
    fn position(&self, t: f64) -> AnyhowResult<Vec<f64>> {
        let key = OrderedFloat(t);
        // Check cache first
        {
            let cache = self.position_cache.lock().unwrap();
            if let Some(cached_position) = cache.get(&key) {
                trace!("Using cached position for t={}", t);
                self.cache_hits.fetch_add(1, Ordering::Relaxed);
                return Ok(cached_position.clone());
            }
        }
        self.cache_misses.fetch_add(1, Ordering::Relaxed);

        // Get the point at parameter t
        let point = self.evaluate(t)?;
        // Convert point tensors to f64
        let position_f64: Vec<f64> = point
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();

        // Cache the result
        {
            let mut cache = self.position_cache.lock().unwrap();
            cache.insert(key, position_f64.clone());
        }

        // Check if we can update L-BFGS
        if let Err(e) = self.maybe_update_lbfgs(t) {
            warn!("Failed to update L-BFGS in position evaluation: {}", e);
        }

        Ok(position_f64)
    }

    fn direction(&self, t: f64) -> AnyhowResult<Vec<f64>> {
        let key = OrderedFloat(t);
        // Check cache first
        {
            let cache = self.gradient_cache.lock().unwrap();
            if let Some(cached_gradient) = cache.get(&key) {
                trace!("Using cached gradient for t={}", t);
                self.cache_hits.fetch_add(1, Ordering::Relaxed);
                return Ok(cached_gradient.clone());
            }
        }
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
        
        // Evaluate function at this position to get gradient
        let position = self.position(t)?; // This will use cache if available
        // Convert position back to tensors for gradient evaluation
        let device = self.start_point[0].device();
        let mut position_tensors = Vec::new();
        let mut idx = 0;
        for start_tensor in &self.start_point {
            let shape = start_tensor.shape();
            let size = shape.elem_count();
            let slice = &position[idx..idx + size];
            let tensor = Tensor::from_slice(slice, shape.dims(), device)
                .map_err(|e| anyhow!("Failed to create tensor from position: {}", e))?;
            position_tensors.push(tensor);
            idx += size;
        }
        // Evaluate gradient at this position
        let gradients = self
            .function
            .gradient(&position_tensors)
            .map_err(|e| anyhow!("Failed to evaluate gradient: {}", e))?;

        // Convert to f64 vector
        let gradient_f64: Vec<f64> = gradients
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();

        // Cache the result
        {
            let mut cache = self.gradient_cache.lock().unwrap();
            cache.insert(key, gradient_f64.clone());
        }

        // Check if we can update L-BFGS
        if let Err(e) = self.maybe_update_lbfgs(t) {
            warn!("Failed to update L-BFGS in gradient evaluation: {}", e);
        }

        Ok(gradient_f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let function = Arc::new(QuadraticFunction::new());
        let lbfgs_state = Arc::new(Mutex::new(LBFGSState::new_with_options(10, 1e-8, true)));
        let path = QuadraticPath::new(
            start_point,
            negative_gradient,
            lbfgs_dir,
            lbfgs_state,
            function,
        );

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
        let function = Arc::new(QuadraticFunction::new());
        let lbfgs_state = Arc::new(Mutex::new(LBFGSState::new_with_options(10, 1e-8, true)));
        let path = QuadraticPath::new(
            start_point,
            negative_gradient,
            lbfgs_dir,
            lbfgs_state,
            function,
        );

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
        //init_logging().unwrap();
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        config.min_lbfgs_iterations = 0; // Enable L-BFGS immediately
        let mut optimizer = QQNOptimizer::new(config);
        // Start at (2, 3)
        let mut params = vec![Tensor::from_slice(&[2.0, 3.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction::new());
        // Take a step
        let _result = optimizer.step(&mut params, function)?;
        // Should move towards origin
        let values = params[0].to_vec1::<f64>()?;
        assert!(values[0].abs() < 2.0);
        assert!(values[1].abs() < 3.0);
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
        let function = Arc::new(QuadraticFunction::new());
        // First step should use steepest descent
        let result = optimizer.step(&mut params, function)?;
        // Check metadata indicates steepest descent was used
        assert_eq!(result.metadata.optimizer_data.get("method"), Some(&0.0));
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
        let function = Arc::new(QuadraticFunction::new());
        let _result = optimizer.step(&mut params, function)?;
        // Should move towards origin
        let values = params[0].to_vec1::<f64>()?;
        assert!(values[0].abs() < 2.0);
        assert!(values[1].abs() < 3.0);
        Ok(())
    }
    #[test]
    fn test_qqn_reset() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = QQNConfig::default();
        config.verbose = false;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction::new());
        // Take some steps
        for _ in 0..3 {
            optimizer.step(&mut params, function.clone())?;
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

        let function = Arc::new(NaNGradientFunction);
        let result = optimizer.step(&mut params, function);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Non-finite gradient"));
        Ok(())
    }
    #[test]
    fn test_qqn_handles_empty_parameters() -> CandleResult<()> {
        let mut config = QQNConfig::default();
        config.verbose = false;
        let mut optimizer = QQNOptimizer::new(config);
        let mut params: Vec<Tensor> = vec![];
        let function = Arc::new(QuadraticFunction::new());
        let result = optimizer.step(&mut params, function);
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
        let function = Arc::new(QuadraticFunction::new());
        // Take multiple steps
        for _ in 0..20 {
            let _ = optimizer.step(&mut params, function.clone())?;
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
        let function = Arc::new(RosenbrockFunction);
        // Take several steps
        for i in 0..10 {
            let _ = optimizer.step(&mut params, function.clone())?;
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
        let function = Arc::new(QuadraticFunction::new());
        let lbfgs_state = Arc::new(Mutex::new(LBFGSState::new_with_options(10, 1e-8, true)));
        let path = QuadraticPath::new(start, neg_grad, lbfgs_dir, lbfgs_state, function);
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