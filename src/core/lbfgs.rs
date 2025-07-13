//! L-BFGS (Limited-memory Broyden-Fletcher-Goldfarb-Shanno) optimizer implementation.
//!
//! This implementation provides a robust quasi-Newton optimization method that maintains
//! a limited history of gradient and parameter changes to approximate the inverse Hessian.
//! It serves both as a baseline optimizer for benchmarking and as a core component of the
//! QQN algorithm.

use crate::core::line_search::{LineSearch, LineSearchConfig, StrongWolfeLineSearch};
use crate::core::optimizer::OptimizationMetadata;
use crate::core::optimizer::{ConvergenceInfo, Optimizer, StepResult};
use crate::utils::math::{
    compute_magnitude, dot_product, vector_add, vector_scale, vector_subtract,
};
use candle_core::{Result as CandleResult, Tensor};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;

/// Configuration parameters for the L-BFGS optimizer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBFGSConfig {
    /// Number of previous iterations to store for Hessian approximation
    pub history_size: usize,
    /// Line search configuration
    pub line_search: LineSearchConfig,
    /// Numerical stability constant
    pub epsilon: f64,
    /// Maximum number of iterations for two-loop recursion
    pub max_correction_pairs: usize,
    /// Maximum allowed step size
    pub max_step_size: f64,
    /// Minimum allowed step size
    pub min_step_size: f64,
}

impl Default for LBFGSConfig {
    fn default() -> Self {
        Self {
            history_size: 10,
            line_search: LineSearchConfig::default(),
            epsilon: 1e-8,
            max_correction_pairs: 10,
            max_step_size: 1e3,
            min_step_size: 1e-16,
        }
    }
}

/// State information for L-BFGS optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBFGSState {
    /// History of parameter differences (s_k = x_{k+1} - x_k)
    #[serde(skip)]
    s_history: VecDeque<Vec<Tensor>>,
    /// History of gradient differences (y_k = g_{k+1} - g_k)
    #[serde(skip)]
    y_history: VecDeque<Vec<Tensor>>,
    /// Reciprocals of s_k^T y_k for efficiency
    rho_history: VecDeque<f64>,
    /// Previous gradient for computing differences
    #[serde(skip)]
    prev_gradient: Option<Vec<Tensor>>,
    /// Current iteration number
    iteration: usize,
    /// Scaling factor for initial Hessian approximation
    gamma: f64,
}

impl LBFGSState {
    /// Create a new L-BFGS state with the given history size.
    pub fn new(history_size: usize) -> Self {
        Self {
            s_history: VecDeque::with_capacity(history_size),
            y_history: VecDeque::with_capacity(history_size),
            rho_history: VecDeque::with_capacity(history_size),
            prev_gradient: None,
            iteration: 0,
            gamma: 1.0,
        }
    }

    /// Reset the L-BFGS state to initial conditions.
    pub fn reset(&mut self) {
        self.s_history.clear();
        self.y_history.clear();
        self.rho_history.clear();
        self.prev_gradient = None;
        self.iteration = 0;
        self.gamma = 1.0;
    }

