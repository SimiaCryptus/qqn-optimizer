# Detailed Analysis: QQN-CubicQuadraticInterpolation on Michalewicz_10D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_10D_m10
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** Michalewicz
**Dimension:** 10
**Success Threshold:** -6.260e0
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -6.259820e0
* **Worst Final Value:** -9.353042e-1
* **Mean Final Value:** -3.343291e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.338e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.716e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2240</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.097</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.312e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.805e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.835e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.740e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.224e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.113e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2780</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.363e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.985e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">679</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.302e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.062e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2846</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.260e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.397e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.625e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.208e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">504</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.776e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.918e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.696e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.816e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.569e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.591e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.096</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.554e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.784e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.404e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.167e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">242</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.978e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.209e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">196</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">224</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.496e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.097e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.284e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.904e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.096</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.795e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-9.353e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.077e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.096</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.816e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.962e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2761</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.786e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1678</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 1 runs
- MaxFunctionEvaluations: 14 runs
- FunctionTolerance: 5 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 7)
**Final Value:** -6.259820e0
**Final Gradient Norm:** 9.397455e-8
**Iterations:** 40
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.733e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.219e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 9.289e-1, 8.188e-1, 8.869e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.733e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.219e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.369e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 9.302e-1, 8.211e-1, 1.014e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-1.964e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.283e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.027e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 9.319e-1, 8.242e-1, 9.257e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-2.064e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.634e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 9.320e-1, 8.244e-1, 9.711e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-3.755e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.390e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.132e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 9.324e-1, 8.252e-1, 9.990e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">35</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.162e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">36</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.162e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">37</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.421e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.360e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">38</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.054e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">39</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.397e-8</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-1, 6.820e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1730.8
- **Average Gradient Evaluations per Run:** 2143.9
- **Average Iterations per Run:** 136.2
- **Average Time per Run:** 0.071s
- **Function Evaluations per Second:** 24207.7
- **Iterations per Second:** 1905.6
### Resource Utilization
- **Total Function Evaluations:** 34617
- **Total Gradient Evaluations:** 42879
- **Total Computation Time:** 1.4s
- **Function/Gradient Ratio:** 0.81
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Michalewicz_10D_m10_results.csv)
* [Convergence Plot](../plots/Michalewicz_10D_m10.png)
* [Log Scale Convergence Plot](../plots/Michalewicz_10D_m10_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on Michalewicz_10D_m10*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
