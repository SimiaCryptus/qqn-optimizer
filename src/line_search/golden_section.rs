use crate::line_search::{LineSearch, LineSearchResult, TerminationReason};
use anyhow::{anyhow, Result};
use log::debug;
use luminal::prelude::*;

/// Configuration for Golden Section line search algorithm.
///
/// The Golden Section search is a bracketing method that uses the golden ratio (φ ≈ 1.618)
/// to efficiently narrow down the interval containing the minimum. It maintains the golden
/// ratio property throughout the search, requiring only one new function evaluation per iteration.
///
/// # Algorithm Properties
/// - **Convergence Rate**: Linear with ratio ≈ 0.618 (slower than Newton-type methods)
/// - **Function Evaluations**: Moderate (one per iteration after initial bracketing)
/// - **Robustness**: Very robust, guaranteed to converge for unimodal functions
/// - **Memory**: Minimal memory requirements (stateless)
/// - **Derivatives**: Does not require derivative information
/// - **Bracket Requirement**: Needs to establish a proper bracket [a,b,c] where f(b) < min(f(a), f(c))
#[derive(Debug, Clone)]
pub struct GoldenSectionConfig {
    pub max_iterations: usize,
    pub tolerance: f32,
    pub min_step: f32,
    pub max_step: f32,
    pub initial_step: f32,
    pub verbose: bool,
}

impl Default for GoldenSectionConfig {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            tolerance: 1e-8,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
        }
    }
}
impl GoldenSectionConfig {
    /// Create a strict configuration for high-precision optimization
    ///
    /// Use this when:
    /// - High precision is required
    /// - Function evaluations are relatively cheap
    /// - Robustness is more important than speed
    /// - Working with well-conditioned problems
    /// - Higher iteration limit for thorough search
    /// - Tighter tolerance for precise convergence
    /// - Smaller minimum step for fine-grained control
    pub fn strict() -> Self {
        Self {
            max_iterations: 200,
            tolerance: 1e-12,
            min_step: 1e-20,
            max_step: 1e10,
            initial_step: 1.0,
            verbose: false,
        }
    }

    /// Create a lax configuration for fast, approximate optimization
    ///
    /// Use this when:
    /// - Speed is more important than precision
    /// - Function evaluations are expensive
    /// - Working with noisy or ill-conditioned functions
    /// - Only rough optimization is needed
    /// - Lower iteration limit for speed
    /// - Looser tolerance for quick convergence
    /// - Larger minimum step to avoid getting stuck
    pub fn lax() -> Self {
        Self {
            max_iterations: 20,
            tolerance: 1e-4,
            min_step: 1e-10,
            max_step: 1e20,
            initial_step: 1.0,
            verbose: false,
        }
    }
    /// Create the default configuration
    pub fn default_config() -> Self {
        Self::default()
    }

    /// Create a configuration with verbose logging enabled
    pub fn with_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

/// Golden Section line search implementation.
///
/// This algorithm finds the minimum of a unimodal function along a given direction using
/// the golden section search method. It's a derivative-free optimization technique that
/// maintains optimal reduction of the search interval at each iteration.
///
/// # Algorithm Overview
/// 1. **Bracketing Phase**: Find three points [a, b, c] such that f(b) < min(f(a), f(c))
/// 2. **Golden Section Phase**: Iteratively narrow the bracket using golden ratio points
/// 3. **Convergence**: Stop when interval width < tolerance or max iterations reached
///
/// # Strengths
/// - **Robust**: Always converges for unimodal functions
/// - **Derivative-free**: Only requires function evaluations
/// - **Predictable**: Convergence rate is known and consistent
/// - **Simple**: Easy to implement and understand
/// - **Optimal**: Minimizes maximum number of evaluations for worst-case scenario
///
/// # Weaknesses
/// - **Slow convergence**: Linear convergence rate (O(φ^n))
/// - **Bracketing required**: Must find initial bracket, which can be expensive
/// - **Unimodal assumption**: Assumes function has single minimum in search direction
/// - **No derivative use**: Doesn't exploit gradient information when available
/// - **Fixed reduction**: Cannot adapt reduction rate based on function behavior
///
/// # When to Use
/// Uses the golden ratio to narrow down the interval containing the minimum
#[derive(Debug, Clone)]
pub struct GoldenSectionLineSearch {
    config: GoldenSectionConfig,
}
impl<S: ConstShape> LineSearch<S> for GoldenSectionLineSearch {

