use crate::utils::math::*;
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
    
    /// Get the known optimal value (if available)
    fn optimal_value(&self) -> Option<f64>;
    
    /// Get the convergence tolerance for this problem
    fn convergence_tolerance(&self) -> f64;
    
    /// Get bounds for the problem (if any)
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        None
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
    fn name(&self) -> &str {
        &self.name
    }
    
    fn dimension(&self) -> usize {
        self.dimension
    }
    
    fn initial_point(&self) -> Vec<f64> {
        // Start at (-1.2, 1.0, -1.2, 1.0, ...) for classic initialization
        (0..self.dimension)
            .map(|i| if i % 2 == 0 { -1.2 } else { 1.0 })
            .collect()
    }
    
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        
        let mut sum = 0.0;
        for i in 0..self.dimension - 1 {
            let term1 = x[i + 1] - x[i] * x[i];
            let term2 = 1.0 - x[i];
            sum += 100.0 * term1 * term1 + term2 * term2;
        }
        
        Ok(sum)
    }
    
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        
        let mut grad = vec![0.0; self.dimension];
        
        // First component
        if self.dimension > 1 {
            grad[0] = -400.0 * x[0] * (x[1] - x[0] * x[0]) - 2.0 * (1.0 - x[0]);
        }
        
        // Middle components
        for i in 1..self.dimension - 1 {
            grad[i] = 200.0 * (x[i] - x[i - 1] * x[i - 1])
                - 400.0 * x[i] * (x[i + 1] - x[i] * x[i])
                - 2.0 * (1.0 - x[i]);
        }
        
        // Last component
        if self.dimension > 1 {
            let last = self.dimension - 1;
            grad[last] = 200.0 * (x[last] - x[last - 1] * x[last - 1]);
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
        // Rosenbrock is typically unbounded, but we can set reasonable bounds
        let lower = vec![-5.0; self.dimension];
        let upper = vec![5.0; self.dimension];
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
        Self::with_parameter(dimension, 10.0)
    }
    
    pub fn with_parameter(dimension: usize, a: f64) -> Self {
        Self {
            dimension,
            a,
            name: format!("Rastrigin_{}D_A{}", dimension, a),
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
        // Start at random point in [-5, 5]
        (0..self.dimension)
            .map(|i| 2.0 * (i as f64 / self.dimension as f64) - 1.0)
            .collect()
    }
    
    fn evaluate(&self, x: &[f64]) -> Result<f64> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        
        let sum: f64 = x.iter()
            .map(|&xi| xi * xi - self.a * (2.0 * PI * xi).cos())
            .sum();
        
        Ok(self.a * self.dimension as f64 + sum)
    }
    
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != self.dimension {
            return Err(anyhow::anyhow!("Input dimension mismatch"));
        }
        
        let grad: Vec<f64> = x.iter()
            .map(|&xi| 2.0 * xi + 2.0 * PI * self.a * (2.0 * PI * xi).sin())
            .collect();
        
        Ok(grad)
    }
    
    fn optimal_value(&self) -> Option<f64> {
        Some(0.0)
    }
    
    fn convergence_tolerance(&self) -> f64 {
        1e-4 // Rastrigin is highly multimodal, so we use a looser tolerance
    }
    
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-5.12; self.dimension];
        let upper = vec![5.12; self.dimension];
        Some((lower, upper))
    }
}

/// Sphere function: f(x) = Σ x_i²
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
        // Start at (1, 1, ..., 1)
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
        let lower = vec![-10.0; self.dimension];
        let upper = vec![10.0; self.dimension];
        Some((lower, upper))
    }
}

/// Beale function: f(x,y) = (1.5 - x + xy)² + (2.25 - x + xy²)² + (2.625 - x + xy³)²
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

impl Default for BealeFunction {
    fn default() -> Self {
        Self::new()
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
        
        let term1 = 1.5 - x1 + x1 * x2;
        let term2 = 2.25 - x1 + x1 * x2 * x2;
        let term3 = 2.625 - x1 + x1 * x2 * x2 * x2;
        
        Ok(term1 * term1 + term2 * term2 + term3 * term3)
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
        
        // Partial derivative with respect to x1
        let dx1 = 2.0 * term1 * (-1.0 + x2)
                + 2.0 * term2 * (-1.0 + x2 * x2)
                + 2.0 * term3 * (-1.0 + x2 * x2 * x2);
        
        // Partial derivative with respect to x2
        let dx2 = 2.0 * term1 * x1
                + 2.0 * term2 * x1 * 2.0 * x2
                + 2.0 * term3 * x1 * 3.0 * x2 * x2;
        
        Ok(vec![dx1, dx2])
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

/// Booth function: f(x,y) = (x + 2y - 7)² + (2x + y - 5)²
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

impl Default for BoothFunction {
    fn default() -> Self {
        Self::new()
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
        
        let term1 = x1 + 2.0 * x2 - 7.0;
        let term2 = 2.0 * x1 + x2 - 5.0;
        
        Ok(term1 * term1 + term2 * term2)
    }
    
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Booth function requires 2D input"));
        }
        
        let x1 = x[0];
        let x2 = x[1];
        
        let term1 = x1 + 2.0 * x2 - 7.0;
        let term2 = 2.0 * x1 + x2 - 5.0;
        
        let dx1 = 2.0 * term1 + 2.0 * term2 * 2.0;
        let dx2 = 2.0 * term1 * 2.0 + 2.0 * term2;
        
        Ok(vec![dx1, dx2])
    }
    
