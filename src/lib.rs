#![allow(clippy::doc_overindented_list_items)]
#![allow(clippy::doc_lazy_continuation)]

pub mod analysis;
pub mod benchmarks;
pub mod experiment_runner;
pub mod line_search;
pub mod optimizers;
pub mod utils;
// Re-export commonly used items for easier testing
pub use benchmarks::functions::OptimizationProblem;
pub use benchmarks::unified_tests::{
    generate_test_report, test_multiple_problems, ProblemTestConfig, ProblemTestResults,
    UnifiedProblemTester,
};

// Re-export commonly used types
pub use optimizers::{
    lbfgs::{LBFGSConfig, LBFGSOptimizer},
    optimizer::{ConvergenceInfo, Optimizer, StepResult},
    qqn::{QQNConfig, QQNOptimizer},
};

pub use line_search::{
    LineSearch, LineSearchConfig, LineSearchMethod, LineSearchResult, TerminationReason,
};

pub use experiment_runner::{optimizer_sets, problem_sets};

#[cfg(feature = "plotting")]
pub use analysis::plotting::{ExtendedOptimizationTrace, PlotConfig, PlottingEngine};

// Re-export ML problems for easier access
pub use crate::benchmarks::ml_problems::{
    LinearRegression, LogisticRegression, NeuralNetworkTraining, SupportVectorMachine,
};

// Re-export commonly used types
pub use crate::optimizers::adam::{AdamConfig, AdamOptimizer, AdamState};
// Error types
pub use anyhow::{Error, Result};
pub use benchmarks::analytic_functions::AckleyFunction;
pub use benchmarks::analytic_functions::BealeFunction;
pub use benchmarks::analytic_functions::RastriginFunction;
pub use benchmarks::analytic_functions::RosenbrockFunction;
pub use benchmarks::analytic_functions::SphereFunction;
// Re-export ML problems for easier access
pub use benchmarks::mnist::MnistNeuralNetwork;
#[cfg(feature = "onednn")]
pub use benchmarks::mnist_onednn::MnistOneDnnNeuralNetwork;

/// Current version of the QQN optimizer framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging for the framework
pub fn init_logging(debug: bool) -> Result<()> {
    init_logging_with_mode(true, debug)
}

/// Initialize logging with configurable format mode
/// - compact: if true, uses compact format (raw output only)
/// - compact: if false, uses default format (with timestamp, level, source)
pub fn init_logging_with_mode(compact: bool, debug: bool) -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let env_filter = if debug {
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "qqn_optimizer=debug".into())
    } else {
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "qqn_optimizer=info".into())
    };

    let registry = tracing_subscriber::registry().with(env_filter);

    if compact {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_level(false)
                    .with_target(false)
                    .with_thread_ids(false)
                    .with_thread_names(false)
                    .with_file(false)
                    .with_line_number(false),
            )
            .try_init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize logging: {}", e))?;
    } else {
        registry
            .with(tracing_subscriber::fmt::layer())
            .try_init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize logging: {}", e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        // VERSION is a const string, so this check ensures it's properly defined
        assert!(!VERSION.is_empty(), "VERSION should not be empty");
        assert!(
            VERSION.contains('.'),
            "VERSION should contain a dot for version format"
        );
    }
}
