use crate::core::line_search::OneDimensionalProblem;
use crate::core::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::{anyhow, Error};
use log::debug;

/// Configuration for bisection line search algorithm.
///
/// The bisection line search is a robust method that finds optimal step sizes by:
/// 1. Finding a "far point" where the function behavior changes
/// 2. Using bisection to locate where the gradient becomes zero
///
/// This approach is particularly effective for well-behaved functions but may be
/// slower than more sophisticated methods like Wolfe line search for complex problems.
///
/// # Trade-offs
/// - **Strengths**: Robust, guaranteed convergence for unimodal functions, simple to understand
/// - **Weaknesses**: Can be slow for complex functions, may not handle poorly conditioned problems optimally
#[derive(Debug, Clone)]
pub struct BisectionConfig {
    pub max_iterations: usize,     // Maximum bisection iterations
    pub gradient_tolerance: f64,   // Tolerance for gradient being zero
    pub min_step: f64,             // Minimum step size
    pub max_step: f64,             // Maximum step size
    pub initial_step: f64,         // Initial step size
    pub verbose: bool,             // Enable verbose logging
    /// Method for finding the far point to establish search bracket:
    /// - Method 1: Gradient-based bracketing - finds point where f(t) < f(0) and gradient > 0
    /// - Method 2: Function-value-based bracketing - finds point where f(t) > f(0)
    ///
    /// Method 1 is generally more robust for optimization, Method 2 is simpler but less precise.
    pub line_bracket_method: u8,
}

impl Default for BisectionConfig {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            gradient_tolerance: 1e-16,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
            line_bracket_method: 1, // 1 for far point method, 2 for simple far point
        }
    }
}
impl BisectionConfig {
    /// Create a strict configuration with tight tolerances and more iterations
    ///
    /// Use this when:
    /// - High precision is critical
    /// - Computational cost is not a primary concern
    /// - Working with well-conditioned problems
    /// Suitable for high-precision optimization where accuracy is critical
    pub fn strict() -> Self {
        Self {
            max_iterations: 100,
            gradient_tolerance: 1e-16,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
            line_bracket_method: 1,
        }
    }

    /// Create a lax configuration with loose tolerances and fewer iterations
    ///
    /// Use this when:
    /// - Speed is more important than precision
    /// - Working with noisy or ill-conditioned functions
    /// - Performing exploratory optimization
    /// Suitable for fast optimization where speed is more important than precision
    pub fn lax() -> Self {
        Self {
            max_iterations: 20,
            gradient_tolerance: 1e-6,
            min_step: 1e-8,
            max_step: 1e8,
            initial_step: 1.0,
            verbose: false,
            line_bracket_method: 1,
        }
    }
    /// Create the default configuration
    pub fn default_config() -> Self {
        Self::default()
    }

    /// Create a configuration with verbose logging enabled
    ///
    /// Useful for:
    /// - Debugging line search behavior
    /// - Understanding convergence patterns
    pub fn verbose() -> Self {
        Self {
            verbose: true,
            ..Self::default()
        }
    }
}

/// Bisection line search implementation for one-dimensional optimization.
///
/// This line search method uses a two-phase approach:
/// 1. **Bracketing Phase**: Find a "far point" that establishes a search interval
/// 2. **Bisection Phase**: Use bisection method to find where the gradient is zero
///
/// # Algorithm Overview
/// The bisection method is particularly effective when:
/// - The objective function is unimodal along the search direction
/// - Gradient information is reliable and inexpensive to compute
/// - Robustness is preferred over speed
///
/// # Performance Characteristics
/// - **Time Complexity**: O(log(1/ε)) where ε is the desired tolerance
/// - **Function Evaluations**: Typically 10-50 per line search
/// - **Convergence**: Linear convergence rate
///
/// # Limitations
/// - May be slower than Wolfe conditions for well-behaved problems
/// - Requires gradient evaluations at each iteration
/// - Less effective for functions with multiple local minima along search direction
#[derive(Debug, Clone)]
pub struct BisectionLineSearch {
    config: BisectionConfig,
}


