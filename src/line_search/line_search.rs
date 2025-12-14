#![allow(clippy::type_complexity)]

use anyhow::Result;
use dfdx::prelude::{ConstShape, Shape};
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Context for line search containing all necessary graph tensors
/// This allows the line search to re-evaluate the objective and gradient
/// at different points along the search direction
#[derive(Debug, Clone)]
pub struct LineSearchContext {
    /// The parameter tensors that will be updated
    pub params: GraphTensor,
    /// The loss/objective tensor
    pub loss: GraphTensor,
    /// The gradient tensors (computed via Autograd)
    pub gradient: GraphTensor,
}

/// Line search result containing step size and evaluation counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchResult {
    pub step_size: f32,
    pub success: bool,
    pub termination_reason: TerminationReason,
    /// Number of function evaluations performed
    pub num_f_evals: usize,
    /// Number of gradient evaluations performed
    pub num_g_evals: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TerminationReason {
    WolfeConditionsSatisfied,
    ArmijoConditionSatisfied,
    MaxIterationsReached,
    StepSizeTooSmall,
    FunctionEvaluationError,
    InvalidDirection,
    /// Curvature condition satisfied (for strong Wolfe)
    CurvatureConditionSatisfied,
    /// Exact minimum found (for exact line search)
    ExactMinimumFound,
}

/// General line search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchConfig {
    pub method: LineSearchMethod,
    pub c1: f32,
    pub c2: f32,
    pub max_iterations: usize,
    pub initial_step: f32,
    pub min_step: f32,
    pub max_step: f32,
    pub verbose: bool,           // Enable verbose logging
    pub line_bracket_method: u8, // 1: gradient-based bracketing, 2: function-value-based bracketing
    /// Tolerance for exact line search methods
    pub exact_tolerance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineSearchMethod {
    StrongWolfe,
    Backtracking,
    Bisection,
    GoldenSection,
    MoreThuente,
    CubicQuadraticInterpolation,
    /// Exact line search using gradient information
    Exact,
    /// Parametric curve search (for curved search paths)
    ParametricCurve,
}

impl Default for LineSearchConfig {
    fn default() -> Self {
        Self {
            method: LineSearchMethod::StrongWolfe,
            c2: 0.1,
            c1: 1e-8,
            max_iterations: 5,
            initial_step: 1.0,
            min_step: 1e-8,
            max_step: 100.0,
            verbose: false,
            line_bracket_method: 1, // Default to gradient-based bracketing
            exact_tolerance: 1e-10,
        }
    }
}

/// Create a line search algorithm from configuration
pub fn create_line_search<S: ConstShape>(_config: LineSearchConfig) -> Box<dyn LineSearch> {
    // Implementations will be restored in subsequent tasks
    unimplemented!("Line search implementations are being migrated to Luminal");
}

/// Trait for line search algorithms
pub trait LineSearch: Send + Sync + Debug {
    /// Perform 1D line search optimization
    ///
    /// The line search can re-execute the graph to evaluate the objective
    /// and gradient at different step sizes. This is critical for exact
    /// line search methods.
    ///
    /// # Arguments
    /// * `cx` - The compute graph (will be executed multiple times)
    /// * `params` - Parameter tensor to update for evaluation
    /// * `loss` - Loss tensor to retrieve after execution
    /// * `gradient` - Gradient tensor to retrieve after execution
    /// * `current_params` - Current parameter values
    /// * `direction` - Search direction
    /// * `initial_loss` - Loss at current_params (step=0)
    /// * `initial_gradient` - Gradient at current_params (step=0)
    ///
    /// # Returns
    /// LineSearchResult with optimal step size found
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
    ) -> Result<LineSearchResult>;
    /// Evaluate the objective function at a given step size
    ///
    /// This helper method sets parameters to `current + step * direction`,
    /// executes the graph, and returns the loss value.
    fn evaluate_at_step(
        &self,
        cx: &mut Graph,
        params: GraphTensor,
        loss: GraphTensor,
        current_params: &[f32],
        direction: &[f32],
        step: f32,
    ) -> Result<f32> {
        let candidate_params: Vec<f32> = current_params
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + step * d)
            .collect();
        cx.set_tensor(params.id, 0, Tensor::new(candidate_params));
        cx.execute();
        let f_val = loss
            .data()
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast loss data"))?[0];
        Ok(f_val)
    }
    /// Evaluate both objective and gradient at a given step size
    ///
    /// This is more efficient than separate calls when both are needed.
    fn evaluate_with_gradient(
        &self,
        cx: &mut Graph,
        params: GraphTensor,
        loss: GraphTensor,
        gradient: GraphTensor,
        current_params: &[f32],
        direction: &[f32],
        step: f32,
    ) -> Result<(f32, Vec<f32>)> {
        let candidate_params: Vec<f32> = current_params
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + step * d)
            .collect();
        cx.set_tensor(params.id, 0, Tensor::new(candidate_params));
        cx.execute();
        // Get loss
        let f_val = loss
            .data()
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast loss data"))?[0];
        // Get gradient
        let grad_data = gradient
            .data()
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast gradient data"))?
            .clone();
        Ok((f_val, grad_data))
    }
    /// Compute directional derivative at a given step size
    ///
    /// Returns g(step)^T * direction where g is the gradient at that step
    fn directional_derivative_at_step(
        &self,
        cx: &mut Graph,
        params: GraphTensor,
        gradient: GraphTensor,
        current_params: &[f32],
        direction: &[f32],
        step: f32,
    ) -> Result<f32> {
        let candidate_params: Vec<f32> = current_params
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + step * d)
            .collect();
        cx.set_tensor(params.id, 0, Tensor::new(candidate_params));
        cx.execute();
        let grad_binding = gradient.data();
        let grad_data = grad_binding
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast gradient data"))?;
        let deriv: f32 = grad_data
            .iter()
            .zip(direction.iter())
            .map(|(g, d)| g * d)
            .sum();
        Ok(deriv)
    }

    /// Reset internal state
    fn reset(&mut self);
    /// Clone the line search algorithm
    fn clone_box(&self) -> Box<dyn LineSearch>;
    /// Get as Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_search_result_serialization() {
        use serde_json;
        let result = LineSearchResult {
            step_size: 0.5,
            success: true,
            termination_reason: TerminationReason::WolfeConditionsSatisfied,
            num_f_evals: 3,
            num_g_evals: 2,
        };
        // Test serialization
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"step_size\":0.5"));
        // Test deserialization
        let deserialized: LineSearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.step_size, result.step_size);
        assert_eq!(deserialized.num_f_evals, 3);
    }
}