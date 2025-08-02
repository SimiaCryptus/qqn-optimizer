# Detailed Analysis: GD on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** GD
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.175821e-1
* **Worst Final Value:** 1.340885e-1
* **Mean Final Value:** 1.280627e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.241e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.570e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.599</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.341e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.873e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.608</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.321e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.422e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.641</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.303e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.300e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.599</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.298e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.800e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.318e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.727e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.579</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.273e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.823e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.283e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.512e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.319e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.719e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.259e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.197e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.285e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.630e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.305e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.749e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.326e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.724e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.231e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.296e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.236e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.072e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.314e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.215e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.252e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.042e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.285e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.718e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.248e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.198e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.176e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.563e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 1.175821e-1
**Final Gradient Norm:** 9.563176e-2
**Iterations:** 1665
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.896e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.994e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.081e-1, -1.376e-1, 1.032e-1, -1.812e-1, -2.122e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.896e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.994e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.077e-1, -1.375e-1, 1.033e-1, -1.810e-1, -2.121e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.880e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.714e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.074e-1, -1.374e-1, 1.033e-1, -1.808e-1, -2.119e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.865e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.441e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.071e-1, -1.373e-1, 1.033e-1, -1.807e-1, -2.118e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.851e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.181e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.067e-1, -1.372e-1, 1.033e-1, -1.805e-1, -2.117e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1661</td><td style="border: 1px solid #ddd; padding: 4px;">1.176e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.502e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.658e-2, -2.062e-2, 8.462e-2, -1.220e-1, -2.510e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1662</td><td style="border: 1px solid #ddd; padding: 4px;">1.176e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.615e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.653e-2, -2.054e-2, 8.463e-2, -1.220e-1, -2.511e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1663</td><td style="border: 1px solid #ddd; padding: 4px;">1.176e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.785e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.649e-2, -2.048e-2, 8.464e-2, -1.220e-1, -2.511e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1664</td><td style="border: 1px solid #ddd; padding: 4px;">1.176e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.812e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.645e-2, -2.041e-2, 8.462e-2, -1.220e-1, -2.512e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1665</td><td style="border: 1px solid #ddd; padding: 4px;">1.176e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.563e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.645e-2, -2.041e-2, 8.462e-2, -1.220e-1, -2.512e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1668.0
- **Average Gradient Evaluations per Run:** 3334.0
- **Average Iterations per Run:** 1665.0
- **Average Time per Run:** 4.458s
- **Function Evaluations per Second:** 374.2
- **Iterations per Second:** 373.5
### Resource Utilization
- **Total Function Evaluations:** 33360
- **Total Gradient Evaluations:** 66680
- **Total Computation Time:** 89.2s
- **Function/Gradient Ratio:** 0.50
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
*Detailed report for GD on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
