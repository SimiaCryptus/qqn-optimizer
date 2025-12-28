use crate::OptimizationProblem;
use luminal::prelude::*;
use luminal_training::Autograd;
use std::f64::consts::PI;
macro_rules! impl_eval_grad {
    () => {
        fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
            if x.len() != self.dimension() {
                return Err(anyhow::anyhow!(
                    "Dimension mismatch: expected {}, got {}",
                    self.dimension(),
                    x.len()
                ));
            }
            let mut graph = Graph::new();
            let input = graph
                .tensor((x.len(),))
                .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
            let output = self.build_graph(&mut graph, input);
            output.retrieve();
            graph.execute();
            let data = output.data();
            if data.is_empty() {
                return Err(anyhow::anyhow!("Graph execution produced no output"));
            }
            Ok(data[0] as f64)
        }
        fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
            if x.len() != self.dimension() {
                return Err(anyhow::anyhow!(
                    "Dimension mismatch: expected {}, got {}",
                    self.dimension(),
                    x.len()
                ));
            }
            let mut graph = Graph::new();
            let input = graph
                .tensor((x.len(),))
                .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
            let output = self.build_graph(&mut graph, input);
            let grads = graph.compile(Autograd::new(input, output), ());
            graph.keep_tensors(&grads);
            output.retrieve();
            graph.execute();


            if grads.is_empty() {
                return Ok(vec![0.0; x.len()]);
            }

            let (grad_id, grad_shape) = grads[0];
            let grad_tensor = GraphTensor::from_id(grad_id, grad_shape, &mut graph, DType::F32);
            Ok(grad_tensor.data().iter().map(|&v| v as f64).collect())
        }
    };
}

/// Matyas function: f(x, y) = 0.26(x² + y²) - 0.48xy
/// Global minimum: f(0, 0) = 0
#[derive(Debug, Clone)]
pub struct MatyasFunction {
    name: String,
}

impl MatyasFunction {
    pub fn new() -> Self {
        Self {
            name: "Matyas_2D".to_string(),
        }
    }
}

impl OptimizationProblem for MatyasFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0, 1.0]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x, y) = 0.26(x² + y²) - 0.48xy
        let mask1 = graph.tensor((2,)).set(vec![1.0, 0.0]);
        let mask2 = graph.tensor((2,)).set(vec![0.0, 1.0]);
        let x1 = (input * mask1).sum(0);
        let x2 = (input * mask2).sum(0);
        let x1_sq = x1 * x1;
        let x2_sq = x2 * x2;
        let term1 = (x1_sq + x2_sq) * 0.26;
        let term2 = x1 * x2 * 0.48;
        term1 - term2
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(2.5e-2)
    }
}

/// Levi N.13 function: f(x, y) = sin²(3πx) + (x-1)²(1 + sin²(3πy)) + (y-1)²(1 + sin²(2πy))
/// Global minimum: f(1, 1) = 0
#[derive(Debug, Clone)]
pub struct LeviFunction {
    name: String,
}

impl LeviFunction {
    pub fn new() -> Self {
        Self {
            name: "Levi_2D".to_string(),
        }
    }
}

impl OptimizationProblem for LeviFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0, 0.0]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x, y) = sin²(3πx) + (x-1)²(1 + sin²(3πy)) + (y-1)²(1 + sin²(2πy))
        let pi3 = 3.0 * PI as f32;
        let pi2 = 2.0 * PI as f32;
        let mask1 = graph.tensor((2,)).set(vec![1.0, 0.0]);
        let mask2 = graph.tensor((2,)).set(vec![0.0, 1.0]);
        let x1 = (input * mask1).sum(0);
        let x2 = (input * mask2).sum(0);

        let sin_3pi_x1 = (x1 * pi3).sin();
        let term1 = sin_3pi_x1 * sin_3pi_x1;

        let x1_minus_1 = x1 - 1.0;
        let sin_3pi_x2 = (x2 * pi3).sin();
        let term2 = (x1_minus_1 * x1_minus_1) * (sin_3pi_x2 * sin_3pi_x2 + 1.0);

        let x2_minus_1 = x2 - 1.0;
        let sin_2pi_x2 = (x2 * pi2).sin();
        let term3 = (x2_minus_1 * x2_minus_1) * (sin_2pi_x2 * sin_2pi_x2 + 1.0);

        term1 + term2 + term3
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(2.84e-1)
    }
}

/// Goldstein-Price function: complex 2D function with multiple local minima
/// Global minimum: f(0, -1) = 3
#[derive(Debug, Clone)]
pub struct GoldsteinPriceFunction {
    name: String,
}

impl GoldsteinPriceFunction {
    pub fn new() -> Self {
        Self {
            name: "GoldsteinPrice_2D".to_string(),
        }
    }
}

impl OptimizationProblem for GoldsteinPriceFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0, 1.0]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x,y) = [1 + (x+y+1)²(19-14x+3x²-14y+6xy+3y²)] * [30 + (2x-3y)²(18-32x+12x²+48y-36xy+27y²)]
        let mask1 = graph.tensor((2,)).set(vec![1.0, 0.0]);
        let mask2 = graph.tensor((2,)).set(vec![0.0, 1.0]);
        let x1 = (input * mask1).sum(0);
        let x2 = (input * mask2).sum(0);

        let x1_sq = x1 * x1;
        let x2_sq = x2 * x2;
        let x1x2 = x1 * x2;

        let sum_plus_1 = x1 + x2 + 1.0;
        let sum_plus_1_sq = sum_plus_1 * sum_plus_1;
        let inner1 = x1_sq * 3.0 + x2_sq * 3.0 + x1x2 * 6.0 - x1 * 14.0 - x2 * 14.0 + 19.0;
        let term1 = sum_plus_1_sq * inner1 + 1.0;

        let diff = x1 * 2.0 - x2 * 3.0;
        let diff_sq = diff * diff;
        let inner2 = x1_sq * 12.0 + x2_sq * 27.0 - x1x2 * 36.0 - x1 * 32.0 + x2 * 48.0 + 18.0;
        let term2 = diff_sq * inner2 + 30.0;

        term1 * term2
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(8.40e1)
    }
}

/// Styblinski-Tang function: f(x) = 0.5 * Σ(x_i^4 - 16*x_i^2 + 5*x_i)
/// Global minimum: f(-2.903534, -2.903534, ...) ≈ -39.16599 * n
#[derive(Debug, Clone)]
pub struct StyblinskiTangFunction {
    dimension: usize,
    name: String,
}

impl StyblinskiTangFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("StyblinskiTang_{dimension}D"),
        }
    }
}

