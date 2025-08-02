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
* **Best Final Value:** 3.774063e-2
* **Worst Final Value:** 3.818778e-2
* **Mean Final Value:** 3.801900e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.810e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.962e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1257</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.930</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.809e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.305e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">961</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.569e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.816e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.772e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">91</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.226</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.074e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1561</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.789e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.695e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">817</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.650</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.814e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.788e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">961</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.960</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.784e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.865e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.781e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.089e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">925</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.810e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.942e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">693</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">913</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.794e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.631e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.807e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.390e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.811e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.035e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.784e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.285e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.838</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.012e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.786e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.830e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.795e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.718e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.952</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.808e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.865e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1497</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.774e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.080e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">795</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.606</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.809e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.832e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">961</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 79.8
* **Average Function Evaluations:** 959.5
* **Average Time to Convergence:** 1.947s
* **Fastest Convergence:** 58 iterations (1.417s)
* **Slowest Convergence:** 106 iterations (2.582s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 19)
**Final Value:** 3.774063e-2
**Final Gradient Norm:** 1.079598e-1
**Iterations:** 66
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.389e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.541e-2, 1.083e-1, -3.484e-2, 5.099e-2, 7.247e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.389e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.905e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.486e-2, 1.007e-1, -3.061e-2, 4.155e-2, 7.445e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.768e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.799e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.586e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.513e-2, 1.019e-1, -2.911e-2, 3.841e-2, 7.675e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.615e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.224e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.035e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.577e-2, 1.027e-1, -2.639e-2, 3.228e-2, 8.082e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.586e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.400e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.035e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.833e-2, 1.037e-1, -2.167e-2, 2.313e-2, 8.805e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">62</td><td style="border: 1px solid #ddd; padding: 4px;">3.975e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.240e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.035e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.121e-1, 4.114e-1, -1.115e-1, -5.524e-1, 2.723e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">63</td><td style="border: 1px solid #ddd; padding: 4px;">3.930e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.006e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.035e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.193e-1, 4.226e-1, -1.166e-1, -5.708e-1, 2.646e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">64</td><td style="border: 1px solid #ddd; padding: 4px;">3.882e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.103e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.035e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.281e-1, 4.332e-1, -1.147e-1, -5.837e-1, 2.597e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">65</td><td style="border: 1px solid #ddd; padding: 4px;">3.825e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.118e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.035e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.360e-1, 4.391e-1, -1.105e-1, -5.893e-1, 2.568e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">66</td><td style="border: 1px solid #ddd; padding: 4px;">3.774e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.080e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.360e-1, 4.391e-1, -1.105e-1, -5.893e-1, 2.568e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 959.5
- **Average Gradient Evaluations per Run:** 1271.2
- **Average Iterations per Run:** 79.8
- **Average Time per Run:** 1.947s
- **Function Evaluations per Second:** 492.9
- **Iterations per Second:** 41.0
### Resource Utilization
- **Total Function Evaluations:** 19190
- **Total Gradient Evaluations:** 25424
- **Total Computation Time:** 38.9s
- **Function/Gradient Ratio:** 0.75
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
