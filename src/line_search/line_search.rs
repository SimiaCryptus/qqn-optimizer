#![allow(clippy::type_complexity)]

use crate::line_search::{
    BacktrackingConfig, BacktrackingLineSearch, BisectionConfig, BisectionLineSearch,
    CubicQuadraticConfig, CubicQuadraticLineSearch, GoldenSectionConfig, GoldenSectionLineSearch,
    MoreThuenteConfig, MoreThuenteLineSearch, StrongWolfeConfig, StrongWolfeLineSearch,
};
use crate::optimizers::optimizer::OptimizationContext;
use anyhow::Result;
use dfdx::prelude::{ConstShape, Shape};
use itertools::Itertools;
use luminal::graph::Graph;
use luminal::prelude::{Data, Tensor, ToShape};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use crate::optimizers::{GDConfig, GDOptimizer};
use crate::region::trust_region::{TrustRegion, TrustRegionConfig, TrustRegionOptimizer};

/// Line search result containing step size and evaluation counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchResult {
    pub step_size: f64,
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
    pub c1: f64,
    pub c2: f64,
    pub max_iterations: usize,
    pub initial_step: f64,
    pub min_step: f64,
    pub max_step: f64,
    pub verbose: bool,           // Enable verbose logging
    pub line_bracket_method: u8, // 1: gradient-based bracketing, 2: function-value-based bracketing
    /// Tolerance for exact line search methods
    pub exact_tolerance: f64,
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
            c1: 1e-5,
            max_iterations: 5,
            initial_step: 1.0,
            min_step: 1e-5,
            max_step: 100.0,
            verbose: false,
            line_bracket_method: 1, // Default to gradient-based bracketing
            exact_tolerance: 1e-6,
        }
    }
}

