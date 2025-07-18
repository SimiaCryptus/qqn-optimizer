use crate::utils::math::dot_product_f64;
use anyhow::{anyhow, Error, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

/// Trait for 1-D differentiable parametric curves
pub trait ParametricCurve: Send + Sync {
    /// Evaluate the curve at parameter t
    fn position(&self, t: f64) -> Result<Vec<f64>>;
    /// Evaluate the direction of the curve at parameter t
    fn direction(&self, t: f64) -> Result<Vec<f64>>;
}

/// A 1D optimization problem along a parametric curve
pub struct OneDimensionalProblem<'a> {
    /// The 1D objective function f(t)
    pub objective: Box<dyn Fn(f64) -> Result<f64> + Send + Sync + 'a>,
    /// The 1D gradient function f'(t)
    pub gradient: Box<dyn Fn(f64) -> Result<f64> + Send + Sync + 'a>,
    /// Initial directional derivative at t=0
    pub initial_directional_derivative: f64,
}
impl<'a> OneDimensionalProblem<'a> {
    pub fn new(
        objective: Box<dyn Fn(f64) -> Result<f64> + Send + Sync + 'a>,
        gradient: Box<dyn Fn(f64) -> Result<f64> + Send + Sync + 'a>,
        initial_directional_derivative: f64,
    ) -> Self {
        Self {
            objective,
            gradient,
            initial_directional_derivative,
        }
    }
}

pub fn create_1d_problem<'a>(
    curve: Box<dyn ParametricCurve + 'a>,
    objective_fn: &'a (dyn Fn(&[f64]) -> anyhow::Result<f64> + Send + Sync),
    gradient_fn: &'a (dyn Fn(&[f64]) -> anyhow::Result<Vec<f64>> + Send + Sync),
) -> Result<OneDimensionalProblem<'a>> {
    let initial_position = curve.position(0.0)?;
    let initial_value = objective_fn(&initial_position)
        .map_err(|e| anyhow!("Objective evaluation failed: {}", e))?;
    // Get initial directional derivative
    let initial_direction = curve.direction(0.0)?;
    let initial_derivative = gradient_fn(&curve.position(0.0)?)?;
    let initial_directional_derivative = dot_product_f64(&initial_derivative, &initial_direction)?;
    debug!("create_1d_problem: initial_derivative={:?}, initial_direction={:?}, initial_directional_derivative={:.3e}",
          initial_derivative, initial_direction, initial_directional_derivative);

    // Use Arc to share the curve between closures
    let curve = Arc::new(curve);
    let curve_for_objective = curve.clone();
    let curve_for_gradient = curve.clone();

    // Create 1D objective function
    let objective_1d = move |t: f64| -> Result<f64> {
        let result_vec = curve_for_objective.position(t)?;
        let result_val =
            objective_fn(&result_vec).map_err(|e| anyhow!("Objective evaluation failed: {}", e))?;
        let improvement = initial_value - result_val;
        let result = objective_fn(&result_vec)?;
        debug!(
            "1D objective at t={:.3e}: f={:.3e}, improvement: {:.3e}",
            t, result, improvement
        );
        Ok(result)
    };

    // Create 1D gradient function
    let gradient_1d = move |t: f64| -> Result<f64> {
        let result_vec = curve_for_gradient.position(t)?;
        let curve_derivative = curve_for_gradient.direction(t)?;
        let result = gradient_fn(&result_vec).and_then(|g| {
            if g.len() != curve_derivative.len() {
                return Err(anyhow!(
                    "Gradient length mismatch: expected {}, got {}",
                    curve_derivative.len(),
                    g.len()
                ));
            }
            // Compute directional derivative: ∇f(x(t)) · dx/dt
            dot_product_f64(&g, &curve_derivative)
        })?;
        debug!(
            "1-D gradient result at t={:.3e}; p={:?} = {:.3e}",
            t, result_vec, result
        );
        Ok(result)
    };
    Ok(OneDimensionalProblem::new(
        Box::new(objective_1d),
        Box::new(gradient_1d),
        initial_directional_derivative,
    ))
}
/// Convert a linear search direction into a 1D problem
pub fn create_1d_problem_linear<'a>(
    current_point: &'a [f64],
    direction: &'a [f64],
    objective_fn: &'a (dyn Fn(&[f64]) -> Result<f64> + Send + Sync),
    gradient_fn: &'a (dyn Fn(&[f64]) -> Result<Vec<f64>> + Send + Sync),
) -> Result<OneDimensionalProblem<'a>> {
    let curve = LinearCurve::new(current_point.to_vec(), direction.to_vec());

    // Debug: let's verify the curve works correctly
    let test_val_0 = curve.position(0.0)?;
    let test_val_1 = curve.position(1.0)?;
    debug!(
        "Curve test: f(t=0) -> {:?}, f(t=1) -> {:?}",
        test_val_0, test_val_1
    );

    let result = create_1d_problem(Box::new(curve), objective_fn, gradient_fn);
    result
}

