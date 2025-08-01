pub mod math;
pub mod timing;

pub use math::{
    clamp_vector, compute_magnitude, dot_product, dot_product_f64, is_finite, norm_inf, norm_l1,
    norm_l2, vector_add, vector_scale, vector_subtract,
};

/// Common mathematical constants
pub mod constants {
    /// Machine epsilon for f64
    pub const EPSILON: f64 = f64::EPSILON;

    /// Square root of machine epsilon
    pub const SQRT_EPSILON: f64 = 1.4901161193847656e-8;

    /// Default tolerance for convergence checks
    pub const DEFAULT_TOLERANCE: f64 = 1e-6;

    /// Maximum safe value for numerical computations
    pub const MAX_SAFE_VALUE: f64 = 1e100;

    /// Minimum safe value for numerical computations
    pub const MIN_SAFE_VALUE: f64 = 1e-100;
    /// Default maximum iterations for optimization
    pub const DEFAULT_MAX_ITERATIONS: usize = 1000;
    /// Default gradient tolerance
    pub const DEFAULT_GRADIENT_TOLERANCE: f64 = 1e-8;
}

/// Utility functions for working with file paths
pub mod paths {
    use std::path::{Path, PathBuf};

    /// Create output directory if it doesn't exist
    pub fn ensure_output_dir(path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// Generate timestamped filename
    pub fn timestamped_filename(base: &str, extension: &str) -> String {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        format!("{base}_{timestamp}.{extension}")
    }

    /// Get results directory path
    pub fn results_dir() -> PathBuf {
        PathBuf::from("results")
    }

    /// Get experiments directory path
    pub fn experiments_dir() -> PathBuf {
        PathBuf::from("experiments")
    }
    /// Create a unique experiment directory with timestamp
    pub fn create_experiment_dir(experiment_name: &str) -> std::io::Result<PathBuf> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let dir_name = format!("{experiment_name}_{timestamp}");
        let path = experiments_dir().join(dir_name);
        ensure_output_dir(&path)?;
        Ok(path)
    }
    /// Get the path for a specific result file
    pub fn result_file_path(filename: &str) -> PathBuf {
        results_dir().join(filename)
    }
}

/// Validation utilities
pub mod validation {
    use crate::optimizers::OptResult;

    /// Validate that a vector contains only finite values
    pub fn validate_finite(values: &[f64]) -> OptResult<()> {
        for (i, &val) in values.iter().enumerate() {
            if !val.is_finite() {
                return Err(crate::optimizers::OptError::InvalidInput(format!(
                    "Non-finite value {val} at index {i}"
                )));
            }
        }
        Ok(())
    }

    /// Validate that a value is within reasonable bounds
    pub fn validate_bounds(value: f64, min: f64, max: f64) -> OptResult<()> {
        if value < min || value > max {
            return Err(crate::optimizers::OptError::InvalidInput(format!(
                "Value {value} outside bounds [{min}, {max}]"
            )));
        }
        Ok(())
    }
    /// Validate that a vector has the expected dimension
    pub fn validate_dimension(values: &[f64], expected: usize) -> OptResult<()> {
        if values.len() != expected {
            return Err(crate::optimizers::OptError::InvalidInput(format!(
                "Expected dimension {expected}, got {}",
                values.len()
            )));
        }
        Ok(())
    }
    /// Validate that a matrix has the expected shape
    pub fn validate_matrix_shape(matrix: &[Vec<f64>], rows: usize, cols: usize) -> OptResult<()> {
        if matrix.len() != rows {
            return Err(crate::optimizers::OptError::InvalidInput(format!(
                "Expected {} rows, got {}",
                rows,
                matrix.len()
            )));
        }
        for (i, row) in matrix.iter().enumerate() {
            if row.len() != cols {
                return Err(crate::optimizers::OptError::InvalidInput(format!(
                    "Row {} has {} columns, expected {}",
                    i,
                    row.len(),
                    cols
                )));
            }
        }
        Ok(())
    }


    /// Validate optimizer configuration
    pub trait ValidateConfig {
        fn validate(&self) -> OptResult<()>;
    }
}


/// Formatting utilities for display
pub mod formatting {
    /// Format a vector for display with limited precision
    pub fn format_vector(v: &[f64], precision: usize) -> String {
        let formatted: Vec<String> = v.iter()
            .map(|x| format!("{:.prec$}", x, prec = precision))
            .collect();
        format!("[{}]", formatted.join(", "))
    }

    /// Format iteration progress
    pub fn format_progress(iter: usize, max_iter: usize, value: f64, gradient_norm: f64) -> String {
        format!(
            "Iteration {}/{}: f = {:.6e}, ||g|| = {:.6e}",
            iter, max_iter, value, gradient_norm
        )
    }

    /// Format convergence status
    pub fn format_convergence(converged: bool, reason: &str, iterations: usize) -> String {
        if converged {
            format!("Converged: {} (iterations: {})", reason, iterations)
        } else {
            format!("Failed to converge: {} (iterations: {})", reason, iterations)
        }
    }
}

