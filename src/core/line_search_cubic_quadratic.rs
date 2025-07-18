use crate::core::line_search::OneDimensionalProblem;
use crate::core::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use log::debug;

/// Configuration for Cubic/Quadratic interpolation line search
#[derive(Debug, Clone)]
pub struct CubicQuadraticConfig {
    pub c1: f64,
    pub c2: f64,
    pub max_iterations: usize,
    pub min_step: f64,
    pub max_step: f64,
    pub initial_step: f64,
    pub verbose: bool,
    pub interpolation_safeguard: f64, // Minimum fraction of interval to move
    pub extrapolation_factor: f64,    // Factor for extrapolation steps
    pub line_bracket_method: u8, // 1: gradient-based bracketing, 2: function-value-based bracketing
}

impl Default for CubicQuadraticConfig {
    fn default() -> Self {
        Self {
            c1: 1e-4,
            c2: 0.9,
            max_iterations: 50,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
            interpolation_safeguard: 0.1,
            extrapolation_factor: 2.0,
            line_bracket_method: 1, // Default to method 1
        }
    }
}

/// Cubic/Quadratic interpolation line search
#[derive(Debug, Clone)]
pub struct CubicQuadraticLineSearch {
    config: CubicQuadraticConfig,
}

impl CubicQuadraticLineSearch {
    pub fn new(config: CubicQuadraticConfig) -> Self {
        Self { config }
    }
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("CubicQuadratic: {}", message);
        }
    }

    /// Cubic interpolation between two points with function and gradient values
    fn cubic_interpolate(&self, a: f64, fa: f64, ga: f64, b: f64, fb: f64, gb: f64) -> Option<f64> {
        let h = b - a;
        if h.abs() < 1e-15 {
            return None;
        }

        let d1 = ga + gb - 3.0 * (fa - fb) / h;
        let d2_squared = d1 * d1 - ga * gb;
        if d2_squared < 0.0 {
            return None; // No real solution
        }
        let d2 = d2_squared.sqrt();

        // Choose the root that gives a point between a and b
        let numerator1 = gb + d2 - d1;
        let numerator2 = gb - d2 - d1;
        let denominator = gb - ga + 2.0 * d2;

        if denominator.abs() < 1e-15 {
            return None;
        }

        let t1 = b - h * numerator1 / denominator;
        let t2 = b - h * numerator2 / denominator;

        // Choose the root that's between a and b
        let min_ab = a.min(b);
        let max_ab = a.max(b);

        let t = if t1 >= min_ab && t1 <= max_ab {
            t1
        } else if t2 >= min_ab && t2 <= max_ab {
            t2
        } else {
            // If neither root is in the interval, use the closer one
            if (t1 - 0.5 * (a + b)).abs() < (t2 - 0.5 * (a + b)).abs() {
                t1
            } else {
                t2
            }
        };

        // Safeguard: ensure we move at least a minimum fraction
        let min_move = self.config.interpolation_safeguard * (b - a).abs();
        let max_a = a.max(b);
        let min_a = a.min(b);
        if t < min_a + min_move {
            Some(min_a + min_move)
        } else if t > max_a - min_move {
            Some(max_a - min_move)
        } else {
            Some(t)
        }
    }
    /// Quadratic interpolation using function values and one gradient
    fn quadratic_interpolate(&self, a: f64, fa: f64, ga: f64, b: f64, fb: f64) -> Option<f64> {
        let h = b - a;
        if h.abs() < 1e-15 {
            return None;
        }

        let denom = 2.0 * (fb - fa - ga * h);
        if denom.abs() < 1e-15 {
            return None;
        }
        let t = a - ga * h * h / denom;
        // Safeguard
        let min_move = self.config.interpolation_safeguard * (b - a).abs();
        let max_a = a.max(b);
        let min_a = a.min(b);
        if t < min_a + min_move {
            Some(min_a + min_move)
        } else if t > max_a - min_move {
            Some(max_a - min_move)
        } else {
            Some(t)
        }
    }
    /// Check Wolfe conditions
    fn check_wolfe(
        &self,
        f0: f64,
        f_alpha: f64,
        g_alpha: f64,
        alpha: f64,
        g0: f64,
    ) -> (bool, bool) {
        let armijo = f_alpha <= f0 + self.config.c1 * alpha * g0;
        let curvature = g_alpha.abs() <= self.config.c2 * g0.abs();
        (armijo, curvature)
    }
}

