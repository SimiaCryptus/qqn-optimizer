# Detailed Analysis: Adam on Rosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rosenbrock_10D
**Optimizer:** Adam
**Problem Family:** Rosenbrock
**Dimension:** 10
**Success Threshold:** 9.700e0
**Total Runs:** 20
**Successful Runs:** 18 (90.0%)

### Quick Statistics
* **Best Final Value:** 9.675207e0
* **Worst Final Value:** 9.926981e0
* **Mean Final Value:** 9.714602e0
* **Success Rate:** 90.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.695e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.883e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.899e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.304e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.639e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.675e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.659e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1208</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2419</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2419</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.696e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.712e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.695e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.254e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.699e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.783e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.691e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.263e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.692e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.117e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.686e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.070e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1225</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.927e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.341e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.695e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.389e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.693e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.187e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.425e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2229</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2229</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.690e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.370e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.689e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.511e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.366e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2297</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2297</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.700e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.107e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.689e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.552e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.698e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.406e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (18 out of 20)

* **Average Iterations to Convergence:** 1133.4
* **Average Function Evaluations:** 2269.9
* **Average Time to Convergence:** 0.050s
* **Fastest Convergence:** 1044 iterations (0.044s)
* **Slowest Convergence:** 1213 iterations (0.054s)

### Failed Runs (2 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 2 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 9.675207e0
**Final Gradient Norm:** 9.659277e0
**Iterations:** 1208
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.655e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.519e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.374e0, 8.661e-1, -1.345e0, 1.088e0, -1.297e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.655e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.519e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.373e0, 8.651e-1, -1.344e0, 1.087e0, -1.296e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.648e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.514e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.372e0, 8.641e-1, -1.343e0, 1.086e0, -1.295e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.640e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.508e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.371e0, 8.631e-1, -1.342e0, 1.085e0, -1.294e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.633e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.502e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.370e0, 8.621e-1, -1.341e0, 1.084e0, -1.293e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1204</td><td style="border: 1px solid #ddd; padding: 4px;">9.788e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.166e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.255e-1, 1.111e-1, -1.297e-2, 1.606e-2, -1.381e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1205</td><td style="border: 1px solid #ddd; padding: 4px;">9.757e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.116e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.246e-1, 1.108e-1, -1.153e-2, 1.562e-2, -1.265e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1206</td><td style="border: 1px solid #ddd; padding: 4px;">9.729e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.066e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.236e-1, 1.105e-1, -1.009e-2, 1.518e-2, -1.149e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1207</td><td style="border: 1px solid #ddd; padding: 4px;">9.701e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.016e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.227e-1, 1.103e-1, -8.653e-3, 1.475e-2, -1.034e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1208</td><td style="border: 1px solid #ddd; padding: 4px;">9.675e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.659e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.227e-1, 1.103e-1, -8.653e-3, 1.475e-2, -1.034e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2293.1
- **Average Gradient Evaluations per Run:** 2293.1
- **Average Iterations per Run:** 1145.0
- **Average Time per Run:** 0.050s
- **Function Evaluations per Second:** 45430.4
- **Iterations per Second:** 22684.5
### Resource Utilization
- **Total Function Evaluations:** 45862
- **Total Gradient Evaluations:** 45862
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 2)**
- Final Value: 9.899030e0
- Final Gradient Norm: 1.304046e1
- Iterations: 1249
- Function Evaluations: 2502
- Reason: MaxFunctionEvaluations

**Run 2 (ID: 11)**
- Final Value: 9.926981e0
- Final Gradient Norm: 1.341125e1
- Iterations: 1249
- Function Evaluations: 2502
- Reason: MaxFunctionEvaluations



## Data Files
* [Raw CSV Data](../data/problems/Rosenbrock_10D_results.csv)
* [Convergence Plot](../plots/Rosenbrock_10D.png)
* [Log Scale Convergence Plot](../plots/Rosenbrock_10D_log.png)


---
*Detailed report for Adam on Rosenbrock_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
