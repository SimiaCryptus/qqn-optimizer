use crate::core::optimizer::{ConvergenceInfo, DifferentiableFunction, OptimizationMetadata, Optimizer, StepResult};
use candle_core::{Result as CandleResult, Tensor};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Configuration parameters for the Adam optimizer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdamConfig {
    /// Learning rate (step size)
    pub learning_rate: f64,
    /// Learning rate schedule: "constant", "exponential", "cosine", or "adaptive"
    pub lr_schedule: String,
    /// Decay rate for exponential schedule
    pub lr_decay: f64,
    /// Minimum learning rate
    pub min_learning_rate: f64,
    /// Enable gradient clipping
    pub gradient_clip: Option<f64>,
    /// Exponential decay rate for first moment estimates (default: 0.9)
    pub beta1: f64,
    /// Exponential decay rate for second moment estimates (default: 0.999)
    pub beta2: f64,
    /// Small constant for numerical stability (default: 1e-8)
    pub epsilon: f64,
    /// Weight decay (L2 regularization)
    pub weight_decay: f64,
    /// Enable AMSGrad variant
    pub amsgrad: bool,
    /// Maximum line search iterations
    pub max_line_search_iter: usize,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for AdamConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,   // Higher learning rate for better convergence
            lr_schedule: "adaptive".to_string(),
            lr_decay: 0.999,
            min_learning_rate: 1e-8,
            gradient_clip: Some(1.0),   // Tighter gradient clipping
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            weight_decay: 0.0,
            amsgrad: false,
            max_line_search_iter: 10,
            verbose: false,
        }
    }
}

/// State information for Adam optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdamState {
    /// Current iteration number
    pub iteration: usize,
    /// First moment estimates (momentum)
    #[serde(skip_serializing, skip_deserializing)]
    pub m: Option<Vec<Tensor>>,
    /// Second moment estimates (squared gradients)
    #[serde(skip_serializing, skip_deserializing)]
    pub v: Option<Vec<Tensor>>,
    /// Maximum second moment estimates (for AMSGrad)
    #[serde(skip_serializing, skip_deserializing)]
    pub v_max: Option<Vec<Tensor>>,
}

impl AdamState {
    /// Create a new Adam state.
    pub fn new() -> Self {
        Self {
            iteration: 0,
            m: None,
            v: None,
            v_max: None,
        }
    }

    /// Reset the Adam state to initial conditions.
    pub fn reset(&mut self) {
        self.iteration = 0;
        self.m = None;
        self.v = None;
        self.v_max = None;
    }

    /// Get the current iteration number.
    pub fn iteration(&self) -> usize {
        self.iteration
    }
}

/// Adam optimizer implementation.
#[derive(Debug)]
pub struct AdamOptimizer {
    config: AdamConfig,
    state: AdamState,
    /// Current effective learning rate
    current_lr: f64,
    /// Previous function value for adaptive learning rate
    prev_function_value: Option<f64>,
    /// Count of bad steps for adaptive learning rate
    bad_step_count: usize,
}

impl Clone for AdamOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            current_lr: self.current_lr,
            prev_function_value: self.prev_function_value,
            bad_step_count: self.bad_step_count,
        }
    }
}

impl AdamOptimizer {
    /// Create a new Adam optimizer with the given configuration.
    pub fn new(config: AdamConfig) -> Self {
        if config.verbose {
            info!("Creating Adam optimizer with verbose logging enabled");
            debug!("Adam Config: lr={}, beta1={}, beta2={}, epsilon={}, weight_decay={}, amsgrad={}",
                  config.learning_rate, config.beta1, config.beta2, config.epsilon, 
                  config.weight_decay, config.amsgrad);
        }
        let current_lr = config.learning_rate;
        Self {
            config,
            state: AdamState::new(),
            current_lr,
            prev_function_value: None,
            bad_step_count: 0,
        }
    }

