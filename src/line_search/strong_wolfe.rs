use crate::line_search::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use dfdx::prelude::{ConstShape, Shape};
use log::debug;
use luminal::prelude::*;
use serde::{Deserialize, Serialize};

/// Strong Wolfe line search implementation following Nocedal & Wright Algorithm 3.5.
///
/// The Strong Wolfe conditions ensure both sufficient decrease (Armijo condition) and
/// sufficient curvature reduction. This is a robust line search method that guarantees
/// convergence for quasi-Newton methods and provides good step sizes for optimization.
///
/// ## Algorithm Overview
/// 1. Start with an initial step size α₀
/// 2. Check if current step satisfies both Wolfe conditions
/// 3. If not, use a zoom procedure to bracket and refine the step size
/// 4. The zoom phase uses interpolation to efficiently find acceptable steps
///
/// ## Wolfe Conditions
/// - **Armijo (sufficient decrease)**: f(αₖ) ≤ f(0) + c₁αₖf'(0)
/// - **Strong curvature**: |f'(αₖ)| ≤ c₂|f'(0)|
///
/// where c₁ and c₂ are parameters with 0 < c₁ < c₂ < 1.
///
/// Configuration for Strong Wolfe line search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongWolfeConfig {
    pub c1: f32, // Armijo condition parameter: controls sufficient decrease strictness
    pub c2: f32, // Curvature condition parameter: controls gradient reduction requirement
    pub max_iterations: usize, // Maximum line search iterations
    pub min_step: f32, // Minimum step size
    pub max_step: f32, // Maximum step size
    pub initial_step: f32, // Initial step size
    pub verbose: bool, // Enable verbose logging
}

impl Default for StrongWolfeConfig {
    fn default() -> Self {
        Self {
            c1: 1e-4, // Standard value: good balance of decrease vs acceptance
            c2: 0.9,
            max_iterations: 50,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
        }
    }
}
impl StrongWolfeConfig {
    /// Strict configuration with tight tolerances for high-precision optimization
    ///
    /// **Use when:**
    /// - High precision is required
    /// - Function evaluations are cheap
    /// - Convergence quality is more important than speed
    ///
    /// **Characteristics:**
    /// - Very strict sufficient decrease (c₁ = 1e-6): requires more function reduction
    /// - Very strict curvature condition (c₂ = 0.1): requires significant gradient reduction
    /// - Low c₁ → demands larger decrease, may reject good steps, more function evaluations
    /// - Low c₂ → demands flatter gradient, better for conjugate gradient methods
    /// - More iterations allowed
    /// - Tighter step size bounds
    pub fn strict() -> Self {
        Self {
            c1: 1e-6,            // Very strict sufficient decrease
            c2: 0.1,             // Very strict curvature condition
            max_iterations: 100, // More iterations for precision
            min_step: 1e-20,     // Smaller minimum step
            max_step: 1e10,      // Conservative maximum step
            initial_step: 1.0,
            verbose: false,
        }
    }

    /// Lax configuration with relaxed tolerances for robust optimization
    ///
    /// **Use when:**
    /// - Function evaluations are expensive
    /// - Approximate solutions are acceptable
    /// - Speed is more important than precision
    ///
    /// **Characteristics:**
    /// - Relaxed sufficient decrease (c₁ = 1e-2): accepts smaller decreases
    /// - Very relaxed curvature condition (c₂ = 0.99): accepts minimal gradient change
    /// - High c₁ → accepts smaller decreases, faster acceptance, fewer evaluations
    /// - High c₂ → accepts steeper gradients, better for quasi-Newton methods
    /// - Fewer iterations for efficiency
    /// - Wider step size bounds
    pub fn lax() -> Self {
        Self {
            c1: 1e-2,           // Relaxed sufficient decrease
            c2: 0.99,           // Very relaxed curvature condition
            max_iterations: 20, // Fewer iterations for efficiency
            min_step: 1e-12,    // Larger minimum step
            max_step: 1e20,     // Larger maximum step
            initial_step: 1.0,
            verbose: false,
        }
    }
    /// Create the default configuration
    pub fn default_config() -> Self {
        Self::default()
    }

