# Detailed Analysis: Adam-Fast on Schwefel_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Schwefel_2D
**Optimizer:** Adam-Fast
**Problem Family:** Schwefel
**Dimension:** 2
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 7.106958e2
* **Worst Final Value:** 7.106958e2
* **Mean Final Value:** 7.106958e2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.212e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">987</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1977</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1976</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.268e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">983</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1969</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1968</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.265e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">983</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1969</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1968</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.213e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">987</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1977</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1976</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.235e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.241e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">986</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1975</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.238e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1973</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1972</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.209e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">986</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1975</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.239e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.241e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.236e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1973</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1972</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.214e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">986</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1975</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.212e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">986</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1975</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.239e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">986</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1975</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.238e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.264e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.265e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.265e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">983</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1969</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1968</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.240e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">982</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1967</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1966</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.107e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.213e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1973</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1972</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
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
**Final Value:** 7.106958e2
**Final Gradient Norm:** 1.208923e-3
**Iterations:** 986
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.466e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.704e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e2, 9.994e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.466e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.704e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.993e1, 9.984e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.456e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.715e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.983e1, 9.974e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.447e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.725e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.973e1, 9.964e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.437e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.735e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.963e1, 9.954e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">981</td><td style="border: 1px solid #ddd; padding: 4px;">7.107e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.311e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">982</td><td style="border: 1px solid #ddd; padding: 4px;">7.107e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.290e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">983</td><td style="border: 1px solid #ddd; padding: 4px;">7.107e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.270e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">984</td><td style="border: 1px solid #ddd; padding: 4px;">7.107e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.249e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">985</td><td style="border: 1px solid #ddd; padding: 4px;">7.107e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.229e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1972.4
- **Average Gradient Evaluations per Run:** 1971.4
- **Average Iterations per Run:** 984.7
- **Average Time per Run:** 0.037s
- **Function Evaluations per Second:** 53036.4
- **Iterations per Second:** 26477.9
### Resource Utilization
- **Total Function Evaluations:** 39448
- **Total Gradient Evaluations:** 39428
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Schwefel_2D_results.csv)
* [Convergence Plot](../plots/Schwefel_2D.png)
* [Log Scale Convergence Plot](../plots/Schwefel_2D_log.png)


---
*Detailed report for Adam-Fast on Schwefel_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
