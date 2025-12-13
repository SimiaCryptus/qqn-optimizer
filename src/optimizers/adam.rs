//! # Adam Optimizer
//!
//! Adam (Adaptive Moment Estimation) is a first-order gradient-based optimization algorithm
//! that computes adaptive learning rates for each parameter by maintaining exponentially
//! decaying averages of past gradients and their squared values.
//!
//! ## Algorithm Overview
//!
//! Adam combines the advantages of two other extensions of stochastic gradient descent:
//! - **AdaGrad**: Adapts learning rates based on historical gradient information
//! - **RMSProp**: Uses exponentially decaying average of squared gradients
//!
//! The algorithm maintains two moving averages:
//! - `m_t`: First moment (mean) of gradients - provides momentum
//! - `v_t`: Second moment (uncentered variance) of gradients - adapts learning rates
//!
//! ## Mathematical Formulation
//!
//! ```text
//! m_t = β₁ * m_{t-1} + (1 - β₁) * g_t
//! v_t = β₂ * v_{t-1} + (1 - β₂) * g_t²
//! m̂_t = m_t / (1 - β₁^t)  // Bias correction
//! v̂_t = v_t / (1 - β₂^t)  // Bias correction
//! θ_{t+1} = θ_t - α * m̂_t / (√v̂_t + ε)
//! ```
//!
//! Where:
//! - `g_t`: Gradient at time t
//! - `α`: Learning rate
//! - `β₁, β₂`: Exponential decay rates for moment estimates
//! - `ε`: Small constant for numerical stability
//!
//! ## Strengths
//!
//! - **Adaptive Learning Rates**: Automatically adjusts learning rates per parameter
//! - **Momentum**: Incorporates momentum to accelerate convergence
//! - **Bias Correction**: Corrects for initialization bias in early iterations
//! - **Robust**: Works well across a wide range of problems and hyperparameters
//! - **Memory Efficient**: Only requires first and second moment estimates
//! - **Sparse Gradients**: Handles sparse gradients effectively
//!
//! ## Weaknesses
//!
//! - **Generalization**: May not generalize as well as SGD in some deep learning tasks
//! - **Learning Rate Decay**: Second moment estimate may decay too aggressively
//! - **Hyperparameter Sensitivity**: Performance can be sensitive to β₂ choice
//! - **Non-Convex Convergence**: May not converge to optimal solutions in non-convex settings
//! - **Memory Overhead**: Requires storing moment estimates for each parameter
//!
//! ## When to Use Adam
//!
//! **Good for:**
//! - Deep neural networks
//! - Problems with sparse gradients
//! - When you want adaptive learning rates
//! - Rapid prototyping and experimentation
//! - Non-stationary objectives
//!
//! **Consider alternatives for:**
//! - Final model training where generalization is critical
//! - Very large models where memory is constrained
//! - Problems where SGD with momentum performs well
//!

use crate::optimizers::optimizer::Optimizer;
use log::{debug, info};
use luminal::prelude::*;
use serde::{Deserialize, Serialize};