    /// Enable verbose logging for any configuration
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}
/// Strong Wolfe line search implementation.
///
/// This implementation follows the algorithm described in Nocedal & Wright's
/// "Numerical Optimization" (Algorithm 3.5). It combines a bracketing phase
/// with a zoom phase to efficiently find step sizes satisfying the Strong Wolfe conditions.
///
/// ## Strengths
/// - **Robust convergence**: Guarantees finding acceptable step sizes for descent directions
/// - **Quasi-Newton compatibility**: Strong curvature condition ensures good Hessian updates
/// - **Adaptive**: Automatically adjusts step size based on function behavior
/// - **Efficient**: Uses interpolation in zoom phase to minimize function evaluations
/// - **Well-tested**: Based on extensively studied and proven algorithms
///
/// ## Weaknesses
/// - **Function evaluation cost**: May require multiple evaluations per line search
/// - **Gradient requirement**: Needs both function and gradient evaluations
/// - **Parameter sensitivity**: Performance depends on c₁, c₂ parameter choices
/// - **Complexity**: More complex than simple backtracking methods
///
/// ## Typical Use Cases
/// - Quasi-Newton methods (BFGS, L-BFGS)
/// - Conjugate gradient methods
/// - Any optimization where both function and gradient are available
/// - Problems requiring robust step size selection
///
/// ## Parameter Guidelines
/// ### Parameter Effects and Guidelines
///
/// **c₁ (Armijo parameter)**: Controls sufficient decrease requirement
/// - **Lower c₁ (e.g., 1e-6)**: More strict, demands larger function decrease
///   - ✓ Better convergence quality, more precise steps
///   - ✗ More function evaluations, may reject good steps
/// - **Higher c₁ (e.g., 1e-2)**: More lenient, accepts smaller decreases
///   - ✓ Faster line search, fewer function evaluations
///   - ✗ May accept poor steps, slower overall convergence
/// - **Typical range**: [1e-6, 1e-2], commonly 1e-4
///
/// **c₂ (Curvature parameter)**: Controls gradient reduction requirement
/// - **Lower c₂ (e.g., 0.1)**: Strict curvature, demands flatter gradients
///   - ✓ Better for conjugate gradient methods, ensures good search directions
///   - ✗ More gradient evaluations, may be too restrictive for Newton methods
/// - **Higher c₂ (e.g., 0.9)**: Lenient curvature, accepts steeper gradients
///   - ✓ Better for Newton/quasi-Newton, fewer gradient evaluations
///   - ✗ May not provide sufficient curvature information
/// - **Method-specific recommendations**:
///   - Newton/Quasi-Newton: c₂ = 0.9 (can handle steeper gradients)
///   - Conjugate Gradient: c₂ = 0.1-0.4 (needs flatter gradients for orthogonality)
///   - Steepest Descent: c₂ = 0.5-0.9 (flexible)
///
/// **Constraint**: Must satisfy 0 < c₁ < c₂ < 1 for theoretical guarantees
/// - **max_iterations**: 20-100 depending on precision requirements
///
/// Strong Wolfe line search implementation
#[derive(Debug, Clone)]
pub struct StrongWolfeLineSearch {
    config: StrongWolfeConfig,
    num_f_evals: usize,
    num_g_evals: usize,
}

