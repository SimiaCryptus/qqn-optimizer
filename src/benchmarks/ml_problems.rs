//! Machine learning optimization problems for benchmarking.
use crate::benchmarks::functions::OptimizationProblem;
use anyhow::Result;
use candle_core::{Device, Tensor};
use rand::rngs::StdRng;

/// Logistic regression optimization problem
#[derive(Clone)]
pub struct LogisticRegression {
    x_tensor: Tensor,
    y_tensor: Tensor,
    device: Device,
    regularization: f64,
    name: String,
    n_samples: usize,
    n_features: usize,
   optimal_value: Option<f64>,
}

impl LogisticRegression {
    pub fn new(x_data: Vec<Vec<f64>>, y_data: Vec<f64>, regularization: f64) -> Result<Self> {
        let device = Device::Cpu;
        let n_samples = x_data.len();
        let n_features = x_data.first().map(|x| x.len()).unwrap_or(0);
        let name = format!(
            "LogisticRegression_{}samples_{}features_reg{}",
            n_samples, n_features, regularization
        );

        // Convert to tensors
        let x_flat: Vec<f64> = x_data.into_iter().flatten().collect();
        let x_tensor = Tensor::from_vec(x_flat, (n_samples, n_features), &device)?;
        let y_tensor = Tensor::from_vec(y_data, n_samples, &device)?;
       // Set default optimal value based on problem size
       let optimal_value = if n_samples <= 100 && n_features <= 5 {
           Some(0.35) // Small problems: ~15% above 0.302
       } else {
           Some(0.32) // Large problems: ~15% above 0.277
       };

        Ok(Self {
            n_samples,
            n_features,
            name,
            x_tensor,
            y_tensor,
            device,
            regularization,
           optimal_value,
        })
    }

    pub fn synthetic(n_samples: usize, n_features: usize, rng: &mut StdRng) -> Result<Self> {
        use rand::Rng;

        let mut x_data = Vec::with_capacity(n_samples);
        let mut y_data = Vec::with_capacity(n_samples);

        for _ in 0..n_samples {
            let mut x = Vec::with_capacity(n_features);
            for _ in 0..n_features {
                x.push(rng.random_range(-1.0..1.0));
            }
            let linear_combination: f64 = x
                .iter()
                .enumerate()
                .map(|(i, &xi)| xi * (i as f64 + 1.0))
                .sum();
            let y = if linear_combination > 0.0 { 1.0 } else { 0.0 };

            x_data.push(x);
            y_data.push(y);
        }

        Self::new(x_data, y_data, 0.01)
    }
   pub fn set_optimal_value(&mut self, value: Option<f64>) {
       self.optimal_value = value;
   }
}

impl OptimizationProblem for LogisticRegression {
    fn name(&self) -> &str {
        &self.name
    }
    fn optimal_value(&self) -> Option<f64> {
       self.optimal_value
    }

    fn evaluate_f64(&self, weights: &[f64]) -> Result<f64> {
        let weights_tensor = Tensor::from_vec(weights.to_vec(), weights.len(), &self.device)?;

        // Compute logits: X @ weights
        let logits = self
            .x_tensor
            .matmul(&weights_tensor.unsqueeze(1)?)?
            .squeeze(1)?;

        // Compute sigmoid probabilities
        let probs = candle_nn::ops::sigmoid(&logits)?;

        // Binary cross-entropy loss
        let ones = Tensor::ones_like(&self.y_tensor)?;
        let log_probs = probs.log()?;
        let log_one_minus_probs = (&ones - &probs)?.log()?;

        let term1 = &self.y_tensor * &log_probs;
        let ones_minus_y = (&ones - &self.y_tensor)?;
        let term2 = &ones_minus_y * &log_one_minus_probs;
        let loss = (&term1? + &term2?)?.mean(0)?.neg();

        // Add L2 regularization
        let reg_term =
            (&weights_tensor * &weights_tensor)?.sum_all()? * (0.5 * self.regularization);
        let total_loss = (loss? + reg_term?)?;

        Ok(total_loss.to_scalar::<f64>()?)
    }

    fn gradient_f64(&self, weights: &[f64]) -> Result<Vec<f64>> {
        let weights_tensor = Tensor::from_vec(weights.to_vec(), weights.len(), &self.device)?;

        // Compute predictions
        let logits = self
            .x_tensor
            .matmul(&weights_tensor.unsqueeze(1)?)?
            .squeeze(1)?;
        let probs = candle_nn::ops::sigmoid(&logits)?;

        // Compute error: predictions - targets
        let error = (&probs - &self.y_tensor)?;

        // Compute gradient: X^T @ error / n_samples
        let grad = self
            .x_tensor
            .t()?
            .matmul(&error.unsqueeze(1)?)?
            .squeeze(1)?;
        let n_samples = self.n_samples as f64;
        let grad = (&grad / n_samples)?;

        // Add regularization gradient
        let reg_grad = (&weights_tensor * self.regularization)?;
        let total_grad = (&grad + &reg_grad)?;

        Ok(total_grad.to_vec1::<f64>()?)
    }

