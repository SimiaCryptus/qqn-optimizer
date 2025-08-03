# Detailed Analysis: QQN-Bisection-1 on SparseRosenbrock_4D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_4D
**Optimizer:** QQN-Bisection-1
**Problem Family:** SparseRosenbrock
**Dimension:** 4
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 3 (15.0%)

### Quick Statistics
* **Best Final Value:** 9.121408e-8
* **Worst Final Value:** 1.601001e0
* **Mean Final Value:** 4.561571e-1
* **Success Rate:** 15.0%


## Run-by-Run Analysis
<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 12px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Run</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Final Value</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Iterations</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Evals</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Evals</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Time (s)</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Converged</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Reason</th>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.813e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.474e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.187e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.811e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1619</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.563e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.208e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2606</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.224e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.210e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.149e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.928e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2590</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.601e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.840e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2651</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.730e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.588e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.811e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.512e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2425</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.861e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.474e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.121e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.303e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1505</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.873e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.244e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2690</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.426e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.212e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.011e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.379e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.140e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.709e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.500e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.837e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.021e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.803e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2420</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.110e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.221e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2580</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.453e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.328e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.199e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.106e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.401e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.971e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2717</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (3 out of 20)

* **Average Iterations to Convergence:** 75.0
* **Average Function Evaluations:** 1654.0
* **Average Time to Convergence:** 0.045s
* **Fastest Convergence:** 63 iterations (0.037s)
* **Slowest Convergence:** 83 iterations (0.053s)

### Failed Runs (17 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 17 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 9.121408e-8
**Final Gradient Norm:** 7.302663e-3
**Iterations:** 63
**Convergence Reason:** FunctionTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.835e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.570e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.033e0, 1.064e0, -1.121e0, 9.446e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.835e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.570e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.719e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.028e0, 1.065e0, -9.953e-1, 9.989e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.107e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.562e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.555e-1, 9.768e-1, -9.527e-1, 8.950e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.059e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.576e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.525e-1, 9.505e-1, -9.384e-1, 8.746e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">7.761e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.651e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.338e-1, 6.677e-1, -7.905e-1, 6.068e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">59</td><td style="border: 1px solid #ddd; padding: 4px;">6.052e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.741e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.006e0, 1.012e0, 1.005e0, 1.009e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">60</td><td style="border: 1px solid #ddd; padding: 4px;">1.048e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.227e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.441e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.006e0, 1.012e0, 1.005e0, 1.009e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">61</td><td style="border: 1px solid #ddd; padding: 4px;">8.251e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.432e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.003e0, 1.001e0, 1.002e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">62</td><td style="border: 1px solid #ddd; padding: 4px;">4.101e-6</td><td style="border: 1px solid #ddd; padding: 4px;">4.649e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">63</td><td style="border: 1px solid #ddd; padding: 4px;">9.121e-8</td><td style="border: 1px solid #ddd; padding: 4px;">7.303e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2290.2
- **Average Gradient Evaluations per Run:** 2516.9
- **Average Iterations per Run:** 79.1
- **Average Time per Run:** 0.056s
- **Function Evaluations per Second:** 40927.0
- **Iterations per Second:** 1413.5
### Resource Utilization
- **Total Function Evaluations:** 45805
- **Total Gradient Evaluations:** 50338
- **Total Computation Time:** 1.1s
- **Function/Gradient Ratio:** 0.91
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/SparseRosenbrock_4D_results.csv)
* [Convergence Plot](../plots/SparseRosenbrock_4D.png)
* [Log Scale Convergence Plot](../plots/SparseRosenbrock_4D_log.png)


---
*Detailed report for QQN-Bisection-1 on SparseRosenbrock_4D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
