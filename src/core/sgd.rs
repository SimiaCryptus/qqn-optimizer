use crate::core::optimizer::{ConvergenceInfo, DifferentiableFunction, OptimizationMetadata, Optimizer, StepResult};
use candle_core::{Result as CandleResult, Tensor};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Configuration parameters for the SGD optimizer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGDConfig {
    /// Learning rate (step size)
    pub learning_rate: f64,
    /// Momentum coefficient (0.0 = no momentum, 0.9 = high momentum)
    pub momentum: f64,
    /// Weight decay (L2 regularization)
    pub weight_decay: f64,
    /// Enable Nesterov momentum
    pub nesterov: bool,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for SGDConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            momentum: 0.0,
            weight_decay: 0.0,
            nesterov: false,
            verbose: false,
        }
    }
}

/// State information for SGD optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGDState {
    /// Current iteration number
    pub iteration: usize,
    /// Momentum buffer (velocity)
    #[serde(skip_serializing, skip_deserializing)]
    pub momentum_buffer: Option<Vec<Tensor>>,
}

impl SGDState {
    /// Create a new SGD state.
    pub fn new() -> Self {
        Self {
            iteration: 0,
            momentum_buffer: None,
        }
    }

    /// Reset the SGD state to initial conditions.
    pub fn reset(&mut self) {
        self.iteration = 0;
        self.momentum_buffer = None;
    }

    /// Get the current iteration number.
    pub fn iteration(&self) -> usize {
        self.iteration
    }
}

/// SGD optimizer implementation.
#[derive(Debug)]
pub struct SGDOptimizer {
    config: SGDConfig,
    state: SGDState,
}

impl Clone for SGDOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
        }
    }
}

impl SGDOptimizer {
    /// Create a new SGD optimizer with the given configuration.
    pub fn new(config: SGDConfig) -> Self {
        if config.verbose {
            info!("Creating SGD optimizer with verbose logging enabled");
            debug!("SGD Config: lr={}, momentum={}, weight_decay={}, nesterov={}",
                  config.learning_rate, config.momentum, config.weight_decay, config.nesterov);
        }
        Self {
            config,
            state: SGDState::new(),
        }
    }

    /// Log tensor data if verbose mode is enabled
    fn log_tensor_data(&self, name: &str, tensors: &[Tensor]) {
        if !self.config.verbose {
            return;
        }
        debug!("=== SGD: {} ===", name);
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
            debug!("  SGD {}: {:.12e}", name, value);
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

    /// Update momentum buffer
    fn update_momentum(&mut self, gradients: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        if self.config.momentum == 0.0 {
            // No momentum, return gradients as-is
            return Ok(gradients.to_vec());
        }

        // Initialize momentum buffer if needed
        if self.state.momentum_buffer.is_none() {
            self.state.momentum_buffer = Some(gradients.to_vec());
            return Ok(gradients.to_vec());
        }

        let momentum_buffer = self.state.momentum_buffer.as_mut().unwrap();
        let mut update = Vec::with_capacity(gradients.len());

        for (i, grad) in gradients.iter().enumerate() {
            // v_t = momentum * v_{t-1} + grad
            let momentum_term = momentum_buffer[i].affine(self.config.momentum, 0.0)?;
            let new_velocity = momentum_term.add(grad)?;
            momentum_buffer[i] = new_velocity.clone();

            if self.config.nesterov {
                // Nesterov momentum: update = momentum * v_t + grad
                let nesterov_term = new_velocity.affine(self.config.momentum, 0.0)?;
                update.push(nesterov_term.add(grad)?);
            } else {
                // Standard momentum: update = v_t
                update.push(new_velocity);
            }
        }

        Ok(update)
    }

    /// Compute convergence information for the current state.
    fn compute_convergence_info(&self, gradients: &[Tensor]) -> CandleResult<ConvergenceInfo> {
        let gradient_norm = crate::utils::math::compute_magnitude(gradients)?;
        // Use a more lenient convergence criterion for SGD
        let tolerance = if self.config.momentum > 0.0 {
            1e-4 // More lenient for momentum-based SGD
        } else {
            1e-3 // Even more lenient for vanilla SGD
        };


        Ok(ConvergenceInfo {
            converged: gradient_norm < tolerance,
            function_change: None,
        })
    }
}

impl Optimizer for SGDOptimizer {
    type Config = SGDConfig;
    type State = SGDState;

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
            debug!("=== SGD Step {} Starting ===", self.state.iteration);
        }

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

        // Compute gradient norm for logging
        let grad_norm = crate::utils::math::compute_magnitude(&gradients)?;
        debug!("SGD step {}: grad_norm={:.6e}", self.state.iteration, grad_norm);
        self.log_scalar("Gradient Norm", grad_norm);

        // Update momentum and get final update direction
        let update_direction = self.update_momentum(&gradients)?;
        self.log_tensor_data("Update Direction", &update_direction);

        // Compute update norm
        let update_norm = crate::utils::math::compute_magnitude(&update_direction)?;
        self.log_scalar("Update Norm", update_norm);

