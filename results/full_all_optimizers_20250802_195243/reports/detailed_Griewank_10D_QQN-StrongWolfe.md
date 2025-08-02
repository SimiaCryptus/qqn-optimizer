# Detailed Analysis: QQN-StrongWolfe on Griewank_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Griewank_10D
**Optimizer:** QQN-StrongWolfe
**Problem Family:** Griewank
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 9.857285e-3
* **Worst Final Value:** 2.596183e1
* **Mean Final Value:** 6.059062e0
* **Success Rate:** 0.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.706e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.771e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.098</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.596e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.494e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2932</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.857e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.756e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.832e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.372e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.857e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.850e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">322</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.371e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.342e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2581</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.353e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.652e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">879</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">834</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.477e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.646e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2739</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.096</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.646e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.278e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">319</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2342</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.591e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.518e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2966</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.458e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.874e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2727</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.624e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.193e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">912</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.596e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.496e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.928e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.566e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2463</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.844e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.103e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">323</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.060e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.584e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">318</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2606</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.059e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.777e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">469</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">437</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.478e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.540e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.161e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.061e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">279</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2422</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.594e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.503e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2636</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1765</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 14 runs
- FunctionTolerance: 3 runs
- NumericalError: 3 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 9.857285e-3
**Final Gradient Norm:** 7.850232e-6
**Iterations:** 322
**Convergence Reason:** MaxFunctionEvaluations

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.600e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.536e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e2, 1.002e2, 1.001e2, 9.982e1, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.600e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.536e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.996e1, 1.001e2, 1.000e2, 9.977e1, 1.000e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.598e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.524e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.991e1, 1.001e2, 9.998e1, 9.972e1, 9.996e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.596e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.514e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.751e1, 1.038e2, 8.877e1, 8.548e1, 8.549e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.039e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.390e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.625e1, 6.581e1, 4.534e1, -2.486e1, -2.087e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">318</td><td style="border: 1px solid #ddd; padding: 4px;">9.857e-3</td><td style="border: 1px solid #ddd; padding: 4px;">8.947e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.140e0, 5.112e-8, -5.433e0, -5.605e-8, -3.834e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">319</td><td style="border: 1px solid #ddd; padding: 4px;">9.857e-3</td><td style="border: 1px solid #ddd; padding: 4px;">8.659e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.140e0, -3.786e-8, -5.433e0, -5.343e-8, -3.667e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">320</td><td style="border: 1px solid #ddd; padding: 4px;">9.857e-3</td><td style="border: 1px solid #ddd; padding: 4px;">8.381e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.140e0, 4.716e-8, -5.433e0, -5.094e-8, -3.508e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">321</td><td style="border: 1px solid #ddd; padding: 4px;">9.857e-3</td><td style="border: 1px solid #ddd; padding: 4px;">8.111e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.140e0, -3.593e-8, -5.433e0, -4.856e-8, -3.355e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">322</td><td style="border: 1px solid #ddd; padding: 4px;">9.857e-3</td><td style="border: 1px solid #ddd; padding: 4px;">7.850e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.140e0, -3.593e-8, -5.433e0, -4.856e-8, -3.355e-7, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2281.2
- **Average Gradient Evaluations per Run:** 2144.8
- **Average Iterations per Run:** 201.1
- **Average Time per Run:** 0.091s
- **Function Evaluations per Second:** 25155.5
- **Iterations per Second:** 2217.0
### Resource Utilization
- **Total Function Evaluations:** 45624
- **Total Gradient Evaluations:** 42897
- **Total Computation Time:** 1.8s
- **Function/Gradient Ratio:** 1.06
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 3
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Griewank_10D_results.csv)
* [Convergence Plot](../plots/Griewank_10D.png)
* [Log Scale Convergence Plot](../plots/Griewank_10D_log.png)


---
*Detailed report for QQN-StrongWolfe on Griewank_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
