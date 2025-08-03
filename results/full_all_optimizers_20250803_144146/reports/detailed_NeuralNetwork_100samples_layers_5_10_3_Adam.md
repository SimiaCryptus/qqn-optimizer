# Detailed Analysis: Adam on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** Adam
**Problem Family:** Neural Networks
**Dimension:** 93
**Success Threshold:** 1.400e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.446067e-1
* **Worst Final Value:** 1.684184e-1
* **Mean Final Value:** 1.580082e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.592e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.098e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.651e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.571e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.001e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.522e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.416e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.635e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.270e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.656e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.749e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.602e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.372e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.474e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.654e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.658e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.228e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.068</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.570e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.632e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.615e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.143e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.587e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.708e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.623e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.805e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.446e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.724e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.548e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.441e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.536e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.066e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.503e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.421e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.072</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.545e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.502e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.684e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.120e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.583e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.722e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 1.446067e-1
**Final Gradient Norm:** 4.723769e-2
**Iterations:** 1249
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.668e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.409e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.110e-2, 1.855e-1, -7.496e-2, 1.240e-1, -6.470e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.668e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.409e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.010e-2, 1.845e-1, -7.396e-2, 1.250e-1, -6.570e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.652e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.400e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.910e-2, 1.835e-1, -7.296e-2, 1.260e-1, -6.670e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.636e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.391e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.810e-2, 1.825e-1, -7.196e-2, 1.270e-1, -6.768e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.620e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.382e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.710e-2, 1.815e-1, -7.096e-2, 1.280e-1, -6.867e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1245</td><td style="border: 1px solid #ddd; padding: 4px;">1.447e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.289e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.817e-1, 4.732e-1, 5.260e-1, 4.441e-1, -4.609e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1246</td><td style="border: 1px solid #ddd; padding: 4px;">1.447e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.398e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.822e-1, 4.734e-1, 5.265e-1, 4.445e-1, -4.612e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1247</td><td style="border: 1px solid #ddd; padding: 4px;">1.446e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.842e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.827e-1, 4.736e-1, 5.269e-1, 4.448e-1, -4.613e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1248</td><td style="border: 1px solid #ddd; padding: 4px;">1.446e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.386e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.832e-1, 4.737e-1, 5.270e-1, 4.449e-1, -4.615e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1249</td><td style="border: 1px solid #ddd; padding: 4px;">1.446e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.724e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.832e-1, 4.737e-1, 5.270e-1, 4.449e-1, -4.615e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2502.0
- **Average Gradient Evaluations per Run:** 2502.0
- **Average Iterations per Run:** 1249.0
- **Average Time per Run:** 2.063s
- **Function Evaluations per Second:** 1212.5
- **Iterations per Second:** 605.3
### Resource Utilization
- **Total Function Evaluations:** 50040
- **Total Gradient Evaluations:** 50040
- **Total Computation Time:** 41.3s
- **Function/Gradient Ratio:** 1.00
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
*Detailed report for Adam on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
