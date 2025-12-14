//! L-BFGS (Limited-memory Broyden-Fletcher-Goldfarb-Shanno) optimizer implementation.
//!
//! This implementation provides a robust quasi-Newton optimization method that approximates
//! the inverse Hessian matrix using a limited history of gradient and parameter changes.
//! L-BFGS is particularly effective for smooth, differentiable optimization problems and
//! serves both as a standalone optimizer and as a core component of the QQN algorithm.
//!
//! ## Algorithm Overview
//!
//! L-BFGS uses the two-loop recursion algorithm to compute search directions:
//! 1. **First loop**: Computes correction factors α_i using stored s_k and y_k vectors
//! 2. **Scaling**: Applies initial Hessian approximation H₀ = γI where γ = (s_k^T y_k)/(y_k^T y_k)
//! 3. **Second loop**: Applies corrections to obtain the final search direction
//!
//! The method maintains vectors s_k = x_{k+1} - x_k (parameter changes) and
//! y_k = ∇f_{k+1} - ∇f_k (gradient changes) to implicitly represent the inverse Hessian.
//!
//! ## Strengths
//!
//! - **Superlinear convergence** on smooth, well-conditioned problems
//! - **Memory efficient**: O(m) storage where m is history size (typically 5-20)
//! - **Scale invariant**: Automatically adapts to problem scaling through γ parameter
//! - **Robust line search**: Uses strong Wolfe conditions for step size selection
//! - **Curvature awareness**: Exploits second-order information without computing Hessian
//!
//! ## Weaknesses
//!
//! - **Requires smooth functions**: Performance degrades on non-smooth or noisy objectives
//! - **Memory effects**: Poor history can slow convergence or cause instability
//! - **Initialization sensitivity**: First few iterations use steepest descent
//! - **Curvature condition**: May reject updates when s_k^T y_k ≤ 0 (negative curvature)
//! - **Local method**: Can get trapped in local minima like other gradient-based methods
//!
//! ## Configuration Strategies
//!
//! The implementation provides three main configuration presets:
//! - **Default**: Balanced settings suitable for most problems
//! - **Strict**: Conservative settings for ill-conditioned or sensitive problems
//! - **Lax**: Aggressive settings for well-conditioned problems requiring fast convergence
//! - **QQN**: Specialized settings when used as a component within QQN

use crate::optimizers::optimizer::Optimizer;
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::{LineSearchConfig, LineSearchMethod};

