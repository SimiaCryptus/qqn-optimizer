# Detailed Analysis: Adam-WeightDecay on Levy_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_5D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Levy
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 12 (60.0%)

### Quick Statistics
* **Best Final Value:** 9.654780e-7
* **Worst Final Value:** 3.660098e-6
* **Mean Final Value:** 1.433948e-6
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
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.018e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.179e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.286e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.767e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2388</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.892e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.225e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.963e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.436e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.655e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.463e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.045e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.385e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.620e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.890e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.839e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.403e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.895e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.535e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">934</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1871</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1870</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.815e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.413e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1072</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.881e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.522e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.941e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.376e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.967e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.280e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.220e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.725e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2336</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.881e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.293e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">973</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.074e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.625e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.961e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.538e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.827e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.123e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.982e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.557e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.660e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.578e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (12 out of 20)

* **Average Iterations to Convergence:** 1029.8
* **Average Function Evaluations:** 2062.5
* **Average Time to Convergence:** 0.049s
* **Fastest Convergence:** 971 iterations (0.044s)
* **Slowest Convergence:** 973 iterations (0.054s)

### Failed Runs (8 out of 20)

**Failure Reasons:**
- FunctionTolerance: 8 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 9.654780e-7
**Final Gradient Norm:** 1.463113e-3
**Iterations:** 991
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.339e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.691e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.947e0, 2.115e0, 2.007e0, 2.046e0, 1.972e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.339e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.691e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.944e0, 2.112e0, 2.004e0, 2.043e0, 1.969e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.323e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.692e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.941e0, 2.109e0, 2.001e0, 2.040e0, 1.966e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.306e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.693e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.938e0, 2.106e0, 1.998e0, 2.037e0, 1.963e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.290e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.695e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.935e0, 2.103e0, 1.995e0, 2.034e0, 1.960e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">987</td><td style="border: 1px solid #ddd; padding: 4px;">1.122e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.543e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e-1, 1.001e0, 1.000e0, 1.000e0, 1.001e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">988</td><td style="border: 1px solid #ddd; padding: 4px;">1.080e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.522e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e-1, 1.001e0, 1.000e0, 1.000e0, 1.001e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">989</td><td style="border: 1px solid #ddd; padding: 4px;">1.041e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.502e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e-1, 1.001e0, 1.000e0, 1.000e0, 1.001e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">990</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.482e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e-1, 1.001e0, 1.000e0, 1.000e0, 1.001e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">991</td><td style="border: 1px solid #ddd; padding: 4px;">9.655e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.463e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e-1, 1.001e0, 1.000e0, 1.000e0, 1.001e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2087.2
- **Average Gradient Evaluations per Run:** 2086.8
- **Average Iterations per Run:** 1042.1
- **Average Time per Run:** 0.050s
- **Function Evaluations per Second:** 42094.0
- **Iterations per Second:** 21016.7
### Resource Utilization
- **Total Function Evaluations:** 41744
- **Total Gradient Evaluations:** 41736
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Levy_5D_results.csv)
* [Convergence Plot](../plots/Levy_5D.png)
* [Log Scale Convergence Plot](../plots/Levy_5D_log.png)


---
*Detailed report for Adam-WeightDecay on Levy_5D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
