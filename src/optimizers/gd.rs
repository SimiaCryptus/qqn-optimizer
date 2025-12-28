//! # Gradient Descent (GD) Optimizer
//!
//! This module implements a comprehensive gradient descent optimizer with advanced features
//! for robust optimization across different problem types.
//!
//! ## Algorithm Overview
//!
//! The gradient descent algorithm follows the iterative update rule:
//! ```text
//! θ_{t+1} = θ_t - α * ∇f(θ_t)
//! ```
//! where:
//! - θ_t are the parameters at iteration t
//! - α is the learning rate (step size)
//! - ∇f(θ_t) is the gradient of the objective function at θ_t
//!
//! ## Key Features
//!
//! ### Momentum Support
//! - **Standard Momentum**: Accumulates gradients with exponential decay
//!   ```text
//!   v_t = β * v_{t-1} + ∇f(θ_t)
//!   θ_{t+1} = θ_t - α * v_t
//!   ```
//! - **Nesterov Momentum**: Uses look-ahead gradient for better convergence
//!   ```text
//!   v_t = β * v_{t-1} + ∇f(θ_t)
//!   θ_{t+1} = θ_t - α * (β * v_t + ∇f(θ_t))
//!   ```
//!
//! ### Adaptive Learning Rate
//! Automatically adjusts learning rate based on gradient magnitude to prevent
//! divergence with large gradients while maintaining progress with small gradients.
//!
//! ### Gradient Clipping
//! Prevents gradient explosion by clipping gradients to a maximum norm,
//! essential for training stability on challenging optimization landscapes.
//!
//! ### Weight Decay (L2 Regularization)
//! Adds regularization term to prevent overfitting: `grad += weight_decay * param`
//!
//! ## Strengths
//! - Simple and well-understood algorithm
//! - Guaranteed convergence for convex functions with appropriate learning rate
//! - Memory efficient (O(n) for momentum, O(1) otherwise)
//! - Robust with gradient clipping and adaptive learning rate
//! - Good baseline performance across many problem types
//!
//! ## Weaknesses
//! - Can be slow on ill-conditioned problems
//! - Sensitive to learning rate selection
//! - May oscillate in narrow valleys (partially addressed by momentum)
//! - No automatic scaling for different parameter dimensions
//! - Can get stuck in saddle points (though momentum helps)
//!
//! ## When to Use
//! - **Good for**: Well-conditioned problems, when simplicity is valued, baseline comparisons
//! - **Avoid for**: Highly ill-conditioned problems, when fast convergence is critical
//! - **Consider alternatives**: Adam/AdamW for adaptive per-parameter scaling, L-BFGS for smooth functions

