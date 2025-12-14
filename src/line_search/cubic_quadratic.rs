use crate::line_search::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::anyhow;
use log::debug;
use luminal::graph::Graph;
use luminal::graph_tensor::GraphTensor;

/// A sophisticated line search algorithm that uses cubic and quadratic interpolation
/// to efficiently find step sizes satisfying the Wolfe conditions.
///
/// # Algorithm Overview
///
/// This line search combines cubic and quadratic interpolation techniques:
/// 1. **Cubic Interpolation**: When both function values and gradients are available
///    at two points, uses cubic Hermite interpolation to find a better step size
/// 2. **Quadratic Interpolation**: When only one gradient is available, falls back
///    to quadratic interpolation using function values and one gradient
/// 3. **Safeguarding**: Ensures interpolated points don't get too close to interval
///    boundaries, maintaining numerical stability
/// 4. **Bracketing**: Maintains an interval containing acceptable step sizes
///
/// # Strengths
/// - High-order interpolation provides fast convergence for smooth functions
/// - Robust safeguarding prevents numerical issues
/// - Adaptive between cubic and quadratic interpolation
/// - Configurable for different precision/speed tradeoffs
#[derive(Debug, Clone)]
pub struct CubicQuadraticConfig {
    pub c1: f32,
    pub c2: f32,
    pub max_iterations: usize,
    pub min_step: f32,
    pub max_step: f32,
    pub initial_step: f32,
    pub verbose: bool,
    /// Minimum fraction of interval to move during interpolation.
    /// Prevents interpolated points from getting too close to interval boundaries,
    /// which could cause numerical instability. Typical values: 0.05-0.2.
    pub interpolation_safeguard: f32, // Minimum fraction of interval to move
    /// Factor for extrapolation when curvature condition fails.
    /// When the curvature condition is not satisfied (gradient magnitude too large),
    /// the step size is multiplied by this factor. Typical values: 1.5-3.0.
    pub extrapolation_factor: f32, // Factor for extrapolation steps
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
        }
    }
}
/// Configuration presets for different optimization scenarios
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
        }
    }
    /// Create the default configuration
    ///
    /// Provides a balanced setup suitable for most optimization problems:
    /// - Standard Wolfe conditions (c1=1e-4, c2=0.1)
    /// - Moderate iteration limit (20)
    /// - Conservative safeguards for stability
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

