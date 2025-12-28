use crate::line_search::line_search::create_line_search;
use crate::line_search::{LineSearch, LineSearchConfig, LineSearchMethod};
use crate::optimizers::{GDConfig, GDOptimizer};
use crate::region::trust_region::{TrustRegion, TrustRegionConfig, TrustRegionOptimizer};
use crate::optimizers::lbfgs::LBFGSState;
use crate::optimizers::optimizer::{
    ConvergenceInfo, OptimizationContext, OptimizationMetadata, Optimizer, StepResult,
};
use anyhow::Result;
use itertools::Itertools;
use log::{debug, info, trace, warn};
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Configuration for the QQN optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QQNConfig {
    /// Name of the optimizer instance
    pub name: String,
    /// L-BFGS history length
    pub lbfgs_history: usize,
    /// Minimum number of iterations before enabling L-BFGS
    pub min_lbfgs_iterations: usize,
    /// Line search configuration
    pub line_search: LineSearchConfig,
    /// Numerical stability constant
    pub epsilon: f64,
    /// Enable verbose logging of tensor data and internal state
    pub verbose: bool,
    pub min_step_persist: f64,
    pub min_step_size: f64,
    /// Scaling factor for gradient descent direction in steepest descent
    pub gradient_scale_factor: f64,
}

impl Default for QQNConfig {
    fn default() -> Self {
        Self {
            lbfgs_history: 10,
            min_lbfgs_iterations: 1,
            line_search: LineSearchConfig {
                method: LineSearchMethod::Bisection,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-6,
            verbose: false,
            min_step_persist: 1e-1,
            min_step_size: 1e-10,
            gradient_scale_factor: 1.0,
            name: "QQN".to_string(),
        }
    }
}

impl QQNConfig {
    pub fn strict() -> Self {
        Self {
            lbfgs_history: 20,
            min_lbfgs_iterations: 5,
            line_search: LineSearchConfig {
                method: LineSearchMethod::Bisection,
                max_iterations: 50,
                c1: 1e-4,
                c2: 0.9,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            verbose: false,
            min_step_persist: 1e-2,
            min_step_size: 1e-10,
            gradient_scale_factor: 1.0,
            name: "QQN-Strict".to_string(),
        }
    }

    pub fn lax() -> Self {
        Self {
            lbfgs_history: 5,
            min_lbfgs_iterations: 1,
            line_search: LineSearchConfig {
                method: LineSearchMethod::Bisection,
                max_iterations: 20,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-4,
            verbose: false,
            min_step_persist: 1e-2,
            min_step_size: 1e-10,
            gradient_scale_factor: 1.0,
            name: "QQN-Lax".to_string(),
        }
    }

    pub fn verbose() -> Self {
        Self {
            verbose: true,
            name: "QQN-Verbose".to_string(),
            ..Self::default()
        }
    }
}

/// State information for the QQN optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QQNState {
    /// Current iteration number
    pub iteration: usize,
    /// L-BFGS internal state
    pub lbfgs_state: LBFGSState,
    /// Previous ideal step size for line search initialization
    pub previous_step_size: Option<f64>,
    
    /// Previous parameters (for L-BFGS update)
    #[serde(skip)]
    pub prev_params: Option<Vec<f64>>,
    /// Previous gradients (for L-BFGS update)
    #[serde(skip)]
    pub prev_gradient: Option<Vec<f64>>,
}

impl QQNState {
    pub fn new(lbfgs_history: usize) -> Self {
        Self {
            iteration: 0,
            // Disable checks for QQN as per original implementation logic
            lbfgs_state: LBFGSState::new_with_options(lbfgs_history, 1e-8, true),
            previous_step_size: None,
            prev_params: None,
            prev_gradient: None,
        }
    }
    
    pub fn reset(&mut self) {
        self.iteration = 0;
        self.lbfgs_state.reset();
        self.previous_step_size = None;
        self.prev_params = None;
        self.prev_gradient = None;
    }
}

#[derive(Debug)]
pub struct QQNOptimizer {
    config: QQNConfig,
    state: QQNState,
    // Used for steepest descent phase
    linear_line_search: Box<dyn LineSearch>,
    trust_region: Option<Box<dyn TrustRegion>>,
}

impl Clone for QQNOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            linear_line_search: self.linear_line_search.clone_box(),
            trust_region: self.trust_region.clone(),
        }
    }
}

