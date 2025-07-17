//! Core optimizer traits and types for the QQN research framework.
//!
//! This module defines the fundamental abstractions that all optimization algorithms
//! must implement, along with supporting types for tracking optimization progress
//! and convergence behavior.

pub(crate) use crate::utils::math::DifferentiableFunction;
use candle_core::{Result, Tensor};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::time::Duration;

/// Additional metadata that optimizers can provide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetadata {
    /// Optimizer-specific data (e.g., QQN magnitude ratios, L-BFGS curvature info)
    pub optimizer_data: std::collections::HashMap<String, f64>,
    /// Timing information for different phases of the step
    pub timing_info: TimingInfo,
    /// Memory usage information
    pub memory_info: MemoryInfo,
}
impl Default for OptimizationMetadata {
    fn default() -> Self {
        Self {
            optimizer_data: std::collections::HashMap::new(),
            timing_info: TimingInfo::default(),
            memory_info: MemoryInfo::default(),
        }
    }
}
/// Core trait that all optimization algorithms must implement.
///
/// This trait provides a unified interface for different optimization methods,
/// enabling easy benchmarking and comparison between algorithms.
pub trait Optimizer: Send + Sync + std::fmt::Debug + 'static {
    /// Clone the optimizer (required for trait object safety)
    fn clone_box(&self) -> Box<dyn Optimizer>;
    /// Get optimizer configuration as a string for serialization
    fn config_string(&self) -> String {
        format!("{:?}", self)
    }

    /// Perform a single optimization step using a differentiable function
    ///
    /// # Arguments
    /// * `params` - Mutable reference to parameter tensors to be updated
    /// * `function` - Differentiable function to optimize
    ///
    /// # Returns
    /// A `StepResult` containing information about the optimization step
    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
    ) -> Result<StepResult>;

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
}

/// Result of a single optimization step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step size used in this iteration
    pub step_size: f64,

    /// Information about convergence status
    pub convergence_info: ConvergenceInfo,

    /// Additional optimizer-specific metadata
    pub metadata: OptimizationMetadata,
}

/// Information about convergence status and criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceInfo {
    /// Whether the optimizer has converged
    pub converged: bool,

    /// Change in function value from previous iteration
    pub function_change: Option<f64>,
}

impl Default for ConvergenceInfo {
    fn default() -> Self {
        Self {
            converged: false,
            function_change: None,
        }
    }
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
    pub fn with_function_change(mut self, change: f64) -> Self {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Peak memory usage during this step (in bytes)
    pub peak_memory: Option<usize>,

    /// Memory allocated for optimizer state (in bytes)
    pub state_memory: Option<usize>,

    /// Memory allocated for temporary computations (in bytes)
    pub temp_memory: Option<usize>,
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self {
            peak_memory: None,
            state_memory: None,
            temp_memory: None,
        }
    }
}

/// Configuration for convergence checking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceConfig {
    /// Tolerance for gradient norm convergence
    pub gradient_tolerance: f64,

    /// Tolerance for function value change convergence
    pub function_tolerance: f64,

    /// Tolerance for parameter change convergence
    pub parameter_tolerance: f64,

    /// Maximum number of iterations
    pub max_iterations: usize,

    /// Maximum optimization time
    pub max_time: Option<Duration>,

    /// Require multiple criteria to be satisfied
    pub require_multiple_criteria: bool,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            gradient_tolerance: 1e-6,
            function_tolerance: 1e-9,
            parameter_tolerance: 1e-8,
            max_iterations: 1000,
            max_time: None,
            require_multiple_criteria: false,
        }
    }
}

/// Utility for checking convergence based on multiple criteria
pub struct ConvergenceChecker {
    config: ConvergenceConfig,
    iteration_count: usize,
    start_time: std::time::Instant,
    previous_function_value: Option<f64>,
    previous_parameters: Option<Vec<Tensor>>,
}

impl ConvergenceChecker {
    /// Create a new convergence checker
    pub fn new(config: ConvergenceConfig) -> Self {
        Self {
            config,
            iteration_count: 0,
            start_time: std::time::Instant::now(),
            previous_function_value: None,
            previous_parameters: None,
        }
    }

    /// Check convergence for the current iteration
    pub fn check_convergence(
        &mut self,
        function_value: f64,
        gradients: &[Tensor],
        parameters: &[Tensor],
    ) -> Result<ConvergenceInfo> {
        self.iteration_count += 1;

        // Compute gradient norm
        let gradient_norm = crate::utils::math::compute_magnitude(gradients)?;

        // Check gradient norm convergence
        let gradient_converged = gradient_norm < self.config.gradient_tolerance;

        // Check function change convergence
        let function_change = self
            .previous_function_value
            .map(|prev| (function_value - prev).abs());
        let function_converged = function_change
            .map(|change| change < self.config.function_tolerance)
            .unwrap_or(false);

        // Check parameter change convergence
        let parameter_change = match self.previous_parameters {
            Some(ref prev_params) => Some(crate::utils::math::compute_parameter_change(
                parameters,
                prev_params,
            )?),
            _ => None,
        };
        let parameter_converged = parameter_change
            .map(|change| change < self.config.parameter_tolerance)
            .unwrap_or(false);

        // Check iteration limit
        let max_iterations_reached = self.iteration_count >= self.config.max_iterations;

        // Check time limit
        let max_time_reached = self
            .config
            .max_time
            .map(|max_time| self.start_time.elapsed() >= max_time)
            .unwrap_or(false);

        // Determine overall convergence
        let converged = if self.config.require_multiple_criteria {
            gradient_converged && (function_converged || parameter_converged)
        } else {
            gradient_converged || function_converged || parameter_converged
        } || max_iterations_reached
            || max_time_reached;
        // Update state for next iteration
        self.previous_function_value = Some(function_value);
        self.previous_parameters = Some(parameters.to_vec());

        Ok(ConvergenceInfo {
            converged,
            function_change,
        })
    }

    /// Reset the convergence checker for a new optimization run
    pub fn reset(&mut self) {
        self.iteration_count = 0;
        self.start_time = std::time::Instant::now();
        self.previous_function_value = None;
        self.previous_parameters = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, Tensor};


    #[test]
    fn test_convergence_info_builder() {
        let info = ConvergenceInfo::default().with_function_change(1e-10);

        assert_eq!(info.function_change, Some(1e-10));
    }

    #[test]
    fn test_convergence_config_default() {
        let config = ConvergenceConfig::default();
        assert_eq!(config.gradient_tolerance, 1e-6);
        assert_eq!(config.max_iterations, 1000);
        assert!(!config.require_multiple_criteria);
    }

    #[test]
    fn test_convergence_checker_basic() -> Result<()> {
        let config = ConvergenceConfig {
            gradient_tolerance: 1e-3,
            ..Default::default()
        };
        let mut checker = ConvergenceChecker::new(config);

        // Create test tensors
        let device = Device::Cpu;
        let gradients = vec![Tensor::from_slice(&[1e-4, 1e-4], (2,), &device)?];
        let parameters = vec![Tensor::from_slice(&[1.0, 2.0], (2,), &device)?];

        let info = checker.check_convergence(1.0, &gradients, &parameters)?;

        // Should converge due to small gradient norm
        assert!(info.converged);

        Ok(())
    }
}
