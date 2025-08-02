# Detailed Analysis: L-BFGS-Conservative on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** L-BFGS-Conservative
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 4 (20.0%)

### Quick Statistics
* **Best Final Value:** 3.818271e-2
* **Worst Final Value:** 5.526571e-2
* **Mean Final Value:** 4.389468e-2
* **Success Rate:** 20.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.838e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.409e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.188e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">780</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.336e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.191e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3371</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1638</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.046e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.428e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2190</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.740</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.527e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.843e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.321e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.002e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">229</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">932</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.749</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.820e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.361e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.325e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">217</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.591</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.274e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.032e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">544</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2822</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.761</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.261e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.026e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2795</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.008e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.182e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2944</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.655</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.394e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.289e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.395e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.705e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.350e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.068e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3542</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1466</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.388e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.097e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1847</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.048e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.510e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2805</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2198</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.692e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.694e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2784</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.789</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.626e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.869e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.645e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1815</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.766e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.622e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2930</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (4 out of 20)

* **Average Iterations to Convergence:** 206.2
* **Average Function Evaluations:** 1184.5
* **Average Time to Convergence:** 1.466s
* **Fastest Convergence:** 194 iterations (1.344s)
* **Slowest Convergence:** 217 iterations (1.591s)

### Failed Runs (16 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 15 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 8)
**Final Value:** 3.818271e-2
**Final Gradient Norm:** 7.324596e-2
**Iterations:** 217
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.234e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.231e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.923e-2, -2.434e-2, -2.210e-1, -5.441e-2, 1.688e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.234e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.231e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.832e-2, -2.740e-2, -2.200e-1, -5.296e-2, 1.624e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.971e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.102e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.748e-2, -3.036e-2, -2.191e-1, -5.157e-2, 1.563e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.826e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.797e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.712e-2, -3.182e-2, -2.187e-1, -5.091e-2, 1.535e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.799e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.301e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.694e-2, -3.262e-2, -2.185e-1, -5.060e-2, 1.521e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">213</td><td style="border: 1px solid #ddd; padding: 4px;">3.837e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.094e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.459e-1, -3.736e-3, 6.063e-1, 4.986e-1, -3.019e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">214</td><td style="border: 1px solid #ddd; padding: 4px;">3.829e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.805e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.813e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.461e-1, -3.712e-3, 6.064e-1, 4.987e-1, -3.021e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">215</td><td style="border: 1px solid #ddd; padding: 4px;">3.829e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.787e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.463e-1, -3.755e-3, 6.066e-1, 4.986e-1, -3.023e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">216</td><td style="border: 1px solid #ddd; padding: 4px;">3.827e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.668e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.471e-1, -3.142e-3, 6.080e-1, 4.990e-1, -3.034e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">217</td><td style="border: 1px solid #ddd; padding: 4px;">3.818e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.325e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.471e-1, -3.142e-3, 6.080e-1, 4.990e-1, -3.034e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2571.8
- **Average Gradient Evaluations per Run:** 1711.7
- **Average Iterations per Run:** 425.7
- **Average Time per Run:** 3.085s
- **Function Evaluations per Second:** 833.7
- **Iterations per Second:** 138.0
### Resource Utilization
- **Total Function Evaluations:** 51437
- **Total Gradient Evaluations:** 34233
- **Total Computation Time:** 61.7s
- **Function/Gradient Ratio:** 1.50
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
*Detailed report for L-BFGS-Conservative on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
