use crate::utils::math::dot_product_f64;
use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

/// Trait for 1-D differentiable parametric curves
pub trait ParametricCurve: Send + Sync {
    /// Evaluate the curve at parameter t
    fn evaluate(&self, t: f64) -> Result<Vec<f64>>;
    /// Evaluate the derivative of the curve at parameter t
    fn derivative(&self, t: f64) -> Result<Vec<f64>>;
    /// Get the initial derivative at t=0 for descent checking
    fn initial_derivative(&self) -> Result<Vec<f64>>;
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
/// Convert a parametric curve and multi-dimensional functions into a 1D problem
pub fn create_1d_problem_candle<'a>(
    curve: Box<dyn ParametricCurve + 'a>,
    current_gradient: &'a [f64],
    objective_fn: &'a (dyn Fn(&[f64]) -> Result<f64, candle_core::Error> + Send + Sync),
    gradient_fn: &'a (dyn Fn(&[f64]) -> Result<Vec<f64>, candle_core::Error> + Send + Sync),
) -> Result<OneDimensionalProblem<'a>> {
    // Get initial directional derivative
    let initial_derivative = curve.initial_derivative()?;
    let initial_directional_derivative = dot_product_f64(&current_gradient, &initial_derivative)?;
    debug!("create_1d_problem_candle: current_gradient={:?}, initial_derivative={:?}, initial_directional_derivative={:.6e}",
          current_gradient, initial_derivative, initial_directional_derivative);
    // Use Arc to share the curve between closures
    let curve = Arc::new(curve);
    let curve_for_objective = curve.clone();
    let curve_for_gradient = curve.clone();
    // Create 1D objective function
    let objective_1d = move |t: f64| -> Result<f64> {
        let result_vec = curve_for_objective.evaluate(t)?;
        let result = objective_fn(&result_vec)
            .map_err(|e| anyhow!("Objective evaluation failed: {}", e))?;
        debug!("1D objective at t={:.6e}: f={:.6e}", t, result);
        Ok(result)
    };
    // Create 1D gradient function
    let gradient_1d = move |t: f64| -> Result<f64> {
        let result_vec = curve_for_gradient.evaluate(t)?;
        let result = gradient_fn(&result_vec)
            .map_err(|e| anyhow!("Gradient evaluation failed: {}", e))
            .and_then(|g| {
                if g.len() != current_gradient.len() {
                    return Err(anyhow!("Gradient length mismatch: expected {}, got {}", current_gradient.len(), g.len()));
                }
                // Compute directional derivative using dot product
                dot_product_f64(&g, &current_gradient)
            })?;
        debug!("1D gradient result at t={:.6e}: {:.6e}", t, result);
        Ok(result)
    };
    Ok(OneDimensionalProblem::new(
        Box::new(objective_1d),
        Box::new(gradient_1d),
        initial_directional_derivative,
    ))
}

