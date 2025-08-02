# Detailed Analysis: Trust Region-Conservative on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** Trust Region-Conservative
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.820362e-3
* **Worst Final Value:** 5.774876e0
* **Mean Final Value:** 1.221911e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.705e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.357e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">581</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1745</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.124e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.171e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">734</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.462e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.643e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">808</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1618</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.379e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.039e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.624e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.372e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.316e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.408e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.990e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.974e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.787e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.168e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">802</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.915e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.022e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.200e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.422e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.657e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.982e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.370e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.088e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.938e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.039e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">824</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1650</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.364e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.552e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2303</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.775e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.759e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.845e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.612e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.835e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.705e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.551e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.077e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">726</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1454</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.820e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.424e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">822</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2468</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.630e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 8 runs
- MaxFunctionEvaluations: 12 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 19)
**Final Value:** 3.820362e-3
**Final Gradient Norm:** 3.423513e-1
**Iterations:** 822
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.182e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.845e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.585e-1, 1.764e-1, 3.919e-1, 2.953e-1, 2.850e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.182e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.845e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.515e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[3.585e-1, 1.764e-1, 3.920e-1, 2.953e-1, 2.853e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.172e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.844e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.516e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[3.586e-1, 1.764e-1, 3.920e-1, 2.953e-1, 2.856e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.162e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.843e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.518e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[3.586e-1, 1.765e-1, 3.921e-1, 2.953e-1, 2.859e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.152e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.842e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.519e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[3.587e-1, 1.765e-1, 3.921e-1, 2.953e-1, 2.861e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">817</td><td style="border: 1px solid #ddd; padding: 4px;">3.147e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.994e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.002e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.214e-1, 2.007e-1, 1.120e-1, 7.344e-2, 5.830e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">818</td><td style="border: 1px solid #ddd; padding: 4px;">2.310e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.486e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.868e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.449e-1, 1.891e-1, 1.019e-1, 7.671e-2, 6.101e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">819</td><td style="border: 1px solid #ddd; padding: 4px;">1.553e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.244e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.456e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.821e-1, 1.726e-1, 1.153e-1, 8.386e-2, 6.651e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">820</td><td style="border: 1px solid #ddd; padding: 4px;">8.501e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.340e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.463e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.445e-1, 2.037e-1, 1.176e-1, 9.744e-2, 7.892e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">821</td><td style="border: 1px solid #ddd; padding: 4px;">3.820e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.424e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.445e-1, 2.037e-1, 1.176e-1, 9.744e-2, 7.892e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2651.3
- **Average Gradient Evaluations per Run:** 1768.2
- **Average Iterations per Run:** 882.5
- **Average Time per Run:** 0.028s
- **Function Evaluations per Second:** 94073.5
- **Iterations per Second:** 31312.9
### Resource Utilization
- **Total Function Evaluations:** 53026
- **Total Gradient Evaluations:** 35364
- **Total Computation Time:** 0.6s
- **Function/Gradient Ratio:** 1.50
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
*Detailed report for Trust Region-Conservative on Trigonometric_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
