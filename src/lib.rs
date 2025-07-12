//! # QQN Optimizer Research Framework
//!
//! This crate implements the Quadratic Quasi-Newton (QQN) optimization algorithm
//! for academic research and benchmarking. It provides a comprehensive framework
//! for comparing optimization algorithms across various problem domains.
//!
//! ## Core Features
//!
//! - **QQN Algorithm**: Novel optimization method with magnitude-based switching
//! - **Benchmarking Suite**: Standardized test problems and evaluation metrics
//! - **Statistical Analysis**: Rigorous performance comparison and validation
//! - **Visualization**: Comprehensive plotting and analysis tools
//!
//! ## Quick Start
//!
//! ```rust
//! use qqn_optimizer::{QQNOptimizer, QQNConfig, RosenbrockFunction};
//!
//! // Create optimizer
//! let config = QQNConfig::default();
//! let mut optimizer = QQNOptimizer::new(config);
//!
//! // Define problem
//! let problem = RosenbrockFunction::new(2);
//! let mut x = problem.initial_point();
//!
//! // Optimization loop
//! for _ in 0..1000 {
//!     let gradient = problem.gradient(&x)?;
//!     let step_result = optimizer.step(&mut x, &gradient)?;
//!     
//!     if step_result.convergence_info.converged {
//!         break;
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod analysis;
pub mod benchmarks;
pub mod config;
pub mod core;
pub mod utils;

// Re-export commonly used types
pub use core::{
    lbfgs::{LBFGSConfig, LBFGSOptimizer},
    optimizer::{ConvergenceInfo, Optimizer, StepResult},
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
    reporting::{AcademicReport, ReportGenerator},
    statistics::{ConvergenceComparison, PerformanceProfiles, StatisticalAnalysis},
};

pub use config::{ExperimentConfig, OptimizerConfig, ProblemConfig};

// Error types
pub use anyhow::{Error, Result};

/// Current version of the QQN optimizer framework
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration for academic research
pub fn default_research_config() -> config::ExperimentConfig {
    config::ExperimentConfig::default_research()
}

/// Initialize logging for the framework
pub fn init_logging() -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "qqn_optimizer=info".into()),
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

    #[test]
    fn test_default_config() {
        let config = default_research_config();
        assert!(!config.name.is_empty());
        assert!(!config.problems.is_empty());
        assert!(!config.optimizers.is_empty());
    }
}