impl LineSearch for BisectionLineSearch {
    fn optimize_1d(
        &mut self,
        problem: &OneDimensionalProblem,
    ) -> anyhow::Result<LineSearchResult> {
        let directional_derivative = problem.initial_directional_derivative;
        self.log_verbose(&format!("Starting bisection line search"));
        self.log_verbose(&format!(
            "Initial directional derivative: {:.3e}",
            directional_derivative
        ));

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        // Step 1: Find the far point
        let config = self.config.clone();
        let far_point = match config.line_bracket_method {
            1 => find_far_point_1(
                problem,
                (problem.objective)(0.0)?,
                config.initial_step,
                config.max_iterations,
                config.min_step,
                config.gradient_tolerance,
                config.max_step,
            )?,
            2 => find_far_point_2(
                problem,
                (problem.objective)(0.0)?,
                config.initial_step,
                config.max_iterations,
                config.max_step,
            )?,
            _ => {
                return Err(anyhow!(
                    "Invalid line_bracket_method: {}. Must be 1 or 2",
                    config.line_bracket_method
                ))
            }
        };

        // Step 2: Verify we have a proper bracket for bisection
        let grad_0 = problem.initial_directional_derivative;
        let grad_far = (problem.gradient)(far_point)?;

        self.log_verbose(&format!(
            "Bracket: grad(0)={:.3e}, grad({:.3e})={:.3e}",
            grad_0, far_point, grad_far
        ));

        // Step 3: Perform bisection search for zero gradient
        let step_size = if grad_0 * grad_far < 0.0 {
            // We have a proper bracket, use bisection
            self.find_zero_gradient(0.0, far_point, problem)?
        } else {
            // No proper bracket, return the far point if it's an improvement
            let f0 = (problem.objective)(0.0)?;
            let f_far = (problem.objective)(far_point)?;
            if f_far < f0 {
                self.log_verbose(&format!(
                    "No gradient sign change, but far point provides improvement"
                ));
                far_point
            } else {
                // Use a small step that provides some improvement
                let mut test_step = far_point * 0.1;
                let mut best_step = 0.0;
                let mut best_f = f0;

                // Try progressively smaller steps
                for _ in 0..10 {
                    if test_step < self.config.min_step {
                        break;
                    }
                    let f_test = (problem.objective)(test_step)?;
                    if f_test < best_f {
                        best_f = f_test;
                        best_step = test_step;
                    }
                    test_step *= 0.5;
                }

                if best_step > 0.0 {
                    self.log_verbose(&format!(
                        "Found improvement with small step: {:.3e}",
                        best_step
                    ));
                    best_step
                } else {
                    return Err(anyhow!("Cannot find any improvement"));
                }
            }
        };

        // Verify the final step size provides improvement
        let f0 = (problem.objective)(0.0)?;
        let f_final = (problem.objective)(step_size)?;

        if f_final >= f0 {
            return Err(anyhow!("Final step size does not provide improvement"));
        }

        // Check final gradient
        let final_gradient = (problem.gradient)(step_size)?;
        let success = step_size >= self.config.min_step && step_size <= self.config.max_step;

        self.log_verbose(&format!(
            "Final result: alpha={:.3e}, f_improvement={:.3e}, grad={:.3e}, success={}",
            step_size,
            f0 - f_final,
            final_gradient,
            success
        ));

        Ok(LineSearchResult {
            step_size,
            success,
            termination_reason: if success {
                TerminationReason::WolfeConditionsSatisfied
            } else {
                TerminationReason::MaxIterationsReached
            },
        })
    }

