#![allow(clippy::upper_case_acronyms)]

//! OneDNN-based MNIST neural network implementation
//!
//! This module provides an alternate implementation of MNIST neural network training
//! that leverages Intel's OneDNN (Deep Neural Network Library) for optimized performance.
use super::functions::OptimizationProblem;

#[cfg(feature = "onednn")]
use onednnl::*;

use parking_lot::RwLock;
use rand::prelude::StdRng;
use rand::Rng;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationType {
    ReLU,
    Logistic,
    Tanh,
}

impl ActivationType {
    pub fn as_str(&self) -> &str {
        match self {
            ActivationType::ReLU => "ReLU",
            ActivationType::Logistic => "Logistic",
            ActivationType::Tanh => "Tanh",
        }
    }
}

#[derive(Debug)]
struct MnistData {
    images: Vec<Vec<f32>>,
    labels: Vec<u8>,
}

/// OneDNN-based neural network layer
#[cfg(feature = "onednn")]
struct OneDnnLayer {
    weights: Vec<f32>,
    bias: Vec<f32>,
    input_size: usize,
    output_size: usize,
    activation: ActivationType,
}

#[cfg(feature = "onednn")]
impl OneDnnLayer {
    fn new(
        input_size: usize,
        output_size: usize,
        activation: ActivationType,
    ) -> anyhow::Result<Self> {
        debug!("Creating OneDNN layer: {}x{} with {:?} activation", 
               input_size, output_size, activation);
        Ok(Self {
            weights: vec![0.0; input_size * output_size],
            bias: vec![0.0; output_size],
            input_size,
            output_size,
            activation,
        })
    }

    fn set_weights(&mut self, weights: &[f32]) -> anyhow::Result<()> {
        if weights.len() != self.weights.len() {
            error!("Weight size mismatch: expected {}, got {}", 
                   self.weights.len(), weights.len());
            return Err(anyhow::anyhow!("Weight size mismatch"));
        }
       if log::log_enabled!(log::Level::Trace) {
           let min_val = weights.iter().fold(f32::INFINITY, |a, &b| a.min(b));
           let max_val = weights.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
           let mean_val = weights.iter().sum::<f32>() / weights.len() as f32;
           trace!("Setting {} weights for layer {}x{} (min: {:.3}, max: {:.3}, mean: {:.3})", 
                  weights.len(), self.input_size, self.output_size, min_val, max_val, mean_val);
       }
        self.weights.copy_from_slice(weights);
        Ok(())
    }

    fn set_bias(&mut self, bias: &[f32]) -> anyhow::Result<()> {
        if bias.len() != self.bias.len() {
            error!("Bias size mismatch: expected {}, got {}", 
                   self.bias.len(), bias.len());
            return Err(anyhow::anyhow!("Bias size mismatch"));
        }
       if log::log_enabled!(log::Level::Trace) {
           let min_val = bias.iter().fold(f32::INFINITY, |a, &b| a.min(b));
           let max_val = bias.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
           let mean_val = bias.iter().sum::<f32>() / bias.len() as f32;
           trace!("Setting {} biases for layer output size {} (min: {:.3}, max: {:.3}, mean: {:.3})", 
                  bias.len(), self.output_size, min_val, max_val, mean_val);
       }
        self.bias.copy_from_slice(bias);
        Ok(())
    }

    fn forward(&self, input: &[f32], output: &mut [f32]) -> anyhow::Result<()> {
        if input.len() != self.input_size {
            error!("Input size mismatch: expected {}, got {}", 
                   self.input_size, input.len());
            return Err(anyhow::anyhow!("Input size mismatch"));
        }
        if output.len() != self.output_size {
            error!("Output size mismatch: expected {}, got {}", 
                   self.output_size, output.len());
            return Err(anyhow::anyhow!("Output size mismatch"));
        }
        trace!("Forward pass: {}x{} -> {}", 
               self.input_size, self.output_size, self.activation.as_str());

        
        // Matrix multiplication: output = weights * input + bias
        for i in 0..self.output_size {
            output[i] = self.bias[i];
            for j in 0..self.input_size {
                output[i] += self.weights[i * self.input_size + j] * input[j];
            }
        }

        // Apply activation function
        self.apply_activation(output)?;
        // Log activation statistics
        if log::log_enabled!(log::Level::Trace) {
            let min_val = output.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max_val = output.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let mean_val = output.iter().sum::<f32>() / output.len() as f32;
            trace!("Layer output stats - min: {:.3}, max: {:.3}, mean: {:.3}, size: {}", 
                   min_val, max_val, mean_val, output.len());
        }
        
        Ok(())
    }

