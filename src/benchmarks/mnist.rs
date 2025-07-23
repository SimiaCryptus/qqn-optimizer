use crate::OptimizationProblem;
use candle_core::{Device, Tensor};
use candle_nn::{linear, ops::softmax, Linear, Module, VarBuilder, VarMap};
use rand::{Rng};
use std::fs;
use std::path::Path;
use rand::prelude::StdRng;

#[derive(Debug)]
struct MnistData {
    images: Vec<Vec<u8>>,
    labels: Vec<u8>,
}

#[derive(Debug, Clone)]
struct MLP {
    ln1: Linear,
    ln2: Linear,
}

impl MLP {
    fn new(
        vs: VarBuilder,
        input_dim: usize,
        hidden_dim: usize,
        output_dim: usize,
    ) -> candle_core::Result<Self> {
        let ln1 = linear(input_dim, hidden_dim, vs.pp("ln1"))?;
        let ln2 = linear(hidden_dim, output_dim, vs.pp("ln2"))?;
        Ok(Self { ln1, ln2 })
    }
}

impl Module for MLP {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let xs = self.ln1.forward(xs)?;
        let xs = xs.relu()?;
        self.ln2.forward(&xs)
    }
}

/// MNIST-like neural network training problem
#[derive(Clone)]
pub struct MnistNeuralNetwork {
    x_tensor: Tensor, // 28x28 = 784 features
    y_tensor: Tensor, // 10 classes (one-hot encoded)
    device: Device,
    name: String,
    varmap: VarMap,
    model: MLP,
    optimal_value: Option<f64>,
}

impl MnistNeuralNetwork {
    pub fn new(
        x_data: Vec<Vec<f64>>,
        y_data: Vec<Vec<f64>>,
        hidden_size: usize,
        rng: &mut StdRng,
    ) -> anyhow::Result<Self> {
        let device = Device::Cpu;
        let n_samples = x_data.len();
        let name = format!("MNIST_NN_{}samples_hidden{}", n_samples, hidden_size);

        // Convert to tensors
        let input_dim = x_data.first().map(|x| x.len()).unwrap_or(784);
        let output_dim = y_data.first().map(|y| y.len()).unwrap_or(10);

        let x_flat: Vec<f64> = x_data.into_iter().flatten().collect();
        let y_flat: Vec<f64> = y_data.into_iter().flatten().collect();

        let x_tensor = Tensor::from_vec(x_flat, (n_samples, input_dim), &device)?;
        let y_tensor = Tensor::from_vec(y_flat, (n_samples, output_dim), &device)?;
        // Create model with proper candle layers
        let varmap = VarMap::new();
        let vs = VarBuilder::from_varmap(&varmap, candle_core::DType::F64, &device);
        let model = MLP::new(vs, input_dim, hidden_size, output_dim)?;
        // Initialize with He initialization for ReLU activation
        let mut instance = Self {
            x_tensor,
            y_tensor,
            device,
            name,
            varmap,
            model,
            optimal_value: Option::from(0.5_f64),
        };
        instance.he_initialize(rng)?; // Use seed 42 for reproducibility

        Ok(instance)
    }

    pub fn set_optimal_value(&mut self, value: Option<f64>) {
        self.optimal_value = value;
    }

    pub fn load_mnist(n_samples: Option<usize>, hidden_size: usize, rng: &mut StdRng) -> anyhow::Result<Self> {
        if !Path::new("data/train-images-idx3-ubyte").exists() {
            println!("MNIST files not found, downloading...");
            let _mnist_data = Self::download_mnist_data()?;
        }
        let mnist_data = Self::try_load_mnist_files()?;
        let actual_samples = n_samples.unwrap_or(1000).min(mnist_data.images.len());
        let mut x_data = Vec::new();
        let mut y_data = Vec::new();

        for i in 0..actual_samples {
            // Convert image data to f64 and normalize to [0, 1]
            let image: Vec<f64> = mnist_data.images[i]
                .iter()
                .map(|&pixel| pixel as f64 / 255.0)
                .collect();

            // Convert label to one-hot encoding
            let mut label = vec![0.0; 10];
            label[mnist_data.labels[i] as usize] = 1.0;

            x_data.push(image);
            y_data.push(label);
        }

        Self::new(x_data, y_data, hidden_size, rng)
    }