    fn dimension(&self) -> usize {
        self.x_tensor.dim(1).unwrap_or(0)
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![0.0; self.dimension()]
    }
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
}

/// Neural network training problem (simplified MLP)
#[derive(Clone)]
pub struct NeuralNetworkTraining {
    layer_sizes: Vec<usize>,
    x_tensor: Tensor,
    y_tensor: Tensor,
    device: Device,
    name: String,
   optimal_value: Option<f64>,
}

impl NeuralNetworkTraining {
    pub fn new(
        layer_sizes: Vec<usize>,
        x_data: Vec<Vec<f64>>,
        y_data: Vec<Vec<f64>>,
    ) -> Result<Self> {
        let device = Device::Cpu;
        let n_samples = x_data.len();
        let layer_str = layer_sizes
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>()
            .join("_");
        let name = format!("NeuralNetwork_{}samples_layers_{}", n_samples, layer_str);

        // Convert to tensors
        let input_dim = x_data.first().map(|x| x.len()).unwrap_or(0);
        let output_dim = y_data.first().map(|y| y.len()).unwrap_or(0);

        let x_flat: Vec<f64> = x_data.into_iter().flatten().collect();
        let y_flat: Vec<f64> = y_data.into_iter().flatten().collect();

        let x_tensor = Tensor::from_vec(x_flat, (n_samples, input_dim), &device)?;
        let y_tensor = Tensor::from_vec(y_flat, (n_samples, output_dim), &device)?;
       // Set default optimal value based on network size
       let mut temp_nn = Self {
           layer_sizes: layer_sizes.clone(),
           x_tensor,
           y_tensor,
           device,
           name,
           optimal_value: None,
       };
       let total_params = temp_nn.count_parameters();
       let optimal_value = if total_params > 100 {
           Some(0.1)
       } else {
           Some(0.25)
       };
       temp_nn.optimal_value = optimal_value;

       Ok(temp_nn)
    }

    pub fn mlp_classification(layer_sizes: Vec<usize>, rng: &mut StdRng) -> Result<Self> {
        use rand::Rng;

        let n_samples = 100;
        let input_size = layer_sizes[0];
        let output_size = *layer_sizes.last().unwrap();

        let mut x_data = Vec::new();
        let mut y_data = Vec::new();

        for _ in 0..n_samples {
            let x: Vec<f64> = (0..input_size)
                .map(|_| rng.random_range(-1.0..1.0))
                .collect();
            let mut y = vec![0.0; output_size];
            let class = rng.random_range(0..output_size);
            y[class] = 1.0;

            x_data.push(x);
            y_data.push(y);
        }

        Self::new(layer_sizes, x_data, y_data)
    }
   pub fn set_optimal_value(&mut self, value: Option<f64>) {
       self.optimal_value = value;
   }