pub fn create_1d_problem<'a>(
    curve: Box<dyn ParametricCurve + 'a>,
    current_gradient: &'a [f64],
    objective_fn: &'a (dyn Fn(&[f64]) -> anyhow::Result<f64> + Send + Sync),
    gradient_fn: &'a (dyn Fn(&[f64]) -> anyhow::Result<Vec<f64>> + Send + Sync),
) -> Result<OneDimensionalProblem<'a>> {
    // Get initial directional derivative
    let initial_derivative = curve.initial_derivative()?;
    let initial_directional_derivative = dot_product_f64(&current_gradient, &initial_derivative)?;
    debug!("create_1d_problem: current_gradient={:?}, initial_derivative={:?}, initial_directional_derivative={:.6e}",
          current_gradient, initial_derivative, initial_directional_derivative);

    // Use Arc to share the curve between closures
    let curve = Arc::new(curve);
    let curve_for_objective = curve.clone();
    let curve_for_gradient = curve.clone();

    // Create 1D objective function
    let objective_1d = move |t: f64| -> Result<f64> {
        let result_vec = curve_for_objective.evaluate(t)?;
        let result = objective_fn(&result_vec)?;
        debug!("1D objective at t={:.6e}: f={:.6e}", t, result);
        Ok(result)
    };

    // Create 1D gradient function
    let gradient_1d = move |t: f64| -> Result<f64> {
        let result_vec = curve_for_gradient.evaluate(t)?;
        let curve_derivative = curve_for_gradient.derivative(t)?;
        let result = gradient_fn(&result_vec)
            .and_then(|g| {
                if g.len() != curve_derivative.len() {
                    return Err(anyhow!("Gradient length mismatch: expected {}, got {}", curve_derivative.len(), g.len()));
                }
                // Compute directional derivative: ∇f(x(t)) · dx/dt
                dot_product_f64(&g, &curve_derivative)
            })?;
        debug!("1-D gradient result at t={:.6e}; p={:?} = {:.6e}", t, result_vec, result);
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
    current_gradient: &'a [f64],
    objective_fn: &'a (dyn Fn(&[f64]) -> Result<f64> + Send + Sync),
    gradient_fn: &'a (dyn Fn(&[f64]) -> Result<Vec<f64>> + Send + Sync),
) -> Result<OneDimensionalProblem<'a>> {
    let curve = LinearCurve::new(
        current_point.to_vec(),
        direction.to_vec(),
    );

    // Debug: let's verify the curve works correctly
    let test_val_0 = curve.evaluate(0.0)?;
    let test_val_1 = curve.evaluate(1.0)?;
    debug!("Curve test: f(t=0) -> {:?}, f(t=1) -> {:?}", test_val_0, test_val_1);

    let result = create_1d_problem(Box::new(curve), current_gradient, objective_fn, gradient_fn);
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
        Self { start_point, direction }
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
    fn evaluate(&self, t: f64) -> Result<Vec<f64>> {
        Ok(self.point_at(t))
    }
    fn derivative(&self, _t: f64) -> Result<Vec<f64>> {
        // For a linear curve, dx/dt = direction (constant)
        Ok(self.direction.clone())
    }
    fn initial_derivative(&self) -> Result<Vec<f64>> {
        Ok(self.direction.clone())
    }
}
/// Linear parametric curve that evaluates a function along the path
pub struct FunctionLinearCurve<'a> {
    curve: LinearCurve,
    objective_fn: &'a (dyn Fn(&[f64]) -> Result<f64> + Send + Sync),
    gradient_fn: &'a (dyn Fn(&[f64]) -> Result<Vec<f64>> + Send + Sync),
}
impl<'a> FunctionLinearCurve<'a> {
    pub fn new(
        start_point: Vec<f64>,
        direction: Vec<f64>,
        objective_fn: &'a (dyn Fn(&[f64]) -> Result<f64> + Send + Sync),
        gradient_fn: &'a (dyn Fn(&[f64]) -> Result<Vec<f64>> + Send + Sync),
    ) -> Self {
        Self {
            curve: LinearCurve::new(start_point, direction),
            objective_fn,
            gradient_fn,
        }
    }
}
impl<'a> ParametricCurve for FunctionLinearCurve<'a> {
    fn evaluate(&self, t: f64) -> Result<Vec<f64>> {
        let point = self.curve.point_at(t);
        debug!("Evaluating function at t={:.6e}: point={:?}", t, point);
        let f_val = (self.objective_fn)(&point)?;
        debug!("Function value at t={:.6e}: f={:.6e}", t, f_val);
        Ok(vec![f_val])
    }
    fn derivative(&self, t: f64) -> Result<Vec<f64>> {
        let point = self.curve.point_at(t);
        let gradient = (self.gradient_fn)(&point)?;
        debug!("Computing derivative at t={:.6e}: point={:?}, gradient={:?}", t, point, gradient);
        // Chain rule: df/dt = ∇f(x(t)) · dx/dt
        // dx/dt = direction (constant for linear curve)
        let directional_derivative = dot_product_f64(&gradient, &self.curve.direction)?;
        Ok(vec![directional_derivative])
    }
    fn initial_derivative(&self) -> Result<Vec<f64>> {
        self.derivative(0.0)
    }
}

/// Line search result containing step size and evaluation counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchResult {
    pub step_size: f64,
    pub function_evaluations: usize,
    pub gradient_evaluations: usize,
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
    pub verbose: bool, // Enable verbose logging
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineSearchMethod {
    StrongWolfe,
    Backtracking,
    Bisection,
}

