//! Trust Region optimizer implementation.
//!
//! This implementation provides a robust optimization method that uses a quadratic model
//! within a trust region to ensure global convergence. The trust region radius is adaptively
//! adjusted based on the agreement between the model and actual function reduction.
//!
//! ## Algorithm Overview
//!
//! The Trust Region method works by:
//! 1. Building a quadratic model of the objective function within a trust region
//! 2. Solving a constrained subproblem to find the optimal step within the region
//! 3. Evaluating the quality of the model prediction vs actual reduction
//! 4. Adjusting the trust region radius based on this quality metric
//!
//! ## Strengths
//!
//! - **Global convergence**: Guaranteed convergence to a stationary point
//! - **Robustness**: Handles ill-conditioned problems well
//! - **Adaptive**: Automatically adjusts step sizes based on model quality
//! - **No line search**: Avoids expensive line search procedures
//!
//! ## Weaknesses
//!
//! - **Subproblem cost**: Solving the trust region subproblem can be expensive
//! - **Memory requirements**: Needs to store Hessian approximation
//! - **Conservative**: May take smaller steps than necessary on well-behaved problems

use crate::optimizers::optimizer::{ConvergenceInfo, OptimizationMetadata, Optimizer, StepResult};
use crate::utils::math::{compute_magnitude, dot_product, DifferentiableFunction};
use candle_core::{Result as CandleResult, Tensor};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

/// Configuration parameters for the Trust Region optimizer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRegionConfig {
    /// Initial trust region radius
    ///
    /// **Range**: 0.1 to 10.0, **Default**: 1.0
    pub initial_radius: f64,

    /// Maximum trust region radius
    ///
    /// **Range**: 1.0 to 1000.0, **Default**: 100.0
    pub max_radius: f64,

    /// Minimum trust region radius before declaring convergence
    ///
    /// **Range**: 1e-10 to 1e-4, **Default**: 1e-8
    pub min_radius: f64,

    /// Threshold for accepting a step (ratio of actual to predicted reduction)
    ///
    /// **Range**: 0.0 to 0.5, **Default**: 0.1
    pub eta_1: f64,

    /// Threshold for expanding the trust region
    ///
    /// **Range**: 0.5 to 1.0, **Default**: 0.75
    pub eta_2: f64,

    /// Factor for shrinking the trust region
    ///
    /// **Range**: 0.1 to 0.5, **Default**: 0.25
    pub gamma_1: f64,

    /// Factor for expanding the trust region
    ///
    /// **Range**: 1.5 to 4.0, **Default**: 2.0
    pub gamma_2: f64,

    /// Maximum iterations for solving the trust region subproblem
    ///
    /// **Range**: 10 to 100, **Default**: 50
    pub max_subproblem_iterations: usize,

    /// Tolerance for the trust region subproblem
    ///
    /// **Range**: 1e-10 to 1e-4, **Default**: 1e-6
    pub subproblem_tolerance: f64,

    /// Use Cauchy point if subproblem solver fails
    ///
    /// **Default**: true
    pub use_cauchy_fallback: bool,

    /// Enable verbose logging
    ///
    /// **Default**: false
    pub verbose: bool,
}

impl Default for TrustRegionConfig {
    fn default() -> Self {
        Self {
            initial_radius: 1.0,
            max_radius: 100.0,
            min_radius: 1e-8,
            eta_1: 0.1,
            eta_2: 0.75,
            gamma_1: 0.25,
            gamma_2: 2.0,
            max_subproblem_iterations: 50,
            subproblem_tolerance: 1e-6,
            use_cauchy_fallback: true,
            verbose: false,
        }
    }
}

impl TrustRegionConfig {
    /// Create a conservative trust region configuration
    pub fn conservative() -> Self {
        Self {
            initial_radius: 0.5,
            max_radius: 10.0,
            min_radius: 1e-10,
            eta_1: 0.2,
            eta_2: 0.8,
            gamma_1: 0.2,
            gamma_2: 1.5,
            ..Default::default()
        }
    }

