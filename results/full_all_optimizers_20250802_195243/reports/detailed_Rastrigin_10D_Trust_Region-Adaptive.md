# Detailed Analysis: Trust Region-Adaptive on Rastrigin_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rastrigin_10D
**Optimizer:** Trust Region-Adaptive
**Problem Family:** Rastrigin
**Dimension:** 10
**Success Threshold:** 4.180e1
**Total Runs:** 20
**Successful Runs:** 8 (40.0%)

### Quick Statistics
* **Best Final Value:** 4.156441e1
* **Worst Final Value:** 4.981756e1
* **Mean Final Value:** 4.393215e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.160e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.747e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">492</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1479</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">987</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.185e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.073e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.159e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.725e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">977</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.159e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.560e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">427</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.163e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.719e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.975e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.461e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1241</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.484e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.230e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.163e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.875e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">494</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.180e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.881e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.483e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.553e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1098</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.182e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.244e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">568</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1706</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.158e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.686e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.677e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.643e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">515</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.481e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.369e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.478e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.724e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">932</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.976e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.176e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">914</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.182e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.287e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.481e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.178e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">750</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.982e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.428e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">441</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.156e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.292e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">498</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1497</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (8 out of 20)

* **Average Iterations to Convergence:** 463.1
* **Average Function Evaluations:** 1392.4
* **Average Time to Convergence:** 0.010s
* **Fastest Convergence:** 386 iterations (0.008s)
* **Slowest Convergence:** 494 iterations (0.010s)

### Failed Runs (12 out of 20)

**Failure Reasons:**
- GradientTolerance: 12 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 4.156441e1
**Final Gradient Norm:** 6.292241e1
**Iterations:** 498
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.659e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.623e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.145e0, 2.271e0, 2.471e0, 2.133e0, 1.506e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.659e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.623e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.540e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.145e0, 2.271e0, 2.471e0, 2.132e0, 1.506e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.657e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.624e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.540e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.144e0, 2.270e0, 2.471e0, 2.132e0, 1.506e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.654e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.624e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.540e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.144e0, 2.269e0, 2.470e0, 2.131e0, 1.506e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.652e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.624e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.539e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.143e0, 2.269e0, 2.470e0, 2.131e0, 1.506e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">494</td><td style="border: 1px solid #ddd; padding: 4px;">4.256e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.436e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.884e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.991e0, 1.997e0, 1.990e0, 1.218e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">495</td><td style="border: 1px solid #ddd; padding: 4px;">4.231e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.406e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.903e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.991e0, 1.997e0, 1.990e0, 1.214e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">496</td><td style="border: 1px solid #ddd; padding: 4px;">4.206e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.372e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.923e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.991e0, 1.997e0, 1.990e0, 1.210e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">497</td><td style="border: 1px solid #ddd; padding: 4px;">4.181e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.334e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.947e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.991e0, 1.997e0, 1.990e0, 1.206e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">498</td><td style="border: 1px solid #ddd; padding: 4px;">4.156e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.292e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.991e0, 1.997e0, 1.990e0, 1.206e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1397.8
- **Average Gradient Evaluations per Run:** 932.7
- **Average Iterations per Run:** 465.1
- **Average Time per Run:** 0.010s
- **Function Evaluations per Second:** 142874.8
- **Iterations per Second:** 47543.2
### Resource Utilization
- **Total Function Evaluations:** 27957
- **Total Gradient Evaluations:** 18654
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Rastrigin_10D_results.csv)
* [Convergence Plot](../plots/Rastrigin_10D.png)
* [Log Scale Convergence Plot](../plots/Rastrigin_10D_log.png)


---
*Detailed report for Trust Region-Adaptive on Rastrigin_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
