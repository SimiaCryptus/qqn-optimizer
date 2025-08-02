# Detailed Analysis: Adam on Sphere_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Sphere_2D
**Optimizer:** Adam
**Problem Family:** Sphere
**Dimension:** 2
**Success Threshold:** 5.000e-3
**Total Runs:** 20
**Successful Runs:** 8 (40.0%)

### Quick Statistics
* **Best Final Value:** 4.964209e-3
* **Worst Final Value:** 1.277169e-2
* **Mean Final Value:** 7.600045e-3
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.971e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.995e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.413e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.993e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.413e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.964e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.409e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1068</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.870e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.884e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.991e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.413e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1242</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.768e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.645e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.193e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.574e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.981e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.412e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.985e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.412e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.935e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.665e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.589e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.958e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.182e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.175e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.897e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.886e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.260e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.948e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.542e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.277e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.260e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.181e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.173e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.987e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.412e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.930e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.993e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (8 out of 20)

* **Average Iterations to Convergence:** 1162.5
* **Average Function Evaluations:** 2328.0
* **Average Time to Convergence:** 0.046s
* **Fastest Convergence:** 1068 iterations (0.043s)
* **Slowest Convergence:** 1220 iterations (0.053s)

### Failed Runs (12 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 12 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 4.964209e-3
**Final Gradient Norm:** 1.409143e-1
**Iterations:** 1068
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.432e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.393e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.255e-1, 8.661e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.432e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.393e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.245e-1, 8.651e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.428e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.390e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.235e-1, 8.641e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.425e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.387e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.225e-1, 8.631e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.421e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.385e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.215e-1, 8.621e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1064</td><td style="border: 1px solid #ddd; padding: 4px;">5.132e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.433e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.316e-2, 5.680e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1065</td><td style="border: 1px solid #ddd; padding: 4px;">5.090e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.427e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.297e-2, 5.657e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1066</td><td style="border: 1px solid #ddd; padding: 4px;">5.047e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.421e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.279e-2, 5.635e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1067</td><td style="border: 1px solid #ddd; padding: 4px;">5.006e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.415e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.260e-2, 5.612e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1068</td><td style="border: 1px solid #ddd; padding: 4px;">4.964e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.409e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.260e-2, 5.612e-2]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2432.4
- **Average Gradient Evaluations per Run:** 2432.4
- **Average Iterations per Run:** 1214.4
- **Average Time per Run:** 0.048s
- **Function Evaluations per Second:** 50784.2
- **Iterations per Second:** 25354.5
### Resource Utilization
- **Total Function Evaluations:** 48648
- **Total Gradient Evaluations:** 48648
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Sphere_2D_results.csv)
* [Convergence Plot](../plots/Sphere_2D.png)
* [Log Scale Convergence Plot](../plots/Sphere_2D_log.png)


---
*Detailed report for Adam on Sphere_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
