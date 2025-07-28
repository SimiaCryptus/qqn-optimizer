
pub mod line_search;
pub use line_search::{
    LineSearch, LineSearchConfig, LineSearchMethod,
    LineSearchResult, TerminationReason,
};

/// Tolerance for numerical comparisons
pub const NUMERICAL_TOLERANCE: f64 = 1e-12;

/// Maximum number of line search iterations
pub const MAX_LINE_SEARCH_ITERATIONS: usize = 50;

/// Default L-BFGS history size
pub const DEFAULT_LBFGS_HISTORY: usize = 10;

pub mod line_search_cubic_quadratic;
pub mod line_search_golden_section;
pub mod line_search_more_thuente;
pub mod line_search_bisection;
pub mod line_search_backtracking;
pub mod line_search_strong_wolfe;

pub use line_search_backtracking::{
    BacktrackingConfig, BacktrackingLineSearch,
};
pub use line_search_strong_wolfe::{
    StrongWolfeConfig, StrongWolfeLineSearch,
};
pub use line_search_bisection::{
    BisectionConfig,BisectionLineSearch,
};
pub use line_search_cubic_quadratic::{
    CubicQuadraticConfig, CubicQuadraticLineSearch,
};
pub use line_search_golden_section::{
    GoldenSectionConfig, GoldenSectionLineSearch,
};
pub use line_search_more_thuente::{
    MoreThuenteConfig, MoreThuenteLineSearch,
};

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