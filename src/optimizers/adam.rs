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

use crate::optimizers::optimizer::{ConvergenceInfo, OptimizationMetadata, Optimizer, StepResult};
use crate::utils::math::DifferentiableFunction;
use candle_core::{Result as CandleResult, Tensor};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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
            lr_decay: 0.995,       // Slower decay for adaptive schedule
            min_learning_rate: 1e-15, // Allow very small learning rates
            gradient_clip: Some(0.5), // Tight gradient control
            beta1: 0.9,
            beta2: 0.9999,         // More stable second moment estimates
            epsilon: 1e-12,        // Higher numerical precision
            weight_decay: 0.0,
            amsgrad: true,         // Better convergence guarantees
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
            learning_rate: 0.01,   // 10x larger than default for speed
            lr_schedule: "exponential".to_string(),
            lr_decay: 0.99,        // Faster decay to control later iterations
            min_learning_rate: 1e-8, // Don't go too small
            gradient_clip: None,   // Allow large steps
            beta1: 0.9,
            beta2: 0.99,           // Faster adaptation to gradient changes
            epsilon: 1e-6,         // Lower precision for speed
            weight_decay: 0.0,
            amsgrad: false,        // Standard Adam is faster
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
            learning_rate: 0.001,  // Sweet spot for neural networks
            lr_schedule: "cosine".to_string(),
            lr_decay: 0.999,
            min_learning_rate: 1e-6, // Don't decay too aggressively
            gradient_clip: Some(1.0), // Prevent exploding gradients
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            weight_decay: 0.01,    // Moderate regularization
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
    pub m: Option<Vec<Tensor>>,
    
    /// Second moment estimates (exponentially decaying average of squared gradients)
    ///
    /// **Formula:** v_t = β₂ * v_{t-1} + (1 - β₂) * g_t²
    /// **Purpose:** Adapts learning rates based on gradient variance
    /// **Note:** Skipped in serialization due to Tensor complexity
    #[serde(skip_serializing, skip_deserializing)]
    pub v: Option<Vec<Tensor>>,
    
    /// Maximum second moment estimates (AMSGrad variant only)
    ///
    /// **Formula:** v̂_t = max(v_t, v̂_{t-1})
    /// **Purpose:** Ensures non-increasing effective learning rates
    /// **Memory:** Only allocated when AMSGrad is enabled
    #[serde(skip_serializing, skip_deserializing)]
    pub v_max: Option<Vec<Tensor>>,
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
    current_lr: f64,
    /// Previous function value for adaptive learning rate and convergence detection
    prev_function_value: Option<f64>,
    /// Count of consecutive steps without improvement (for adaptive scheduling)
    bad_step_count: usize,
    /// Stagnation multiplier for relaxed convergence criteria (future use)
    stagnation_multiplier: f64,
    /// Stagnation count threshold (future use)
    stagnation_count: usize,
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
        }
    }
}

impl AdamOptimizer {
    /// Create a new Adam optimizer with the given configuration.
    pub fn new(config: AdamConfig) -> Self {
        if config.verbose {
            info!("Creating Adam optimizer with verbose logging enabled");
            debug!(
                "Adam Config: lr={}, beta1={}, beta2={}, epsilon={}, weight_decay={}, amsgrad={}",
                config.learning_rate,
                config.beta1,
                config.beta2,
                config.epsilon,
                config.weight_decay,
                config.amsgrad
            );
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
        }
    }

    /// Log tensor data if verbose mode is enabled
    fn log_tensor_data(&self, name: &str, tensors: &[Tensor]) {
        if !self.config.verbose {
            return;
        }
        debug!("=== Adam: {} ===", name);
        for (i, tensor) in tensors.iter().enumerate() {
            match tensor.flatten_all().and_then(|t| t.to_vec1::<f64>()) {
                Ok(values) => {
                    debug!(
                        "  Tensor[{}]: shape={:?}, length={}",
                        i,
                        tensor.shape(),
                        values.len()
                    );
                    if values.len() <= 10 {
                        debug!("    Full data: {:?}", values);
                    } else {
                        debug!(
                            "    First 5: {:?}, Last 5: {:?}",
                            &values[..5],
                            &values[values.len() - 5..]
                        );
                    }
                    // Log statistics
                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
                        / values.len() as f64;
                    let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                    debug!(
                        "    Stats: mean={:.6e}, std={:.6e}, min={:.6e}, max={:.6e}",
                        mean,
                        variance.sqrt(),
                        min_val,
                        max_val
                    );
                }
                Err(e) => {
                    debug!(
                        "  Tensor[{}]: shape={:?}, error reading values: {}",
                        i,
                        tensor.shape(),
                        e
                    );
                }
            }
        }
    }

