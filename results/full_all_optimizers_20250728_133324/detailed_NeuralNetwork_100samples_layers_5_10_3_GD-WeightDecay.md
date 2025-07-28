# Detailed Analysis: GD-WeightDecay on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** GD-WeightDecay
**Problem Family:** ML Neural Networks
**Dimension:** 93
**Success Threshold:** 1.360e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.040198e-1
* **Worst Final Value:** 2.144951e-1
* **Mean Final Value:** 2.082266e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.051e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.507e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.116e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.426e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.479</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.104e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.360e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.078e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.724e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.479</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.097e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.037e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.484</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.114e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.779e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.076e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.097e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.073e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.481e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.067e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.909e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.048e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.400e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.066e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.728e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.046e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.878e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.083e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.852e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.145e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.152e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.067e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.032e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.088e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.900e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.094e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.270e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.126e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.532e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.068e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.797e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.040e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.107e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.479</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 2.040198e-1
**Final Gradient Norm:** 6.107243e-2
**Iterations:** 332
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.440e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.169e-2, 4.612e-2, -2.273e-1, 2.751e-2, -6.372e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.440e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.246e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.168e-2, 4.605e-2, -2.273e-1, 2.753e-2, -6.375e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.414e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.233e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.166e-2, 4.591e-2, -2.273e-1, 2.757e-2, -6.380e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.369e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.209e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.163e-2, 4.572e-2, -2.272e-1, 2.762e-2, -6.387e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.309e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.177e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.160e-2, 4.550e-2, -2.272e-1, 2.768e-2, -6.395e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">328</td><td style="border: 1px solid #ddd; padding: 4px;">2.041e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.173e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.279e-2, 6.514e-2, -2.520e-1, 6.455e-2, -1.238e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">329</td><td style="border: 1px solid #ddd; padding: 4px;">2.041e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.131e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.272e-2, 6.520e-2, -2.522e-1, 6.469e-2, -1.240e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">330</td><td style="border: 1px solid #ddd; padding: 4px;">2.041e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.122e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.265e-2, 6.526e-2, -2.524e-1, 6.482e-2, -1.242e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">331</td><td style="border: 1px solid #ddd; padding: 4px;">2.041e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.114e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.257e-2, 6.531e-2, -2.525e-1, 6.496e-2, -1.244e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">332</td><td style="border: 1px solid #ddd; padding: 4px;">2.040e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.107e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.257e-2, 6.531e-2, -2.525e-1, 6.496e-2, -1.244e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 335.0
- **Average Gradient Evaluations per Run:** 668.0
- **Average Iterations per Run:** 332.0
- **Average Time per Run:** 0.478s
- **Function Evaluations per Second:** 700.3
- **Iterations per Second:** 694.0
### Resource Utilization
- **Total Function Evaluations:** 6700
- **Total Gradient Evaluations:** 13360
- **Total Computation Time:** 9.6s
- **Function/Gradient Ratio:** 0.50
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
*Detailed report for GD-WeightDecay on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
