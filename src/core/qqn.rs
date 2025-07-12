use crate::core::lbfgs::LBFGSState;
use crate::core::optimizer::OptimizationMetadata;
use crate::core::{ConvergenceInfo, LineSearch, Optimizer, StepResult, StrongWolfeLineSearch};
use crate::utils::math::{compute_magnitude, magnitude_relative_difference, scale_tensors, combine_tensors};
use candle_core::{Device, Result as CandleResult, Tensor};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
/// QQN trace information for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QQNTrace {
    pub magnitude_ratios: Vec<f64>,
    pub quadratic_path_usage: Vec<bool>,
    pub step_sizes: Vec<f64>,
}
impl QQNTrace {
    pub fn new() -> Self {
        Self {
            magnitude_ratios: Vec::new(),
            quadratic_path_usage: Vec::new(),
            step_sizes: Vec::new(),
        }
    }
}

/// Configuration for the QQN optimizer
#[derive(Debug, Clone)]
pub struct QQNConfig {
    /// Magnitude difference threshold (τ) for switching between L-BFGS and quadratic path
    pub threshold: f64,
    /// L-BFGS history length
    pub lbfgs_history: usize,
    /// Line search configuration
    pub line_search: crate::core::line_search::StrongWolfeConfig,
    /// Numerical stability constant
    pub epsilon: f64,
}