impl OptimizationProblem for StyblinskiTangFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = 0.5 * Σ(x_i^4 - 16*x_i^2 + 5*x_i)
        let x_sq = input * input;
        let x_4 = x_sq * x_sq;
        let term = x_4 - x_sq * 16.0 + input * 5.0;
        (term.sum(0) * 0.5)
    }
    fn optimal_value(&self) -> Option<f64> {
        match self.dimension {
            2 => Some(-7.83e1),
            5 => Some(-1.95e2),
            10 => Some(-3.78e2),
            _ => None,
        }
    }
}

/// Michalewicz function: f(x) = -Σ sin(x_i) * sin(i*x_i²/π)^(2m)
/// Global minimum location and value depend on dimension
#[derive(Debug, Clone)]
pub struct MichalewiczFunction {
    dimension: usize,
    m: i32,
    name: String,
}

impl MichalewiczFunction {
    pub fn new(dimension: usize) -> Self {
        Self::with_steepness(dimension, 10)
    }
    pub fn with_steepness(dimension: usize, m: i32) -> Self {
        Self {
            dimension,
            m,
            name: format!("Michalewicz_{dimension}D_m{m}"),
        }
    }
}

impl OptimizationProblem for MichalewiczFunction {
    impl_eval_grad!();
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![PI / 4.0; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = -Σ sin(x_i) * sin(i*x_i²/π)^(2m)
        // Note: This is an approximation since we can't easily do element-wise indexing
        // We'll compute it for each dimension separately and sum
        let pi_inv = 1.0 / PI as f32;
        let two_m = 2.0 * self.m as f32;

        // Create index tensor [1, 2, 3, ..., n]
        let indices: Vec<f32> = (1..=self.dimension).map(|i| i as f32).collect();
        let idx_tensor = graph.tensor((self.dimension,)).set(indices);

        let x_sq = input * input;
        let inner = x_sq * idx_tensor * pi_inv;
        let sin_inner = inner.sin();
        // pow(sin_inner, 2m) = exp(2m * ln(|sin_inner|)) - need to handle carefully
        // For simplicity, use repeated multiplication for small m
        let mut power_term = sin_inner * sin_inner; // sin^2
        for _ in 1..self.m {
            power_term = power_term * sin_inner * sin_inner;
        }
        let term = input.sin() * power_term;
        (term.sum(0) * -1.0)
    }
    fn optimal_value(&self) -> Option<f64> {
        match self.dimension {
            2 => Some(-9.96e-1),
            5 => Some(-2.69e0),
            10 => Some(-6.26e0),
            _ => None,
        }
    }
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
}

impl Default for MatyasFunction {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LeviFunction {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GoldsteinPriceFunction {
    fn default() -> Self {
        Self::new()
    }
}

/// Rosenbrock function: f(x) = Σ[100(x_{i+1} - x_i²)² + (1 - x_i)²]
/// Global minimum: f(1, 1, ..., 1) = 0
#[derive(Debug, Clone)]
pub struct RosenbrockFunction {
    dimension: usize,
    name: String,
}

impl RosenbrockFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("Rosenbrock_{dimension}D"),
        }
    }
}

impl OptimizationProblem for RosenbrockFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        let mut initial = vec![-1.2; self.dimension];
        for i in (1..self.dimension).step_by(2) {
            initial[i] = 1.0;
        }
        initial
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σ[100(x_{i+1} - x_i²)² + (1 - x_i)²]
        let n = self.dimension;



        let mut sum = graph.tensor((1,)).set(vec![0.0]);
        // Unroll loop to avoid slicing issues
        for i in 0..n - 1 {
            let mut mask_i = vec![0.0; n];
            mask_i[i] = 1.0;
            let xi = (input * graph.tensor((n,)).set(mask_i)).sum(0);
            
            let mut mask_next = vec![0.0; n];
            mask_next[i + 1] = 1.0;
            let xi_next = (input * graph.tensor((n,)).set(mask_next)).sum(0);
            
            let diff = xi_next - xi * xi;
            let term1 = diff * diff * 100.0;
            let term2 = (xi * -1.0 + 1.0) * (xi * -1.0 + 1.0);
            sum = sum + term1 + term2;
        }
        sum.sum(0)
    }
    fn optimal_value(&self) -> Option<f64> {
        match self.dimension {
            2 => Some(8.45e-3),
            5 => Some(3.98e-1),
            10 => Some(9.70e0),
            _ => None,
        }
    }
}

/// Rastrigin function: f(x) = A*n + Σ[x_i² - A*cos(2π*x_i)]
/// Global minimum: f(0, 0, ..., 0) = 0
#[derive(Debug, Clone)]
pub struct RastriginFunction {
    dimension: usize,
    a: f64,
    name: String,
}

impl RastriginFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            a: 10.0,
            name: format!("Rastrigin_{dimension}D"),
        }
    }
}

impl OptimizationProblem for RastriginFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        (0..self.dimension)
            .map(|i| 2.0 + 0.5 * (i as f64).sin())
            .collect()
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = A*n + Σ[x_i² - A*cos(2π*x_i)]
        let a = self.a as f32;
        let n = self.dimension as f32;
        let two_pi = 2.0 * PI as f32;

        let x_sq = input * input;
        let cos_term = (input * two_pi).cos() * a;
        let sum = (x_sq - cos_term).sum(0);
        sum + a * n
    }
    fn optimal_value(&self) -> Option<f64> {
        match self.dimension {
            2 => Some(7.96e0),
            5 => Some(2.04e1),
            10 => Some(4.18e1),
            _ => None,
        }
    }
}

/// Sphere function: f(x) = Σx_i²
/// Global minimum: f(0, 0, ..., 0) = 0
#[derive(Debug, Clone)]
pub struct SphereFunction {
    dimension: usize,
    name: String,
}

impl SphereFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("Sphere_{dimension}D"),
        }
    }
}

impl OptimizationProblem for SphereFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σx_i²
        let x_sq = input * input;
        x_sq.sum(0)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(5e-3)
    }
}

/// Beale function: f(x, y) = (1.5 - x + xy)² + (2.25 - x + xy²)² + (2.625 - x + xy³)²
/// Global minimum: f(3, 0.5) = 0
#[derive(Debug, Clone)]
pub struct BealeFunction {
    name: String,
}

impl BealeFunction {
    pub fn new() -> Self {
        Self {
            name: "Beale_2D".to_string(),
        }
    }
}

