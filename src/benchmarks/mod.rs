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
pub mod runner;

// Re-export commonly used types

pub use evaluation::{
    BenchmarkConfig, BenchmarkResults, BenchmarkRunner, OptimizationTrace, SingleResult,
};
pub use functions::{
    AckleyFunction, BealeFunction, BoothFunction, HimmelblauFunction, RastriginFunction,
    RosenbrockFunction, SphereFunction,
};
pub use ml_problems::{
    LinearRegression, LogisticRegression, NeuralNetworkTraining, SupportVectorMachine,
};