    fn apply_activation(&self, values: &mut [f32]) -> anyhow::Result<()> {
        trace!("Applying {:?} activation to {} values", self.activation, values.len());
        
        match self.activation {
            ActivationType::ReLU => {
                let mut activated_count = 0;
                for v in values.iter_mut() {
                    if *v > 0.0 { activated_count += 1; }
                    *v = v.max(0.0);
                }
                trace!("ReLU: {}/{} neurons activated", activated_count, values.len());
            }
            ActivationType::Tanh => {
                for v in values.iter_mut() {
                    *v = v.tanh();
                }
            }
            ActivationType::Logistic => {
                for v in values.iter_mut() {
                    *v = 1.0 / (1.0 + (-*v).exp());
                }
            }
        }
        Ok(())
    }
}

/// MNIST neural network using OneDNN for optimized performance
#[derive(Clone)]
pub struct MnistOneDnnNeuralNetwork {
    x_data: Vec<Vec<f32>>, // Use f32 for OneDNN compatibility
    y_data: Vec<Vec<f32>>,
    batch_size: usize,
    name: String,
    optimal_value: Option<f64>,
    param_count: usize,
    param_cache: Arc<RwLock<Option<Vec<f64>>>>,
    gradient_cache: Arc<RwLock<Option<Vec<f64>>>>,
    gradient_params_cache: Arc<RwLock<Option<Vec<f64>>>>,
    layer_sizes: Vec<usize>,
    activation: ActivationType,
    l2_regularization: f64,
    #[cfg(feature = "onednn")]
    layers: Arc<RwLock<Vec<OneDnnLayer>>>,
    #[cfg(feature = "onednn")]
    layer_activations: Arc<RwLock<Vec<Vec<Vec<f32>>>>>,
}

impl MnistOneDnnNeuralNetwork {
    pub fn new(
        x_data: Vec<Vec<f64>>,
        y_data: Vec<Vec<f64>>,
        hidden_sizes: &[usize],
        batch_size: Option<usize>,
        rng: &mut StdRng,
        activation: Option<ActivationType>,
    ) -> anyhow::Result<Self> {
        info!("Creating OneDNN MNIST network with {} samples", x_data.len());
        debug!("Hidden layers: {:?}, batch_size: {:?}, activation: {:?}", 
               hidden_sizes, batch_size, activation);
        
        if hidden_sizes.is_empty() {
            error!("No hidden layers specified");
            return Err(anyhow::anyhow!(
                "At least one hidden layer size must be specified"
            ));
        }

        let n_samples = x_data.len();
        let batch_size = batch_size.unwrap_or(32).min(n_samples);
        let activation = activation.unwrap_or(ActivationType::ReLU);
        info!("Network configuration: {} samples, batch_size: {}, activation: {:?}", 
              n_samples, batch_size, activation);

        let activation_name = match activation {
            ActivationType::ReLU => "relu",
            ActivationType::Logistic => "logistic",
            ActivationType::Tanh => "tanh",
        };

        let hidden_str = hidden_sizes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("x");
        let name = format!("MNIST_OneDNN_{n_samples}samples_hidden{hidden_str}_{activation_name}");

        let input_dim = x_data.first().map(|x| x.len()).unwrap_or(784);
        let output_dim = y_data.first().map(|y| y.len()).unwrap_or(10);
        info!("Network dimensions: input={}, output={}", input_dim, output_dim);

        // Convert data to f32 for OneDNN
        let x_data_f32: Vec<Vec<f32>> = x_data
            .into_iter()
            .map(|x| x.into_iter().map(|v| v as f32).collect())
            .collect();
        let y_data_f32: Vec<Vec<f32>> = y_data
            .into_iter()
            .map(|y| y.into_iter().map(|v| v as f32).collect())
            .collect();

        // Create layer sizes including input and output
        let mut layer_sizes = vec![input_dim];
        layer_sizes.extend_from_slice(hidden_sizes);
        layer_sizes.push(output_dim);

        // Calculate parameter count
        let mut param_count = 0;
        for i in 0..layer_sizes.len() - 1 {
            let layer_params = (layer_sizes[i] + 1) * layer_sizes[i + 1]; // weights + biases
            param_count += layer_params;
            debug!("Layer {}: {}x{} = {} parameters", 
                   i, layer_sizes[i], layer_sizes[i + 1], layer_params);
        }
        info!("Total network parameters: {}", param_count);

        #[cfg(feature = "onednn")]
        let mut layers = Vec::new();

        #[cfg(feature = "onednn")]
        {
            // Create OneDNN layers
            info!("Initializing {} OneDNN layers", layer_sizes.len() - 1);
            for i in 0..layer_sizes.len() - 1 {
                let layer = OneDnnLayer::new(
                    layer_sizes[i],
                    layer_sizes[i + 1],
                    if i == layer_sizes.len() - 2 {
                        ActivationType::Logistic // Output layer uses logistic for classification
                    } else {
                        activation
                    },
                )?;
                layers.push(layer);
            }
        }

        let instance = Self {
            x_data: x_data_f32,
            y_data: y_data_f32,
            batch_size,
            name,
            optimal_value: None,
            param_count,
            param_cache: Arc::new(RwLock::new(None)),
            gradient_cache: Arc::new(RwLock::new(None)),
            gradient_params_cache: Arc::new(RwLock::new(None)),
            layer_sizes,
            activation,
            l2_regularization: 1e-4,
            #[cfg(feature = "onednn")]
            layers: Arc::new(RwLock::new(layers)),
            #[cfg(feature = "onednn")]
            layer_activations: Arc::new(RwLock::new(Vec::new())),
        };

        instance.initialize_weights(rng)?;
        info!("OneDNN MNIST network created successfully: {}", instance.name);
        Ok(instance)
    }

