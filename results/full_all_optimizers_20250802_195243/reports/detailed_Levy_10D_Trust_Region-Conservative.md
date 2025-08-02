# Detailed Analysis: Trust Region-Conservative on Levy_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_10D
**Optimizer:** Trust Region-Conservative
**Problem Family:** Levy
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 9.376477e-3
* **Worst Final Value:** 1.682371e-2
* **Mean Final Value:** 1.209555e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.335e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.837e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">684</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.094e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.342e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.006e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.919e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">692</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2078</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.014e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.817e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">612</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1838</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1226</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.110e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.243e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1865</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.810e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">661</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.230e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.033e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">626</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1880</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.072e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.219e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.120e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.437e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">645</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.285e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.053e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">678</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.234e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.998e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1408</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.376e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.474e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">708</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.118e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.298e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">606</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1214</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.054e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.283e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">686</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.631e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.513e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">616</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1234</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.265e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.924e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">637</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1913</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1276</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.682e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.724e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.125e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.602e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.666e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.960e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2099</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.843e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.685e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1300</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 12)
**Final Value:** 9.376477e-3
**Final Gradient Norm:** 6.473559e-2
**Iterations:** 708
**Convergence Reason:** GradientTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.060e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.582e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.901e0, 2.142e0, 2.131e0, 2.058e0, 2.096e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.060e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.582e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.792e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.899e0, 2.141e0, 2.130e0, 2.057e0, 2.095e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">7.050e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.583e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.791e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.898e0, 2.140e0, 2.129e0, 2.056e0, 2.094e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">7.040e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.584e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.791e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.896e0, 2.140e0, 2.128e0, 2.055e0, 2.093e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">7.030e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.584e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.790e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.895e0, 2.139e0, 2.128e0, 2.054e0, 2.093e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">703</td><td style="border: 1px solid #ddd; padding: 4px;">4.389e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.129e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.698e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.041e0, 1.040e0, 1.038e0, 1.039e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">704</td><td style="border: 1px solid #ddd; padding: 4px;">3.475e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.771e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.646e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.027e0, 1.027e0, 1.025e0, 1.026e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">705</td><td style="border: 1px solid #ddd; padding: 4px;">2.586e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.395e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.168e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.999e-1, 1.013e0, 1.012e0, 1.012e0, 1.012e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">706</td><td style="border: 1px solid #ddd; padding: 4px;">1.734e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.011e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.892e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">707</td><td style="border: 1px solid #ddd; padding: 4px;">9.376e-3</td><td style="border: 1px solid #ddd; padding: 4px;">6.474e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 1.000e0, 1.000e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1976.9
- **Average Gradient Evaluations per Run:** 1318.6
- **Average Iterations per Run:** 658.3
- **Average Time per Run:** 0.016s
- **Function Evaluations per Second:** 122789.7
- **Iterations per Second:** 40888.5
### Resource Utilization
- **Total Function Evaluations:** 39538
- **Total Gradient Evaluations:** 26372
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Levy_10D_results.csv)
* [Convergence Plot](../plots/Levy_10D.png)
* [Log Scale Convergence Plot](../plots/Levy_10D_log.png)


---
*Detailed report for Trust Region-Conservative on Levy_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