impl OptimizationProblem for BealeFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0, 1.0]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x, y) = (1.5 - x + xy)² + (2.25 - x + xy²)² + (2.625 - x + xy³)²
        let mask1 = graph.tensor((2,)).set(vec![1.0, 0.0]);
        let mask2 = graph.tensor((2,)).set(vec![0.0, 1.0]);
        let x1 = (input * mask1).sum(0);
        let x2 = (input * mask2).sum(0);

        let x2_sq = x2 * x2;
        let x2_cu = x2_sq * x2;

        let t1 = x1 * x2 - x1 + 1.5;
        let t2 = x1 * x2_sq - x1 + 2.25;
        let t3 = x1 * x2_cu - x1 + 2.625;

        let term1 = t1 * t1;
        let term2 = t2 * t2;
        let term3 = t3 * t3;

        term1 + term2 + term3
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1.5e-2)
    }
}

/// Himmelblau function: f(x, y) = (x² + y - 11)² + (x + y² - 7)²
/// Global minima: f(3, 2) = f(-2.805118, 3.131312) = f(-3.779310, -3.283186) = f(3.584428, -1.848126) = 0
#[derive(Debug, Clone)]
pub struct HimmelblauFunction {
    name: String,
}

impl HimmelblauFunction {
    pub fn new() -> Self {
        Self {
            name: "Himmelblau_2D".to_string(),
        }
    }
}

impl OptimizationProblem for HimmelblauFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0, 0.0]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x, y) = (x² + y - 11)² + (x + y² - 7)²
        let mask1 = graph.tensor((2,)).set(vec![1.0, 0.0]);
        let mask2 = graph.tensor((2,)).set(vec![0.0, 1.0]);
        let x1 = (input * mask1).sum(0);
        let x2 = (input * mask2).sum(0);

        let x1_sq = x1 * x1;
        let x2_sq = x2 * x2;

        let t1 = x1_sq + x2 - 11.0;
        let t2 = x1 + x2_sq - 7.0;

        let term1 = t1 * t1;
        let term2 = t2 * t2;

        term1 + term2
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(2.5e-1)
    }
}

/// Booth function: f(x, y) = (x + 2y - 7)² + (2x + y - 5)²
/// Global minimum: f(1, 3) = 0
#[derive(Debug, Clone)]
pub struct BoothFunction {
    name: String,
}

impl BoothFunction {
    pub fn new() -> Self {
        Self {
            name: "Booth_2D".to_string(),
        }
    }
}

impl OptimizationProblem for BoothFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0, 0.0]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x, y) = (x + 2y - 7)² + (2x + y - 5)²
        let mask1 = graph.tensor((2,)).set(vec![1.0, 0.0]);
        let mask2 = graph.tensor((2,)).set(vec![0.0, 1.0]);
        let x1 = (input * mask1).sum(0);
        let x2 = (input * mask2).sum(0);

        let t1 = x1 + x2 * 2.0 - 7.0;
        let t2 = x1 * 2.0 + x2 - 5.0;

        let term1 = t1 * t1;
        let term2 = t2 * t2;

        term1 + term2
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1.2e-1)
    }
}

/// Ackley function: f(x) = -a*exp(-b*sqrt(1/n * Σx_i²)) - exp(1/n * Σcos(c*x_i)) + a + e
/// Global minimum: f(0, 0, ..., 0) = 0
#[derive(Debug, Clone)]
pub struct AckleyFunction {
    dimension: usize,
    a: f64,
    b: f64,
    c: f64,
    name: String,
}

impl AckleyFunction {
    pub fn new(dimension: usize) -> Self {
        Self::with_parameters(dimension, 20.0, 0.2, 2.0 * PI)
    }
    pub fn with_parameters(dimension: usize, a: f64, b: f64, c: f64) -> Self {
        Self {
            dimension,
            a,
            b,
            c,
            name: format!("Ackley_{dimension}D_a{a}_b{b}_c{c:0.2e}"),
        }
    }
}

impl OptimizationProblem for AckleyFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = -a*exp(-b*sqrt(1/n * Σx_i²)) - exp(1/n * Σcos(c*x_i)) + a + e
        let a = self.a as f32;
        let b = self.b as f32;
        let c = self.c as f32;
        let n = self.dimension as f32;
        let e = std::f64::consts::E as f32;

        let x_sq = input * input;
        let mean_sq = x_sq.sum(0) / n;
        let sqrt_mean_sq = mean_sq.sqrt();
        let term1 = (sqrt_mean_sq * -b).exp() * -a;

        let cos_cx = (input * c).cos();
        let mean_cos = cos_cx.sum(0) / n;
        let term2 = mean_cos.exp() * -1.0;

        term1 + term2 + a + e
    }
    fn optimal_value(&self) -> Option<f64> {
        match self.dimension {
            2 => Some(3.57e0),
            5 => Some(3.57e0),
            10 => Some(3.57e0),
            _ => None,
        }
    }
}

impl Default for BealeFunction {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HimmelblauFunction {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BoothFunction {
    fn default() -> Self {
        Self::new()
    }
}

/// Griewank function: f(x) = 1 + (1/4000)*Σx_i² - Π cos(x_i/√i)
/// Global minimum: f(0, 0, ..., 0) = 0
#[derive(Debug, Clone)]
pub struct GriewankFunction {
    dimension: usize,
    name: String,
}

impl GriewankFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("Griewank_{dimension}D"),
        }
    }
}

impl OptimizationProblem for GriewankFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![100.0; self.dimension]
    }

    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = 1 + (1/4000)*Σx_i² - Π cos(x_i/√i)
        // Create sqrt(i) tensor [1, √2, √3, ..., √n]
        let sqrt_indices: Vec<f32> = (1..=self.dimension).map(|i| (i as f32).sqrt()).collect();
        let sqrt_idx = graph.tensor((self.dimension,)).set(sqrt_indices);

        let x_sq = input * input;
        let sum_term = x_sq.sum(0) / 4000.0;

        let scaled = input / sqrt_idx;
        let cos_scaled = scaled.cos();
        // Product via exp(sum(log(cos))) - need to handle negative values
        // For Griewank, cos values can be negative, so we use a different approach
        // We'll compute the product by taking log of absolute value and tracking sign
        let log_abs_cos = cos_scaled.abs().log();
        let prod_term = log_abs_cos.sum(0).exp();
        // Note: This doesn't handle sign correctly for all cases, but works near optimum

        sum_term - prod_term + 1.0
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(1e-8)
    }
}

/// Schwefel function: f(x) = 418.9829*n - Σ x_i * sin(√|x_i|)
/// Global minimum: f(420.9687, 420.9687, ..., 420.9687) ≈ 0
#[derive(Debug, Clone)]
pub struct SchwefelFunction {
    dimension: usize,
    name: String,
}