use crate::optimizers::optimizer::SafeTensor;
use crate::optimizers::optimizer::{OptimizationContext, Optimizer};
use crate::optimizers::OptimizationMetadata;
use crate::{ConvergenceInfo, StepResult};
use log::{debug, info};
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration parameters for the GD optimizer.
///
/// This struct controls all aspects of the gradient descent optimization process,
/// from basic parameters like learning rate to advanced features like adaptive
/// learning rate scaling and gradient clipping.
///
/// # Parameter Guidelines
///
/// ## Learning Rate (`learning_rate`)
/// - **Range**: Typically 1e-4 to 1e-1
/// - **Too high**: Causes divergence, oscillation, or overshooting
/// - **Too low**: Slow convergence, may get stuck in local minima
/// - **Recommendation**: Start with 0.01, adjust based on convergence behavior
///
/// ## Momentum (`momentum`)
/// - **Range**: 0.0 to 0.99 (0.0 = no momentum)
/// - **Benefits**: Accelerates convergence, helps escape local minima, smooths oscillations
/// - **Drawbacks**: Can overshoot, requires tuning, adds memory overhead
/// - **Recommendation**: 0.9 for most problems, 0.0 for simple convex functions
///
/// ## Weight Decay (`weight_decay`)
/// - **Range**: 0.0 to 1e-2 (0.0 = no regularization)
/// - **Purpose**: Prevents overfitting by penalizing large parameter values
/// - **Effect**: Equivalent to L2 regularization in the loss function
/// - **Recommendation**: 1e-4 to 1e-3 for regularization, 0.0 for pure optimization
///
/// ## Gradient Clipping (`max_grad_norm`)
/// - **Range**: 1.0 to 100.0 (0.0 = no clipping)
/// - **Purpose**: Prevents gradient explosion, stabilizes training
/// - **Trade-off**: Too aggressive clipping can slow convergence
/// - **Recommendation**: 10.0 for most problems, 1.0 for unstable functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDConfig {
    /// Learning rate (step size)
    ///
    /// Optimizer name for identification and logging
    ///
    /// This name is used to identify the optimizer variant and configuration.
    /// It's automatically set by the configuration presets but can be customized.
    pub name: String,
    ///
    /// Controls the size of parameter updates. Higher values lead to faster
    /// convergence but risk overshooting or divergence. Lower values are
    /// more stable but converge slowly.
    pub learning_rate: f64,

    /// Momentum coefficient (0.0 = no momentum, 0.9 = high momentum)
    ///
    /// Accumulates gradients from previous iterations to accelerate convergence
    /// and smooth out oscillations. Higher values provide more acceleration
    /// but can cause overshooting.
    pub momentum: f64,

    /// Weight decay (L2 regularization)
    ///
    /// Adds a penalty term proportional to parameter magnitude, equivalent
    /// to L2 regularization. Helps prevent overfitting by encouraging
    /// smaller parameter values.
    pub weight_decay: f64,

    /// Enable Nesterov momentum
    ///
    /// Uses "look-ahead" gradients for better convergence properties.
    /// Only effective when momentum > 0. Generally provides better
    /// convergence than standard momentum.
    pub nesterov: bool,

    /// Maximum gradient norm for clipping (0.0 = no clipping)
    ///
    /// Clips gradients to this maximum norm to prevent gradient explosion.
    /// Essential for training stability on functions with large gradients
    /// or steep regions.
    pub max_grad_norm: f64,

    /// Enable adaptive learning rate based on gradient magnitude
    ///
    /// Automatically reduces learning rate when gradients are large to
    /// prevent divergence. Uses a sigmoid-like scaling function that
    /// preserves learning rate for moderate gradients.
    pub adaptive_lr: bool,

    /// Minimum learning rate when using adaptive scaling
    ///
    /// Prevents adaptive learning rate from becoming too small, which
    /// could halt optimization progress. Only used when adaptive_lr is true.
    pub min_learning_rate: f64,

    /// Enable verbose logging
    ///
    /// Provides detailed logging of optimization progress including
    /// parameter values, gradients, and internal computations.
    /// Useful for debugging but can impact performance.
    pub verbose: bool,
}

impl Default for GDConfig {
    fn default() -> Self {
        Self {
            name: "GD-Strict".to_string(),
            learning_rate: 0.01,
            momentum: 0.0,
            weight_decay: 0.0,
            nesterov: false,
            max_grad_norm: 10.0,
            adaptive_lr: true,
            min_learning_rate: 1e-7,
            verbose: false,
        }
    }
}

impl GDConfig {
    /// Create a strict configuration with conservative settings for stable convergence.
    ///
    /// **Name**: "GD-Strict"
    ///
    /// **Best for**:
    /// - Ill-conditioned problems with high condition numbers
    /// - Functions with large or unstable gradients
    /// - Production environments where stability is critical
    /// **Best for**:
    /// - Ill-conditioned problems with high condition numbers
    /// - Functions with large or unstable gradients
    /// - Production environments where stability is critical
    /// - When you need guaranteed convergence over speed
    ///
    /// **Characteristics**:
    /// - Very small learning rate (0.001) for stability
    /// - No momentum to avoid overshooting
    /// - Aggressive gradient clipping (norm = 1.0)
    /// - Adaptive learning rate enabled for additional safety
    pub fn strict() -> Self {
        Self {
            learning_rate: 0.001,
            name: "GD-Debug".to_string(),
            momentum: 0.0,
            weight_decay: 0.0,
            nesterov: false,
            max_grad_norm: 1.0,
            adaptive_lr: true,
            min_learning_rate: 1e-8,
            verbose: false,
        }
    }

