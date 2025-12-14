use anyhow::Result;
/// Trait defining an optimization problem interface
pub trait OptimizationProblem: Send + Sync {
    /// Get the problem name
    fn name(&self) -> &str;
    /// Get the problem dimension
    fn dimension(&self) -> usize;
    /// Get the initial starting point
    fn initial_point(&self) -> Vec<f32>;
    /// Evaluate the objective function at point x
    fn evaluate_f64(&self, x: &[f32]) -> Result<f32>;
    /// Compute the gradient at point x
    fn gradient_f64(&self, x: &[f32]) -> Result<Vec<f32>>;
    /// Get the optimal value if known
    fn optimal_value(&self) -> Option<f32>;
    /// Clone this optimization problem
    fn clone_problem(&self) -> Box<dyn OptimizationProblem>;
}

/// Wrapper to make benchmark functions work with the new DifferentiableFunction trait
pub struct BenchmarkFunctionWrapper<T: OptimizationProblem> {
    problem: T,
}
impl<T: OptimizationProblem> BenchmarkFunctionWrapper<T> {}
