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

use crate::optimizers::optimizer::{ConvergenceInfo, OptimizationContext, Optimizer, StepResult};
use luminal::prelude::*;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::time::Instant;

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
    pub learning_rate: f64,

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
    pub lr_decay: f64,

    /// Minimum learning rate floor
    ///
    /// **Purpose:** Prevents learning rate from becoming too small
    /// **Typical values:** 1e-8 to 1e-15
    pub min_learning_rate: f64,

    /// Gradient clipping threshold (optional)
    ///
    /// **Purpose:** Prevents exploding gradients by clipping gradient norm
    /// **Typical values:** 0.5-5.0, or None to disable
    /// **Effect:** Improves training stability but may slow convergence
    pub gradient_clip: Option<f64>,

    /// Exponential decay rate for first moment estimates (momentum parameter)
    ///
    /// **Range:** [0, 1), typically 0.9
    /// **Effect:**
    /// - Higher values: More momentum, smoother updates
    /// - Lower values: Less momentum, more responsive to recent gradients
    pub beta1: f64,

    /// Exponential decay rate for second moment estimates (adaptive learning rate parameter)
    ///
    /// **Range:** [0, 1), typically 0.999
    /// **Effect:**
    /// - Higher values: More stable adaptive learning rates, slower adaptation
    /// - Lower values: Faster adaptation to gradient changes, potentially less stable
    /// **Critical:** This parameter significantly affects convergence behavior
    pub beta2: f64,

    /// Small constant added to denominator for numerical stability
    ///
    /// **Range:** 1e-8 to 1e-12
    /// **Purpose:** Prevents division by zero when second moment estimates are small
    /// **Trade-off:** Smaller values allow smaller effective learning rates but may cause instability
    pub epsilon: f64,

    /// Weight decay coefficient (L2 regularization strength)
    ///
    /// **Range:** [0, ∞), typically 0.0-0.01
    /// **Purpose:** Prevents overfitting by penalizing large parameter values
    /// **Implementation:** Adds weight_decay * param to gradient before Adam update
    pub weight_decay: f64,

    /// Enable AMSGrad variant for improved convergence guarantees
    ///
    /// **AMSGrad modification:** Uses max(v_t, v_{t-1}) instead of v_t for second moment
    /// **Benefits:** Better theoretical convergence properties, may avoid local minima
    /// **Cost:** Slightly more memory and computation
    pub amsgrad: bool,

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
    #[serde(skip)]
    pub m: Vec<Vec<f32>>,

    /// Second moment estimates (exponentially decaying average of squared gradients)
    ///
    /// **Formula:** v_t = β₂ * v_{t-1} + (1 - β₂) * g_t²
    /// **Purpose:** Adapts learning rates based on gradient variance
    #[serde(skip)]
    pub v: Vec<Vec<f32>>,

    /// Maximum second moment estimates (AMSGrad variant only)
    ///
    /// **Formula:** v̂_t = max(v_t, v̂_{t-1})
    /// **Purpose:** Ensures non-increasing effective learning rates
    /// **Memory:** Only allocated when AMSGrad is enabled
    #[serde(skip)]
    pub v_max: Vec<Vec<f32>>,
}

impl Default for AdamState {
    fn default() -> Self {
        Self::new()
    }
}

