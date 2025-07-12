//! Utility functions and shared components.
//!
//! This module provides:
//! - Mathematical operations and tensor utilities
//! - Logging and debugging tools
//! - Serialization helpers
//! - Common data structures

pub mod logging;
pub mod serialization;
pub mod random;
pub mod math;

pub use logging::{
    setup_tracing, log_optimization_step, log_convergence_info,
    PerformanceTimer, OptimizationLogger,
};
pub use serialization::{
    save_results, load_results, save_config, load_config,
    ResultsSerializer, ConfigSerializer,
};
pub use random::{RandomGenerator, random_starting_point, random_starting_points};
pub use math::{
    compute_magnitude, dot_product, vector_add, vector_subtract, vector_scale,
    norm_l2, norm_l1, norm_inf, is_finite, clamp_vector, lerp, angle_between,
    dot_product_f64,
};

use crate::core::OptResult;


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
        format!("{}_{}.{}", base, timestamp, extension)
    }
    
    /// Get results directory path
    pub fn results_dir() -> PathBuf {
        PathBuf::from("results")
    }
    
    /// Get experiments directory path
    pub fn experiments_dir() -> PathBuf {
        PathBuf::from("experiments")
    }
}

/// Validation utilities
pub mod validation {
    use crate::core::OptResult;
    
    /// Validate that a vector contains only finite values
    pub fn validate_finite(values: &[f64]) -> OptResult<()> {
        for (i, &val) in values.iter().enumerate() {
            if !val.is_finite() {
                return Err(anyhow::anyhow!(
                    "Non-finite value {} at index {}", val, i
                ).into());
            }
        }
        Ok(())
    }
    
    /// Validate that a value is within reasonable bounds
    pub fn validate_bounds(value: f64, min: f64, max: f64) -> OptResult<()> {
        if value < min || value > max {
            return Err(anyhow::anyhow!(
                "Value {} outside bounds [{}, {}]", value, min, max
            ));
        }
        Ok(())
    }
    
    /// Validate optimizer configuration
    pub fn validate_optimizer_config<T: std::fmt::Debug>(config: &T) -> OptResult<()> {
        // Basic validation - specific implementations would add more checks
        tracing::debug!("Validating optimizer config: {:?}", config);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(constants::EPSILON > 0.0);
        assert!(constants::SQRT_EPSILON > constants::EPSILON);
        assert!(constants::DEFAULT_TOLERANCE > constants::SQRT_EPSILON);
        assert!(constants::MAX_SAFE_VALUE > 1.0);
        assert!(constants::MIN_SAFE_VALUE < 1.0);
    }

    #[test]
    fn test_paths() {
        let filename = paths::timestamped_filename("test", "json");
        assert!(filename.contains("test"));
        assert!(filename.ends_with(".json"));
        
        let results_dir = paths::results_dir();
        assert_eq!(results_dir.to_str().unwrap(), "results");
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
    }
}