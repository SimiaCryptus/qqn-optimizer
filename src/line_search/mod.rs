//! Line search algorithms for optimization.
//!
//! This module provides various line search methods used in optimization algorithms
//! to find an appropriate step size along a given search direction.

// Re-export main types
pub mod line_search;
pub use line_search::{
    LineSearch, LineSearchConfig, LineSearchMethod, LineSearchResult, TerminationReason,
};

/// Tolerance for numerical comparisons in line search algorithms.
/// This value is used to determine when two floating-point numbers are effectively equal.
pub const NUMERICAL_TOLERANCE: f64 = 1e-12;

/// Maximum number of line search iterations allowed before termination.
/// This prevents infinite loops in pathological cases.
pub const MAX_LINE_SEARCH_ITERATIONS: usize = 50;

/// Default L-BFGS history size for quasi-Newton methods.
/// This determines how many past iterations are used to approximate the Hessian.
pub const DEFAULT_LBFGS_HISTORY: usize = 10;
/// Default initial step size for line search algorithms.
pub const DEFAULT_INITIAL_STEP_SIZE: f64 = 1.0;
// Line search implementations

pub mod backtracking;
pub mod bisection;
pub mod cubic_quadratic;
pub mod golden_section;
pub mod more_thuente;
pub mod strong_wolfe;
// Re-export line search implementations

pub use backtracking::{BacktrackingConfig, BacktrackingLineSearch};
pub use bisection::{BisectionConfig, BisectionLineSearch};
pub use cubic_quadratic::{CubicQuadraticConfig, CubicQuadraticLineSearch};
pub use golden_section::{GoldenSectionConfig, GoldenSectionLineSearch};
pub use more_thuente::{MoreThuenteConfig, MoreThuenteLineSearch};
pub use strong_wolfe::{StrongWolfeConfig, StrongWolfeLineSearch};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        // Verify numerical tolerance is reasonable
        assert!(NUMERICAL_TOLERANCE > 0.0);
        assert!(NUMERICAL_TOLERANCE < 1e-6);
        // Verify iteration limits
        assert!(MAX_LINE_SEARCH_ITERATIONS > 0);
        assert!(MAX_LINE_SEARCH_ITERATIONS <= 100, "Too many iterations may indicate convergence issues");
        // Verify L-BFGS history size
        assert!(DEFAULT_LBFGS_HISTORY > 0);
        assert!(DEFAULT_LBFGS_HISTORY <= 50, "Large history sizes may cause memory issues");
        // Verify initial step size
        assert!(DEFAULT_INITIAL_STEP_SIZE > 0.0);
        assert!(DEFAULT_INITIAL_STEP_SIZE <= 10.0, "Large initial steps may cause overshooting");
    }
    #[test]
    fn test_module_exports() {
        // Ensure all expected types are accessible
        let _: LineSearchMethod;
        let _: TerminationReason;
        // Ensure all line search implementations are accessible
        let _: BacktrackingLineSearch;
        let _: BisectionLineSearch;
        let _: CubicQuadraticLineSearch;
        let _: GoldenSectionLineSearch;
        let _: MoreThuenteLineSearch;
        let _: StrongWolfeLineSearch;
    }
    #[test]
    fn test_numerical_precision() {
        // Ensure numerical tolerance is appropriate for f64 precision
        assert!(NUMERICAL_TOLERANCE >= f64::EPSILON * 100.0);
    }
}