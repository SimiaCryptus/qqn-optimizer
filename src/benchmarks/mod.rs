//! Benchmarking suite for optimization algorithms.
//!
//! This module provides:
//! - Standard mathematical test functions
//! - Machine learning optimization problems
//! - Benchmark execution framework
//! - Performance evaluation metrics

pub mod evaluation;
pub mod functions;
pub mod ml_problems;
pub mod mnist;
pub mod analytic_functions;
// Re-export commonly used types

pub use analytic_functions::AckleyFunction;
pub use analytic_functions::BealeFunction;
pub use analytic_functions::BoothFunction;
pub use analytic_functions::GriewankFunction;
pub use analytic_functions::HimmelblauFunction;
pub use analytic_functions::LevyFunction;
pub use analytic_functions::MichalewiczFunction;
pub use analytic_functions::RastriginFunction;
pub use analytic_functions::RosenbrockFunction;
pub use analytic_functions::SchwefelFunction;
pub use analytic_functions::SphereFunction;
pub use analytic_functions::ZakharovFunction;
pub use ml_problems::{
    LinearRegression, LogisticRegression, NeuralNetworkTraining, SupportVectorMachine,
};
