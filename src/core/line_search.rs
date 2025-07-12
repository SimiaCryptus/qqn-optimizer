use crate::utils::math::{compute_magnitude, dot_product_f64};
use anyhow::{Result, anyhow};
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use candle_core::Tensor;

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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineSearchMethod {
    StrongWolfe,
    Backtracking,
}
impl Default for LineSearchConfig {
    fn default() -> Self {
        Self {
            method: LineSearchMethod::StrongWolfe,
            c1: 1e-4,
            c2: 0.9,
            max_iterations: 50,
            initial_step: 1.0,
        }
    }
}


/// Trait for line search algorithms
pub trait LineSearch: Send + Sync + Debug {
    /// Perform line search along given direction
    fn search(
        &mut self,
       current_point: &[f64],
       direction: &[f64],
       current_value: f64,
       current_gradient: &[f64],
       objective_fn: &dyn Fn(&[f64]) -> anyhow::Result<f64>,
       gradient_fn: &dyn Fn(&[f64]) -> anyhow::Result<Vec<f64>>,
    ) -> Result<LineSearchResult>;

    /// Reset internal state
    fn reset(&mut self);
    /// Clone the line search algorithm
    fn clone_box(&self) -> Box<dyn LineSearch>;
}

/// Configuration for Strong Wolfe line search
#[derive(Debug, Clone)]
pub struct StrongWolfeConfig {
    pub c1: f64,              // Armijo condition parameter (0 < c1 < c2 < 1)
    pub c2: f64,              // Curvature condition parameter
    pub max_iterations: usize, // Maximum line search iterations
    pub min_step: f64,        // Minimum step size
    pub max_step: f64,        // Maximum step size
    pub initial_step: f64,    // Initial step size
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

    /// Check Armijo condition: f(x + α*p) ≤ f(x) + c1*α*∇f(x)ᵀp
    fn armijo_condition(
        &self,
        f0: f64,
        f_alpha: f64,
        alpha: f64,
        directional_derivative: f64,
    ) -> bool {
        f_alpha <= f0 + self.config.c1 * alpha * directional_derivative
    }

    /// Check curvature condition: |∇f(x + α*p)ᵀp| ≤ c2*|∇f(x)ᵀp|
    fn curvature_condition(
        &self,
        grad_alpha_dot_p: f64,
        directional_derivative: f64,
    ) -> bool {
        grad_alpha_dot_p.abs() <= self.config.c2 * directional_derivative.abs()
    }

    /// Zoom phase of Strong Wolfe line search
    fn zoom(
        &self,
        alpha_lo: f64,
        alpha_hi: f64,
        f0: f64,
        directional_derivative: f64,
        current_point: &[f64],
        direction: &[f64],
        objective_fn: &dyn Fn(&[f64]) -> Result<f64>,
        gradient_fn: &dyn Fn(&[f64]) -> Result<Vec<f64>>,
    ) -> Result<(f64, usize, usize)> {
        let mut alpha_lo = alpha_lo;
        let mut alpha_hi = alpha_hi;
        let mut f_evals = 0;
        let mut g_evals = 0;

        for _ in 0..self.config.max_iterations {
            // Interpolate to find new trial point
            let alpha_j = 0.5 * (alpha_lo + alpha_hi); // Simple bisection
            
            // Evaluate function at trial point
            let trial_point: Vec<f64> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(x, d)| x + alpha_j * d)
                .collect();

            let f_alpha_j = objective_fn(&trial_point)?;
            f_evals += 1;

            // Check Armijo condition
            if !self.armijo_condition(f0, f_alpha_j, alpha_j, directional_derivative) {
                alpha_hi = alpha_j;
                continue;
            }

            // Evaluate gradient at trial point
            let grad_alpha_j = gradient_fn(&trial_point)?;
            g_evals += 1;

            let grad_alpha_j_dot_p = dot_product_f64(&grad_alpha_j, direction)?;

            // Check curvature condition
            if self.curvature_condition(grad_alpha_j_dot_p, directional_derivative) {
                return Ok((alpha_j, f_evals, g_evals));
            }

            // Update interval
            if grad_alpha_j_dot_p * (alpha_hi - alpha_lo) >= 0.0 {
                alpha_hi = alpha_lo;
            }
            alpha_lo = alpha_j;
        }

        // Return best point found
        Ok((alpha_lo, f_evals, g_evals))
    }
}

