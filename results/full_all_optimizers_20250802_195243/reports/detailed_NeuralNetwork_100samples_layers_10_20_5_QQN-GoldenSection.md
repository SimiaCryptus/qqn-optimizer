# Detailed Analysis: QQN-GoldenSection on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** QQN-GoldenSection
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 17 (85.0%)

### Quick Statistics
* **Best Final Value:** 3.789962e-2
* **Worst Final Value:** 4.070546e-2
* **Mean Final Value:** 3.828561e-2
* **Success Rate:** 85.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.801e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.597e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">91</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.798e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.966e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.790e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.093e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.800e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.011e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.793e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.063e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">67</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2833</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.812</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.847e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.225e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">625</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.800</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.071e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.389e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">625</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.231e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">422</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.813e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.598e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.813e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.684e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.811e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.105e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3967</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.813e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.010e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.793e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.781e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3957</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.728e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.817e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.188e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.068</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.941e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.026e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.818</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.741e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4377</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">626</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.808</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.813e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.376e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4219</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.815e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.945e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.260</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.800e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.462e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3411</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">488</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (17 out of 20)

* **Average Iterations to Convergence:** 84.4
* **Average Function Evaluations:** 3560.4
* **Average Time to Convergence:** 2.271s
* **Fastest Convergence:** 67 iterations (1.812s)
* **Slowest Convergence:** 104 iterations (2.808s)

### Failed Runs (3 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 3 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 3.789962e-2
**Final Gradient Norm:** 8.092664e-2
**Iterations:** 75
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.114e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.187e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.333e-2, -2.028e-2, -7.638e-2, -1.668e-1, -1.225e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.114e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.187e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.386e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.697e-2, -1.543e-2, -7.779e-2, -1.665e-1, -1.197e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.638e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.926e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.042e-2, -1.318e-2, -8.060e-2, -1.707e-1, -1.081e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.584e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.882e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-2, -6.777e-3, -8.760e-2, -1.827e-1, -7.998e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.201e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.745e-2, -2.003e-3, -9.018e-2, -1.948e-1, -5.079e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">71</td><td style="border: 1px solid #ddd; padding: 4px;">3.932e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.210e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.402e-1, -2.351e-1, -6.144e-1, -3.405e-1, 1.223e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">72</td><td style="border: 1px solid #ddd; padding: 4px;">3.895e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.290e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.389e-1, -2.375e-1, -6.097e-1, -3.354e-1, 1.322e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">73</td><td style="border: 1px solid #ddd; padding: 4px;">3.852e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.006e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.380e-1, -2.365e-1, -6.056e-1, -3.310e-1, 1.365e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">74</td><td style="border: 1px solid #ddd; padding: 4px;">3.824e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.304e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.359e-1, -2.356e-1, -6.072e-1, -3.292e-1, 1.401e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">75</td><td style="border: 1px solid #ddd; padding: 4px;">3.790e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.093e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.359e-1, -2.356e-1, -6.072e-1, -3.292e-1, 1.401e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3686.0
- **Average Gradient Evaluations per Run:** 526.2
- **Average Iterations per Run:** 87.2
- **Average Time per Run:** 2.351s
- **Function Evaluations per Second:** 1567.7
- **Iterations per Second:** 37.1
### Resource Utilization
- **Total Function Evaluations:** 73720
- **Total Gradient Evaluations:** 10525
- **Total Computation Time:** 47.0s
- **Function/Gradient Ratio:** 7.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 6)**
- Final Value: 3.846849e-2
- Final Gradient Norm: 7.225335e-2
- Iterations: 103
- Function Evaluations: 4386
- Reason: MaxFunctionEvaluations

**Run 2 (ID: 7)**
- Final Value: 4.070546e-2
- Final Gradient Norm: 7.389117e-2
- Iterations: 103
- Function Evaluations: 4389
- Reason: MaxFunctionEvaluations

**Run 3 (ID: 16)**
- Final Value: 3.941123e-2
- Final Gradient Norm: 1.025552e-1
- Iterations: 104
- Function Evaluations: 4418
- Reason: MaxFunctionEvaluations



## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for QQN-GoldenSection on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
