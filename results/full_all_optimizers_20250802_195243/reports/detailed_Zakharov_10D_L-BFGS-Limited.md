# Detailed Analysis: L-BFGS-Limited on Zakharov_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_10D
**Optimizer:** L-BFGS-Limited
**Problem Family:** Zakharov
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 11 (55.0%)

### Quick Statistics
* **Best Final Value:** 5.258153e-9
* **Worst Final Value:** 9.861400e-1
* **Mean Final Value:** 9.287340e-2
* **Success Rate:** 55.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.612e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.210e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1278</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.084e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.053e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.162e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.111e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">269</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">812</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.259e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">398</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3804</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.120e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.775e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3979</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.236e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">830</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.994e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.071e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">302</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">911</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.367e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.166e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.907e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.878e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">656</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.258e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.001e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2654</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">454</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.728e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.966e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.742e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.060e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.674e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.228e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">782</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">414</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.585e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.083e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">338</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.787e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.983e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">306</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2697</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.610e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.113e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2511</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">750</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.239e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.188e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">285</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.685e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.508e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.853e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.375e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3955</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.885e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.554e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (11 out of 20)

* **Average Iterations to Convergence:** 155.5
* **Average Function Evaluations:** 1401.7
* **Average Time to Convergence:** 0.022s
* **Fastest Convergence:** 43 iterations (0.006s)
* **Slowest Convergence:** 306 iterations (0.043s)

### Failed Runs (9 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 5.258153e-9
**Final Gradient Norm:** 4.001184e-4
**Iterations:** 151
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.901e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.353e5</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.578e-1, 1.051e0, 9.689e-1, 1.109e0, 9.027e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.901e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.353e5</td><td style="border: 1px solid #ddd; padding: 4px;">9.811e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.078e-1, 9.515e-1, 8.189e-1, 9.088e-1, 6.527e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.072e5</td><td style="border: 1px solid #ddd; padding: 4px;">2.324e5</td><td style="border: 1px solid #ddd; padding: 4px;">9.811e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.578e-1, 8.515e-1, 6.689e-1, 7.088e-1, 4.026e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.190e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.391e4</td><td style="border: 1px solid #ddd; padding: 4px;">1.966e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.076e-1, 7.512e-1, 5.186e-1, 5.086e-1, 1.525e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.242e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.618e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.125e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.125e-1, 7.632e-1, 5.387e-1, 5.359e-1, 1.883e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">147</td><td style="border: 1px solid #ddd; padding: 4px;">3.406e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.561e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.694e-3, -1.863e-3, -1.398e-3, -1.443e-3, -6.941e-4, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">148</td><td style="border: 1px solid #ddd; padding: 4px;">1.228e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.092e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.350e-5, -6.613e-5, -4.343e-5, -4.114e-5, -7.122e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">149</td><td style="border: 1px solid #ddd; padding: 4px;">1.561e-7</td><td style="border: 1px solid #ddd; padding: 4px;">7.369e-3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.317e-5, -6.746e-5, -4.725e-5, -4.677e-5, -1.562e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">150</td><td style="border: 1px solid #ddd; padding: 4px;">1.613e-8</td><td style="border: 1px solid #ddd; padding: 4px;">5.251e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.539e-5, -3.776e-5, -2.640e-5, -2.611e-5, -8.605e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">151</td><td style="border: 1px solid #ddd; padding: 4px;">5.258e-9</td><td style="border: 1px solid #ddd; padding: 4px;">4.001e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.539e-5, -3.776e-5, -2.640e-5, -2.611e-5, -8.605e-6, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2588.7
- **Average Gradient Evaluations per Run:** 695.6
- **Average Iterations per Run:** 230.6
- **Average Time per Run:** 0.034s
- **Function Evaluations per Second:** 76404.0
- **Iterations per Second:** 6806.0
### Resource Utilization
- **Total Function Evaluations:** 51774
- **Total Gradient Evaluations:** 13912
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 3.72
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Zakharov_10D_results.csv)
* [Convergence Plot](../plots/Zakharov_10D.png)
* [Log Scale Convergence Plot](../plots/Zakharov_10D_log.png)


---
*Detailed report for L-BFGS-Limited on Zakharov_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
