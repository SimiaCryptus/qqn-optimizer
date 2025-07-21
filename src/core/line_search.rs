use crate::core::line_search_bisection::BisectionLineSearch;
use crate::utils::math::dot_product_f64;
use anyhow::{anyhow, Error, Result};
use log::{debug, warn};
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
pub struct OneDimensionalProblem {
    /// The 1D objective function f(t)
    pub objective: Arc<dyn Fn(f64) -> Result<f64> + Send + Sync>,
    /// The 1D gradient function f'(t)
    pub gradient: Arc<dyn Fn(f64) -> Result<f64> + Send + Sync>,
    /// Initial directional derivative at t=0
    pub initial_directional_derivative: f64,

}
impl Debug for OneDimensionalProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OneDimensionalProblem")
            .field("initial_directional_derivative", &self.initial_directional_derivative)
            .field("objective", &"<closure>")
            .field("gradient", &"<closure>")
            .finish()
    }
}

impl OneDimensionalProblem {
    pub fn new(
        objective: Arc<dyn Fn(f64) -> Result<f64> + Send + Sync>,
        gradient: Arc<dyn Fn(f64) -> Result<f64> + Send + Sync>,
        initial_directional_derivative: f64,
    ) -> Self {
        assert!(
            initial_directional_derivative < 0.0,
            "Initial directional derivative must be negative for descent direction"
        );
        Self {
            objective,
            gradient,
            initial_directional_derivative,
        }
    }
}

pub fn create_1d_problem(
    curve: Box<dyn ParametricCurve>,
    objective_fn: Arc<dyn Fn(&[f64]) -> Result<f64> + Send + Sync>,
    gradient_fn: Arc<dyn Fn(&[f64]) -> Result<Vec<f64>> + Send + Sync>,
) -> Result<OneDimensionalProblem> {
    let initial_position = curve.position(0.0)
        .map_err(|e| anyhow!("Failed to evaluate curve at t=0: {}", e))?;
    let initial_direction = curve.direction(0.0)
        .map_err(|e| anyhow!("Failed to evaluate curve direction at t=0: {}", e))?;
    let initial_value = objective_fn(&initial_position).map_err(|e| anyhow!("Objective evaluation failed: {}", e))?;
    let initial_gradient = gradient_fn(&initial_position)?; // This is ∇f
    let initial_directional_derivative = dot_product_f64(&initial_gradient, &initial_direction)?;
    debug!("create_1d_problem: initial_derivative={:?}, initial_direction={:?}, initial_directional_derivative={:.3e}",
          initial_gradient, initial_direction, initial_directional_derivative);
    // Check for zero direction
    let direction_norm = initial_direction.iter().map(|x| x * x).sum::<f64>().sqrt();
    if direction_norm < 1e-16 {
        return Err(anyhow!("Direction vector is essentially zero (norm = {:.3e})", direction_norm));
    }

    // For descent: ∇f · d < 0
    if initial_directional_derivative > 0.0 {
        // Warn and flip the direction of the gradient fn
        warn!("Initial directional derivative is positive ({:.3e}), flipping direction", initial_directional_derivative);
        let negative_gradient_fn = {
            let gradient_fn = gradient_fn.clone();
            Arc::new(move |x: &[f64]| -> Result<Vec<f64>, Error> {
                gradient_fn(x).map(|g| g.iter().map(|v| -v).collect())
            })
        };
        return create_1d_problem(
            curve,
            objective_fn, // Keep the objective function
            negative_gradient_fn, // Negate the gradient
        );
    } else if initial_directional_derivative == 0.0 {
        return Err(anyhow!("Initial directional derivative must be negative for descent direction: {:.3e}", initial_directional_derivative));
    }


    // Use Arc to share the curve between closures
    let curve = Arc::new(curve);
    let curve_for_objective = curve.clone();
    let curve_for_gradient = curve.clone();
    let objective_fn_for_closure = objective_fn.clone();
    let gradient_fn_for_closure = gradient_fn.clone();

    // Create 1D objective function
    let objective_1d = Arc::new(move |t: f64| -> Result<f64> {
        let result_vec = curve_for_objective.position(t)?;
        let result = objective_fn_for_closure(&result_vec)?;
        debug!(
            "1D objective at t={:.3e}: f={:.3e}, improvement: {:.3e}",
            t, result, (initial_value - result)
        );
        Ok(result)
    });

    // Create 1D gradient function
    let gradient_1d = Arc::new(move |t: f64| -> Result<f64> {
        let result_vec = curve_for_gradient.position(t)?;
        let curve_derivative = curve_for_gradient.direction(t)?;
        let result = gradient_fn_for_closure(&result_vec).and_then(|g| {
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
    });
    Ok(OneDimensionalProblem::new(
        objective_1d,
        gradient_1d,
        initial_directional_derivative,
    ))
}
/// Convert a linear search direction into a 1D problem
pub fn create_1d_problem_linear(
    current_point: &[f64],
    direction: &[f64],
    objective_fn: Arc<dyn Fn(&[f64]) -> Result<f64> + Send + Sync>,
    gradient_fn: Arc<dyn Fn(&[f64]) -> Result<Vec<f64>> + Send + Sync>,
) -> Result<OneDimensionalProblem> {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TerminationReason {
    WolfeConditionsSatisfied,
    ArmijoConditionSatisfied,
    MaxIterationsReached,
    StepSizeTooSmall,
    FunctionEvaluationError,
    InvalidDirection,
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
    use crate::core::line_search_backtracking::{
        BacktrackingConfig, BacktrackingLineSearch,
    };
    use crate::core::line_search_bisection::BisectionConfig;
    use crate::core::line_search_cubic_quadratic::{CubicQuadraticConfig, CubicQuadraticLineSearch};
    use crate::core::line_search_golden_section::{GoldenSectionConfig, GoldenSectionLineSearch};
    use crate::core::line_search_more_thuente::{MoreThuenteConfig, MoreThuenteLineSearch};
    use crate::core::line_search_strong_wolfe::{StrongWolfeConfig, StrongWolfeLineSearch};
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
                ..CubicQuadraticConfig::default()
            }))
        }
    }
}