    fn reset(&mut self) {
        // Bisection line search is stateless, nothing to reset
    }

    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl BisectionLineSearch {
    pub fn new(config: BisectionConfig) -> Self {
        Self { config }
    }
    
    /// Set the initial step size for the next line search
    ///
    /// The initial step size affects the bracketing phase performance.
    /// Larger steps may find the bracket faster but risk overshooting,
    /// while smaller steps are more conservative but may require more iterations.
    pub fn set_initial_step(&mut self, step: f64) {
        self.config.initial_step = step.clamp(self.config.min_step, self.config.max_step);
    }
    /// Create with default configuration
    pub fn default_search() -> Self {
        Self::new(BisectionConfig::default())
    }
    /// Create with strict configuration
    pub fn strict() -> Self {
        Self::new(BisectionConfig::strict())
    }
    /// Create with lax configuration
    pub fn lax() -> Self {
        Self::new(BisectionConfig::lax())
    }
    /// Log line search details if verbose mode is enabled
    pub(crate) fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("Bisection: {}", message);
        }
    }

    /// Find the point where gradient is approximately zero using bisection
    ///
    /// This is the core bisection algorithm that assumes we have a proper bracket
    /// where the gradient changes sign. If no proper bracket exists, it returns
    /// the point with the smallest absolute gradient.
    ///
    /// # Returns
    /// The step size where the gradient is closest to zero within the given interval.
    pub(crate) fn find_zero_gradient(
        &self,
        left: f64,
        right: f64,
        problem: &OneDimensionalProblem,
    ) -> anyhow::Result<f64> {
        let mut a = left;
        let mut b = right;

        self.log_verbose(&format!(
            "Finding zero gradient in interval [{:.3e}, {:.3e}]",
            a, b
        ));
        // Verify we have a proper bracket with opposite gradient signs
        let grad_a = (problem.gradient)(a)?;
        let grad_b = (problem.gradient)(b)?;
        if grad_a * grad_b > 0.0 {
            self.log_verbose(&format!(
                "Warning: gradients have same sign at endpoints: grad({:.3e})={:.3e}, grad({:.3e})={:.3e}",
                a, grad_a, b, grad_b
            ));
            // Return the point with smaller absolute gradient
            return Ok(if grad_a.abs() < grad_b.abs() { a } else { b });
        }

        for i in 0..self.config.max_iterations {
            let mid = 0.5 * (a + b);
            // Evaluate gradient at midpoint
            let grad_mid = (problem.gradient)(mid)?;
            self.log_verbose(&format!(
                "  Line Search Iteration {}: mid={:.3e}, grad={:.3e}",
                i, mid, grad_mid
            ));
            // Check if gradient is close enough to zero
            if grad_mid.abs() <= self.config.gradient_tolerance {
                self.log_verbose(&format!("Found zero gradient at alpha={:.3e}", mid));
                return Ok(mid);
            }
            // Check if interval is too small
            if (b - a) < self.config.min_step {
                self.log_verbose(&format!("Interval too small, returning mid={:.3e}", mid));
                return Ok(mid);
            }
            // Update interval based on sign of gradient
            let grad_a = (problem.gradient)(a)?;
            if grad_a * grad_mid < 0.0 {
                // Zero is between a and mid
                b = mid;
            } else {
                // Zero is between mid and b (or doesn't exist in interval)
                a = mid;
            }
        }
        // Return midpoint if max iterations reached
        let final_alpha = 0.5 * (a + b);
        self.log_verbose(&format!(
            "Max iterations reached, returning alpha={:.3e}",
            final_alpha
        ));
        Ok(final_alpha)
    }
}


