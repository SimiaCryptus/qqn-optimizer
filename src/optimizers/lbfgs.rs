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

use crate::line_search::line_search::{create_1d_problem_linear, create_line_search};
use crate::line_search::{LineSearch, LineSearchConfig, LineSearchMethod};
use crate::optimizers::optimizer::OptimizationMetadata;
use crate::optimizers::optimizer::{ConvergenceInfo, Optimizer, StepResult};
use crate::utils::math::{
    compute_magnitude, dot_product, log_tensor, tensors_to_f64, vector_add, vector_scale,
    vector_subtract, DifferentiableFunction,
};
use candle_core::{Device, Result as CandleResult, Tensor};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Instant;

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
    pub epsilon: f64,

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
    pub max_step_size: f64,

    /// Minimum allowed step size before declaring convergence failure.
    ///
    /// **Range**: 1e-20 to 1e-10, **Default**: 1e-16
    ///
    /// Prevents infinite loops when line search cannot find acceptable step.
    /// Very small values allow more persistent optimization attempts.
    pub min_step_size: f64,

    /// Maximum allowed parameter change per iteration (L∞ norm).
    ///
    /// **Range**: 0.01 to 1000+, **Default**: 1.0
    ///
    /// Prevents large parameter jumps that might destabilize optimization.
    /// Useful for problems where parameters have physical meaning or constraints.
    /// Set to 0.0 to disable this constraint.
    pub max_param_change: f64,

    /// Gradient clipping threshold to prevent numerical overflow.
    ///
    /// **Range**: 0.0 (disabled) to 1e6+, **Default**: 1e3
    ///
    /// Clips gradient norm to this value if exceeded. Useful for problems with
    /// occasional large gradients. Set to 0.0 to disable clipping.
    pub gradient_clip: f64,

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
                method: LineSearchMethod::StrongWolfe,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-10, // Higher precision
            max_correction_pairs: 5,
            max_step_size: 0.5,    // Conservative step size
            min_step_size: 1e-20,  // Allow very small steps
            max_param_change: 0.1, // Small parameter changes
            gradient_clip: 1e2,    // Conservative gradient clipping
            enable_recovery: true,
            recovery_patience: 10, // Patient recovery
            verbose: false,
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
                method: LineSearchMethod::StrongWolfe,
                ..LineSearchConfig::default()
            },
            epsilon: 1e-6, // Lower precision for speed
            max_correction_pairs: 20,
            max_step_size: 50.0,     // Large step sizes allowed
            min_step_size: 1e-12,    // Reasonable minimum
            max_param_change: 100.0, // Large parameter changes allowed
            gradient_clip: 1e6,      // High gradient clipping threshold
            enable_recovery: true,
            recovery_patience: 2, // Quick recovery trigger
            verbose: false,
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
                method: LineSearchMethod::StrongWolfe,
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
pub struct LBFGSState {
    /// History of parameter differences (s_k = x_{k+1} - x_k).
    ///
    /// Each entry represents how parameters changed in a previous iteration.
    /// Used in the two-loop recursion to apply curvature corrections.
    #[serde(skip_serializing, skip_deserializing)]
    s_history: VecDeque<Vec<Tensor>>,

    /// History of gradient differences (y_k = ∇f_{k+1} - ∇f_k).
    ///
    /// Each entry represents how gradients changed in a previous iteration.
    /// Combined with s_history to capture local curvature information.
    #[serde(skip_serializing, skip_deserializing)]
    y_history: VecDeque<Vec<Tensor>>,

    /// Precomputed reciprocals ρ_k = 1/(s_k^T y_k) for computational efficiency.
    ///
    /// These values are used repeatedly in the two-loop recursion, so precomputing
    /// them avoids redundant dot product calculations.
    rho_history: VecDeque<f64>,

    /// Previous gradient for computing y_k differences in next update.
    ///
    /// Stored from the previous iteration to compute y_k = ∇f_k - ∇f_{k-1}
    /// when the next update occurs.
    #[serde(skip_serializing, skip_deserializing)]
    prev_gradient: Option<Vec<Tensor>>,

    /// Current iteration number for tracking optimization progress.
    iteration: usize,

    /// Scaling factor γ for initial Hessian approximation H₀ = γI.
    ///
    /// Updated each iteration as γ = (s_k^T y_k)/(y_k^T y_k) to capture
    /// the characteristic scale of the problem's curvature.
    gamma: f64,

    /// Numerical stability constant for avoiding division by zero and other issues.
    epsilon: f64,

    /// Best function value encountered during optimization.
    ///
    /// Used to track progress and trigger recovery mechanisms when
    /// no improvement is observed for extended periods.
    best_function_value: Option<f64>,

    /// Counter for iterations without improvement in function value.
    ///
    /// When this exceeds recovery_patience, the recovery mechanism
    /// may reset the L-BFGS history to escape poor approximations.
    no_improvement_count: usize,

    /// Previous parameters stored for potential recovery from numerical issues.
    ///
    /// If the current iteration produces non-finite values, optimization
    /// can revert to this previous state.
    #[serde(skip_serializing, skip_deserializing)]
    prev_params: Option<Vec<Tensor>>,

    /// Flag to disable certain safety checks when used within QQN.
    ///
    /// When true, skips some numerical validation that QQN handles at a higher level,
    /// allowing for more aggressive local optimization behavior.
    disable_checks: bool,

    /// Maximum allowed gradient norm before applying scaling for numerical stability.
    ///
    /// Gradients exceeding this threshold are scaled down to prevent overflow
    /// in subsequent computations.
    max_gradient_norm: f64,
}

impl LBFGSState {
    /// Create a new L-BFGS state with the given history size.
    pub fn new(history_size: usize, epsilon: f64) -> Self {
        Self::new_with_options(history_size, epsilon, false)
    }