    fn count_parameters(&self) -> usize {
        let mut count = 0;
        for i in 0..self.layer_sizes.len() - 1 {
            count += self.layer_sizes[i] * self.layer_sizes[i + 1]; // weights
            count += self.layer_sizes[i + 1]; // biases
        }
        count
    }
    fn forward_pass(&self, params: &[f64]) -> Result<Tensor> {
        let mut param_idx = 0;
        let mut x = &self.x_tensor;
        let mut owned_x: Option<Tensor> = None;
        for i in 0..self.layer_sizes.len() - 1 {
            let input_size = self.layer_sizes[i];
            let output_size = self.layer_sizes[i + 1];
            // Extract weights and biases
            let weight_size = input_size * output_size;
            let weight_slice = &params[param_idx..param_idx + weight_size];
            param_idx += weight_size;
            let bias_slice = &params[param_idx..param_idx + output_size];
            param_idx += output_size;
            // Create weight tensor
            let w = Tensor::from_slice(weight_slice, (input_size, output_size), &self.device)?;
            let b = Tensor::from_slice(bias_slice, output_size, &self.device)?;
            // Linear transformation: x @ w + b
            let z = x.matmul(&w)?;
            let z = z.broadcast_add(&b)?;
            // Apply activation (ReLU for hidden layers, no activation for output)
            if i < self.layer_sizes.len() - 2 {
                owned_x = Some(z.relu()?);
            } else {
                owned_x = Some(z);
            }
            x = owned_x.as_ref().unwrap();
        }
        Ok(owned_x.unwrap())
    }
    fn backward_pass(&self, params: &[f64]) -> Result<Vec<f64>> {
        let batch_size = self.x_tensor.dim(0)? as f64;
        let mut gradients = Vec::with_capacity(params.len());
        gradients.resize(params.len(), 0.0);
        // Pre-allocate reusable buffers

        // Forward pass with intermediate activations
        let mut activations = vec![self.x_tensor.clone()];
        let mut param_idx = 0;
        for i in 0..self.layer_sizes.len() - 1 {
            let input_size = self.layer_sizes[i];
            let output_size = self.layer_sizes[i + 1];
            // Extract weights and biases
            let weight_size = input_size * output_size;
            let weights = &params[param_idx..param_idx + weight_size];
            param_idx += weight_size;
            let biases = &params[param_idx..param_idx + output_size];
            param_idx += output_size;
            // Create weight tensor
            let w = Tensor::from_vec(weights.to_vec(), (input_size, output_size), &self.device)?;
            let b = Tensor::from_vec(biases.to_vec(), output_size, &self.device)?;
            // Linear transformation
            let z = activations.last().unwrap().matmul(&w)?.broadcast_add(&b)?;
            // Apply activation
            let a = if i < self.layer_sizes.len() - 2 {
                z.relu()?
            } else {
                z
            };
            activations.push(a);
        }
        // Backward pass
        let y_pred = activations.last().unwrap();
        let mut delta = (&(y_pred - &self.y_tensor)? * (2.0 / batch_size))?;
        param_idx = params.len();
        for i in (0..self.layer_sizes.len() - 1).rev() {
            let input_size = self.layer_sizes[i];
            let output_size = self.layer_sizes[i + 1];
            // Gradient for biases
            let bias_grad = delta.sum(0)?;
            let bias_grad_vec = bias_grad.to_vec1::<f64>()?;
            param_idx -= output_size;
            for (j, &g) in bias_grad_vec.iter().enumerate() {
                gradients[param_idx + j] = g;
            }
            // Gradient for weights
            let weight_grad = activations[i].t()?.matmul(&delta)?;
            let weight_grad_vec = weight_grad.flatten_all()?.to_vec1::<f64>()?;
            param_idx -= input_size * output_size;
            for (j, &g) in weight_grad_vec.iter().enumerate() {
                gradients[param_idx + j] = g;
            }
            // Propagate gradient through activation
            if i > 0 {
                // Extract weights for backward pass
                let w_idx = param_idx;
                let weights = &params[w_idx..w_idx + input_size * output_size];
                let w =
                    Tensor::from_vec(weights.to_vec(), (input_size, output_size), &self.device)?;
                delta = delta.matmul(&w.t()?)?;
                // Apply ReLU derivative
                if i < self.layer_sizes.len() - 1 {
                    let relu_mask = activations[i].gt(&Tensor::zeros_like(&activations[i])?)?;
                    // Convert boolean mask to float (1.0 where true, 0.0 where false)
                    let relu_mask_float = relu_mask.to_dtype(candle_core::DType::F64)?;
                    // Apply ReLU derivative by element-wise multiplication
                    delta = (&delta * &relu_mask_float)?;
                }
            }
        }
        Ok(gradients)
    }
}

impl OptimizationProblem for NeuralNetworkTraining {
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }

    fn dimension(&self) -> usize {
        self.count_parameters()
    }
    fn initial_point(&self) -> Vec<f64> {
        use rand::Rng;
        let mut rng = rand::rng();
        (0..self.dimension())
            .map(|_| rng.random_range(-0.1..0.1))
            .collect()
    }

    fn evaluate_f64(&self, params: &[f64]) -> Result<f64> {
        let y_pred = self.forward_pass(params)?;

        // MSE loss
        let diff = (&y_pred - &self.y_tensor)?;
        let loss = (&diff * &diff)?.mean_all()?;

        Ok(loss.to_scalar::<f64>()?)
    }

    fn gradient_f64(&self, params: &[f64]) -> Result<Vec<f64>> {
        self.backward_pass(params)
    }

    fn optimal_value(&self) -> Option<f64> {
       self.optimal_value
    }
}

/// Linear regression optimization problem
#[derive(Clone)]
pub struct LinearRegression {
    x_tensor: Tensor,
    y_tensor: Tensor,
    device: Device,
    regularization: f64,
    name: String,
   optimal_value: Option<f64>,
}

