# QQN Optimizer Research Framework

A Rust-based implementation of the Quadratic Quasi-Newton (QQN) optimization algorithm for academic research and publication. This framework provides a complete optimization toolkit with benchmarking capabilities, designed to validate QQN's theoretical contributions and enable reproducible research results.

## Overview

The Quadratic Quasi-Newton (QQN) algorithm addresses reliability issues in L-BFGS by detecting when the quasi-Newton approximation may be unreliable and smoothly blending it with the guaranteed descent direction of the gradient using quadratic interpolation.

### Key Features

- **Complete QQN Implementation**: Magnitude-based switching with quadratic path construction
- **Comprehensive Benchmarking**: Mathematical functions and machine learning problems
- **Statistical Analysis**: Rigorous performance comparison with significance testing
- **Visualization Tools**: Convergence plots and performance analysis charts
- **Reproducible Research**: Deterministic execution with configurable experiments

## Quick Start

### Installation

```bash
# Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install libfontconfig1-dev pkg-config
# Install system dependencies (macOS)
brew install fontconfig pkg-config
# Install system dependencies (CentOS/RHEL/Fedora)
sudo yum install fontconfig-devel pkgconfig
# or for newer versions:
sudo dnf install fontconfig-devel pkgconfig
# Clone and build
git clone https://github.com/SimiaCryptus/qqn-optimizer
cd qqn-optimizer
cargo build --release
```

### Basic Usage

```bash
# Run comprehensive benchmark suite
./target/release/qqn-optimizer benchmark -c experiments/configs/qqn_validation.yaml -o results/ -v

# Analyze results
./target/release/qqn-optimizer analyze -r results/qqn_validation -a convergence

# Generate plots
./target/release/qqn-optimizer plot -r results/qqn_validation -p performance -f png

# Validate implementation
./target/release/qqn-optimizer validate --math-tests --perf-tests
```

### Programmatic Usage

```rust
use qqn_optimizer::{QQNOptimizer, QQNConfig, RosenbrockFunction, OptimizationProblem};

// Create optimizer
let config = QQNConfig {
    threshold: 0.01,
    lbfgs_history: 10,
    ..Default::default()
};
let mut optimizer = QQNOptimizer::new(config);

// Define problem
let problem = RosenbrockFunction::new(2);
let mut x = problem.initial_point();

// Optimization loop
for iteration in 0..1000 {
    let gradient = problem.gradient(&x)?;
    let step_result = optimizer.step(&mut x, &gradient)?;

    if step_result.convergence_info.converged {
        println!("Converged in {} iterations", iteration);
        break;
    }
}
```

## Algorithm Overview

QQN combines the efficiency of L-BFGS with the reliability of gradient descent through:

1. **Magnitude Comparison**: Compare L-BFGS direction magnitude with gradient magnitude
2. **Adaptive Switching**: Use threshold τ to decide between L-BFGS and hybrid approach
3. **Quadratic Interpolation**: Smooth blending via d(t) = t(1-t)g_scaled + t²d_lbfgs
4. **Guaranteed Descent**: Maintains descent property while leveraging curvature information

## Benchmarking Suite

### Mathematical Functions
- **Rosenbrock**: Classic non-convex optimization test
- **Rastrigin**: Highly multimodal function
- **Sphere**: Simple convex quadratic
- **Beale**: Two-dimensional non-convex function

### Machine Learning Problems
- **Logistic Regression**: Binary classification with regularization
- **Neural Networks**: Multi-layer perceptron training
- **Custom Problems**: Extensible framework for new problems

### Baseline Optimizers
- **L-BFGS**: Limited-memory quasi-Newton method
- **Adam**: Adaptive moment estimation
- **SGD**: Stochastic gradient descent with momentum

## Configuration

Experiments are configured via YAML files:

```yaml
name: "QQN Validation Study"
description: "Comprehensive benchmarking of QQN vs baselines"

problems:
  - name: "rosenbrock_100d"
    problem_type:
      Rosenbrock:
        dimension: 100

optimizers:
  - name: "qqn_default"
    optimizer_type:
      QQN:
        threshold: 0.01
        lbfgs_history: 10

benchmark_settings:
  max_iterations: 1000
  tolerance: 1e-6
  num_runs: 10
  random_seed: 42
```

## Performance Requirements

| Problem Size | Max Time/Step | Memory Limit |
|--------------|---------------|--------------|
| 100D | 1ms | 10MB |
| 1,000D | 10ms | 100MB |
| 10,000D | 100ms | 1GB |

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out html

# Performance benchmarks
cargo bench

# Property-based tests
cargo test --features proptest
```

## Documentation

- **API Documentation**: `cargo doc --open`
- **User Guide**: [docs/user_guide.md](docs/user_guide.md)
- **Algorithm Details**: [docs/algorithm.md](docs/algorithm.md)
- **Examples**: [examples/](examples/)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Citation

If you use this software in your research, please cite:

```bibtex
@software{qqn_optimizer_2024,
  title = {QQN Optimizer: A Rust Implementation of Quadratic Quasi-Newton Optimization},
  author = {Your Name},
  year = {2024},
  url = {https://github.com/your-org/qqn-optimizer},
  version = {1.0.0}
}
```

## Acknowledgments

- L-BFGS implementation inspired by Nocedal & Wright (2006)
- Benchmarking framework follows optimization research best practices
- Statistical analysis methods from Moré & Wild (2009)

## References

1. Liu, D. C., & Nocedal, J. (1989). On the limited memory BFGS method for large scale optimization. Mathematical Programming, 45(1-3), 503-528.

2. Nocedal, J., & Wright, S. (2006). Numerical optimization. Springer Science & Business Media.

3. Moré, J. J., & Wild, S. M. (2009). Benchmarking derivative-free optimization algorithms. SIAM Journal on Optimization, 20(1), 172-191.