impl QQNOptimizer {
    pub fn new(config: QQNConfig) -> Self {
        info!("Creating QQN optimizer '{}'", config.name);
        let line_search = create_line_search(config.line_search.clone());
        Self {
            state: QQNState::new(config.lbfgs_history),
            config,
            linear_line_search: line_search,
            trust_region: None,
        }
    }
    pub fn with_trust_region(mut self, region: Box<dyn TrustRegion>) -> Self {
        self.trust_region = Some(region);
        self
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

    fn write_params(&self, ctx: &mut OptimizationContext, params: &[f64]) -> Result<()> {
        let shapes = ctx.weights.iter().map(|w| w.shape.to_shape().iter().map(
            |&d| d.to_usize().unwrap()
        ).collect_vec()).collect::<Vec<_>>();
        
        let mut weights_data = Self::unflatten_tensors(params, &shapes)?;
        // Use the context's write_weights method to ensure proper graph update
        ctx.write_weights(&mut weights_data);
        Ok(())
    }

    fn evaluate_loss(&self, ctx: &mut OptimizationContext, params: &[f64]) -> Result<f64> {
        self.write_params(ctx, params)?;
        ctx.graph().execute();
        let loss = ctx.loss.data().as_any().downcast_ref::<Vec<f32>>().unwrap()[0] as f64;
        Ok(loss)
    }

    /// Perform steepest descent step using the configured linear line search
    fn steepest_descent_step(
        &mut self,
        ctx: &mut OptimizationContext,
        current_params: &[f64],
        current_grads: &[f64],
        current_loss: f64,
    ) -> StepResult {
        debug!("Using steepest descent (iteration {})", self.state.iteration);
        
        // Direction is negative gradient
        let direction = vec_scale(current_grads, -self.config.gradient_scale_factor);
        
        // Use standard line search
        let ls_result = self.linear_line_search.search(
            ctx.clone(),
            current_params,
            &direction,
            current_loss,
            current_grads,
            self.trust_region.as_deref(),
        ).unwrap_or_else(|e| {
            warn!("Steepest descent line search failed: {}", e);
            crate::line_search::line_search::LineSearchResult {
                step_size: self.config.min_step_size,
                success: false,
                termination_reason: crate::line_search::line_search::TerminationReason::FunctionEvaluationError,
                num_f_evals: 0,
                num_g_evals: 0,
            }
        });

        let step_size = ls_result.step_size;
        let actual_step_size = step_size * self.config.gradient_scale_factor;
        
        // Update parameters
        let mut new_params = vec_add(current_params, &vec_scale(&direction, step_size));
        
        if let Some(region) = &self.trust_region {
            region.project(&mut new_params);
        }
        
        // Write back
        if let Err(e) = self.write_params(ctx, &new_params) {
            warn!("Failed to write params: {}", e);
        }

        // Update L-BFGS history (even if using steepest descent, we build history)
        // We need gradient at new position.
        // If line search didn't compute it, we might need to.
        // For simplicity, we'll skip L-BFGS update here or do it in the main loop if we had the new gradient.
        // But typically we need to execute graph to get new gradient.
        ctx.graph().execute();
        let new_grads = Self::flatten_tensors(&ctx.gradients);
        
        if let Some(prev_p) = &self.state.prev_params {
            if let Some(prev_g) = &self.state.prev_gradient {
                // We use current_params as "old" (from start of step) and new_params as "new"
                let _ = self.state.lbfgs_state.update(current_params, &new_params, &new_grads, current_grads);
            }
        }

        StepResult {
            step_size: actual_step_size,
            convergence_info: ConvergenceInfo {
                converged: false,
                function_change: None,
            },
        }
    }

    /// Search along the quadratic path: x(t) = x0 + t(1-t)(-g) + t^2 d_lbfgs
    fn search_quadratic(
        &self,
        ctx: &mut OptimizationContext,
        start_params: &[f64],
        neg_grad: &[f64],
        lbfgs_dir: &[f64],
        initial_loss: f64,
        grad_norm_sq: f64,
    ) -> Result<(f64, f64)> {
        // Simple backtracking on the curve
        let c1 = self.config.line_search.c1;
        let mut t = if let Some(prev) = self.state.previous_step_size {
            prev.max(1.0) // Try to be aggressive
        } else {
            1.0
        };
        
        let decay = 0.5;
        let max_iter = self.config.line_search.max_iterations;
        
        // Slope at t=0 is -||g||^2
        let slope = -grad_norm_sq;
        
        for _ in 0..max_iter {
            // x(t) = x0 + t(1-t)(-g) + t^2 d_lbfgs
            //      = x0 + (t - t^2)(-g) + t^2 d_lbfgs
            let term1 = vec_scale(neg_grad, t * (1.0 - t));
            let term2 = vec_scale(lbfgs_dir, t * t);
            let displacement = vec_add(&term1, &term2);
            let mut candidate = vec_add(start_params, &displacement);
            
            if let Some(region) = &self.trust_region {
                region.project(&mut candidate);
            }
            
            let loss = self.evaluate_loss(ctx, &candidate)?;
            
            // Armijo-like condition
            if loss <= initial_loss + c1 * t * slope {
                return Ok((t, loss));
            }
            
            t *= decay;
            if t < self.config.min_step_size {
                break;
            }
        }
        
        Ok((0.0, initial_loss))
    }
}

impl Optimizer for QQNOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(&mut self, ctx: &mut OptimizationContext) -> StepResult {
        let start_time = Instant::now();
        