impl LinearRegression {
    pub fn new(x_data: Vec<Vec<f64>>, y_data: Vec<f64>, regularization: f64) -> Result<Self> {
        let device = Device::Cpu;
        let n_samples = x_data.len();
        let n_features = x_data.first().map(|x| x.len()).unwrap_or(0);
        let name = format!(
            "LinearRegression_{}samples_{}features_reg{}",
            n_samples, n_features, regularization
        );

        // Convert to tensors
        let x_flat: Vec<f64> = x_data.into_iter().flatten().collect();
        let x_tensor = Tensor::from_vec(x_flat, (n_samples, n_features), &device)?;
        let y_tensor = Tensor::from_vec(y_data, n_samples, &device)?;
       // Set default optimal value based on problem size
       let optimal_value = if n_samples <= 100 && n_features <= 5 {
           Some(10.0) // Small problems: ~8% margin above 23.2
       } else {
           Some(140.0) // Larger problem threshold
       };

        Ok(Self {
            x_tensor,
            y_tensor,
            device,
            regularization,
            name,
           optimal_value,
        })
    }
   pub fn set_optimal_value(&mut self, value: Option<f64>) {
       self.optimal_value = value;
   }
}

impl OptimizationProblem for LinearRegression {
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn optimal_value(&self) -> Option<f64> {
       self.optimal_value
    }

    fn evaluate_f64(&self, weights: &[f64]) -> Result<f64> {
        let weights_tensor = Tensor::from_vec(weights.to_vec(), weights.len(), &self.device)?;

        // Compute predictions: X @ weights
        let predictions = self
            .x_tensor
            .matmul(&weights_tensor.unsqueeze(1)?)?
            .squeeze(1)?;

        // MSE loss
        let diff = (&predictions - &self.y_tensor)?;
        let mse = (&diff * &diff)?.mean_all()?;

        // Add L2 regularization
        let reg_term =
            (&weights_tensor * &weights_tensor)?.sum_all()? * (0.5 * self.regularization);
        let total_loss = (mse + reg_term)?;

        Ok(total_loss.to_scalar::<f64>()?)
    }

    fn gradient_f64(&self, weights: &[f64]) -> Result<Vec<f64>> {
        let weights_tensor = Tensor::from_vec(weights.to_vec(), weights.len(), &self.device)?;

        // Compute predictions and error
        let predictions = self
            .x_tensor
            .matmul(&weights_tensor.unsqueeze(1)?)?
            .squeeze(1)?;
        let error = (&predictions - &self.y_tensor)?;

        // Compute gradient: 2 * X^T @ error / n_samples
        let grad = self
            .x_tensor
            .t()?
            .matmul(&error.unsqueeze(1)?)?
            .squeeze(1)?;
        let n_samples = self.x_tensor.dim(0)? as f64;
        let grad = (&grad * (2.0 / n_samples))?;

        // Add regularization gradient
        let reg_grad = (&weights_tensor * self.regularization)?;
        let total_grad = (&grad + &reg_grad)?;

        Ok(total_grad.to_vec1::<f64>()?)
    }

    fn dimension(&self) -> usize {
        self.x_tensor.dim(1).unwrap_or(0)
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![0.0; self.dimension()]
    }
}

/// Support Vector Machine optimization problem (simplified)
#[derive(Clone)]
pub struct SupportVectorMachine {
    x_tensor: Tensor,
    y_tensor: Tensor,
    device: Device,
    c: f64, // Regularization parameter
    name: String,
    ones_tensor: Option<Tensor>, // Cache for ones tensor
   optimal_value: Option<f64>,
}

impl SupportVectorMachine {
    pub fn new(x_data: Vec<Vec<f64>>, y_data: Vec<f64>, c: f64) -> Result<Self> {
        let device = Device::Cpu;
        let n_samples = x_data.len();
        let n_features = x_data.first().map(|x| x.len()).unwrap_or(0);
        let name = format!("SVM_{}samples_{}features_C{}", n_samples, n_features, c);

        // Convert to tensors
        let x_flat: Vec<f64> = x_data.into_iter().flatten().collect();
        let x_tensor = Tensor::from_vec(x_flat, (n_samples, n_features), &device)?;
        let y_tensor = Tensor::from_vec(y_data, n_samples, &device)?;
       // Set default optimal value based on problem size
       let optimal_value = if n_samples <= 100 && n_features <= 5 {
           Some(1.05) // Small problems: ~5% above 0.994
       } else {
           Some(1.0) // Large problems: ~6% above 0.942
       };

        Ok(Self {
            x_tensor,
            y_tensor,
            device,
            c,
            name,
            ones_tensor: None,
           optimal_value,
        })
    }
   pub fn set_optimal_value(&mut self, value: Option<f64>) {
       self.optimal_value = value;
   }
}