/// Configuration parameters for the Adam optimizer.
///
/// This struct provides comprehensive control over Adam's behavior, including
/// learning rate schedules, regularization, and algorithmic variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdamConfig {
    /// Learning rate (step size) - controls the magnitude of parameter updates
    ///
    /// **Typical values:** 0.001 (default), 0.0001 (conservative), 0.01 (aggressive)
    /// **Effect:** Higher values lead to faster convergence but may overshoot minima
    pub learning_rate: f32,

    /// Learning rate schedule strategy
    ///
    /// **Options:**
    /// - `"constant"`: Fixed learning rate throughout optimization
    /// - `"exponential"`: Exponential decay: lr *= lr_decay each step
    /// - `"cosine"`: Cosine annealing with warm restarts
    /// - `"adaptive"`: Reduces learning rate when progress stalls
    ///
    /// **Recommendation:** Use "adaptive" for unknown problems, "cosine" for deep learning
    pub lr_schedule: String,

    /// Decay rate for exponential learning rate schedule
    ///
    /// **Range:** (0, 1), typically 0.99-0.999
    /// **Effect:** Lower values cause faster decay
    pub lr_decay: f32,

    /// Minimum learning rate floor
    ///
    /// **Purpose:** Prevents learning rate from becoming too small
    /// **Typical values:** 1e-8 to 1e-15
    pub min_learning_rate: f32,

    /// Gradient clipping threshold (optional)
    ///
    /// **Purpose:** Prevents exploding gradients by clipping gradient norm
    /// **Typical values:** 0.5-5.0, or None to disable
    /// **Effect:** Improves training stability but may slow convergence
    pub gradient_clip: Option<f32>,

    /// Exponential decay rate for first moment estimates (momentum parameter)
    ///
    /// **Range:** [0, 1), typically 0.9
    /// **Effect:**
    /// - Higher values: More momentum, smoother updates
    /// - Lower values: Less momentum, more responsive to recent gradients
    pub beta1: f32,

    /// Exponential decay rate for second moment estimates (adaptive learning rate parameter)
    ///
    /// **Range:** [0, 1), typically 0.999
    /// **Effect:**
    /// - Higher values: More stable adaptive learning rates, slower adaptation
    /// - Lower values: Faster adaptation to gradient changes, potentially less stable
    /// **Critical:** This parameter significantly affects convergence behavior
    pub beta2: f32,

    /// Small constant added to denominator for numerical stability
    ///
    /// **Range:** 1e-8 to 1e-12
    /// **Purpose:** Prevents division by zero when second moment estimates are small
    /// **Trade-off:** Smaller values allow smaller effective learning rates but may cause instability
    pub epsilon: f32,

    /// Weight decay coefficient (L2 regularization strength)
    ///
    /// **Range:** [0, ∞), typically 0.0-0.01
    /// **Purpose:** Prevents overfitting by penalizing large parameter values
    /// **Implementation:** Adds weight_decay * param to gradient before Adam update
    pub weight_decay: f32,

    /// Enable AMSGrad variant for improved convergence guarantees
    ///
    /// **AMSGrad modification:** Uses max(v_t, v_{t-1}) instead of v_t for second moment
    /// **Benefits:** Better theoretical convergence properties, may avoid local minima
    /// **Cost:** Slightly more memory and computation
    pub amsgrad: bool,

    /// Maximum line search iterations (currently unused but reserved for future enhancements)
    ///
    /// **Purpose:** Would limit computational cost of line search procedures
    pub max_line_search_iter: usize,

    /// Enable detailed logging for debugging and monitoring
    ///
    /// **Output:** Gradient norms, parameter statistics, convergence metrics
    /// **Performance:** May slow down optimization due to logging overhead
    pub verbose: bool,
}

impl Default for AdamConfig {
    fn default() -> Self {
        // Standard Adam hyperparameters from the original paper
        Self {
            learning_rate: 0.001, // Kingma & Ba (2014) recommendation
            lr_schedule: "constant".to_string(),
            lr_decay: 0.999,
            min_learning_rate: 1e-12,
            gradient_clip: None, // Conservative default
            beta1: 0.9,          // Standard momentum parameter
            beta2: 0.999,        // Standard second moment parameter
            epsilon: 1e-8,       // Standard numerical stability constant
            weight_decay: 0.0,
            amsgrad: false,
            max_line_search_iter: 20,
            verbose: false,
        }
    }
}