    /// Create a lax configuration with aggressive settings for fast convergence.
    ///
    /// **Name**: "GD-Lax"
    ///
    /// **Best for**:
    /// - Well-conditioned problems with reasonable condition numbers
    /// - Experimentation and hyperparameter tuning
    /// - When convergence speed is prioritized over stability
    /// - Functions with well-behaved gradients
    ///
    /// **Characteristics**:
    /// - Large learning rate (0.1) for fast progress
    /// - High momentum (0.9) with Nesterov acceleration
    /// - Relaxed gradient clipping (norm = 100.0)
    /// - Adaptive learning rate disabled for consistent behavior
    pub fn lax() -> Self {
        Self {
            learning_rate: 0.1,
            name: "GD-Lax".to_string(),
            momentum: 0.9,
            weight_decay: 0.0,
            nesterov: true,
            max_grad_norm: 100.0,
            adaptive_lr: false,
            min_learning_rate: 1e-6,
            verbose: false,
        }
    }

    /// Create a configuration optimized for the Rosenbrock function and similar challenging landscapes.
    ///
    /// **Name**: "GD-Rosenbrock"
    ///
    /// **Best for**:
    /// - Non-convex optimization problems
    /// - Functions with narrow valleys or ridges (like Rosenbrock)
    /// - Problems with vastly different curvatures in different directions
    /// - Optimization landscapes with saddle points
    ///
    /// **Characteristics**:
    /// - Moderate learning rate (0.001) to handle steep gradients
    /// - High momentum (0.9) with Nesterov to navigate valleys
    /// - Moderate gradient clipping for stability
    /// - Adaptive learning rate to handle varying gradient magnitudes
    ///
    /// **Note**: The Rosenbrock function f(x,y) = (1-x)² + 100(y-x²)² is a classic
    /// test case with a narrow curved valley that challenges most optimizers.
    pub fn rosenbrock() -> Self {
        Self {
            learning_rate: 0.001,
            name: "GD-Rosenbrock".to_string(),
            momentum: 0.9,
            weight_decay: 0.0,
            nesterov: true,
            max_grad_norm: 10.0,
            adaptive_lr: true,
            min_learning_rate: 1e-7,
            verbose: false,
        }
    }

    /// Create a configuration with verbose logging enabled for debugging.
    ///
    /// **Name**: "GD-Debug"
    ///
    /// **Best for**:
    /// - Debugging optimization problems
    /// - Understanding optimizer behavior
    /// - Analyzing convergence patterns
    /// - Educational purposes
    ///
    /// **Characteristics**:
    /// - Based on default configuration for balanced behavior
    /// - Verbose logging enabled for detailed output
    /// - Shows parameter values, gradients, and internal computations
    ///
    /// **Warning**: Verbose logging can significantly impact performance
    /// and should not be used in production or performance-critical code.
    pub fn debug() -> Self {
        Self {
            name: "GD-Debug".to_string(),
            learning_rate: 0.01,
            momentum: 0.0,
            weight_decay: 0.0,
            nesterov: false,
            max_grad_norm: 10.0,
            adaptive_lr: true,
            min_learning_rate: 1e-7,
            verbose: true,
        }
    }
}

/// State information for GD optimization.
///
/// Maintains the internal state of the gradient descent optimizer across
/// optimization steps. This includes iteration counting and momentum buffers.
///
/// # Serialization Note
///
/// The momentum buffer is excluded from serialization (`serde(skip)`) because
/// raw data cannot be easily serialized. When deserializing, the momentum
/// buffer will be reinitialized on the first optimization step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GDState {
    /// Current iteration number
    ///
    /// Tracks the number of optimization steps taken. Used for logging,
    /// convergence criteria, and potential learning rate scheduling.
    pub iteration: usize,

    /// Momentum buffer (velocity)
    ///
    /// Stores the accumulated momentum for each parameter tensor.
    /// Only allocated when momentum > 0. The buffer has the same
    /// structure as the parameter tensors.
    #[serde(skip_serializing, skip_deserializing)]
    pub momentum_buffer: Vec<Vec<f32>>,
}

impl Default for GDState {
    fn default() -> Self {
        Self::new()
    }
}

impl GDState {
    /// Create a new GD state.
    ///
    /// Initializes the state with zero iterations and no momentum buffer.
    /// The momentum buffer will be allocated on the first step if momentum > 0.
    pub fn new() -> Self {
        Self {
            iteration: 0,
            momentum_buffer: Vec::new(),
        }
    }

