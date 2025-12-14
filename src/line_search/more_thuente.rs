use crate::line_search::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::{anyhow, Result};
use log::debug;
use luminal::prelude::*;
use std::f32::EPSILON;

/// Configuration for the More-Thuente line search algorithm.
///
/// The More-Thuente line search is a sophisticated algorithm that finds step sizes
/// satisfying the strong Wolfe conditions. It maintains an interval that brackets
/// an acceptable step and uses polynomial interpolation to efficiently narrow
/// this interval.
///
/// # Parameters
/// - `c1`: Armijo condition parameter (sufficient decrease)
/// - `c2`: Strong Wolfe curvature condition parameter
/// - `xtol`: Relative tolerance for step size convergence
/// - `ftol`: Tolerance for function decrease (currently unused but reserved)
/// - `min_step`/`max_step`: Bounds on acceptable step sizes
/// - `max_iterations`: Maximum number of function evaluations
/// - `initial_step`: Starting step size for the search
#[derive(Debug, Clone)]
pub struct MoreThuenteConfig {
    pub c1: f32,
    pub c2: f32,
    pub max_iterations: usize,
    pub min_step: f32,
    pub max_step: f32,
    pub initial_step: f32,
    pub verbose: bool,
    pub xtol: f32, // Relative tolerance for step size
    pub ftol: f32, // Tolerance for function decrease
}

impl Default for MoreThuenteConfig {
    /// Creates a balanced configuration suitable for most optimization problems.
    ///
    /// Uses standard values from numerical optimization literature:
    /// - c1 = 1e-4: Standard Armijo parameter ensuring sufficient decrease
    /// - c2 = 0.9: Standard strong Wolfe parameter for quasi-Newton methods
    /// - xtol = 1e-12: Tight relative tolerance for step size
    /// - ftol = 1e-6: Function tolerance (reserved for future use)
    /// - 50 iterations: Usually sufficient for most well-conditioned problems
    fn default() -> Self {
        Self {
            c1: 1e-4, // Standard Armijo parameter
            c2: 0.9,  // Standard strong Wolfe parameter
            max_iterations: 50,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
            xtol: 1e-12, // Relative tolerance for step size
            ftol: 1e-6,  // Relative tolerance for function decrease
        }
    }
}
impl MoreThuenteConfig {
    /// Create a strict configuration for high-precision optimization
    ///
    /// Best for:
    /// - High-precision scientific computing
    /// - When function evaluations are cheap
    /// - Problems requiring very accurate line searches
    ///
    /// Trade-offs:
    /// - More function evaluations per line search
    /// - Tighter tolerances for very accurate line searches
    /// - More iterations allowed
    /// - Stricter Wolfe conditions
    pub fn strict() -> Self {
        Self {
            c1: 1e-4,
            c2: 0.1, // Much stricter curvature condition
            max_iterations: 100,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
            xtol: 1e-15, // Very tight step tolerance
            ftol: 1e-8,  // Very tight function tolerance
        }
    }
    /// Create a lax configuration for fast, approximate line searches
    ///
    /// Best for:
    /// - Large-scale optimization where speed is critical
    /// - When function evaluations are expensive
    /// - Early stages of optimization algorithms
    ///
    /// Trade-offs:
    /// - Less precise step sizes but faster convergence to approximate solutions
    /// - Looser tolerances for faster convergence
    /// - Fewer iterations
    /// - More permissive Wolfe conditions
    pub fn lax() -> Self {
        Self {
            c1: 1e-3, // More permissive Armijo condition
            c2: 0.9,  // Standard curvature condition
            max_iterations: 20,
            min_step: 1e-12,
            max_step: 1e12,
            initial_step: 1.0,
            verbose: false,
            xtol: 1e-8, // Looser step tolerance
            ftol: 1e-4, // Looser function tolerance
        }
    }
    /// Create the default configuration
    pub fn default_config() -> Self {
        Self::default()
    }
}