/// Find far point using gradient-based method (Method 1).
///
/// This method searches for a point where:
/// - f(t) < f(0) (function value is still better than starting point)
/// - gradient > 0 (function is starting to increase, indicating we've passed the minimum)
///
/// This approach is more sophisticated than simple function-value bracketing because
/// it uses gradient information to identify when we've likely passed the optimal point.
///
/// # Parameters
/// - `f0`: Function value at the starting point (t=0)
/// - `initial_step`: Starting step size for the search
/// Looks for a point where f(t) < f(0) and gradient is positive (function starts increasing)
pub(crate) fn find_far_point_1(
    problem: &OneDimensionalProblem,
    f0: f64,
    initial_step: f64,
    max_iterations: usize,
    min_step: f64,
    gradient_tolerance: f64,
    max_step: f64,
) -> anyhow::Result<f64, Error> {
    let mut t = initial_step;
    let mut iteration = 0;
    debug!("Finding far point starting from t={:.3e}", t);
    while iteration < max_iterations {
        let f_t = (problem.objective)(t)?;
        let grad_t = (problem.gradient)(t)?;
        debug!(
            "  Line Search Iteration {}: t={:.3e}, f={:.3e}, grad={:.3e}, f0={:.3e}",
            iteration, t, f_t, grad_t, f0
        );
        // Check if this point satisfies our far point criteria:
        // 1. Function value is still better than f(0)
        // 2. Gradient is positive (function is increasing)
        if f_t < f0 && grad_t > 0.0 {
            debug!("Found far point at t={:.3e}", t);
            return Ok(t);
        }
        if f_t >= f0 {
            debug!("Function value too high at t={:.3e}, reducing step", t);
            t *= 0.5;
            if t < min_step {
                debug!("Step size too small, using minimum step");
                return Ok(min_step);
            }
        }
        // If gradient is still negative, step is too small
        else if grad_t < 0.0 {
            debug!("Gradient still negative at t={:.3e}, increasing step", t);
            t *= 2.0;
            if t > max_step {
                debug!("Step size too large, using maximum step");
                return Ok(max_step);
            }
        }
        // If gradient is approximately zero, we found our point
        else if grad_t.abs() <= gradient_tolerance {
            debug!("Found zero gradient at t={:.3e}", t);
            return Ok(t);
        }
        iteration += 1;
    }
    // If we can't find a proper far point, return the last valid step
    debug!("Max iterations reached, returning t={:.3e}", t);
    Ok(t)
}

