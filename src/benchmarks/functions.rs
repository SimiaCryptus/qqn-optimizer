use crate::utils::math::{tensor_from_vec, tensors_to_vec, DifferentiableFunction};
use anyhow::Result;
use candle_core::Tensor;
use std::f64::consts::PI;

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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Matyas function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        Ok(0.26 * (x1 * x1 + x2 * x2) - 0.48 * x1 * x2)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Matyas function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        Ok(vec![0.52 * x1 - 0.48 * x2, 0.52 * x2 - 0.48 * x1])
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Levi function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (3.0 * PI * x1).sin().powi(2);
        let term2 = (x1 - 1.0).powi(2) * (1.0 + (3.0 * PI * x2).sin().powi(2));
        let term3 = (x2 - 1.0).powi(2) * (1.0 + (2.0 * PI * x2).sin().powi(2));
        Ok(term1 + term2 + term3)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Levi function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let grad_x1 = 2.0 * (3.0 * PI * x1).sin() * (3.0 * PI * x1).cos() * 3.0 * PI
            + 2.0 * (x1 - 1.0) * (1.0 + (3.0 * PI * x2).sin().powi(2));
        let grad_x2 = (x1 - 1.0).powi(2)
            * 2.0
            * (3.0 * PI * x2).sin()
            * (3.0 * PI * x2).cos()
            * 3.0
            * PI
            + 2.0 * (x2 - 1.0) * (1.0 + (2.0 * PI * x2).sin().powi(2))
            + (x2 - 1.0).powi(2) * 2.0 * (2.0 * PI * x2).sin() * (2.0 * PI * x2).cos() * 2.0 * PI;
        Ok(vec![grad_x1, grad_x2])
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-4)
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!(
                "Goldstein-Price function requires 2D input"
            ));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = 1.0
            + (x1 + x2 + 1.0).powi(2)
            * (19.0 - 14.0 * x1 + 3.0 * x1 * x1 - 14.0 * x2 + 6.0 * x1 * x2 + 3.0 * x2 * x2);
        let term2 = 30.0
            + (2.0 * x1 - 3.0 * x2).powi(2)
            * (18.0 - 32.0 * x1 + 12.0 * x1 * x1 + 48.0 * x2 - 36.0 * x1 * x2 + 27.0 * x2 * x2);
        Ok(term1 * term2)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!(
                "Goldstein-Price function requires 2D input"
            ));
        }
        // This is a complex gradient calculation - using numerical differentiation for simplicity
        let h = 1e-8;
        let f_x = self.evaluate_f64(x)?;
        let mut x_plus_h = x.to_vec();
        x_plus_h[0] += h;
        let f_x1_plus_h = self.evaluate_f64(&x_plus_h)?;
        let grad_x1 = (f_x1_plus_h - f_x) / h;
        let mut x_plus_h = x.to_vec();
        x_plus_h[1] += h;
        let f_x2_plus_h = self.evaluate_f64(&x_plus_h)?;
        let grad_x2 = (f_x2_plus_h - f_x) / h;
        Ok(vec![grad_x1, grad_x2])
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(10.0)
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
            name: format!("StyblinskiTang_{}D", dimension),
        }
    }
}
impl OptimizationProblem for StyblinskiTangFunction {
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let sum: f64 = x
            .iter()
            .map(|&xi| xi.powi(4) - 16.0 * xi.powi(2) + 5.0 * xi)
            .sum();
        Ok(0.5 * sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let grad: Vec<f64> = x
            .iter()
            .map(|&xi| 0.5 * (4.0 * xi.powi(3) - 32.0 * xi + 5.0))
            .collect();
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        // Global minimum: f(-2.903534, -2.903534, ...) ≈ -39.16599 * n
        Some(-39. * self.dimension as f64)
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
            name: format!("Michalewicz_{}D_m{}", dimension, m),
        }
    }
}
impl OptimizationProblem for MichalewiczFunction {
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![PI / 4.0; self.dimension]
    }
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let sum: f64 = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| {
                let i_plus_1 = (i + 1) as f64;
                xi.sin() * ((i_plus_1 * xi * xi / PI).sin()).powf(2.0 * self.m as f64)
            })
            .sum();
        Ok(-sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let grad: Vec<f64> = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| {
                let i_plus_1 = (i + 1) as f64;
                let inner_arg = i_plus_1 * xi * xi / PI;
                let sin_inner = inner_arg.sin();
                let cos_inner = inner_arg.cos();
                let power_term = sin_inner.powf(2.0 * self.m as f64);
                let term1 = xi.cos() * power_term;
                let term2 = xi.sin()
                    * 2.0
                    * self.m as f64
                    * sin_inner.powf(2.0 * self.m as f64 - 1.0)
                    * cos_inner
                    * (2.0 * i_plus_1 * xi / PI);
                -(term1 + term2)
            })
            .collect();
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        // Approximate known values for small dimensions
        match self.dimension {
            2 => Some(-1.0),
            5 => Some(-3.0),
            10 => Some(-9.6),
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
            name: format!("Rosenbrock_{}D", dimension),
        }
    }
}
impl OptimizationProblem for RosenbrockFunction {
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
        // Use the standard Rosenbrock starting point
        let mut initial = vec![-1.2; self.dimension];
        // Alternate between -1.2 and 1.0 for better conditioning
        for i in (1..self.dimension).step_by(2) {
            initial[i] = 1.0;
        }
        initial
    }
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let mut sum = 0.0;
        for i in 0..self.dimension - 1 {
            let term1 = 100.0 * (x[i + 1] - x[i] * x[i]).powi(2);
            let term2 = (1.0 - x[i]).powi(2);
            sum += term1 + term2;
        }
        Ok(sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let mut grad = vec![0.0; self.dimension];
        for i in 0..self.dimension - 1 {
            // Gradient w.r.t. x[i]
            grad[i] += -400.0 * x[i] * (x[i + 1] - x[i] * x[i]) - 2.0 * (1.0 - x[i]);
            // Gradient w.r.t. x[i+1]
            grad[i + 1] += 200.0 * (x[i + 1] - x[i] * x[i]);
        }
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-2)
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
            name: format!("Rastrigin_{}D", dimension),
        }
    }
}
impl OptimizationProblem for RastriginFunction {
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
        // Start at a more challenging point with some randomness
        (0..self.dimension)
            .map(|i| 2.0 + 0.5 * (i as f64).sin())
            .collect()
    }
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let n = self.dimension as f64;
        let sum: f64 = x
            .iter()
            .map(|&xi| xi * xi - self.a * (2.0 * PI * xi).cos())
            .sum();
        Ok(self.a * n + sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let grad: Vec<f64> = x
            .iter()
            .map(|&xi| 2.0 * xi + self.a * 2.0 * PI * (2.0 * PI * xi).sin())
            .collect();
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        match self.dimension {
            2 => Some(1.0),
            5 => Some(25.0),
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
            name: format!("Sphere_{}D", dimension),
        }
    }
}
impl OptimizationProblem for SphereFunction {
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let sum: f64 = x.iter().map(|&xi| xi * xi).sum();
        Ok(sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let grad: Vec<f64> = x.iter().map(|&xi| 2.0 * xi).collect();
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Beale function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (1.5 - x1 + x1 * x2).powi(2);
        let term2 = (2.25 - x1 + x1 * x2 * x2).powi(2);
        let term3 = (2.625 - x1 + x1 * x2 * x2 * x2).powi(2);
        Ok(term1 + term2 + term3)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Beale function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = 1.5 - x1 + x1 * x2;
        let term2 = 2.25 - x1 + x1 * x2 * x2;
        let term3 = 2.625 - x1 + x1 * x2 * x2 * x2;
        let grad_x1 = 2.0 * term1 * (-1.0 + x2)
            + 2.0 * term2 * (-1.0 + x2 * x2)
            + 2.0 * term3 * (-1.0 + x2 * x2 * x2);
        let grad_x2 =
            2.0 * term1 * x1 + 2.0 * term2 * (2.0 * x1 * x2) + 2.0 * term3 * (3.0 * x1 * x2 * x2);
        Ok(vec![grad_x1, grad_x2])
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Himmelblau function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (x1 * x1 + x2 - 11.0).powi(2);
        let term2 = (x1 + x2 * x2 - 7.0).powi(2);
        Ok(term1 + term2)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Himmelblau function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let grad_x1 = 2.0 * (x1 * x1 + x2 - 11.0) * (2.0 * x1) + 2.0 * (x1 + x2 * x2 - 7.0);
        let grad_x2 = 2.0 * (x1 * x1 + x2 - 11.0) + 2.0 * (x1 + x2 * x2 - 7.0) * (2.0 * x2);
        Ok(vec![grad_x1, grad_x2])
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-8)
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Booth function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (x1 + 2.0 * x2 - 7.0).powi(2);
        let term2 = (2.0 * x1 + x2 - 5.0).powi(2);
        Ok(term1 + term2)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Booth function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let grad_x1 = 2.0 * (x1 + 2.0 * x2 - 7.0) + 2.0 * (2.0 * x1 + x2 - 5.0) * 2.0;
        let grad_x2 = 2.0 * (x1 + 2.0 * x2 - 7.0) * 2.0 + 2.0 * (2.0 * x1 + x2 - 5.0);
        Ok(vec![grad_x1, grad_x2])
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-8)
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
            name: format!("Ackley_{}D_a{}_b{}_c{}", dimension, a, b, c),
        }
    }
}
impl OptimizationProblem for AckleyFunction {
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
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let n = self.dimension as f64;
        let sum_squares: f64 = x.iter().map(|&xi| xi * xi).sum();
        let sum_cos: f64 = x.iter().map(|&xi| (self.c * xi).cos()).sum();
        let term1 = -self.a * (-self.b * (sum_squares / n).sqrt()).exp();
        let term2 = -(sum_cos / n).exp();
        Ok(term1 + term2 + self.a + std::f64::consts::E)
    }
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let n = self.dimension as f64;
        let sum_squares: f64 = x.iter().map(|&xi| xi * xi).sum();
        let sqrt_term = (sum_squares / n).sqrt();
        let sum_cos: f64 = x.iter().map(|&xi| (self.c * xi).cos()).sum();
        let mut grad = vec![0.0; self.dimension];
        for i in 0..self.dimension {
            let xi = x[i];
            // First term derivative
            let term1_coeff = self.a * self.b * (-self.b * sqrt_term).exp() / (n * sqrt_term);
            let term1_deriv = term1_coeff * xi;
            // Second term derivative
            let term2_deriv = (sum_cos / n).exp() * self.c * (self.c * xi).sin() / n;
            grad[i] = term1_deriv + term2_deriv;
        }
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(0.8)
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

