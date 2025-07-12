//! Benchmarking suite for optimization algorithms.
//!
//! This module provides:
//! - Standard mathematical test functions
//! - Machine learning optimization problems
//! - Benchmark execution framework
//! - Performance evaluation metrics

pub mod functions;
pub mod ml_problems;
pub mod evaluation;
pub mod runner;

// Re-export commonly used types

pub use functions::{
    RosenbrockFunction, RastriginFunction, SphereFunction, BealeFunction,
    HimmelblauFunction, BoothFunction, AckleyFunction,
};
pub use ml_problems::{
    LogisticRegression, NeuralNetworkTraining, LinearRegression,
    SupportVectorMachine,
};
pub use evaluation::{
    BenchmarkRunner, BenchmarkConfig, BenchmarkResults, SingleResult,
    OptimizationTrace,
};

use crate::core::OptResult;
use crate::benchmarks::functions::OptimizationProblem;

/// Standard benchmark problems for optimization research
pub fn standard_benchmark_suite() -> OptResult<Vec<Box<dyn OptimizationProblem>>> {
    let mut problems: Vec<Box<dyn OptimizationProblem>> = Vec::new();

    // Mathematical functions
    problems.push(Box::new(RosenbrockFunction::new(2)));
    problems.push(Box::new(RosenbrockFunction::new(10)));
    problems.push(Box::new(RosenbrockFunction::new(100)));
    
    problems.push(Box::new(RastriginFunction::new(2)));
    problems.push(Box::new(RastriginFunction::new(10)));
    problems.push(Box::new(RastriginFunction::new(100)));
    
    problems.push(Box::new(SphereFunction::new(10)));
    problems.push(Box::new(SphereFunction::new(100)));
    problems.push(Box::new(SphereFunction::new(1000)));
    
    problems.push(Box::new(BealeFunction::new()));
    problems.push(Box::new(BoothFunction::new()));
    problems.push(Box::new(HimmelblauFunction::new()));
    problems.push(Box::new(AckleyFunction::new(10)));


    Ok(problems)
}

/// Machine learning benchmark problems
pub fn ml_benchmark_suite() -> OptResult<Vec<Box<dyn OptimizationProblem>>> {
    let mut problems: Vec<Box<dyn OptimizationProblem>> = Vec::new();

    // Logistic regression problems
    problems.push(Box::new(LogisticRegression::synthetic(100, 10)?));
    problems.push(Box::new(LogisticRegression::synthetic(1000, 50)?));
    
    // Neural network problems
    problems.push(Box::new(NeuralNetworkTraining::mlp_classification(
        vec![784, 128, 64, 10]
    )?));

    Ok(problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_benchmark_suite() {
        let problems = standard_benchmark_suite().unwrap();
        assert!(!problems.is_empty());
        assert!(problems.len() >= 10);
    }

    #[test]
    fn test_ml_benchmark_suite() {
        let problems = ml_benchmark_suite().unwrap();
        assert!(!problems.is_empty());
    }
}
