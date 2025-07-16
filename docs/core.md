# QQN Optimizer: Technical Documentation

## Table of Contents

1. [Mathematical Foundation](#mathematical-foundation)
2. [System Architecture](#system-architecture)
3. [Implementation Details](#implementation-details)
4. [API Reference](#api-reference)
5. [Performance Analysis](#performance-analysis)
6. [Extension Guide](#extension-guide)

---

## 1. Mathematical Foundation

### 1.1 Overview

The Quadratic-Quadratic Newton (QQN) optimizer is a novel quasi-Newton optimization method that combines gradient
descent and L-BFGS directions through a quadratic interpolation path. This approach aims to achieve faster convergence
than traditional methods by exploring a richer search space.

### 1.2 Core Algorithm

#### 1.2.1 Quadratic Path Construction

Given:

- Current gradient: $\mathbf{g}_k \in \mathbb{R}^n$
- L-BFGS direction: $\mathbf{d}_k^{\text{LBFGS}} \in \mathbb{R}^n$

The QQN method constructs a parametric quadratic path:

$$\mathbf{d}(t) = t(1-t)(-\mathbf{g}_k) + t^2\mathbf{d}_k^{\text{LBFGS}}, \quad t \in [0,1]$$

**Properties:**

- $\mathbf{d}(0) = \mathbf{0}$ (no movement at $t=0$)
- $\mathbf{d}(1) = \mathbf{d}_k^{\text{LBFGS}}$ (full L-BFGS step at $t=1$)
- $\mathbf{d}'(0) = -\mathbf{g}_k$ (initial direction is steepest descent)

#### 1.2.2 Optimal Parameter Selection

The optimal parameter $t^*$ is found by solving:

$$t^* = \arg\min_{t \in [0,1]} f(\mathbf{x}_k + \mathbf{d}(t))$$

This is accomplished using line search along the quadratic path.

#### 1.2.3 Parameter Update

Once $t^*$ is determined:

$$\mathbf{x}_{k+1} = \mathbf{x}_k + \mathbf{d}(t^*)$$

### 1.3 L-BFGS Component

#### 1.3.1 Two-Loop Recursion

The L-BFGS direction is computed using the two-loop recursion algorithm:

```
Algorithm: L-BFGS Two-Loop Recursion
Input: gradient g_k, history {s_i, y_i}_{i=k-m}^{k-1}
Output: search direction d_k

1. q ← g_k
2. For i = k-1, k-2, ..., k-m:
   a. ρ_i ← 1/(y_i^T s_i)
   b. α_i ← ρ_i s_i^T q
   c. q ← q - α_i y_i
3. γ_k ← (s_{k-1}^T y_{k-1})/(y_{k-1}^T y_{k-1})
4. r ← γ_k q
5. For i = k-m, k-m+1, ..., k-1:
   a. β ← ρ_i y_i^T r
   b. r ← r + s_i(α_i - β)
6. d_k ← -r
```

Where:

- $\mathbf{s}_i = \mathbf{x}_{i+1} - \mathbf{x}_i$ (parameter differences)
- $\mathbf{y}_i = \mathbf{g}_{i+1} - \mathbf{g}_i$ (gradient differences)
- $m$ is the history size (typically 3-20)

#### 1.3.2 Curvature Condition

Updates are only stored if they satisfy the curvature condition:

$$\mathbf{s}_k^T \mathbf{y}_k > \epsilon \|\mathbf{s}_k\|_2 \|\mathbf{y}_k\|_2$$

where $\epsilon > 0$ is a small constant (typically $10^{-8}$).

### 1.4 Line Search Methods

#### 1.4.1 Strong Wolfe Conditions

The step size $\alpha$ must satisfy:

1. **Armijo condition** (sufficient decrease):
   $$f(\mathbf{x}_k + \alpha \mathbf{p}_k) \leq f(\mathbf{x}_k) + c_1 \alpha \nabla f(\mathbf{x}_k)^T \mathbf{p}_k$$

2. **Curvature condition**:
   $$|\nabla f(\mathbf{x}_k + \alpha \mathbf{p}_k)^T \mathbf{p}_k| \leq c_2 |\nabla f(\mathbf{x}_k)^T \mathbf{p}_k|$$

where $0 < c_1 < c_2 < 1$ (typically $c_1 = 10^{-4}$, $c_2 = 0.9$).

#### 1.4.2 Bisection Line Search

For the quadratic path, we use bisection to find where the directional derivative is zero:

$$\frac{d}{dt}f(\mathbf{x}_k + \mathbf{d}(t)) = \nabla f(\mathbf{x}_k + \mathbf{d}(t))^T \mathbf{d}'(t) = 0$$

### 1.5 Convergence Analysis

#### 1.5.1 Theoretical Properties

Under standard assumptions (Lipschitz continuous gradient, strong convexity), QQN exhibits:

1. **Global convergence**: The algorithm converges to a stationary point from any starting point
2. **Superlinear convergence**: Near the optimum, convergence rate approaches that of Newton's method
3. **Robustness**: The quadratic interpolation provides additional flexibility in non-convex regions

#### 1.5.2 Complexity Analysis

- **Per-iteration cost**: $O(mn)$ where $m$ is history size, $n$ is problem dimension
- **Memory requirement**: $O(mn)$ for storing L-BFGS history
- **Function evaluations**: Typically 1-5 per iteration (line search dependent)

---

## 2. System Architecture

### 2.1 High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                      QQN Optimizer System                     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │   Core      │  │  Benchmarks  │  │    Analysis      │   │
│  │  Optimizer  │  │   Framework  │  │    Framework     │   │
│  │             │  │              │  │                  │   │
│  │ • QQN       │  │ • Problems   │  │ • Statistics     │   │
│  │ • L-BFGS    │  │ • Runner     │  │ • Plotting       │   │
│  │ • Line      │  │ • Metrics    │  │ • Reporting      │   │
│  │   Search    │  │              │  │                  │   │
│  └──────┬──────┘  └──────┬───────┘  └────────┬─────────┘   │
│         │                 │                    │             │
│  ┌──────┴─────────────────┴────────────────────┴────────┐   │
│  │                    Common Infrastructure              │   │
│  │                                                       │   │
│  │  • Tensor Operations  • Math Utilities               │   │
│  │  • Configuration      • Error Handling               │   │
│  │  • Logging           • Serialization                │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Module Organization

#### 2.2.1 Core Modules

```
src/core/
├── mod.rs           # Module exports and error types
├── optimizer.rs     # Optimizer trait and common types
├── lbfgs.rs        # L-BFGS implementation
├── qqn.rs          # QQN optimizer implementation
├── line_search.rs  # Line search algorithms
└── sgd.rs          # SGD implementation (for comparison)
```

#### 2.2.2 Key Traits and Interfaces

```rust
/// Core optimizer trait
pub trait Optimizer: Send + Sync + Debug {
    type Config: Clone + Debug + Send + Sync;
    type State: Clone + Debug + Send + Sync;

    fn new(config: Self::Config) -> Self;
    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
    ) -> Result<StepResult>;
    fn reset(&mut self);
    fn state(&self) -> &Self::State;
    fn name(&self) -> &str;
}

/// Differentiable function interface
pub trait DifferentiableFunction: Send + Sync {
    fn evaluate(&self, params: &[Tensor]) -> Result<f64>;
    fn gradient(&self, params: &[Tensor]) -> Result<Vec<Tensor>>;
}

/// Line search trait
pub trait LineSearch: Send + Sync + Debug {
    fn search(
        &mut self,
        params: &[Tensor],
        direction: &[Tensor],
        gradients: &[Tensor],
        function: &dyn DifferentiableFunction,
    ) -> Result<LineSearchResult>;
}
```

### 2.3 Data Flow

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Parameters  │────▶│   Gradient   │────▶│   L-BFGS     │
│     x_k      │     │  Computation │     │  Direction   │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                   │
                     ┌──────────────┐              ▼
                     │   Quadratic  │     ┌──────────────┐
                     │     Path     │◀────│   Quadratic  │
                     │   d(t)       │     │ Construction │
                     └──────┬───────┘     └──────────────┘
                            │
                            ▼
                     ┌──────────────┐
                     │ Line Search  │
                     │  Find t*     │
                     └──────┬───────┘
                            │
                            ▼
                     ┌──────────────┐
                     │   Update     │
                     │  x_{k+1}     │
                     └──────────────┘
```

### 2.4 Memory Management

#### 2.4.1 L-BFGS History Storage

```rust
pub struct LBFGSState {
    s_history: VecDeque<Vec<Tensor>>,  // Parameter differences
    y_history: VecDeque<Vec<Tensor>>,  // Gradient differences
    rho_history: VecDeque<f64>,        // Curvature information
    prev_gradient: Option<Vec<Tensor>>, // Previous gradient
    iteration: usize,
    gamma: f64,                        // Hessian scaling factor
}
```

Memory usage: $O(2mn)$ where $m$ is history size, $n$ is parameter count.

#### 2.4.2 Tensor Lifecycle

1. **Creation**: Tensors created on specified device (CPU/GPU)
2. **Computation**: In-place operations where possible
3. **Cleanup**: Automatic via Rust's ownership system

### 2.5 Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum OptError {
    #[error("Tensor operation failed: {0}")]
    TensorError(#[from] candle_core::Error),

    #[error("Numerical error: {0}")]
    NumericalError(String),

    #[error("Convergence error: {0}")]
    ConvergenceError(String),

    #[error("Line search failed: {0}")]
    LineSearchError(String),
}
```

---

## 3. Implementation Details

### 3.1 QQN Step Implementation

```rust
fn step(&mut self, params: &mut [Tensor], function: &dyn DifferentiableFunction)
        -> CandleResult<StepResult> {
    // 1. Compute gradients
    let gradients = function.gradient(params)?;

    // 2. Check if we should use L-BFGS
    if self.state.iteration < self.config.min_lbfgs_iterations {
        return self.steepest_descent_step(params, &gradients, function);
    }

    // 3. Compute L-BFGS direction
    let lbfgs_direction = self.state.lbfgs_state.compute_direction(&gradients)?;

    // 4. Create quadratic path
    let quadratic_path = self.create_quadratic_path(&gradients, &lbfgs_direction)?;

    // 5. Find optimal t using line search
    let line_search_result = self.find_optimal_t_line_search(
        params, &quadratic_path, &gradients, function
    )?;

    // 6. Compute final direction
    let direction = quadratic_path.evaluate(line_search_result.step_size)?;

    // 7. Update parameters
    for (param, dir) in params.iter_mut().zip(direction.iter()) {
        *param = param.add(dir)?;
    }

    // 8. Update L-BFGS state
    self.state.lbfgs_state.update(&gradients, &direction, 1.0)?;

    self.state.iteration += 1;

    Ok(StepResult { /* ... */ })
}
```

### 3.2 Numerical Stability

#### 3.2.1 Gradient Clipping

```rust
fn clip_gradients(gradients: &mut [Tensor], max_norm: f64) -> CandleResult<()> {
    let grad_norm = compute_magnitude(gradients)?;
    if grad_norm > max_norm {
        let scale = max_norm / grad_norm;
        for grad in gradients.iter_mut() {
            *grad = grad.affine(scale, 0.0)?;
        }
    }
    Ok(())
}
```

#### 3.2.2 Numerical Checks

```rust
fn validate_direction(direction: &[Tensor]) -> CandleResult<()> {
    for tensor in direction {
        let values = tensor.flatten_all()?.to_vec1::<f64>()?;
        if values.iter().any(|&x| !x.is_finite()) {
            return Err(candle_core::Error::Msg("Non-finite direction".into()));
        }
    }
    Ok(())
}
```

### 3.3 Performance Optimizations

#### 3.3.1 Vectorized Operations

```rust
// Efficient dot product using BLAS-like operations
pub fn dot_product(a: &[Tensor], b: &[Tensor]) -> CandleResult<f64> {
    let mut result = 0.0;
    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        // Flatten and compute dot product in one pass
        let flat_a = tensor_a.flatten_all()?;
        let flat_b = tensor_b.flatten_all()?;
        result += (flat_a * flat_b)?.sum_all()?.to_scalar::<f64>()?;
    }
    Ok(result)
}
```

#### 3.3.2 Memory Reuse

```rust
pub struct TensorPool {
    available: Vec<Tensor>,
    device: Device,
}

impl TensorPool {
    pub fn get(&mut self, shape: &[usize]) -> CandleResult<Tensor> {
        // Reuse existing tensor if available
        if let Some(tensor) = self.available.pop() {
            if tensor.shape() == shape {
                return Ok(tensor);
            }
        }
        // Create new tensor if needed
        Tensor::zeros(shape, DType::F64, &self.device)
    }
}
```

---

## 4. API Reference

### 4.1 Creating an Optimizer

```rust
use qqn_optimizer::{QQNOptimizer, QQNConfig, LineSearchConfig};

// Basic usage
let optimizer = QQNOptimizer::new(QQNConfig::default ());

// Custom configuration
let config = QQNConfig {
lbfgs_history: 20,
min_lbfgs_iterations: 3,
line_search: LineSearchConfig {
method: LineSearchMethod::Bisection,
max_iterations: 50,
..Default::default ()
},
epsilon: 1e-8,
verbose: true,
};
let optimizer = QQNOptimizer::new(config);
```

### 4.2 Optimization Loop

```rust
use qqn_optimizer::{Optimizer, DifferentiableFunction};

// Define your objective function
struct MyFunction;
impl DifferentiableFunction for MyFunction {
    fn evaluate(&self, params: &[Tensor]) -> Result<f64> {
        // Compute objective value
    }

    fn gradient(&self, params: &[Tensor]) -> Result<Vec<Tensor>> {
        // Compute gradients
    }
}

// Optimization loop
let mut params = initialize_parameters();
let function = MyFunction;
let mut optimizer = QQNOptimizer::new(QQNConfig::default ());

for iteration in 0..max_iterations {
let result = optimizer.step( & mut params, & function) ?;

if result.convergence_info.converged {
println ! ("Converged at iteration {}", iteration);
break;
}
}
```

### 4.3 Benchmarking

```rust
use qqn_optimizer::benchmarks::{BenchmarkRunner, RosenbrockFunction};

let mut runner = BenchmarkRunner::new();
runner.add_optimizer("QQN", Box::new(QQNOptimizer::new(QQNConfig::default ())));
runner.add_optimizer("L-BFGS", Box::new(LBFGSOptimizer::new(LBFGSConfig::default ())));

let problem = RosenbrockFunction::new(10); // 10-dimensional
let results = runner.run_benchmark( & problem, 10) ?; // 10 runs

// Analyze results
let analysis = StatisticalAnalysis::new( & results);
analysis.print_summary();
```

### 4.4 Custom Line Search

```rust
use qqn_optimizer::core::line_search::{LineSearch, LineSearchResult};

struct MyLineSearch;
impl LineSearch for MyLineSearch {
    fn search(
        &mut self,
        params: &[Tensor],
        direction: &[Tensor],
        gradients: &[Tensor],
        function: &dyn DifferentiableFunction,
    ) -> Result<LineSearchResult> {
        // Custom line search implementation
    }
}
```

---

## 6. Extension Guide

### 6.1 Adding New Optimizers

```rust
use qqn_optimizer::core::{Optimizer, StepResult};

pub struct MyOptimizer {
    config: MyConfig,
    state: MyState,
}

impl Optimizer for MyOptimizer {
    type Config = MyConfig;
    type State = MyState;

    fn new(config: Self::Config) -> Self {
        // Initialize optimizer
    }

    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
    ) -> Result<StepResult> {
        // Implement optimization step
    }

    // Implement other required methods...
}
```

### 6.2 Custom Benchmark Problems

```rust
use qqn_optimizer::benchmarks::OptimizationProblem;

pub struct MyProblem {
    dimension: usize,
}

impl OptimizationProblem for MyProblem {
    fn name(&self) -> &str {
        "MyProblem"
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn evaluate(&self, x: &[f64]) -> Result<f64> {
        // Compute objective value
    }

    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
        // Compute gradient
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(0.0) // If known
    }
}
```

### 6.3 Analysis Extensions

```rust
use qqn_optimizer::analysis::MetricCalculator;

pub struct MyMetric;
impl MetricCalculator for MyMetric {
    fn calculate(&self, trace: &OptimizationTrace) -> f64 {
        // Custom metric calculation
    }
}
```

### 6.4 Integration with ML Frameworks

```rust
// PyTorch integration example
use tch::{Tensor as TorchTensor, Device};

fn torch_to_candle(torch_tensor: &TorchTensor) -> Result<candle_core::Tensor> {
    let data: Vec<f64> = torch_tensor.to_kind(tch::Kind::Double).flatten(0, -1);
    let shape = torch_tensor.size();
    candle_core::Tensor::from_vec(data, &shape, &candle_core::Device::Cpu)
}
```

---

## Appendices

### A. Configuration Schema

```yaml
experiment:
  name: "QQN Benchmark Study"
  problems:
    - type: Rosenbrock
      dimension: 100
    - type: Rastrigin
      dimension: 50
  optimizers:
    - type: QQN
      lbfgs_history: 10
      line_search:
        method: Bisection
    - type: LBFGS
      history: 10
  benchmark:
    max_iterations: 1000
    tolerance: 1e-6
    num_runs: 20
```

### B. Troubleshooting Guide

| Issue                | Possible Cause      | Solution                                          |
|----------------------|---------------------|---------------------------------------------------|
| NaN in gradients     | Numerical overflow  | Check function scaling, use gradient clipping     |
| Slow convergence     | Poor conditioning   | Adjust L-BFGS history size, check problem scaling |
| Line search failures | Non-smooth function | Use backtracking instead of strong Wolfe          |
| Memory issues        | Large history size  | Reduce L-BFGS history, use gradient checkpointing |

### C. References

1. Liu, D. C., & Nocedal, J. (1989). On the limited memory BFGS method for large scale optimization. Mathematical
   programming, 45(1), 503-528.
2. Nocedal, J., & Wright, S. (2006). Numerical optimization. Springer Science & Business Media.
3. Byrd, R. H., Lu, P., Nocedal, J., & Zhu, C. (1995). A limited memory algorithm for bound constrained optimization.
   SIAM Journal on scientific computing, 16(5), 1190-1208.