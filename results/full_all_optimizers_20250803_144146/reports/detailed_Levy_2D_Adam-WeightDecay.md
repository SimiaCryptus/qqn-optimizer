# Detailed Analysis: Adam-WeightDecay on Levy_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_2D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Levy
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.670239e-7
* **Worst Final Value:** 9.978976e-7
* **Mean Final Value:** 9.829950e-7
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.904e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.098e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.799e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.064e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.762e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.652e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.913e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.144e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.915e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.030e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.881e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.083e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.751e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.617e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.908e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.012e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1072</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.886e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.789e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.859e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.079e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1226</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2455</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2455</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.798e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.068e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1231</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.979e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.769e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.723e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.593e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.670e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.964e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">981</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1965</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1965</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.900e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.058e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.759e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.335e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">911</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1825</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1825</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.764e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.080e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.804e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.255e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2411</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2411</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.815e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.124e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">940</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.808e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.080e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 1113.0
* **Average Function Evaluations:** 2229.0
* **Average Time to Convergence:** 0.048s
* **Fastest Convergence:** 911 iterations (0.039s)
* **Slowest Convergence:** 1210 iterations (0.055s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 9.670239e-7
**Final Gradient Norm:** 4.964021e-4
**Iterations:** 981
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.086e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.906e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.911e0, 1.885e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.086e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.906e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.908e0, 1.882e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.079e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.905e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.905e0, 1.879e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.073e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.903e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.902e0, 1.876e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.066e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.902e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.899e0, 1.873e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">977</td><td style="border: 1px solid #ddd; padding: 4px;">1.116e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.286e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">978</td><td style="border: 1px solid #ddd; padding: 4px;">1.077e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.200e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">979</td><td style="border: 1px solid #ddd; padding: 4px;">1.039e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.118e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">980</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.039e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">981</td><td style="border: 1px solid #ddd; padding: 4px;">9.670e-7</td><td style="border: 1px solid #ddd; padding: 4px;">4.964e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2229.0
- **Average Gradient Evaluations per Run:** 2229.0
- **Average Iterations per Run:** 1113.0
- **Average Time per Run:** 0.048s
- **Function Evaluations per Second:** 46124.5
- **Iterations per Second:** 23031.2
### Resource Utilization
- **Total Function Evaluations:** 44580
- **Total Gradient Evaluations:** 44580
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Levy_2D_results.csv)
* [Convergence Plot](../plots/Levy_2D.png)
* [Log Scale Convergence Plot](../plots/Levy_2D_log.png)


---
*Detailed report for Adam-WeightDecay on Levy_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
