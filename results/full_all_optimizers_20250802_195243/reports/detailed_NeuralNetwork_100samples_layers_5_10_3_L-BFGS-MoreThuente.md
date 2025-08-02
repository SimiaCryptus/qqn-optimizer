# Detailed Analysis: L-BFGS-MoreThuente on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** Neural Networks
**Dimension:** 93
**Success Threshold:** 1.400e-1
**Total Runs:** 20
**Successful Runs:** 3 (15.0%)

### Quick Statistics
* **Best Final Value:** 1.398785e-1
* **Worst Final Value:** 1.637270e-1
* **Mean Final Value:** 1.510591e-1
* **Success Rate:** 15.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.521e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.589e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2861</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.636e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.441e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.948</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.456e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.683e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2864</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.599e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">221</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1723</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1281</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.576e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.646e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.603e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.985e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.941</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.500e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.018e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.637e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.815e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">323</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2825</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.947</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.870e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.868</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.622e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.756e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.553e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.924e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.959</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.471e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.599e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.946</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.115e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.948</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.513e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.787e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.950</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.461e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.924e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2871</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.948</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.080e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">260</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1561</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.563e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.480e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.469e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.110e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.959</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.450e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.759e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.960</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.540e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.486e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.952</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (3 out of 20)

* **Average Iterations to Convergence:** 277.3
* **Average Function Evaluations:** 2187.0
* **Average Time to Convergence:** 1.489s
* **Fastest Convergence:** 221 iterations (1.168s)
* **Slowest Convergence:** 351 iterations (1.868s)

### Failed Runs (17 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 17 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 1.398785e-1
**Final Gradient Norm:** 6.599089e-2
**Iterations:** 221
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.444e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.295e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.095e-1, 9.126e-2, 2.690e-2, -1.036e-2, 1.026e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.444e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.295e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.887e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.041e-1, 9.945e-2, 1.797e-2, -9.764e-3, 1.019e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.251e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.059e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.936e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.028e-1, 9.808e-2, 5.208e-3, -1.009e-2, 1.096e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.219e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.583e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.987e-2, 9.821e-2, -1.484e-2, -1.113e-2, 1.216e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.202e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.863e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.892e-2, 9.764e-2, -3.349e-2, -1.222e-2, 1.338e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">217</td><td style="border: 1px solid #ddd; padding: 4px;">1.403e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.595e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.049e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.865e-1, 1.759e-1, 1.439e-1, -1.163e-1, 4.618e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">218</td><td style="border: 1px solid #ddd; padding: 4px;">1.401e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.841e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.911e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.873e-1, 1.754e-1, 1.454e-1, -1.171e-1, 4.620e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">219</td><td style="border: 1px solid #ddd; padding: 4px;">1.401e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.724e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.909e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.917e-1, 1.781e-1, 1.481e-1, -1.202e-1, 4.599e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">220</td><td style="border: 1px solid #ddd; padding: 4px;">1.400e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.220e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.842e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.897e-1, 1.751e-1, 1.493e-1, -1.191e-1, 4.609e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">221</td><td style="border: 1px solid #ddd; padding: 4px;">1.399e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.599e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.897e-1, 1.751e-1, 1.493e-1, -1.191e-1, 4.609e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2755.1
- **Average Gradient Evaluations per Run:** 2074.3
- **Average Iterations per Run:** 339.6
- **Average Time per Run:** 1.880s
- **Function Evaluations per Second:** 1465.6
- **Iterations per Second:** 180.6
### Resource Utilization
- **Total Function Evaluations:** 55102
- **Total Gradient Evaluations:** 41486
- **Total Computation Time:** 37.6s
- **Function/Gradient Ratio:** 1.33
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_5_10_3_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3_log.png)


---
*Detailed report for L-BFGS-MoreThuente on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
