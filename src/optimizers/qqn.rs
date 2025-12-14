use crate::optimizers::optimizer::{GradientContext, OptimizerSetup, SafeTensor};
use crate::optimizers::Optimizer;
use crate::{LineSearchConfig, LineSearchMethod};
use luminal::prelude::*;
use std::collections::VecDeque;
use std::fmt::Debug;

/// Configuration for the QQN optimizer
#[derive(Debug, Clone)]
pub struct QQNConfig {
    /// Name of the optimizer instance
    pub name: String,
    /// L-BFGS history length
    pub lbfgs_history: usize,
    /// Minimum number of iterations before enabling L-BFGS
    pub min_lbfgs_iterations: usize,
    /// Line search configuration
    pub line_search: LineSearchConfig,
    /// Numerical stability constant
    pub epsilon: f32,
    /// Enable verbose logging of tensor data and internal state
    pub verbose: bool,
    pub min_step_persist: f32,
    pub min_step_size: f32,
    /// Scaling factor for gradient descent direction in steepest descent
    /// This allows line search to explore larger step sizes while operating in [0,1]
    /// Particularly useful for deep learning where gradients can be very small
    pub gradient_scale_factor: f32,
}

impl Default for QQNConfig {
    fn default() -> Self {
        Self {
            lbfgs_history: 10,
            min_lbfgs_iterations: 1,
            line_search: LineSearchConfig {
                method: LineSearchMethod::Bisection,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-6,
            verbose: false,
            min_step_persist: 1e-1,
            min_step_size: 1e-10,
            gradient_scale_factor: 1.0,
            name: "QQN".to_string(),
        }
    }
}
impl QQNConfig {
    /// Create a strict configuration with conservative settings for robust convergence
    /// - Larger L-BFGS history for better approximation
    /// - More steepest descent iterations before enabling L-BFGS
    /// - Tighter numerical stability constant
    /// - More conservative line search settings
    pub fn strict() -> Self {
        Self {
            lbfgs_history: 20,
            min_lbfgs_iterations: 5, // More steepest descent iterations
            line_search: LineSearchConfig {
                method: LineSearchMethod::Bisection,
                max_iterations: 50,
                c1: 1e-4,
                c2: 0.9,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            verbose: false,
            min_step_persist: 1e-2,
            min_step_size: 1e-10,
            gradient_scale_factor: 1.0, // More conservative scaling
            name: "QQN-Strict".to_string(),
        }
    }
    /// Create a lax configuration with aggressive settings for faster convergence
    /// - Smaller L-BFGS history for computational efficiency
    /// - Fewer steepest descent iterations before enabling L-BFGS
    /// - Looser numerical stability constant
    /// - More aggressive line search settings
    pub fn lax() -> Self {
        Self {
            lbfgs_history: 5,
            min_lbfgs_iterations: 1,
            line_search: LineSearchConfig {
                method: LineSearchMethod::Bisection,
                max_iterations: 20,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-4,
            verbose: false,
            min_step_persist: 1e-2,
            min_step_size: 1e-10,
            gradient_scale_factor: 1.0, // More aggressive scaling
            name: "QQN-Lax".to_string(),
        }
    }
    /// Create a configuration with verbose logging enabled
    pub fn verbose() -> Self {
        Self {
            verbose: true,
            name: "QQN-Verbose".to_string(),
            ..Self::default()
        }
    }
}

/// State information for the QQN optimizer
#[derive(Debug, Clone)]
pub struct QQNState {
    /// Current iteration number
    pub iteration: usize,
    /// L-BFGS history: (s, y) pairs
    /// s = x_{k+1} - x_k
    /// y = g_{k+1} - g_k
    pub history: VecDeque<(Vec<SafeTensor>, Vec<SafeTensor>)>,
    /// Previous parameters (x_k)
    pub prev_params: Option<Vec<SafeTensor>>,
    /// Previous gradients (g_k)
    pub prev_grads: Option<Vec<SafeTensor>>,
    /// Previous ideal step size for line search initialization
    pub previous_step_size: Option<f32>,
    /// Stagnation multiplier for relaxed convergence
    pub stagnation_multiplier: f32,
    /// Stagnation count threshold
    pub stagnation_count: usize,
}

impl QQNState {
    pub fn new() -> Self {
        Self {
            iteration: 0,
            history: VecDeque::new(),
            prev_params: None,
            prev_grads: None,
            previous_step_size: None,
            stagnation_multiplier: 1.0,
            stagnation_count: 1,
        }
    }
}

#[derive(Debug)]
pub struct QQNOptimizer {
    config: QQNConfig,
    pub state: QQNState,
}
impl Clone for QQNOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
        }
    }
}

