# Detailed Analysis: Trust Region-Conservative on SparseQuadratic_5D_pattern[1, 3]
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseQuadratic_5D_pattern[1, 3]
**Optimizer:** Trust Region-Conservative
**Problem Family:** SparseQuadratic
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 8.954598e-3
* **Worst Final Value:** 5.153386e0
* **Mean Final Value:** 4.055158e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.736e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.008e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.742e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.046e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.759e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1761</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.054e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.763e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1656</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.153e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.809e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.079e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.775e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">600</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1803</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.016e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.746e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">499</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1500</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.037e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.754e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">657</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1316</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.003e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.902e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1640</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1094</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.062e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.767e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1941</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.072e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.770e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">591</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1776</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.101e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.785e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1947</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1298</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.101e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.784e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.268e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.039e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">593</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1781</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.143e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.805e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">922</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.059e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.764e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.714e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.044e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1655</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.955e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.003e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1931</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.059e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.764e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.073e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.771e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">614</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1230</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 4 runs
- FunctionTolerance: 16 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 8.954598e-3
**Final Gradient Norm:** 2.003494e-1
**Iterations:** 643
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.387e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.348e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.187e0, 1.031e0, 9.603e-1, 1.058e0, 1.094e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.387e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.348e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.870e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.187e0, 1.030e0, 9.596e-1, 1.057e0, 1.093e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.377e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.343e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.871e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.186e0, 1.029e0, 9.588e-1, 1.056e0, 1.092e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.367e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.339e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.873e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.185e0, 1.028e0, 9.581e-1, 1.055e0, 1.091e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.357e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.335e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.874e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.184e0, 1.027e0, 9.573e-1, 1.054e0, 1.091e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">638</td><td style="border: 1px solid #ddd; padding: 4px;">2.108e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.063e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.265e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.805e-2, 3.799e-2, 4.414e-2, 3.963e-2, 5.233e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">639</td><td style="border: 1px solid #ddd; padding: 4px;">1.227e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.335e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.282e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.533e-2, 2.122e-2, 2.653e-2, 2.226e-2, 3.171e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">640</td><td style="border: 1px solid #ddd; padding: 4px;">4.311e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.382e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.235e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.938e-3, -5.897e-3, -3.524e-3, -5.946e-3, -3.768e-3]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">641</td><td style="border: 1px solid #ddd; padding: 4px;">1.256e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.371e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.428e-2, 4.858e-2, 3.120e-2, 4.895e-2, 3.301e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">642</td><td style="border: 1px solid #ddd; padding: 4px;">8.955e-3</td><td style="border: 1px solid #ddd; padding: 4px;">2.003e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.428e-2, 4.858e-2, 3.120e-2, 4.895e-2, 3.301e-2]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1737.1
- **Average Gradient Evaluations per Run:** 1158.2
- **Average Iterations per Run:** 578.1
- **Average Time per Run:** 0.012s
- **Function Evaluations per Second:** 144493.5
- **Iterations per Second:** 48086.9
### Resource Utilization
- **Total Function Evaluations:** 34742
- **Total Gradient Evaluations:** 23164
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/SparseQuadratic_5D_pattern[1,_3]_results.csv)
* [Convergence Plot](../plots/SparseQuadratic_5D_pattern[1,_3].png)
* [Log Scale Convergence Plot](../plots/SparseQuadratic_5D_pattern[1,_3]_log.png)


---
*Detailed report for Trust Region-Conservative on SparseQuadratic_5D_pattern[1, 3]*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