    /// Reset the GD state to initial conditions.
    ///
    /// Clears the iteration counter and momentum buffer, effectively
    /// restarting the optimization from scratch. Useful for multiple
    /// optimization runs or when changing problem parameters.
    pub fn reset(&mut self) {
        self.iteration = 0;
        self.momentum_buffer.clear();
    }

    /// Get the current iteration number.
    ///
    /// Returns the number of optimization steps completed. This can be
    /// used for convergence analysis or learning rate scheduling.
    pub fn iteration(&self) -> usize {
        self.iteration
    }
}

/// GD optimizer implementation.
///
/// The main gradient descent optimizer that implements the [`Optimizer`] trait.
/// Supports various advanced features including momentum, adaptive learning rates,
/// gradient clipping, and weight decay.
///
/// # Algorithm Details
///
/// The optimizer implements the following update sequence:
///
/// 1. **Gradient Computation**: Compute ∇f(θ_t) using the provided function
/// 2. **Weight Decay**: Add L2 regularization term: `grad += weight_decay * param`
/// 3. **Gradient Clipping**: Clip gradients if norm exceeds `max_grad_norm`
/// 4. **Adaptive Learning Rate**: Scale learning rate based on gradient magnitude
/// 5. **Momentum Update**: Accumulate gradients with momentum (if enabled)
/// 6. **Parameter Update**: Apply the final update: `param -= learning_rate * update`
///
/// # Memory Usage
///
/// - **Without momentum**: O(1) additional memory
/// - **With momentum**: O(n) additional memory for momentum buffers
/// - **Temporary allocations**: O(n) for gradient computations and updates
///
/// # Thread Safety
///
/// The optimizer is not thread-safe due to mutable state. Use separate
/// optimizer instances for concurrent optimization or implement external
/// synchronization.
#[derive(Debug)]
pub struct GDOptimizer {
    config: GDConfig,
    state: GDState,
    /// Stagnation multiplier for relaxed convergence criteria
    ///
    /// Used to relax convergence criteria when the optimizer appears
    /// to be making slow progress. Higher values make convergence
    /// detection more lenient.
    stagnation_multiplier: f64,

    /// Number of consecutive steps with minimal progress before
    /// applying stagnation-based convergence relaxation.
    stagnation_count: usize,
}

impl Clone for GDOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            stagnation_multiplier: self.stagnation_multiplier,
            stagnation_count: self.stagnation_count,
        }
    }
}

impl GDOptimizer {
    /// Create a new GD optimizer with the given configuration.
    pub fn new(config: GDConfig) -> Self {
        info!(
            "Creating GD optimizer '{}' with full configuration:",
            config.name
        );
        info!(
            "  Learning Rate: {}, Momentum: {}, Weight Decay: {}, Nesterov: {}",
            config.learning_rate, config.momentum, config.weight_decay, config.nesterov
        );
        info!(
            "  Max Grad Norm: {}, Adaptive LR: {}, Min LR: {}",
            config.max_grad_norm, config.adaptive_lr, config.min_learning_rate
        );
        info!("  Verbose: {}", config.verbose);
        if config.verbose {
            debug!("Creating GD optimizer with verbose logging enabled");
        }
        Self {
            config,
            state: GDState::new(),
            stagnation_multiplier: 10.0,
            stagnation_count: 5,
        }
    }
}

