# Detailed Analysis: Adam-WeightDecay on Zakharov_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_10D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Zakharov
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.292415e-1
* **Worst Final Value:** 2.605170e-1
* **Mean Final Value:** 1.792772e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.679e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.389e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">704</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.987e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">702</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.971e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.844e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">671</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">670</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.605e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.279e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">320</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">642</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.403e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.670e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">336</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">675</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">674</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.338e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.665e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">322</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">647</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.170e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.182e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">336</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">675</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">674</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.292e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.847e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">702</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.971e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.991e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.826e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.861e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">742</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.447e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">691</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">690</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.456e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.294e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.341e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.627e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.537e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.502e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">663</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.372e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.464e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.555e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.285e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">637</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">636</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.328e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.022e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">706</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.477e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.758e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">685</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">684</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.896e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.925e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.915e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.046e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">713</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">712</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 8)
**Final Value:** 1.292415e-1
**Final Gradient Norm:** 4.846531e0
**Iterations:** 350
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.521e5</td><td style="border: 1px solid #ddd; padding: 4px;">9.002e5</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.370e-1, 9.612e-1, 9.463e-1, 1.040e0, 9.460e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.521e5</td><td style="border: 1px solid #ddd; padding: 4px;">9.002e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.340e-1, 9.582e-1, 9.433e-1, 1.037e0, 9.430e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.445e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.924e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.310e-1, 9.552e-1, 9.403e-1, 1.034e0, 9.400e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.370e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.846e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.280e-1, 9.522e-1, 9.373e-1, 1.031e0, 9.370e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.296e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.769e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.250e-1, 9.492e-1, 9.343e-1, 1.028e0, 9.340e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">345</td><td style="border: 1px solid #ddd; padding: 4px;">8.538e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.987e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.849e-1, -7.345e-2, -8.870e-2, 2.165e-3, -8.981e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">346</td><td style="border: 1px solid #ddd; padding: 4px;">9.420e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.183e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.824e-1, -7.448e-2, -8.989e-2, 2.559e-4, -9.126e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">347</td><td style="border: 1px solid #ddd; padding: 4px;">1.060e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.205e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.794e-1, -7.497e-2, -9.055e-2, -1.169e-3, -9.221e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">348</td><td style="border: 1px solid #ddd; padding: 4px;">1.172e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.999e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.759e-1, -7.501e-2, -9.075e-2, -2.157e-3, -9.271e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">349</td><td style="border: 1px solid #ddd; padding: 4px;">1.253e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.548e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.500e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.720e-1, -7.466e-2, -9.057e-2, -2.748e-3, -9.281e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 683.1
- **Average Gradient Evaluations per Run:** 682.1
- **Average Iterations per Run:** 340.1
- **Average Time per Run:** 0.016s
- **Function Evaluations per Second:** 42468.6
- **Iterations per Second:** 21141.1
### Resource Utilization
- **Total Function Evaluations:** 13662
- **Total Gradient Evaluations:** 13642
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.00
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
*Detailed report for Adam-WeightDecay on Zakharov_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
