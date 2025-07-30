use crate::OptimizationProblem;
use candle_core::{Device, Tensor};
use candle_nn::{linear, ops::softmax, Linear, Module, VarBuilder, VarMap};
use parking_lot::RwLock;
use rand::prelude::StdRng;
use rand::Rng;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::Arc;
#[derive(Debug, Clone, Copy)]
pub enum ActivationType {
    ReLU,
    Logistic,
    Sinewave,
}

#[derive(Debug)]
struct MnistData {
    images: Vec<Vec<u8>>,
    labels: Vec<u8>,
}

#[derive(Debug, Clone)]
struct MLP {
    layers: Vec<Linear>,
    activation: ActivationType,
}

impl MLP {
    fn new(
        vs: VarBuilder,
        input_dim: usize,
        hidden_dims: &[usize],
        output_dim: usize,
        activation: ActivationType,
    ) -> candle_core::Result<Self> {
        let mut layers = Vec::new();
        let mut prev_dim = input_dim;

        // Create hidden layers
        for (i, &hidden_dim) in hidden_dims.iter().enumerate() {
            layers.push(linear(prev_dim, hidden_dim, vs.pp(format!("ln{i}")))?);
            prev_dim = hidden_dim;
        }

        // Create output layer
        layers.push(linear(
            prev_dim,
            output_dim,
            vs.pp(format!("ln{}", hidden_dims.len())),
        )?);

        Ok(Self { layers, activation })
    }
    fn apply_activation(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        match self.activation {
            ActivationType::ReLU => xs.relu(),
            ActivationType::Logistic => {
                // Implement sigmoid manually: 1 / (1 + exp(-x))
                let neg_xs = xs.neg()?;
                let exp_neg_xs = neg_xs.exp()?;
                let one_plus_exp = (exp_neg_xs + 1.0)?;
                one_plus_exp.recip()
            }
            ActivationType::Sinewave => xs.sin(),
        }
    }
}

impl Module for MLP {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let mut xs = xs.clone();

        // Apply all layers except the last one with activation
        for (i, layer) in self.layers.iter().enumerate() {
            xs = layer.forward(&xs)?;

            // Apply activation to all but the last layer
            if i < self.layers.len() - 1 {
                xs = self.apply_activation(&xs)?;
            }
        }

        Ok(xs)
    }
}

/// MNIST-like neural network training problem
#[derive(Clone)]
pub struct MnistNeuralNetwork {
    x_data: Vec<Vec<f64>>, // Store raw data instead of tensors
    y_data: Vec<Vec<f64>>, // Store raw labels
    batch_size: usize,
    device: Device,
    name: String,
    varmap: VarMap,
    model: MLP,
    optimal_value: Option<f64>,
    param_count: usize,
    param_cache: Arc<RwLock<Option<Vec<f64>>>>,
    gradient_cache: Arc<RwLock<Option<Vec<f64>>>>,
    batch_tensors: Arc<RwLock<Option<(Tensor, Tensor)>>>, // Cache for batch tensors
    dropout_rate: f64,
    l2_regularization: f64,
    activation: ActivationType,
    precision: candle_core::DType,
}

