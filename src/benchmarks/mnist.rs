#![allow(clippy::upper_case_acronyms)]
use rand::prelude::StdRng;
use std::fs;
use std::path::Path;

#[derive(Debug)]
#[derive(Clone)]
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
