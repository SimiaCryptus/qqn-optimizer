# Detailed Analysis: Trust Region-Standard on StyblinskiTang_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** StyblinskiTang_10D
**Optimizer:** Trust Region-Standard
**Problem Family:** StyblinskiTang
**Dimension:** 10
**Success Threshold:** -3.780e2
**Total Runs:** 20
**Successful Runs:** 8 (40.0%)

### Quick Statistics
* **Best Final Value:** -3.788135e2
* **Worst Final Value:** -2.995610e2
* **Mean Final Value:** -3.677335e2
* **Success Rate:** 40.0%


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
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.785e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">751</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.507e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.645e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">768</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.380e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.980e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">680</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.788e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.970e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.786e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.988e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">759</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.788e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.292e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">759</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.772e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.991e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">377</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.774e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.066e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">758</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.781e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.182e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.996e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.278e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">742</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.633e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.646e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1097</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">732</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.782e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.343e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.492e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.139e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">704</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.773e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.995e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">377</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.782e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.506e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.782e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.352e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">377</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.127e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">760</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.766e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.532e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.772e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.017e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">377</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.631e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.249e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1094</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">730</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (8 out of 20)

* **Average Iterations to Convergence:** 376.4
* **Average Function Evaluations:** 1132.1
* **Average Time to Convergence:** 0.008s
* **Fastest Convergence:** 376 iterations (0.008s)
* **Slowest Convergence:** 378 iterations (0.008s)

### Failed Runs (12 out of 20)

**Failure Reasons:**
- GradientTolerance: 10 runs
- FunctionTolerance: 2 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** -3.788135e2
**Final Gradient Norm:** 1.970472e1
**Iterations:** 376
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.551e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.200e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.802e-1, -4.471e-2, -7.066e-2, 1.174e-1, -9.888e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.551e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.200e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.334e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.175e-1, -6.704e-2, -9.587e-2, 1.131e-1, -1.272e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-3.607e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.332e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.505e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.510e-1, -8.716e-2, -1.186e-1, 1.092e-1, -1.528e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-4.651e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.451e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.890e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.818e-1, -1.056e-1, -1.394e-1, 1.056e-1, -1.762e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-5.689e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.560e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.409e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.104e-1, -1.228e-1, -1.589e-1, 1.022e-1, -1.980e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">372</td><td style="border: 1px solid #ddd; padding: 4px;">-3.748e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.058e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.860e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.882e0, -2.838e0, -2.853e0, -1.725e0, -2.864e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">373</td><td style="border: 1px solid #ddd; padding: 4px;">-3.758e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.041e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.898e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.884e0, -2.843e0, -2.857e0, -1.772e0, -2.867e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">374</td><td style="border: 1px solid #ddd; padding: 4px;">-3.768e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.022e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.946e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.885e0, -2.848e0, -2.861e0, -1.821e0, -2.870e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">375</td><td style="border: 1px solid #ddd; padding: 4px;">-3.778e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.998e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.004e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.887e0, -2.853e0, -2.864e0, -1.870e0, -2.873e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">376</td><td style="border: 1px solid #ddd; padding: 4px;">-3.788e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.970e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.887e0, -2.853e0, -2.864e0, -1.870e0, -2.873e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1120.5
- **Average Gradient Evaluations per Run:** 747.7
- **Average Iterations per Run:** 372.6
- **Average Time per Run:** 0.008s
- **Function Evaluations per Second:** 144889.6
- **Iterations per Second:** 48188.8
### Resource Utilization
- **Total Function Evaluations:** 22409
- **Total Gradient Evaluations:** 14954
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/StyblinskiTang_10D_results.csv)
* [Convergence Plot](../plots/StyblinskiTang_10D.png)
* [Log Scale Convergence Plot](../plots/StyblinskiTang_10D_log.png)


---
*Detailed report for Trust Region-Standard on StyblinskiTang_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
