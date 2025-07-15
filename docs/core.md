# QQN Optimizer Framework - Core Technical Documentation

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Module Documentation](#core-module-documentation)
3. [Optimizer Trait System](#optimizer-trait-system)
4. [L-BFGS Implementation](#l-bfgs-implementation)
5. [QQN Algorithm Implementation](#qqn-algorithm-implementation)
6. [Line Search Algorithms](#line-search-algorithms)
7. [Mathematical Utilities](#mathematical-utilities)
8. [Error Handling and Safety](#error-handling-and-safety)
9. [Performance Considerations](#performance-considerations)
10. [Extension Points](#extension-points)

## Architecture Overview

The QQN optimizer framework is built around a modular, trait-based architecture that enables easy extension and
benchmarking of optimization algorithms. The core design principles include:

- **Type Safety**: Extensive use of Rust's type system to prevent runtime errors
- **Memory Safety**: Zero-copy operations where possible, careful memory management
- **Numerical Stability**: Robust handling of edge cases and numerical precision issues
- **Extensibility**: Clean trait boundaries for adding new optimizers and problems
- **Performance**: Efficient tensor operations using the Candle framework

### Core Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    QQN Framework Core                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Optimizer  │  │ LineSearch  │  │ DifferentiableFunc  │  │
│  │   Traits    │  │   Traits    │  │      Traits         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │    QQN      │  │   L-BFGS    │  │    Line Search      │  │
│  │ Optimizer   │  │ Optimizer   │  │   Algorithms        │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Mathematical│  │   Tensor    │  │     Validation      │  │
│  │  Utilities  │  │ Operations  │  │    & Safety         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Core Module Documentation

### Module Structure

```rust
src/core/
├── mod .rs              // Public API and re-exports
├── optimizer.rs        // Core optimizer traits and types
├── lbfgs.rs           // L-BFGS implementation
├── qqn.rs             // QQN algorithm implementation
└── line_search.rs     // Line search algorithms
```

### Key Types and Constants

```rust
/// Core result type used throughout the optimization framework
pub type OptResult<T> = anyhow::Result<T>;

/// Tolerance for numerical comparisons
pub const NUMERICAL_TOLERANCE: f64 = 1e-12;

/// Maximum number of line search iterations
pub const MAX_LINE_SEARCH_ITERATIONS: usize = 50;

/// Default L-BFGS history size
pub const DEFAULT_LBFGS_HISTORY: usize = 10;
```

## Optimizer Trait System

### Core Optimizer Trait

The `Optimizer` trait defines the interface that all optimization algorithms must implement:

```rust
pub trait Optimizer: Send + Sync + std::fmt::Debug {
    type Config: Clone + Debug + Send + Sync;
    type State: Clone + Debug + Send + Sync;

    /// Create a new optimizer instance
    fn new(config: Self::Config) -> Self where
        Self: Sized;

    /// Perform a single optimization step
    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
    ) -> Result<StepResult>;

    /// Perform a step with pre-computed gradients
    fn step_with_gradients(
        &mut self,
        params: &mut [Tensor],
        gradients: &[Tensor],
    ) -> Result<StepResult>;

    /// Reset optimizer state
    fn reset(&mut self);

    /// Get current state
    fn state(&self) -> &Self::State;

    /// Get optimizer name
    fn name(&self) -> &str;

    /// Check convergence
    fn has_converged(&self) -> bool { false }
}
```

### DifferentiableFunction Trait

Defines functions that can be optimized:

```rust
pub trait DifferentiableFunction: Send + Sync {
    /// Evaluate function at given point
    fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64>;

    /// Compute gradients at given point
    fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>>;

    /// Compute both value and gradients (optional optimization)
    fn evaluate_with_gradient(&self, params: &[Tensor]) -> CandleResult<(f64, Vec<Tensor>)> {
        let value = self.evaluate(params)?;
        let grad = self.gradient(params)?;
        Ok((value, grad))
    }
}
```

### StepResult Structure

Contains comprehensive information about each optimization step:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step size used
    pub step_size: f64,

    /// Function evaluations consumed
    pub function_evaluations: usize,

    /// Gradient evaluations consumed
    pub gradient_evaluations: usize,

    /// Convergence information
    pub convergence_info: ConvergenceInfo,

    /// Additional metadata
    pub metadata: OptimizationMetadata,
}
```

## L-BFGS Implementation

### Algorithm Overview

The L-BFGS (Limited-memory BFGS) implementation serves as both a baseline optimizer and a core component of the QQN
algorithm. It maintains a limited history of gradient and parameter changes to approximate the inverse Hessian matrix.

### Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LBFGSConfig {
    /// Number of previous iterations to store
    pub history_size: usize,

    /// Line search configuration
    pub line_search: LineSearchConfig,

    /// Numerical stability constant
    pub epsilon: f64,

    /// Maximum correction pairs
    pub max_correction_pairs: usize,

    /// Step size bounds
    pub max_step_size: f64,
    pub min_step_size: f64,

    /// Verbose logging
    pub verbose: bool,
}
```

### State Management

```rust
#[derive(Debug, Clone)]
pub struct LBFGSState {
    /// Parameter differences: s_k = x_{k+1} - x_k
    s_history: VecDeque<Vec<Tensor>>,

    /// Gradient differences: y_k = g_{k+1} - g_k
    y_history: VecDeque<Vec<Tensor>>,

    /// Reciprocals: ρ_k = 1/(s_k^T y_k)
    rho_history: VecDeque<f64>,

    /// Previous gradient for differences
    prev_gradient: Option<Vec<Tensor>>,

    /// Current iteration
    iteration: usize,

    /// Hessian scaling factor
    gamma: f64,
}
```

### Two-Loop Recursion Algorithm

The core L-BFGS direction computation uses the two-loop recursion:

```rust
impl LBFGSState {
    pub fn compute_direction(&mut self, gradient: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        // Input validation
        if gradient.is_empty() {
            return Err(candle_core::Error::Msg("Empty gradient vector".into()));
        }

        // Check for very small gradients
        let grad_norm = compute_magnitude(gradient)?;
        if grad_norm < 1e-10 {
            return Ok(gradient.iter().map(|g| g.neg()).collect::<CandleResult<Vec<_>>>()?);
        }

        // No history available - use steepest descent
        if self.s_history.is_empty() {
            return Ok(gradient.iter().map(|g| g.neg()).collect::<CandleResult<Vec<_>>>()?);
        }

        let mut q = gradient.to_vec();
        let mut alpha = Vec::with_capacity(self.s_history.len());

        // First loop: compute α values and update q
        for i in (0..self.s_history.len()).rev() {
            let s_i = &self.s_history[i];
            let rho_i = self.rho_history[i];

            // Skip if numerical issues
            if !rho_i.is_finite() || rho_i.abs() < 1e-16 {
                continue;
            }

            let alpha_i = rho_i * dot_product(s_i, &q)?;
            alpha.push(alpha_i);

            // q = q - α_i * y_i
            let y_i = &self.y_history[i];
            let scaled_y = vector_scale(y_i, alpha_i)?;
            q = vector_subtract(&q, &scaled_y)?;
        }

        alpha.reverse();

        // Apply initial Hessian scaling: r = γ * q
        let safe_gamma = self.gamma.max(1e-6).min(1e6);
        let mut r = vector_scale(&q, safe_gamma)?;

        // Second loop: compute final direction
        for i in 0..self.s_history.len() {
            if i >= alpha.len() { continue; }

            let s_i = &self.s_history[i];
            let y_i = &self.y_history[i];
            let rho_i = self.rho_history[i];
            let alpha_i = alpha[i];

            let beta = rho_i * dot_product(y_i, &r)?;
            let correction_factor = alpha_i - beta;

            if !correction_factor.is_finite() { continue; }

            // r = r + (α_i - β) * s_i
            let correction = vector_scale(s_i, correction_factor)?;
            r = vector_add(&r, &correction)?;
        }

        // Return negative for descent direction
        Ok(r.iter().map(|t| t.neg()).collect::<CandleResult<Vec<_>>>()?)
    }
}
```

### State Update with Curvature Condition

```rust
impl LBFGSState {
    pub fn update(
        &mut self,
        new_gradient: &[Tensor],
        step_direction: &[Tensor],
        step_size: f64,
    ) -> CandleResult<()> {
        // Validate inputs
        if !step_size.is_finite() || step_size <= 0.0 {
            return Ok(()); // Skip invalid updates
        }

        // Compute parameter difference: s_k = α * p_k
        let s_k = vector_scale(step_direction, step_size)?;

        if let Some(prev_grad) = &self.prev_gradient {
            // Compute gradient difference: y_k = ∇f_{k+1} - ∇f_k
            let y_k = vector_subtract(new_gradient, prev_grad)?;

            // Check curvature condition: s_k^T y_k > ε
            let s_dot_y = dot_product(&s_k, &y_k)?;

            if s_dot_y > self.epsilon() {
                let rho_k = 1.0 / s_dot_y;

                // Maintain history size limit
                if self.s_history.len() >= self.s_history.capacity() {
                    self.s_history.pop_front();
                    self.y_history.pop_front();
                    self.rho_history.pop_front();
                }

                self.s_history.push_back(s_k);
                self.y_history.push_back(y_k.clone());
                self.rho_history.push_back(rho_k);

                // Update Hessian scaling: γ = (s_k^T y_k) / (y_k^T y_k)
                let y_dot_y = dot_product(&y_k, &y_k)?;
                if y_dot_y > self.epsilon() {
                    let new_gamma = s_dot_y / y_dot_y;
                    if new_gamma.is_finite() && new_gamma > 0.0 {
                        self.gamma = new_gamma.max(1e-6).min(10.0);
                    }
                }
            }
        }

        self.prev_gradient = Some(new_gradient.to_vec());
        self.iteration += 1;
        Ok(())
    }
}
```

## QQN Algorithm Implementation

### Core Innovation

The QQN (Quadratic Quasi-Newton) algorithm introduces a novel quadratic interpolation path between the steepest descent
direction and the L-BFGS direction:

```
d(t) = t(1-t)(-∇f) + t²d_LBFGS
```

where `t ∈ [0,1]` is optimized via line search.

### Configuration

```rust
#[derive(Debug, Clone)]
pub struct QQNConfig {
    /// L-BFGS history length
    pub lbfgs_history: usize,

    /// Line search configuration
    pub line_search: StrongWolfeConfig,

    /// Numerical stability constant
    pub epsilon: f64,

    /// Verbose logging
    pub verbose: bool,
}
```

### State Management

```rust
#[derive(Debug, Clone)]
pub struct QQNState {
    /// Current iteration number
    pub iteration: usize,

    /// Internal L-BFGS state
    pub lbfgs_state: LBFGSState,
}
```

### Quadratic Path Implementation

```rust
#[derive(Debug, Clone)]
pub struct QuadraticPath {
    negative_gradient: Vec<Tensor>,
    lbfgs_direction: Vec<Tensor>,
}

impl QuadraticPath {
    /// Evaluate path at parameter t: d(t) = t(1-t)(-g) + t²d_lbfgs
    pub fn evaluate(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        let t_clamped = t.max(0.0).min(1.0);

        let gradient_coeff = t_clamped * (1.0 - t_clamped);
        let lbfgs_coeff = t_clamped * t_clamped;

        let gradient_term = scale_tensors(&self.negative_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;

        combine_tensors(&gradient_term, &lbfgs_term)
    }

    /// Compute derivative: d'(t) = (1-2t)(-g) + 2t*d_lbfgs
    pub fn derivative(&self, t: f64) -> CandleResult<Vec<Tensor>> {
        let gradient_coeff = 1.0 - 2.0 * t;
        let lbfgs_coeff = 2.0 * t;

        let gradient_term = scale_tensors(&self.negative_gradient, gradient_coeff)?;
        let lbfgs_term = scale_tensors(&self.lbfgs_direction, lbfgs_coeff)?;

        combine_tensors(&gradient_term, &lbfgs_term)
    }
}
```

### QQN Step Algorithm

```rust
impl Optimizer for QQNOptimizer {
    fn step(
        &mut self,
        params: &mut [Tensor],
        function: &dyn DifferentiableFunction,
    ) -> CandleResult<StepResult> {
        // 1. Compute gradients
        let gradients = function.gradient(params)?;

        // 2. Compute L-BFGS direction
        let lbfgs_direction = self.state.lbfgs_state.compute_direction(&gradients)?;

        // 3. Create quadratic path
        let quadratic_path = self.create_quadratic_path(&gradients, &lbfgs_direction)?;

        // 4. Find optimal t via line search
        let (optimal_t, f_evals, g_evals) =
            self.find_optimal_t_line_search(params, &quadratic_path, &gradients, function)?;

        // 5. Evaluate final direction
        let direction = quadratic_path.evaluate(optimal_t)?;

        // 6. Apply step
        for (param, dir) in params.iter_mut().zip(direction.iter()) {
            *param = param.add(dir)?;
        }

        // 7. Update L-BFGS state
        self.state.lbfgs_state.update(&gradients, &direction, 1.0)?;

        self.state.iteration += 1;

        // 8. Return results
        Ok(StepResult {
            step_size: optimal_t,
            function_evaluations: f_evals,
            gradient_evaluations: g_evals,
            convergence_info: ConvergenceInfo::default(),
            metadata: self.create_metadata(&gradients, &direction, optimal_t)?,
        })
    }
}
```

## Line Search Algorithms

### ParametricCurve Trait

Enables line search along arbitrary parametric curves:

```rust
pub trait ParametricCurve: Send + Sync + Debug {
    /// Evaluate curve at parameter t
    fn evaluate(&self, t: f64) -> Result<Vec<f64>>;

    /// Evaluate derivative at parameter t
    fn derivative(&self, t: f64) -> Result<Vec<f64>>;

    /// Get initial derivative for descent checking
    fn initial_derivative(&self) -> Result<Vec<f64>>;

    /// Clone the curve
    fn clone_box(&self) -> Box<dyn ParametricCurve>;
}
```

### Strong Wolfe Line Search

Implements the Strong Wolfe conditions for robust line search:

```rust
impl LineSearch for StrongWolfeLineSearch {
    fn search_along_curve(
        &mut self,
        curve: &dyn ParametricCurve,
        current_value: f64,
        current_gradient: &[f64],
        objective_fn: &dyn Fn(&[f64]) -> Result<f64>,
        gradient_fn: &dyn Fn(&[f64]) -> Result<Vec<f64>>,
    ) -> Result<LineSearchResult> {
        let initial_derivative = curve.initial_derivative()?;
        let directional_derivative = dot_product_f64(current_gradient, &initial_derivative)?;

        // Ensure descent direction
        if directional_derivative >= 0.0 {
            return Err(anyhow!("Direction is not a descent direction"));
        }

        let mut alpha = self.config.initial_step;
        let mut alpha_prev = 0.0;
        let mut f_prev = current_value;

        for i in 0..self.config.max_iterations {
            let trial_point = curve.evaluate(alpha)?;
            let f_alpha = objective_fn(&trial_point)?;

            // Check Armijo condition and sufficient decrease
            if !self.armijo_condition(current_value, f_alpha, alpha, directional_derivative)
                || (i > 0 && f_alpha >= f_prev) {
                return self.zoom(alpha_prev, alpha, current_value, directional_derivative,
                                 curve, objective_fn, gradient_fn);
            }

            let grad_alpha = gradient_fn(&trial_point)?;
            let curve_derivative = curve.derivative(alpha)?;
            let grad_alpha_dot_p = dot_product_f64(&grad_alpha, &curve_derivative)?;

            // Check curvature condition
            if self.curvature_condition(grad_alpha_dot_p, directional_derivative) {
                return Ok(LineSearchResult {
                    step_size: alpha,
                    success: true,
                    termination_reason: TerminationReason::WolfeConditionsSatisfied,
                    // ... other fields
                });
            }

            if grad_alpha_dot_p >= 0.0 {
                return self.zoom(alpha, alpha_prev, current_value, directional_derivative,
                                 curve, objective_fn, gradient_fn);
            }

            alpha_prev = alpha;
            f_prev = f_alpha;
            alpha = alpha.min(self.config.max_step) * 2.0;
        }

        // Return failure result
        Ok(LineSearchResult {
            step_size: alpha_prev,
            success: false,
            termination_reason: TerminationReason::MaxIterationsReached,
            // ... other fields
        })
    }
}
```

### Wolfe Conditions

```rust
impl StrongWolfeLineSearch {
    /// Armijo condition: f(x + αp) ≤ f(x) + c₁α∇f(x)ᵀp
    fn armijo_condition(&self, f0: f64, f_alpha: f64, alpha: f64, directional_derivative: f64) -> bool {
        let threshold = f0 + self.config.c1 * alpha * directional_derivative;
        f_alpha <= threshold
    }

    /// Curvature condition: |∇f(x + αp)ᵀp| ≤ c₂|∇f(x)ᵀp|
    fn curvature_condition(&self, grad_alpha_dot_p: f64, directional_derivative: f64) -> bool {
        let threshold = self.config.c2 * directional_derivative.abs();
        grad_alpha_dot_p.abs() <= threshold
    }
}
```

## Mathematical Utilities

### Tensor Operations

```rust
/// Compute L2 norm of tensor vector
pub fn compute_magnitude(tensors: &[Tensor]) -> CandleResult<f64> {
    if tensors.is_empty() { return Ok(0.0); }

    let mut sum_of_squares = 0.0;
    for tensor in tensors {
        let values = tensor.flatten_all()?.to_vec1::<f64>()?;
        for &val in &values {
            if !val.is_finite() {
                return Ok(f64::INFINITY);
            }
            sum_of_squares += val * val;
        }
    }
    Ok(sum_of_squares.sqrt())
}

/// Compute dot product between tensor vectors
pub fn dot_product(a: &[Tensor], b: &[Tensor]) -> CandleResult<f64> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg("Length mismatch".to_string()));
    }

    let mut result = 0.0;
    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        let values_a = tensor_a.flatten_all()?.to_vec1::<f64>()?;
        let values_b = tensor_b.flatten_all()?.to_vec1::<f64>()?;

        for (val_a, val_b) in values_a.iter().zip(values_b.iter()) {
            result += val_a * val_b;
        }
    }
    Ok(result)
}

/// Scale tensor vector by scalar
pub fn vector_scale(tensors: &[Tensor], scale: f64) -> CandleResult<Vec<Tensor>> {
    let mut result = Vec::with_capacity(tensors.len());
    for tensor in tensors {
        let scale_tensor = Tensor::new(scale, tensor.device())?;
        result.push(tensor.broadcast_mul(&scale_tensor)?);
    }
    Ok(result)
}
```

### Numerical Stability

```rust
/// Check if all values are finite
pub fn is_finite(values: &[f64]) -> bool {
    values.iter().all(|x| x.is_finite())
}

/// Clamp values to safe range
pub fn clamp_vector(values: &[f64], min_val: f64, max_val: f64) -> Vec<f64> {
    values.iter().map(|&x| x.clamp(min_val, max_val)).collect()
}

/// Check if value is effectively zero
pub fn is_zero(value: f64, tolerance: f64) -> bool {
    value.abs() < tolerance
}
```

## Error Handling and Safety

### Error Types

The framework uses `anyhow::Result` for error handling with custom error types:

```rust
pub type OptResult<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum OptimizationError {
    #[error("Numerical instability detected: {0}")]
    NumericalInstability(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Convergence failure: {0}")]
    ConvergenceFailure(String),

    #[error("Tensor operation failed: {0}")]
    TensorError(#[from] candle_core::Error),
}
```

### Safety Measures

1. **Input Validation**: All public functions validate inputs for:
    - Empty vectors
    - Dimension mismatches
    - Non-finite values
    - Out-of-range parameters

2. **Numerical Stability**:
    - Curvature condition checking in L-BFGS
    - Step size clamping
    - Gradient magnitude validation
    - Safe gamma scaling

3. **Memory Safety**:
    - Bounded history storage
    - Careful tensor cloning
    - Resource cleanup on errors

### Example Safety Implementation

```rust
impl LBFGSOptimizer {
    fn step(&mut self, params: &mut [Tensor], function: &dyn DifferentiableFunction)
            -> CandleResult<StepResult> {

        // Input validation
        if params.is_empty() {
            return Err(candle_core::Error::Msg("Empty parameters".into()));
        }

        let gradients = function.gradient(params)?;

        // Check for non-finite gradients
        for (i, grad) in gradients.iter().enumerate() {
            let grad_vec = grad.flatten_all()?.to_vec1::<f64>()?;
            if grad_vec.iter().any(|&x| !x.is_finite()) {
                return Err(candle_core::Error::Msg(
                    format!("Non-finite gradient at index {}", i)
                ));
            }
        }

        // Compute direction with fallback
        let direction = match self.state.compute_direction(&gradients) {
            Ok(dir) => dir,
            Err(_) => {
                warn!("L-BFGS direction computation failed, using steepest descent");
                gradients.iter().map(|g| g.neg()).collect::<CandleResult<Vec<_>>>()?
            }
        };

        // Validate direction is descent
        let dot_product = crate::utils::math::dot_product(&gradients, &direction)?;
        if dot_product >= 0.0 {
            warn!("Non-descent direction detected, using steepest descent");
            let direction = gradients.iter().map(|g| g.neg()).collect::<CandleResult<Vec<_>>>()?;
        }

        // Continue with safe step...
    }
}
```

## Performance Considerations

### Memory Management

1. **Bounded History**: L-BFGS maintains fixed-size history using `VecDeque`
2. **Zero-Copy Operations**: Minimize tensor cloning where possible
3. **Efficient Tensor Operations**: Use Candle's optimized operations

### Computational Efficiency

1. **Early Termination**: Skip invalid history pairs in L-BFGS
2. **Vectorized Operations**: Batch tensor operations
3. **Lazy Evaluation**: Compute expensive operations only when needed

### Benchmarking Results

Typical performance characteristics:

- **L-BFGS**: O(mn) per iteration where m = history size, n = problem dimension
- **QQN**: O(mn + line_search_cost) per iteration
- **Memory**: O(mn) for L-BFGS history storage

## Extension Points

### Adding New Optimizers

1. Implement the `Optimizer` trait:

```rust
#[derive(Debug, Clone)]
pub struct MyOptimizer {
    config: MyConfig,
    state: MyState,
}

impl Optimizer for MyOptimizer {
    type Config = MyConfig;
    type State = MyState;

    fn new(config: Self::Config) -> Self { /* ... */ }
    fn step(&mut self, params: &mut [Tensor], function: &dyn DifferentiableFunction)
            -> CandleResult<StepResult> { /* ... */ }
    // ... other required methods
}
```

2. Add configuration support in `config.rs`
3. Register in benchmark framework

### Adding New Line Search Methods

1. Implement the `LineSearch` trait:

```rust
#[derive(Debug, Clone)]
pub struct MyLineSearch {
    config: MyLineSearchConfig,
}

impl LineSearch for MyLineSearch {
    fn search_along_curve(
        &mut self,
        curve: &dyn ParametricCurve,
        current_value: f64,
        current_gradient: &[f64],
        objective_fn: &dyn Fn(&[f64]) -> Result<f64>,
        gradient_fn: &dyn Fn(&[f64]) -> Result<Vec<f64>>,
    ) -> Result<LineSearchResult> {
        // Implementation
    }
}
```

### Adding New Problem Types

1. Implement `DifferentiableFunction`:

```rust
pub struct MyProblem {
    dimension: usize,
    // ... other fields
}

impl DifferentiableFunction for MyProblem {
    fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> { /* ... */ }
    fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> { /* ... */ }
}
```

2. Add to benchmark suite
3. Update configuration system

This comprehensive technical documentation provides the foundation for understanding, using, and extending the QQN
optimizer framework. The modular design and robust error handling make it suitable for both research and production use
cases.
