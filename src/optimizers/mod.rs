//! Optimization algorithms for machine learning and numerical optimization.
//! 
//! This module provides various optimization algorithms including L-BFGS, QQN, Adam, and trust region methods.

/// Result type for optimization operations
pub type OptResult<T> = Result<T, OptError>;

/// Comprehensive error type for optimization operations
#[derive(Debug, thiserror::Error)]
pub enum OptError {
    #[error("Tensor operation failed: {0}")]
    TensorError(#[from] candle_core::Error),

    #[error("Numerical error: {0}")]
    NumericalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Convergence error: {0}")]
    ConvergenceError(String),

    #[error("Line search failed: {0}")]
    LineSearchError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("Not enough history: required {required}, available {available}")]
    InsufficientHistory { required: usize, available: usize },
}
// Core optimizers

pub mod lbfgs;
pub mod optimizer;
pub mod qqn;

// Additional optimizers
pub mod adam;
pub mod gd;
pub mod trust_region;

/// Tolerance for numerical comparisons
pub const NUMERICAL_TOLERANCE: f64 = 1e-12;

/// Maximum number of line search iterations
pub const MAX_LINE_SEARCH_ITERATIONS: usize = 50;

/// Default L-BFGS history size
pub const DEFAULT_LBFGS_HISTORY: usize = 10;

/// Default trust region radius
pub const DEFAULT_TRUST_RADIUS: f64 = 1.0;

/// Minimum trust region radius before termination
pub const MIN_TRUST_RADIUS: f64 = 1e-10;

// Re-exports for convenience
pub use lbfgs::{LBFGSConfig, LBFGSOptimizer, LBFGSState};
pub use optimizer::{ConvergenceInfo, OptimizationMetadata, Optimizer, StepResult};
pub use qqn::{QQNConfig, QQNOptimizer, QQNState, QuadraticPath};
pub use adam::{AdamConfig, AdamOptimizer, AdamState};
pub use gd::{GDConfig, GDOptimizer, GDState};
pub use trust_region::{TrustRegionConfig, TrustRegionOptimizer, TrustRegionState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_validity() {
        // Verify numerical tolerance is reasonable
        assert!(NUMERICAL_TOLERANCE > 0.0);
        assert!(NUMERICAL_TOLERANCE < 1e-6);
        
        // Verify iteration limits
        assert!(MAX_LINE_SEARCH_ITERATIONS >= 10);
        assert!(MAX_LINE_SEARCH_ITERATIONS <= 100);
        
        // Verify history size
        assert!(DEFAULT_LBFGS_HISTORY >= 3);
        assert!(DEFAULT_LBFGS_HISTORY <= 50);
        
        // Verify trust region parameters
        assert!(DEFAULT_TRUST_RADIUS > 0.0);
        assert!(MIN_TRUST_RADIUS > 0.0);
        assert!(MIN_TRUST_RADIUS < DEFAULT_TRUST_RADIUS);
    }

    #[test]
    fn test_error_display() {
        let err = OptError::DimensionMismatch { expected: 10, actual: 5 };
        assert_eq!(err.to_string(), "Dimension mismatch: expected 10, got 5");
        
        let err = OptError::InsufficientHistory { required: 5, available: 3 };
        assert_eq!(err.to_string(), "Not enough history: required 5, available 3");
    }
}