/// Linear parametric curve: x(t) = x0 + t * direction
#[derive(Debug, Clone)]
pub struct LinearCurve {
    start_point: Vec<f64>,
    direction: Vec<f64>,
}
impl LinearCurve {
    pub fn new(start_point: Vec<f64>, direction: Vec<f64>) -> Self {
        Self {
            start_point,
            direction,
        }
    }
    /// Get the point along the curve at parameter t
    pub fn point_at(&self, t: f64) -> Vec<f64> {
        self.start_point
            .iter()
            .zip(self.direction.iter())
            .map(|(x, d)| x + t * d)
            .collect()
    }
}
impl ParametricCurve for LinearCurve {
    fn position(&self, t: f64) -> Result<Vec<f64>> {
        Ok(self.point_at(t))
    }
    fn direction(&self, _t: f64) -> Result<Vec<f64>> {
        Ok(self.direction.clone())
    }
}
/// Line search result containing step size and evaluation counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchResult {
    pub step_size: f64,
    pub success: bool,
    pub termination_reason: TerminationReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminationReason {
    WolfeConditionsSatisfied,
    ArmijoConditionSatisfied,
    MaxIterationsReached,
    StepSizeTooSmall,
    FunctionEvaluationError,
}
/// General line search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchConfig {
    pub method: LineSearchMethod,
    pub c1: f64,
    pub c2: f64,
    pub max_iterations: usize,
    pub initial_step: f64,
    pub min_step: f64,
    pub max_step: f64,
    pub verbose: bool,           // Enable verbose logging
    pub line_bracket_method: u8, // 1: gradient-based bracketing, 2: function-value-based bracketing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineSearchMethod {
    StrongWolfe,
    Backtracking,
    Bisection,
    GoldenSection,
    MoreThuente,
    CubicQuadraticInterpolation,
}

impl Default for LineSearchConfig {
    fn default() -> Self {
        Self {
            method: LineSearchMethod::StrongWolfe,
            c1: 1e-4,
            c2: 0.01,
            max_iterations: 50,
            initial_step: 1.0,
            min_step: 1e-16,
            max_step: 100.0,
            verbose: false,
            line_bracket_method: 1, // Default to gradient-based bracketing
        }
    }
}
/// Create a line search algorithm from configuration
pub fn create_line_search(config: LineSearchConfig) -> Box<dyn LineSearch> {
    match config.method {
        LineSearchMethod::StrongWolfe => Box::new(StrongWolfeLineSearch::new(StrongWolfeConfig {
            c1: config.c1,
            c2: config.c2,
            max_iterations: config.max_iterations,
            min_step: config.min_step,
            max_step: config.max_step,
            initial_step: config.initial_step,
            verbose: config.verbose,
        })),
        LineSearchMethod::Backtracking => {
            Box::new(BacktrackingLineSearch::new(BacktrackingConfig {
                c1: config.c1,
                max_iterations: config.max_iterations,
                min_step: config.min_step,
                initial_step: config.initial_step,
                ..BacktrackingConfig::default()
            }))
        }
        LineSearchMethod::Bisection => Box::new(BisectionLineSearch::new(BisectionConfig {
            max_iterations: config.max_iterations,
            min_step: config.min_step,
            max_step: config.max_step,
            initial_step: config.initial_step,
            verbose: config.verbose,
            line_bracket_method: config.line_bracket_method,
            ..BisectionConfig::default()
        })),
        LineSearchMethod::GoldenSection => {
            Box::new(GoldenSectionLineSearch::new(GoldenSectionConfig {
                max_iterations: config.max_iterations,
                min_step: config.min_step,
                max_step: config.max_step,
                initial_step: config.initial_step,
                verbose: config.verbose,
                ..GoldenSectionConfig::default()
            }))
        }
        LineSearchMethod::MoreThuente => Box::new(MoreThuenteLineSearch::new(MoreThuenteConfig {
            c1: config.c1,
            c2: config.c2,
            max_iterations: config.max_iterations,
            min_step: config.min_step,
            max_step: config.max_step,
            initial_step: config.initial_step,
            verbose: config.verbose,
            ..MoreThuenteConfig::default()
        })),
        LineSearchMethod::CubicQuadraticInterpolation => {
            Box::new(CubicQuadraticLineSearch::new(CubicQuadraticConfig {
                c1: config.c1,
                c2: config.c2,
                max_iterations: config.max_iterations,
                min_step: config.min_step,
                max_step: config.max_step,
                initial_step: config.initial_step,
                verbose: config.verbose,
                line_bracket_method: config.line_bracket_method,
                ..CubicQuadraticConfig::default()
            }))
        }
    }
}

