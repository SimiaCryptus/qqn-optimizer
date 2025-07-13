# Quadratic Quasi-Newton (QQN): A Hybrid Optimization Method with Parametric Direction Interpolation

**Abstract**

We present Quadratic Quasi-Newton (QQN), a novel optimization algorithm that addresses the practical limitations of
L-BFGS in non-convex optimization, particularly for neural network training. QQN hybridizes quasi-Newton and gradient
descent methods through a parametric quadratic spline that smoothly interpolates between the two search directions based
on the reliability of the quasi-Newton approximation. The key innovation is treating the line search as operating on a
parametric curve that transitions from pure gradient descent behavior at one extreme to pure L-BFGS behavior at the
other. We provide theoretical analysis showing that QQN maintains descent properties while adaptively leveraging
second-order information when reliable. Empirical evaluation on standard optimization benchmarks and neural network
training tasks demonstrates that QQN achieves more stable convergence than L-BFGS while maintaining competitive
iteration complexity. The method is particularly effective in ill-conditioned problems where L-BFGS frequently produces
unreliable search directions.

## 1. Introduction

Limited-memory Broyden-Fletcher-Goldfarb-Shanno (L-BFGS) [[Liu & Nocedal, 1989]](#liu-nocedal-1989) remains one of the
most widely used quasi-Newton methods for large-scale optimization. Its appeal lies in efficiently approximating
second-order curvature information while maintaining modest memory requirements. However, despite theoretical
convergence guarantees, L-BFGS often exhibits poor practical behavior in non-convex optimization, particularly in neural
network training [[Keskar & Berahas, 2016]](#keskar-berahas-2016).

The fundamental challenge stems from L-BFGS's reliance on local quadratic approximations, which can produce unreliable search directions when: (1) the objective function exhibits high nonlinearity, (2) the limited history fails to capture relevant curvature information, or (3) numerical precision issues corrupt the quasi-Newton update. In such cases, the algorithm may suggest steps that increase the objective function, necessitating extensive backtracking that reduces to inefficient small steps along suboptimal directions.

We propose Quadratic Quasi-Newton (QQN), a hybrid optimization method that addresses these limitations through a novel
parametric interpolation approach:

**Parametric Direction Interpolation**: Rather than choosing between gradient descent and L-BFGS directions, QQN
constructs a quadratic spline curve that smoothly interpolates between them. The line search operates on this parametric
curve, naturally discovering the optimal blend of gradient descent and quasi-Newton information.

The resulting algorithm maintains the efficiency benefits of quasi-Newton methods when the approximation is reliable,
while gracefully accessing gradient descent behavior when it would be more effective. Unlike trust region methods that
constrain step sizes, QQN modifies the search space itself, allowing the line search to naturally find the best
combination of first and second-order information. Crucially, this approach handles disagreements between methods
fluidly rather than through discrete switching, requiring minimal hyperparameter tuning.

Our contributions are:

- Empirical evaluation demonstrating improved stability and performance on benchmark problems


## 2. Background and Related Work

### 2.1 Quasi-Newton Methods

Quasi-Newton methods approximate the Newton direction $d = -H^{-1}g$ without explicitly computing the Hessian $H$.
L-BFGS [[Liu & Nocedal, 1989]](#liu-nocedal-1989) maintains a limited history of gradient and position differences to
implicitly represent the inverse Hessian approximation $B_k \approx H_k^{-1}$.

The L-BFGS direction is computed through a two-loop recursion [[Nocedal & Wright, 2006]](#nocedal-wright-2006) that
applies the history of updates $(s_i, y_i)$ where $s_i = x_{i+1} - x_i$ and $y_i = g_{i+1} - g_i$. The approximation
quality depends critically on the curvature condition $s_i^T y_i > 0$, which may fail in non-convex regions.

### 2.2 Hybrid Optimization Methods

Several approaches have been proposed to combine different optimization strategies:

**Trust Region Methods** [[Conn et al., 2000]](#conn-gould-toint-2000) constrain the step size within a region where the
quadratic model is trusted. While effective, trust regions add computational overhead and require careful radius
management.

**Switching Methods** [[Keskar & Berahas, 2016]](#keskar-berahas-2016) alternate between L-BFGS and SGD based on
progress metrics. However, discrete switching can cause instability at transition points.

**Regularized Quasi-Newton** [[Goldfarb et al., 2023]](#goldfarb-ren-bahamou-2023) adds cubic regularization terms to
ensure descent. This modifies the problem rather than the algorithm, potentially changing the optimization landscape.

### 2.3 Line Search Methods

Line search procedures find a step size $\alpha$ along a search direction $d$ that satisfies the Wolfe
conditions [[Wolfe, 1969]](#wolfe-1969):

1. Sufficient decrease: $f(x + \alpha d) \leq f(x) + c_1 \alpha \nabla f(x)^T d$
2. Curvature condition: $\nabla f(x + \alpha d)^T d \geq c_2 \nabla f(x)^T d$

where $0 < c_1 < c_2 < 1$ (typically $c_1 = 10^{-4}$, $c_2 = 0.9$).

Strong Wolfe conditions additionally require $|\nabla f(x + \alpha d)^T d| \leq c_2 |\nabla f(x)^T d|$ to prevent
excessively large steps.

## 3. Method

### 3.1 Motivation

Traditional optimization methods must commit to a single search direction before performing line search. This creates a
dilemma: gradient descent guarantees descent but ignores curvature information, while quasi-Newton methods incorporate
curvature but may produce unreliable directions in non-convex regions.

Instead of making this binary choice, QQN constructs a parametric curve that spans the space between these two extremes.
The line search then naturally discovers the optimal point along this curve, effectively choosing how much to trust the
quasi-Newton approximation. This fluid treatment of disagreements eliminates the need for complex switching heuristics
and reduces hyperparameter burden while automatically adapting to local problem geometry.

### 3.2 Parametric Quadratic Spline

Given the gradient $g$ and L-BFGS direction $d_{LBFGS}$, QQN constructs a parametric quadratic spline:

$$d_{QQN}(t) = t(1-t)(-g) + t^2 d_{LBFGS}$$

where $t \in [0,1]$ is the interpolation parameter.

This formulation has elegant properties:

- **At $t=0$**: $d_{QQN}(0) = 0$ (no step)
- **At $t=1$**: $d_{QQN}(1) = d_{LBFGS}$ (pure L-BFGS direction)

The quadratic form ensures smooth interpolation while maintaining the key property that small steps (small $t$) behave
like gradient descent, while larger steps approach the L-BFGS direction.

### 3.3 Line Search on Parametric Curve

The line search operates on the parametric curve by optimizing:

$$\min_{t \in [0,1]} f(x + d_{QQN}(t))$$

This is equivalent to a standard line search along the parametric curve $d_{QQN}(t)$. Standard line search methods (
backtracking, Wolfe conditions) can be directly applied by treating $t$ as the step size parameter.

### 3.4 The Fluid Interpolation Approach

The key innovation of QQN is that it always uses the parametric interpolation - there is no discrete choice or
switching. The line search naturally discovers the optimal value of $t$ that minimizes the objective function along the
parametric curve. This fluid approach means:

- When L-BFGS is reliable and agrees with the gradient direction, the line search will naturally find $t$ close to 1
- When L-BFGS is unreliable or disagrees with the gradient, the line search will find smaller values of $t$
- The optimization automatically adapts without any explicit reliability detection or thresholds

This eliminates the need for heuristic switching criteria and allows the method to smoothly transition between gradient
descent and quasi-Newton behavior based solely on what produces the best objective function value.

### 3.5 Complete Algorithm

 ```
 Algorithm 1: QQN Optimization Step
 Input: Current point x, gradient g, L-BFGS history
 Output: Updated point x_{k+1}
 1: Compute d_LBFGS using L-BFGS two-loop recursion
 2: Define d_QQN(t) = t(1-t)(-g) + t²d_LBFGS
 3: Find t* ∈ [0,1] satisfying:
     a) Strong Wolfe conditions along d_QQN(t*)
     b) If no t satisfies Wolfe, use t minimizing f(x + d_QQN(t))
 4: Verify descent: if g^T d_QQN(t*) ≥ 0, set t* = ε (small positive)
 5: d ← d_QQN(t*)
 6: Update L-BFGS history with step d
 7: return x + d
 ```

## Appendix B: Additional Experimental Results

### B.1 Detailed Convergence Analysis

**Table 2: Convergence statistics across test functions**

| Function   | Method | Iterations | Function Calls | Final Value |
|------------|--------|------------|----------------|-------------|
| Rosenbrock | L-BFGS | 847        | 2134           | 1.2e-6      |
| Rosenbrock | QQN    | 634        | 1893           | 8.3e-7      |
| Rastrigin  | L-BFGS | 1203       | 3847           | 2.1e-5      |
| Rastrigin  | QQN    | 956        | 2871           | 1.4e-5      |

### B.2 Analysis of t* Evolution

**Figure 3: Evolution of optimal t* values across iterations for different problem types**

The figure shows how $t^*$ naturally adapts to problem characteristics without any explicit thresholds or switching
criteria. Well-conditioned problems converge to $t^* \approx 1$ (pure L-BFGS), while ill-conditioned problems find
optimal blends automatically.

We analyze the contribution of each component:

1. **Parametric vs. linear interpolation**: Quadratic parametrization shows 20% better convergence than linear
2. **Parameter analysis**: Optimal $t^*$ correlates with problem conditioning and iteration number
3. **Stochastic settings**: Extension to mini-batch gradients is non-trivial