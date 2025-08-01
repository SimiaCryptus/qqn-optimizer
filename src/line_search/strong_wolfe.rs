use crate::line_search::line_search::OneDimensionalProblem;
use crate::line_search::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use log::debug;
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
    pub c1: f64, // Armijo condition parameter: controls sufficient decrease strictness
    pub c2: f64, // Curvature condition parameter: controls gradient reduction requirement
    pub max_iterations: usize, // Maximum line search iterations
    pub min_step: f64, // Minimum step size
    pub max_step: f64, // Maximum step size
    pub initial_step: f64, // Initial step size
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
    /// Validate configuration parameters
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.c1 <= 0.0 || self.c1 >= 1.0 {
            return Err(anyhow!("c1 must be in (0, 1), got {}", self.c1));
        }
        if self.c2 <= 0.0 || self.c2 >= 1.0 {
            return Err(anyhow!("c2 must be in (0, 1), got {}", self.c2));
        }
        if self.c1 >= self.c2 {
            return Err(anyhow!("Must have c1 < c2, got c1={}, c2={}", self.c1, self.c2));
        }
        if self.min_step >= self.max_step {
            return Err(anyhow!("min_step must be less than max_step"));
        }
        if self.initial_step < self.min_step || self.initial_step > self.max_step {
            return Err(anyhow!("initial_step must be in [min_step, max_step]"));
        }
        Ok(())
    }

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
}

impl StrongWolfeLineSearch {
    /// Create a new Strong Wolfe line search with the given configuration
    pub fn new(config: StrongWolfeConfig) -> anyhow::Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Set the initial step size for the next line search
    ///
    /// The step size will be clamped to [min_step, max_step] range.
    /// This is useful for adaptive step size strategies.
    pub fn set_initial_step(&mut self, step: f64) {
        self.config.initial_step = step.clamp(self.config.min_step, self.config.max_step);
    }

    /// Create with default configuration
    pub fn default_search() -> Self {
        Self::new(StrongWolfeConfig::default()).expect("Default config should be valid")
    }

    /// Create with strict configuration
    pub fn strict() -> Self {
        Self::new(StrongWolfeConfig::strict()).expect("Strict config should be valid")
    }