impl Optimizer for GDOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn config_string(&self) -> String {
        format!(
            "GD(lr={}, momentum={}, weight_decay={}, nesterov={}, max_grad_norm={}, adaptive_lr={})",
            self.config.learning_rate,
            self.config.momentum,
            self.config.weight_decay,
            self.config.nesterov,
            self.config.max_grad_norm,
            self.config.adaptive_lr
        )
    }

    fn step(&mut self, ctx: &mut OptimizationContext) -> StepResult {
        let gradients = &ctx.gradients;
        let weight_length = (&ctx.weights).len();
        if self.config.verbose {
            debug!(
                "GD Step {}: Processing {} tensors",
                self.state.iteration, weight_length
            );
        }

        // 1. Retrieve all data to CPU
        let mut all_weights_data: Vec<Vec<f32>> = (&ctx.weights).iter().map(|w| w.data()).collect();
        let all_grads_data: Vec<Vec<f32>> = gradients.iter().map(|g| g.data()).collect();

        // Initialize momentum if needed
        if self.state.momentum_buffer.len() != weight_length {
            self.state.momentum_buffer = all_weights_data
                .iter()
                .map(|w| vec![0.0; w.len()])
                .collect();
        }

        // 2. Calculate global gradient norm (after weight decay)
        let mut total_norm_sq = 0.0;
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

        let total_norm = total_norm_sq.sqrt();
        if self.config.verbose {
            debug!("Global gradient norm: {:.6e}", total_norm);
        }

        // 3. Determine scaling factor for clipping
        let clip_scale =
            if self.config.max_grad_norm > 0.0 && total_norm > self.config.max_grad_norm {
                let scale = self.config.max_grad_norm / total_norm;
                if self.config.verbose {
                    debug!(
                        "Clipping gradients: norm {:.6e} > max {:.6e}, scale = {:.6e}",
                        total_norm, self.config.max_grad_norm, scale
                    );
                }
                scale
            } else {
                1.0
            };

        // 4. Determine learning rate
        let mut lr = self.config.learning_rate;
        if self.config.adaptive_lr {
            let original_lr = lr;
            // Simple adaptive scaling: reduce LR if gradients are very large
            if total_norm > 1.0 {
                lr /= total_norm.sqrt();
            }
            if lr < self.config.min_learning_rate {
                lr = self.config.min_learning_rate;
            }
            if self.config.verbose && (lr != original_lr) {
                debug!(
                    "Adaptive LR: scaled from {:.6e} to {:.6e} (min: {:.6e})",
                    original_lr, lr, self.config.min_learning_rate
                );
            }
        }

        // 5. Apply updates
        for i in 0..weight_length {
            let w_vec = &mut all_weights_data[i];
            let g_vec = &all_grads_data[i];
            let m_vec = &mut self.state.momentum_buffer[i];
            // Statistics for verbose logging
            let mut update_sum = 0.0;
            let mut update_abs_max = 0.0;

            if self.config.verbose {
                debug!(
                    "Updating tensor {}: size = {}, lr = {:.6e}",
                    i,
                    w_vec.len(),
                    lr
                );
                // Log first 5 weights and gradients
                for j in 0..w_vec.len().min(5) {
                    debug!(
                        "  Weight[{}] = {:.6e}, Grad[{}] = {:.6e}, Momentum[{}] = {:.6e}",
                        j, w_vec[j], j, g_vec[j], j, m_vec[j]
                    );
                }
            }

            for j in 0..w_vec.len() {
                let mut g = g_vec[j] as f64;
                let w = w_vec[j] as f64;

                // Weight decay
                if self.config.weight_decay > 0.0 {
                    g += self.config.weight_decay * w;
                }

                // Clipping
                g *= clip_scale;

                // Momentum
                if self.config.momentum > 0.0 {
                    m_vec[j] = (self.config.momentum * m_vec[j] as f64 + g) as f32;

                    if self.config.nesterov {
                        g = self.config.momentum * m_vec[j] as f64 + g;
                    } else {
                        g = m_vec[j] as f64;
                    }
                }

                // Update
                let update = lr * g;
                w_vec[j] = (w - update) as f32;

                if self.config.verbose {
                    update_sum += update.abs();
                    if update.abs() > update_abs_max {
                        update_abs_max = update.abs();
                    }
                }
            }
            if self.config.verbose {
                let update_mean = update_sum / w_vec.len() as f64;
                debug!(
                    "Tensor {}: mean update = {:.6e}, max update = {:.6e}",
                    i, update_mean, update_abs_max
                );
            }
        }
        ctx.write_weights(&mut all_weights_data);

        StepResult {
            step_size: lr,
            convergence_info: ConvergenceInfo::default(),
        }
    }

    fn reset(&mut self) {
        self.state.reset();
    }

    fn name(&self) -> &str {
        &self.config.name
    }
    fn stagnation_multiplier(&self) -> f64 {
        self.stagnation_multiplier
    }
    fn stagnation_count(&self) -> usize {
        self.stagnation_count
    }

    fn set_stagnation_multiplier(&mut self, multiplier: f64) {
        self.stagnation_multiplier = multiplier;
    }

    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
    fn learning_rate(&self) -> Option<f64> {
        Some(self.config.learning_rate)
    }
    fn set_learning_rate(&mut self, lr: f64) {
        self.config.learning_rate = lr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gd_config_strict() {
        let config = GDConfig::strict();
        assert_eq!(config.learning_rate, 0.001);
        assert_eq!(config.momentum, 0.0);
        assert_eq!(config.max_grad_norm, 1.0);
        assert!(config.adaptive_lr);
        assert!(!config.verbose);
        let optimizer = GDOptimizer::new(config);
        assert_eq!(optimizer.name(), "GD-Debug");
    }
    #[test]
    fn test_gd_config_lax() {
        let config = GDConfig::lax();
        assert_eq!(config.learning_rate, 0.1);
        assert_eq!(config.momentum, 0.9);
        assert!(config.nesterov);
        assert_eq!(config.max_grad_norm, 100.0);
        assert!(!config.adaptive_lr);
        assert!(!config.verbose);
        let optimizer = GDOptimizer::new(config);
        assert_eq!(optimizer.name(), "GD-Lax");
    }
    #[test]
    fn test_gd_config_rosenbrock() {
        let config = GDConfig::rosenbrock();
        assert_eq!(config.learning_rate, 0.001);
        assert_eq!(config.momentum, 0.9);
        assert!(config.nesterov);
        assert_eq!(config.max_grad_norm, 10.0);
        assert!(config.adaptive_lr);
        assert!(!config.verbose);
        let optimizer = GDOptimizer::new(config);
        assert_eq!(optimizer.name(), "GD-Rosenbrock");
    }
    #[test]
    fn test_gd_config_debug() {
        let config = GDConfig::debug();
        assert_eq!(config.learning_rate, 0.01);
        assert_eq!(config.momentum, 0.0);
        assert!(!config.nesterov);
        assert_eq!(config.max_grad_norm, 10.0);
        assert!(config.adaptive_lr);
        assert!(config.verbose);
        let optimizer = GDOptimizer::new(config);
        assert_eq!(optimizer.name(), "GD-Debug");
    }

    #[test]
    fn test_gd_state_creation() {
        let state = GDState::new();
        assert_eq!(state.iteration(), 0);
        assert!(state.momentum_buffer.is_empty());
    }
    #[test]
    fn test_gd_state_reset() {
        let mut state = GDState::new();
        state.iteration = 10;
        state.momentum_buffer = vec![]; // Should be empty or populated
        state.reset();
        assert_eq!(state.iteration(), 0);
        assert!(state.momentum_buffer.is_empty());
    }

    #[test]
    fn test_gd_optimizer_creation() {
        let config = GDConfig::default();
        let optimizer = GDOptimizer::new(config);

        assert_eq!(optimizer.name(), "GD-Strict");
        assert_eq!(optimizer.state.iteration(), 0);
    }

    #[test]
    fn test_gd_with_momentum() {
        let mut config = GDConfig {
            momentum: 0.9,
            ..Default::default()
        };
        config.name = "GD-Momentum".to_string();
        let optimizer = GDOptimizer::new(config);
        assert_eq!(optimizer.name(), "GD-Momentum");
    }

    #[test]
    fn test_gd_with_nesterov() {
        let mut config = GDConfig {
            momentum: 0.9,
            nesterov: true,
            adaptive_lr: false, // Disable for predictable testing
            ..Default::default()
        };
        config.name = "GD-Nesterov".to_string();
        let optimizer = GDOptimizer::new(config);
        assert_eq!(optimizer.name(), "GD-Nesterov");
    }

    #[test]
    fn test_gd_reset() {
        let config = GDConfig::default();
        let mut optimizer = GDOptimizer::new(config);

        // Manually set some state
        optimizer.state.iteration = 5;

        optimizer.reset();
        assert_eq!(optimizer.state.iteration(), 0);

        assert!(optimizer.state.momentum_buffer.is_empty());
    }
    #[test]
    fn test_gd_learning_rate() {
        let config = GDConfig::default();
        let mut optimizer = GDOptimizer::new(config);
        assert_eq!(optimizer.learning_rate(), Some(0.01));
        optimizer.set_learning_rate(0.001);
        assert_eq!(optimizer.learning_rate(), Some(0.001));
    }
}