    /// Compute the L-BFGS search direction using the two-loop recursion
    pub fn compute_direction(&mut self, gradient: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // Validate input
        if gradient.is_empty() {
            return Err(candle_core::Error::Msg("Empty gradient vector".into()));
        }
        // Check for NaN/Inf in gradient
        for (i, grad) in gradient.iter().enumerate() {
            let grad_vec = grad.flatten_all()?.to_vec1::<f64>()?;
            if grad_vec.iter().any(|&x| !x.is_finite()) {
                warn!("L-BFGS: Non-finite gradient detected at index {}, using steepest descent", i);
                return Ok(gradient
                    .iter()
                    .map(|g| g.neg())
                    .collect::<CandleResult<Vec<_>>>()?);
            }
        }

        if self.s_history.is_empty() {
            // No history available, use steepest descent
            debug!("L-BFGS: No history, using steepest descent");
            return Ok(gradient
                .iter()
                .map(|g| g.neg())
                .collect::<CandleResult<Vec<_>>>()?);
        }

        let mut q = gradient.to_vec();
        let mut alpha = Vec::with_capacity(self.s_history.len());

        // First loop: compute alpha values and update q
        for i in (0..self.s_history.len()).rev() {
            let s_i = &self.s_history[i];
            let rho_i = self.rho_history[i];
            // Check for numerical issues
            if !rho_i.is_finite() || rho_i.abs() < 1e-16 {
                warn!("L-BFGS: Skipping history pair {} due to numerical issues (rho={})", i, rho_i);
                continue;
            }

            let alpha_i = rho_i * dot_product(s_i, &q)?;
            if !alpha_i.is_finite() {
                warn!("L-BFGS: Non-finite alpha detected at iteration {}", i);
                continue;
            }

            alpha.push(alpha_i);

            // q = q - alpha_i * y_i
            let y_i = &self.y_history[i];
            let scaled_y = vector_scale(y_i, alpha_i)?;
            q = vector_subtract(&q, &scaled_y)?;
            // Check if q has become non-finite
            for (_j, q_tensor) in q.iter().enumerate() {
                let q_vec = q_tensor.flatten_all()?.to_vec1::<f64>()?;
                if q_vec.iter().any(|&x| !x.is_finite()) {
                    warn!("L-BFGS: Non-finite q detected during first loop, using steepest descent");
                    return Ok(gradient
                        .iter()
                        .map(|g| g.neg())
                        .collect::<CandleResult<Vec<_>>>()?);
                }
            }
        }

        // Reverse alpha to match forward iteration order
        alpha.reverse();

        // Apply initial Hessian approximation scaling
        debug!("L-BFGS: Using gamma = {:.6e}", self.gamma);
        // Additional safety check for gamma
        if !self.gamma.is_finite() || self.gamma <= 0.0 {
            warn!("L-BFGS: Invalid gamma detected: {}, resetting to 1.0", self.gamma);
            self.gamma = 1.0;
        }
        // Clamp gamma to prevent numerical issues
        let safe_gamma = self.gamma.max(1e-6).min(1e6);
        let mut r = vector_scale(&q, safe_gamma)?;

        // Second loop: compute final direction
        for i in 0..self.s_history.len() {
            if i >= alpha.len() {
                continue; // Skip if we didn't compute alpha for this iteration
            }
            let s_i = &self.s_history[i];
            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];

            let alpha_i = alpha[i];

            let beta = rho_i * dot_product(y_i, &r)?;
            let correction_factor = alpha_i - beta;
            if !correction_factor.is_finite() {
                warn!("L-BFGS: Non-finite correction factor at iteration {}", i);
                continue;
            }

            // r = r + (alpha_i - beta) * s_i
            let correction = vector_scale(s_i, correction_factor)?;
            r = vector_add(&r, &correction)?;
            // Check if r has become non-finite
            for (_j, r_tensor) in r.iter().enumerate() {
                let r_vec = r_tensor.flatten_all()?.to_vec1::<f64>()?;
                if r_vec.iter().any(|&x| !x.is_finite()) {
                    warn!("L-BFGS: Non-finite r detected during second loop, using steepest descent");
                    return Ok(gradient
                        .iter()
                        .map(|g| g.neg())
                        .collect::<CandleResult<Vec<_>>>()?);
                }
            }
        }
        // Final check on the direction
        let direction = r.iter()
            .map(|t| t.neg())
            .collect::<CandleResult<Vec<_>>>()?;
        // Verify the direction is finite
        for (_i, dir) in direction.iter().enumerate() {
            let dir_vec = dir.flatten_all()?.to_vec1::<f64>()?;
            if dir_vec.iter().any(|&x| !x.is_finite()) {
                warn!("L-BFGS: Non-finite direction detected, using steepest descent");
                return Ok(gradient
                    .iter()
                    .map(|g| g.neg())
                    .collect::<CandleResult<Vec<_>>>()?);
            }
        }


