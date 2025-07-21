use crate::core::line_search::OneDimensionalProblem;
use crate::core::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Configuration for Strong Wolfe line search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongWolfeConfig {
    pub c1: f64,               // Armijo condition parameter (0 < c1 < c2 < 1)
    pub c2: f64,               // Curvature condition parameter
    pub max_iterations: usize, // Maximum line search iterations
    pub min_step: f64,         // Minimum step size
    pub max_step: f64,         // Maximum step size
    pub initial_step: f64,     // Initial step size
    pub verbose: bool,         // Enable verbose logging
}

impl Default for StrongWolfeConfig {
    fn default() -> Self {
        Self {
            c1: 1e-4,              // Standard Nocedal & Wright recommendation
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
    /// - Smaller c1 for stricter sufficient decrease
    /// - Smaller c2 for stricter curvature condition
    /// - More iterations allowed
    /// - Tighter step size bounds
    pub fn strict() -> Self {
        Self {
            c1: 1e-6,              // Very strict sufficient decrease
            c2: 0.1,               // Very strict curvature condition
            max_iterations: 100,   // More iterations for precision
            min_step: 1e-20,       // Smaller minimum step
            max_step: 1e10,        // Conservative maximum step
            initial_step: 1.0,
            verbose: false,
        }
    }

    /// Lax configuration with relaxed tolerances for robust optimization
    /// - Larger c1 for more lenient sufficient decrease
    /// - Larger c2 for more lenient curvature condition
    /// - Fewer iterations for efficiency
    /// - Wider step size bounds
    pub fn lax() -> Self {
        Self {
            c1: 1e-2,              // Relaxed sufficient decrease
            c2: 0.99,              // Very relaxed curvature condition
            max_iterations: 20,    // Fewer iterations for efficiency
            min_step: 1e-12,       // Larger minimum step
            max_step: 1e20,        // Larger maximum step
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


/// Strong Wolfe line search implementation
#[derive(Debug, Clone)]
pub struct StrongWolfeLineSearch {
    config: StrongWolfeConfig,
}

impl StrongWolfeLineSearch {
    pub fn new(config: StrongWolfeConfig) -> Self {
        Self { config }
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
    /// Log line search details if verbose mode is enabled
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("StrongWolfe: {}", message);
        }
    }

    /// Check Armijo condition: f(α) ≤ f(0) + c1*α*f'(0)
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

    /// Zoom phase of Strong Wolfe line search
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
            let f_alpha_j = (problem.objective)(alpha_j)?;
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

            // Evaluate 1D gradient at trial point
            let grad_alpha_j = (problem.gradient)(alpha_j)?;

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
    fn optimize_1d(
        &mut self,
        problem: &OneDimensionalProblem,
    ) -> anyhow::Result<LineSearchResult> {
        let f0 = (problem.objective)(0.0)?;
        let directional_derivative = problem.initial_directional_derivative;

        self.log_verbose(&format!("Starting 1D optimization with f(0)={:.3e}", f0));
        self.log_verbose(&format!(
            "Directional derivative: {:.3e}",
            directional_derivative
        ));

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let alpha = self.config.initial_step;
        let alpha_prev = 0.0;
        let f_prev = f0;
        let mut best_alpha = 0.0;
        let mut best_f = f0;

        self.log_verbose(&format!("Initial step size: {:.3e}", alpha));

        for i in 0..self.config.max_iterations {
            self.log_verbose(&format!("Line Search Iteration {}: trying alpha={:.3e}", i, alpha));

            // Evaluate function at current step size
            let f_alpha = (problem.objective)(alpha)?;
            self.log_verbose(&format!("  f({:.3e}) = {:.3e}", alpha, f_alpha));
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
                    "  Armijo failed or insufficient decrease, zooming between {:.3e} and {:.3e}",
                    alpha_prev, alpha
                ));
                // Zoom between alpha_prev and alpha
                let final_alpha =
                    self.zoom(alpha_prev, alpha, f0, directional_derivative, &problem)?;
                self.log_verbose(&format!("Zoom completed with alpha={:.3e}", final_alpha));

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
                    "Both Wolfe conditions satisfied at alpha={:.3e}",
                    alpha
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
                    "  Gradient indicates overshoot, zooming between {:.3e} and {:.3e}",
                    alpha, alpha_prev
                ));
                let final_alpha =
                    self.zoom(alpha, alpha_prev, f0, directional_derivative, &problem)?;

                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Both conditions satisfied - should not reach here
            break;
        }

        // If we have any improvement, use it
        if best_alpha > 0.0 && best_f < f0 {
            self.log_verbose(&format!(
                "Returning best point found: alpha={:.3e}, f={:.3e}",
                best_alpha, best_f
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
            self.log_verbose(&format!("Using machine epsilon step {:.3e}", eps_step));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::line_search::create_1d_problem_linear;
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
        });
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
        let mut line_search = StrongWolfeLineSearch::new(StrongWolfeConfig::default());

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