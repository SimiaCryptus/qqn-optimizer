# Rust + ML Tour for QQN Implementation

## Rust Language Essentials

### Ownership & Memory Model

- **No garbage collector**: Memory managed through ownership system
- **Move semantics**: Values transferred between variables (not copied)
- **Borrowing**: `&T` (immutable) and `&mut T` (mutable) references
- **Lifetimes**: Compiler ensures references don't outlive data
- **RAII**: Resources cleaned up when owner goes out of scope

```rust
let v = vec![1, 2, 3];  // v owns the vector
let v2 = v;             // v moved to v2, v no longer valid
let len = v2.len();     // OK
// let len2 = v.len();  // ERROR: v was moved
```

### Error Handling

- **No exceptions**: Use `Result<T, E>` and `Option<T>`
- **`?` operator**: Early return on error
- **Pattern matching**: `match` for control flow

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

let result = divide(10.0, 2.0)?; // Returns early if error
```

### Traits (Interfaces)

- **Behavior definition**: Like interfaces but more powerful
- **Generics**: `<T>` with trait bounds
- **Associated types**: Types tied to trait implementations

```rust
trait Optimizer {
    type Config;
    fn step(&mut self, params: &mut [f64]) -> Result<(), Error>;
}
```

### Key Syntax Differences

- **Immutable by default**: `let x = 5` vs `let mut x = 5`
- **No null**: Use `Option<T>` instead
- **Pattern matching**: Exhaustive `match` statements
- **Closures**: `|x| x * 2` syntax
- **Macros**: `println!`, `vec!`, `format!` end with `!`

## Candle ML Framework

### Core Concepts

- **Tensors**: `candle_core::Tensor` - similar to PyTorch tensors
- **Devices**: CPU vs CUDA, explicit device management
- **Automatic differentiation**: Built-in gradient computation
- **Modules**: Neural network layers in `candle_nn`

### Tensor Operations

```rust
use candle_core::{Tensor, Device, DType};

let device = Device::Cpu;
let a = Tensor::randn(0f32, 1f32, (2, 3), &device)?;
let b = Tensor::ones((3, 4), DType::F32, &device)?;
let c = a.matmul(&b)?;  // Matrix multiplication
```

### Optimization Framework

```rust
use candle_nn::optim::{Optimizer, SGD};

let mut optimizer = SGD::new(params, 0.01)?;
optimizer.step(&grads)?;
```

### Key Differences from PyTorch

- **Explicit error handling**: Everything returns `Result`
- **Device management**: Must specify device explicitly
- **Immutable tensors**: Operations return new tensors
- **No autograd by default**: Must explicitly enable gradient tracking

## Essential Crates for QQN

### Core Dependencies

```toml
[dependencies]
candle-core = "0.3"
candle-nn = "0.3"
candle-datasets = "0.3"  # For benchmarks
anyhow = "1.0"           # Error handling
serde = { version = "1.0", features = ["derive"] }
```

### Numerical Computing

- **`nalgebra`**: Linear algebra (if you need more than Candle)
- **`ndarray`**: NumPy-like arrays (alternative to tensors)
- **`statrs`**: Statistical functions

### Benchmarking & Testing

- **`criterion`**: Performance benchmarking
- **`approx`**: Floating-point comparisons
- **`proptest`**: Property-based testing

## QQN Implementation Architecture

### Trait Design Pattern

```rust
pub trait Optimizer {
    type Config: Clone + Debug;

    fn new(config: Self::Config) -> Self;
    fn step(&mut self, params: &mut [Tensor], grads: &[Tensor]) -> Result<()>;
    fn reset(&mut self);
}

pub struct QQNOptimizer {
    config: QQNConfig,
    lbfgs_state: LBFGSState,
    magnitude_tracker: MagnitudeTracker,
}
```

### Error Handling Strategy

```rust
use anyhow::{Result, Context};

impl QQNOptimizer {
    fn compute_direction(&self, grad: &Tensor) -> Result<Tensor> {
        let lbfgs_dir = self.lbfgs_state.compute_direction(grad)
            .context("Failed to compute L-BFGS direction")?;

        let grad_mag = self.compute_magnitude(grad)?;
        let lbfgs_mag = self.compute_magnitude(&lbfgs_dir)?;

        if (lbfgs_mag - grad_mag).abs() / grad_mag > self.config.threshold {
            self.create_quadratic_path(grad, &lbfgs_dir)
        } else {
            Ok(lbfgs_dir)
        }
    }
}
```

## Development Workflow

### Project Structure

```
qqn-optimizer/
├── Cargo.toml           # Dependencies and metadata
├── src/
│   ├── lib.rs          # Public API exports
│   ├── optimizer.rs    # Trait definitions
│   ├── qqn.rs         # QQN implementation
│   ├── lbfgs.rs       # L-BFGS baseline
│   └── benchmarks.rs  # Test problems
├── examples/           # Usage examples
├── benches/           # Criterion benchmarks
└── tests/             # Integration tests
```

### Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_magnitude_computation() {
        let tensor = Tensor::from_slice(&[3.0, 4.0], &[2], &Device::Cpu).unwrap();
        let mag = compute_magnitude(&tensor).unwrap();
        assert_relative_eq!(mag, 5.0, epsilon = 1e-10);
    }
}
```

### Performance Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_qqn_step(c: &mut Criterion) {
    let mut optimizer = QQNOptimizer::new(QQNConfig::default());
    let params = vec![Tensor::randn(0f32, 1f32, (100,), &Device::Cpu).unwrap()];

    c.bench_function("qqn_step", |b| {
        b.iter(|| {
            optimizer.step(black_box(¶ms), black_box(&grads))
        })
    });
}
```

## Key Learning Resources

### Language Fundamentals

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: Interactive exercises

### ML-Specific

- **Candle Examples**: https://github.com/huggingface/candle/tree/main/candle-examples
- **Candle Book**: https://huggingface.co/docs/candle/
- **Burn Framework**: Alternative ML framework to compare approaches

### Optimization Background

- **nalgebra docs**: For linear algebra operations
- **Scientific computing in Rust**: https://arewelearningyet.com/

## Common Gotchas for Java Developers

1. **No null pointer exceptions**: Use `Option<T>` instead of null
2. **Explicit memory management**: No GC, but ownership prevents leaks
3. **Immutable by default**: Must explicitly use `mut`
4. **No inheritance**: Use composition and traits instead
5. **Explicit error handling**: No exceptions, use `Result<T, E>`
6. **Borrowing rules**: Can't have mutable and immutable references simultaneously
7. **Move semantics**: Values consumed unless explicitly cloned

## Development Tools

### Essential Tools

- **rustc**: Compiler with excellent error messages
- **cargo**: Package manager and build tool
- **clippy**: Linter with optimization suggestions
- **rustfmt**: Code formatter
- **rust-analyzer**: LSP for IDEs

### IDE Setup

- **VS Code**: rust-analyzer extension
- **IntelliJ**: Rust plugin
- **Vim/Neovim**: rust-analyzer + coc.nvim

### Debugging

- **`dbg!` macro**: Print debugging
- **`gdb`/`lldb`: Native debugging
- **`cargo test`**: Built-in test runner

This should give you the lay of the land for implementing QQN in Rust with Candle. The ownership system will be the
biggest adjustment, but it prevents entire classes of bugs common in C++/Java.