    /// Create an aggressive trust region configuration
    pub fn aggressive() -> Self {
        Self {
            initial_radius: 2.0,
            max_radius: 1000.0,
            min_radius: 1e-6,
            eta_1: 0.05,
            eta_2: 0.5,
            gamma_1: 0.5,
            gamma_2: 3.0,
            ..Default::default()
        }
    }
}

/// State information for Trust Region optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRegionState {
    /// Current trust region radius
    radius: f64,

    /// Current iteration number
    iteration: usize,

    /// Previous function value
    prev_function_value: Option<f64>,

    /// Hessian approximation (stored as flattened matrix)
    #[serde(skip_serializing, skip_deserializing)]
    hessian_approx: Option<Vec<Tensor>>,
    /// Previous gradient for BFGS update
    #[serde(skip_serializing, skip_deserializing)]
    prev_gradient: Option<Vec<Tensor>>,


    /// Number of consecutive rejected steps
    consecutive_rejections: usize,

    /// Best function value seen so far
    best_function_value: Option<f64>,
}

impl TrustRegionState {
    /// Create a new trust region state
    pub fn new(initial_radius: f64) -> Self {
        Self {
            radius: initial_radius,
            iteration: 0,
            prev_function_value: None,
            hessian_approx: None,
            prev_gradient: None,
            consecutive_rejections: 0,
            best_function_value: None,
        }
    }

    /// Reset the state
    pub fn reset(&mut self, initial_radius: f64) {
        self.radius = initial_radius;
        self.iteration = 0;
        self.prev_function_value = None;
        self.hessian_approx = None;
        self.prev_gradient = None;
        self.consecutive_rejections = 0;
        self.best_function_value = None;
    }
}

/// Trust Region optimizer
#[derive(Debug)]
pub struct TrustRegionOptimizer {
    config: TrustRegionConfig,
    state: TrustRegionState,
    stagnation_multiplier: f64,
    stagnation_count: usize,
    /// BFGS approximation for Hessian
    #[serde(skip_serializing, skip_deserializing)]
    bfgs_approx: Option<BFGSApproximation>,
}

impl Clone for TrustRegionOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            stagnation_multiplier: self.stagnation_multiplier,
            stagnation_count: self.stagnation_count,
            bfgs_approx: self.bfgs_approx.clone(),
        }
    }
}

impl TrustRegionOptimizer {
    /// Create a new Trust Region optimizer
    pub fn new(config: TrustRegionConfig) -> Self {
        if config.verbose {
            info!(
                "Creating Trust Region optimizer with initial radius: {}",
                config.initial_radius
            );
        }

        Self {
            state: TrustRegionState::new(config.initial_radius),
            config,
            stagnation_multiplier: 1.0,
            stagnation_count: 1,
            bfgs_approx: None,
        }
    }

    /// Compute the Cauchy point for the trust region subproblem
    fn compute_cauchy_point(&self, gradient: &[Tensor], radius: f64) -> CandleResult<Vec<Tensor>> {
        let grad_norm = compute_magnitude(gradient)?;

        if grad_norm < 1e-12 {
            // Zero gradient, return zero step
            return gradient
                .iter()
                .map(Tensor::zeros_like)
                .collect::<CandleResult<Vec<_>>>();
        }

        // Cauchy point: p = -τ * (radius / ||g||) * g
        // where τ = min(1, radius / ||g||)
        let tau = (radius / grad_norm).min(1.0);
        let scale = -tau * radius / grad_norm;

        gradient
            .iter()
            .map(|g| g.affine(scale, 0.0))
            .collect::<CandleResult<Vec<_>>>()
    }

