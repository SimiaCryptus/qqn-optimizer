//! L-BFGS (Limited-memory Broyden-Fletcher-Goldfarb-Shanno) optimizer implementation.
//!
//! This implementation provides a robust quasi-Newton optimization method that approximates
//! the inverse Hessian matrix using a limited history of gradient and parameter changes.
//! L-BFGS is particularly effective for smooth, differentiable optimization problems and
//! serves both as a standalone optimizer and as a core component of the QQN algorithm.

use crate::line_search::line_search::create_line_search;
use crate::line_search::{LineSearch, LineSearchConfig, LineSearchMethod};
use crate::optimizers::optimizer::{
    ConvergenceInfo, OptimizationContext, OptimizationMetadata, Optimizer, StepResult,
};
use anyhow::Result;
use log::{debug, info, trace, warn};
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;
use itertools::Itertools;

/// Configuration parameters for the L-BFGS optimizer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBFGSConfig {
    /// Number of previous iterations to store for Hessian approximation.
    pub history_size: usize,

    /// Line search configuration for step size selection.
    pub line_search: LineSearchConfig,

    /// Numerical stability constant for avoiding division by zero.
    pub epsilon: f64,

    /// Maximum number of correction pairs to use in two-loop recursion.
    pub max_correction_pairs: usize,

    /// Maximum allowed step size in any single iteration.
    pub max_step_size: f64,

    /// Minimum allowed step size before declaring convergence failure.
    pub min_step_size: f64,

    /// Maximum allowed parameter change per iteration (L∞ norm).
    pub max_param_change: f64,

    /// Gradient clipping threshold to prevent numerical overflow.
    pub gradient_clip: f64,

    /// Enable recovery mechanism when optimization stagnates.
    pub enable_recovery: bool,

    /// Number of iterations without improvement before triggering recovery.
    pub recovery_patience: usize,

    /// Enable verbose logging of tensor data and internal state.
    pub verbose: bool,
    /// Name identifier for this optimizer instance.
    pub name: String,
}

