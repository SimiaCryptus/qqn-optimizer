# QQN Algorithm: Mathematical Specification

## Abstract

The Quadratic Quasi-Newton (QQN) algorithm is a hybrid optimization method that combines gradient descent and L-BFGS
through parametric quadratic interpolation. This document provides a complete mathematical specification of the
algorithm, including theoretical properties, convergence analysis, and implementation details.

## 1. Problem Formulation

Consider the unconstrained optimization problem:
$$\min_{x \in \mathbb{R}^n} f(x)$$
where $f: \mathbb{R}^n \to \mathbb{R}$ is continuously differentiable. We assume:
**Assumption 1** (Smoothness): $f$ is continuously differentiable with Lipschitz continuous gradient:
$$\|\nabla f(x) - \nabla f(y)\| \leq L \|x - y\|, \quad \forall x, y \in \mathbb{R}^n$$
**Assumption 2** (Lower boundedness): $f$ is bounded below: $f(x) \geq f_* > -\infty$ for all $x$.

## 2. L-BFGS Background

### 2.1 L-BFGS Direction Computation

L-BFGS maintains a history of $m$ correction pairs $\{(s_i, y_i)\}_{i=k-m}^{k-1}$ where:

- $s_i = x_{i+1} - x_i$ (position difference)
- $y_i = \nabla f(x_{i+1}) - \nabla f(x_i)$ (gradient difference)
  The L-BFGS direction $d_k^{LBFGS}$ is computed via the two-loop recursion:
  **Algorithm 1** (L-BFGS Two-Loop Recursion)

```
Input: gradient g_k, history {(s_i, y_i)}
1. q ← g_k
2. For i = k-1, k-2, ..., k-m:
   a. ρ_i ← 1/(y_i^T s_i)
   b. α_i ← ρ_i s_i^T q
   c. q ← q - α_i y_i
3. r ← γ_k q  where γ_k = (s_{k-1}^T y_{k-1})/(y_{k-1}^T y_{k-1})
4. For i = k-m, k-m+1, ..., k-1:
   a. β ← ρ_i y_i^T r
   b. r ← r + s_i(α_i - β)
5. Return d_k^{LBFGS} = -r
```

### 2.2 Curvature Condition

The update $(s_i, y_i)$ is accepted only if the curvature condition holds:
$$s_i^T y_i > \epsilon$$
where $\epsilon > 0$ is a small tolerance (typically $\epsilon = 10^{-6}$).

## 3. QQN Algorithm Specification

### 3.1 Parametric Quadratic Interpolation

Given the current gradient $g_k = \nabla f(x_k)$ and L-BFGS direction $d_k^{LBFGS}$, QQN defines the parametric
direction:
$$d_k(t) = t(1-t)(-g_k) + t^2 d_k^{LBFGS}, \quad t \in [0,1]$$
**Proposition 1** (Boundary Behavior): The parametric direction satisfies:

- $d_k(0) = 0$ (null step)
- $d_k(1) = d_k^{LBFGS}$ (pure L-BFGS direction)
- $\lim_{t \to 0^+} \frac{d_k(t)}{t} = -g_k$ (gradient descent tangent)
  **Proof**: Direct computation:
- $d_k(0) = 0 \cdot 1 \cdot (-g_k) + 0^2 d_k^{LBFGS} = 0$
- $d_k(1) = 1 \cdot 0 \cdot (-g_k) + 1^2 d_k^{LBFGS} = d_k^{LBFGS}$
- $\frac{d_k(t)}{t} = (1-t)(-g_k) + t d_k^{LBFGS} \to -g_k$ as $t \to 0^+$ □

### 3.2 Descent Property