/// More-Thuente line search implementation.
///
/// This is a robust and efficient line search algorithm that finds step sizes satisfying
/// the strong Wolfe conditions. It's particularly well-suited for quasi-Newton methods
/// and other optimization algorithms requiring reliable step size selection.
///
/// # Algorithm Overview
///
/// The More-Thuente algorithm maintains an interval [stx, sty] that brackets an acceptable
/// step size. It uses polynomial interpolation (cubic when possible, quadratic as fallback)
/// to generate trial points and systematically narrows the bracket until convergence.
///
/// # Strengths
/// - **Robust**: Handles ill-conditioned functions and numerical difficulties gracefully
/// - **Efficient**: Uses sophisticated interpolation to minimize function evaluations
/// - **Reliable**: Guarantees strong Wolfe conditions when they exist
/// - **Well-tested**: Extensively used in production optimization software
/// - **Adaptive**: Automatically adjusts to function behavior during the search
///
/// # Weaknesses
/// - **Complex**: More sophisticated than simple backtracking, harder to understand/debug
/// - **Overhead**: May be overkill for simple problems where backtracking suffices
/// - **Parameter sensitivity**: Performance can depend on c1, c2 parameter choices
/// - **Memory**: Maintains more state than simpler line search methods
///
/// # When to Use
/// - **Recommended for**: Quasi-Newton methods (BFGS, L-BFGS), conjugate gradient
/// - **Good for**: Problems where function evaluations are moderately expensive
/// - **Consider alternatives for**: Very cheap functions (backtracking), very expensive functions (trust region)
///
/// # References
/// Based on "Line Search Algorithms with Guaranteed Sufficient Decrease" by
/// Moré and Thuente (1994), ACM Transactions on Mathematical Software.
///
/// # Example
/// ```rust,ignore
/// let line_search = MoreThuenteLineSearch::moderate().with_verbose();
/// let result = line_search.optimize_1d(&problem)?;
/// ```
#[derive(Debug, Clone)]
pub struct MoreThuenteLineSearch<S: Shape> {
    config: MoreThuenteConfig,
}

impl<S: Shape> MoreThuenteLineSearch<S> {
    /// Set the initial step size for the next line search
    ///
    /// The step size will be clamped to [min_step, max_step] bounds.
    /// A good initial step size can significantly improve performance.
    pub fn set_initial_step(&mut self, step: f32) {
        self.config.initial_step = step.clamp(self.config.min_step, self.config.max_step);
    }
    pub fn new(config: MoreThuenteConfig) -> Self {
        Self { config }
    }
    /// Create with default configuration
    pub fn default_search() -> Self {
        Self::new(MoreThuenteConfig::default())
    }

    /// Create a strict configuration for high-precision optimization
    ///
    /// Uses tighter tolerances and more iterations for problems requiring
    /// very accurate line searches. Best when function evaluations are
    /// relatively cheap and high precision is needed.
    ///
    /// - Tighter tolerances for very accurate line searches
    /// - More iterations allowed
    /// - Stricter Wolfe conditions
    pub fn strict() -> Self {
        Self::new(MoreThuenteConfig::strict())
    }

    /// Create a lax configuration for fast, approximate line searches
    ///
    /// Uses looser tolerances and fewer iterations for problems where
    /// speed is more important than precision. Best for large-scale
    /// optimization or when function evaluations are expensive.
    ///
    /// - Looser tolerances for faster convergence
    /// - Fewer iterations
    /// - More permissive Wolfe conditions
    pub fn lax() -> Self {
        Self::new(MoreThuenteConfig::lax())
    }

