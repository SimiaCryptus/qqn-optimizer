use crate::utils::math::dot_product_f64;
use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Trait for 1-D differentiable parametric curves
pub trait ParametricCurve<'a>: Send + Sync + Debug {
    /// Evaluate the curve at parameter t
    fn evaluate(&self, t: f64) -> Result<Vec<f64>>;
    /// Evaluate the derivative of the curve at parameter t
    fn derivative(&self, t: f64) -> Result<Vec<f64>>;
    /// Get the initial derivative at t=0 for descent checking
    fn initial_derivative(&self) -> Result<Vec<f64>>;
    /// Clone the curve
    fn clone_box(&self) -> Box<dyn ParametricCurve<'a>>;
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
pub fn create_1d_problem<'a>(
    curve: Box<dyn ParametricCurve<'a> + 'a>,
    current_gradient: &'a [f64],
    objective_fn: &'a (dyn Fn(&[f64]) -> Result<f64> + Send + Sync),
    gradient_fn: &'a (dyn Fn(&[f64]) -> Result<Vec<f64>> + Send + Sync),
) -> Result<OneDimensionalProblem<'a>> {
    // Get initial directional derivative
    let initial_derivative = curve.initial_derivative()?;
    let initial_directional_derivative = dot_product_f64(current_gradient, &initial_derivative)?;
    // Clone the curve for use in closures
    let curve_for_objective = curve.clone_box();
    let curve_for_gradient = curve;
    // Create 1D objective function
    let objective_1d = move |t: f64| -> Result<f64> {
        let point = curve_for_objective.evaluate(t)?;
        (objective_fn)(&point)
    };
    // Create 1D gradient function
    let gradient_1d = Box::new(move |t: f64| -> Result<f64> {
        let point = curve_for_gradient.evaluate(t)?;
        let curve_derivative = curve_for_gradient.derivative(t)?;
        let gradient_vec = (gradient_fn)(&point)?;
        // Compute directional derivative: gradient · curve_derivative
        dot_product_f64(&gradient_vec, &curve_derivative)
    });
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
    let curve = LinearCurve::new(current_point.to_vec(), direction.to_vec());
    create_1d_problem(Box::new(curve), current_gradient, objective_fn, gradient_fn)
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
}
impl<'a> ParametricCurve<'a> for LinearCurve {
    fn evaluate(&self, t: f64) -> Result<Vec<f64>> {
        Ok(self.start_point
            .iter()
            .zip(self.direction.iter())
            .map(|(x, d)| x + t * d)
            .collect())
    }
    fn derivative(&self, _t: f64) -> Result<Vec<f64>> {
        // For linear curves, derivative is constant
        Ok(self.direction.clone())
    }
    fn initial_derivative(&self) -> Result<Vec<f64>> {
        Ok(self.direction.clone())
    }
    fn clone_box(&self) -> Box<dyn ParametricCurve<'a>> {
        Box::new(self.clone())
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
            c2: 0.9,
            max_iterations: 50,
            initial_step: 1.0,
            min_step: 1e-16,
            max_step: 1e16,
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
                verbose: false,
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
                verbose: false,
            }))
        }
    }
}

/// Trait for line search algorithms
pub trait LineSearch: Send + Sync + Debug {
    /// Perform line search along a given direction
    /// This is the primary interface that should be used by optimizers
    fn search(
        &mut self,
        params: &[candle_core::Tensor],
        direction: &[candle_core::Tensor],
        gradients: &[candle_core::Tensor],
        function: &dyn crate::core::optimizer::DifferentiableFunction,
    ) -> Result<LineSearchResult> {
        // Convert to f64 for line search
        let params_f64: Vec<f64> = params
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();
        let direction_f64: Vec<f64> = direction
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();
        let gradients_f64: Vec<f64> = gradients
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();
        // Create closures with let bindings to ensure they live long enough
        let objective_fn = |x: &[f64]| {
            let tensors = crate::core::qqn::f64_to_tensors(x, params)?;
            function.evaluate(&tensors)
                .map_err(|e| anyhow!("Function evaluation failed: {}", e))
        };
        let gradient_fn = |x: &[f64]| {
            let tensors = crate::core::qqn::f64_to_tensors(x, params)?;
            let grads = function.gradient(&tensors)
                .map_err(|e| anyhow!("Gradient evaluation failed: {}", e))?;
            Ok(grads
                .iter()
                .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
                .collect())
        };

        // Create 1D problem
        let problem = create_1d_problem_linear(
            &params_f64,
            &direction_f64,
            &gradients_f64,
            &objective_fn,
            &gradient_fn,
        )?;
        self.optimize_1d(&problem)
    }

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

        for _ in 0..self.config.max_iterations {
            // Interpolate to find new trial point
            let alpha_j = 0.5 * (alpha_lo + alpha_hi); // Simple bisection


            // Evaluate 1D function at trial point
            let f_alpha_j = (problem.objective)(alpha_j)?;
            f_evals += 1;

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

        // Return best point found
        Ok((alpha_lo, f_evals, g_evals))
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
        let mut f_evals = 0;
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
        let mut total_f_evals = 0;
        let mut total_g_evals = 0;
        let mut iteration_count = 0;
        self.log_verbose(&format!("Starting window search with initial window={:.6e}", window_size));
        while window_size >= self.config.min_step && iteration_count < self.config.max_iterations {
            iteration_count += 1;
            self.log_verbose(&format!("Trying window size: {:.6e}", window_size));
            // Check if we can find a sign change in the gradient within this window
            let grad_0 = problem.initial_directional_derivative;
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
        // If we can't find a good window, return a small step
        let final_alpha = window_size.max(self.config.min_step);
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
    use crate::init_logging;
    use approx::assert_relative_eq;

    fn quadratic_function(x: &[f64]) -> Result<f64> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }

    fn quadratic_gradient1(x: &[f64]) -> Result<Vec<f64>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }

    fn quadratic_gradient2(x: &[f64]) -> Result<f64> {
        // ∇f(x) = x
        Ok(x[0])
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
        init_logging();
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
        init_logging();
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
        init_logging();
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
        assert!(result.function_evaluations >= 0);
        assert!(result.gradient_evaluations > 0);
        // For quadratic function, optimal step should be 1.0 (where gradient is zero)
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }
    #[test]
    fn test_bisection_non_descent() {
        init_logging();
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
}