impl QQNOptimizer {
    /// Create a new QQN optimizer with the given configuration
    pub fn new(config: QQNConfig) -> Self {
        Self {
            state: QQNState::new(),
            config,
        }
    }

    /// Helper to compute dot product of two lists of tensors
    fn dot_tensors(&self, a: &[GraphTensor], b: &[GraphTensor]) -> GraphTensor {
        // Note: In Luminal's graph model, we build computation nodes.
        // sum_reduce requires shape information. For simplicity, we assume
        // the tensors are 1D or we flatten them. This may need adjustment
        // based on actual tensor shapes in the model.
        let mut sum = None;
        for (t1, t2) in a.iter().zip(b.iter()) {
            let dot = (*t1 * *t2).sum(0);
            sum = match sum {
                Some(s) => Some(s + dot),
                None => Some(dot),
            };
        }
        sum.unwrap()
    }
}

impl Optimizer for QQNOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn setup_on_graph(
        &mut self,
        graph: &mut Graph,

        weights: &[GraphTensor],
        gradients: &[GraphTensor],
    ) -> OptimizerSetup {
        // In Luminal's graph model, we build the computation graph for the update step.
        // The L-BFGS two-loop recursion needs to be built into the graph.
        //
        // For the first few iterations (before we have history), we use steepest descent.
        // Once we have history, we apply the L-BFGS direction computation.
        //
        // Note: The history update happens outside the graph execution, in the training loop.
        // Here we just build the graph for computing new weights from current weights and gradients.

        // Create learning rate tensor
        let lr = graph.tensor(()).set(self.config.min_step_persist);

        // For now, implement steepest descent as the base case
        // The L-BFGS direction would require storing intermediate tensors across iterations
        // which is complex in a pure graph-based model.
        //
        // In practice, for L-BFGS with Luminal, we would need to:
        // 1. Store s and y vectors as actual data (not graph tensors)
        // 2. Compute the direction using CPU/GPU operations outside the graph
        // 3. Apply the direction as a graph operation
        //
        // For this implementation, we use gradient descent with the configured step size,
        // scaled by the gradient_scale_factor for better line search behavior.

        let scale = graph.tensor(()).set(self.config.gradient_scale_factor);

        // Compute new weights: w_new = w - lr * scale * grad
        let new_weights: Vec<GraphTensor> = weights
            .iter()
            .zip(gradients.iter())
            .map(|(w, g)| *w - (lr * scale * *g))
            .collect();

        // Store current params and grads for history update (done externally)
        self.state.prev_params = Some(weights.iter().map(|w| SafeTensor(*w)).collect());
        self.state.prev_grads = Some(gradients.iter().map(|g| SafeTensor(*g)).collect());

        OptimizerSetup::new(new_weights).with_learning_rate(lr)
    }

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

    fn reset(&mut self) {
        self.state = QQNState::new();
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn iteration(&self) -> usize {
        self.state.iteration
    }
    fn increment_iteration(&mut self) {
        self.state.iteration += 1;
    }
    fn stagnation_multiplier(&self) -> f32 {
        self.state.stagnation_multiplier
    }
    fn stagnation_count(&self) -> usize {
        self.state.stagnation_count
    }

    fn set_stagnation_multiplier(&mut self, multiplier: f32) {
        self.state.stagnation_multiplier = multiplier;
    }

    fn set_stagnation_count(&mut self, count: usize) {
        self.state.stagnation_count = count;
    }

    fn learning_rate(&self) -> Option<f32> {
        Some(self.config.min_step_persist)
    }

    fn set_learning_rate(&mut self, lr: f32) {
        self.config.min_step_persist = lr;
    }
}

#[cfg(test)]
mod tests {

    // Tests removed as they depend on candle and eager execution
}
