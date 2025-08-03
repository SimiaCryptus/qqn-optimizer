# Detailed Analysis: QQN-StrongWolfe on Michalewicz_5D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_5D_m10
**Optimizer:** QQN-StrongWolfe
**Problem Family:** Michalewicz
**Dimension:** 5
**Success Threshold:** -2.690e0
**Total Runs:** 20
**Successful Runs:** 5 (25.0%)

### Quick Statistics
* **Best Final Value:** -2.794134e0
* **Worst Final Value:** -5.688430e-5
* **Mean Final Value:** -1.762116e0
* **Success Rate:** 25.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.898e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.941e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.688e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.056e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2656</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.693e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.933e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">979</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">776</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.764e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">256</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.794e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.627e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">588</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.735e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.846e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.886e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.144e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">577</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">425</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.388e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.993e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.839e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.873e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2256</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.377e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">611</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.886e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.589e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2218</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.388e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.613e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2729</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.968e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.194e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.736e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.664e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.981e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.735e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.692e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.690e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.244e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1403</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.798e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.296e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2742</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2304</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.692e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.719e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">865</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.967e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.545e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (5 out of 20)

* **Average Iterations to Convergence:** 43.4
* **Average Function Evaluations:** 784.6
* **Average Time to Convergence:** 0.025s
* **Fastest Convergence:** 25 iterations (0.010s)
* **Slowest Convergence:** 69 iterations (0.044s)

### Failed Runs (15 out of 20)

**Failure Reasons:**
- GradientTolerance: 1 runs
- FunctionTolerance: 8 runs
- MaxFunctionEvaluations: 6 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** -2.794134e0
**Final Gradient Norm:** 9.626738e-1
**Iterations:** 39
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.156e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.926e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 9.499e-1, 8.852e-1, 6.033e-1, 8.481e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.156e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.926e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.088e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 9.499e-1, 8.859e-1, 6.033e-1, 9.970e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-8.392e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.160e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 9.499e-1, 8.860e-1, 6.033e-1, 9.966e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-8.392e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.133e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 9.499e-1, 8.861e-1, 6.033e-1, 9.967e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-8.392e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.389e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 9.499e-1, 8.863e-1, 6.033e-1, 9.967e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">35</td><td style="border: 1px solid #ddd; padding: 4px;">-1.809e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.783e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 1.216e0, 1.285e0, 6.078e-1, 9.978e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">36</td><td style="border: 1px solid #ddd; padding: 4px;">-1.811e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.599e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 1.308e0, 1.286e0, 6.077e-1, 9.990e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">37</td><td style="border: 1px solid #ddd; padding: 4px;">-1.884e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.589e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 1.531e0, 1.272e0, 6.077e-1, 9.220e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">38</td><td style="border: 1px solid #ddd; padding: 4px;">-2.384e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.528e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.172e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 1.565e0, 1.289e0, 6.077e-1, 1.001e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">39</td><td style="border: 1px solid #ddd; padding: 4px;">-2.794e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.627e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.890e-1, 1.565e0, 1.289e0, 6.077e-1, 1.001e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1288.2
- **Average Gradient Evaluations per Run:** 1078.4
- **Average Iterations per Run:** 59.6
- **Average Time per Run:** 0.040s
- **Function Evaluations per Second:** 32106.1
- **Iterations per Second:** 1486.7
### Resource Utilization
- **Total Function Evaluations:** 25764
- **Total Gradient Evaluations:** 21568
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 1.19
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Michalewicz_5D_m10_results.csv)
* [Convergence Plot](../plots/Michalewicz_5D_m10.png)
* [Log Scale Convergence Plot](../plots/Michalewicz_5D_m10_log.png)


---
*Detailed report for QQN-StrongWolfe on Michalewicz_5D_m10*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
