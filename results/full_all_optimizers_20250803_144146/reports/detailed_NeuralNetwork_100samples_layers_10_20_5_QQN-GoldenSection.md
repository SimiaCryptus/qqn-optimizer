# Detailed Analysis: QQN-GoldenSection on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** QQN-GoldenSection
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 14 (70.0%)

### Quick Statistics
* **Best Final Value:** 3.784405e-2
* **Worst Final Value:** 4.135055e-2
* **Mean Final Value:** 3.852357e-2
* **Success Rate:** 70.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.833e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.815e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">625</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.782</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.093e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.699e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.135e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.411e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.793e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.744e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.807e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.904e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.798e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.835e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.562</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.817e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.277e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.614</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.791e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.491e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.929</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.091e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.061e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.834e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.175e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.813</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.179e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3747</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.393</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.808e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.629e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2781</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">398</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.791e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.306e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.840e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.780e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.813</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.807e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.275e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.791e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.389e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3201</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">458</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.817e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.418e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3327</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.784e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.957e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3957</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.801e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.097e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">596</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.652</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.809e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.731e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3411</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">488</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (14 out of 20)

* **Average Iterations to Convergence:** 80.3
* **Average Function Evaluations:** 3381.6
* **Average Time to Convergence:** 2.160s
* **Fastest Convergence:** 58 iterations (1.562s)
* **Slowest Convergence:** 99 iterations (2.652s)

### Failed Runs (6 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 6 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 3.784405e-2
**Final Gradient Norm:** 8.957197e-2
**Iterations:** 94
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.563e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.640e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.606e-2, 2.698e-1, -7.157e-2, 1.872e-1, -9.110e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.563e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.640e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.647e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.727e-2, 2.608e-1, -7.135e-2, 1.601e-1, -1.127e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.620e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.588e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.331e-2, 2.528e-1, -6.776e-2, 1.595e-1, -2.342e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.582e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.439e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.167e-2, 2.260e-1, -5.625e-2, 1.536e-1, -5.977e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.509e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.053e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.015e-1, 1.968e-1, -3.276e-2, 1.873e-1, -1.067e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">90</td><td style="border: 1px solid #ddd; padding: 4px;">3.905e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.396e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.234e0, 2.808e-1, 1.410e-1, 2.703e-1, -8.794e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">91</td><td style="border: 1px solid #ddd; padding: 4px;">3.882e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.883e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.242e0, 2.748e-1, 1.328e-1, 2.587e-1, -8.809e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">92</td><td style="border: 1px solid #ddd; padding: 4px;">3.853e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.715e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.252e0, 2.639e-1, 1.241e-1, 2.475e-1, -8.803e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">93</td><td style="border: 1px solid #ddd; padding: 4px;">3.820e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.506e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.259e0, 2.568e-1, 1.160e-1, 2.414e-1, -8.795e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">94</td><td style="border: 1px solid #ddd; padding: 4px;">3.784e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.957e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.259e0, 2.568e-1, 1.160e-1, 2.414e-1, -8.795e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3690.9
- **Average Gradient Evaluations per Run:** 527.6
- **Average Iterations per Run:** 87.3
- **Average Time per Run:** 2.355s
- **Function Evaluations per Second:** 1567.2
- **Iterations per Second:** 37.1
### Resource Utilization
- **Total Function Evaluations:** 73819
- **Total Gradient Evaluations:** 10552
- **Total Computation Time:** 47.1s
- **Function/Gradient Ratio:** 7.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for QQN-GoldenSection on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