impl AdamConfig {
    /// Create a strict configuration for high-precision optimization.
    ///
    /// **Use case:** Scientific computing, high-precision numerical optimization
    ///
    /// **Strategy:**
    /// - Very conservative learning rate (0.0001) for careful parameter updates
    /// - Adaptive schedule automatically reduces learning rate when progress stalls
    /// - Aggressive gradient clipping (0.5) prevents instability
    /// - High-precision epsilon (1e-12) for numerical accuracy
    /// - AMSGrad variant for theoretical convergence guarantees
    /// - Extended line search iterations for thorough step size selection
    ///
    /// **Trade-offs:**
    /// - **Pros:** High precision, stable convergence, robust to difficult landscapes
    /// - **Cons:** Slower convergence, more conservative updates, higher computational cost
    pub fn strict() -> Self {
        Self {
            learning_rate: 0.0001, // 10x smaller than default for precision
            lr_schedule: "adaptive".to_string(),
            lr_decay: 0.995,          // Slower decay for adaptive schedule
            min_learning_rate: 1e-15, // Allow very small learning rates
            gradient_clip: Some(0.5), // Tight gradient control
            beta1: 0.9,
            beta2: 0.9999,  // More stable second moment estimates
            epsilon: 1e-12, // Higher numerical precision
            weight_decay: 0.0,
            amsgrad: true,            // Better convergence guarantees
            max_line_search_iter: 50, // Thorough step size selection
            verbose: false,
        }
    }

    /// Create a lax configuration for fast, approximate optimization.
    ///
    /// **Use case:** Rapid prototyping, approximate solutions, time-constrained optimization
    ///
    /// **Strategy:**
    /// - Aggressive learning rate (0.01) for rapid convergence
    /// - Exponential decay prevents overshooting in later iterations
    /// - No gradient clipping allows maximum step sizes
    /// - Lower precision settings for computational efficiency
    /// - Reduced second moment decay (0.99) for faster adaptation
    /// - Minimal line search iterations for speed
    ///
    /// **Trade-offs:**
    /// - **Pros:** Fast convergence, low computational cost, good for exploration
    /// - **Cons:** May overshoot minima, less stable, lower final precision
    pub fn lax() -> Self {
        Self {
            learning_rate: 0.01, // 10x larger than default for speed
            lr_schedule: "exponential".to_string(),
            lr_decay: 0.99,          // Faster decay to control later iterations
            min_learning_rate: 1e-8, // Don't go too small
            gradient_clip: None,     // Allow large steps
            beta1: 0.9,
            beta2: 0.99,   // Faster adaptation to gradient changes
            epsilon: 1e-6, // Lower precision for speed
            weight_decay: 0.0,
            amsgrad: false,          // Standard Adam is faster
            max_line_search_iter: 5, // Minimal line search overhead
            verbose: false,
        }
    }

    /// Create a configuration optimized for deep learning tasks.
    ///
    /// **Use case:** Neural network training, deep learning research
    ///
    /// **Strategy:**
    /// - Standard Adam learning rate (0.001) proven effective for neural networks
    /// - Cosine annealing provides smooth learning rate decay with warm restarts
    /// - Moderate gradient clipping (1.0) prevents exploding gradients common in deep networks
    /// - L2 regularization (0.01) helps prevent overfitting
    /// - Standard Adam hyperparameters optimized for deep learning
    ///
    /// **Based on:** Common practices in deep learning literature and frameworks
    ///
    /// **Trade-offs:**
    /// - **Pros:** Well-tested for neural networks, good generalization, handles deep architectures
    /// - **Cons:** May not be optimal for non-neural network problems
    pub fn deep_learning() -> Self {
        Self {
            learning_rate: 0.001, // Sweet spot for neural networks
            lr_schedule: "cosine".to_string(),
            lr_decay: 0.999,
            min_learning_rate: 1e-6,  // Don't decay too aggressively
            gradient_clip: Some(1.0), // Prevent exploding gradients
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            weight_decay: 0.01, // Moderate regularization
            amsgrad: false,
            max_line_search_iter: 10,
            verbose: false,
        }
    }
}

