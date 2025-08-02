# Detailed Analysis: Trust Region-Conservative on Barrier_5D_mu0.1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Barrier_5D_mu0.1
**Optimizer:** Trust Region-Conservative
**Problem Family:** Barrier
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 9.993316e-1
* **Worst Final Value:** 1.182030e0
* **Mean Final Value:** 1.043387e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.034e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.915e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.035e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.994e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">790</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.317e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">422</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1269</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">847</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.351e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">782</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.352e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">371</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">744</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.080e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.665e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1311</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.273e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">692</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.062e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.521e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">972</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.171e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.023e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">792</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.002e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.420e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">469</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">940</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.000e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.024e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1280</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.160e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">944</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.301e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">736</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.999e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.690e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">865</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.137e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.390e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">954</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">636</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.182e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.051e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">371</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">744</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.318e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">393</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">788</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.156e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.494e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">954</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.993e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.594e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">931</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.001e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.184e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 13 runs
- FunctionTolerance: 3 runs
- NumericalError: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 19)
**Final Value:** 9.993316e-1
**Final Gradient Norm:** 5.594249e-2
**Iterations:** 464
**Convergence Reason:** NumericalError

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.582e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.523e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.158e0, 9.764e-1, 1.192e0, 1.095e0, 8.285e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.582e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.523e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.211e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.157e0, 9.755e-1, 1.191e0, 1.094e0, 8.278e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">5.572e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.518e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.213e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.156e0, 9.746e-1, 1.190e0, 1.093e0, 8.270e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.562e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.514e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.215e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.155e0, 9.737e-1, 1.189e0, 1.092e0, 8.262e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.552e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.509e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.218e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.154e0, 9.728e-1, 1.187e0, 1.091e0, 8.255e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">459</td><td style="border: 1px solid #ddd; padding: 4px;">1.006e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.327e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.298e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.335e-1, 2.303e-1, 2.342e-1, 2.324e-1, 2.282e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">460</td><td style="border: 1px solid #ddd; padding: 4px;">9.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.376e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.809e-1, 1.943e-1, 1.781e-1, 1.858e-1, 2.034e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">461</td><td style="border: 1px solid #ddd; padding: 4px;">1.013e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.598e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.112e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.399e-1, 2.332e-1, 2.415e-1, 2.373e-1, 2.296e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">462</td><td style="border: 1px solid #ddd; padding: 4px;">1.001e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.163e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.315e-1, 1.684e-1, 1.229e-1, 1.461e-1, 1.888e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">463</td><td style="border: 1px solid #ddd; padding: 4px;">1.067e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.016e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.775e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.294e-1, 2.190e-1, 2.347e-1, 2.233e-1, 2.187e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1250.3
- **Average Gradient Evaluations per Run:** 834.2
- **Average Iterations per Run:** 416.0
- **Average Time per Run:** 0.009s
- **Function Evaluations per Second:** 142901.6
- **Iterations per Second:** 47544.3
### Resource Utilization
- **Total Function Evaluations:** 25007
- **Total Gradient Evaluations:** 16684
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 4
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Barrier_5D_mu0.1_results.csv)
* [Convergence Plot](../plots/Barrier_5D_mu0.1.png)
* [Log Scale Convergence Plot](../plots/Barrier_5D_mu0.1_log.png)


---
*Detailed report for Trust Region-Conservative on Barrier_5D_mu0.1*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