    /// Create the default/moderate configuration
    ///
    /// Balanced configuration suitable for most optimization problems.
    /// Same as `Default::default()` but more explicit about the intent.
    /// Good starting point for most applications.
    pub fn moderate() -> Self {
        Self::new(MoreThuenteConfig::default())
    }
    /// Create a configuration with verbose logging enabled
    pub fn with_verbose(mut self) -> Self {
        self.config.verbose = true;
        self
    }
    /// Log a message if verbose mode is enabled
    /// Uses the `log` crate's debug level for output
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("MoreThuente: {message}");
        }
    }
    /// Check strong Wolfe conditions
    ///
    /// Returns (armijo_satisfied, curvature_satisfied) where:
    /// - Armijo condition: f(α) ≤ f(0) + c1 * α * f'(0) (sufficient decrease)
    /// - Strong Wolfe curvature: |f'(α)| ≤ c2 * |f'(0)| (curvature condition)
    ///
    /// Both conditions must be satisfied for the step to be acceptable.
    /// The strong Wolfe conditions ensure both sufficient decrease and
    /// that the step size is not too small (avoiding slow convergence).
    fn check_wolfe_conditions(
        &self,
        f0: f32,
        f_alpha: f32,
        grad_alpha: f32,
        alpha: f32,
        grad0: f32,
    ) -> (bool, bool) {
        // Armijo condition: f(α) ≤ f(0) + c1 * α * f'(0)
        let armijo = f_alpha <= f0 + self.config.c1 * alpha * grad0;
        // Strong Wolfe curvature condition: |f'(α)| ≤ c2 * |f'(0)|
        let curvature = grad_alpha.abs() <= self.config.c2 * grad0.abs();
        (armijo, curvature)
    }
    /// Update interval using More-Thuente rules
    ///
    /// This is the core of the More-Thuente algorithm. It maintains an interval
    /// [stx, sty] that brackets an acceptable step and uses polynomial interpolation
    /// to generate new trial points.
    ///
    /// The algorithm handles four main cases:
    /// 1. Higher function value: Use cubic interpolation to find minimum
    /// 2. Lower function value with opposite derivative signs: Bracket found
    /// 3. Lower function value with decreasing gradient magnitude: Extrapolate
    /// 4. Lower function value with non-decreasing gradient: Handle bracketed case
    ///
    /// Each case uses different interpolation strategies optimized for that
    /// particular situation. The method includes safeguards against numerical
    /// instability and ensures the new step stays within reasonable bounds.
    fn update_interval(
        &self,
        stx: &mut f32,
        fx: &mut f32,
        gx: &mut f32,
        sty: &mut f32,
        fy: &mut f32,
        gy: &mut f32,
        stp: f32,
        fp: f32,
        gp: f32,
        brackt: &mut bool,
    ) -> f32 {
        let sgnd = gp * (*gx / gx.abs());
        // Safeguard against numerical issues
        let bound = if *brackt {
            (*stx).min(*sty)..=(*stx).max(*sty)
        } else {
            self.config.min_step..=self.config.max_step
        };

        // Case 1: Higher function value
        if fp > *fx {
            let theta = 3.0 * (*fx - fp) / (stp - *stx) + *gx + gp;
            let s = theta.abs().max(gx.abs()).max(gp.abs());
            let discriminant = (theta / s).powi(2) - (*gx / s) * (gp / s);
            let mut gamma = if discriminant > 0.0 {
                s * discriminant.sqrt()
            } else {
                0.0
            };
            if stp < *stx {
                gamma = -gamma;
            }
            let p = (gamma - *gx) + theta;
            let q = ((gamma - *gx) + gamma) + gp;
            let r = if q.abs() > EPSILON { p / q } else { 0.5 };
            let stpc = *stx + r * (stp - *stx);
            let stpq = *stx + ((*gx / ((*fx - fp) / (stp - *stx) + *gx)) / 2.0) * (stp - *stx);
            let stpf = if (stpc - *stx).abs() < (stpq - *stx).abs() {
                stpc
            } else {
                stpc + (stpq - stpc) / 2.0
            };
            *brackt = true;
            stpf.clamp(*bound.start(), *bound.end())
        }
        // Case 2: Lower function value, derivatives have opposite signs
        else if sgnd < 0.0 {
            let theta = 3.0 * (*fx - fp) / (stp - *stx) + *gx + gp;
            let s = theta.abs().max(gx.abs()).max(gp.abs());
            let discriminant = (theta / s).powi(2) - (*gx / s) * (gp / s);
            let mut gamma = if discriminant > 0.0 {
                s * discriminant.sqrt()
            } else {
                0.0
            };
            if stp > *stx {
                gamma = -gamma;
            }
            let p = (gamma - gp) + theta;
            let q = (gamma - gp) + gamma + *gx;
            let r = if q.abs() > EPSILON { p / q } else { 0.5 };
            let stpc = stp + r * (*stx - stp);
            let stpq = stp + (gp / (gp - *gx)) * (*stx - stp);
            let stpf = if (stpc - stp).abs() > (stpq - stp).abs() {
                stpc
            } else {
                stpq
            };
            *brackt = true;
            stpf.clamp(*bound.start(), *bound.end())
        }
        // Case 3: Lower function value, derivatives have same sign, decreasing
        else if gp.abs() < gx.abs() {
            let theta = 3.0 * (*fx - fp) / (stp - *stx) + *gx + gp;
            let s = theta.abs().max(gx.abs()).max(gp.abs());
            let mut gamma = if s == 0.0 {
                0.0
            } else {
                let discriminant = (theta / s).powi(2) - (*gx / s) * (gp / s);
                if discriminant > 0.0 {
                    s * discriminant.sqrt()
                } else {
                    0.0
                }
            };
            if stp > *stx {
                gamma = -gamma;
            }
            let p = (gamma - gp) + theta;
            let q = ((gamma - gp) + gamma) + *gx;
            let r = if q.abs() > EPSILON { p / q } else { 0.5 };
            let stpc = if r < 0.0 && gamma != 0.0 {
                stp + r * (*stx - stp)
            } else if stp > *stx {
                self.config.max_step
            } else {
                self.config.min_step
            };
            let stpq = if (gp - *gx).abs() > EPSILON {
                stp + (gp / (gp - *gx)) * (*stx - stp)
            } else {
                stp
            };
            let stpf = if *brackt {
                if (stp - stpc).abs() < (stp - stpq).abs() {
                    stpc
                } else {
                    stpq
                }
            } else if (stp - stpc).abs() > (stp - stpq).abs() {
                stpc
            } else {
                stpq
            };
            stpf.clamp(*bound.start(), *bound.end())
        }
        // Case 4: Lower function value, derivatives have same sign, not decreasing
        else if *brackt {
            let theta = 3.0 * (fp - *fy) / (*sty - stp) + *gy + gp;
            let s = theta.abs().max(gy.abs()).max(gp.abs());
            let discriminant = (theta / s).powi(2) - (*gy / s) * (gp / s);
            let mut gamma = if discriminant > 0.0 {
                s * discriminant.sqrt()
            } else {
                0.0
            };
            if stp > *sty {
                gamma = -gamma;
            }
            let p = (gamma - gp) + theta;
            let q = ((gamma - gp) + gamma) + *gy;
            let r = if q.abs() > EPSILON { p / q } else { 0.5 };
            let stpc = stp + r * (*sty - stp);
            stpc.clamp(*bound.start(), *bound.end())
        } else if stp > *stx {
            self.config.max_step
        } else {
            self.config.min_step
        }
    }
    /// Update the interval endpoints based on function values and gradients
    ///
    /// This method updates the bracket endpoints (stx, sty) based on the
    /// current trial point. It implements the More-Thuente update rules
    /// that ensure the interval always contains an acceptable step.
    ///
    /// The logic ensures that stx always corresponds to the best point
    /// found so far that satisfies the Armijo condition.
    fn update_endpoints(
        &self,
        stx: &mut f32,
        fx: &mut f32,
        gx: &mut f32,
        sty: &mut f32,
        fy: &mut f32,
        gy: &mut f32,
        stp: f32,
        fp: f32,
        gp: f32,
        f0: f32,
        g0: f32,
        iter: usize,
    ) {
        if fp > f0 + self.config.c1 * stp * g0 || (fp >= *fx && iter > 0) {
            *sty = stp;
            *fy = fp;
            *gy = gp;
        } else {
            if gp.abs() <= self.config.c2 * g0.abs() {
                return;
            }
            if gp * (*gx - gp) > 0.0 {
                *sty = *stx;
                *fy = *fx;
                *gy = *gx;
            }
            *stx = stp;
            *fx = fp;
            *gx = gp;
        }
    }
}

