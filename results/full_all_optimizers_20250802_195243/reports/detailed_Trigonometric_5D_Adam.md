# Detailed Analysis: Adam on Trigonometric_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_5D
**Optimizer:** Adam
**Problem Family:** Trigonometric
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 17 (85.0%)

### Quick Statistics
* **Best Final Value:** 8.420242e-7
* **Worst Final Value:** 8.542889e-2
* **Mean Final Value:** 1.124796e-2
* **Success Rate:** 85.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.781e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.974e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.483e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.057e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">263</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.872e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.878e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1066</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.420e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.002e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">308</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">619</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">619</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.840e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.854e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.707e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.933e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.828e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.812e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.910e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.902e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.705e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.966e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">663</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.652e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.961e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">733</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">733</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.890e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.975e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">441</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.543e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.125e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.847e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.770e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.794e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.765e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.896e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.878e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.677e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.547e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.835e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.785e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.804e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.008e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1773</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1773</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.418e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.431e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">742</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.533e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.296e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (17 out of 20)

* **Average Iterations to Convergence:** 846.0
* **Average Function Evaluations:** 1695.0
* **Average Time to Convergence:** 0.037s
* **Fastest Convergence:** 263 iterations (0.012s)
* **Slowest Convergence:** 1207 iterations (0.053s)

### Failed Runs (3 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 2 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 8.420242e-7
**Final Gradient Norm:** 7.001825e-3
**Iterations:** 308
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.057e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.478e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.552e-2, 6.610e-2, 5.454e-2, 2.876e-1, 1.032e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.057e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.478e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.652e-2, 6.510e-2, 5.354e-2, 2.866e-1, 1.022e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.048e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.464e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.752e-2, 6.410e-2, 5.254e-2, 2.856e-1, 1.012e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.040e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.450e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.852e-2, 6.310e-2, 5.154e-2, 2.846e-1, 1.002e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.032e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.436e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.952e-2, 6.210e-2, 5.054e-2, 2.836e-1, 9.920e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">304</td><td style="border: 1px solid #ddd; padding: 4px;">3.577e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.483e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.594e-4, 5.303e-8, 8.237e-8, 3.943e-4, -1.545e-6]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">305</td><td style="border: 1px solid #ddd; padding: 4px;">2.615e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.263e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.355e-4, 3.363e-8, 6.676e-8, 3.307e-4, -1.270e-6]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">306</td><td style="border: 1px solid #ddd; padding: 4px;">1.861e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.060e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.102e-4, 2.041e-8, 4.980e-8, 2.721e-4, -9.959e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">307</td><td style="border: 1px solid #ddd; padding: 4px;">1.280e-6</td><td style="border: 1px solid #ddd; padding: 4px;">8.723e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.840e-4, 1.283e-8, 3.242e-8, 2.182e-4, -7.290e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">308</td><td style="border: 1px solid #ddd; padding: 4px;">8.420e-7</td><td style="border: 1px solid #ddd; padding: 4px;">7.002e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.840e-4, 1.283e-8, 3.242e-8, 2.182e-4, -7.290e-7]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1728.1
- **Average Gradient Evaluations per Run:** 1728.0
- **Average Iterations per Run:** 862.5
- **Average Time per Run:** 0.038s
- **Function Evaluations per Second:** 45952.1
- **Iterations per Second:** 22934.8
### Resource Utilization
- **Total Function Evaluations:** 34562
- **Total Gradient Evaluations:** 34561
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 12)**
- Final Value: 8.542889e-2
- Final Gradient Norm: 6.124956e-3
- Iterations: 1249
- Function Evaluations: 2502
- Reason: MaxFunctionEvaluations

**Run 2 (ID: 19)**
- Final Value: 5.418191e-2
- Final Gradient Norm: 7.430679e-2
- Iterations: 370
- Function Evaluations: 743
- Reason: FunctionTolerance

**Run 3 (ID: 20)**
- Final Value: 8.533184e-2
- Final Gradient Norm: 1.296138e-3
- Iterations: 1249
- Function Evaluations: 2502
- Reason: MaxFunctionEvaluations



## Data Files
* [Raw CSV Data](../data/problems/Trigonometric_5D_results.csv)
* [Convergence Plot](../plots/Trigonometric_5D.png)
* [Log Scale Convergence Plot](../plots/Trigonometric_5D_log.png)


---
*Detailed report for Adam on Trigonometric_5D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
