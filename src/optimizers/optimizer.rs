//! Core optimizer traits and types for the QQN research framework.
//!
//! This module defines the fundamental abstractions that all optimization algorithms
//! must implement, along with supporting types for tracking optimization progress
//! and convergence behavior.

use log::error;
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::time::Duration;

/// Context for gradient computation and re-evaluation
/// This struct holds all the tensors needed to compute and retrieve
/// gradients, allowing for repeated evaluation during line search
#[derive(Debug, Clone)]
pub struct OptimizationContext {
    /// The weight/parameter tensors
    pub weights: Vec<GraphTensor>,
    /// The gradient tensors (one per weight tensor)
    pub gradients: Vec<GraphTensor>,
    /// The loss tensor
    pub loss: GraphTensor,
}

impl OptimizationContext {
    /// Create a new gradient context
    pub fn new(weights: Vec<GraphTensor>, gradients: Vec<GraphTensor>, loss: GraphTensor) -> Self {
        loss.retrieve();
        for grad in gradients.iter() {
            grad.retrieve();
        }
        weights.retrieve();
        loss.graph().compile(
            <()>::default(),
            (
                weights.clone(),
                loss,
                gradients.clone()
            ),
        );
        Self {
            weights,
            gradients,
            loss,
        }
    }

    pub fn graph(&self) -> &mut Graph {
        self.loss.graph()
    }
    pub(crate) fn write_weights(&mut self, all_weights_data: &mut Vec<Vec<f32>>) {
        // Clear all current tensor entries to prepare for updates
        self.graph().tensors.clear();
        for i in 0..self.weights.len() {
            let w_vec = &mut all_weights_data[i];
            // Write back to graph tensor
            self.graph()
                .tensors
                .insert((self.weights[i].id, 0), Tensor::new(w_vec.clone()));
        }
    }
}

/// A wrapper around GraphTensor that implements Send and Sync.
/// This is necessary because GraphTensor contains a raw pointer to the Graph,
/// which is !Send and !Sync. We assert safety because the Optimizer is typically
/// moved to a thread before the Graph is populated or used, and once running,
/// it stays on that thread.
#[derive(Debug, Clone, Copy)]
pub struct SafeTensor(pub GraphTensor);
unsafe impl Send for SafeTensor {}
unsafe impl Sync for SafeTensor {}
impl std::ops::Deref for SafeTensor {
    type Target = GraphTensor;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<GraphTensor> for SafeTensor {
    fn from(t: GraphTensor) -> Self {
        SafeTensor(t)
    }
}

/// Additional metadata that optimizers can provide
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationMetadata {
    /// Optimizer-specific data (e.g., QQN magnitude ratios, L-BFGS curvature info)
    pub optimizer_data: std::collections::HashMap<String, f64>,
    /// Timing information for different phases of the step
    pub timing_info: TimingInfo,
    /// Memory usage information
    pub memory_info: MemoryInfo,
}

/// Result of a complete optimization run
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Final function value
    pub fx: f64,
    /// Number of function evaluations
    pub num_f_evals: usize,
    /// Number of gradient evaluations  
    pub num_g_evals: usize,
    /// Whether optimization converged
    pub converged: bool,
    /// Final parameters
    pub x: Vec<f64>,
    /// History of loss values (if tracked)
    pub loss_history: Option<Vec<f64>>,
    /// History of gradient norms (if tracked)
    pub gradient_norm_history: Option<Vec<f64>>,
}