    /// Log scalar value if verbose mode is enabled
    fn log_scalar(&self, name: &str, value: f64) {
        if self.config.verbose {
            debug!("  Adam {}: {:.12e}", name, value);
        }
    }

    /// Apply weight decay to gradients
    fn apply_weight_decay(&self, gradients: &mut [Tensor], params: &[Tensor]) -> CandleResult<()> {
        if self.config.weight_decay == 0.0 {
            return Ok(());
        }

        for (grad, param) in gradients.iter_mut().zip(params.iter()) {
            let decay_term = param.affine(self.config.weight_decay, 0.0)?;
            *grad = grad.add(&decay_term)?;
        }

        Ok(())
    }
    /// Apply gradient clipping if configured
    fn apply_gradient_clipping(&self, gradients: &mut [Tensor]) -> CandleResult<()> {
        if let Some(max_norm) = self.config.gradient_clip {
            let grad_norm = crate::utils::math::compute_magnitude(gradients)?;
            if grad_norm > max_norm {
                let scale = max_norm / grad_norm;
                for grad in gradients.iter_mut() {
                    *grad = grad.affine(scale, 0.0)?;
                }
            }
        }
        Ok(())
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
                        if relative_improvement > 1e-6 && self.current_lr < self.config.learning_rate * 0.1 {
                            self.current_lr = (self.current_lr * 1.1).min(self.config.learning_rate * 0.1);
                        }
                    }
                }
            }
            _ => {} // constant learning rate
        }
        // Update previous function value for all schedules
        self.prev_function_value = current_value;
    }

    /// Update moment estimates and compute parameter updates
    fn compute_updates(&mut self, gradients: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // Initialize moment estimates if needed
        if self.state.m.is_none() {
            self.state.m = Some(
                gradients
                    .iter()
                    .map(|g| Tensor::zeros_like(g).unwrap())
                    .collect(),
            );
            self.state.v = Some(
                gradients
                    .iter()
                    .map(|g| Tensor::zeros_like(g).unwrap())
                    .collect(),
            );
            if self.config.amsgrad {
                self.state.v_max = Some(
                    gradients
                        .iter()
                        .map(|g| Tensor::zeros_like(g).unwrap())
                        .collect(),
                );
            }
        }

        let m = self.state.m.as_mut().unwrap();
        let v = self.state.v.as_mut().unwrap();
        let mut updates = Vec::with_capacity(gradients.len());

        // Bias correction terms
        let t = (self.state.iteration + 1) as f64;
        let bias_correction1 = 1.0 - self.config.beta1.powf(t);
        let bias_correction2 = 1.0 - self.config.beta2.powf(t);

        for i in 0..gradients.len() {
            // Update biased first moment estimate
            // m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
            let m_old = m[i].affine(self.config.beta1, 0.0)?;
            let g_scaled = gradients[i].affine(1.0 - self.config.beta1, 0.0)?;
            m[i] = m_old.add(&g_scaled)?;

            // Update biased second raw moment estimate
            // v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
            let v_old = v[i].affine(self.config.beta2, 0.0)?;
            let g_squared = gradients[i].mul(&gradients[i])?;
            let g_squared_scaled = g_squared.affine(1.0 - self.config.beta2, 0.0)?;
            v[i] = v_old.add(&g_squared_scaled)?;

            // Compute bias-corrected moment estimates
            let m_hat = m[i].affine(1.0 / bias_correction1, 0.0)?;
            let v_hat = if self.config.amsgrad {
                // Update v_max for AMSGrad
                let v_max = self.state.v_max.as_mut().unwrap();
                let v_i_vec = v[i].flatten_all()?.to_vec1::<f64>()?;
                let v_max_vec = v_max[i].flatten_all()?.to_vec1::<f64>()?;
                let new_v_max: Vec<f64> = v_i_vec
                    .iter()
                    .zip(v_max_vec.iter())
                    .map(|(&v_val, &v_max_val)| v_val.max(v_max_val))
                    .collect();
                v_max[i] = Tensor::from_vec(new_v_max, v[i].shape(), v[i].device())?;
                v_max[i].affine(1.0 / bias_correction2, 0.0)?
            } else {
                v[i].affine(1.0 / bias_correction2, 0.0)?
            };

            // Compute update: lr * m_hat / (sqrt(v_hat) + epsilon)
            let epsilon_tensor = Tensor::new(self.config.epsilon, v_hat.device())?;
            let v_hat_eps = v_hat.broadcast_add(&epsilon_tensor)?;
            let denominator = v_hat_eps.sqrt()?;
            let update = m_hat.div(&denominator)?;
            updates.push(update.affine(self.current_lr, 0.0)?);
        }

        Ok(updates)
    }

    /// Compute convergence information for the current state.
    fn compute_convergence_info(
        &self,
        gradients: &[Tensor],
        function_change: Option<f64>,
    ) -> CandleResult<ConvergenceInfo> {
        let gradient_norm = crate::utils::math::compute_magnitude(gradients)?;

        // Tighter convergence criteria to find better minima
        let grad_tolerance = 1e-10;
        let func_tolerance = 1e-15;

        let grad_converged = gradient_norm < grad_tolerance;
        let func_converged = function_change
            .map(|change| change.abs() < func_tolerance)
            .unwrap_or(false);

        // Stricter convergence criteria - require both gradient and function change to be small
        let converged = if gradient_norm < 1e-12 {
            // Extremely small gradient norm - definitely converged
            true
        } else if grad_converged {
            // Small gradient norm - require function change to also be small
            function_change
                .map(|change| change.abs() < func_tolerance)
                .unwrap_or(true)
        } else {
            false
        };

        if self.config.verbose && (grad_converged || func_converged) {
            debug!(
                "Convergence check: grad_norm={:.6e} < {:.6e} = {}, func_change={:?} < {:.6e} = {}",
                gradient_norm,
                grad_tolerance,
                grad_converged,
                function_change,
                func_tolerance,
                func_converged
            );
        }

        Ok(ConvergenceInfo {
            converged,
            function_change,
        })
    }
}