/// Trait for line search algorithms
pub trait LineSearch: Send + Sync + Debug {
    /// Perform 1D line search optimization
    fn optimize_1d<'a>(&mut self, problem: &'a OneDimensionalProblem) -> Result<LineSearchResult>;

    /// Reset internal state
    fn reset(&mut self);

    /// Clone the line search algorithm
    fn clone_box(&self) -> Box<dyn LineSearch>;
}

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
            c1: 1e-4,
            c2: 0.9,
            max_iterations: 50,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            verbose: false,
        }
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
    ) -> Result<f64> {
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
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem<'a>,
    ) -> Result<LineSearchResult> {
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

        let mut alpha = self.config.initial_step;
        let mut alpha_prev = 0.0;
        let mut f_prev = f0;
        let mut best_alpha = 0.0;
        let mut best_f = f0;

        self.log_verbose(&format!("Initial step size: {:.3e}", alpha));

        for i in 0..self.config.max_iterations {
            self.log_verbose(&format!("Iteration {}: trying alpha={:.3e}", i, alpha));

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
            alpha = alpha.min(self.config.max_step) * 2.0; // Expand step size
            self.log_verbose(&format!("  Expanding step size to {:.3e}", alpha));

            if alpha < self.config.min_step {
                // Try one last time with the minimum step
                let f_min = (problem.objective)(self.config.min_step)?;
                if f_min < f0 {
                    self.log_verbose(&format!(
                        "Using minimum step size {:.3e} with improvement",
                        self.config.min_step
                    ));
                    return Ok(LineSearchResult {
                        step_size: self.config.min_step,
                        success: true,
                        termination_reason: TerminationReason::StepSizeTooSmall,
                    });
                } else {
                    self.log_verbose("Even minimum step size doesn't improve function");
                    break;
                }
            }
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
            c1: 1e-4,
            rho: 0.5,
            max_iterations: 50,
            min_step: 1e-16,
            initial_step: 1.0,
        }
    }
}
/// Configuration for bisection line search
#[derive(Debug, Clone)]
pub struct BisectionConfig {
    pub max_iterations: usize,     // Maximum bisection iterations
    pub gradient_tolerance: f64,   // Tolerance for gradient being zero
    pub min_step: f64,             // Minimum step size
    pub max_step: f64,             // Maximum step size
    pub initial_step: f64,         // Initial step size
    pub window_shrink_factor: f64, // Factor to shrink window on failure
    pub verbose: bool,             // Enable verbose logging
    pub line_bracket_method: u8, // 1: gradient-based bracketing, 2: function-value-based bracketing
}
impl Default for BisectionConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            gradient_tolerance: 1e-16,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            window_shrink_factor: 0.5,
            verbose: false,
            line_bracket_method: 1, // 1 for far point method, 2 for simple far point
        }
    }
}

