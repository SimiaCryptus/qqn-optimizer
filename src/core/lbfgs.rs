//! L-BFGS (Limited-memory Broyden-Fletcher-Goldfarb-Shanno) optimizer implementation.
//!
//! This implementation provides a robust quasi-Newton optimization method that maintains
//! a limited history of gradient and parameter changes to approximate the inverse Hessian.
//! It serves both as a baseline optimizer for benchmarking and as a core component of the
//! QQN algorithm.

use crate::core::optimizer::{Optimizer, StepResult, ConvergenceInfo};
use crate::core::line_search::{LineSearch, StrongWolfeLineSearch, LineSearchConfig};
use crate::utils::math::{compute_magnitude, dot_product, vector_add, vector_subtract, vector_scale};
use candle_core::{Tensor, Device, Result as CandleResult};
use std::collections::VecDeque;

/// Configuration parameters for the L-BFGS optimizer.
#[derive(Debug, Clone)]
pub struct LBFGSConfig {
    /// Number of previous iterations to store for Hessian approximation
    pub history_size: usize,
    /// Line search configuration
    pub line_search: LineSearchConfig,
    /// Numerical stability constant
    pub epsilon: f64,
    /// Maximum number of iterations for two-loop recursion
    pub max_correction_pairs: usize,
}

impl Default for LBFGSConfig {
    fn default() -> Self {
        Self {
            history_size: 10,
            line_search: LineSearchConfig::strong_wolfe(),
            epsilon: 1e-8,
            max_correction_pairs: 10,
        }
    }
}

/// State information for L-BFGS optimization.
#[derive(Debug, Clone)]
pub struct LBFGSState {
    /// History of parameter differences (s_k = x_{k+1} - x_k)
    s_history: VecDeque<Vec<Tensor>>,
    /// History of gradient differences (y_k = g_{k+1} - g_k)
    y_history: VecDeque<Vec<Tensor>>,
    /// Reciprocals of s_k^T y_k for efficiency
    rho_history: VecDeque<f64>,
    /// Previous gradient for computing differences
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

    /// Compute the L-BFGS search direction using the two-loop recursion algorithm.
    pub fn compute_direction(&self, gradient: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        if self.s_history.is_empty() {
            // No history available, use steepest descent
            return Ok(gradient.iter()
                .map(|g| g.neg())
                .collect::<CandleResult<Vec<_>>>()?);
        }

        let mut q = gradient.to_vec();
        let mut alpha = Vec::with_capacity(self.s_history.len());

        // First loop: compute alpha values and update q
        for i in (0..self.s_history.len()).rev() {
            let s_i = &self.s_history[i];
            let rho_i = self.rho_history[i];
            
            let alpha_i = rho_i * dot_product(s_i, &q)?;
            alpha.push(alpha_i);
            
            // q = q - alpha_i * y_i
            let y_i = &self.y_history[i];
            let scaled_y = vector_scale(y_i, alpha_i)?;
            q = vector_subtract(&q, &scaled_y)?;
        }

        // Reverse alpha to match forward iteration order
        alpha.reverse();

        // Apply initial Hessian approximation scaling
        let mut r = vector_scale(&q, self.gamma)?;

        // Second loop: compute final direction
        for i in 0..self.s_history.len() {
            let s_i = &self.s_history[i];
            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];
            let alpha_i = alpha[i];
            
            let beta = rho_i * dot_product(y_i, &r)?;
            let correction_factor = alpha_i - beta;
            
            // r = r + (alpha_i - beta) * s_i
            let correction = vector_scale(s_i, correction_factor)?;
            r = vector_add(&r, &correction)?;
        }

