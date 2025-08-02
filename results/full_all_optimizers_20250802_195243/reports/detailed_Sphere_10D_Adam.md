# Detailed Analysis: Adam on Sphere_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Sphere_10D
**Optimizer:** Adam
**Problem Family:** Sphere
**Dimension:** 10
**Success Threshold:** 5.000e-3
**Total Runs:** 20
**Successful Runs:** 19 (95.0%)

### Quick Statistics
* **Best Final Value:** 4.941205e-3
* **Worst Final Value:** 5.213320e-3
* **Mean Final Value:** 4.976744e-3
* **Success Rate:** 95.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.952e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.407e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.973e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.983e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.412e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1201</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.960e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.409e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.967e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1206</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.944e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.406e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.942e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.406e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.955e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.408e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1225</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.971e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.941e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.406e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.993e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.413e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1094</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.950e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.407e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2497</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2497</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.948e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.407e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.980e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.411e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1197</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.213e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.444e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.953e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.408e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.969e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.968e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.980e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.411e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1246</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.993e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.413e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (19 out of 20)

* **Average Iterations to Convergence:** 1191.5
* **Average Function Evaluations:** 2386.1
* **Average Time to Convergence:** 0.052s
* **Fastest Convergence:** 1094 iterations (0.046s)
* **Slowest Convergence:** 1225 iterations (0.060s)

### Failed Runs (1 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 4.941205e-3
**Final Gradient Norm:** 1.405874e-1
**Iterations:** 1095
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.758e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.919e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.636e-1, 8.840e-1, 1.000e0, 9.116e-1, 8.601e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.758e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.919e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.626e-1, 8.830e-1, 9.993e-1, 9.106e-1, 8.591e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.739e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.912e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.616e-1, 8.820e-1, 9.983e-1, 9.096e-1, 8.581e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.720e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.906e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.606e-1, 8.810e-1, 9.973e-1, 9.086e-1, 8.571e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.702e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.900e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.596e-1, 8.800e-1, 9.963e-1, 9.076e-1, 8.561e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1091</td><td style="border: 1px solid #ddd; padding: 4px;">5.213e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.444e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.051e-2, 8.993e-3, 2.807e-2, 1.227e-2, 6.696e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1092</td><td style="border: 1px solid #ddd; padding: 4px;">5.144e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.434e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.036e-2, 8.906e-3, 2.787e-2, 1.216e-2, 6.626e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1093</td><td style="border: 1px solid #ddd; padding: 4px;">5.075e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.425e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.020e-2, 8.820e-3, 2.768e-2, 1.205e-2, 6.558e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1094</td><td style="border: 1px solid #ddd; padding: 4px;">5.008e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.415e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.004e-2, 8.735e-3, 2.748e-2, 1.195e-2, 6.490e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1095</td><td style="border: 1px solid #ddd; padding: 4px;">4.941e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.004e-2, 8.735e-3, 2.748e-2, 1.195e-2, 6.490e-3, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2391.8
- **Average Gradient Evaluations per Run:** 2391.8
- **Average Iterations per Run:** 1194.4
- **Average Time per Run:** 0.052s
- **Function Evaluations per Second:** 46048.1
- **Iterations per Second:** 22994.7
### Resource Utilization
- **Total Function Evaluations:** 47837
- **Total Gradient Evaluations:** 47837
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 15)**
- Final Value: 5.213320e-3
- Final Gradient Norm: 1.444066e-1
- Iterations: 1249
- Function Evaluations: 2502
- Reason: MaxFunctionEvaluations



## Data Files
* [Raw CSV Data](../data/problems/Sphere_10D_results.csv)
* [Convergence Plot](../plots/Sphere_10D.png)
* [Log Scale Convergence Plot](../plots/Sphere_10D_log.png)


---
*Detailed report for Adam on Sphere_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
