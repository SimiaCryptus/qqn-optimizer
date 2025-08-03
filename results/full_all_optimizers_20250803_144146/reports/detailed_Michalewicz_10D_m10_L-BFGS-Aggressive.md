# Detailed Analysis: L-BFGS-Aggressive on Michalewicz_10D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_10D_m10
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** Michalewicz
**Dimension:** 10
**Success Threshold:** -6.260e0
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -3.262955e0
* **Worst Final Value:** 3.391619e-1
* **Mean Final Value:** -1.300776e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.730e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.588e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.284e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.028e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.313e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.614e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">336</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.689e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.356e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.688e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.047e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.392e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.436e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.551e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.154e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.872e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.543e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.035e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.741e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.839e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.386e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.265e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.708e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.345e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.030e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.263e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.211e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.627e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.183e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.115e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.243e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.917e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.475e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3847</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.484e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.330e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.519e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.686e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.013e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.126e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.071e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.622e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 14 runs
- FunctionTolerance: 6 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 13)
**Final Value:** -3.262955e0
**Final Gradient Norm:** 1.211378e1
**Iterations:** 384
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-3.262e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.212e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.501e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-3.262e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.212e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.501e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-3.262e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.212e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.501e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-3.262e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.212e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.501e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-3.262e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.212e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.501e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">380</td><td style="border: 1px solid #ddd; padding: 4px;">-3.263e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.211e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.502e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">381</td><td style="border: 1px solid #ddd; padding: 4px;">-3.263e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.211e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.502e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">382</td><td style="border: 1px solid #ddd; padding: 4px;">-3.263e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.211e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.502e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">383</td><td style="border: 1px solid #ddd; padding: 4px;">-3.263e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.211e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.502e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">384</td><td style="border: 1px solid #ddd; padding: 4px;">-3.263e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.211e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.983e-1, 8.446e-1, 8.461e-1, 8.094e-1, 9.502e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3050.9
- **Average Gradient Evaluations per Run:** 918.2
- **Average Iterations per Run:** 304.8
- **Average Time per Run:** 0.021s
- **Function Evaluations per Second:** 143166.3
- **Iterations per Second:** 14300.4
### Resource Utilization
- **Total Function Evaluations:** 61019
- **Total Gradient Evaluations:** 18365
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 3.32
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Michalewicz_10D_m10_results.csv)
* [Convergence Plot](../plots/Michalewicz_10D_m10.png)
* [Log Scale Convergence Plot](../plots/Michalewicz_10D_m10_log.png)


---
*Detailed report for L-BFGS-Aggressive on Michalewicz_10D_m10*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