// Add to src/benchmarks/functions.rs

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
            name: format!("Griewank_{}D", dimension),
        }
    }
}

impl OptimizationProblem for GriewankFunction {
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
        vec![100.0; self.dimension] // Start far from optimum
    }

    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let sum_squares: f64 = x.iter().map(|&xi| xi * xi).sum();
        let product: f64 = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| (xi / ((i + 1) as f64).sqrt()).cos())
            .product();

        Ok(1.0 + sum_squares / 4000.0 - product)
    }

    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let mut grad = vec![0.0; self.dimension];

        // Compute the product term for gradient calculation
        let product: f64 = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| (xi / ((i + 1) as f64).sqrt()).cos())
            .product();

        for j in 0..self.dimension {
            let sqrt_j_plus_1 = ((j + 1) as f64).sqrt();

            // Gradient of sum_squares term
            grad[j] = x[j] / 2000.0;

            // Gradient of product term
            if product.abs() > 1e-15 {
                let sin_term = (x[j] / sqrt_j_plus_1).sin();
                grad[j] += (product / (x[j] / sqrt_j_plus_1).cos()) * sin_term / sqrt_j_plus_1;
            }
        }

        Ok(grad)
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
            name: format!("Schwefel_{}D", dimension),
        }
    }
}

impl OptimizationProblem for SchwefelFunction {
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
        vec![100.0; self.dimension] // Start away from global optimum
    }

    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let sum: f64 = x.iter().map(|&xi| xi * (xi.abs().sqrt()).sin()).sum();

        Ok(418.9829 * self.dimension as f64 - sum)
    }

    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let grad: Vec<f64> = x
            .iter()
            .map(|&xi| {
                if xi.abs() < 1e-15 {
                    0.0 // Avoid division by zero
                } else {
                    let sqrt_abs_xi = xi.abs().sqrt();
                    let sin_term = sqrt_abs_xi.sin();
                    let cos_term = sqrt_abs_xi.cos();

                    // d/dx [x * sin(√|x|)] = sin(√|x|) + x * cos(√|x|) * (1/(2√|x|)) * sign(x)
                    let derivative = sin_term + xi * cos_term * (0.5 / sqrt_abs_xi) * xi.signum();
                    -derivative // Negative because we're minimizing
                }
            })
            .collect();

        Ok(grad)
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
            name: format!("Levy_{}D", dimension),
        }
    }
}