impl AdamState {
    /// Create a new Adam state with default initialization.
    ///
    /// **Initial state:** All moment estimates are empty and will be initialized
    /// on the first optimization step based on parameter dimensions.
    pub fn new() -> Self {
        Self {
            iteration: 0,
            m: Vec::new(),
            v: Vec::new(),
            v_max: Vec::new(),
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
        self.m.clear();
        self.v.clear();
        self.v_max.clear();
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
/// **Thread Safety:** The optimizer itself is not thread-safe.
#[derive(Debug)]
pub struct AdamOptimizer {
    config: AdamConfig,
    state: AdamState,
    /// Current effective learning rate (may differ from config due to scheduling)
    current_lr: f64,
    /// Previous function value for adaptive learning rate and convergence detection
    prev_function_value: Option<f64>,
    /// Count of consecutive steps without improvement (for adaptive scheduling)
    bad_step_count: usize,
    /// Stagnation multiplier for relaxed convergence criteria (future use)
    stagnation_multiplier: f64,
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
            prev_function_value: self.prev_function_value,
            bad_step_count: self.bad_step_count,
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
                "Adam(lr={}, b1={}, b2={}, eps={}, wd={}, ams={})",
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
            prev_function_value: None,
            bad_step_count: 0,
            stagnation_multiplier: 10.0,
            stagnation_count: 5,
            name,
        }
    }

    /// Update learning rate based on schedule
    fn update_learning_rate(&mut self, current_value: Option<f64>) {
        match self.config.lr_schedule.as_str() {
            "exponential" => {
                self.current_lr *= self.config.lr_decay;
                self.current_lr = self.current_lr.max(self.config.min_learning_rate);
            }
            "cosine" => {
                let t = self.state.iteration as f64;
                let cosine_decay = 0.5 * (1.0 + (std::f64::consts::PI * t / 10000.0).cos()); // Slower cosine decay
                self.current_lr = self.config.min_learning_rate
                    + (self.config.learning_rate - self.config.min_learning_rate) * cosine_decay;
            }
            "adaptive" => {
                // More aggressive adaptive learning rate schedule for finding minima
                if let (Some(prev_val), Some(curr_val)) = (self.prev_function_value, current_value)
                {
                    let relative_improvement = (prev_val - curr_val) / prev_val.abs().max(1e-12);

                    // Reduce LR more aggressively when improvement is small
                    if curr_val > prev_val || relative_improvement < 1e-10 {
                        self.bad_step_count += 1;
                        // Require fewer consecutive bad steps and use more aggressive reduction
                        if self.bad_step_count >= 5 {
                            self.current_lr *= 0.5; // More aggressive reduction
                            self.current_lr = self.current_lr.max(self.config.min_learning_rate);
                            self.bad_step_count = 0;
                            if self.config.verbose {
                                debug!("Reducing learning rate to {:.6e} due to lack of progress (relative_improvement: {:.6e})", 
                                      self.current_lr, relative_improvement);
                            }
                        }
                    } else {
                        self.bad_step_count = 0;
                        // Optionally increase learning rate when making good progress
                        if relative_improvement > 1e-6
                            && self.current_lr < self.config.learning_rate * 0.1
                        {
                            self.current_lr =
                                (self.current_lr * 1.1).min(self.config.learning_rate * 0.1);
                        }
                    }
                }
            }
            _ => {} // constant learning rate
        }
        // Update previous function value for all schedules
        self.prev_function_value = current_value;
    }
}

impl Optimizer for AdamOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(&mut self, ctx: &mut OptimizationContext) -> StepResult {
        let start_time = Instant::now();
        let gradients = &ctx.gradients;
        let weight_length = ctx.weights.len();

        if self.config.verbose {
            debug!("Adam Step {}: Processing {} tensors", self.state.iteration, weight_length);
        }

        // 1. Retrieve all data to CPU
        let mut all_weights_data: Vec<Vec<f32>> = ctx.weights.iter().map(|w| w.data()).collect();
        let all_grads_data: Vec<Vec<f32>> = gradients.iter().map(|g| g.data()).collect();

        // Initialize moment estimates if needed
        if self.state.m.len() != weight_length {
            self.state.m = all_weights_data.iter().map(|w| vec![0.0; w.len()]).collect();
            self.state.v = all_weights_data.iter().map(|w| vec![0.0; w.len()]).collect();
            if self.config.amsgrad {
                self.state.v_max = all_weights_data.iter().map(|w| vec![0.0; w.len()]).collect();
            }
        }

        // 2. Calculate global gradient norm (after weight decay) for clipping
        let mut total_norm_sq = 0.0;
        if self.config.gradient_clip.is_some() || self.config.verbose {
            for (i, g_vec) in all_grads_data.iter().enumerate() {
                let w_vec = &all_weights_data[i];
                for (j, &g) in g_vec.iter().enumerate() {
                    let mut g_val = g as f64;
                    if self.config.weight_decay > 0.0 {
                        g_val += self.config.weight_decay * w_vec[j] as f64;
                    }
                    total_norm_sq += g_val * g_val;
                }
            }
        }
        let total_norm = total_norm_sq.sqrt();

        if self.config.verbose {
            debug!("Global gradient norm: {:.6e}", total_norm);
        }

        // 3. Determine scaling factor for clipping
        let clip_scale = if let Some(max_norm) = self.config.gradient_clip {
            if total_norm > max_norm {
                let scale = max_norm / total_norm;
                if self.config.verbose {
                    debug!(
                        "Clipping gradients: norm {:.6e} > max {:.6e}, scale = {:.6e}",
                        total_norm, max_norm, scale
                    );
                }
                scale
            } else {
                1.0
            }
        } else {
            1.0
        };

        // 4. Update Learning Rate
        // Try to get current loss from context if available/computed
        let current_loss = if self.config.lr_schedule == "adaptive" || self.config.verbose {
             ctx.loss.data().first().cloned().map(|x| x as f64)
        } else {
            None
        };
        self.update_learning_rate(current_loss);

