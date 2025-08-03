# Detailed Analysis: Trust Region-Aggressive on IllConditionedRosenbrock_10D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_10D_alpha100
**Optimizer:** Trust Region-Aggressive
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 9.595986e0
* **Worst Final Value:** 9.275376e2
* **Mean Final Value:** 6.554301e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.687e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.148e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">570</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1712</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.894e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.046e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">461</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">924</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.892e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.069e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">618</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1238</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.275e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.869e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">656</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1314</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.030e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.619e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.002e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.269e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">575</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1727</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.079e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.055e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">964</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.604e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.210e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">966</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.055e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.850e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.847e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.054e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">500</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.031e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.606e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.015e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.440e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1778</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.026e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.398e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1542</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.596e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.225e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">673</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.688e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.426e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">964</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.005e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.323e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1280</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.871e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.031e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">543</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.757e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.541e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1679</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.090e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.126e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">598</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1796</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1198</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.809e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.358e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">558</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1676</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 2 runs
- GradientTolerance: 18 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 9.595986e0
**Final Gradient Norm:** 2.225455e0
**Iterations:** 673
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.687e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.674e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.062e0, 1.101e0, -1.382e0, 9.989e-1, -1.128e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.687e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.674e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.496e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.062e0, 1.101e0, -1.381e0, 9.985e-1, -1.127e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.683e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.670e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.062e0, 1.100e0, -1.380e0, 9.982e-1, -1.127e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.679e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.667e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.500e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.062e0, 1.099e0, -1.380e0, 9.978e-1, -1.127e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.675e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.664e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.502e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.062e0, 1.099e0, -1.379e0, 9.974e-1, -1.127e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">668</td><td style="border: 1px solid #ddd; padding: 4px;">2.221e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.858e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.829e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.104e-1, 2.454e-1, 4.748e-3, 3.006e-2, -4.810e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">669</td><td style="border: 1px solid #ddd; padding: 4px;">1.849e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.035e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.944e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.008e-1, 2.439e-1, 2.548e-2, 2.271e-2, -3.030e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">670</td><td style="border: 1px solid #ddd; padding: 4px;">1.489e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.020e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.951e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.900e-1, 2.428e-1, 4.771e-2, 1.593e-2, -1.070e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">671</td><td style="border: 1px solid #ddd; padding: 4px;">1.159e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.600e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.539e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.756e-1, 2.420e-1, 7.386e-2, 1.100e-2, 1.378e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">672</td><td style="border: 1px solid #ddd; padding: 4px;">9.596e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.225e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.756e-1, 2.420e-1, 7.386e-2, 1.100e-2, 1.378e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1618.0
- **Average Gradient Evaluations per Run:** 1079.3
- **Average Iterations per Run:** 538.6
- **Average Time per Run:** 0.011s
- **Function Evaluations per Second:** 143367.9
- **Iterations per Second:** 47727.3
### Resource Utilization
- **Total Function Evaluations:** 32361
- **Total Gradient Evaluations:** 21586
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.50
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
*Detailed report for Trust Region-Aggressive on IllConditionedRosenbrock_10D_alpha100*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
