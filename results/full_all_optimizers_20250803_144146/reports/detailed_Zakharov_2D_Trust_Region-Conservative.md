# Detailed Analysis: Trust Region-Conservative on Zakharov_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_2D
**Optimizer:** Trust Region-Conservative
**Problem Family:** Zakharov
**Dimension:** 2
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.047941e-2
* **Worst Final Value:** 6.573238e0
* **Mean Final Value:** 8.836734e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.138e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.576e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.129e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.659e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.025e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2600</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1734</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.183e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.008e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">924</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.924e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.162e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2645</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.922e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.689e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.845e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.611e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1832</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.573e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.590e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.241e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.911e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.858e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.957e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.828e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.697e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">701</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.101e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.153e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.940e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.887e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">735</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1472</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.048e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.692e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2588</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1726</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.483e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.176e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.250e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.025e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">515</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.706e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2195</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.093e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.220e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.927e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.366e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.487e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.181e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">795</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 8 runs
- GradientTolerance: 12 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 1.047941e-2
**Final Gradient Norm:** 2.691697e-1
**Iterations:** 862
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.573e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.971e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.031e0, 9.412e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.573e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.971e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.073e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.031e0, 9.407e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.563e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.969e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.078e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.030e0, 9.403e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.553e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.967e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.083e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.030e0, 9.398e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.543e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.966e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.088e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.030e0, 9.394e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">857</td><td style="border: 1px solid #ddd; padding: 4px;">2.302e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.287e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.042e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.125e-1, -3.176e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">858</td><td style="border: 1px solid #ddd; padding: 4px;">1.425e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.498e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.003e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.250e-2, -2.943e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">859</td><td style="border: 1px solid #ddd; padding: 4px;">6.169e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.584e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.313e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.200e-2, -1.141e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">860</td><td style="border: 1px solid #ddd; padding: 4px;">3.035e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.843e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.639e-2, 7.610e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">861</td><td style="border: 1px solid #ddd; padding: 4px;">1.048e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.692e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.639e-2, 7.610e-2]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2569.1
- **Average Gradient Evaluations per Run:** 1713.4
- **Average Iterations per Run:** 855.3
- **Average Time per Run:** 0.016s
- **Function Evaluations per Second:** 156235.3
- **Iterations per Second:** 52013.6
### Resource Utilization
- **Total Function Evaluations:** 51382
- **Total Gradient Evaluations:** 34268
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Zakharov_2D_results.csv)
* [Convergence Plot](../plots/Zakharov_2D.png)
* [Log Scale Convergence Plot](../plots/Zakharov_2D_log.png)


---
*Detailed report for Trust Region-Conservative on Zakharov_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