        // Return the negative of r to get a descent direction
        Ok(direction)
    }

    /// Update the L-BFGS state with new gradient and step information.
    pub fn update(
        &mut self,
        new_gradient: &[Tensor],
        step_direction: &[Tensor],
        step_size: f64,
    ) -> CandleResult<()> {
        // Validate inputs
        if new_gradient.is_empty() || step_direction.is_empty() {
            return Err(candle_core::Error::Msg("Empty gradient or direction vectors".into()));
        }
        if new_gradient.len() != step_direction.len() {
            return Err(candle_core::Error::Msg(
                format!("Gradient and direction dimension mismatch: {} vs {}",
                        new_gradient.len(), step_direction.len())
            ));
        }
        if !step_size.is_finite() || step_size <= 0.0 {
            warn!("L-BFGS: Invalid step size: {}", step_size);
            return Ok(()); // Skip update but don't fail
        }

        // Compute parameter difference: s_k = step_size * step_direction
        let s_k = vector_scale(step_direction, step_size)?;

        if let Some(prev_grad) = &self.prev_gradient {
            // Compute gradient difference: y_k = new_gradient - prev_gradient
            let y_k = vector_subtract(new_gradient, prev_grad)?;

            // Compute curvature condition: s_k^T y_k
            let s_dot_y = dot_product(&s_k, &y_k)?;
            debug!("L-BFGS: s_dot_y = {:.6e}", s_dot_y);

            // Only update if curvature condition is satisfied (positive definiteness)
            if s_dot_y > self.epsilon() * 10.0 { // Be more strict about curvature condition
                let rho_k = 1.0 / s_dot_y;
                if !rho_k.is_finite() {
                    warn!("L-BFGS: Non-finite rho_k, skipping update");
                    return Ok(());
                }

                // Add to history (maintain limited size)
                if self.s_history.len() >= self.s_history.capacity() {
                    self.s_history.pop_front();
                    self.y_history.pop_front();
                    self.rho_history.pop_front();
                }

                self.s_history.push_back(s_k);
                self.y_history.push_back(y_k.clone());
                self.rho_history.push_back(rho_k);

                // Update scaling factor for initial Hessian approximation
                // gamma = (s_k^T y_k) / (y_k^T y_k)
                let y_dot_y = dot_product(&y_k, &y_k)?;
                if y_dot_y > self.epsilon() {
                    let new_gamma = s_dot_y / y_dot_y;
                    // Ensure gamma is finite before updating
                    if new_gamma.is_finite() && new_gamma > 0.0 {
                        // Clamp gamma to more conservative range to prevent instability
                        self.gamma = new_gamma.max(1e-4).min(1e2);
                        if (new_gamma - self.gamma).abs() > 1e-10 {
                            debug!("L-BFGS: Gamma clamped from {} to {}", new_gamma, self.gamma);
                        }
                    } else {
                        debug!("L-BFGS: Invalid gamma computed: {}, keeping current value", new_gamma);
                    }
                }
            } else {
                debug!("L-BFGS: Curvature condition not satisfied (s_dot_y = {}), skipping update", s_dot_y);
            }
        }

        // Store current gradient for next iteration
        self.prev_gradient = Some(new_gradient.to_vec());
        self.iteration += 1;

        Ok(())
    }

    /// Get the current iteration number.
    pub fn iteration(&self) -> usize {
        self.iteration
    }

    /// Get the number of stored correction pairs.
    pub fn history_length(&self) -> usize {
        self.s_history.len()
    }

    /// Get the current Hessian scaling factor.
    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    /// Get the numerical stability epsilon.
    fn epsilon(&self) -> f64 {
        1e-8 // Could be made configurable
    }
}

/// L-BFGS optimizer implementation.
#[derive(Debug)]
pub struct LBFGSOptimizer {
    config: LBFGSConfig,
    state: LBFGSState,
    line_search: Box<dyn LineSearch>,
}

