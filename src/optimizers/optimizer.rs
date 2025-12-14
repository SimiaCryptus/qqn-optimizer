//! Core optimizer traits and types for the QQN research framework.
//!
//! This module defines the fundamental abstractions that all optimization algorithms
//! must implement, along with supporting types for tracking optimization progress
//! and convergence behavior.

use dfdx::prelude::Shape;
use dfdx::shapes::ConstShape;
use luminal::prelude::*;
use luminal_training::Autograd;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::time::Duration;

/// Context for gradient computation and re-evaluation
/// This struct holds all the tensors needed to compute and retrieve
/// gradients, allowing for repeated evaluation during line search
#[derive(Debug, Clone)]
pub struct GradientContext {
    /// The weight/parameter tensors
    pub weights: Vec<GraphTensor>,
    /// The gradient tensors (one per weight tensor)
    pub gradients: Vec<GraphTensor>,
    /// The loss tensor
    pub loss: GraphTensor,
    /// Input tensor (for setting new data)
    pub input: Option<NodeIndex>,
    /// Target tensor (for supervised learning)
    pub target: Option<NodeIndex>,
}
impl GradientContext {
    /// Create a new gradient context
    pub fn new(weights: Vec<GraphTensor>, gradients: Vec<GraphTensor>, loss: GraphTensor) -> Self {
        Self {
            weights,
            gradients,
            loss,
            input: None,
            target: None,
        }
    }
    /// Set the input tensor index
    pub fn with_input(mut self, input: NodeIndex) -> Self {
        self.input = Some(input);
        self
    }
    /// Set the target tensor index
    pub fn with_target(mut self, target: NodeIndex) -> Self {
        self.target = Some(target);
        self
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

/// Result of setting up an optimizer on a graph
/// Contains the new weight tensors and any optimizer-specific state tensors
#[derive(Debug)]
pub struct OptimizerSetup {
    /// The new weight tensors after optimization step (to be transferred back to weights)
    pub new_weights: Vec<GraphTensor>,
    /// Learning rate tensor (if applicable)
    pub learning_rate: Option<GraphTensor>,
    /// Any additional state tensors that need to be kept across iterations
    pub state_tensors: Vec<NodeIndex>,
    /// Gradient context for re-evaluation during line search
    pub gradient_context: Option<GradientContext>,
}
impl OptimizerSetup {
    /// Create a new optimizer setup
    pub fn new(new_weights: Vec<GraphTensor>) -> Self {
        Self {
            new_weights,
            learning_rate: None,
            state_tensors: Vec::new(),
            gradient_context: None,
        }
    }
    /// Set the learning rate tensor
    pub fn with_learning_rate(mut self, lr: GraphTensor) -> Self {
        self.learning_rate = Some(lr);
        self
    }
    /// Add state tensors to keep
    pub fn with_state_tensors(mut self, tensors: Vec<NodeIndex>) -> Self {
        self.state_tensors = tensors;
        self
    }
    /// Set the gradient context
    pub fn with_gradient_context(mut self, ctx: GradientContext) -> Self {
        self.gradient_context = Some(ctx);
        self
    }
}

/// Additional metadata that optimizers can provide
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationMetadata {
    /// Optimizer-specific data (e.g., QQN magnitude ratios, L-BFGS curvature info)
    pub optimizer_data: std::collections::HashMap<String, f32>,
    /// Timing information for different phases of the step
    pub timing_info: TimingInfo,
    /// Memory usage information
    pub memory_info: MemoryInfo,
}

/// Result of a complete optimization run
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Final function value
    pub fx: f32,
    /// Number of function evaluations
    pub num_f_evals: usize,
    /// Number of gradient evaluations  
    pub num_g_evals: usize,
    /// Whether optimization converged
    pub converged: bool,
    /// Final parameters
    pub x: Vec<f32>,
    /// History of loss values (if tracked)
    pub loss_history: Option<Vec<f32>>,
    /// History of gradient norms (if tracked)
    pub gradient_norm_history: Option<Vec<f32>>,
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

    /// Set up the optimizer on the graph.
    ///
    /// This method adds the optimization update operations to the graph.
    /// It takes the current weights and their gradients (computed via Autograd)
    /// and returns new weight tensors representing the updated values.
    ///
    /// The returned `OptimizerSetup` includes a `GradientContext` that allows
    /// the line search to re-evaluate the objective and gradient at different
    /// step sizes by re-executing the graph.
    ///
    /// # Arguments
    /// * `graph` - The compute graph
    /// * `weights` - The current weight tensors
    /// * `gradients` - The gradient tensors (computed via Autograd::new(&weights, loss))
    ///
    /// # Returns
    /// An `OptimizerSetup` containing new weight tensors and any state tensors
    fn setup_on_graph(
        &mut self,
        graph: &mut Graph,
        weights: &[GraphTensor],
        gradients: &[GraphTensor],
    ) -> OptimizerSetup;
    /// Set up the optimizer with full gradient context for line search.
    ///
    /// This extended setup method provides the loss tensor, enabling the
    /// optimizer to create a complete GradientContext for exact line search.
    ///
    /// # Arguments
    /// * `graph` - The compute graph
    /// * `weights` - The current weight tensors
    /// * `gradients` - The gradient tensors
    /// * `loss` - The loss tensor for re-evaluation
    ///
    /// # Returns
    /// An `OptimizerSetup` with gradient context for line search
    fn setup_with_line_search(
        &mut self,
        graph: &mut Graph,
        weights: &[GraphTensor],
        gradients: &[GraphTensor],
        loss: GraphTensor,
    ) -> OptimizerSetup {
        let mut setup = self.setup_on_graph(graph, weights, gradients);
        // Create gradient context for line search
        let grad_ctx = GradientContext::new(weights.to_vec(), gradients.to_vec(), loss);
        setup.gradient_context = Some(grad_ctx);
        setup
    }

    /// Compute gradients for the given weights and loss using Autograd.
    ///
    /// This is a convenience method that wraps Luminal's Autograd.
    /// Override this if you need custom gradient computation.
    fn compute_gradients(
        &self,
        graph: &mut Graph,
        weights: &[GraphTensor],
        loss: GraphTensor,
    ) -> Vec<GraphTensor> {
        // Use Luminal's Autograd to compute gradients
        let weight_refs: Vec<_> = weights.iter().collect();
        graph.compile(Autograd::new(&weight_refs, loss), ())
    }
    /// Evaluate the objective function at given parameters.
    ///
    /// This method sets the parameters, executes the graph, and returns
    /// the loss value. Used by line search methods.
    fn evaluate_objective(
        &self,
        graph: &mut Graph,
        weights: &[GraphTensor],
        loss: GraphTensor,
        params: &[Vec<f32>],
    ) -> anyhow::Result<f32> {
        // Set parameters
        for (w, p) in weights.iter().zip(params.iter()) {
            graph.set_tensor(w.id, 0, Tensor::new(p.clone()));
        }
        // Execute graph
        graph.execute();
        // Get loss value
        let loss_tensor = graph
            .get_tensor(loss.id, 0)
            .ok_or_else(|| anyhow::anyhow!("Failed to get loss tensor"))?;
        let f_val = loss_tensor
            .data
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast loss data"))?[0];
        Ok(f_val)
    }
    /// Evaluate objective and gradients at given parameters.
    ///
    /// This method sets the parameters, executes the graph, and returns
    /// both the loss value and gradient values. Used by line search methods
    /// that need gradient information.
    fn evaluate_with_gradients(
        &self,
        graph: &mut Graph,
        weights: &[GraphTensor],
        gradients: &[GraphTensor],
        loss: GraphTensor,
        params: &[Vec<f32>],
    ) -> anyhow::Result<(f32, Vec<Vec<f32>>)> {
        // Set parameters
        for (w, p) in weights.iter().zip(params.iter()) {
            graph.set_tensor(w.id, 0, Tensor::new(p.clone()));
        }
        // Execute graph
        graph.execute();
        // Get loss value
        let loss_tensor = graph
            .get_tensor(loss.id, 0)
            .ok_or_else(|| anyhow::anyhow!("Failed to get loss tensor"))?;
        let f_val = loss_tensor
            .data
            .as_any()
            .downcast_ref::<Vec<f32>>()
            .ok_or_else(|| anyhow::anyhow!("Failed to downcast loss data"))?[0];
        // Get gradient values
        let mut grad_vals = Vec::with_capacity(gradients.len());
        for g in gradients {
            let grad_tensor = graph
                .get_tensor(g.id, 0)
                .ok_or_else(|| anyhow::anyhow!("Failed to get gradient tensor"))?;
            let g_data = grad_tensor
                .data
                .as_any()
                .downcast_ref::<Vec<f32>>()
                .ok_or_else(|| anyhow::anyhow!("Failed to downcast gradient data"))?
                .clone();
            grad_vals.push(g_data);
        }
        Ok((f_val, grad_vals))
    }

    /// Reset the optimizer state (useful for multiple runs)
    fn reset(&mut self);

    /// Get the name of this optimizer (for reporting and analysis)
    fn name(&self) -> &str;

    /// Check if the optimizer has converged based on its internal criteria
    fn has_converged(&self) -> bool {
        false // Default implementation - most optimizers don't track convergence internally
    }

    /// Get the current iteration number
    fn iteration(&self) -> usize;
    /// Increment the iteration counter
    fn increment_iteration(&mut self);

    /// Get the stagnation multiplier for relaxed convergence criteria
    /// This multiplier is applied to tolerance values to make convergence less strict
    fn stagnation_multiplier(&self) -> f32 {
        1.0 // Default multiplier - no relaxation
    }

    /// Get the stagnation count threshold for applying relaxed convergence
    /// When stagnation is detected for this many iterations, relaxed criteria are used
    fn stagnation_count(&self) -> usize {
        1 // Default count - apply relaxation after 1 iteration of stagnation
    }

    /// Set the stagnation multiplier (mutable)
    fn set_stagnation_multiplier(&mut self, multiplier: f32);

    /// Set the stagnation count threshold (mutable)
    fn set_stagnation_count(&mut self, count: usize);
    /// Get the learning rate (if applicable)
    fn learning_rate(&self) -> Option<f32> {
        None
    }
    /// Set the learning rate (if applicable)
    fn set_learning_rate(&mut self, _lr: f32) {
        // Default: no-op for optimizers without configurable learning rate
    }
}

/// Result of a single optimization step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step size used in this iteration
    pub step_size: f32,

    /// Information about convergence status
    pub convergence_info: ConvergenceInfo,

    /// Additional optimizer-specific metadata
    pub metadata: OptimizationMetadata,
}

/// Information about convergence status and criteria
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvergenceInfo {
    /// Whether the optimizer has converged
    pub converged: bool,

    /// Change in function value from previous iteration
    pub function_change: Option<f32>,
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
    pub fn with_function_change(mut self, change: f32) -> Self {
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

/// Additional metadata that optimizers can provide
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
}
