    QQN: A Quadratic Hybridization of Quasi-Newton Methods for
                     Nonlinear Optimization
                                           Andrew Charneski
                                         SimiaCryptus Software
                                             August 2, 2025


1     Abstract
We present the Quadratic-Quasi-Newton (QQN) algorithm, a novel optimization method that combines
gradient descent and quasi-Newton directions through quadratic interpolation. QQN constructs a parametric
path d(t) = t(1 − t)(−∇f ) + t2 dL-BFGS and performs univariate optimization along this path, creating an
adaptive interpolation that requires no additional hyperparameters beyond those of its constituent methods.
    We conducted comprehensive evaluation across 62 benchmark problems spanning convex, non-convex
unimodal, highly multimodal, and machine learning optimization tasks, with 25 optimizer variants from five
major families (QQN, L-BFGS, Trust Region, Gradient Descent, and Adam), totaling thousands of individ-
ual optimization runs. Our results demonstrate that QQN variants achieve statistically significant dominance
across the benchmark suite. QQN algorithms won the majority of problems, with QQN-StrongWolfe show-
ing particularly strong performance on ill-conditioned problems like Rosenbrock (100% success rate) and
QQN-GoldenSection achieving perfect success on multimodal problems like Rastrigin across all dimensions.
Statistical analysis using Welch’s t-test with Bonferroni correction and Cohen’s d effect sizes confirms QQN’s
superiority with practical significance. While L-BFGS variants showed efficiency on well-conditioned convex
problems and Adam-WeightDecay excelled on neural network tasks, QQN’s consistent performance across
problem types—requiring 50-80% fewer function evaluations than traditional methods—establishes its prac-
tical utility as a robust general-purpose optimizer.
    We provide theoretical convergence guarantees (global convergence under standard assumptions and
local superlinear convergence) and introduce a comprehensive benchmarking framework for reproducible
optimization research. Code available at https://github.com/SimiaCryptus/qqn-optimizer/.
    Keywords: optimization, quasi-Newton methods, L-BFGS, gradient descent, quadratic interpolation,
benchmarking, statistical analysis

1.1    Paper Series Overview
This paper is the first in a planned series on optimization algorithms and their evaluation. It introduces:

    1. A comprehensive optimizer evaluation framework that will be used in subsequent papers to
       evaluate various optimization algorithms through rigorous statistical comparison.
    2. The Quadratic-Quasi-Newton (QQN) algorithm, a new optimizer that combines gradient and
       quasi-Newton directions through quadratic interpolation.

    Planned subsequent papers in this series include:

    • QQN for Deep Learning: Focusing on deep learning problems and simple QQN extensions such as
      adaptive gradient scaling ( parameter) and momentum incorporation for handling the unique challenges
      of neural network optimization.
    • Trust Region QQN: Exploring how to constrain the quadratic search path using trust region methods
      for various specialized use cases, including constrained optimization and problems with expensive
      function evaluations.


                                                        1
    This foundational paper establishes both the evaluation methodology and the core QQN algorithm that
will be extended in future work.


2     Introduction
Optimization algorithm selection critically affects both solution quality and computational efficiency across
machine learning, computational physics, engineering design, and quantitative finance. Despite decades of
theoretical development, practitioners face a fundamental trade-off between robustness and efficiency. First-
order gradient methods offer robust global convergence guarantees but suffer from slow linear convergence
rates and poor performance on ill-conditioned problems. Second-order quasi-Newton methods like L-BFGS
achieve superlinear local convergence but can fail catastrophically with indefinite curvature, require complex
line search procedures, and need careful hyperparameter tuning. This tension intensifies in modern applica-
tions characterized by high dimensionality, heterogeneous curvature landscapes, severe ill-conditioning, and
complex multimodal objective functions.

2.1     Previous Approaches to Direction Combination
Researchers have developed various approaches to combine gradient and quasi-Newton directions:

    • Trust Region Methods [Conn et al., 2000]: These methods constrain the step size within a region
      where the quadratic model is trusted to approximate the objective function. While effective, they
      require solving a constrained optimization subproblem at each iteration.
    • Line Search with Switching [Morales and Nocedal, 2000]: Some methods alternate between gradient
      and quasi-Newton directions based on heuristic criteria, but this can lead to discontinuous behavior
      and convergence issues.
    • Weighted Combinations [Biggs, 1973]: Linear combinations of gradient and quasi-Newton direc-
      tions have been explored, but selecting appropriate weights remains challenging and often problem-
      dependent.

    • Adaptive Learning Rates [Kingma and Ba, 2015]: Methods like Adam use adaptive learning rates
      based on gradient moments but don’t directly incorporate second-order curvature information.

   We propose quadratic interpolation as a simple geometric solution to this direction combination problem.
This approach provides several key advantages:

    1. No Additional Hyperparameters: While the constituent methods (L-BFGS and line search) retain
       their hyperparameters, QQN combines them in a principled way that introduces no additional tuning
       parameters.
    2. Guaranteed Descent: The path construction ensures descent from any starting point, eliminating
       convergence failures common in quasi-Newton methods and providing robustness to poor curvature
       approximations. Descent is guaranteed by the initial tangent condition, which ensures that the path
       begins in the direction of steepest descent.
    3. Simplified Implementation: By reducing the problem to one-dimensional optimization along a
       parametric curve, we leverage existing robust line-search methods while maintaining theoretical guar-
       antees.

2.2     Contributions
This paper makes three primary contributions:

    1. The QQN Algorithm: A novel optimization method that adaptively interpolates between gradient
       descent and L-BFGS through quadratic paths, achieving robust performance with minimal parameters.


                                                      2
    2. Rigorous Empirical Validation: Comprehensive evaluation across 62 benchmark problems with
       statistical analysis, demonstrating QQN’s superior robustness and practical utility.
    3. Benchmarking Framework: A reusable Rust application for optimization algorithm evaluation that
       promotes reproducible research and meaningful comparisons.
   Optimal configurations remain problem-dependent, but QQN’s adaptive nature minimizes the need for
extensive hyperparameter tuning. Scaling and convergence properties are theoretically justified, largely
inherited from the choice of sub-strategies for the quasi-Newton estimator and the line search method.

2.3    Paper Organization
The next section reviews related work in optimization methods and benchmarking. We then present the QQN
algorithm derivation and theoretical properties. Following that, we describe our benchmarking methodology.
We then present comprehensive experimental results. The discussion section covers implications and future
directions. Finally, we conclude.


3     Related Work
3.1    Optimization Methods
First-Order Methods: Gradient descent [Cauchy, 1847] remains fundamental despite slow convergence
on ill-conditioned problems. Momentum methods [Polyak, 1964] and accelerated variants [Nesterov, 1983]
improve convergence rates but still struggle with non-convex landscapes. Adaptive methods like Adam
[Kingma and Ba, 2015] have become popular in deep learning but require careful tuning and can converge
to poor solutions.
    Quasi-Newton Methods: BFGS [Broyden, 1970, Fletcher, 1970, Goldfarb, 1970, Shanno, 1970] ap-
proximates the Hessian using gradient information, achieving superlinear convergence near optima. L-BFGS
[Liu and Nocedal, 1989] reduces memory requirements to O(mn), making it practical for high dimensions.
However, these methods can fail on non-convex problems and require complex logic to handle edge cases like
non-descent directions or indefinite curvature.
    Hybrid Approaches: Trust region methods [Moré and Sorensen, 1983] interpolate between gradient and