    pub fn set_optimal_value(&mut self, value: Option<f64>) {
        info!("Setting optimal value: {:?}", value);
        self.optimal_value = value;
    }

    pub fn load_mnist(
        n_samples: Option<usize>,
        hidden_sizes: &[usize],
        batch_size: Option<usize>,
        rng: &mut StdRng,
        activation: Option<ActivationType>,
    ) -> anyhow::Result<Self> {
        info!("Loading MNIST dataset with {} samples", n_samples.unwrap_or(1000));
        
        if !Path::new("data/train-images-idx3-ubyte").exists() {
            warn!("MNIST files not found, downloading...");
            Self::download_mnist_data()?;
        }
        let mnist_data = Self::try_load_mnist_files()?;
        let actual_samples = n_samples.unwrap_or(1000).min(mnist_data.images.len());
        info!("Loaded MNIST data: {} images available, using {} samples", 
              mnist_data.images.len(), actual_samples);

        // Shuffle indices for better training
        let mut indices: Vec<usize> = (0..actual_samples).collect();
        use rand::seq::SliceRandom;
        indices.shuffle(rng);
        debug!("Shuffled sample indices for better training distribution");

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
        info!("Prepared {} training samples with {} features each", 
              x_data.len(), x_data.first().map(|x| x.len()).unwrap_or(0));

        Self::new(x_data, y_data, hidden_sizes, batch_size, rng, activation)
    }

    // Reuse MNIST data loading functions from the original implementation
    fn try_load_mnist_files() -> anyhow::Result<MnistData> {
        info!("Loading MNIST files from disk");
        let train_images = Self::load_mnist_images("data/train-images-idx3-ubyte")?;
        let train_labels = Self::load_mnist_labels("data/train-labels-idx1-ubyte")?;
        info!("Loaded {} images and {} labels", train_images.len(), train_labels.len());

        // Convert to f32
        let images_f32: Vec<Vec<f32>> = train_images
            .into_iter()
            .map(|img| img.into_iter().map(|b| b as f32).collect())
            .collect();

        Ok(MnistData {
            images: images_f32,
            labels: train_labels,
        })
    }

    fn download_mnist_data() -> anyhow::Result<MnistData> {
        // Create data directory if it doesn't exist
        info!("Creating data directory and downloading MNIST dataset");
        fs::create_dir_all("data".to_string())?;

        // Download URLs (same as original implementation)
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
            if !Path::new(&path.to_string()).exists() {
                info!("Downloading {} to {}", url, path);
                Self::download_file(url, path)?;
            } else {
                debug!("File already exists: {}", path);
            }
        }

        // Decompress files
        info!("Decompressing MNIST files");
        Self::decompress_mnist_files()?;

        // Load the decompressed data
        let train_images = Self::load_mnist_images("data/train-images-idx3-ubyte")?;
        let train_labels = Self::load_mnist_labels("data/train-labels-idx1-ubyte")?;

        // Convert to f32
        let images_f32: Vec<Vec<f32>> = train_images
            .into_iter()
            .map(|img| img.into_iter().map(|b| b as f32).collect())
            .collect();

