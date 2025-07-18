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
}
