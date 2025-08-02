# Detailed Analysis: QQN-StrongWolfe on Michalewicz_10D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_10D_m10
**Optimizer:** QQN-StrongWolfe
**Problem Family:** Michalewicz
**Dimension:** 10
**Success Threshold:** -6.260e0
**Total Runs:** 20
**Successful Runs:** 1 (5.0%)

### Quick Statistics
* **Best Final Value:** -6.269151e0
* **Worst Final Value:** -2.129484e0
* **Mean Final Value:** -4.713662e0
* **Success Rate:** 5.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.189e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.672e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2728</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2310</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.129e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.637e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.237e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.138e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1504</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.613e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.342e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">53</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.377e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.816e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2072</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.789e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">235</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.208e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.061e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">922</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.757e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.001e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">304</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.653e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.933e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.404e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.357e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">489</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">388</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.260e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.372e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.937e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.246e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2237</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.281e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.449e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2840</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.269e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.002e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.260e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.089e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">85</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.462e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.318e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2896</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.483e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1544</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.716e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.730e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.260e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.086e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">954</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.658e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.979e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1981</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (1 out of 20)

* **Average Iterations to Convergence:** 74.0
* **Average Function Evaluations:** 1546.0
* **Average Time to Convergence:** 0.051s
* **Fastest Convergence:** 74 iterations (0.051s)
* **Slowest Convergence:** 74 iterations (0.051s)

### Failed Runs (19 out of 20)

**Failure Reasons:**
- GradientTolerance: 2 runs
- FunctionTolerance: 8 runs
- MaxFunctionEvaluations: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** -6.269151e0
**Final Gradient Norm:** 4.002461e0
**Iterations:** 74
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.032e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.237e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 9.450e-1, 6.444e-1, 7.846e-1, 6.908e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.032e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.237e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.211e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 9.450e-1, 6.444e-1, 7.849e-1, 6.910e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-2.546e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.081e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 9.450e-1, 6.444e-1, 7.852e-1, 6.911e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-2.753e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.850e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 9.450e-1, 6.444e-1, 7.852e-1, 6.912e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-2.865e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.430e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 9.450e-1, 6.444e-1, 7.853e-1, 6.912e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">70</td><td style="border: 1px solid #ddd; padding: 4px;">-5.776e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.046e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 1.564e0, 6.448e-1, 1.069e0, 1.042e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">71</td><td style="border: 1px solid #ddd; padding: 4px;">-5.832e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.163e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 1.568e0, 6.448e-1, 1.110e0, 9.903e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">72</td><td style="border: 1px solid #ddd; padding: 4px;">-6.240e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.342e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 1.570e0, 6.448e-1, 1.114e0, 9.986e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">73</td><td style="border: 1px solid #ddd; padding: 4px;">-6.257e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.472e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 1.570e0, 6.448e-1, 1.114e0, 9.960e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">74</td><td style="border: 1px solid #ddd; padding: 4px;">-6.269e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.002e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[6.762e-1, 1.570e0, 6.448e-1, 1.114e0, 9.960e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1916.5
- **Average Gradient Evaluations per Run:** 1461.9
- **Average Iterations per Run:** 84.5
- **Average Time per Run:** 0.063s
- **Function Evaluations per Second:** 30208.6
- **Iterations per Second:** 1331.1
### Resource Utilization
- **Total Function Evaluations:** 38330
- **Total Gradient Evaluations:** 29238
- **Total Computation Time:** 1.3s
- **Function/Gradient Ratio:** 1.31
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
*Detailed report for QQN-StrongWolfe on Michalewicz_10D_m10*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
