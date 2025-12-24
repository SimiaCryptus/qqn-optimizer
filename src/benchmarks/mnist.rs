#![allow(clippy::upper_case_acronyms)]
use crate::OptimizationProblem;
use luminal::prelude::*;
use luminal_training::Autograd;
use rand::prelude::StdRng;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MnistData {
    images: Vec<Vec<u8>>,
    labels: Vec<u8>,
}

impl MnistData {
    pub fn load_mnist(
        n_samples: Option<usize>,
        rng: &mut StdRng,
    ) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        if !Path::new("data/train-images-idx3-ubyte").exists() {
            println!("MNIST files not found, downloading...");
            Self::download_mnist_data().expect("Failed to download MNIST data");
        }
        let mnist_data = Self::try_load_mnist_files().expect("Failed to load MNIST data");
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
        (x_data, y_data)
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
}
macro_rules! impl_eval_grad {
    () => {
        fn evaluate_f64(&self, x: &[f64]) -> anyhow::Result<f64> {
            if x.len() != self.dimension() {
                return Err(anyhow::anyhow!(
                    "Dimension mismatch: expected {}, got {}",
                    self.dimension(),
                    x.len()
                ));
            }
            let mut graph = Graph::new();
            let input = graph
                .tensor((x.len(),))
                .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
            let output = self.build_graph(&mut graph, input);
            output.retrieve();
            graph.execute();
            let data = output.data();
            if data.is_empty() {
                return Err(anyhow::anyhow!("Graph execution produced no output"));
            }
            Ok(data[0] as f64)
        }
        fn gradient_f64(&self, x: &[f64]) -> anyhow::Result<Vec<f64>> {
            if x.len() != self.dimension() {
                return Err(anyhow::anyhow!(
                    "Dimension mismatch: expected {}, got {}",
                    self.dimension(),
                    x.len()
                ));
            }
            let mut graph = Graph::new();
            let input = graph
                .tensor((x.len(),))
                .set(x.iter().map(|&v| v as f32).collect::<Vec<f32>>());
            let output = self.build_graph(&mut graph, input);
            let grads = graph.compile(Autograd::new(input, output), ());
            graph.keep_tensors(&grads);
            output.retrieve();
            graph.execute();
            if grads.is_empty() {
                return Ok(vec![0.0; x.len()]);
            }
            let (grad_id, grad_shape) = grads[0];
            let grad_tensor = GraphTensor::from_id(grad_id, grad_shape, &mut graph, DType::F32);
            Ok(grad_tensor.data().iter().map(|&v| v as f64).collect())
        }
    };
}
#[derive(Debug, Clone)]
pub struct MnistProblem {
    name: String,
    train_x: Vec<Vec<f64>>,
    train_y: Vec<Vec<f64>>,
    hidden_size: usize,
}

impl MnistProblem {
    pub fn new(n_samples: usize, hidden_size: usize, rng: &mut StdRng) -> Self {
        let (x, y) = MnistData::load_mnist(Some(n_samples), rng);
        Self {
            name: format!("Mnist_MLP_{}samples_{}hidden", n_samples, hidden_size),
            train_x: x,
            train_y: y,
            hidden_size,
        }
    }
}

impl OptimizationProblem for MnistProblem {
    impl_eval_grad!();
    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn dimension(&self) -> usize {
        let n_input = 784;
        let n_output = 10;
        // W1 + B1 + W2 + B2
        (n_input * self.hidden_size) + self.hidden_size + (self.hidden_size * n_output) + n_output
    }
    fn initial_point(&self) -> Vec<f64> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..self.dimension())
            .map(|_| rng.gen_range(-0.1..0.1))
            .collect()
    }
    fn build_graph(&self, graph: &mut Graph, params: GraphTensor) -> GraphTensor {
        let n_input = 784;
        let n_hidden = self.hidden_size;
        let n_output = 10;
        let batch_size = self.train_x.len();
        // Load Data
        let mut x_flat: Vec<f32> = Vec::with_capacity(batch_size * n_input);
        for sample in &self.train_x {
            x_flat.extend(sample.iter().map(|&v| v as f32));
        }
        let x = graph.tensor((batch_size, n_input)).set(x_flat);
        let mut y_flat: Vec<f32> = Vec::with_capacity(batch_size * n_output);
        for sample in &self.train_y {
            y_flat.extend(sample.iter().map(|&v| v as f32));
        }
        let y = graph.tensor((batch_size, n_output)).set(y_flat);
        // Indices for slicing params
        let w1_size = n_input * n_hidden;
        let b1_size = n_hidden;
        let w2_size = n_hidden * n_output;
        let b2_size = n_output;
        let w1_end = w1_size;
        let b1_end = w1_end + b1_size;
        let w2_end = b1_end + w2_size;
        // Helper to extract parameter block
        let mut get_param = |start: usize, size: usize, shape: (usize, usize)| {
            let indices: Vec<f32> = (start..start + size).map(|i| i as f32).collect();
            let idx = graph.tensor((size,)).set(indices);
            params.gather(idx).split_dims(0, shape.1)
        };
        let w1 = get_param(0, w1_size, (n_input, n_hidden));
        let b1 = get_param(w1_end, b1_size, (1, n_hidden));
        let w2 = get_param(b1_end, w2_size, (n_hidden, n_output));
        let b2 = get_param(w2_end, b2_size, (1, n_output));
        // Forward pass
        let h = (x.matmul(w1) + b1).relu();
        let logits = h.matmul(w2) + b2;
        // MSE Loss on Sigmoid probabilities
        let preds = logits.sigmoid();
        let diff = preds - y;
        (diff * diff).mean(vec![0, 1])
    }
    fn optimal_value(&self) -> Option<f64> {
        Some(0.0)
    }
}