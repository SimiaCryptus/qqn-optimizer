# Detailed Analysis: Adam-WeightDecay on Griewank_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Griewank_5D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Griewank
**Dimension:** 5
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.227253e1
* **Worst Final Value:** 1.229913e1
* **Mean Final Value:** 1.228347e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.535e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">804</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1611</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.572e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">805</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1613</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1612</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.746e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">869</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1741</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1740</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.564e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">815</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1633</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1632</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.681e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">840</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1683</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1682</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.607e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">804</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1611</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.657e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">796</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.589e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">796</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.554e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1573</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.691e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1719</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1718</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.704e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1704</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.674e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">835</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1673</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.692e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">842</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1687</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1686</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.227e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.485e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">799</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1601</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1600</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.674e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1691</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1690</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.534e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">780</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1562</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.725e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">848</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.625e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1693</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1692</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.517e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">794</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1591</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1590</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.230e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.811e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 1.227253e1
**Final Gradient Norm:** 1.485087e-1
**Iterations:** 799
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.349e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.095e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.991e1, 9.988e1, 1.001e2, 1.000e2, 1.002e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.349e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.095e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.991e1, 9.988e1, 1.001e2, 1.000e2, 1.002e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.349e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.100e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.990e1, 9.988e1, 1.001e2, 1.000e2, 1.001e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.349e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.106e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.990e1, 9.988e1, 1.001e2, 1.000e2, 1.001e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.349e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.111e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.990e1, 9.987e1, 1.001e2, 1.000e2, 1.001e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">794</td><td style="border: 1px solid #ddd; padding: 4px;">1.227e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.004e2, 9.766e1, 9.776e1, 9.987e1, 9.794e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">795</td><td style="border: 1px solid #ddd; padding: 4px;">1.227e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.487e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.004e2, 9.766e1, 9.776e1, 9.987e1, 9.794e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">796</td><td style="border: 1px solid #ddd; padding: 4px;">1.227e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.486e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.004e2, 9.766e1, 9.775e1, 9.987e1, 9.794e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">797</td><td style="border: 1px solid #ddd; padding: 4px;">1.227e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.486e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.004e2, 9.766e1, 9.775e1, 9.987e1, 9.794e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">798</td><td style="border: 1px solid #ddd; padding: 4px;">1.227e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.486e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.500e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.004e2, 9.766e1, 9.775e1, 9.987e1, 9.794e1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1651.7
- **Average Gradient Evaluations per Run:** 1650.7
- **Average Iterations per Run:** 824.4
- **Average Time per Run:** 0.037s
- **Function Evaluations per Second:** 44668.9
- **Iterations per Second:** 22293.9
### Resource Utilization
- **Total Function Evaluations:** 33034
- **Total Gradient Evaluations:** 33014
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Griewank_5D_results.csv)
* [Convergence Plot](../plots/Griewank_5D.png)
* [Log Scale Convergence Plot](../plots/Griewank_5D_log.png)


---
*Detailed report for Adam-WeightDecay on Griewank_5D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
