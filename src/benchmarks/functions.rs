use anyhow::Result;
use luminal::generic_compiler::GenericCompiler;
use luminal::op::DType;
use luminal::prelude::{Graph, GraphTensor};
use luminal_training::Autograd;

/// Trait defining an optimization problem interface
pub trait OptimizationProblem: Send + Sync {
    /// Get the problem name
    fn name(&self) -> &str;
    /// Get the problem dimension
    fn dimension(&self) -> usize;
    /// Build the computational graph for the objective function, returns the output tensor
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor;
    /// Get the initial starting point
    fn initial_point(&self) -> Vec<f64>;
    /// Get the optimal value if known
    fn optimal_value(&self) -> Option<f64>;
    /// Clone this optimization problem
    fn clone_problem(&self) -> Box<dyn OptimizationProblem>;
    /// Evaluate the objective function at point x using the graph
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        let mut graph = Graph::new();
        let input = graph
            .tensor((x.len(),))
            .set(x.iter().map(|&v| v as f32).collect::<Vec<_>>());
        let mut output = self.build_graph(&mut graph, input);
        output.retrieve();
        graph.compile(<()>::default(), (&mut output,));
        graph.execute();
        let data = output.data();
        if data.is_empty() {
            anyhow::bail!("Graph execution returned empty output");
        }
        Ok(data[0] as f64)
    }
    /// Compute the gradient at point x using automatic differentiation
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        let mut graph = Graph::new();
        let mut input = graph
            .tensor((x.len(),))
            .set(x.iter().map(|&v| v as f32).collect::<Vec<_>>());
        let mut output = self.build_graph(&mut graph, input);

        // Use Autograd to compute gradients with respect to input
        let input_vector = vec![input.id];
        let grads = graph.compile(Autograd::new(&input_vector, output), ());
        // Keep the gradient tensors so they aren't optimized away
        input.keep();
        graph.keep_tensors(&grads);
        output.keep();
        // Also retrieve the gradient tensor
        if grads.is_empty() {
            anyhow::bail!("Autograd returned no gradients");
        }
        let mut grad_tensor = GraphTensor::from_id(grads[0].0, input.shape, &mut graph, DType::F32);
        grad_tensor.retrieve();

        graph.compile(
            (
                #[cfg(not(feature = "cuda"))]
                GenericCompiler::default(),
                #[cfg(feature = "metal")]
                luminal_metal::MetalCompiler::<f32>::default(),
                #[cfg(feature = "cuda")]
                luminal_cuda::CudaCompiler::<f32>::default(),
            ),
            (&mut input, &mut output, &mut grad_tensor),
        );
        // Execute the graph
        graph.execute();

        // Extract gradient values
        let grad_data = grad_tensor.data();
        if grad_data.is_empty() {
            anyhow::bail!("Gradient computation returned empty output");
        }
        // Require in_data to be same size as grad_data
        if x.len() != grad_data.len() {
            anyhow::bail!(
                "Gradient size mismatch: input size {} vs gradient size {}",
                x.len(),
                grad_data.len()
            );
        }
        Ok(grad_data.iter().map(|&v| v as f64).collect())
    }
}

/// Wrapper to make benchmark functions work with the new DifferentiableFunction trait
pub struct BenchmarkFunctionWrapper<T: OptimizationProblem> {
    problem: T,
}
impl<T: OptimizationProblem> BenchmarkFunctionWrapper<T> {}