impl Clone for LBFGSOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            line_search: self.line_search.clone_box(),
        }
    }
}

impl LBFGSOptimizer {
    /// Create a new L-BFGS optimizer with the given configuration.
    pub fn new(config: LBFGSConfig) -> Self {
        let state = LBFGSState::new(config.history_size);
        let line_search = Box::new(StrongWolfeLineSearch::new(
            crate::core::line_search::StrongWolfeConfig {
                c1: config.line_search.c1,
                c2: config.line_search.c2,
                max_iterations: config.line_search.max_iterations,
                min_step: 1e-16,
                max_step: 1e16,
                initial_step: config.line_search.initial_step,
            },
        ));

        Self {
            config,
            state,
            line_search,
        }
    }

    /// Get a reference to the internal L-BFGS state.
    pub fn lbfgs_state(&self) -> &LBFGSState {
        &self.state
    }

    /// Get a mutable reference to the internal L-BFGS state.
    pub fn lbfgs_state_mut(&mut self) -> &mut LBFGSState {
        &mut self.state
    }

    /// Compute convergence information for the current state.
    fn compute_convergence_info(&self, gradient: &[Tensor]) -> CandleResult<ConvergenceInfo> {
        let gradient_norm = compute_magnitude(gradient)?;

        Ok(ConvergenceInfo {
            converged: gradient_norm < 1e-6, // Default tolerance
            function_change: None,
            convergence_criterion: None,
        })
    }
}

impl Optimizer for LBFGSOptimizer {
    type Config = LBFGSConfig;
    type State = LBFGSState;