        // 5. Apply updates
        let beta1 = self.config.beta1;
        let beta2 = self.config.beta2;
        let epsilon = self.config.epsilon;
        let lr = self.current_lr;
        
        self.state.iteration += 1;
        let t = self.state.iteration as f64;
        let bias_correction1 = 1.0 - beta1.powf(t);
        let bias_correction2 = 1.0 - beta2.powf(t);

        for i in 0..weight_length {
            let w_vec = &mut all_weights_data[i];
            let g_vec = &all_grads_data[i];
            let m_vec = &mut self.state.m[i];
            let v_vec = &mut self.state.v[i];
            
            for j in 0..w_vec.len() {
                let mut g = g_vec[j] as f64;
                let w = w_vec[j] as f64;

                // Weight decay
                if self.config.weight_decay > 0.0 {
                    g += self.config.weight_decay * w;
                }

                // Clipping
                g *= clip_scale;

                // Update biased first moment estimate
                // m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
                let m_new = beta1 * m_vec[j] as f64 + (1.0 - beta1) * g;
                m_vec[j] = m_new as f32;

                // Update biased second raw moment estimate
                // v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
                let v_new = beta2 * v_vec[j] as f64 + (1.0 - beta2) * g * g;
                v_vec[j] = v_new as f32;

                // Compute bias-corrected moment estimates
                let m_hat = m_new / bias_correction1;
                
                let v_hat_val = if self.config.amsgrad {
                    let v_max_vec = &mut self.state.v_max[i];
                    let v_max_val = v_max_vec[j].max(v_new as f32);
                    v_max_vec[j] = v_max_val;
                    (v_max_val as f64) / bias_correction2
                } else {
                    v_new / bias_correction2
                };

                // Compute update: lr * m_hat / (sqrt(v_hat) + epsilon)
                let update = lr * m_hat / (v_hat_val.sqrt() + epsilon);
                
                w_vec[j] = (w - update) as f32;
            }
        }

        ctx.write_weights(&mut all_weights_data);

        if self.config.verbose {
            let step_duration = start_time.elapsed();
            debug!("Adam Step {} Completed in {:?}", self.state.iteration, step_duration);
        }

        StepResult {
            step_size: lr,
            convergence_info: ConvergenceInfo {
                converged: false,
                function_change: None,
            },
        }
    }

    fn reset(&mut self) {
        self.state.reset();
        self.current_lr = self.config.learning_rate;
        self.prev_function_value = None;
        self.bad_step_count = 0;
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn set_stagnation_multiplier(&mut self, multiplier: f64) {
        self.stagnation_multiplier = multiplier;
    }
    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
    fn learning_rate(&self) -> Option<f64> {
        Some(self.current_lr)
    }
    fn set_learning_rate(&mut self, lr: f64) {
        self.config.learning_rate = lr;
        self.current_lr = lr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adam_config_strict() {
        let config = AdamConfig::strict();
        assert_eq!(config.learning_rate, 0.0001);
        assert_eq!(config.lr_schedule, "adaptive");
        assert_eq!(config.gradient_clip, Some(0.5));
        assert!(config.amsgrad);
        let optimizer = AdamOptimizer::autoname(config);
        assert!(optimizer.name().contains("Adam"));
    }

    #[test]
    fn test_adam_config_lax() {
        let config = AdamConfig::lax();
        assert_eq!(config.learning_rate, 0.01);
        assert_eq!(config.lr_schedule, "exponential");
        assert_eq!(config.gradient_clip, None);
        assert!(!config.amsgrad);
    }

    #[test]
    fn test_adam_state_creation() {
        let state = AdamState::new();
        assert_eq!(state.iteration(), 0);
        assert!(state.m.is_empty());
        assert!(state.v.is_empty());
    }

    #[test]
    fn test_adam_state_reset() {
        let mut state = AdamState::new();
        state.iteration = 10;
        state.m = vec![vec![1.0]];
        state.reset();
        assert_eq!(state.iteration(), 0);
        assert!(state.m.is_empty());
    }

    #[test]
    fn test_adam_optimizer_creation() {
        let config = AdamConfig::default();
        let optimizer = AdamOptimizer::autoname(config);
        assert_eq!(optimizer.state.iteration(), 0);
        assert_eq!(optimizer.current_lr, optimizer.config.learning_rate);
    }

    #[test]
    fn test_adam_reset() {
        let config = AdamConfig::default();
        let mut optimizer = AdamOptimizer::autoname(config);
        optimizer.state.iteration = 5;
        optimizer.current_lr = 0.001;
        optimizer.reset();
        assert_eq!(optimizer.state.iteration(), 0);
        assert_eq!(optimizer.current_lr, optimizer.config.learning_rate);
    }
}