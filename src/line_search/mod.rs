#![allow(clippy::module_inception)]

pub mod line_search;
pub use line_search::{
    LineSearch, LineSearchConfig, LineSearchMethod, LineSearchResult, TerminationReason,
};

/// Tolerance for numerical comparisons
pub const NUMERICAL_TOLERANCE: f32 = 1e-12;

/// Maximum number of line search iterations
pub const MAX_LINE_SEARCH_ITERATIONS: usize = 50;

/// Default L-BFGS history size
pub const DEFAULT_LBFGS_HISTORY: usize = 10;

pub mod backtracking;
pub mod bisection;
pub mod cubic_quadratic;
pub mod golden_section;
pub mod more_thuente;
pub mod strong_wolfe;

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
        assert!(NUMERICAL_TOLERANCE > 0.0);
        assert!(NUMERICAL_TOLERANCE < 1e-6);
        assert!(MAX_LINE_SEARCH_ITERATIONS > 0);
        assert!(DEFAULT_LBFGS_HISTORY > 0);
    }
}
