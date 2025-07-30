# Fashion-MNIST Alternate Problem Suite

This directory contains an alternate version of the MNIST problem suite using the Fashion-MNIST dataset, which provides a more challenging and realistic benchmark for optimization algorithms.

## Overview

Fashion-MNIST is a dataset of Zalando's article images consisting of 60,000 training examples and 10,000 test examples. Each example is a 28x28 grayscale image, associated with a label from 10 classes:

1. **T-shirt/top**
2. **Trouser**
3. **Pullover**
4. **Dress**
5. **Coat**
6. **Sandal**
7. **Shirt**
8. **Sneaker**
9. **Bag**
10. **Ankle boot**

## Why Fashion-MNIST?

Fashion-MNIST serves as a more challenging alternative to the original MNIST digit classification:

- **More realistic**: Real-world clothing items vs. handwritten digits
- **More challenging**: Fashion items have more complex patterns and variations
- **Same format**: Maintains 28x28 image size for compatibility
- **Better evaluation**: Provides more meaningful assessment of optimization algorithms

## Implementation

The Fashion-MNIST alternate problem suite is implemented in:

- **`src/benchmarks/fashion_mnist.rs`**: Core Fashion-MNIST neural network implementation
- **`src/experiment_runner/problem_sets.rs`**: Problem set definitions for various configurations
- **`tests/fashion_mnist_test.rs`**: Comprehensive tests
- **`examples/fashion_mnist_demo.rs`**: Usage demonstration

## Available Problem Variants

The suite includes 6 different problem configurations:

1. **FashionMNIST_ReLU_20**: Single hidden layer (20 units) with ReLU activation
2. **FashionMNIST_Logistic_20**: Single hidden layer (20 units) with Logistic activation  
3. **FashionMNIST_ReLU_30**: Single hidden layer (30 units) with ReLU activation
4. **FashionMNIST_ReLU_20x3**: Three hidden layers (20 units each) with ReLU activation
5. **FashionMNIST_Logistic_20x3**: Three hidden layers (20 units each) with Logistic activation
6. **FashionMNIST_Sinewave_15x25x15**: Three hidden layers with Sinewave activation

## Usage

### Basic Usage

```rust
use qqn_optimizer::benchmarks::fashion_mnist::{FashionMnistNeuralNetwork, ActivationType};
use rand::prelude::StdRng;
use rand::SeedableRng;

let mut rng = StdRng::seed_from_u64(42);

// Create a Fashion-MNIST neural network
let network = FashionMnistNeuralNetwork::create_single_hidden(
    Some(1000), // 1000 samples
    32,         // 32 hidden units
    Some(32),   // Batch size
    &mut rng,
    Some(ActivationType::ReLU),
)?;

// Use with optimization algorithms
let initial_point = network.initial_point();
let loss = network.evaluate_f64(&initial_point)?;
let gradient = network.gradient_f64(&initial_point)?;
```

### Using Problem Sets

```rust
use qqn_optimizer::experiment_runner::problem_sets::fashion_mnist_problems;

// Get all Fashion-MNIST problem variants
let problems = fashion_mnist_problems(1000); // 1000 samples each

for problem in problems {
    // Use problem.problem for optimization
    println!("Problem: {}", problem.name.unwrap_or(problem.family));
}
```

## Features

- **Automatic Data Download**: Downloads Fashion-MNIST data from official repository
- **Multiple Activations**: ReLU, Logistic (Sigmoid), and Sinewave activation functions
- **Flexible Architecture**: Support for various hidden layer configurations
- **Batch Processing**: Efficient batch-based training
- **Gradient Computation**: Automatic differentiation using Candle framework
- **Caching**: Parameter and gradient caching for efficiency
- **Regularization**: L2 regularization support
- **Initialization**: Proper weight initialization for different activation functions

## Data Download

The implementation automatically downloads Fashion-MNIST data on first use:

```
data/
├── fashion-train-images-idx3-ubyte
├── fashion-train-labels-idx1-ubyte
├── fashion-t10k-images-idx3-ubyte
└── fashion-t10k-labels-idx1-ubyte
```

## Testing

Run Fashion-MNIST tests:

```bash
cargo test fashion_mnist --release
```

## Example

Run the demonstration example:

```bash
cargo run --example fashion_mnist_demo --release
```

## Integration with Optimization Framework

Fashion-MNIST problems integrate seamlessly with the existing optimization framework:

- Implements `OptimizationProblem` trait
- Compatible with all optimizers (QQN, L-BFGS, Adam, etc.)
- Supports performance analysis and reporting
- Works with benchmark evaluation infrastructure

This alternate problem suite provides a more challenging and realistic benchmark for evaluating optimization algorithms on machine learning tasks.