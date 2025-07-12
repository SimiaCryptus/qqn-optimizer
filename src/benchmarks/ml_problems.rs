//! Machine learning optimization problems for benchmarking.
 use crate::benchmarks::functions::OptimizationProblem;
 use crate::core::OptResult;

/// Logistic regression optimization problem
pub struct LogisticRegression {
    x_data: Vec<Vec<f64>>,
    y_data: Vec<f64>,
    regularization: f64,
}

impl LogisticRegression {
    pub fn new(x_data: Vec<Vec<f64>>, y_data: Vec<f64>, regularization: f64) -> Self {
        Self { x_data, y_data, regularization }
    }
    
    pub fn synthetic(n_samples: usize, n_features: usize) -> OptResult<Self> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let mut x_data = Vec::new();
        let mut y_data = Vec::new();
        
        for _ in 0..n_samples {
            let x: Vec<f64> = (0..n_features).map(|_| rng.gen_range(-1.0..1.0)).collect();
            let linear_combination: f64 = x.iter().enumerate().map(|(i, &xi)| xi * (i as f64 + 1.0)).sum();
            let y = if linear_combination > 0.0 { 1.0 } else { 0.0 };
            
            x_data.push(x);
            y_data.push(y);
        }
        
        Ok(Self::new(x_data, y_data, 0.01))
    }
}

impl OptimizationProblem for LogisticRegression {
    fn name(&self) -> &str {
        "Logistic Regression"
    }
    fn optimal_value(&self) -> Option<f64> {
        None // Logistic regression doesn't have a known global optimum
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-6
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        None // No bounds by default
    }


    fn evaluate(&self, weights: &[f64]) -> OptResult<f64> {
        let mut loss = 0.0;
        
        for (x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let linear: f64 = x.iter().zip(weights.iter()).map(|(xi, wi)| xi * wi).sum();
            let prob = 1.0 / (1.0 + (-linear).exp());
            loss -= y * prob.ln() + (1.0 - y) * (1.0 - prob).ln();
        }
        
        // Add L2 regularization
        let reg_term: f64 = weights.iter().map(|w| w * w).sum();
        loss += 0.5 * self.regularization * reg_term;
        
        Ok(loss / self.x_data.len() as f64)
    }
    
    fn gradient(&self, weights: &[f64]) -> OptResult<Vec<f64>> {
        let mut grad = vec![0.0; weights.len()];
        
        for (x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let linear: f64 = x.iter().zip(weights.iter()).map(|(xi, wi)| xi * wi).sum();
            let prob = 1.0 / (1.0 + (-linear).exp());
            let error = prob - y;
            
            for (i, &xi) in x.iter().enumerate() {
                grad[i] += error * xi;
            }
        }
        
        // Add regularization gradient
        for (i, &wi) in weights.iter().enumerate() {
            grad[i] += self.regularization * wi;
        }
        
        // Normalize by number of samples
        for gi in grad.iter_mut() {
            *gi /= self.x_data.len() as f64;
        }
        
        Ok(grad)
    }
    
    fn dimension(&self) -> usize {
        self.x_data.first().map(|x| x.len()).unwrap_or(0)
    }
    
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0; self.dimension()]
    }
}

/// Neural network training problem (simplified MLP)
pub struct NeuralNetworkTraining {
    layer_sizes: Vec<usize>,
    x_data: Vec<Vec<f64>>,
    y_data: Vec<Vec<f64>>,
}

impl NeuralNetworkTraining {
    pub fn new(layer_sizes: Vec<usize>, x_data: Vec<Vec<f64>>, y_data: Vec<Vec<f64>>) -> Self {
        Self { layer_sizes, x_data, y_data }
    }
    
    pub fn mlp_classification(layer_sizes: Vec<usize>) -> OptResult<Self> {
        // Generate synthetic classification data
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let n_samples = 100;
        let input_size = layer_sizes[0];
        let output_size = *layer_sizes.last().unwrap();
        
        let mut x_data = Vec::new();
        let mut y_data = Vec::new();
        
        for _ in 0..n_samples {
            let x: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
            let mut y = vec![0.0; output_size];
            let class = rng.gen_range(0..output_size);
            y[class] = 1.0;
            
            x_data.push(x);
            y_data.push(y);
        }
        
        Ok(Self::new(layer_sizes, x_data, y_data))
    }
    
    fn count_parameters(&self) -> usize {
        let mut count = 0;
        for i in 0..self.layer_sizes.len() - 1 {
            count += self.layer_sizes[i] * self.layer_sizes[i + 1]; // weights
            count += self.layer_sizes[i + 1]; // biases
        }
        count
    }
}

impl OptimizationProblem for NeuralNetworkTraining {
    fn name(&self) -> &str {
        "Neural Network Training"
    }
    fn optimal_value(&self) -> Option<f64> {
        None // Neural network training doesn't have a known global optimum
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-5
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        None // No bounds by default for neural network weights
    }
    
    fn evaluate(&self, params: &[f64]) -> OptResult<f64> {
        // Simplified forward pass and loss computation
        // In practice, this would implement full neural network forward pass
        let mut loss = 0.0;
        
        for (x, y_true) in self.x_data.iter().zip(self.y_data.iter()) {
            // Simplified: just compute MSE with linear transformation
            let output_size = y_true.len();
            let input_size = x.len();
            
            if params.len() >= input_size * output_size {
                let mut y_pred = vec![0.0; output_size];
                for i in 0..output_size {
                    for j in 0..input_size {
                        let weight_idx = i * input_size + j;
                        if weight_idx < params.len() {
                            y_pred[i] += x[j] * params[weight_idx];
                        }
                    }
                }
                
                // MSE loss
                for (pred, &true_val) in y_pred.iter().zip(y_true.iter()) {
                    let diff = pred - true_val;
                    loss += diff * diff;
                }
            }
        }
        
        Ok(loss / self.x_data.len() as f64)
    }
    