    fn try_load_mnist_files() -> anyhow::Result<MnistData> {
        // Try to load from standard MNIST file locations
        let train_images = Self::load_mnist_images("data/train-images-idx3-ubyte")?;
        let train_labels = Self::load_mnist_labels("data/train-labels-idx1-ubyte")?;

        Ok(MnistData {
            images: train_images,
            labels: train_labels,
        })
    }

    fn download_mnist_data() -> anyhow::Result<MnistData> {
        // Create data directory if it doesn't exist
        fs::create_dir_all("data")?;

        // Download URLs
        let urls = [
            (
                "https://raw.githubusercontent.com/fgnt/mnist/master/train-images-idx3-ubyte.gz",
                "data/train-images-idx3-ubyte.gz",
            ),
            (
                "https://raw.githubusercontent.com/fgnt/mnist/master/train-labels-idx1-ubyte.gz",
                "data/train-labels-idx1-ubyte.gz",
            ),
            (
                "https://raw.githubusercontent.com/fgnt/mnist/master/t10k-images-idx3-ubyte.gz",
                "data/t10k-images-idx3-ubyte.gz",
            ),
            (
                "https://raw.githubusercontent.com/fgnt/mnist/master/t10k-labels-idx1-ubyte.gz",
                "data/t10k-labels-idx1-ubyte.gz",
            ),
        ];

        // Download files if they don't exist
        for (url, path) in &urls {
            if !Path::new(path).exists() {
                println!("Downloading {}...", url);
                Self::download_file(url, path)?;
            }
        }

        // Decompress files
        Self::decompress_mnist_files()?;

        // Load the decompressed data
        let train_images = Self::load_mnist_images("data/train-images-idx3-ubyte")?;
        let train_labels = Self::load_mnist_labels("data/train-labels-idx1-ubyte")?;

        Ok(MnistData {
            images: train_images,
            labels: train_labels,
        })
    }

    fn download_file(url: &str, path: &str) -> anyhow::Result<()> {
        // Use std::process::Command to download with curl or wget as fallback
        // This avoids the async runtime conflict
        let output = std::process::Command::new("curl")
            .args(&["-L", "-o", path, url])
            .output();

        match output {
            Ok(result) if result.status.success() => Ok(()),
            _ => {
                // Fallback to wget if curl fails
                let output = std::process::Command::new("wget")
                    .args(&["-O", path, url])
                    .output();

                match output {
                    Ok(result) if result.status.success() => Ok(()),
                    _ => Err(anyhow::anyhow!(
                        "Failed to download {} - neither curl nor wget available",
                        url
                    )),
                }
            }
        }
    }

    fn decompress_mnist_files() -> anyhow::Result<()> {
        use flate2::read::GzDecoder;
        use std::fs::File;
        use std::io::BufReader;

        let files = [
            (
                "data/train-images-idx3-ubyte.gz",
                "data/train-images-idx3-ubyte",
            ),
            (
                "data/train-labels-idx1-ubyte.gz",
                "data/train-labels-idx1-ubyte",
            ),
            (
                "data/t10k-images-idx3-ubyte.gz",
                "data/t10k-images-idx3-ubyte",
            ),
            (
                "data/t10k-labels-idx1-ubyte.gz",
                "data/t10k-labels-idx1-ubyte",
            ),
        ];

        for (gz_path, out_path) in &files {
            if Path::new(gz_path).exists() && !Path::new(out_path).exists() {
                println!("Decompressing {}...", gz_path);
                let gz_file = File::open(gz_path)?;
                let mut decoder = GzDecoder::new(BufReader::new(gz_file));
                let mut out_file = File::create(out_path)?;
                std::io::copy(&mut decoder, &mut out_file)?;
            }
        }

        Ok(())
    }