        Ok(r)
    }

    /// Update the L-BFGS state with new gradient and step information.
    pub fn update(&mut self, 
                  new_gradient: &[Tensor], 
                  step_direction: &[Tensor], 
                  step_size: f64) -> CandleResult<()> {
        // Compute parameter difference: s_k = step_size * step_direction
        let s_k = vector_scale(step_direction, step_size)?;

        if let Some(prev_grad) = &self.prev_gradient {
            // Compute gradient difference: y_k = new_gradient - prev_gradient
            let y_k = vector_subtract(new_gradient, prev_grad)?;

            // Compute curvature condition: s_k^T y_k
            let s_dot_y = dot_product(&s_k, &y_k)?;

            // Only update if curvature condition is satisfied (positive definiteness)
            if s_dot_y > self.epsilon() {
                let rho_k = 1.0 / s_dot_y;

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
                    self.gamma = s_dot_y / y_dot_y;
                }
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
pub struct LBFGSOptimizer {
    config: LBFGSConfig,
    state: LBFGSState,
    line_search: Box<dyn LineSearch>,
}

impl LBFGSOptimizer {
    /// Create a new L-BFGS optimizer with the given configuration.
    pub fn new(config: LBFGSConfig) -> Self {
        let line_search = Box::new(StrongWolfeLineSearch::new(config.line_search.clone()));
        let state = LBFGSState::new(config.history_size);

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

    /// Check convergence based on gradient norm.
    fn check_convergence(&self, gradient: &[Tensor], tolerance: f64) -> CandleResult<bool> {
        let grad_norm = compute_magnitude(gradient)?;
        Ok(grad_norm < tolerance)
    }

    /// Compute convergence information for the current state.
    fn compute_convergence_info(&self, gradient: &[Tensor]) -> CandleResult<ConvergenceInfo> {
        let gradient_norm = compute_magnitude(gradient)?;
        
        Ok(ConvergenceInfo {
            gradient_norm,
            converged: gradient_norm < 1e-6, // Default tolerance
            stagnation_detected: false, // Could be enhanced with step size monitoring
            max_iterations_reached: false, // Handled by caller
        })
    }
}

impl Optimizer for LBFGSOptimizer {
    type Config = LBFGSConfig;
    type State = LBFGSState;

    fn new(config: Self::Config) -> Self {
        Self::new(config)
    }

    fn step(&mut self, 
            params: &mut [Tensor], 
            gradients: &[Tensor]) -> CandleResult<StepResult> {
        // Compute L-BFGS search direction
        let search_direction = self.state.compute_direction(gradients)?;

        // Perform line search to find optimal step size
        let line_search_result = self.line_search.search(
            params,
            &search_direction,
            gradients,
        )?;

        // Update parameters: x_{k+1} = x_k + alpha * p_k
        for (param, direction) in params.iter_mut().zip(&search_direction) {
            let step = direction.mul_scalar(line_search_result.step_size)?;
            *param = param.add(&step)?;
        }

        // Update L-BFGS state with new information
        self.state.update(gradients, &search_direction, line_search_result.step_size)?;

        // Compute convergence information
        let convergence_info = self.compute_convergence_info(gradients)?;

        Ok(StepResult {
            step_size: line_search_result.step_size,
            function_evaluations: line_search_result.function_evaluations,
            gradient_evaluations: line_search_result.gradient_evaluations,
            convergence_info,
        })
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
    use crate::utils::math::*;
    use candle_core::Device;
    use approx::assert_relative_eq;

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
        let state = LBFGSState::new(5);
        
        let gradient = vec![
            Tensor::from_slice(&[1.0, 2.0], &[2], &device)?,
        ];

        let direction = state.compute_direction(&gradient)?;
        
        // Should return negative gradient (steepest descent)
        let expected = vec![
            Tensor::from_slice(&[-1.0, -2.0], &[2], &device)?,
        ];

        assert_relative_eq!(
            direction[0].to_vec2::<f64>()?[0][0],
            expected[0].to_vec2::<f64>()?[0][0],
            epsilon = 1e-10
        );
        assert_relative_eq!(
            direction[0].to_vec2::<f64>()?[0][1],
            expected[0].to_vec2::<f64>()?[0][1],
            epsilon = 1e-10
        );

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
            let grad = vec![Tensor::from_slice(&[1.0 + i as f64 * 0.1, 1.0], &[2], &device)?];
            let direction = vec![Tensor::from_slice(&[-1.0, -1.0], &[2], &device)?];
            state.update(&grad, &direction, 0.1)?;
        }
        
        // Should maintain only the history size limit
        assert!(state.history_length() <= 2);

        Ok(())
    }
}