pub mod analysis;
pub mod benchmarks;
pub mod core;
pub mod utils;

// Re-export commonly used types
pub use core::{
    lbfgs::{LBFGSConfig, LBFGSOptimizer},
    line_search::{BacktrackingConfig, LineSearchConfig, LineSearchMethod, StrongWolfeConfig},
    optimizer::{ConvergenceInfo, Optimizer, StepResult},
    qqn::{QQNConfig, QQNOptimizer},
};

pub use benchmarks::{
    functions::{
        AckleyFunction, BealeFunction, OptimizationProblem, RastriginFunction, RosenbrockFunction,
        SphereFunction,
    },
    ml_problems::{LogisticRegression, NeuralNetworkTraining},
};

pub use analysis::{
    plotting::{ExtendedOptimizationTrace, PlotConfig, PlottingEngine},
    reporting::AcademicReport,
    statistics::{ConvergenceComparison, PerformanceProfiles, StatisticalAnalysis},
};

// Error types
pub use anyhow::{Error, Result};

pub use crate::core::adam::{AdamConfig, AdamOptimizer, AdamState};

/// Current version of the QQN optimizer framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging for the framework
pub fn init_logging() -> Result<()> {
    init_logging_with_mode(true)
}

/// Initialize logging with configurable format mode
/// - compact: if true, uses compact format (raw output only)
/// - compact: if false, uses default format (with timestamp, level, source)
pub fn init_logging_with_mode(compact: bool) -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "qqn_optimizer=debug".into());

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
        assert!(!VERSION.is_empty());
        assert!(VERSION.contains('.'));
    }
}