    fn gradient(&self, params: &[f64]) -> OptResult<Vec<f64>> {
        let mut grad = vec![0.0; params.len()];
        
        // Simplified gradient computation
        for (x, y_true) in self.x_data.iter().zip(self.y_data.iter()) {
            let output_size = y_true.len();
            let input_size = x.len();
            
            if params.len() >= input_size * output_size {
                let mut y_pred = vec![0.0; output_size];
                for i in 0..output_size {
                    for j in 0..input_size {
                        let weight_idx = i * input_size + j;
                        if weight_idx < params.len() {
                            y_pred[i] += x[j] * params[weight_idx];
                        }
                    }
                }
                
                // Gradient computation
                for i in 0..output_size {
                    let error = y_pred[i] - y_true[i];
                    for j in 0..input_size {
                        let weight_idx = i * input_size + j;
                        if weight_idx < grad.len() {
                            grad[weight_idx] += 2.0 * error * x[j];
                        }
                    }
                }
            }
        }
        
        // Normalize by number of samples
        for gi in grad.iter_mut() {
            *gi /= self.x_data.len() as f64;
        }
        
        Ok(grad)
    }
    
    fn dimension(&self) -> usize {
        self.count_parameters()
    }
    
    fn initial_point(&self) -> Vec<f64> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..self.dimension()).map(|_| rng.gen_range(-0.1..0.1)).collect()
    }
}

/// Linear regression optimization problem
pub struct LinearRegression {
    x_data: Vec<Vec<f64>>,
    y_data: Vec<f64>,
    regularization: f64,
}

impl LinearRegression {
    pub fn new(x_data: Vec<Vec<f64>>, y_data: Vec<f64>, regularization: f64) -> Self {
        Self { x_data, y_data, regularization }
    }
}

impl OptimizationProblem for LinearRegression {
    fn name(&self) -> &str {
        "Linear Regression"
    }
    fn optimal_value(&self) -> Option<f64> {
        None // Linear regression doesn't have a known global optimum
    }
    fn convergence_tolerance(&self) -> f64 {
        1e-6
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        None // No bounds by default
    }
    
    fn evaluate(&self, weights: &[f64]) -> OptResult<f64> {
        let mut loss = 0.0;
        
        for (x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let pred: f64 = x.iter().zip(weights.iter()).map(|(xi, wi)| xi * wi).sum();
            let error = pred - y;
            loss += error * error;
        }
        
        // Add L2 regularization
        let reg_term: f64 = weights.iter().map(|w| w * w).sum();
        loss += 0.5 * self.regularization * reg_term;
        
        Ok(loss / self.x_data.len() as f64)
    }
    
    fn gradient(&self, weights: &[f64]) -> OptResult<Vec<f64>> {
        let mut grad = vec![0.0; weights.len()];
        
        for (x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let pred: f64 = x.iter().zip(weights.iter()).map(|(xi, wi)| xi * wi).sum();
            let error = pred - y;
            
            for (i, &xi) in x.iter().enumerate() {
                grad[i] += 2.0 * error * xi;
            }
        }
        
        // Add regularization gradient
        for (i, &wi) in weights.iter().enumerate() {
            grad[i] += self.regularization * wi;
        }
        
        // Normalize by number of samples
        for gi in grad.iter_mut() {
            *gi /= self.x_data.len() as f64;
        }
        
        Ok(grad)
    }
    
    fn dimension(&self) -> usize {
        self.x_data.first().map(|x| x.len()).unwrap_or(0)
    }
    
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0; self.dimension()]
    }
}

/// Support Vector Machine optimization problem (simplified)
pub struct SupportVectorMachine {
    x_data: Vec<Vec<f64>>,
    y_data: Vec<f64>,
    c: f64, // Regularization parameter
}

impl SupportVectorMachine {
    pub fn new(x_data: Vec<Vec<f64>>, y_data: Vec<f64>, c: f64) -> Self {
        Self { x_data, y_data, c }
    }
}

impl OptimizationProblem for SupportVectorMachine {
    fn evaluate(&self, weights: &[f64]) -> OptResult<f64> {
        let mut loss = 0.0;
        
        // Hinge loss
        for (x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let score: f64 = x.iter().zip(weights.iter()).map(|(xi, wi)| xi * wi).sum();
            let margin = y * score;
            if margin < 1.0 {
                loss += 1.0 - margin;
            }
        }
        
        // Regularization term
        let reg_term: f64 = weights.iter().map(|w| w * w).sum();
        
        Ok(self.c * loss / self.x_data.len() as f64 + 0.5 * reg_term)
    }
    
    fn gradient(&self, weights: &[f64]) -> OptResult<Vec<f64>> {
        let mut grad = vec![0.0; weights.len()];
        
        for (x, &y) in self.x_data.iter().zip(self.y_data.iter()) {
            let score: f64 = x.iter().zip(weights.iter()).map(|(xi, wi)| xi * wi).sum();
            let margin = y * score;
            
            if margin < 1.0 {
                for (i, &xi) in x.iter().enumerate() {
                    grad[i] -= self.c * y * xi / self.x_data.len() as f64;
                }
            }
        }
        
        // Add regularization gradient
        for (i, &wi) in weights.iter().enumerate() {
            grad[i] += wi;
        }
        
        Ok(grad)
    }
    
    fn dimension(&self) -> usize {
        self.x_data.first().map(|x| x.len()).unwrap_or(0)
    }
    
    fn initial_point(&self) -> Vec<f64> {
        vec![0.0; self.dimension()]
    }
}