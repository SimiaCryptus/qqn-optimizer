# Detailed Analysis: L-BFGS-Limited on Michalewicz_10D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_10D_m10
**Optimizer:** L-BFGS-Limited
**Problem Family:** Michalewicz
**Dimension:** 10
**Success Threshold:** -6.260e0
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -5.591623e0
* **Worst Final Value:** -6.440008e-1
* **Mean Final Value:** -3.523592e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.627e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.003e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.463e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.015e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3897</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.462e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.881e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.404e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.479e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.406e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.456e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3956</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.565e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.104e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.674e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.242e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.446e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.839e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.405e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.231e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">342</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3978</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.440e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.952e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.373e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.032e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1504</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.462e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.618e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3961</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.553e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.277e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.950e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.860e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">307</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">926</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.815e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">956</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.592e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.410e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">309</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">932</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.778e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.757e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.244e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.290e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3940</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.880e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">268</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.307e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.952e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">280</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 14 runs
- FunctionTolerance: 6 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 16)
**Final Value:** -5.591623e0
**Final Gradient Norm:** 7.410013e-3
**Iterations:** 309
**Convergence Reason:** MaxFunctionEvaluations

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.685e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.972e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.635e-1, 8.466e-1, 8.144e-1, 7.350e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.685e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.972e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.635e-1, 8.467e-1, 8.157e-1, 7.367e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-1.772e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.149e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.635e-1, 8.467e-1, 8.163e-1, 7.375e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-2.151e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.394e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.635e-1, 8.467e-1, 8.170e-1, 7.385e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-2.162e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.071e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.635e-1, 8.467e-1, 8.177e-1, 7.394e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">305</td><td style="border: 1px solid #ddd; padding: 4px;">-5.592e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.248e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.638e-1, 8.507e-1, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">306</td><td style="border: 1px solid #ddd; padding: 4px;">-5.592e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.331e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.638e-1, 8.507e-1, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">307</td><td style="border: 1px solid #ddd; padding: 4px;">-5.592e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.138e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.638e-1, 8.507e-1, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">308</td><td style="border: 1px solid #ddd; padding: 4px;">-5.592e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.291e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.638e-1, 8.507e-1, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">309</td><td style="border: 1px solid #ddd; padding: 4px;">-5.592e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.410e-3</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[9.015e-1, 9.638e-1, 8.507e-1, 1.114e0, 9.967e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3009.3
- **Average Gradient Evaluations per Run:** 752.5
- **Average Iterations per Run:** 249.4
- **Average Time per Run:** 0.038s
- **Function Evaluations per Second:** 79265.9
- **Iterations per Second:** 6570.5
### Resource Utilization
- **Total Function Evaluations:** 60187
- **Total Gradient Evaluations:** 15049
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 4.00
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
*Detailed report for L-BFGS-Limited on Michalewicz_10D_m10*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