    fn load_mnist_images(path: &str) -> anyhow::Result<Vec<Vec<u8>>> {
        use std::fs::File;
        use std::io::{BufReader, Read};

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read magic number
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        // Read number of images
        let mut num_images_bytes = [0u8; 4];
        reader.read_exact(&mut num_images_bytes)?;
        let num_images = u32::from_be_bytes(num_images_bytes) as usize;

        // Read dimensions
        let mut rows_bytes = [0u8; 4];
        let mut cols_bytes = [0u8; 4];
        reader.read_exact(&mut rows_bytes)?;
        reader.read_exact(&mut cols_bytes)?;
        let rows = u32::from_be_bytes(rows_bytes) as usize;
        let cols = u32::from_be_bytes(cols_bytes) as usize;

        // Read image data
        let mut images = Vec::new();
        for _ in 0..num_images {
            let mut image = vec![0u8; rows * cols];
            reader.read_exact(&mut image)?;
            images.push(image);
        }

        Ok(images)
    }

    fn load_mnist_labels(path: &str) -> anyhow::Result<Vec<u8>> {
        use std::fs::File;
        use std::io::{BufReader, Read};

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read magic number
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        // Read number of labels
        let mut num_labels_bytes = [0u8; 4];
        reader.read_exact(&mut num_labels_bytes)?;
        let num_labels = u32::from_be_bytes(num_labels_bytes) as usize;

        // Read labels
        let mut labels = vec![0u8; num_labels];
        reader.read_exact(&mut labels)?;

        Ok(labels)
    }

    /// Create MNIST problem with automatic fallback
    pub fn create(n_samples: Option<usize>, hidden_size: usize, rng: &mut StdRng) -> anyhow::Result<Self> {
        // Validate hidden size to prevent overflow
        if hidden_size > 1000 {
            return Err(anyhow::anyhow!(
                "Hidden size too large: {} (max 1000)",
                hidden_size
            ));
        }
        let samples = n_samples.unwrap_or(1000);
        if samples > 10000 {
            return Err(anyhow::anyhow!("Too many samples: {} (max 10000)", samples));
        }

        // Try to load real MNIST data first
        Self::load_mnist(Some(samples), hidden_size, rng)
    }

    fn count_parameters(&self) -> usize {
        // Count parameters from the actual model
        self.varmap.data().lock().unwrap().len()
    }

    fn set_parameters(&self, params: &[f64]) -> anyhow::Result<()> {
        // Check all parameters for non-finite values before setting
        for (i, &param) in params.iter().enumerate() {
            if !param.is_finite() {
                return Err(anyhow::anyhow!(
                    "Attempting to set non-finite parameter at index {}: {}",
                    i,
                    param
                ));
            }
        }

        // Set model parameters from flat vector
        let mut param_idx = 0;
        let data = self.varmap.data().lock().unwrap();

        for (_name, var) in data.iter() {
            let tensor = var.as_tensor();
            let elem_count = tensor.elem_count();

            if param_idx + elem_count > params.len() {
                return Err(anyhow::anyhow!("Not enough parameters provided"));
            }

            let param_slice = &params[param_idx..param_idx + elem_count];
            let new_tensor = Tensor::from_vec(param_slice.to_vec(), tensor.shape(), &self.device)?;
            var.set(&new_tensor)?;

            param_idx += elem_count;
        }

        Ok(())
    }