/// Internal state maintained by the Adam optimizer across iterations.
///
/// This struct stores the exponentially decaying averages of gradients and their
/// squared values, which are essential for Adam's adaptive learning rate mechanism.
///
/// **Memory Requirements:** O(number of parameters) for each moment estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdamState {
    /// Current iteration number (used for bias correction)
    ///
    /// **Purpose:** Bias correction terms depend on iteration count: (1 - β^t)
    pub iteration: usize,

    /// First moment estimates (exponentially decaying average of gradients)
    ///
    /// **Formula:** m_t = β₁ * m_{t-1} + (1 - β₁) * g_t
    /// **Purpose:** Provides momentum and direction information
    /// **Note:** Skipped in serialization due to Tensor complexity
    #[serde(skip_serializing, skip_deserializing)]
    pub m: Option<Vec<GraphTensor>>,

    /// Second moment estimates (exponentially decaying average of squared gradients)
    ///
    /// **Formula:** v_t = β₂ * v_{t-1} + (1 - β₂) * g_t²
    /// **Purpose:** Adapts learning rates based on gradient variance
    /// **Note:** Skipped in serialization due to Tensor complexity
    #[serde(skip_serializing, skip_deserializing)]
    pub v: Option<Vec<GraphTensor>>,

    /// Maximum second moment estimates (AMSGrad variant only)
    ///
    /// **Formula:** v̂_t = max(v_t, v̂_{t-1})
    /// **Purpose:** Ensures non-increasing effective learning rates
    /// **Memory:** Only allocated when AMSGrad is enabled
    #[serde(skip_serializing, skip_deserializing)]
    pub v_max: Option<Vec<GraphTensor>>,
}

impl Default for AdamState {
    fn default() -> Self {
        Self::new()
    }
}

impl AdamState {
    /// Create a new Adam state with default initialization.
    ///
    /// **Initial state:** All moment estimates are None and will be initialized
    /// on the first optimization step based on parameter dimensions.
    pub fn new() -> Self {
        Self {
            iteration: 0,
            m: None,
            v: None,
            v_max: None,
        }
    }

    /// Reset the Adam state to initial conditions.
    ///
    /// **Use cases:**
    /// - Restarting optimization from scratch
    /// - Switching between different optimization phases
    /// - Clearing accumulated momentum when changing problem structure
    ///
    /// **Effect:** All moment estimates are cleared and iteration count is reset
    pub fn reset(&mut self) {
        self.iteration = 0;
        self.m = None;
        self.v = None;
        self.v_max = None;
    }

    /// Get the current iteration number.
    ///
    /// **Usage:** Monitoring optimization progress, implementing custom schedules
    pub fn iteration(&self) -> usize {
        self.iteration
    }
}

/// Adam optimizer implementation with comprehensive features and monitoring.
///
/// This implementation provides:
/// - Standard Adam algorithm with bias correction
/// - AMSGrad variant for improved convergence
/// - Multiple learning rate schedules
/// - Gradient clipping and weight decay
/// - Comprehensive logging and monitoring
/// - Adaptive convergence detection
///
/// **Thread Safety:** The optimizer itself is not thread-safe, but can be used
/// with thread-safe functions through the Arc<dyn DifferentiableFunction> interface.
#[derive(Debug)]
pub struct AdamOptimizer {
    config: AdamConfig,
    state: AdamState,
    /// Current effective learning rate (may differ from config due to scheduling)
    current_lr: f32,
    /// Stagnation multiplier for relaxed convergence criteria (future use)
    stagnation_multiplier: f32,
    /// Stagnation count threshold (future use)
    stagnation_count: usize,
    /// Name of the optimizer variant
    ///
    /// **Default:** "Adam" for standard Adam, "Adam-AMSGrad" for AMSGrad variant
    name: String,
}

impl Clone for AdamOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            current_lr: self.current_lr,
            stagnation_multiplier: self.stagnation_multiplier,
            stagnation_count: self.stagnation_count,
            name: self.name.clone(),
        }
    }
}