impl LineSearch for CubicQuadraticLineSearch {
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem<'a>,
    ) -> anyhow::Result<LineSearchResult> {
        let f0 = (problem.objective)(0.0)?;
        let g0 = problem.initial_directional_derivative;
        if g0 >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }
        // Verify we can make progress
        let test_step = self.config.min_step;
        let f_test = (problem.objective)(test_step)?;
        if f_test >= f0 {
            let eps_step = f64::EPSILON.sqrt();
            let f_eps = (problem.objective)(eps_step)?;
            if f_eps < f0 {
                return Ok(LineSearchResult {
                    step_size: eps_step,
                    success: true,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                });
            }
            return Err(anyhow!("Function appears to be ill-conditioned: no improvement possible within machine precision"));
        }

        let mut alpha = self.config.initial_step;
        let mut alpha_prev = 0.0;
        let mut f_prev = f0;
        let mut g_prev = g0;
        let mut best_alpha = 0.0;
        let mut best_f = f0;

        self.log_verbose(&format!(
            "Starting with f(0)={:.3e}, g(0)={:.3e}, initial_step={:.3e}",
            f0, g0, alpha
        ));
        for iter in 0..self.config.max_iterations {
            // Evaluate at current step
            let f_alpha = (problem.objective)(alpha)?;
            let g_alpha = (problem.gradient)(alpha)?;
            // Track best point
            if f_alpha < best_f {
                best_f = f_alpha;
                best_alpha = alpha;
            }

            self.log_verbose(&format!(
                "Iteration {}: alpha={:.3e}, f={:.3e}, g={:.3e}",
                iter, alpha, f_alpha, g_alpha
            ));
            // Check Wolfe conditions
            let (armijo, curvature) = self.check_wolfe(f0, f_alpha, g_alpha, alpha, g0);
            if armijo && curvature {
                self.log_verbose("Wolfe conditions satisfied");
                return Ok(LineSearchResult {
                    step_size: alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }
            // If Armijo condition fails or function increased, interpolate
            if !armijo || (iter > 0 && f_alpha >= f_prev) {
                self.log_verbose("Interpolating due to Armijo failure or function increase");

                // Determine the interval to interpolate in
                let (left, right, f_left, g_left, f_right, g_right) = if iter == 0 {
                    // First iteration: interpolate between 0 and alpha
                    (0.0, alpha, f0, g0, f_alpha, g_alpha)
                } else {
                    // Later iterations: interpolate between alpha_prev and alpha
                    (alpha_prev, alpha, f_prev, g_prev, f_alpha, g_alpha)
                };

                // Try cubic interpolation first, then quadratic
                let new_alpha = self
                    .cubic_interpolate(left, f_left, g_left, right, f_right, g_right)
                    .or_else(|| self.quadratic_interpolate(left, f_left, g_left, right, f_right))
                    .unwrap_or(0.5 * (left + right));

                // Ensure the new step is within bounds
                alpha = new_alpha
                    .max(self.config.min_step)
                    .min(self.config.max_step);

                self.log_verbose(&format!("Interpolated new alpha: {:.3e}", alpha));
                continue;
            }
            // If curvature condition fails but Armijo is satisfied
            if !curvature {
                // Curvature condition failed - gradient magnitude too large
                // This means we need a larger step size
                alpha_prev = alpha;
                f_prev = f_alpha;
                g_prev = g_alpha;
                alpha = (alpha * self.config.extrapolation_factor).min(self.config.max_step);

                self.log_verbose(&format!("Extrapolated for curvature: {:.3e}", alpha));
                continue;
            }
        }

        // Return best point found if we have improvement
        if best_alpha > 0.0 && best_f < f0 {
            Ok(LineSearchResult {
                step_size: best_alpha,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
            })
        } else {
            // Try a very small step as last resort
            let small_step = self.config.min_step * 10.0;
            let f_small = (problem.objective)(small_step)?;
            if f_small < f0 {
                Ok(LineSearchResult {
                    step_size: small_step,
                    success: true,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                })
            } else {
                Err(anyhow!("Line search failed to find any improvement"))
            }
        }
    }
    fn reset(&mut self) {
        // Stateless
    }
    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::line_search::create_1d_problem_linear;
    use approx::assert_relative_eq;

    fn quadratic_function(x: &[f64]) -> anyhow::Result<f64> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }

    fn quadratic_gradient1(x: &[f64]) -> anyhow::Result<Vec<f64>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }


    #[test]
    fn test_cubic_quadratic_interpolation() {
        let mut line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig {
            verbose: false,
            ..CubicQuadraticConfig::default()
        });
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
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
        // Cubic/quadratic interpolation should find good step
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }
}
