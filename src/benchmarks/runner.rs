//! Helper module for running benchmarks with concrete optimizer types

use crate::benchmarks::evaluation::{BenchmarkError, BenchmarkResults, BenchmarkRunner};
use crate::benchmarks::functions::OptimizationProblem;
use crate::core::lbfgs::LBFGSOptimizer;
use crate::core::qqn::QQNOptimizer;
use crate::Optimizer;

/// Run benchmarks for L-BFGS optimizer
pub async fn run_lbfgs_benchmarks(
    runner: &BenchmarkRunner,
    problems: &[Box<dyn OptimizationProblem>],
    optimizers: Vec<LBFGSOptimizer>,
) -> Result<BenchmarkResults, BenchmarkError> {
    let mut results = BenchmarkResults::new(runner.config.clone());

    for problem in problems {
        for optimizer in &optimizers {
            for run_id in 0..runner.config.num_runs {
                let result = runner
                    .run_single_benchmark(problem.as_ref(), optimizer, run_id, &optimizer.name().to_string())
                    .await?;

                results.add_result(result);
            }
        }
    }

    Ok(results)
}

/// Run benchmarks for QQN optimizer
pub async fn run_qqn_benchmarks(
    runner: &BenchmarkRunner,
    problems: &[Box<dyn OptimizationProblem>],
    optimizers: Vec<QQNOptimizer>,
) -> Result<BenchmarkResults, BenchmarkError> {
    let mut results = BenchmarkResults::new(runner.config.clone());

    for problem in problems {
        for optimizer in &optimizers {
            for run_id in 0..runner.config.num_runs {
                let result = runner
                    .run_single_benchmark(problem.as_ref(), optimizer, run_id, &optimizer.name().to_string())
                    .await?;

                results.add_result(result);
            }
        }
    }

    Ok(results)
}

/// Run benchmarks comparing multiple optimizer types
pub async fn run_comparison_benchmarks(
    runner: &BenchmarkRunner,
    problems: &[Box<dyn OptimizationProblem>],
    lbfgs_optimizers: Vec<LBFGSOptimizer>,
    qqn_optimizers: Vec<QQNOptimizer>,
) -> Result<BenchmarkResults, BenchmarkError> {
    let mut results = BenchmarkResults::new(runner.config.clone());

    // Run L-BFGS benchmarks
    let lbfgs_results = run_lbfgs_benchmarks(runner, problems, lbfgs_optimizers).await?;
    for result in lbfgs_results.results {
        results.add_result(result);
    }

    // Run QQN benchmarks
    let qqn_results = run_qqn_benchmarks(runner, problems, qqn_optimizers).await?;
    for result in qqn_results.results {
        results.add_result(result);
    }

    Ok(results)
}