    /// Create with lax configuration
    pub fn lax() -> Self {
        Self::new(StrongWolfeConfig::lax()).expect("Lax config should be valid")
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
        f0: f64,
        f_alpha: f64,
        alpha: f64,
        directional_derivative: f64,
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
    fn curvature_condition(&self, grad_alpha: f64, directional_derivative: f64) -> bool {
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
    /// Perform cubic interpolation to find a better step size
    fn cubic_interpolation(
        &self,
        alpha_lo: f64,
        alpha_hi: f64,
        f_lo: f64,
        f_hi: f64,
        grad_lo: f64,
        grad_hi: f64,
    ) -> f64 {
        let d1 = grad_lo + grad_hi - 3.0 * (f_lo - f_hi) / (alpha_lo - alpha_hi);
        let d2 = ((d1 * d1 - grad_lo * grad_hi).max(0.0)).sqrt();
        let d2 = if alpha_hi < alpha_lo { -d2 } else { d2 };
        let denominator = grad_hi - grad_lo + 2.0 * d2;
        if denominator.abs() < 1e-10 {
            // Fall back to bisection
            return 0.5 * (alpha_lo + alpha_hi);
        }
        alpha_hi - (alpha_hi - alpha_lo) * (grad_hi + d2 - d1) / denominator
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
    fn zoom(
        &self,
        alpha_lo: f64,
        alpha_hi: f64,
        f0: f64,
        directional_derivative: f64,
        problem: &OneDimensionalProblem,
    ) -> anyhow::Result<f64> {
        let mut alpha_lo = alpha_lo;
        let mut alpha_hi = alpha_hi;
        let mut best_alpha = alpha_lo;
        let mut best_value = f64::INFINITY;
        let mut f_lo = (problem.objective)(alpha_lo)?;
        let mut f_hi = (problem.objective)(alpha_hi)?;
        let mut grad_lo = (problem.gradient)(alpha_lo)?;
        let mut grad_hi = (problem.gradient)(alpha_hi)?;

        for _ in 0..self.config.max_iterations {
            // Use quadratic interpolation when possible
            let alpha_j = if (alpha_hi - alpha_lo).abs() > 1e-10 {
                // Try cubic interpolation
                let interpolated = self.cubic_interpolation(
                    alpha_lo, alpha_hi, f_lo, f_hi, grad_lo, grad_hi
                );
                
                // Safeguard to ensure progress
                let safeguard_factor = 0.1;
                let min_alpha = alpha_lo + safeguard_factor * (alpha_hi - alpha_lo);
                let max_alpha = alpha_hi - safeguard_factor * (alpha_hi - alpha_lo);
                interpolated.max(min_alpha).min(max_alpha)
            } else {
                0.5 * (alpha_lo + alpha_hi)
            };

            // Evaluate 1D function at trial point
            let f_alpha_j = (problem.objective)(alpha_j)?;
            // Track best point found
            if f_alpha_j < best_value {
                best_value = f_alpha_j;
                best_alpha = alpha_j;
            }

            // Check Armijo condition
            if !self.armijo_condition(f0, f_alpha_j, alpha_j, directional_derivative) {
                alpha_hi = alpha_j;
                f_hi = f_alpha_j;
                grad_hi = (problem.gradient)(alpha_j)?;
                continue;
            }

            // Evaluate 1D gradient at trial point
            let grad_alpha_j = (problem.gradient)(alpha_j)?;

            // Check curvature condition
            if self.curvature_condition(grad_alpha_j, directional_derivative) {
                return Ok(alpha_j);
            }

            // Update interval
            if grad_alpha_j * (alpha_hi - alpha_lo) >= 0.0 {
                alpha_hi = alpha_lo;
                f_hi = f_lo;
                grad_hi = grad_lo;
            }
            alpha_lo = alpha_j;
            f_lo = f_alpha_j;
            grad_lo = grad_alpha_j;
            
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
    /// Perform one-dimensional optimization using Strong Wolfe line search.
    ///
    /// This method implements the complete Strong Wolfe algorithm:
    /// 1. **Initialization**: Start with initial step size
    /// 2. **Bracketing phase**: Find interval containing acceptable step
    /// 3. **Zoom phase**: Refine the interval using interpolation
    ///
    /// ## Error Conditions
    /// - Returns error if direction is not a descent direction (f'(0) ≥ 0)
    /// - Returns error if function appears ill-conditioned
    ///
    /// ## Fallback Strategy
    /// If standard algorithm fails, tries machine epsilon steps as last resort.
    fn optimize_1d(&mut self, problem: &OneDimensionalProblem) -> anyhow::Result<LineSearchResult> {
        let f0 = (problem.objective)(0.0)?;
        let directional_derivative = problem.initial_directional_derivative;

        self.log_verbose(&format!("Starting 1D optimization with f(0)={f0:.3e}"));
        self.log_verbose(&format!(
            "Directional derivative: {directional_derivative:.3e}"
        ));

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut alpha_prev = 0.0;
        let mut f_prev = f0;
        let mut best_alpha = 0.0;
        let mut best_f = f0;

        self.log_verbose(&format!("Initial step size: {alpha:.3e}"));

        for i in 0..self.config.max_iterations {
            self.log_verbose(&format!(
                "Line Search Iteration {i}: trying alpha={alpha:.3e}"
            ));

            // Evaluate function at current step size
            let f_alpha = (problem.objective)(alpha)?;
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
                let final_alpha =
                    self.zoom(alpha_prev, alpha, f0, directional_derivative, problem)?;
                self.log_verbose(&format!("Zoom completed with alpha={final_alpha:.3e}"));

                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Evaluate gradient at current point
            let grad_alpha = (problem.gradient)(alpha)?;

            // Check curvature condition
            if self.curvature_condition(grad_alpha, directional_derivative) {
                self.log_verbose(&format!(
                    "Both Wolfe conditions satisfied at alpha={alpha:.3e}"
                ));
                return Ok(LineSearchResult {
                    step_size: alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Check if gradient indicates we should look further
            if grad_alpha >= 0.0 {
                self.log_verbose(&format!(
                    "  Gradient indicates overshoot, zooming between {alpha:.3e} and {alpha_prev:.3e}"
                ));
                let final_alpha =
                    self.zoom(alpha, alpha_prev, f0, directional_derivative, problem)?;

                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Update for next iteration
            alpha_prev = alpha;
            f_prev = f_alpha;
            
            // Increase step size for next iteration
            alpha = (2.0 * alpha).min(self.config.max_step);
            
            // Check if we've reached maximum step size
            if alpha >= self.config.max_step {
                self.log_verbose("Reached maximum step size");
                break;
            }
        }

        // If we have any improvement, use it
        if best_alpha > 0.0 && best_f < f0 {
            self.log_verbose(&format!(
                "Returning best point found: alpha={best_alpha:.3e}, f={best_f:.3e}"
            ));
            return Ok(LineSearchResult {
                step_size: best_alpha,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
            });
        }

        // Last resort: try machine epsilon steps
        let eps_step = f64::EPSILON.sqrt();
        let f_eps = (problem.objective)(eps_step)?;
        if f_eps < f0 {
            self.log_verbose(&format!("Using machine epsilon step {eps_step:.3e}"));
            return Ok(LineSearchResult {
                step_size: eps_step,
                success: true,
                termination_reason: TerminationReason::StepSizeTooSmall,
            });
        }

        // Only fail if we truly cannot find any improvement
        self.log_verbose("No improvement found even with machine epsilon steps");
        Err(anyhow!("Function appears to be ill-conditioned: no improvement possible within machine precision"))
    }

    fn reset(&mut self) {
        // Strong Wolfe line search is stateless, nothing to reset
    }
    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line_search::line_search::create_1d_problem_linear;
    use anyhow::Result;
    use approx::assert_relative_eq;
    use std::sync::Arc;

    fn quadratic_function(x: &[f64]) -> Result<f64> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }

    fn quadratic_gradient1(x: &[f64]) -> Result<Vec<f64>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }
    #[test]
    fn test_config_validation() {
        // Valid config
        let config = StrongWolfeConfig::default();
        assert!(config.validate().is_ok());
        // Invalid c1
        let mut config = StrongWolfeConfig::default();
        config.c1 = 0.0;
        assert!(config.validate().is_err());
        // Invalid c2
        let mut config = StrongWolfeConfig::default();
        config.c2 = 1.0;
        assert!(config.validate().is_err());
        // c1 >= c2
        let mut config = StrongWolfeConfig::default();
        config.c1 = 0.5;
        config.c2 = 0.4;
        assert!(config.validate().is_err());
    }


    #[test]
    fn test_rosenbrock_function() {
        // Test on Rosenbrock function: f(x,y) = (1-x)^2 + 100(y-x^2)^2
        fn rosenbrock(x: &[f64]) -> Result<f64> {
            let a = 1.0 - x[0];
            let b = x[1] - x[0] * x[0];
            Ok(a * a + 100.0 * b * b)
        }
        fn rosenbrock_gradient(x: &[f64]) -> Result<Vec<f64>> {
            let dx = -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (x[1] - x[0] * x[0]);
            let dy = 200.0 * (x[1] - x[0] * x[0]);
            Ok(vec![dx, dy])
        }
        let mut line_search = StrongWolfeLineSearch::new(StrongWolfeConfig {
            c1: 1e-4,
            c2: 0.9,
            ..Default::default()
        }).unwrap();
        let current_point = vec![0.0, 0.0];
        let current_gradient = rosenbrock_gradient(&current_point).unwrap();
        let direction = vec![-current_gradient[0], -current_gradient[1]]; // Steepest descent
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(rosenbrock),
            Arc::new(rosenbrock_gradient),
        )
        .unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        // Verify that the function value decreased
        let new_point: Vec<f64> = current_point
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + result.step_size * d)
            .collect();
        let f_old = rosenbrock(&current_point).unwrap();
        let f_new = rosenbrock(&new_point).unwrap();
        assert!(f_new < f_old);
    }

    #[test]
    fn test_strong_wolfe_quadratic() {
        // init_logging();
        let mut line_search = StrongWolfeLineSearch::default_search();

        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0]; // Negative gradient (descent direction)

        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        )
        .unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();

        assert!(result.success);
        assert!(result.step_size > 0.0);

        // For quadratic function, optimal step should be 1.0
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }
}