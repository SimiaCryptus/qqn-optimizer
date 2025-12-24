use crate::line_search::{LineSearch, LineSearchResult, TerminationReason};
use crate::optimizers::{GDConfig, GDOptimizer};
use crate::region::trust_region::{TrustRegion, TrustRegionConfig, TrustRegionOptimizer};
use crate::optimizers::optimizer::OptimizationContext;
use anyhow::{anyhow, Result};
use luminal::prelude::*;

/// Configuration parameters for the backtracking line search algorithm.
///
/// The backtracking line search uses the Armijo condition to ensure sufficient decrease
/// in the objective function. Starting from an initial step size, it repeatedly reduces
/// the step by a factor `rho` until the Armijo condition is satisfied:
///
/// f(x + α*p) ≤ f(x) + c1*α*∇f(x)ᵀp
///
/// where:
/// - α is the step size
/// - p is the search direction
/// - c1 is the Armijo parameter (typically 1e-4)
///
/// # Parameter Guidelines
/// - `c1`: Controls strictness of sufficient decrease (0 < c1 < 1, typically 1e-4)
/// - `rho`: Backtracking factor (0 < rho < 1, typically 0.5)
/// - Smaller `rho` means more aggressive backtracking (faster convergence but more function evaluations)
/// - Larger `c1` means stricter acceptance criteria (more conservative steps)
#[derive(Debug, Clone)]
pub struct BacktrackingConfig {
    /// Armijo condition parameter (0 < c1 < 1, typically 1e-4).
    /// Controls the required amount of decrease in the objective function.
    /// Smaller values are more permissive, larger values are more strict.
    pub c1: f64,

    /// Backtracking factor (0 < rho < 1, typically 0.5).
    /// The step size is multiplied by this factor in each backtracking iteration.
    /// Smaller values lead to more aggressive backtracking.
    pub rho: f64,

    /// Maximum number of backtracking iterations before giving up.
    pub max_iterations: usize,

    /// Minimum allowable step size. If the step becomes smaller than this,
    /// the algorithm will attempt to use this minimum step if it provides improvement.
    pub min_step: f64,

    /// Initial step size to try. This is reset for each line search.
    pub initial_step: f64,

    /// Maximum allowable step size (typically f64::MAX for no limit).
    pub max_step: f64,
}

impl Default for BacktrackingConfig {
    fn default() -> Self {
        Self {
            c1: 1e-4,            // Standard Armijo parameter
            rho: 0.5,            // Standard backtracking factor
            max_iterations: 100, // More generous iteration limit
            min_step: 1e-12,     // More practical minimum step
            initial_step: 1.0,
            max_step: f64::MAX, // No upper limit by default
        }
    }
}
impl BacktrackingConfig {
    /// Create a strict configuration with conservative parameters.
    ///
    /// This configuration is suitable for problems where robustness is more important
    /// than speed, or when dealing with noisy or ill-conditioned functions.
    ///
    /// # Parameters
    /// - Stricter Armijo condition (c1 = 1e-3) - requires more decrease
    /// - More aggressive backtracking (rho = 0.3) - reduces step size faster
    /// - Higher iteration limit (200) - allows more thorough search
    /// - Smaller minimum step (1e-15) - can handle very small steps
    ///
    /// # Use Cases
    /// - Ill-conditioned optimization problems
    /// - Functions with numerical noise
    /// - When convergence reliability is critical
    pub fn strict() -> Self {
        Self {
            c1: 1e-3,            // Stricter Armijo condition
            rho: 0.3,            // More aggressive backtracking
            max_iterations: 200, // More iterations for thorough search
            min_step: 1e-15,     // Smaller minimum step
            initial_step: 1.0,
            max_step: f64::MAX, // No upper limit by default
        }
    }
    /// Create a lax configuration with permissive parameters.
    ///
    /// This configuration prioritizes speed over robustness and is suitable for
    /// well-behaved functions where fast convergence is desired.
    ///
    /// # Parameters
    /// - Relaxed Armijo condition (c1 = 1e-6) - accepts smaller decreases
    /// - Conservative backtracking (rho = 0.8) - reduces step size slowly
    /// - Lower iteration limit (50) - gives up sooner for speed
    /// - Larger minimum step (1e-10) - avoids very small steps
    ///
    /// # Use Cases
    /// - Well-conditioned smooth functions
    /// - When computational budget is limited
    /// - Initial exploration phases of optimization
    pub fn lax() -> Self {
        Self {
            c1: 1e-6,           // More permissive Armijo condition
            rho: 0.8,           // Less aggressive backtracking
            max_iterations: 50, // Fewer iterations for speed
            min_step: 1e-10,    // Larger minimum step
            initial_step: 1.0,
            max_step: f64::MAX, // No upper limit by default
        }
    }
    /// Create the default configuration.
    ///
    /// Equivalent to `BacktrackingConfig::default()`. Provides balanced parameters
    /// suitable for most optimization problems.
    pub fn default_config() -> Self {
        Self::default()
    }
}

