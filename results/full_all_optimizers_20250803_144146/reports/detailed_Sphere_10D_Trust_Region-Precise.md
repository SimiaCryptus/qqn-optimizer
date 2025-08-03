# Detailed Analysis: Trust Region-Precise on Sphere_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Sphere_10D
**Optimizer:** Trust Region-Precise
**Problem Family:** Sphere
**Dimension:** 10
**Success Threshold:** 5.000e-3
**Total Runs:** 20
**Successful Runs:** 13 (65.0%)

### Quick Statistics
* **Best Final Value:** 1.014565e-4
* **Worst Final Value:** 2.829484e0
* **Mean Final Value:** 5.266676e-1
* **Success Rate:** 65.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.050e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.479e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.829e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.364e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.360e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.377e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">347</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.323e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.300e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.238e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.226e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.018e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.985e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.666e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.033e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.814e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.517e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.082e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.511e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">462</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">309</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.032e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.426e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.297e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.277e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.320e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.298e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">543</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.396e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.326e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.046e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.998e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.999e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.238e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.225e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.268e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.252e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">298</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.015e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.015e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.719e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.292e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">285</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.635e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.086e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (13 out of 20)

* **Average Iterations to Convergence:** 164.6
* **Average Function Evaluations:** 496.8
* **Average Time to Convergence:** 0.003s
* **Fastest Convergence:** 141 iterations (0.003s)
* **Slowest Convergence:** 181 iterations (0.004s)

### Failed Runs (7 out of 20)

**Failure Reasons:**
- FunctionTolerance: 7 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 1.014565e-4
**Final Gradient Norm:** 2.014512e-2
**Iterations:** 171
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.057e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.502e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.166e0, 9.431e-1, 9.228e-1, 1.012e0, 8.404e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.057e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.502e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.613e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.162e0, 9.403e-1, 9.201e-1, 1.009e0, 8.379e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.051e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.483e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.641e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.159e0, 9.375e-1, 9.174e-1, 1.006e0, 8.354e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.044e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.463e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.670e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.156e0, 9.347e-1, 9.146e-1, 1.003e0, 8.329e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.038e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.444e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.699e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.152e0, 9.319e-1, 9.119e-1, 1.000e0, 8.304e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">167</td><td style="border: 1px solid #ddd; padding: 4px;">1.925e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.774e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.123e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.318e-1, 1.066e-1, 1.043e-1, 1.144e-1, 9.500e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">168</td><td style="border: 1px solid #ddd; padding: 4px;">1.350e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.350e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.504e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.013e-1, 8.194e-2, 8.018e-2, 8.793e-2, 7.302e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">169</td><td style="border: 1px solid #ddd; padding: 4px;">7.978e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.649e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.106e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.162e-2, 4.984e-2, 4.877e-2, 5.348e-2, 4.442e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">170</td><td style="border: 1px solid #ddd; padding: 4px;">2.952e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.436e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.819e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.612e-3, -2.922e-3, -2.859e-3, -3.136e-3, -2.604e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">171</td><td style="border: 1px solid #ddd; padding: 4px;">1.015e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.015e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.612e-3, -2.922e-3, -2.859e-3, -3.136e-3, -2.604e-3, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 504.0
- **Average Gradient Evaluations per Run:** 336.6
- **Average Iterations per Run:** 167.0
- **Average Time per Run:** 0.003s
- **Function Evaluations per Second:** 145124.1
- **Iterations per Second:** 48086.7
### Resource Utilization
- **Total Function Evaluations:** 10080
- **Total Gradient Evaluations:** 6733
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Sphere_10D_results.csv)
* [Convergence Plot](../plots/Sphere_10D.png)
* [Log Scale Convergence Plot](../plots/Sphere_10D_log.png)


---
*Detailed report for Trust Region-Precise on Sphere_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
