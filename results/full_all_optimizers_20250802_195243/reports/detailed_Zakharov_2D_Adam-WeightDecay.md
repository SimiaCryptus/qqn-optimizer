# Detailed Analysis: Adam-WeightDecay on Zakharov_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_2D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Zakharov
**Dimension:** 2
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.740059e-9
* **Worst Final Value:** 9.969608e-9
* **Mean Final Value:** 9.882503e-9
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.966e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.025e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2219</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2219</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.903e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.209e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2069</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2069</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.913e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.121e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.784e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.992e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">832</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1667</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1667</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.950e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.204e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1098</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.746e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.007e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.896e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.127e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">980</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.940e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.180e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.946e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.024e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.740e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.233e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.862e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.233e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2255</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2255</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.823e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.109e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.970e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.124e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.967e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.114e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1789</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1789</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.943e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.119e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">997</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1997</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1997</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.808e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.096e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1701</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1701</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.880e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.249e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.858e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.158e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1093</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.832e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.006e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.924e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.142e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 1018.8
* **Average Function Evaluations:** 2040.6
* **Average Time to Convergence:** 0.043s
* **Fastest Convergence:** 807 iterations (0.035s)
* **Slowest Convergence:** 1093 iterations (0.047s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 9.740059e-9
**Final Gradient Norm:** 2.232632e-4
**Iterations:** 1121
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.102e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.440e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.711e-1, 1.155e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.102e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.440e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.681e-1, 1.152e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.092e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.423e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.651e-1, 1.149e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.083e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.406e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.621e-1, 1.146e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.073e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.389e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.591e-1, 1.143e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1117</td><td style="border: 1px solid #ddd; padding: 4px;">1.090e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.362e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.662e-5, 6.959e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1118</td><td style="border: 1px solid #ddd; padding: 4px;">1.060e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.329e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.569e-5, 6.862e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1119</td><td style="border: 1px solid #ddd; padding: 4px;">1.030e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.296e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.478e-5, 6.766e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1120</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.264e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.387e-5, 6.671e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1121</td><td style="border: 1px solid #ddd; padding: 4px;">9.740e-9</td><td style="border: 1px solid #ddd; padding: 4px;">2.233e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.387e-5, 6.671e-5]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2040.6
- **Average Gradient Evaluations per Run:** 2040.6
- **Average Iterations per Run:** 1018.8
- **Average Time per Run:** 0.043s
- **Function Evaluations per Second:** 47287.3
- **Iterations per Second:** 23608.9
### Resource Utilization
- **Total Function Evaluations:** 40812
- **Total Gradient Evaluations:** 40812
- **Total Computation Time:** 0.9s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Zakharov_2D_results.csv)
* [Convergence Plot](../plots/Zakharov_2D.png)
* [Log Scale Convergence Plot](../plots/Zakharov_2D_log.png)


---
*Detailed report for Adam-WeightDecay on Zakharov_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
