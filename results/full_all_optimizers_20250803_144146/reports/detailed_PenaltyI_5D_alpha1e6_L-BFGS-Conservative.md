# Detailed Analysis: L-BFGS-Conservative on PenaltyI_5D_alpha1e6
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** PenaltyI_5D_alpha1e6
**Optimizer:** L-BFGS-Conservative
**Problem Family:** PenaltyI
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.812497e0
* **Worst Final Value:** 3.032244e0
* **Mean Final Value:** 2.846235e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.829e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.403e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">197</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.812e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2964</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.813e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.955e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.833e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.206e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.839e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.950e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.812e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.514e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.862e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.945e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">263</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.831e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.852e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.812e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.065e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1957</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">833</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.812e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.578e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.828e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.342e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3924</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.844e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.439e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.817e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.479e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">839</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.903e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.269e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">749</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.848e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.511e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3870</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.032e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.549e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.837e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.716e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.905e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.409e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">211</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">875</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.834e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.856e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3810</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1195</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.819e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.149e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3947</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 13 runs
- FunctionTolerance: 7 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 9)
**Final Value:** 2.812497e0
**Final Gradient Norm:** 7.065224e-2
**Iterations:** 199
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.451e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.175e6</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.912e-1, 4.015e-1, 4.224e-1, 6.568e-1, 3.526e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.451e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.175e6</td><td style="border: 1px solid #ddd; padding: 4px;">1.444e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.073e-1, 3.643e-1, 3.801e-1, 5.568e-1, 3.274e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.963e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.861e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.444e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.234e-1, 3.270e-1, 3.377e-1, 4.568e-1, 3.021e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.918e4</td><td style="border: 1px solid #ddd; padding: 4px;">5.972e5</td><td style="border: 1px solid #ddd; padding: 4px;">2.888e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[3.396e-1, 2.898e-1, 2.953e-1, 3.568e-1, 2.769e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.378e4</td><td style="border: 1px solid #ddd; padding: 4px;">3.084e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.444e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.557e-1, 2.525e-1, 2.529e-1, 2.568e-1, 2.517e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">194</td><td style="border: 1px solid #ddd; padding: 4px;">2.812e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.176e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">195</td><td style="border: 1px solid #ddd; padding: 4px;">2.812e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.683e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">196</td><td style="border: 1px solid #ddd; padding: 4px;">2.812e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.228e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">197</td><td style="border: 1px solid #ddd; padding: 4px;">2.812e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.809e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">198</td><td style="border: 1px solid #ddd; padding: 4px;">2.812e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.422e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1, 2.500e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3087.5
- **Average Gradient Evaluations per Run:** 889.9
- **Average Iterations per Run:** 213.4
- **Average Time per Run:** 0.033s
- **Function Evaluations per Second:** 93138.9
- **Iterations per Second:** 6437.5
### Resource Utilization
- **Total Function Evaluations:** 61750
- **Total Gradient Evaluations:** 17797
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 3.47
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/PenaltyI_5D_alpha1e6_results.csv)
* [Convergence Plot](../plots/PenaltyI_5D_alpha1e6.png)
* [Log Scale Convergence Plot](../plots/PenaltyI_5D_alpha1e6_log.png)


---
*Detailed report for L-BFGS-Conservative on PenaltyI_5D_alpha1e6*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