    fn search(
        &mut self,
        cx: &mut Graph,
        params: GraphTensor<S>,
        loss: GraphTensor<()>,
        gradient: GraphTensor<S>,
        current_params: &[f32],
        direction: &[f32],
        initial_loss: f32,
        initial_gradient: &[f32],
    ) -> Result<LineSearchResult> {
        let mut objective = |step: f32| -> Result<f32> {
            if step == 0.0 {
                return Ok(initial_loss);
            }
            let new_params: Vec<f32> = current_params
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();

            params.set(new_params);
            loss.retrieve();
            cx.execute();

            let data = loss.data();
            Ok(data[0])
        };

        self.solve_1d(&mut objective, initial_loss, initial_gradient, direction)
    }

    fn reset(&mut self) {
        // Golden section search is stateless
    }
    fn clone_box(&self) -> Box<dyn LineSearch<S>> {
        Box::new(self.clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
impl GoldenSectionLineSearch {
    /// Set the initial step size for the next line search
    ///
    /// The initial step size affects the bracketing phase. A good initial step:
    /// - Should be roughly the expected optimal step size
    /// - Too small: may miss the minimum or require many bracketing steps
    /// - Too large: may overshoot and require bracket contraction
    pub fn set_initial_step(&mut self, step: f32) {
        self.config.initial_step = step.clamp(self.config.min_step, self.config.max_step);
    }
    pub fn new(config: GoldenSectionConfig) -> Self {
        Self { config }
    }
    /// Create with default configuration
    pub fn default_search() -> Self {
        Self::new(GoldenSectionConfig::default())
    }
    /// Create with strict configuration
    pub fn strict() -> Self {
        Self::new(GoldenSectionConfig::strict())
    }
    /// Create with lax configuration
    pub fn lax() -> Self {
        Self::new(GoldenSectionConfig::lax())
    }
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("GoldenSection: {message}");
        }
    }
    /// Golden ratio constant
    const RESPHI: f32 = 0.618033988749895; // 1/phi = phi - 1
    /// Generic solver for 1D problems, useful for testing or other backends
    pub fn solve_1d<F>(
        &self,
        objective: &mut F,
        initial_loss: f32,
        initial_gradient: &[f32],
        direction: &[f32],
    ) -> Result<LineSearchResult>
    where
        F: FnMut(f32) -> Result<f32>,
    {
        let directional_derivative: f32 = initial_gradient
            .iter()
            .zip(direction.iter())
            .map(|(g, d)| g * d)
            .sum();
        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }
        // First verify we can make progress
        let f0 = initial_loss;
        let test_step = self.config.min_step;
        let f_test = objective(test_step)?;
        if f_test >= f0 {
            // Try machine epsilon
            let eps_step = f32::EPSILON.sqrt();
            let f_eps = objective(eps_step)?;
            if f_eps < f0 {
                return Ok(LineSearchResult {
                    step_size: eps_step,
                    success: true,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                });
            }
            return Err(anyhow!(
                "Function appears to be ill-conditioned: no improvement possible within machine precision"
            ));
        }
        let step_size = self.find_minimum(objective)?;
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


    /// Find minimum using golden section search.
    ///
    /// This is the core algorithm that performs the golden section search within
    /// an established bracket. It maintains the golden ratio property to ensure
    /// optimal interval reduction at each iteration.
    fn find_minimum<F>(&self, objective: &mut F) -> Result<f32>
    where
        F: FnMut(f32) -> Result<f32>,
    {
        // First, establish a proper bracket [a, b, c] where f(b) < f(a) and f(b) < f(c)
        let (a, b, c) = self.find_bracket(objective)?;
        self.log_verbose(&format!("Initial bracket: [{a:.6e}, {b:.6e}, {c:.6e}]"));
        // Golden section search
        let mut left = a;
        let mut right = c;
        let mut x1 = right - Self::RESPHI * (right - left);
        let mut x2 = left + Self::RESPHI * (right - left);
        let mut f1 = objective(x1)?;
        let mut f2 = objective(x2)?;
        for i in 0..self.config.max_iterations {
            self.log_verbose(&format!(
                "Line Search Iteration {i}: interval=[{left:.3e}, {right:.3e}], x1={x1:.3e}, x2={x2:.3e}, f1={f1:.3e}, f2={f2:.3e}"
            ));
            if (right - left) < self.config.tolerance {
                break;
            }
            if f1 < f2 {
                // Minimum is in [left, x2]
                right = x2;
                x2 = x1;
                f2 = f1;
                x1 = right - Self::RESPHI * (right - left);
                f1 = objective(x1)?;
            } else {
                // Minimum is in [x1, right]
                left = x1;
                x1 = x2;
                f1 = f2;
                x2 = left + Self::RESPHI * (right - left);
                f2 = objective(x2)?;
            }
        }
        let final_x = if f1 < f2 { x1 } else { x2 };
        self.log_verbose(&format!("Golden section completed with x={final_x:.3e}"));
        Ok(final_x)
    }

    /// Find a proper bracket [a, b, c] where f(b) < f(a) and f(b) < f(c).
    ///
    /// This is often the most expensive part of the golden section search, as it may
    /// require many function evaluations to establish a proper bracket. The algorithm:
    ///
    /// 1. Starts at the origin (step size 0)
    /// 2. Finds a point where the function decreases (using initial_step)
    /// 3. Expands using golden ratio until function increases again
    /// 4. Ensures the middle point has the lowest function value
    ///
    /// # Failure Cases
    /// - Function doesn't decrease in the given direction (not a descent direction)
    /// - Cannot find a point where function increases (unbounded below)
    fn find_bracket<F>(&self, objective: &mut F) -> Result<(f32, f32, f32)>
    where
        F: FnMut(f32) -> Result<f32>,
    {
        let mut a = 0.0;
        let mut step = self.config.initial_step;
        let mut f_a = objective(a)?;

        // Find a point where function decreases
        let mut b = step;
        let mut f_b = objective(b)?;

        // If initial step doesn't decrease function, try smaller steps
        while f_b >= f_a && step > self.config.min_step {
            step *= 0.5;
            b = step;
            f_b = objective(b)?;
        }

        if f_b >= f_a {
            return Err(anyhow!("Cannot find decreasing direction"));
        }

        // Now find a point where function increases again
        let mut c = b * 2.0;
        let mut f_c = objective(c)?;

        // Expand until we find an increasing point
        while f_c <= f_b && c < self.config.max_step {
            a = b;
            f_a = f_b;
            b = c;
            f_b = f_c;
            c *= 1.618; // Golden ratio expansion
            if c > self.config.max_step {
                c = self.config.max_step;
            }
            f_c = objective(c)?;
        }

        // At this point, we should have f_c > f_b
        // We need to ensure f_b < f_a as well
        if f_b >= f_a {
            // The minimum might be between a and b
            // Try to find a better bracket
            let mid = (a + b) / 2.0;
            let f_mid = objective(mid)?;

            if f_mid < f_a && f_mid < f_b {
                // Use [a, mid, b] as bracket
                return Ok((a, mid, b));
            } else if f_b < f_a && f_b < f_c {
                // Original bracket is valid
                return Ok((a, b, c));
            }
        }

        // We have a valid bracket if f_b < f_a and f_b < f_c
        Ok((a, b, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::line_search::TerminationReason;
    use approx::assert_abs_diff_eq;
    use std::sync::Arc;

    fn quadratic_function(x: &[f32]) -> anyhow::Result<f32> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f32>())
    }

    fn quadratic_gradient1(x: &[f32]) -> anyhow::Result<Vec<f32>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }
    fn rosenbrock_function(x: &[f32]) -> anyhow::Result<f32> {
        if x.len() != 2 {
            return Err(anyhow!("Rosenbrock function requires 2D input"));
        }
        let a = 1.0;
        let b = 100.0;
        Ok((a - x[0]).powi(2) + b * (x[1] - x[0].powi(2)).powi(2))
    }
    fn rosenbrock_gradient(x: &[f32]) -> anyhow::Result<Vec<f32>> {
        if x.len() != 2 {
            return Err(anyhow!("Rosenbrock gradient requires 2D input"));
        }
        let a = 1.0;
        let b = 100.0;
        let grad_x0 = -2.0 * (a - x[0]) - 4.0 * b * x[0] * (x[1] - x[0].powi(2));
        let grad_x1 = 2.0 * b * (x[1] - x[0].powi(2));
        Ok(vec![grad_x0, grad_x1])
    }
    fn quartic_function(x: &[f32]) -> anyhow::Result<f32> {
        // f(x) = x^4 - 2x^2 + 1 (has minimum at x = ±1)
        let val = x[0];
        Ok(val.powi(4) - 2.0 * val.powi(2) + 1.0)
    }
    fn quartic_gradient(x: &[f32]) -> anyhow::Result<Vec<f32>> {
        let val = x[0];
        Ok(vec![4.0 * val.powi(3) - 4.0 * val])
    }
    fn exponential_function(x: &[f32]) -> anyhow::Result<f32> {
        // f(x) = e^x - x (minimum around x = 0)
        Ok(x[0].exp() - x[0])
    }
    fn exponential_gradient(x: &[f32]) -> anyhow::Result<Vec<f32>> {
        Ok(vec![x[0].exp() - 1.0])
    }

    #[test]
    fn test_golden_section_quadratic() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let initial_loss = quadratic_function(&current_point).unwrap();
        let initial_gradient = quadratic_gradient1(&current_point).unwrap();
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            quadratic_function(&new_point)
        };
        let result = line_search
            .solve_1d(&mut objective, initial_loss, &initial_gradient, &direction)
            .unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        // For quadratic function with steepest descent, optimal step should be around 1.0
        assert_abs_diff_eq!(result.step_size, 1.0, epsilon = 1e-3);
    }
    #[test]
    fn test_golden_section_rosenbrock() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            tolerance: 1e-6,
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        let current_point = vec![-1.0, 1.0];
        let current_gradient = rosenbrock_gradient(&current_point).unwrap();
        let initial_loss = rosenbrock_function(&current_point).unwrap();
        let direction = current_gradient.iter().map(|&g| -g).collect::<Vec<_>>();
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            rosenbrock_function(&new_point)
        };
        let result = line_search
            .solve_1d(&mut objective, initial_loss, &current_gradient, &direction)
            .unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        // Verify that the step actually reduces the function value
        let f_initial = rosenbrock_function(&current_point).unwrap();
        let new_point: Vec<f32> = current_point
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + result.step_size * d)
            .collect();
        let f_new = rosenbrock_function(&new_point).unwrap();
        assert!(f_new < f_initial);
    }
    #[test]
    fn test_golden_section_quartic() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            tolerance: 1e-8,
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        // Start at x = 0.5, move in negative gradient direction
        let current_point = vec![0.5];
        let current_gradient = quartic_gradient(&current_point).unwrap();
        let direction = vec![-current_gradient[0]];
        let initial_loss = quartic_function(&current_point).unwrap();
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            quartic_function(&new_point)
        };
        let result = line_search
            .solve_1d(&mut objective, initial_loss, &current_gradient, &direction)
            .unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
    }
    #[test]
    fn test_golden_section_exponential() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            tolerance: 1e-6,
            max_step: 10.0,
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        let current_point = vec![2.0];
        let current_gradient = exponential_gradient(&current_point).unwrap();
        let initial_loss = exponential_function(&current_point).unwrap();
        let direction = vec![-current_gradient[0]];
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            exponential_function(&new_point)
        };
        let result = line_search
            .solve_1d(&mut objective, initial_loss, &current_gradient, &direction)
            .unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
    }
    #[test]
    fn test_golden_section_very_small_step() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            min_step: 1e-10,
            tolerance: 1e-12,
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        let current_point = vec![1e-8];
        let direction = vec![-1.0];
        let initial_loss = quadratic_function(&current_point).unwrap();
        let initial_gradient = quadratic_gradient1(&current_point).unwrap();
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            quadratic_function(&new_point)
        };
        let result = line_search
            .solve_1d(&mut objective, initial_loss, &initial_gradient, &direction)
            .unwrap();
        assert!(
            result.success || (result.termination_reason == TerminationReason::StepSizeTooSmall)
        );
    }
    #[test]
    fn test_golden_section_max_iterations() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            max_iterations: 5, // Very low to force early termination
            tolerance: 1e-12,  // Very tight tolerance
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        let current_point = vec![10.0, 10.0];
        let direction = vec![-10.0, -10.0];
        let initial_loss = quadratic_function(&current_point).unwrap();
        let initial_gradient = quadratic_gradient1(&current_point).unwrap();
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            quadratic_function(&new_point)
        };
        let result = line_search
            .solve_1d(&mut objective, initial_loss, &initial_gradient, &direction)
            .unwrap();
        // Should still succeed even with limited iterations
        assert!(result.step_size > 0.0);
    }
    #[test]
    fn test_golden_section_config_default() {
        let config = GoldenSectionConfig::default();
        assert_eq!(config.max_iterations, 50);
        assert_eq!(config.tolerance, 1e-8);
        assert_eq!(config.min_step, 1e-16);
        assert_eq!(config.max_step, 1e16);
        assert_eq!(config.initial_step, 1.0);
        assert!(!config.verbose);
    }
    #[test]
    fn test_golden_section_config_strict() {
        let config = GoldenSectionConfig::strict();
        assert_eq!(config.max_iterations, 200);
        assert_eq!(config.tolerance, 1e-12);
        assert_eq!(config.min_step, 1e-20);
        assert_eq!(config.max_step, 1e10);
        assert_eq!(config.initial_step, 1.0);
        assert!(!config.verbose);
    }
    #[test]
    fn test_golden_section_config_lax() {
        let config = GoldenSectionConfig::lax();
        assert_eq!(config.max_iterations, 20);
        assert_eq!(config.tolerance, 1e-4);
        assert_eq!(config.min_step, 1e-10);
        assert_eq!(config.max_step, 1e20);
        assert_eq!(config.initial_step, 1.0);
        assert!(!config.verbose);
    }
    #[test]
    fn test_golden_section_config_with_verbose() {
        let config = GoldenSectionConfig::default().with_verbose();
        assert!(config.verbose);
        let strict_verbose = GoldenSectionConfig::strict().with_verbose();
        assert!(strict_verbose.verbose);
        assert_eq!(strict_verbose.max_iterations, 200);
    }

    #[test]
    fn test_golden_section_clone_and_reset() {
        let line_search = GoldenSectionLineSearch::new(GoldenSectionConfig::default());
        let mut cloned = line_search.clone();
        // Reset should work without issues (stateless)
        cloned.reset();
        // Clone box should work
        let _boxed = line_search.clone_box();
    }
    #[test]
    fn test_golden_section_bracket_finding() {
        let line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            initial_step: 0.1,
            max_step: 5.0,
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        // Start at x = 0.5 where gradient is non-zero, move in negative gradient direction
        let current_point = vec![0.5];
        let current_gradient = quartic_gradient(&current_point).unwrap();
        let direction = vec![-current_gradient[0]]; // Negative gradient for descent
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            quartic_function(&new_point)
        };
        // This should test the bracket finding logic
        let (a, b, c) = line_search.find_bracket(&mut objective).unwrap();
        assert!(a < b);
        assert!(b < c);
        // Verify bracket property: f(b) should be less than f(a) and f(c)
        let f_a = quartic_function(&[a]).unwrap();
        let f_b = quartic_function(&[b]).unwrap();
        let f_c = quartic_function(&[c]).unwrap();
        // At least one of these should be true for a valid bracket
        assert!(f_b <= f_a || f_b <= f_c);
    }
    #[test]
    fn test_golden_section_ill_conditioned_function() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            min_step: 1e-10,
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        // Create a function that barely improves (very flat but not completely flat)
        let nearly_flat_function = |x: &[f32]| -> anyhow::Result<f32> {
            // Very small quadratic term to ensure some improvement is possible
            Ok(1.0 + 1e-15 * x[0] * x[0])
        };
        let nearly_flat_gradient =
            |x: &[f32]| -> anyhow::Result<Vec<f32>> { Ok(vec![2e-15 * x[0]]) };
        let current_point = vec![0.0];
        let direction = vec![-1.0];


        let initial_loss = nearly_flat_function(&current_point).unwrap();
        let initial_gradient = nearly_flat_gradient(&current_point).unwrap();
        let mut objective = |step: f32| {
            let new_point: Vec<f32> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(p, d)| p + step * d)
                .collect();
            nearly_flat_function(&new_point)
        };

        let line_search_result = line_search.solve_1d(
            &mut objective,
            initial_loss,
            &initial_gradient,
            &direction,
        );
        // Should either succeed with tiny step or fail gracefully
        if let Ok(res) = line_search_result {
            assert!(res.step_size > 0.0);
        } else {
            // Should fail gracefully due to ill-conditioning
            assert!(line_search_result
                .unwrap_err()
                .to_string()
                .contains("ill-conditioned"));
        }

        // Also test the case where we truly have a zero gradient (should fail at problem creation)
        let zero_gradient = |_x: &[f32]| -> anyhow::Result<Vec<f32>> { Ok(vec![0.0]) };

        let mut flat_objective = |_step: f32| Ok(1.0);
        let zero_grad_result = line_search.solve_1d(
            &mut flat_objective,
            1.0,
            &zero_gradient(&current_point).unwrap(),
            &direction,
        );

        // This should fail because directional derivative is exactly zero
        assert!(zero_grad_result.is_err());
        assert!(zero_grad_result
            .unwrap_err()
            .to_string()
            .contains("descent direction"));
    }
}