use anyhow::Result;
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
    fn evaluate(&self, x: &[f64]) -> Result<f64>;
    /// Compute the gradient at point x
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>>;
    /// Get the optimal value if known
    fn optimal_value(&self) -> Option<f64>;
    /// Get convergence tolerance
    fn convergence_tolerance(&self) -> f64;
    /// Get problem bounds if any
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)>;
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
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![-1.2; self.dimension]
    }
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
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
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
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
        Some(0.0)
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-6
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-5.0; self.dimension];
        let upper = vec![10.0; self.dimension];
        Some((lower, upper))
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
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![2.0; self.dimension]
    }
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
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
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
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
        Some(0.0)
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-4
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-5.12; self.dimension];
        let upper = vec![5.12; self.dimension];
        Some((lower, upper))
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
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let sum: f64 = x.iter().map(|&xi| xi * xi).sum();
        Ok(sum)
    }
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let grad: Vec<f64> = x.iter().map(|&xi| 2.0 * xi).collect();
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(0.0)
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-8
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-100.0; self.dimension];
        let upper = vec![100.0; self.dimension];
        Some((lower, upper))
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
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0, 1.0]
    }
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
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
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
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
        Some(0.0)
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-6
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-4.5, -4.5];
        let upper = vec![4.5, 4.5];
        Some((lower, upper))
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
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0, 0.0]
    }
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Himmelblau function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (x1 * x1 + x2 - 11.0).powi(2);
        let term2 = (x1 + x2 * x2 - 7.0).powi(2);
        Ok(term1 + term2)
    }
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
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
        Some(0.0)
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-6
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-5.0, -5.0];
        let upper = vec![5.0, 5.0];
        Some((lower, upper))
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
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        2
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0, 0.0]
    }
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Booth function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (x1 + 2.0 * x2 - 7.0).powi(2);
        let term2 = (2.0 * x1 + x2 - 5.0).powi(2);
        Ok(term1 + term2)
    }
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
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
        Some(0.0)
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-6
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-10.0, -10.0];
        let upper = vec![10.0, 10.0];
        Some((lower, upper))
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
            name: format!("Ackley_{}D", dimension),
        }
    }
}
impl OptimizationProblem for AckleyFunction {
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        self.dimension
    }
    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimension]
    }
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
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
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
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
        Some(0.0)
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-4
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-32.768; self.dimension];
        let upper = vec![32.768; self.dimension];
        Some((lower, upper))
    }
}