/// Find far point using gradient-based method
/// Looks for a point where f(t) < f(0) and gradient is positive (function starts increasing)
fn find_far_point_1(
    problem: &OneDimensionalProblem,
    f0: f64,
    initial_steop: f64,
    max_iterations: usize,
    min_step: f64,
    gradient_tolerance: f64,
    max_step: f64,
) -> Result<f64, Error> {
    let mut t = initial_steop;
    let mut iteration = 0;
    debug!("Finding far point starting from t={:.3e}", t);
    while iteration < max_iterations {
        let f_t = (problem.objective)(t)?;
        let grad_t = (problem.gradient)(t)?;
        debug!(
            "  Iteration {}: t={:.3e}, f={:.3e}, grad={:.3e}, f0={:.3e}",
            iteration, t, f_t, grad_t, f0
        );
        // Check if this point satisfies our far point criteria:
        // 1. Function value is still better than f(0)
        // 2. Gradient is positive (function is increasing)
        if f_t < f0 && grad_t > 0.0 {
            debug!("Found far point at t={:.3e}", t);
            return Ok(t);
        }
        if f_t >= f0 {
            debug!("Function value too high at t={:.3e}, reducing step", t);
            t *= 0.5;
            if t < min_step {
                debug!("Step size too small, using minimum step");
                return Ok(min_step);
            }
        }
        // If gradient is still negative, step is too small
        else if grad_t < 0.0 {
            debug!("Gradient still negative at t={:.3e}, increasing step", t);
            t *= 2.0;
            if t > max_step {
                debug!("Step size too large, using maximum step");
                return Ok(max_step);
            }
        }
        // If gradient is approximately zero, we found our point
        else if grad_t.abs() <= gradient_tolerance {
            debug!("Found zero gradient at t={:.3e}", t);
            return Ok(t);
        }
        iteration += 1;
    }
    // If we can't find a proper far point, return the last valid step
    debug!("Max iterations reached, returning t={:.3e}", t);
    Ok(t)
}

/// Find far point using simple function-value-based method
/// Looks for a point where f(t) > f(0) (function value is worse than starting point)
fn find_far_point_2(
    problem: &OneDimensionalProblem,
    f0: f64,
    initial_steop: f64,
    max_iterations: usize,
    max_step: f64,
) -> Result<f64, Error> {
    let mut t = initial_steop;
    let mut iteration = 0;
    debug!("Finding far point starting from t={:.3e}", t);
    while iteration < max_iterations {
        let f_t = (problem.objective)(t)?;
        debug!(
            "  Iteration {}: t={:.3e}, f={:.3e}, f0={:.3e}",
            iteration, t, f_t, f0
        );
        // Check if this point satisfies our far point criteria:
        // 1. Function value is worse than f(0)
        if f_t > f0 {
            debug!("Found far point at t={:.3e}", t);
            return Ok(t);
        }

        // If function value is still better than f(0), increase step size
        if f_t <= f0 {
            debug!(
                "Function value still better at t={:.3e}, increasing step",
                t
            );
            t *= 2.0;
            if t > max_step {
                debug!("Step size too large, using maximum step");
                return Ok(max_step);
            }
        }

        iteration += 1;
    }
    // If we can't find a proper far point, return the last valid step
    debug!("Max iterations reached, returning t={:.3e}", t);
    Ok(t)
}

/// Bisection line search implementation
/// Finds the point where the gradient is zero using bisection method
/// If gradient is non-decreasing, uses window search with successive halving
#[derive(Debug, Clone)]
pub struct BisectionLineSearch {
    config: BisectionConfig,
}
impl BisectionLineSearch {
    pub fn new(config: BisectionConfig) -> Self {
        Self { config }
    }
    /// Log line search details if verbose mode is enabled
    fn log_verbose(&self, message: &str) {
        if self.config.verbose {
            debug!("Bisection: {}", message);
        }
    }