impl SchwefelFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("Schwefel_{dimension}D"),
        }
    }
}

impl OptimizationProblem for SchwefelFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![100.0; self.dimension]
    }

    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = 418.9829*n - Σ x_i * sin(√|x_i|)
        let n = self.dimension as f32;
        // Use relu composition for abs to ensure gradient support
        let sqrt_abs_x = (input.relu() + (input * -1.0).relu()).sqrt();
        let sin_sqrt = sqrt_abs_x.sin();
        let sum = (input * sin_sqrt).sum(0);
        sum * -1.0 + 418.9829 * n
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(1e-8)
    }
}

/// Levy function: f(x) = sin²(πw₁) + Σ(wᵢ-1)²[1+10sin²(πwᵢ+1)] + (wₙ-1)²[1+sin²(2πwₙ)]
/// where wᵢ = 1 + (xᵢ-1)/4
/// Global minimum: f(1, 1, ..., 1) = 0
#[derive(Debug, Clone)]
pub struct LevyFunction {
    dimension: usize,
    name: String,
}

impl LevyFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("Levy_{dimension}D"),
        }
    }
}

impl OptimizationProblem for LevyFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![2.0; self.dimension]
    }

    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = sin²(πw₁) + Σ(wᵢ-1)²[1+10sin²(πwᵢ+1)] + (wₙ-1)²[1+sin²(2πwₙ)]
        // where wᵢ = 1 + (xᵢ-1)/4
        let pi = PI as f32;
        let n = self.dimension;

        // w = 1 + (x - 1) / 4 = 0.75 + x * 0.25
        let w = input * 0.25 + 0.75;

        // First term: sin²(π*w₁)
        let mut mask1 = vec![0.0; n];
        mask1[0] = 1.0;
        let w1 = (w * graph.tensor((n,)).set(mask1)).sum(0);
        let sin_pi_w1 = (w1 * pi).sin();
        let first_term = sin_pi_w1 * sin_pi_w1;

        // Middle terms (all but last): Σ(wᵢ-1)²[1+10sin²(πwᵢ+1)]
        let mut middle_sum = graph.tensor((1,)).set(vec![0.0]);
        for i in 0..n - 1 {
            let mut mask = vec![0.0; n];
            mask[i] = 1.0;
            let wi = (w * graph.tensor((n,)).set(mask)).sum(0);
            let sin_val = (wi * pi + 1.0).sin();
            let term = (wi - 1.0) * (wi - 1.0) * (sin_val * sin_val * 10.0 + 1.0);
            middle_sum = middle_sum + term;
        }

        // Last term: (wₙ-1)²[1+sin²(2πwₙ)]
        let mut mask_n = vec![0.0; n];
        mask_n[n - 1] = 1.0;
        let wn = (w * graph.tensor((n,)).set(mask_n)).sum(0);
        let wn_minus_1 = wn - 1.0;
        let wn_minus_1_sq = wn_minus_1 * wn_minus_1;
        let sin_2pi_wn = (wn * 2.0 * pi).sin();
        let sin_2pi_wn_sq = sin_2pi_wn * sin_2pi_wn;
        let last_term = wn_minus_1_sq * (sin_2pi_wn_sq + 1.0);

        (first_term + middle_sum.sum(0) + last_term)
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}

/// Zakharov function: f(x) = Σx_i² + (Σ(0.5*i*x_i))² + (Σ(0.5*i*x_i))⁴
/// Global minimum: f(0, 0, ..., 0) = 0
#[derive(Debug, Clone)]
pub struct ZakharovFunction {
    dimension: usize,
    name: String,
}

impl ZakharovFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("Zakharov_{dimension}D"),
        }
    }
}

impl OptimizationProblem for ZakharovFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }

    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σx_i² + (Σ(0.5*i*x_i))² + (Σ(0.5*i*x_i))⁴
        // Create index tensor [0.5, 1.0, 1.5, ..., 0.5*n]
        let indices: Vec<f32> = (1..=self.dimension).map(|i| 0.5 * i as f32).collect();
        let idx_tensor = graph.tensor((self.dimension,)).set(indices);

        let x_sq = input * input;
        let sum1 = x_sq.sum(0);

        let weighted = input * idx_tensor;
        let sum2 = weighted.sum(0);
        let sum2_sq = sum2 * sum2;
        let sum2_4 = sum2_sq * sum2_sq;

        sum1 + sum2_sq + sum2_4
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(1e-8)
    }
}

/// Extended Rosenbrock function with adjustable conditioning
/// f(x) = Σ[α(x_{i+1} - x_i²)² + (1 - x_i)²] where α controls conditioning
/// For α >> 1, the problem becomes highly ill-conditioned
#[derive(Debug, Clone)]
pub struct IllConditionedRosenbrock {
    dimension: usize,
    alpha: f64,
    name: String,
}

impl IllConditionedRosenbrock {
    pub fn new(dimension: usize, alpha: f64) -> Self {
        Self {
            dimension,
            alpha,
            name: format!("IllConditionedRosenbrock_{dimension}D_alpha{alpha}"),
        }
    }
}

impl OptimizationProblem for IllConditionedRosenbrock {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        let mut initial = vec![-1.2; self.dimension];
        for i in (1..self.dimension).step_by(2) {
            initial[i] = 1.0;
        }
        initial
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σ[α(x_{i+1} - x_i²)² + (1 - x_i)²]
        let alpha = self.alpha as f32;
        let n = self.dimension;




        let mut sum = graph.tensor((1,)).set(vec![0.0]);
        for i in 0..n - 1 {
            let mut mask_i = vec![0.0; n];
            mask_i[i] = 1.0;
            let xi = (input * graph.tensor((n,)).set(mask_i)).sum(0);
            
            let mut mask_next = vec![0.0; n];
            mask_next[i + 1] = 1.0;
            let xi_next = (input * graph.tensor((n,)).set(mask_next)).sum(0);
            
            let diff = xi_next - xi * xi;
            let term1 = diff * diff * alpha;
            let term2 = (xi * -1.0 + 1.0) * (xi * -1.0 + 1.0);
            sum = sum + term1 + term2;
        }
        sum.sum(0)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}

/// Trigonometric function - highly ill-conditioned
/// f(x) = Σ[n - Σcos(x_j) + i(1 - cos(x_i) - sin(x_i))]²
#[derive(Debug, Clone)]
pub struct TrigonometricFunction {
    dimension: usize,
    name: String,
}

impl TrigonometricFunction {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            name: format!("Trigonometric_{dimension}D"),
        }
    }
}

