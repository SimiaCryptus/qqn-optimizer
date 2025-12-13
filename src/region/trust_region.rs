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

use std::fmt::Debug;
use crate::optimizers::optimizer::{
    ConvergenceInfo, OptimizationContext, OptimizationMetadata, Optimizer, StepResult,
};
use itertools::Itertools;
use log::{debug, info, warn};
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;



/// Trait for defining a trust region or constraint that projects parameters
pub trait TrustRegion: Send + Sync + Debug {
    /// Project parameters into the valid region
    fn project(&self, params: &mut [f64]);
    /// Clone the trust region
    fn clone_box(&self) -> Box<dyn TrustRegion>;
}

impl Clone for Box<dyn TrustRegion> {
    fn clone(&self) -> Box<dyn TrustRegion> {
        self.clone_box()
    }
}


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
    /// Name of the optimizer
    ///
    /// **Default**: "TrustRegion"
    pub name: String,
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
            name: "TrustRegion".to_string(),
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
            name: "TrustRegion-Conservative".to_string(),
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
            name: "TrustRegion-Aggressive".to_string(),
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
    hessian_approx: Option<Vec<f64>>,

    /// Number of consecutive rejected steps
    consecutive_rejections: usize,

    /// Best function value seen so far
    best_function_value: Option<f64>,
    /// Pending step from previous iteration (for delayed evaluation)
    #[serde(skip)]
    pending_step: Option<Vec<f64>>,
    /// Predicted reduction of the pending step
    #[serde(skip)]
    pending_model_reduction: Option<f64>,
}

impl TrustRegionState {
    /// Create a new trust region state
    pub fn new(initial_radius: f64) -> Self {
        Self {
            radius: initial_radius,
            iteration: 0,
            prev_function_value: None,
            hessian_approx: None,
            consecutive_rejections: 0,
            best_function_value: None,
            pending_step: None,
            pending_model_reduction: None,
        }
    }

    /// Reset the state
    pub fn reset(&mut self, initial_radius: f64) {
        self.radius = initial_radius;
        self.iteration = 0;
        self.prev_function_value = None;
        self.hessian_approx = None;
        self.consecutive_rejections = 0;
        self.best_function_value = None;
        self.pending_step = None;
        self.pending_model_reduction = None;
    }
}

/// Trust Region optimizer
#[derive(Debug)]
pub struct TrustRegionOptimizer {
    config: TrustRegionConfig,
    state: TrustRegionState,
    stagnation_multiplier: f64,
    stagnation_count: usize,
}

impl Clone for TrustRegionOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            stagnation_multiplier: self.stagnation_multiplier,
            stagnation_count: self.stagnation_count,
        }
    }
}

impl TrustRegionOptimizer {
    /// Create a new Trust Region optimizer
    pub fn new(config: TrustRegionConfig) -> Self {
        info!(
            "Creating Trust Region optimizer '{}' with parameters: \
             initial_radius={}, max_radius={}, min_radius={}, \
             eta_1={}, eta_2={}, gamma_1={}, gamma_2={}, \
             max_subproblem_iterations={}, subproblem_tolerance={}, \
             use_cauchy_fallback={}, verbose={}",
            config.name,
            config.initial_radius,
            config.max_radius,
            config.min_radius,
            config.eta_1,
            config.eta_2,
            config.gamma_1,
            config.gamma_2,
            config.max_subproblem_iterations,
            config.subproblem_tolerance,
            config.use_cauchy_fallback,
            config.verbose
        );
        Self {
            state: TrustRegionState::new(config.initial_radius),
            config,
            stagnation_multiplier: 1.0,
            stagnation_count: 1,
        }
    }





    /// Solve the trust region subproblem using dogleg method
    fn solve_subproblem(
        &self,
        gradient: &[f64],
        _hessian_approx: Option<&[f64]>,
        radius: f64,




    ) -> Vec<f64> {
        // Using B = I approximation (Steepest Descent with Trust Region)
        // Minimize m(p) = g^T p + 0.5 p^T p  s.t. ||p|| <= radius
        // Unconstrained minimizer: p = -g

        let grad_norm = vec_norm(gradient);
        if grad_norm < 1e-12 {
            return vec![0.0; gradient.len()];
        }

        // If ||-g|| <= radius, take full step
        if grad_norm <= radius {
            vec_scale(gradient, -1.0)
        } else {
            // Take step to boundary: -radius * g / ||g||
            vec_scale(gradient, -radius / grad_norm)
        }
    }

    /// Evaluate the quadratic model at a given step
    fn evaluate_model(&self, gradient: &[f64], step: &[f64]) -> f64 {
        // m(p) = g^T p + 0.5 p^T B p

        // Assuming B = I
        let linear_term = vec_dot(gradient, step);
        let quadratic_term = 0.5 * vec_dot(step, step);

        linear_term + quadratic_term
    }
}