/// Create a line search algorithm from configuration
pub fn create_line_search(config: LineSearchConfig) -> Box<dyn LineSearch> {
    if config.verbose {
        println!("Initializing Line Search: {:?}", config.method);
        println!("Configuration: {:#?}", config);
    }
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

fn unflatten_tensors(
    flat: &[f64],
    shapes: &[Vec<usize>],
) -> Result<Vec<Vec<f32>>> {
    let mut result = Vec::new();
    let mut offset = 0;
    for shape in shapes {
        let size: usize = shape.iter().product();
        if offset + size > flat.len() {
            return Err(anyhow::anyhow!("Size mismatch in unflattening"));
        }
        let chunk = &flat[offset..offset + size];
        result.push(chunk.iter().map(|&x| x as f32).collect());
        offset += size;
    }
    Ok(result)
}

/// Trait for line search algorithms
pub trait LineSearch: Send + Sync + Debug {
    /// Perform 1D line search optimization
    ///
    /// The line search can re-execute the graph to evaluate the objective
    /// and gradient at different step sizes. This is critical for exact
    /// line search methods. The graph's resulting state after execution should
    /// correspond to the parameters at the optimal step size found.
    ///
    /// # Arguments
    /// * `cx` - The compute graph (will be executed multiple times)
    /// * `context` - Gradient context containing weights, gradients, and loss
    /// * `current_params` - Current parameter values
    /// * `direction` - Search direction
    /// * `initial_loss` - Loss at current_params (step=0)
    /// * `initial_gradient` - Gradient at current_params (step=0)
    ///
    /// # Returns
    /// LineSearchResult with optimal step size found
    fn search(
        &mut self,
        context: OptimizationContext,
        current_params: &[f64],
        direction: &[f64],
        initial_loss: f64,
        initial_gradient: &[f64],
        trust_region: Option<&dyn TrustRegion>,
    ) -> Result<LineSearchResult>;
    /// Check if verbose logging is enabled
    fn is_verbose(&self) -> bool {
        false
    }


    /// Evaluate the objective function at a given step size
    ///
    /// This helper method sets parameters to `current + step * direction`,
    /// executes the graph, and returns the loss value.
    fn evaluate_at_step(
        &self,
        context: &mut OptimizationContext,
        current_params: &[f64],
        direction: &[f64],
        step: f64,
        trust_region: Option<&dyn TrustRegion>,
    ) -> Result<f64> {
        if self.is_verbose() {
            println!("LineSearch: Evaluating f(x + alpha * d) at alpha = {:.6e}", step);
        }
        let mut candidate_params: Vec<f64> = current_params
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + step * d)
            .collect();
        if let Some(region) = trust_region {
            region.project(&mut candidate_params);
        }


        let shapes = context.weights.iter().map(|w| w.shape.to_shape().iter().map(
            |&d| d.to_usize().unwrap()
        ).collect_vec()).collect::<Vec<_>>();
        
        let mut weights_data = unflatten_tensors(&candidate_params, &shapes)?;
        context.write_weights(&mut weights_data);

        context.graph().execute();
        let f_val = context
            .loss
            .data()
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast loss data"))?[0] as f64;
        if self.is_verbose() {
            println!("LineSearch: f(x + alpha * d) = {:.6e}", f_val);
        }
        Ok(f_val)
    }
    /// Evaluate both objective and gradient at a given step size
    ///
    /// This is more efficient than separate calls when both are needed.
    fn evaluate_with_gradient(
        &self,
        context: &mut OptimizationContext,
        current_params: &[f64],
        direction: &[f64],
        step: f64,
        trust_region: Option<&dyn TrustRegion>,
    ) -> Result<(f64, Vec<f64>)> {
        if self.is_verbose() {
            println!("LineSearch: Evaluating f and g at alpha = {:.6e}", step);
        }
        let mut candidate_params: Vec<f64> = current_params
            .iter()
            .zip(direction.iter())
            .map(|(x, d)| x + step * d)
            .collect();
        if let Some(region) = trust_region {
            region.project(&mut candidate_params);
        }


        let shapes = context.weights.iter().map(|w| w.shape.to_shape().iter().map(
            |&d| d.to_usize().unwrap()
        ).collect_vec()).collect::<Vec<_>>();
        
        let mut weights_data = unflatten_tensors(&candidate_params, &shapes)?;
        context.write_weights(&mut weights_data);

        context.graph().execute();
        // Get loss
        let f_val = context
            .loss
            .data()
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast loss data"))?[0] as f64;
        // Get gradient
        let mut grad_data = Vec::with_capacity(current_params.len());
        for tensor_data in &context.gradients.iter().map(|g| g.data()).collect_vec() {
            let g_data = tensor_data
                .as_any()
                .downcast_ref::<Vec<f32>>()
                .ok_or_else(|| anyhow::anyhow!("Failed to downcast gradient data"))?.iter()
                .map(|&v| v as f64).collect::<Vec<f64>>();
            grad_data.extend_from_slice(g_data.as_slice());
        }
        if self.is_verbose() {
            let grad_norm: f64 = grad_data.iter().map(|x| x * x).sum::<f64>().sqrt();
            println!("LineSearch: f = {:.6e}, |g| = {:.6e}", f_val, grad_norm);
        }

        Ok((f_val, grad_data))
    }

    /// Reset internal state
    fn reset(&mut self);
    /// Clone the line search algorithm
    fn clone_box(&self) -> Box<dyn LineSearch>;
    /// Get as Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
impl Clone for Box<dyn LineSearch> {
    fn clone(&self) -> Box<dyn LineSearch> {
        self.clone_box()
    }
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
    #[test]
    fn test_create_line_search_configurations() {
        // Test StrongWolfe
        let config = LineSearchConfig {
            method: LineSearchMethod::StrongWolfe,
            c1: 1e-4,
            c2: 0.9,
            ..Default::default()
        };
        let mut ls = create_line_search(config);
        assert!(ls
            .as_any_mut()
            .downcast_mut::<StrongWolfeLineSearch>()
            .is_some());
        // Test Backtracking
        let config = LineSearchConfig {
            method: LineSearchMethod::Backtracking,
            ..Default::default()
        };
        let mut ls = create_line_search(config);
        assert!(ls
            .as_any_mut()
            .downcast_mut::<BacktrackingLineSearch>()
            .is_some());
        // Test Bisection
        let config = LineSearchConfig {
            method: LineSearchMethod::Bisection,
            ..Default::default()
        };
        let mut ls = create_line_search(config);
        assert!(ls
            .as_any_mut()
            .downcast_mut::<BisectionLineSearch>()
            .is_some());
    }
}