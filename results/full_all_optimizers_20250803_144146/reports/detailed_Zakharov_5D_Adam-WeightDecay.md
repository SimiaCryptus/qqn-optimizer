# Detailed Analysis: Adam-WeightDecay on Zakharov_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_5D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Zakharov
**Dimension:** 5
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 12 (60.0%)

### Quick Statistics
* **Best Final Value:** 9.756997e-9
* **Worst Final Value:** 2.858790e-2
* **Mean Final Value:** 8.635769e-3
* **Success Rate:** 60.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.757e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.989e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.814e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.007e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.796e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.001e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1066</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.424e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.831e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">730</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.336e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.775e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.802e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.987e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.999e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.009e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2279</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2279</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.520e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.439e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.997e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.017e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.815e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.996e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.770e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.992e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.859e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.087e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">659</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">658</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.694e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.957e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">673</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.786e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.999e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.414e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.727e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">680</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.898e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.004e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2221</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2221</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.864e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.015e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1079</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.250e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.625e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">730</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.774e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.105e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">326</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">655</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">654</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.918e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.008e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (12 out of 20)

* **Average Iterations to Convergence:** 1091.3
* **Average Function Evaluations:** 2185.7
* **Average Time to Convergence:** 0.049s
* **Fastest Convergence:** 1057 iterations (0.045s)
* **Slowest Convergence:** 1110 iterations (0.053s)

### Failed Runs (8 out of 20)

**Failure Reasons:**
- FunctionTolerance: 8 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 9.756997e-9
**Final Gradient Norm:** 1.988931e-4
**Iterations:** 1129
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.453e3</td><td style="border: 1px solid #ddd; padding: 4px;">6.649e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.175e0, 8.859e-1, 1.019e0, 1.141e0, 9.384e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.453e3</td><td style="border: 1px solid #ddd; padding: 4px;">6.649e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.172e0, 8.829e-1, 1.016e0, 1.138e0, 9.354e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.413e3</td><td style="border: 1px solid #ddd; padding: 4px;">6.591e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.169e0, 8.799e-1, 1.013e0, 1.135e0, 9.324e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.373e3</td><td style="border: 1px solid #ddd; padding: 4px;">6.533e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.166e0, 8.769e-1, 1.010e0, 1.132e0, 9.294e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.333e3</td><td style="border: 1px solid #ddd; padding: 4px;">6.475e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.163e0, 8.739e-1, 1.007e0, 1.129e0, 9.264e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1125</td><td style="border: 1px solid #ddd; padding: 4px;">1.083e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.096e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.074e-6, 5.582e-6, 1.320e-5, 7.329e-5, -7.044e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1126</td><td style="border: 1px solid #ddd; padding: 4px;">1.055e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.068e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.047e-6, 5.511e-6, 1.303e-5, 7.233e-5, -6.953e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1127</td><td style="border: 1px solid #ddd; padding: 4px;">1.028e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.042e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.021e-6, 5.441e-6, 1.287e-5, 7.139e-5, -6.863e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1128</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.015e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.995e-6, 5.372e-6, 1.271e-5, 7.046e-5, -6.774e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1129</td><td style="border: 1px solid #ddd; padding: 4px;">9.757e-9</td><td style="border: 1px solid #ddd; padding: 4px;">1.989e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.995e-6, 5.372e-6, 1.271e-5, 7.046e-5, -6.774e-5]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1586.3
- **Average Gradient Evaluations per Run:** 1585.9
- **Average Iterations per Run:** 791.6
- **Average Time per Run:** 0.036s
- **Function Evaluations per Second:** 44562.7
- **Iterations per Second:** 22239.2
### Resource Utilization
- **Total Function Evaluations:** 31726
- **Total Gradient Evaluations:** 31718
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Zakharov_5D_results.csv)
* [Convergence Plot](../plots/Zakharov_5D.png)
* [Log Scale Convergence Plot](../plots/Zakharov_5D_log.png)


---
*Detailed report for Adam-WeightDecay on Zakharov_5D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
