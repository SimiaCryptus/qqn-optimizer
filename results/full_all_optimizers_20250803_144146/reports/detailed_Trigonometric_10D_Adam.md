# Detailed Analysis: Adam on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** Adam
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 7 (35.0%)

### Quick Statistics
* **Best Final Value:** 9.679432e-7
* **Worst Final Value:** 7.196430e-1
* **Mean Final Value:** 9.699890e-2
* **Success Rate:** 35.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.356e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.786e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.770e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.848e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">980</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.007e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.482e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.679e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.964e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.937e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.890e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.910e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.083e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.813e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.885e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.196e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.014e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">838</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1679</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1678</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.420e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.390e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.488e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.351e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">544</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.978e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.975e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">995</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.803e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.648e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.163e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.005e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.066</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.553e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.293e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.739e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.305e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.867e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.885e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">825</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1653</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1653</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.909e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.851e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.892e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.922e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">829</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1661</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1661</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.196e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.164e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.668e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.612e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (7 out of 20)

* **Average Iterations to Convergence:** 898.1
* **Average Function Evaluations:** 1799.3
* **Average Time to Convergence:** 0.047s
* **Fastest Convergence:** 646 iterations (0.033s)
* **Slowest Convergence:** 1009 iterations (0.053s)

### Failed Runs (13 out of 20)

**Failure Reasons:**
- FunctionTolerance: 3 runs
- MaxFunctionEvaluations: 10 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 9.679432e-7
**Final Gradient Norm:** 1.963805e-3
**Iterations:** 646
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.223e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.567e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.552e-2, 6.610e-2, 5.454e-2, 2.876e-1, 1.032e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.223e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.567e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.652e-2, 6.710e-2, 5.554e-2, 2.866e-1, 1.022e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.166e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.560e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.752e-2, 6.810e-2, 5.654e-2, 2.856e-1, 1.012e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.109e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.553e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.852e-2, 6.910e-2, 5.754e-2, 2.846e-1, 1.002e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.051e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.546e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.952e-2, 7.010e-2, 5.854e-2, 2.836e-1, 9.920e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">642</td><td style="border: 1px solid #ddd; padding: 4px;">1.226e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.209e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.076e-3, 3.419e-7, 1.950e-7, 1.508e-7, 1.166e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">643</td><td style="border: 1px solid #ddd; padding: 4px;">1.156e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.145e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.045e-3, 3.222e-7, 1.838e-7, 1.421e-7, 1.100e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">644</td><td style="border: 1px solid #ddd; padding: 4px;">1.089e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.083e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.014e-3, 3.036e-7, 1.733e-7, 1.340e-7, 1.036e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">645</td><td style="border: 1px solid #ddd; padding: 4px;">1.027e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.023e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.848e-4, 2.860e-7, 1.633e-7, 1.262e-7, 9.767e-8, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">646</td><td style="border: 1px solid #ddd; padding: 4px;">9.679e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.964e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.848e-4, 2.860e-7, 1.633e-7, 1.262e-7, 9.767e-8, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2080.5
- **Average Gradient Evaluations per Run:** 2080.3
- **Average Iterations per Run:** 1038.5
- **Average Time per Run:** 0.054s
- **Function Evaluations per Second:** 38509.9
- **Iterations per Second:** 19222.5
### Resource Utilization
- **Total Function Evaluations:** 41610
- **Total Gradient Evaluations:** 41607
- **Total Computation Time:** 1.1s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Trigonometric_10D_results.csv)
* [Convergence Plot](../plots/Trigonometric_10D.png)
* [Log Scale Convergence Plot](../plots/Trigonometric_10D_log.png)


---
*Detailed report for Adam on Trigonometric_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
