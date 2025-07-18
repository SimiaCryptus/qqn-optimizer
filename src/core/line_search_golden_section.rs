use crate::core::line_search::OneDimensionalProblem;
use crate::core::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use log::debug;

/// Configuration for Golden Section line search
#[derive(Debug, Clone)]
pub struct GoldenSectionConfig {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub min_step: f64,
    pub max_step: f64,
    pub initial_step: f64,
    pub verbose: bool,
}

impl Default for GoldenSectionConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-8,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
        }
    }
}

/// Golden Section line search implementation
/// Uses the golden ratio to narrow down the interval containing the minimum
#[derive(Debug, Clone)]
pub struct GoldenSectionLineSearch {
    config: GoldenSectionConfig,
}
impl LineSearch for GoldenSectionLineSearch {
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem<'a>,
    ) -> anyhow::Result<LineSearchResult> {
        let directional_derivative = problem.initial_directional_derivative;
        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }
        // First verify we can make progress
        let f0 = (problem.objective)(0.0)?;
        let test_step = self.config.min_step;
        let f_test = (problem.objective)(test_step)?;
        if f_test >= f0 {
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
            return Err(anyhow!("Function appears to be ill-conditioned: no improvement possible within machine precision"));
        }

        let step_size = self.find_minimum(problem)?;
        let success = step_size >= self.config.min_step && step_size <= self.config.max_step;
        Ok(LineSearchResult {
            step_size,
            success,
            termination_reason: if success {
                TerminationReason::WolfeConditionsSatisfied
            } else {
                TerminationReason::StepSizeTooSmall
            },
        })
    }
    fn reset(&mut self) {
        // Golden section search is stateless
    }
    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
    }
}
impl GoldenSectionLineSearch {
    pub fn new(config: GoldenSectionConfig) -> Self {
        Self { config }
    }
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("GoldenSection: {}", message);
        }
    }
    /// Golden ratio constant
    const RESPHI: f64 = 0.618033988749895; // 1/phi = phi - 1
    /// Find minimum using golden section search
    fn find_minimum(&self, problem: &OneDimensionalProblem) -> anyhow::Result<f64> {
        // First, establish a proper bracket [a, b, c] where f(b) < f(a) and f(b) < f(c)
        let (a, b, c) = self.find_bracket(problem)?;
        self.log_verbose(&format!(
            "Initial bracket: [{:.6e}, {:.6e}, {:.6e}]",
            a, b, c
        ));
        // Golden section search
        let mut left = a;
        let mut right = c;
        let mut x1 = left + Self::RESPHI * (right - left);
        let mut x2 = right - Self::RESPHI * (right - left);
        let mut f1 = (problem.objective)(x1)?;
        let mut f2 = (problem.objective)(x2)?;
        for i in 0..self.config.max_iterations {
            self.log_verbose(&format!(
                "Iteration {}: interval=[{:.3e}, {:.3e}], x1={:.3e}, x2={:.3e}, f1={:.3e}, f2={:.3e}",
                i, left, right, x1, x2, f1, f2
            ));
            if (right - left) < self.config.tolerance {
                break;
            }
            if f1 < f2 {
                // Minimum is in [left, x2]
                right = x2;
                x2 = x1;
                f2 = f1;
                x1 = left + Self::RESPHI * (right - left);
                f1 = (problem.objective)(x1)?;
            } else {
                // Minimum is in [x1, right]
                left = x1;
                x1 = x2;
                f1 = f2;
                x2 = right - Self::RESPHI * (right - left);
                f2 = (problem.objective)(x2)?;
            }
        }
        let final_x = if f1 < f2 { x1 } else { x2 };
        self.log_verbose(&format!("Golden section completed with x={:.3e}", final_x));
        Ok(final_x)
    }
    /// Find a proper bracket [a, b, c] where f(b) < f(a) and f(b) < f(c)
    fn find_bracket(&self, problem: &OneDimensionalProblem) -> anyhow::Result<(f64, f64, f64)> {
        let mut a = 0.0;
        let mut b = self.config.initial_step;
        // Ensure b > a
        if b <= a {
            b = a + self.config.initial_step;
        }
        let mut f_a = (problem.objective)(a)?;
        let mut f_b = (problem.objective)(b)?;
        // If f(b) > f(a), we need to look in the other direction or between them
        if f_b > f_a {
            // Try a point between a and b
            let mid = 0.5 * (a + b);
            let f_mid = (problem.objective)(mid)?;
            if f_mid < f_a && f_mid < f_b {
                // Found a minimum between a and b
                return Ok((a, mid, b));
            }
            // Look to the left of a (if possible)
            let left = (a - self.config.initial_step).max(self.config.min_step);
            if left < a {
                let f_left = (problem.objective)(left)?;
                if f_left > f_a {
                    // We have a bracket [left, a, b] but f(a) < f(b)
                    // Try to find a better middle point
                    let new_mid = 0.5 * (a + b);
                    let f_new_mid = (problem.objective)(new_mid)?;
                    if f_new_mid < f_a {
                        return Ok((a, new_mid, b));
                    }
                }
            }
        }
        // Standard bracketing: expand until we find a point where function increases
        while f_b <= f_a && b < self.config.max_step {
            let mut c = b + 2.0 * (b - a);
            if c > self.config.max_step {
                c = self.config.max_step;
            }
            let f_c = (problem.objective)(c)?;
            if f_c > f_b {
                // Found bracket [a, b, c]
                return Ok((a, b, c));
            }
            // Move forward
            a = b;
            f_a = f_b;
            b = c;
            f_b = f_c;
            if b >= self.config.max_step {
                break;
            }
        }
        // If we couldn't find a proper bracket, create one with max_step
        let c = self.config.max_step.min(b * 2.0);
        let f_c = (problem.objective)(c)?;
        // Return the best bracket we can find
        if f_b < f_a && f_b < f_c {
            Ok((a, b, c))
        } else if f_a < f_b && f_a < f_c {
            // Minimum might be at the left endpoint
            let mid = 0.5 * (a + b);
            Ok((a, mid, b))
        } else {
            // Use a small interval around the initial step
            let mid = 0.5 * self.config.initial_step;
            Ok((0.0, mid, self.config.initial_step))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::line_search::create_1d_problem_linear;

    fn quadratic_function(x: &[f64]) -> anyhow::Result<f64> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }

    fn quadratic_gradient1(x: &[f64]) -> anyhow::Result<Vec<f64>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }

    #[test]
    fn test_golden_section_quadratic() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let _current_gradient = quadratic_gradient1(&current_point).unwrap();
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
    }
}
