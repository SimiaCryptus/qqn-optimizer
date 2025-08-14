//! Core optimizer traits and types for the QQN research framework.
//!
//! This module defines the fundamental abstractions that all optimization algorithms
//! must implement, along with supporting types for tracking optimization progress
//! and convergence behavior.

pub(crate) use crate::utils::math::DifferentiableFunction;
use candle_core::Result as CandleResult;
use candle_core::Tensor;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

/// Additional metadata that optimizers can provide
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationMetadata {
    /// Optimizer-specific data (e.g., QQN magnitude ratios, L-BFGS curvature info)
    pub optimizer_data: std::collections::HashMap<String, f64>,
    /// Timing information for different phases of the step
    pub timing_info: TimingInfo,
    /// Memory usage information
    pub memory_info: MemoryInfo,
}
/// Result of a complete optimization run
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Final function value
    pub fx: f64,
    /// Number of function evaluations
    pub num_f_evals: usize,
    /// Number of gradient evaluations  
    pub num_g_evals: usize,
    /// Whether optimization converged
    pub converged: bool,
    /// Final parameters
    pub x: Vec<f64>,
}

/// Core trait that all optimization algorithms must implement.
///
/// This trait provides a unified interface for different optimization methods,
/// enabling easy benchmarking and comparison between algorithms.
pub trait Optimizer: Send + Sync + Debug + 'static {
    /// Clone the optimizer (required for trait object safety)
    fn clone_box(&self) -> Box<dyn Optimizer>;
    /// Get optimizer configuration as a string for serialization
    fn config_string(&self) -> String {
        format!("{self:?}")
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
        function: Arc<dyn DifferentiableFunction + Send + Sync>,
    ) -> CandleResult<StepResult>;
    /// Optimize a function using closures (for compatibility with examples)
    ///
    /// # Arguments
    /// * `f` - Function to minimize
    /// * `g` - Gradient function
    /// * `x0` - Initial parameters
    /// * `max_evals` - Maximum function evaluations
    /// * `tol` - Gradient tolerance
    ///
    /// # Returns
    /// An `OptimizationResult` with the final state
    fn optimize(
        &mut self,
        f: Box<dyn Fn(&[f64]) -> f64 + Send + Sync>,
        g: Box<dyn Fn(&[f64]) -> Vec<f64> + Send + Sync>,
        x0: Vec<f64>,
        max_evals: usize,
        tol: f64,
    ) -> OptimizationResult {
        use crate::utils::math::DifferentiableFunction;
        use candle_core::{Device, Tensor};
        // Create a wrapper function that implements DifferentiableFunction
        struct ClosureFunction {
            f: Box<dyn Fn(&[f64]) -> f64 + Send + Sync>,
            g: Box<dyn Fn(&[f64]) -> Vec<f64> + Send + Sync>,
            f_evals: std::sync::Arc<std::sync::atomic::AtomicUsize>,
            g_evals: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }
        impl DifferentiableFunction for ClosureFunction {
            fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
                self.f_evals
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let x: Vec<f64> = params
                    .iter()
                    .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
                    .collect();
                Ok((self.f)(&x))
            }
            fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
                self.g_evals
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let x: Vec<f64> = params
                    .iter()
                    .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
                    .collect();
                let grad = (self.g)(&x);
                let device = &Device::Cpu;
                Ok(vec![Tensor::from_slice(&grad, &[grad.len()], device)?])
            }
        }
        let f_evals = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let g_evals = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let function = Arc::new(ClosureFunction {
            f,
            g,
            f_evals: f_evals.clone(),
            g_evals: g_evals.clone(),
        });
        // Convert initial point to tensor
        let device = &Device::Cpu;
        let mut params = vec![Tensor::from_slice(&x0, &[x0.len()], device).unwrap()];
        let mut converged = false;
        let mut iterations = 0;
        while iterations < max_evals {
            // Check gradient norm for convergence
            let grad = function.gradient(&params).unwrap();
            let grad_norm: f64 = grad[0]
                .flatten_all()
                .unwrap()
                .to_vec1::<f64>()
                .unwrap()
                .iter()
                .map(|x| x * x)
                .sum::<f64>()
                .sqrt();
            if grad_norm < tol {
                converged = true;
                break;
            }
            // Take optimization step
            match self.step(&mut params, function.clone()) {
                Ok(result) => {
                    if result.convergence_info.converged {
                        converged = true;
                        break;
                    }
                }
                Err(_) => break,
            }
            iterations += 1;
        }
        let final_x: Vec<f64> = params
            .iter()
            .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
            .collect();
        let final_fx = function.evaluate(&params).unwrap();
        OptimizationResult {
            fx: final_fx,
            num_f_evals: f_evals.load(std::sync::atomic::Ordering::Relaxed),
            num_g_evals: g_evals.load(std::sync::atomic::Ordering::Relaxed),
            converged,
            x: final_x,
        }
    }

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
    /// Get the stagnation multiplier for relaxed convergence criteria
    /// This multiplier is applied to tolerance values to make convergence less strict
    fn stagnation_multiplier(&self) -> f64 {
        1.0 // Default multiplier - no relaxation
    }
    /// Get the stagnation count threshold for applying relaxed convergence
    /// When stagnation is detected for this many iterations, relaxed criteria are used
    fn stagnation_count(&self) -> usize {
        1 // Default count - apply relaxation after 1 iteration of stagnation
    }
    /// Set the stagnation multiplier (mutable)
    fn set_stagnation_multiplier(&mut self, multiplier: f64);
    /// Set the stagnation count threshold (mutable)
    fn set_stagnation_count(&mut self, count: usize);
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvergenceInfo {
    /// Whether the optimizer has converged
    pub converged: bool,

    /// Change in function value from previous iteration
    pub function_change: Option<f64>,
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryInfo {
    /// Peak memory usage during this step (in bytes)
    pub peak_memory: Option<usize>,

    /// Memory allocated for optimizer state (in bytes)
    pub state_memory: Option<usize>,

    /// Memory allocated for temporary computations (in bytes)
    pub temp_memory: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_info_builder() {
        let info = ConvergenceInfo::default().with_function_change(1e-10);

        assert_eq!(info.function_change, Some(1e-10));
    }
}
