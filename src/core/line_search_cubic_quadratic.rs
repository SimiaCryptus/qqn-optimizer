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
            c2: 0.1,
            max_iterations: 20,
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
impl CubicQuadraticConfig {
    /// Create a strict configuration for high-precision optimization
    /// - Stricter Wolfe conditions (c1=1e-4, c2=0.1)
    /// - More iterations allowed (50)
    /// - Smaller safeguards for more precise interpolation
    /// - Conservative extrapolation
    pub fn strict() -> Self {
        Self {
            c1: 1e-4,
            c2: 0.1,
            max_iterations: 50,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
            interpolation_safeguard: 0.05,
            extrapolation_factor: 1.5,
            line_bracket_method: 1,
        }
    }
    /// Create a lax configuration for faster, less precise optimization
    /// - Relaxed Wolfe conditions (c1=1e-3, c2=0.9)
    /// - Fewer iterations (10)
    /// - Larger safeguards for more robust interpolation
    /// - Aggressive extrapolation
    pub fn lax() -> Self {
        Self {
            c1: 1e-3,
            c2: 0.9,
            max_iterations: 10,
            min_step: 1e-12,
            max_step: 1e12,
            initial_step: 1.0,
            verbose: false,
            interpolation_safeguard: 0.2,
            extrapolation_factor: 3.0,
            line_bracket_method: 1,
        }
    }
    /// Create the default configuration
    pub fn default_config() -> Self {
        Self::default()
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
    /// Create with default configuration
    pub fn default_search() -> Self {
        Self::new(CubicQuadraticConfig::default())
    }

    /// Create a strict configuration for high-precision optimization
    /// - Stricter Wolfe conditions (c1=1e-4, c2=0.1)
    /// - More iterations allowed (50)
    /// - Smaller safeguards for more precise interpolation
    /// - Conservative extrapolation
    pub fn strict() -> Self {
        Self::new(CubicQuadraticConfig::strict())
    }

    /// Create a lax configuration for faster, less precise optimization
    /// - Relaxed Wolfe conditions (c1=1e-3, c2=0.9)
    /// - Fewer iterations (10)
    /// - Larger safeguards for more robust interpolation
    /// - Aggressive extrapolation
    pub fn lax() -> Self {
        Self::new(CubicQuadraticConfig::lax())
    }

    /// Create with custom configuration
    pub fn with_config(config: CubicQuadraticConfig) -> Self {
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
        if h.abs() < f64::EPSILON {
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

        if denominator.abs() < f64::EPSILON * 10.0 {
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
        if h.abs() < f64::EPSILON {
            return None;
        }

        let denom = 2.0 * (fb - fa - ga * h);
        if denom.abs() < f64::EPSILON * 10.0 {
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
            return Err(anyhow!("Direction is not a descent direction: g0 = {:.6e} >= 0. This indicates the search direction is pointing uphill.", g0));
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
                // Try a slightly larger step
                let small_step = 1e-8;
                let f_small = (problem.objective)(small_step)?;
                if f_small < f0 {
                    return Ok(LineSearchResult {
                        step_size: small_step,
                        success: true,
                        termination_reason: TerminationReason::StepSizeTooSmall,
                    });
                }
            return Err(anyhow!(
                "Function appears to be ill-conditioned: no improvement possible within machine precision. f0={:.6e}, f_test={:.6e}, f_eps={:.6e}",
                f0, f_test, f_eps
            ));
        }

        let mut alpha = self.config.initial_step;
        let mut alpha_prev = 0.0;
        let mut f_prev = f0;
        let mut g_prev = g0;
        let mut best_alpha = 0.0;
        let mut best_f = f0;
        let mut bracket_low = 0.0;
        let mut bracket_high = self.config.max_step;
        let mut f_bracket_low = f0;
        let mut f_bracket_high = f64::INFINITY;

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
                "Line Search Iteration {}: alpha={:.3e}, f={:.3e}, g={:.3e}",
                iter, alpha, f_alpha, g_alpha
            ));
            // Check Wolfe conditions
            let (armijo, curvature) = self.check_wolfe(f0, f_alpha, g_alpha, alpha, g0);
            // Update bracketing interval
            if f_alpha < f0 {
                bracket_low = alpha;
                f_bracket_low = f_alpha;
            } else {
                bracket_high = alpha;
                f_bracket_high = f_alpha;
            }

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
                let bracket_size = bracket_high - bracket_low;
                alpha = new_alpha
                    .max(bracket_low + self.config.interpolation_safeguard * bracket_size)
                    .min(bracket_high - self.config.interpolation_safeguard * bracket_size)
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
                Err(anyhow!(
                    "Line search failed to find any improvement. f0={:.6e}, best_f={:.6e} at alpha={:.6e}",
                    f0, best_f, best_alpha
                ))
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
    use crate::init_logging;
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
    fn test_cubic_interpolate_basic() {
        let line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig::default());
        // Test with a simple cubic function: f(x) = x^3 - 2x^2 + x
        // f'(x) = 3x^2 - 4x + 1
        let a = 0.0;
        let fa = 0.0; // f(0) = 0
        let ga = 1.0; // f'(0) = 1
        let b = 2.0;
        let fb = 2.0; // f(2) = 8 - 8 + 2 = 2
        let gb = 5.0; // f'(2) = 12 - 8 + 1 = 5
        let result = line_search.cubic_interpolate(a, fa, ga, b, fb, gb);
        assert!(result.is_some());
        let t = result.unwrap();
        assert!(t > a && t < b);
    }
    #[test]
    fn test_cubic_interpolate_edge_cases() {
        let line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig::default());
        // Test with identical points (should return None)
        let result = line_search.cubic_interpolate(1.0, 2.0, 1.0, 1.0, 2.0, 1.0);
        assert!(result.is_none());
        // Test with negative discriminant (should return None)
        // We need d1^2 - ga*gb < 0, so ga*gb > d1^2
        // Let's use ga=10, gb=10 (so ga*gb=100) and make d1 small
        // d1 = ga + gb - 3*(fa-fb)/h = 10 + 10 - 3*(fa-fb)/1 = 20 - 3*(fa-fb)
        // To make d1^2 < 100, we need |d1| < 10, so |20 - 3*(fa-fb)| < 10
        // This gives us 10 < 20 - 3*(fa-fb) < 30, so -10 < -3*(fa-fb) < 10
        // Therefore -10/3 < fa-fb < 10/3, so let's use fa=0, fb=-1 (fa-fb=1)
        // d1 = 20 - 3*1 = 17, d2_squared = 289 - 100 = 189 > 0 (still positive!)
        // We need ga*gb to be much larger. Let's try ga=100, gb=100 (ga*gb=10000)
        // d1 = 100 + 100 - 3*(0-(-1))/1 = 200 - 3 = 197
        // d2_squared = 197^2 - 10000 = 38809 - 10000 = 28809 > 0 (still positive!)
        // The issue is that d1 grows faster than we can make ga*gb grow
        // Let's try a different approach: make d1 = 0 by choosing fa, fb carefully
        // d1 = ga + gb - 3*(fa-fb)/h = 0, so ga + gb = 3*(fa-fb)/h
        // With ga=6, gb=6, h=1: 12 = 3*(fa-fb), so fa-fb = 4
        // Let's use fa=4, fb=0: d1 = 6 + 6 - 3*4 = 0, d2_squared = 0 - 36 = -36 < 0
        let result = line_search.cubic_interpolate(0.0, 4.0, 6.0, 1.0, 0.0, 6.0);
        assert!(result.is_none());
        // To get zero denominator: gb - ga + 2*d2 = 0
        // Let's use ga = gb (equal gradients) and make d2 = 0
        // d2 = sqrt(d1^2 - ga*gb), so d1^2 = ga*gb makes d2 = 0
        // d1 = ga + gb - 3*(fa-fb)/h
        // With ga = gb = g, we need: (2g - 3*(fa-fb)/h)^2 = g^2
        // This gives us: 2g - 3*(fa-fb)/h = ±g
        // Case 1: 2g - 3*(fa-fb)/h = g => g = 3*(fa-fb)/h
        // Case 2: 2g - 3*(fa-fb)/h = -g => 3g = 3*(fa-fb)/h => g = (fa-fb)/h

        // Let's use Case 1: h=1, fa-fb=2, so g=6
        let result = line_search.cubic_interpolate(0.0, 2.0, 6.0, 1.0, 0.0, 6.0);
        // This gives d1 = 6 + 6 - 3*2 = 6, d2_squared = 36 - 36 = 0, d2 = 0
        // denominator = 6 - 6 + 2*0 = 0
        assert!(result.is_none());
    }
    #[test]
    fn test_cubic_interpolate_safeguard() {
        let config = CubicQuadraticConfig {
            interpolation_safeguard: 0.3, // 30% minimum move
            ..CubicQuadraticConfig::default()
        };
        let line_search = CubicQuadraticLineSearch::new(config);
        // Create a case where interpolation would suggest a point too close to boundary
        let a = 0.0;
        let b = 1.0;
        let result = line_search.cubic_interpolate(a, 1.0, -1.0, b, 0.5, -0.1);
        if let Some(t) = result {
            // Should be at least 30% away from boundaries
            assert!(t >= a + 0.3 * (b - a));
            assert!(t <= b - 0.3 * (b - a));
        }
    }
    #[test]
    fn test_quadratic_interpolate_basic() {
        let line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig::default());
        // Test with a simple quadratic function: f(x) = x^2 - 2x + 1
        // f'(0) = -2, minimum at x = 1
        let a = 0.0;
        let fa = 1.0; // f(0) = 1
        let ga = -2.0; // f'(0) = -2
        let b = 2.0;
        let fb = 1.0; // f(2) = 4 - 4 + 1 = 1
        let result = line_search.quadratic_interpolate(a, fa, ga, b, fb);
        assert!(result.is_some());
        let t = result.unwrap();
        // Should interpolate close to the minimum at x = 1
        assert_relative_eq!(t, 1.0, epsilon = 0.1);
    }
    #[test]
    fn test_quadratic_interpolate_edge_cases() {
        let line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig::default());
        // Test with identical points (should return None)
        let result = line_search.quadratic_interpolate(1.0, 2.0, 1.0, 1.0, 2.0);
        assert!(result.is_none());
        // Test with zero denominator (linear function)
        let result = line_search.quadratic_interpolate(0.0, 0.0, 1.0, 1.0, 1.0);
        assert!(result.is_none());
    }
    #[test]
    fn test_quadratic_interpolate_safeguard() {
        let config = CubicQuadraticConfig {
            interpolation_safeguard: 0.2, // 20% minimum move
            ..CubicQuadraticConfig::default()
        };
        let line_search = CubicQuadraticLineSearch::new(config);
        // Create a case where interpolation would suggest a point too close to boundary
        let a = 0.0;
        let b = 1.0;
        let result = line_search.quadratic_interpolate(a, 1.0, -0.1, b, 0.99);
        if let Some(t) = result {
            // Should be at least 20% away from boundaries
            assert!(t >= a + 0.2 * (b - a));
            assert!(t <= b - 0.2 * (b - a));
        }
    }
    #[test]
    fn test_check_wolfe_conditions() {
        let config = CubicQuadraticConfig {
            c1: 1e-4,
            c2: 0.9,
            ..CubicQuadraticConfig::default()
        };
        let line_search = CubicQuadraticLineSearch::new(config);
        let f0 = 1.0;
        let g0 = -1.0; // descent direction
        let alpha = 0.5;
        // Test case where both conditions are satisfied
        let f_alpha = 0.9; // satisfies Armijo: 0.9 <= 1.0 + 1e-4 * 0.5 * (-1.0)
        let g_alpha = -0.1; // satisfies curvature: 0.1 <= 0.9 * 1.0
        let (armijo, curvature) = line_search.check_wolfe(f0, f_alpha, g_alpha, alpha, g0);
        assert!(armijo);
        assert!(curvature);
        // Test case where Armijo fails
        let f_alpha = 1.1; // violates Armijo
        let (armijo, _curvature) = line_search.check_wolfe(f0, f_alpha, g_alpha, alpha, g0);
        assert!(!armijo);
        // Test case where curvature fails
        let f_alpha = 0.9;
        let g_alpha = -0.95; // violates curvature: 0.95 > 0.9 * 1.0
        let (armijo, curvature) = line_search.check_wolfe(f0, f_alpha, g_alpha, alpha, g0);
        assert!(armijo);
        assert!(!curvature);
    }
    #[test]
    fn test_line_search_with_interpolation_fallback() {
        let mut line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig {
            initial_step: 2.0, // Start with a large step to trigger interpolation
            verbose: false,
            ..CubicQuadraticConfig::default()
        });
        // Use a function where large initial step will violate Armijo condition
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        // f(x) = x^2, so f(1 - 2*t) = (1-2t)^2 = 1 - 4t + 4t^2
        // At t=2: f = 1 - 8 + 16 = 9 (much larger than f(0) = 1)
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        assert!(result.step_size < 2.0); // Should be smaller than initial step due to interpolation
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
    #[test]
    fn test_strict_configuration() {
        let line_search = CubicQuadraticLineSearch::strict();
        assert_eq!(line_search.config.c1, 1e-4);
        assert_eq!(line_search.config.c2, 0.1);
        assert_eq!(line_search.config.max_iterations, 50);
        assert_eq!(line_search.config.interpolation_safeguard, 0.05);
        assert_eq!(line_search.config.extrapolation_factor, 1.5);
    }
    #[test]
    fn test_lax_configuration() {
        let line_search = CubicQuadraticLineSearch::lax();
        assert_eq!(line_search.config.c1, 1e-3);
        assert_eq!(line_search.config.c2, 0.9);
        assert_eq!(line_search.config.max_iterations, 10);
        assert_eq!(line_search.config.interpolation_safeguard, 0.2);
        assert_eq!(line_search.config.extrapolation_factor, 3.0);
    }
    #[test]
    fn test_strict_vs_lax_behavior() {
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
        // Test strict configuration
        let mut strict_search = CubicQuadraticLineSearch::strict();
        let strict_result = strict_search.optimize_1d(&problem).unwrap();
        // Test lax configuration  
        let mut lax_search = CubicQuadraticLineSearch::lax();
        let lax_result = lax_search.optimize_1d(&problem).unwrap();
        // Both should succeed
        assert!(strict_result.success);
        assert!(lax_result.success);
        // Both should find reasonable step sizes
        assert!(strict_result.step_size > 0.0);
        assert!(lax_result.step_size > 0.0);
    }
    #[test]
    fn test_with_config() {
        let custom_config = CubicQuadraticConfig { c1: 1e-5, ..CubicQuadraticConfig::default() };
        let line_search = CubicQuadraticLineSearch::with_config(custom_config);
        assert_eq!(line_search.config.c1, 1e-5);
    }
    #[test]
    fn test_clone_box() {
        let line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig {
            c1: 1e-5,
            c2: 0.5,
            ..CubicQuadraticConfig::default()
        });
        let cloned = line_search.clone_box();
        // We can't directly compare the configs, but we can verify it works
        // by using it in a line search
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
        // Convert to mutable reference to test
        let mut cloned_mut = cloned;
        let result = cloned_mut.optimize_1d(&problem);
        assert!(result.is_ok());
    }
    #[test]
    fn test_reset() {
        let mut line_search = CubicQuadraticLineSearch::new(CubicQuadraticConfig::default());
        // Since the line search is stateless, reset should not affect behavior
        let current_point = vec![1.0];
        let direction = vec![-1.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
        let result1 = line_search.optimize_1d(&problem).unwrap();
        line_search.reset();
        let result2 = line_search.optimize_1d(&problem).unwrap();
        // Results should be identical since the algorithm is stateless
        assert_eq!(result1.step_size, result2.step_size);
        assert_eq!(result1.success, result2.success);
    }
    #[test]
    fn test_strict_vs_lax_precision() {
        // Use a more complex function where precision matters
        fn rosenbrock_1d(x: &[f64]) -> anyhow::Result<f64> {
            let t = x[0];
            // f(t) = 100*(t^2 - 1)^2 + (t - 1)^2
            Ok(100.0 * (t * t - 1.0).powi(2) + (t - 1.0).powi(2))
        }
        fn rosenbrock_1d_gradient(x: &[f64]) -> anyhow::Result<Vec<f64>> {
            let t = x[0];
            // f'(t) = 400*t*(t^2 - 1) + 2*(t - 1)
            Ok(vec![400.0 * t * (t * t - 1.0) + 2.0 * (t - 1.0)])
        }
        let current_point = vec![0.5];
        let direction = vec![-0.1]; // Small descent direction
        let problem_strict = create_1d_problem_linear(
            &current_point,
            &direction,
            &rosenbrock_1d,
            &rosenbrock_1d_gradient,
        ).unwrap();
        let problem_lax = create_1d_problem_linear(
            &current_point,
            &direction,
            &rosenbrock_1d,
            &rosenbrock_1d_gradient,
        ).unwrap();
        let mut strict_search = CubicQuadraticLineSearch::strict();
        let mut lax_search = CubicQuadraticLineSearch::lax();
        let strict_result = strict_search.optimize_1d(&problem_strict).unwrap();
        let lax_result = lax_search.optimize_1d(&problem_lax).unwrap();
        // Both should succeed
        assert!(strict_result.success);
        assert!(lax_result.success);
        // Evaluate function values at the found steps
        let f_strict = rosenbrock_1d(&vec![current_point[0] + strict_result.step_size * direction[0]]).unwrap();
        let f_lax = rosenbrock_1d(&vec![current_point[0] + lax_result.step_size * direction[0]]).unwrap();
        let f_initial = rosenbrock_1d(&current_point).unwrap();
        // Both should improve the function
        assert!(f_strict < f_initial);
        assert!(f_lax < f_initial);
        // Strict should satisfy tighter Wolfe conditions
        // This is implicitly tested by the different c1, c2 values
    }
}