impl LineSearch for StrongWolfeLineSearch {
    fn search(
        &mut self,
        current_point: &[f64],
        direction: &[f64],
        current_value: f64,
        current_gradient: &[f64],
        objective_fn: &dyn Fn(&[f64]) -> Result<f64>,
        gradient_fn: &dyn Fn(&[f64]) -> Result<Vec<f64>>,
    ) -> Result<LineSearchResult> {
        // Check that direction is a descent direction
        let directional_derivative = dot_product_f64(current_gradient, direction)?;
        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut alpha_prev = 0.0;
        let mut f_prev = current_value;
        let mut f_evals = 0;
        let mut g_evals = 0;

        for i in 0..self.config.max_iterations {
            // Evaluate function at current step size
            let trial_point: Vec<f64> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(x, d)| x + alpha * d)
                .collect();

            let f_alpha = objective_fn(&trial_point)?;
            f_evals += 1;

            // Check Armijo condition and sufficient decrease
            if !self.armijo_condition(current_value, f_alpha, alpha, directional_derivative) 
                || (i > 0 && f_alpha >= f_prev) {
                // Zoom between alpha_prev and alpha
                let (final_alpha, zoom_f_evals, zoom_g_evals) = self.zoom(
                    alpha_prev,
                    alpha,
                    current_value,
                    directional_derivative,
                    current_point,
                    direction,
                    objective_fn,
                    gradient_fn,
                )?;
                
                return Ok(LineSearchResult {
                    step_size: final_alpha,
                    function_evaluations: f_evals + zoom_f_evals,
                    gradient_evaluations: g_evals + zoom_g_evals,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Evaluate gradient at current point
            let grad_alpha = gradient_fn(&trial_point)?;
            g_evals += 1;

            let grad_alpha_dot_p = dot_product_f64(&grad_alpha, direction)?;

            // Check curvature condition
            if self.curvature_condition(grad_alpha_dot_p, directional_derivative) {
                return Ok(LineSearchResult {
                    step_size: alpha,
                    function_evaluations: f_evals,
                    gradient_evaluations: g_evals,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                });
            }

            // Check if gradient indicates we should look further
            if grad_alpha_dot_p >= 0.0 {
                let (final_alpha, zoom_f_evals, zoom_g_evals) = self.zoom(
                    alpha,
                    alpha_prev,
                    current_value,
                    directional_derivative,
                    current_point,
                    direction,
                    objective_fn,
                    gradient_fn,
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

            if alpha < self.config.min_step {
                break;
            }
        }

        // Line search failed to converge
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
    pub c1: f64,              // Armijo condition parameter
    pub rho: f64,             // Backtracking factor (0 < rho < 1)
    pub max_iterations: usize, // Maximum backtracking iterations
    pub min_step: f64,        // Minimum step size
    pub initial_step: f64,    // Initial step size
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
    fn search(
        &mut self,
        current_point: &[f64],
        direction: &[f64],
        current_value: f64,
        current_gradient: &[f64],
        objective_fn: &dyn Fn(&[f64]) -> Result<f64>,
        _gradient_fn: &dyn Fn(&[f64]) -> Result<Vec<f64>>,
    ) -> Result<LineSearchResult> {
        // Check that direction is a descent direction
       let directional_derivative = dot_product_f64(current_gradient, direction)?;
        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut f_evals = 0;

        for _ in 0..self.config.max_iterations {
            // Evaluate function at current step size
            let trial_point: Vec<f64> = current_point
                .iter()
                .zip(direction.iter())
                .map(|(x, d)| x + alpha * d)
                .collect();

            let f_alpha = objective_fn(&trial_point)?;
            f_evals += 1;

            // Check Armijo condition
            let armijo_threshold = current_value + self.config.c1 * alpha * directional_derivative;
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn quadratic_function(x: &[f64]) -> Result<f64> {
        // f(x) = 0.5 * x^T * x (simple quadratic)
        Ok(0.5 * x.iter().map(|xi| xi * xi).sum::<f64>())
    }

    fn quadratic_gradient(x: &[f64]) -> Result<Vec<f64>> {
        // ∇f(x) = x
        Ok(x.to_vec())
    }

    #[test]
    fn test_strong_wolfe_quadratic() {
        let mut line_search = StrongWolfeLineSearch::new(StrongWolfeConfig::default());
        
        let current_point = vec![2.0, 3.0];
        let direction = vec![-2.0, -3.0]; // Negative gradient (descent direction)
        let current_value = quadratic_function(&current_point).unwrap();
        let current_gradient = quadratic_gradient(&current_point).unwrap();

        let result = line_search.search(
            &current_point,
            &direction,
            current_value,
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient,
        ).unwrap();

        assert!(result.success);
        assert!(result.step_size > 0.0);
        assert!(result.function_evaluations > 0);
        
        // For quadratic function, optimal step should be 1.0
        assert_relative_eq!(result.step_size, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_backtracking_quadratic() {
        let mut line_search = BacktrackingLineSearch::new(BacktrackingConfig::default());
        
        let current_point = vec![1.0, 1.0];
        let direction = vec![-1.0, -1.0]; // Negative gradient
        let current_value = quadratic_function(&current_point).unwrap();
        let current_gradient = quadratic_gradient(&current_point).unwrap();

        let result = line_search.search(
            &current_point,
            &direction,
            current_value,
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient,
        ).unwrap();

        assert!(result.success);
        assert!(result.step_size > 0.0);
        assert!(result.function_evaluations > 0);
    }

    #[test]
    fn test_non_descent_direction() {
        let mut line_search = StrongWolfeLineSearch::new(StrongWolfeConfig::default());
        
        let current_point = vec![1.0, 1.0];
        let direction = vec![1.0, 1.0]; // Positive gradient (ascent direction)
        let current_value = quadratic_function(&current_point).unwrap();
        let current_gradient = quadratic_gradient(&current_point).unwrap();

        let result = line_search.search(
            &current_point,
            &direction,
            current_value,
            &current_gradient,
            &quadratic_function,
            &quadratic_gradient,
        );

        assert!(result.is_err());
    }
}