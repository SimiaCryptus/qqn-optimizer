# Detailed Analysis: QQN-Bisection-2 on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** QQN-Bisection-2
**Problem Family:** ML Neural Networks
**Dimension:** 325
**Success Threshold:** 3.540e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 4.170785e-2
* **Worst Final Value:** 5.980191e-2
* **Mean Final Value:** 5.208971e-2
* **Success Rate:** 0.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.493e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.312e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.773</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.171e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.047e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.765</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.980e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.157e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.501e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.250e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.598e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.239e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.901e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.341e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.148e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.843e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.929e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.167e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.538e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.919e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.165e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.768</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.847e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.803e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.294e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.147e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.768</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.663e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.056e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.773</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.920e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.175e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.926e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.735e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.989e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.352e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.708e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.967e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.437e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.134e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.579e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.126e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.794</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.009e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.950e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 4.170785e-2
**Final Gradient Norm:** 1.047011e-1
**Iterations:** 47
**Convergence Reason:** MaxFunctionEvaluations

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.123e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.198e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.470e-1, -7.142e-2, -1.502e-1, -3.064e-2, 1.786e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.123e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.198e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.628e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.481e-1, -7.543e-2, -1.550e-1, -3.245e-2, 1.786e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.610e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.725e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.506e-1, -8.668e-2, -1.526e-1, -3.935e-2, 1.891e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.571e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.808e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.554e-1, -1.066e-1, -1.514e-1, -5.042e-2, 2.070e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.518e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.706e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.695e-1, -1.679e-1, -1.959e-1, -7.329e-2, 2.390e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">43</td><td style="border: 1px solid #ddd; padding: 4px;">4.514e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.198e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.961e-1, 3.971e-1, -1.883e-1, -1.504e-1, 7.120e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">44</td><td style="border: 1px solid #ddd; padding: 4px;">4.412e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.606e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.949e-1, 3.984e-1, -1.976e-1, -1.546e-1, 7.109e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">45</td><td style="border: 1px solid #ddd; padding: 4px;">4.312e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.591e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.966e-1, 4.014e-1, -2.007e-1, -1.534e-1, 7.088e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">46</td><td style="border: 1px solid #ddd; padding: 4px;">4.230e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.102e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.036e-1, 4.103e-1, -2.014e-1, -1.461e-1, 7.109e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">47</td><td style="border: 1px solid #ddd; padding: 4px;">4.171e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.047e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.036e-1, 4.103e-1, -2.014e-1, -1.461e-1, 7.109e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 535.4
- **Average Gradient Evaluations per Run:** 473.9
- **Average Iterations per Run:** 47.0
- **Average Time per Run:** 0.769s
- **Function Evaluations per Second:** 695.8
- **Iterations per Second:** 61.0
### Resource Utilization
- **Total Function Evaluations:** 10707
- **Total Gradient Evaluations:** 9478
- **Total Computation Time:** 15.4s
- **Function/Gradient Ratio:** 1.13
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](convergence_NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](convergence_NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for QQN-Bisection-2 on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
