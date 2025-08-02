# Detailed Analysis: Trust Region-Conservative on Barrier_10D_mu0.1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Barrier_10D_mu0.1
**Optimizer:** Trust Region-Conservative
**Problem Family:** Barrier
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.998198e0
* **Worst Final Value:** 2.182949e0
* **Mean Final Value:** 2.054012e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.136e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.238e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">840</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1682</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.005e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.414e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">760</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.022e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.682e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">855</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2568</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1712</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.000e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.192e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.061e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.990e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">804</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.027e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.884e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1702</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.105e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2304</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.082e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.525e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">818</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1638</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.077e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.916e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">800</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2403</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.097e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.018e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2601</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1734</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.032e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.651e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2679</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1786</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.031e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.535e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">935</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2808</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1872</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.000e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.158e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">729</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.000e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.282e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1692</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.035e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.853e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">792</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.070e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.512e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2514</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1676</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.183e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.089e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">812</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1626</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.999e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.015e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2579</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.044e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.542e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2655</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.182e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.086e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2534</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1690</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 10 runs
- GradientTolerance: 8 runs
- NumericalError: 2 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 7)
**Final Value:** 1.998198e0
**Final Gradient Norm:** 5.105058e-2
**Iterations:** 767
**Convergence Reason:** NumericalError

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.609e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.867e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.160e-1, 8.363e-1, 8.319e-1, 1.010e0, 1.067e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.609e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.867e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.705e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.155e-1, 8.358e-1, 8.315e-1, 1.009e0, 1.066e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.599e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.863e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.706e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.150e-1, 8.354e-1, 8.310e-1, 1.008e0, 1.066e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.589e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.707e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.145e-1, 8.349e-1, 8.306e-1, 1.008e0, 1.065e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.579e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.856e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.708e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.140e-1, 8.345e-1, 8.301e-1, 1.007e0, 1.065e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">762</td><td style="border: 1px solid #ddd; padding: 4px;">2.004e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.234e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.475e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.271e-1, 2.265e-1, 2.264e-1, 2.280e-1, 2.286e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">763</td><td style="border: 1px solid #ddd; padding: 4px;">1.998e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.325e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.008e-1, 2.051e-1, 2.053e-1, 1.951e-1, 1.912e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">764</td><td style="border: 1px solid #ddd; padding: 4px;">2.014e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.728e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.073e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.285e-1, 2.274e-1, 2.273e-1, 2.303e-1, 2.317e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">765</td><td style="border: 1px solid #ddd; padding: 4px;">1.999e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.184e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.808e-1, 1.910e-1, 1.915e-1, 1.657e-1, 1.544e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">766</td><td style="border: 1px solid #ddd; padding: 4px;">2.070e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.730e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.833e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.210e-1, 2.207e-1, 2.207e-1, 2.228e-1, 2.256e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2487.1
- **Average Gradient Evaluations per Run:** 1658.4
- **Average Iterations per Run:** 828.1
- **Average Time per Run:** 0.018s
- **Function Evaluations per Second:** 134860.0
- **Iterations per Second:** 44906.3
### Resource Utilization
- **Total Function Evaluations:** 49741
- **Total Gradient Evaluations:** 33168
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 2
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Barrier_10D_mu0.1_results.csv)
* [Convergence Plot](../plots/Barrier_10D_mu0.1.png)
* [Log Scale Convergence Plot](../plots/Barrier_10D_mu0.1_log.png)


---
*Detailed report for Trust Region-Conservative on Barrier_10D_mu0.1*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