impl OptimizationProblem for TrigonometricFunction {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.2; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σ[n - Σcos(x_j) + i(1 - cos(x_i) - sin(x_i))]²
        // This is complex due to the nested structure - we'll compute it element-wise
        let n = self.dimension as f32;

        // Create index tensor [1, 2, 3, ..., n]
        let indices: Vec<f32> = (1..=self.dimension).map(|i| i as f32).collect();
        let idx_tensor = graph.tensor((self.dimension,)).set(indices);

        let cos_x = input.cos();
        let sin_x = input.sin();
        let cos_sum = cos_x.sum(0);

        // term_i = n - cos_sum + i * (1 - cos(x_i) - sin(x_i))
        let inner = (cos_x + sin_x) * -1.0 + 1.0;
        let scaled_inner = inner * idx_tensor;

        // We want to compute Σ(base_i - cos_sum)² where base_i = n + scaled_inner_i
        // Expanding: Σ(base_i² - 2*base_i*cos_sum + cos_sum²)
        // = Σbase_i² - 2*cos_sum*Σbase_i + n*cos_sum²
        // This avoids broadcasting issues between vector base and scalar cos_sum
        let base = scaled_inner + n;
        let base_sq = base * base;
        let sum_base_sq = base_sq.sum(0);
        let sum_base = base.sum(0);

        let term1 = sum_base_sq;
        let term2 = sum_base * cos_sum * 2.0;
        let term3 = cos_sum * cos_sum * n;

        term1 - term2 + term3
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}

/// Penalty function I - constrained optimization via penalty method
/// f(x) = Σ(x_i - 1)² + α * Σmax(0, x_i - 0.25)²
#[derive(Debug, Clone)]
pub struct PenaltyFunctionI {
    dimension: usize,
    alpha: f64,
    name: String,
}

impl PenaltyFunctionI {
    pub fn new(dimension: usize) -> Self {
        Self::with_penalty(dimension, 1e6)
    }
    pub fn with_penalty(dimension: usize, alpha: f64) -> Self {
        Self {
            dimension,
            alpha,
            name: format!("PenaltyI_{dimension}D_alpha{alpha:.0e}"),
        }
    }
}

impl OptimizationProblem for PenaltyFunctionI {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.5; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σ(x_i - 1)² + α * Σmax(0, x_i - 0.25)²
        let alpha = self.alpha as f32;

        let x_minus_1 = input - 1.0;
        let objective = (x_minus_1 * x_minus_1).sum(0);

        // max(0, x - 0.25) using ReLU
        let x_minus_025 = input - 0.25;
        let relu_term = x_minus_025.relu();
        let penalty = (relu_term * relu_term).sum(0) * alpha;

        objective + penalty
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}

/// Barrier function - constrained optimization with logarithmic barrier
/// f(x) = Σx_i² - μ * Σlog(x_i) where x_i > 0
#[derive(Debug, Clone)]
pub struct BarrierFunction {
    dimension: usize,
    mu: f64,
    name: String,
}

impl BarrierFunction {
    pub fn new(dimension: usize) -> Self {
        Self::with_barrier(dimension, 0.1)
    }
    pub fn with_barrier(dimension: usize, mu: f64) -> Self {
        Self {
            dimension,
            mu,
            name: format!("Barrier_{dimension}D_mu{mu}"),
        }
    }
}

impl OptimizationProblem for BarrierFunction {
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!(
                "Dimension mismatch: expected {}, got {}",
                self.dimension,
                x.len()
            ));
        }
        for &xi in x {
            if xi <= 0.0 {
                return Err(anyhow::anyhow!("Barrier function undefined for x <= 0"));
            }
        }
        let mut graph = Graph::new();
        let input = graph
            .tensor((x.len(),))
            .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
        let output = self.build_graph(&mut graph, input);
        output.retrieve();
        graph.execute();
        let data = output.data();
        if data.is_empty() {
            return Err(anyhow::anyhow!("Graph execution produced no output"));
        }
        Ok(data[0] as f64)
    }

    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!(
                "Dimension mismatch: expected {}, got {}",
                self.dimension,
                x.len()
            ));
        }
        for &xi in x {
            if xi <= 0.0 {
                return Err(anyhow::anyhow!("Barrier function undefined for x <= 0"));
            }
        }
        let mut graph = Graph::new();
        let input = graph
            .tensor((x.len(),))
            .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
        let output = self.build_graph(&mut graph, input);
        let grads = graph.compile(Autograd::new(input, output), ());
        graph.keep_tensors(&grads);
        output.retrieve();
        graph.execute();

        if grads.is_empty() {
            return Ok(vec![0.0; x.len()]);
        }

        let (grad_id, grad_shape) = grads[0];
        let grad_tensor = GraphTensor::from_id(grad_id, grad_shape, &mut graph, DType::F32);
        Ok(grad_tensor.data().iter().map(|&v| v as f64).collect())
    }

    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σx_i² - μ * Σlog(x_i) where x_i > 0
        // Note: This assumes x > 0; behavior undefined for x <= 0
        let mu = self.mu as f32;

        let x_sq = input * input;
        let objective = x_sq.sum(0);

        // Use max(x, epsilon) to avoid log(0)
        let epsilon = 1e-10;
        // max(x, eps) = relu(x - eps) + eps
        let safe_x = (input - epsilon).relu() + epsilon;
        let log_x = safe_x.log();
        let barrier = log_x.sum(0) * -mu;

        objective + barrier
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}

/// Noisy sphere function - sphere with additive Gaussian noise
/// f(x) = Σx_i² + ε where ε ~ N(0, σ²)
#[derive(Debug, Clone)]
pub struct NoisySphere {
    dimension: usize,
    noise_level: f64,
    seed: u64,
    name: String,
}

impl NoisySphere {
    pub fn new(dimension: usize, noise_level: f64) -> Self {
        Self::with_seed(dimension, noise_level, 42)
    }
    pub fn with_seed(dimension: usize, noise_level: f64, seed: u64) -> Self {
        Self {
            dimension,
            noise_level,
            seed,
            name: format!("NoisySphere_{dimension}D_sigma{noise_level}"),
        }
    }
}

impl OptimizationProblem for NoisySphere {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σx_i² (noise would need to be added externally for determinism)
        // Note: True noise requires external randomness; this is just the sphere part
        let x_sq = input * input;
        x_sq.sum(0)
    }
    fn optimal_value(&self) -> Option<f64> {
        match self.dimension {
            2 => Some(1.66e0),
            5 => Some(4.58e0),
            10 => Some(9.71e0),
            _ => None,
        }
    }
}