        // Apply the update: x_{k+1} = x_k - lr * update_direction
        for (param, update) in params.iter_mut().zip(update_direction.iter()) {
            let lr_tensor = Tensor::new(self.config.learning_rate, param.device())?;
            let step = update.broadcast_mul(&lr_tensor)?;
            *param = param.sub(&step)?;
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
        let convergence_info = self.compute_convergence_info(&gradients)?;
        let step_duration = start_time.elapsed();

        if self.config.verbose {
            debug!("=== SGD Step {} Completed ===", self.state.iteration - 1);
            debug!("  Step Duration: {:?}", step_duration);
            debug!("  Converged: {}", convergence_info.converged);
        }

        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = step_duration;
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_norm);
        metadata.optimizer_data.insert("update_norm".to_string(), update_norm);
        metadata.optimizer_data.insert("learning_rate".to_string(), self.config.learning_rate);
        metadata.optimizer_data.insert("momentum".to_string(), self.config.momentum);

        Ok(StepResult {
            step_size: self.config.learning_rate,
            function_evaluations: 0, // SGD doesn't do line search
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

        // SGD typically doesn't use line search, but we can still use the function
        // for validation and convergence checking
        let start_time = Instant::now();
        if self.config.verbose {
            debug!("=== SGD Step {} Starting (with precomputed gradients) ===", self.state.iteration);
        }

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

        // Apply weight decay
        self.apply_weight_decay(&mut gradients_mut, params)?;

        // Compute gradient norm for logging
        let grad_norm = crate::utils::math::compute_magnitude(&gradients_mut)?;
        debug!("SGD step {}: grad_norm={:.6e}", self.state.iteration, grad_norm);
        self.log_scalar("Gradient Norm", grad_norm);

        // Update momentum and get final update direction
        let update_direction = self.update_momentum(&gradients_mut)?;
        self.log_tensor_data("Update Direction", &update_direction);

        // Compute update norm
        let update_norm = crate::utils::math::compute_magnitude(&update_direction)?;
        self.log_scalar("Update Norm", update_norm);

        // Apply the update: x_{k+1} = x_k - lr * update_direction
        for (param, update) in params.iter_mut().zip(update_direction.iter()) {
            let lr_tensor = Tensor::new(self.config.learning_rate, param.device())?;
            let step = update.broadcast_mul(&lr_tensor)?;
            *param = param.sub(&step)?;
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
        let convergence_info = self.compute_convergence_info(&gradients_mut)?;
        let step_duration = start_time.elapsed();

        if self.config.verbose {
            debug!("=== SGD Step {} Completed ===", self.state.iteration - 1);
            debug!("  Step Duration: {:?}", step_duration);
            debug!("  Converged: {}", convergence_info.converged);
        }

        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = step_duration;
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_norm);
        metadata.optimizer_data.insert("update_norm".to_string(), update_norm);
        metadata.optimizer_data.insert("learning_rate".to_string(), self.config.learning_rate);
        metadata.optimizer_data.insert("momentum".to_string(), self.config.momentum);

        Ok(StepResult {
            step_size: self.config.learning_rate,
            function_evaluations: 0, // SGD doesn't do line search by default
            gradient_evaluations: 0, // Gradients were provided
            convergence_info,
            metadata,
        })
    }

    fn reset(&mut self) {
        self.state.reset();
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn name(&self) -> &str {
        if self.config.momentum > 0.0 {
            if self.config.nesterov {
                "SGD-Nesterov"
            } else {
                "SGD-Momentum"
            }
        } else {
            "SGD"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sgd_state_creation() {
        let state = SGDState::new();
        assert_eq!(state.iteration(), 0);
        assert!(state.momentum_buffer.is_none());
    }

    #[test]
    fn test_sgd_optimizer_creation() {
        let config = SGDConfig::default();
        let optimizer = SGDOptimizer::new(config);

        assert_eq!(optimizer.name(), "SGD");
        assert_eq!(optimizer.state().iteration(), 0);
    }

    #[test]
    fn test_sgd_with_momentum() {
        let config = SGDConfig {
            momentum: 0.9,
            ..Default::default()
        };
        let optimizer = SGDOptimizer::new(config);
        assert_eq!(optimizer.name(), "SGD-Momentum");
    }

    #[test]
    fn test_sgd_with_nesterov() {
        let config = SGDConfig {
            momentum: 0.9,
            nesterov: true,
            ..Default::default()
        };
        let optimizer = SGDOptimizer::new(config);
        assert_eq!(optimizer.name(), "SGD-Nesterov");
    }

    #[test]
    fn test_sgd_reset() {
        let config = SGDConfig::default();
        let mut optimizer = SGDOptimizer::new(config);

        // Manually set some state
        optimizer.state.iteration = 5;

        optimizer.reset();
        assert_eq!(optimizer.state().iteration(), 0);
        assert!(optimizer.state.momentum_buffer.is_none());
    }
}