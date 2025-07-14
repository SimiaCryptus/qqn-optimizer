# QQN Optimizer User Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Basic Usage](#basic-usage)
3. [Running Benchmarks](#running-benchmarks)
4. [Generating Academic Artifacts](#generating-academic-artifacts)
5. [Advanced Configuration](#advanced-configuration)
6. [Troubleshooting](#troubleshooting)

## Getting Started

### Installation

1. **Prerequisites**
   - Rust 1.70 or later
   - Git
   - (Optional) LaTeX distribution for PDF generation

2. **Clone and Build**
   ```bash
   git clone https://github.com/yourusername/qqn-optimizer
   cd qqn-optimizer
   cargo build --release
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

### Quick Start Example

```rust
use qqn_optimizer::{QQNOptimizer, QQNConfig};
use qqn_optimizer::benchmarks::functions::RosenbrockFunction;
use candle_core::{Device, Tensor};

fn main() -> anyhow::Result<()> {
    // Initialize logging
    qqn_optimizer::init_logging()?;

    // Create optimizer
    let config = QQNConfig::default();
    let mut optimizer = QQNOptimizer::new(config);

    // Define problem
    let problem = RosenbrockFunction::new(2);

    // Initial parameters
    let mut x = vec![Tensor::from_slice(
        &problem.initial_point(),
        &[2],
        &Device::Cpu
    )?];

    // Optimization loop
    for i in 0..100 {
        let gradients = compute_gradients(&x, &problem)?;
        let result = optimizer.step_with_gradients(&mut x, &gradients)?;

        if result.convergence_info.converged {
            println!("Converged at iteration {}", i);
            break;
        }
    }

    Ok(())
}
```

## Basic Usage

### Using QQN Optimizer

```rust
use qqn_optimizer::{QQNOptimizer, QQNConfig};

// Configure QQN
let mut config = QQNConfig::default();
config.lbfgs_history = 10;  // L-BFGS memory size
config.epsilon = 1e-8;      // Numerical stability

// Create optimizer
let mut optimizer = QQNOptimizer::new(config);

// Optimize with custom function
let function = MyObjectiveFunction::new();
optimizer.step(&mut parameters, &function)?;
```

### Comparing with L-BFGS

```rust
use qqn_optimizer::{LBFGSOptimizer, LBFGSConfig};

// Create L-BFGS for comparison
let lbfgs_config = LBFGSConfig::default();
let mut lbfgs = LBFGSOptimizer::new(lbfgs_config);

// Run both optimizers
let qqn_result = run_optimizer(&mut qqn_optimizer, &problem);
let lbfgs_result = run_optimizer(&mut lbfgs, &problem);

// Compare results
println!("QQN iterations: {}", qqn_result.iterations);
println!("L-BFGS iterations: {}", lbfgs_result.iterations);
```

## Running Benchmarks

### Command Line Interface

```bash
# Run comprehensive benchmarks
cargo run --release --bin benchmark -- --output results/

# Run specific problem set
cargo run --release --bin benchmark -- \
    --problems rosenbrock,rastrigin \
    --dimensions 2,10,50 \
    --output results/

# Run with custom configuration
cargo run --release --bin benchmark -- \
    --config experiments/my_config.yaml \
    --output results/
```

### Programmatic Benchmarking

```rust
use qqn_optimizer::benchmarks::{BenchmarkRunner, BenchmarkConfig};
use qqn_optimizer::benchmarks::functions::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure benchmarks
    let config = BenchmarkConfig {
        max_iterations: 1000,
        tolerance: 1e-6,
        num_runs: 30,  // For statistical significance
        ..Default::default()
    };

    // Create benchmark runner
    let runner = BenchmarkRunner::new(config);

    // Define problems
    let problems: Vec<Box<dyn OptimizationProblem>> = vec![
        Box::new(RosenbrockFunction::new(10)),
        Box::new(RastriginFunction::new(10)),
        Box::new(SphereFunction::new(10)),
    ];

    // Define optimizers
    let optimizers = vec![
        create_qqn_optimizer(),
        create_lbfgs_optimizer(),
    ];

    // Run benchmarks
    let results = runner.run_benchmarks(problems, optimizers).await?;

    // Save results
    results.save_to_file(Path::new("results/benchmark_results.json"))?;

    Ok(())
}
```

## Generating Academic Artifacts

### 1. Comprehensive Benchmark Report

```bash
# Generate full academic report with all artifacts
cargo test --release benchmark_experiments::test_comprehensive_benchmarks

# This generates:
# - HTML report with all results
# - LaTeX tables for papers
# - Performance plots (PNG/PDF)
# - CSV data for external analysis
```

### 2. LaTeX Tables for Papers

```rust
use qqn_optimizer::analysis::reporting::LaTeXExporter;

// After running benchmarks
let results = load_benchmark_results("results/benchmark_results.json")?;
let analysis = StatisticalAnalysis::new(&results);

// Generate LaTeX tables
let latex_exporter = LaTeXExporter::new();
let performance_table = latex_exporter.export_performance_table(&analysis)?;
let significance_table = latex_exporter.export_significance_table(&analysis)?;

// Save to files
fs::write("tables/performance.tex", performance_table)?;
fs::write("tables/significance.tex", significance_table)?;
```

### 3. Publication-Ready Plots

```rust
use qqn_optimizer::analysis::plotting::{PlottingEngine, PlotConfig};

// Configure plotting for publication
let plot_config = PlotConfig {
    width: 800,
    height: 600,
    output_format: "pdf".to_string(),  // For LaTeX
    color_scheme: "academic".to_string(),
};

let plotter = PlottingEngine::new("plots/".to_string())
    .with_config(plot_config);

// Generate convergence plots
plotter.convergence_plot(&traces, "convergence_comparison")?;

// Generate performance profiles
plotter.performance_profiles(&profiles, "performance_profiles")?;

// Generate box plots for statistical analysis
plotter.performance_boxplot(&results, "algorithm_comparison")?;
```

### 4. Statistical Analysis Report

```rust
use qqn_optimizer::analysis::statistics::StatisticalAnalysis;

// Perform comprehensive analysis
let analysis = StatisticalAnalysis::new(&benchmark_results);

// Generate academic report sections
let report = AcademicReport::new()
    .with_title("QQN Optimizer: Experimental Validation")
    .add_section("Methodology", &methodology_text)
    .add_section("Results", &analysis.summary())
    .add_section("Statistical Tests", &analysis.significance_report())
    .add_section("Performance Profiles", &analysis.performance_analysis());

// Export to LaTeX
report.to_latex_file("paper/results_section.tex")?;

// Export to Markdown for README
report.to_markdown_file("results_summary.md")?;
```

### 5. Reproducibility Package

```bash
# Generate complete reproducibility package
cargo run --release --bin generate_reproducibility_package -- \
    --output reproducibility/

# This creates:
# reproducibility/
# ├── README.md           # Instructions
# ├── config/            # Experiment configurations
# ├── data/              # Raw results
# ├── scripts/           # Analysis scripts
# ├── figures/           # All plots
# └── tables/            # LaTeX tables
```

## Advanced Configuration

### Custom Experiment Configuration

Create `experiment.yaml`:

```yaml
name: "QQN vs Baselines Comprehensive Study"
description: "Comparing QQN with state-of-the-art optimizers"

problems:
  - name: "rosenbrock_10d"
    type: Rosenbrock
    dimension: 10

  - name: "rastrigin_20d"
    type: Rastrigin
    dimension: 20

  - name: "neural_net_mnist"
    type: NeuralNetwork
    architecture:
      layers: [784, 128, 64, 10]
      activation: "relu"

optimizers:
  - name: "qqn_default"
    type: QQN
    lbfgs_history: 10
    line_search:
      method: StrongWolfe
      c1: 1e-4
      c2: 0.9

  - name: "lbfgs_baseline"
    type: LBFGS
    history: 10

  - name: "adam_baseline"
    type: Adam
    learning_rate: 0.001
    beta1: 0.9
    beta2: 0.999

benchmark:
  max_iterations: 1000
  tolerance: 1e-6
  num_runs: 50
  parallel_runs: true
  random_seed: 42

analysis:
  statistical_tests: ["TTest", "MannWhitney", "Wilcoxon"]
  confidence_level: 0.95
  performance_profiles: true
  convergence_analysis: true

output:
  results_dir: "results/comprehensive_study"
  generate_plots: true
  plot_formats: ["png", "pdf", "svg"]
  export_csv: true
  latex_tables: true
```

### Running Custom Experiments

```rust
use qqn_optimizer::config::ExperimentConfig;
use qqn_optimizer::experiments::ExperimentRunner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = ExperimentConfig::from_file("experiment.yaml")?;

    // Create and run experiment
    let runner = ExperimentRunner::new(config);
    let results = runner.run().await?;

    // Generate all artifacts
    runner.generate_artifacts(&results)?;

    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **Font/Plotting Issues in Tests**
   ```bash
   # If you see font-related errors:
   export MPLBACKEND=Agg  # Use non-interactive backend
   cargo test
   ```

2. **Memory Issues with Large Problems**
   ```rust
   // Reduce L-BFGS history size
   config.lbfgs_history = 5;  // Instead of default 10
   ```

3. **Convergence Issues**
   ```rust
   // Adjust line search parameters
   config.line_search.c1 = 1e-4;  // More lenient Armijo
   config.line_search.initial_step = 0.1;  // Smaller initial step
   ```

### Performance Tips

1. **Enable Release Mode**
   ```bash
   cargo build --release
   cargo run --release
   ```

2. **Parallel Benchmarking**
   ```rust
   config.parallel_runs = true;
   config.num_threads = 8;  // Adjust based on CPU
   ```

3. **Profiling**
   ```bash
   cargo flamegraph --bin benchmark -- --problems sphere --dimensions 100
   ```

### Getting Help

- **Documentation**: Run `cargo doc --open`
- **Examples**: See `examples/` directory
- **Issues**: GitHub Issues page
- **Academic Questions**: See paper citations in README

## Example: Complete Academic Study

Here's a complete example for generating all artifacts for a paper:

```bash
#!/bin/bash
# run_academic_study.sh

# 1. Run comprehensive benchmarks
cargo run --release --bin academic_study -- \
    --config configs/academic_study.yaml \
    --output results/study_$(date +%Y%m%d)

# 2. Generate LaTeX tables
cargo run --release --bin generate_tables -- \
    --input results/study_*/benchmark_results.json \
    --output paper/tables/

# 3. Generate plots
cargo run --release --bin generate_plots -- \
    --input results/study_*/benchmark_results.json \
    --output paper/figures/ \
    --format pdf \
    --style academic

# 4. Run statistical analysis
cargo run --release --bin statistical_analysis -- \
    --input results/study_*/benchmark_results.json \
    --output paper/analysis/ \
    --tests all \
    --confidence 0.95

# 5. Generate reproducibility package
cargo run --release --bin package_results -- \
    --study results/study_* \
    --output reproducibility.zip

echo "Academic artifacts generated successfully!"
echo "LaTeX tables: paper/tables/"
echo "Figures: paper/figures/"
echo "Analysis: paper/analysis/"
echo "Reproducibility: reproducibility.zip"
```

This will generate everything needed for academic publication, including supplementary materials for reproducibility.