impl Optimizer for TrustRegionOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,
        ctx: &mut OptimizationContext,
    ) -> StepResult {
        let start_time = Instant::now();

        if self.config.verbose {
            debug!(
                "Trust Region step {} starting with radius: {}",
                self.state.iteration, self.state.radius
            );
        }

        // Evaluate function and gradient at current point
        let current_params = flatten_tensors(&ctx.weights);
        let gradient = flatten_tensors(&ctx.gradients);
        let current_value = ctx.loss.data()[0] as f64;
        let grad_norm = vec_norm(&gradient);

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
        // Check if we have a pending step to evaluate from previous iteration
        if let Some(step) = self.state.pending_step.take() {
            let model_reduction = self.state.pending_model_reduction.take().unwrap_or(0.0);
            let prev_value = self.state.prev_function_value.unwrap_or(current_value);
            let actual_reduction = prev_value - current_value;
            // Compute rho
            let rho = if model_reduction.abs() < 1e-12 {
                if actual_reduction > 0.0 { 1.0 } else { 0.0 }
            } else {
                actual_reduction / model_reduction
            };
            let step_norm = vec_norm(&step);
            if self.config.verbose {
                debug!(
                    "Evaluating pending step: rho={:.6e}, actual_red={:.6e}, model_red={:.6e}",
                    rho, actual_reduction, model_reduction
                );
            }
            if rho > self.config.eta_1 {
                // Accept step
                self.state.consecutive_rejections = 0;
                // Update radius
                if rho > self.config.eta_2 && step_norm > 0.9 * self.state.radius {
                    self.state.radius = (self.config.gamma_2 * self.state.radius).min(self.config.max_radius);
                }
                // Update prev_function_value to current_value (which is the new accepted point)
                self.state.prev_function_value = Some(current_value);
            } else {
                // Reject step
                self.state.consecutive_rejections += 1;
                self.state.radius *= self.config.gamma_1;
                // Revert weights: w_old = current - step
                let w_old = vec_add(&current_params, &vec_scale(&step, -1.0));
                let shapes = ctx.weights.iter().map(|w| w.shape.to_shape().iter().map(|&d| d.to_usize().unwrap()).collect_vec()).collect::<Vec<_>>();
                let mut old_weights_data = unflatten_tensors(&w_old, &shapes);
                ctx.write_weights(&mut old_weights_data);
                return StepResult {
                    step_size: 0.0,
                    convergence_info: ConvergenceInfo { converged: false, function_change: Some(0.0) },
                };
            }
        } else {
            // No pending step. We are at a valid point.
            self.state.prev_function_value = Some(current_value);
        }


        // Check for convergence
        let converged = grad_norm < 1e-6 || self.state.radius < self.config.min_radius;

        if self.config.verbose {
            debug!("Convergence check: grad_norm = {:.6e} (< 1e-6?), radius = {:.6e} (< {}?), converged = {}", 
                  grad_norm, self.state.radius, self.config.min_radius, converged);
        }

        if converged {
            return StepResult {
                step_size: 0.0,
                convergence_info: ConvergenceInfo::converged(),
            };
        }

        // Solve trust region subproblem
        let step = self.solve_subproblem(
            &gradient,
            self.state.hessian_approx.as_deref(),
            self.state.radius,
        );
        let step_norm = vec_norm(&step);

        // Evaluate model reduction
        let model_reduction = -self.evaluate_model(&gradient, &step);

        // Compute trial point
        let trial_params = vec_add(&current_params, &step);









        // Apply trial weights
        let shapes = ctx.weights.iter().map(|w| w.shape.to_shape().iter().map(|&d| d.to_usize().unwrap()).collect_vec()).collect::<Vec<_>>();
        let mut trial_weights_data = unflatten_tensors(&trial_params, &shapes);
        ctx.write_weights(&mut trial_weights_data);

        // Update state
        self.state.iteration += 1;
        self.state.pending_step = Some(step);
        self.state.pending_model_reduction = Some(model_reduction);

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
        metadata.optimizer_data.insert(
            "consecutive_rejections".to_string(),
            self.state.consecutive_rejections as f64,
        );

        StepResult {
            step_size: step_norm,
            convergence_info: ConvergenceInfo {
                converged: false,
                function_change: None,
            },
        }
    }

    fn reset(&mut self) {
        self.state.reset(self.config.initial_radius);
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn set_stagnation_multiplier(&mut self, multiplier: f64) {
        self.stagnation_multiplier = multiplier;
    }

    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
}
fn vec_dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn vec_norm(a: &[f64]) -> f64 {
    vec_dot(a, a).sqrt()
}
fn vec_scale(a: &[f64], s: f64) -> Vec<f64> {
    a.iter().map(|x| x * s).collect()
}
fn vec_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}
fn flatten_tensors(tensors: &[GraphTensor]) -> Vec<f64> {
    tensors
        .iter()
        .flat_map(|t| {
            t.data()
                .into_iter()
                .map(|x| x as f64)
                .collect::<Vec<f64>>()
        })
        .collect()
}
fn unflatten_tensors(flat: &[f64], shapes: &[Vec<usize>]) -> Vec<Vec<f32>> {
    let mut result = Vec::new();
    let mut offset = 0;
    for shape in shapes {
        let size: usize = shape.iter().product();
        let chunk = &flat[offset..offset + size];
        result.push(chunk.iter().map(|&x| x as f32).collect());
        offset += size;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;




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
        assert_eq!(conservative.name, "TrustRegion-Conservative");

        let aggressive = TrustRegionConfig::aggressive();
        assert_eq!(aggressive.initial_radius, 2.0);
        assert_eq!(aggressive.gamma_2, 3.0);
        assert_eq!(aggressive.name, "TrustRegion-Aggressive");
    }
}