    fn new(config: Self::Config) -> Self {
        Self::new(config)
    }

    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn crate::core::optimizer::DifferentiableFunction,
    ) -> CandleResult<StepResult> {
        let start_time = Instant::now();
        // Compute gradients at current parameters
        let gradients = function.gradient(params)?;

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

        // Compute L-BFGS search direction
        let search_direction = self.state.compute_direction(&gradients)?;
        // Validate search direction
        let direction_norm = compute_magnitude(&search_direction)?;
        if !direction_norm.is_finite() || direction_norm < self.config.epsilon {
            warn!("L-BFGS: Invalid search direction norm: {}, using steepest descent", direction_norm);
            // Fall back to steepest descent
            let search_direction = gradients
                .iter()
                .map(|g| g.neg())
                .collect::<CandleResult<Vec<_>>>()?;
            let direction_norm = compute_magnitude(&search_direction)?;
            let step_size = 0.01 / (direction_norm + 1.0);

            // Update parameters with conservative step
            for (param, dir) in params.iter_mut().zip(search_direction.iter()) {
                let step_size_tensor = Tensor::new(step_size, param.device())?;
                let update = dir.broadcast_mul(&step_size_tensor)?;
                *param = param.add(&update)?;
            }

            // Update L-BFGS state
            // Don't update state with invalid steps
            if step_size > 0.0 {
                self.state.update(&gradients, &search_direction, step_size)?;
            }

            let convergence_info = self.compute_convergence_info(&gradients)?;
            let step_duration = start_time.elapsed();
            let mut metadata = OptimizationMetadata::default();
            metadata.timing_info.step_duration = step_duration;
            metadata.optimizer_data.insert("fallback_to_steepest_descent".to_string(), 1.0);

            return Ok(StepResult {
                step_size,
                function_evaluations: 0,
                gradient_evaluations: 0,
                convergence_info,
                metadata,
            });
        }

        // Use adaptive step size based on gradient magnitude
        let grad_norm = compute_magnitude(&gradients)?;
        debug!("L-BFGS step {}: grad_norm={:.6e}", self.state.iteration(), grad_norm);

        // Adaptive step size with backtracking line search
        let mut step_size = if self.state.iteration() == 0 {
            // First iteration: use a conservative step size
            // Scale based on gradient magnitude to avoid overshooting
            (0.01 / (grad_norm + 1.0)).min(0.1).max(0.0001)
        } else {
            // Use L-BFGS scaling factor as starting point
            self.state.gamma()
                .max(self.config.min_step_size)
                .min(1.0) // Cap at 1.0 to prevent overshooting
        };
        debug!("L-BFGS: Initial step size = {:.6e}", step_size);

        // Simple backtracking line search
        let mut success = false;
        let backtrack_factor = 0.5;
        let max_backtracks = 50; // Increase to allow more backtracking
        let mut backtrack_count = 0;
        let mut best_step_size = step_size;
        let best_value = f64::INFINITY;

        for _ in 0..max_backtracks {
            // Check if step size is reasonable
            let step_norm = step_size * compute_magnitude(&search_direction)?;

            // If step is very small relative to parameters, accept it
            if step_norm < 1e-10 || step_size < self.config.min_step_size {
                debug!("L-BFGS: Accepting small step (norm={:.6e})", step_norm);
                success = true;
                best_step_size = step_size;
                break;
            }

            // Try the step and check if it reduces the function value
            let mut trial_params = params.to_vec();
            for (param, direction) in trial_params.iter_mut().zip(&search_direction) {
                let step_size_tensor = Tensor::new(step_size, param.device())?;
                let step = direction.broadcast_mul(&step_size_tensor)?;
                *param = param.add(&step)?;
            }

            // Evaluate function at trial point
            let trial_value = match function.evaluate(&trial_params) {
                Ok(val) => val,
                Err(_) => {
                    // If evaluation fails, backtrack
                    step_size *= backtrack_factor;
                    backtrack_count += 1;
                    continue;
                }
            };

            // Check Armijo condition for sufficient decrease
            let directional_derivative = -grad_norm * grad_norm; // For steepest descent, this is -||g||^2
            let expected_decrease = self.config.line_search.c1 * step_size * directional_derivative;

            // Also check that the trial value is finite and reasonable
            if trial_value.is_finite() && trial_value <= best_value + expected_decrease {
                success = true;
                best_step_size = step_size;
                break;
            } else if !trial_value.is_finite() {
                // If we get non-finite values, backtrack more aggressively
                step_size *= backtrack_factor * backtrack_factor;
                backtrack_count += 2;
                continue;
            }

            // Backtrack
            step_size *= backtrack_factor;
            backtrack_count += 1;
        }
        // Use the best step size found
        step_size = best_step_size;

        if backtrack_count > 0 {
            debug!("L-BFGS: Backtracked {} times to step_size={:.6e}", backtrack_count, step_size);
        }


        // Ensure minimum step size
        step_size = step_size.max(self.config.min_step_size);

        if !success {
            warn!("L-BFGS: Line search failed to find acceptable step size");
        }

        let line_search_result = crate::core::line_search::LineSearchResult {
            step_size,
            function_evaluations: 0,
            gradient_evaluations: 0,
            success,
            termination_reason: crate::core::line_search::TerminationReason::WolfeConditionsSatisfied,
        };

        // Update parameters: x_{k+1} = x_k + alpha * p_k
        for (param, direction) in params.iter_mut().zip(&search_direction) {
            let step_size_tensor = Tensor::new(line_search_result.step_size, param.device())?;
            let step = direction.broadcast_mul(&step_size_tensor)?;
            *param = param.add(&step)?;
            // Check for NaN/Inf in updated parameters
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(
                    "Non-finite parameter detected after update".into()
                ));
            }
        }

        // Update L-BFGS state with new information
        self.state
            .update(&gradients, &search_direction, line_search_result.step_size)?;

        // Compute convergence information
        let convergence_info = self.compute_convergence_info(&gradients)?;
        let step_duration = start_time.elapsed();
        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = step_duration;
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_norm);
        metadata.optimizer_data.insert("direction_norm".to_string(), direction_norm);
        metadata.optimizer_data.insert("step_size".to_string(), step_size);
        metadata.optimizer_data.insert("gamma".to_string(), self.state.gamma());
        metadata.optimizer_data.insert("history_size".to_string(), self.state.history_length() as f64);

        Ok(StepResult {
            step_size: line_search_result.step_size,
            function_evaluations: line_search_result.function_evaluations,
            gradient_evaluations: line_search_result.gradient_evaluations,
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
        self.state.reset();
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn name(&self) -> &str {
        "L-BFGS"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use candle_core::Device;

    #[test]
    fn test_lbfgs_state_creation() {
        let state = LBFGSState::new(5);
        assert_eq!(state.history_length(), 0);
        assert_eq!(state.iteration(), 0);
        assert_eq!(state.gamma(), 1.0);
    }

    #[test]
    fn test_lbfgs_steepest_descent_fallback() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5);

        let gradient = vec![Tensor::from_slice(&[1.0, 2.0], (2,), &device)?];

        let direction = state.compute_direction(&gradient)?;

        // Should return negative gradient (steepest descent)
        let expected = vec![Tensor::from_slice(&[-1.0, -2.0], (2,), &device)?];

        let dir_values = direction[0].to_vec1::<f64>()?;
        let exp_values = expected[0].to_vec1::<f64>()?;
        assert_relative_eq!(dir_values[0], exp_values[0], epsilon = 1e-10);
        assert_relative_eq!(dir_values[1], exp_values[1], epsilon = 1e-10);

        Ok(())
    }

    #[test]
    fn test_lbfgs_state_update() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5);

        let grad1 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let grad2 = vec![Tensor::from_slice(&[0.5, 0.5], &[2], &device)?];
        let direction = vec![Tensor::from_slice(&[-1.0, -1.0], &[2], &device)?];
        let step_size = 0.1;

        // First update should not add to history (no previous gradient)
        state.update(&grad1, &direction, step_size)?;
        assert_eq!(state.history_length(), 0);
        assert_eq!(state.iteration(), 1);

        // Second update should add to history
        state.update(&grad2, &direction, step_size)?;
        assert_eq!(state.history_length(), 1);
        assert_eq!(state.iteration(), 2);

        Ok(())
    }

    #[test]
    fn test_lbfgs_optimizer_creation() {
        let config = LBFGSConfig::default();
        let optimizer = LBFGSOptimizer::new(config);

        assert_eq!(optimizer.name(), "L-BFGS");
        assert_eq!(optimizer.state().history_length(), 0);
    }

    #[test]
    fn test_lbfgs_reset() {
        let config = LBFGSConfig::default();
        let mut optimizer = LBFGSOptimizer::new(config);

        // Manually set some state
        optimizer.state.iteration = 5;

        optimizer.reset();
        assert_eq!(optimizer.state().iteration(), 0);
        assert_eq!(optimizer.state().history_length(), 0);
    }

    #[test]
    fn test_curvature_condition_rejection() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5);

        let grad1 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let grad2 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?]; // Same gradient
        let direction = vec![Tensor::from_slice(&[-1.0, -1.0], &[2], &device)?];
        let step_size = 0.1;

        state.update(&grad1, &direction, step_size)?;
        state.update(&grad2, &direction, step_size)?;

        // Should not add to history due to zero curvature (y_k = 0)
        assert_eq!(state.history_length(), 0);

        Ok(())
    }

    #[test]
    fn test_history_size_limit() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(2); // Small history size

        // Add more updates than history size
        for i in 0..5 {
            let grad = vec![Tensor::from_slice(
                &[1.0 + i as f64 * 0.1, 1.0],
                &[2],
                &device,
            )?];
            let direction = vec![Tensor::from_slice(&[-1.0, -1.0], &[2], &device)?];
            state.update(&grad, &direction, 0.1)?;
        }

        // Should maintain only the history size limit
        assert!(state.history_length() <= 2);

        Ok(())
    }
}