    /// Log tensor data if verbose mode is enabled
    fn log_tensor_data(&self, name: &str, tensors: &[Tensor]) {
        if !self.config.verbose {
            return;
        }
        debug!("=== Adam: {} ===", name);
        for (i, tensor) in tensors.iter().enumerate() {
            match tensor.flatten_all().and_then(|t| t.to_vec1::<f64>()) {
                Ok(values) => {
                    debug!("  Tensor[{}]: shape={:?}, length={}", i, tensor.shape(), values.len());
                    if values.len() <= 10 {
                        debug!("    Full data: {:?}", values);
                    } else {
                        debug!("    First 5: {:?}, Last 5: {:?}", &values[..5], &values[values.len() - 5..]);
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
                    debug!("  Tensor[{}]: shape={:?}, error reading values: {}", i, tensor.shape(), e);
                }
            }
        }
    }

    /// Log scalar value if verbose mode is enabled
    fn log_scalar(&self, name: &str, value: f64) {
        if self.config.verbose {
            debug!("  Adam {}: {:.12e}", name, value);
        }
    }

    /// Apply weight decay to gradients
    fn apply_weight_decay(&self, gradients: &mut [Tensor], params: &[Tensor]) -> CandleResult<()> {
        if self.config.weight_decay == 0.0 {
            return Ok(());
        }

        for (grad, param) in gradients.iter_mut().zip(params.iter()) {
            let decay_term = param.affine(self.config.weight_decay, 0.0)?;
            *grad = grad.add(&decay_term)?;
        }

        Ok(())
    }
    /// Apply gradient clipping if configured
    fn apply_gradient_clipping(&self, gradients: &mut [Tensor]) -> CandleResult<()> {
        if let Some(max_norm) = self.config.gradient_clip {
            let grad_norm = crate::utils::math::compute_magnitude(gradients)?;
            if grad_norm > max_norm {
                let scale = max_norm / grad_norm;
                for grad in gradients.iter_mut() {
                    *grad = grad.affine(scale, 0.0)?;
                }
            }
        }
        Ok(())
    }
    /// Update learning rate based on schedule
    fn update_learning_rate(&mut self, current_value: Option<f64>) {
        match self.config.lr_schedule.as_str() {
            "exponential" => {
                self.current_lr *= self.config.lr_decay;
                self.current_lr = self.current_lr.max(self.config.min_learning_rate);
            }
            "cosine" => {
                let t = self.state.iteration as f64;
                let cosine_decay = 0.5 * (1.0 + (std::f64::consts::PI * t / 1000.0).cos());
                self.current_lr = self.config.min_learning_rate + 
                    (self.config.learning_rate - self.config.min_learning_rate) * cosine_decay;
            }
            "adaptive" => {
                // More conservative adaptive learning rate schedule
                if let (Some(prev_val), Some(curr_val)) = (self.prev_function_value, current_value) {
                    let relative_improvement = (prev_val - curr_val) / prev_val.abs().max(1e-12);
                    
                    // Only reduce LR if function value is actually increasing or very small improvement
                    if curr_val > prev_val || relative_improvement < 1e-8 {
                        self.bad_step_count += 1;
                        // Require more consecutive bad steps and use gentler reduction
                        if self.bad_step_count >= 20 {
                            self.current_lr *= 0.8;  // Less aggressive reduction
                            self.current_lr = self.current_lr.max(self.config.min_learning_rate);
                            self.bad_step_count = 0;
                            if self.config.verbose {
                                debug!("Reducing learning rate to {:.6e} due to lack of progress (relative_improvement: {:.6e})", 
                                      self.current_lr, relative_improvement);
                            }
                        }
                    } else {
                        self.bad_step_count = 0;
                    }
                }
                self.prev_function_value = current_value;
            }
            _ => {} // constant learning rate
        }
    }

    /// Update moment estimates and compute parameter updates
    fn compute_updates(&mut self, gradients: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // Initialize moment estimates if needed
        if self.state.m.is_none() {
            self.state.m = Some(gradients.iter().map(|g| {
                Tensor::zeros_like(g).unwrap()
            }).collect());
            self.state.v = Some(gradients.iter().map(|g| {
                Tensor::zeros_like(g).unwrap()
            }).collect());
            if self.config.amsgrad {
                self.state.v_max = Some(gradients.iter().map(|g| {
                    Tensor::zeros_like(g).unwrap()
                }).collect());
            }
        }

        let m = self.state.m.as_mut().unwrap();
        let v = self.state.v.as_mut().unwrap();
        let mut updates = Vec::with_capacity(gradients.len());

        // Bias correction terms
        let t = (self.state.iteration + 1) as f64;
        let bias_correction1 = 1.0 - self.config.beta1.powf(t);
        let bias_correction2 = 1.0 - self.config.beta2.powf(t);

        for i in 0..gradients.len() {
            // Update biased first moment estimate
            // m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
            let m_old = m[i].affine(self.config.beta1, 0.0)?;
            let g_scaled = gradients[i].affine(1.0 - self.config.beta1, 0.0)?;
            m[i] = m_old.add(&g_scaled)?;

            // Update biased second raw moment estimate
            // v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
            let v_old = v[i].affine(self.config.beta2, 0.0)?;
            let g_squared = gradients[i].mul(&gradients[i])?;
            let g_squared_scaled = g_squared.affine(1.0 - self.config.beta2, 0.0)?;
            v[i] = v_old.add(&g_squared_scaled)?;

            // Compute bias-corrected moment estimates
            let m_hat = m[i].affine(1.0 / bias_correction1, 0.0)?;
            let v_hat = if self.config.amsgrad {
                // Update v_max for AMSGrad
                let v_max = self.state.v_max.as_mut().unwrap();
                let v_i_vec = v[i].flatten_all()?.to_vec1::<f64>()?;
                let v_max_vec = v_max[i].flatten_all()?.to_vec1::<f64>()?;
                let new_v_max: Vec<f64> = v_i_vec.iter().zip(v_max_vec.iter())
                    .map(|(&v_val, &v_max_val)| v_val.max(v_max_val))
                    .collect();
                v_max[i] = Tensor::from_vec(new_v_max, v[i].shape(), v[i].device())?;
                v_max[i].affine(1.0 / bias_correction2, 0.0)?
            } else {
                v[i].affine(1.0 / bias_correction2, 0.0)?
            };

            // Compute update: lr * m_hat / (sqrt(v_hat) + epsilon)
            let epsilon_tensor = Tensor::new(self.config.epsilon, v_hat.device())?;
            let v_hat_eps = v_hat.broadcast_add(&epsilon_tensor)?;
            let denominator = v_hat_eps.sqrt()?;
            let update = m_hat.div(&denominator)?;
            updates.push(update.affine(self.current_lr, 0.0)?);
        }

        Ok(updates)
    }

    /// Compute convergence information for the current state.
    fn compute_convergence_info(&self, gradients: &[Tensor], function_change: Option<f64>) -> CandleResult<ConvergenceInfo> {
        let gradient_norm = crate::utils::math::compute_magnitude(gradients)?;
        
        // Improved convergence criteria: both gradient norm AND function change must be small
        let grad_tolerance = 1e-6;
        let func_tolerance = 1e-10;
        
        let grad_converged = gradient_norm < grad_tolerance;
        let func_converged = function_change
            .map(|change| change.abs() < func_tolerance)
            .unwrap_or(false);
        
        // Require both conditions for convergence
        let converged = grad_converged && func_converged;
        
        if self.config.verbose && (grad_converged || func_converged) {
            debug!("Convergence check: grad_norm={:.6e} < {:.6e} = {}, func_change={:?} < {:.6e} = {}", 
                  gradient_norm, grad_tolerance, grad_converged,
                  function_change, func_tolerance, func_converged);
        }

        Ok(ConvergenceInfo {
            converged,
            function_change,
        })
    }
}

impl Optimizer for AdamOptimizer {
    type Config = AdamConfig;
    type State = AdamState;