/// Cubic/Quadratic interpolation line search implementation
///
/// # Weaknesses
/// - Requires gradient evaluations, making it more expensive per iteration
/// - May struggle with non-smooth or discontinuous functions
/// - Cubic interpolation can be sensitive to numerical errors in gradients
/// - Performance depends heavily on the quality of gradient information
///
/// # Best Use Cases
/// - Smooth, well-conditioned optimization problems
/// - When gradient information is reliable and relatively cheap to compute
/// - Problems where high-precision line search is beneficial
impl CubicQuadraticLineSearch {
    /// Set the initial step size for the next line search
    ///
    /// The step size will be clamped to the configured min/max bounds.
    /// Set the initial step size for the next line search
    pub fn set_initial_step(&mut self, step: f32) {
        self.config.initial_step = step.clamp(self.config.min_step, self.config.max_step);
    }
    pub fn new(config: CubicQuadraticConfig) -> Self {
        Self { config }
    }
    /// Create with default configuration
    /// Equivalent to `CubicQuadraticLineSearch::new(CubicQuadraticConfig::default())`
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
    /// Log a message if verbose mode is enabled
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("CubicQuadratic: {message}");
        }
    }

    /// Cubic interpolation between two points with function and gradient values
    ///
    /// Uses cubic Hermite interpolation to find a point that minimizes a cubic polynomial
    /// fitted through two points with known function values and gradients.
    ///
    /// # Arguments
    /// * `a`, `b` - The two x-coordinates
    /// * `fa`, `fb` - Function values at a and b
    /// * `ga`, `gb` - Gradient values at a and b
    ///
    /// # Returns
    /// * `Some(t)` - Interpolated point between a and b (with safeguarding)
    /// * `None` - If interpolation fails (e.g., identical points, negative discriminant)
    ///
    /// The returned point is safeguarded to be at least `interpolation_safeguard`
    /// fraction away from both interval endpoints.
    fn cubic_interpolate(&self, a: f32, fa: f32, ga: f32, b: f32, fb: f32, gb: f32) -> Option<f32> {
        let h = b - a;
        if h.abs() < f32::EPSILON {
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

        if denominator.abs() < f32::EPSILON * 10.0 {
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
    ///
    /// Uses quadratic interpolation when only one gradient is available.
    /// Fits a quadratic polynomial through two function values and one gradient.
    ///
    /// # Arguments
    /// * `a`, `b` - The two x-coordinates  
    /// * `fa`, `fb` - Function values at a and b
    /// * `ga` - Gradient value at point a
    ///
    /// # Returns
    /// * `Some(t)` - Interpolated point (with safeguarding)
    /// * `None` - If interpolation fails (e.g., zero denominator)
    fn quadratic_interpolate(&self, a: f32, fa: f32, ga: f32, b: f32, fb: f32) -> Option<f32> {
        let h = b - a;
        if h.abs() < f32::EPSILON {
            return None;
        }

        let denom = 2.0 * (fb - fa - ga * h);
        if denom.abs() < f32::EPSILON * 10.0 {
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
    ///
    /// Evaluates both the Armijo (sufficient decrease) and curvature conditions.
    ///
    /// # Returns
    /// * `(armijo, curvature)` - Tuple of booleans indicating which conditions are satisfied
    ///
    /// - Armijo: f(α) ≤ f(0) + c₁·α·f'(0)
    /// - Curvature: |f'(α)| ≤ c₂·|f'(0)|
    fn check_wolfe(
        &self,
        f0: f32,
        f_alpha: f32,
        g_alpha: f32,
        alpha: f32,
        g0: f32,
    ) -> (bool, bool) {
        let armijo = f_alpha <= f0 + self.config.c1 * alpha * g0;
        let curvature = g_alpha.abs() <= self.config.c2 * g0.abs();
        (armijo, curvature)
    }
}

impl LineSearch for CubicQuadraticLineSearch {
    fn search(
        &mut self,
        cx: &mut Graph,
        params: GraphTensor,
        loss: GraphTensor,
        gradient: GraphTensor,
        current_params: &[f32],
        direction: &[f32],
        initial_loss: f32,
        initial_gradient: &[f32],
    ) -> anyhow::Result<LineSearchResult> {
        let f0 = initial_loss;
        let mut num_f_evals = 0usize;
        let mut num_g_evals = 0usize;
        let g0: f32 = initial_gradient
            .iter()
            .zip(direction.iter())
            .map(|(g, d)| g * d)
            .sum();

        if g0 >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction: g0 = {:.6e} >= 0. This indicates the search direction is pointing uphill.", g0));
        }
        // Helper to evaluate function and gradient
        let evaluate = |alpha: f32, cx: &mut Graph| -> anyhow::Result<(f32, f32)> {
            let (loss_val, grad_val) = self.evaluate_with_gradient(
                cx,
                params,
                loss,
                gradient,
                current_params,
                direction,
                alpha,
            )?;
            let dir_deriv: f32 = grad_val
                .iter()
                .zip(direction.iter())
                .map(|(g, d)| g * d)
                .sum();
            Ok((loss_val, dir_deriv))
        };

        // Verify we can make progress
        let test_step = self.config.min_step;
        let (f_test, _) = evaluate(test_step, cx)?;
        num_f_evals += 1;
        num_g_evals += 1;
        if f_test >= f0 {
            let eps_step = f32::EPSILON.sqrt();
            let (f_eps, _) = evaluate(eps_step, cx)?;
            num_f_evals += 1;
            num_g_evals += 1;
            if f_eps < f0 {
                return Ok(LineSearchResult {
                    step_size: eps_step,
                    success: true,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                    num_f_evals,
                    num_g_evals,
                });
            }
            // Try a slightly larger step
            let small_step = 1e-8;
            let (f_small, _) = evaluate(small_step, cx)?;
            num_f_evals += 1;
            num_g_evals += 1;
            if f_small < f0 {
                return Ok(LineSearchResult {
                    step_size: small_step,
                    success: true,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                    num_f_evals,
                    num_g_evals,
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

        self.log_verbose(&format!(
            "Starting with f(0)={f0:.3e}, g(0)={g0:.3e}, initial_step={alpha:.3e}"
        ));
        for iter in 0..self.config.max_iterations {
            // Evaluate at current step
            let (f_alpha, g_alpha) = evaluate(alpha, cx)?;
            num_f_evals += 1;
            num_g_evals += 1;
            // Track best point
            if f_alpha < best_f {
                best_f = f_alpha;
                best_alpha = alpha;
            }

            self.log_verbose(&format!(
                "Line Search Iteration {iter}: alpha={alpha:.3e}, f={f_alpha:.3e}, g={g_alpha:.3e}"
            ));
            // Check Wolfe conditions
            let (armijo, curvature) = self.check_wolfe(f0, f_alpha, g_alpha, alpha, g0);
            // Update bracketing interval
            if f_alpha < f0 {
                bracket_low = alpha;
            } else {
                bracket_high = alpha;
            }

            if armijo && curvature {
                self.log_verbose("Wolfe conditions satisfied");
                return Ok(LineSearchResult {
                    step_size: alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                    num_f_evals,
                    num_g_evals,
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

                self.log_verbose(&format!("Interpolated new alpha: {alpha:.3e}"));
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

                self.log_verbose(&format!("Extrapolated for curvature: {alpha:.3e}"));
                continue;
            }
        }

        // Return best point found if we have improvement
        if best_alpha > 0.0 && best_f < f0 {
            Ok(LineSearchResult {
                step_size: best_alpha,
                success: true,
                termination_reason: TerminationReason::MaxIterationsReached,
                num_f_evals,
                num_g_evals,
            })
        } else {
            // Try a very small step as last resort
            let small_step = self.config.min_step * 10.0;
            let (f_small, _) = evaluate(small_step, cx)?;
            num_f_evals += 1;
            num_g_evals += 1;
            if f_small < f0 {
                Ok(LineSearchResult {
                    step_size: small_step,
                    success: true,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                    num_f_evals,
                    num_g_evals,
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
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::sync::Arc;

    fn quadratic_function(x: &[f32]) -> anyhow::Result<f32> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f32>())
    }

    fn quadratic_gradient1(x: &[f32]) -> anyhow::Result<Vec<f32>> {
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
    fn test_with_config() {
        let custom_config = CubicQuadraticConfig {
            c1: 1e-5,
            ..CubicQuadraticConfig::default()
        };
        let line_search = CubicQuadraticLineSearch::with_config(custom_config);
        assert_eq!(line_search.config.c1, 1e-5);
    }
}