**Theorem 1** (Descent Direction): If $\|g_k\| > 0$, then there exists $\bar{t} > 0$ such that $g_k^T d_k(t) < 0$ for
all $t \in (0, \bar{t}]$.
**Proof**: The directional derivative at $t = 0$ is:
$$\frac{d}{dt}[g_k^T d_k(t)]_{t=0} = g_k^T \frac{d}{dt}[t(1-t)(-g_k) + t^2 d_k^{LBFGS}]_{t=0} = g_k^T(-g_k) = -\|g_k\|^2 < 0$$
By continuity, there exists $\bar{t} > 0$ such that $g_k^T d_k(t) < 0$ for $t \in (0, \bar{t}]$. □

### 3.3 Line Search on Parametric Curve

QQN performs line search to find $t_k^* \in [0,1]$ satisfying the Strong Wolfe conditions:
**Armijo Condition**:
$$f(x_k + d_k(t)) \leq f(x_k) + c_1 t \nabla f(x_k)^T d_k'(0)$$
**Curvature Condition**:
$$|\nabla f(x_k + d_k(t))^T d_k'(t)| \leq c_2 |\nabla f(x_k)^T d_k'(0)|$$
where $d_k'(t) = \frac{d}{dt}d_k(t) = (1-2t)(-g_k) + 2t d_k^{LBFGS}$ and $0 < c_1 < c_2 < 1$.
**Note**: The initial directional derivative is:
$$d_k'(0) = -g_k$$

### 3.4 Complete QQN Algorithm

**Algorithm 2** (QQN Optimization)

```
Input: Initial point x_0, tolerance τ > 0, max iterations N
Initialize: L-BFGS history H_0 = ∅, k = 0
While k < N and ||∇f(x_k)|| > τ:
  1. Compute gradient: g_k ← ∇f(x_k)
  2. Compute L-BFGS direction: d_k^{LBFGS} ← LBFGS_Direction(g_k, H_k)
  3. Define parametric direction: d_k(t) = t(1-t)(-g_k) + t²d_k^{LBFGS}
  4. Line search: Find t_k^* ∈ [0,1] satisfying Strong Wolfe conditions
     If no such t exists, set t_k^* = argmin_{t∈[0,1]} f(x_k + d_k(t))
  5. Descent verification: If g_k^T d_k(t_k^*) ≥ 0, set t_k^* = min(0.01, ||g_k||)
  6. Update position: x_{k+1} ← x_k + d_k(t_k^*)
  7. Update L-BFGS history: H_{k+1} ← Update_LBFGS_History(H_k, d_k(t_k^*), g_{k+1} - g_k)
  8. k ← k + 1
Return x_k
```

## 4. Theoretical Analysis

### 4.1 Global Convergence

**Theorem 2** (Global Convergence): Under Assumptions 1-2, if the line search satisfies the Strong Wolfe conditions,
then:
$$\liminf_{k \to \infty} \|\nabla f(x_k)\| = 0$$
**Proof Sketch**: The proof follows the standard quasi-Newton convergence analysis:

1. **Descent Property**: By Theorem 1, each iteration produces descent when $\|\nabla f(x_k)\| > 0$.
2. **Sufficient Decrease**: The Armijo condition ensures:
   $$f(x_{k+1}) \leq f(x_k) - c_1 t_k^* \|g_k\|^2$$
3. **Bounded Sequence**: Since $f$ is bounded below and decreasing, $\{f(x_k)\}$ converges.
4. **Gradient Convergence**: Summing the sufficient decrease inequalities:
   $$\sum_{k=0}^{\infty} c_1 t_k^* \|g_k\|^2 \leq f(x_0) - f_* < \infty$$
   If $\liminf_{k \to \infty} \|\nabla f(x_k)\| > 0$, then $\|g_k\| \geq \delta > 0$ infinitely often.
   The curvature condition ensures $t_k^* \geq t_{min} > 0$, leading to contradiction. □

### 4.2 Local Convergence Rate

