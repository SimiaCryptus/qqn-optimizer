pub mod analysis;
pub mod benchmarks;
pub mod config;
pub mod core;
pub mod utils;

// Re-export commonly used types
pub use core::{
    lbfgs::{LBFGSConfig, LBFGSOptimizer},
    line_search::{BacktrackingConfig, LineSearchConfig, LineSearchMethod, StrongWolfeConfig},
    optimizer::{ConvergenceInfo, Optimizer, OptimizerBox, StepResult},
    qqn::{QQNConfig, QQNOptimizer},
};

pub use benchmarks::{
    evaluation::{BenchmarkResults, BenchmarkRunner},
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

pub use config::{ExperimentConfig, OptimizerConfig, ProblemConfig};

// Error types
pub use anyhow::{Error, Result};

pub use crate::core::adam::{AdamConfig, AdamOptimizer, AdamState};
/// Current version of the QQN optimizer framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging for the framework
pub fn init_logging() -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "qqn_optimizer=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize logging: {}", e))?;

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