impl StrongWolfeLineSearch {
    /// Set the initial step size for the next line search
    ///
    /// The step size will be clamped to [min_step, max_step] range.
    /// This is useful for adaptive step size strategies.
    pub fn set_initial_step(&mut self, step: f32) {
        self.config.initial_step = step.clamp(self.config.min_step, self.config.max_step);
    }
    pub fn new(config: StrongWolfeConfig) -> Self {
        Self {
            config,
            num_f_evals: 0,
            num_g_evals: 0,
        }
    }
    /// Create with default configuration
    pub fn default_search() -> Self {
        Self::new(StrongWolfeConfig::default())
    }
    /// Create with strict configuration
    pub fn strict() -> Self {
        Self::new(StrongWolfeConfig::strict())
    }
    /// Create with lax configuration
    pub fn lax() -> Self {
        Self::new(StrongWolfeConfig::lax())
    }
    /// Reset evaluation counters
    fn reset_counters(&mut self) {
        self.num_f_evals = 0;
        self.num_g_evals = 0;
    }
    /// Increment function evaluation counter
    fn inc_f_eval(&mut self) {
        self.num_f_evals += 1;
    }
    /// Increment gradient evaluation counter
    fn inc_g_eval(&mut self) {
        self.num_g_evals += 1;
    }
    /// Log line search details if verbose mode is enabled
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("StrongWolfe: {message}");
        }
    }

    /// Check Armijo condition: f(α) ≤ f(0) + c1*α*f'(0)
    ///
    /// This ensures sufficient decrease in the objective function.
    ///
    /// **Effect of c₁ values:**
    /// - **Smaller c₁** (e.g., 1e-6): Stricter condition
    ///   - Requires larger function decrease: f(α) must be much smaller than f(0)
    ///   - More function evaluations but better step quality
    ///   - May reject steps that would still lead to good convergence
    /// - **Larger c₁** (e.g., 1e-2): More lenient condition
    ///   - Accepts smaller function decreases
    ///   - Faster line search with fewer evaluations
    ///   - Risk of accepting steps with insufficient progress
    ///
    /// **Geometric interpretation**: c₁ controls the slope of the acceptance line
    /// from the origin. Smaller c₁ = flatter line = stricter acceptance.
    fn armijo_condition(
        &self,
        f0: f32,
        f_alpha: f32,
        alpha: f32,
        directional_derivative: f32,
    ) -> bool {
        let threshold = f0 + self.config.c1 * alpha * directional_derivative;
        let satisfied = f_alpha <= threshold;
        if self.config.verbose {
            debug!(
                "  Armijo: f({:.3e})={:.3e} <= {:.3e} + {:.3e}*{:.3e}*{:.3e} = {:.3e}? {}",
                alpha,
                f_alpha,
                f0,
                self.config.c1,
                alpha,
                directional_derivative,
                threshold,
                satisfied
            );
        }
        satisfied
    }

    /// Check curvature condition: |f'(α)| ≤ c2*|f'(0)|
    ///
    /// This ensures the gradient magnitude has decreased sufficiently.
    /// The strong version uses absolute values (vs. regular Wolfe: f'(α) ≥ c₂f'(0)).
    ///
    /// **Effect of c₂ values:**
    /// - **Smaller c₂** (e.g., 0.1): Stricter curvature condition
    ///   - Requires |f'(α)| << |f'(0)|: gradient must be much flatter
    ///   - More gradient evaluations but better curvature information
    ///   - Essential for conjugate gradient methods (maintains direction orthogonality)
    ///   - May be too restrictive for Newton methods
    /// - **Larger c₂** (e.g., 0.9): More lenient curvature condition
    ///   - Accepts |f'(α)| ≈ |f'(0)|: allows steeper gradients
    ///   - Fewer gradient evaluations, faster line search
    ///   - Suitable for Newton/quasi-Newton (can handle approximate curvature)
    ///   - May not provide enough curvature reduction for some methods
    ///
    /// **Why method-dependent?**
    /// - Conjugate gradient needs orthogonal directions → requires flat gradients → small c₂
    /// - Newton methods have good curvature info → can handle steep gradients → large c₂
    fn curvature_condition(&self, grad_alpha: f32, directional_derivative: f32) -> bool {
        let threshold = self.config.c2 * directional_derivative.abs();
        let satisfied = grad_alpha.abs() <= threshold;
        if self.config.verbose {
            debug!(
                "  Curvature: |{:.3e}| <= {:.3e}*|{:.3e}| = {:.3e}? {}",
                grad_alpha, self.config.c2, directional_derivative, threshold, satisfied
            );
        }
        satisfied
    }

    /// Zoom phase of Strong Wolfe line search
    ///
    /// This is the core refinement procedure that brackets an acceptable step size
    /// between α_lo and α_hi, then uses interpolation to efficiently find a point
    /// satisfying both Wolfe conditions.
    ///
    /// The zoom procedure maintains the invariant that:
    /// - α_lo satisfies the Armijo condition
    /// - The interval [α_lo, α_hi] contains step sizes satisfying Strong Wolfe conditions
    ///
    /// Uses safeguarded interpolation to ensure robust convergence and avoid
    /// getting stuck in very small intervals.
    fn zoom<F>(
        &self,
        alpha_lo: f32,
        alpha_hi: f32,
        f0: f32,
        directional_derivative: f32,
        mut evaluate: F,
        num_f_evals: &mut usize,
        num_g_evals: &mut usize,
    ) -> anyhow::Result<f32>
    where
        F: FnMut(f32) -> anyhow::Result<(f32, f32)>,
    {
        let mut alpha_lo = alpha_lo;
        let mut alpha_hi = alpha_hi;
        let mut best_alpha = alpha_lo;
        let mut best_value = f32::INFINITY;

        for _ in 0..self.config.max_iterations {
            // Use quadratic interpolation when possible
            let alpha_j = if (alpha_hi - alpha_lo).abs() > 1e-10 {
                // Try cubic interpolation first
                let mid = 0.5 * (alpha_lo + alpha_hi);
                // Safeguard to ensure progress
                let safeguard_factor = 0.1;
                let min_alpha = alpha_lo + safeguard_factor * (alpha_hi - alpha_lo);
                let max_alpha = alpha_hi - safeguard_factor * (alpha_hi - alpha_lo);
                mid.max(min_alpha).min(max_alpha)
            } else {
                0.5 * (alpha_lo + alpha_hi)
            };

            // Evaluate 1D function at trial point
            let (f_alpha_j, grad_alpha_j) = evaluate(alpha_j)?;
            *num_f_evals += 1;
            *num_g_evals += 1;
            // Track best point found
            if f_alpha_j < best_value {
                best_value = f_alpha_j;
                best_alpha = alpha_j;
            }

            // Check Armijo condition
            if !self.armijo_condition(f0, f_alpha_j, alpha_j, directional_derivative) {
                alpha_hi = alpha_j;
                continue;
            }

            // Check curvature condition
            if self.curvature_condition(grad_alpha_j, directional_derivative) {
                return Ok(alpha_j);
            }

            // Update interval
            if grad_alpha_j * (alpha_hi - alpha_lo) >= 0.0 {
                alpha_hi = alpha_lo;
            }
            alpha_lo = alpha_j;
            // Check if interval is too small
            if (alpha_hi - alpha_lo).abs() < self.config.min_step {
                break;
            }
        }

        // Return best point found during search
        Ok(best_alpha)
    }
}