**Theorem 3** (Superlinear Convergence): If $f$ is twice continuously differentiable near a solution $x^*$
with $\nabla^2 f(x^*)$ positive definite, and the L-BFGS approximation converges to $[\nabla^2 f(x^*)]^{-1}$, then QQN
converges superlinearly.
**Proof Sketch**: Near the solution, the L-BFGS direction becomes increasingly accurate. The line search will
find $t_k^* \to 1$, reducing QQN to L-BFGS with its known superlinear convergence properties.

### 4.3 Robustness Properties

**Proposition 2** (Robustness to L-BFGS Failure): If the L-BFGS direction $d_k^{LBFGS}$ is not a descent direction (
i.e., $g_k^T d_k^{LBFGS} \geq 0$), then QQN automatically reduces to gradient descent behavior for small $t$.
**Proof**: For small $t$, $d_k(t) \approx t(-g_k)$, which is always a descent direction when $\|g_k\| > 0$. □

## 5. Implementation Considerations

### 5.1 Numerical Stability

**Gradient Magnitude Check**: If $\|g_k\| < \epsilon_{grad}$, terminate (approximate critical point).
**L-BFGS Reliability**: Skip L-BFGS updates if curvature condition $s_k^T y_k \leq \epsilon$ fails.
**Step Size Bounds**: Constrain $t_k^* \in [\epsilon_{min}, 1]$ where $\epsilon_{min} > 0$ prevents null steps.

### 5.2 Line Search Implementation

The line search operates on the function $\phi(t) = f(x_k + d_k(t))$ with derivative:
$$\phi'(t) = \nabla f(x_k + d_k(t))^T d_k'(t)$$
Standard line search algorithms (backtracking, zoom) can be applied directly.

### 5.3 Computational Complexity

**Per Iteration Cost**:

- L-BFGS direction: $O(mn)$ where $m$ is history size, $n$ is dimension
- Parametric evaluation: $O(n)$
- Line search: $O(L \cdot n)$ where $L$ is number of line search steps
- Total: $O((m + L)n)$
  **Memory Requirements**: $O(mn)$ for L-BFGS history storage.

## 6. Algorithmic Variants

### 6.1 Limited Memory Variant

For very large problems, limit L-BFGS history to $m \leq 20$ correction pairs.

### 6.2 Adaptive Parametrization

Alternative parametrizations can be explored:

- **Linear**: $d_k(t) = (1-t)(-g_k) + t d_k^{LBFGS}$
- **Cubic**: $d_k(t) = t(1-t)^2(-g_k) + t^3 d_k^{LBFGS}$

### 6.3 Trust Region Integration

QQN can be combined with trust regions by constraining $\|d_k(t)\| \leq \Delta_k$.

## 7. Convergence Criteria

**Gradient Norm**: $\|\nabla f(x_k)\| \leq \tau_g$
**Function Change**: $|f(x_{k+1}) - f(x_k)| \leq \tau_f (1 + |f(x_k)|)$
**Parameter Change**: $\|x_{k+1} - x_k\| \leq \tau_x (1 + \|x_k\|)$

## 8. Conclusion

The QQN algorithm provides a mathematically principled approach to combining gradient descent and quasi-Newton methods
through parametric interpolation. The theoretical analysis demonstrates global convergence under standard assumptions,
while the parametric formulation provides automatic adaptation between first and second-order methods based on local
problem geometry.
The key mathematical insight is that the quadratic parametrization $d_k(t) = t(1-t)(-g_k) + t^2 d_k^{LBFGS}$ naturally
transitions from gradient descent behavior (small $t$) to quasi-Newton behavior ($t \approx 1$), with the line search
automatically discovering the optimal balance.

## References

- Liu, D. C., & Nocedal, J. (1989). On the limited memory BFGS method for large scale optimization. Mathematical
  Programming, 45(1-3), 503-528.
- Nocedal, J., & Wright, S. J. (2006). Numerical Optimization (2nd ed.). Springer.
- Wolfe, P. (1969). Convergence conditions for ascent methods. SIAM Review, 11(2), 226-235.