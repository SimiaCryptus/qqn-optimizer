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
* **Best Final Value:** 3.810484e-2
* **Worst Final Value:** 7.988191e-2
* **Mean Final Value:** 4.850831e-2
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
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.810e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.109e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">239</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">959</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.403e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.555e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">552</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2792</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2214</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.820e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.709e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2691</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1723</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.154e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.894e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">515</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.692</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.984e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.638e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.180e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.049e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2861</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.830</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.816e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.679e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">226</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.814e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.200e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.988e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.053e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">825</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.888e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.139e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.823</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.951e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.096e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2960</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.450e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.440e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2811</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.335e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.552e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2972</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.809</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.761e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.150e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3314</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.658</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.638e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.211e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.894</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.288e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.610e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2805</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.729e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.885e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">534</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.817</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.669e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.411e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.070e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">504</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2980</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.650</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.126e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.442e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (4 out of 20)

* **Average Iterations to Convergence:** 294.2
* **Average Function Evaluations:** 1631.8
* **Average Time to Convergence:** 2.122s
* **Fastest Convergence:** 226 iterations (1.592s)
* **Slowest Convergence:** 428 iterations (3.166s)

### Failed Runs (16 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 15 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 3.810484e-2
**Final Gradient Norm:** 1.109346e-1
**Iterations:** 239
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.854e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.141e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.574e-3, -2.097e-2, 5.602e-2, -9.196e-2, -1.008e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.854e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.141e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.381e-3, -2.251e-2, 5.753e-2, -9.153e-2, -9.914e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.740e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.910e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.195e-3, -2.390e-2, 5.888e-2, -9.119e-2, -9.756e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.680e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.374e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.114e-3, -2.443e-2, 5.943e-2, -9.106e-2, -9.692e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.668e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.073e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.057e-3, -2.468e-2, 5.975e-2, -9.097e-2, -9.659e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">235</td><td style="border: 1px solid #ddd; padding: 4px;">3.855e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.069e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.234e-1, -2.038e-1, 6.434e-1, 5.260e-2, -5.906e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">236</td><td style="border: 1px solid #ddd; padding: 4px;">3.848e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.153e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.235e-1, -2.032e-1, 6.427e-1, 5.145e-2, -5.912e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">237</td><td style="border: 1px solid #ddd; padding: 4px;">3.847e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.840e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.235e-1, -2.037e-1, 6.427e-1, 5.173e-2, -5.907e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">238</td><td style="border: 1px solid #ddd; padding: 4px;">3.846e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.207e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.232e-1, -2.026e-1, 6.390e-1, 4.584e-2, -5.909e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">239</td><td style="border: 1px solid #ddd; padding: 4px;">3.810e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.109e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.232e-1, -2.026e-1, 6.390e-1, 4.584e-2, -5.909e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2547.8
- **Average Gradient Evaluations per Run:** 1841.0
- **Average Iterations per Run:** 458.6
- **Average Time per Run:** 3.325s
- **Function Evaluations per Second:** 766.3
- **Iterations per Second:** 137.9
### Resource Utilization
- **Total Function Evaluations:** 50956
- **Total Gradient Evaluations:** 36821
- **Total Computation Time:** 66.5s
- **Function/Gradient Ratio:** 1.38
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
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