impl MnistNeuralNetwork {
    pub fn new(
        x_data: Vec<Vec<f64>>,
        y_data: Vec<Vec<f64>>,
        hidden_sizes: &[usize],
        batch_size: Option<usize>,
        rng: &mut StdRng,
        activation: Option<ActivationType>,
    ) -> anyhow::Result<Self> {
        if hidden_sizes.is_empty() {
            return Err(anyhow::anyhow!(
                "At least one hidden layer size must be specified"
            ));
        }

        // Use CUDA if available
        let device = Device::cuda_if_available(0)?;
        let n_samples = x_data.len();
        let batch_size = batch_size.unwrap_or(32).min(n_samples);
        let activation = activation.unwrap_or(ActivationType::ReLU);
        let activation_name = match activation {
            ActivationType::ReLU => "relu",
            ActivationType::Logistic => "logistic",
            ActivationType::Sinewave => "sine",
        };
        let hidden_str = hidden_sizes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("x");
        let name = format!("MNIST_NN_{n_samples}samples_hidden{hidden_str}_{activation_name}");

        let input_dim = x_data.first().map(|x| x.len()).unwrap_or(784);
        let output_dim = y_data.first().map(|y| y.len()).unwrap_or(10);
        let precision = candle_core::DType::F64;

        // Create model with proper candle layers
        let varmap = VarMap::new();
        let vs = VarBuilder::from_varmap(&varmap, precision, &device);
        let model = MLP::new(vs, input_dim, hidden_sizes, output_dim, activation)?;

        // Pre-calculate parameter count
        let mut param_count = 0;
        let mut prev_dim = input_dim;
        for &hidden_dim in hidden_sizes {
            param_count += (prev_dim + 1) * hidden_dim;
            prev_dim = hidden_dim;
        }
        param_count += (prev_dim + 1) * output_dim;

        // Initialize with appropriate initialization for the activation
        let instance = Self {
            x_data,
            y_data,
            batch_size,
            device,
            name,
            varmap,
            model,
            optimal_value: None,
            param_count,
            param_cache: Arc::new(RwLock::new(None)),
            gradient_cache: Arc::new(RwLock::new(None)),
            batch_tensors: Arc::new(RwLock::new(None)),
            dropout_rate: 0.2,
            l2_regularization: 1e-4,
            activation,
            precision,
        };
        instance.initialize_weights(rng)?;

        Ok(instance)
    }

    pub fn set_optimal_value(&mut self, value: Option<f64>) {
        self.optimal_value = value;
    }