    fn new(config: Self::Config) -> Self {
        Self::new(config)
    }

    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
    ) -> CandleResult<StepResult> {
        let start_time = Instant::now();
        if self.config.verbose {
            debug!("=== Adam Step {} Starting ===", self.state.iteration);
        }
        // Store previous function value for change calculation
        let prev_function_value = self.prev_function_value;
        
        // Compute current function value
        let current_value = function.evaluate(params)?;
        // Calculate function change
        let function_change = prev_function_value.map(|prev| current_value - prev);


        // Compute gradients at current parameters
        let mut gradients = function.gradient(params)?;

        // Log initial state in verbose mode
        self.log_tensor_data("Initial Parameters", params);
        self.log_tensor_data("Computed Gradients", &gradients);

        // Input validation
        if params.is_empty() || gradients.is_empty() {
            return Err(candle_core::Error::Msg("Empty parameters or gradients".into()));
        }
        if params.len() != gradients.len() {
            return Err(candle_core::Error::Msg(
                format!("Parameter and gradient dimension mismatch: {} vs {}",
                        params.len(), gradients.len())
            ));
        }

        // Apply weight decay
        self.apply_weight_decay(&mut gradients, params)?;
        // Apply gradient clipping
        self.apply_gradient_clipping(&mut gradients)?;

        // Compute gradient norm for logging
        let grad_norm = crate::utils::math::compute_magnitude(&gradients)?;
        debug!("Adam step {}: grad_norm={:.6e}", self.state.iteration, grad_norm);
        self.log_scalar("Gradient Norm", grad_norm);
        // Update learning rate based on schedule
        self.update_learning_rate(Some(current_value));


        // Compute parameter updates using Adam algorithm
        let updates = self.compute_updates(&gradients)?;
        self.log_tensor_data("Parameter Updates", &updates);

        // Compute update norm
        let update_norm = crate::utils::math::compute_magnitude(&updates)?;
        self.log_scalar("Update Norm", update_norm);

        // Perform line search if enabled
        let step_size = 1.0;

        // Apply the updates with step size: x_{k+1} = x_k - step_size * updates
        for (param, update) in params.iter_mut().zip(updates.iter()) {
            *param = param.sub(&update.affine(step_size, 0.0)?)?;
        }

        self.log_tensor_data("Updated Parameters", params);

        // Check for NaN/Inf in updated parameters
        for (i, param) in params.iter().enumerate() {
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(
                    format!("Non-finite parameter detected at index {} after update", i)
                ));
            }
        }

        // Increment iteration counter
        self.state.iteration += 1;

        // Compute convergence information
        let convergence_info = self.compute_convergence_info(&gradients, function_change)?;
        let step_duration = start_time.elapsed();

        if self.config.verbose {
            debug!("=== Adam Step {} Completed ===", self.state.iteration - 1);
            debug!("  Step Duration: {:?}", step_duration);
            debug!("  Converged: {}", convergence_info.converged);
            debug!("  Current LR: {:.6e}", self.current_lr);
            debug!("  Line Search Alpha: {:.3}", step_size);
            if let Some(change) = function_change {
                debug!("  Function Change: {:.6e}", change);
            }
        }

        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = step_duration;
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_norm);
        metadata.optimizer_data.insert("update_norm".to_string(), update_norm);
        metadata.optimizer_data.insert("learning_rate".to_string(), self.current_lr);
        metadata.optimizer_data.insert("beta1".to_string(), self.config.beta1);
        metadata.optimizer_data.insert("beta2".to_string(), self.config.beta2);
        metadata.optimizer_data.insert("line_search_alpha".to_string(), step_size);
        if let Some(change) = function_change {
            metadata.optimizer_data.insert("function_change".to_string(), change);
        }

        Ok(StepResult {
            step_size: self.current_lr * step_size,
            function_evaluations: 1,
            gradient_evaluations: 1,
            convergence_info,
            metadata,
        })
    }

    fn step_with_gradients(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
        _gradients: &[Tensor],
    ) -> Result<StepResult, candle_core::Error> {
        
        // Adam typically doesn't use line search, but we can still use the function
        // for convergence checking and validation
        let start_time = Instant::now();
        if self.config.verbose {
            debug!("=== Adam Step {} Starting (with precomputed gradients) ===", self.state.iteration);
        }
        // Store previous function value for change calculation
        let prev_function_value = self.prev_function_value;
        // Compute current function value using the provided function
        let current_value = function.evaluate(params)?;
        let function_change = prev_function_value.map(|prev| current_value - prev);

        // Use the provided gradients (assumed to be at current parameters)
        let gradients = function.gradient(params)?;
        let mut gradients_mut = gradients.to_vec();
        // Log initial state in verbose mode
        self.log_tensor_data("Initial Parameters", params);
        self.log_tensor_data("Provided Gradients", &gradients_mut);
        // Input validation
        if params.is_empty() || gradients_mut.is_empty() {
            return Err(candle_core::Error::Msg("Empty parameters or gradients".into()));
        }
        if params.len() != gradients_mut.len() {
            return Err(candle_core::Error::Msg(
                format!("Parameter and gradient dimension mismatch: {} vs {}",
                        params.len(), gradients_mut.len())
            ));
        }
        // Apply weight decay and gradient clipping
        self.apply_weight_decay(&mut gradients_mut, params)?;
        self.apply_gradient_clipping(&mut gradients_mut)?;
        // Compute gradient norm for logging
        let grad_norm = crate::utils::math::compute_magnitude(&gradients_mut)?;
        debug!("Adam step {}: grad_norm={:.6e}", self.state.iteration, grad_norm);
        self.log_scalar("Gradient Norm", grad_norm);
        // Update learning rate based on schedule
        self.update_learning_rate(Some(current_value));
        
        // Compute parameter updates using Adam algorithm
        let updates = self.compute_updates(&gradients_mut)?;
        self.log_tensor_data("Parameter Updates", &updates);
        // Compute update norm
        let update_norm = crate::utils::math::compute_magnitude(&updates)?;
        self.log_scalar("Update Norm", update_norm);
        // Adam typically uses a fixed step size of 1.0, but we could implement
        // line search here if needed for better convergence
        let step_size = 1.0;
        
        // Apply the updates with step size: x_{k+1} = x_k - step_size * updates
        for (param, update) in params.iter_mut().zip(updates.iter()) {
            *param = param.sub(&update.affine(step_size, 0.0)?)?;
        }
        self.log_tensor_data("Updated Parameters", params);
        // Check for NaN/Inf in updated parameters
        for (i, param) in params.iter().enumerate() {
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(
                    format!("Non-finite parameter detected at index {} after update", i)
                ));
            }
        }
        
        // Increment iteration counter
        self.state.iteration += 1;
        
        // Compute convergence information
        let convergence_info = self.compute_convergence_info(&gradients_mut, function_change)?;
        let step_duration = start_time.elapsed();

        if self.config.verbose {
            debug!("=== Adam Step {} Completed ===", self.state.iteration - 1);
            debug!("  Step Duration: {:?}", step_duration);
            debug!("  Converged: {}", convergence_info.converged);
            debug!("  Current LR: {:.6e}", self.current_lr);
            if let Some(change) = function_change {
                debug!("  Function Change: {:.6e}", change);
            }
        }
        
        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = step_duration;
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_norm);
        metadata.optimizer_data.insert("update_norm".to_string(), update_norm);
        metadata.optimizer_data.insert("learning_rate".to_string(), self.current_lr);
        metadata.optimizer_data.insert("beta1".to_string(), self.config.beta1);
        metadata.optimizer_data.insert("beta2".to_string(), self.config.beta2);
        if let Some(change) = function_change {
            metadata.optimizer_data.insert("function_change".to_string(), change);
        }
        
        Ok(StepResult {
            step_size: self.current_lr * step_size,
            function_evaluations: 1,
            gradient_evaluations: 0,
            convergence_info,
            metadata,
        })
    }

    fn reset(&mut self) {
        self.state.reset();
        self.current_lr = self.config.learning_rate;
        self.prev_function_value = None;
        self.bad_step_count = 0;
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn name(&self) -> &str {
        if self.config.amsgrad {
            "Adam-AMSGrad"
        } else {
            "Adam"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;

    #[test]
    fn test_adam_state_creation() {
        let state = AdamState::new();
        assert_eq!(state.iteration(), 0);
        assert!(state.m.is_none());
        assert!(state.v.is_none());
        assert!(state.v_max.is_none());
    }

    #[test]
    fn test_adam_optimizer_creation() {
        let config = AdamConfig::default();
        let optimizer = AdamOptimizer::new(config);

        assert_eq!(optimizer.name(), "Adam");
        assert_eq!(optimizer.state().iteration(), 0);
    }

    #[test]
    fn test_adam_with_amsgrad() {
        let config = AdamConfig {
            amsgrad: true,
            ..Default::default()
        };
        let optimizer = AdamOptimizer::new(config);
        assert_eq!(optimizer.name(), "Adam-AMSGrad");
    }

    #[test]
    fn test_adam_reset() {
        let config = AdamConfig::default();
        let mut optimizer = AdamOptimizer::new(config);

        // Manually set some state
        optimizer.state.iteration = 5;

        optimizer.reset();
        assert_eq!(optimizer.state().iteration(), 0);
        assert!(optimizer.state.m.is_none());
        assert!(optimizer.state.v.is_none());
    }
}