/// Statistical utilities
pub mod stats {
    /// Calculate mean of a slice
    pub fn mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }

    /// Calculate standard deviation
    pub fn std_dev(values: &[f64]) -> f64 {
        if values.len() <= 1 {
            return 0.0;
        }
        let m = mean(values);
        let variance = values.iter()
            .map(|x| (x - m).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;
        variance.sqrt()
    }

    /// Calculate min and max values
    pub fn min_max(values: &[f64]) -> Option<(f64, f64)> {
        if values.is_empty() {
            return None;
        }
        let mut min = values[0];
        let mut max = values[0];
        for &v in values.iter().skip(1) {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        Some((min, max))
    }
}

/// Progress reporting utilities
pub mod progress {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Simple progress reporter
    pub struct ProgressReporter {
        current: Arc<AtomicUsize>,
        total: usize,
        name: String,
    }

    impl ProgressReporter {
        /// Create a new progress reporter
        pub fn new(name: impl Into<String>, total: usize) -> Self {
            Self {
                current: Arc::new(AtomicUsize::new(0)),
                total,
                name: name.into(),
            }
        }

        /// Increment progress
        pub fn increment(&self) {
            let current = self.current.fetch_add(1, Ordering::Relaxed) + 1;
            if current % (self.total / 10).max(1) == 0 || current == self.total {
                let percentage = (current as f64 / self.total as f64) * 100.0;
                tracing::info!("{}: {:.1}% ({}/{})", self.name, percentage, current, self.total);
            }
        }

        /// Get current progress
        pub fn current(&self) -> usize {
            self.current.load(Ordering::Relaxed)
        }

        /// Check if complete
        pub fn is_complete(&self) -> bool {
            self.current() >= self.total
        }
    }
}

/// Error recovery utilities
pub mod recovery {
    use crate::optimizers::OptResult;

    /// Attempt to recover from numerical issues by scaling
    pub fn rescale_if_needed(values: &mut [f64], max_value: f64) -> bool {
        let max_abs = values.iter().map(|x| x.abs()).fold(0.0, f64::max);
        if max_abs > max_value {
            let scale = max_value / max_abs;
            for v in values.iter_mut() {
                *v *= scale;
            }
            tracing::warn!("Rescaled values by factor {}", scale);
            return true;
        }
        false
    }

    /// Try to recover from a failed optimization step
    pub fn try_recover<F, T>(mut f: F, max_attempts: usize) -> OptResult<T>
    where
        F: FnMut() -> OptResult<T>,
    {
        for attempt in 1..=max_attempts {
            match f() {
                Ok(result) => return Ok(result),
                Err(e) if attempt < max_attempts => {
                    tracing::warn!("Attempt {} failed: {:?}, retrying...", attempt, e);
                    std::thread::sleep(std::time::Duration::from_millis(100 * attempt as u64));
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }
}

/// Serialization utilities
pub mod serialization {
    use serde::{Deserialize, Serialize};
    use std::path::Path;

    /// Save data to JSON file
    pub fn save_json<T: Serialize>(data: &T, path: &Path) -> std::io::Result<()> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    /// Load data from JSON file
    pub fn load_json<T: for<'de> Deserialize<'de>>(path: &Path) -> std::io::Result<T> {
        let file = std::fs::File::open(path)?;
        serde_json::from_reader(file)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    /// Save data to CSV file (for simple numeric data)
    pub fn save_csv(data: &[Vec<f64>], headers: &[&str], path: &Path) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        
        // Write headers
        writeln!(file, "{}", headers.join(","))?;
        
        // Write data
        for row in data {
            let row_str: Vec<String> = row.iter().map(|x| x.to_string()).collect();
            writeln!(file, "{}", row_str.join(","))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_paths() {
        let filename = paths::timestamped_filename("test", "json");
        assert!(filename.contains("test"));
        assert!(filename.ends_with(".json"));

        let results_dir = paths::results_dir();
        assert_eq!(results_dir.to_str().unwrap(), "results");
        // Test experiment directory creation
        if let Ok(exp_dir) = paths::create_experiment_dir("test_exp") {
            assert!(exp_dir.to_string_lossy().contains("test_exp"));
        }
    }

    #[test]
    fn test_validation() {
        // Test finite validation
        assert!(validation::validate_finite(&[1.0, 2.0, 3.0]).is_ok());
        assert!(validation::validate_finite(&[1.0, f64::NAN, 3.0]).is_err());
        assert!(validation::validate_finite(&[1.0, f64::INFINITY, 3.0]).is_err());

        // Test bounds validation
        assert!(validation::validate_bounds(5.0, 0.0, 10.0).is_ok());
        assert!(validation::validate_bounds(-1.0, 0.0, 10.0).is_err());
        assert!(validation::validate_bounds(11.0, 0.0, 10.0).is_err());
        // Test dimension validation
        assert!(validation::validate_dimension(&[1.0, 2.0, 3.0], 3).is_ok());
        assert!(validation::validate_dimension(&[1.0, 2.0], 3).is_err());
    }
    #[test]
    fn test_formatting() {
        let v = vec![1.234567, 2.345678, 3.456789];
        let formatted = formatting::format_vector(&v, 2);
        assert_eq!(formatted, "[1.23, 2.35, 3.46]");
        let progress = formatting::format_progress(10, 100, 1.23e-4, 5.67e-6);
        assert!(progress.contains("10/100"));
        assert!(progress.contains("1.230000e-4"));
    }
    #[test]
    fn test_stats() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(stats::mean(&values), 3.0);
        let std = stats::std_dev(&values);
        assert!((std - 1.5811388300841898).abs() < 1e-10);
        let (min, max) = stats::min_max(&values).unwrap();
        assert_eq!(min, 1.0);
        assert_eq!(max, 5.0);
        // Test empty slice
        assert_eq!(stats::mean(&[]), 0.0);
        assert!(stats::min_max(&[]).is_none());
    }
    #[test]
    fn test_timing() {
        let (result, duration) = timing::measure("test", || {
            std::thread::sleep(Duration::from_millis(10));
            42
        });
        assert_eq!(result, 42);
        assert!(duration.as_millis() >= 10);
    }
    #[test]
    fn test_recovery() {
        let mut values = vec![1e200, -1e200, 1e150];
        assert!(recovery::rescale_if_needed(&mut values, 1e100));
        assert!(values.iter().all(|&x| x.abs() <= 1e100));
    }
}