impl Default for QQNConfig {
    fn default() -> Self {
        Self {
            threshold: 0.01,
            lbfgs_history: 10,
            line_search: crate::core::line_search::StrongWolfeConfig::default(),
            epsilon: 1e-8,
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
    /// History of magnitude ratios
    pub magnitude_ratios: Vec<f64>,
    /// Number of times quadratic path was used
    pub quadratic_path_count: usize,
    /// Number of times standard L-BFGS was used
    pub lbfgs_count: usize,
}

impl QQNState {
    pub fn new(lbfgs_history: usize) -> Self {
        Self {
            iteration: 0,
            lbfgs_state: LBFGSState::new(lbfgs_history),
            magnitude_ratios: Vec::new(),
            quadratic_path_count: 0,
            lbfgs_count: 0,
        }
    }
}

/// Quadratic Quasi-Newton optimizer implementation
/// 
/// QQN addresses L-BFGS reliability issues by detecting when the quasi-Newton
/// approximation may be unreliable and smoothly blending it with the guaranteed
/// descent direction of the gradient using quadratic interpolation.
/// 
/// # Algorithm Overview
/// 
/// 1. Compute L-BFGS search direction
/// 2. Compare magnitudes of L-BFGS direction and gradient
/// 3. If magnitude difference exceeds threshold, create hybrid quadratic path
/// 4. Perform line search on the chosen path
/// 5. Update parameters and L-BFGS history
#[derive(Debug, Clone)]
pub struct QQNOptimizer {
    config: QQNConfig,
    state: QQNState,
}

impl QQNOptimizer {
    /// Create a new QQN optimizer with the given configuration
    pub fn new(config: QQNConfig) -> Self {
        
        Self {
            state: QQNState::new(config.lbfgs_history),
            config,
        }
    }

    /// Computes the quadratic interpolation path between scaled gradient and L-BFGS directions.
    ///
    /// The quadratic path is defined as:
    /// ```text
    /// d(t) = t(1-t) * g_scaled + t² * d_lbfgs
    /// ```
    ///
    /// where:
    /// - `g_scaled` is the gradient scaled to match L-BFGS magnitude
    /// - `d_lbfgs` is the L-BFGS search direction
    /// - `t ∈ [0, 1]` is the interpolation parameter
    ///
    /// # Properties
    ///
    /// - At t=0: d(0) = 0 (zero step)
    /// - At t=1: d(1) = d_lbfgs (pure L-BFGS step)
    /// - Derivative at t=0: d'(0) = g_scaled (gradient descent direction)
    /// - Smooth transition between gradient descent and quasi-Newton behavior
    fn create_quadratic_path(&self,
                           gradient: &[Tensor],
                           lbfgs_direction: &[Tensor]) -> CandleResult<QuadraticPath> {
        // Scale gradient to match L-BFGS magnitude
        let grad_magnitude = compute_magnitude(gradient)?;
        let lbfgs_magnitude = compute_magnitude(lbfgs_direction)?;
        let scale_factor = lbfgs_magnitude / grad_magnitude.max(self.config.epsilon);

        let scaled_gradient = scale_tensors(gradient, -scale_factor)?; // Negative for descent

        Ok(QuadraticPath::new(scaled_gradient, lbfgs_direction.to_vec()))
    }

    /// Evaluate the objective function at the given parameters
    /// This is a placeholder - in practice, this would be provided by the problem
    fn evaluate_function(&self, _params: &[Tensor]) -> CandleResult<f64> {
        // This would be provided by the optimization problem
        // For now, return a dummy value
        Ok(0.0)
    }

    /// Compute gradients at the given parameters
    /// This is a placeholder - in practice, this would be provided by the problem
    fn compute_gradients(&self, _params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // This would be provided by the optimization problem
        // For now, return empty vector
        Ok(Vec::new())
    }
}

impl Optimizer for QQNOptimizer {
    type Config = QQNConfig;
    type State = QQNState;

    fn new(config: Self::Config) -> Self {
        Self::new(config)
    }


    fn step(&mut self,
           params: &mut [Tensor],
           gradients: &[Tensor]) -> CandleResult<StepResult> {
        
        // 1. Compute L-BFGS direction
        let lbfgs_direction = self.state.lbfgs_state.compute_direction(gradients)?;

      // Calculate magnitude ratio
        let grad_magnitude = compute_magnitude(gradients)?;
        let lbfgs_magnitude = compute_magnitude(&lbfgs_direction)?;
        let relative_diff = magnitude_relative_difference(lbfgs_magnitude, grad_magnitude);

        // Store magnitude ratio for analysis
        self.state.magnitude_ratios.push(relative_diff);

      // Choose optimization path
      let (search_direction, used_quadratic) = if relative_diff <= self.config.threshold {
            // Use standard L-BFGS
            self.state.lbfgs_count += 1;
            (lbfgs_direction, false)
        } else {
            // Create hybrid quadratic path
            self.state.quadratic_path_count += 1;
            let quadratic_path = self.create_quadratic_path(gradients, &lbfgs_direction)?;
            // For simplicity, use t=0.5 as the interpolation parameter
            // In a full implementation, this could be optimized
            let direction = quadratic_path.evaluate(0.5)?;
            (direction, true)
        };

        // 4. Perform line search
        let line_search = StrongWolfeLineSearch::new(self.config.line_search.clone());
        
        // Convert tensors to f64 vectors for line search
        let current_point: Vec<f64> = params.iter()
            .map(|p| p.to_scalar::<f64>().unwrap_or(0.0))
            .collect();
        let direction_vec: Vec<f64> = search_direction.iter()
            .map(|d| d.to_scalar::<f64>().unwrap_or(0.0))
            .collect();
        let gradient_vec: Vec<f64> = gradients.iter()
            .map(|g| g.to_scalar::<f64>().unwrap_or(0.0))
            .collect();
        
        let current_value = self.evaluate_function(params)?;
        
        let objective_fn = |_x: &[f64]| -> anyhow::Result<f64> { Ok(0.0) };
        let gradient_fn = |_x: &[f64]| -> anyhow::Result<Vec<f64>> { Ok(vec![0.0; gradient_vec.len()]) };
        
        let mut line_search_mut = line_search;
        let line_search_result = line_search_mut.search(
            &current_point,
            &direction_vec,
            current_value,
            &gradient_vec,
            &objective_fn,
            &gradient_fn,
        ).map_err(|e| candle_core::Error::Msg(format!("Line search failed: {}", e)))?;

      // Update parameters
        for (param, direction) in params.iter_mut().zip(search_direction.iter()) {
            let step_size_tensor = Tensor::new(line_search_result.step_size, param.device())?;
            let update = direction.broadcast_mul(&step_size_tensor)?;
            *param = param.add(&update)?;
        }

      // Update L-BFGS state
        self.state.lbfgs_state.update(gradients, &search_direction, line_search_result.step_size)?;
        self.state.iteration += 1;

        // 7. Create convergence info
        let convergence_info = ConvergenceInfo {
            converged: grad_magnitude < 1e-6, // Simple convergence check
            gradient_norm: grad_magnitude,
            function_change: None, // Would need previous function value
            parameter_change: Some(line_search_result.step_size * compute_magnitude(&search_direction)?),
            convergence_criterion: None,
        };

        Ok(StepResult {
            step_size: line_search_result.step_size,
            function_evaluations: line_search_result.function_evaluations,
            gradient_evaluations: line_search_result.gradient_evaluations,
            convergence_info,
            metadata: OptimizationMetadata::default(),
        })
    }

    fn reset(&mut self) {
        self.state = QQNState::new(self.config.lbfgs_history);
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
    scaled_gradient: Vec<Tensor>,
    lbfgs_direction: Vec<Tensor>,
}

impl QuadraticPath {
    /// Create a new quadratic path
    pub fn new(scaled_gradient: Vec<Tensor>, lbfgs_direction: Vec<Tensor>) -> Self {
        Self {
            scaled_gradient,
            lbfgs_direction,
        }
    }

    /// Evaluate the quadratic path at parameter t ∈ [0, 1]
    /// 
    /// d(t) = t(1-t) * g_scaled + t² * d_lbfgs
    pub fn evaluate(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        let t_complement = 1.0 - t;
        let gradient_coeff = t * t_complement;
        let lbfgs_coeff = t * t;

        let gradient_term = scale_tensors(&self.scaled_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;
        
        combine_tensors(&gradient_term, &lbfgs_term)
    }

    /// Compute the derivative of the quadratic path at parameter t
    /// 
    /// d'(t) = (1-2t) * g_scaled + 2t * d_lbfgs
    pub fn derivative(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        let gradient_coeff = 1.0 - 2.0 * t;
        let lbfgs_coeff = 2.0 * t;

        let gradient_term = scale_tensors(&self.scaled_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;
        
        combine_tensors(&gradient_term, &lbfgs_term)
    }

    /// Get the scaled gradient component
    pub fn scaled_gradient(&self) -> &[Tensor] {
        &self.scaled_gradient
    }

    /// Get the L-BFGS direction component
    pub fn lbfgs_direction(&self) -> &[Tensor] {
        &self.lbfgs_direction
    }
}

/// Tracker for magnitude differences and QQN behavior analysis
#[derive(Debug, Clone)]
pub struct MagnitudeTracker {
    /// History of magnitude ratios
    pub ratios: Vec<f64>,
    /// Threshold used for switching
    pub threshold: f64,
    /// Statistics about switching behavior
    pub switch_count: usize,
    pub lbfgs_count: usize,
}

impl MagnitudeTracker {
    pub fn new(threshold: f64) -> Self {
        Self {
            ratios: Vec::new(),
            threshold,
            switch_count: 0,
            lbfgs_count: 0,
        }
    }

    pub fn record_ratio(&mut self, ratio: f64) {
        self.ratios.push(ratio);
        if ratio > self.threshold {
            self.switch_count += 1;
        } else {
            self.lbfgs_count += 1;
        }
    }

    pub fn switching_frequency(&self) -> f64 {
        if self.ratios.is_empty() {
            0.0
        } else {
            self.switch_count as f64 / self.ratios.len() as f64
        }
    }

    pub fn average_ratio(&self) -> f64 {
        if self.ratios.is_empty() {
            0.0
        } else {
            self.ratios.iter().sum::<f64>() / self.ratios.len() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use candle_core::Device;

    #[test]
    fn test_qqn_config_default() {
        let config = QQNConfig::default();
        assert_eq!(config.threshold, 0.01);
        assert_eq!(config.lbfgs_history, 10);
        assert_eq!(config.epsilon, 1e-8);
    }

    #[test]
    fn test_quadratic_path_evaluation() -> CandleResult<()> {
        let device = Device::Cpu;
        let gradient = vec![Tensor::from_slice(&[1.0, 0.0], &[2], &device)?];
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[2], &device)?];

        let path = QuadraticPath::new(gradient, lbfgs_dir);

        // At t=0, should be zero vector
        let result_0 = path.evaluate(0.0)?;
        assert_relative_eq!(result_0[0].to_vec2::<f64>()?[0][0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result_0[0].to_vec2::<f64>()?[0][1], 0.0, epsilon = 1e-10);

        // At t=1, should be L-BFGS direction
        let result_1 = path.evaluate(1.0)?;
        assert_relative_eq!(result_1[0].to_vec2::<f64>()?[0][0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result_1[0].to_vec2::<f64>()?[0][1], 1.0, epsilon = 1e-10);

        Ok(())
    }

    #[test]
    fn test_quadratic_path_derivative() -> CandleResult<()> {
        let device = Device::Cpu;
        let gradient = vec![Tensor::from_slice(&[1.0, 0.0], &[2], &device)?];
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[2], &device)?];

        let path = QuadraticPath::new(gradient, lbfgs_dir);

        // At t=0, derivative should be gradient
        let deriv_0 = path.derivative(0.0)?;
        assert_relative_eq!(deriv_0[0].to_vec2::<f64>()?[0][0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(deriv_0[0].to_vec2::<f64>()?[0][1], 0.0, epsilon = 1e-10);

        // At t=1, derivative should be 2 * L-BFGS direction - gradient
        let deriv_1 = path.derivative(1.0)?;
        assert_relative_eq!(deriv_1[0].to_vec2::<f64>()?[0][0], -1.0, epsilon = 1e-10);
        assert_relative_eq!(deriv_1[0].to_vec2::<f64>()?[0][1], 2.0, epsilon = 1e-10);

        Ok(())
    }

    #[test]
    fn test_magnitude_tracker() {
        let mut tracker = MagnitudeTracker::new(0.1);
        
        tracker.record_ratio(0.05); // Below threshold
        tracker.record_ratio(0.15); // Above threshold
        tracker.record_ratio(0.08); // Below threshold
        
        assert_eq!(tracker.switch_count, 1);
        assert_eq!(tracker.lbfgs_count, 2);
        assert_relative_eq!(tracker.switching_frequency(), 1.0/3.0, epsilon = 1e-10);
        assert_relative_eq!(tracker.average_ratio(), (0.05 + 0.15 + 0.08) / 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_qqn_state_initialization() {
        let state = QQNState::new(5);
        assert_eq!(state.iteration, 0);
        assert_eq!(state.magnitude_ratios.len(), 0);
        assert_eq!(state.quadratic_path_count, 0);
        assert_eq!(state.lbfgs_count, 0);
    }
}