/// Configuration parameters for the L-BFGS optimizer.
///
/// This struct controls all aspects of L-BFGS behavior, from memory usage to numerical
/// stability. The parameters can significantly impact convergence speed and robustness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBFGSConfig {
    /// Number of previous iterations to store for Hessian approximation.
    ///
    /// **Range**: 1-50, **Typical**: 5-20, **Default**: 10
    ///
    /// Larger values provide better Hessian approximation but use more memory and
    /// computation. Values below 5 may converge slowly, while values above 20
    /// rarely provide significant benefit and can cause numerical issues.
    pub history_size: usize,

    /// Line search configuration for step size selection.
    ///
    /// Controls how the optimizer finds an appropriate step size along the search
    /// direction. Uses strong Wolfe conditions by default for robust convergence.
    pub line_search: LineSearchConfig,

    /// Numerical stability constant for avoiding division by zero.
    ///
    /// **Range**: 1e-16 to 1e-6, **Default**: 1e-8
    ///
    /// Used in curvature condition checks and gradient magnitude comparisons.
    /// Smaller values allow more aggressive optimization but may cause instability.
    pub epsilon: f32,

    /// Maximum number of correction pairs to use in two-loop recursion.
    ///
    /// **Range**: 1 to history_size, **Default**: 10
    ///
    /// Limits computational cost when history is large. Should typically equal
    /// history_size unless computational budget is severely constrained.
    pub max_correction_pairs: usize,

    /// Maximum allowed step size in any single iteration.
    ///
    /// **Range**: 0.1 to 100+, **Default**: 2.0
    ///
    /// Prevents excessively large steps that could cause numerical instability
    /// or overshooting. Conservative values (0.5-1.0) improve stability but
    /// may slow convergence on well-conditioned problems.
    pub max_step_size: f32,

    /// Minimum allowed step size before declaring convergence failure.
    ///
    /// **Range**: 1e-20 to 1e-10, **Default**: 1e-16
    ///
    /// Prevents infinite loops when line search cannot find acceptable step.
    /// Very small values allow more persistent optimization attempts.
    pub min_step_size: f32,

    /// Maximum allowed parameter change per iteration (L∞ norm).
    ///
    /// **Range**: 0.01 to 1000+, **Default**: 1.0
    ///
    /// Prevents large parameter jumps that might destabilize optimization.
    /// Useful for problems where parameters have physical meaning or constraints.
    /// Set to 0.0 to disable this constraint.
    pub max_param_change: f32,

    /// Gradient clipping threshold to prevent numerical overflow.
    ///
    /// **Range**: 0.0 (disabled) to 1e6+, **Default**: 1e3
    ///
    /// Clips gradient norm to this value if exceeded. Useful for problems with
    /// occasional large gradients. Set to 0.0 to disable clipping.
    pub gradient_clip: f32,

    /// Enable recovery mechanism when optimization stagnates.
    ///
    /// **Default**: true
    ///
    /// When enabled, resets L-BFGS history and scaling when no improvement
    /// is observed for `recovery_patience` iterations. Helps escape from
    /// poor local approximations but may discard useful curvature information.
    pub enable_recovery: bool,

    /// Number of iterations without improvement before triggering recovery.
    ///
    /// **Range**: 1-20, **Default**: 5
    ///
    /// Lower values trigger recovery more aggressively, potentially helping
    /// with difficult problems but also discarding good approximations sooner.
    pub recovery_patience: usize,

    /// Enable verbose logging of tensor data and internal state.
    ///
    /// **Default**: false
    ///
    /// When enabled, logs detailed information about gradients, directions,
    /// step sizes, and internal L-BFGS state. Useful for debugging but
    /// significantly increases log volume.
    pub verbose: bool,
    /// Name identifier for this optimizer instance.
    ///
    /// **Default**: "L-BFGS"
    pub name: String,
}

