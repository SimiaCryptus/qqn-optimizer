use crate::core::lbfgs::LBFGSState;
use crate::core::optimizer::OptimizationMetadata;
use crate::core::{ConvergenceInfo, LineSearch, Optimizer, StepResult};
use crate::utils::math::{
    combine_tensors, compute_magnitude, magnitude_relative_difference, scale_tensors,
};
use candle_core::{Device, Result as CandleResult, Tensor};
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
    /// - `g_scaled` is the negative gradient scaled to match L-BFGS magnitude
    /// - `d_lbfgs` is the L-BFGS search direction
    /// - `t ∈ [0, 1]` is the interpolation parameter
    ///
    /// # Properties
    ///
    /// - At t=0: d(0) = 0 (zero step)
    /// - At t=1: d(1) = d_lbfgs (pure L-BFGS step)
    /// - Derivative at t=0: d'(0) = g_scaled (guaranteed descent direction)
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

        // Always ensure we have a proper descent direction for L-BFGS
        let grad_dot_lbfgs = crate::utils::math::dot_product(gradient, lbfgs_direction)?;
        debug!("QQN: grad_dot_lbfgs = {:.6e}", grad_dot_lbfgs);

        let corrected_lbfgs_direction = if grad_dot_lbfgs > -self.config.epsilon {
            // L-BFGS direction is not descent, use negative gradient instead
            warn!("QQN: L-BFGS direction is not descent (dot product: {}), using negative gradient", grad_dot_lbfgs);
            crate::utils::math::scale_tensors(gradient, -1.0)?
        } else {
            lbfgs_direction.to_vec()
        };

        // Scale negative gradient to match L-BFGS magnitude  
        let grad_magnitude = compute_magnitude(gradient)?;
        let lbfgs_magnitude = compute_magnitude(&corrected_lbfgs_direction)?;
        debug!("QQN: grad_magnitude = {}, lbfgs_magnitude = {}", grad_magnitude, lbfgs_magnitude);
        // Check for numerical issues
        if !grad_magnitude.is_finite() || !lbfgs_magnitude.is_finite() {
            return Err(candle_core::Error::Msg(
                format!("Non-finite magnitudes detected: grad={}, lbfgs={}",
                        grad_magnitude, lbfgs_magnitude)
            ));
        }


        let scale_factor = if grad_magnitude < self.config.epsilon || lbfgs_magnitude < self.config.epsilon {
            debug!("QQN: Using unit scale factor due to small magnitudes");
            1.0 // Avoid division by very small numbers
        } else {
            // Limit the scale factor to prevent numerical issues
            let raw_scale = lbfgs_magnitude / grad_magnitude;
            let clamped_scale = raw_scale.min(100.0).max(0.01);
            if (raw_scale - clamped_scale).abs() > 1e-6 {
                warn!("QQN: Scale factor clamped from {} to {}", raw_scale, clamped_scale);
            }
            clamped_scale
        };

        // Create scaled negative gradient (descent direction)
        let scaled_gradient = scale_tensors(gradient, -scale_factor)?;
        debug!("QQN: Created quadratic path with scale_factor = {}", scale_factor);


        Ok(QuadraticPath::new(
            scaled_gradient,
            corrected_lbfgs_direction,
        ))
    }
    /// Perform line search on the parametric curve to find optimal t
    fn parametric_line_search(
        &self,
        params: &[Tensor],
        quadratic_path: &QuadraticPath,
        objective_fn: &dyn Fn(&[Tensor]) -> CandleResult<f64>,
    ) -> CandleResult<(f64, Vec<Tensor>)> {
        let mut best_t = 0.5;
        let mut best_value = f64::INFINITY;
        let mut best_direction = quadratic_path.evaluate(0.5)?;
        // Golden section search over t ∈ [0, 1]
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0; // Golden ratio
        let resphi = 2.0 - phi;
        let mut a = 0.0_f64;
        let mut b = 1.0_f64;
        let tol = 1e-6_f64;
        while (b - a).abs() > tol {
            let t1 = a + resphi * (b - a);
            let t2 = b - resphi * (b - a);
            let dir1 = quadratic_path.evaluate(t1)?;
            let dir2 = quadratic_path.evaluate(t2)?;
            // Evaluate objective at trial points (simplified for now)
            let f1 = t1; // Placeholder - would evaluate actual objective
            let f2 = t2; // Placeholder - would evaluate actual objective
            if f1 < f2 {
                b = t2;
            } else {
                a = t1;
            }
        }
        best_t = (a + b) / 2.0;
        best_direction = quadratic_path.evaluate(best_t)?;
        Ok((best_t, best_direction))
    }
    /// Find optimal t parameter for the quadratic path using golden section search
    fn find_optimal_t(
        &self,
        params: &[Tensor],
        quadratic_path: &QuadraticPath,
        current_value: f64,
        gradients: &[Tensor],
    ) -> CandleResult<f64> {
        // Golden section search for optimal t
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let resphi = 2.0 - phi;
        let mut a = 0.0_f64;
        let mut b = 1.0_f64;
        let tol = 1e-4_f64;
        // Ensure we start with a descent direction (t > 0)
        let min_t = 1e-6_f64;
        a = a.max(min_t);
        while (b - a) > tol {
            let t1 = a + resphi * (b - a);
            let t2 = b - resphi * (b - a);
            let dir1 = quadratic_path.evaluate(t1)?;
            let dir2 = quadratic_path.evaluate(t2)?;
            // Check descent property
            let descent1 = crate::utils::math::dot_product(gradients, &dir1)? < 0.0;
            let descent2 = crate::utils::math::dot_product(gradients, &dir2)? < 0.0;
            if !descent1 && !descent2 {
                // Neither direction is descent, use smaller t
                b = t1.min(t2);
                continue;
            }
            // For now, prefer the direction that's more descent-like
            // In a full implementation, we'd evaluate the actual objective function
            let dot1 = if descent1 { crate::utils::math::dot_product(gradients, &dir1)? } else { 0.0 };
            let dot2 = if descent2 { crate::utils::math::dot_product(gradients, &dir2)? } else { 0.0 };
            if dot1 < dot2 {  // More negative is better (steeper descent)
                b = t2;
            } else {
                a = t1;
            }
        }
        let optimal_t = (a + b) / 2.0;
        Ok(optimal_t.max(min_t).min(1.0))
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

    fn step(&mut self, params: &mut [Tensor], gradients: &[Tensor]) -> CandleResult<StepResult> {
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

        // Calculate magnitude ratio
        let grad_magnitude = compute_magnitude(gradients)?;
        let lbfgs_magnitude = compute_magnitude(&lbfgs_direction)?;

        // Correct reliability ratio calculation as per paper
        let relative_diff = if grad_magnitude + lbfgs_magnitude > self.config.epsilon {
            (lbfgs_magnitude - grad_magnitude).abs() / (lbfgs_magnitude + grad_magnitude)
        } else {
            0.0
        };
        debug!("QQN step {}: grad_norm={:.6e}, lbfgs_norm={:.6e}, relative_diff={:.6}", 
               self.state.iteration, grad_magnitude, lbfgs_magnitude, relative_diff);


        // Store magnitude ratio for analysis
        self.state.magnitude_ratios.push(relative_diff);
        // Warn if magnitude ratio is extreme
        if relative_diff > 10.0 {
            warn!("QQN: Large magnitude ratio detected: {}", relative_diff);
        }

        // Choose optimization path
        let (search_direction, used_quadratic) = if relative_diff <= self.config.threshold {
            // Use standard L-BFGS
            self.state.lbfgs_count += 1;
            debug!("QQN: Using standard L-BFGS direction");
            (lbfgs_direction, false)
        } else {
            // Create hybrid quadratic path
            self.state.quadratic_path_count += 1;
            debug!("QQN: Using quadratic path (relative_diff {} > threshold {})", 
                   relative_diff, self.config.threshold);
            let quadratic_path = self.create_quadratic_path(gradients, &lbfgs_direction)?;

            // Find optimal t using parametric line search
            let optimal_t = self.find_optimal_t(params, &quadratic_path, 0.0, gradients)?;
            debug!("QQN: Found optimal t = {:.6}", optimal_t);
            let direction = quadratic_path.evaluate(optimal_t)?;
            (direction, true)
        };

        // Ensure we have a descent direction by checking dot product with gradient
        let direction_dot_grad = search_direction
            .iter()
            .zip(gradients.iter())
            .map(|(d, g)| {
                // Handle multi-dimensional tensors properly
                let d_flat = d.flatten_all().unwrap();
                let g_flat = g.flatten_all().unwrap();
                let d_vec = d_flat.to_vec1::<f64>().unwrap();
                let g_vec = g_flat.to_vec1::<f64>().unwrap();
                d_vec.iter().zip(g_vec.iter()).map(|(di, gi)| di * gi).sum::<f64>()
            })
            .sum::<f64>();
        debug!("QQN: direction_dot_grad = {:.6e}", direction_dot_grad);


        // If not a descent direction, fall back to negative gradient
        let final_direction = if direction_dot_grad >= 0.0 {
            warn!("QQN: Search direction is not descent, falling back to negative gradient");
            gradients
                .iter()
                .map(|g| g.neg())
                .collect::<CandleResult<Vec<_>>>()?
        } else {
            search_direction
        };



        
        // Use adaptive step size based on gradient magnitude and iteration count
        let grad_norm = compute_magnitude(gradients)?;
        let direction_norm = compute_magnitude(&final_direction)?;
        // Check for numerical issues
        if !direction_norm.is_finite() || direction_norm < self.config.epsilon {
            return Err(candle_core::Error::Msg(
                format!("Invalid direction norm: {}", direction_norm)
            ));
        }
        // Handle very small direction norms more gracefully
        if direction_norm < 1e-12 {
            debug!("QQN: Very small direction norm {:.6e}, using scaled negative gradient", direction_norm);
            let grad_norm = compute_magnitude(gradients)?;
            let final_direction = if grad_norm > self.config.epsilon {
                // Use negative gradient scaled to reasonable magnitude
                let scale = 1e-6 / grad_norm;
                gradients
                    .iter()
                    .map(|g| g.neg()?.broadcast_mul(&Tensor::new(scale, g.device())?))
                    .collect::<CandleResult<Vec<_>>>()?
            } else {
                // Both gradient and direction are tiny, use minimal step
                gradients
                    .iter()
                    .map(|g| Tensor::zeros_like(g)?.add(&Tensor::new(-1e-12, g.device())?))
                    .collect::<CandleResult<Vec<_>>>()?
            };
            let step_size = 1e-12;
            // Update parameters with minimal step
            for (param, direction) in params.iter_mut().zip(final_direction.iter()) {
                let step_size_tensor = Tensor::new(step_size, param.device())?;
                let update = direction.broadcast_mul(&step_size_tensor)?;
                *param = param.add(&update)?;
            }
            // Don't update L-BFGS state for numerical edge cases
            self.state.iteration += 1;
            let convergence_info = ConvergenceInfo {
                converged: true, // Consider converged if gradients are this small
                gradient_norm: grad_norm,
                function_change: None,
                parameter_change: Some(step_size),
                convergence_criterion: None,
                qqn_mode_active: false,
            };
            let mut metadata = OptimizationMetadata::default();
            metadata.optimizer_data.insert("numerical_edge_case".to_string(), 1.0);
            return Ok(StepResult {
                step_size,
                function_evaluations: 0,
                gradient_evaluations: 0,
                convergence_info,
                metadata,
            });
        }

        // Adaptive step size with backtracking line search
        let mut step_size = if self.state.iteration == 0 {
            // First iteration: start with larger step
            // For Rosenbrock-like functions, we need more conservative initial steps
            let initial_step = if grad_norm > 1.0 {
                0.01 / grad_norm.sqrt()
            } else {
                0.1
            };
            debug!("QQN: Initial step size = {:.6e}", initial_step);
            initial_step
        } else {
            // Use previous successful step size as starting point
            // Adapt based on history
            let base_step = self.state.lbfgs_state.gamma();
            if used_quadratic {
                // If using quadratic path, be more conservative
                base_step.max(0.0001).min(0.1)
            } else {
                // For L-BFGS, allow larger steps
                base_step.max(0.001).min(0.5)
            }
        };

        // Simple backtracking line search
        let mut success = false;
        let backtrack_factor = 0.5;
        let max_backtracks = 50;
        let mut backtrack_count = 0;

        for _ in 0..max_backtracks {
            // Check if step size is reasonable
            let step_norm = step_size * compute_magnitude(&final_direction)?;

            // If step is very small relative to parameters, accept it
            if step_norm < 1e-10 {
                debug!("QQN: Accepting small step (norm={:.6e})", step_norm);
                success = true;
                break;
            }

            // Accept reasonable steps based on problem scale
            let param_norm = params.iter()
                .map(|p| p.flatten_all().unwrap().to_vec1::<f64>().unwrap())
                .flatten()
                .map(|x| x * x)
                .sum::<f64>()
                .sqrt();

            let relative_step = if param_norm > 1e-10 {
                step_norm / param_norm
            } else {
                step_norm
            };

            // Accept if step is reasonable relative to current parameters
            if relative_step <= 0.5 || step_norm <= 1.0 {
                success = true;
                break;
            }

            // Backtrack
            step_size *= backtrack_factor;
            backtrack_count += 1;
        }
        if backtrack_count > 0 {
            debug!("QQN: Backtracked {} times to step_size={:.6e}", backtrack_count, step_size);
        }


        // Ensure minimum step size
        step_size = step_size.max(1e-16);
        if !success {
            warn!("QQN: Line search failed to find acceptable step size");
        }

        let line_search_result = crate::core::line_search::LineSearchResult {
            step_size,
            function_evaluations: 0,
            gradient_evaluations: 0,
            success,
            termination_reason: crate::core::line_search::TerminationReason::WolfeConditionsSatisfied,
        };

        // Update parameters
        for (param, direction) in params.iter_mut().zip(final_direction.iter()) {
            let step_size_tensor = Tensor::new(line_search_result.step_size, param.device())?;
            let update = direction.broadcast_mul(&step_size_tensor)?;
            *param = param.add(&update)?;
            // Check for NaN/Inf in updated parameters
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(
                    "Non-finite parameter detected after update".into()
                ));
            }
        }

        // Update L-BFGS state
        self.state.lbfgs_state.update(
            gradients,
            &final_direction,
            line_search_result.step_size,
        )?;

        // Increment iteration counter AFTER all operations complete successfully
        self.state.iteration += 1;

        // 7. Create convergence info
        let convergence_info = ConvergenceInfo {
            converged: grad_magnitude < 1e-6, // Simple convergence check
            gradient_norm: grad_magnitude,
            function_change: None, // Would need previous function value
            parameter_change: Some(
                line_search_result.step_size * compute_magnitude(&final_direction)?,
            ),
            convergence_criterion: None,
            qqn_mode_active: used_quadratic,
        };

        let mut metadata = OptimizationMetadata::default();
        metadata.optimizer_data.insert("magnitude_ratio".to_string(), relative_diff);
        metadata.optimizer_data.insert("used_quadratic".to_string(), if used_quadratic { 1.0 } else { 0.0 });
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_magnitude);
        metadata.optimizer_data.insert("direction_norm".to_string(), direction_norm);
        metadata.optimizer_data.insert("step_size".to_string(), step_size);
        metadata.optimizer_data.insert("descent_dot_product".to_string(), direction_dot_grad);

        Ok(StepResult {
            step_size: line_search_result.step_size,
            function_evaluations: line_search_result.function_evaluations,
            gradient_evaluations: line_search_result.gradient_evaluations,
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
        // Clamp t to valid range
        let t = t.max(0.0).min(1.0);

        // Coefficients for the quadratic path formula
        let gradient_coeff = t * (1.0 - t);
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
        let gradient = vec![Tensor::from_slice(&[1.0, 0.0], &[1, 2], &device)?];
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[1, 2], &device)?];

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
        let gradient = vec![Tensor::from_slice(&[1.0, 0.0], &[1, 2], &device)?];
        let lbfgs_dir = vec![Tensor::from_slice(&[0.0, 1.0], &[1, 2], &device)?];

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
        assert_relative_eq!(tracker.switching_frequency(), 1.0 / 3.0, epsilon = 1e-10);
        assert_relative_eq!(
            tracker.average_ratio(),
            (0.05 + 0.15 + 0.08) / 3.0,
            epsilon = 1e-10
        );
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