# Detailed Analysis: QQN-StrongWolfe on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** QQN-StrongWolfe
**Problem Family:** ML Neural Networks
**Dimension:** 325
**Success Threshold:** 3.540e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.312050e-2
* **Worst Final Value:** 7.124595e-2
* **Mean Final Value:** 5.004625e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.246e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.773e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.819</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.408e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.196e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.825</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.182e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.479e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.537e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.725e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.838</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.574e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.144e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.550e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.370e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.520e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.200e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.817</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.542e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.902e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.817</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.430e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.404e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">541</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.905e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.291e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.005e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.933e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.829</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.312e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.643e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.826</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.516e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.989e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.026e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.332e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.125e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.387e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.831</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.649e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.826e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.410e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.630e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.198e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.035e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.822</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.322e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.822e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.832</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.639e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.024e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.826</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 12)
**Final Value:** 3.312050e-2
**Final Gradient Norm:** 8.643201e-2
**Iterations:** 66
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.435e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.474e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.933e-2, 7.522e-2, -4.273e-2, -2.687e-1, -1.403e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.435e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.474e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.875e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.422e-2, 7.906e-2, -3.978e-2, -2.704e-1, -1.440e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.612e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.621e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.314e-2, 7.893e-2, -3.965e-2, -2.628e-1, -1.406e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.574e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.543e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.595e-2, 8.129e-2, -3.654e-2, -2.467e-1, -1.371e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.505e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.101e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.136e-3, 8.524e-2, -1.980e-2, -2.418e-1, -4.649e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">62</td><td style="border: 1px solid #ddd; padding: 4px;">3.410e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.691e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-7.511e-3, 3.394e-1, 8.707e-2, -7.444e-2, -5.963e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">63</td><td style="border: 1px solid #ddd; padding: 4px;">3.384e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.097e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.655e-3, 3.407e-1, 8.318e-2, -7.242e-2, -5.953e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">64</td><td style="border: 1px solid #ddd; padding: 4px;">3.362e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.293e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.185e-2, 3.449e-1, 7.409e-2, -6.949e-2, -5.924e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">65</td><td style="border: 1px solid #ddd; padding: 4px;">3.331e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.687e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.478e-2, 3.482e-1, 6.696e-2, -6.567e-2, -5.897e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">66</td><td style="border: 1px solid #ddd; padding: 4px;">3.312e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.643e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.478e-2, 3.482e-1, 6.696e-2, -6.567e-2, -5.897e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 474.6
- **Average Gradient Evaluations per Run:** 535.0
- **Average Iterations per Run:** 63.4
- **Average Time per Run:** 0.824s
- **Function Evaluations per Second:** 575.9
- **Iterations per Second:** 76.9
### Resource Utilization
- **Total Function Evaluations:** 9491
- **Total Gradient Evaluations:** 10701
- **Total Computation Time:** 16.5s
- **Function/Gradient Ratio:** 0.89
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
*Detailed report for QQN-StrongWolfe on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
