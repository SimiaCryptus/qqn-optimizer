use crate::utils::math::{tensor_from_vec, tensors_to_vec, DifferentiableFunction};
use anyhow::Result;
use candle_core::Tensor;
/// Trait defining an optimization problem interface
pub trait OptimizationProblem: Send + Sync {
    /// Get the problem name
    fn name(&self) -> &str;
    /// Get the problem dimension
    fn dimension(&self) -> usize;
    /// Get the initial starting point
    fn initial_point(&self) -> Vec<f64>;
    /// Evaluate the objective function at point x
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64>;
    /// Compute the gradient at point x
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>>;
    /// Get the optimal value if known
    fn optimal_value(&self) -> Option<f64>;
    /// Clone this optimization problem
    fn clone_problem(&self) -> Box<dyn OptimizationProblem>;
}

/// Wrapper to make benchmark functions work with the new DifferentiableFunction trait
pub struct BenchmarkFunctionWrapper<T: OptimizationProblem> {
    problem: T,
}
impl<T: OptimizationProblem> BenchmarkFunctionWrapper<T> {}
impl<T: OptimizationProblem> DifferentiableFunction for BenchmarkFunctionWrapper<T> {
    fn evaluate(&self, params: &[Tensor]) -> candle_core::Result<f64> {
        let x_vec = tensors_to_vec(params);
        self.problem
            .evaluate_f64(&x_vec)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))
    }
    fn gradient(&self, params: &[Tensor]) -> candle_core::Result<Vec<Tensor>> {
        let x_vec = tensors_to_vec(params);
        let grad_vec = self
            .problem
            .gradient_f64(&x_vec)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
        Ok(vec![tensor_from_vec(grad_vec)])
    }
}