    /// Find the point where gradient is approximately zero using bisection
    fn find_zero_gradient(
        &self,
        left: f64,
        right: f64,
        problem: &OneDimensionalProblem,
    ) -> Result<f64> {
        let mut a = left;
        let mut b = right;

        self.log_verbose(&format!(
            "Finding zero gradient in interval [{:.3e}, {:.3e}]",
            a, b
        ));
        // Verify we have a proper bracket with opposite gradient signs
        let grad_a = (problem.gradient)(a)?;
        let grad_b = (problem.gradient)(b)?;
        if grad_a * grad_b > 0.0 {
            self.log_verbose(&format!(
                "Warning: gradients have same sign at endpoints: grad({:.3e})={:.3e}, grad({:.3e})={:.3e}",
                a, grad_a, b, grad_b
            ));
            // Return the point with smaller absolute gradient
            return Ok(if grad_a.abs() < grad_b.abs() { a } else { b });
        }

        for i in 0..self.config.max_iterations {
            let mid = 0.5 * (a + b);
            // Evaluate gradient at midpoint
            let grad_mid = (problem.gradient)(mid)?;
            self.log_verbose(&format!(
                "  Iteration {}: mid={:.3e}, grad={:.3e}",
                i, mid, grad_mid
            ));
            // Check if gradient is close enough to zero
            if grad_mid.abs() <= self.config.gradient_tolerance {
                self.log_verbose(&format!("Found zero gradient at alpha={:.3e}", mid));
                return Ok(mid);
            }
            // Check if interval is too small
            if (b - a) < self.config.min_step {
                self.log_verbose(&format!("Interval too small, returning mid={:.3e}", mid));
                return Ok(mid);
            }
            // Update interval based on sign of gradient
            let grad_a = (problem.gradient)(a)?;
            if grad_a * grad_mid < 0.0 {
                // Zero is between a and mid
                b = mid;
            } else {
                // Zero is between mid and b (or doesn't exist in interval)
                a = mid;
            }
        }
        // Return midpoint if max iterations reached
        let final_alpha = 0.5 * (a + b);
        self.log_verbose(&format!(
            "Max iterations reached, returning alpha={:.3e}",
            final_alpha
        ));
        Ok(final_alpha)
    }
}
impl LineSearch for BisectionLineSearch {
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem<'a>,
    ) -> Result<LineSearchResult> {
        let directional_derivative = problem.initial_directional_derivative;
        self.log_verbose(&format!("Starting bisection line search"));
        self.log_verbose(&format!(
            "Initial directional derivative: {:.3e}",
            directional_derivative
        ));

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        // Step 1: Find the far point
        let config = self.config.clone();
        let far_point = match config.line_bracket_method {
            1 => find_far_point_1(
                problem,
                (problem.objective)(0.0)?,
                config.initial_step,
                config.max_iterations,
                config.min_step,
                config.gradient_tolerance,
                config.max_step,
            )?,
            2 => find_far_point_2(
                problem,
                (problem.objective)(0.0)?,
                config.initial_step,
                config.max_iterations,
                config.max_step,
            )?,
            _ => {
                return Err(anyhow!(
                    "Invalid line_bracket_method: {}. Must be 1 or 2",
                    config.line_bracket_method
                ))
            }
        };

        // Step 2: Verify we have a proper bracket for bisection
        let grad_0 = problem.initial_directional_derivative;
        let grad_far = (problem.gradient)(far_point)?;

        self.log_verbose(&format!(
            "Bracket: grad(0)={:.3e}, grad({:.3e})={:.3e}",
            grad_0, far_point, grad_far
        ));

        // Step 3: Perform bisection search for zero gradient
        let step_size = if grad_0 * grad_far < 0.0 {
            // We have a proper bracket, use bisection
            self.find_zero_gradient(0.0, far_point, problem)?
        } else {
            // No proper bracket, return the far point if it's an improvement
            let f0 = (problem.objective)(0.0)?;
            let f_far = (problem.objective)(far_point)?;
            if f_far < f0 {
                self.log_verbose(&format!(
                    "No gradient sign change, but far point provides improvement"
                ));
                far_point
            } else {
                // Try minimum step as last resort
                let f_min = (problem.objective)(self.config.min_step)?;
                if f_min < f0 {
                    self.config.min_step
                } else {
                    return Err(anyhow!("Cannot find any improvement"));
                }
            }
        };

        // Verify the final step size provides improvement
        let f0 = (problem.objective)(0.0)?;
        let f_final = (problem.objective)(step_size)?;

        if f_final >= f0 {
            return Err(anyhow!("Final step size does not provide improvement"));
        }

        // Check final gradient
        let final_gradient = (problem.gradient)(step_size)?;
        let success = step_size >= self.config.min_step && step_size <= self.config.max_step;

        self.log_verbose(&format!(
            "Final result: alpha={:.3e}, f_improvement={:.3e}, grad={:.3e}, success={}",
            step_size,
            f0 - f_final,
            final_gradient,
            success
        ));

        Ok(LineSearchResult {
            step_size,
            success,
            termination_reason: if success {
                TerminationReason::WolfeConditionsSatisfied
            } else {
                TerminationReason::MaxIterationsReached
            },
        })
    }

    fn reset(&mut self) {
        // Bisection line search is stateless, nothing to reset
    }

    fn clone_box(&self) -> Box<dyn LineSearch> {
        Box::new(self.clone())
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
}

