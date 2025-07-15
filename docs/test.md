# QQN Optimizer Testing and Reporting Documentation

## Table of Contents

1. [Overview](#overview)
2. [Test Suite Architecture](#test-suite-architecture)
3. [Benchmark Framework](#benchmark-framework)
4. [Statistical Analysis](#statistical-analysis)
5. [Report Generation](#report-generation)
6. [Usage Guide](#usage-guide)
7. [Interpreting Results](#interpreting-results)
8. [Extending the Framework](#extending-the-framework)

## Overview

The QQN Optimizer testing and reporting framework provides comprehensive tools for evaluating optimization algorithms,
performing statistical analysis, and generating publication-ready reports. The framework is designed to support academic
research and industrial applications requiring rigorous performance validation.

### Key Features

- **Automated Benchmarking**: Run optimization algorithms on standard test functions
- **Statistical Analysis**: Perform significance tests and effect size calculations
- **Visualization**: Generate convergence plots and performance profiles
- **Academic Reporting**: Create LaTeX tables and HTML reports for publications
- **Reproducibility**: Ensure consistent results with seed control and detailed logging

## Test Suite Architecture

### Core Components

#### 1. Benchmark Runner (`BenchmarkRunner`)

The main orchestrator for running optimization experiments.

```rust
pub struct BenchmarkRunner {
   config: BenchmarkConfig,
}

pub struct BenchmarkConfig {
   pub max_iterations: usize,      // Maximum iterations per run
   pub tolerance: f64,             // Convergence tolerance
   pub time_limit: Duration,       // Time limit per run
   pub random_seed: u64,           // For reproducibility
   pub num_runs: usize,            // Runs per configuration
}
```

#### 2. Optimization Problems

Test problems implement the `OptimizationProblem` trait:

```rust
pub trait OptimizationProblem {
   fn name(&self) -> &str;
   fn dimension(&self) -> usize;
   fn initial_point(&self) -> Vec<f64>;
   fn evaluate(&self, x: &[f64]) -> Result<f64>;
   fn gradient(&self, x: &[f64]) -> Result<Vec<f64>>;
   fn optimal_value(&self) -> Option<f64>;
   fn convergence_tolerance(&self) -> f64;
}
```

Available test functions include:

- **Convex**: Sphere, Quadratic
- **Non-convex**: Rosenbrock, Rastrigin, Ackley
- **Multi-modal**: Griewank, Schwefel, Levy
- **2D Problems**: Beale, Booth, Himmelblau, Matyas

#### 3. Result Storage

Results are stored in structured formats:

```rust
pub struct SingleResult {
   pub problem_name: String,
   pub optimizer_name: String,
   pub run_id: usize,
   pub final_value: f64,
   pub final_gradient_norm: f64,
   pub iterations: usize,
   pub convergence_achieved: bool,
   pub execution_time: Duration,
   pub trace: OptimizationTrace,
   pub convergence_reason: ConvergenceReason,
}
```

## Benchmark Framework

### Running Benchmarks

#### Basic Usage
```rust
use qqn_optimizer::benchmarks::*;

#[tokio::main]
async fn main() -> Result<()> {
   // Configure benchmark
   let config = BenchmarkConfig {
      max_iterations: 1000,
      tolerance: 1e-6,
      num_runs: 10,
      ..Default::default()
   };

   // Create runner
   let runner = BenchmarkRunner::new(config);

   // Define problems
   let problems = vec![
      Box::new(SphereFunction::new(10)),
      Box::new(RosenbrockFunction::new(10)),
   ];

   // Define optimizers
   let optimizers = vec![
      Box::new(QQNOptimizer::new(QQNConfig::default())),
      Box::new(LBFGSOptimizer::new(LBFGSConfig::default())),
   ];

   // Run benchmarks
   let results = runner.run_benchmarks(problems, optimizers).await?;

   // Save results
   results.save_to_file(Path::new("results.json"))?;

   Ok(())
}
```

#### Comprehensive Experiments

The `ExperimentRunner` provides high-level experiment management:

```rust
let runner = ExperimentRunner::new("output_directory".to_string());
runner.run_comparative_benchmarks().await?;
```

This automatically:

1. Runs multiple optimizers on standard test problems
2. Performs statistical analysis
3. Generates plots and visualizations
4. Creates HTML and LaTeX reports

### Convergence Tracking

The framework tracks detailed optimization progress:

```rust
pub struct OptimizationTrace {
   pub iterations: Vec<IterationData>,
   pub total_function_evaluations: usize,
   pub total_gradient_evaluations: usize,
}

pub struct IterationData {
   pub iteration: usize,
   pub function_value: f64,
   pub gradient_norm: f64,
   pub step_size: f64,
   pub parameters: Vec<f64>,
   pub timestamp: Duration,
}
```

## Statistical Analysis

### Analysis Components

#### 1. Convergence Comparison

Compares convergence behavior across optimizers:

```rust
pub struct ConvergenceComparison {
   pub optimizer_stats: HashMap<String, OptimizerStatistics>,
   pub pairwise_comparisons: Vec<PairwiseComparison>,
   pub significance_tests: Vec<SignificanceTest>,
}
```

#### 2. Statistical Tests

- **Welch's t-test**: For normally distributed results
- **Mann-Whitney U test**: Non-parametric alternative
- **Effect Size**: Cohen's d for practical significance

#### 3. Performance Profiles

Evaluates solver performance across problem sets:

```rust
pub struct PerformanceProfiles {
   pub tolerance_levels: Vec<f64>,
   pub profiles: HashMap<String, ProfileData>,
}
```

### Running Statistical Analysis

```rust
use qqn_optimizer::analysis::statistics::StatisticalAnalysis;

// Create analysis from results
let analysis = StatisticalAnalysis::new( & benchmark_results);

// Access different analyses
let convergence = analysis.convergence_comparison();
let profiles = analysis.performance_profiles();
let robustness = analysis.robustness_analysis();

// Get significance tests
for test in analysis.significance_tests() {
if test.is_significant() {
println ! ("{} significantly outperforms {}",
test.optimizer_a, test.optimizer_b);
}
}
```

## Report Generation

### HTML Reports

The framework generates comprehensive HTML reports with:

1. **Executive Summary**
   - Total problems tested
   - Success rates by optimizer
   - Key findings

2. **Detailed Results**
   - Performance tables for each problem
   - Convergence statistics
   - Timing information

3. **Statistical Analysis**
   - Pairwise significance tests
   - Effect sizes
   - Confidence intervals

4. **Visualizations**
   - Convergence plots
   - Performance profiles
   - Box plots

### LaTeX Export

For academic publications:

```rust
use qqn_optimizer::tests::performance_analysis::LaTeXTableGenerator;

// Generate performance table
let latex_table = LaTeXTableGenerator::generate_performance_table( & results);

// Generate significance table
let significance_table = LaTeXTableGenerator::generate_significance_table( & analysis);
```

Example LaTeX output:
```latex
\begin{table}[htbp]
\centering
\caption{Performance comparison of optimization algorithms}
\label{tab:performance}
\begin{tabular}{lcccc}
\toprule
Algorithm & Mean Final Value & Std Dev & Mean Iterations & Success Rate \\
\midrule
QQN & 1.23e-08 & 3.45e-09 & 145.3 & 98.5\% \\
L-BFGS & 2.34e-08 & 5.67e-09 & 167.8 & 95.0\% \\
\bottomrule
\end{tabular}
\end{table}
```

### CSV Export

For further analysis in external tools:

```rust
// Export detailed results
results.save_to_csv(Path::new("detailed_results.csv")) ?;

// Export summary statistics
let summary = results.summary_statistics();
summary.save_to_csv(Path::new("summary.csv")) ?;
```

## Usage Guide

### Quick Start

1. **Run Basic Benchmarks**
```bash
cargo test test_comprehensive_benchmarks -- --nocapture
```

2. **Generate Academic Report**

```bash
cargo test test_academic_citation_format -- --nocapture
```

3. **Export LaTeX Tables**

```bash
cargo test test_latex_table_generation -- --nocapture
```

### Custom Experiments

```rust
// Define custom problem
struct MyProblem {
   dimension: usize,
}

impl OptimizationProblem for MyProblem {
   fn evaluate(&self, x: &[f64]) -> Result<f64> {
      // Your objective function
   }

   fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
      // Your gradient computation
   }
   // ... other trait methods
}

// Run custom benchmark
let problems = vec![Box::new(MyProblem { dimension: 10 })];
let results = runner.run_benchmarks(problems, optimizers).await?;
```

### Configuration Options

```rust
// Aggressive convergence testing
let config = BenchmarkConfig {
max_iterations: 10000,
tolerance: 1e-12,
time_limit: Duration::from_secs(3600),
num_runs: 50,
random_seed: 42,
};

// Quick testing
let config = BenchmarkConfig {
max_iterations: 100,
tolerance: 1e-6,
time_limit: Duration::from_secs(60),
num_runs: 3,
random_seed: 42,
};
```

## Interpreting Results

### Performance Metrics

1. **Final Value**: Lower is better for minimization
2. **Gradient Norm**: Indicates convergence quality
3. **Iterations**: Efficiency measure
4. **Success Rate**: Robustness indicator
5. **Execution Time**: Computational efficiency

### Statistical Significance

- **p-value < 0.05**: Statistically significant difference
- **Effect Size**:
   - Small: |d| < 0.2
   - Medium: 0.2 ≤ |d| < 0.8
   - Large: |d| ≥ 0.8

### Performance Profiles

Performance profiles show the fraction of problems solved within a performance ratio τ:

- Higher curves indicate better performance
- Leftmost point: fraction where optimizer was best
- Rightmost value: robustness (fraction eventually solved)

## Extending the Framework

### Adding New Test Problems

```rust
use qqn_optimizer::benchmarks::functions::OptimizationProblem;

pub struct CustomFunction {
   // Your fields
}

impl OptimizationProblem for CustomFunction {
   // Implement required methods
}

// Register in experiment
let problems = vec![
   Box::new(CustomFunction::new()),
   // ... other problems
];
```

### Adding New Analysis Metrics

```rust
impl BenchmarkResults {
   pub fn custom_metric(&self) -> HashMap<String, f64> {
      let mut metrics = HashMap::new();

      for (optimizer, results) in self.group_by_optimizer() {
         // Calculate your metric
         let metric_value = calculate_custom_metric(results);
         metrics.insert(optimizer, metric_value);
      }

      metrics
   }
}
```

### Custom Visualizations

```rust
use qqn_optimizer::analysis::plotting::PlottingEngine;

impl PlottingEngine {
   pub fn custom_plot(&self, data: &CustomData) -> Result<()> {
      // Use plotters crate to create custom visualizations
   }
}
```

## Best Practices

1. **Reproducibility**
   - Always set random seeds
   - Document hardware and software versions
   - Save raw results for future analysis

2. **Statistical Rigor**
   - Run sufficient repetitions (≥10)
   - Report confidence intervals
   - Use appropriate statistical tests

3. **Fair Comparison**
   - Use same starting points
   - Apply identical convergence criteria
   - Consider problem-specific tolerances

4. **Reporting**
   - Include both tables and visualizations
   - Report failures and edge cases
   - Provide interpretation guidelines

## Troubleshooting

### Common Issues

1. **Font Rendering Errors in Tests**
   - The framework handles missing fonts gracefully
   - Plots are optional for test success

2. **Numerical Instabilities**
   - Check gradient implementations
   - Verify function continuity
   - Use appropriate scaling

3. **Memory Issues**
   - Reduce trace storage frequency
   - Limit problem dimensions
   - Use streaming analysis for large datasets

### Debug Mode

Enable detailed logging:

```rust
env_logger::builder()
.filter_level(log::LevelFilter::Debug)
.init();
```

## References

1. Dolan, E. D., & Moré, J. J. (2002). Benchmarking optimization software with performance profiles. Mathematical
   Programming, 91(2), 201-213.

2. Moré, J. J., Garbow, B. S., & Hillstrom, K. E. (1981). Testing unconstrained optimization software. ACM Transactions
   on Mathematical Software, 7(1), 17-41.

3. Nocedal, J., & Wright, S. (2006). Numerical Optimization. Springer Science & Business Media.