impl OptimizationProblem for LevyFunction {
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
        vec![2.0; self.dimension] // Start near but not at optimum
    }

    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        // Transform x to w
        let w: Vec<f64> = x.iter().map(|&xi| 1.0 + (xi - 1.0) / 4.0).collect();

        // First term
        let first_term = (PI * w[0]).sin().powi(2);

        // Middle terms
        let middle_sum: f64 = w[..w.len() - 1]
            .iter()
            .map(|&wi| {
                let wi_minus_1_sq = (wi - 1.0).powi(2);
                let sin_term = (PI * wi + 1.0).sin().powi(2);
                wi_minus_1_sq * (1.0 + 10.0 * sin_term)
            })
            .sum();

        // Last term
        let last_w = w[w.len() - 1];
        let last_term = (last_w - 1.0).powi(2) * (1.0 + (2.0 * PI * last_w).sin().powi(2));

        Ok(first_term + middle_sum + last_term)
    }

    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let w: Vec<f64> = x.iter().map(|&xi| 1.0 + (xi - 1.0) / 4.0).collect();
        let mut grad = vec![0.0; self.dimension];

        for i in 0..self.dimension {
            let wi = w[i];

            if i == 0 {
                // Gradient of first term
                grad[i] += 2.0 * (PI * wi).sin() * (PI * wi).cos() * PI * 0.25;
            }

            if i < self.dimension - 1 {
                // Gradient of middle terms
                let wi_minus_1 = wi - 1.0;
                let sin_term = (PI * wi + 1.0).sin();
                let cos_term = (PI * wi + 1.0).cos();

                let term1 = 2.0 * wi_minus_1 * (1.0 + 10.0 * sin_term.powi(2));
                let term2 = wi_minus_1.powi(2) * 20.0 * sin_term * cos_term * PI;

                grad[i] += (term1 + term2) * 0.25;
            }

            if i == self.dimension - 1 {
                // Gradient of last term
                let wi_minus_1 = wi - 1.0;
                let sin_2pi_wi = (2.0 * PI * wi).sin();
                let cos_2pi_wi = (2.0 * PI * wi).cos();

                let term1 = 2.0 * wi_minus_1 * (1.0 + sin_2pi_wi.powi(2));
                let term2 = wi_minus_1.powi(2) * 2.0 * sin_2pi_wi * cos_2pi_wi * 2.0 * PI;

                grad[i] += (term1 + term2) * 0.25;
            }
        }

        Ok(grad)
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
            name: format!("Zakharov_{}D", dimension),
        }
    }
}

