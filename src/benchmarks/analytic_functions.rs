use std::f64::consts::PI;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use crate::OptimizationProblem;

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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Matyas function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        Ok(0.26 * (x1 * x1 + x2 * x2) - 0.48 * x1 * x2)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Matyas function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        Ok(vec![0.52 * x1 - 0.48 * x2, 0.52 * x2 - 0.48 * x1])
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let sum: f64 = x
            .iter()
            .map(|&xi| xi.powi(4) - 16.0 * xi.powi(2) + 5.0 * xi)
            .sum();
        Ok(0.5 * sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
        match self.dimension {
            2 => Some(8.45e-3),  // Already set in problem_sets.rs
            5 => Some(3.98e-1),  // Already set in problem_sets.rs
            10 => Some(9.70e0),  // Already set in problem_sets.rs
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
            2 => Some(7.96e0),   // Already set in problem_sets.rs
            5 => Some(2.04e1),   // Already set in problem_sets.rs
            10 => Some(4.18e1),  // Already set in problem_sets.rs
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let sum: f64 = x.iter().map(|&xi| xi * xi).sum();
        Ok(sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let grad: Vec<f64> = x.iter().map(|&xi| 2.0 * xi).collect();
        Ok(grad)
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Himmelblau function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (x1 * x1 + x2 - 11.0).powi(2);
        let term2 = (x1 + x2 * x2 - 7.0).powi(2);
        Ok(term1 + term2)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Booth function requires 2D input"));
        }
        let x1 = x[0];
        let x2 = x[1];
        let term1 = (x1 + 2.0 * x2 - 7.0).powi(2);
        let term2 = (2.0 * x1 + x2 - 5.0).powi(2);
        Ok(term1 + term2)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
            name: format!("Ackley_{}D_a{}_b{}_c{:0.2e}", dimension, a, b, c),
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
        match self.dimension {
            2 => Some(3.57e0),   // Already set in problem_sets.rs
            5 => Some(3.57e0),   // Already set in problem_sets.rs
            10 => Some(3.57e0),  // Already set in problem_sets.rs
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

    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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

    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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

    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }

        let sum: f64 = x.iter().map(|&xi| xi * (xi.abs().sqrt()).sin()).sum();

        Ok(418.9829 * self.dimension as f64 - sum)
    }

    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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

    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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

    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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

    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
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

    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
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
            name: format!("IllConditionedRosenbrock_{}D_alpha{}", dimension, alpha),
        }
    }
}
impl OptimizationProblem for IllConditionedRosenbrock {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let mut sum = 0.0;
        for i in 0..self.dimension - 1 {
            let term1 = self.alpha * (x[i + 1] - x[i] * x[i]).powi(2);
            let term2 = (1.0 - x[i]).powi(2);
            sum += term1 + term2;
        }
        Ok(sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let mut grad = vec![0.0; self.dimension];
        for i in 0..self.dimension - 1 {
            grad[i] += -4.0 * self.alpha * x[i] * (x[i + 1] - x[i] * x[i]) - 2.0 * (1.0 - x[i]);
            grad[i + 1] += 2.0 * self.alpha * (x[i + 1] - x[i] * x[i]);
        }
        Ok(grad)
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
            name: format!("Trigonometric_{}D", dimension),
        }
    }
}
impl OptimizationProblem for TrigonometricFunction {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let n = self.dimension as f64;
        let cos_sum: f64 = x.iter().map(|&xi| xi.cos()).sum();
        let mut total = 0.0;
        for i in 0..self.dimension {
            let term = n - cos_sum + (i + 1) as f64 * (1.0 - x[i].cos() - x[i].sin());
            total += term * term;
        }
        Ok(total)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let n = self.dimension as f64;
        let cos_sum: f64 = x.iter().map(|&xi| xi.cos()).sum();
        let mut grad = vec![0.0; self.dimension];
        for j in 0..self.dimension {
            for i in 0..self.dimension {
                let term = n - cos_sum + (i + 1) as f64 * (1.0 - x[i].cos() - x[i].sin());
                if i == j {
                    let deriv = x[j].sin() + (i + 1) as f64 * (x[i].sin() - x[i].cos());
                    grad[j] += 2.0 * term * deriv;
                } else {
                    grad[j] += 2.0 * term * x[j].sin();
                }
            }
        }
        Ok(grad)
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
            name: format!("PenaltyI_{}D_alpha{:.0e}", dimension, alpha),
        }
    }
}
impl OptimizationProblem for PenaltyFunctionI {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let objective: f64 = x.iter().map(|&xi| (xi - 1.0).powi(2)).sum();
        let penalty: f64 = x.iter()
            .map(|&xi| self.alpha * (xi - 0.25).max(0.0).powi(2))
            .sum();
        Ok(objective + penalty)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let grad: Vec<f64> = x.iter()
            .map(|&xi| {
                let obj_grad = 2.0 * (xi - 1.0);
                let penalty_grad = if xi > 0.25 {
                    2.0 * self.alpha * (xi - 0.25)
                } else {
                    0.0
                };
                obj_grad + penalty_grad
            })
            .collect();
        Ok(grad)
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
            name: format!("Barrier_{}D_mu{}", dimension, mu),
        }
    }
}
impl OptimizationProblem for BarrierFunction {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        // Check feasibility
        if x.iter().any(|&xi| xi <= 0.0) {
            return Err(anyhow::anyhow!("Barrier function requires x > 0"));
        }
        let objective: f64 = x.iter().map(|&xi| xi * xi).sum();
        let x1: Vec<f64> = x.iter().map(|&xi| xi.ln()).collect();
        let barrier: f64 = -self.mu * x1.iter().sum::<f64>();
        Ok(objective + barrier)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        if x.iter().any(|&xi| xi <= 0.0) {
            return Err(anyhow::anyhow!("Barrier function requires x > 0"));
        }
        let grad: Vec<f64> = x.iter()
            .map(|&xi| 2.0 * xi - self.mu / xi)
            .collect();
        Ok(grad)
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
            name: format!("NoisySphere_{}D_sigma{}", dimension, noise_level),
        }
    }
}
impl OptimizationProblem for NoisySphere {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let sphere_value: f64 = x.iter().map(|&xi| xi * xi).sum();
        // Generate deterministic noise based on x coordinates
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        for &xi in x {
            xi.to_bits().hash(&mut hasher);
        }
        self.seed.hash(&mut hasher);
        let hash = hasher.finish();
        let mut rng = ChaCha8Rng::seed_from_u64(hash);
        let noise: f64 = rng.random::<f64>() * 2.0 - 1.0; // [-1, 1]
        Ok(sphere_value + self.noise_level * noise)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        // Use finite differences for noisy gradient
        let h = 1e-6;
        let mut grad = vec![0.0; self.dimension];
        for i in 0..self.dimension {
            let mut x_plus = x.to_vec();
            let mut x_minus = x.to_vec();
            x_plus[i] += h;
            x_minus[i] -= h;
            let f_plus = self.evaluate_f64(&x_plus)?;
            let f_minus = self.evaluate_f64(&x_minus)?;
            grad[i] = (f_plus - f_minus) / (2.0 * h);
        }
        Ok(grad)
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
            name: format!("SparseRosenbrock_{}D", dimension),
        }
    }
}
impl OptimizationProblem for SparseRosenbrock {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let mut sum = 0.0;
        for i in (0..self.dimension).step_by(2) {
            let term1 = 100.0 * (x[i + 1] - x[i] * x[i]).powi(2);
            let term2 = (1.0 - x[i]).powi(2);
            sum += term1 + term2;
        }
        Ok(sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let mut grad = vec![0.0; self.dimension];
        for i in (0..self.dimension).step_by(2) {
            grad[i] = -400.0 * x[i] * (x[i + 1] - x[i] * x[i]) - 2.0 * (1.0 - x[i]);
            grad[i + 1] = 200.0 * (x[i + 1] - x[i] * x[i]);
        }
        Ok(grad)
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
        // Default sparsity: interact with neighbors at distance 1 and 3
        Self::with_pattern(dimension, vec![1, 3])
    }
    pub fn with_pattern(dimension: usize, sparsity_pattern: Vec<usize>) -> Self {
        Self {
            dimension,
            sparsity_pattern: sparsity_pattern.clone(),
            name: format!("SparseQuadratic_{}D_pattern{:?}", dimension, sparsity_pattern),
        }
    }
}
impl OptimizationProblem for SparseQuadratic {
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
    fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        // Diagonal terms
        let mut sum: f64 = x.iter().map(|&xi| xi * xi).sum();
        // Sparse off-diagonal terms
        for i in 0..self.dimension {
            for &k in &self.sparsity_pattern {
                if i + k < self.dimension {
                    sum += 0.1 * x[i] * x[i + k];
                }
            }
        }
        Ok(sum)
    }
    fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        let mut grad = vec![0.0; self.dimension];
        // Diagonal terms
        for i in 0..self.dimension {
            grad[i] = 2.0 * x[i];
        }
        // Sparse off-diagonal terms
        for i in 0..self.dimension {
            for &k in &self.sparsity_pattern {
                if i + k < self.dimension {
                    grad[i] += 0.1 * x[i + k];
                    grad[i + k] += 0.1 * x[i];
                }
            }
        }
        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(1e-6)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::benchmarks::analytic_functions::{AckleyFunction, BealeFunction, BoothFunction, GoldsteinPriceFunction, GriewankFunction, HimmelblauFunction, LeviFunction, LevyFunction, MatyasFunction, MichalewiczFunction, RastriginFunction, RosenbrockFunction, SchwefelFunction, SphereFunction, StyblinskiTangFunction, ZakharovFunction, IllConditionedRosenbrock, TrigonometricFunction, PenaltyFunctionI, BarrierFunction, NoisySphere, SparseRosenbrock, SparseQuadratic};

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
        test_gradient_numerical(&problem, &point, 1e-5);
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
        // Variables 0,1 should not affect gradients of 2,3
        assert_ne!(grad[0], 0.0);
        assert_ne!(grad[1], 0.0);
        assert_eq!(grad[2], 0.0);
        assert_eq!(grad[3], 0.0);
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
        let grad = custom.gradient_f64(&vec![1.0, 0.0, 0.0, 0.0]).unwrap();
        // Only x[0] and x[2] should interact
        assert_ne!(grad[0], 2.0); // Not just diagonal
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
}