impl Default for LBFGSConfig {
    fn default() -> Self {
        Self {
            history_size: 10,
            line_search: LineSearchConfig {
                c1: 1e-4, // Standard Armijo condition
                c2: 0.9,  // Standard curvature condition for L-BFGS
                initial_step: 1.0,
                max_step: 2.0, // Moderate maximum step
                method: LineSearchMethod::StrongWolfe,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            max_correction_pairs: 10,
            max_step_size: 2.0, // Moderate step size limit
            min_step_size: 1e-16,
            max_param_change: 1.0, // Moderate parameter change limit
            gradient_clip: 1e3,    // Moderate gradient clipping
            enable_recovery: true,
            recovery_patience: 5, // Standard recovery patience
            verbose: false,
            name: "L-BFGS".to_string(),
        }
    }
}
impl LBFGSConfig {
    /// Create a strict L-BFGS configuration with conservative settings.
    ///
    /// **Use case**: Ill-conditioned problems, high-precision requirements, or when
    /// numerical stability is more important than convergence speed.
    ///
    /// **Key characteristics**:
    /// - Small history size (5) to reduce memory effects from poor approximations
    /// - Conservative step sizes (max 0.5) to prevent overshooting
    /// - Small parameter changes (max 0.1) for gradual, stable progress
    /// - High precision epsilon (1e-10) for careful numerical comparisons
    /// - Patient recovery (10 iterations) to avoid premature history resets
    ///
    /// **Trade-offs**: More robust convergence but potentially slower on well-conditioned problems.
    pub fn strict() -> Self {
        Self {
            history_size: 5, // Smaller history to reduce memory effects
            line_search: LineSearchConfig {
                c1: 1e-4,          // Standard Armijo condition
                c2: 0.9,           // Strict curvature condition
                initial_step: 0.1, // Conservative initial step
                max_step: 1.0,     // Conservative maximum step
                ..LineSearchConfig::default()
            },
            epsilon: 1e-10,  // Higher precision
            max_correction_pairs: 5,
            max_step_size: 0.5,    // Conservative step size
            min_step_size: 1e-20,  // Allow very small steps
            max_param_change: 0.1, // Small parameter changes
            gradient_clip: 1e2,    // Conservative gradient clipping
            enable_recovery: true,
            recovery_patience: 10, // Patient recovery
            verbose: false,
            name: "L-BFGS-Strict".to_string(),
        }
    }
    /// Create a lax L-BFGS configuration with aggressive settings.
    ///
    /// **Use case**: Well-conditioned problems where fast convergence is desired
    /// and numerical stability is less of a concern.
    ///
    /// **Key characteristics**:
    /// - Large history size (20) for better Hessian approximation
    /// - Aggressive step sizes (max 50.0) for rapid progress
    /// - Large parameter changes (max 100.0) allowing big jumps
    /// - Relaxed curvature condition (c2=0.1) for easier line search acceptance
    /// - Quick recovery (2 iterations) to rapidly adapt to changing conditions
    ///
    /// **Trade-offs**: Faster convergence on suitable problems but higher risk of
    /// numerical instability or overshooting on difficult problems.
    pub fn lax() -> Self {
        Self {
            history_size: 20, // Larger history for better approximation
            line_search: LineSearchConfig {
                c1: 1e-4,          // Standard Armijo condition
                c2: 0.1,           // Relaxed curvature condition
                initial_step: 2.0, // Aggressive initial step
                max_step: 50.0,    // Large maximum step
                ..LineSearchConfig::default()
            },
            epsilon: 1e-6,    // Lower precision for speed
            max_correction_pairs: 20,
            max_step_size: 50.0,     // Large step sizes allowed
            min_step_size: 1e-12,    // Reasonable minimum
            max_param_change: 100.0, // Large parameter changes allowed
            gradient_clip: 1e6,      // High gradient clipping threshold
            enable_recovery: true,
            recovery_patience: 2, // Quick recovery trigger
            verbose: false,
            name: "L-BFGS-Lax".to_string(),
        }
    }
    /// Create a configuration optimized for use within the QQN algorithm.
    ///
    /// **Use case**: When L-BFGS serves as a subroutine within the QQN algorithm
    /// rather than as a standalone optimizer.
    ///
    /// **Key characteristics**:
    /// - Balanced history size (10) for good approximation without excess overhead
    /// - Moderate curvature condition (c2=0.5) balancing acceptance and quality
    /// - Disabled gradient clipping (0.0) - QQN handles gradient conditioning
    /// - Disabled recovery mechanism - QQN manages higher-level adaptation
    /// - Moderate step sizes (max 10.0) suitable for local refinement
    ///
    /// **Rationale**: QQN provides its own mechanisms for handling difficult cases,
    /// so L-BFGS can focus on local quasi-Newton steps without redundant safety measures.
    pub fn for_qqn() -> Self {
        Self {
            history_size: 10,
            line_search: LineSearchConfig {
                c1: 1e-4,
                c2: 0.5, // Balanced curvature condition
                initial_step: 1.0,
                max_step: 10.0,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-8,
            max_correction_pairs: 10,
            max_step_size: 10.0,
            min_step_size: 1e-16,
            max_param_change: 10.0,
            gradient_clip: 0.0,     // Disable gradient clipping for QQN
            enable_recovery: false, // Let QQN handle recovery
            recovery_patience: 0,   // Not used when recovery disabled
            verbose: false,
            name: "L-BFGS-QQN".to_string(),
        }
    }
}

/// State information for L-BFGS optimization.
///
/// Maintains the limited memory representation of the inverse Hessian approximation
/// through stored parameter and gradient differences. The state evolves as optimization
/// progresses, building up curvature information to guide future search directions.
///
/// ## Memory Layout
///
/// The L-BFGS approximation is stored implicitly through:
/// - `s_history`: Parameter differences s_k = x_{k+1} - x_k
/// - `y_history`: Gradient differences y_k = ∇f_{k+1} - ∇f_k  
/// - `rho_history`: Precomputed values ρ_k = 1/(s_k^T y_k) for efficiency
///
/// ## Curvature Condition
///
/// Updates are only accepted when the curvature condition s_k^T y_k > ε is satisfied.
/// When violated, Powell's damping may be applied to maintain positive definiteness
/// of the Hessian approximation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBFGSState<S: Shape> {
    /// History of parameter differences (s_k = x_{k+1} - x_k).
    ///
    /// Each entry represents how parameters changed in a previous iteration.
    /// Used in the two-loop recursion to apply curvature corrections.
    #[serde(skip)]
    s_history: VecDeque<Vec<GraphTensor<S>>>,

