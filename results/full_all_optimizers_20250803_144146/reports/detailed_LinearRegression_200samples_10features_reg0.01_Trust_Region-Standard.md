# Detailed Analysis: Trust Region-Standard on LinearRegression_200samples_10features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LinearRegression_200samples_10features_reg0.01
**Optimizer:** Trust Region-Standard
**Problem Family:** Regression
**Dimension:** 10
**Success Threshold:** 4.820e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 7.133030e-1
* **Worst Final Value:** 1.232830e4
* **Mean Final Value:** 2.560395e3
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.566e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.001e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">260</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.558e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.719e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">260</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.514e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.667e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">252</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.440e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.592e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">393</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.190</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.233e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.699e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">372</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.166e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.570e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.262e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.478e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.273e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.692e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">375</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.133e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.224e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.702e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.928e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">238</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.271e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.518e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">230</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.239e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.631e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">252</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.130e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.555e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">372</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.647e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.827e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">256</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.527e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.320e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">256</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.912e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.338e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">372</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.578e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.726e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">246</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.223e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.688e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">375</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.193e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.337e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">362</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">242</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.569e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.726e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 16 runs
- GradientTolerance: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 9)
**Final Value:** 7.133030e-1
**Final Gradient Norm:** 9.223676e-1
**Iterations:** 121
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.192e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.466e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.173e-2, 1.094e-1, -9.921e-3, -3.574e-2, -1.553e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.192e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.466e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.054e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.760e-2, 1.125e-1, -1.018e-2, -2.970e-2, -1.485e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.182e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.456e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.072e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.346e-2, 1.155e-1, -1.042e-2, -2.362e-2, -1.416e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.172e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.445e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.091e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.930e-2, 1.186e-1, -1.063e-2, -1.752e-2, -1.347e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.162e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.434e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.109e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.514e-2, 1.217e-1, -1.082e-2, -1.138e-2, -1.278e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">116</td><td style="border: 1px solid #ddd; padding: 4px;">4.132e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.914e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.555e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.443e-1, 8.517e-1, 8.384e-1, 1.484e0, 1.772e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">117</td><td style="border: 1px solid #ddd; padding: 4px;">3.204e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.352e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.983e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.473e-1, 8.805e-1, 9.270e-1, 1.559e0, 1.877e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">118</td><td style="border: 1px solid #ddd; padding: 4px;">2.300e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.710e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.690e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.471e-1, 9.155e-1, 1.048e0, 1.656e0, 2.015e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">119</td><td style="border: 1px solid #ddd; padding: 4px;">1.444e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.939e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.156e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.378e-1, 9.623e-1, 1.240e0, 1.803e0, 2.227e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">120</td><td style="border: 1px solid #ddd; padding: 4px;">7.133e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.224e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.378e-1, 9.623e-1, 1.240e0, 1.803e0, 2.227e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 372.2
- **Average Gradient Evaluations per Run:** 248.3
- **Average Iterations per Run:** 123.2
- **Average Time per Run:** 0.178s
- **Function Evaluations per Second:** 2092.5
- **Iterations per Second:** 692.3
### Resource Utilization
- **Total Function Evaluations:** 7445
- **Total Gradient Evaluations:** 4966
- **Total Computation Time:** 3.6s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/LinearRegression_200samples_10features_reg0.01_results.csv)
* [Convergence Plot](../plots/LinearRegression_200samples_10features_reg0.01.png)
* [Log Scale Convergence Plot](../plots/LinearRegression_200samples_10features_reg0.01_log.png)


---
*Detailed report for Trust Region-Standard on LinearRegression_200samples_10features_reg0.01*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
