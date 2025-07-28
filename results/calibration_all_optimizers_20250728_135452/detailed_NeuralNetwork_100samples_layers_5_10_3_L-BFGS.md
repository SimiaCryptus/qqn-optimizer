# Detailed Analysis: L-BFGS on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** L-BFGS
**Problem Family:** ML Neural Networks
**Dimension:** 93
**Success Threshold:** 1.360e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.519457e-1
* **Worst Final Value:** 3.487749e-1
* **Mean Final Value:** 1.913522e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.703e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.460e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.603e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.989e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">598</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.956e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.019e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">320</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.556e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.039e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">598</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.711e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.573e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.952e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.625e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.192e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.141e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.600e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.394e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">609</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">401</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.022e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.598e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.488e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.692e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.519e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.874e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">593</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.392</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.090e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.111e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">203</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.533e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.507e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">619</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.115e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.654e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.635e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.481e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">605</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.643e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.505e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.622e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.183e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">599</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.706e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.159e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">600</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.388</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.041e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.099e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">225</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.583e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.164e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">611</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 12 runs
- FunctionTolerance: 8 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 11)
**Final Value:** 1.519457e-1
**Final Gradient Norm:** 6.873603e-2
**Iterations:** 100
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.071e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.084e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.750e-1, -4.020e-2, 6.072e-2, -7.466e-2, 1.104e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.071e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.084e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.668e-1, -3.701e-2, 6.091e-2, -7.505e-2, 1.171e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.221e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.534e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.637e-1, -3.125e-2, 5.972e-2, -7.197e-2, 1.189e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.214e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.827e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.635e-1, -3.044e-2, 5.971e-2, -7.168e-2, 1.195e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.204e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.475e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.637e-1, -2.245e-2, 5.952e-2, -6.866e-2, 1.244e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">96</td><td style="border: 1px solid #ddd; padding: 4px;">1.531e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.022e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.755e-1, 3.813e-2, 2.980e-1, 3.629e-2, 4.234e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">97</td><td style="border: 1px solid #ddd; padding: 4px;">1.527e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.031e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.755e-1, 3.652e-2, 2.958e-1, 3.511e-2, 4.252e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">98</td><td style="border: 1px solid #ddd; padding: 4px;">1.526e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.723e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.791e-1, 3.926e-2, 2.992e-1, 3.770e-2, 4.273e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">99</td><td style="border: 1px solid #ddd; padding: 4px;">1.522e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.085e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.791e-1, 3.813e-2, 3.014e-1, 3.654e-2, 4.288e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">100</td><td style="border: 1px solid #ddd; padding: 4px;">1.519e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.874e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.791e-1, 3.813e-2, 3.014e-1, 3.654e-2, 4.288e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 466.6
- **Average Gradient Evaluations per Run:** 303.8
- **Average Iterations per Run:** 74.5
- **Average Time per Run:** 0.296s
- **Function Evaluations per Second:** 1576.6
- **Iterations per Second:** 251.6
### Resource Utilization
- **Total Function Evaluations:** 9331
- **Total Gradient Evaluations:** 6075
- **Total Computation Time:** 5.9s
- **Function/Gradient Ratio:** 1.54
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/NeuralNetwork_100samples_layers_5_10_3_results.csv)
* [Convergence Plot](convergence_NeuralNetwork_100samples_layers_5_10_3.png)
* [Log Scale Convergence Plot](convergence_NeuralNetwork_100samples_layers_5_10_3_log.png)


---
*Detailed report for L-BFGS on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