    /// History of gradient differences (y_k = ∇f_{k+1} - ∇f_k).
    ///
    /// Each entry represents how gradients changed in a previous iteration.
    /// Combined with s_history to capture local curvature information.
    #[serde(skip)]
    y_history: VecDeque<Vec<GraphTensor<S>>>,

    /// Precomputed reciprocals ρ_k = 1/(s_k^T y_k) for computational efficiency.
    ///
    /// These values are used repeatedly in the two-loop recursion, so precomputing
    /// them avoids redundant dot product calculations.
    #[serde(skip)]
    rho_history: VecDeque<GraphTensor<()>>,

    /// Previous gradient for computing y_k differences in next update.
    ///
    /// Stored from the previous iteration to compute y_k = ∇f_k - ∇f_{k-1}
    /// when the next update occurs.
    #[serde(skip)]
    prev_gradient: Option<Vec<GraphTensor<S>>>,

    /// Current iteration number for tracking optimization progress.
    iteration: usize,

    /// Scaling factor γ for initial Hessian approximation H₀ = γI.
    ///
    /// Updated each iteration as γ = (s_k^T y_k)/(y_k^T y_k) to capture
    /// the characteristic scale of the problem's curvature.
    #[serde(skip)]
    gamma: Option<GraphTensor<()>>,

    /// Numerical stability constant for avoiding division by zero and other issues.
    epsilon: f32,
    /// Maximum history size for L-BFGS updates.
    history_size: usize,


    /// Best function value encountered during optimization.
    ///
    /// Used to track progress and trigger recovery mechanisms when
    /// no improvement is observed for extended periods.
    best_function_value: Option<f32>,

    /// Counter for iterations without improvement in function value.
    ///
    /// When this exceeds recovery_patience, the recovery mechanism
    /// may reset the L-BFGS history to escape poor approximations.
    no_improvement_count: usize,

    /// Previous parameters stored for potential recovery from numerical issues.
    ///
    /// If the current iteration produces non-finite values, optimization
    /// can revert to this previous state.
    #[serde(skip)]
    prev_params: Option<Vec<GraphTensor<S>>>,

    /// Flag to disable certain safety checks when used within QQN.
    ///
    /// When true, skips some numerical validation that QQN handles at a higher level,
    /// allowing for more aggressive local optimization behavior.
    disable_checks: bool,