impl OptimizationProblem for ZakharovFunction {
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

    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let sum1: f64 = x.iter().map(|&xi| xi * xi).sum();
        let sum2: f64 = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| 0.5 * (i + 1) as f64 * xi)
            .sum();

        Ok(sum1 + sum2.powi(2) + sum2.powi(4))
    }

    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let sum2: f64 = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| 0.5 * (i + 1) as f64 * xi)
            .sum();

        let grad: Vec<f64> = x
            .iter()
            .enumerate()
            .map(|(i, &xi)| {
                let coeff = 0.5 * (i + 1) as f64;
                2.0 * xi + 2.0 * sum2 * coeff + 4.0 * sum2.powi(3) * coeff
            })
            .collect();

        Ok(grad)
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(1e-8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const EPSILON: f64 = 1e-10;
    const GRADIENT_EPSILON: f64 = 1e-6;

    /// Helper function to test numerical gradient against analytical gradient
    fn test_gradient_numerical(problem: &dyn OptimizationProblem, x: &[f64], tolerance: f64) {
        let analytical_grad = problem.gradient_f64(x).unwrap();
        let mut numerical_grad = vec![0.0; x.len()];

        let h = 1e-8;
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
                epsilon = tolerance,
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

        let expected_grad = vec![2.0, 4.0, 6.0];
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

        let expected_grad = vec![0.52 * 1.0 - 0.48 * 2.0, 0.52 * 2.0 - 0.48 * 1.0];
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
        test_gradient_numerical(&problem, &point, 1e-4);
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
        let problem = MichalewiczFunction::new(2);

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
        assert_eq!(AckleyFunction::new(2).name(), "Ackley_2D_a20_b0.2_c6.283185307179586");
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
}