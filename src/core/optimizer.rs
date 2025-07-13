//! Core optimizer traits and types for the QQN research framework.
//!
//! This module defines the fundamental abstractions that all optimization algorithms
//! must implement, along with supporting types for tracking optimization progress
//! and convergence behavior.

use candle_core::Result as CandleResult;
use candle_core::{Result, Tensor};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::time::Duration;
use std::collections::HashMap;
/// Trait for differentiable functions that can compute both value and gradients
pub trait DifferentiableFunction: Send + Sync {
    /// Evaluate the function at the given point
    fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64>;
    /// Compute gradients at the given point
    fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>>;
    /// Compute both value and gradients (default implementation calls both separately)
    fn evaluate_with_gradient(&self, params: &[Tensor]) -> CandleResult<(f64, Vec<Tensor>)> {
        let value = self.evaluate(params)?;
        let grad = self.gradient(params)?;
        Ok((value, grad))
    }
}
/// Wrapper for functions that only provide objective evaluation
pub struct ObjectiveOnlyFunction<F>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
{
    objective_fn: F,
}
impl<F> ObjectiveOnlyFunction<F>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
{
    pub fn new(objective_fn: F) -> Self {
        Self { objective_fn }
    }
}
impl<F> DifferentiableFunction for ObjectiveOnlyFunction<F>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
{
    fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
        (self.objective_fn)(params)
    }
    fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // Numerical gradient computation using finite differences
        let h = 1e-8; // Step size for finite differences
        let mut gradients = Vec::new();

        for (i, param) in params.iter().enumerate() {
            let param_shape = param.shape();
            let param_data = param.to_vec1::<f64>()?;
            let mut grad_data = vec![0.0; param_data.len()];

            for j in 0..param_data.len() {
                // Forward difference
                let mut params_plus = params.to_vec();
                let mut data_plus = param_data.clone();
                data_plus[j] += h;
                params_plus[i] = Tensor::from_vec(data_plus, param_shape, param.device())?;
                let f_plus = (self.objective_fn)(&params_plus)?;

                // Backward difference
                let mut params_minus = params.to_vec();
                let mut data_minus = param_data.clone();
                data_minus[j] -= h;
                params_minus[i] = Tensor::from_vec(data_minus, param_shape, param.device())?;
                let f_minus = (self.objective_fn)(&params_minus)?;

                // Central difference
                grad_data[j] = (f_plus - f_minus) / (2.0 * h);
            }

            gradients.push(Tensor::from_vec(grad_data, param_shape, param.device())?);
        }
        Ok(gradients)
    }
}
/// Wrapper for separate objective and gradient functions
pub struct SeparateFunctions<F, G>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
    G: Fn(&[Tensor]) -> CandleResult<Vec<Tensor>> + Send + Sync,
{
    objective_fn: F,
    gradient_fn: G,
}
impl<F, G> SeparateFunctions<F, G>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
    G: Fn(&[Tensor]) -> CandleResult<Vec<Tensor>> + Send + Sync,
{
    pub fn new(objective_fn: F, gradient_fn: G) -> Self {
        Self { objective_fn, gradient_fn }
    }
}
impl<F, G> DifferentiableFunction for SeparateFunctions<F, G>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
    G: Fn(&[Tensor]) -> CandleResult<Vec<Tensor>> + Send + Sync,
{
    fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
        (self.objective_fn)(params)
    }
    fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        (self.gradient_fn)(params)
    }
}

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
/// Trait object-safe version of Optimizer for dynamic dispatch
pub trait OptimizerBox: Send + Sync + std::fmt::Debug {
    /// Perform a single optimization step with slice-based interface
    fn step_slice(
        &mut self,
        params: &mut [f64],
        gradients: &[f64],
    ) -> std::result::Result<StepResult, Box<dyn Error + Send + Sync>>;
    /// Reset the optimizer state
    fn reset(&mut self);
    /// Get the name of this optimizer
    fn name(&self) -> &str;
    /// Create a boxed clone of this optimizer
    fn clone_box(&self) -> Box<dyn OptimizerBox>;
    /// Check if the optimizer has converged
    fn has_converged(&self) -> bool {
        false
    }
}
/// Core trait that all optimization algorithms must implement.
///
/// This trait provides a unified interface for different optimization methods,
/// enabling easy benchmarking and comparison between algorithms.
pub trait Optimizer: Send + Sync + std::fmt::Debug {
    /// Configuration type for this optimizer
    type Config: Clone + Debug + Send + Sync;
    /// Internal state type for this optimizer
    type State: Clone + Debug + Send + Sync;
    /// Create a new optimizer instance with the given configuration
    fn new(config: Self::Config) -> Self
    where
        Self: Sized;


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

    /// Perform a single optimization step with pre-computed gradients
    ///
    /// # Arguments
    /// * `params` - Mutable reference to parameter tensors to be updated
    /// * `gradients` - Gradient tensors for the current parameters
    /// * `objective_value` - Function to evaluate objective at given parameters
    ///
    /// # Returns
    /// A `StepResult` containing information about the optimization step
    fn step_with_gradients(
        &mut self,
        params: &mut [Tensor],
        gradients: &[Tensor],
    ) -> Result<StepResult> {
        // Default implementation: create a thread-safe function wrapper
        let gradients_clone = gradients.to_vec();

        let function = SeparateFunctions::new(
            move |_params: &[Tensor]| -> CandleResult<f64> {
                // Since we have pre-computed gradients, return a dummy value
                // The actual objective evaluation should be done externally
                Ok(0.0)
            },
            move |_: &[Tensor]| Ok(gradients_clone.clone()),
        );
        self.step(params, &function)
    }

