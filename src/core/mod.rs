//! Core optimization algorithms and traits.
//!
//! This module contains the fundamental building blocks for optimization algorithms,
//! - Base traits for optimizers and problems
//! - QQN algorithm implementation
//! - Baseline optimizers (L-BFGS, Adam, SGD)
//! - Line search algorithms
//! - Mathematical utilities
/// Core result type used throughout the optimization framework
pub type OptResult<T> = anyhow::Result<T>;

pub mod lbfgs;
pub mod line_search;
pub mod optimizer;
pub mod qqn;
pub use lbfgs::{LBFGSConfig, LBFGSOptimizer, LBFGSState};
pub use line_search::{
    BacktrackingConfig, BacktrackingLineSearch, LineSearch, LineSearchConfig, LineSearchMethod,
    LineSearchResult, StrongWolfeConfig, StrongWolfeLineSearch, TerminationReason,
};
pub use optimizer::{ConvergenceInfo, OptimizationMetadata, Optimizer, OptimizerBox, StepResult};
pub use qqn::{QQNConfig, QQNOptimizer, QQNState, QuadraticPath};


/// Tolerance for numerical comparisons
pub const NUMERICAL_TOLERANCE: f64 = 1e-12;

/// Maximum number of line search iterations
pub const MAX_LINE_SEARCH_ITERATIONS: usize = 50;

/// Default L-BFGS history size
pub const DEFAULT_LBFGS_HISTORY: usize = 10;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(NUMERICAL_TOLERANCE > 0.0);
        assert!(NUMERICAL_TOLERANCE < 1e-6);
        assert!(MAX_LINE_SEARCH_ITERATIONS > 0);
        assert!(DEFAULT_LBFGS_HISTORY > 0);
    }
}
use serde::{Deserialize, Serialize};