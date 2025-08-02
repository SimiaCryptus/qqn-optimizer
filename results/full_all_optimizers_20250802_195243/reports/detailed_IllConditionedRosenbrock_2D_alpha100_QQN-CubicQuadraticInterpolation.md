# Detailed Analysis: QQN-CubicQuadraticInterpolation on IllConditionedRosenbrock_2D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_2D_alpha100
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 7 (35.0%)

### Quick Statistics
* **Best Final Value:** 4.343042e-10
* **Worst Final Value:** 9.891793e-2
* **Mean Final Value:** 3.479461e-2
* **Success Rate:** 35.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.081e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.770e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.464e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.048e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.214e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.209e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.066e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.426e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.814e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.530e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2846</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.897e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.122e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">706</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.018e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.008e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.686e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.598e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2864</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.343e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.190e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">501</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.364e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.912e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.892e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.991e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.678e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.441e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1793</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.539e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.181e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.130e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.437e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">679</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.146e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.873e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.479e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2864</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.540e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.399e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.047e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.984e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.230e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">716</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.245e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.100e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2864</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (7 out of 20)

* **Average Iterations to Convergence:** 79.1
* **Average Function Evaluations:** 916.9
* **Average Time to Convergence:** 0.037s
* **Fastest Convergence:** 35 iterations (0.015s)
* **Slowest Convergence:** 118 iterations (0.055s)

### Failed Runs (13 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 13 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 9)
**Final Value:** 4.343042e-10
**Final Gradient Norm:** 9.189892e-4
**Iterations:** 35
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.024e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.649e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.192e0, 9.170e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.024e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.649e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-9</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.192e0, 9.170e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.024e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.649e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.100e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.502e0, 2.027e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.484e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.458e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.532e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.467e0, 2.038e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.102e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.434e0, 2.052e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">31</td><td style="border: 1px solid #ddd; padding: 4px;">1.213e-3</td><td style="border: 1px solid #ddd; padding: 4px;">2.983e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.022e0, 1.043e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">32</td><td style="border: 1px solid #ddd; padding: 4px;">4.886e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.277e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.002e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">33</td><td style="border: 1px solid #ddd; padding: 4px;">4.253e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.771e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.002e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">34</td><td style="border: 1px solid #ddd; padding: 4px;">1.127e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.422e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">35</td><td style="border: 1px solid #ddd; padding: 4px;">4.343e-10</td><td style="border: 1px solid #ddd; padding: 4px;">9.190e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1722.9
- **Average Gradient Evaluations per Run:** 2273.4
- **Average Iterations per Run:** 144.6
- **Average Time per Run:** 0.070s
- **Function Evaluations per Second:** 24447.5
- **Iterations per Second:** 2051.8
### Resource Utilization
- **Total Function Evaluations:** 34458
- **Total Gradient Evaluations:** 45469
- **Total Computation Time:** 1.4s
- **Function/Gradient Ratio:** 0.76
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/IllConditionedRosenbrock_2D_alpha100_results.csv)
* [Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100.png)
* [Log Scale Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on IllConditionedRosenbrock_2D_alpha100*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
