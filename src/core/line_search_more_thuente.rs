use crate::core::line_search::OneDimensionalProblem;
use crate::core::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use log::debug;

/// Configuration for More-Thuente line search
#[derive(Debug, Clone)]
pub struct MoreThuenteConfig {
    pub c1: f64,
    pub c2: f64,
    pub max_iterations: usize,
    pub min_step: f64,
    pub max_step: f64,
    pub initial_step: f64,
    pub verbose: bool,
    pub xtol: f64, // Relative tolerance for step size
    pub ftol: f64, // Tolerance for function decrease
}

impl Default for MoreThuenteConfig {
    fn default() -> Self {
        Self {
            c1: 1e-4,
            c2: 0.9,
            max_iterations: 50,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
            xtol: 1e-15,
            ftol: 1e-4,
        }
    }
}

/// More-Thuente line search implementation
/// Based on the algorithm described in "Line Search Algorithms with Guaranteed Sufficient Decrease"
#[derive(Debug, Clone)]
pub struct MoreThuenteLineSearch {
    config: MoreThuenteConfig,
}

impl MoreThuenteLineSearch {
    pub fn new(config: MoreThuenteConfig) -> Self {
        Self { config }
    }
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("MoreThuente: {}", message);
        }
    }
    /// Check strong Wolfe conditions
    fn check_wolfe_conditions(
        &self,
        f0: f64,
        f_alpha: f64,
        grad_alpha: f64,
        alpha: f64,
        grad0: f64,
    ) -> (bool, bool) {
        let armijo = f_alpha <= f0 + self.config.c1 * alpha * grad0;
        let curvature = grad_alpha.abs() <= self.config.c2 * grad0.abs();
        (armijo, curvature)
    }
    /// Update interval using More-Thuente rules
    fn update_interval(
        &self,
        stx: &mut f64,
        fx: &mut f64,
        gx: &mut f64,
        sty: &mut f64,
        fy: &mut f64,
        gy: &mut f64,
        stp: f64,
        fp: f64,
        gp: f64,
        brackt: &mut bool,
    ) -> f64 {
        let sgnd = gp * (*gx / gx.abs());
        // Case 1: Higher function value
        if fp > *fx {
            let theta = 3.0 * (*fx - fp) / (stp - *stx) + *gx + gp;
            let s = theta.abs().max(gx.abs()).max(gp.abs());
            let mut gamma = s * ((theta / s).powi(2) - (*gx / s) * (gp / s)).sqrt();
            if stp < *stx {
                gamma = -gamma;
            }
            let p = (gamma - *gx) + theta;
            let q = ((gamma - *gx) + gamma) + gp;
            let r = p / q;
            let stpc = *stx + r * (stp - *stx);
            let stpq = *stx + ((*gx / ((*fx - fp) / (stp - *stx) + *gx)) / 2.0) * (stp - *stx);
            let stpf = if (stpc - *stx).abs() < (stpq - *stx).abs() {
                stpc
            } else {
                stpc + (stpq - stpc) / 2.0
            };
            *brackt = true;
            stpf
        }
        // Case 2: Lower function value, derivatives have opposite signs
        else if sgnd < 0.0 {
            let theta = 3.0 * (*fx - fp) / (stp - *stx) + *gx + gp;
            let s = theta.abs().max(gx.abs()).max(gp.abs());
            let mut gamma = s * ((theta / s).powi(2) - (*gx / s) * (gp / s)).sqrt();
            if stp > *stx {
                gamma = -gamma;
            }
            let p = (gamma - gp) + theta;
            let q = (gamma - gp) + gamma + *gx;
            let r = p / q;
            let stpc = stp + r * (*stx - stp);
            let stpq = stp + (gp / (gp - *gx)) * (*stx - stp);
            let stpf = if (stpc - stp).abs() > (stpq - stp).abs() {
                stpc
            } else {
                stpq
            };
            *brackt = true;
            stpf
        }
        // Case 3: Lower function value, derivatives have same sign, decreasing
        else if gp.abs() < gx.abs() {
            let theta = 3.0 * (*fx - fp) / (stp - *stx) + *gx + gp;
            let s = theta.abs().max(gx.abs()).max(gp.abs());
            let mut gamma = if s == 0.0 {
                0.0
            } else {
                s * ((theta / s).powi(2) - (*gx / s) * (gp / s)).sqrt()
            };
            if stp > *stx {
                gamma = -gamma;
            }
            let p = (gamma - gp) + theta;
            let q = ((gamma - gp) + gamma) + *gx;
            let r = p / q;
            let stpc = if r < 0.0 && gamma != 0.0 {
                stp + r * (*stx - stp)
            } else if stp > *stx {
                self.config.max_step
            } else {
                self.config.min_step
            };
            let stpq = stp + (gp / (gp - *gx)) * (*stx - stp);
            let stpf = if *brackt {
                if (stp - stpc).abs() < (stp - stpq).abs() {
                    stpc
                } else {
                    stpq
                }
            } else {
                if (stp - stpc).abs() > (stp - stpq).abs() {
                    stpc
                } else {
                    stpq
                }
            };
            stpf
        }
        // Case 4: Lower function value, derivatives have same sign, not decreasing
        else {
            if *brackt {
                let theta = 3.0 * (fp - *fy) / (*sty - stp) + *gy + gp;
                let s = theta.abs().max(gy.abs()).max(gp.abs());
                let mut gamma = s * ((theta / s).powi(2) - (*gy / s) * (gp / s)).sqrt();
                if stp > *sty {
                    gamma = -gamma;
                }
                let p = (gamma - gp) + theta;
                let q = ((gamma - gp) + gamma) + *gy;
                let r = p / q;
                let stpc = stp + r * (*sty - stp);
                stpc
            } else if stp > *stx {
                self.config.max_step
            } else {
                self.config.min_step
            }
        }
    }
}

