use crate::OptimizationProblem;
use candle_core::{Device, Tensor};
use candle_nn::ops::softmax;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct MnistData {
    images: Vec<Vec<u8>>,
    labels: Vec<u8>,
}

/// MNIST-like neural network training problem
#[derive(Clone)]
pub struct MnistNeuralNetwork {
    x_tensor: Tensor, // 28x28 = 784 features
    y_tensor: Tensor, // 10 classes (one-hot encoded)
    device: Device,
    hidden_size: usize,
    name: String,
}

impl MnistNeuralNetwork {
    pub fn new(x_data: Vec<Vec<f64>>, y_data: Vec<Vec<f64>>, hidden_size: usize) -> anyhow::Result<Self> {
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

        Ok(Self {
            x_tensor,
            y_tensor,
            device,
            hidden_size,
            name,
        })
    }
    /// Load real MNIST data
    pub fn load_mnist(n_samples: Option<usize>, hidden_size: usize) -> anyhow::Result<Self> {
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

        Self::new(x_data, y_data, hidden_size)
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
            ("https://raw.githubusercontent.com/fgnt/mnist/master/train-images-idx3-ubyte.gz", "data/train-images-idx3-ubyte.gz"),
            ("https://raw.githubusercontent.com/fgnt/mnist/master/train-labels-idx1-ubyte.gz", "data/train-labels-idx1-ubyte.gz"),
            ("https://raw.githubusercontent.com/fgnt/mnist/master/t10k-images-idx3-ubyte.gz", "data/t10k-images-idx3-ubyte.gz"),
            ("https://raw.githubusercontent.com/fgnt/mnist/master/t10k-labels-idx1-ubyte.gz", "data/t10k-labels-idx1-ubyte.gz"),
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
                    _ => Err(anyhow::anyhow!("Failed to download {} - neither curl nor wget available", url))
                }
            }
        }
    }

    fn decompress_mnist_files() -> anyhow::Result<()> {
        use flate2::read::GzDecoder;
        use std::fs::File;
        use std::io::BufReader;

        let files = [
            ("data/train-images-idx3-ubyte.gz", "data/train-images-idx3-ubyte"),
            ("data/train-labels-idx1-ubyte.gz", "data/train-labels-idx1-ubyte"),
            ("data/t10k-images-idx3-ubyte.gz", "data/t10k-images-idx3-ubyte"),
            ("data/t10k-labels-idx1-ubyte.gz", "data/t10k-labels-idx1-ubyte"),
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
    pub fn create(n_samples: Option<usize>, hidden_size: usize) -> anyhow::Result<Self> {
        // Validate hidden size to prevent overflow
        if hidden_size > 1000 {
            return Err(anyhow::anyhow!("Hidden size too large: {} (max 1000)", hidden_size));
        }
        let samples = n_samples.unwrap_or(1000);
        if samples > 10000 {
            return Err(anyhow::anyhow!("Too many samples: {} (max 10000)", samples));
        }

        // Try to load real MNIST data first
        Self::load_mnist(Some(samples), hidden_size)
    }

    fn count_parameters(&self) -> usize {
        let input_size = 784;
        let hidden_size = self.hidden_size;
        let output_size = 10;
        // Weights: input->hidden + hidden->output
        // Biases: hidden + output
        input_size * hidden_size + hidden_size + hidden_size * output_size + output_size
    }
    fn forward_pass(&self, params: &[f64]) -> anyhow::Result<Tensor> {
        let input_size = 784;
        let hidden_size = self.hidden_size;
        let output_size = 10;
        let mut param_idx = 0;
        // Extract input-to-hidden weights and biases
        let w1_size = input_size * hidden_size;
        let w1_data = &params[param_idx..param_idx + w1_size];
        param_idx += w1_size;
        let b1_data = &params[param_idx..param_idx + hidden_size];
        param_idx += hidden_size;
        // Extract hidden-to-output weights and biases
        let w2_size = hidden_size * output_size;
        let w2_data = &params[param_idx..param_idx + w2_size];
        param_idx += w2_size;
        let b2_data = &params[param_idx..param_idx + output_size];
        // Create weight and bias tensors
        let w1 = Tensor::from_vec(w1_data.to_vec(), (input_size, hidden_size), &self.device)?;
        let b1 = Tensor::from_vec(b1_data.to_vec(), hidden_size, &self.device)?;
        let w2 = Tensor::from_vec(w2_data.to_vec(), (hidden_size, output_size), &self.device)?;
        let b2 = Tensor::from_vec(b2_data.to_vec(), output_size, &self.device)?;
        // Forward pass: input -> hidden -> output
        let hidden = self.x_tensor.matmul(&w1)?.broadcast_add(&b1)?.relu()?;
        let output = hidden.matmul(&w2)?.broadcast_add(&b2)?;
        // Apply softmax for classification
        let output = softmax(&output, 1)?;
        Ok(output)
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
        use rand::Rng;
        let mut rng = rand::rng();
        // Xavier initialization
        let input_size = 784;
        let hidden_size = self.hidden_size;
        let output_size = 10;
        let mut params = Vec::new();
        // Input to hidden weights
        let xavier_1 = (6.0 / (input_size + hidden_size) as f64).sqrt();
        for _ in 0..(input_size * hidden_size) {
            params.push(rng.random_range(-xavier_1..xavier_1));
        }
        // Hidden biases
        for _ in 0..hidden_size {
            params.push(0.0);
        }
        // Hidden to output weights
        let xavier_2 = (6.0 / (hidden_size + output_size) as f64).sqrt();
        for _ in 0..(hidden_size * output_size) {
            params.push(rng.random_range(-xavier_2..xavier_2));
        }
        // Output biases
        for _ in 0..output_size {
            params.push(0.0);
        }
        params
    }

    fn evaluate_f64(&self, params: &[f64]) -> anyhow::Result<f64> {
        let y_pred = self.forward_pass(params)?;

        // Cross-entropy loss
        let log_probs = y_pred.log()?;
        let loss = (&self.y_tensor * &log_probs)?.sum_keepdim(1)?.mean_all()?.neg()?;

        Ok(loss.to_scalar::<f64>()?)
    }
    fn gradient_f64(&self, params: &[f64]) -> anyhow::Result<Vec<f64>> {
        // Use finite differences for gradient computation
        // In production, you'd implement proper backpropagation with Candle's autodiff
        let eps = 1e-5;
        let mut grad = vec![0.0; params.len()];
        let base_loss = self.evaluate_f64(params)?;

        for i in 0..params.len() {
            let mut params_plus = params.to_vec();
            params_plus[i] += eps;
            let loss_plus = self.evaluate_f64(&params_plus)?;
            grad[i] = (loss_plus - base_loss) / eps;
        }

        Ok(grad)
    }
    fn optimal_value(&self) -> Option<f64> {
        // For real MNIST classification, good models achieve around 0.05-0.15 cross-entropy loss
        // For synthetic data, we use a more lenient threshold
        if self.name.contains("MNIST") && self.x_tensor.dim(0).unwrap_or(0) > 500 {
            Some(0.2) // Real MNIST data threshold
        } else {
            Some(0.5) // Synthetic data threshold
        }
    }
}