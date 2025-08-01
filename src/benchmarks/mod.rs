//! Benchmarking suite for optimization algorithms.
//!
//! This module provides:
//! - Standard mathematical test functions
//! - Machine learning optimization problems
//! - Benchmark execution framework
//! - Performance evaluation metrics
//!
//! # Examples
//!
//! ```rust
//! use qqn_optimizer::benchmarks::{RosenbrockFunction, BenchmarkRunner};
//!
//! // Create a test function
//! let rosenbrock = RosenbrockFunction::new(2);
//!
//! // Run benchmarks
//! let runner = BenchmarkRunner::new();
//! let results = runner.run(&rosenbrock);
//! ```
//!
//! # Feature Flags
//!
//! - `onednn`: Enables Intel oneDNN optimized MNIST implementation
// Core modules

pub mod analytic_functions;
pub mod evaluation;
pub mod functions;
pub mod ml_problems;
pub mod mnist;
#[cfg(feature = "onednn")]
pub mod mnist_onednn;

// Re-exports for convenience
// Standard mathematical test functions
pub use analytic_functions::{AnalyticFunction, OptimizationProblem};

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
// Machine learning optimization problems
pub use ml_problems::{
    LinearRegression, LogisticRegression, NeuralNetworkTraining, SupportVectorMachine,
};
// Evaluation and benchmarking utilities
pub use evaluation::{BenchmarkResult, BenchmarkRunner, PerformanceMetrics};
// Common types from functions module
pub use functions::{ObjectiveFunction, TestFunction};
// MNIST-specific exports
pub use mnist::{MnistDataset, MnistProblem};
/// Prelude module for convenient imports
pub mod prelude {
    pub use super::analytic_functions::{AnalyticFunction, OptimizationProblem};
    pub use super::evaluation::{BenchmarkResult, BenchmarkRunner};
    pub use super::functions::ObjectiveFunction;
    pub use super::ml_problems::{LinearRegression, LogisticRegression};
    // Re-export commonly used test functions
    pub use super::{
        RosenbrockFunction,
        SphereFunction,
        RastriginFunction,
        AckleyFunction,
    };
}