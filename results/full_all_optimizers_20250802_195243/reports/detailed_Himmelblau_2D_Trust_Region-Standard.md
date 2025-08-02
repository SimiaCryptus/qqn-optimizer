# Detailed Analysis: Trust Region-Standard on Himmelblau_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Himmelblau_2D
**Optimizer:** Trust Region-Standard
**Problem Family:** Himmelblau
**Dimension:** 2
**Success Threshold:** 2.500e-1
**Total Runs:** 20
**Successful Runs:** 16 (80.0%)

### Quick Statistics
* **Best Final Value:** 3.760955e-3
* **Worst Final Value:** 3.030555e-1
* **Mean Final Value:** 1.500617e-1
* **Success Rate:** 80.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.576e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.451e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.761e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.822e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.437e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.292e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">498</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.501e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.310e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">525</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.068e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.400e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">347</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.405e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.125e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.544e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.716e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.300e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.018e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.623e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.164e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.697e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.153e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">515</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.644e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.390e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.406e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.514e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">347</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.272e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.356e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">498</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.184e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.429e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.327e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.614e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.209e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.836e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.017e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.206e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.031e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.374e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.189e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.049e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.034e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.725e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">525</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (16 out of 20)

* **Average Iterations to Convergence:** 170.9
* **Average Function Evaluations:** 515.8
* **Average Time to Convergence:** 0.003s
* **Fastest Convergence:** 165 iterations (0.003s)
* **Slowest Convergence:** 174 iterations (0.003s)

### Failed Runs (4 out of 20)

**Failure Reasons:**
- GradientTolerance: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 3.760955e-3
**Final Gradient Norm:** 7.821513e-1
**Iterations:** 170
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.691e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.773e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.298e-2, -1.807e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.691e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.773e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.606e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.057e-1, 9.913e-3]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.681e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.888e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.463e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.278e-1, 3.657e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.670e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.997e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.336e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.493e-1, 6.206e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.660e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.102e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.224e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.704e-1, 8.651e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">166</td><td style="border: 1px solid #ddd; padding: 4px;">3.201e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.122e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.712e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.778e0, 1.901e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">167</td><td style="border: 1px solid #ddd; padding: 4px;">2.273e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.812e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.518e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.829e0, 1.923e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">168</td><td style="border: 1px solid #ddd; padding: 4px;">1.377e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.432e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.983e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.893e0, 1.952e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">169</td><td style="border: 1px solid #ddd; padding: 4px;">5.527e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.245e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.082e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.992e0, 1.995e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">170</td><td style="border: 1px solid #ddd; padding: 4px;">3.761e-3</td><td style="border: 1px solid #ddd; padding: 4px;">7.822e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.992e0, 1.995e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 516.7
- **Average Gradient Evaluations per Run:** 345.4
- **Average Iterations per Run:** 171.3
- **Average Time per Run:** 0.003s
- **Function Evaluations per Second:** 165182.2
- **Iterations per Second:** 54762.4
### Resource Utilization
- **Total Function Evaluations:** 10334
- **Total Gradient Evaluations:** 6908
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 1)**
- Final Value: 2.575941e-1
- Final Gradient Norm: 6.450991e0
- Iterations: 175
- Function Evaluations: 527
- Reason: GradientTolerance

**Run 2 (ID: 10)**
- Final Value: 2.696702e-1
- Final Gradient Norm: 6.153010e0
- Iterations: 171
- Function Evaluations: 515
- Reason: GradientTolerance

**Run 3 (ID: 11)**
- Final Value: 2.643754e-1
- Final Gradient Norm: 4.389821e0
- Iterations: 172
- Function Evaluations: 518
- Reason: GradientTolerance

**Run 4 (ID: 18)**
- Final Value: 3.030555e-1
- Final Gradient Norm: 4.373690e0
- Iterations: 173
- Function Evaluations: 521
- Reason: GradientTolerance



## Data Files
* [Raw CSV Data](../data/problems/Himmelblau_2D_results.csv)
* [Convergence Plot](../plots/Himmelblau_2D.png)
* [Log Scale Convergence Plot](../plots/Himmelblau_2D_log.png)


---
*Detailed report for Trust Region-Standard on Himmelblau_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
