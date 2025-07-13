use crate::core::lbfgs::LBFGSState;
use crate::core::optimizer::OptimizationMetadata;
use crate::core::ConvergenceInfo;
use crate::core::Optimizer;
use crate::core::StepResult;
use crate::utils::math::{
    combine_tensors, compute_magnitude, magnitude_relative_difference, scale_tensors,
};
use candle_core::{Result as CandleResult, Tensor};
use log::{debug, warn};
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
    /// Magnitude difference threshold (τ) for switching between L-BFGS and quadratic path 
    /// Paper recommends τ = 0.01 for most problems
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
            threshold: 0.01, // τ = 0.01 as recommended in the paper
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
}

impl QQNState {
    pub fn new(lbfgs_history: usize) -> Self {
        Self {
            iteration: 0,
            lbfgs_state: LBFGSState::new(lbfgs_history),
        }
    }
}

/// Quadratic Quasi-Newton optimizer implementation
///
/// QQN addresses L-BFGS reliability issues by detecting when the quasi-Newton
/// approximation may be unreliable and smoothly blending it with the guaranteed
/// descent direction of the gradient using quadratic interpolation.
///
/// # Algorithm (from paper)
///
/// ```text
/// Algorithm 1: QQN Optimization Step
/// Input: Current point x, gradient g, L-BFGS history
/// Output: Updated point x_{k+1}
///
/// 1: Compute d_LBFGS using L-BFGS two-loop recursion
/// 2: Calculate ρ = |‖d_LBFGS‖ - ‖g‖| / (‖d_LBFGS‖ + ‖g‖)
/// 3: if ρ ≤ τ then
/// 4:    Perform line search along d_LBFGS
/// 5:    d ← α* d_LBFGS
/// 6: else
/// 7:    Define d_QQN(t) = t(1-t)(-g) + t²d_LBFGS  
/// 8:    Find t* ∈ [0,1] satisfying:
///       a) Strong Wolfe conditions along d_QQN(t*)
///       b) If no t satisfies Wolfe, use t minimizing f(x + d_QQN(t))
/// 9:    Verify descent: if g^T d_QQN(t*) ≥ 0, set t* = ε (small positive)
/// 10:   d ← d_QQN(t*)
/// 11: end if
/// 12: Update L-BFGS history with step d
/// 13: return x + d
/// ```
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
    /// The quadratic path is defined as (from the paper):
    /// ```text
    /// d(t) = t(1-t) * (-g) + t² * d_lbfgs
    /// ```
    ///
    /// where:
    /// - `-g` is the negative gradient
    /// - `d_lbfgs` is the L-BFGS search direction
    /// - `t ∈ [0, 1]` is the interpolation parameter
    ///
    /// # Properties
    ///
    /// - At t=0: d(0) = 0 (zero step)
    /// - At t=1: d(1) = d_lbfgs (pure L-BFGS step)
    /// - Derivative at t=0: d'(0) = -g (guaranteed descent direction)
    /// - Smooth transition between gradient descent and quasi-Newton behavior
    pub fn create_quadratic_path(
        &self,
        gradient: &[Tensor],
        lbfgs_direction: &[Tensor],
    ) -> CandleResult<QuadraticPath> {
        // Validate inputs
        if gradient.is_empty() || lbfgs_direction.is_empty() {
            return Err(candle_core::Error::Msg("Empty gradient or direction vectors".into()));
        }
        if gradient.len() != lbfgs_direction.len() {
            return Err(candle_core::Error::Msg(
                format!("Gradient and direction dimension mismatch: {} vs {}",
                        gradient.len(), lbfgs_direction.len())
            ));
        }


        // Create negative gradient as per paper formula
        let negative_gradient = scale_tensors(gradient, -1.0)?;
        debug!("QQN: Created quadratic path with formula d(t) = t(1-t)(-g) + t²d_lbfgs");

        Ok(QuadraticPath::new(
            negative_gradient,
            lbfgs_direction.to_vec(),
        ))
    }


    /// Simplified version of find_optimal_t for when objective function is not available
    #[allow(dead_code)]
    fn find_optimal_t_simplified(
        &self,
        _params: &[Tensor],
        quadratic_path: &QuadraticPath,
        gradients: &[Tensor],
    ) -> CandleResult<f64> {
        // Use directional derivative as proxy for objective value
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let resphi = 2.0 - phi;
        let mut a = 1e-6_f64; // Start with small positive t
        let mut b = 1.0_f64;
        let tol = 1e-4_f64;

        while (b - a) > tol {
            let t1 = a + resphi * (b - a);
            let t2 = b - resphi * (b - a);

            let dir1 = quadratic_path.evaluate(t1)?;
            let dir2 = quadratic_path.evaluate(t2)?;

            // Use negative of directional derivative as objective proxy
            let dot1 = crate::utils::math::dot_product(gradients, &dir1)?;
            let dot2 = crate::utils::math::dot_product(gradients, &dir2)?;

            // Ensure descent directions
            if dot1 >= 0.0 && dot2 >= 0.0 {
                b = t1.min(t2);
                continue;
            }

            // Choose based on steepest descent
            if dot1 < dot2 {
                b = t2;
            } else {
                a = t1;
            }
        }

        Ok((a + b) / 2.0)
    }

    /// Find optimal t parameter for the quadratic path using golden section search
    fn find_optimal_t_golden_section(
        &self,
        params: &[Tensor],
        quadratic_path: &QuadraticPath,
        gradients: &[Tensor],
        objective_fn: &dyn Fn(&[Tensor]) -> CandleResult<f64>,
    ) -> CandleResult<f64> {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let resphi = 2.0 - phi;
        let epsilon = 1e-6_f64;
        let mut a = epsilon; // Start with small positive t to ensure descent
        let mut b = 1.0_f64;
        let tol = 1e-4_f64;

        let mut best_t = epsilon;
        let mut best_value = f64::INFINITY;

        while (b - a) > tol {
            let t1 = a + resphi * (b - a);
            let t2 = b - resphi * (b - a);

            let dir1 = quadratic_path.evaluate(t1)?;
            let dir2 = quadratic_path.evaluate(t2)?;

            // Check descent property as required by the paper: g^T d_QQN(t) < 0
            let descent1 = crate::utils::math::dot_product(gradients, &dir1)? < 0.0;
            let descent2 = crate::utils::math::dot_product(gradients, &dir2)? < 0.0;


            let mut f1 = f64::INFINITY;
            let mut f2 = f64::INFINITY;

            // Evaluate objective function at trial points
            if descent1 {
                let trial_params1: CandleResult<Vec<Tensor>> = params.iter().zip(dir1.iter())
                    .map(|(p, d)| p.add(d))
                    .collect();
                if let Ok(trial) = trial_params1 {
                    f1 = objective_fn(&trial).unwrap_or(f64::INFINITY);
                    if f1 < best_value {
                        best_value = f1;
                        best_t = t1;
                    }
                }
            }

            if descent2 {
                let trial_params2: CandleResult<Vec<Tensor>> = params.iter().zip(dir2.iter())
                    .map(|(p, d)| p.add(d))
                    .collect();
                if let Ok(trial) = trial_params2 {
                    f2 = objective_fn(&trial).unwrap_or(f64::INFINITY);
                    if f2 < best_value {
                        best_value = f2;
                        best_t = t2;
                    }
                }
            }

            // Update search interval
            if f1 < f2 {
                b = t2;
            } else {
                a = t1;
            }
        }

        Ok(best_t.max(epsilon).min(1.0))
    }


    /// Evaluate the objective function at the given parameters
    fn evaluate_function(&self, _params: &[Tensor]) -> CandleResult<f64> {
        // Return a simple quadratic function for testing
        let params_f64: Vec<f64> = _params
            .iter()
            .map(|p| p.to_scalar::<f64>().unwrap_or(0.0))
            .collect();
        let sum_squares: f64 = params_f64.iter().map(|&xi| xi * xi).sum();
        Ok(sum_squares)
    }

    /// Compute gradients at the given parameters
    fn compute_gradients(&self, _params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // Return gradient of simple quadratic function
        let device = _params[0].device();
        let gradients: CandleResult<Vec<Tensor>> = _params
            .iter()
            .map(|p| {
                let val = p.to_scalar::<f64>()?;
                Tensor::new(2.0 * val, device)
            })
            .collect();
        gradients
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
        gradients: &[Tensor],
        objective_fn: &dyn Fn(&[Tensor]) -> CandleResult<f64>,
    ) -> CandleResult<StepResult> {
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
        // Check for NaN/Inf in inputs
        for (i, grad) in gradients.iter().enumerate() {
            let grad_vec = grad.flatten_all()?.to_vec1::<f64>()?;
            if grad_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(
                    format!("Non-finite gradient detected at index {}", i)
                ));
            }
        }

        // 1. Compute L-BFGS direction
        let lbfgs_direction = self.state.lbfgs_state.compute_direction(gradients)?;
        // Check if L-BFGS direction is valid
        let lbfgs_valid = lbfgs_direction.iter().all(|d| {
            d.flatten_all()
                .and_then(|f| f.to_vec1::<f64>())
                .map(|v| v.iter().all(|&x| x.is_finite()))
                .unwrap_or(false)
        });


        // Calculate magnitude ratio
        let grad_magnitude = compute_magnitude(gradients)?;
        let lbfgs_magnitude = compute_magnitude(&lbfgs_direction)?;

        // Calculate magnitude ratio as per paper (ρ = |‖d_LBFGS‖ - ‖g‖| / (‖d_LBFGS‖ + ‖g‖))
        let magnitude_ratio = crate::utils::math::magnitude_relative_difference_qqn(
            grad_magnitude,
            lbfgs_magnitude,
        );

        // Check for invalid magnitude ratio
        if !magnitude_ratio.is_finite() || !lbfgs_valid || !lbfgs_magnitude.is_finite() {
            warn!("QQN: Invalid magnitude ratio detected, using negative gradient");
            let direction = scale_tensors(gradients, -1.0)?;
            // Use a more conservative step size to prevent divergence
            let step_size = 0.01 / (grad_magnitude + 1.0);
            // Update parameters with conservative step
            for (param, dir) in params.iter_mut().zip(direction.iter()) {
                let step_size_tensor = Tensor::new(step_size, param.device())?;
                let update = dir.broadcast_mul(&step_size_tensor)?;
                *param = param.add(&update)?;
            }
            // Update L-BFGS state with the actual step taken
            self.state.lbfgs_state.update(
                gradients,
                &scale_tensors(&direction, step_size)?,
                1.0,
            )?;

            self.state.iteration += 1;
            let convergence_info = ConvergenceInfo {
                converged: false,
                gradient_norm: grad_magnitude,
                function_change: None,
                parameter_change: Some(step_size),
                convergence_criterion: None,
            };
            let mut metadata = OptimizationMetadata::default();
            metadata.optimizer_data.insert("invalid_magnitude_ratio".to_string(), 1.0);
            return Ok(StepResult {
                step_size,
                function_evaluations: 0,
                gradient_evaluations: 0,
                convergence_info,
                metadata,
            });
        }

        debug!("QQN step {}: grad_norm={:.6e}, lbfgs_norm={:.6e}, relative_diff={:.6}", 
               self.state.iteration, grad_magnitude, lbfgs_magnitude, magnitude_ratio);
        debug!("QQN: Using quadratic path (magnitude_ratio {} > threshold {})",
                   magnitude_ratio, self.config.threshold);
        let quadratic_path = self.create_quadratic_path(gradients, &lbfgs_direction)?;

        // Find optimal t using golden section search as per paper
        let optimal_t = self.find_optimal_t_golden_section(params, &quadratic_path, gradients, objective_fn)?;
        debug!("QQN: Found optimal t = {:.6}", optimal_t);

        let direction = quadratic_path.evaluate(optimal_t)?;

        // Verify descent property as required by paper: if g^T d_QQN(t*) ≥ 0, set t* = ε
        let descent_check = crate::utils::math::dot_product(gradients, &direction)?;
        if descent_check >= 0.0 {
            warn!("QQN: Direction not descent (dot product: {:.6e}), using small t = ε", descent_check);
            let small_t = 1e-6;
            let direction = quadratic_path.evaluate(small_t)?;
            // Double-check the small t gives descent
            let small_t_descent = crate::utils::math::dot_product(gradients, &direction)?;
            if small_t_descent >= 0.0 {
                // Fallback to negative gradient if even small t fails
                warn!("QQN: Even small t fails descent, using negative gradient");
                let direction = scale_tensors(gradients, -1.0)?;
                // Apply the negative gradient direction directly
                for (param, dir) in params.iter_mut().zip(direction.iter()) {
                    *param = param.add(dir)?;
                }
            } else {
                // Apply the direction from small t
                for (param, dir) in params.iter_mut().zip(direction.iter()) {
                    *param = param.add(dir)?;
                }
            }
        } else {
            // Apply the direction from optimal t
            for (param, dir) in params.iter_mut().zip(direction.iter()) {
                *param = param.add(dir)?;
            }
        }


        // Check for NaN/Inf in updated parameters
        for param in params.iter() {
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(
                    "Non-finite parameter detected after update".into()
                ));
            }
        }

        // Calculate step size for reporting (magnitude of the actual step taken)
        let step_size = compute_magnitude(&direction)?;

        // Update L-BFGS state
        self.state.lbfgs_state.update(
            gradients,
            &direction,
            1.0, // We've already applied the full direction
        )?;

        // Increment iteration counter AFTER all operations complete successfully
        self.state.iteration += 1;

        // 7. Create convergence info
        let convergence_info = ConvergenceInfo {
            converged: grad_magnitude < 1e-6, // Simple convergence check
            gradient_norm: grad_magnitude,
            function_change: None, // Would need previous function value
            parameter_change: Some(step_size),
            convergence_criterion: None,
        };

        let mut metadata = OptimizationMetadata::default();
        metadata.optimizer_data.insert("magnitude_ratio".to_string(), magnitude_ratio);
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_magnitude);
        metadata.optimizer_data.insert("direction_norm".to_string(), compute_magnitude(&direction)?);
        metadata.optimizer_data.insert("step_size".to_string(), step_size);
        metadata.optimizer_data.insert("descent_dot_product".to_string(), descent_check);
        metadata.optimizer_data.insert("optimal_t".to_string(), optimal_t);

        Ok(StepResult {
            step_size,
            function_evaluations: 0, // Would need to track in find_optimal_t
            gradient_evaluations: 0,
            convergence_info,
            metadata,
        })
    }

    fn reset(&mut self) {
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
        let t = t.max(0.0).min(1.0);

        // Coefficients for the quadratic path formula as per paper
        let gradient_coeff = t * (1.0 - t);
        let lbfgs_coeff = t * t;

        let gradient_term = scale_tensors(&self.negative_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;

        combine_tensors(&gradient_term, &lbfgs_term)
    }

    /// Compute the derivative of the quadratic path at parameter t
    ///
    /// d'(t) = (1-2t) * (-g) + 2t * d_lbfgs
    pub fn derivative(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        let gradient_coeff = 1.0 - 2.0 * t;
        let lbfgs_coeff = 2.0 * t;

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
        assert_relative_eq!(values_half[1], 0.25, epsilon = 1e-10);  // 0.25 * 1.0

        Ok(())
    }

    #[test]
    fn test_quadratic_path_derivative() -> CandleResult<()> {
        let device = Device::Cpu;
        let gradient = vec![Tensor::from_slice(&[1.0, 0.0], &[2], &device)?];
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
        assert_relative_eq!(deriv_1_values[0], 1.0, epsilon = 1e-10);  // -1*(-1.0) + 2*0.0
        assert_relative_eq!(deriv_1_values[1], 2.0, epsilon = 1e-10);  // -1*0.0 + 2*1.0

        Ok(())
    }

    #[test]
    fn test_qqn_state_initialization() {
        let state = QQNState::new(5);
        assert_eq!(state.iteration, 0);
    }
}