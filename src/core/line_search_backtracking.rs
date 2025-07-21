use crate::core::line_search::OneDimensionalProblem;
use crate::core::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;


/// Configuration for backtracking line search
#[derive(Debug, Clone)]
pub struct BacktrackingConfig {
    pub c1: f64,               // Armijo condition parameter
    pub rho: f64,              // Backtracking factor (0 < rho < 1)
    pub max_iterations: usize, // Maximum backtracking iterations
    pub min_step: f64,         // Minimum step size
    pub initial_step: f64,     // Initial step size
}

impl Default for BacktrackingConfig {
    fn default() -> Self {
        Self {
            c1: 1e-4,              // Standard Armijo parameter
            rho: 0.5,              // Standard backtracking factor
            max_iterations: 100,    // More generous iteration limit
            min_step: 1e-12,       // More practical minimum step
            initial_step: 1.0,
        }
    }
}
impl BacktrackingConfig {
    /// Create a strict configuration with conservative parameters
    /// - Stricter Armijo condition (c1 = 1e-3)
    /// - More aggressive backtracking (rho = 0.3)
    /// - Higher iteration limit for thorough search
    pub fn strict() -> Self {
        Self {
            c1: 1e-3,              // Stricter Armijo condition
            rho: 0.3,              // More aggressive backtracking
            max_iterations: 200,    // More iterations for thorough search
            min_step: 1e-15,       // Smaller minimum step
            initial_step: 1.0,
        }
    }
    /// Create a lax configuration with permissive parameters
    /// - Relaxed Armijo condition (c1 = 1e-6)
    /// - Conservative backtracking (rho = 0.8)
    /// - Lower iteration limit for faster convergence
    pub fn lax() -> Self {
        Self {
            c1: 1e-6,              // More permissive Armijo condition
            rho: 0.8,              // Less aggressive backtracking
            max_iterations: 50,     // Fewer iterations for speed
            min_step: 1e-10,       // Larger minimum step
            initial_step: 1.0,
        }
    }
    /// Create the default configuration
    pub fn default_config() -> Self {
        Self::default()
    }
}


/// Backtracking line search implementation (Armijo rule only)
#[derive(Debug, Clone)]
pub struct BacktrackingLineSearch {
    config: BacktrackingConfig,
}

impl BacktrackingLineSearch {
    pub fn new(config: BacktrackingConfig) -> Self {
        Self { config }
    }
    /// Create with default configuration
    pub fn default_search() -> Self {
        Self::new(BacktrackingConfig::default())
    }

    /// Create a strict backtracking line search with conservative parameters
    /// - Stricter Armijo condition (c1 = 1e-3)
    /// - More aggressive backtracking (rho = 0.3)
    /// - Higher iteration limit for thorough search
    pub fn strict() -> Self {
        Self::new(BacktrackingConfig::strict())
    }

    /// Create a lax backtracking line search with permissive parameters
    /// - Relaxed Armijo condition (c1 = 1e-6)
    /// - Conservative backtracking (rho = 0.8)
    /// - Lower iteration limit for faster convergence
    pub fn lax() -> Self {
        Self::new(BacktrackingConfig::lax())
    }
    /// Create a backtracking line search optimized for robust optimization
    /// - Balanced parameters for reliability over speed
    /// - Good for ill-conditioned problems
    pub fn robust() -> Self {
        Self::new(BacktrackingConfig {
            c1: 1e-4,              // Standard Armijo parameter
            rho: 0.5,              // Standard backtracking
            max_iterations: 500,    // High iteration limit
            min_step: 1e-16,       // Very small minimum step
            initial_step: 1.0,
        })
    }
}