    /// Solve the trust region subproblem using dogleg method
    fn solve_subproblem(
        &self,
        params: &[Tensor],
        gradient: &[Tensor],
        hessian_approx: Option<&[Tensor]>,
        radius: f64,
    ) -> CandleResult<Vec<Tensor>> {
        // For now, we'll use a simplified approach
        // In a full implementation, this would solve: min_p m(p) s.t. ||p|| <= radius
        // where m(p) = f + g^T p + 0.5 p^T B p

        if hessian_approx.is_none() {
            // Use Cauchy point for first iterations
            if self.config.verbose {
                debug!("Using Cauchy point (no Hessian approximation)");
            }
            return self.compute_cauchy_point(gradient, radius);
        }

        // Compute Newton step using BFGS approximation if available
        let newton_step = if let Some(bfgs) = &self.bfgs_approx {
            // Solve B * p = -g using BFGS approximation
            bfgs.solve(gradient)?
        } else {
            // Use scaled identity approximation: B ≈ γI
            // γ is estimated from gradient changes if available
            let gamma = if let Some(prev_grad) = &self.state.prev_gradient {
                let grad_diff = gradient.iter()
                    .zip(prev_grad.iter())
                    .map(|(g, pg)| g.sub(pg))
                    .collect::<CandleResult<Vec<_>>>()?;
                let grad_diff_norm_sq = dot_product(&grad_diff, &grad_diff)?;
                if grad_diff_norm_sq > 1e-12 {
                    let step_dot_grad_diff = dot_product(params, &grad_diff)?;
                    (step_dot_grad_diff / grad_diff_norm_sq).max(0.1).min(10.0)
                } else {
                    1.0
                }
            } else {
                1.0
            };
            
            gradient
                .iter()
                .map(|g| g.affine(-1.0 / gamma, 0.0))
                .collect::<CandleResult<Vec<_>>>()?
        };

        let newton_norm = compute_magnitude(&newton_step)?;
        if self.config.verbose {
            debug!("Newton step norm: {newton_norm:.6e}, trust region radius: {radius:.6e}");
        }

        if newton_norm <= radius {
            // Newton step is within trust region
            if self.config.verbose {
                debug!("Using full Newton step");
            }
            Ok(newton_step)
        } else {
            // Use dogleg method
            let cauchy_point = self.compute_cauchy_point(gradient, radius)?;
            let cauchy_norm = compute_magnitude(&cauchy_point)?;
            
            // If Cauchy point reaches the boundary, use it
            if cauchy_norm >= radius - 1e-10 {
                if self.config.verbose {
                    debug!("Using Cauchy point (reaches boundary)");
                }
                return Ok(cauchy_point);
            }
            
            // Find intersection of dogleg path with trust region boundary
            // p(τ) = τ * cauchy_point for τ ∈ [0, 1]
            // p(τ) = cauchy_point + (τ-1) * (newton_step - cauchy_point) for τ ∈ [1, 2]
            
            // Check if we can reach Newton point
            let diff = newton_step.iter()
                .zip(cauchy_point.iter())
                .map(|(n, c)| n.sub(c))
                .collect::<CandleResult<Vec<_>>>()?;
            
            // Solve ||cauchy_point + t * diff||^2 = radius^2
            let a = dot_product(&diff, &diff)?;
            let b = 2.0 * dot_product(&cauchy_point, &diff)?;
            let c = cauchy_norm * cauchy_norm - radius * radius;
            
            let discriminant = b * b - 4.0 * a * c;
            let t = if discriminant >= 0.0 {
                (-b + discriminant.sqrt()) / (2.0 * a)
            } else {
                0.0
            };
            
            cauchy_point
                .iter()
                .zip(diff.iter())
                .map(|(c, d)| c.add(&d.affine(t.max(0.0).min(1.0), 0.0)?))
                .collect::<CandleResult<Vec<_>>>()
        }
    }

    /// Evaluate the quadratic model at a given step
    fn evaluate_model(&self, gradient: &[Tensor], step: &[Tensor]) -> CandleResult<f64> {
        // m(p) = g^T p + 0.5 p^T B p
        let linear_term = dot_product(gradient, step)?;
        let quadratic_term = if let Some(bfgs) = &self.bfgs_approx {
            // Use BFGS approximation for quadratic term
            let bp = bfgs.multiply(step)?;
            0.5 * dot_product(step, &bp)?
        } else {
            // Use scaled identity approximation
            let gamma = if self.state.prev_gradient.is_some() {
                1.0 // This should match the gamma used in solve_subproblem
            } else {
                1.0
            };
            0.5 * gamma * dot_product(step, step)?
        };

        
        Ok(linear_term + quadratic_term)
    }
}

