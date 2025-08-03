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
* **Best Final Value:** 1.193612e-1
* **Worst Final Value:** 1.331599e-1
* **Mean Final Value:** 1.279330e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.302e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.245e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.422</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.293e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.542e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.260e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.870e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.645</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.316e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.726e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.332e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.809e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.317e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.897e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.212e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.632e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.298e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.182e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.287e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.899e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.266e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.249e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.253e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.864e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.679</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.258e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.271e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.339e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.620</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.270e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.866e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.596</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.271e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.041e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.504</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.383e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.496</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.305e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.634e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.289e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.701e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.277e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.400e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.194e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.789e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.551</td>
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
**Final Value:** 1.193612e-1
**Final Gradient Norm:** 9.788594e-2
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.935e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.607e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.783e-1, -1.145e-1, 2.598e-1, -1.096e-1, -1.368e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.935e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.607e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.779e-1, -1.143e-1, 2.599e-1, -1.094e-1, -1.368e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.917e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.280e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.775e-1, -1.141e-1, 2.601e-1, -1.092e-1, -1.369e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.900e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.969e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.771e-1, -1.139e-1, 2.602e-1, -1.090e-1, -1.369e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.884e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.671e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.767e-1, -1.137e-1, 2.604e-1, -1.088e-1, -1.370e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1661</td><td style="border: 1px solid #ddd; padding: 4px;">1.194e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.854e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.202e-1, -2.667e-2, 2.308e-1, -2.698e-2, -1.950e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1662</td><td style="border: 1px solid #ddd; padding: 4px;">1.194e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.777e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.202e-1, -2.660e-2, 2.308e-1, -2.694e-2, -1.951e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1663</td><td style="border: 1px solid #ddd; padding: 4px;">1.194e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.743e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.203e-1, -2.652e-2, 2.307e-1, -2.690e-2, -1.951e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1664</td><td style="border: 1px solid #ddd; padding: 4px;">1.194e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.967e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.202e-1, -2.647e-2, 2.307e-1, -2.686e-2, -1.952e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1665</td><td style="border: 1px solid #ddd; padding: 4px;">1.194e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.789e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.202e-1, -2.647e-2, 2.307e-1, -2.686e-2, -1.952e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1668.0
- **Average Gradient Evaluations per Run:** 3334.0
- **Average Iterations per Run:** 1665.0
- **Average Time per Run:** 4.558s
- **Function Evaluations per Second:** 366.0
- **Iterations per Second:** 365.3
### Resource Utilization
- **Total Function Evaluations:** 33360
- **Total Gradient Evaluations:** 66680
- **Total Computation Time:** 91.2s
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
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
