# Detailed Analysis: QQN-CubicQuadraticInterpolation on SparseRosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_10D
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** SparseRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 11 (55.0%)

### Quick Statistics
* **Best Final Value:** 1.891252e-8
* **Worst Final Value:** 3.940173e-1
* **Mean Final Value:** 1.417493e-1
* **Success Rate:** 55.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.407e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.720e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1639</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.882e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.342e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.865e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.719e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.506e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.947e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1683</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2203</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.071</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.076e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.870e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.091</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.891e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.709e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.059e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.803e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">913</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.730e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.209e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.940e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.743e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.314e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.105e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.540e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.599e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.849e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.441e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.472e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.529e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.386e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1494</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1961</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.793e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.540e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.373e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.263e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1257</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.620e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.305e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.092</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.914e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.424e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">920</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.690e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.838e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1977</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.209e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.290e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1259</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (11 out of 20)

* **Average Iterations to Convergence:** 109.3
* **Average Function Evaluations:** 1272.4
* **Average Time to Convergence:** 0.054s
* **Fastest Convergence:** 78 iterations (0.038s)
* **Slowest Convergence:** 144 iterations (0.071s)

### Failed Runs (9 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 6)
**Final Value:** 1.891252e-8
**Final Gradient Norm:** 5.708622e-3
**Iterations:** 100
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.582e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.399e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.080e0, 8.317e-1, -1.208e0, 9.813e-1, -1.097e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.582e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.399e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-9</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.080e0, 8.317e-1, -1.208e0, 9.813e-1, -1.097e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.582e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.399e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.497e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.571e-1, 9.322e-1, -8.558e-1, 1.124e0, -9.028e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">4.183e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.881e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.245e-1, 8.952e-1, -9.805e-1, 1.058e0, -9.667e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.066e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.312e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.390e-1, 8.856e-1, -1.024e0, 1.036e0, -9.776e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">96</td><td style="border: 1px solid #ddd; padding: 4px;">8.533e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.289e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.961e-1, 9.921e-1, 9.999e-1, 1.000e0, 1.003e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">97</td><td style="border: 1px solid #ddd; padding: 4px;">6.659e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.919e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.980e-1, 9.959e-1, 9.999e-1, 1.000e0, 1.002e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">98</td><td style="border: 1px solid #ddd; padding: 4px;">3.659e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.148e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.211e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.980e-1, 9.960e-1, 1.000e0, 1.000e0, 1.002e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">99</td><td style="border: 1px solid #ddd; padding: 4px;">1.454e-5</td><td style="border: 1px solid #ddd; padding: 4px;">4.528e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">100</td><td style="border: 1px solid #ddd; padding: 4px;">1.891e-8</td><td style="border: 1px solid #ddd; padding: 4px;">5.709e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1670.9
- **Average Gradient Evaluations per Run:** 2197.7
- **Average Iterations per Run:** 141.2
- **Average Time per Run:** 0.071s
- **Function Evaluations per Second:** 23614.7
- **Iterations per Second:** 1996.3
### Resource Utilization
- **Total Function Evaluations:** 33418
- **Total Gradient Evaluations:** 43953
- **Total Computation Time:** 1.4s
- **Function/Gradient Ratio:** 0.76
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/SparseRosenbrock_10D_results.csv)
* [Convergence Plot](../plots/SparseRosenbrock_10D.png)
* [Log Scale Convergence Plot](../plots/SparseRosenbrock_10D_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on SparseRosenbrock_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