    fn optimal_value(&self) -> Option<f64> {
        Some(0.0)
    }
    
    fn convergence_tolerance(&self) -> f64 {
        1e-8
    }
    
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        let lower = vec![-10.0, -10.0];
        let upper = vec![10.0, 10.0];
        Some((lower, upper))
    }
}

/// Himmelblau function: f(x,y) = (x² + y - 11)² + (x + y² - 7)²
/// Has four global minima, all with value 0
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

impl Default for HimmelblauFunction {
    fn default() -> Self {
        Self::new()
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
        
        let term1 = x1 * x1 + x2 - 11.0;
        let term2 = x1 + x2 * x2 - 7.0;
        
        Ok(term1 * term1 + term2 * term2)
    }
    
    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
        if x.len() != 2 {
            return Err(anyhow::anyhow!("Himmelblau function requires 2D input"));
        }
        
        let x1 = x[0];
        let x2 = x[1];
        
        let term1 = x1 * x1 + x2 - 11.0;
        let term2 = x1 + x2 * x2 - 7.0;
        
        let dx1 = 2.0 * term1 * 2.0 * x1 + 2.0 * term2;
        let dx2 = 2.0 * term1 + 2.0 * term2 * 2.0 * x2;
        
        Ok(vec![dx1, dx2])
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_rosenbrock_2d() {
        let problem = RosenbrockFunction::new(2);
        
        // Test evaluation at optimum
        let optimum = vec![1.0, 1.0];
        let value = problem.evaluate(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-10);
        
        // Test gradient at optimum
        let grad = problem.gradient(&optimum).unwrap();
        assert_relative_eq!(grad[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(grad[1], 0.0, epsilon = 1e-10);
        
        // Test evaluation at initial point
        let initial = problem.initial_point();
        let initial_value = problem.evaluate(&initial).unwrap();
        assert!(initial_value > 0.0);
    }

    #[test]
    fn test_sphere_function() {
        let problem = SphereFunction::new(3);
        
        // Test evaluation at optimum
        let optimum = vec![0.0, 0.0, 0.0];
        let value = problem.evaluate(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-10);
        
        // Test gradient at optimum
        let grad = problem.gradient(&optimum).unwrap();
        for &g in &grad {
            assert_relative_eq!(g, 0.0, epsilon = 1e-10);
        }
        
        // Test evaluation at (1, 1, 1)
        let point = vec![1.0, 1.0, 1.0];
        let value = problem.evaluate(&point).unwrap();
        assert_relative_eq!(value, 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_beale_function() {
        let problem = BealeFunction::new();
        
        // Test evaluation at optimum
        let optimum = vec![3.0, 0.5];
        let value = problem.evaluate(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-10);
        
        // Test gradient at optimum
        let grad = problem.gradient(&optimum).unwrap();
        assert_relative_eq!(grad[0], 0.0, epsilon = 1e-8);
        assert_relative_eq!(grad[1], 0.0, epsilon = 1e-8);
    }

    #[test]
    fn test_rastrigin_function() {
        let problem = RastriginFunction::new(2);
        
        // Test evaluation at optimum
        let optimum = vec![0.0, 0.0];
        let value = problem.evaluate(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-10);
        
        // Test gradient at optimum
        let grad = problem.gradient(&optimum).unwrap();
        assert_relative_eq!(grad[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(grad[1], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_booth_function() {
        let problem = BoothFunction::new();
        
        // Test evaluation at optimum
        let optimum = vec![1.0, 3.0];
        let value = problem.evaluate(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-10);
        
        // Test gradient at optimum
        let grad = problem.gradient(&optimum).unwrap();
        assert_relative_eq!(grad[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(grad[1], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_himmelblau_function() {
        let problem = HimmelblauFunction::new();
        
        // Test evaluation at one of the optima
        let optimum = vec![3.0, 2.0];
        let value = problem.evaluate(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-10);
        
        // Test gradient at optimum
        let grad = problem.gradient(&optimum).unwrap();
        assert_relative_eq!(grad[0], 0.0, epsilon = 1e-8);
        assert_relative_eq!(grad[1], 0.0, epsilon = 1e-8);
    }

    #[test]
    fn test_dimension_mismatch() {
        let problem = RosenbrockFunction::new(3);
        
        // Test with wrong dimension
        let wrong_dim = vec![1.0, 2.0]; // 2D instead of 3D
        assert!(problem.evaluate(&wrong_dim).is_err());
        assert!(problem.gradient(&wrong_dim).is_err());
    }
}