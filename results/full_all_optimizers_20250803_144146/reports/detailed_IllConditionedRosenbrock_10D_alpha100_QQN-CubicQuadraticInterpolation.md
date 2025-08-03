# Detailed Analysis: QQN-CubicQuadraticInterpolation on IllConditionedRosenbrock_10D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_10D_alpha100
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 15 (75.0%)

### Quick Statistics
* **Best Final Value:** 1.131821e-7
* **Worst Final Value:** 4.986511e0
* **Mean Final Value:** 6.002010e-1
* **Success Rate:** 75.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.968e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.265e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.465e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.051e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.508e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.224e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1272</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.132e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.054e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1813</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.239e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.040e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.987e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.502e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.498e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.492e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.704e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.478e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">931</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1219</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.584e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.430e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.718e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.564e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.035e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">143</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2263</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.072</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.561e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.540e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1579</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.939e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.821e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.595e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.273e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.933e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.712e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2655</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.718e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.836e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.247e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.623e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1779</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.446e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.758e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.187e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.840e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">927</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1219</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.941e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.309e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1488</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1965</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (15 out of 20)

* **Average Iterations to Convergence:** 126.3
* **Average Function Evaluations:** 1502.5
* **Average Time to Convergence:** 0.063s
* **Fastest Convergence:** 78 iterations (0.039s)
* **Slowest Convergence:** 168 iterations (0.085s)

### Failed Runs (5 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 5 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 1.131821e-7
**Final Gradient Norm:** 1.054410e-2
**Iterations:** 116
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.360e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.442e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.057e0, 9.860e-1, -1.275e0, 1.065e0, -1.235e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.360e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.442e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-9</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.057e0, 9.860e-1, -1.275e0, 1.065e0, -1.235e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.360e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.442e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.974e-1, 1.260e-1, -5.348e-1, 1.681e-1, -4.727e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.376e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.887e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.613e-1, 2.186e-1, -3.623e-1, 1.104e-1, -3.492e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.296e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.320e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.451e-1, 2.079e-1, -1.711e-1, 6.261e-2, -1.942e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">112</td><td style="border: 1px solid #ddd; padding: 4px;">1.930e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.093e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.999e-1, 9.999e-1, 9.997e-1, 9.996e-1, 9.992e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">113</td><td style="border: 1px solid #ddd; padding: 4px;">1.892e-4</td><td style="border: 1px solid #ddd; padding: 4px;">8.318e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 9.999e-1, 9.999e-1, 9.998e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">114</td><td style="border: 1px solid #ddd; padding: 4px;">2.079e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.008e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 9.999e-1, 9.999e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">115</td><td style="border: 1px solid #ddd; padding: 4px;">6.905e-6</td><td style="border: 1px solid #ddd; padding: 4px;">8.492e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">116</td><td style="border: 1px solid #ddd; padding: 4px;">1.132e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.054e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1666.2
- **Average Gradient Evaluations per Run:** 2202.3
- **Average Iterations per Run:** 139.7
- **Average Time per Run:** 0.070s
- **Function Evaluations per Second:** 23633.6
- **Iterations per Second:** 1980.9
### Resource Utilization
- **Total Function Evaluations:** 33323
- **Total Gradient Evaluations:** 44047
- **Total Computation Time:** 1.4s
- **Function/Gradient Ratio:** 0.76
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 1)**
- Final Value: 1.967863e0
- Final Gradient Norm: 3.264793e0
- Iterations: 180
- Function Evaluations: 2156
- Reason: MaxFunctionEvaluations

**Run 2 (ID: 6)**
- Final Value: 4.986511e0
- Final Gradient Norm: 5.501879e0
- Iterations: 179
- Function Evaluations: 2158
- Reason: MaxFunctionEvaluations

**Run 3 (ID: 9)**
- Final Value: 4.960082e0
- Final Gradient Norm: 4.584342e0
- Iterations: 179
- Function Evaluations: 2153
- Reason: MaxFunctionEvaluations

**Run 4 (ID: 13)**
- Final Value: 8.938640e-2
- Final Gradient Norm: 5.821177e-1
- Iterations: 180
- Function Evaluations: 2160
- Reason: MaxFunctionEvaluations

**Run 5 (ID: 16)**
- Final Value: 1.718408e-4
- Final Gradient Norm: 4.835910e-1
- Iterations: 181
- Function Evaluations: 2159
- Reason: MaxFunctionEvaluations



## Data Files
* [Raw CSV Data](../data/problems/IllConditionedRosenbrock_10D_alpha100_results.csv)
* [Convergence Plot](../plots/IllConditionedRosenbrock_10D_alpha100.png)
* [Log Scale Convergence Plot](../plots/IllConditionedRosenbrock_10D_alpha100_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on IllConditionedRosenbrock_10D_alpha100*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
