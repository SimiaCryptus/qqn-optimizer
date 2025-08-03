# Detailed Analysis: L-BFGS-Conservative on LinearRegression_100samples_5features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LinearRegression_100samples_5features_reg0.01
**Optimizer:** L-BFGS-Conservative
**Problem Family:** Regression
**Dimension:** 5
**Success Threshold:** 7.150e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 7.154664e-2
* **Worst Final Value:** 7.154664e-2
* **Mean Final Value:** 7.154664e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.317e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">830</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.034e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">973</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">778</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.066e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.377</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.271e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1078</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.889e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">833</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.635e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1923</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.579</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.735e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">422</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.861e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">205</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">822</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.312</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.247e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">225</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">902</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.354e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1754</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.680</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.576e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1483</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.319e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">276</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.761e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.264e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">512</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.806</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.923e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">734</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.529e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1068</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.180e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.378e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">903</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">722</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.744e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">300</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.715e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 7.154664e-2
**Final Gradient Norm:** 3.888867e-5
**Iterations:** 166
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.667e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.373e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.568e-2, -1.876e-1, 5.597e-2, 1.560e-1, 9.427e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.667e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.373e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.497e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.261e-3, -1.408e-1, 9.275e-2, 2.309e-1, 1.943e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.538e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.993e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.564e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.008e-2, -9.385e-2, 1.304e-1, 3.061e-1, 2.943e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.414e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.614e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.617e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[4.990e-2, -4.248e-2, 1.924e-1, 3.867e-1, 3.943e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.287e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.215e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.735e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[6.666e-2, 1.343e-2, 2.801e-1, 4.726e-1, 4.943e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">161</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.282e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">162</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.200e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">163</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.120e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">164</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.042e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">165</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.964e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1306.5
- **Average Gradient Evaluations per Run:** 1044.8
- **Average Iterations per Run:** 260.7
- **Average Time per Run:** 0.407s
- **Function Evaluations per Second:** 3208.5
- **Iterations per Second:** 640.2
### Resource Utilization
- **Total Function Evaluations:** 26130
- **Total Gradient Evaluations:** 20896
- **Total Computation Time:** 8.1s
- **Function/Gradient Ratio:** 1.25
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/LinearRegression_100samples_5features_reg0.01_results.csv)
* [Convergence Plot](../plots/LinearRegression_100samples_5features_reg0.01.png)
* [Log Scale Convergence Plot](../plots/LinearRegression_100samples_5features_reg0.01_log.png)


---
*Detailed report for L-BFGS-Conservative on LinearRegression_100samples_5features_reg0.01*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
