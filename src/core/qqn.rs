use crate::core::lbfgs::LBFGSState;
use crate::core::line_search::{LineSearch, ParametricCurve, StrongWolfeLineSearch};
use crate::core::optimizer::OptimizationMetadata;
use crate::core::ConvergenceInfo;
use crate::core::Optimizer;
use crate::core::StepResult;
use crate::utils::math::{combine_tensors, compute_magnitude, scale_tensors};
use anyhow::Result as AnyhowResult;
use candle_core::{Result as CandleResult, Tensor};
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
    /// Line search configuration
    pub line_search: crate::core::line_search::StrongWolfeConfig,
    /// Numerical stability constant
    pub epsilon: f64,
    /// Enable verbose logging of tensor data and internal state
    pub verbose: bool,
}

impl Default for QQNConfig {
    fn default() -> Self {
        Self {
            lbfgs_history: 10,
            line_search: crate::core::line_search::StrongWolfeConfig::default(),
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
            lbfgs_state: LBFGSState::new(lbfgs_history),
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
            "Creating QQN optimizer with config: lbfgs_history={}, epsilon={}, verbose={}",
            config.lbfgs_history, config.epsilon, config.verbose
        );
        let line_search = Box::new(StrongWolfeLineSearch::new(config.line_search.clone()));
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
        debug!("=== {} ===", name);
        for (i, tensor) in tensors.iter().enumerate() {
            match tensor.flatten_all().and_then(|t| t.to_vec1::<f64>()) {
                Ok(values) => {
                    debug!(
                        "  Tensor[{}]: shape={:?}, values={:?}",
                        i,
                        tensor.shape(),
                        values
                    );
                    if values.len() <= 10 {
                        debug!("    Full data: {:?}", values);
                    } else {
                        debug!(
                            "    First 5: {:?}, Last 5: {:?}",
                            &values[..5],
                            &values[values.len() - 5..]
                        );
                    }
                    // Log statistics
                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
                    let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                    debug!("    Stats: mean={:.6e}, std={:.6e}, min={:.6e}, max={:.6e}",
                          mean, variance.sqrt(), min_val, max_val);
                }
                Err(e) => {
                    debug!(
                        "  Tensor[{}]: shape={:?}, error reading values: {}",
                        i,
                        tensor.shape(),
                        e
                    );
                }
            }
        }
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
        gradient: &[Tensor],
        lbfgs_direction: &[Tensor],
    ) -> CandleResult<QuadraticPath> {
        debug!("Creating quadratic path between gradient and L-BFGS direction");
        // Log input tensors in verbose mode
        self.log_tensor_data("Input Gradient", gradient);
        self.log_tensor_data("Input L-BFGS Direction", lbfgs_direction);

        // Validate inputs
        if gradient.is_empty() || lbfgs_direction.is_empty() {
            warn!("Empty gradient or direction vectors provided to create_quadratic_path");
            return Err(candle_core::Error::Msg(
                "Empty gradient or direction vectors".into(),
            ));
        }
        if gradient.len() != lbfgs_direction.len() {
            warn!(
                "Dimension mismatch in create_quadratic_path: gradient={}, direction={}",
                gradient.len(),
                lbfgs_direction.len()
            );
            return Err(candle_core::Error::Msg(format!(
                "Gradient and direction dimension mismatch: {} vs {}",
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
            negative_gradient,
            lbfgs_direction.to_vec(),
        ))
    }

    /// Find optimal t parameter for the quadratic path using line search
    fn find_optimal_t_line_search(
        &self,
        params: &[Tensor],
        quadratic_path: &QuadraticPath,
        gradients: &[Tensor],
        function: &dyn crate::core::optimizer::DifferentiableFunction,
    ) -> CandleResult<(f64, usize, usize)> {
        debug!("Starting line search for optimal t along quadratic path");

        // Convert current parameters and gradients to f64 vectors
        let current_point_f64: Vec<f64> = params
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();
        let current_gradient_f64: Vec<f64> = gradients
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();

        // Get current function value
        let current_value = function.evaluate(params)?;

        // Create a parametric curve adapter for the quadratic path
        let curve_adapter =
            QuadraticCurveAdapter::new(current_point_f64.clone(), quadratic_path.clone());

        // Create objective and gradient functions that work with f64 vectors
        let objective_fn = |x: &[f64]| -> AnyhowResult<f64> {
            // Convert f64 vector back to tensors
            let tensors = self.f64_to_tensors(x, params)?;
            Ok(function.evaluate(&tensors)?)
        };

        let gradient_fn = |x: &[f64]| -> AnyhowResult<Vec<f64>> {
            // Convert f64 vector back to tensors
            let tensors = self.f64_to_tensors(x, params)?;
            let grad_tensors = function.gradient(&tensors)?;
            // Convert gradients back to f64
            Ok(grad_tensors
                .iter()
                .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
                .collect())
        };

        // Perform line search along the curve
        let mut line_search = self.line_search.clone_box();
        let result = line_search
            .search_along_curve(
                &curve_adapter,
                current_value,
                &current_gradient_f64,
                &objective_fn,
                &gradient_fn,
            )
            .map_err(|e| candle_core::Error::Msg(e.to_string()))?;

        debug!(
            "Line search completed: t*={:.6}, success={}",
            result.step_size, result.success
        );

        Ok((
            result.step_size,
            result.function_evaluations,
            result.gradient_evaluations,
        ))
    }

    /// Helper function to convert f64 vector to tensors
    fn f64_to_tensors(&self, values: &[f64], template: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        let mut offset = 0;
        let mut result = Vec::new();

        for t in template {
            let shape = t.shape();
            let numel = shape.elem_count();
            let slice = &values[offset..offset + numel];
            let tensor = Tensor::from_slice(slice, shape, t.device())?;
            result.push(tensor);
            offset += numel;
        }

        Ok(result)
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
        function: &dyn crate::core::optimizer::DifferentiableFunction,
    ) -> CandleResult<StepResult> {
        info!(
            "QQN step {}: starting optimization step",
            self.state.iteration
        );
        self.log_optimization_state(self.state.iteration, "Starting step");

        // Log initial state in verbose mode
        self.log_tensor_data("Initial Parameters", params);

        // Compute gradients at current parameters
        let gradients = function.gradient(params)?;
        // Log computed gradients in verbose mode
        self.log_tensor_data("Computed Gradients", &gradients);

        // Input validation
        if params.is_empty() || gradients.is_empty() {
            warn!("Empty parameters or gradients provided to QQN step");
            return Err(candle_core::Error::Msg(
                "Empty parameters or gradients".into(),
            ));
        }
        if params.len() != gradients.len() {
            warn!(
                "Parameter/gradient dimension mismatch: {} vs {}",
                params.len(),
                gradients.len()
            );
            return Err(candle_core::Error::Msg(format!(
                "Parameter and gradient dimension mismatch: {} vs {}",
                params.len(),
                gradients.len()
            )));
        }
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
        // Compute gradient norm for logging
        let grad_norm = compute_magnitude(&gradients)?;
        debug!("Gradient norm: {:.6e}", grad_norm);
        self.log_scalar("Gradient Norm", grad_norm);

        // 1. Compute L-BFGS direction
        debug!("Computing L-BFGS direction");
        let lbfgs_direction = self.state.lbfgs_state.compute_direction(&gradients)?;
        // Log L-BFGS direction in verbose mode
        self.log_tensor_data("L-BFGS Direction", &lbfgs_direction);

        // Check if L-BFGS direction is valid (i.e., all finite)
        let lbfgs_valid = lbfgs_direction.iter().all(|d| {
            d.flatten_all()
                .and_then(|f| f.to_vec1::<f64>())
                .map(|v| v.iter().all(|&x| x.is_finite()))
                .unwrap_or(false)
        });

        if !lbfgs_valid {
            warn!("Invalid L-BFGS direction detected, falling back to gradient descent");
            let direction = scale_tensors(&gradients, -1.0)?;
            let step_size = 0.01;
            debug!("Using gradient descent with step size {}", step_size);

            for (param, dir) in params.iter_mut().zip(direction.iter()) {
                let step_size_tensor = Tensor::new(step_size, param.device())?;
                let update = dir.broadcast_mul(&step_size_tensor)?;
                *param = param.add(&update)?;
            }
            // Update L-BFGS state with the actual step taken
            self.state.lbfgs_state.update(
                &gradients,
                &scale_tensors(&direction, step_size)?,
                1.0,
            )?;

            self.state.iteration += 1;
            let convergence_info = ConvergenceInfo {
                converged: false,
                function_change: None,
                convergence_criterion: None,
            };
            let mut metadata = OptimizationMetadata::default();
            metadata
                .optimizer_data
                .insert("invalid_magnitude_ratio".to_string(), 1.0);
            return Ok(StepResult {
                step_size,
                function_evaluations: 0,
                gradient_evaluations: 0,
                convergence_info,
                metadata,
            });
        }

        let quadratic_path = self.create_quadratic_path(&gradients, &lbfgs_direction)?;
        let (optimal_t, f_evals, g_evals) =
            self.find_optimal_t_line_search(params, &quadratic_path, &gradients, function)?;
        info!("Found optimal t = {:.6}", optimal_t);
        self.log_scalar("Optimal t", optimal_t);
        self.log_line_search_details(optimal_t, f_evals, g_evals);

        let direction = quadratic_path.evaluate(optimal_t)?;
        // Log final direction in verbose mode
        self.log_tensor_data("Final Direction", &direction);

        let direction_norm = compute_magnitude(&direction)?;
        let dot_product = crate::utils::math::dot_product(&gradients, &direction)?;
        debug!(
            "Final direction: ||d||={:.6e}, g^T d={:.6e}",
            direction_norm, dot_product
        );
        self.log_scalar("Direction Norm", direction_norm);
        self.log_scalar("Descent Dot Product", dot_product);
        if dot_product >= 0.0 {
            warn!(
                "Non-descent direction detected! g^T d = {:.6e} >= 0",
                dot_product
            );
        }

        // Apply the step
        trace!("Applying step to parameters");
        let step_norm = compute_magnitude(&direction)?;
        debug!("Step norm before application: {:.6e}", step_norm);
        self.log_scalar("Step Norm", step_norm);

        // Safety check: if step is too large, scale it down
        let max_step_norm = 10.0; // Conservative maximum step size
        let scaling_factor = if step_norm > max_step_norm {
            warn!(
                "Step norm {:.6e} exceeds maximum {:.6e}, scaling down",
                step_norm, max_step_norm
            );
            self.log_scalar("Scaling Factor", max_step_norm / step_norm);
            max_step_norm / step_norm
        } else {
            self.log_scalar("Scaling Factor", 1.0);
            1.0
        };

        for (param, dir) in params.iter_mut().zip(direction.iter()) {
            if scaling_factor < 1.0 {
                let scaled_dir = dir.affine(scaling_factor, 0.0)?;
                *param = param.add(&scaled_dir)?;
            } else {
                *param = param.add(dir)?;
            }
        }
        // Log updated parameters in verbose mode
        self.log_tensor_data("Updated Parameters", params);

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
        self.state.lbfgs_state.update(
            &gradients, // Use gradients at old parameters
            &direction, 1.0, // We've already applied the full direction
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
            convergence_criterion: None,
        };

        let mut metadata = OptimizationMetadata::default();
        metadata
            .optimizer_data
            .insert("gradient_norm".to_string(), grad_norm);
        metadata
            .optimizer_data
            .insert("direction_norm".to_string(), compute_magnitude(&direction)?);
        metadata
            .optimizer_data
            .insert("optimal_t".to_string(), optimal_t);
        metadata
            .optimizer_data
            .insert("dot_product".to_string(), dot_product);

        Ok(StepResult {
            step_size: optimal_t, // This is the t value used for the step
            function_evaluations: f_evals,
            gradient_evaluations: g_evals,
            convergence_info,
            metadata,
        })
    }
    fn step_with_gradients(
        &mut self,
        params: &mut [Tensor],
        gradients: &[Tensor],
    ) -> CandleResult<StepResult> {
        // Create a thread-safe function wrapper that uses the provided gradients
        let gradients_clone = gradients.to_vec();

        let function = crate::core::optimizer::SeparateFunctions::new(
            move |_params: &[Tensor]| -> CandleResult<f64> {
                // Since we have pre-computed gradients, return a dummy value
                // The actual objective evaluation should be done externally if needed
                Ok(0.0)
            },
            move |_: &[Tensor]| Ok(gradients_clone.clone()),
        );
        self.step(params, &function)
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
    negative_gradient: Vec<Tensor>,
    lbfgs_direction: Vec<Tensor>,
}

impl QuadraticPath {
    /// Create a new quadratic path
    pub fn new(negative_gradient: Vec<Tensor>, lbfgs_direction: Vec<Tensor>) -> Self {
        Self {
            negative_gradient,
            lbfgs_direction,
        }
    }

    /// Evaluate the quadratic path at parameter t ∈ [0, 1]
    ///
    /// d(t) = t(1-t) * (-g) + t² * d_lbfgs
    pub fn evaluate(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        // Clamp t to valid range
        let t_clamped = t.max(0.0).min(1.0);
        if (t - t_clamped).abs() > 1e-10 {
            trace!(
                "QuadraticPath::evaluate: clamped t from {} to {}",
                t,
                t_clamped
            );
        }
        let t = t_clamped;

        // Coefficients for the quadratic path formula as per paper
        let gradient_coeff = t * (1.0 - t);
        let lbfgs_coeff = t * t;
        trace!(
            "QuadraticPath::evaluate(t={}): gradient_coeff={}, lbfgs_coeff={}",
            t,
            gradient_coeff,
            lbfgs_coeff
        );

        let gradient_term = scale_tensors(&self.negative_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;
        // Log intermediate terms for debugging
        trace!(
            "QuadraticPath::evaluate: gradient_term magnitude={:.6e}, lbfgs_term magnitude={:.6e}",
            compute_magnitude(&gradient_term).unwrap_or(0.0),
            compute_magnitude(&lbfgs_term).unwrap_or(0.0)
        );

        combine_tensors(&gradient_term, &lbfgs_term)
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
/// Adapter to use QuadraticPath with the ParametricCurve interface
#[derive(Debug, Clone)]
struct QuadraticCurveAdapter {
    start_point: Vec<f64>,
    quadratic_path: QuadraticPath,
}
impl QuadraticCurveAdapter {
    fn new(start_point: Vec<f64>, quadratic_path: QuadraticPath) -> Self {
        Self {
            start_point,
            quadratic_path,
        }
    }
}
impl ParametricCurve for QuadraticCurveAdapter {
    fn evaluate(&self, t: f64) -> AnyhowResult<Vec<f64>> {
        // Get the direction at parameter t
        let direction = self.quadratic_path.evaluate(t)?;
        // Convert direction tensors to f64 and add to start point
        let mut result = self.start_point.clone();
        let mut offset = 0;
        for dir_tensor in &direction {
            let dir_vec = dir_tensor.flatten_all()?.to_vec1::<f64>()?;
            for (i, &val) in dir_vec.iter().enumerate() {
                result[offset + i] += val;
            }
            offset += dir_vec.len();
        }
        Ok(result)
    }
    fn derivative(&self, t: f64) -> AnyhowResult<Vec<f64>> {
        // Get the derivative of the quadratic path
        let deriv = self.quadratic_path.derivative(t)?;
        // Convert to f64 vector
        Ok(deriv
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect())
    }
    fn initial_derivative(&self) -> AnyhowResult<Vec<f64>> {
        self.derivative(0.0)
    }
    fn clone_box(&self) -> Box<dyn ParametricCurve> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use candle_core::Device;

    #[test]
    fn test_quadratic_path_evaluation() -> CandleResult<()> {
        let device = Device::Cpu;
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[2], &device)?];

        // Create negative gradient as per paper formula
        let negative_gradient = vec![Tensor::from_slice(&[-1.0, 0.0], &[2], &device)?];
        let path = QuadraticPath::new(negative_gradient, lbfgs_dir);

        // At t=0, should be zero vector
        let result_0 = path.evaluate(0.0)?;
        let values_0 = result_0[0].to_vec1::<f64>()?;
        assert_relative_eq!(values_0[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(values_0[1], 0.0, epsilon = 1e-10);

        // At t=1, should be L-BFGS direction
        let result_1 = path.evaluate(1.0)?;
        let values_1 = result_1[0].to_vec1::<f64>()?;
        assert_relative_eq!(values_1[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(values_1[1], 1.0, epsilon = 1e-10);

        // At t=0.5, should be 0.5*(1-0.5)*(-g) + 0.5²*d_lbfgs = 0.25*(-g) + 0.25*d_lbfgs
        let result_half = path.evaluate(0.5)?;
        let values_half = result_half[0].to_vec1::<f64>()?;
        assert_relative_eq!(values_half[0], -0.25, epsilon = 1e-10); // 0.25 * (-1.0)
        assert_relative_eq!(values_half[1], 0.25, epsilon = 1e-10); // 0.25 * 1.0

        Ok(())
    }

    #[test]
    fn test_quadratic_path_derivative() -> CandleResult<()> {
        let device = Device::Cpu;
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[2], &device)?];

        // Create negative gradient as per paper formula
        let negative_gradient = vec![Tensor::from_slice(&[-1.0, 0.0], &[2], &device)?];
        let path = QuadraticPath::new(negative_gradient, lbfgs_dir);

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
}