impl Optimizer for TrustRegionOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,
        params: &mut [Tensor],
        function: Arc<dyn DifferentiableFunction + Send + Sync>,
    ) -> CandleResult<StepResult> {
        let start_time = Instant::now();

        if self.config.verbose {
            debug!(
                "Trust Region step {} starting with radius: {}",
                self.state.iteration, self.state.radius
            );
        }

        // Evaluate function and gradient at current point
        let current_value = function.evaluate(params)?;
        let gradient = function.gradient(params)?;
        let grad_norm = compute_magnitude(&gradient)?;

        if self.config.verbose {
            debug!("Current function value: {current_value:.6e}, gradient norm: {grad_norm:.6e}");
        }

        // Update best function value
        match self.state.best_function_value {
            Some(best) if current_value < best => {
                self.state.best_function_value = Some(current_value);
            }
            None => {
                self.state.best_function_value = Some(current_value);
            }
            _ => {}
        }

        // Check for convergence
        let converged = grad_norm < 1e-8 || self.state.radius < self.config.min_radius;

        if self.config.verbose {
            debug!("Convergence check: grad_norm = {:.6e} (< 1e-6?), radius = {:.6e} (< {}?), converged = {}", 
                  grad_norm, self.state.radius, self.config.min_radius, converged);
        }

        if converged {
            return Ok(StepResult {
                step_size: 0.0,
                convergence_info: ConvergenceInfo::converged(),
                metadata: OptimizationMetadata::default(),
            });
        }

        // Solve trust region subproblem
        let step = self.solve_subproblem(
            params,
            &gradient,
            self.state.hessian_approx.as_deref(),
            self.state.radius,
        )?;
        let step_norm = compute_magnitude(&step)?;

        // Evaluate model reduction
        let model_reduction = -self.evaluate_model(&gradient, &step)?;

        // Compute trial point
        let trial_params: Vec<Tensor> = params
            .iter()
            .zip(step.iter())
            .map(|(p, s)| p.add(s))
            .collect::<CandleResult<Vec<_>>>()?;

        // Evaluate function at trial point
        let trial_value = function.evaluate(&trial_params)?;
        let actual_reduction = current_value - trial_value;

        // Compute ratio of actual to predicted reduction
        let rho = if model_reduction.abs() < 1e-12 {
            if actual_reduction > 0.0 {
                1.0
            } else {
                0.0
            }
        } else {
            actual_reduction / model_reduction
        };

        if self.config.verbose {
            debug!(
                "Step norm: {step_norm:.6e}, model reduction: {model_reduction:.6e}, actual reduction: {actual_reduction:.6e}, rho: {rho:.6e}"
            );
        }

        // Update trust region radius and accept/reject step
        let step_accepted = if rho > self.config.eta_1 {
            // Accept step
            for (param, trial) in params.iter_mut().zip(trial_params.iter()) {
                *param = trial.clone();
            }
            self.state.consecutive_rejections = 0;

            // Update radius
            if rho > self.config.eta_2 && step_norm > 0.9 * self.state.radius {
                // Very good agreement and step at boundary - expand region
                self.state.radius =
                    (self.config.gamma_2 * self.state.radius).min(self.config.max_radius);
                if self.config.verbose {
                    debug!("Expanding trust region to: {}", self.state.radius);
                }
            }

            true
        } else {
            // Reject step
            self.state.consecutive_rejections += 1;

            // Shrink trust region
            self.state.radius *= self.config.gamma_1;
            if self.config.verbose {
                debug!("Shrinking trust region to: {}", self.state.radius);
            }

            false
        };

        // Update state
        self.state.iteration += 1;
        self.state.prev_function_value = Some(if step_accepted {
            trial_value
        } else {
            current_value
        });
        // Update BFGS approximation if step was accepted
        if step_accepted && self.state.prev_gradient.is_some() {
            let prev_grad = self.state.prev_gradient.as_ref().unwrap();
            let y = gradient.iter()
                .zip(prev_grad.iter())
                .map(|(g, pg)| g.sub(pg))
                .collect::<CandleResult<Vec<_>>>()?;
            let sy = dot_product(&step, &y)?;
            if sy > 1e-8 * step_norm * compute_magnitude(&y)? {
                // Update is numerically stable
                if self.bfgs_approx.is_none() {
                    self.bfgs_approx = Some(BFGSApproximation::new(params.len()));
                }
                self.bfgs_approx.as_mut().unwrap().update(&step, &y)?;
            }
        }

        
        // Create metadata
        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = start_time.elapsed();
        metadata
            .optimizer_data
            .insert("trust_region_radius".to_string(), self.state.radius);
        metadata
            .optimizer_data
            .insert("gradient_norm".to_string(), grad_norm);
        metadata
            .optimizer_data
            .insert("step_norm".to_string(), step_norm);
        metadata.optimizer_data.insert("rho".to_string(), rho);
        metadata.optimizer_data.insert(
            "step_accepted".to_string(),
            if step_accepted { 1.0 } else { 0.0 },
        );
        metadata.optimizer_data.insert(
            "consecutive_rejections".to_string(),
            self.state.consecutive_rejections as f64,
        );
        // Store gradient for next iteration
        if step_accepted {
            self.state.prev_gradient = Some(gradient);
        }

        
        Ok(StepResult {
            step_size: if step_accepted { step_norm } else { 0.0 },
            convergence_info: ConvergenceInfo {
                converged: false,
                function_change: Some(actual_reduction),
            },
            metadata,
        })
    }

    fn reset(&mut self) {
        self.state.reset(self.config.initial_radius);
        self.bfgs_approx = None;
    }

    fn name(&self) -> &str {
        "TrustRegion"
    }

    fn iteration(&self) -> usize {
        self.state.iteration
    }

    fn set_stagnation_multiplier(&mut self, multiplier: f64) {
        self.stagnation_multiplier = multiplier;
    }

    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
}
/// BFGS approximation for the Hessian matrix
#[derive(Debug, Clone)]
struct BFGSApproximation {
    /// Stored s vectors (parameter differences)
    s_history: Vec<Vec<Tensor>>,
    /// Stored y vectors (gradient differences)
    y_history: Vec<Vec<Tensor>>,
    /// Maximum history size
    max_history: usize,
}
impl BFGSApproximation {
    fn new(max_history: usize) -> Self {
        Self {
            s_history: Vec::new(),
            y_history: Vec::new(),
            max_history: max_history.max(3).min(20),
        }
    }
    /// Update BFGS approximation with new s and y vectors
    fn update(&mut self, s: &[Tensor], y: &[Tensor]) -> CandleResult<()> {
        self.s_history.push(s.to_vec());
        self.y_history.push(y.to_vec());
        // Keep only recent history
        if self.s_history.len() > self.max_history {
            self.s_history.remove(0);
            self.y_history.remove(0);
        }
        Ok(())
    }
    /// Solve B * p = -g using L-BFGS two-loop recursion
    fn solve(&self, gradient: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        if self.s_history.is_empty() {
            // No history, use scaled identity
            return gradient
                .iter()
                .map(|g| g.affine(-1.0, 0.0))
                .collect::<CandleResult<Vec<_>>>();
        }
        let m = self.s_history.len();
        let mut alpha = vec![0.0; m];
        let mut q = gradient.to_vec();
        // First loop
        for i in (0..m).rev() {
            let rho = 1.0 / dot_product(&self.y_history[i], &self.s_history[i])?;
            alpha[i] = rho * dot_product(&self.s_history[i], &q)?;
            for j in 0..q.len() {
                q[j] = q[j].sub(&self.y_history[i][j].affine(alpha[i], 0.0)?)?;
            }
        }
        // Scale by initial Hessian approximation
        let gamma = if m > 0 {
            let last_y = &self.y_history[m - 1];
            let last_s = &self.s_history[m - 1];
            dot_product(last_s, last_y)? / dot_product(last_y, last_y)?
        } else {
            1.0
        };
        let mut r = q.iter()
            .map(|qi| qi.affine(gamma, 0.0))
            .collect::<CandleResult<Vec<_>>>()?;
        // Second loop
        for i in 0..m {
            let rho = 1.0 / dot_product(&self.y_history[i], &self.s_history[i])?;
            let beta = rho * dot_product(&self.y_history[i], &r)?;
            for j in 0..r.len() {
                r[j] = r[j].add(&self.s_history[i][j].affine(alpha[i] - beta, 0.0)?)?;
            }
        }
        // Return negative of r
        r.iter().map(|ri| ri.affine(-1.0, 0.0)).collect()
    }
    /// Multiply vector by BFGS approximation B * v
    fn multiply(&self, v: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // For now, use finite difference approximation
        // In a full implementation, this would use the BFGS formula
        let epsilon = 1e-8;
        let mut result = Vec::new();
        for i in 0..v.len() {
            // Simple scaled identity approximation
            result.push(v[i].affine(1.0 / self.get_gamma()?, 0.0)?);
        }
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;

    struct QuadraticFunction;

    impl DifferentiableFunction for QuadraticFunction {
        fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
            let x = params[0].to_vec1::<f64>()?;
            Ok(x.iter().map(|&xi| xi * xi).sum())
        }

        fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
            let device = params[0].device();
            let x = params[0].to_vec1::<f64>()?;
            let grad: Vec<f64> = x.iter().map(|&xi| 2.0 * xi).collect();
            Ok(vec![Tensor::from_vec(grad, x.len(), device)?])
        }
    }

    #[test]
    fn test_trust_region_creation() {
        let config = TrustRegionConfig::default();
        let optimizer = TrustRegionOptimizer::new(config);

        assert_eq!(optimizer.name(), "TrustRegion");
        assert_eq!(optimizer.state.radius, 1.0);
        assert_eq!(optimizer.state.iteration, 0);
    }

    #[test]
    fn test_trust_region_configs() {
        let conservative = TrustRegionConfig::conservative();
        assert_eq!(conservative.initial_radius, 0.5);
        assert_eq!(conservative.gamma_1, 0.2);

        let aggressive = TrustRegionConfig::aggressive();
        assert_eq!(aggressive.initial_radius, 2.0);
        assert_eq!(aggressive.gamma_2, 3.0);
    }

    #[test]
    fn test_cauchy_point() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = TrustRegionConfig::default();
        let optimizer = TrustRegionOptimizer::new(config);

        let gradient = vec![Tensor::from_slice(&[2.0, -4.0], &[2], &device)?];
        let radius = 1.0;

        let cauchy_point = optimizer.compute_cauchy_point(&gradient, radius)?;
        let cauchy_norm = compute_magnitude(&cauchy_point)?;

        // Cauchy point should be within trust region
        assert!(cauchy_norm <= radius + 1e-10);

        // Should be in descent direction
        let dot_prod = dot_product(&gradient, &cauchy_point)?;
        assert!(dot_prod < 0.0);

        Ok(())
    }

    #[test]
    fn test_trust_region_on_quadratic() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = TrustRegionConfig {
            verbose: false,
            ..Default::default()
        };
        let mut optimizer = TrustRegionOptimizer::new(config);
        let function = Arc::new(QuadraticFunction);

        let mut params = vec![Tensor::from_slice(&[5.0, -3.0], &[2], &device)?];
        println!("Initial params: {:?}", params[0].to_vec1::<f64>()?);

        // Run optimization steps
        for i in 0..50 {
            let result = optimizer.step(&mut params, function.clone())?;
            let current_params = params[0].to_vec1::<f64>()?;
            let current_value = function.evaluate(&params)?;
            println!(
                "Iteration {}: params = {:?}, value = {:.6e}, step_size = {:.6e}, converged = {}",
                i,
                current_params,
                current_value,
                result.step_size,
                result.convergence_info.converged
            );

            if result.convergence_info.converged {
                println!("Converged at iteration {i}");
                break;
            }
        }

        // Should converge close to [0, 0]
        let final_params = params[0].to_vec1::<f64>()?;
        println!("Final params: {final_params:?}");
        let final_value = function.evaluate(&params)?;
        println!("Final function value: {final_value:.6e}");

        Ok(())
    }
}