    fn get_parameters(&self) -> anyhow::Result<Vec<f64>> {
        let mut params = Vec::new();
        let data = self.varmap.data().lock().unwrap();

        for (_, var) in data.iter() {
            let tensor = var.as_tensor();
            let values = tensor.flatten_all()?.to_vec1::<f64>()?;
            // Check retrieved parameters for non-finite values
            for (i, &val) in values.iter().enumerate() {
                if !val.is_finite() {
                    return Err(anyhow::anyhow!(
                        "Non-finite parameter retrieved at index {}: {}",
                        params.len() + i,
                        val
                    ));
                }
            }
            params.extend(values);
        }

        Ok(params)
    }
    /// Initialize weights using He initialization (optimal for ReLU activation)
    fn he_initialize(&self, rng: &mut StdRng) -> anyhow::Result<()> {
        let data = self.varmap.data().lock().unwrap();
        for (_name, var) in data.iter() {
            let tensor = var.as_tensor();
            let shape = tensor.shape();
            let dims = shape.dims();
            if dims.len() == 2 {
                // This is a weight matrix
                let fan_in = dims[1]; // Number of input units
                let std_dev = (2.0 / fan_in as f64).sqrt();
                // Generate He-initialized weights
                let mut weights = Vec::new();
                for _ in 0..tensor.elem_count() {
                    // Sample from normal distribution with He scaling
                    let normal: f64 = rng.sample(rand_distr::StandardNormal);
                    weights.push(normal * std_dev);
                }
                let new_tensor = Tensor::from_vec(weights, shape, &self.device)?;
                var.set(&new_tensor)?;
            } else if dims.len() == 1 {
                // This is a bias vector - initialize to small positive values for ReLU
                let mut biases = Vec::new();
                for _ in 0..tensor.elem_count() {
                    biases.push(0.01); // Small positive bias for ReLU units
                }
                let new_tensor = Tensor::from_vec(biases, shape, &self.device)?;
                var.set(&new_tensor)?;
            }
        }
        Ok(())
    }
    /// Verify the quality of weight initialization
    pub fn verify_initialization(&self) -> anyhow::Result<()> {
        println!("\n=== Weight Initialization Quality Check ===");
        let data = self.varmap.data().lock().unwrap();
        for (name, var) in data.iter() {
            let tensor = var.as_tensor();
            let values = tensor.flatten_all()?.to_vec1::<f64>()?;
            if values.is_empty() {
                continue;
            }
            // Calculate statistics
            let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
            let variance: f64 = values.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / values.len() as f64;
            let std_dev = variance.sqrt();
            let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            // Check for dead neurons (all zeros)
            let zero_count = values.iter().filter(|&&x| x.abs() < 1e-10).count();
            let zero_percentage = (zero_count as f64 / values.len() as f64) * 100.0;
            // Check for extreme values
            let extreme_count = values.iter()
                .filter(|&&x| x.abs() > 3.0 * std_dev)
                .count();
            let extreme_percentage = (extreme_count as f64 / values.len() as f64) * 100.0;
            println!("\nParameter: {}", name);
            println!("  Shape: {:?}", tensor.shape());
            println!("  Mean: {:.6}", mean);
            println!("  Std Dev: {:.6}", std_dev);
            println!("  Min/Max: {:.6} / {:.6}", min, max);
            println!("  Zero values: {} ({:.2}%)", zero_count, zero_percentage);
            println!("  Extreme values (>3σ): {} ({:.2}%)", extreme_count, extreme_percentage);
            // Determine if this is a weight or bias based on shape
            let dims = tensor.shape().dims();
            if dims.len() == 2 {
                // Weight matrix - check He initialization criteria
                let fan_in = dims[1];
                let expected_std = (2.0 / fan_in as f64).sqrt();
                let std_ratio = std_dev / expected_std;
                println!("  Expected std (He): {:.6}", expected_std);
                println!("  Actual/Expected ratio: {:.3}", std_ratio);
                if std_ratio < 0.8 || std_ratio > 1.2 {
                    println!("  ⚠️  Warning: Standard deviation deviates significantly from He initialization");
                } else {
                    println!("  ✓ Standard deviation is within expected range");
                }
            } else if dims.len() == 1 {
                // Bias vector
                if mean.abs() < 0.005 {
                    println!("  ⚠️  Warning: Bias initialization might be too small for ReLU");
                } else {
                    println!("  ✓ Bias initialization appropriate for ReLU");
                }
            }
            // General health checks
            if zero_percentage > 10.0 {
                println!("  ⚠️  Warning: High percentage of zero values");
            }
            if extreme_percentage > 5.0 {
                println!("  ⚠️  Warning: High percentage of extreme values");
            }
            if !mean.is_finite() || !std_dev.is_finite() {
                println!("  ❌ Error: Non-finite values detected!");
            }
        }
        println!("\n=== End of Initialization Check ===\n");
        Ok(())
    }
}

