# QQN Steepest Descent Fallback: Adaptive Quadratic Path Scaling

## The Problem: Inefficient Pure Gradient Steps

When the QQN algorithm falls back to pure gradient descent (when L-BFGS direction equals the negative gradient), it encounters a fundamental inefficiency. The standard QQN path becomes:

```
d(t) = t(1-t)(-∇f) + t²(-∇f) = t(-∇f)
```

This linear path means that at `t = 1`, we only take a step of size `||∇f||`. For many optimization problems, especially in early iterations, this is far too conservative. The 1D line search is effectively searching for the optimal step size between 0 and `||∇f||`, which may be orders of magnitude smaller than the actual optimal step size.

### Example: The Quadratic Case

Consider minimizing `f(x) = ½xᵀAx` starting far from the origin. The optimal step size is `1/λ_max(A)`, but `||∇f|| = ||Ax||` can be arbitrarily large. The QQN path would restrict us to tiny steps, requiring many iterations to make progress.

## The Solution: Adaptive Quadratic Scaling

We introduce a `max_learning_rate` parameter that defines a reasonable upper bound for gradient steps. When in pure gradient descent mode, we construct a quadratic path that:

1. Maintains the same initial derivative (pure gradient descent direction)
2. Maps `t = 1` to a step of size `max_learning_rate * ||∇f||`
3. Preserves the smooth quadratic nature of QQN paths

## Mathematical Derivation

We want a quadratic path `d(t) = at² + bt` such that:

1. `d(0) = 0` (start at current point)
2. `d'(0) = -∇f` (initial direction is negative gradient)
3. `d(1) = -max_learning_rate * ||∇f|| * (∇f/||∇f||)` (reach max step at t=1)

From constraints 1 and 2:
- `b = -∇f`

From constraint 3:
- `a + b = -max_learning_rate * ||∇f|| * (∇f/||∇f||)`
- `a = -max_learning_rate * ∇f + ∇f`
- `a = (1 - max_learning_rate) * (-∇f)`

Therefore:
```
d(t) = t² * (1 - max_learning_rate) * (-∇f) + t * (-∇f)
     = t * (-∇f) * [1 + t(1 - max_learning_rate)]
     = t * (-∇f) * [1 - t(max_learning_rate - 1)]
```

For `max_learning_rate > 1`, this creates a quadratic path that accelerates beyond the linear gradient step.

## Implementation Strategy

```python
def compute_qqn_path(gradient, d_lbfgs, max_learning_rate=10.0):
    """
    Compute QQN path with adaptive scaling for pure gradient descent.

    Args:
        gradient: Current gradient vector
        d_lbfgs: L-BFGS direction (may equal -gradient)
        max_learning_rate: Maximum step size as multiple of ||gradient||

    Returns:
        Path function d(t) for t in [0, 1]
    """
    # Check if we're in pure gradient descent mode
    direction_similarity = np.dot(-gradient, d_lbfgs) / (np.linalg.norm(gradient) * np.linalg.norm(d_lbfgs))

    if direction_similarity > 0.99:  # Essentially the same direction
        # Use scaled quadratic path
        def path(t):
            scale_factor = 1 - t * (max_learning_rate - 1)
            return t * (-gradient) * scale_factor
    else:
        # Use standard QQN path
        def path(t):
            return t * (1 - t) * (-gradient) + t**2 * d_lbfgs

    return path
```

## Benefits of This Approach

### 1. **Maintains QQN's Theoretical Properties**
- Still guarantees descent: `d'(0) = -∇f`
- Preserves quadratic path structure
- Smooth transition between conservative and aggressive steps

### 2. **Dramatically Improves Early Convergence**
- Allows larger steps when far from optimum
- Automatically scales based on gradient magnitude
- Prevents unnecessarily conservative behavior

### 3. **Simple to Implement and Tune**
- Single parameter with clear interpretation
- Default value (e.g., 10.0) works well across many problems
- Can be adjusted based on problem knowledge

### 4. **Preserves Robustness**
- 1D line search still ensures we don't overshoot
- If `max_learning_rate` is too large, line search will find appropriate `t < 1`
- Graceful degradation to standard behavior

## Choosing max_learning_rate

The `max_learning_rate` parameter should be chosen based on problem characteristics:

- **Conservative default**: 2.0 - 5.0 (safe for most problems)
- **Moderate default**: 10.0 (good balance of speed and safety)
- **Aggressive**: 50.0 - 100.0 (when function is known to be well-behaved)

For specific problem classes:
- **Quadratic problems**: Can use `1/expected_condition_number`
- **Neural networks**: Often benefit from larger values (10-50)
- **Ill-conditioned problems**: Smaller values (1-5) for stability

## Integration with QQN Algorithm

The modified QQN algorithm becomes:

```
1. Compute gradient g = ∇f(x)
2. Compute L-BFGS direction d_lbfgs
3. Check if d_lbfgs ≈ -g (pure gradient case)
4. If pure gradient:
   - Use scaled quadratic path with max_learning_rate
5. Else:
   - Use standard QQN quadratic interpolation
6. Perform 1D optimization on selected path
7. Update position and L-BFGS memory
```

## Theoretical Considerations

This modification preserves QQN's convergence guarantees:

1. **Global convergence**: Still guaranteed since `d'(0) = -∇f`
2. **Local convergence**: Unaffected since near optimum, L-BFGS provides distinct direction
3. **Descent property**: Maintained by construction

The only change is in the search space for pure gradient steps, which is expanded to allow more efficient optimization while maintaining all safety properties through the 1D line search.

## Conclusion

The adaptive quadratic scaling for pure gradient descent steps is a crucial enhancement to the QQN algorithm. It addresses the key limitation of overly conservative steps when L-BFGS cannot provide useful curvature information, while maintaining all the theoretical guarantees and practical robustness of the original method. This simple modification can reduce iteration counts by orders of magnitude in the early stages of optimization, making QQN competitive with specialized first-order methods while retaining its superior convergence properties near the solution.