        // 1. Extract current state
        let current_params = Self::flatten_tensors(&ctx.weights);
        let current_grads = Self::flatten_tensors(&ctx.gradients);
        let current_loss = ctx.loss.data()[0] as f64;
        
        let grad_norm = vec_norm(&current_grads);
        debug!("QQN Step {}: Loss={:.6e}, |g|={:.6e}", self.state.iteration, current_loss, grad_norm);

        // Check convergence
        if grad_norm < self.config.epsilon {
            return StepResult {
                step_size: 0.0,
                convergence_info: ConvergenceInfo {
                    converged: true,
                    function_change: Some(0.0),
                },
            };
        }

        // 2. Update L-BFGS history from previous step if available
        // Note: We do this at the start of the step using (prev_x, curr_x, prev_g, curr_g)
        if let (Some(prev_p), Some(prev_g)) = (&self.state.prev_params, &self.state.prev_gradient) {
            if let Err(e) = self.state.lbfgs_state.update(prev_p, &current_params, &current_grads, prev_g) {
                warn!("L-BFGS update failed: {}", e);
            }
        }

        // 3. Decide strategy
        let result = if self.state.iteration < self.config.min_lbfgs_iterations {
            // Steepest Descent
            self.steepest_descent_step(ctx, &current_params, &current_grads, current_loss)
        } else {
            // QQN Step
            match self.state.lbfgs_state.estimate_optimum(&current_grads) {
                Ok(lbfgs_dir) => {
                    let neg_grad = vec_scale(&current_grads, -1.0);
                    
                    // Perform quadratic path search
                    let search_res = self.search_quadratic(
                        ctx, 
                        &current_params, 
                        &neg_grad, 
                        &lbfgs_dir, 
                        current_loss, 
                        grad_norm * grad_norm
                    );

                    match search_res {
                        Ok((t, final_loss)) => {
                            if t < self.config.min_step_size {
                                debug!("QQN step too small, falling back to steepest descent");
                                self.steepest_descent_step(ctx, &current_params, &current_grads, current_loss)
                            } else {
                                // Apply the step
                                // x(t) = x0 + t(1-t)(-g) + t^2 d_lbfgs
                                let term1 = vec_scale(&neg_grad, t * (1.0 - t));
                                let term2 = vec_scale(&lbfgs_dir, t * t);
                                let displacement = vec_add(&term1, &term2);
                                let mut new_params = vec_add(&current_params, &displacement);
                                
                                if let Some(region) = &self.trust_region {
                                    region.project(&mut new_params);
                                }
                                
                                if let Err(e) = self.write_params(ctx, &new_params) {
                                    warn!("Failed to write params: {}", e);
                                }
                                
                                // Persist step size if significant
                                if t > self.config.min_step_persist {
                                    self.state.previous_step_size = Some(t);
                                } else {
                                    self.state.previous_step_size = None;
                                }

                                let function_decrease = current_loss - final_loss;
                                
                                StepResult {
                                    step_size: t,
                                    convergence_info: ConvergenceInfo {
                                        converged: false,
                                        function_change: Some(function_decrease),
                                    },
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Quadratic search failed: {}, falling back to steepest descent", e);
                            self.steepest_descent_step(ctx, &current_params, &current_grads, current_loss)
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to estimate L-BFGS direction: {}, falling back", e);
                    self.steepest_descent_step(ctx, &current_params, &current_grads, current_loss)
                }
            }
        };

        // 4. Save state for next iteration
        // We need to save the parameters and gradients *before* the update we just did?
        // No, L-BFGS update needs (x_k, x_{k+1}, g_k, g_{k+1}).
        // We are currently at step k. We just computed x_{k+1}.
        // In the NEXT call to step(), we will be at k+1.
        // So we need to store x_k and g_k now.
        self.state.prev_params = Some(current_params);
        self.state.prev_gradient = Some(current_grads);
        
        self.state.iteration += 1;
        
        // Add metadata
        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = start_time.elapsed();
        metadata.optimizer_data.insert("iteration".to_string(), self.state.iteration as f64);
        metadata.optimizer_data.insert("step_size".to_string(), result.step_size);
        
        result
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

fn vec_norm(a: &[f64]) -> f64 {
    a.iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn vec_scale(a: &[f64], s: f64) -> Vec<f64> {
    a.iter().map(|x| x * s).collect()
}

fn vec_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}