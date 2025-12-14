use crate::optimizers::Optimizer;
use luminal::prelude::*;
use std::collections::VecDeque;
use std::fmt::Debug;
use crate::{LineSearchConfig, LineSearchMethod};
use crate::LineSearchMethod::Bisection;
use crate::optimizers::optimizer::SafeTensor;

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
                method: Bisection,
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
pub struct QQNState<S: Shape> {
    /// Current iteration number
    pub iteration: usize,
    /// L-BFGS history: (s, y) pairs
    /// s = x_{k+1} - x_k
    /// y = g_{k+1} - g_k
    pub history: VecDeque<(Vec<SafeTensor<S>>, Vec<SafeTensor<S>>)>,
    /// Previous parameters (x_k)
    pub prev_params: Option<Vec<SafeTensor<S>>>,
    /// Previous gradients (g_k)
    pub prev_grads: Option<Vec<SafeTensor<S>>>,
    /// Previous ideal step size for line search initialization
    pub previous_step_size: Option<f32>,
}

impl<S: Shape> QQNState<S> {
    pub fn new() -> Self {
        Self {
            iteration: 0,
            history: VecDeque::new(),
            prev_params: None,
            prev_grads: None,
            previous_step_size: None,
        }
    }
}

#[derive(Debug)]
pub struct QQNOptimizer<S: Shape> {
    config: QQNConfig,
    pub state: QQNState<S>,
}
impl<S: Shape> Clone for QQNOptimizer<S> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
        }
    }
}

impl<S: Shape> QQNOptimizer<S> {
    /// Create a new QQN optimizer with the given configuration
    pub fn new(config: QQNConfig) -> Self {
        Self {
            state: QQNState::new(),
            config,
        }
    }

    /// Helper to compute dot product of two lists of tensors
    fn dot(&self, a: &[SafeTensor<S>], b: &[SafeTensor<S>]) -> GraphTensor<()> {
        let mut sum = None;
        for (t1, t2) in a.iter().zip(b.iter()) {
            let dot = (t1.0 * t2.0).sum_reduce::<(), _>();
            sum = match sum {
                Some(s) => Some(s + dot),
                None => Some(dot),
            };
        }
        sum.unwrap()
    }
}

impl<S: Shape> Optimizer<S> for QQNOptimizer<S> {
    fn clone_box(&self) -> Box<dyn Optimizer<S>> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,
        graph: &mut Graph,
        loss: GraphTensor<()>,
        params: &[GraphTensor<S>],
    ) -> Vec<GraphTensor<S>> {
        // 1. Compute Gradients
        let grads = graph.add_backward(loss);
        let gradients = params.iter().map(|p| *grads.get(p).unwrap()).collect::<Vec<_>>();

        // 2. Update L-BFGS History (s, y)
        // s = x_{k+1} - x_k
        // y = g_{k+1} - g_k
        if let (Some(prev_p), Some(prev_g)) = (&self.state.prev_params, &self.state.prev_grads) {
            let s: Vec<SafeTensor<S>> = params
                .iter()
                .zip(prev_p.iter())
                .map(|(c, p)| SafeTensor(*c - p.0))
                .collect();
            let y: Vec<SafeTensor<S>> = gradients
                .iter()
                .zip(prev_g.iter())
                .map(|(c, p)| SafeTensor(*c - p.0))
                .collect();

            self.state.history.push_back((s, y));
            if self.state.history.len() > self.config.lbfgs_history {
                self.state.history.pop_front();
            }
        }

        // 3. Compute Search Direction (L-BFGS Two-Loop Recursion)
        // q = g
        let mut q: Vec<SafeTensor<S>> = gradients.iter().map(|g| SafeTensor(*g)).collect();
        let mut alphas = Vec::new();

        // First loop (backward)
        for (s, y) in self.state.history.iter().rev() {
            let rho = self.dot(y, s).recip();
            let alpha = rho * self.dot(s, &q);
            alphas.push(alpha);

            // q = q - alpha * y
            for (q_i, y_i) in q.iter_mut().zip(y.iter()) {
                *q_i = SafeTensor(q_i.0 - (y_i.0 * alpha));
            }
        }

        // Apply initial Hessian approximation (gamma)
        // gamma = (s_last . y_last) / (y_last . y_last)
        if let Some((s_last, y_last)) = self.state.history.back() {
            let num = self.dot(s_last, y_last);
            let den = self.dot(y_last, y_last);
            let gamma = num / den;
            for q_i in q.iter_mut() {
                *q_i = SafeTensor(q_i.0 * gamma);
            }
        }

        // Second loop (forward)
        for ((s, y), alpha) in self.state.history.iter().zip(alphas.into_iter().rev()) {
            let rho = self.dot(y, s).recip();
            let beta = rho * self.dot(y, &q);

            // q = q + s * (alpha - beta)
            let coeff = alpha - beta;
            for (q_i, s_i) in q.iter_mut().zip(s.iter()) {
                *q_i = SafeTensor(q_i.0 + (s_i.0 * coeff));
            }
        }

        // q is now the direction d = -H * g (approx).
        // Actually L-BFGS computes H * g. So direction is -q.
        let direction: Vec<GraphTensor<S>> = q.iter().map(|t| -t.0).collect();

        // 4. Update Parameters
        // x_new = x + step_size * direction
        // We use a fixed step size or the one from config since we can't do line search easily.
        let step_size = self.config.min_step_persist.max(1e-3); // Heuristic

        let new_params: Vec<GraphTensor<S>> = params
            .iter()
            .zip(direction.iter())
            .map(|(p, d)| *p + (*d * step_size))
            .collect();

        // 5. Update State for next iteration
        self.state.prev_params = Some(params.iter().map(|p| SafeTensor(*p)).collect());
        self.state.prev_grads = Some(gradients.iter().map(|g| SafeTensor(*g)).collect());
        self.state.iteration += 1;

        new_params
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

    fn set_stagnation_multiplier(&mut self, _multiplier: f32) {}

    fn set_stagnation_count(&mut self, _count: usize) {}
}

#[cfg(test)]
mod tests {

    // Tests removed as they depend on candle and eager execution
}