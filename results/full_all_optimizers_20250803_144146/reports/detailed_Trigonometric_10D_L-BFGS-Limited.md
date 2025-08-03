# Detailed Analysis: L-BFGS-Limited on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** L-BFGS-Limited
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 9.815501e-6
* **Worst Final Value:** 8.193696e-2
* **Mean Final Value:** 7.311431e-3
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.121e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.642e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3962</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.784e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.737e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3929</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1079</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.296e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.873e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3927</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.596e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.711e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.194e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.195e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">989</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.075e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.392e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3941</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.505e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.230e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.308e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.831e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3933</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.657e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.550e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.025e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.760e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3962</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.022e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3928</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.816e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.354e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.009e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.163e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3913</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1094</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.520e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.024e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.658e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.578e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3935</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.009e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.944e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.523e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.620e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">347</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3962</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.554e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.378e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3952</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.537e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.735e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">347</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.699e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.022e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3940</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 12)
**Final Value:** 9.815501e-6
**Final Gradient Norm:** 1.353792e-2
**Iterations:** 355
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.528e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.202e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.232e-2, 1.916e-1, 1.781e-1, 3.832e-1, 4.499e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.528e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.202e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.125e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.558e-2, 2.116e-1, 1.827e-1, 3.726e-1, 4.578e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.597e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.874e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.711e-2, 2.101e-1, 1.753e-1, 3.511e-1, 4.150e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.671e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.096e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.499e-2, 2.145e-1, 1.617e-1, 3.032e-1, 3.091e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.350e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.166e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.642e-2, 2.118e-1, 1.524e-1, 2.754e-1, 2.722e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">351</td><td style="border: 1px solid #ddd; padding: 4px;">1.065e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.067e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.232e-3, 3.386e-5, 3.313e-6, 1.887e-6, 1.329e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">352</td><td style="border: 1px solid #ddd; padding: 4px;">1.039e-5</td><td style="border: 1px solid #ddd; padding: 4px;">6.704e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.163e-3, 3.237e-5, 3.130e-6, 1.769e-6, 1.238e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">353</td><td style="border: 1px solid #ddd; padding: 4px;">1.020e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.191e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.151e-3, 3.295e-5, 3.181e-6, 1.806e-6, 1.270e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">354</td><td style="border: 1px solid #ddd; padding: 4px;">9.884e-6</td><td style="border: 1px solid #ddd; padding: 4px;">6.713e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.083e-3, 3.133e-5, 2.988e-6, 1.681e-6, 1.174e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">355</td><td style="border: 1px solid #ddd; padding: 4px;">9.816e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.354e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.083e-3, 3.133e-5, 2.988e-6, 1.681e-6, 1.174e-6, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3943.6
- **Average Gradient Evaluations per Run:** 1063.8
- **Average Iterations per Run:** 352.9
- **Average Time per Run:** 0.058s
- **Function Evaluations per Second:** 67924.7
- **Iterations per Second:** 6079.2
### Resource Utilization
- **Total Function Evaluations:** 78872
- **Total Gradient Evaluations:** 21277
- **Total Computation Time:** 1.2s
- **Function/Gradient Ratio:** 3.71
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Trigonometric_10D_results.csv)
* [Convergence Plot](../plots/Trigonometric_10D.png)
* [Log Scale Convergence Plot](../plots/Trigonometric_10D_log.png)


---
*Detailed report for L-BFGS-Limited on Trigonometric_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