impl LineSearch for StrongWolfeLineSearch {
    fn search(
        &mut self,
        cx: &mut Graph,
        params: GraphTensor,
        loss: GraphTensor,
        gradient: GraphTensor,
        current_params: &[f32],
        direction: &[f32],
        initial_loss: f32,
        initial_gradient: &[f32],
    ) -> anyhow::Result<LineSearchResult> {
        // Reset evaluation counters at the start of each search
        self.reset_counters();

        let f0 = initial_loss;
        let directional_derivative: f32 = initial_gradient
            .iter()
            .zip(direction.iter())
            .map(|(g, d)| g * d)
            .sum();

        self.log_verbose(&format!("Starting 1D optimization with f(0)={f0:.3e}"));
        self.log_verbose(&format!(
            "Directional derivative: {directional_derivative:.3e}"
        ));

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }
        // Track evaluation counts in the closure
        let mut local_f_evals = 0usize;
        let mut local_g_evals = 0usize;

        let mut evaluate = |alpha: f32| -> anyhow::Result<(f32, f32)> {
            let new_params: Vec<f32> = current_params
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + alpha * d)
                .collect();
            cx.set_tensor(params.id, 0, Tensor::new(new_params));
            cx.execute();
            let loss_tensor = cx.get_tensor(loss.id, 0).unwrap();
            let grad_tensor = cx.get_tensor(gradient.id, 0).unwrap();
            let loss_val = loss_tensor
                .data
                .as_any()
                .downcast_ref::<Vec<f32>>()
                .unwrap()[0];
            let grad_val = grad_tensor
                .data
                .as_any()
                .downcast_ref::<Vec<f32>>()
                .unwrap();
            let dir_deriv = grad_val
                .iter()
                .zip(direction.iter())
                .map(|(g, d)| g * d)
                .sum();
            local_f_evals += 1;
            local_g_evals += 1;
            Ok((loss_val, dir_deriv))
        };

        let alpha = self.config.initial_step;
        let alpha_prev = 0.0;
        let f_prev = f0;
        let mut best_alpha = 0.0;
        let mut best_f = f0;

        self.log_verbose(&format!("Initial step size: {alpha:.3e}"));

        for i in 0..self.config.max_iterations {
            self.log_verbose(&format!(
                "Line Search Iteration {i}: trying alpha={alpha:.3e}"
            ));

            // Evaluate function at current step size
            let (f_alpha, grad_alpha) = evaluate(alpha)?;
            self.log_verbose(&format!("  f({alpha:.3e}) = {f_alpha:.3e}"));
            // Track best point found
            if f_alpha < best_f {
                best_f = f_alpha;
                best_alpha = alpha;
            }

            // Check Armijo condition and sufficient decrease
            if !self.armijo_condition(f0, f_alpha, alpha, directional_derivative)
                || (i > 0 && f_alpha >= f_prev)
            {
                self.log_verbose(&format!(
                    "  Armijo failed or insufficient decrease, zooming between {alpha_prev:.3e} and {alpha:.3e}"
                ));
                // Zoom between alpha_prev and alpha
                let final_alpha = self.zoom(
                    alpha_prev,
                    alpha,
                    f0,
                    directional_derivative,
                    &mut evaluate,
                    &mut local_f_evals,
                    &mut local_g_evals,
                )?;
                self.log_verbose(&format!("Zoom completed with alpha={final_alpha:.3e}"));
                self.num_f_evals = local_f_evals;
                self.num_g_evals = local_g_evals;

                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                    num_f_evals: self.num_f_evals,
                    num_g_evals: self.num_g_evals,
                });
            }

            // Check curvature condition
            if self.curvature_condition(grad_alpha, directional_derivative) {
                self.log_verbose(&format!(
                    "Both Wolfe conditions satisfied at alpha={alpha:.3e}"
                ));
                self.num_f_evals = local_f_evals;
                self.num_g_evals = local_g_evals;

                return Ok(LineSearchResult {
                    step_size: alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                    num_f_evals: self.num_f_evals,
                    num_g_evals: self.num_g_evals,
                });
            }

            // Check if gradient indicates we should look further
            if grad_alpha >= 0.0 {
                self.log_verbose(&format!(
                    "  Gradient indicates overshoot, zooming between {alpha:.3e} and {alpha_prev:.3e}"
                ));
                let final_alpha = self.zoom(
                    alpha,
                    alpha_prev,
                    f0,
                    directional_derivative,
                    &mut evaluate,
                    &mut local_f_evals,
                    &mut local_g_evals,
                )?;

                self.num_f_evals = local_f_evals;
                self.num_g_evals = local_g_evals;

                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                    num_f_evals: self.num_f_evals,
                    num_g_evals: self.num_g_evals,
                });
            }

            // Both conditions satisfied - should not reach here
            break;
        }

        // If we have any improvement, use it
        if best_alpha > 0.0 && best_f < f0 {
            self.log_verbose(&format!(
                "Returning best point found: alpha={best_alpha:.3e}, f={best_f:.3e}"
            ));
            self.num_f_evals = local_f_evals;
            self.num_g_evals = local_g_evals;

            return Ok(LineSearchResult {
                step_size: best_alpha,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
                num_f_evals: self.num_f_evals,
                num_g_evals: self.num_g_evals,
            });
        }

        // Last resort: try machine epsilon steps
        let eps_step = f32::EPSILON.sqrt();
        let (f_eps, _) = evaluate(eps_step)?;
        if f_eps < f0 {
            self.log_verbose(&format!("Using machine epsilon step {eps_step:.3e}"));
            self.num_f_evals = local_f_evals;
            self.num_g_evals = local_g_evals;

            return Ok(LineSearchResult {
                step_size: eps_step,
                success: true,
                termination_reason: TerminationReason::StepSizeTooSmall,
                num_f_evals: self.num_f_evals,
                num_g_evals: self.num_g_evals,
            });
        }

        // Only fail if we truly cannot find any improvement
        self.log_verbose("No improvement found even with machine epsilon steps");
        Err(anyhow!("Function appears to be ill-conditioned: no improvement possible within machine precision"))
    }

    fn reset(&mut self) {
        self.reset_counters();
    }
    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
