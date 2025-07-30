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
}

pub mod lbfgs;
pub mod optimizer;
pub mod qqn;
pub use lbfgs::{LBFGSConfig, LBFGSOptimizer, LBFGSState};
pub use optimizer::{ConvergenceInfo, OptimizationMetadata, Optimizer, StepResult};
pub use qqn::{QQNConfig, QQNOptimizer, QQNState, QuadraticPath};

/// Tolerance for numerical comparisons
pub const NUMERICAL_TOLERANCE: f64 = 1e-12;

/// Maximum number of line search iterations
pub const MAX_LINE_SEARCH_ITERATIONS: usize = 50;

/// Default L-BFGS history size
pub const DEFAULT_LBFGS_HISTORY: usize = 10;

pub mod adam;
pub mod gd;
pub mod trust_region;

pub use gd::{GDConfig, GDOptimizer, GDState};
pub use trust_region::{TrustRegionConfig, TrustRegionOptimizer, TrustRegionState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        // Verify our constants have sensible values at compile time
        const _: () = assert!(NUMERICAL_TOLERANCE > 0.0);
        const _: () = assert!(NUMERICAL_TOLERANCE < 1e-6);
        const _: () = assert!(MAX_LINE_SEARCH_ITERATIONS > 0);
        const _: () = assert!(DEFAULT_LBFGS_HISTORY > 0);

        // These are runtime assertions to verify our constants are reasonable
        // (clippy complains about constant assertions, so we do runtime checks)
    }
}