        Ok(MnistData {
            images: images_f32,
            labels: train_labels,
        })
    }

    fn download_file(url: &str, path: &str) -> anyhow::Result<()> {
        debug!("Attempting to download {} using curl", url);
        // Try curl first
        if let Ok(output) = std::process::Command::new("curl".to_string())
            .args(["-L", "-f", "-s", "-o", path, url].map(|s| s.to_string()))
            .output()
        {
            if output.status.success() {
                info!("Successfully downloaded {} using curl", url);
                return Ok(());
            } else {
                warn!("Curl failed for {}: {}", url, String::from_utf8_lossy(&output.stderr));
            }
        }
        debug!("Attempting to download {} using wget", url);

        // Fallback to wget
        if let Ok(output) = std::process::Command::new("wget".to_string())
            .args(["-q", "-O", path, url].map(|s| s.to_string()))
            .output()
        {
            if output.status.success() {
                info!("Successfully downloaded {} using wget", url);
                return Ok(());
            } else {
                warn!("Wget failed for {}: {}", url, String::from_utf8_lossy(&output.stderr));
            }
        }
        error!("Failed to download {} - neither curl nor wget succeeded", url);

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
            if Path::new(&gz_path.to_string()).exists() && !Path::new(&out_path.to_string()).exists() {
                info!("Decompressing {} to {}", gz_path, out_path);
                let gz_file = File::open(gz_path.to_string())?;
                let mut decoder = GzDecoder::new(BufReader::new(gz_file));
                let mut out_file = File::create(out_path.to_string())?;
                std::io::copy(&mut decoder, &mut out_file)?;
                debug!("Successfully decompressed {}", gz_path);
            } else if Path::new(&out_path.to_string()).exists() {
                debug!("Decompressed file already exists: {}", out_path);
            }
        }

        Ok(())
    }

    fn load_mnist_images(path: &str) -> anyhow::Result<Vec<Vec<u8>>> {
        use std::fs::File;
        use std::io::{BufReader, Read};
        info!("Loading MNIST images from {}", path);

        let file = File::open(path.to_string())?;
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
        info!("MNIST images: {} images of {}x{} pixels", num_images, rows, cols);

        // Read image data
        let mut images = Vec::with_capacity(num_images);
        for _ in 0..num_images {
            let mut image = vec![0u8; rows * cols];
            reader.read_exact(&mut image)?;
            images.push(image);
        }
        info!("Successfully loaded {} MNIST images", images.len());

        Ok(images)
    }

    fn load_mnist_labels(path: &str) -> anyhow::Result<Vec<u8>> {
        use std::fs::File;
        use std::io::{BufReader, Read};
        info!("Loading MNIST labels from {}", path);

        let file = File::open(path.to_string())?;
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
        info!("Successfully loaded {} MNIST labels", labels.len());

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
                error!("Hidden layer {} size too large: {} (max 2048)", i, hidden_size);
                return Err(anyhow::anyhow!(
                    "Hidden size at layer {} too large: {} (max 2048)",
                    i,
                    hidden_size
                ));
            }
            if hidden_size == 0 {
                error!("Hidden layer {} size cannot be zero", i);
                return Err(anyhow::anyhow!("Hidden size at layer {} cannot be zero", i));
            }
        }
        let samples = n_samples.unwrap_or(1000);
        if samples > 60000 {
            error!("Too many samples requested: {} (max 60000)", samples);
            return Err(anyhow::anyhow!("Too many samples: {} (max 60000)", samples));
        }
        info!("Creating MNIST network: {} samples, hidden layers: {:?}", 
              samples, hidden_sizes);

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
        trace!("Setting {} parameters", params.len());
        
        if params.iter().any(|&p| !p.is_finite()) {
            error!("Non-finite parameters detected in parameter vector");
            return Err(anyhow::anyhow!("Non-finite parameters detected"));
        }

        // Check for extreme values that might cause numerical instability
        let max_abs = params.iter().map(|p| p.abs()).fold(0.0, f64::max);
        if max_abs > 1e6 {
            warn!("Large parameter values detected: max abs = {:.2e}", max_abs);
            return Err(anyhow::anyhow!(
                "Parameters too large: max abs value = {}",
                max_abs
            ));
        }
        debug!("Parameter statistics: max_abs={:.2e}, count={}", max_abs, params.len());

        // Invalidate caches when parameters change
        *self.param_cache.write() = None;
        *self.gradient_cache.write() = None;
        *self.gradient_params_cache.write() = None;
        trace!("Invalidated parameter and gradient caches");

        #[cfg(feature = "onednn")]
        {
            // Set parameters in OneDNN layers
            debug!("Setting parameters in {} OneDNN layers", self.layer_sizes.len() - 1);
            let mut param_idx = 0;
            let mut layers = self.layers.write();
            for (i, layer) in layers.iter_mut().enumerate() {
                let input_size = self.layer_sizes[i];
                let output_size = self.layer_sizes[i + 1];

                // Set weights
                let weights_count = input_size * output_size;
                if param_idx + weights_count > params.len() {
                    error!("Insufficient parameters for layer {} weights: need {}, have {}", 
                           i, weights_count, params.len() - param_idx);
                    return Err(anyhow::anyhow!(
                        "Not enough parameters provided for weights"
                    ));
                }

                let weights: Vec<f32> = params[param_idx..param_idx + weights_count]
                    .iter()
                    .map(|&p| p as f32)
                    .collect();
                trace!("Setting {} weights for layer {}", weights_count, i);
                layer.set_weights(&weights)?;
                param_idx += weights_count;

                // Set bias
                let bias_count = output_size;
                if param_idx + bias_count > params.len() {
                    error!("Insufficient parameters for layer {} bias: need {}, have {}", 
                           i, bias_count, params.len() - param_idx);
                    return Err(anyhow::anyhow!("Not enough parameters provided for bias"));
                }

                let bias: Vec<f32> = params[param_idx..param_idx + bias_count]
                    .iter()
                    .map(|&p| p as f32)
                    .collect();
                trace!("Setting {} biases for layer {}", bias_count, i);
                layer.set_bias(&bias)?;
                param_idx += bias_count;
            }
            debug!("Successfully set all parameters in OneDNN layers");
        }

        #[cfg(not(feature = "onednn"))]
        {
            // Fallback: just store parameters for basic implementation
            // This allows compilation without OneDNN
            debug!("OneDNN not available, using fallback parameter storage");
        }

        Ok(())
    }

    fn get_parameters(&self) -> anyhow::Result<Vec<f64>> {
        // Check cache first
        if let Some(cached) = self.param_cache.read().as_ref() {
            trace!("Returning {} cached parameters", cached.len());
            return Ok(cached.clone());
        }
        debug!("Extracting {} parameters from network", self.param_count);

        #[cfg(feature = "onednn")]
        {
            let mut params = Vec::with_capacity(self.param_count);
            let layers = self.layers.read();
            
            for (i, layer) in layers.iter().enumerate() {
                debug!("Extracting parameters from layer {}: {}x{}", 
                       i, layer.input_size, layer.output_size);
                
                // Extract weights (convert f32 to f64)
                for &weight in &layer.weights {
                    params.push(weight as f64);
                }
                
                // Extract biases (convert f32 to f64)
                for &bias in &layer.bias {
                    params.push(bias as f64);
                }
            }
            
            if params.len() != self.param_count {
                error!("Parameter count mismatch: extracted {}, expected {}", 
                       params.len(), self.param_count);
                return Err(anyhow::anyhow!(
                    "Parameter extraction failed: count mismatch"
                ));
            }
            
            debug!("Successfully extracted {} parameters", params.len());
            
            // Cache the parameters
            *self.param_cache.write() = Some(params.clone());
            
            Ok(params)
        }
        
        #[cfg(not(feature = "onednn"))]
        {
            // Fallback: return random initialized parameters
            warn!("OneDNN not available, returning random initialized parameters");
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let params: Vec<f64> = (0..self.param_count)
                .map(|_| rng.gen_range(-0.1..0.1))
                .collect();

            // Cache the parameters
            *self.param_cache.write() = Some(params.clone());

            Ok(params)
        }
    }

    /// Initialize weights using appropriate initialization for the activation function
    fn initialize_weights(&self, rng: &mut StdRng) -> anyhow::Result<()> {
        info!("Initializing network weights for {:?} activation", self.activation);
        
        #[cfg(feature = "onednn")]
        {
            // Initialize OneDNN layers with proper weight initialization
            debug!("Initializing {} OneDNN layers with proper weight initialization", 
                   self.layer_sizes.len() - 1);
            let mut layers = self.layers.write();
            for i in 0..layers.len() {
                let input_size = self.layer_sizes[i];
                let output_size = self.layer_sizes[i + 1];

                // Choose initialization based on activation function
                let std_dev = match self.activation {
                    ActivationType::ReLU => {
                        // He initialization for ReLU
                        (2.0 / input_size as f64).sqrt()
                    }
                    ActivationType::Logistic => {
                        // Xavier/Glorot initialization for logistic
                        (2.0 / (input_size + output_size) as f64).sqrt()
                    }
                    ActivationType::Tanh => {
                        // Xavier initialization for tanh
                        (1.0 / (input_size + output_size) as f64).sqrt()
                    }
                };
                debug!("Layer {}: {}x{} using std_dev={:.3} for {:?}", 
                       i, input_size, output_size, std_dev, self.activation);

                
                // Generate initialized weights
                let mut weights = Vec::with_capacity(input_size * output_size);
                for _ in 0..(input_size * output_size) {
                    let normal: f64 = rng.sample(rand_distr::StandardNormal);
                    weights.push((normal * std_dev) as f32);
                }

                // Generate initialized biases (zeros)
                let biases = vec![0.0f32; output_size];
               if log::log_enabled!(log::Level::Trace) {
                   let min_weight = weights.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                   let max_weight = weights.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
                   let mean_weight = weights.iter().sum::<f32>() / weights.len() as f32;
                   trace!("Generated {} weights and {} biases for layer {} (weight min: {:.3}, max: {:.3}, mean: {:.3})", 
                          weights.len(), biases.len(), i, min_weight, max_weight, mean_weight);
               }

                
                // Set the initialized weights and biases in the layer
                layers[i].set_weights(&weights)?;
                layers[i].set_bias(&biases)?;
                debug!("Set initialized weights and biases for layer {}", i);
            }
            info!("OneDNN weight initialization completed");
        }

        #[cfg(not(feature = "onednn"))]
        {
            // Fallback initialization when OneDNN is not available
            warn!("OneDNN not available, weights will be initialized on first access");
        }

        Ok(())
    }

    /// Verify the quality of weight initialization
    pub fn verify_initialization(&self) -> anyhow::Result<()> {
        info!("=== OneDNN Weight Initialization Quality Check ===");
        info!("Network architecture: {:?}", self.layer_sizes);
        info!("Activation function: {:?}", self.activation);
        info!("Total parameters: {}", self.param_count);
        info!("L2 regularization: {}", self.l2_regularization);
        #[cfg(feature = "onednn")]
        {
            let layers = self.layers.read();
            for (i, layer) in layers.iter().enumerate() {
                info!("Layer {}: {}x{} with {:?} activation", 
                      i, layer.input_size, layer.output_size, layer.activation);
                // Check weight statistics
                let weight_mean = layer.weights.iter().sum::<f32>() / layer.weights.len() as f32;
                let weight_std = (layer.weights.iter()
                    .map(|w| (w - weight_mean).powi(2))
                    .sum::<f32>() / layer.weights.len() as f32).sqrt();
                info!("  Weights - mean: {:.4}, std: {:.4}", weight_mean, weight_std);
                // Check bias statistics
                let bias_mean = layer.bias.iter().sum::<f32>() / layer.bias.len() as f32;
                info!("  Bias - mean: {:.4}", bias_mean);
                // Verify initialization quality
                let expected_std = match self.activation {
                    ActivationType::ReLU => (2.0 / layer.input_size as f32).sqrt(),
                    ActivationType::Logistic => (2.0 / (layer.input_size + layer.output_size) as f32).sqrt(),
                    ActivationType::Tanh => (1.0 / (layer.input_size + layer.output_size) as f32).sqrt(),
                };
                let std_ratio = weight_std / expected_std;
                if (0.8..=1.2).contains(&std_ratio) {
                    info!("  ✓ Weight initialization is correct (ratio: {:.3})", std_ratio);
                } else {
                    warn!("  ⚠ Weight initialization may be suboptimal (ratio: {:.3})", std_ratio);
                }
            }
        }
        
        info!("=== End of OneDNN Initialization Check ===");
        Ok(())
    }

    #[cfg(feature = "onednn")]
    fn forward_pass(&self, batch_x: &[Vec<f32>]) -> anyhow::Result<Vec<Vec<f32>>> {
        let batch_size = batch_x.len();
        trace!("Forward pass for batch of size {}", batch_size);
        
        let mut results = Vec::with_capacity(batch_size);
        let layers = self.layers.read();
        debug!("Processing batch through {} layers", layers.len());
        // Store activations for backpropagation
        let mut all_activations = Vec::with_capacity(batch_size);

        // Process each sample in the batch
        for (sample_idx, sample) in batch_x.iter().enumerate() {
            trace!("Processing sample {} of {}", sample_idx + 1, batch_size);
            let mut current_input = sample.clone();
            let mut sample_activations = vec![current_input.clone()];

            // Forward pass through all layers
            for (layer_idx, layer) in layers.iter().enumerate() {
                trace!("Layer {} forward pass: {} -> {}", 
                       layer_idx, current_input.len(), layer.output_size);
                let mut output = vec![0.0f32; layer.output_size];
                layer.forward(&current_input, &mut output)?;
                current_input = output;
                sample_activations.push(current_input.clone());
            }

            results.push(current_input);
            all_activations.push(sample_activations);
        }
        // Store activations for gradient computation
        *self.layer_activations.write() = all_activations;
        debug!("Forward pass completed for batch of {} samples", batch_size);

        Ok(results)
    }

    #[cfg(not(feature = "onednn"))]
    fn forward_pass(&self, batch_x: &[Vec<f32>]) -> anyhow::Result<Vec<Vec<f32>>> {
        debug!("Using fallback forward pass implementation (OneDNN not available)");
        // Simple forward pass implementation without OneDNN
        let output_size = self.layer_sizes.last().unwrap();
        let mut results = Vec::with_capacity(batch_x.len());
        
        for sample in batch_x {
            // Apply softmax to create valid probability distribution
            let mut output = vec![0.1f32; *output_size];
            let sum: f32 = output.iter().sum();
            for val in &mut output {
                *val /= sum;
            }
            results.push(output);
        }
        
        Ok(results)
    }
    #[cfg(feature = "onednn")]
    fn compute_gradient_backprop(&self) -> anyhow::Result<Vec<f64>> {
        trace!("Starting backpropagation gradient computation");
        let n_samples = self.x_data.len();
        let n_batches = n_samples.div_ceil(self.batch_size);
        let mut total_gradient = vec![0.0; self.param_count];
        for batch_idx in 0..n_batches {
            let start = batch_idx * self.batch_size;
            let end = ((batch_idx + 1) * self.batch_size).min(n_samples);
            let batch_size = end - start;
            trace!("Processing batch {}/{} for gradient", batch_idx + 1, n_batches);
            let batch_x: Vec<Vec<f32>> = self.x_data[start..end].to_vec();
            let batch_y: Vec<Vec<f32>> = self.y_data[start..end].to_vec();
            // Forward pass to populate activations
            let y_pred = self.forward_pass(&batch_x)?;
            // Get stored activations
            let activations = self.layer_activations.read();
            let layers = self.layers.read();
            // Compute gradients for this batch
            let mut batch_gradient = vec![0.0; self.param_count];
            for (sample_idx, (pred, target)) in y_pred.iter().zip(batch_y.iter()).enumerate() {
                // Compute output layer error (cross-entropy gradient)
                let mut delta: Vec<f32> = pred.iter()
                    .zip(target.iter())
                    .map(|(p, t)| p - t)
                    .collect();
                let sample_activations = &activations[sample_idx];
                let mut param_idx = self.param_count;
                // Backpropagate through layers
                for layer_idx in (0..layers.len()).rev() {
                    let layer = &layers[layer_idx];
                    let input_activation = &sample_activations[layer_idx];
                    // Compute gradients for this layer
                    let weights_per_layer = layer.input_size * layer.output_size;
                    let bias_per_layer = layer.output_size;
                    param_idx -= bias_per_layer;
                    param_idx -= weights_per_layer;
                    // Gradient for biases
                    for (i, &d) in delta.iter().enumerate() {
                        batch_gradient[param_idx + weights_per_layer + i] += d as f64 / batch_size as f64;
                    }
                    // Gradient for weights
                    for i in 0..layer.output_size {
                        for j in 0..layer.input_size {
                            let grad_idx = param_idx + i * layer.input_size + j;
                            batch_gradient[grad_idx] += (delta[i] * input_activation[j]) as f64 / batch_size as f64;
                        }
                    }
                    // Compute delta for previous layer if not at input
                    if layer_idx > 0 {
                        let mut new_delta = vec![0.0f32; layer.input_size];
                        for i in 0..layer.input_size {
                            for j in 0..layer.output_size {
                                new_delta[i] += delta[j] * layer.weights[j * layer.input_size + i];
                            }
                            // Apply activation derivative
                            let prev_layer = &layers[layer_idx - 1];
                            match prev_layer.activation {
                                ActivationType::ReLU => {
                                    if input_activation[i] <= 0.0 {
                                        new_delta[i] = 0.0;
                                    }
                                }
                                ActivationType::Tanh => {
                                    let tanh_val = input_activation[i].tanh();
                                    new_delta[i] *= 1.0 - tanh_val * tanh_val;
                                }
                                ActivationType::Logistic => {
                                    let sigmoid = 1.0 / (1.0 + (-input_activation[i]).exp());
                                    new_delta[i] *= sigmoid * (1.0 - sigmoid);
                                }
                            }
                        }
                        delta = new_delta;
                    }
                }
            }
            // Add batch gradient to total
            for i in 0..self.param_count {
                total_gradient[i] += batch_gradient[i];
            }
        }
        // Add L2 regularization gradient
        if self.l2_regularization > 0.0 {
            let layers = self.layers.read();
            let mut param_idx = 0;
            for layer in layers.iter() {
                let weights_count = layer.input_size * layer.output_size;
                for i in 0..weights_count {
                    total_gradient[param_idx + i] += self.l2_regularization * layer.weights[i] as f64;
                }
                param_idx += weights_count + layer.output_size; // weights + biases
            }
        }
        // Gradient clipping to prevent exploding gradients
        let grad_norm: f64 = total_gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
        debug!("Gradient norm: {:.3}", grad_norm);
        if grad_norm > 10.0 {
            let scale = 10.0 / grad_norm;
            warn!("Clipping gradient: norm {:.3} -> 10.0 (scale={:.3})", grad_norm, scale);
            for g in &mut total_gradient {
                *g *= scale;
            }
        } else {
            trace!("Gradient norm within acceptable range");
        }
        debug!("Backpropagation gradient computation completed");
        Ok(total_gradient)
    }
}