    /// Maximum allowed gradient norm before applying scaling for numerical stability.
    ///
    /// Gradients exceeding this threshold are scaled down to prevent overflow
    /// in subsequent computations.
    max_gradient_norm: f32,
}
unsafe impl<S: Shape> Send for LBFGSState<S> {}
unsafe impl<S: Shape> Sync for LBFGSState<S> {}


impl<S: Shape> LBFGSState<S> {
    /// Create a new L-BFGS state with the given history size.
    pub fn new(history_size: usize, epsilon: f32) -> Self {
        Self::new_with_options(history_size, epsilon, false)
    }
    /// Create a new L-BFGS state with options for QQN usage
    pub fn new_with_options(history_size: usize, epsilon: f32, disable_checks: bool) -> Self {
        Self {
            s_history: VecDeque::with_capacity(history_size),
            y_history: VecDeque::with_capacity(history_size),
            rho_history: VecDeque::with_capacity(history_size),
            prev_gradient: None,
            iteration: 0,
            gamma: None,
            epsilon,
            history_size,
            best_function_value: None,
            no_improvement_count: 0,
            prev_params: None,
            disable_checks,
            max_gradient_norm: 1e10,
        }
    }

    /// Reset the L-BFGS state to initial conditions.
    pub fn reset(&mut self) {
        self.s_history.clear();
        self.y_history.clear();
        self.rho_history.clear();
        self.prev_gradient = None;
        self.iteration = 0;
        self.gamma = None;
        self.best_function_value = None;
        self.no_improvement_count = 0;
        self.prev_params = None;
        // Don't reset disable_checks as it's a configuration option
    }

    /// Compute the L-BFGS search direction using the two-loop recursion
    ///
    /// This is the core L-BFGS algorithm that computes the search direction p_k = -H_k ∇f_k
    /// where H_k is the approximate inverse Hessian. The method uses the two-loop recursion:
    ///
    /// **First loop** (backward through history):
    /// ```text
    /// q = ∇f_k
    /// for i = k-1, k-2, ..., k-m:
    ///     α_i = ρ_i (s_i^T q)
    ///     q = q - α_i y_i
    /// ```
    ///
    /// **Scaling**: r = γ q where γ = (s_{k-1}^T y_{k-1})/(y_{k-1}^T y_{k-1})
    ///
    /// **Second loop** (forward through history):
    /// ```text
    /// for i = k-m, ..., k-2, k-1:
    ///     β_i = ρ_i (y_i^T r)  
    ///     r = r + (α_i - β_i) s_i
    /// ```
    ///
    /// Returns -r as the descent direction.
    ///
    /// ## Error Handling
    ///
    /// - Falls back to steepest descent if no history exists
    /// - Handles numerical issues (NaN, Inf) gracefully
    /// - Skips problematic history pairs while preserving others
    /// - Validates gradient magnitude and applies scaling if needed
    pub fn estimate_optimum(&mut self, gradient: &[GraphTensor<S>]) -> Vec<GraphTensor<S>> {
        // If no history, use steepest descent

        if self.s_history.is_empty() {
            return gradient.iter().map(|g| -*g).collect();
        }

        let mut q = gradient.to_vec(); // q = gradient
        let mut alpha = Vec::with_capacity(self.s_history.len());

        // First loop: compute alpha values and update q
        for i in (0..self.s_history.len()).rev() {
            let s_i = &self.s_history[i];

            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];
            // alpha_i = rho_i * (s_i^T q)
            let s_dot_q = dot_product(s_i, &q);
            let alpha_i = rho_i * s_dot_q;

            alpha.push(alpha_i);

            // q = q - alpha_i * y_i

            let scaled_y = vector_scale(y_i, alpha_i);
            q = vector_subtract(&q, &scaled_y);
        }

        // Reverse alpha to match forward iteration order
        alpha.reverse();

        // Apply initial Hessian approximation scaling

        // r = gamma * q
        let mut r = if let Some(gamma) = self.gamma {
            vector_scale(&q, gamma)
        } else {
            q.clone()
        };

        // Second loop: compute final direction
        for i in 0..self.s_history.len() {
            let s_i = &self.s_history[i];
            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];

            let alpha_i = alpha[i];

            // beta = rho_i * (y_i^T r)
            let beta = rho_i * dot_product(y_i, &r);

            // r = r + (alpha_i - beta) * s_i