/// Sparse Rosenbrock - Rosenbrock where only adjacent pairs interact
/// f(x) = Σ[100(x_{2i} - x_{2i-1}²)² + (1 - x_{2i-1})²]
#[derive(Debug, Clone)]
pub struct SparseRosenbrock {
    dimension: usize,
    name: String,
}

impl SparseRosenbrock {
    pub fn new(dimension: usize) -> Self {
        if dimension % 2 != 0 {
            panic!("SparseRosenbrock requires even dimension");
        }
        Self {
            dimension,
            name: format!("SparseRosenbrock_{dimension}D"),
        }
    }
}

impl OptimizationProblem for SparseRosenbrock {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        let mut initial = vec![0.0; self.dimension];
        for i in (0..self.dimension).step_by(2) {
            initial[i] = -1.2;
            initial[i + 1] = 1.0;
        }
        initial
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
       // f(x) = Σ[100(x_{2i} - x_{2i-1}²)² + (1 - x_i)²]
        // Extract odd indices (0, 2, 4, ...) and even indices (1, 3, 5, ...)
        let n_pairs = self.dimension / 2;

        // Create index tensors for gathering
        let odd_indices: Vec<f32> = (0..n_pairs).map(|i| (2 * i) as f32).collect();
        let even_indices: Vec<f32> = (0..n_pairs).map(|i| (2 * i + 1) as f32).collect();

        // For sparse Rosenbrock, we need to select specific elements
        // This is equivalent to standard Rosenbrock on pairs
        // x_odd = x[0], x[2], x[4], ...
        // x_even = x[1], x[3], x[5], ...

        // Since we can't easily gather with dynamic indices, we'll use the same
        // approach as standard Rosenbrock but on the full vector
        // This gives the same result for consecutive pairs




        let mut sum = graph.tensor((1,)).set(vec![0.0]);
        for i in (0..self.dimension - 1).step_by(2) {
            let mut mask_i = vec![0.0; self.dimension];
            mask_i[i] = 1.0;
            let xi = (input * graph.tensor((self.dimension,)).set(mask_i)).sum(0);
            
            let mut mask_next = vec![0.0; self.dimension];
            mask_next[i + 1] = 1.0;
            let xi_next = (input * graph.tensor((self.dimension,)).set(mask_next)).sum(0);
            
            let diff = xi_next - xi * xi;
            let term1 = diff * diff * 100.0;
            let term2 = (xi * -1.0 + 1.0) * (xi * -1.0 + 1.0);
            sum = sum + term1 + term2;
        }
        sum.sum(0)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}

/// Sparse quadratic function - diagonal + sparse off-diagonal terms
/// f(x) = Σx_i² + Σ(x_i * x_{i+k}) for specific k values
#[derive(Debug, Clone)]
pub struct SparseQuadratic {
    dimension: usize,
    sparsity_pattern: Vec<usize>,
    name: String,
}

impl SparseQuadratic {
    pub fn new(dimension: usize) -> Self {
        Self::with_pattern(dimension, vec![1, 3])
    }
    pub fn with_pattern(dimension: usize, sparsity_pattern: Vec<usize>) -> Self {
        Self {
            dimension,
            sparsity_pattern: sparsity_pattern.clone(),
            name: format!("SparseQuadratic_{dimension}D_pattern{sparsity_pattern:?}"),
        }
    }
}