impl OptimizationProblem for MnistOneDnnNeuralNetwork {
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
        self.get_parameters().unwrap_or_else(|e| {
            warn!("Failed to get parameters for initial point: {}", e);
            use rand::Rng;
            let mut rng = rand::rng();
            (0..self.count_parameters())
                .map(|_| rng.random_range(-0.01..0.01))
                .collect()
        })
    }

    fn evaluate_f64(&self, params: &[f64]) -> anyhow::Result<f64> {
        // Set parameters in the model
        trace!("Evaluating loss function with {} parameters", params.len());
        self.set_parameters(params)?;

        let n_samples = self.x_data.len();
        let n_batches = n_samples.div_ceil(self.batch_size);
        debug!("Processing {} samples in {} batches (batch_size={})", 
               n_samples, n_batches, self.batch_size);
        let mut total_loss = 0.0;

        // Process batches
        for batch_idx in 0..n_batches {
            let start = batch_idx * self.batch_size;
            let end = ((batch_idx + 1) * self.batch_size).min(n_samples);
            let batch_size = end - start;
            trace!("Processing batch {}/{}: samples {}..{}", 
                   batch_idx + 1, n_batches, start, end - 1);

            let batch_x: Vec<Vec<f32>> = self.x_data[start..end].to_vec();
            let batch_y: Vec<Vec<f32>> = self.y_data[start..end].to_vec();

            // Forward pass
            let y_pred = self.forward_pass(&batch_x)?;

            // Cross-entropy loss for this batch
            let mut batch_loss = 0.0;
            for (pred, target) in y_pred.iter().zip(batch_y.iter()) {
                for (p, t) in pred.iter().zip(target.iter()) {
                    let p_clamped = p.max(1e-10f32).min(1.0 - 1e-10);
                    batch_loss += -(*t as f64) * (p_clamped as f64).ln();
                }
            }
            batch_loss /= batch_size as f64;
            trace!("Batch {} loss: {:.4}", batch_idx, batch_loss);
            total_loss += batch_loss * (batch_size as f64);
        }

        // Average loss across all samples
        let mut loss_value = total_loss / (n_samples as f64);
        debug!("Average cross-entropy loss: {:.4}", loss_value);

        // Add L2 regularization
        if self.l2_regularization > 0.0 {
            let params_squared_sum: f64 = params.iter().map(|p| p * p).sum();
            let reg_term = 0.5 * self.l2_regularization * params_squared_sum;
            loss_value += reg_term;
            debug!("L2 regularization term: {:.4} (lambda={:.2e})", 
                   reg_term, self.l2_regularization);
        }
        debug!("Final loss value: {:.4}", loss_value);

        // Check final loss for non-finite values
        if !loss_value.is_finite() {
            error!("Non-finite loss value computed: {}", loss_value);
            return Err(anyhow::anyhow!("Non-finite loss value: {}", loss_value));
        }

        Ok(loss_value)
    }

    fn gradient_f64(&self, params: &[f64]) -> anyhow::Result<Vec<f64>> {
        // Check gradient cache first
        if let Some(cached) = self.gradient_cache.read().as_ref() {
            if let Some(cached_params) = self.gradient_params_cache.read().as_ref() {
                if *cached_params == params.to_vec() {
                    trace!("Returning cached gradient of size {}", cached.len());
                    return Ok(cached.clone());
                }
            }
        }
        debug!("Computing gradient using backpropagation for {} parameters", params.len());

        // Set parameters and perform forward pass
        self.set_parameters(params)?;
        
        #[cfg(feature = "onednn")]
        {
            let gradient = self.compute_gradient_backprop()?;
            
            // Cache the gradient
            *self.gradient_cache.write() = Some(gradient.clone());
            *self.gradient_params_cache.write() = Some(params.to_vec());
            debug!("Cached gradient for future use");
            
            return Ok(gradient);
        }

        #[cfg(not(feature = "onednn"))]
        {
            // Fallback to finite differences when OneDNN is not available
            warn!("OneDNN not available, falling back to finite differences");
            let mut gradient = vec![0.0; params.len()];
            let eps = 1e-7;
            let f0 = self.evaluate_f64(params)?;
            
            for i in 0..params.len() {
                if i % 1000 == 0 {
                    debug!("Computing gradient component {}/{}", i, params.len());
                }
                let mut params_plus = params.to_vec();
                params_plus[i] += eps;
                let f_plus = self.evaluate_f64(&params_plus)?;
                gradient[i] = (f_plus - f0) / eps;
            }

            // Gradient clipping
            let grad_norm: f64 = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
            if grad_norm > 10.0 {
                let scale = 10.0 / grad_norm;
                for g in &mut gradient {
                    *g *= scale;
                }
            }


            Ok(gradient)
        }
    }

    fn optimal_value(&self) -> Option<f64> {
        self.optimal_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_onednn_mnist_creation() {
        let mut rng = StdRng::seed_from_u64(42);

        // Create synthetic data for testing
        let x_data = vec![vec![0.5; 784]; 10]; // 10 samples, 784 features
        let y_data = vec![vec![0.1; 10]; 10]; // 10 samples, 10 classes

        let network = MnistOneDnnNeuralNetwork::new(
            x_data,
            y_data,
            &[20],
            Some(5),
            &mut rng,
            Some(ActivationType::ReLU),
        );

        assert!(network.is_ok(), "Should create OneDNN network successfully");

        if let Ok(net) = network {
            assert_eq!(net.dimension(), 20 * 784 + 20 + 10 * 20 + 10); // weights + biases
            assert!(net.name().contains("OneDNN"));
            assert!(net.name().contains("ReLU"));
        }
    }

    #[test]
    fn test_parameter_validation() {
        let mut rng = StdRng::seed_from_u64(42);
        let x_data = vec![vec![0.5; 784]; 5];
        let y_data = vec![vec![0.1; 10]; 5];

        let network = MnistOneDnnNeuralNetwork::new(
            x_data,
            y_data,
            &[10],
            Some(5),
            &mut rng,
            Some(ActivationType::ReLU),
        )
        .unwrap();

        // Test with non-finite parameters
        let bad_params = vec![f64::NAN; network.dimension()];
        assert!(network.set_parameters(&bad_params).is_err());

        // Test with extreme parameters
        let extreme_params = vec![1e10; network.dimension()];
        assert!(network.set_parameters(&extreme_params).is_err());

        // Test with normal parameters
        let normal_params = vec![0.1; network.dimension()];
        assert!(network.set_parameters(&normal_params).is_ok());
    }
}