    /// Create a new L-BFGS state with options for QQN usage
    pub fn new_with_options(history_size: usize, epsilon: f64, disable_checks: bool) -> Self {
        Self {
            s_history: VecDeque::with_capacity(history_size),
            y_history: VecDeque::with_capacity(history_size),
            rho_history: VecDeque::with_capacity(history_size),
            prev_gradient: None,
            iteration: 0,
            gamma: 1.0,
            epsilon,
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
        self.gamma = 1.0;
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
    pub fn estimate_optimum(
        &mut self,
        position: &[Tensor],
        gradient: &[Tensor],
    ) -> CandleResult<Vec<Tensor>> {
        // Validate input
        self.validate_inputs(position, gradient)?;

        if !self.disable_checks {
            // Check gradient magnitude to avoid numerical issues
            let grad_norm = compute_magnitude(gradient)?;
            if grad_norm < self.epsilon {
                debug!("L-BFGS: Very small gradient norm {grad_norm:.6e}, using steepest descent");
                return gradient
                    .iter()
                    .map(|g| g.neg())
                    .collect::<CandleResult<Vec<_>>>();
            }

            // Check for extremely large gradients
            if grad_norm > self.max_gradient_norm {
                warn!("L-BFGS: Extremely large gradient norm {grad_norm:.6e}, scaling down");
                let scale = self.max_gradient_norm / grad_norm;
                return gradient
                    .iter()
                    .map(|g| g.affine(-scale, 0.0))
                    .collect::<CandleResult<Vec<_>>>();
            }

            // Check for NaN/Inf in gradient
            if !self.check_finite_tensors(gradient, "gradient")? {
                warn!("L-BFGS: Non-finite gradient detected, using steepest descent");
                return gradient
                    .iter()
                    .map(|g| g.neg())
                    .collect::<CandleResult<Vec<_>>>();
            }
        }

        if self.s_history.is_empty() {
            debug!("L-BFGS: No history, using steepest descent");
            return gradient
                .iter()
                .map(|g| g.neg())
                .collect::<CandleResult<Vec<_>>>();
        }

        let mut q = gradient.to_vec();
        let mut alpha = Vec::with_capacity(self.s_history.len());

        // First loop: compute alpha values and update q
        for i in (0..self.s_history.len()).rev() {
            let s_i = &self.s_history[i];
            let rho_i = self.rho_history[i];

            // Check for numerical issues
            if !rho_i.is_finite() || rho_i.abs() < 1e-16 {
                warn!("L-BFGS: Skipping history pair {i} due to numerical issues (rho={rho_i})");
                alpha.push(0.0); // Push zero alpha to maintain indexing
                continue;
            }

            let alpha_i = rho_i * dot_product(s_i, &q)?;
            if !alpha_i.is_finite() {
                warn!("L-BFGS: Non-finite alpha detected at iteration {i}");
                alpha.push(0.0); // Push zero alpha to maintain indexing
                continue;
            }

            alpha.push(alpha_i);

            // q = q - alpha_i * y_i
            let y_i = &self.y_history[i];
            let scaled_y = vector_scale(y_i, alpha_i)?;
            q = vector_subtract(&q, &scaled_y)?;

            if !self.disable_checks {
                // Check if q has become non-finite
                if !self.check_finite_tensors(&q, "q (first loop)")? {
                    return gradient
                        .iter()
                        .map(|g| g.neg())
                        .collect::<CandleResult<Vec<_>>>();
                }
            }
        }

        // Reverse alpha to match forward iteration order
        alpha.reverse();

        // Apply initial Hessian approximation scaling
        debug!("L-BFGS: Using gamma = {:.6e}", self.gamma);

        // Ensure gamma is valid
        if !self.gamma.is_finite() || self.gamma <= 0.0 {
            warn!(
                "L-BFGS: Invalid gamma detected: {}, resetting to 1.0",
                self.gamma
            );
            self.gamma = 1.0;
        }

        let safe_gamma = self.gamma.max(1e-12).min(1e12);

        let mut r = vector_scale(&q, safe_gamma)?;

        // Second loop: compute final direction
        for i in 0..self.s_history.len() {
            if i >= alpha.len() || alpha[i] == 0.0 {
                continue; // Skip if we didn't compute alpha for this iteration or alpha is zero
            }

            let s_i = &self.s_history[i];
            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];

            let alpha_i = alpha[i];

            let beta = rho_i * dot_product(y_i, &r)?;
            let correction_factor = alpha_i - beta;

            if !correction_factor.is_finite() {
                warn!("L-BFGS: Non-finite correction factor at iteration {i}");
                continue;
            }

            // r = r + (alpha_i - beta) * s_i
            let correction = vector_scale(s_i, correction_factor)?;
            r = vector_add(&r, &correction)?;

            if !self.disable_checks {
                // Check if r has become non-finite
                if !self.check_finite_tensors(&r, "r (second loop)")? {
                    return gradient
                        .iter()
                        .map(|g| g.neg())
                        .collect::<CandleResult<Vec<_>>>();
                }
            }
        }

        // Return the negative of r to get a descent direction
        let direction = r
            .iter()
            .map(|t| t.neg())
            .collect::<CandleResult<Vec<_>>>()?;

        if !self.disable_checks {
            // Final check on the direction
            // Verify the direction is finite
            if !self.check_finite_tensors(&direction, "final direction")? {
                return gradient
                    .iter()
                    .map(|g| g.neg())
                    .collect::<CandleResult<Vec<_>>>();
            }
        }

        Ok(direction)
    }

    /// Compute the L-BFGS search direction without negation
    /// This is used by QQN which needs the actual direction, not the descent direction
    pub fn compute_direction(&mut self, gradient: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // Validate input
        if gradient.is_empty() {
            return Err(candle_core::Error::Msg("Empty gradient vector".into()));
        }

        if !self.disable_checks {
            // Check gradient magnitude to avoid numerical issues
            let grad_norm = compute_magnitude(gradient)?;
            if grad_norm < self.epsilon {
                debug!(
                    "L-BFGS: Very small gradient norm {grad_norm:.6e}, returning negative gradient"
                );
                return gradient
                    .iter()
                    .map(|g| g.neg())
                    .collect::<CandleResult<Vec<_>>>();
            }
        }

        if self.s_history.is_empty() {
            debug!("L-BFGS: No history, returning negative gradient");
            return gradient
                .iter()
                .map(|g| g.neg())
                .collect::<CandleResult<Vec<_>>>();
        }

        let mut q = gradient.to_vec();
        let mut alpha = Vec::with_capacity(self.s_history.len());

        // First loop: compute alpha values and update q
        for i in (0..self.s_history.len()).rev() {
            let s_i = &self.s_history[i];
            let rho_i = self.rho_history[i];

            if !rho_i.is_finite() || rho_i.abs() < 1e-16 {
                warn!("L-BFGS: Skipping history pair {i} due to numerical issues (rho={rho_i})");
                alpha.push(0.0);
                continue;
            }

            let alpha_i = rho_i * dot_product(s_i, &q)?;
            if !alpha_i.is_finite() {
                warn!("L-BFGS: Non-finite alpha detected at iteration {i}");
                alpha.push(0.0);
                continue;
            }

            alpha.push(alpha_i);

            // q = q - alpha_i * y_i
            let y_i = &self.y_history[i];
            let scaled_y = vector_scale(y_i, alpha_i)?;
            q = vector_subtract(&q, &scaled_y)?;
        }

        // Reverse alpha to match forward iteration order
        alpha.reverse();

        // Apply initial Hessian approximation scaling
        debug!("L-BFGS: Using gamma = {:.6e}", self.gamma);

        let safe_gamma = if !self.disable_checks {
            self.gamma.max(1e-6).min(1e6)
        } else {
            self.gamma
        };

        let mut r = vector_scale(&q, safe_gamma)?;

        // Second loop: compute final direction
        for i in 0..self.s_history.len() {
            if i >= alpha.len() || alpha[i] == 0.0 {
                continue;
            }

            let s_i = &self.s_history[i];
            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];
            let alpha_i = alpha[i];

            let beta = rho_i * dot_product(y_i, &r)?;
            let correction_factor = alpha_i - beta;

            if !correction_factor.is_finite() {
                warn!("L-BFGS: Non-finite correction factor at iteration {i}");
                continue;
            }

            // r = r + (alpha_i - beta) * s_i
            let correction = vector_scale(s_i, correction_factor)?;
            r = vector_add(&r, &correction)?;
        }

        // Return the negative of r as the direction (this gives us -H*g)
        r.iter().map(|t| t.neg()).collect::<CandleResult<Vec<_>>>()
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
    pub fn update(
        &mut self,
        old_params: &[Tensor],
        new_params: &[Tensor],
        new_gradient: &[Tensor],
    ) -> CandleResult<()> {
        // Early validation to avoid expensive computations
        self.validate_update_inputs(old_params, new_params, new_gradient)?;

        // Compute parameter difference: s_k = new_params - old_params
        let s_k = vector_subtract(new_params, old_params)?;

        // Check if there was any actual movement
        let s_k_norm = compute_magnitude(&s_k)?;

        // Use epsilon-based threshold for consistency
        if s_k_norm < self.epsilon {
            debug!("L-BFGS: Parameter change too small ({s_k_norm:.6e}), skipping update");
            // Still update the previous gradient for next iteration
            self.prev_gradient = Some(new_gradient.to_vec());
            return Ok(());
        }

        if let Some(prev_grad) = &self.prev_gradient {
            // Reserve capacity to avoid reallocations
            if self.s_history.capacity() == 0 {
                self.s_history.reserve(self.s_history.capacity());
                self.y_history.reserve(self.y_history.capacity());
                self.rho_history.reserve(self.rho_history.capacity());
            }

            // Compute gradient difference: y_k = new_gradient - prev_gradient

            let y_k = vector_subtract(new_gradient, prev_grad)?;
            let grad_norm = compute_magnitude(&y_k)?;

            // Compute curvature condition: s_k^T y_k
            let s_dot_y = dot_product(&s_k, &y_k)?;
            debug!(
                "L-BFGS: s_dot_y = {:.6e}, s_k_norm = {:.6e}, y_k_norm = {:.6e}",
                s_dot_y,
                s_k_norm,
                compute_magnitude(&y_k)?
            );

            // Implement Powell's damping for negative curvature
            let curvature_threshold = self.epsilon() * grad_norm.max(1.0);
            let (s_k_final, y_k_final, s_dot_y_final) = if s_dot_y < curvature_threshold {
                if self.disable_checks {
                    // When used in QQN, skip Powell damping and accept the update
                    (s_k, y_k, s_dot_y)
                } else {
                    // Apply Powell's damping
                    let theta = if s_dot_y < 0.2 * curvature_threshold {
                        0.8 * curvature_threshold / (curvature_threshold - s_dot_y)
                    } else {
                        1.0
                    };

                    if theta < 1.0 {
                        debug!("L-BFGS: Applying Powell damping with theta = {theta:.6e}");
                        // y_k_damped = theta * y_k + (1 - theta) * B_k * s_k
                        // For simplicity, we'll use a scaled identity approximation for B_k
                        let scaled_s = vector_scale(&s_k, self.gamma)?;
                        let damped_y = vector_add(
                            &vector_scale(&y_k, theta)?,
                            &vector_scale(&scaled_s, 1.0 - theta)?,
                        )?;
                        let damped_s_dot_y = dot_product(&s_k, &damped_y)?;
                        (s_k, damped_y, damped_s_dot_y)
                    } else {
                        (s_k, y_k, s_dot_y)
                    }
                }
            } else {
                (s_k, y_k, s_dot_y)
            };

            // Now check if the (possibly damped) curvature condition is satisfied
            if self.disable_checks || s_dot_y_final > curvature_threshold {
                let rho_k = 1.0 / s_dot_y_final;

                if !self.disable_checks && !rho_k.is_finite() {
                    warn!("L-BFGS: Non-finite rho_k, skipping update");
                    self.prev_gradient = Some(new_gradient.to_vec());
                    return Ok(());
                }

                // Add to history (maintain limited size)
                if self.s_history.len() >= self.s_history.capacity() {
                    self.s_history.pop_front();
                    self.y_history.pop_front();
                    self.rho_history.pop_front();
                }

                self.s_history.push_back(s_k_final);
                self.y_history.push_back(y_k_final.clone());
                self.rho_history.push_back(rho_k);

                // Update scaling factor for initial Hessian approximation
                // gamma = (s_k^T y_k) / (y_k^T y_k)
                let y_dot_y = dot_product(&y_k_final, &y_k_final)?;
                if y_dot_y > self.epsilon {
                    let new_gamma = s_dot_y_final / y_dot_y;

                    // Ensure gamma is finite before updating
                    if new_gamma.is_finite() && new_gamma > 0.0 {
                        // Less conservative gamma clamping for better performance
                        self.gamma = new_gamma.max(1e-8).min(1e8);

                        if (new_gamma - self.gamma).abs() > 1e-10 {
                            debug!("L-BFGS: Gamma clamped from {} to {}", new_gamma, self.gamma);
                        }
                    } else {
                        debug!(
                            "L-BFGS: Invalid gamma computed: {new_gamma}, keeping current value"
                        );
                    }
                }
            } else {
                debug!("L-BFGS: Curvature condition not satisfied even after damping (s_dot_y = {s_dot_y_final:.6e}, threshold = {curvature_threshold:.6e}), skipping update");
            }
        }

        // Store current gradient for next iteration
        self.prev_gradient = Some(new_gradient.to_vec());
        self.iteration += 1;

        Ok(())
    }

    /// Get the current iteration number.
    pub fn iteration(&self) -> usize {
        self.iteration
    }

    /// Get the number of stored correction pairs.
    pub fn history_length(&self) -> usize {
        self.s_history.len()
    }

    /// Get the current Hessian scaling factor.
    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    /// Get the numerical stability epsilon.
    fn epsilon(&self) -> f64 {
        self.epsilon
    }

    /// Validate input tensors have matching dimensions
    fn validate_inputs(&self, position: &[Tensor], gradient: &[Tensor]) -> CandleResult<()> {
        if gradient.is_empty() {
            return Err(candle_core::Error::Msg("Empty gradient vector".into()));
        }

        if position.is_empty() {
            return Err(candle_core::Error::Msg("Empty parameter vector".into()));
        }

        if position.len() != gradient.len() {
            return Err(candle_core::Error::Msg(format!(
                "Parameter and gradient dimension mismatch: {} vs {}",
                position.len(),
                gradient.len()
            )));
        }

        Ok(())
    }

    /// Validate update inputs
    fn validate_update_inputs(
        &self,
        old_params: &[Tensor],
        new_params: &[Tensor],
        new_gradient: &[Tensor],
    ) -> CandleResult<()> {
        if old_params.is_empty() || new_params.is_empty() || new_gradient.is_empty() {
            return Err(candle_core::Error::Msg(
                "Empty parameter or gradient vectors".into(),
            ));
        }

        if old_params.len() != new_params.len() || new_params.len() != new_gradient.len() {
            return Err(candle_core::Error::Msg(format!(
                "Parameter and gradient dimension mismatch: old={}, new={}, grad={}",
                old_params.len(),
                new_params.len(),
                new_gradient.len()
            )));
        }

        Ok(())
    }

    /// Check if all tensors contain finite values
    fn check_finite_tensors(&self, tensors: &[Tensor], context: &str) -> CandleResult<bool> {
        for (i, tensor) in tensors.iter().enumerate() {
            let values = tensor.flatten_all()?.to_vec1::<f64>()?;
            if values.iter().any(|&x| !x.is_finite()) {
                warn!("L-BFGS: Non-finite {context} detected at index {i}");
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// L-BFGS optimizer implementation.
#[derive(Debug)]
pub struct LBFGSOptimizer {
    config: LBFGSConfig,
    state: LBFGSState,
    line_search: Box<dyn LineSearch>,
}

impl Clone for LBFGSOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            line_search: self.line_search.clone_box(),
        }
    }
}

impl LBFGSOptimizer {
    /// Create a new L-BFGS optimizer with the given configuration.
    pub fn new(config: LBFGSConfig) -> Self {
        if config.verbose {
            info!("Creating L-BFGS optimizer with verbose logging enabled");
            debug!("L-BFGS Config: history_size={}, epsilon={:.6e}, max_step_size={:.6e}, min_step_size={:.6e}, max_param_change={:.6e}, gradient_clip={:.6e}",
                  config.history_size, config.epsilon, config.max_step_size, config.min_step_size, config.max_param_change, config.gradient_clip);
        }

        let state = LBFGSState::new(config.history_size, config.epsilon);
        let line_search = create_line_search(config.line_search.clone());

        Self {
            config,
            state,
            line_search,
        }
    }

    /// Log tensor data if verbose mode is enabled
    fn log_tensor_data(&self, name: &str, tensors: &[Tensor]) {
        if !self.config.verbose {
            return;
        }
        debug!("=== L-BFGS: {name} ===");
        log_tensor(tensors);
    }

    /// Log scalar value if verbose mode is enabled
    fn log_scalar(&self, name: &str, value: f64) {
        if self.config.verbose {
            debug!("  L-BFGS {name}: {value:.12e}");
        }
    }

    /// Log L-BFGS state if verbose mode is enabled
    fn log_lbfgs_state(&self, additional_info: &str) {
        if !self.config.verbose {
            return;
        }
        debug!("=== L-BFGS State ===");
        debug!("  Iteration: {}", self.state.iteration());
        debug!("  History Length: {}", self.state.history_length());
        debug!("  Gamma: {:.6e}", self.state.gamma());
        debug!("  Additional Info: {additional_info}");
    }

    /// Get a reference to the internal L-BFGS state.
    pub fn lbfgs_state(&self) -> &LBFGSState {
        &self.state
    }

    /// Get a mutable reference to the internal L-BFGS state.
    pub fn lbfgs_state_mut(&mut self) -> &mut LBFGSState {
        &mut self.state
    }

    /// Compute convergence information for the current state.
    fn compute_convergence_info(&self, gradient: &[Tensor]) -> CandleResult<ConvergenceInfo> {
        let gradient_norm = compute_magnitude(gradient)?;

        Ok(ConvergenceInfo {
            converged: gradient_norm < 1e-6, // Default tolerance
            function_change: None,
        })
    }
}

impl Optimizer for LBFGSOptimizer {
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
            debug!("=== L-BFGS Step {} Starting ===", self.state.iteration());
        }

        // Store current parameters for potential recovery
        if self.config.enable_recovery {
            self.state.prev_params = Some(params.to_vec());
        }

        // Compute gradients at current parameters
        let gradients = function.gradient(params)?;

        // Apply gradient clipping if enabled
        let gradients = if self.config.gradient_clip > 0.0 {
            let grad_norm = compute_magnitude(&gradients)?;
            if grad_norm > self.config.gradient_clip {
                warn!(
                    "L-BFGS: Clipping gradient from {:.6e} to {:.6e}",
                    grad_norm, self.config.gradient_clip
                );
                let scale_factor = self.config.gradient_clip / grad_norm;
                gradients
                    .iter()
                    .map(|g| g.affine(scale_factor, 0.0))
                    .collect::<CandleResult<Vec<_>>>()?
            } else {
                gradients
            }
        } else {
            gradients
        };

        // Log initial state in verbose mode
        self.log_tensor_data("Initial Parameters", params);
        self.log_tensor_data("Computed Gradients", &gradients);

        // Input validation
        self.state.validate_inputs(params, &gradients)?;

        // Compute L-BFGS search direction
        self.log_lbfgs_state("Before computing direction");
        let search_direction = self.state.estimate_optimum(params, &gradients)?;
        self.log_tensor_data("L-BFGS Search Direction", &search_direction);

        // Validate search direction
        let direction_norm = compute_magnitude(&search_direction)?;
        self.log_scalar("Direction Norm", direction_norm);

        if !direction_norm.is_finite() || direction_norm < self.config.epsilon {
            warn!(
                "L-BFGS: Invalid search direction norm: {direction_norm}, using steepest descent"
            );

            // Fall back to steepest descent
            let search_direction = gradients
                .iter()
                .map(|g| g.neg())
                .collect::<CandleResult<Vec<_>>>()?;
            let direction_norm = compute_magnitude(&search_direction)?;
            let step_size = 0.01 / (direction_norm + 1.0);

            self.log_scalar("Fallback Step Size", step_size);
            self.log_tensor_data("Fallback Direction", &search_direction);

            // Update parameters with conservative step
            let old_params_vec = params.to_vec();
            for (param, dir) in params.iter_mut().zip(search_direction.iter()) {
                let step_size_tensor = Tensor::new(step_size, param.device())?;
                let update = dir.broadcast_mul(&step_size_tensor)?;
                *param = param.add(&update)?;
            }

            self.log_tensor_data("Updated Parameters (Fallback)", params);

            // Update L-BFGS state
            if step_size > 0.0 {
                self.state.update(&old_params_vec, params, &gradients)?;
            }

            let convergence_info = self.compute_convergence_info(&gradients)?;
            let step_duration = start_time.elapsed();

            let mut metadata = OptimizationMetadata::default();
            metadata.timing_info.step_duration = step_duration;
            metadata
                .optimizer_data
                .insert("fallback_to_steepest_descent".to_string(), 1.0);

            return Ok(StepResult {
                step_size,
                convergence_info,
                metadata,
            });
        }

        // Use adaptive step size based on gradient magnitude
        let grad_norm = compute_magnitude(&gradients)?;
        self.log_scalar("Gradient Norm", grad_norm);
        debug!(
            "L-BFGS step {}: grad_norm={:.6e}",
            self.state.iteration(),
            grad_norm
        );

        // Improved step size initialization for better scaling
        let step_size = if self.state.iteration() == 0 {
            // First iteration: use problem-aware scaling
            let param_scale = params
                .iter()
                .map(|p| compute_magnitude(&[p.clone()]))
                .collect::<CandleResult<Vec<_>>>()?
                .into_iter()
                .fold(0.0_f64, |a, b| a.max(b));

            // Better initial step size estimation
            let scale_factor = param_scale.max(1.0);
            let normalized_grad_norm = grad_norm / scale_factor;
            let initial_step = if normalized_grad_norm > 1.0 {
                1.0 / normalized_grad_norm
            } else {
                1.0
            };
            initial_step.max(1e-4).min(10.0)
        } else {
            // Subsequent iterations: use gamma-based scaling
            let dir_norm = compute_magnitude(&search_direction)?;
            if dir_norm > 0.0 {
                // Use gamma for better step size estimation
                let gamma_step = (self.state.gamma() * 2.0).min(10.0) / dir_norm;
                gamma_step
                    .max(self.config.min_step_size)
                    .min(self.config.max_step_size)
            } else {
                self.config.min_step_size
            }
        };

        debug!("L-BFGS: Initial step size = {step_size:.6e}");

        // Use the configured line search
        let mut line_search = self.line_search.clone_box();

        // Create a more conservative line search configuration for problematic cases
        if grad_norm > 1e6 || direction_norm > 1e6 {
            warn!("L-BFGS: Large gradients detected (grad_norm={grad_norm:.2e}, dir_norm={direction_norm:.2e}), using very conservative step size");

            // For very large gradients, use an extremely conservative fixed step
            let conservative_step = (1e-6 / (grad_norm + 1.0)).max(1e-12).min(1e-6);

            // Update parameters with conservative step
            let old_params = params.to_vec();
            for (param, direction) in params.iter_mut().zip(&search_direction) {
                let step_size_tensor = Tensor::new(conservative_step, param.device())?;
                let step = direction.broadcast_mul(&step_size_tensor)?;
                *param = param.add(&step)?;
            }

            // Update L-BFGS state
            self.state.update(&old_params, params, &gradients)?;

            let convergence_info = self.compute_convergence_info(&gradients)?;
            let step_duration = start_time.elapsed();

            let mut metadata = OptimizationMetadata::default();
            metadata.timing_info.step_duration = step_duration;
            metadata
                .optimizer_data
                .insert("conservative_step_used".to_string(), 1.0);
            metadata
                .optimizer_data
                .insert("conservative_step_size".to_string(), conservative_step);

            return Ok(StepResult {
                step_size: conservative_step,
                convergence_info,
                metadata,
            });
        }

        // Convert tensors to f64 vectors for line search
        let current_point = tensors_to_f64(params)?;
        let direction_f64 = tensors_to_f64(&search_direction)?;

        // Perform line search in a separate scope to avoid borrow conflicts
        let line_search_result = {
            // Create objective and gradient functions that work with f64 vectors
            let function_clone = function.clone();
            let objective_fn = move |x: &[f64]| -> anyhow::Result<f64> {
                let device = &Device::Cpu;
                let x_tensors = [Tensor::new(x, device)?].to_vec();
                function_clone
                    .evaluate(&x_tensors)
                    .map_err(|e| anyhow::anyhow!("Function evaluation failed: {}", e))
            };

            let function_clone2 = function.clone();
            let gradient_fn = move |x: &[f64]| -> anyhow::Result<Vec<f64>> {
                let device = &Device::Cpu;
                let x_tensors = [Tensor::new(x, device)?].to_vec();
                let grad_tensors = function_clone2
                    .gradient(&x_tensors)
                    .map_err(|e| anyhow::anyhow!("Gradient evaluation failed: {}", e))?;
                tensors_to_f64(&grad_tensors)
                    .map_err(|e| anyhow::anyhow!("Tensor conversion failed: {}", e))
            };

            // Create 1D problem
            let problem = create_1d_problem_linear(
                &current_point,
                &direction_f64,
                Arc::new(objective_fn),
                Arc::new(gradient_fn),
            )
            .map_err(|e| candle_core::Error::Msg(format!("Failed to create 1D problem: {e}")))?;

            // Perform line search
            line_search
                .optimize_1d(&problem)
                .map_err(|e| candle_core::Error::Msg(format!("Line search failed: {e}")))?
        };

        if self.config.verbose {
            debug!("=== Line Search Result ===");
            debug!("  Step Size: {:.12e}", line_search_result.step_size);
            debug!("  Success: {}", line_search_result.success);
        }

        // Limit the actual step size based on maximum parameter change
        let mut actual_step_size = line_search_result.step_size;
        if self.config.max_param_change > 0.0 {
            // Compute the maximum change that would occur
            let max_change = search_direction
                .iter()
                .map(|d| {
                    let d_vec = d.flatten_all()?.to_vec1::<f64>()?;
                    Ok(d_vec.iter().map(|x| x.abs()).fold(0.0, f64::max) * actual_step_size)
                })
                .collect::<CandleResult<Vec<_>>>()?
                .into_iter()
                .fold(0.0, f64::max);

            if max_change > self.config.max_param_change {
                let scale = self.config.max_param_change / max_change;
                actual_step_size *= scale;
                warn!("L-BFGS: Limiting step size from {:.6e} to {:.6e} due to max_param_change constraint", 
                      line_search_result.step_size, actual_step_size);
            }
        }

        // Update parameters: x_{k+1} = x_k + alpha * p_k
        let old_params = params.to_vec();
        for (param, direction) in params.iter_mut().zip(&search_direction) {
            let step_size_tensor = Tensor::new(actual_step_size, param.device())?;
            let step = direction.broadcast_mul(&step_size_tensor)?;
            *param = param.add(&step)?;

            // Check for NaN/Inf in updated parameters
            if !self
                .state
                .check_finite_tensors(&[param.clone()], "updated parameter")?
            {
                // Recovery: restore previous parameters if available
                if let Some(prev_params) = &self.state.prev_params {
                    warn!("L-BFGS: Non-finite parameters detected, restoring previous state");
                    for (param, prev) in params.iter_mut().zip(prev_params.iter()) {
                        *param = prev.clone();
                    }

                    // Reset L-BFGS state
                    self.state.reset();

                    return Ok(StepResult {
                        step_size: 0.0,
                        convergence_info: ConvergenceInfo {
                            converged: false,
                            function_change: None,
                        },
                        metadata: OptimizationMetadata::default(),
                    });
                } else {
                    return Err(candle_core::Error::Msg(
                        "Non-finite parameter detected after update".into(),
                    ));
                }
            }
        }

        self.log_tensor_data("Updated Parameters", params);

        // Check for improvement and update best value
        let current_value = function.evaluate(params)?;
        let improved = match self.state.best_function_value {
            Some(best) => {
                if current_value < best {
                    self.state.best_function_value = Some(current_value);
                    self.state.no_improvement_count = 0;
                    true
                } else {
                    self.state.no_improvement_count += 1;
                    false
                }
            }
            _ => {
                self.state.best_function_value = Some(current_value);
                true
            }
        };

        // Enhanced recovery mechanism
        if self.config.enable_recovery
            && self.state.no_improvement_count >= self.config.recovery_patience
            && !improved
        {
            warn!(
                "L-BFGS: No improvement for {} iterations, triggering recovery",
                self.state.no_improvement_count
            );

            // More aggressive recovery: reset history and scaling
            self.state.s_history.clear();
            self.state.y_history.clear();
            self.state.rho_history.clear();

            // Reset gamma to a value that might work better for the current scale
            let param_scale = params
                .iter()
                .map(|p| compute_magnitude(&[p.clone()]))
                .collect::<CandleResult<Vec<_>>>()?
                .into_iter()
                .fold(0.0_f64, |a, b| a.max(b));

            self.state.gamma = (1.0 / (grad_norm / param_scale.max(1.0)))
                .max(0.1)
                .min(10.0);
            self.state.no_improvement_count = 0;

            debug!(
                "L-BFGS: Recovery triggered, new gamma = {:.6e}",
                self.state.gamma
            );
        }

        // Update L-BFGS state with new information
        self.state.update(&old_params, params, &gradients)?;
        self.log_lbfgs_state("After state update");

        // Compute convergence information
        let convergence_info = self.compute_convergence_info(&gradients)?;
        let step_duration = start_time.elapsed();

        if self.config.verbose {
            debug!(
                "=== L-BFGS Step {} Completed ===",
                self.state.iteration() - 1
            );
            debug!("  Step Duration: {step_duration:?}");
            debug!("  Converged: {}", convergence_info.converged);
        }

        let mut metadata = OptimizationMetadata::default();
        metadata.timing_info.step_duration = step_duration;
        metadata
            .optimizer_data
            .insert("gradient_norm".to_string(), grad_norm);
        metadata
            .optimizer_data
            .insert("direction_norm".to_string(), direction_norm);
        metadata
            .optimizer_data
            .insert("step_size".to_string(), actual_step_size);
        metadata
            .optimizer_data
            .insert("gamma".to_string(), self.state.gamma());
        metadata.optimizer_data.insert(
            "history_size".to_string(),
            self.state.history_length() as f64,
        );
        metadata
            .optimizer_data
            .insert("function_value".to_string(), current_value);

        if let Some(best) = self.state.best_function_value {
            metadata
                .optimizer_data
                .insert("best_function_value".to_string(), best);
        }

        metadata.optimizer_data.insert(
            "no_improvement_count".to_string(),
            self.state.no_improvement_count as f64,
        );

        Ok(StepResult {
            step_size: actual_step_size,
            convergence_info,
            metadata,
        })
    }