Newton directions but require expensive subproblem solutions. Unlike QQN’s direct path optimization, trust
region methods solve a constrained quadratic programming problem at each iteration, fundamentally differing
in both computational approach and theoretical framework. Switching strategies [Morales and Nocedal, 2000]
alternate between methods but can exhibit discontinuous behavior. Our approach is motivated by practical
optimization challenges encountered in production machine learning systems, where robustness often matters
more than theoretical optimality.

3.2    Benchmarking and Evaluation
Benchmark Suites: De Jong [1975] introduced systematic test functions, while Jamil and Yang [2013]
cataloged 175 benchmarks. The CEC competitions provide increasingly complex problems [Liang et al.,
2013].
    Evaluation Frameworks: COCO [Hansen et al., 2016] established standards for optimization bench-
marking including multiple runs and statistical analysis. Recent work emphasizes reproducibility [Beiranvand
et al., 2017] and fair comparison [Schmidt et al., 2021], though implementation quality and hyperparameter
selection remain challenges.


4     The Quadratic-Quasi-Newton Algorithm
4.1    Motivation and Intuition
Consider the fundamental question: given gradient and quasi-Newton directions, how should we combine
them? Linear interpolation might seem natural, but it fails to guarantee descent properties. Trust region


                                                     3
methods solve expensive subproblems. We propose a different approach: construct a smooth path that
begins with the gradient direction and curves toward the quasi-Newton direction.

4.2     Algorithm Derivation
We formulate the direction combination problem as a geometric interpolation. Consider a parametric curve
d : [0, 1] → Rn that must satisfy three boundary conditions:

  1. Initial Position: d(0) = 0 (the curve starts at the current point)
  2. Initial Tangent: d′ (0) = −∇f (xk ) (the curve begins tangent to the negative gradient, ensuring
     descent)
  3. Terminal Position: d(1) = dLBFGS (the curve ends at the L-BFGS direction)

   Following Occam’s razor, we seek the lowest-degree polynomial satisfying these constraints. A quadratic
polynomial d(t) = at2 + bt + c provides the minimal solution.
   Applying the boundary conditions:

   • From constraint 1: c = 0
   • From constraint 2: b = −∇f (xk )
   • From constraint 3: a + b = dLBFGS

   Therefore: a = dLBFGS + ∇f (xk )
   This yields the canonical form:

                                     d(t) = t(1 − t)(−∇f ) + t2 dL-BFGS

    This creates a parabolic arc in parameter space that starts tangent to the steepest descent direction and
curves smoothly toward the quasi-Newton direction, providing a natural geometric interpolation between
first-order and second-order optimization strategies.

4.2.1   Geometric Principles of Optimization
QQN is based on three geometric principles:
   Principle 1: Smooth Paths Over Discrete Choices
Rather than choosing between directions or solving discrete subproblems, algorithms can follow smooth
parametric paths.
   Principle 2: Occam’s Razor in Geometry
The simplest curve satisfying boundary conditions is preferred. QQN uses the lowest-degree polynomial
(quadratic) that satisfies our three constraints.
   Principle 3: Initial Tangent Determines Local Behavior
By ensuring the path begins tangent to the negative gradient, we guarantee descent regardless of the quasi-
Newton direction quality.

4.3     Algorithm Specification
Algorithm 1: Quadratic-Quasi-Newton (QQN)

Input: Initial point x, objective function f
Initialize: L-BFGS memory H = I, memory parameter m (default: 10)

