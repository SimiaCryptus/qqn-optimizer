# Detailed Analysis: L-BFGS-Conservative on PenaltyI_10D_alpha1e6
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** PenaltyI_10D_alpha1e6
**Optimizer:** L-BFGS-Conservative
**Problem Family:** PenaltyI
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 5.625008e0
* **Worst Final Value:** 6.124238e0
* **Mean Final Value:** 5.732209e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.741e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.135e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3752</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.636e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.612e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">347</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3597</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.759e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.800e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">197</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.782e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.453e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">267</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.703e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.832e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.643e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.791e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.768e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.311e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">496</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2962</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.072</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.652e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.338e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">782</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.703e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.390e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1218</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.720e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.552e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3800</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.625e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.319e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">509</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.655e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.802e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3401</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1611</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.686e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.058e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.772e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.922e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">847</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.700e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.202e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">221</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.635e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.930e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.645e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.397e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.636e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.639e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">302</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3735</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1274</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.124e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.949e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3461</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.057e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.727e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">375</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 12 runs
- FunctionTolerance: 8 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 11)
**Final Value:** 5.625008e0
**Final Gradient Norm:** 7.318581e0
**Iterations:** 50
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.151e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.691e6</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.323e-1, 4.516e-1, 5.052e-1, 5.759e-1, 4.945e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.151e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.691e6</td><td style="border: 1px solid #ddd; padding: 4px;">2.115e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.117e-1, 4.012e-1, 4.414e-1, 4.944e-1, 4.334e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.022e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.268e6</td><td style="border: 1px solid #ddd; padding: 4px;">2.115e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.912e-1, 3.508e-1, 3.776e-1, 4.129e-1, 3.722e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.787e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.454e5</td><td style="border: 1px solid #ddd; padding: 4px;">4.230e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[2.706e-1, 3.004e-1, 3.138e-1, 3.314e-1, 3.111e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.463e4</td><td style="border: 1px solid #ddd; padding: 4px;">4.225e5</td><td style="border: 1px solid #ddd; padding: 4px;">2.114e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.499e-1, 2.499e-1, 2.499e-1, 2.499e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">45</td><td style="border: 1px solid #ddd; padding: 4px;">5.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.319e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">46</td><td style="border: 1px solid #ddd; padding: 4px;">5.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.319e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">47</td><td style="border: 1px solid #ddd; padding: 4px;">5.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.319e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">48</td><td style="border: 1px solid #ddd; padding: 4px;">5.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.319e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">49</td><td style="border: 1px solid #ddd; padding: 4px;">5.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.319e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2611.7
- **Average Gradient Evaluations per Run:** 911.9
- **Average Iterations per Run:** 217.2
- **Average Time per Run:** 0.034s
- **Function Evaluations per Second:** 76231.5
- **Iterations per Second:** 6338.4
### Resource Utilization
- **Total Function Evaluations:** 52233
- **Total Gradient Evaluations:** 18238
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 2.86
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/PenaltyI_10D_alpha1e6_results.csv)
* [Convergence Plot](../plots/PenaltyI_10D_alpha1e6.png)
* [Log Scale Convergence Plot](../plots/PenaltyI_10D_alpha1e6_log.png)


---
*Detailed report for L-BFGS-Conservative on PenaltyI_10D_alpha1e6*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