/// Find far point using simple function-value-based method (Method 2).
///
/// This method searches for a point where f(t) > f(0), meaning the function value
/// is worse than the starting point. This is simpler than the gradient-based method
/// but may be less precise in identifying the optimal bracket.
///
/// # Use Cases
/// - When gradient computation is expensive or unreliable
/// - For functions where gradient-based bracketing fails
/// - As a fallback when Method 1 doesn't converge
/// Looks for a point where f(t) > f(0) (function value is worse than starting point)
pub(crate) fn find_far_point_2(
    problem: &OneDimensionalProblem,
    f0: f64,
    initial_steop: f64,
    max_iterations: usize,
    max_step: f64,
) -> anyhow::Result<f64, Error> {
    let mut t = initial_steop;
    let mut iteration = 0;
    debug!("Finding far point starting from t={:.3e}", t);
    while iteration < max_iterations {
        let f_t = (problem.objective)(t)?;
        debug!(
            "  Line Search Iteration {}: t={:.3e}, f={:.3e}, f0={:.3e}",
            iteration, t, f_t, f0
        );
        // Check if this point satisfies our far point criteria:
        // 1. Function value is worse than f(0)
        if f_t > f0 {
            debug!("Found far point at t={:.3e}", t);
            return Ok(t);
        }

        // If function value is still better than f(0), increase step size
        if f_t <= f0 {
            debug!(
                "Function value still better at t={:.3e}, increasing step",
                t
            );
            t *= 2.0;
            if t > max_step {
                debug!("Step size too large, using maximum step");
                return Ok(max_step);
            }
        }

        iteration += 1;
    }
    // If we can't find a proper far point, return the last valid step
    debug!("Max iterations reached, returning t={:.3e}", t);
    Ok(t)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::line_search::create_1d_problem_linear;
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
    fn cubic_function(x: &[f64]) -> Result<f64> {
        // f(x) = x^3 - 2*x^2 + x (has zero gradient at x = 1/3 and x = 1)
        let val = x[0];
        Ok(val * val * val - 2.0 * val * val + val)
    }
    fn cubic_gradient(x: &[f64]) -> Result<Vec<f64>> {
        // ∇f(x) = 3*x^2 - 4*x + 1
        let val = x[0];
        Ok(vec![3.0 * val * val - 4.0 * val + 1.0])
    }

    #[test]
    fn test_bisection_quadratic() {
        let mut line_search = BisectionLineSearch::new(BisectionConfig {
            verbose: false,
            ..BisectionConfig::default()
        });
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
        // For quadratic function, optimal step should be 1.0 (where gradient is zero)
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_find_zero_gradient_proper_bracket() {
        let line_search = BisectionLineSearch::new(BisectionConfig {
            gradient_tolerance: 1e-8,
            max_iterations: 50,
            verbose: false,
            ..BisectionConfig::default()
        });
        // Create a 1D problem with a function that actually has a zero gradient in our search interval
        // Use f(x) = x² - 0.1 which has zero gradient at x = 0
        let simple_quadratic = |x: &[f64]| -> Result<f64> {
            Ok(x[0] * x[0] - 0.1)
        };
        let simple_quadratic_grad = |x: &[f64]| -> Result<Vec<f64>> {
            Ok(vec![2.0 * x[0]])
        };

        let current_point = vec![-0.5];
        let direction = vec![1.0]; // Positive direction
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(simple_quadratic),
            Arc::new(simple_quadratic_grad),
        ).unwrap();
        // At t=0.3: x = -0.5 + 0.3 = -0.2, grad = 2*(-0.2) = -0.4 < 0
        // At t=0.7: x = -0.5 + 0.7 = 0.2, grad = 2*(0.2) = 0.4 > 0
        // Zero gradient should be at t=0.5 where x = 0
        let result = line_search.find_zero_gradient(0.3, 0.7, &problem).unwrap();
        assert_relative_eq!(result, 0.5, epsilon = 1e-6);
        // Verify gradient is indeed close to zero
        let grad_at_result = (problem.gradient)(result).unwrap();
        assert!(grad_at_result.abs() < 1e-6);
    }
    #[test]
    fn test_find_zero_gradient_no_bracket() {
        let line_search = BisectionLineSearch::new(BisectionConfig {
            gradient_tolerance: 1e-8,
            max_iterations: 50,
            verbose: false,
            ..BisectionConfig::default()
        });
        // Create a 1D problem with cubic function using descent direction
        let current_point = vec![0.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(cubic_function),
            Arc::new(cubic_gradient),
        ).unwrap();
        // Test with no proper bracket: both gradients have same sign
        let result = line_search.find_zero_gradient(0.1, 0.2, &problem).unwrap();
        // Should return the point with smaller absolute gradient
        let grad = problem.gradient;
        let grad_left = grad(0.1).unwrap().abs();
        let grad_right = grad(0.2).unwrap().abs();
        let expected = if grad_left < grad_right { 0.1 } else { 0.2 };
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }
    #[test]
    fn test_find_zero_gradient_tolerance() {
        let line_search = BisectionLineSearch::new(BisectionConfig {
            gradient_tolerance: 1e-2, // Loose tolerance
            max_iterations: 50,
            verbose: false,
            ..BisectionConfig::default()
        });
        // Create a 1D problem with a simple quadratic that has zero gradient at x=0
        // f(x) = x² which has gradient f'(x) = 2x, zero at x=0
        let simple_quadratic = |x: &[f64]| -> Result<f64> {
            Ok(x[0] * x[0])
        };
        let simple_quadratic_grad = |x: &[f64]| -> Result<Vec<f64>> {
            Ok(vec![2.0 * x[0]])
        };

        let current_point = vec![-0.1];
        let direction = vec![1.0]; // Positive direction
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(simple_quadratic),
            Arc::new(simple_quadratic_grad),
        ).unwrap();

        // At t=0.05: x = -0.1 + 0.05 = -0.05, grad = 2*(-0.05) = -0.1 < 0
        // At t=0.15: x = -0.1 + 0.15 = 0.05, grad = 2*(0.05) = 0.1 > 0
        // Zero gradient should be at t=0.1 where x = 0
        let result = line_search.find_zero_gradient(0.05, 0.15, &problem).unwrap();
        // With loose tolerance, should terminate early when gradient is small enough
        let grad_at_result = (problem.gradient)(result).unwrap();
        assert!(grad_at_result.abs() <= 1e-2);
    }
    #[test]
    fn test_find_zero_gradient_min_step() {
        let line_search = BisectionLineSearch::new(BisectionConfig {
            gradient_tolerance: 1e-16, // Very tight tolerance
            min_step: 1e-2, // Large minimum step
            max_iterations: 50,
            verbose: false,
            ..BisectionConfig::default()
        });
        // Create a 1D problem with cubic function using descent direction
        let current_point = vec![0.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(cubic_function),
            Arc::new(cubic_gradient),
        ).unwrap();
        let result = line_search.find_zero_gradient(0.3, 0.4, &problem).unwrap();
        // Should terminate when interval becomes smaller than min_step
        assert!(result >= 0.3 && result <= 0.4);
    }
    #[test]
    fn test_find_zero_gradient_max_iterations() {
        //init_logging().unwrap();
        let line_search = BisectionLineSearch::new(BisectionConfig {
            gradient_tolerance: 1e-16, // Very tight tolerance
            min_step: 1e-16, // Very small minimum step
            max_iterations: 3, // Very few iterations
            verbose: false,
            ..BisectionConfig::default()
        });
        // Create a 1D problem with cubic function using descent direction
        let current_point = vec![0.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(cubic_function),
            Arc::new(cubic_gradient),
        ).unwrap();
        let result = line_search.find_zero_gradient(0.2, 0.5, &problem).unwrap();
        // Should return midpoint after max iterations
        assert!(result >= 0.2 && result <= 0.5);
    }
    #[test]
    fn test_bisection_with_different_bracket_methods() {
        // Test with bracket method 1
        let mut line_search1 = BisectionLineSearch::new(BisectionConfig {
            line_bracket_method: 1,
            verbose: false,
            ..BisectionConfig::default()
        });
        // Test with bracket method 2
        let mut line_search2 = BisectionLineSearch::new(BisectionConfig {
            line_bracket_method: 2,
            verbose: false,
            ..BisectionConfig::default()
        });
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        ).unwrap();
        let result1 = line_search1.optimize_1d(&problem).unwrap();
        let result2 = line_search2.optimize_1d(&problem).unwrap();
        assert!(result1.success);
        assert!(result2.success);
        // Both methods should find similar solutions for this simple case
        assert_relative_eq!(result1.step_size, result2.step_size, epsilon = 1e-3);
    }
    #[test]
    fn test_bisection_invalid_bracket_method() {
        let mut line_search = BisectionLineSearch::new(BisectionConfig {
            line_bracket_method: 3, // Invalid method
            verbose: false,
            ..BisectionConfig::default()
        });
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        ).unwrap();
        let result = line_search.optimize_1d(&problem);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid line_bracket_method"));
    }
    #[test]
    fn test_line_search_clone_and_reset() {
        let line_search = BisectionLineSearch::new(BisectionConfig::default());
        // Test cloning
        let mut cloned = line_search.clone();
        // Test reset (should not panic)
        cloned.reset();
        // Verify the clone works
        assert_eq!(cloned.config.max_iterations, line_search.config.max_iterations);
    }
    #[test]
    fn test_config_constructors() {
        // Test default config
        let default_config = BisectionConfig::default();
        assert_eq!(default_config.max_iterations, 50);
        assert_eq!(default_config.gradient_tolerance, 1e-16);
        assert_eq!(default_config.min_step, 1e-16);
        assert!(!default_config.verbose);
        // Test strict config
        let strict_config = BisectionConfig::strict();
        assert_eq!(strict_config.max_iterations, 100);
        assert_eq!(strict_config.gradient_tolerance, 1e-16);
        assert_eq!(strict_config.min_step, 1e-16);
        assert!(!strict_config.verbose);
        // Test lax config
        let lax_config = BisectionConfig::lax();
        assert_eq!(lax_config.max_iterations, 20);
        assert_eq!(lax_config.gradient_tolerance, 1e-6);
        assert_eq!(lax_config.min_step, 1e-8);
        assert_eq!(lax_config.max_step, 1e8);
        assert!(!lax_config.verbose);
        // Test verbose config
        let verbose_config = BisectionConfig::verbose();
        assert!(verbose_config.verbose);
        assert_eq!(verbose_config.max_iterations, 50); // Should inherit from default
    }
    #[test]
    fn test_lax_config_performance() {
        // Verify that lax config can be used successfully
        let line_search = BisectionLineSearch::new(BisectionConfig::lax());
        // This test ensures the lax config doesn't break functionality
        assert_eq!(line_search.config.max_iterations, 20);
    }
}