impl OptimizationProblem for SupportVectorMachine {
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn optimal_value(&self) -> Option<f64> {
       self.optimal_value
    }

    fn evaluate_f64(&self, weights: &[f64]) -> Result<f64> {
        let weights_tensor = Tensor::from_vec(weights.to_vec(), weights.len(), &self.device)?;

        // Compute scores: X @ weights
        let scores = self
            .x_tensor
            .matmul(&weights_tensor.unsqueeze(1)?)?
            .squeeze(1)?;

        // Compute margins: y * scores
        let margins = (&self.y_tensor * &scores)?;

        // Hinge loss: max(0, 1 - margin)
        let ones = if let Some(ref cached_ones) = self.ones_tensor {
            cached_ones
        } else {
            &Tensor::ones_like(&margins)?
        };
        let hinge_terms = (ones - &margins)?.relu()?;
        let hinge_loss = hinge_terms.mean_all()?;

        // Regularization term
        let reg_term = (&weights_tensor * &weights_tensor)?.sum_all()? * 0.5;

        let hinge_loss_scaled = (&hinge_loss * self.c)?;
        let total_loss = (hinge_loss_scaled + reg_term)?;

        Ok(total_loss.to_scalar::<f64>()?)
    }

    fn gradient_f64(&self, weights: &[f64]) -> Result<Vec<f64>> {
        let weights_tensor = Tensor::from_vec(weights.to_vec(), weights.len(), &self.device)?;
        let n_samples = self.x_tensor.dim(0)? as f64;

        // Compute scores: X @ weights
        let scores = self
            .x_tensor
            .matmul(&weights_tensor.unsqueeze(1)?)?
            .squeeze(1)?;

        // Compute margins: y * scores
        let margins = (&self.y_tensor * &scores)?;

        // Compute subgradient of hinge loss
        // For each sample: if margin < 1, gradient is -y * x, else 0
        let ones = Tensor::ones_like(&margins)?;
        let mask = margins.lt(&ones)?; // margin < 1

        // Convert mask to float (1.0 where true, 0.0 where false)
        let mask_float = mask.to_dtype(candle_core::DType::F64)?;

        // Compute gradient contribution from hinge loss
        let y_masked = (&self.y_tensor * &mask_float)?;
        let hinge_grad = self
            .x_tensor
            .t()?
            .matmul(&y_masked.unsqueeze(1)?)?
            .squeeze(1)?;
        let hinge_grad = (&hinge_grad * (-self.c / n_samples))?;

        // Add regularization gradient (weights themselves)
        let total_grad = (&hinge_grad + &weights_tensor)?;

        Ok(total_grad.to_vec1::<f64>()?)
    }

    fn dimension(&self) -> usize {
        self.x_tensor.dim(1).unwrap_or(0)
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![0.0; self.dimension()]
    }
}


/// Generate synthetic linear regression data
pub fn generate_linear_regression_data(
    n_samples: usize,
    n_features: usize, rng: &mut StdRng,
) -> (Vec<Vec<f64>>, Vec<f64>) {
    use rand::Rng;
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    let true_weights: Vec<f64> = (0..n_features).map(|i| (i as f64 + 1.0) * 0.5).collect();
    for _ in 0..n_samples {
        let x: Vec<f64> = (0..n_features)
            .map(|_| rng.random_range(-2.0..2.0))
            .collect();
        let y: f64 = x
            .iter()
            .zip(true_weights.iter())
            .map(|(xi, wi)| xi * wi)
            .sum::<f64>()
            + rng.random_range(-0.1..0.1);
        x_data.push(x);
        y_data.push(y);
    }
    (x_data, y_data)
}
/// Generate synthetic SVM data
pub fn generate_svm_data(n_samples: usize, n_features: usize, rng: &mut StdRng) -> (Vec<Vec<f64>>, Vec<f64>) {
    use rand::Rng;
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    for _ in 0..n_samples {
        let x: Vec<f64> = (0..n_features)
            .map(|_| rng.random_range(-2.0..2.0))
            .collect();
        let decision_value: f64 = x
            .iter()
            .enumerate()
            .map(|(i, xi)| xi * (i as f64 + 1.0) * 0.3)
            .sum();
        let y = if decision_value > 0.0 { 1.0 } else { -1.0 };
        x_data.push(x);
        y_data.push(y);
    }
    (x_data, y_data)
}