/// Backtracking line search implementation using the Armijo rule.
///
/// This is a simple but robust line search method that starts with an initial step size
/// and repeatedly reduces it by a constant factor until the Armijo sufficient decrease
/// condition is satisfied:
///
/// f(x + α*p) ≤ f(x) + c1*α*∇f(x)ᵀp
///
/// # Algorithm
/// 1. Start with initial step size α₀
/// 2. Evaluate f(x + α*p)
/// 3. If Armijo condition is satisfied, return α
/// 4. Otherwise, set α ← ρ*α and repeat from step 2
/// 5. Stop if α becomes too small or max iterations reached
///
/// # Strengths
/// - Simple and robust
/// - Guaranteed to find acceptable step (under mild conditions)
/// - Low memory requirements
/// - Works well for most optimization algorithms
///
/// # Weaknesses
/// - Only uses first-order information (no curvature)
/// - May take many iterations for ill-conditioned problems
/// - Can be conservative (smaller steps than necessary)
/// - No strong Wolfe conditions (may not ensure good curvature condition)
///
/// # Typical Use Cases
/// - Gradient descent and quasi-Newton methods
/// - When simplicity and robustness are preferred over efficiency
/// - Problems where second-order information is unavailable or unreliable
#[derive(Debug, Clone)]
pub struct BacktrackingLineSearch {
    config: BacktrackingConfig,
}

impl BacktrackingLineSearch {
    /// Set the initial step size for the next line search.
    ///
    /// The step size will be clamped to the range [min_step, max_step].
    /// This is useful for adaptive step size strategies where the initial
    /// step is based on previous iterations.
    /// * `step` - The desired initial step size
    pub fn set_initial_step(&mut self, step: f64) {
        self.config.initial_step = step.clamp(self.config.min_step, self.config.max_step);
    }
    /// Create a new backtracking line search with the given configuration.
    pub fn new(config: BacktrackingConfig) -> Self {
        Self { config }
    }

    /// Create a backtracking line search with default configuration.
    ///
    /// Uses balanced parameters suitable for most optimization problems:
    /// - c1 = 1e-4 (standard Armijo parameter)
    /// - rho = 0.5 (moderate backtracking)
    /// - max_iterations = 100
    /// - min_step = 1e-12
    pub fn default_search() -> Self {
        Self::new(BacktrackingConfig::default())
    }

    /// Create a strict backtracking line search with conservative parameters.
    ///
    /// This variant prioritizes robustness over speed and is recommended for:
    /// - Ill-conditioned problems
    /// - Noisy objective functions  
    /// - When convergence reliability is critical
    ///
    /// Uses stricter acceptance criteria and more thorough search.
    pub fn strict() -> Self {
        Self::new(BacktrackingConfig::strict())
    }

    /// Create a lax backtracking line search with permissive parameters.
    ///
    /// This variant prioritizes speed over robustness and is recommended for:
    /// - Well-conditioned smooth functions
    /// - When computational budget is limited
    /// - Initial exploration phases
    ///
    /// Uses more permissive acceptance criteria for faster convergence.
    pub fn lax() -> Self {
        Self::new(BacktrackingConfig::lax())
    }

    /// Create a backtracking line search optimized for robust optimization.
    ///
    /// This variant is designed for maximum reliability when dealing with
    /// challenging optimization problems. It uses:
    /// - Standard Armijo parameter (c1 = 1e-4) for balanced acceptance
    /// - Moderate backtracking (rho = 0.5)
    /// - Very high iteration limit (500) for thorough search
    /// - Very small minimum step (1e-16) to handle difficult cases
    ///
    /// # Use Cases
    /// - Highly ill-conditioned problems
    /// - Functions with multiple scales
    /// - When other line search methods fail
    /// - Research or exploratory optimization
    pub fn robust() -> Self {
        Self::new(BacktrackingConfig {
            c1: 1e-4,            // Standard Armijo parameter
            rho: 0.5,            // Standard backtracking
            max_iterations: 500, // High iteration limit
            min_step: 1e-16,     // Very small minimum step
            initial_step: 1.0,
            max_step: f64::MAX, // No upper limit by default
        })
    }
}