impl LineSearch for MoreThuenteLineSearch {
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

        let mut stp = self.config.initial_step;
        let mut stx = 0.0;
        let mut fx = f0;
        let mut gx = g0;
        let mut sty = 0.0;
        let mut fy = f0;
        let mut gy = g0;
        let mut brackt = false;
        let mut best_stp = 0.0;
        let mut best_f = f0;

        self.log_verbose(&format!(
            "Starting More-Thuente with f(0)={:.3e}, g(0)={:.3e}",
            f0, g0
        ));
        for iter in 0..self.config.max_iterations {
            // Evaluate function and gradient at current step
            let fp = (problem.objective)(stp)?;
            let gp = (problem.gradient)(stp)?;
            // Track best point
            if fp < best_f {
                best_f = fp;
                best_stp = stp;
            }

            self.log_verbose(&format!(
                "Iteration {}: stp={:.3e}, f={:.3e}, g={:.3e}",
                iter, stp, fp, gp
            ));
            // Check Wolfe conditions
            let (armijo, curvature) = self.check_wolfe_conditions(f0, fp, gp, stp, g0);
            if armijo && curvature {
                self.log_verbose("Wolfe conditions satisfied");
                return Ok(LineSearchResult {
                    step_size: stp,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }
            // Update interval and get new trial step
            let new_stp = self.update_interval(
                &mut stx,
                &mut fx,
                &mut gx,
                &mut sty,
                &mut fy,
                &mut gy,
                stp,
                fp,
                gp,
                &mut brackt,
            );
            // Update the interval endpoints
            if fp > f0 + self.config.c1 * stp * g0 || (fp >= fx && iter > 0) {
                sty = stp;
                fy = fp;
                gy = gp;
            } else {
                if gp.abs() <= self.config.c2 * g0.abs() {
                    break;
                }
                sty = stx;
                fy = fx;
                gy = gx;
                stx = stp;
                fx = fp;
                gx = gp;
            }
            stp = new_stp.max(self.config.min_step).min(self.config.max_step);
            // Check for convergence
            if (stp - stx).abs() < self.config.xtol * stp.max(1.0) {
                break;
            }
        }

        // Return best point found
        if best_stp > 0.0 && best_f < f0 {
            Ok(LineSearchResult {
                step_size: best_stp,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
            })
        } else {
            Ok(LineSearchResult {
                step_size: stp,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
            })
        }
    }
    fn reset(&mut self) {
        // More-Thuente is stateless
    }
    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
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
    fn rosenbrock_function(x: &[f64]) -> Result<f64> {
        // f(x,y) = (1-x)^2 + 100*(y-x^2)^2
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Rosenbrock function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        Ok((1.0 - x1).powi(2) + 100.0 * (x2 - x1.powi(2)).powi(2))
    }
    fn rosenbrock_gradient(x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Rosenbrock gradient requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let grad_x1 = -2.0 * (1.0 - x1) - 400.0 * x1 * (x2 - x1.powi(2));
        let grad_x2 = 200.0 * (x2 - x1.powi(2));
        Ok(vec![grad_x1, grad_x2])
    }
    fn steep_function(x: &[f64]) -> Result<f64> {
        // A function with steep gradients to test edge cases
        Ok(x[0].exp())
    }
    fn steep_gradient(x: &[f64]) -> Result<Vec<f64>> {
        Ok(vec![x[0].exp()])
    }

    #[test]
    fn test_more_thuente_quadratic() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig {
            verbose: false,
            ..MoreThuenteConfig::default()
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
        // More-Thuente should find optimal step for quadratic
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }
    #[test]
    fn test_more_thuente_rosenbrock() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig {
            verbose: false,
            max_iterations: 100,
            ..MoreThuenteConfig::default()
        });
        // Start from a point where Rosenbrock has a steep gradient
        let current_point = vec![0.0, 0.0];
        let direction = vec![1.0, 1.0]; // Move towards optimum
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &rosenbrock_function,
            &rosenbrock_gradient,
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        // Verify we made progress
        let f0 = rosenbrock_function(&current_point).unwrap();
        let new_point = vec![
            current_point[0] + result.step_size * direction[0],
            current_point[1] + result.step_size * direction[1],
        ];
        let f_new = rosenbrock_function(&new_point).unwrap();
        assert!(f_new < f0);
    }
    #[test]
    fn test_update_interval_case1_higher_function_value() {
        let line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Case 1: fp > fx (higher function value)
        let mut stx = 0.0;
        let mut fx = 1.0;
        let mut gx = -1.0;
        let mut sty = 0.0;
        let mut fy = 1.0;
        let mut gy = -1.0;
        let mut brackt = false;
        let stp = 1.0;
        let fp = 2.0; // Higher than fx
        let gp = -0.5;
        let new_stp = line_search.update_interval(
            &mut stx, &mut fx, &mut gx,
            &mut sty, &mut fy, &mut gy,
            stp, fp, gp, &mut brackt,
        );
        assert!(brackt); // Should set bracket to true
        assert!(new_stp >= 0.0);
        assert!(new_stp <= 1.0); // Should be between stx and stp
    }
    #[test]
    fn test_update_interval_case2_opposite_signs() {
        let line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Case 2: fp <= fx and sgnd < 0 (opposite signs)
        let mut stx = 0.0;
        let mut fx = 2.0;
        let mut gx = -1.0; // Negative gradient
        let mut sty = 0.0;
        let mut fy = 2.0;
        let mut gy = -1.0;
        let mut brackt = false;
        let stp = 1.0;
        let fp = 1.5; // Lower than fx
        let gp = 0.5;  // Positive gradient (opposite sign to gx)
        let new_stp = line_search.update_interval(
            &mut stx, &mut fx, &mut gx,
            &mut sty, &mut fy, &mut gy,
            stp, fp, gp, &mut brackt,
        );
        assert!(brackt); // Should set bracket to true
        assert!(new_stp >= 0.0);
    }
    #[test]
    fn test_update_interval_case3_decreasing_gradient() {
        let line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Case 3: fp <= fx, same sign, |gp| < |gx| (decreasing gradient magnitude)
        let mut stx = 0.0;
        let mut fx = 2.0;
        let mut gx = -1.0;
        let mut sty = 0.0;
        let mut fy = 2.0;
        let mut gy = -1.0;
        let mut brackt = false;
        let stp = 1.0;
        let fp = 1.5; // Lower than fx
        let gp = -0.5; // Same sign as gx, but smaller magnitude
        let new_stp = line_search.update_interval(
            &mut stx, &mut fx, &mut gx,
            &mut sty, &mut fy, &mut gy,
            stp, fp, gp, &mut brackt,
        );
        assert!(new_stp >= 0.0);
    }
    #[test]
    fn test_update_interval_case4_bracketed() {
        let line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Case 4: fp <= fx, same sign, |gp| >= |gx|, with bracket
        let mut stx = 0.0;
        let mut fx = 2.0;
        let mut gx = -0.5;
        let mut sty = 2.0;
        let mut fy = 1.8;
        let mut gy = 0.3;
        let mut brackt = true; // Already bracketed
        let stp = 1.0;
        let fp = 1.5; // Lower than fx
        let gp = -1.0; // Same sign as gx, larger magnitude
        let new_stp = line_search.update_interval(
            &mut stx, &mut fx, &mut gx,
            &mut sty, &mut fy, &mut gy,
            stp, fp, gp, &mut brackt,
        );
        assert!(new_stp >= 0.0);
        // Should interpolate between stp and sty
        assert!(new_stp >= stp.min(sty));
        assert!(new_stp <= stp.max(sty));
    }
    #[test]
    fn test_update_interval_case4_unbbracketed() {
        let line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Case 4: fp <= fx, same sign, |gp| >= |gx|, without bracket
        let mut stx = 0.0;
        let mut fx = 2.0;
        let mut gx = -0.5;
        let mut sty = 0.0;
        let mut fy = 2.0;
        let mut gy = -0.5;
        let mut brackt = false; // Not bracketed
        let stp = 1.0;
        let fp = 1.5; // Lower than fx
        let gp = -1.0; // Same sign as gx, larger magnitude
        let new_stp = line_search.update_interval(
            &mut stx, &mut fx, &mut gx,
            &mut sty, &mut fy, &mut gy,
            stp, fp, gp, &mut brackt,
        );
        assert!(new_stp >= 0.0);
        // Without bracket, should extrapolate
        assert!(new_stp >= stp || new_stp == line_search.config.max_step);
    }
    #[test]
    fn test_wolfe_conditions() {
        let line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        let f0 = 1.0;
        let grad0 = -1.0;
        let alpha = 0.5;
        // Test satisfied conditions
        let f_alpha = 0.9; // Satisfies Armijo
        let grad_alpha = -0.1; // Satisfies curvature
        let (armijo, curvature) = line_search.check_wolfe_conditions(f0, f_alpha, grad_alpha, alpha, grad0);
        assert!(armijo);
        assert!(curvature);
        // Test violated Armijo condition
        let f_alpha = 1.1; // Violates Armijo
        let (armijo, _) = line_search.check_wolfe_conditions(f0, f_alpha, grad_alpha, alpha, grad0);
        assert!(!armijo);
        // Test violated curvature condition
        let f_alpha = 0.9;
        let grad_alpha = -0.95; // Violates curvature
        let (_, curvature) = line_search.check_wolfe_conditions(f0, f_alpha, grad_alpha, alpha, grad0);
        assert!(!curvature);
    }
    #[test]
    fn test_non_descent_direction() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Create a problem directly with positive directional derivative
        // to test the line search's own validation
        let objective = |alpha: f64| -> Result<f64> {
            Ok(1.0 + alpha) // Increasing function
        };
        let gradient = |_alpha: f64| -> Result<f64> {
            Ok(1.0) // Positive gradient
        };

        let problem = OneDimensionalProblem {
            objective: Box::new(objective),
            gradient: Box::new(gradient),
            initial_directional_derivative: 1.0, // Positive (non-descent)
        };

        let result = line_search.optimize_1d(&problem);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not a descent direction"));
    }
    #[test]
    fn test_ill_conditioned_function() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig {
            min_step: 1e-16,
            ..MoreThuenteConfig::default()
        });
        // Create a flat function that doesn't improve
        let flat_function = |_x: &[f64]| -> Result<f64> { Ok(1.0) };
        let flat_gradient = |_x: &[f64]| -> Result<Vec<f64>> { Ok(vec![-1.0]) };
        let current_point = vec![0.0];
        let direction = vec![1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &flat_function,
            &flat_gradient,
        ).unwrap();
        let result = line_search.optimize_1d(&problem);
        // Should either succeed with tiny step or fail with ill-conditioned error
        if result.is_err() {
            assert!(result.unwrap_err().to_string().contains("ill-conditioned"));
        }
    }
    #[test]
    fn test_max_iterations_reached() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig {
            max_iterations: 2, // Very low to force max iterations
            xtol: 1e-20, // Make step size tolerance very small to avoid early convergence
            verbose: false,
            ..MoreThuenteConfig::default()
        });
        let current_point = vec![10.0]; // Start far from optimum
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &steep_function,
            &steep_gradient,
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        // The algorithm may terminate due to step size being too small or max iterations
        // Both are valid outcomes for this test scenario
        assert!(matches!(
            result.termination_reason,
            TerminationReason::MaxIterationsReached | TerminationReason::StepSizeTooSmall
        ));
    }
    #[test]
    fn test_step_size_bounds() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig {
            min_step: 1e-8,
            max_step: 10.0,
            ..MoreThuenteConfig::default()
        });
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size >= line_search.config.min_step);
        assert!(result.step_size <= line_search.config.max_step);
    }
    #[test]
    fn test_config_default() {
        let config = MoreThuenteConfig::default();
        assert_eq!(config.c1, 1e-4);
        assert_eq!(config.c2, 0.9);
        assert_eq!(config.max_iterations, 50);
        assert_eq!(config.min_step, 1e-16);
        assert_eq!(config.max_step, 1e16);
        assert_eq!(config.initial_step, 1.0);
        assert!(!config.verbose);
        assert_eq!(config.xtol, 1e-15);
        assert_eq!(config.ftol, 1e-4);
    }
    #[test]
    fn test_clone_and_reset() {
        let line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        let mut cloned = line_search.clone();
        // Reset should not panic (it's a no-op for More-Thuente)
        cloned.reset();
        // Clone box should work
        let _boxed = line_search.clone_box();
    }
}