impl<S: Shape> LineSearch<S> for MoreThuenteLineSearch<S> {
    fn search(
        &mut self,
        cx: &mut Graph,
        params: GraphTensor<S>,
        loss: GraphTensor<S>,
        gradient: GraphTensor<S>,
        current_params: &[f32],
        direction: &[f32],
        initial_loss: f32,
        initial_gradient: &[f32],
    ) -> Result<LineSearchResult> {
        let f0 = initial_loss;
        let g0: f32 = initial_gradient
            .iter()
            .zip(direction.iter())
            .map(|(g, d)| g * d)
            .sum();

        // Validate input
        if g0 >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }
        if !f0.is_finite() || !g0.is_finite() {
            return Err(anyhow!("Initial function value or gradient is not finite"));
        }
        // Helper to evaluate function and gradient at a step size
        let mut evaluate = |step: f32| -> Result<(f32, f32)> {
            let new_params_data: Vec<f32> = current_params
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            params.set_on(cx, new_params_data);
            loss.retrieve_on(cx);
            params.grad().retrieve_on(cx);
            cx.execute();
            let loss_val = loss
                .data(cx)
                .ok_or_else(|| anyhow!("Failed to retrieve loss"))?[0];
            let grad_data = params
                .grad()
                .data(cx)
                .ok_or_else(|| anyhow!("Failed to retrieve gradients"))?;
            let dir_deriv: f32 = grad_data
                .iter()
                .zip(direction.iter())
                .map(|(g, d)| g * d)
                .sum();
            Ok((loss_val, dir_deriv))
        };


