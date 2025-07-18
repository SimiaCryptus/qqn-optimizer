use crate::core::line_search::OneDimensionalProblem;
use crate::core::line_search_backtracking::BisectionLineSearch;
use crate::core::{line_search, LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use log::debug;

/// Configuration for bisection line search
#[derive(Debug, Clone)]
pub struct BisectionConfig {
    pub max_iterations: usize,     // Maximum bisection iterations
    pub gradient_tolerance: f64,   // Tolerance for gradient being zero
    pub min_step: f64,             // Minimum step size
    pub max_step: f64,             // Maximum step size
    pub initial_step: f64,         // Initial step size
    pub window_shrink_factor: f64, // Factor to shrink window on failure
    pub verbose: bool,             // Enable verbose logging
    pub line_bracket_method: u8, // 1: gradient-based bracketing, 2: function-value-based bracketing
}

impl Default for BisectionConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            gradient_tolerance: 1e-16,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            window_shrink_factor: 0.5,
            verbose: false,
            line_bracket_method: 1, // 1 for far point method, 2 for simple far point
        }
    }
}

impl LineSearch for BisectionLineSearch {
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem<'a>,
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
            1 => line_search::find_far_point_1(
                problem,
                (problem.objective)(0.0)?,
                config.initial_step,
                config.max_iterations,
                config.min_step,
                config.gradient_tolerance,
                config.max_step,
            )?,
            2 => line_search::find_far_point_2(
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
                // Try minimum step as last resort
                let f_min = (problem.objective)(self.config.min_step)?;
                if f_min < f0 {
                    self.config.min_step
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
}

impl BisectionLineSearch {
    pub fn new(config: BisectionConfig) -> Self {
        Self { config }
    }
    /// Log line search details if verbose mode is enabled
    pub(crate) fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("Bisection: {}", message);
        }
    }

    /// Find the point where gradient is approximately zero using bisection
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
                "  Iteration {}: mid={:.3e}, grad={:.3e}",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::line_search::create_1d_problem_linear;
    use crate::init_logging;
    use anyhow::Result;
    use approx::assert_relative_eq;

    fn quadratic_function(x: &[f64]) -> Result<f64> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }

    fn quadratic_gradient1(x: &[f64]) -> Result<Vec<f64>> {
        // ∇f(x) = x
        Ok(x.to_vec())
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
            &quadratic_function,
            &quadratic_gradient1,
        )
            .unwrap();

        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        // For quadratic function, optimal step should be 1.0 (where gradient is zero)
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_bisection_non_descent() {
        //init_logging().unwrap();
        let current_point = vec![1.0, 1.0];
        let direction = vec![1.0, 1.0]; // Positive gradient (ascent direction)
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        );

        // The problem creation should fail because direction is not a descent direction
        assert!(problem.is_err());
        if let Err(err) = problem {
            assert!(err
                .to_string()
                .contains("Initial directional derivative must be negative"));
        }
    }
}