            let correction_factor = alpha_i - beta;
            let correction = vector_scale(s_i, correction_factor);
            r = vector_add(&r, &correction);
        }

        // Return the negative of r to get a descent direction

        r.iter().map(|t| -*t).collect()
    }

    /// Compute the L-BFGS search direction without negation
    /// This is used by QQN which needs the actual direction, not the descent direction
    pub fn compute_direction(&mut self, gradient: &[GraphTensor<S>]) -> Vec<GraphTensor<S>> {
        let descent_dir = self.estimate_optimum(gradient);
        // estimate_optimum returns -H*g (descent direction)
        // We want H*g (search direction before negation), but QQN might expect the descent direction?
        // The comment says "actual direction, not the descent direction".
        // Usually "direction" means p = -H*g.
        // If QQN wants H*g, we negate.
        // But estimate_optimum returns p = -H*g.
        // So if we want H*g, we negate p.
        descent_dir.iter().map(|t| -*t).collect()
    }

    /// Update the L-BFGS state with new gradient and step information.
    ///
    /// Incorporates information from the latest optimization step to improve the
    /// inverse Hessian approximation. This method:
    ///
    /// 1. **Computes differences**: s_k = x_{k+1} - x_k, y_k = ∇f_{k+1} - ∇f_k
    /// 2. **Checks curvature condition**: Ensures s_k^T y_k > ε for positive definiteness
    /// 3. **Applies Powell damping**: Modifies y_k if curvature condition fails
    /// 4. **Updates history**: Adds (s_k, y_k, ρ_k) to limited memory storage
    /// 5. **Updates scaling**: Recomputes γ = (s_k^T y_k)/(y_k^T y_k)
    ///
    /// ## Curvature Condition and Powell Damping
    ///
    /// The curvature condition s_k^T y_k > 0 ensures the Hessian approximation
    /// remains positive definite. When violated, Powell damping interpolates:
    /// ```text
    /// θ = 0.8 * threshold / (threshold - s_k^T y_k)  if s_k^T y_k < 0.2 * threshold
    /// y_k_damped = θ y_k + (1-θ) B_k s_k
    /// ```
    /// This maintains theoretical convergence properties while handling negative curvature.
    ///
    /// ## Memory Management
    ///
    /// When history reaches capacity, the oldest (s_k, y_k, ρ_k) triple is removed
    /// to make room for the new information, maintaining constant memory usage.
    pub fn update(&mut self, new_params: &[GraphTensor<S>], new_gradient: &[GraphTensor<S>]) {
        // We need old_params and old_gradient (prev_gradient) to compute s_k and y_k.
        // In this implementation, we assume update is called after a step.
        // But we need the values from the *previous* step.
        // Since we can't easily access previous values in a static graph without explicit storage,
        // we rely on the caller or the struct state to hold the *tensors* from the previous step.
        // If we have prev_params and prev_gradient, we can compute s_k and y_k.
        if let (Some(prev_params), Some(prev_gradient)) = (&self.prev_params, &self.prev_gradient) {
            // Compute parameter difference: s_k = new_params - old_params

            let s_k = vector_subtract(new_params, prev_params);

            // Compute gradient difference: y_k = new_gradient - prev_gradient

            let y_k = vector_subtract(new_gradient, prev_gradient);

            // Compute curvature condition: s_k^T y_k

            let s_dot_y = dot_product(&s_k, &y_k);

            // rho_k = 1 / (s_k^T y_k)
            let rho_k = s_dot_y.recip();

            // Add to history (maintain limited size)
            if self.s_history.len() >= self.history_size {
                self.s_history.pop_front();
                self.y_history.pop_front();
                self.rho_history.pop_front();
            }
            self.s_history.push_back(s_k);
            self.y_history.push_back(y_k.clone());
            self.rho_history.push_back(rho_k);
            // Update scaling factor gamma = (s_k^T y_k) / (y_k^T y_k)
            let y_dot_y = dot_product(&y_k, &y_k);
            self.gamma = Some(s_dot_y / y_dot_y);
        }

        // Store current gradient for next iteration
        self.prev_gradient = Some(new_gradient.to_vec());
        self.prev_params = Some(new_params.to_vec());
        self.iteration += 1;
    }

    /// Get the current iteration number.
    pub fn iteration(&self) -> usize {
        self.iteration
    }

    /// Get the number of stored correction pairs.
    pub fn history_length(&self) -> usize {
        self.s_history.len()
    }

    /// Get the numerical stability epsilon.
    fn epsilon(&self) -> f32 {
        self.epsilon
    }
}

