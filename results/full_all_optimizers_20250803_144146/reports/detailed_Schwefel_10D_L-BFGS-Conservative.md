# Detailed Analysis: L-BFGS-Conservative on Schwefel_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Schwefel_10D
**Optimizer:** L-BFGS-Conservative
**Problem Family:** Schwefel
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.553479e3
* **Worst Final Value:** 3.553479e3
* **Mean Final Value:** 3.553479e3
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.200e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.655e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1758</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.427e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1762</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.895e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">432</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1730</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.951e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">437</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1750</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.349e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.695e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2205</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1762</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.264e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">437</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2190</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1750</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.576e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.207e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.998e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1704</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.751e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">432</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1730</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.211e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">441</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.136e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.146e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2200</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1758</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.909e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1722</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.429e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1754</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.214e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2205</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1762</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.309e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2214</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.553e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.134e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">434</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1738</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
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
**Final Value:** 3.553479e3
**Final Gradient Norm:** 1.264282e-4
**Iterations:** 437
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.733e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.499e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e2, 1.000e2, 9.988e1, 1.000e2, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.733e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.499e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.104e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.992e1, 9.990e1, 9.978e1, 9.992e1, 1.000e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.728e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.501e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.101e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.982e1, 9.980e1, 9.968e1, 9.982e1, 9.991e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">4.724e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.504e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.098e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.972e1, 9.970e1, 9.958e1, 9.972e1, 9.981e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.719e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.506e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.095e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.962e1, 9.960e1, 9.948e1, 9.962e1, 9.971e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">432</td><td style="border: 1px solid #ddd; padding: 4px;">3.553e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.080e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1, 6.555e1, 6.555e1, 6.555e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">433</td><td style="border: 1px solid #ddd; padding: 4px;">3.553e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.623e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1, 6.555e1, 6.555e1, 6.555e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">434</td><td style="border: 1px solid #ddd; padding: 4px;">3.553e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.551e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1, 6.555e1, 6.555e1, 6.555e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">435</td><td style="border: 1px solid #ddd; padding: 4px;">3.553e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.512e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1, 6.555e1, 6.555e1, 6.555e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">436</td><td style="border: 1px solid #ddd; padding: 4px;">3.553e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.323e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.555e1, 6.555e1, 6.555e1, 6.555e1, 6.555e1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2188.9
- **Average Gradient Evaluations per Run:** 1749.7
- **Average Iterations per Run:** 436.9
- **Average Time per Run:** 0.063s
- **Function Evaluations per Second:** 34942.0
- **Iterations per Second:** 6975.2
### Resource Utilization
- **Total Function Evaluations:** 43778
- **Total Gradient Evaluations:** 34994
- **Total Computation Time:** 1.3s
- **Function/Gradient Ratio:** 1.25
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Schwefel_10D_results.csv)
* [Convergence Plot](../plots/Schwefel_10D.png)
* [Log Scale Convergence Plot](../plots/Schwefel_10D_log.png)


---
*Detailed report for L-BFGS-Conservative on Schwefel_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
