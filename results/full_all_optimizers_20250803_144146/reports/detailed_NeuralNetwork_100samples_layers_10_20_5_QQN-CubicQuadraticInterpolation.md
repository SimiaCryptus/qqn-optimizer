# Detailed Analysis: QQN-CubicQuadraticInterpolation on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 3.772146e-2
* **Worst Final Value:** 3.819419e-2
* **Mean Final Value:** 3.802961e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.772e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.046e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.795e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.275e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.794e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.249e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.814e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.109e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.798e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.706e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1227</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.815e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.608e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.752</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.800e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.870e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">85</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.805e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.495e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">91</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1093</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.812e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.513e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.796</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.795e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.875e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">793</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.695e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1281</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1697</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.588</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.811e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.622e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.869</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.021e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.803e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.871e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.744</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.803e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.075e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">961</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.925</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.802e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.046e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.704e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1297</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.638</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.133e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.097</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.784e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.448e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">793</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.680</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.807e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.942e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 85.7
* **Average Function Evaluations:** 1028.8
* **Average Time to Convergence:** 2.119s
* **Fastest Convergence:** 63 iterations (1.582s)
* **Slowest Convergence:** 132 iterations (3.271s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 3.772146e-2
**Final Gradient Norm:** 1.046061e-1
**Iterations:** 63
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.629e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.705e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.328e-1, -9.984e-2, -1.095e-1, 6.864e-3, 6.781e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.629e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.705e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.892e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.922e-1, -9.097e-2, -1.043e-1, -1.961e-3, 6.921e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.005e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.113e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.115e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.980e-1, -9.202e-2, -1.009e-1, -1.090e-3, 6.952e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.584e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.785e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.986e-1, -9.096e-2, -9.855e-2, -1.503e-3, 7.018e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.565e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.249e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.976e-1, -8.746e-2, -9.245e-2, -2.365e-3, 7.200e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">59</td><td style="border: 1px solid #ddd; padding: 4px;">3.951e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.474e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.531e-1, -1.975e-1, -2.213e-1, -1.869e-1, 9.958e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">60</td><td style="border: 1px solid #ddd; padding: 4px;">3.900e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.433e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.645e-1, -1.944e-1, -2.178e-1, -1.859e-1, 1.001e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">61</td><td style="border: 1px solid #ddd; padding: 4px;">3.862e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.604e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.663e-1, -1.952e-1, -2.162e-1, -1.851e-1, 9.972e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">62</td><td style="border: 1px solid #ddd; padding: 4px;">3.822e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.011e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.788e-1, -1.945e-1, -2.132e-1, -1.859e-1, 9.909e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">63</td><td style="border: 1px solid #ddd; padding: 4px;">3.772e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.046e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.788e-1, -1.945e-1, -2.132e-1, -1.859e-1, 9.909e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1028.8
- **Average Gradient Evaluations per Run:** 1363.5
- **Average Iterations per Run:** 85.7
- **Average Time per Run:** 2.119s
- **Function Evaluations per Second:** 485.5
- **Iterations per Second:** 40.4
### Resource Utilization
- **Total Function Evaluations:** 20576
- **Total Gradient Evaluations:** 27270
- **Total Computation Time:** 42.4s
- **Function/Gradient Ratio:** 0.75
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