impl LineSearch for BacktrackingLineSearch {
    fn search(
        &mut self,
        context: OptimizationContext,
        current_params: &[f64],
        direction: &[f64],
        initial_loss: f64,
        initial_gradient: &[f64],
        trust_region: Option<&dyn TrustRegion>,
    ) -> Result<LineSearchResult> {
        let directional_derivative: f64 = initial_gradient
            .iter()
            .zip(direction.iter())
            .map(|(g, d)| g * d)
            .sum();

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut best_alpha = 0.0;
        let mut best_f = initial_loss;
        let mut num_f_evals = 0;
        let num_g_evals = 0;
        let params = context.weights[0];

        for _ in 0..self.config.max_iterations {
            let mut candidate_params: Vec<f64> = current_params
                .iter()
                .zip(direction.iter())
                .map(|(x, d)| x + alpha * d)
                .collect();
            if let Some(tr) = trust_region {
                tr.project(&mut candidate_params);
            }


            // Update parameters in graph
            context.graph().set_tensor(params.id, 0, Tensor::new(candidate_params.iter().map(|&x| x as f32).collect::<Vec<f32>>()));

            // Execute graph
            context.graph().execute();
            num_f_evals += 1;

            // Get loss value
            let f_alpha = context
                .loss
                .data()
                .as_any()
                .downcast_ref::<Vec<f32>>()
                .ok_or(anyhow!("Failed to downcast tensor data"))?[0] as f64;

            // Track best point
            if f_alpha < best_f {
                best_f = f_alpha;
                best_alpha = alpha;
            }

            // Check Armijo condition
            let armijo_threshold = initial_loss + self.config.c1 * alpha * directional_derivative;
            if f_alpha <= armijo_threshold {
                return Ok(LineSearchResult {
                    step_size: alpha,
                    success: true,
                    termination_reason: TerminationReason::ArmijoConditionSatisfied,
                    num_f_evals,
                    num_g_evals,
                });
            }

            // Backtrack
            alpha *= self.config.rho;

            if alpha < self.config.min_step {
                // Try minimum step
                let mut min_step_params: Vec<f64> = current_params
                    .iter()
                    .zip(direction.iter())
                    .map(|(x, d)| x + self.config.min_step * d)
                    .collect();

                if let Some(tr) = trust_region {
                    tr.project(&mut min_step_params);
                }

                context.graph().set_tensor(params.id, 0, Tensor::new(min_step_params.iter().map(|&x| x as f32).collect::<Vec<f32>>()));
                context.graph().execute();
                num_f_evals += 1;
                let f_min = context
                    .loss
                    .data()
                    .as_any()
                    .downcast_ref::<Vec<f32>>()
                    .ok_or(anyhow!("Failed to downcast tensor data"))?[0] as f64;

                if f_min < initial_loss {
                    return Ok(LineSearchResult {
                        step_size: self.config.min_step,
                        success: true,
                        termination_reason: TerminationReason::StepSizeTooSmall,
                        num_f_evals,
                        num_g_evals,
                    });
                }
                break;
            }
        }

        // Return best point found if any improvement
        if best_alpha > 0.0 && best_f < initial_loss {
            return Ok(LineSearchResult {
                step_size: best_alpha,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
                num_f_evals,
                num_g_evals,
            });
        }

        // Try machine epsilon
        let eps_step = f64::EPSILON.sqrt();
        let mut eps_params: Vec<f64> = current_params
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + eps_step * d)
            .collect();

        if let Some(tr) = trust_region {
            tr.project(&mut eps_params);
        }

        context.graph().set_tensor(params.id, 0, Tensor::new(eps_params.iter().map(|&x| x as f32).collect::<Vec<f32>>()));
        context.graph().execute();
        num_f_evals += 1;
        let f_eps = context
            .loss
            .data()
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or(anyhow!("Failed to downcast tensor data"))?[0] as f64;

        if f_eps < initial_loss {
            return Ok(LineSearchResult {
                step_size: eps_step,
                success: true,
                termination_reason: TerminationReason::StepSizeTooSmall,
                num_f_evals,
                num_g_evals,
            });
        }

        Err(anyhow!("Function appears to be ill-conditioned: no improvement possible within machine precision"))
    }

    /// Reset the line search state.
    ///
    /// For backtracking line search, this is a no-op since the algorithm is stateless.
    /// Each call to `optimize_1d` is independent of previous calls.
    fn reset(&mut self) {
        // Backtracking line search is stateless, nothing to reset
    }
    /// Create a boxed clone of this line search instance.
    ///
    /// This is used by the optimization framework to create independent copies
    /// of the line search for different optimization runs.
    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
    }
    /// Get a mutable reference to this instance as `Any` for downcasting.
    ///
    /// This allows users to access backtracking-specific methods like
    /// `set_initial_step` when they have a `Box<dyn LineSearch>`.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}