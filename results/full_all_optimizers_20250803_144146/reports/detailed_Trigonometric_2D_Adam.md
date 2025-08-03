# Detailed Analysis: Adam on Trigonometric_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_2D
**Optimizer:** Adam
**Problem Family:** Trigonometric
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.403983e-7
* **Worst Final Value:** 9.978111e-7
* **Mean Final Value:** 9.771879e-7
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.510e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.898e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">323</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.620e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.920e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.715e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.940e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">424</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.694e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.935e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">235</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">235</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.859e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.988e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.585e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.913e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.954e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.988e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.743e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.945e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.978e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.993e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">567</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">567</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.862e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.982e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">279</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">561</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">561</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.641e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.925e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.871e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.053e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.936e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.002e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2327</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2327</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.901e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.120e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.838e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.965e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">680</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.642e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.960e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">375</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">753</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">753</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.849e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.880e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.892e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.875e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">603</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.404e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.662e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">509</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">509</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.943e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.076e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 633.2
* **Average Function Evaluations:** 1269.5
* **Average Time to Convergence:** 0.026s
* **Fastest Convergence:** 116 iterations (0.005s)
* **Slowest Convergence:** 1083 iterations (0.050s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 19)
**Final Value:** 9.403983e-7
**Final Gradient Norm:** 3.661903e-3
**Iterations:** 253
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.864e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.623e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.485e-1, 1.227e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.864e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.623e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.475e-1, 1.217e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">5.789e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.594e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.465e-1, 1.207e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.714e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.565e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.455e-1, 1.197e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.639e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.535e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.445e-1, 1.187e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">249</td><td style="border: 1px solid #ddd; padding: 4px;">1.332e-6</td><td style="border: 1px solid #ddd; padding: 4px;">4.338e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.304e-4, 5.094e-4]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">250</td><td style="border: 1px solid #ddd; padding: 4px;">1.222e-6</td><td style="border: 1px solid #ddd; padding: 4px;">4.159e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.083e-4, 4.885e-4]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">251</td><td style="border: 1px solid #ddd; padding: 4px;">1.120e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.987e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.873e-4, 4.684e-4]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">252</td><td style="border: 1px solid #ddd; padding: 4px;">1.027e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.822e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.671e-4, 4.490e-4]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">253</td><td style="border: 1px solid #ddd; padding: 4px;">9.404e-7</td><td style="border: 1px solid #ddd; padding: 4px;">3.662e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.671e-4, 4.490e-4]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1269.5
- **Average Gradient Evaluations per Run:** 1269.5
- **Average Iterations per Run:** 633.2
- **Average Time per Run:** 0.026s
- **Function Evaluations per Second:** 49486.2
- **Iterations per Second:** 24684.6
### Resource Utilization
- **Total Function Evaluations:** 25390
- **Total Gradient Evaluations:** 25390
- **Total Computation Time:** 0.5s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Trigonometric_2D_results.csv)
* [Convergence Plot](../plots/Trigonometric_2D.png)
* [Log Scale Convergence Plot](../plots/Trigonometric_2D_log.png)


---
*Detailed report for Adam on Trigonometric_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
