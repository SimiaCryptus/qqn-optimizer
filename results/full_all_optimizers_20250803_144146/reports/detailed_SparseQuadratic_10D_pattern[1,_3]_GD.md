# Detailed Analysis: GD on SparseQuadratic_10D_pattern[1, 3]
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseQuadratic_10D_pattern[1, 3]
**Optimizer:** GD
**Problem Family:** SparseQuadratic
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.602987e-7
* **Worst Final Value:** 9.997321e-7
* **Mean Final Value:** 9.801013e-7
* **Success Rate:** 100.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.603e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.024e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.666e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.074e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.994e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.059e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.604e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.977e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.887e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.056e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">723</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.832e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.037e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.911e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.068e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.835e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.031e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.855e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.017e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">729</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.888e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.069e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.911e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.078e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.632e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">723</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.848e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.069e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.775e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.044e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.997e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.104e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">713</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.859e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.068e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">711</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.786e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.022e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">713</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.755e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.027e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">719</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.763e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.095e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.619e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.010e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">723</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 357.1
* **Average Function Evaluations:** 360.1
* **Average Time to Convergence:** 0.010s
* **Fastest Convergence:** 356 iterations (0.010s)
* **Slowest Convergence:** 360 iterations (0.011s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 9.602987e-7
**Final Gradient Norm:** 2.023545e-3
**Iterations:** 356
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.084e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.088e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.122e-1, 1.166e0, 8.112e-1, 9.016e-1, 1.096e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.084e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.088e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.919e-1, 1.139e0, 7.920e-1, 8.798e-1, 1.070e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.034e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.924e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.721e-1, 1.114e0, 7.733e-1, 8.586e-1, 1.044e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.870e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.763e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.526e-1, 1.089e0, 7.549e-1, 8.379e-1, 1.020e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.418e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.606e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.337e-1, 1.065e0, 7.370e-1, 8.177e-1, 9.954e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">352</td><td style="border: 1px solid #ddd; padding: 4px;">1.141e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.208e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.661e-4, 4.551e-4, 1.375e-4, 1.359e-4, 1.552e-4, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">353</td><td style="border: 1px solid #ddd; padding: 4px;">1.093e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.161e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.582e-4, 4.454e-4, 1.339e-4, 1.325e-4, 1.508e-4, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">354</td><td style="border: 1px solid #ddd; padding: 4px;">1.047e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.114e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.505e-4, 4.358e-4, 1.305e-4, 1.293e-4, 1.466e-4, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">355</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.068e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.429e-4, 4.265e-4, 1.271e-4, 1.261e-4, 1.424e-4, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">356</td><td style="border: 1px solid #ddd; padding: 4px;">9.603e-7</td><td style="border: 1px solid #ddd; padding: 4px;">2.024e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.429e-4, 4.265e-4, 1.271e-4, 1.261e-4, 1.424e-4, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 360.1
- **Average Gradient Evaluations per Run:** 717.3
- **Average Iterations per Run:** 357.1
- **Average Time per Run:** 0.010s
- **Function Evaluations per Second:** 35276.7
- **Iterations per Second:** 34982.9
### Resource Utilization
- **Total Function Evaluations:** 7203
- **Total Gradient Evaluations:** 14346
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/SparseQuadratic_10D_pattern[1,_3]_results.csv)
* [Convergence Plot](../plots/SparseQuadratic_10D_pattern[1,_3].png)
* [Log Scale Convergence Plot](../plots/SparseQuadratic_10D_pattern[1,_3]_log.png)


---
*Detailed report for GD on SparseQuadratic_10D_pattern[1, 3]*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