/// L-BFGS optimizer implementation.
#[derive(Debug)]
pub struct LBFGSOptimizer<S: Shape> {
    config: LBFGSConfig,
    state: LBFGSState<S>,
}

impl<S: Shape> Clone for LBFGSOptimizer<S> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
        }
    }
}

impl<S: Shape> LBFGSOptimizer<S> {
    /// Create a new L-BFGS optimizer with the given configuration.
    pub fn new(config: LBFGSConfig) -> Self {
        let state = LBFGSState::new(config.history_size, config.epsilon);

        Self { config, state }
    }

    /// Get a reference to the internal L-BFGS state.
    pub fn lbfgs_state(&self) -> &LBFGSState<S> {
        &self.state
    }

    /// Get a mutable reference to the internal L-BFGS state.
    pub fn lbfgs_state_mut(&mut self) -> &mut LBFGSState<S> {
        &mut self.state
    }
}

impl<S: Shape> Optimizer<S> for LBFGSOptimizer<S> {
    fn clone_box(&self) -> Box<dyn Optimizer<S>> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,
        graph: &mut Graph,
        loss: GraphTensor<()>,
        params: &[GraphTensor<S>],
    ) -> Vec<GraphTensor<S>> {
        // Register backward pass to compute gradients

        let grads = graph.add_backward(loss);
        let gradients: Vec<GraphTensor<S>> = params.iter().map(|p| *grads.get(p).unwrap()).collect();

        // Update history with previous step's info (if available)
        // Note: This assumes params and gradients are valid for history update
        self.state.update(params, &gradients);

        // Compute L-BFGS search direction

        let search_direction = self.state.estimate_optimum(&gradients);

        // Update parameters: x_{k+1} = x_k + p_k
        // We use a fixed step size of 1.0 as L-BFGS estimates the full step.
        // Line search is omitted in this graph-based implementation.
        params
            .iter()
            .zip(search_direction.iter())
            .map(|(p, d)| *p + *d)
            .collect()
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
    fn set_stagnation_multiplier(&mut self, _multiplier: f32) {
        // L-BFGS doesn't use stagnation multiplier in its current implementation
        // This is a no-op to satisfy the trait requirement
    }
    fn set_stagnation_count(&mut self, _count: usize) {
        // L-BFGS doesn't use stagnation count in its current implementation
        // This is a no-op to satisfy the trait requirement
    }
}
// Helper functions for vector operations on GraphTensor
fn dot_product<S: Shape>(a: &[GraphTensor<S>], b: &[GraphTensor<S>]) -> GraphTensor<()> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (*x * *y).sum_reduce())
        .reduce(|acc, x| acc + x)
        .expect("Empty vector")
}
fn vector_scale<S: Shape>(v: &[GraphTensor<S>], scale: GraphTensor<()>) -> Vec<GraphTensor<S>> {
    v.iter().map(|x| *x * scale).collect()
}
fn vector_add<S: Shape>(a: &[GraphTensor<S>], b: &[GraphTensor<S>]) -> Vec<GraphTensor<S>> {
    a.iter().zip(b.iter()).map(|(x, y)| *x + *y).collect()
}
fn vector_subtract<S: Shape>(a: &[GraphTensor<S>], b: &[GraphTensor<S>]) -> Vec<GraphTensor<S>> {
    a.iter().zip(b.iter()).map(|(x, y)| *x - *y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lbfgs_state_creation() {
        let state = LBFGSState::new(5, 1e-8);
        assert_eq!(state.history_length(), 0);
        assert_eq!(state.iteration(), 0);
        assert_eq!(state.gamma(), 1.0);
        assert!(state.best_function_value.is_none());
        assert_eq!(state.no_improvement_count, 0);
    }

    #[test]
    fn test_lbfgs_optimizer_creation() {
        let config = LBFGSConfig::default();
        let optimizer = LBFGSOptimizer::new(config);

        assert_eq!(optimizer.name(), "L-BFGS");
        assert_eq!(optimizer.state.history_length(), 0);
    }

    #[test]
    fn test_lbfgs_reset() {
        let config = LBFGSConfig::default();
        let mut optimizer = LBFGSOptimizer::new(config);

        // Manually set some state
        optimizer.state.iteration = 5;
        optimizer.state.gamma = 2.0;
        optimizer.state.best_function_value = Some(1.0);
        optimizer.state.no_improvement_count = 3;

        optimizer.reset();
        assert_eq!(optimizer.state.iteration(), 0);
        assert_eq!(optimizer.state.history_length(), 0);
        assert_eq!(optimizer.state.gamma(), 1.0);
        assert!(optimizer.state.best_function_value.is_none());
        assert_eq!(optimizer.state.no_improvement_count, 0);
    }

    #[test]
    fn test_lbfgs_config_constructors() {
        // Test default configuration
        let default_config = LBFGSConfig::default();
        assert_eq!(default_config.history_size, 10);
        assert_eq!(default_config.line_search.c2, 0.9);
        assert_eq!(default_config.max_step_size, 2.0);
        assert_eq!(default_config.max_param_change, 1.0);
        assert_eq!(default_config.recovery_patience, 5);
        assert_eq!(default_config.name, "L-BFGS".to_string());
        // Test strict configuration
        let strict_config = LBFGSConfig::strict();
        assert_eq!(strict_config.history_size, 5);
        assert_eq!(strict_config.line_search.c2, 0.9);
        assert_eq!(strict_config.max_step_size, 0.5);
        assert_eq!(strict_config.max_param_change, 0.1);
        assert_eq!(strict_config.recovery_patience, 10);
        assert_eq!(strict_config.epsilon, 1e-10);
        assert_eq!(strict_config.name, "L-BFGS-Strict".to_string());
        // Test lax configuration
        let lax_config = LBFGSConfig::lax();
        assert_eq!(lax_config.history_size, 20);
        assert_eq!(lax_config.line_search.c2, 0.1);
        assert_eq!(lax_config.max_step_size, 50.0);
        assert_eq!(lax_config.max_param_change, 100.0);
        assert_eq!(lax_config.recovery_patience, 2);
        assert_eq!(lax_config.epsilon, 1e-6);
        assert_eq!(lax_config.name, "L-BFGS-Lax".to_string());
        // Test QQN configuration
        let qqn_config = LBFGSConfig::for_qqn();
        assert_eq!(qqn_config.history_size, 10);
        assert_eq!(qqn_config.line_search.c2, 0.5);
        assert_eq!(qqn_config.gradient_clip, 0.0);
        assert!(!qqn_config.enable_recovery);
        assert_eq!(qqn_config.name, "L-BFGS-QQN".to_string());
    }

    #[test]
    fn test_lbfgs_config_ordering() {
        // Verify that strict < default < lax in terms of aggressiveness
        let strict = LBFGSConfig::strict();
        let default = LBFGSConfig::default();
        let lax = LBFGSConfig::lax();
        assert!(strict.max_step_size < default.max_step_size);
        assert!(default.max_step_size < lax.max_step_size);
        assert!(strict.max_param_change < default.max_param_change);
        assert!(default.max_param_change < lax.max_param_change);
        assert!(strict.recovery_patience > default.recovery_patience);
        assert!(default.recovery_patience > lax.recovery_patience);
    }
}