/// Core trait that all optimization algorithms must implement.
///
/// This trait provides a unified interface for different optimization methods,
/// enabling easy benchmarking and comparison between algorithms.
///
/// The optimizer works with Luminal's graph-based computation model:
/// 1. `setup_on_graph` adds optimization operations to the graph
/// 2. Gradients are computed externally using `Autograd`
/// 3. The optimizer uses gradients to compute new weight values
///
/// # Gradient Network Tracking
/// The gradient network is constructed separately using Luminal's Autograd.
/// The optimizer receives gradient tensors and can re-execute the graph
/// to recompute loss and gradients at different parameter values.
/// This is critical for exact line search methods.
pub trait Optimizer: Debug + Send + Sync + 'static {
    /// Clone the optimizer (required for trait object safety)
    fn clone_box(&self) -> Box<dyn Optimizer>;

    /// Get optimizer configuration as a string for serialization
    fn config_string(&self) -> String {
        format!("{self:?}")
    }
    /// Perform a single optimization step
    fn step(&mut self, params: &mut OptimizationContext) -> StepResult {
        error!(
            "step_on_graph not implemented for optimizer: {}",
            self.name()
        );
        StepResult {
            step_size: self.learning_rate().unwrap_or(1.0),
            convergence_info: ConvergenceInfo::default(),
        }
    }

    /// Reset the optimizer state (useful for multiple runs)
    fn reset(&mut self);

    /// Get the name of this optimizer (for reporting and analysis)
    fn name(&self) -> &str;

    /// Check if the optimizer has converged based on its internal criteria
    fn has_converged(&self) -> bool {
        false // Default implementation - most optimizers don't track convergence internally
    }

    /// Get the stagnation multiplier for relaxed convergence criteria
    /// This multiplier is applied to tolerance values to make convergence less strict
    fn stagnation_multiplier(&self) -> f64 {
        1.0 // Default multiplier - no relaxation
    }

    /// Get the stagnation count threshold for applying relaxed convergence
    /// When stagnation is detected for this many iterations, relaxed criteria are used
    fn stagnation_count(&self) -> usize {
        1 // Default count - apply relaxation after 1 iteration of stagnation
    }

    /// Set the stagnation multiplier (mutable)
    fn set_stagnation_multiplier(&mut self, multiplier: f64);

    /// Set the stagnation count threshold (mutable)
    fn set_stagnation_count(&mut self, count: usize);
    /// Get the learning rate (if applicable)
    fn learning_rate(&self) -> Option<f64> {
        None
    }
    /// Set the learning rate (if applicable)
    fn set_learning_rate(&mut self, _lr: f64) {
        // Default: no-op for optimizers without configurable learning rate
    }
}

/// Result of a single optimization step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step size used in this iteration
    pub step_size: f64,

    /// Information about convergence status
    pub convergence_info: ConvergenceInfo,
}

/// Information about convergence status and criteria
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvergenceInfo {
    /// Whether the optimizer has converged
    pub converged: bool,

    /// Change in function value from previous iteration
    pub function_change: Option<f64>,
}

impl ConvergenceInfo {
    /// Create convergence info indicating convergence
    pub fn converged() -> Self {
        Self {
            converged: true,
            ..Default::default()
        }
    }

    /// Update convergence status based on function change
    pub fn with_function_change(mut self, change: f64) -> Self {
        self.function_change = Some(change);
        self
    }
}

/// Different convergence criteria that can be satisfied
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConvergenceCriterion {
    /// Gradient norm below threshold
    GradientNorm,
    /// Function value change below threshold
    FunctionChange,
    /// Parameter change below threshold
    ParameterChange,
    /// Maximum iterations reached
    MaxIterations,
    /// Maximum time elapsed
    MaxTime,
    /// User-defined custom criterion
    Custom,
}

/// Timing information for optimization steps
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct TimingInfo {
    /// Time spent computing search direction
    pub direction_computation: Option<Duration>,

    /// Time spent in line search
    pub line_search: Option<Duration>,

    /// Time spent updating parameters
    pub parameter_update: Option<Duration>,

    /// Total step duration
    pub step_duration: Duration,
}

impl Default for TimingInfo {
    fn default() -> Self {
        Self {
            direction_computation: None,
            line_search: None,
            parameter_update: None,
            step_duration: Duration::from_secs(0),
        }
    }
}

/// Memory usage information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryInfo {
    /// Peak memory usage during this step (in bytes)
    pub peak_memory: Option<usize>,

    /// Memory allocated for optimizer state (in bytes)
    pub state_memory: Option<usize>,

    /// Memory allocated for temporary computations (in bytes)
    pub temp_memory: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_info_builder() {
        let info = ConvergenceInfo::default().with_function_change(1e-10);

        assert_eq!(info.function_change, Some(1e-10));
    }
    #[test]
    fn test_convergence_info_static() {
        let info = ConvergenceInfo::converged();
        assert!(info.converged);
        assert!(info.function_change.is_none());
    }
    #[test]
    fn test_timing_info_default() {
        let info = TimingInfo::default();
        assert_eq!(info.step_duration, Duration::from_secs(0));
        assert!(info.direction_computation.is_none());
        assert!(info.line_search.is_none());
        assert!(info.parameter_update.is_none());
    }
    #[test]
    fn test_memory_info_default() {
        let info = MemoryInfo::default();
        assert!(info.peak_memory.is_none());
        assert!(info.state_memory.is_none());
        assert!(info.temp_memory.is_none());
    }
}
