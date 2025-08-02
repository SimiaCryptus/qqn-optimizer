# Detailed Analysis: Trust Region-Precise on Beale_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Beale_2D
**Optimizer:** Trust Region-Precise
**Problem Family:** Beale
**Dimension:** 2
**Success Threshold:** 1.500e-2
**Total Runs:** 20
**Successful Runs:** 9 (45.0%)

### Quick Statistics
* **Best Final Value:** 3.198745e-3
* **Worst Final Value:** 4.169035e-2
* **Mean Final Value:** 2.166132e-2
* **Success Rate:** 45.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.704e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.273e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">211</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">424</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.637e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.471e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">311</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">935</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.931e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.658e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">977</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">652</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.062e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.361e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.925e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.361e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">267</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">803</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.886e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.345e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">310</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">932</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.374e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.058e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">567</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.202e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.928e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">505</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.199e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.491e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">208</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">627</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">419</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.469e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.316e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.782e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.266e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">965</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">644</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.587e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">989</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.125e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.702e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">420</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.222e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.065e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">552</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.108e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.478e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.117e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.807e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.169e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.453e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">336</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.293e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.357e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">208</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">627</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.077e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.210e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.609e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.239e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (9 out of 20)

* **Average Iterations to Convergence:** 221.3
* **Average Function Evaluations:** 667.0
* **Average Time to Convergence:** 0.004s
* **Fastest Convergence:** 157 iterations (0.003s)
* **Slowest Convergence:** 282 iterations (0.005s)

### Failed Runs (11 out of 20)

**Failure Reasons:**
- GradientTolerance: 10 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 9)
**Final Value:** 3.198745e-3
**Final Gradient Norm:** 1.490742e-1
**Iterations:** 208
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.277e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.262e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.382e-1, 9.408e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.277e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.262e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.763e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.384e-1, 9.380e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.270e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.248e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.780e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.386e-1, 9.353e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.264e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.234e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.798e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.388e-1, 9.325e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.258e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.220e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.816e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.390e-1, 9.297e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">204</td><td style="border: 1px solid #ddd; padding: 4px;">1.306e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.429e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.151e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.487e0, 3.402e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">205</td><td style="border: 1px solid #ddd; padding: 4px;">7.805e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.771e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.658e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.644e0, 3.945e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">206</td><td style="border: 1px solid #ddd; padding: 4px;">3.108e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.059e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.859e0, 5.214e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">207</td><td style="border: 1px solid #ddd; padding: 4px;">7.493e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.639e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.328e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.873e0, 4.699e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">208</td><td style="border: 1px solid #ddd; padding: 4px;">3.199e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.491e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.873e0, 4.699e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 743.8
- **Average Gradient Evaluations per Run:** 496.6
- **Average Iterations per Run:** 247.1
- **Average Time per Run:** 0.005s
- **Function Evaluations per Second:** 164036.8
- **Iterations per Second:** 54495.2
### Resource Utilization
- **Total Function Evaluations:** 14876
- **Total Gradient Evaluations:** 9933
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Beale_2D_results.csv)
* [Convergence Plot](../plots/Beale_2D.png)
* [Log Scale Convergence Plot](../plots/Beale_2D_log.png)


---
*Detailed report for Trust Region-Precise on Beale_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