/// Trait for line search algorithms
pub trait LineSearch: Send + Sync + Debug {
    /// Perform 1D line search optimization
    fn optimize_1d(&mut self, problem: &OneDimensionalProblem) -> Result<LineSearchResult>;

    /// Reset internal state
    fn reset(&mut self);

    /// Clone the line search algorithm
    fn clone_box(&self) -> Box<dyn LineSearch>;
}

/// Find far point using gradient-based method
/// Looks for a point where f(t) < f(0) and gradient is positive (function starts increasing)
pub(crate) fn find_far_point_1(
    problem: &OneDimensionalProblem,
    f0: f64,
    initial_step: f64,
    max_iterations: usize,
    min_step: f64,
    gradient_tolerance: f64,
    max_step: f64,
) -> Result<f64, Error> {
    let mut t = initial_step;
    let mut iteration = 0;
    debug!("Finding far point starting from t={:.3e}", t);
    while iteration < max_iterations {
        let f_t = (problem.objective)(t)?;
        let grad_t = (problem.gradient)(t)?;
        debug!(
            "  Line Search Iteration {}: t={:.3e}, f={:.3e}, grad={:.3e}, f0={:.3e}",
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
pub(crate) fn find_far_point_2(
    problem: &OneDimensionalProblem,
    f0: f64,
    initial_step: f64,
    max_iterations: usize,
    max_step: f64,
) -> Result<f64, Error> {
    let mut t = initial_step;
    let mut iteration = 0;
    debug!("Finding far point starting from t={:.3e}", t);
    while iteration < max_iterations {
        let f_t = (problem.objective)(t)?;
        debug!(
            "  Line Search Iteration {}: t={:.3e}, f={:.3e}, f0={:.3e}",
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
        let objective_fn = Arc::new(quadratic_function);
        let gradient_fn = Arc::new(quadratic_gradient1);
        // Calculate expected value before moving objective_fn
        let expected_f0 = objective_fn(&current_point).unwrap();

        let problem = create_1d_problem_linear(
            &current_point,
            &direction,
            objective_fn,
            gradient_fn,
        ).unwrap();
        // Test that f(0) gives the current function value
        let f0 = (problem.objective)(0.0).unwrap();
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
}