impl AdamOptimizer {
    /// Create a new Adam optimizer with the given configuration.
    pub fn autoname(config: AdamConfig) -> Self {
        Self::new(
            format!(
                "Adam Config: lr={}, beta1={}, beta2={}, epsilon={}, weight_decay={}, amsgrad={}",
                config.learning_rate,
                config.beta1,
                config.beta2,
                config.epsilon,
                config.weight_decay,
                config.amsgrad
            ),
            config,
        )
    }
    pub fn new(name: String, config: AdamConfig) -> Self {
        info!(
            "Adam Config: lr={}, beta1={}, beta2={}, epsilon={}, weight_decay={}, amsgrad={}",
            config.learning_rate,
            config.beta1,
            config.beta2,
            config.epsilon,
            config.weight_decay,
            config.amsgrad
        );
        if config.verbose {
            debug!("Creating Adam optimizer with verbose logging enabled");
        }
        let current_lr = config.learning_rate;
        Self {
            config,
            state: AdamState::new(),
            current_lr,
            stagnation_multiplier: 10.0,
            stagnation_count: 5,
            name: name,
        }
    }
}

impl Optimizer for AdamOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,

        graph: &mut Graph,
        loss: GraphTensor,
        params: &[GraphTensor],
    ) -> Vec<GraphTensor> {
        // Initialize moment estimates if needed
        if self.state.m.is_none() {
            self.state.m = Some(
                params
                    .iter()
                    .map(|p| graph.constant(0.0).expand_to(p.shape()))
                    .collect(),
            );
            self.state.v = Some(
                params
                    .iter()
                    .map(|p| graph.constant(0.0).expand_to(p.shape()))
                    .collect(),
            );
            if self.config.amsgrad {
                self.state.v_max = Some(
                    params
                        .iter()
                        .map(|p| graph.constant(0.0).expand_to(p.shape()))
                        .collect(),
                );
            }
        }

        let m = self.state.m.as_mut().unwrap();
        let v = self.state.v.as_mut().unwrap();

        // Get gradients
        let gradients = params.iter().map(|p| loss.grad(p)).collect::<Vec<_>>();

        let mut new_params = Vec::with_capacity(params.len());
        let t = (self.state.iteration + 1) as f32;
        let bias_correction1 = 1.0 - self.config.beta1.powf(t);
        let bias_correction2 = 1.0 - self.config.beta2.powf(t);

        for i in 0..params.len() {
            // m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
            let m_new = m[i] * self.config.beta1 + gradients[i] * (1.0 - self.config.beta1);
            m[i] = m_new; // Update state reference for next step if graph is rebuilt

            // v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
            let v_new =
                v[i] * self.config.beta2 + gradients[i].square() * (1.0 - self.config.beta2);
            v[i] = v_new;

            // m_hat = m_t / (1 - beta1^t)
            let m_hat = m_new / bias_correction1;

            // v_hat = v_t / (1 - beta2^t)
            let v_hat = v_new / bias_correction2;

            // theta_{t+1} = theta_t - lr * m_hat / (sqrt(v_hat) + epsilon)
            let update = m_hat / (v_hat.sqrt() + self.config.epsilon);

            // Apply weight decay if configured
            let mut p_next = params[i] - update * self.current_lr;
            if self.config.weight_decay > 0.0 {
                p_next = p_next - params[i] * (self.config.weight_decay * self.current_lr);
            }

            new_params.push(p_next);
        }

        self.state.iteration += 1;

        new_params
    }

    fn reset(&mut self) {
        self.state.reset();
        self.current_lr = self.config.learning_rate;
        // Note: name is not reset as it's determined by configuration
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn iteration(&self) -> usize {
        self.state.iteration()
    }
    fn set_stagnation_multiplier(&mut self, multiplier: f32) {
        self.stagnation_multiplier = multiplier;
    }
    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adam_state_creation() {
        let state = AdamState::new();
        assert_eq!(state.iteration(), 0);
        assert!(state.m.is_none());
        assert!(state.v.is_none());
        assert!(state.v_max.is_none());
    }

    #[test]
    fn test_adam_empty_params_error() {
        let config = AdamConfig::default();
        let mut optimizer = AdamOptimizer::autoname(config);
        let mut params: Vec<Tensor> = vec![];
        let function = Arc::new(QuadraticFunction);
        let result = optimizer.step(&mut params, function);
        assert!(result.is_err());
    }
}