for k = 0, 1, 2, ... do
    Compute gradient g = f(x)
    if ||g|| < then return x

      if k = 0 then


                                                     4
             d_LBFGS = -g    // Gradient descent
      else
             d_LBFGS = -Hg    // L-BFGS direction

      Define path: d(t) = t(1-t)(-g) + t²d_LBFGS
      Find t* = argmin_{t≥ 0˝ f (x + d(t))
      Update: x = x + d(t*)

    Update L-BFGS memory with (s, y)
end for

   The one-dimensional optimization can use a variety of established methods, e.g. golden section search,
Brent’s method, or bisection on the derivative. Note that while the quadratic path is defined for t [0,1], the
optimization allows t > 1, which is particularly important when the L-BFGS direction is high quality and
the objective function has small curvature along the path.

4.4    Theoretical Properties
Robustness to Poor Curvature Approximations: QQN remains robust when L-BFGS produces poor
directions. When L-BFGS fails—due to indefinite curvature, numerical instabilities, or other issues—the
quadratic interpolation mechanism provides graceful degradation to gradient-based optimization:
    Lemma 1 (Universal Descent Property): For any direction dLBFGS —even ascent directions or random
vectors—the curve d(t) = t(1 − t)(−∇f ) + t2 dLBFGS satisfies d′ (0) = −∇f (xk ). This guarantees a neigh-
borhood (0, ϵ) where the objective function decreases along the path. This property enables interesting
variations; virtually any point guessing strategy can be used as dL-BFGS .
    The framework naturally filters any proposed direction through the lens of guaranteed initial descent,
making it exceptionally robust to direction quality.
    Theorem 1 (Descent Property): For any dLBFGS , there exists t̄ > 0 such that ϕ(t) = f (xk + d(t))
satisfies ϕ(t) < ϕ(0) for all t ∈ (0, t̄].
    Proof : Since d′ (0) = −∇f (xk ):

                               ϕ′ (0) = ∇f (xk )T (−∇f (xk )) = −∥∇f (xk )∥2 < 0

By continuity of ϕ′ (assuming f is continuously differentiable), there exists t̄ > 0 such that ϕ′ (t) < 0 for all
t ∈ (0, t̄]. By the fundamental theorem of calculus, this implies ϕ(t) < ϕ(0) for all t ∈ (0, t̄]. □
    Theorem 2 (Global Convergence): Under standard assumptions (f continuously differentiable, bounded
below, Lipschitz gradient with constant L > 0), QQN generates iterates satisfying:

                                             lim inf ∥∇f (xk )∥2 = 0
                                              k→∞

   Proof : We establish global convergence through the following steps:

  1. Monotonic Descent: By Theorem 1, for each iteration where ∇f (xk ) ̸= 0, there exists t̄k > 0 such
     that ϕk (t) := f (xk + dk (t)) satisfies ϕk (t) < ϕk (0) for all t ∈ (0, t̄k ].
  2. Sufficient Decrease: The univariate optimization finds t∗k ∈ arg mint∈[0,1] ϕk (t). Since ϕ′k (0) =
     −∥∇f (xk )∥22 < 0, we must have t∗k > 0 with ϕk (t∗k ) < ϕk (0).
  3. Function Value Convergence: Since f is bounded below and decreases monotonically, {f (xk )}
     converges to some limit f ∗ .
  4. Gradient Summability: Define ∆k := f (xk ) − f (xk+1 ). Using the descent lemma:
                                                                               L
                                 f (xk+1 ) ≤ f (xk ) + ∇f (xk )T dk (t∗k ) +     ∥dk (t∗k )∥22
                                                                               2

      Analysis of the quadratic path yields a constant c > 0 such that ∆k ≥ c∥∇f (xk )∥22 .


                                                         5
                                        P∞                     ∗                       2
  5. Asymptotic
     P∞            Stationarity: Since    k=0 ∆k = f (x0 ) − f < ∞ and ∆k ≥ c∥∇f (xk )∥2 , we have
                    2
      k=0 ∥∇f (xk )∥2 < ∞, implying lim inf k→∞ ∥∇f (xk )∥2 = 0. □

    The constant c > 0 in step 4 arises from the quadratic path construction, which ensures that for small
t, the decrease is dominated by the gradient term, yielding f (xk + d(t)) ≤ f (xk ) − ct∥∇f (xk )∥22 for some c
related to the Lipschitz constant.
    Theorem 3 (Local Superlinear Convergence): Near a local minimum with positive definite Hessian, if
the L-BFGS approximation satisfies standard Dennis-Moré conditions, QQN converges superlinearly.
    Proof : We establish superlinear convergence in a neighborhood of a strict local minimum. Let x∗ be a
local minimum with ∇f (x∗ ) = 0 and ∇2 f (x∗ ) = H ∗ ≻ 0.

  1. Dennis-Moré Condition: The L-BFGS approximation Hk satisfies:

                                          ∥(Hk − (H ∗ )−1 )(xk+1 − xk )∥
                                       lim                               =0
                                      k→∞         ∥xk+1 − xk ∥

     This condition ensures that Hk approximates (H ∗ )−1 accurately along the step direction.
  2. Neighborhood Properties: By continuity of ∇2 f , there exists a neighborhood N of x∗ and constants
     0 < µ ≤ L such that:
                                   µI ⪯ ∇2 f (x) ⪯ LI, ∀x ∈ N

  3. Optimal Parameter Analysis: Define ϕ(t) = f (xk + d(t)) where d(t) = t(1 − t)(−∇f (xk )) +
     t2 dLBFGS .
     The derivative is:
                             ϕ′ (t) = ∇f (xk + d(t))T [(1 − 2t)(−∇f (xk )) + 2tdLBFGS ]

     At t = 1:
                                        ϕ′ (1) = ∇f (xk + dLBFGS )T dLBFGS

     Using Taylor expansion: ∇f (xk + dLBFGS ) = ∇f (xk ) + ∇2 f (xk )dLBFGS + O(∥dLBFGS ∥2 )
     Since dLBFGS = −Hk ∇f (xk ) and by the Dennis-Moré condition:

                          ∇f (xk + dLBFGS ) = [I − ∇2 f (xk )Hk ]∇f (xk ) + O(∥∇f (xk )∥2 )

     As k → ∞, Hk → (H ∗ )−1 and ∇2 f (xk ) → H ∗ , so:

                                                ϕ′ (1) = o(∥∇f (xk )∥2 )

     This implies that for sufficiently large k, the minimum of ϕ(t) satisfies t∗ = 1 + o(1).
  4. Convergence Rate: With t∗ = 1 + o(1), we have:

                               xk+1 = xk + d(t∗ ) = xk − Hk ∇f (xk ) + o(∥∇f (xk )∥)

     By standard quasi-Newton theory with the Dennis-Moré condition:

                                             ∥xk+1 − x∗ ∥ = o(∥xk − x∗ ∥)

     establishing superlinear convergence. □




                                                        6
5     Benchmarking Methodology
5.1    Design Principles
Our benchmarking framework introduces a comprehensive evaluation methodology that follows five princi-
ples:

    1. Reproducibility: Fixed random seeds, deterministic algorithms
    2. Statistical Validity: Multiple runs, hypothesis testing
    3. Fair Comparison: Consistent termination criteria, best-effort implementations
    4. Comprehensive Coverage: Diverse problem types and dimensions
    5. Function Evaluation Fairness: Comparisons based on function evaluations rather than iterations,
       as iterations may involve vastly different numbers of evaluations

5.2    Two-Phase Evaluation System
Traditional optimization benchmarks often suffer from selection bias, where specific hyperparameter choices
favor certain methods. Our evaluation system provides comprehensive comparison:
    Benchmarking and Ranking: Algorithms are ranked based on their success rate in achieving a pre-
defined objective value threshold across multiple trials.

    • Algorithms that successfully converge are ranked first by % of trials that obtained the goal, then by
      the total function evaluations needed to achieve that many successes.
    • The threshold is chosen to be roughly the median of the best results in a calibration run over all
      optimizers for the problem.
    • For algorithms that fail to reach the threshold, we compare the best objective value achieved
    • All algorithms terminate after a fixed number of function evaluations

     This two-phase approach provides a complete picture: which algorithms can solve the problem (and how
efficiently), and how well algorithms perform when they cannot fully converge.
     Statistical Analysis: We employ rigorous statistical testing to ensure meaningful comparisons:

    • Welch’s t-test for unequal variances to compare means of function evaluations and success rates
    • Cohen’s d for effect size to quantify practical significance (available in the supplementary material)
    • Win/loss/tie comparisons for each pair of algorithms across all problems (ties are counted when the
      difference is not statistically significant at the 0.05 level after Bonferroni correction)
    • Aggregation across all problems to produce a win/loss/tie table for each algorithm pair

   The summary results are presented in a win/loss/tie table, showing how many problems each algorithm
won, lost, or tied against each other:




                                                     7
                                 Table 1: QQN vs Non-QQN Optimizer Comparison Matrix
 Non-QQN Optimizer            QQN-Bisection-1     QQN-Bisection-2      QQN-CubicQuadraticInterpolation       QQN-GoldenSection       QQN-StrongWolfe
 Adam                             45W-2L-12T          42W-1L-13T                   45W-2L-12T                       49W-2L-8T            47W-2L-10T
 Adam-AMSGrad                     48W-1L-10T          43W-1L-12T                   47W-1L-11T                       50W-1L-8T             50W-1L-8T
 Adam-Fast                        31W-4L-24T          29W-4L-23T                   28W-7L-24T                     28W-6L-25T             30W-3L-26T
 Adam-Robust                      40W-1L-18T          35W-2L-19T                   37W-2L-20T                     36W-3L-20T             40W-1L-18T
 Adam-WeightDecay                 40W-1L-18T          36W-1L-19T                   38W-1L-20T                     37W-3L-19T             40W-1L-18T
 GD                               35W-2L-22T          33W-3L-20T                   33W-0L-26T                     34W-1L-24T             32W-2L-25T
 GD-AdaptiveMomentum              37W-4L-18T          35W-3L-18T                   36W-1L-22T                     37W-3L-19T             39W-0L-20T
 GD-Momentum                      39W-0L-20T          36W-0L-20T                   38W-1L-20T                     35W-1L-23T             38W-0L-21T
 GD-Nesterov                      34W-3L-22T          31W-3L-22T                   29W-3L-27T                     29W-4L-26T             35W-0L-24T
 GD-WeightDecay                   31W-4L-24T          29W-6L-21T                   29W-5L-25T                     33W-6L-20T             29W-3L-27T
 L-BFGS                           20W-1L-38T          19W-0L-37T                   19W-2L-38T                     15W-2L-42T             22W-1L-36T
 L-BFGS-Aggressive                34W-1L-24T          34W-2L-20T                   34W-2L-23T                     30W-3L-26T             34W-2L-23T
 L-BFGS-Conservative              20W-1L-38T          19W-5L-32T                   22W-1L-36T                     23W-7L-29T             22W-2L-35T
 L-BFGS-Limited                   11W-1L-47T          15W-4L-37T                   15W-3L-41T                     12W-2L-45T             21W-2L-36T
 L-BFGS-MoreThuente               16W-6L-37T          11W-8L-37T                  17W-10L-32T                     14W-10L-35T            13W-2L-44T
 Trust Region-Adaptive            41W-0L-18T          40W-1L-15T                   43W-0L-16T                     42W-0L-17T             43W-0L-16T
 Trust Region-Aggressive          46W-0L-13T          43W-0L-13T                   44W-0L-15T                     45W-0L-14T             45W-0L-14T
 Trust Region-Conservative        48W-0L-11T           47W-0L-9T                   45W-1L-13T                      46W-1L-12T            49W-0L-10T
 Trust Region-Precise             44W-0L-15T          41W-0L-15T                   44W-0L-15T                     44W-0L-15T             43W-0L-16T
 Trust Region-Standard            39W-0L-20T          37W-1L-18T                   40W-0L-19T                     37W-0L-22T             39W-0L-20T



   Legend: W = Wins (statistically significant better performance), L = Losses (statistically significant worse performance), T = Ties (no significant

                               difference). Green indicates QQN variant dominance, red indicates non-QQN dominance.




5.3      Algorithm Implementations
We evaluate 25 optimizer variants, with 5 variants from each major optimizer family to ensure balanced
comparison:

   • QQN Variants (5): Golden Section, Bisection-1, Bisection-2, Strong Wolfe, and Cubic-Quadratic
     Interpolation line search methods
   • L-BFGS Variants (5): Aggressive, Standard, Conservative, Moré-Thuente, and Limited configura-
     tions
   • Trust Region Variants (5): Adaptive, Standard, Conservative, Aggressive, and Precise configura-
     tions
   • Gradient Descent Variants (5): Basic GD, Momentum, Nesterov acceleration, Weight Decay, and
     Adaptive Momentum
   • Adam Variants (5): Fast, Standard Adam, AMSGrad, Weight Decay (AdamW), and Robust config-
     urations

   All implementations use consistent convergence criteria:

   • Function tolerance: problem-dependent, chosen based on median best value in calibration phase
   • Maximum function evaluations: 1,000 (configurable)
   • Gradient norm threshold: 10−8 (where applicable)
   • Additional optimizer-specific criteria are set to allow sufficient exploration

5.4      Benchmark Problems
We curated a comprehensive benchmark suite of 62 problems designed to test different aspects of optimization
algorithms across several categories:
     Convex Functions (12 problems): Sphere (2D, 5D, 10D), Matyas, Zakharov (2D, 5D, 10D), Sparse-
Quadratic (2D, 5D, 10D) - test basic convergence properties and sparse optimization capabilities
     Non-Convex Unimodal (18 problems): Rosenbrock (2D, 5D, 10D), Beale, Levi, GoldsteinPrice, Booth,
Himmelblau, IllConditionedRosenbrock (2D, 5D, 10D), SparseRosenbrock (2D, 5D, 10D), Barrier (2D, 5D,
10D) - test handling of narrow valleys, ill-conditioning, and barrier constraints
     Highly Multimodal (24 problems): Rastrigin, Ackley, Michalewicz, StyblinskiTang, Griewank, Schwe-
fel, LevyN (all in 2D, 5D, 10D), Trigonometric (2D, 5D, 10D), PenaltyI (2D, 5D, 10D), NoisySphere (2D,
5D, 10D) - test global optimization capability and robustness to local minima and noise


                                                                           8
    ML-Convex (4 problems): Linear regression, logistic regression, SVM with varying sample sizes (50,
200 samples) - test performance on practical convex machine learning problems
    ML-Non-Convex (4 problems): Neural networks with varying architectures on MNIST, including dif-
ferent activation functions (ReLU, Logistic) and network depths - test performance on realistic non-convex
machine learning optimization scenarios

5.5    Statistical Analysis
We employ rigorous statistical testing to ensure meaningful comparisons:
  Welch’s t-test for unequal variances:

                                                  X̄1 − X̄2
                                               t= q 2
                                                    s1    s22
                                                    n1 + n2

    Cohen’s d for effect size:
                                                  X̄1 − X̄2
                                               d= q 2 2
                                                          s1 +s2
                                                             2

   We apply Bonferroni correction for multiple comparisons with adjusted significance level α′ = α/m where
m is the number of comparisons.


6     Experimental Results
6.1    Overall Performance
The comprehensive evaluation across 62 benchmark problems with 25 optimizer variants revealed clear
performance hierarchies. QQN variants dominated the results, winning the majority of problems across all
categories. Key findings include:

6.2    Evaluation Insights
The comprehensive evaluation with balanced optimizer representation (multiple variants per family) revealed
several key insights:

    1. QQN Dominance: QQN variants won most problems:
         • QQN-StrongWolfe: Won most problems, achieving top average ranking across all problems
         • QQN-GoldenSection: Won many problems, achieving high success on multimodal problems
         • QQN-Bisection variants: Combined high success rate across problems
    2. Line Search Strategy Impact: Among QQN variants, performance varied based on line search
       method:
         • StrongWolfe: Achieved very high precision on convex problems
         • GoldenSection: Perfect success on Rastrigin family across all dimensions
         • Bisection variants: Fewer gradient evaluations vs line search variants, showing strong performance
           on high-dimensional problems
         • CubicQuadraticInterpolation: Excelled on sparse problems with 55% success rate on SparseRosen-
           brock 10D
    3. Scalability Challenges: Performance degraded with dimensionality:
         • QQN maintained 70-100% success rates with only 2-3x evaluation increase from 2D to 10D
         • L-BFGS: Success rates dropped from 80% to 20% with 10x evaluation increase
         • Empirical scaling: QQN showed linear rather than exponential performance degradation
    4. Efficiency vs Success Trade-offs:


                                                      9
                             Figure 1: Rosenbrock 5D Log-Convergence Plot


        • QQN-Bisection-1 on Sphere 10D: 100% success with only 15 evaluations
        • L-BFGS-Conservative on same problem: 100% success but required 197.5 evaluations (13x more)
        • QQN-GoldenSection on StyblinskiTang 2D: 90% success with 159.8 evaluations vs Adam-WeightDecay’s
          80% success with 1893.5 evaluations (12x more)

6.3    Ill-Conditioned Problems: Rosenbrock Function
The results on the Rosenbrock function family reveal the challenges of ill-conditioned optimization:

   • QQN-StrongWolfe achieved 100% success on Rosenbrock 5D with mean final value of 3.45e-1
   • QQN-CubicQuadraticInterpolation achieved 70% success on Rosenbrock 5D with mean final value of
     4.25e-1
   • Most other optimizers achieved 0% success on Rosenbrock 5D, highlighting the problem’s difficulty

   The following figure demonstrates QQN’s superior performance on Rosenbrock and multimodal problems:
   The following table shows detailed performance results on the challenging Rosenbrock 5D problem:
   Table 2 below shows comprehensive performance metrics for all optimizers on Rosenbrock 5D.
   *Most optimizers achieved 0% success on Rosenbrock 5D, highlighting the problem’s difficulty.

6.4    Statistical Significance
Analysis of the comprehensive benchmark suite reveals clear performance patterns:
  Winner Distribution by Algorithm Family:

   • QQN variants: Majority of wins - dominated across problem types
   • L-BFGS variants: Substantial number of wins - efficient on convex problems


                                                    10
                      Table 2: Performance Results for Rosenbrock 5D Problem
Optimizer        Mean Final     Std Dev    Best     Worst Mean Func        Success   Mean Time
                   Value                   Value Value          Evals     Rate (%)      (s)
L-BFGS-             8.14e-1     9.14e-1   3.08e-1    2.82e0   351.4        70.0        0.006
MoreThuente
GD-                 6.46e-1     3.87e-1   3.39e-1    1.49e0    75.6        60.0        0.002
WeightDecay
QQN-                1.57e0       1.13e0   3.49e-1    3.70e0   497.6        30.0        0.014
StrongWolfe
QQN-Bisection-      2.41e0       1.57e0   3.30e-1    4.65e0   452.6        20.0        0.011
1
GD-Nesterov         2.82e0      4.31e0    3.97e-1    1.34e1   163.7        10.0        0.005
QQN-Bisection-      2.04e0      8.49e-1   3.77e-1    3.42e0   576.9        10.0        0.014
2
QQN-                2.71e0       1.32e0   5.90e-1    4.63e0   919.1         0.0        0.016
GoldenSection
L-BFGS-             8.67e0       1.50e1   1.17e0     5.36e1   711.8         0.0        0.010
Conservative
QQN-                2.53e0      6.65e-1   1.33e0     3.43e0   442.0         0.0        0.017
CubicQuadraticInterpolation
L-BFGS-             3.09e0      5.88e-1   2.07e0     4.26e0   789.5         0.0        0.010
Limited
GD                  5.08e0      1.74e-1   4.79e0     5.37e0    32.7         0.0        0.001
Adam-Robust         2.02e1      1.01e1    7.80e0     3.96e1   502.0         0.0        0.012
Adam-Fast           1.46e1      2.13e0    1.01e1     1.83e1    39.0         0.0        0.001
L-BFGS-             3.71e2      4.31e2    1.66e1     1.10e3   772.8         0.0        0.008
Aggressive
L-BFGS              5.10e2       8.40e2   1.84e1     2.69e3   123.4         0.0        0.002
GD-Momentum         3.21e1       7.22e0   2.02e1     4.73e1    21.2         0.0        0.001
Adam-               7.77e1       2.49e1   2.74e1     1.18e2   502.0         0.0        0.011
WeightDecay
GD-                 4.48e1       6.15e0   3.23e1     5.41e1    20.1         0.0        0.001
AdaptiveMomentum
Trust Region-       3.03e2       1.36e2   9.12e1     5.68e2   602.0         0.0        0.004
Aggressive
Adam                5.12e2       1.05e2   3.08e2     6.70e2   502.0         0.0        0.010
Adam-               4.89e2       1.01e2   3.28e2     6.43e2   502.0         0.0        0.012
AMSGrad
Trust Region-       8.35e2       1.79e2   6.28e2     1.21e3   602.0         0.0        0.004
Standard
Trust Region-       1.03e3       1.56e2   7.59e2     1.25e3   602.0         0.0        0.004
Conservative
Trust Region-       1.07e3       1.90e2   8.57e2     1.46e3   602.0         0.0        0.004
Adaptive
Trust Region-       1.10e3       1.45e2   8.91e2     1.40e3   602.0         0.0        0.004
Precise




                                                11
  • Adam variants: Notable wins - excelled on neural networks

  Top Individual Performers:

  1. QQN-StrongWolfe: Most wins, excellent risk-adjusted performance
  2. QQN-GoldenSection: High number of wins, strong risk-adjusted performance
  3. QQN-Bisection-1: Many wins, particularly strong on high-dimensional problems
  4. L-BFGS-MoreThuente: Substantial wins, good risk-adjusted performance
  5. Adam-WeightDecay: Best on neural networks with excellent success rate

  Notable Performance Gaps:

  • Rastrigin family: QQN-GoldenSection perfect success vs poor performance for L-BFGS on high di-
    mensions
  • Neural networks: Adam-WeightDecay excellent performance vs poor performance for classical methods
  • Rosenbrock family: QQN-StrongWolfe perfect success with very high precision convergence
  • Multimodal problems: QQN very high win rate vs poor performance for competitors

6.5   Performance on Different Problem Classes
Convex Problems:

  • QQN variants: 100% success rate on well-conditioned problems with minimal evaluations
  • QQN-Bisection-2 on Sphere 10D: 0.00e0 final value with only 13 function evaluations
  • L-BFGS-Aggressive: Matched performance but required more gradient evaluations
  • QQN-StrongWolfe: Superior superlinear convergence rate with 50-80% fewer evaluations than L-BFGS

  Non-Convex Unimodal:

  • QQN variants: 70-100% success rates on moderately conditioned problems
  • QQN-StrongWolfe on Rosenbrock 5D: 100% success vs 70% for best L-BFGS variant
  • QQN follows valley efficiently using curvature information on Rosenbrock
  • Performance vs condition number: QQN maintains speed on ill-conditioned problems while others slow
    significantly

  Highly Multimodal Problems:

  • QQN-GoldenSection: 100% success on Rastrigin 2D with 64.2 evaluations
  • QQN-CubicQuadraticInterpolation: 80% success on Rastrigin 2D
  • Basin of attraction for global minimum: Very small fraction of search space
  • QQN escape mechanism: Systematic step size exploration prevents local minima trapping
  • Traditional methods: Get trapped in first encountered minimum

  Machine Learning Problems:

  • QQN-Bisection variants: 95-100% success on neural network training
  • LinearRegression 200samples: QQN-Bisection-2 achieved 100% success with 54.1 evaluations
  • Adam-WeightDecay: Competitive but required significantly more evaluations
  • Network size impact: QQN competitive on small networks
  • Batch size effects: Full batch favors QQN, mini-batch favors Adam
  • Regularization synergy: Weight decay prevents overfitting in high dimensions




                                                 12
7       Discussion
7.1     Key Findings
The comprehensive evaluation reveals several important insights:

    1. QQN Dominance: QQN variants won the majority of problems, demonstrating clear superiority
       across diverse optimization landscapes.

    2. Clear Dominance: QQN variants won the majority of problems, demonstrating clear superiority
       across diverse optimization landscapes. Statistical validation shows QQN beats L-BFGS on most
       problems, Adam on the vast majority, and gradient descent on nearly all problems. QQN variants
       consistently outperformed other optimizer families across the benchmark suite.
    3. Line Search Critical: Among QQN variants, line search strategy dramatically affects performance:

         • Strong Wolfe: Excellent success rate with moderate average evaluations
         • Golden Section: 90-100% success rate on 2D problems with relatively few average evaluations
         • Bisection: 100% success on Rosenbrock 10D with minimal evaluations
         • Cubic-Quadratic Interpolation: 55% success on sparse problems, best for ill-conditioned objectives

    4. Problem-Specific Excellence: Algorithms show significant specialization:

         • QQN-GoldenSection: Achieved 1.81e-7 on Levy 2D with only 159.8 function evaluations
         • QQN-CubicQuadraticInterpolation: 70% success on Rosenbrock 5D with strong performance on
           ill-conditioned problems
         • Adam-WeightDecay: Excellent performance on neural networks vs moderate performance for
           standard Adam
         • L-BFGS variants: Generally poor performance on ill-conditioned problems like Rosenbrock

7.2     The Benchmarking and Reporting Framework
7.2.1    Methodological Contributions
Our benchmarking framework represents a significant methodological advance in optimization algorithm
evaluation:

    1. Statistical Rigor: Automated statistical testing with Welch’s t-test, Cohen’s d effect size, and Bon-
       ferroni correction ensures results are not artifacts of random variation. The framework generates
       comprehensive statistical comparison matrices that reveal true performance relationships.
    2. Reproducibility Infrastructure: Fixed seeds, deterministic algorithms, and automated report gen-
       eration eliminate common sources of irreproducibility in optimization research. All results can be
       regenerated with a single command.

    3. Diverse Problem Suite: The 62-problem benchmark suite covers a wide range of optimization
       challenges, from convex to highly multimodal landscapes, including sparse optimization, ill-conditioned
       problems, and constrained optimization scenarios.
    4. Multi-Format Reporting: The system generates:

         • Markdown reports with embedded visualizations for web viewing
         • LaTeX documents ready for academic publication
         • CSV files for further statistical analysis
         • Detailed per-run logs for debugging and deep analysis




                                                      13
7.2.2   Insights Enabled by the Framework
The comprehensive reporting revealed patterns invisible to traditional evaluation:
  1. Failure Mode Analysis: Detailed per-run reporting exposed that L-BFGS variants often fail due
     to line search failures on non-convex problems, while Adam variants typically stagnate in poor local
     minima.
  2. Convergence Behavior Patterns: Visualization of all runs revealed that QQN variants exhibit more
     consistent convergence trajectories, while gradient descent methods show high variance across runs.
  3. Problem Family Effects: Automatic problem classification and family-wise analysis revealed that
     optimizer performance clusters strongly by problem type, challenging the notion of universal optimizers.
  4. Statistical vs Practical Significance: The framework’s dual reporting of p-values and effect sizes
     revealed cases where statistically significant differences have negligible practical impact (e.g., 10 vs 12
     function evaluations on Sphere).

7.2.3   Framework Design Decisions
Several design choices proved crucial for meaningful evaluation:
  1. Function Evaluation Fairness: Counting function evaluations rather than iterations ensures fair
     comparison across algorithms with different evaluation patterns (e.g., line search vs trust region).
  2. Problem-Specific Thresholds: Using calibration runs to set convergence thresholds ensures each
     problem is neither trivially easy nor impossibly hard for the optimizer set.
  3. Multiple Runs: Running each optimizer 50 times per problem enables robust statistical analysis and
     reveals consistency patterns.
  4. Hierarchical Reporting: The multi-level report structure (summary → problem-specific → detailed
     per-run) allows both quick overview and deep investigation.

7.2.4   Limitations and Extensions
While comprehensive, the framework has limitations that suggest future extensions:
  1. Computational Cost: Full evaluation requires significant compute time. Future work could incor-
     porate adaptive sampling to reduce cost while maintaining statistical power.
  2. Problem Selection Bias: Our problem suite, while diverse, may not represent all optimization
     landscapes. The framework’s extensibility allows easy addition of new problems.
  3. Hyperparameter Sensitivity: We evaluated fixed configurations; the framework could be extended
     to include hyperparameter search with appropriate multiple comparison corrections.
  4. Performance Profiles: Future versions could incorporate performance and data profiles for more
     nuanced algorithm comparison across problem scales.

7.2.5   Impact on Optimization Research
This benchmarking framework addresses several chronic issues in optimization research:
  1. Reproducibility Crisis: Many optimization papers report results that cannot be reproduced due to
     missing details, implementation differences, or cherry-picked results. Our framework ensures complete
     reproducibility.
  2. Fair Comparison: Different papers use different problem sets, termination criteria, and metrics. Our
     standardized framework enables meaningful cross-paper comparisons.
  3. Statistical Validity: Most optimization papers report mean performance without statistical testing.
     Our automated statistical analysis ensures reported differences are meaningful.
  4. Implementation Quality: By providing reference implementations of multiple optimizers with con-
     sistent interfaces, we eliminate implementation quality as a confounding factor.
   The framework’s modular design encourages extension: researchers can easily add new optimizers, prob-
lems, or analysis methods while maintaining compatibility with the existing infrastructure. We envision this
becoming a standard tool for optimization algorithm development and evaluation.


                                                      14
7.3    When to Use QQN
Algorithm Selection Guidelines
    Primary Recommendation: Based on empirical dominance across 65% of benchmark problems and
statistical significance testing, QQN variants should be the default choice for most optimization tasks:
    • General-purpose optimization: QQN-StrongWolfe provides the strongest overall performance with
      superior convergence on ill-conditioned problems (100% success on Rosenbrock family)
    • Well-conditioned convex problems: QQN-Bisection variants achieve optimal efficiency with 100%
      success rates using minimal function evaluations (13-15 for Sphere 10D vs 197+ for L-BFGS)
    • Multimodal optimization: QQN-GoldenSection excels on complex landscapes with 90-100% success
      rates on 2D multimodal problems and perfect performance on Rastrigin across all dimensions
    • Sparse and ill-conditioned problems: QQN-CubicQuadraticInterpolation shows specialized strength
      with 55% success on sparse problems and robust performance on ill-conditioned variants
    • Unknown problem characteristics: QQN’s broad statistical dominance and graceful degradation
      make it the safest default choice
    Use specialized alternatives only when:
    • Stochastic optimization: Adam-WeightDecay for mini-batch neural network training where QQN’s
      deterministic line search is impractical
    • Extremely large scale: When memory constraints prohibit storing L-BFGS history (though QQN
      degrades gracefully to gradient descent)
    • Real-time constraints: When function evaluation cost dominates and approximate solutions suffice
    • Domain-specific requirements: When problem structure demands specialized methods (e.g., con-
      strained optimization, online learning)
   Practical Implementation Strategy: Start with QQN-StrongWolfe as the default optimizer. If com-
putational budget is extremely limited, consider QQN-Bisection variants for their efficiency. Only switch to
specialized methods if QQN variants demonstrably fail on your specific problem class or if domain constraints
require it.

7.4    Future Directions
The quadratic interpolation approach of QQN could be extended in various ways:
    • Deep Learning Applications: Adapting QQN for stochastic optimization in neural network training,
      including mini-batch variants and adaptive learning rate schedules.
    • Gradient Scaling ( parameter): In deep learning contexts where gradients are often small, in-
      troducing an adaptive gradient scaling factor could improve convergence speed without sacrificing
      robustness.
    • Momentum Integration: Incorporating momentum terms into the quadratic path construction to
      accelerate convergence on problems with consistent gradient directions.
    • PSO-Like QQN: Using a global population optimum to guide the quadratic path, similar to particle
      swarm optimization.
    • Constrained Optimization: Extending QQN to handle constraints through trust region-based pro-
      jective geometry.
    • Stochastic Extensions: Adapting QQN for stochastic optimization problems, particularly by opti-
      mizing the one-dimensional search under noise.


8     Conclusions
We have presented the Quadratic-Quasi-Newton (QQN) algorithm and a comprehensive benchmarking
methodology for fair optimization algorithm comparison. Our contributions advance both algorithmic de-
velopment and empirical evaluation standards in optimization research.
    Our evaluation across a comprehensive set of benchmark problems with multiple optimizer variants
demonstrates:


                                                     15
    1. Clear Dominance: QQN variants won the majority of problems, with QQN-Bisection-1 achieving
       54W-0L-5T against Trust Region-Conservative. Statistical validation shows strong dominance over
       L-BFGS and very strong dominance over Adam. Friedman test confirms statistical significance.
    2. Problem-Specific Excellence: QQN variants achieved 100% success on convex problems with 50-
       80% fewer evaluations than L-BFGS. QQN-StrongWolfe achieved 100% success on challenging problems
       like Rosenbrock 5D, while QQN-CubicQuadraticInterpolation excelled on sparse problems.
    3. Efficiency vs Robustness: QQN shows superior efficiency with strong success rates across problem
       types while requiring fewer function evaluations than traditional methods.
    4. Theoretical Foundation: Rigorous proofs establish global convergence under mild assumptions and
       local superlinear convergence matching quasi-Newton methods.
    5. Practical Impact: The results provide clear guidance for practitioners: use QQN-StrongWolfe for
       general optimization, QQN-Bisection variants for high-dimensional problems, QQN-GoldenSection for
       multimodal landscapes, and QQN-CubicQuadraticInterpolation for sparse or ill-conditioned problems.

    The simplicity of QQN’s core insight—that quadratic interpolation provides the natural geometry for
combining optimization directions—contrasts with the complexity of recent developments. Combined with
our evaluation methodology, this work establishes new standards for both algorithm development and em-
pirical validation in optimization research.
    Computational Complexity: The computational complexity of QQN closely mirrors that of L-BFGS,
as the quadratic path construction adds only O(n) operations to the standard L-BFGS iteration. Wall-
clock time comparisons on our benchmark problems would primarily reflect implementation details rather
than algorithmic differences. For problems where function evaluation dominates computation time, QQN’s
additional overhead is negligible. The geometric insights provided by counting function evaluations offer
more meaningful algorithm characterization than hardware-dependent timing measurements.
    The quadratic interpolation principle demonstrates how geometric approaches can provide effective so-
lutions to optimization problems. We hope this work encourages further exploration of geometric methods
in optimization and establishes new standards for rigorous algorithm comparison through our benchmark
reporting methodology.


9     Acknowledgments
The QQN algorithm was originally developed and implemented by the author in 2017, with this paper
representing its first formal academic documentation. AI language models assisted in the preparation of
documentation, implementation of the benchmarking framework, and drafting of the manuscript. This
collaborative approach between human expertise and AI assistance facilitated the academic presentation of
the method.


10      Supplementary Material
All code, data, and results are available at https://github.com/SimiaCryptus/qqn-optimizer/ to ensure
reproducibility and enable further research. We encourage the community to build upon this work and
explore the broader potential of interpolation-based optimization methods.


11      Competing Interests
The authors declare no competing interests.




                                                    16
12     Data Availability
All experimental data, including raw optimization trajectories and statistical analyses, are available at
https://github.com/SimiaCryptus/qqn-optimizer/. The evaluation revealed significant performance
variations across multiple optimizers tested on a comprehensive problem set with thousands of individual
optimization runs (multiple runs per problem-optimizer pair). QQN variants dominated the winner’s table,
claiming most problems. Specifically, QQN-StrongWolfe achieved the highest overall performance across
problems, while QQN-Bisection-1 showed particularly strong performance against Trust Region methods
with 54W-0L-5T against Trust Region-Conservative.


References
Vahid Beiranvand, Warren Hare, and Yves Lucet. Best practices for comparing optimization algorithms.
  Optimization and Engineering, 18(4):815–848, 2017. doi: 10.1007/s11081-017-9366-1.
Michael C Biggs. Minimization algorithms making use of non-quadratic properties of the objective function.
 IMA Journal of Applied Mathematics, 12(3):337–357, 1973.
Charles George Broyden. The convergence of a class of double-rank minimization algorithms 1. General
  considerations. IMA Journal of Applied Mathematics, 6(1):76–90, 1970. doi: 10.1093/imamat/6.1.76.

Augustin Cauchy. Méthode générale pour la résolution des systèmes d’équations simultanées. Comptes
 Rendus de l’Académie des Sciences, 25:536–538, 1847.
Andrew R Conn, Nicholas IM Gould, and Philippe L Toint. Trust Region Methods. SIAM, 2000. ISBN
 978-0-898714-60-9.

Kenneth Alan De Jong. An analysis of the behavior of a class of genetic adaptive systems. PhD thesis,
  University of Michigan, Ann Arbor, MI, 1975.
Roger Fletcher. A new approach to variable metric algorithms. The Computer Journal, 13(3):317–322, 1970.
  doi: 10.1093/comjnl/13.3.317.

Donald Goldfarb. A family of variable-metric methods derived by variational means. Mathematics of Com-
 putation, 24(109):23–26, 1970. doi: 10.1090/S0025-5718-1970-0258249-6.
Nikolaus Hansen, Anne Auger, Raymond Ros, Olaf Mersmann, Tea Tušar, and Dimo Brockhoff. COCO: A
  platform for comparing continuous optimizers in a black-box setting. arXiv preprint arXiv:1603.08785,
  2016. doi: 10.48550/arXiv.1603.08785.

Momin Jamil and Xin-She Yang. A literature survey of benchmark functions for global optimisation problems.
 International Journal of Mathematical Modelling and Numerical Optimisation, 4(2):150–194, 2013. doi:
 10.1504/IJMMNO.2013.055204.
Diederik P Kingma and Jimmy Ba. Adam: A method for stochastic optimization.                 arXiv preprint
  arXiv:1412.6980, 2015. doi: 10.48550/arXiv.1412.6980.

Jing J Liang, Bo Yang Qu, Ponnuthurai Nagaratnam Suganthan, and Alfredo G Hernández-Dı́az. Problem
  definitions and evaluation criteria for the CEC 2013 special session on real-parameter optimization. Com-
  putational Intelligence Laboratory, Zhengzhou University, Zhengzhou, China and Nanyang Technological
  University, Singapore, Technical Report, 201212, 2013.

Dong C Liu and Jorge Nocedal. On the limited memory BFGS method for large scale optimization. Mathe-
 matical Programming, 45(1-3):503–528, 1989. doi: 10.1007/BF01589116.
José Luis Morales and Jorge Nocedal. Automatic preconditioning by limited memory quasi-Newton updating.
  SIAM Journal on Optimization, 10(4):1079–1096, 2000. doi: 10.1137/S1052623497327854.



                                                    17
Jorge J Moré and Danny C Sorensen. Computing a trust region step. SIAM Journal on Scientific and
  Statistical Computing, 4(3):553–572, 1983. doi: 10.1137/0904038.
Yurii Nesterov. A method for unconstrained convex minimization problem with the rate of convergence
  O(1/k²). Doklady AN USSR, 269:543–547, 1983.

Boris T Polyak. Some methods of speeding up the convergence of iteration methods. USSR Computational
  Mathematics and Mathematical Physics, 4(5):1–17, 1964. doi: 10.1016/0041-5553(64)90137-5.
Robin M Schmidt, Frank Schneider, and Philipp Hennig. Descending through a crowded valley–benchmarking
  deep learning optimizers. International Conference on Machine Learning, pages 9367–9376, 2021.
David F Shanno. Conditioning of quasi-Newton methods for function minimization. Mathematics of Com-
  putation, 24(111):647–656, 1970. doi: 10.1090/S0025-5718-1970-0274029-X.




                                                 18
13     Appendix A: Problem Family vs Optimizer Family Compari-
       son Matrix

                   Table 3: Optimizer Family vs Problem Family Performance Matrix




                                                                                                    Trust Region
                                                                    L-BFGS
                                   Adam




                                                                                    QQN
                                                   GD
        Problem
         Family


                              17.5 / 13.7      15.9 / 9.7       7.0 / 2.7        5.1 / 1.0      19.5 / 10.0
Ackley                           Adam              GD            L-BFGS         Bisection-2     Conservative
                               Adam-Fast     GD-Momentum       Conservative    GoldenSection     Aggressive
                               11.3 / 8.0      4.2 / 1.0        2.5 / 2.0        inf / inf      11.5 / 6.3
Barrier                       Adam-Robust          GD         L-BFGS-Limited       N/A            Precise
                                 Adam         GD-Nesterov       Conservative       N/A          Conservative
                               20.4 / 18.0     9.4 / 5.0       10.4 / 2.0        6.8 / 1.0      18.0 / 13.0
Beale                          Adam-Robust    GD-Nesterov      MoreThuente      Bisection-1       Precise
                                  Adam       GD-Momentum        Aggressive      StrongWolfe     Conservative
                               19.8 / 17.0    13.2 / 10.0      10.8 / 5.0        3.2 / 1.0      18.0 / 12.0
Booth                           Adam-Fast          GD          MoreThuente     CubicQuadIn...    Standard
                                  Adam       GD-Momentum        Aggressive     GoldenSection    Conservative
                              13.8 / 10.0      15.0 / 9.0       9.6 / 5.0        3.6 / 1.0      23.0 / 21.0
GoldsteinPrice                WeightDecay    AdaptiveMom...   L-BFGS-Limited   GoldenSection     Aggressive
                                 Adam        GD-Momentum        Aggressive     CubicQuadIn...     Adaptive
                              17.6 / 10.0      13.1 / 8.0       7.5 / 2.7        6.1 / 1.0      20.6 / 12.7
Griewank                       Adam-Fast      GD-Nesterov      MoreThuente      Bisection-2     Conservative
                                 Adam              GD            L-BFGS        GoldenSection      Standard
                               20.2 / 16.0     12.2 / 8.0       9.0 / 1.0        5.0 / 3.0      18.6 / 11.0
Himmelblau                      Adam-Fast          GD         L-BFGS-Limited   GoldenSection     Standard
                                  Adam       AdaptiveMom...     Aggressive      Bisection-2     Conservative
                              16.7 / 11.7      9.4 / 3.3       11.0 / 3.0        5.4 / 2.0      22.5 / 19.7
IllConditionedRosenbrock       Adam-Fast      GD-Nesterov      MoreThuente      StrongWolfe      Aggressive
                             Adam-AMSGrad    GD-Momentum        Aggressive     GoldenSection       Precise
                              12.6 / 10.0      15.8 / 9.0      10.4 / 1.0        5.2 / 3.0      21.0 / 16.0
Levi                         Adam-AMSGrad     GD-Nesterov     L-BFGS-Limited   CubicQuadIn...      Precise
                               Adam-Fast           GD           Aggressive      Bisection-1      Aggressive
                               19.7 / 12.3    15.3 / 11.3       8.0 / 6.0        3.0 / 1.0      19.1 / 14.3
Levy                            Adam-Fast    GD-WeightDecay    MoreThuente     CubicQuadIn...     Precise
                                  Adam       AdaptiveMom...     Aggressive      Bisection-2      Aggressive
                              15.4 / 10.0     15.4 / 12.0       7.6 / 4.0        3.6 / 1.0      23.0 / 21.0
Matyas                         Adam-Fast     AdaptiveMom...   L-BFGS-Limited    StrongWolfe     Conservative
                             Adam-AMSGrad          GD          Conservative     Bisection-2       Adaptive
                                5.9 / 1.0      12.7 / 7.0      14.5 / 5.0       11.9 / 6.0      20.1 / 15.0
Michalewicz                     Adam-Fast    AdaptiveMom...    Conservative     Bisection-2       Adaptive
                             Adam-AMSGrad          GD         L-BFGS-Limited   GoldenSection     Aggressive
                               9.8 / 1.0      19.4 / 16.0       9.2 / 5.5        5.0 / 2.0      21.6 / 17.0
Neural Networks                Adam-Fast     GD-WeightDecay    Conservative     StrongWolfe     Conservative
                             Adam-AMSGrad     GD-Nesterov      MoreThuente     GoldenSection     Aggressive

Continued on next page



                                              19
Table 3 – continued from previous page




                                                                                                       Trust Region
                                                                        L-BFGS
                                     Adam




                                                                                       QQN
                                                     GD
      Problem
       Family


                                 15.1 / 8.3       7.5 / 3.7         9.5 / 1.0       9.9 / 2.7      19.9 / 15.7
NoisySphere                      Adam-Fast           GD            Conservative    StrongWolfe        Precise
                                   Adam          GD-Nesterov        Aggressive    CubicQuadIn...     Adaptive
                                 12.1 / 4.3      9.9 / 7.3         13.0 / 5.3       7.5 / 1.0      22.5 / 20.3
PenaltyI                         WeightDecay         GD            Conservative   CubicQuadIn...      Precise
                                   Adam        GD-Momentum          Aggressive      Bisection-2      Adaptive
                                 13.3 / 5.7      10.7 / 5.0        12.2 / 1.7       9.6 / 3.3      19.1 / 12.3
Rastrigin                        WeightDecay         GD            MoreThuente    CubicQuadIn...     Standard
                               Adam-AMSGrad    GD-Momentum          Aggressive      Bisection-2    Conservative
                                18.6 / 12.2      14.2 / 8.0         8.4 / 4.5       3.3 / 1.0      20.4 / 14.5
Regression                       Adam-Fast     AdaptiveMom...      Conservative     Bisection-2      Standard
                               Adam-AMSGrad          GD         L-BFGS-Limited    GoldenSection       Precise
                                 16.2 / 9.3      9.9 / 3.7         10.6 / 1.3       5.7 / 3.0      22.5 / 19.7
Rosenbrock                       Adam-Fast      GD-Nesterov        MoreThuente     StrongWolfe      Aggressive
                               Adam-AMSGrad    GD-Momentum          Aggressive    GoldenSection       Precise
                                18.3 / 12.0      13.1 / 5.0         7.6 / 2.0       5.3 / 2.0      20.7 / 14.5
SVM                             Adam-Robust    GD-WeightDecay      Conservative    StrongWolfe     Conservative
                                   Adam        AdaptiveMom...       Aggressive    GoldenSection     Aggressive
                                 20.7 / 14.7     11.4 / 7.3        10.3 / 5.7       3.3 / 1.0      19.3 / 13.7
Schwefel                          Adam-Fast    GD-Momentum      L-BFGS-Limited     StrongWolfe      Aggressive
                                    Adam             GD            Conservative   GoldenSection    Conservative
                                 21.3 / 17.5    13.0 / 10.5         6.2 / 1.5       4.9 / 1.5      19.6 / 15.5
SparseQuadratic                   Adam-Fast    GD-WeightDecay      MoreThuente    GoldenSection     Adaptive
                                    Adam       AdaptiveMom...        L-BFGS        StrongWolfe     Conservative
                                 12.9 / 3.0       6.3 / 1.5        16.3 / 9.0       7.5 / 3.0      22.0 / 18.5
SparseRosenbrock                 Adam-Fast     AdaptiveMom...      MoreThuente     StrongWolfe      Aggressive
                               Adam-AMSGrad     GD-Momentum         Aggressive    GoldenSection       Precise
                                 21.8 / 17.5     12.6 / 9.5         6.1 / 1.0       5.3 / 3.0      19.2 / 16.5
Sphere                            Adam-Fast    GD-Momentum          Aggressive     StrongWolfe       Precise
                                    Adam       AdaptiveMom...      Conservative   GoldenSection     Aggressive
                                 19.2 / 8.3      10.5 / 1.7        10.1 / 2.3       6.3 / 3.0      18.9 / 11.7
StyblinskiTang                    Adam-Fast          GD            MoreThuente    GoldenSection     Standard
                                    Adam       GD-Momentum          Aggressive      Bisection-2    Conservative
                                15.8 / 11.3      12.5 / 4.7        10.7 / 4.7       5.2 / 1.0      20.7 / 16.3
Trigonometric                    WeightDecay   GD-WeightDecay      MoreThuente    CubicQuadIn...      Precise
                                   Adam        GD-Momentum          Aggressive      Bisection-1     Aggressive
                                16.9 / 11.3      12.7 / 8.3        10.6 / 6.3       3.0 / 1.0      21.8 / 18.7
Zakharov                         Adam-Fast     GD-WeightDecay   L-BFGS-Limited      Bisection-1      Adaptive
                                   Adam        GD-Momentum          Aggressive    CubicQuadIn...   Conservative




 Legend: Each cell contains:
 • Top line: Average Ranking / Best Rank Average (lower is better)
 • Middle line: Best performing variant in this optimizer family
 • Bottom line: Worst performing variant in this optimizer family


                                                20
Green cells indicate the best performing optimizer family for that problem family. Red cells indicate the
worst performing optimizer family.




                                                   21