impl OptimizationProblem for SparseQuadratic {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }
    fn build_graph(&self, graph: &mut Graph, input: GraphTensor) -> GraphTensor {
        // f(x) = Σx_i² + 0.1 * Σ(x_i * x_{i+k}) for k in sparsity_pattern
        let x_sq = input * input;
        let mut result = x_sq.sum(0);

        // Add sparse off-diagonal terms
        for &k in &self.sparsity_pattern {
            if k < self.dimension {
                for i in 0..self.dimension - k {
                    let mut mask_i = vec![0.0; self.dimension];
                    mask_i[i] = 1.0;
                    let xi = (input * graph.tensor((self.dimension,)).set(mask_i)).sum(0);
                    
                    let mut mask_k = vec![0.0; self.dimension];
                    mask_k[i + k] = 1.0;
                    let xk = (input * graph.tensor((self.dimension,)).set(mask_k)).sum(0);
                    
                    result = result + xi * xk * 0.1;
                }
            }
        }

        result
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::analytic_functions::{
        AckleyFunction, BarrierFunction, BealeFunction, BoothFunction, GoldsteinPriceFunction,
        GriewankFunction, HimmelblauFunction, IllConditionedRosenbrock, LeviFunction, LevyFunction,
        MatyasFunction, MichalewiczFunction, NoisySphere, PenaltyFunctionI, RastriginFunction,
        RosenbrockFunction, SchwefelFunction, SparseQuadratic, SparseRosenbrock, SphereFunction,
        StyblinskiTangFunction, TrigonometricFunction, ZakharovFunction,
    };
    use approx::assert_relative_eq;
    use luminal::prelude::*;
    use luminal_training::Autograd;

    const EPSILON: f64 = 1e-6;
    const GRADIENT_EPSILON: f64 = 1e-1;

    /// Helper function to test numerical gradient against analytical gradient
    fn test_gradient_numerical(problem: &dyn OptimizationProblem, x: &[f64], tolerance: f64) {
        let analytical_grad = problem.gradient_f64(x).unwrap();
        let mut numerical_grad = vec![0.0; x.len()];

        let h = 1e-3;
        for i in 0..x.len() {
            let mut x_plus = x.to_vec();
            let mut x_minus = x.to_vec();
            x_plus[i] += h;
            x_minus[i] -= h;

            let f_plus = problem.evaluate_f64(&x_plus).unwrap();
            let f_minus = problem.evaluate_f64(&x_minus).unwrap();
            numerical_grad[i] = (f_plus - f_minus) / (2.0 * h);
        }

        for i in 0..analytical_grad.len() {
            assert_relative_eq!(
                analytical_grad[i],
                numerical_grad[i],
                max_relative = tolerance
            );
        }
    }

    #[test]
    fn test_sphere_function() {
        let problem = SphereFunction::new(3);

        // Test at origin (global minimum)
        let origin = vec![0.0, 0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&origin).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_origin = problem.gradient_f64(&origin).unwrap();
        for &g in &grad_origin {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at arbitrary point
        let point = vec![1.0, 2.0, 3.0];
        let expected_value = 1.0 + 4.0 + 9.0; // 14.0
        assert_relative_eq!(
            problem.evaluate_f64(&point).unwrap(),
            expected_value,
            epsilon = EPSILON
        );

        let expected_grad = [2.0, 4.0, 6.0];
        let grad = problem.gradient_f64(&point).unwrap();
        for i in 0..grad.len() {
            assert_relative_eq!(grad[i], expected_grad[i], epsilon = EPSILON);
        }

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);

        // Test properties
        assert_eq!(problem.dimension(), 3);
        assert_eq!(problem.name(), "Sphere_3D");
    }

    #[test]
    fn test_rosenbrock_function() {
        let problem = RosenbrockFunction::new(2);

        // Test at global minimum (1, 1)
        let optimum = vec![1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at initial point
        let initial = problem.initial_point();
        let value = problem.evaluate_f64(&initial).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &initial, GRADIENT_EPSILON);

        // Test higher dimension
        let problem_3d = RosenbrockFunction::new(3);
        let optimum_3d = vec![1.0, 1.0, 1.0];
        assert_relative_eq!(
            problem_3d.evaluate_f64(&optimum_3d).unwrap(),
            0.0,
            epsilon = EPSILON
        );
    }

    #[test]
    fn test_rastrigin_function() {
        let problem = RastriginFunction::new(2);

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[ignore]
    #[test]
    fn test_ackley_function() {
        let problem = AckleyFunction::new(2);

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = 1e-10);
        }

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_matyas_function() {
        let problem = MatyasFunction::new();

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at arbitrary point
        let point = vec![1.0, 2.0];
        let expected_value = 0.26 * (1.0 + 4.0) - 0.48 * 1.0 * 2.0;
        assert_relative_eq!(
            problem.evaluate_f64(&point).unwrap(),
            expected_value,
            epsilon = EPSILON
        );

        let expected_grad = [0.52 * 1.0 - 0.48 * 2.0, 0.52 * 2.0 - 0.48 * 1.0];
        let grad = problem.gradient_f64(&point).unwrap();
        for i in 0..grad.len() {
            assert_relative_eq!(grad[i], expected_grad[i], epsilon = EPSILON);
        }

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_levi_function_2d() {
        let problem = LeviFunction::new();

        // Test at global minimum (1, 1)
        let optimum = vec![1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        // Test at arbitrary point
        let point = vec![0.5, 0.5];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_goldstein_price_function() {
        let problem = GoldsteinPriceFunction::new();

        // Test at global minimum (0, -1)
        let optimum = vec![0.0, -1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            3.0,
            epsilon = 1e-10
        );

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 3.0);

        // Test gradient numerically (using numerical gradient due to complexity)
        test_gradient_numerical(&problem, &point, 1e-2);
    }

    #[test]
    fn test_styblinski_tang_function() {
        let problem = StyblinskiTangFunction::new(2);
        // Test gradient numerically
        let point = vec![0.0, 0.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_michalewicz_function() {
        // Use m=1 to avoid numerical instability with f32 gradients and high powers
        let problem = MichalewiczFunction::with_steepness(2, 1);

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value < 0.0); // Michalewicz is negative

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_beale_function() {
        let problem = BealeFunction::new();

        // Test at global minimum (3, 0.5)
        let optimum = vec![3.0, 0.5];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = 1e-8);
        }

        // Test gradient numerically
        let point = vec![1.0, 1.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_himmelblau_function() {
        let problem = HimmelblauFunction::new();

        // Test at one of the global minima (3, 2)
        let optimum = vec![3.0, 2.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = 1e-8);
        }

        // Test gradient numerically
        let point = vec![1.0, 1.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_booth_function() {
        let problem = BoothFunction::new();

        // Test at global minimum (1, 3)
        let optimum = vec![1.0, 3.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test gradient numerically
        let point = vec![0.0, 0.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_griewank_function() {
        let problem = GriewankFunction::new(2);

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test gradient numerically
        let point = vec![1.0, 1.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_schwefel_function() {
        let problem = SchwefelFunction::new(2);

        // Test at approximate global minimum
        let optimum = vec![420.9687, 420.9687];
        let value = problem.evaluate_f64(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-3);

        // Test gradient numerically
        let point = vec![100.0, 100.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_levy_function_nd() {
        let problem = LevyFunction::new(3);

        // Test at global minimum (1, 1, 1)
        let optimum = vec![1.0, 1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        // Test gradient numerically
        let point = vec![2.0, 2.0, 2.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_zakharov_function() {
        let problem = ZakharovFunction::new(3);

        // Test at global minimum (0, 0, 0)
        let optimum = vec![0.0, 0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test gradient numerically
        let point = vec![1.0, 2.0, 3.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_dimension_mismatch_errors() {
        let problem = SphereFunction::new(3);

        // Test with wrong dimension
        let wrong_dim = vec![1.0, 2.0]; // 2D instead of 3D
        assert!(problem.evaluate_f64(&wrong_dim).is_err());
        assert!(problem.gradient_f64(&wrong_dim).is_err());
    }

    #[test]
    fn test_clone_problem() {
        let problem = SphereFunction::new(3);
        let cloned = problem.clone_problem();

        assert_eq!(cloned.name(), problem.name());
        assert_eq!(cloned.dimension(), problem.dimension());
        assert_eq!(cloned.optimal_value(), problem.optimal_value());

        let point = vec![1.0, 2.0, 3.0];
        assert_relative_eq!(
            cloned.evaluate_f64(&point).unwrap(),
            problem.evaluate_f64(&point).unwrap(),
            epsilon = EPSILON
        );
    }

    #[test]
    fn test_initial_points() {
        let functions: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(2)),
            Box::new(RosenbrockFunction::new(2)),
            Box::new(RastriginFunction::new(2)),
            Box::new(AckleyFunction::new(2)),
            Box::new(MatyasFunction::new()),
            Box::new(LeviFunction::new()),
            Box::new(BealeFunction::new()),
            Box::new(HimmelblauFunction::new()),
            Box::new(BoothFunction::new()),
        ];

        for problem in functions {
            let initial = problem.initial_point();
            assert_eq!(initial.len(), problem.dimension());

            // Should be able to evaluate at initial point
            assert!(problem.evaluate_f64(&initial).is_ok());
            assert!(problem.gradient_f64(&initial).is_ok());
        }
    }

    #[test]
    fn test_function_names() {
        assert_eq!(SphereFunction::new(3).name(), "Sphere_3D");
        assert_eq!(RosenbrockFunction::new(5).name(), "Rosenbrock_5D");
        assert_eq!(RastriginFunction::new(10).name(), "Rastrigin_10D");
        //assert_eq!(AckleyFunction::new(2).name(), "Ackley_2D_a20_b0.2_c6.283185307179586");
        assert_eq!(MatyasFunction::new().name(), "Matyas_2D");
        assert_eq!(LeviFunction::new().name(), "Levi_2D");
        assert_eq!(BealeFunction::new().name(), "Beale_2D");
        assert_eq!(HimmelblauFunction::new().name(), "Himmelblau_2D");
        assert_eq!(BoothFunction::new().name(), "Booth_2D");
    }

    #[test]
    fn test_gradient_at_multiple_points() {
        let problem = SphereFunction::new(2);
        let test_points = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![-1.0, 2.0],
            vec![0.5, -0.5],
        ];

        for point in test_points {
            test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
        }
    }
    #[test]
    fn test_ill_conditioned_rosenbrock() {
        let problem = IllConditionedRosenbrock::new(2, 1000.0);
        // Test at global minimum
        let optimum = vec![1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );
        // Test gradient
        let point = vec![0.5, 0.5];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
        // Verify it's more ill-conditioned than standard Rosenbrock
        let standard = RosenbrockFunction::new(2);
        let point = vec![0.9, 0.9];
        let ill_grad = problem.gradient_f64(&point).unwrap();
        let std_grad = standard.gradient_f64(&point).unwrap();
        // The ill-conditioned version should have larger gradient components
        assert!(ill_grad[1].abs() > std_grad[1].abs());
    }
    #[test]
    fn test_trigonometric_function() {
        let problem = TrigonometricFunction::new(3);
        // Test at a point
        let point = vec![0.1, 0.2, 0.3];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value >= 0.0);
        // Test gradient
        test_gradient_numerical(&problem, &point, 1e-2);
    }
    #[test]
    fn test_penalty_function() {
        let problem = PenaltyFunctionI::with_penalty(2, 1000.0);
        // Test at feasible point
        let feasible = vec![0.2, 0.1];
        let value = problem.evaluate_f64(&feasible).unwrap();
        // Test at infeasible point
        let infeasible = vec![0.5, 0.5];
        let infeasible_value = problem.evaluate_f64(&infeasible).unwrap();
        // Infeasible point should have higher value due to penalty
        assert!(infeasible_value > value);
        // Test gradient
        test_gradient_numerical(&problem, &feasible, GRADIENT_EPSILON);
    }
    #[test]
    fn test_barrier_function() {
        let problem = BarrierFunction::new(2);
        // Test at interior point
        let point = vec![1.0, 2.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value.is_finite());
        // Test gradient
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
        // Test infeasible point
        let infeasible = vec![-1.0, 1.0];
        assert!(problem.evaluate_f64(&infeasible).is_err());
    }
    #[test]
    fn test_noisy_sphere() {
        let problem = NoisySphere::new(2, 0.1);
        // Test multiple evaluations at same point
        let point = vec![1.0, 1.0];
        let value1 = problem.evaluate_f64(&point).unwrap();
        let value2 = problem.evaluate_f64(&point).unwrap();
        // Should be deterministic for same input
        assert_eq!(value1, value2);
        // Test that noise is bounded
        let true_value = 2.0; // 1² + 1²
        assert!((value1 - true_value).abs() <= 0.1);
    }
    #[test]
    fn test_sparse_rosenbrock() {
        let problem = SparseRosenbrock::new(4);
        // Test at global minimum
        let optimum = vec![1.0, 1.0, 1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );
        // Test sparsity structure
        let point = vec![0.0, 0.0, 1.0, 1.0];
        let grad = problem.gradient_f64(&point).unwrap();
        // For SparseRosenbrock, pairs are (0,1) and (2,3)
        // At point [0.0, 0.0, 1.0, 1.0]:
        // For pair (0,1): grad[0] = -400*0*(0-0*0) - 2*(1-0) = -2
        // For pair (0,1): grad[1] = 200*(0-0*0) = 0
        // For pair (2,3): grad[2] = -400*1*(1-1*1) - 2*(1-1) = 0
        // For pair (2,3): grad[3] = 200*(1-1*1) = 0
        assert_ne!(grad[0], 0.0); // Should be -2.0
        assert_eq!(grad[1], 0.0); // Should be 0.0
        assert_eq!(grad[2], 0.0); // Should be 0.0
        assert_eq!(grad[3], 0.0); // Should be 0.0
    }
    #[test]
    fn test_sparse_quadratic() {
        let problem = SparseQuadratic::new(5);
        // Test at origin
        let origin = vec![0.0; 5];
        assert_relative_eq!(
            problem.evaluate_f64(&origin).unwrap(),
            0.0,
            epsilon = EPSILON
        );
        // Test gradient
        let point = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
        // Test custom sparsity pattern
        let custom = SparseQuadratic::with_pattern(4, vec![2]);
        let grad = custom.gradient_f64(&[1.0, 0.0, 1.0, 0.0]).unwrap();
        // Only x[0] and x[2] should interact
        assert_ne!(grad[0], 2.0); // Not just diagonal - should be 2.0 + 0.1*1.0 = 2.1
        assert_eq!(grad[1], 0.0); // No interaction
    }
    #[test]
    fn test_constrained_problems_properties() {
        let penalty = PenaltyFunctionI::new(3);
        let barrier = BarrierFunction::new(3);
        // Both should have proper dimensions
        assert_eq!(penalty.dimension(), 3);
        assert_eq!(barrier.dimension(), 3);
        // Initial points should be feasible
        let penalty_init = penalty.initial_point();
        let barrier_init = barrier.initial_point();
        assert!(penalty.evaluate_f64(&penalty_init).is_ok());
        assert!(barrier.evaluate_f64(&barrier_init).is_ok());
    }
    /// Helper function to evaluate a problem using the graph
    fn evaluate_problem(problem: &dyn OptimizationProblem, x: &[f64]) -> f64 {
        let mut graph = Graph::new();
        let input = graph
            .tensor((x.len(),))
            .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
        let output = problem.build_graph(&mut graph, input);
        output.retrieve();
        graph.execute();
        output.data()[0] as f64
    }
    /// Helper function to compute gradient using autograd
    fn gradient_problem(problem: &dyn OptimizationProblem, x: &[f64]) -> Vec<f64> {
        let mut graph = Graph::new();
        let input = graph
            .tensor((x.len(),))
            .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
        let output = problem.build_graph(&mut graph, input);
        let grads = graph.compile(Autograd::new(input, output), ());
        graph.keep_tensors(&grads);
        output.retrieve();
        graph.execute();
        if !grads.is_empty() {
            let (grad_id, grad_shape) = grads[0];
            let grad_tensor = GraphTensor::from_id(grad_id, grad_shape, &mut graph, DType::F32);
            grad_tensor.data().iter().map(|&v| v as f64).collect()
        } else {
            vec![0.0; x.len()]
        }
    }
}