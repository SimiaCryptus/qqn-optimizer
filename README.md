# QQN Optimizer: Quadratic-Quasi-Newton Optimization Algorithm

[![Rust](https://github.com/SimiaCryptus/qqn-optimizer/workflows/Rust/badge.svg)](https://github.com/SimiaCryptus/qqn-optimizer/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/qqn-optimizer.svg)](https://crates.io/crates/qqn-optimizer)
[![Documentation](https://docs.rs/qqn-optimizer/badge.svg)](https://docs.rs/qqn-optimizer)

A comprehensive optimization library implementing the Quadratic-Quasi-Newton (QQN) algorithm alongside a rigorous benchmarking framework for optimization algorithm evaluation.
📄 **[Read the Academic Paper](papers/intro/paper.pdf)** - Complete mathematical foundation and theoretical analysis

http://dx.doi.org/10.13140/RG.2.2.15200.19206


## Table of Contents

* [Overview](#overview)
* [Key Features](#key-features)
* [Installation](#installation)
* [Quick Start](#quick-start)
* [The QQN Algorithm](#the-qqn-algorithm)
* [Benchmarking Framework](#benchmarking-framework)
* [Usage Examples](#usage-examples)
* [Benchmark Results](#benchmark-results)
* [API Documentation](#api-documentation)
* [Contributing](#contributing)
* [Academic Paper](#academic-paper) 📄 **[PDF](papers/intro/paper.pdf)**
* [License](#license)

## Overview

The QQN Optimizer introduces a novel optimization algorithm that combines gradient descent and L-BFGS directions through quadratic interpolation. Unlike traditional approaches that choose between optimization directions or solve expensive subproblems, QQN constructs a smooth parametric path that guarantees descent while adaptively balancing first-order and second-order information.

**Key Innovation**: QQN constructs a quadratic path `d(t) = t(1-t)(-∇f) + t²d_LBFGS` that starts tangent to the gradient direction and curves toward the quasi-Newton direction, then performs univariate optimization along this path.

## Key Features

### Algorithm Capabilities
* **Robust Convergence**: Guaranteed descent property regardless of L-BFGS direction quality
* **No Additional Hyperparameters**: Combines existing methods without introducing new tuning parameters
* **Superlinear Local Convergence**: Inherits L-BFGS convergence properties near optima
* **Multiple Line Search Methods**: Supports Backtracking, Strong Wolfe, Golden Section, Bisection, and more

### Comprehensive Benchmarking
* **62 Benchmark Problems**: Covering convex, non-convex, multimodal, and ML problems
* **25 Optimizer Variants**: QQN, L-BFGS, Trust Region, Gradient Descent, and Adam variants
* **Statistical Rigor**: Automated statistical testing with Welch's t-test and effect size analysis
* **Reproducible Results**: Fixed seeds and deterministic algorithms ensure reproducibility

### Reporting and Analysis
* **Multi-Format Output**: Generates Markdown, LaTeX, CSV, and HTML reports
* **Convergence Visualization**: Automatic generation of convergence plots and performance profiles
* **Statistical Comparison**: Win/loss/tie matrices with significance testing
* **Performance Metrics**: Success rates, function evaluations, and convergence analysis

## Installation

### Prerequisites
* For report generation: `pandoc` and LaTeX distribution with `pdflatex` (optional)
* For OneDNN support: Intel OneDNN library (optional, see [OneDNN Installation](#onednn-installation))

### From Source
```bash
git clone https://github.com/SimiaCryptus/qqn-optimizer.git
cd qqn-optimizer
cargo build --release
```

### OneDNN Installation
For enhanced performance with neural network problems, you can install Intel OneDNN:
```bash
# Ubuntu/Debian systems
./install_onednn.py
# Or install from source
./install_onednn.py --source
# Then build with OneDNN support
cargo build --release --features onednn
```

### Using Docker
```bash
docker build -t qqn-optimizer .
docker run -v $(pwd)/results:/app/results qqn-optimizer benchmark
```

### As a Library
Add to your `Cargo.toml`:
```toml
[dependencies]
qqn-optimizer = { git = "https://github.com/SimiaCryptus/qqn-optimizer.git" }
```

## Quick Start

### Running Benchmarks
```bash
# Run full benchmark suite (may take hours)
cargo run --release -- benchmark

# Run calibration benchmarks (faster, for testing)
cargo run --release -- calibration

# Run specific problem sets
cargo run --release -- benchmark --problems analytic
cargo run --release -- benchmark --problems ml
# Generate reports from existing results
./process_results_md.sh  # Convert markdown to HTML
./process_results_tex.sh # Convert LaTeX tables to PDF
```

### Using QQN in Your Code
```rust
use qqn_optimizer::optimizers::qqn::QQNOptimizer;
use qqn_optimizer::line_search::strong_wolfe::StrongWolfeLineSearch;

// Define your objective function
fn rosenbrock(x: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.len()-1 {
        let a = 1.0 - x[i];
        let b = x[i+1] - x[i] * x[i];
        sum += a * a + 100.0 * b * b;
    }
    sum
}

// Define gradient function
fn rosenbrock_grad(x: &[f64]) -> Vec<f64> {
    let mut grad = vec![0.0; x.len()];
    for i in 0..x.len()-1 {
        grad[i] += -2.0 * (1.0 - x[i]) - 400.0 * x[i] * (x[i+1] - x[i] * x[i]);
        if i > 0 {
            grad[i] += 200.0 * (x[i] - x[i-1] * x[i-1]);
        }
    }
    if x.len() > 1 {
        let last = x.len() - 1;
        grad[last] = 200.0 * (x[last] - x[last-1] * x[last-1]);
    }
    grad
}

// Create and run optimizer
let line_search = StrongWolfeLineSearch::new();
let mut optimizer = QQNOptimizer::new(line_search);

let initial_point = vec![-1.0, 1.0]; // Starting point
let result = optimizer.optimize(
    &rosenbrock,
    &rosenbrock_grad,
    initial_point,
    1000, // max function evaluations
    1e-8  // gradient tolerance
);

println!("Optimum found at: {:?}", result.x);
println!("Function value: {}", result.fx);
println!("Function evaluations: {}", result.num_f_evals);
```

## The QQN Algorithm

### Mathematical Foundation

QQN addresses the fundamental question: given gradient and quasi-Newton directions, how should we combine them? The algorithm constructs a quadratic path satisfying three constraints:

1. **Initial Position**: `d(0) = 0` (starts at current point)
2. **Initial Tangent**: `d'(0) = -∇f(x)` (begins with steepest descent)
3. **Terminal Position**: `d(1) = d_LBFGS` (ends at L-BFGS direction)

This yields the canonical form:
```
d(t) = t(1-t)(-∇f) + t²d_LBFGS
```

### Key Properties

* **Guaranteed Descent**: The initial tangent condition ensures descent regardless of L-BFGS quality
* **Adaptive Interpolation**: Automatically balances first-order and second-order information
* **Robust to Failures**: Gracefully degrades to gradient descent when L-BFGS fails
* **No Additional Parameters**: Uses existing L-BFGS and line search parameters

### Convergence Guarantees

* **Global Convergence**: Under standard assumptions, converges to stationary points
* **Superlinear Local Convergence**: Near optima with positive definite Hessian, achieves superlinear convergence matching L-BFGS

## Benchmarking Framework

### Problem Suite

The benchmark suite includes 62 carefully selected problems across five categories:

* **Convex Functions** (6): Sphere, Matyas, Zakharov variants
* **Non-Convex Unimodal** (12): Rosenbrock, Beale, Levy variants
* **Highly Multimodal** (24): Rastrigin, Ackley, Michalewicz, StyblinskiTang
* **ML-Convex** (8): Linear regression, logistic regression, SVM
* **ML-Non-Convex** (9): Neural networks with varying architectures

### Statistical Analysis

The framework employs rigorous statistical methods:

* **Multiple Runs**: 50 runs per problem-optimizer pair for statistical validity
* **Welch's t-test**: For comparing means with unequal variances
* **Cohen's d**: For measuring effect sizes
* **Bonferroni Correction**: For multiple comparison adjustment
* **Win/Loss/Tie Analysis**: Comprehensive pairwise comparisons

### Evaluation Methodology

1. **Calibration Phase**: Determines problem-specific convergence thresholds
2. **Benchmarking Phase**: Evaluates all optimizers with consistent criteria
3. **Statistical Analysis**: Automated significance testing and effect size calculation
4. **Report Generation**: Multi-format output with visualizations

## Usage Examples

### Custom Optimizer Implementation

```rust
use qqn_optimizer::optimizers::traits::Optimizer;
use qqn_optimizer::line_search::backtracking::BacktrackingLineSearch;

struct MyCustomOptimizer {
    line_search: BacktrackingLineSearch,
}

impl Optimizer for MyCustomOptimizer {
    fn optimize<F, G>(
        &mut self,
        f: &F,
        grad: &G,
        x0: Vec<f64>,
        max_f_evals: usize,
        grad_tol: f64,
    ) -> OptimizationResult
    where
        F: Fn(&[f64]) -> f64,
        G: Fn(&[f64]) -> Vec<f64>,
    {
        // Your optimization logic here
        todo!()
    }
}
```

### Running Specific Benchmarks

```rust
use qqn_optimizer::benchmarks::evaluation::run_benchmark;
use qqn_optimizer::problem_sets::analytic_problems;
use qqn_optimizer::optimizer_sets::qqn_variants;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problems = analytic_problems();
    let optimizers = qqn_variants();

    run_benchmark(
        "my_benchmark_",
        1000,  // max function evaluations
        10,    // number of runs
        Duration::from_secs(60), // timeout
        problems,
        optimizers,
    ).await?;

    Ok(())
}
```

### Custom Problem Definition

```rust
use qqn_optimizer::benchmarks::evaluation::ProblemSpec;

fn my_custom_problem() -> ProblemSpec {
    ProblemSpec {
        name: "MyProblem".to_string(),
        function: Box::new(|x: &[f64]| {
            // Your objective function
            x.iter().map(|xi| xi * xi).sum()
        }),
        gradient: Box::new(|x: &[f64]| {
            // Your gradient function
            x.iter().map(|xi| 2.0 * xi).collect()
        }),
        initial_point: vec![1.0, 1.0, 1.0],
        bounds: None, // Optional bounds
        global_minimum: Some(0.0), // Known global minimum
    }
}
```

## Benchmark Results

### Overall Performance

Based on comprehensive evaluation across 62 problems with over 31,000 optimization runs:

* **QQN Dominance**: QQN variants won 36 out of 62 problems (58%)
* **Top Performers**:
    * QQN-Bisection-1: 8 wins
    * QQN-StrongWolfe: 7 wins
    * L-BFGS: 6 wins
    * QQN-GoldenSection: 6 wins

### Performance by Problem Type

**Convex Problems**:
* QQN-Bisection: 100% success on Sphere problems with 12-16 evaluations
* L-BFGS: 100% success on Sphere_10D with only 15 evaluations

**Non-Convex Problems**:
* QQN-StrongWolfe: 35% success on Rosenbrock_5D (best among all)
* QQN-GoldenSection: 100% success on Beale_2D

**Multimodal Problems**:
* QQN-StrongWolfe: 90% success on StyblinskiTang_2D
* Adam-Fast: Best on Michalewicz functions (45-60% success)

**Machine Learning Problems**:
* Adam-Fast: Best on neural networks (32.5-60% success)
* L-BFGS variants: 100% success on SVM problems

### Key Insights

1. **Robustness**: QQN maintains consistent performance across problem types
2. **Efficiency**: Competitive function evaluation counts with high success rates
3. **Scalability**: Performance degrades gracefully with dimensionality
4. **Specialization**: Some algorithms excel on specific problem classes

## API Documentation

### Core Traits

```rust
pub trait Optimizer {
    fn optimize<F, G>(
        &mut self,
        f: &F,
        grad: &G,
        x0: Vec<f64>,
        max_f_evals: usize,
        grad_tol: f64,
    ) -> OptimizationResult;
}

pub trait LineSearch {
    fn search<F, G>(
        &mut self,
        f: &F,
        grad: &G,
        x: &[f64],
        fx: f64,
        gx: &[f64],
        direction: &[f64],
    ) -> LineSearchResult;
}
```

### QQN Optimizer Variants

* `QQNOptimizer<BacktrackingLineSearch>`: Basic backtracking line search
* `QQNOptimizer<StrongWolfeLineSearch>`: Strong Wolfe conditions
* `QQNOptimizer<GoldenSectionLineSearch>`: Golden section search
* `QQNOptimizer<BisectionLineSearch>`: Bisection on derivative
* `QQNOptimizer<MoreThuenteLineSearch>`: Moré-Thuente line search

### Benchmarking API

```rust
// Run benchmark with custom configuration
pub async fn run_benchmark(
    prefix: &str,
    max_evals: usize,
    num_runs: usize,
    timeout: Duration,
    problems: Vec<ProblemSpec>,
    optimizers: Vec<OptimizerSpec>,
) -> Result<(), Box<dyn Error + Send + Sync>>;

// Generate reports from benchmark results
pub fn generate_reports(
    results_dir: &str,
    output_formats: &[ReportFormat],
) -> Result<(), Box<dyn Error>>;
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

```bash
git clone https://github.com/SimiaCryptus/qqn-optimizer.git
cd qqn-optimizer
cargo build
cargo test
```
### Benchmark Report Processing
The project includes scripts to process benchmark results into various formats:
```bash
# Process markdown reports to HTML
./process_results_md.sh
# Process LaTeX table exports to PDF
./process_results_tex.sh
```
These scripts automatically:
* Convert `.md` files to `.html` with proper link updates
* Compile `.tex` files to `.pdf` using pdflatex
* Handle recursive directory processing
* Provide detailed logging and error handling


### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test benchmark_reports

# Benchmark tests (slow)
cargo test --release calibration
# Test with OneDNN support (if installed)
cargo test --release --features onednn
```

### Code Style

We use `rustfmt` and `clippy` for code formatting and linting:

```bash
cargo fmt
cargo clippy -- -D warnings
```

## Academic Paper
📄 **[Download Full Paper (PDF)](papers/intro/paper.pdf)**


This work is documented in our academic paper (in preparation):

**"Quadratic-Quasi-Newton Optimization: Combining Gradient and Quasi-Newton Directions Through Quadratic Interpolation"**

The paper provides:
* Complete mathematical derivation of the QQN algorithm
* Theoretical convergence analysis
* Comprehensive experimental evaluation
* Comparison with existing optimization methods

Paper draft and supplementary materials available in the [`papers/`](papers/) directory. **[Direct link to paper PDF](papers/intro/paper.pdf)**.

## Citing This Work

If you use QQN Optimizer in your research, please cite:

```bibtex
@article{qqn2024,
  title={Quadratic-Quasi-Newton Optimization: Combining Gradient and Quasi-Newton Directions Through Quadratic Interpolation},
  author={[Author Name]},
  journal={[Journal Name]},
  year={2024},
  url={https://github.com/SimiaCryptus/qqn-optimizer/}
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

* The QQN algorithm was originally developed in 2017
* AI language models assisted in documentation and benchmarking framework development
* Thanks to the Rust optimization community for inspiration and feedback

## Support

* **Documentation**: [API Docs](https://docs.rs/qqn-optimizer) (when published)

## Project Structure

```
qqn-optimizer/
├── src/                    # Core library source code
│   ├── optimizers/        # Optimizer implementations
│   ├── line_search/       # Line search algorithms
│   ├── benchmarks/        # Benchmarking framework
│   └── problem_sets/      # Test problem definitions
├── papers/                # Academic paper drafts
├── results/               # Benchmark results (generated)
├── scripts/               # Utility scripts
├── process_results_*.sh   # Report processing scripts
├── install_onednn.py      # OneDNN installation script
└── Dockerfile            # Container configuration
```

---

**Note**: This is research software. While we strive for correctness and performance, please validate results for your specific use case. The benchmarking framework is designed to facilitate fair comparison and reproducible research in optimization algorithms.