# Detailed Analysis: Adam-AMSGrad on Rosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rosenbrock_10D
**Optimizer:** Adam-AMSGrad
**Problem Family:** Rosenbrock
**Dimension:** 10
**Success Threshold:** 9.700e0
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.663794e0
* **Worst Final Value:** 9.697957e0
* **Mean Final Value:** 9.688023e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.688e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.309e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.681e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.128e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.698e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.540e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.698e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.967e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.681e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.063e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.678e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.217e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.680e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.171e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.679e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.212e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1232</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.698e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.034e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2345</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.683e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.356e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.695e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.166e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.691e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.107e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.693e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.027e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2501</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2501</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.664e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.764e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1216</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.693e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.147e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.689e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.180e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.693e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.396e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.691e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.312e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.922e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.695e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.088e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1228</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.066</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 1175.0
* **Average Function Evaluations:** 2353.0
* **Average Time to Convergence:** 0.059s
* **Fastest Convergence:** 1083 iterations (0.053s)
* **Slowest Convergence:** 1223 iterations (0.070s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 9.663794e0
**Final Gradient Norm:** 1.763843e1
**Iterations:** 1216
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.200e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.209e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.054e0, 9.107e-1, -1.370e0, 9.101e-1, -1.297e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.200e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.209e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.053e0, 9.097e-1, -1.369e0, 9.091e-1, -1.296e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.194e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.204e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.052e0, 9.087e-1, -1.368e0, 9.081e-1, -1.295e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.188e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.199e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.051e0, 9.077e-1, -1.367e0, 9.071e-1, -1.294e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.182e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.194e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.050e0, 9.067e-1, -1.366e0, 9.061e-1, -1.293e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1212</td><td style="border: 1px solid #ddd; padding: 4px;">9.835e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.916e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.542e-3, 1.469e-2, -5.659e-2, 1.576e-2, -1.874e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1213</td><td style="border: 1px solid #ddd; padding: 4px;">9.791e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.878e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.082e-3, 1.436e-2, -5.535e-2, 1.553e-2, -1.788e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1214</td><td style="border: 1px solid #ddd; padding: 4px;">9.748e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.840e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.267e-2, 1.403e-2, -5.410e-2, 1.530e-2, -1.703e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1215</td><td style="border: 1px solid #ddd; padding: 4px;">9.706e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.802e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.630e-2, 1.371e-2, -5.286e-2, 1.508e-2, -1.619e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1216</td><td style="border: 1px solid #ddd; padding: 4px;">9.664e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.764e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.630e-2, 1.371e-2, -5.286e-2, 1.508e-2, -1.619e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2353.0
- **Average Gradient Evaluations per Run:** 2353.0
- **Average Iterations per Run:** 1175.0
- **Average Time per Run:** 0.059s
- **Function Evaluations per Second:** 39838.1
- **Iterations per Second:** 19893.7
### Resource Utilization
- **Total Function Evaluations:** 47060
- **Total Gradient Evaluations:** 47060
- **Total Computation Time:** 1.2s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Rosenbrock_10D_results.csv)
* [Convergence Plot](../plots/Rosenbrock_10D.png)
* [Log Scale Convergence Plot](../plots/Rosenbrock_10D_log.png)


---
*Detailed report for Adam-AMSGrad on Rosenbrock_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
