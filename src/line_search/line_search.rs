#![allow(clippy::type_complexity)]

use anyhow::Result;
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Line search result containing step size and evaluation counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchResult {
    pub step_size: f32,
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
    pub c1: f32,
    pub c2: f32,
    pub max_iterations: usize,
    pub initial_step: f32,
    pub min_step: f32,
    pub max_step: f32,
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
            c2: 0.1,
            c1: 1e-8,
            max_iterations: 5,
            initial_step: 1.0,
            min_step: 1e-8,
            max_step: 100.0,
            verbose: false,
            line_bracket_method: 1, // Default to gradient-based bracketing
        }
    }
}
/// Create a line search algorithm from configuration
pub fn create_line_search<S: Shape>(_config: LineSearchConfig) -> Box<dyn LineSearch<S>> {
    // Implementations will be restored in subsequent tasks
    unimplemented!("Line search implementations are being migrated to Luminal");
}

/// Trait for line search algorithms
pub trait LineSearch<S: Shape>: Send + Sync + Debug {
    /// Perform 1D line search optimization
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
    ) -> Result<LineSearchResult>;

    /// Reset internal state
    fn reset(&mut self);
    /// Clone the line search algorithm
    fn clone_box(&self) -> Box<dyn LineSearch<S>>;
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
        };
        // Test serialization
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"step_size\":0.5"));
        // Test deserialization
        let deserialized: LineSearchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.step_size, result.step_size);
    }
}