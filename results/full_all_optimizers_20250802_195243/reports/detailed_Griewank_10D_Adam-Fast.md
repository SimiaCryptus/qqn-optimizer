# Detailed Analysis: Adam-Fast on Griewank_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Griewank_10D
**Optimizer:** Adam-Fast
**Problem Family:** Griewank
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.888780e0
* **Worst Final Value:** 1.503926e1
* **Mean Final Value:** 1.168763e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.215e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.134e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">977</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">976</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.502e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.746e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">759</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">758</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.889e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.765e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">897</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1797</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1796</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.958e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.154e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.215e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.597e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">479</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">961</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">960</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.251e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.934e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">713</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">712</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.215e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.036e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.502e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.406e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">784</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.502e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.194e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">784</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.502e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.016e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">783</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">782</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.242e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.087e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">813</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">812</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.917e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.282e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">557</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.649e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">310</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.251e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.785e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">727</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">726</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.135e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">300</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">603</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.242e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.775e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.610e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.647e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.882e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">813</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.917e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.146e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.917e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.131e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">585</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 2.888780e0
**Final Gradient Norm:** 1.764865e-1
**Iterations:** 897
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.597e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.501e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e2, 9.991e1, 9.982e1, 1.001e2, 9.981e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.597e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.501e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.993e1, 1.000e2, 9.972e1, 1.000e2, 9.971e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.593e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.502e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.983e1, 1.001e2, 9.962e1, 9.991e1, 9.961e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.588e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.973e1, 1.002e2, 9.952e1, 9.981e1, 9.951e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.584e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.462e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.963e1, 1.003e2, 9.942e1, 9.971e1, 9.941e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">892</td><td style="border: 1px solid #ddd; padding: 4px;">2.854e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.873e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.441e1, 3.565e1, 3.267e1, 3.813e1, 3.552e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">893</td><td style="border: 1px solid #ddd; padding: 4px;">2.873e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.145e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.440e1, 3.559e1, 3.273e1, 3.812e1, 3.558e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">894</td><td style="border: 1px solid #ddd; padding: 4px;">2.886e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.201e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.443e1, 3.553e1, 3.278e1, 3.809e1, 3.560e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">895</td><td style="border: 1px solid #ddd; padding: 4px;">2.892e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.062e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.449e1, 3.546e1, 3.280e1, 3.803e1, 3.559e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">896</td><td style="border: 1px solid #ddd; padding: 4px;">2.892e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.867e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.455e1, 3.542e1, 3.280e1, 3.795e1, 3.554e1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 950.2
- **Average Gradient Evaluations per Run:** 949.2
- **Average Iterations per Run:** 473.6
- **Average Time per Run:** 0.022s
- **Function Evaluations per Second:** 43159.2
- **Iterations per Second:** 21511.5
### Resource Utilization
- **Total Function Evaluations:** 19004
- **Total Gradient Evaluations:** 18984
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Griewank_10D_results.csv)
* [Convergence Plot](../plots/Griewank_10D.png)
* [Log Scale Convergence Plot](../plots/Griewank_10D_log.png)


---
*Detailed report for Adam-Fast on Griewank_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
