# Detailed Analysis: GD-Nesterov on IllConditionedRosenbrock_10D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_10D_alpha100
**Optimizer:** GD-Nesterov
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.122605e0
* **Worst Final Value:** 1.188096e0
* **Mean Final Value:** 1.169065e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.178e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.391e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1525</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.188e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.419e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.186e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.413e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.141e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.291e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1505</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.154e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.326e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1509</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.169e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.367e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.173e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1517</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1520</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.160e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.343e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1508</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1511</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.123e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.239e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1504</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.187e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.417e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.167e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.362e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.166e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.358e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.168e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.365e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1508</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1511</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.186e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.412e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1511</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1514</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.175e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.383e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1512</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1515</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.170e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.369e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.180e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.398e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1520</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.178e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.391e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.148e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.310e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.185e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.411e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1514</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1517</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 9)
**Final Value:** 1.122605e0
**Final Gradient Norm:** 6.238673e1
**Iterations:** 1504
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.067e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.108e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.295e0, 9.417e-1, -1.346e0, 8.879e-1, -1.079e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.067e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.108e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.286e0, 9.253e-1, -1.323e0, 8.766e-1, -1.067e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.964e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.017e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.260e0, 8.808e-1, -1.262e0, 8.453e-1, -1.034e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.707e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.784e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.227e0, 8.248e-1, -1.186e0, 8.042e-1, -9.904e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.423e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.521e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.185e0, 7.591e-1, -1.095e0, 7.529e-1, -9.367e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1499</td><td style="border: 1px solid #ddd; padding: 4px;">1.257e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.603e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.003e0, 9.890e-1, 1.012e0, 9.803e-1, 1.011e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1500</td><td style="border: 1px solid #ddd; padding: 4px;">1.123e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.239e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.949e-1, 1.009e0, 9.831e-1, 1.014e0, 9.772e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1501</td><td style="border: 1px solid #ddd; padding: 4px;">1.257e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.603e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.003e0, 9.890e-1, 1.012e0, 9.803e-1, 1.011e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1502</td><td style="border: 1px solid #ddd; padding: 4px;">1.123e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.239e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.949e-1, 1.009e0, 9.831e-1, 1.014e0, 9.772e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1503</td><td style="border: 1px solid #ddd; padding: 4px;">1.257e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.603e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.003e0, 9.890e-1, 1.012e0, 9.803e-1, 1.011e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1514.9
- **Average Gradient Evaluations per Run:** 3025.8
- **Average Iterations per Run:** 1511.9
- **Average Time per Run:** 0.051s
- **Function Evaluations per Second:** 29674.2
- **Iterations per Second:** 29615.5
### Resource Utilization
- **Total Function Evaluations:** 30298
- **Total Gradient Evaluations:** 60516
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/IllConditionedRosenbrock_10D_alpha100_results.csv)
* [Convergence Plot](../plots/IllConditionedRosenbrock_10D_alpha100.png)
* [Log Scale Convergence Plot](../plots/IllConditionedRosenbrock_10D_alpha100_log.png)


---
*Detailed report for GD-Nesterov on IllConditionedRosenbrock_10D_alpha100*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