    fn reset(&mut self) {
        self.state.reset();
    }

    fn name(&self) -> &str {
        "L-BFGS"
    }

    fn iteration(&self) -> usize {
        self.state.iteration()
    }

    fn set_stagnation_multiplier(&mut self, _multiplier: f64) {
        // L-BFGS doesn't use stagnation multiplier in its current implementation
        // This is a no-op to satisfy the trait requirement
    }

    fn set_stagnation_count(&mut self, _count: usize) {
        // L-BFGS doesn't use stagnation count in its current implementation
        // This is a no-op to satisfy the trait requirement
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::analytic_functions::RosenbrockFunction;
    use approx::assert_relative_eq;
    use candle_core::Device;
    use std::sync::Arc;

    impl DifferentiableFunction for RosenbrockFunction {
        fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
            let x = params[0].to_vec1::<f64>()?;
            let term1 = (1.0 - x[0]).powi(2);
            let term2 = 100.0 * (x[1] - x[0].powi(2)).powi(2);
            Ok(term1 + term2)
        }
        fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
            let x = params[0].to_vec1::<f64>()?;
            let y = params[1].to_vec1::<f64>()?;

            let dx = -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (y[0] - x[0].powi(2));
            let dy = 200.0 * (y[0] - x[0].powi(2));
            Ok(vec![
                Tensor::from_slice(&[dx], &[1], params[0].device())?,
                Tensor::from_slice(&[dy], &[1], params[0].device())?,
            ])
        }
    }
    // Simple quadratic function for testing
    struct QuadraticFunction;
    impl DifferentiableFunction for QuadraticFunction {
        fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
            let x = params[0].to_vec1::<f64>()?;
            Ok(x.iter().map(|&xi| xi * xi).sum())
        }
        fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
            let device = params[0].device();
            let x = params[0].to_vec1::<f64>()?;
            let grad: Vec<f64> = x.iter().map(|&xi| 2.0 * xi).collect();
            Ok(vec![Tensor::from_vec(grad, x.len(), device)?])
        }
    }

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
    fn test_lbfgs_steepest_descent_fallback() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5, 1e-8);
        let params = vec![Tensor::from_slice(&[1.0, 2.0], (2,), &device)?];

        let gradient = vec![Tensor::from_slice(&[1.0, 2.0], (2,), &device)?];

        let direction = state.estimate_optimum(&params, &gradient)?;

        // Should return negative gradient (steepest descent)
        let expected = [Tensor::from_slice(&[-1.0, -2.0], (2,), &device)?];

        let dir_values = direction[0].to_vec1::<f64>()?;
        let exp_values = expected[0].to_vec1::<f64>()?;
        assert_relative_eq!(dir_values[0], exp_values[0], epsilon = 1e-10);
        assert_relative_eq!(dir_values[1], exp_values[1], epsilon = 1e-10);

        Ok(())
    }

    #[test]
    fn test_lbfgs_state_update() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5, 1e-8);
        let old_params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let new_params1 = vec![Tensor::from_slice(&[0.9, 0.9], &[2], &device)?];
        let new_params2 = vec![Tensor::from_slice(&[0.8, 0.8], &[2], &device)?];

        let grad1 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let grad2 = vec![Tensor::from_slice(&[0.5, 0.5], &[2], &device)?];

        // First update should not add to history (no previous gradient)
        state.update(&old_params, &new_params1, &grad1)?;
        assert_eq!(state.history_length(), 0);
        assert_eq!(state.iteration(), 1);

        // Second update should add to history
        state.update(&new_params1, &new_params2, &grad2)?;
        assert_eq!(state.history_length(), 1);
        assert_eq!(state.iteration(), 2);

        Ok(())
    }
    #[test]
    fn test_lbfgs_direction_with_history() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5, 1e-8);
        // Build up some history with more distinct gradients and directions
        // First iteration: gradient [2.0, 4.0], move from [0, 0] to [-0.1, -0.2]
        let params0 = vec![Tensor::from_slice(&[0.0, 0.0], &[2], &device)?];
        let params1 = vec![Tensor::from_slice(&[-0.1, -0.2], &[2], &device)?];
        let grad1 = vec![Tensor::from_slice(&[2.0, 4.0], &[2], &device)?];

        // Second iteration: gradient [1.0, 1.0], move from [-0.1, -0.2] to [-0.2, -0.4]
        let params2 = vec![Tensor::from_slice(&[-0.2, -0.4], &[2], &device)?];
        let grad2 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];

        state.update(&params0, &params1, &grad1)?;
        state.update(&params1, &params2, &grad2)?;
        // Now compute a direction with history
        let current_params = vec![Tensor::from_slice(&[-0.2, -0.4], &[2], &device)?];
        let grad3 = vec![Tensor::from_slice(&[0.8, 0.4], &[2], &device)?];
        let direction = state.estimate_optimum(&current_params, &grad3)?;
        // Direction should be different from steepest descent due to history
        let steepest_descent = [Tensor::from_slice(&[-0.8, -0.4], &[2], &device)?];
        let dir_values = direction[0].to_vec1::<f64>()?;
        let sd_values = steepest_descent[0].to_vec1::<f64>()?;
        debug!("Direction values: {dir_values:?}");
        // Should not be exactly equal to steepest descent
        assert!(
            (dir_values[0] - sd_values[0]).abs() > 1e-10
                || (dir_values[1] - sd_values[1]).abs() > 1e-10
        );
        Ok(())
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
    fn test_curvature_condition_rejection() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5, 1e-8);
        let old_params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let new_params = vec![Tensor::from_slice(&[0.9, 0.9], &[2], &device)?];

        let grad1 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let grad2 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?]; // Same gradient

        state.update(&old_params, &new_params, &grad1)?;
        state.update(&new_params, &old_params, &grad2)?; // Move back to test zero curvature

        // With Powell damping, zero curvature gets corrected and update is accepted
        // The original test expected rejection, but Powell damping allows acceptance
        assert_eq!(state.history_length(), 1);

        Ok(())
    }

    #[test]
    fn test_history_size_limit() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(2, 1e-8); // Small history size

        // Add more updates than history size
        let mut old_params = vec![Tensor::from_slice(&[0.0, 0.0], &[2], &device)?];
        for i in 0..5 {
            let new_params = vec![Tensor::from_slice(
                &[0.0 - (i + 1) as f64 * 0.1, 0.0 - (i + 1) as f64 * 0.1],
                &[2],
                &device,
            )?];
            let grad = vec![Tensor::from_slice(
                &[1.0 + i as f64 * 0.1, 1.0],
                &[2],
                &device,
            )?];
            state.update(&old_params, &new_params, &grad)?;
            old_params = new_params;
        }

        // Should maintain only the history size limit
        assert!(state.history_length() <= 2);

        Ok(())
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
        // Test strict configuration
        let strict_config = LBFGSConfig::strict();
        assert_eq!(strict_config.history_size, 5);
        assert_eq!(strict_config.line_search.c2, 0.9);
        assert_eq!(strict_config.max_step_size, 0.5);
        assert_eq!(strict_config.max_param_change, 0.1);
        assert_eq!(strict_config.recovery_patience, 10);
        assert_eq!(strict_config.epsilon, 1e-10);
        // Test lax configuration
        let lax_config = LBFGSConfig::lax();
        assert_eq!(lax_config.history_size, 20);
        assert_eq!(lax_config.line_search.c2, 0.1);
        assert_eq!(lax_config.max_step_size, 50.0);
        assert_eq!(lax_config.max_param_change, 100.0);
        assert_eq!(lax_config.recovery_patience, 2);
        assert_eq!(lax_config.epsilon, 1e-6);
        // Test QQN configuration
        let qqn_config = LBFGSConfig::for_qqn();
        assert_eq!(qqn_config.history_size, 10);
        assert_eq!(qqn_config.line_search.c2, 0.5);
        assert_eq!(qqn_config.gradient_clip, 0.0);
        assert!(!qqn_config.enable_recovery);
    }
    #[test]
    fn test_lbfgs_strict_config_behavior() -> CandleResult<()> {
        let device = Device::Cpu;
        let strict_config = LBFGSConfig::strict();
        let mut optimizer = LBFGSOptimizer::new(strict_config);
        let function = Arc::new(QuadraticFunction);
        let mut params = vec![Tensor::from_slice(&[5.0, -3.0], &[2], &device)?];
        // Run a step with strict configuration
        let result = optimizer.step(&mut params, function)?;
        // Should take conservative steps
        assert!(result.step_size <= 0.5);
        assert!(result.step_size > 0.0);
        Ok(())
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

    #[test]
    fn test_lbfgs_on_quadratic() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = LBFGSConfig::default();
        config.verbose = false;
        let mut optimizer = LBFGSOptimizer::new(config);
        let function = Arc::new(QuadraticFunction);
        let mut params = vec![Tensor::from_slice(&[5.0, -3.0], &[2], &device)?];
        // Run a few optimization steps
        for _ in 0..10 {
            let result = optimizer.step(&mut params, function.clone())?;
            if result.convergence_info.converged {
                break;
            }
        }
        // Should converge close to [0, 0]
        let final_params = params[0].to_vec1::<f64>()?;
        assert!(final_params[0].abs() < 1e-4);
        assert!(final_params[1].abs() < 1e-4);
        let _result = optimizer.step(&mut params, function)?;
        let final_params = params[0].to_vec1::<f64>()?;
        assert!(final_params[0].abs() < 1e-4);
        assert!(final_params[1].abs() < 1e-4);
        Ok(())
    }
    #[ignore]
    #[test]
    fn test_lbfgs_on_rosenbrock() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = LBFGSConfig::default();
        config.verbose = false;
        config.max_step_size = 1.0;
        let mut optimizer = LBFGSOptimizer::new(config);
        let function = Arc::new(RosenbrockFunction::new(2));
        let mut params = vec![
            Tensor::from_slice(&[-1.2], &[1], &device)?,
            Tensor::from_slice(&[1.0], &[1], &device)?,
        ];
        // Run optimization steps
        for i in 0..100 {
            let result = optimizer.step(&mut params, function.clone())?;
            // Check if we're making progress
            if i > 0 && result.step_size < 1e-10 {
                break;
            }
            if result.convergence_info.converged {
                break;
            }
        }
        // Should get close to the optimum at [1, 1]
        let x = params[0].to_vec1::<f64>()?[0];
        let y = params[1].to_vec1::<f64>()?[0];
        // Rosenbrock is difficult, so we allow some tolerance
        assert!((x - 1.0).abs() < 0.1, "x = {x}, expected close to 1.0");
        assert!((y - 1.0).abs() < 0.1, "y = {y}, expected close to 1.0");
        Ok(())
    }
    #[test]
    fn test_lbfgs_gradient_clipping() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = LBFGSConfig::default();
        config.gradient_clip = 1.0;
        config.verbose = false;
        let mut optimizer = LBFGSOptimizer::new(config);
        // Create a function with large gradients
        struct LargeGradientFunction;
        impl DifferentiableFunction for LargeGradientFunction {
            fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
                let x = params[0].to_vec1::<f64>()?;
                Ok(x[0] * x[0])
            }
            fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
                let device = params[0].device();
                Ok(vec![Tensor::from_slice(&[1000.0], &[1], device)?])
            }
        }
        let function = Arc::new(LargeGradientFunction);
        let mut params = vec![Tensor::from_slice(&[1.0], &[1], &device)?];
        let result = optimizer.step(&mut params, function)?;
        // Step should be taken despite large gradient
        assert!(result.step_size > 0.0);
        Ok(())
    }
    #[test]
    fn test_lbfgs_recovery_mechanism() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = LBFGSConfig::default();
        config.enable_recovery = true;
        config.recovery_patience = 2;
        config.verbose = false;
        let mut optimizer = LBFGSOptimizer::new(config);
        // Function that returns constant value (no improvement)
        struct ConstantFunction;
        impl DifferentiableFunction for ConstantFunction {
            fn evaluate(&self, _params: &[Tensor]) -> CandleResult<f64> {
                Ok(1.0)
            }
            fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
                let device = params[0].device();
                Ok(vec![Tensor::from_slice(&[0.1], &[1], device)?])
            }
        }
        let function = Arc::new(ConstantFunction);
        let mut params = vec![Tensor::from_slice(&[1.0], &[1], &device)?];
        // Run enough steps to trigger recovery
        for _ in 0..5 {
            optimizer.step(&mut params, function.clone())?;
        }
        // Recovery should have been triggered (no_improvement_count should be reset)
        // Note: history might not be empty because the current step can add to it after recovery
        assert_eq!(optimizer.state.no_improvement_count, 0);
        Ok(())
    }
    #[test]
    fn test_lbfgs_nan_handling() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = LBFGSConfig::default();
        config.verbose = false;
        let mut optimizer = LBFGSOptimizer::new(config);
        // Function that returns NaN gradient
        struct NaNFunction;
        impl DifferentiableFunction for NaNFunction {
            fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
                let x = params[0].to_vec1::<f64>()?;
                Ok(x[0] * x[0])
            }
            fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
                let device = params[0].device();
                Ok(vec![Tensor::from_slice(&[f64::NAN], &[1], device)?])
            }
        }
        let function = Arc::new(NaNFunction);
        let mut params = vec![Tensor::from_slice(&[1.0], &[1], &device)?];
        // Should handle NaN gracefully (fallback to steepest descent)
        let result = optimizer.step(&mut params, function);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn test_lbfgs_gamma_update() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5, 1e-8);
        // Create gradients that will result in positive curvature
        let params0 = vec![Tensor::from_slice(&[0.0, 0.0], &[2], &device)?];
        let params1 = vec![Tensor::from_slice(&[-0.5, -0.5], &[2], &device)?];
        let params2 = vec![Tensor::from_slice(&[-1.0, -1.0], &[2], &device)?];
        let grad1 = vec![Tensor::from_slice(&[2.0, 2.0], &[2], &device)?];
        let grad2 = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        state.update(&params0, &params1, &grad1)?;
        state.update(&params1, &params2, &grad2)?;
        // Gamma should have been updated from default 1.0
        assert!(state.gamma() != 1.0);
        assert!(state.gamma() > 0.0);
        assert!(state.gamma().is_finite());
        Ok(())
    }
    #[test]
    fn test_lbfgs_empty_input_handling() -> CandleResult<()> {
        let mut state = LBFGSState::new(5, 1e-8);
        // Empty gradient should return error
        let empty_gradient: Vec<Tensor> = vec![];
        let empty_params: Vec<Tensor> = vec![];
        let result = state.estimate_optimum(&empty_params, &empty_gradient);
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_lbfgs_dimension_mismatch() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut config = LBFGSConfig::default();
        config.verbose = false;
        let mut optimizer = LBFGSOptimizer::new(config);
        // Function with mismatched gradient dimensions
        struct MismatchedFunction;
        impl DifferentiableFunction for MismatchedFunction {
            fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
                let x = params[0].to_vec1::<f64>()?;
                Ok(x[0] * x[0])
            }
            fn gradient(&self, _params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
                // Return wrong number of gradient tensors
                Ok(vec![])
            }
        }
        let function = MismatchedFunction;
        let mut params = vec![Tensor::from_slice(&[1.0], &[1], &device)?];
        let result = optimizer.step(&mut params, Arc::new(function));
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_lbfgs_very_small_gradient() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5, 1e-8);
        // Very small gradient
        let params = vec![Tensor::from_slice(&[1.0, 1.0], &[2], &device)?];
        let gradient = vec![Tensor::from_slice(&[1e-12, 1e-12], &[2], &device)?];
        let direction = state.estimate_optimum(&params, &gradient)?;
        // Should still return a valid direction (negative gradient)
        let dir_values = direction[0].to_vec1::<f64>()?;
        assert!(dir_values[0].is_finite());
        assert!(dir_values[1].is_finite());
        Ok(())
    }
    #[test]
    fn test_lbfgs_compute_direction_dimension_mismatch() -> CandleResult<()> {
        let device = Device::Cpu;
        let mut state = LBFGSState::new(5, 1e-8);
        // Mismatched dimensions
        let params = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let gradient = vec![
            Tensor::from_slice(&[1.0], &[1], &device)?,
            Tensor::from_slice(&[2.0], &[1], &device)?,
        ];
        let result = state.estimate_optimum(&params, &gradient);
        assert!(result.is_err());
        Ok(())
    }
}