    /// Reset the optimizer state (useful for multiple runs)
    fn reset(&mut self);

    /// Get the current internal state of the optimizer
    fn state(&self) -> &Self::State;

    /// Get the name of this optimizer (for reporting and analysis)
    fn name(&self) -> &str;

    /// Check if the optimizer has converged based on its internal criteria
    fn has_converged(&self) -> bool {
        false // Default implementation - most optimizers don't track convergence internally
    }
    /// Create a boxed clone of this optimizer
    fn clone_box(&self) -> Box<dyn OptimizerBox>
    where
        Self: Clone + 'static,
    {
        Box::new(self.clone())
    }
}
/// Blanket implementation to make any Optimizer work as OptimizerBox
impl<T> OptimizerBox for T
where
    T: Optimizer + Clone + 'static,
{
    fn step_slice(
        &mut self,
        params: &mut [f64],
        gradients: &[f64],
    ) -> std::result::Result<StepResult, Box<dyn Error + Send + Sync>> {
        // Convert slices to tensors
        let device = candle_core::Device::Cpu;
        let param_tensor = candle_core::Tensor::from_slice(params, params.len(), &device)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        let grad_tensor = candle_core::Tensor::from_slice(gradients, gradients.len(), &device)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let mut param_tensors = vec![param_tensor];
        let grad_tensors = vec![grad_tensor];

        // Call the tensor-based step method
        // Create a dummy objective function that returns 0.0
        let result = self
            .step_with_gradients(&mut param_tensors, &grad_tensors)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        // Copy results back to slice
        if let Some(tensor) = param_tensors.first() {
            let data = tensor
                .to_vec1::<f64>()
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            params.copy_from_slice(&data);
        }

        Ok(result)
    }
    fn reset(&mut self) {
        Optimizer::reset(self);
    }
    fn name(&self) -> &str {
        Optimizer::name(self)
    }
    fn clone_box(&self) -> Box<dyn OptimizerBox> {
        Box::new(self.clone())
    }
    fn has_converged(&self) -> bool {
        Optimizer::has_converged(self)
    }
}

/// Result of a single optimization step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step size used in this iteration
    pub step_size: f64,

    /// Number of function evaluations used in this step
    pub function_evaluations: usize,

    /// Number of gradient evaluations used in this step
    pub gradient_evaluations: usize,

    /// Information about convergence status
    pub convergence_info: ConvergenceInfo,

    /// Additional optimizer-specific metadata
    pub metadata: OptimizationMetadata,
}

impl StepResult {
    /// Create a new step result with basic information
    pub fn new(step_size: f64, function_evals: usize, gradient_evals: usize) -> Self {
        Self {
            step_size,
            function_evaluations: function_evals,
            gradient_evaluations: gradient_evals,
            convergence_info: ConvergenceInfo::default(),
            metadata: OptimizationMetadata::default(),
        }
    }

    /// Create a step result indicating convergence
    pub fn converged(step_size: f64, function_evals: usize, gradient_evals: usize) -> Self {
        Self {
            step_size,
            function_evaluations: function_evals,
            gradient_evaluations: gradient_evals,
            convergence_info: ConvergenceInfo::converged(),
            metadata: OptimizationMetadata::default(),
        }
    }
}

/// Information about convergence status and criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceInfo {
    /// Whether the optimizer has converged
    pub converged: bool,

    /// Change in function value from previous iteration
    pub function_change: Option<f64>,

    /// Convergence criterion that was satisfied (if any)
    pub convergence_criterion: Option<ConvergenceCriterion>,
}

impl Default for ConvergenceInfo {
    fn default() -> Self {
        Self {
            converged: false,
            function_change: None,
            convergence_criterion: None,
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
        let parameter_change = if let Some(ref prev_params) = self.previous_parameters {
            Some(crate::utils::math::compute_parameter_change(
                parameters,
                prev_params,
            )?)
        } else {
            None
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

        // Determine convergence criterion
        let convergence_criterion = if max_iterations_reached {
            Some(ConvergenceCriterion::MaxIterations)
        } else if max_time_reached {
            Some(ConvergenceCriterion::MaxTime)
        } else if gradient_converged {
            Some(ConvergenceCriterion::GradientNorm)
        } else if function_converged {
            Some(ConvergenceCriterion::FunctionChange)
        } else if parameter_converged {
            Some(ConvergenceCriterion::ParameterChange)
        } else {
            None
        };

        // Update state for next iteration
        self.previous_function_value = Some(function_value);
        self.previous_parameters = Some(parameters.to_vec());

        Ok(ConvergenceInfo {
            converged,
            function_change,
            convergence_criterion,
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
    fn test_step_result_creation() {
        let result = StepResult::new(0.1, 1, 1);
        assert_eq!(result.step_size, 0.1);
        assert_eq!(result.function_evaluations, 1);
        assert_eq!(result.gradient_evaluations, 1);
        assert!(!result.convergence_info.converged);

        let converged_result = StepResult::converged(0.05, 2, 1);
        assert!(converged_result.convergence_info.converged);
    }

    #[test]
    fn test_convergence_info_builder() {
        let info = ConvergenceInfo::default()
            .with_function_change(1e-10);

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