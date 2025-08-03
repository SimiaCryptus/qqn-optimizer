# Detailed Analysis: L-BFGS-Conservative on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** L-BFGS-Conservative
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 5 (25.0%)

### Quick Statistics
* **Best Final Value:** 7.522013e-7
* **Worst Final Value:** 1.888343e2
* **Mean Final Value:** 9.453997e0
* **Success Rate:** 25.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.888e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.303e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.653e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.177e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">258</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.483e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.707e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.908e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.296e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">269</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.562e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.483e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3749</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.574e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.287e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.532e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.005e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">598</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.639e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.184e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.073e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.412e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.879e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.373e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">762</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.247e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.942e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">269</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3924</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.552e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.490e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3911</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.282e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.679e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1069</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.233e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.070e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">716</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.115e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.352e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">300</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3812</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.565e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.462e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">246</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.430e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.964e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3822</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.522e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.140e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.904e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.876e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">252</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3992</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.724e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (5 out of 20)

* **Average Iterations to Convergence:** 157.2
* **Average Function Evaluations:** 1499.6
* **Average Time to Convergence:** 0.026s
* **Fastest Convergence:** 88 iterations (0.015s)
* **Slowest Convergence:** 258 iterations (0.044s)

### Failed Runs (15 out of 20)

**Failure Reasons:**
- FunctionTolerance: 1 runs
- MaxFunctionEvaluations: 14 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 7.522013e-7
**Final Gradient Norm:** 1.140251e-2
**Iterations:** 191
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.548e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.836e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.768e-2, 7.205e-4, 1.864e-1, 1.256e-1, 1.824e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.548e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.836e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.729e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.014e-2, 3.232e-3, 1.879e-1, 1.213e-1, 1.659e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.951e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.466e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.853e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.245e-2, 5.203e-3, 1.866e-1, 1.130e-1, 1.429e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.991e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.580e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.355e-2, 5.342e-3, 1.793e-1, 9.905e-2, 1.142e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.020e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.663e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.368e-2, 5.350e-3, 1.767e-1, 9.544e-2, 1.075e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">187</td><td style="border: 1px solid #ddd; padding: 4px;">2.716e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.336e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.563e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.881e-4, -2.788e-4, -3.509e-4, 2.202e-5, 5.919e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">188</td><td style="border: 1px solid #ddd; padding: 4px;">2.525e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.427e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.173e-4, -2.602e-4, -3.224e-4, 4.210e-6, 5.200e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">189</td><td style="border: 1px solid #ddd; padding: 4px;">1.843e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.243e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.099e-4, -1.793e-4, -1.003e-4, -8.468e-6, -1.536e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">190</td><td style="border: 1px solid #ddd; padding: 4px;">1.170e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.306e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.180e-4, -1.488e-4, -4.647e-5, -1.325e-5, -1.810e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">191</td><td style="border: 1px solid #ddd; padding: 4px;">7.522e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.140e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.180e-4, -1.488e-4, -4.647e-5, -1.325e-5, -1.810e-6, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3123.2
- **Average Gradient Evaluations per Run:** 944.3
- **Average Iterations per Run:** 234.0
- **Average Time per Run:** 0.043s
- **Function Evaluations per Second:** 73201.9
- **Iterations per Second:** 5484.6
### Resource Utilization
- **Total Function Evaluations:** 62463
- **Total Gradient Evaluations:** 18886
- **Total Computation Time:** 0.9s
- **Function/Gradient Ratio:** 3.31
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
*Detailed report for L-BFGS-Conservative on Trigonometric_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
