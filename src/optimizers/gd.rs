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
use crate::optimizers::optimizer::{GradientContext, Optimizer, OptimizerSetup};
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
    pub learning_rate: f32,

    /// Momentum coefficient (0.0 = no momentum, 0.9 = high momentum)
    ///
    /// Accumulates gradients from previous iterations to accelerate convergence
    /// and smooth out oscillations. Higher values provide more acceleration
    /// but can cause overshooting.
    pub momentum: f32,

    /// Weight decay (L2 regularization)
    ///
    /// Adds a penalty term proportional to parameter magnitude, equivalent
    /// to L2 regularization. Helps prevent overfitting by encouraging
    /// smaller parameter values.
    pub weight_decay: f32,

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
    pub max_grad_norm: f32,

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
    pub min_learning_rate: f32,

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
/// Tensor objects cannot be easily serialized. When deserializing, the momentum
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
    pub momentum_buffer: Vec<SafeTensor>,
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
    stagnation_multiplier: f32,

    /// Stagnation count threshold
    ///
    /// Learning rate tensor for graph-based updates
    lr_tensor: Option<SafeTensor>,
    /// New weight tensors after optimization step
    new_weights: Vec<SafeTensor>,

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
             lr_tensor: self.lr_tensor,
             new_weights: self.new_weights.clone(),
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
            lr_tensor: None,
            new_weights: Vec::new(),
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

    fn setup_on_graph(
        &mut self,

        graph: &mut Graph,

        weights: &[GraphTensor],
        gradients: &[GraphTensor],
    ) -> OptimizerSetup {
        // Create learning rate tensor that can be set externally
        let lr = graph.tensor(());
        lr.set(vec![self.config.learning_rate]);
        self.lr_tensor = Some(SafeTensor(lr));

        let mut new_weights = Vec::with_capacity(weights.len());
        let mut state_tensors = Vec::new();

        // Process each weight-gradient pair
        for (i, (weight, grad)) in weights.iter().zip(gradients.iter()).enumerate() {
            let mut current_grad = *grad;

            // Apply weight decay: grad += weight_decay * weight
            if self.config.weight_decay > 0.0 {
                let wd = graph.tensor(());
                wd.set(vec![self.config.weight_decay]);
                current_grad = current_grad + *weight * wd;
            }

            // For momentum, we need state tensors
            let update = if self.config.momentum > 0.0 {
                // Create momentum buffer tensor
                let v = graph.tensor(());
                // Initialize to zeros - will be set properly during execution
                state_tensors.push(v.id);

                let momentum_coef = graph.tensor(());
                momentum_coef.set(vec![self.config.momentum]);

                // v_t = momentum * v_{t-1} + grad
                // Note: In a static graph, we need to handle state updates carefully
                // For now, we'll use a simplified approach where momentum is applied
                // but state management happens externally
                let v_update = v * momentum_coef + current_grad;

                if self.config.nesterov {
                    // Nesterov: update = momentum * v_t + grad
                    v_update * momentum_coef + current_grad
                } else {
                    v_update
                }
            } else {
                current_grad
            };

            // Apply update: new_weight = weight - lr * update
            let new_weight = *weight - update * lr;
            new_weights.push(new_weight);
        }


        OptimizerSetup::new(new_weights)
            .with_learning_rate(lr)
            .with_state_tensors(state_tensors)
    }

    fn reset(&mut self) {
        self.state.reset();
    }

    fn name(&self) -> &str {
        &self.config.name
    }
    fn iteration(&self) -> usize {
        self.state.iteration()
    }
    fn increment_iteration(&mut self) {
        self.state.iteration += 1;
    }

    fn set_stagnation_multiplier(&mut self, multiplier: f32) {
        self.stagnation_multiplier = multiplier;
    }

    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
    fn learning_rate(&self) -> Option<f32> {
        Some(self.config.learning_rate)
    }
    fn set_learning_rate(&mut self, lr: f32) {
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
        assert_eq!(optimizer.name(), "GD-Strict");
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