        // Verify we can make progress
        let test_step = self.config.min_step;
        let (f_test, _) = evaluate(test_step)?;
        if f_test >= f0 {
            let eps_step = f32::EPSILON.sqrt();
            let (f_eps, _) = evaluate(eps_step)?;
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
        let mut stx = 0.0_f64;
        let mut fx = f0;
        let mut gx = g0;
        let mut sty = 0.0;
        let mut fy = f0;
        let mut gy = g0;
        let mut brackt = false;
        let mut best_stp = 0.0;
        let mut best_f = f0;

        self.log_verbose(&format!(
            "Starting More-Thuente with f(0)={f0:.3e}, g(0)={g0:.3e}"
        ));

        for iter in 0..self.config.max_iterations {
            // Ensure step is within bounds
            stp = stp.clamp(self.config.min_step, self.config.max_step);
            // If we have a bracket, ensure step is within it
            if brackt {
                let step_min = stx.min(sty);
                let step_max = stx.max(sty);
                stp = stp.clamp(step_min, step_max);
            }

            // Evaluate function and gradient at current step
            let (fp, gp) = evaluate(stp)?;
            // Check for NaN or infinite values
            if !fp.is_finite() || !gp.is_finite() {
                self.log_verbose(&format!("Non-finite values at step {stp}: f={fp}, g={gp}"));
                // Return best point found so far
                if best_stp > 0.0 && best_f < f0 {
                    return Ok(LineSearchResult {
                        step_size: best_stp,
                        success: true,
                        termination_reason: TerminationReason::MaxIterationsReached,
                    });
                }
                return Err(anyhow!("Non-finite function or gradient value encountered"));
            }

            // Track best point
            if fp < best_f {
                best_f = fp;
                best_stp = stp;
            }

            self.log_verbose(&format!(
                "Line Search Iteration {iter}: stp={stp:.3e}, f={fp:.3e}, g={gp:.3e}"
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
            // Check for convergence based on interval width
            if brackt {
                let width = (sty - stx).abs();
                if width <= self.config.xtol * stx.abs().max(1.0) {
                    self.log_verbose("Converged: interval width below tolerance");
                    return Ok(LineSearchResult {
                        step_size: stp,
                        success: true,
                        termination_reason: TerminationReason::StepSizeTooSmall,
                    });
                }
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
            self.update_endpoints(
                &mut stx, &mut fx, &mut gx, &mut sty, &mut fy, &mut gy, stp, fp, gp, f0, g0, iter,
            );

            // Update step
            stp = new_stp;

            // Check for convergence
            if (stp - stx).abs() < self.config.xtol * stp.max(1.0) {
                self.log_verbose("Converged: step size change below tolerance");
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

    fn clone_box(&self) -> Box<dyn LineSearch<S>> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::line_search::line_search::create_1d_problem_linear;
    // use anyhow::Result;
    // use approx::assert_relative_eq;
    // use std::sync::Arc;

    fn quadratic_function(x: &[f32]) -> Result<f32> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f32>())
    }

    fn quadratic_gradient1(x: &[f32]) -> Result<Vec<f32>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }
    fn rosenbrock_function(x: &[f32]) -> Result<f32> {
        // f(x,y) = (1-x)^2 + 100*(y-x^2)^2
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Rosenbrock function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        Ok((1.0 - x1).powi(2) + 100.0 * (x2 - x1.powi(2)).powi(2))
    }
    fn rosenbrock_gradient(x: &[f32]) -> Result<Vec<f32>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Rosenbrock gradient requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let grad_x1 = -2.0 * (1.0 - x1) - 400.0 * x1 * (x2 - x1.powi(2));
        let grad_x2 = 200.0 * (x2 - x1.powi(2));
        Ok(vec![grad_x1, grad_x2])
    }
    fn steep_function(x: &[f32]) -> Result<f32> {
        // A function with steep gradients to test edge cases
        Ok(x[0].exp())
    }
    fn steep_gradient(x: &[f32]) -> Result<Vec<f32>> {
        Ok(vec![x[0].exp()])
    }

    /*
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
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
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
            Arc::new(rosenbrock_function),
            Arc::new(rosenbrock_gradient),
        )
        .unwrap();
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
    */
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
        let gp = 0.5; // Positive gradient (opposite sign to gx)
        let new_stp = line_search.update_interval(
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
        assert!(new_stp >= 0.0);
        // Without bracket, should extrapolate
        // The actual behavior depends on the specific case
        assert!(new_stp == line_search.config.max_step || new_stp == line_search.config.min_step);
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
        let (armijo, curvature) =
            line_search.check_wolfe_conditions(f0, f_alpha, grad_alpha, alpha, grad0);
        assert!(armijo);
        assert!(curvature);
        // Test violated Armijo condition
        let f_alpha = 1.1; // Violates Armijo
        let (armijo, _) = line_search.check_wolfe_conditions(f0, f_alpha, grad_alpha, alpha, grad0);
        assert!(!armijo);
        // Test violated curvature condition
        let f_alpha = 0.9;
        let grad_alpha = -0.95; // Violates curvature
        let (_, curvature) =
            line_search.check_wolfe_conditions(f0, f_alpha, grad_alpha, alpha, grad0);
        assert!(!curvature);
    }
    /*
    #[test]
    fn test_non_descent_direction() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Create a problem directly with positive directional derivative
        // to test the line search's own validation
        let objective = |alpha: f32| -> Result<f32> {
            Ok(1.0 + alpha) // Increasing function
        };
        let gradient = |_alpha: f32| -> Result<f32> {
            Ok(1.0) // Positive gradient
        };

        let problem = OneDimensionalProblem {
            objective: Arc::new(objective),
            gradient: Arc::new(gradient),
            initial_directional_derivative: 1.0, // Positive (non-descent)
        };

        let result = line_search.optimize_1d(&problem);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not a descent direction"));
    }
    #[test]
    fn test_ill_conditioned_function() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig {
            min_step: 1e-16,
            ..MoreThuenteConfig::default()
        });
        // Create a flat function that doesn't improve
        let flat_function = |_x: &[f32]| -> Result<f32> { Ok(1.0) };
        let flat_gradient = |_x: &[f32]| -> Result<Vec<f32>> { Ok(vec![-1.0]) };
        let current_point = vec![0.0];
        let direction = vec![1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(flat_function),
            Arc::new(flat_gradient),
        )
        .unwrap();
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
            xtol: 1e-20,       // Make step size tolerance very small to avoid early convergence
            verbose: false,
            ..MoreThuenteConfig::default()
        });
        let current_point = vec![10.0]; // Start far from optimum
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(steep_function),
            Arc::new(steep_gradient),
        )
        .unwrap();
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
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        )
        .unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size >= line_search.config.min_step);
        assert!(result.step_size <= line_search.config.max_step);
    }
    */
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
        assert_eq!(config.xtol, 1e-12);
        assert_eq!(config.ftol, 1e-6);
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
    #[test]
    fn test_static_constructors() {
        // Test strict configuration
        let strict = MoreThuenteLineSearch::strict();
        assert_eq!(strict.config.c2, 0.1); // Stricter curvature
        assert_eq!(strict.config.max_iterations, 100);
        assert_eq!(strict.config.xtol, 1e-15);
        assert_eq!(strict.config.ftol, 1e-8);
        // Test lax configuration
        let lax = MoreThuenteLineSearch::lax();
        assert_eq!(lax.config.c1, 1e-3); // More permissive Armijo
        assert_eq!(lax.config.max_iterations, 20);
        assert_eq!(lax.config.xtol, 1e-8);
        assert_eq!(lax.config.ftol, 1e-4);
        // Test moderate configuration
        let moderate = MoreThuenteLineSearch::moderate();
        let default = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        assert_eq!(moderate.config.c1, default.config.c1);
        assert_eq!(moderate.config.c2, default.config.c2);
        assert_eq!(moderate.config.xtol, default.config.xtol);
        assert_eq!(moderate.config.ftol, default.config.ftol);
    }
    #[test]
    fn test_with_verbose() {
        let line_search = MoreThuenteLineSearch::moderate().with_verbose();
        assert!(line_search.config.verbose);
        let strict_verbose = MoreThuenteLineSearch::strict().with_verbose();
        assert!(strict_verbose.config.verbose);
        assert_eq!(strict_verbose.config.c2, 0.1); // Should preserve other settings
    }
    /*
    #[test]
    fn test_strict_vs_lax_behavior() {
        // This test verifies that strict and lax configurations behave differently
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(quadratic_function),
            Arc::new(quadratic_gradient1),
        )
        .unwrap();
        let mut strict = MoreThuenteLineSearch::strict();
        let mut lax = MoreThuenteLineSearch::lax();
        let strict_result = strict.optimize_1d(&problem).unwrap();
        let lax_result = lax.optimize_1d(&problem).unwrap();
        // Both should succeed
        assert!(strict_result.success);
        assert!(lax_result.success);
        // For this simple quadratic, both should find similar step sizes
        // but strict might be more precise
        assert_relative_eq!(strict_result.step_size, 1.0, epsilon = 1e-8);
        assert_relative_eq!(lax_result.step_size, 1.0, epsilon = 1e-4);
    }
    #[test]
    fn test_numerical_stability() {
        // Test with very small gradients
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        let tiny_gradient_fn = |x: &[f32]| -> Result<f32> { Ok(x[0] * 1e-15) };
        let tiny_gradient_grad = |_: &[f32]| -> Result<Vec<f32>> { Ok(vec![1e-15]) };
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            Arc::new(tiny_gradient_fn),
            Arc::new(tiny_gradient_grad),
        )
        .unwrap();
        let result = line_search.optimize_1d(&problem);
        // Should handle tiny gradients gracefully
        assert!(result.is_ok());
    }
    #[test]
    fn test_nan_handling() {
        let mut line_search = MoreThuenteLineSearch::new(MoreThuenteConfig::default());
        // Function that returns NaN after certain step
        let nan_function = |alpha: f32| -> Result<f32> {
            if alpha > 0.5 {
                Ok(f32::NAN)
            } else {
                Ok(1.0 - alpha)
            }
        };
        let nan_gradient = |alpha: f32| -> Result<f32> {
            if alpha > 0.5 {
                Ok(f32::NAN)
            } else {
                Ok(-1.0)
            }
        };
        let problem = OneDimensionalProblem {
            objective: Arc::new(nan_function),
            gradient: Arc::new(nan_gradient),
            initial_directional_derivative: -1.0,
        };
        let result = line_search.optimize_1d(&problem);
        // Should either succeed with a step < 0.5 or fail gracefully
        if let Ok(res) = result {
            assert!(res.step_size <= 0.5);
        } else {
            assert!(result.unwrap_err().to_string().contains("Non-finite"));
        }
    }
    */
}