    pub fn load_mnist(
        n_samples: Option<usize>,
        hidden_sizes: &[usize],
        batch_size: Option<usize>,
        rng: &mut StdRng,
        activation: Option<ActivationType>,
    ) -> anyhow::Result<Self> {
        if !Path::new("data/train-images-idx3-ubyte").exists() {
            println!("MNIST files not found, downloading...");
            Self::download_mnist_data()?;
        }
        let mnist_data = Self::try_load_mnist_files()?;
        let actual_samples = n_samples.unwrap_or(1000).min(mnist_data.images.len());
        // Shuffle indices for better training
        let mut indices: Vec<usize> = (0..actual_samples).collect();
        use rand::seq::SliceRandom;
        indices.shuffle(rng);

        let mut x_data = Vec::with_capacity(actual_samples);
        let mut y_data = Vec::with_capacity(actual_samples);

        for &i in &indices {
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

        Self::new(x_data, y_data, hidden_sizes, batch_size, rng, activation)
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
                println!("Downloading {url}...");
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
        // Try curl first
        if let Ok(output) = std::process::Command::new("curl")
            .args(["-L", "-f", "-s", "-o", path, url])
            .output()
        {
            if output.status.success() {
                return Ok(());
            }
        }

        // Fallback to wget
        if let Ok(output) = std::process::Command::new("wget")
            .args(["-q", "-O", path, url])
            .output()
        {
            if output.status.success() {
                return Ok(());
            }
        }

        Err(anyhow::anyhow!(
            "Failed to download {} - neither curl nor wget available",
            url
        ))
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
                println!("Decompressing {gz_path}...");
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
        let mut images = Vec::with_capacity(num_images);
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

    pub fn create(
        n_samples: Option<usize>,
        hidden_sizes: &[usize],
        batch_size: Option<usize>,
        rng: &mut StdRng,
        activation: Option<ActivationType>,
    ) -> anyhow::Result<Self> {
        // Validate hidden sizes to prevent overflow
        for (i, &hidden_size) in hidden_sizes.iter().enumerate() {
            if hidden_size > 2048 {
                return Err(anyhow::anyhow!(
                    "Hidden size at layer {} too large: {} (max 2048)",
                    i,
                    hidden_size
                ));
            }
            if hidden_size == 0 {
                return Err(anyhow::anyhow!("Hidden size at layer {} cannot be zero", i));
            }
        }
        let samples = n_samples.unwrap_or(1000);
        if samples > 60000 {
            return Err(anyhow::anyhow!("Too many samples: {} (max 60000)", samples));
        }

        // Try to load real MNIST data first
        Self::load_mnist(Some(samples), hidden_sizes, batch_size, rng, activation)
    }
    /// Convenience function to create a network with a single hidden layer
    pub fn create_single_hidden(
        n_samples: Option<usize>,
        hidden_size: usize,
        batch_size: Option<usize>,
        rng: &mut StdRng,
        activation: Option<ActivationType>,
    ) -> anyhow::Result<Self> {
        Self::create(n_samples, &[hidden_size], batch_size, rng, activation)
    }

    fn count_parameters(&self) -> usize {
        self.param_count
    }

    fn set_parameters(&self, params: &[f64]) -> anyhow::Result<()> {
        // Check all parameters for non-finite values before setting
        if params.iter().any(|&p| !p.is_finite()) {
            return Err(anyhow::anyhow!("Non-finite parameters detected"));
        }
        // Check for extreme values that might cause numerical instability
        let max_abs = params.iter().map(|p| p.abs()).fold(0.0, f64::max);
        if max_abs > 1e6 {
            return Err(anyhow::anyhow!(
                "Parameters too large: max abs value = {}",
                max_abs
            ));
        }

        // Invalidate caches when parameters change
        *self.param_cache.write() = None;
        *self.gradient_cache.write() = None;

        // Set model parameters from flat vector
        let mut param_idx = 0;
        let mut data = self.varmap.data().lock().unwrap();

        for (_name, var) in data.iter_mut() {
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
        // Check cache first
        if let Some(cached) = self.param_cache.read().as_ref() {
            return Ok(cached.clone());
        }

        let mut params = Vec::with_capacity(self.param_count);

        let data = self.varmap.data().lock().unwrap();

        for (_, var) in data.iter() {
            let tensor = var.as_tensor();
            let values = tensor.flatten_all()?.to_vec1::<f64>()?;
            params.extend(values);
        }
        // Cache the parameters
        *self.param_cache.write() = Some(params.clone());

        Ok(params)
    }

    /// Initialize weights using appropriate initialization for the activation function
    fn initialize_weights(&self, rng: &mut StdRng) -> anyhow::Result<()> {
        let mut data = self.varmap.data().lock().unwrap();
        for (_name, var) in data.iter_mut() {
            let tensor = var.as_tensor();
            let shape = tensor.shape();
            let dims = shape.dims();
            if dims.len() == 2 {
                // This is a weight matrix
                let fan_in = dims[1]; // Number of input units
                let fan_out = dims[0]; // Number of output units

                // Choose initialization based on activation function
                let std_dev = match self.activation {
                    ActivationType::ReLU => {
                        // He initialization for ReLU
                        (2.0 / fan_in as f64).sqrt()
                    }
                    ActivationType::Logistic => {
                        // Xavier/Glorot initialization for logistic
                        (2.0 / (fan_in + fan_out) as f64).sqrt()
                    }
                    ActivationType::Sinewave => {
                        // For sine activation, use a smaller initialization
                        // to keep inputs in the linear region of sine
                        (1.0 / (fan_in + fan_out) as f64).sqrt()
                    }
                };

                // Generate initialized weights
                let mut weights = Vec::with_capacity(tensor.elem_count());
                for _ in 0..tensor.elem_count() {
                    // Sample from normal distribution with appropriate scaling
                    let normal: f64 = rng.sample(rand_distr::StandardNormal);
                    weights.push(normal * std_dev);
                }
                let new_tensor = Tensor::from_vec(weights, shape, &self.device)?;
                var.set(&new_tensor)?;
            } else if dims.len() == 1 {
                // This is a bias vector - initialize to zeros
                let biases = vec![0.0; tensor.elem_count()];
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
            let variance: f64 =
                values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
            let std_dev = variance.sqrt();
            let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            // Check for dead neurons (all zeros)
            let zero_count = values.iter().filter(|&&x| x.abs() < 1e-10).count();
            let zero_percentage = (zero_count as f64 / values.len() as f64) * 100.0;
            // Check for extreme values
            let extreme_count = values
                .iter()
                .filter(|&&x| x.abs() > 3.0 * std_dev + mean.abs())
                .count();
            let extreme_percentage = (extreme_count as f64 / values.len() as f64) * 100.0;
            println!("\nParameter: {name}");
            println!("  Shape: {:?}", tensor.shape());
            println!("  Mean: {mean:.6}");
            println!("  Std Dev: {std_dev:.6}");
            println!("  Min/Max: {min:.6} / {max:.6}");
            println!("  Zero values: {zero_count} ({zero_percentage:.2}%)");
            println!("  Extreme values (>3σ): {extreme_count} ({extreme_percentage:.2}%)");
            // Determine if this is a weight or bias based on shape
            let dims = tensor.shape().dims();
            if dims.len() == 2 {
                // Weight matrix - check He initialization criteria
                let fan_in = dims[1];
                let fan_out = dims[0];
                let expected_std = match self.activation {
                    ActivationType::ReLU => (2.0 / fan_in as f64).sqrt(),
                    ActivationType::Logistic => (2.0 / (fan_in + fan_out) as f64).sqrt(),
                    ActivationType::Sinewave => (1.0 / (fan_in + fan_out) as f64).sqrt(),
                };
                let std_ratio = std_dev / expected_std;
                let init_name = match self.activation {
                    ActivationType::ReLU => "He",
                    ActivationType::Logistic => "Xavier/Glorot",
                    ActivationType::Sinewave => "Small Xavier",
                };
                println!("  Expected std ({init_name}): {expected_std:.6}");
                println!("  Actual/Expected ratio: {std_ratio:.3}");
                if !(0.8..=1.2).contains(&std_ratio) {
                    println!("  ⚠️  Warning: Standard deviation deviates significantly from {init_name} initialization");
                } else {
                    println!("  ✓ Standard deviation is within expected range");
                }
            } else if dims.len() == 1 {
                // Bias vector
                if mean.abs() > 0.01 {
                    println!("  ⚠️  Warning: Bias should be initialized to zero");
                } else {
                    println!("  ✓ Bias initialization is correct");
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
        // Set parameters in the model
        self.set_parameters(params)?;

        let n_samples = self.x_data.len();
        let n_batches = n_samples.div_ceil(self.batch_size);
        let mut total_loss = 0.0;

        // Process batches in parallel using rayon
        let batch_losses: Vec<(f64, usize)> = (0..n_batches)
            .into_par_iter()
            .map(|batch_idx| -> anyhow::Result<(f64, usize)> {
                let start = batch_idx * self.batch_size;
                let end = ((batch_idx + 1) * self.batch_size).min(n_samples);
                let batch_size = end - start;

                // Use Tensor::cat for efficient batch creation
                let x_tensors: Vec<Tensor> = (start..end)
                    .map(|i| {
                        Tensor::from_vec(
                            self.x_data[i].clone(),
                            (1, self.x_data[0].len()),
                            &self.device,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let x_batch = Tensor::cat(&x_tensors, 0)?;

                let y_tensors: Vec<Tensor> = (start..end)
                    .map(|i| {
                        Tensor::from_vec(
                            self.y_data[i].clone(),
                            (1, self.y_data[0].len()),
                            &self.device,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let y_batch = Tensor::cat(&y_tensors, 0)?;

                // Forward pass
                let y_pred = self.model.forward(&x_batch)?;
                let y_pred = softmax(&y_pred, 1)?;

                // Cross-entropy loss for this batch
                let log_probs = y_pred.clamp(1e-10, 1.0 - 1e-10)?.log()?;
                let batch_loss = (&y_batch * &log_probs)?.sum_keepdim(1)?.mean_all()?.neg()?;

                let batch_loss_value = batch_loss.to_scalar::<f64>()?;
                Ok((batch_loss_value, batch_size))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Aggregate batch losses
        for (loss, size) in batch_losses {
            total_loss += loss * (size as f64);
        }

        // Average loss across all samples
        let mut loss_value = total_loss / (n_samples as f64);

        // Add L2 regularization
        if self.l2_regularization > 0.0 {
            let params_squared_sum: f64 = params.iter().map(|p| p * p).sum();
            loss_value += 0.5 * self.l2_regularization * params_squared_sum;
        }

        // Check final loss for non-finite values
        if !loss_value.is_finite() {
            return Err(anyhow::anyhow!("Non-finite loss value: {}", loss_value));
        }

        Ok(loss_value)
    }

    fn gradient_f64(&self, params: &[f64]) -> anyhow::Result<Vec<f64>> {
        // Check gradient cache first
        if let Some(cached) = self.gradient_cache.read().as_ref() {
            if let Some(cached_params) = self.param_cache.read().as_ref() {
                if cached_params == params {
                    return Ok(cached.clone());
                }
            }
        }

        // Set parameters
        self.set_parameters(params)?;
        let n_samples = self.x_data.len();
        let n_batches = n_samples.div_ceil(self.batch_size);

        // Accumulate gradients across batches
        let mut accumulated_grads = vec![0.0; self.param_count];

        // Process batches in parallel
        let batch_grads: Vec<Vec<f64>> = (0..n_batches)
            .into_par_iter()
            .map(|batch_idx| -> anyhow::Result<Vec<f64>> {
                let start = batch_idx * self.batch_size;
                let end = ((batch_idx + 1) * self.batch_size).min(n_samples);
                let batch_size = end - start;

                // Use Tensor::cat for efficient batch creation
                let x_tensors: Vec<Tensor> = (start..end)
                    .map(|i| {
                        Tensor::from_vec(
                            self.x_data[i].clone(),
                            (1, self.x_data[0].len()),
                            &self.device,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let x_batch = Tensor::cat(&x_tensors, 0)?;

                let y_tensors: Vec<Tensor> = (start..end)
                    .map(|i| {
                        Tensor::from_vec(
                            self.y_data[i].clone(),
                            (1, self.y_data[0].len()),
                            &self.device,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let y_batch = Tensor::cat(&y_tensors, 0)?;

                // Create variables for autodiff
                let mut vars = Vec::with_capacity(self.model.layers.len() * 2); // Each layer has weights and biases

                let data = self.varmap.data().lock().unwrap();
                for (_, var) in data.iter() {
                    vars.push(var.clone());
                }
                drop(data);

                // Forward pass with autodiff
                let y_pred = self.model.forward(&x_batch)?;
                let y_pred = softmax(&y_pred, 1)?;

                // Compute loss
                let log_probs = y_pred.clamp(1e-10, 1.0 - 1e-10)?.log()?;
                let loss = (&y_batch * &log_probs)?.sum_keepdim(1)?.mean_all()?.neg()?;

                // Compute gradients using candle's autodiff
                let grads = loss.backward()?;

                // Extract gradients in the same order as parameters
                let mut batch_grads = vec![0.0; self.param_count];
                let mut grad_idx = 0;

                for var in &vars {
                    if let Some(grad) = grads.get(var) {
                        let grad_values = grad.flatten_all()?.to_vec1::<f64>()?;
                        for (i, &g) in grad_values.iter().enumerate() {
                            batch_grads[grad_idx + i] = g * (batch_size as f64);
                        }
                        grad_idx += grad_values.len();
                    } else {
                        // If no gradient, assume zero
                        let tensor = var.as_tensor();
                        grad_idx += tensor.elem_count();
                    }
                }
                Ok(batch_grads)
            })
            .collect::<Result<Vec<_>, _>>()?;
        // Aggregate gradients from all batches
        for batch_grad in batch_grads {
            for (i, &g) in batch_grad.iter().enumerate() {
                accumulated_grads[i] += g;
            }
        }

        // Average gradients across all samples
        for g in &mut accumulated_grads {
            *g /= n_samples as f64;
        }

        // Add L2 regularization gradient
        if self.l2_regularization > 0.0 {
            for (i, g) in accumulated_grads.iter_mut().enumerate() {
                *g += self.l2_regularization * params[i];
            }
        }

        // Gradient clipping to prevent exploding gradients
        let grad_norm: f64 = accumulated_grads.iter().map(|g| g * g).sum::<f64>().sqrt();
        if grad_norm > 10.0 {
            let scale = 10.0 / grad_norm;
            for g in &mut accumulated_grads {
                *g *= scale;
            }
        }
        // Cache the gradient
        *self.gradient_cache.write() = Some(accumulated_grads.clone());

        Ok(accumulated_grads)
    }
    fn optimal_value(&self) -> Option<f64> {
        self.optimal_value
    }
}