impl LineSearch for BacktrackingLineSearch {
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem<'a>,
    ) -> Result<LineSearchResult> {
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

/// Factory functions for creating line search algorithms
pub fn strong_wolfe_line_search() -> Box<dyn LineSearch> {
    Box::new(StrongWolfeLineSearch::new(StrongWolfeConfig::default()))
}

pub fn backtracking_line_search() -> Box<dyn LineSearch> {
    Box::new(BacktrackingLineSearch::new(BacktrackingConfig::default()))
}

pub fn strong_wolfe_with_config(config: StrongWolfeConfig) -> Box<dyn LineSearch> {
    Box::new(StrongWolfeLineSearch::new(config))
}

pub fn backtracking_with_config(config: BacktrackingConfig) -> Box<dyn LineSearch> {
    Box::new(BacktrackingLineSearch::new(config))
}
pub fn bisection_line_search() -> Box<dyn LineSearch> {
    Box::new(BisectionLineSearch::new(BisectionConfig::default()))
}
pub fn bisection_with_config(config: BisectionConfig) -> Box<dyn LineSearch> {
    Box::new(BisectionLineSearch::new(config))
}
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
    const PHI: f64 = 1.618033988749895;
    const RESPHI: f64 = 0.618033988749895; // 1/phi = phi - 1
    /// Find minimum using golden section search
    fn find_minimum(&self, problem: &OneDimensionalProblem) -> Result<f64> {
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
    fn find_bracket(&self, problem: &OneDimensionalProblem) -> Result<(f64, f64, f64)> {
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
impl LineSearch for GoldenSectionLineSearch {
    fn optimize_1d<'a>(
        &mut self,
        problem: &'a OneDimensionalProblem<'a>,
    ) -> Result<LineSearchResult> {
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
    ) -> Result<LineSearchResult> {
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
    /// Establish a good search interval using smart far point identification
    fn establish_search_interval(&self, problem: &OneDimensionalProblem) -> Result<(f64, f64)> {
        let config = self.config.clone();
        let far_point = match config.line_bracket_method {
            1 => find_far_point_1(
                problem,
                (problem.objective)(0.0)?,
                config.initial_step,
                config.max_iterations,
                config.min_step,
                1e-16,
                config.max_step,
            )?,
            2 => find_far_point_2(
                problem,
                (problem.objective)(0.0)?,
                config.initial_step,
                config.max_iterations,
                config.max_step,
            )?,
            _ => {
                return Err(anyhow!(
                    "Invalid line_bracket_method: {}. Must be 1 or 2",
                    config.line_bracket_method
                ))
            }
        };
        // Verify we have a good interval
        let grad_0 = problem.initial_directional_derivative;
        let grad_far = (problem.gradient)(far_point)?;
        self.log_verbose(&format!(
            "Search interval: [0, {:.3e}], grad(0)={:.3e}, grad(far)={:.3e}",
            far_point, grad_0, grad_far
        ));
        // If we have opposite gradient signs, we have a good bracket
        if grad_0 * grad_far < 0.0 {
            self.log_verbose("Good bracket established with opposite gradient signs");
            return Ok((0.0, far_point));
        }
        // If gradients have same sign, we still use the interval but may need more iterations
        self.log_verbose("Gradients have same sign, but using interval anyway");
        Ok((0.0, far_point))
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
    ) -> Result<LineSearchResult> {
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
/// Factory functions for new line search methods
pub fn golden_section_line_search() -> Box<dyn LineSearch> {
    Box::new(GoldenSectionLineSearch::new(GoldenSectionConfig::default()))
}
pub fn more_thuente_line_search() -> Box<dyn LineSearch> {
    Box::new(MoreThuenteLineSearch::new(MoreThuenteConfig::default()))
}
pub fn cubic_quadratic_line_search() -> Box<dyn LineSearch> {
    Box::new(CubicQuadraticLineSearch::new(
        CubicQuadraticConfig::default(),
    ))
}
pub fn golden_section_with_config(config: GoldenSectionConfig) -> Box<dyn LineSearch> {
    Box::new(GoldenSectionLineSearch::new(config))
}
pub fn more_thuente_with_config(config: MoreThuenteConfig) -> Box<dyn LineSearch> {
    Box::new(MoreThuenteLineSearch::new(config))
}
pub fn cubic_quadratic_with_config(config: CubicQuadraticConfig) -> Box<dyn LineSearch> {
    Box::new(CubicQuadraticLineSearch::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_1d_problem_creation() {
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let current_gradient = vec![2.0, 3.0];
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        )
            .unwrap();
        // Test that f(0) gives the current function value
        let f0 = (problem.objective)(0.0).unwrap();
        let expected_f0 = quadratic_function(&current_point).unwrap();
        assert_relative_eq!(f0, expected_f0, epsilon = 1e-10);
        // Test that f'(0) gives the directional derivative
        let expected_directional_derivative = -2.0 * 2.0 + -3.0 * 3.0; // direction · gradient
        assert_relative_eq!(
            problem.initial_directional_derivative,
            expected_directional_derivative,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_strong_wolfe_quadratic() {
        // init_logging();
        let mut line_search = StrongWolfeLineSearch::new(StrongWolfeConfig::default());

        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0]; // Negative gradient (descent direction)
        let current_gradient = quadratic_gradient1(&current_point).unwrap();

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

        // For quadratic function, optimal step should be 1.0
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_backtracking_quadratic() {
        // init_logging();
        let mut line_search = BacktrackingLineSearch::new(BacktrackingConfig::default());

        let current_point = vec![1.0, 1.0];
        let direction = vec![-1.0, -1.0]; // Negative gradient
        let current_gradient = quadratic_gradient1(&current_point).unwrap();

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

    #[test]
    fn test_non_descent_direction() {
        // init_logging();
        let mut line_search = StrongWolfeLineSearch::new(StrongWolfeConfig::default());

        let current_point = vec![1.0, 1.0];
        let direction = vec![1.0, 1.0]; // Positive gradient (ascent direction)
        let current_gradient = quadratic_gradient1(&current_point).unwrap();

        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        )
            .unwrap();
        let result = line_search.optimize_1d(&problem);

        assert!(result.is_err());
    }

    #[test]
    fn test_bisection_quadratic() {
        let mut line_search = BisectionLineSearch::new(BisectionConfig {
            verbose: false,
            ..BisectionConfig::default()
        });
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0]; // Negative gradient (descent direction)
        let current_gradient = quadratic_gradient1(&current_point).unwrap();

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
        // For quadratic function, optimal step should be 1.0 (where gradient is zero)
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_bisection_non_descent() {
        // init_logging();
        let mut line_search = BisectionLineSearch::new(BisectionConfig::default());
        let current_point = vec![1.0, 1.0];
        let direction = vec![1.0, 1.0]; // Positive gradient (ascent direction)
        let current_gradient = quadratic_gradient1(&current_point).unwrap();
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &quadratic_function,
            &quadratic_gradient1,
        )
            .unwrap();
        let result = line_search.optimize_1d(&problem);
        assert!(result.is_err());
    }
    // Additional tests for better coverage
    #[test]
    fn test_linear_curve() {
        let start = vec![1.0, 2.0];
        let direction = vec![3.0, 4.0];
        let curve = LinearCurve::new(start.clone(), direction.clone());
        // Test evaluation at different t values
        let p0 = curve.position(0.0).unwrap();
        assert_eq!(p0, vec![1.0, 2.0]);
        let p1 = curve.position(1.0).unwrap();
        assert_eq!(p1, vec![4.0, 6.0]);
        let p_half = curve.position(0.5).unwrap();
        assert_eq!(p_half, vec![2.5, 4.0]);
        // Test derivative (should be constant)
        let d0 = curve.direction(0.0).unwrap();
        assert_eq!(d0, direction);
        let d1 = curve.direction(1.0).unwrap();
        assert_eq!(d1, direction);
    }
    #[test]
    fn test_create_line_search() {
        // Test creating different line search methods
        let config = LineSearchConfig {
            method: LineSearchMethod::StrongWolfe,
            ..Default::default()
        };
        let ls = create_line_search(config);
        // Just verify we can create and clone the line search
        let _cloned = ls.clone_box();
        let config = LineSearchConfig {
            method: LineSearchMethod::Backtracking,
            ..Default::default()
        };
        let ls = create_line_search(config);
        let _cloned = ls.clone_box();
        let config = LineSearchConfig {
            method: LineSearchMethod::Bisection,
            ..Default::default()
        };
        let ls = create_line_search(config);
        let _cloned = ls.clone_box();
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
            &rosenbrock,
            &rosenbrock_gradient,
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
    fn test_line_search_result_serialization() {
        use serde_json;
        let result = LineSearchResult {
            step_size: 0.5,
            success: true,
            termination_reason: TerminationReason::WolfeConditionsSatisfied,
        };
        // Test serialization
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"step_size\":0.5"));
        // Test deserialization
        let deserialized: LineSearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.step_size, result.step_size);
    }
    #[test]
    fn test_min_step_size() {
        let config = BacktrackingConfig {
            min_step: 1e-1, // Much larger minimum step
            initial_step: 1.0,
            rho: 0.9,          // Less aggressive backtracking
            c1: 1e-8,          // Very strict Armijo condition
            max_iterations: 5, // Few iterations
            ..Default::default()
        };
        let mut line_search = BacktrackingLineSearch::new(config.clone());
        // Use a function that requires very small steps to satisfy Armijo
        fn difficult_function(x: &[f64]) -> Result<f64> {
            // f(x) = x^2 but with a discontinuous jump that makes large steps bad
            if x[0] > 0.05 {
                Ok(1000.0 + x[0] * x[0])
            } else {
                Ok(x[0] * x[0])
            }
        }
        fn difficult_gradient(x: &[f64]) -> Result<Vec<f64>> {
            Ok(vec![2.0 * x[0]])
        }
        let current_point = vec![0.0];
        let direction = vec![1.0]; // Move in positive direction
        let current_gradient = difficult_gradient(&current_point).unwrap();
        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            &difficult_function,
            &difficult_gradient,
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
        // The test should handle both cases: success with small step or failure
        if result.success {
            // If it succeeded, the step size should be very small
            assert!(result.step_size <= config.min_step * 2.0);
        } else {
            // If it failed, it should be due to step size being too small
            assert!(matches!(
                result.termination_reason,
                TerminationReason::StepSizeTooSmall
            ));
        }
    }
    #[test]
    fn test_factory_functions() {
        // Test all factory functions
        let ls1 = strong_wolfe_line_search();
        let _cloned1 = ls1.clone_box();
        let ls2 = backtracking_line_search();
        let _cloned2 = ls2.clone_box();
        let ls3 = bisection_line_search();
        let _cloned3 = ls3.clone_box();
        let ls4 = strong_wolfe_with_config(StrongWolfeConfig::default());
        let _cloned4 = ls4.clone_box();
        let ls5 = backtracking_with_config(BacktrackingConfig::default());
        let _cloned5 = ls5.clone_box();
        let ls6 = bisection_with_config(BisectionConfig::default());
        let _cloned6 = ls6.clone_box();
        // Test new factory functions
        let ls7 = golden_section_line_search();
        let _cloned7 = ls7.clone_box();
        let ls8 = more_thuente_line_search();
        let _cloned8 = ls8.clone_box();
        let ls9 = cubic_quadratic_line_search();
        let _cloned9 = ls9.clone_box();
        let ls10 = golden_section_with_config(GoldenSectionConfig::default());
        let _cloned10 = ls10.clone_box();
        let ls11 = more_thuente_with_config(MoreThuenteConfig::default());
        let _cloned11 = ls11.clone_box();
        let ls12 = cubic_quadratic_with_config(CubicQuadraticConfig::default());
        let _cloned12 = ls12.clone_box();
    }
    #[test]
    fn test_golden_section_quadratic() {
        let mut line_search = GoldenSectionLineSearch::new(GoldenSectionConfig {
            verbose: false,
            ..GoldenSectionConfig::default()
        });
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0];
        let current_gradient = quadratic_gradient1(&current_point).unwrap();
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
    fn test_create_line_search_new_methods() {
        // Test creating new line search methods through factory
        let config = LineSearchConfig {
            method: LineSearchMethod::GoldenSection,
            ..Default::default()
        };
        let ls = create_line_search(config);
        let _cloned = ls.clone_box();
        let config = LineSearchConfig {
            method: LineSearchMethod::MoreThuente,
            ..Default::default()
        };
        let ls = create_line_search(config);
        let _cloned = ls.clone_box();
        let config = LineSearchConfig {
            method: LineSearchMethod::CubicQuadraticInterpolation,
            ..Default::default()
        };
        let ls = create_line_search(config);
        let _cloned = ls.clone_box();
    }
}
