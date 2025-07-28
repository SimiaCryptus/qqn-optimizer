# Detailed Analysis: QQN-MoreThuente on Zakharov_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_5D
**Optimizer:** QQN-MoreThuente
**Problem Family:** Zakharov
**Dimension:** 5
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 3 (15.0%)

### Quick Statistics
* **Best Final Value:** 4.183357e-21
* **Worst Final Value:** 1.630370e0
* **Mean Final Value:** 1.093643e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.420e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.067e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.161e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.884e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.202e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.911e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.942e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.815e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.376e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.962e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.291e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.967e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.511e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.799e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.584e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.125e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.606e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.038e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.630e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.065e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.839e-16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.259e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.132e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.901e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.410e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.411e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.364e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.035e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.029e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.731e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.025e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.850e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.337e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.930e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.022e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.853e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.183e-21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.187e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.918e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (3 out of 20)
- **Average Iterations to Convergence:** 13.3
- **Average Function Evaluations:** 188.7
- **Average Time to Convergence:** 0.004s
- **Fastest Convergence:** 13 iterations (0.004s)
- **Slowest Convergence:** 14 iterations (0.004s)
### Failed Runs (17 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 17 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 19)
**Final Value:** 4.183357e-21
**Final Gradient Norm:** 1.293800e-10
**Iterations:** 13
**Convergence Reason:** GradientTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.376e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.019e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.090e0, 9.082e-1, 8.676e-1, 8.677e-1, 9.811e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.376e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.019e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.965e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[8.213e-1, 3.712e-1, 6.248e-2, -2.056e-1, -3.603e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.215e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.101e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.436e-9</td><td style="border: 1px solid #ddd; padding: 4px;">[8.213e-1, 3.712e-1, 6.248e-2, -2.056e-1, -3.603e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.215e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.101e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.913e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[8.213e-1, 3.712e-1, 6.252e-2, -2.055e-1, -3.603e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.215e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.096e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.702e-1, 3.650e-1, 9.050e-2, -1.464e-1, -2.785e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">8</td><td style="border: 1px solid #ddd; padding: 4px;">5.384e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.871e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.750e-4, -1.413e-4, -5.312e-5, 2.202e-5, 6.075e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">1.290e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.441e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.640e-7, 1.951e-7, -6.984e-8, -3.050e-7, -4.570e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">2.568e-12</td><td style="border: 1px solid #ddd; padding: 4px;">1.108e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.444e-7, 5.061e-8, -1.647e-8, -7.601e-8, -1.143e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">11</td><td style="border: 1px solid #ddd; padding: 4px;">1.579e-13</td><td style="border: 1px solid #ddd; padding: 4px;">2.728e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.282e-11, -5.209e-11, -6.287e-12, -7.669e-13, 1.878e-11]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">12</td><td style="border: 1px solid #ddd; padding: 4px;">4.183e-21</td><td style="border: 1px solid #ddd; padding: 4px;">1.294e-10</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.282e-11, -5.209e-11, -6.287e-12, -7.669e-13, 1.878e-11]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 440.6
- **Average Gradient Evaluations per Run:** 479.6
- **Average Iterations per Run:** 42.0
- **Average Time per Run:** 0.008s
- **Function Evaluations per Second:** 52917.5
- **Iterations per Second:** 5038.9
### Resource Utilization
- **Total Function Evaluations:** 8811
- **Total Gradient Evaluations:** 9591
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 0.92
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Zakharov_5D_results.csv)
* [Convergence Plot](convergence_Zakharov_5D.png)
* [Log Scale Convergence Plot](convergence_Zakharov_5D_log.png)


---
*Detailed report for QQN-MoreThuente on Zakharov_5D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