impl OptimizationProblem for MnistNeuralNetwork {
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
        // Model is already initialized with proper Xavier initialization
        // Just return the current parameters
        self.get_parameters()
            .unwrap_or_else(|_| vec![0.0; self.count_parameters()])
    }

    fn evaluate_f64(&self, params: &[f64]) -> anyhow::Result<f64> {
        // Check input parameters for non-finite values
        for (i, &param) in params.iter().enumerate() {
            if !param.is_finite() {
                return Err(anyhow::anyhow!(
                    "Non-finite parameter at index {}: {}",
                    i,
                    param
                ));
            }
        }

        // Set parameters in the model
        self.set_parameters(params)?;

        // Forward pass using candle's Module trait
        let y_pred = self.model.forward(&self.x_tensor)?;
        // Check for non-finite values in predictions
        let pred_values = y_pred.flatten_all()?.to_vec1::<f64>()?;
        for (i, &val) in pred_values.iter().enumerate() {
            if !val.is_finite() {
                return Err(anyhow::anyhow!(
                    "Non-finite prediction at index {}: {}",
                    i,
                    val
                ));
            }
        }

        let y_pred = softmax(&y_pred, 1)?;
        // Check softmax output for non-finite values
        let softmax_values = y_pred.flatten_all()?.to_vec1::<f64>()?;
        for (i, &val) in softmax_values.iter().enumerate() {
            if !val.is_finite() {
                return Err(anyhow::anyhow!(
                    "Non-finite softmax output at index {}: {}",
                    i,
                    val
                ));
            }
        }
        
        // Cross-entropy loss
        let log_probs = y_pred.clamp(1e-15, 1.0)?.log()?;
        // Check log probabilities for non-finite values
        let log_prob_values = log_probs.flatten_all()?.to_vec1::<f64>()?;
        for (i, &val) in log_prob_values.iter().enumerate() {
            if !val.is_finite() {
                return Err(anyhow::anyhow!(
                    "Non-finite log probability at index {}: {}",
                    i,
                    val
                ));
            }
        }

        let loss = (&self.y_tensor * &log_probs)?
            .sum_keepdim(1)?
            .mean_all()?
            .neg()?;

        let loss_value = loss.to_scalar::<f64>()?;

        // Check final loss for non-finite values
        if !loss_value.is_finite() {
            return Err(anyhow::anyhow!("Non-finite loss value: {}", loss_value));
        }

        Ok(loss_value)
    }

    fn gradient_f64(&self, params: &[f64]) -> anyhow::Result<Vec<f64>> {
        // Check input parameters for non-finite values
        for (i, &param) in params.iter().enumerate() {
            if !param.is_finite() {
                return Err(anyhow::anyhow!(
                    "Non-finite parameter in gradient computation at index {}: {}",
                    i,
                    param
                ));
            }
        }

        // Set parameters
        self.set_parameters(params)?;

        // Create variables for autodiff
        let mut vars = Vec::new();
        let data = self.varmap.data().lock().unwrap();
        for (_, var) in data.iter() {
            vars.push(var.clone());
        }
        drop(data);
        // Forward pass with autodiff
        let y_pred = self.model.forward(&self.x_tensor)?;
        let y_pred = softmax(&y_pred, 1)?;
        // Compute loss
        let log_probs = y_pred.log()?;
        let loss = (&self.y_tensor * &log_probs)?
            .sum_keepdim(1)?
            .mean_all()?
            .neg()?;
        // Check loss before backward pass
        let loss_value = loss.to_scalar::<f64>()?;
        if !loss_value.is_finite() {
            return Err(anyhow::anyhow!(
                "Non-finite loss in gradient computation: {}",
                loss_value
            ));
        }

        // Compute gradients using candle's autodiff
        let grads = loss.backward()?;
        // Extract gradients in the same order as parameters
        let mut grad_vec = Vec::new();
        for var in &vars {
            if let Some(grad) = grads.get(var) {
                let grad_values = grad.flatten_all()?.to_vec1::<f64>()?;
                // Check each gradient value for non-finite values
                for (i, &val) in grad_values.iter().enumerate() {
                    if !val.is_finite() {
                        return Err(anyhow::anyhow!(
                            "Non-finite gradient value at index {}: {}",
                            grad_vec.len() + i,
                            val
                        ));
                    }
                }
                grad_vec.extend(grad_values);
            } else {
                // If no gradient, assume zero
                let tensor = var.as_tensor();
                grad_vec.extend(vec![0.0; tensor.elem_count()]);
            }
        }

        Ok(grad_vec)
    }
    fn optimal_value(&self) -> Option<f64> {
        self.optimal_value
    }
}