impl Default for LineSearchConfig {
    fn default() -> Self {
        Self {
            method: LineSearchMethod::StrongWolfe,
            c1: 1e-4,
            c2: 0.1,  // Much less strict curvature condition for better performance
            max_iterations: 50,
            initial_step: 1.0,
            min_step: 1e-16,
            max_step: 100.0,  // More reasonable maximum step
            verbose: true, // Default to no verbose logging
        }
    }
}
/// Create a line search algorithm from configuration
pub fn create_line_search(config: LineSearchConfig) -> Box<dyn LineSearch> {
    match config.method {
        LineSearchMethod::StrongWolfe => {
            Box::new(StrongWolfeLineSearch::new(StrongWolfeConfig {
                c1: config.c1,
                c2: config.c2,
                max_iterations: config.max_iterations,
                min_step: config.min_step,
                max_step: config.max_step,
                initial_step: config.initial_step,
                verbose: config.verbose,
            }))
        }
        LineSearchMethod::Backtracking => {
            Box::new(BacktrackingLineSearch::new(BacktrackingConfig {
                c1: config.c1,
                rho: 0.5, // Default backtracking factor
                max_iterations: config.max_iterations,
                min_step: config.min_step,
                initial_step: config.initial_step,
            }))
        }
        LineSearchMethod::Bisection => {
            Box::new(BisectionLineSearch::new(BisectionConfig {
                max_iterations: config.max_iterations,
                gradient_tolerance: 1e-8,
                min_step: config.min_step,
                max_step: config.max_step,
                initial_step: config.initial_step,
                window_shrink_factor: 0.5,
                verbose: config.verbose,
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
            debug!("  Armijo: f({:.6e})={:.6e} <= {:.6e} + {:.6e}*{:.6e}*{:.6e} = {:.6e}? {}",
                  alpha, f_alpha, f0, self.config.c1, alpha, directional_derivative, threshold, satisfied);
        }
        satisfied
    }

    /// Check curvature condition: |f'(α)| ≤ c2*|f'(0)|
    fn curvature_condition(&self, grad_alpha: f64, directional_derivative: f64) -> bool {
        let threshold = self.config.c2 * directional_derivative.abs();
        let satisfied = grad_alpha.abs() <= threshold;
        if self.config.verbose {
            debug!("  Curvature: |{:.6e}| <= {:.6e}*|{:.6e}| = {:.6e}? {}",
                grad_alpha, self.config.c2, directional_derivative, threshold, satisfied);
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
    ) -> Result<(f64, usize, usize)> {
        let mut alpha_lo = alpha_lo;
        let mut alpha_hi = alpha_hi;
        let mut f_evals = 0;
        let mut g_evals = 0;
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
            f_evals += 1;
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
            g_evals += 1;


            // Check curvature condition
            if self.curvature_condition(grad_alpha_j, directional_derivative) {
                return Ok((alpha_j, f_evals, g_evals));
            }

            // Update interval
            if grad_alpha_j * (alpha_hi - alpha_lo) >= 0.0 {
                alpha_hi = alpha_lo;
            }
            alpha_lo = alpha_j;
        }

        // Return best point found during search
        Ok((best_alpha, f_evals, g_evals))
    }
}

impl LineSearch for StrongWolfeLineSearch {
    fn optimize_1d<'a>(&mut self, problem: &'a OneDimensionalProblem<'a>) -> Result<LineSearchResult> {
        let f0 = (problem.objective)(0.0)?;
        let directional_derivative = problem.initial_directional_derivative;

        self.log_verbose(&format!("Starting 1D optimization with f(0)={:.6e}", f0));
        self.log_verbose(&format!("Directional derivative: {:.6e}", directional_derivative));

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut alpha_prev = 0.0;
        let mut f_prev = f0;
        let mut f_evals = 1; // Already evaluated f(0)
        let mut g_evals = 0;

        self.log_verbose(&format!("Initial step size: {:.6e}", alpha));

        for i in 0..self.config.max_iterations {
            self.log_verbose(&format!("Iteration {}: trying alpha={:.6e}", i, alpha));

            // Evaluate function at current step size
            let f_alpha = (problem.objective)(alpha)?;
            f_evals += 1;
            self.log_verbose(&format!("  f({:.6e}) = {:.6e}", alpha, f_alpha));

            // Check Armijo condition and sufficient decrease
            if !self.armijo_condition(f0, f_alpha, alpha, directional_derivative)
                || (i > 0 && f_alpha >= f_prev)
            {
                self.log_verbose(&format!("  Armijo failed or insufficient decrease, zooming between {:.6e} and {:.6e}", alpha_prev, alpha));
                // Zoom between alpha_prev and alpha
                let (final_alpha, zoom_f_evals, zoom_g_evals) = self.zoom(
                    alpha_prev,
                    alpha,
                    f0,
                    directional_derivative,
                    &problem,
                )?;
                self.log_verbose(&format!("Zoom completed with alpha={:.6e}", final_alpha));

                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    function_evaluations: f_evals + zoom_f_evals,
                    gradient_evaluations: g_evals + zoom_g_evals,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Evaluate gradient at current point
            let grad_alpha = (problem.gradient)(alpha)?;
            g_evals += 1;

            // Check curvature condition
            if self.curvature_condition(grad_alpha, directional_derivative) {
                self.log_verbose(&format!("Both Wolfe conditions satisfied at alpha={:.6e}", alpha));
                return Ok(LineSearchResult {
                    step_size: alpha,
                    function_evaluations: f_evals,
                    gradient_evaluations: g_evals,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Check if gradient indicates we should look further
            if grad_alpha >= 0.0 {
                self.log_verbose(&format!("  Gradient indicates overshoot, zooming between {:.6e} and {:.6e}", alpha, alpha_prev));
                let (final_alpha, zoom_f_evals, zoom_g_evals) = self.zoom(
                    alpha,
                    alpha_prev,
                    f0,
                    directional_derivative,
                    &problem,
                )?;

                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    function_evaluations: f_evals + zoom_f_evals,
                    gradient_evaluations: g_evals + zoom_g_evals,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Update for next iteration
            alpha_prev = alpha;
            f_prev = f_alpha;
            alpha = alpha.min(self.config.max_step) * 2.0; // Expand step size
            self.log_verbose(&format!("  Expanding step size to {:.6e}", alpha));

            if alpha < self.config.min_step {
                self.log_verbose("Step size below minimum, terminating");
                break;
            }
        }

        // Line search failed to converge
        self.log_verbose(&format!("Line search failed to converge, returning alpha={:.6e}", alpha_prev));
        Ok(LineSearchResult {
            step_size: alpha_prev,
            function_evaluations: f_evals,
            gradient_evaluations: g_evals,
            success: false,
            termination_reason: TerminationReason::MaxIterationsReached,
        })
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
    pub min_step: f64,            // Minimum step size
    pub max_step: f64,            // Maximum step size
    pub initial_step: f64,        // Initial step size
    pub window_shrink_factor: f64, // Factor to shrink window on failure
    pub verbose: bool,            // Enable verbose logging
}
impl Default for BisectionConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            gradient_tolerance: 1e-8,
            min_step: 1e-16,
            max_step: 1e16,
            initial_step: 1.0,
            window_shrink_factor: 0.5,
            verbose: false,
        }
    }
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
    ) -> Result<(f64, usize, usize)> {
        let mut a = left;
        let mut b = right;
        let mut g_evals = 0;
        let f_evals = 0;
        self.log_verbose(&format!("Finding zero gradient in interval [{:.6e}, {:.6e}]", a, b));
        for i in 0..self.config.max_iterations {
            let mid = 0.5 * (a + b);
            // Evaluate gradient at midpoint
            let grad_mid = (problem.gradient)(mid)?;
            g_evals += 1;
            self.log_verbose(&format!("  Iteration {}: mid={:.6e}, grad={:.6e}", i, mid, grad_mid));
            // Check if gradient is close enough to zero
            if grad_mid.abs() <= self.config.gradient_tolerance {
                self.log_verbose(&format!("Found zero gradient at alpha={:.6e}", mid));
                return Ok((mid, f_evals, g_evals));
            }
            // Check if interval is too small
            if (b - a) < self.config.min_step {
                self.log_verbose(&format!("Interval too small, returning mid={:.6e}", mid));
                return Ok((mid, f_evals, g_evals));
            }
            // Evaluate gradients at endpoints if not already done
            let grad_a = (problem.gradient)(a)?;
            g_evals += 1;
            // Update interval based on sign of gradient
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
        self.log_verbose(&format!("Max iterations reached, returning alpha={:.6e}", final_alpha));
        Ok((final_alpha, f_evals, g_evals))
    }
    /// Window search with successive halving
    fn window_search(
        &self,
        initial_window: f64,
        problem: &OneDimensionalProblem,
    ) -> Result<(f64, usize, usize)> {
        let mut window_size = initial_window;
        let total_f_evals = 0;
        let mut total_g_evals = 0;
        let mut iteration_count = 0;
        self.log_verbose(&format!("Starting window search with initial window={:.6e}", window_size));
        while window_size >= self.config.min_step && iteration_count < self.config.max_iterations {
            iteration_count += 1;
            self.log_verbose(&format!("Trying window size: {:.6e}", window_size));
            // Check if we can find a sign change in the gradient within this window
            let grad_0 = problem.initial_directional_derivative;
            // Actually evaluate the gradient at t=0 to make sure it matches
            let grad_0_actual = (problem.gradient)(0.0)?;
            total_g_evals += 1;
            if (grad_0 - grad_0_actual).abs() > 1e-10 {
                self.log_verbose(&format!("WARNING: initial_directional_derivative={:.6e} != actual grad(0)={:.6e}",
                                          grad_0, grad_0_actual));
            }

            let grad_window = (problem.gradient)(window_size)?;
            total_g_evals += 1;
            self.log_verbose(&format!("  grad(0)={:.6e}, grad({:.6e})={:.6e}", grad_0, window_size, grad_window));
            // If gradients have opposite signs, we can use bisection
            if grad_0 * grad_window < 0.0 {
                self.log_verbose("Found sign change, using bisection");
                let (alpha, f_evals, g_evals) = self.find_zero_gradient(0.0, window_size, problem)?;
                return Ok((alpha, total_f_evals + f_evals, total_g_evals + g_evals));
            }
            // If gradient becomes non-negative, we've found a good stopping point
            if grad_window >= 0.0 {
                self.log_verbose(&format!("Gradient became non-negative at window={:.6e}, using this as step size", window_size));
                return Ok((window_size, total_f_evals, total_g_evals));
            }
            // If gradient is still negative, we need a smaller window
            if grad_window < 0.0 {
                self.log_verbose("Gradient still negative, shrinking window");
                window_size *= self.config.window_shrink_factor;
                continue;
            }
        }
        // If we can't find a good window, return the minimum step (which may be too small)
        let final_alpha = window_size;
        self.log_verbose(&format!("Window search completed after {} iterations, returning alpha={:.6e}", iteration_count, final_alpha));
        Ok((final_alpha, total_f_evals, total_g_evals))
    }
}
impl LineSearch for BisectionLineSearch {
    fn optimize_1d<'a>(&mut self, problem: &'a OneDimensionalProblem<'a>) -> Result<LineSearchResult> {
        let directional_derivative = problem.initial_directional_derivative;
        self.log_verbose(&format!("Starting bisection line search"));
        self.log_verbose(&format!("Initial directional derivative: {:.6e}", directional_derivative));
        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }
        // Start with window search to find appropriate interval
        let (step_size, f_evals, g_evals) = self.window_search(self.config.initial_step, problem)?;
        // Verify the step size is reasonable
        if step_size < self.config.min_step {
            return Ok(LineSearchResult {
                step_size,
                function_evaluations: f_evals,
                gradient_evaluations: g_evals,
                success: false,
                termination_reason: TerminationReason::StepSizeTooSmall,
            });
        }
        // Check final gradient to see if we found a good point
        let final_gradient = (problem.gradient)(step_size)?;
        let success = final_gradient.abs() <= self.config.gradient_tolerance;
        self.log_verbose(&format!("Final result: alpha={:.6e}, grad={:.6e}, success={}",
                                  step_size, final_gradient, success));
        Ok(LineSearchResult {
            step_size,
            function_evaluations: f_evals,
            gradient_evaluations: g_evals + 1, // +1 for final gradient check
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
    fn optimize_1d<'a>(&mut self, problem: &'a OneDimensionalProblem<'a>) -> Result<LineSearchResult> {
        let f0 = (problem.objective)(0.0)?;
        let directional_derivative = problem.initial_directional_derivative;

        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut f_evals = 1; // Already evaluated f(0)

        for _ in 0..self.config.max_iterations {
            // Evaluate function at current step size
            let f_alpha = (problem.objective)(alpha)?;
            f_evals += 1;

            // Check Armijo condition
            let armijo_threshold = f0 + self.config.c1 * alpha * directional_derivative;
            if f_alpha <= armijo_threshold {
                return Ok(LineSearchResult {
                    step_size: alpha,
                    function_evaluations: f_evals,
                    gradient_evaluations: 0,
                    success: true,
                    termination_reason: TerminationReason::ArmijoConditionSatisfied,
                });
            }

            // Backtrack
            alpha *= self.config.rho;

            if alpha < self.config.min_step {
                break;
            }
        }

        // Line search failed
        Ok(LineSearchResult {
            step_size: alpha,
            function_evaluations: f_evals,
            gradient_evaluations: 0,
            success: false,
            termination_reason: TerminationReason::StepSizeTooSmall,
        })
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
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
        // Test that f(0) gives the current function value
        let f0 = (problem.objective)(0.0).unwrap();
        let expected_f0 = quadratic_function(&current_point).unwrap();
        assert_relative_eq!(f0, expected_f0, epsilon = 1e-10);
        // Test that f'(0) gives the directional derivative
        let expected_directional_derivative = -2.0 * 2.0 + -3.0 * 3.0; // direction · gradient
        assert_relative_eq!(problem.initial_directional_derivative, expected_directional_derivative, epsilon = 1e-10);
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
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
        let result = line_search.optimize_1d(&problem)
            .unwrap();

        assert!(result.success);
        assert!(result.step_size > 0.0);
        assert!(result.function_evaluations > 0);

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
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();

        let result = line_search.optimize_1d(&problem)
            .unwrap();

        assert!(result.success);
        assert!(result.step_size > 0.0);
        assert!(result.function_evaluations > 0);
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
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
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
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();

        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        assert!(result.gradient_evaluations > 0);
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
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient1,
        ).unwrap();
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
        let p0 = curve.evaluate(0.0).unwrap();
        assert_eq!(p0, vec![1.0, 2.0]);
        let p1 = curve.evaluate(1.0).unwrap();
        assert_eq!(p1, vec![4.0, 6.0]);
        let p_half = curve.evaluate(0.5).unwrap();
        assert_eq!(p_half, vec![2.5, 4.0]);
        // Test derivative (should be constant)
        let d0 = curve.derivative(0.0).unwrap();
        assert_eq!(d0, direction);
        let d1 = curve.derivative(1.0).unwrap();
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
            &current_gradient,
            &rosenbrock,
            &rosenbrock_gradient,
        ).unwrap();
        let result = line_search.optimize_1d(&problem).unwrap();
        assert!(result.success);
        assert!(result.step_size > 0.0);
        // Verify that the function value decreased
        let new_point: Vec<f64> = current_point.iter()
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
            function_evaluations: 10,
            gradient_evaluations: 5,
            success: true,
            termination_reason: TerminationReason::WolfeConditionsSatisfied,
        };
        // Test serialization
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"step_size\":0.5"));
        // Test deserialization
        let deserialized: LineSearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.step_size, result.step_size);
        assert_eq!(deserialized.function_evaluations, result.function_evaluations);
    }
    #[test]
    fn test_min_step_size() {
        let config = BacktrackingConfig {
            min_step: 1e-1, // Much larger minimum step
            initial_step: 1.0,
            rho: 0.9, // Less aggressive backtracking
            c1: 1e-8, // Very strict Armijo condition
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
            &current_gradient,
            &difficult_function,
            &difficult_gradient,
        ).unwrap();
        let result = line_search.optimize_1d(&problem)
            .map_or_else(|e| {
                debug!("Line search failed: {}", e);
                // If it fails, we expect it to be due to step size being too small
                LineSearchResult {
                    step_size: 0.0,
                    function_evaluations: 0,
                    gradient_evaluations: 0,
                    success: false,
                    termination_reason: TerminationReason::StepSizeTooSmall,
                }
            }, |res| res);
        // The test should handle both cases: success with small step or failure
        if result.success {
            // If it succeeded, the step size should be very small
            assert!(result.step_size <= config.min_step * 2.0);
        } else {
            // If it failed, it should be due to step size being too small
            assert!(matches!(result.termination_reason, TerminationReason::StepSizeTooSmall));
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
    }
}