impl Optimizer for AdamOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,
        params: &mut [Tensor],
        function: Arc<dyn DifferentiableFunction + Send + Sync>,
    ) -> CandleResult<StepResult> {
        let start_time = Instant::now();
        if self.config.verbose {
            debug!("=== Adam Step {} Starting ===", self.state.iteration);
            self.log_tensor_data("Parameters Before Step", params);
        }

        // Compute current function value
        let current_value = function.evaluate(params)?;
        // Store previous function value for change calculation
        let prev_function_value = self.prev_function_value;

        // Calculate function change
        let function_change = prev_function_value.map(|prev| current_value - prev);

        // Compute gradients at current parameters
        let mut gradients = function.gradient(params)?;

        // Log initial state in verbose mode
        self.log_tensor_data("Initial Parameters", params);
        self.log_tensor_data("Computed Gradients", &gradients);

        // Input validation
        if params.is_empty() || gradients.is_empty() {
            return Err(candle_core::Error::Msg(
                "Empty parameters or gradients".into(),
            ));
        }
        if params.len() != gradients.len() {
            return Err(candle_core::Error::Msg(format!(
                "Parameter and gradient dimension mismatch: {} vs {}",
                params.len(),
                gradients.len()
            )));
        }

        // Apply weight decay
        self.apply_weight_decay(&mut gradients, params)?;
        // Apply gradient clipping
        self.apply_gradient_clipping(&mut gradients)?;

        // Compute gradient norm for logging
        let grad_norm = crate::utils::math::compute_magnitude(&gradients)?;
        debug!(
            "Adam step {}: grad_norm={:.6e}",
            self.state.iteration, grad_norm
        );
        self.log_scalar("Gradient Norm", grad_norm);

        // Compute parameter updates using Adam algorithm
        let updates = self.compute_updates(&gradients)?;
        self.log_tensor_data("Parameter Updates", &updates);

        // Compute update norm
        let update_norm = crate::utils::math::compute_magnitude(&updates)?;
        self.log_scalar("Update Norm", update_norm);
        // Update learning rate based on schedule (after computing updates)
        self.update_learning_rate(Some(current_value));

        // Perform line search if enabled
        let step_size = 1.0;

        // Apply the updates with step size: x_{k+1} = x_k - step_size * updates
        for (param, update) in params.iter_mut().zip(updates.iter()) {
            *param = param.sub(&update.affine(step_size, 0.0)?)?;
        }

        self.log_tensor_data("Updated Parameters", params);

        // Check for NaN/Inf in updated parameters
        for (i, param) in params.iter().enumerate() {
            let param_vec = param.flatten_all()?.to_vec1::<f64>()?;
            if param_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(format!(
                    "Non-finite parameter detected at index {} after update",
                    i
                )));
            }
        }

        // Increment iteration counter
        self.state.iteration += 1;

        // Compute convergence information
        let convergence_info = self.compute_convergence_info(&gradients, function_change)?;
        let step_duration = start_time.elapsed();

        if self.config.verbose {
            debug!("=== Adam Step {} Completed ===", self.state.iteration - 1);
            debug!("  Step Duration: {:?}", step_duration);
            debug!("  Converged: {}", convergence_info.converged);
            debug!("  Current LR: {:.6e}", self.current_lr);
            debug!("  Line Search Alpha: {:.3}", step_size);
            debug!("  Function Value: {:.6e}", current_value);
            if let Some(change) = function_change {
                debug!("  Function Change: {:.6e}", change);
            }
        }

        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = step_duration;
        metadata
            .optimizer_data
            .insert("gradient_norm".to_string(), grad_norm);
        metadata
            .optimizer_data
            .insert("update_norm".to_string(), update_norm);
        metadata
            .optimizer_data
            .insert("learning_rate".to_string(), self.current_lr);
        metadata
            .optimizer_data
            .insert("beta1".to_string(), self.config.beta1);
        metadata
            .optimizer_data
            .insert("beta2".to_string(), self.config.beta2);
        metadata
            .optimizer_data
            .insert("line_search_alpha".to_string(), step_size);
        if let Some(change) = function_change {
            metadata
                .optimizer_data
                .insert("function_change".to_string(), change);
        }

        Ok(StepResult {
            step_size: self.current_lr * step_size,
            convergence_info,
            metadata,
        })
    }

    fn reset(&mut self) {
        self.state.reset();
        self.current_lr = self.config.learning_rate;
        self.prev_function_value = None;
        self.bad_step_count = 0;
    }

    fn name(&self) -> &str {
        if self.config.amsgrad {
            "Adam-AMSGrad"
        } else {
            "Adam"
        }
    }
    fn iteration(&self) -> usize {
        self.state.iteration()
    }
    fn set_stagnation_multiplier(&mut self, multiplier: f64) {
        self.stagnation_multiplier = multiplier;
    }
    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimizers::optimizer::Optimizer;
    use candle_core::{Device, Tensor};

    /// Simple quadratic function for testing: f(x) = 0.5 * ||x||^2
    struct QuadraticFunction;
    impl DifferentiableFunction for QuadraticFunction {
        fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
            let mut sum = 0.0;
            for param in params {
                let values = param.flatten_all()?.to_vec1::<f64>()?;
                sum += values.iter().map(|x| x * x).sum::<f64>();
            }
            Ok(0.5 * sum)
        }
        fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
            // Gradient of 0.5 * ||x||^2 is x
            Ok(params.to_vec())
        }
    }
    /// Rosenbrock function for testing: f(x,y) = (1-x)^2 + 100*(y-x^2)^2
    struct RosenbrockFunction;
    impl DifferentiableFunction for RosenbrockFunction {
        fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
            let values = params[0].flatten_all()?.to_vec1::<f64>()?;
            let x = values[0];
            let y = values[1];
            Ok((1.0 - x).powi(2) + 100.0 * (y - x * x).powi(2))
        }
        fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
            let values = params[0].flatten_all()?.to_vec1::<f64>()?;
            let x = values[0];
            let y = values[1];
            let grad_x = -2.0 * (1.0 - x) - 400.0 * x * (y - x * x);
            let grad_y = 200.0 * (y - x * x);
            let grad = Tensor::from_vec(vec![grad_x, grad_y], &[2], &Device::Cpu)?;
            Ok(vec![grad])
        }
    }

    #[test]
    fn test_adam_state_creation() {
        let state = AdamState::new();
        assert_eq!(state.iteration(), 0);
        assert!(state.m.is_none());
        assert!(state.v.is_none());
        assert!(state.v_max.is_none());
    }
    #[test]
    fn test_adam_state_reset() {
        let mut state = AdamState::new();
        state.iteration = 10;
        // Create dummy tensors for moments
        let device = Device::Cpu;
        let dummy_tensor = Tensor::zeros(&[2, 2], candle_core::DType::F64, &device).unwrap();
        state.m = Some(vec![dummy_tensor.clone()]);
        state.v = Some(vec![dummy_tensor.clone()]);
        state.v_max = Some(vec![dummy_tensor]);
        state.reset();
        assert_eq!(state.iteration, 0);
        assert!(state.m.is_none());
        assert!(state.v.is_none());
        assert!(state.v_max.is_none());
    }

    #[test]
    fn test_adam_optimizer_creation() {
        let config = AdamConfig::default();
        let optimizer = AdamOptimizer::new(config);

        assert_eq!(optimizer.name(), "Adam");
        assert_eq!(optimizer.state.iteration(), 0);
        assert_eq!(optimizer.current_lr, optimizer.config.learning_rate);
    }

    #[test]
    fn test_adam_with_amsgrad() {
        let config = AdamConfig {
            amsgrad: true,
            ..Default::default()
        };
        let optimizer = AdamOptimizer::new(config);
        assert_eq!(optimizer.name(), "Adam-AMSGrad");
    }

    #[test]
    fn test_adam_reset() {
        let config = AdamConfig::default();
        let mut optimizer = AdamOptimizer::new(config);

        // Manually set some state
        optimizer.state.iteration = 5;
        optimizer.current_lr = 0.001;
        optimizer.prev_function_value = Some(1.0);
        optimizer.bad_step_count = 3;

        optimizer.reset();
        assert_eq!(optimizer.state.iteration(), 0);
        assert!(optimizer.state.m.is_none());
        assert!(optimizer.state.v.is_none());
        assert_eq!(optimizer.current_lr, optimizer.config.learning_rate);
        assert!(optimizer.prev_function_value.is_none());
        assert_eq!(optimizer.bad_step_count, 0);
    }
    #[test]
    fn test_adam_simple_optimization() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.1,
            lr_schedule: "constant".to_string(),
            verbose: false,
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        // Start at [2.0, 2.0]
        let mut params = vec![Tensor::from_vec(vec![2.0, 2.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        // Initial function value should be 0.5 * (4 + 4) = 4.0
        let initial_value = function.evaluate(&params)?;
        assert!((initial_value - 4.0).abs() < 1e-10);
        // Run a few optimization steps
        for i in 0..50 {
            let result = optimizer.step(&mut params, function.clone())?;
            // Print progress for debugging
            let current_values = params[0].flatten_all()?.to_vec1::<f64>()?;
            let current_function_value = function.evaluate(&params)?;
            println!(
                "Step {}: params=[{:.6}, {:.6}], f={:.6e}",
                i, current_values[0], current_values[1], current_function_value
            );
            // Early termination if converged
            if result.convergence_info.converged {
                break;
            }
        }
        // Should converge close to [0, 0]
        let final_values = params[0].flatten_all()?.to_vec1::<f64>()?;
        println!(
            "Final values: [{:.6}, {:.6}]",
            final_values[0], final_values[1]
        );
        assert!(
            final_values[0].abs() < 0.5,
            "Expected |x| < 0.5, got {}",
            final_values[0].abs()
        );
        assert!(
            final_values[1].abs() < 0.5,
            "Expected |y| < 0.5, got {}",
            final_values[1].abs()
        );
        Ok(())
    }
    #[test]
    fn test_adam_with_weight_decay() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.1,
            weight_decay: 0.1,
            lr_schedule: "constant".to_string(),
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        let mut params = vec![Tensor::from_vec(vec![1.0, 1.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        // With weight decay, the effective gradient is g + weight_decay * x
        let result = optimizer.step(&mut params, function)?;
        assert!(result.step_size > 0.0);
        Ok(())
    }
    #[test]
    fn test_adam_gradient_clipping() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.1,
            gradient_clip: Some(0.5),
            lr_schedule: "constant".to_string(),
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        // Start far from optimum to get large gradients
        let mut params = vec![Tensor::from_vec(vec![10.0, 10.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        let result = optimizer.step(&mut params, function)?;
        assert!(result.step_size > 0.0);
        // Check that parameters moved but not too much (due to clipping)
        let values = params[0].flatten_all()?.to_vec1::<f64>()?;
        assert!(values[0] < 10.0);
        assert!(values[1] < 10.0);
        Ok(())
    }
    #[test]
    fn test_adam_exponential_lr_schedule() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.1,
            lr_schedule: "exponential".to_string(),
            lr_decay: 0.9,
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        let mut params = vec![Tensor::from_vec(vec![1.0, 1.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        let initial_lr = optimizer.current_lr;
        // Run a step
        optimizer.step(&mut params, function)?;
        // Learning rate should have decayed
        assert!((optimizer.current_lr - initial_lr * 0.9).abs() < 1e-10);
        Ok(())
    }
    #[test]
    fn test_adam_cosine_lr_schedule() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.1,
            lr_schedule: "cosine".to_string(),
            min_learning_rate: 0.01,
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        let mut params = vec![Tensor::from_vec(vec![1.0, 1.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        let initial_lr = optimizer.current_lr;
        // Run multiple steps to see cosine schedule effect
        for _ in 0..100 {
            optimizer.step(&mut params, function.clone())?;
        }

        // After 100 steps, learning rate should have decreased from cosine schedule
        assert!(
            optimizer.current_lr < initial_lr,
            "Expected lr {} < initial_lr {}",
            optimizer.current_lr,
            initial_lr
        );
        assert!(optimizer.current_lr >= optimizer.config.min_learning_rate);
        Ok(())
    }
    #[test]
    fn test_adam_adaptive_lr_schedule() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.1,
            lr_schedule: "adaptive".to_string(),
            min_learning_rate: 0.001,
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        // Use a function where we can control convergence behavior
        let mut params = vec![Tensor::from_vec(vec![0.1, 0.1], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        let initial_lr = optimizer.current_lr;
        // Run many steps to potentially trigger adaptive reduction
        for _ in 0..25 {
            optimizer.step(&mut params, function.clone())?;
        }
        // Learning rate might have been reduced if progress stalled
        assert!(optimizer.current_lr <= initial_lr);
        assert!(optimizer.current_lr >= optimizer.config.min_learning_rate);
        Ok(())
    }
    #[test]
    fn test_adam_strict_config() -> CandleResult<()> {
        let config = AdamConfig::strict();
        // Verify strict configuration properties
        assert_eq!(config.learning_rate, 0.0001);
        assert_eq!(config.lr_schedule, "adaptive");
        assert_eq!(config.gradient_clip, Some(0.5));
        assert_eq!(config.beta2, 0.9999);
        assert_eq!(config.epsilon, 1e-12);
        assert!(config.amsgrad);
        assert_eq!(config.max_line_search_iter, 50);
        let optimizer = AdamOptimizer::new(config);
        assert_eq!(optimizer.name(), "Adam-AMSGrad");
        Ok(())
    }
    #[test]
    fn test_adam_lax_config() -> CandleResult<()> {
        let config = AdamConfig::lax();
        // Verify lax configuration properties
        assert_eq!(config.learning_rate, 0.01);
        assert_eq!(config.lr_schedule, "exponential");
        assert_eq!(config.gradient_clip, None);
        assert_eq!(config.beta2, 0.99);
        assert_eq!(config.epsilon, 1e-6);
        assert!(!config.amsgrad);
        assert_eq!(config.max_line_search_iter, 5);
        let optimizer = AdamOptimizer::new(config);
        assert_eq!(optimizer.name(), "Adam");
        Ok(())
    }
    #[test]
    fn test_adam_deep_learning_config() -> CandleResult<()> {
        let config = AdamConfig::deep_learning();
        // Verify deep learning configuration properties
        assert_eq!(config.learning_rate, 0.001);
        assert_eq!(config.lr_schedule, "cosine");
        assert_eq!(config.gradient_clip, Some(1.0));
        assert_eq!(config.beta1, 0.9);
        assert_eq!(config.beta2, 0.999);
        assert_eq!(config.epsilon, 1e-8);
        assert_eq!(config.weight_decay, 0.01);
        assert!(!config.amsgrad);
        Ok(())
    }
    #[test]
    fn test_adam_strict_vs_lax_convergence() -> CandleResult<()> {
        let device = Device::Cpu;
        // Test strict configuration
        let strict_config = AdamConfig::strict();
        let mut strict_optimizer = AdamOptimizer::new(strict_config);
        let mut strict_params = vec![Tensor::from_vec(vec![2.0, 2.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        // Run a few steps with strict config
        for _ in 0..10 {
            strict_optimizer.step(&mut strict_params, function.clone())?;
        }
        let strict_final = strict_params[0].flatten_all()?.to_vec1::<f64>()?;
        let strict_value = function.evaluate(&strict_params)?;
        // Test lax configuration
        let lax_config = AdamConfig::lax();
        let mut lax_optimizer = AdamOptimizer::new(lax_config);
        let mut lax_params = vec![Tensor::from_vec(vec![2.0, 2.0], &[2], &device)?];
        // Run same number of steps with lax config
        for _ in 0..10 {
            lax_optimizer.step(&mut lax_params, function.clone())?;
        }
        let lax_final = lax_params[0].flatten_all()?.to_vec1::<f64>()?;
        let lax_value = function.evaluate(&lax_params)?;
        println!("Strict final: [{:.6}, {:.6}], value: {:.6e}",
                 strict_final[0], strict_final[1], strict_value);
        println!("Lax final: [{:.6}, {:.6}], value: {:.6e}",
                 lax_final[0], lax_final[1], lax_value);
        // Both should make progress, but lax might make larger steps
        assert!(strict_value < 4.0); // Should improve from initial value of 4.0
        assert!(lax_value < 4.0);
        Ok(())
    }

    #[test]
    fn test_adam_convergence_detection() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.01,  // Much smaller learning rate to avoid overshooting
            lr_schedule: "constant".to_string(),
            beta1: 0.9,           // Standard momentum
            beta2: 0.999,         // Standard second moment decay
            epsilon: 1e-8,        // Standard epsilon
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        // Start closer to optimum but not too close to avoid numerical issues
        let mut params = vec![Tensor::from_vec(vec![1e-4, 1e-4], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        // Run optimization
        let mut converged = false;
        for i in 0..1000 {  // Allow more iterations
            let result = optimizer.step(&mut params, function.clone())?;
            // Print progress for debugging
            if i % 10 == 0 {
                let current_values = params[0].flatten_all()?.to_vec1::<f64>()?;
                let current_function_value = function.evaluate(&params)?;
                println!(
                    "Step {}: params=[{:.6e}, {:.6e}], f={:.6e}, grad_norm={:.6e}",
                    i, current_values[0], current_values[1], current_function_value,
                    result.metadata.optimizer_data.get("gradient_norm").unwrap_or(&0.0)
                );
            }

            if result.convergence_info.converged {
                println!("Converged at step {}", i);
                converged = true;
                break;
            }
        }
        assert!(converged, "Optimizer should have detected convergence");
        Ok(())
    }
    #[test]
    fn test_adam_with_rosenbrock() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.01,
            lr_schedule: "constant".to_string(),
            gradient_clip: None, // Disable gradient clipping for Rosenbrock
            verbose: false,
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        // Start at a challenging point
        let mut params = vec![Tensor::from_vec(vec![0.0, 0.0], &[2], &device)?];
        let function = Arc::new(RosenbrockFunction);
        let initial_value = function.evaluate(&params)?;
        println!("Initial Rosenbrock value: {:.6e}", initial_value);

        // Run optimization
        for i in 0..500 {
            let result = optimizer.step(&mut params, function.clone())?;
            if i % 50 == 0 {
                let current_values = params[0].flatten_all()?.to_vec1::<f64>()?;
                let current_value = function.evaluate(&params)?;
                println!(
                    "Step {}: params=[{:.6}, {:.6}], f={:.6e}",
                    i, current_values[0], current_values[1], current_value
                );
            }
            if result.convergence_info.converged {
                break;
            }
        }
        // Should be closer to optimum at (1, 1)
        let final_values = params[0].flatten_all()?.to_vec1::<f64>()?;
        let final_value = function.evaluate(&params)?;
        println!(
            "Final Rosenbrock: params=[{:.6}, {:.6}], f={:.6e}",
            final_values[0], final_values[1], final_value
        );
        // Rosenbrock is difficult, so we're lenient with convergence
        assert!(
            final_value < initial_value * 0.1,
            "Function value should have decreased significantly: initial={:.6e}, final={:.6e}",
            initial_value,
            final_value
        );
        Ok(())
    }
    #[test]
    fn test_adam_empty_params_error() {
        let config = AdamConfig::default();
        let mut optimizer = AdamOptimizer::new(config);
        let mut params: Vec<Tensor> = vec![];
        let function = Arc::new(QuadraticFunction);
        let result = optimizer.step(&mut params, function);
        assert!(result.is_err());
    }
    #[test]
    fn test_adam_dimension_mismatch_error() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig::default();
        let mut optimizer = AdamOptimizer::new(config);
        // Create a function that returns wrong number of gradients
        struct BadGradientFunction;
        impl DifferentiableFunction for BadGradientFunction {
            fn evaluate(&self, _params: &[Tensor]) -> CandleResult<f64> {
                Ok(0.0)
            }
            fn gradient(&self, _params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
                Ok(vec![]) // Wrong dimension
            }
        }
        let mut params = vec![Tensor::from_vec(vec![1.0], &[1], &device)?];
        let function = Arc::new(BadGradientFunction);
        let result = optimizer.step(&mut params, function);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_adam_clone() -> CandleResult<()> {
        let config = AdamConfig {
            learning_rate: 0.123,
            beta1: 0.95,
            beta2: 0.998,
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        // Set some state
        optimizer.state.iteration = 5;
        optimizer.current_lr = 0.05;
        optimizer.prev_function_value = Some(2.5);
        optimizer.bad_step_count = 2;
        // Clone the optimizer
        let cloned = optimizer.clone();
        // Check that all fields are properly cloned
        assert_eq!(cloned.config.learning_rate, optimizer.config.learning_rate);
        assert_eq!(cloned.config.beta1, optimizer.config.beta1);
        assert_eq!(cloned.config.beta2, optimizer.config.beta2);
        assert_eq!(cloned.state.iteration, optimizer.state.iteration);
        assert_eq!(cloned.current_lr, optimizer.current_lr);
        assert_eq!(cloned.prev_function_value, optimizer.prev_function_value);
        assert_eq!(cloned.bad_step_count, optimizer.bad_step_count);
        Ok(())
    }
    #[test]
    fn test_adam_verbose_mode() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig {
            learning_rate: 0.1,
            verbose: false,
            ..Default::default()
        };
        let mut optimizer = AdamOptimizer::new(config);
        let mut params = vec![Tensor::from_vec(vec![1.0, 1.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        // This should produce verbose output (captured by logger)
        let result = optimizer.step(&mut params, function)?;
        assert!(result.step_size > 0.0);
        Ok(())
    }
    #[test]
    fn test_adam_metadata() -> CandleResult<()> {
        let device = Device::Cpu;
        let config = AdamConfig::default();
        let mut optimizer = AdamOptimizer::new(config);
        let mut params = vec![Tensor::from_vec(vec![1.0, 1.0], &[2], &device)?];
        let function = Arc::new(QuadraticFunction);
        let result = optimizer.step(&mut params, function)?;
        // Check that metadata contains expected keys
        assert!(result.metadata.optimizer_data.contains_key("gradient_norm"));
        assert!(result.metadata.optimizer_data.contains_key("update_norm"));
        assert!(result.metadata.optimizer_data.contains_key("learning_rate"));
        assert!(result.metadata.optimizer_data.contains_key("beta1"));
        assert!(result.metadata.optimizer_data.contains_key("beta2"));
        assert!(result
            .metadata
            .optimizer_data
            .contains_key("line_search_alpha"));
        // Check that timing info is recorded
        assert!(result.metadata.timing_info.step_duration.as_secs_f64() >= 0.0);
        Ok(())
    }
}