impl Default for LBFGSConfig {
    fn default() -> Self {
        Self {
            history_size: 10,
            line_search: LineSearchConfig {
                c1: 1e-4,
                c2: 0.9,
                initial_step: 1.0,
                max_step: 2.0,
                method: LineSearchMethod::StrongWolfe,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            max_correction_pairs: 10,
            max_step_size: 2.0,
            min_step_size: 1e-16,
            max_param_change: 1.0,
            gradient_clip: 1e3,
            enable_recovery: true,
            recovery_patience: 5,
            verbose: false,
            name: "L-BFGS".to_string(),
        }
    }
}

impl LBFGSConfig {
    pub fn strict() -> Self {
        Self {
            history_size: 5,
            line_search: LineSearchConfig {
                c1: 1e-4,
                c2: 0.9,
                initial_step: 0.1,
                max_step: 1.0,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-10,
            max_correction_pairs: 5,
            max_step_size: 0.5,
            min_step_size: 1e-20,
            max_param_change: 0.1,
            gradient_clip: 1e2,
            enable_recovery: true,
            recovery_patience: 10,
            verbose: false,
            name: "L-BFGS-Strict".to_string(),
        }
    }

    pub fn lax() -> Self {
        Self {
            history_size: 20,
            line_search: LineSearchConfig {
                c1: 1e-4,
                c2: 0.1,
                initial_step: 2.0,
                max_step: 50.0,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-6,
            max_correction_pairs: 20,
            max_step_size: 50.0,
            min_step_size: 1e-12,
            max_param_change: 100.0,
            gradient_clip: 1e6,
            enable_recovery: true,
            recovery_patience: 2,
            verbose: false,
            name: "L-BFGS-Lax".to_string(),
        }
    }

    pub fn for_qqn() -> Self {
        Self {
            history_size: 10,
            line_search: LineSearchConfig {
                c1: 1e-4,
                c2: 0.5,
                initial_step: 1.0,
                max_step: 10.0,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            max_correction_pairs: 10,
            max_step_size: 10.0,
            min_step_size: 1e-16,
            max_param_change: 10.0,
            gradient_clip: 0.0,
            enable_recovery: false,
            recovery_patience: 0,
            verbose: false,
            name: "L-BFGS-QQN".to_string(),
        }
    }
}

/// State information for L-BFGS optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBFGSState {
    /// History of parameter differences (s_k = x_{k+1} - x_k).
    #[serde(skip_serializing, skip_deserializing)]
    s_history: VecDeque<Vec<f64>>,

    /// History of gradient differences (y_k = ∇f_{k+1} - ∇f_k).
    #[serde(skip_serializing, skip_deserializing)]
    y_history: VecDeque<Vec<f64>>,

    /// Precomputed reciprocals ρ_k = 1/(s_k^T y_k) for computational efficiency.
    rho_history: VecDeque<f64>,

    /// Previous gradient for computing y_k differences in next update.
    #[serde(skip_serializing, skip_deserializing)]
    prev_gradient: Option<Vec<f64>>,

    /// Current iteration number.
    iteration: usize,

    /// Scaling factor γ for initial Hessian approximation H₀ = γI.
    gamma: f64,

    /// Numerical stability constant.
    epsilon: f64,

    /// Best function value encountered.
    best_function_value: Option<f64>,

    /// Counter for iterations without improvement.
    no_improvement_count: usize,

    /// Previous parameters stored for potential recovery.
    #[serde(skip_serializing, skip_deserializing)]
    prev_params: Option<Vec<f64>>,

    /// Flag to disable certain safety checks when used within QQN.
    disable_checks: bool,

    /// Maximum allowed gradient norm before applying scaling.
    max_gradient_norm: f64,
}

impl LBFGSState {
    pub fn new(history_size: usize, epsilon: f64) -> Self {
        Self::new_with_options(history_size, epsilon, false)
    }

    pub fn new_with_options(history_size: usize, epsilon: f64, disable_checks: bool) -> Self {
        Self {
            s_history: VecDeque::with_capacity(history_size),
            y_history: VecDeque::with_capacity(history_size),
            rho_history: VecDeque::with_capacity(history_size),
            prev_gradient: None,
            iteration: 0,
            gamma: 1.0,
            epsilon,
            best_function_value: None,
            no_improvement_count: 0,
            prev_params: None,
            disable_checks,
            max_gradient_norm: 1e10,
        }
    }

    pub fn reset(&mut self) {
        self.s_history.clear();
        self.y_history.clear();
        self.rho_history.clear();
        self.prev_gradient = None;
        self.iteration = 0;
        self.gamma = 1.0;
        self.best_function_value = None;
        self.no_improvement_count = 0;
        self.prev_params = None;
    }

    /// Compute the L-BFGS search direction using the two-loop recursion.
    pub fn estimate_optimum(&mut self, gradient: &[f64]) -> Result<Vec<f64>> {
        if gradient.is_empty() {
            return Err(anyhow::anyhow!("Empty gradient vector"));
        }
        trace!("Estimating optimum. Gradient norm: {:.6e}", vec_norm(gradient));


        if !self.disable_checks {
            let grad_norm = vec_norm(gradient);
            if grad_norm < self.epsilon {
                debug!("L-BFGS: Very small gradient norm {grad_norm:.6e}, using steepest descent");
                return Ok(vec_neg(gradient));
            }
            if grad_norm > self.max_gradient_norm {
                warn!("L-BFGS: Extremely large gradient norm {grad_norm:.6e}, scaling down");
                let scale = self.max_gradient_norm / grad_norm;
                return Ok(vec_scale(gradient, -scale));
            }
            if !vec_is_finite(gradient) {
                warn!("L-BFGS: Non-finite gradient detected, using steepest descent");
                return Ok(vec_neg(gradient));
            }
        }

        if self.s_history.is_empty() {
            debug!("L-BFGS: No history, using steepest descent");
            return Ok(vec_neg(gradient));
        }

        let mut q = gradient.to_vec();
        let mut alpha = Vec::with_capacity(self.s_history.len());
        trace!("Starting two-loop recursion with history size {}", self.s_history.len());


        // First loop
        for i in (0..self.s_history.len()).rev() {
            let s_i = &self.s_history[i];
            let rho_i = self.rho_history[i];

            if !rho_i.is_finite() || rho_i.abs() < 1e-16 {
                trace!("Skipping history index {} due to bad rho: {}", i, rho_i);
                alpha.push(0.0);
                continue;
            }

            let alpha_i = rho_i * vec_dot(s_i, &q);
            if !alpha_i.is_finite() {
                trace!("Skipping history index {} due to non-finite alpha", i);
                alpha.push(0.0);
                continue;
            }

            alpha.push(alpha_i);
            let y_i = &self.y_history[i];
            q = vec_sub(&q, &vec_scale(y_i, alpha_i));
        }

        alpha.reverse();

        // Apply scaling
        let safe_gamma = self.gamma.max(1e-12).min(1e12);
        trace!("Applying initial Hessian scaling gamma: {:.6e}", safe_gamma);
        let mut r = vec_scale(&q, safe_gamma);

        // Second loop
        for i in 0..self.s_history.len() {
            if i >= alpha.len() || alpha[i] == 0.0 {
                continue;
            }
            let s_i = &self.s_history[i];
            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];
            let alpha_i = alpha[i];

            let beta = rho_i * vec_dot(y_i, &r);
            let correction_factor = alpha_i - beta;

            if !correction_factor.is_finite() {
                trace!("Skipping correction at index {} due to non-finite factor", i);
                continue;
            }

            r = vec_add(&r, &vec_scale(s_i, correction_factor));
        }
        debug!("Estimated direction norm: {:.6e}", vec_norm(&r));

        Ok(vec_neg(&r))
    }

    /// Update the L-BFGS state with new gradient and step information.
    pub fn update(
        &mut self,
        old_params: &[f64],
        new_params: &[f64],
        new_gradient: &[f64],
        old_gradient: &[f64],
    ) -> Result<()> {
        let s_k = vec_sub(new_params, old_params);
        let s_k_norm = vec_norm(&s_k);
        trace!("Updating state. s_k norm: {:.6e}", s_k_norm);


        if s_k_norm < self.epsilon {
            debug!("L-BFGS: Parameter change too small, skipping update");
            self.prev_gradient = Some(new_gradient.to_vec());
            return Ok(());
        }

        let y_k = vec_sub(new_gradient, old_gradient);
        let grad_norm = vec_norm(&y_k);
        let s_dot_y = vec_dot(&s_k, &y_k);
        trace!("y_k norm: {:.6e}, s_dot_y: {:.6e}", grad_norm, s_dot_y);


        // Powell damping
        let curvature_threshold = self.epsilon * grad_norm.max(1.0);
        let (s_k_final, y_k_final, s_dot_y_final) = if s_dot_y < curvature_threshold {
            if self.disable_checks {
                (s_k, y_k, s_dot_y)
            } else {
                debug!("Curvature condition not met (s.y={:.6e} < {:.6e}). Applying Powell damping.", s_dot_y, curvature_threshold);
                let theta = if s_dot_y < 0.2 * curvature_threshold {
                    0.8 * curvature_threshold / (curvature_threshold - s_dot_y)
                } else {
                    1.0
                };
                trace!("Damping theta: {:.6e}", theta);


                if theta < 1.0 {
                    let scaled_s = vec_scale(&s_k, self.gamma);
                    let damped_y = vec_add(
                        &vec_scale(&y_k, theta),
                        &vec_scale(&scaled_s, 1.0 - theta),
                    );
                    let damped_s_dot_y = vec_dot(&s_k, &damped_y);
                    (s_k, damped_y, damped_s_dot_y)
                } else {
                    (s_k, y_k, s_dot_y)
                }
            }
        } else {
            (s_k, y_k, s_dot_y)
        };

        if self.disable_checks || s_dot_y_final > curvature_threshold {
            let rho_k = 1.0 / s_dot_y_final;
            if self.disable_checks || rho_k.is_finite() {
                if self.s_history.len() >= self.s_history.capacity() {
                    self.s_history.pop_front();
                    self.y_history.pop_front();
                    self.rho_history.pop_front();
                }

                self.s_history.push_back(s_k_final);
                self.y_history.push_back(y_k_final.clone());
                self.rho_history.push_back(rho_k);

                let y_dot_y = vec_dot(&y_k_final, &y_k_final);
                if y_dot_y > self.epsilon {
                    let new_gamma = s_dot_y_final / y_dot_y;
                    if new_gamma.is_finite() && new_gamma > 0.0 {
                        self.gamma = new_gamma.max(1e-8).min(1e8);
                        trace!("Updated gamma: {:.6e}", self.gamma);
                    }
                }
                debug!("History updated. Size: {}", self.s_history.len());
            }
        }

        self.prev_gradient = Some(new_gradient.to_vec());
        self.iteration += 1;
        Ok(())
    }

    pub fn iteration(&self) -> usize {
        self.iteration
    }

    pub fn history_length(&self) -> usize {
        self.s_history.len()
    }

    pub fn gamma(&self) -> f64 {
        self.gamma
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
    pub fn new(config: LBFGSConfig) -> Self {
        info!("Creating L-BFGS optimizer '{}'", config.name);
        let state = LBFGSState::new(config.history_size, config.epsilon);
        let line_search = create_line_search(config.line_search.clone());

        Self {
            config,
            state,
            line_search,
        }
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

    fn unflatten_tensors(
        flat: &[f64],
        shapes: &[Vec<usize>],
    ) -> Result<Vec<Vec<f32>>> {
        let mut result = Vec::new();
        let mut offset = 0;
        for shape in shapes {
            let size: usize = shape.iter().product();
            if offset + size > flat.len() {
                return Err(anyhow::anyhow!("Size mismatch in unflattening"));
            }
            let chunk = &flat[offset..offset + size];
            result.push(chunk.iter().map(|&x| x as f32).collect());
            offset += size;
        }
        Ok(result)
    }
}

impl Optimizer for LBFGSOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(&mut self, ctx: &mut OptimizationContext) -> StepResult {
        let start_time = Instant::now();

        // 1. Extract current state
        let current_params = Self::flatten_tensors(&ctx.weights);
        let current_grads = Self::flatten_tensors(&ctx.gradients);
        let current_loss = ctx.loss.data()[0] as f64;
        debug!("Step {}: Loss={:.6e}, |params|={:.6e}, |grads|={:.6e}", 
            self.state.iteration, current_loss, vec_norm(&current_params), vec_norm(&current_grads));


        // 2. Update history if we have previous step info
        let prev_params = self.state.prev_params.take();
        let prev_grads = self.state.prev_gradient.take();

        if let (Some(prev_p), Some(prev_g)) = (&prev_params, &prev_grads) {
            if let Err(e) = self.state.update(prev_p, &current_params, &current_grads, prev_g) {
                warn!("L-BFGS update failed: {}", e);
            }
        }

        // 3. Compute direction
        let direction = match self.state.estimate_optimum(&current_grads) {
            Ok(d) => d,
            Err(e) => {
                warn!("Failed to estimate optimum: {}, using steepest descent", e);
                vec_neg(&current_grads)
            }
        };

        let dir_norm = vec_norm(&direction);
        let grad_norm = vec_norm(&current_grads);
        trace!("Direction norm: {:.6e}, Gradient norm: {:.6e}", dir_norm, grad_norm);


        // 4. Line search
        // We clone the context because LineSearch might modify it during search,
        // but we want to keep our handle to it.
        // Note: LineSearch trait takes OptimizationContext by value, but it contains handles.
        // The LineSearch implementation is responsible for resetting or managing the graph state if needed.
        let ls_result = match self.line_search.search(
            ctx.clone(),
            &current_params,
            &direction,
            current_loss,
            &current_grads,
            None,
        ) {
            Ok(res) => res,
            Err(e) => {
                warn!("Line search failed: {}", e);
                // Fallback to small step
                crate::line_search::line_search::LineSearchResult {
                    step_size: self.config.min_step_size,
                    success: false,
                    termination_reason: crate::line_search::line_search::TerminationReason::FunctionEvaluationError,
                    num_f_evals: 0,
                    num_g_evals: 0,
                }
            }
        };

        let mut step_size = ls_result.step_size;
        debug!("Line search result: step={:.6e}, success={:?}", step_size, ls_result.success);

        // Limit parameter change
        if self.config.max_param_change > 0.0 {
            let max_change = direction.iter().map(|d| d.abs()).fold(0.0, f64::max) * step_size;
            if max_change > self.config.max_param_change {
                trace!("Limiting parameter change. Max change: {:.6e} > Limit: {:.6e}", max_change, self.config.max_param_change);
                step_size *= self.config.max_param_change / max_change;
            }
        }

        // 5. Update parameters
        let new_params = vec_add(&current_params, &vec_scale(&direction, step_size));

        // 6. Write back to context
        let shapes = ctx.weights.iter().map(|w| w.shape.to_shape().iter().map(
            |&d| d.to_usize().unwrap()
        ).collect_vec()).collect::<Vec<_>>();
        match Self::unflatten_tensors(&new_params, &shapes) {
            Ok(mut new_weights_data) => ctx.write_weights(&mut new_weights_data),
            Err(e) => warn!("Failed to write weights: {}", e),
        }

        // 7. Save state for next iter
        self.state.prev_params = Some(current_params);
        // Note: We don't have the gradient at new_params yet (unless line search computed it and we could retrieve it).
        // Standard L-BFGS implementation often evaluates gradient at new position at the start of next step.
        // However, our update logic requires (s_k, y_k). s_k = x_{k+1} - x_k. y_k = g_{k+1} - g_k.
        // We have x_k, g_k. We just computed x_{k+1}.
        // In the NEXT call to step(), we will read x_{k+1} (as current) and g_{k+1} (as current).
        // We will have x_k stored in prev_params.
        // We need g_k stored in prev_gradient.
        self.state.prev_gradient = Some(current_grads);

        // Check convergence
        let converged = grad_norm < 1e-6; // Simple check

        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = start_time.elapsed();
        metadata.optimizer_data.insert("gradient_norm".to_string(), grad_norm);
        metadata.optimizer_data.insert("step_size".to_string(), step_size);
        metadata.optimizer_data.insert("gamma".to_string(), self.state.gamma);

        StepResult {
            step_size,
            convergence_info: ConvergenceInfo {
                converged,
                function_change: None,
            },
        }
    }

    fn reset(&mut self) {
        self.state.reset();
    }

    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn set_stagnation_multiplier(&mut self, _multiplier: f64) {}
    fn set_stagnation_count(&mut self, _count: usize) {}
}

// --- Vector Math Helpers ---

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

fn vec_sub(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}

fn vec_neg(a: &[f64]) -> Vec<f64> {
    a.iter().map(|x| -x).collect()
}

fn vec_is_finite(a: &[f64]) -> bool {
    a.iter().all(|x| x.is_finite())
}