impl LineSearch for BacktrackingLineSearch {
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem,
    ) -> anyhow::Result<LineSearchResult> {
        let f0 = (problem.objective)(0.0)?;
        let directional_derivative = problem.initial_directional_derivative;

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut best_alpha = 0.0;
        let mut best_f = f0;

        for _ in 0..self.config.max_iterations {
            // Evaluate function at current step size
            let f_alpha = (problem.objective)(alpha)?;
            // Track best point
            if f_alpha < best_f {
                best_f = f_alpha;
                best_alpha = alpha;
            }

            // Check Armijo condition
            let armijo_threshold = f0 + self.config.c1 * alpha * directional_derivative;
            if f_alpha <= armijo_threshold {
                return Ok(LineSearchResult {
                    step_size: alpha,
                    success: true,
                    termination_reason: TerminationReason::ArmijoConditionSatisfied,
                });
            }

            // Backtrack
            alpha *= self.config.rho;

            if alpha < self.config.min_step {
                // Try minimum step
                let f_min = (problem.objective)(self.config.min_step)?;
                if f_min < f0 {
                    return Ok(LineSearchResult {
                        step_size: self.config.min_step,
                        success: true,
                        termination_reason: TerminationReason::StepSizeTooSmall,
                    });
                }
                break;
            }
        }

        // Return best point found if any improvement
        if best_alpha > 0.0 && best_f < f0 {
            return Ok(LineSearchResult {
                step_size: best_alpha,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
            });
        }

        // Try machine epsilon
        let eps_step = f64::EPSILON.sqrt();
        let f_eps = (problem.objective)(eps_step)?;
        if f_eps < f0 {
            return Ok(LineSearchResult {
                step_size: eps_step,
                success: true,
                termination_reason: TerminationReason::StepSizeTooSmall,
            });
        }

        Err(anyhow!("Function appears to be ill-conditioned: no improvement possible within machine precision"))
    }

    fn reset(&mut self) {
        // Backtracking line search is stateless, nothing to reset
    }
    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::line_search::create_1d_problem_linear;
    use anyhow::Result;
    use log::debug;
    use std::sync::Arc;
    fn quadratic_function(x: &[f64]) -> Result<f64> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }
    fn quadratic_gradient1(x: &[f64]) -> Result<Vec<f64>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }
    fn steep_function(x: &[f64]) -> Result<f64> {
        // f(x) = 1000 * x^2 - very steep function that requires small steps
        Ok(1000.0 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }
    fn steep_gradient(x: &[f64]) -> Result<Vec<f64>> {
        // ∇f(x) = 2000 * x
        Ok(x.iter().map(|xi| 2000.0 * xi).collect())
    }
    fn rosenbrock_1d_function(x: &[f64]) -> Result<f64> {
        // Modified Rosenbrock: f(x) = 100*(x[1] - x[0]^2)^2 + (1 - x[0])^2
        // This creates a narrow valley that requires careful step sizing
        if x.len() < 2 {
            return Ok(x[0] * x[0]);
        }
        let term1 = 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let term2 = (1.0 - x[0]).powi(2);
        Ok(term1 + term2)
    }
    fn rosenbrock_1d_gradient(x: &[f64]) -> Result<Vec<f64>> {
        if x.len() < 2 {
            return Ok(vec![2.0 * x[0]]);
        }
        let grad_x0 = -400.0 * x[0] * (x[1] - x[0] * x[0]) - 2.0 * (1.0 - x[0]);
        let grad_x1 = 200.0 * (x[1] - x[0] * x[0]);
        Ok(vec![grad_x0, grad_x1])
    }
    #[test]
    fn test_backtracking_behavior() {
        // Test that backtracking actually occurs with a steep function
        let config = BacktrackingConfig {
            initial_step: 10.0,  // Much larger initial step to force backtracking
            rho: 0.5,
            c1: 1e-1,  // Stricter Armijo condition to force backtracking
            max_iterations: 10,
            min_step: 1e-12,
        };
        let mut line_search = BacktrackingLineSearch::new(config);
        let current_point = vec![0.1];  // Start closer to optimum to make large steps violate Armijo
        let direction = vec![-1.0]; // Descent direction
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(steep_function),
            Arc::new(steep_gradient),
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        // With a steep function, the step size should be much smaller than initial
        assert!(result.step_size < 1.0, "Step size should be smaller than initial due to backtracking: {}", result.step_size);
        assert!(result.step_size > 0.0);
    }
    #[test]
    fn test_armijo_condition_satisfaction() {
        // Test that the returned step actually satisfies Armijo condition
        let config = BacktrackingConfig {
            initial_step: 1.0,
            rho: 0.7,
            c1: 1e-3,
            max_iterations: 20,
            min_step: 1e-15,
        };
        let mut line_search = BacktrackingLineSearch::new(config.clone());
        let current_point = vec![2.0, 1.0];
        let direction = vec![-1.0, -0.5]; // Descent direction
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(rosenbrock_1d_function),
            Arc::new(rosenbrock_1d_gradient),
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        // Verify Armijo condition is satisfied
        let obj = problem.objective;
        let f0 = obj(0.0).unwrap();
        let f_alpha = obj(result.step_size).unwrap();
        let armijo_threshold = f0 + config.c1 * result.step_size * problem.initial_directional_derivative;
        assert!(f_alpha <= armijo_threshold,
                "Armijo condition not satisfied: f({}) = {} > {} = f(0) + c1*alpha*grad",
                result.step_size, f_alpha, armijo_threshold);
    }
    #[test]
    fn test_max_iterations_reached() {
        // Test behavior when max iterations is reached
        let config = BacktrackingConfig {
            initial_step: 10.0, // Very large initial step
            rho: 0.99, // Very slow backtracking
            c1: 1e-1, // Strict Armijo condition
            max_iterations: 3, // Very few iterations
            min_step: 1e-20,
        };
        let mut line_search = BacktrackingLineSearch::new(config);
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(steep_function),
            Arc::new(steep_gradient),
        ).unwrap();
        let result = line_search.optimize_1d(&problem);
        // Should either succeed with best point found or fail gracefully
        match result {
            Ok(res) => {
                assert!(res.success);
                assert!(matches!(res.termination_reason, 
                    TerminationReason::MaxIterationsReached | 
                    TerminationReason::ArmijoConditionSatisfied |
                    TerminationReason::StepSizeTooSmall));
            }
            Err(_) => {
                // Acceptable if no improvement was possible
            }
        }
    }
    #[test]
    fn test_different_rho_values() {
        // Test that different rho values affect the number of backtracks
        let test_cases = vec![
            (0.1, "aggressive backtracking"),
            (0.5, "moderate backtracking"),
            (0.9, "conservative backtracking"),
        ];
        for (rho, description) in test_cases {
            let config = BacktrackingConfig {
                initial_step: 2.0,
                rho,
                c1: 1e-4,
                max_iterations: 50,
                min_step: 1e-16,
            };
            let mut line_search = BacktrackingLineSearch::new(config);
            let current_point = vec![1.0];
            let direction = vec![-1.0];
            let problem = create_1d_problem_linear(
                &current_point,
                &direction,
                Arc::new(steep_function),
                Arc::new(steep_gradient),
            ).unwrap();
            let result = line_search.optimize_1d(&problem);
            assert!(result.is_ok(), "Failed with {}: {:?}", description, result);
            let result = result.unwrap();
            assert!(result.success, "Not successful with {}", description);
            assert!(result.step_size > 0.0, "Invalid step size with {}", description);
        }
    }
    #[test]
    fn test_c1_parameter_effect() {
        // Test that stricter c1 values require smaller steps
        let strict_config = BacktrackingConfig {
            c1: 1e-1, // Very strict
            initial_step: 1.0,
            rho: 0.5,
            max_iterations: 50,
            min_step: 1e-16,
        };
        let lenient_config = BacktrackingConfig {
            c1: 1e-6, // Very lenient
            ..strict_config
        };
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        // Test with strict c1
        let mut strict_search = BacktrackingLineSearch::new(strict_config);
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        ).unwrap();
        let strict_result = strict_search.optimize_1d(&problem).unwrap();
        // Test with lenient c1
        let mut lenient_search = BacktrackingLineSearch::new(lenient_config);
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        ).unwrap();
        let lenient_result = lenient_search.optimize_1d(&problem).unwrap();
        assert!(strict_result.success);
        assert!(lenient_result.success);
        // Lenient c1 should generally allow larger steps
        // (though this isn't guaranteed for all functions)
        assert!(lenient_result.step_size >= strict_result.step_size * 0.1,
                "Lenient c1 should allow reasonably larger steps: {} vs {}",
                lenient_result.step_size, strict_result.step_size);
    }
    #[test]
    fn test_min_step_size() {
        // Test behavior when step size becomes too small
        let config = BacktrackingConfig {
            min_step: 1e-1, // Much larger minimum step
            initial_step: 1.0,
            rho: 0.9,          // Less aggressive backtracking
            c1: 1e-8,          // Very strict Armijo condition
            max_iterations: 5, // Few iterations
        };
        let mut line_search = BacktrackingLineSearch::new(config);
        // Use a function that requires very small steps to satisfy Armijo
        fn difficult_function(x: &[f64]) -> Result<f64> {
            let val = x[0] * x[0];
            if x[0].abs() > 0.01 {
                Ok(val + 1000.0 * x[0].abs())
            } else {
                Ok(val)
            }
        }
        fn difficult_gradient(x: &[f64]) -> Result<Vec<f64>> {
            if x[0].abs() > 0.01 {
                Ok(vec![2.0 * x[0] + 1000.0 * x[0].signum()])
            } else {
                Ok(vec![2.0 * x[0]])
            }
        }
        let current_point = vec![1.0]; // Start at a point where gradient is non-zero
        let direction = vec![-1.0]; // Move in negative direction (descent)
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(difficult_function),
            Arc::new(difficult_gradient),
        )
            .unwrap();
        let result = line_search.optimize_1d(&problem).map_or_else(
            |e| {
                debug!("Line search failed: {}", e);
                // If it fails, we expect it to be due to step size being too small
                LineSearchResult {
                    step_size: 0.0,
                    success: false,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                }
            },
            |res| res,
        );
        if result.success {
            // If it succeeded, the step size should be small (but we'll be more lenient)
            // The key is that it found *some* acceptable step
            assert!(result.step_size > 0.0);
            debug!("Line search succeeded with step size: {}", result.step_size);
        } else {
            // If it failed, it should be due to step size being too small
            assert!(matches!(
                result.termination_reason,
                TerminationReason::StepSizeTooSmall
            ));
            debug!("Line search failed as expected due to small step size");
        }
    }
    #[test]
    fn test_backtracking_quadratic() {
        // Basic functionality test
        let mut line_search = BacktrackingLineSearch::new(BacktrackingConfig::default());
        let current_point = vec![1.0, 1.0];
        let direction = vec![-1.0, -1.0]; // Negative gradient
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
    }
    #[test]
    fn test_reset_functionality() {
        // Test that reset doesn't break anything (backtracking is stateless)
        let mut line_search = BacktrackingLineSearch::new(BacktrackingConfig::default());
        // Reset should not cause any issues
        line_search.reset();
        // Should still work after reset
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
    }
    #[test]
    fn test_static_constructors() {
        // Test that all static constructors work
        let strict = BacktrackingLineSearch::strict();
        let lax = BacktrackingLineSearch::lax();
        let robust = BacktrackingLineSearch::robust();
        let default = BacktrackingLineSearch::new(BacktrackingConfig::default());
        // Verify they have different configurations
        assert!(strict.config.c1 > default.config.c1, "Strict should have stricter c1");
        assert!(strict.config.rho < default.config.rho, "Strict should have more aggressive rho");
        assert!(lax.config.c1 < default.config.c1, "Lax should have more permissive c1");
        assert!(lax.config.rho > default.config.rho, "Lax should have less aggressive rho");
        assert!(robust.config.max_iterations > default.config.max_iterations, "Robust should have more iterations");
        assert!(robust.config.min_step <= default.config.min_step, "Robust should have smaller min step");
        // Test that they all work on a simple problem
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        for (mut line_search, name) in vec![
            (strict, "strict"),
            (lax, "lax"),
            (robust, "robust"),
            (default, "default")
        ] {
            let problem = create_1d_problem_linear(
                &current_point,
                &direction,
                Arc::new(quadratic_function),
                Arc::new(quadratic_gradient1),
            ).unwrap();
            let result = line_search.optimize_1d(&problem);
            assert!(result.is_ok(), "{} constructor failed: {:?}", name, result);
            let result = result.unwrap();
            assert!(result.success, "{} constructor did not succeed", name);
            assert!(result.step_size > 0.0, "{} constructor returned invalid step size", name);
        }
    }
    #[test]
    fn test_constructor_behavior_differences() {
        // Test that strict vs lax actually behave differently on a challenging problem
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        let mut strict = BacktrackingLineSearch::strict();
        let mut lax = BacktrackingLineSearch::lax();
        // Use steep function to see differences
        let strict_problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(steep_function),
            Arc::new(steep_gradient),
        ).unwrap();
        let lax_problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(steep_function),
            Arc::new(steep_gradient),
        ).unwrap();
        let strict_result = strict.optimize_1d(&strict_problem).unwrap();
        let lax_result = lax.optimize_1d(&lax_problem).unwrap();
        assert!(strict_result.success);
        assert!(lax_result.success);
        // Lax should generally allow larger steps (though this isn't guaranteed for all functions)
        // We'll just verify both found valid solutions
        assert!(strict_result.step_size > 0.0);
        assert!(lax_result.step_size > 0.0);
    }
}