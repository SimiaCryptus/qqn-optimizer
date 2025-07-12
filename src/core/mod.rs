//! Core optimization algorithms and traits.
//!
//! This module contains the fundamental building blocks for optimization algorithms,
pub mod optimizer;
// Re-export commonly used types
pub use optimizer::{
    Optimizer, StepResult, ConvergenceInfo, ConvergenceCriterion,
    OptimizationMetadata, TimingInfo, MemoryInfo,
    ConvergenceConfig, ConvergenceChecker,
};
//! - Base traits for optimizers and problems
//! - QQN algorithm implementation
//! - Baseline optimizers (L-BFGS, Adam, SGD)
//! - Line search algorithms
//! - Mathematical utilities

pub mod optimizer;
pub mod qqn;
pub mod lbfgs;
pub mod line_search;

// Re-export commonly used types
pub use optimizer::{Optimizer, StepResult, ConvergenceInfo, OptimizationProblem};
pub use qqn::{QQNOptimizer, QQNConfig, QQNState, QuadraticPath};
pub use lbfgs::{LBFGSOptimizer, LBFGSConfig, LBFGSState};
pub use line_search::{LineSearch, LineSearchConfig, StrongWolfeLineSearch, BacktrackingLineSearch};

/// Common result type for optimization operations
pub type OptResult<T> = anyhow::Result<T>;

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
pub mod line_search;
pub mod lbfgs;
pub mod qqn;
pub use optimizer::*;
pub use line_search::*;
pub use lbfgs::*;
pub use qqn::*;
pub use lbfgs::*;