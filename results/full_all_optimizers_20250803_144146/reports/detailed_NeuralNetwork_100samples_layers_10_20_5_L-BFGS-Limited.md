# Detailed Analysis: L-BFGS-Limited on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** L-BFGS-Limited
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.919279e-2
* **Worst Final Value:** 1.288499e-1
* **Mean Final Value:** 5.794380e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.523e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.664e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1232</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.864e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.061e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">543</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.871e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.634e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.201e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.428e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">532</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1601</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.641e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.866e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">543</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3371</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.198e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.218e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4093</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">920</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.074e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.488e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.044e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">542</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.630e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.170e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3419</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.472</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.136e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.128e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.268e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.332e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">534</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3403</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1607</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.723e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.210e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.043e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.709e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1652</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.288e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.480e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">227</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">686</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.253e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.208e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.851e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.272e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1598</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.821e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.003e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">524</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3425</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1577</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.499</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.919e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.107e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">543</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.756e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.193e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.398</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.215e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.855e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">544</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1637</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.463</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 1 runs
- MaxFunctionEvaluations: 19 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 3.919279e-2
**Final Gradient Norm:** 5.106623e-2
**Iterations:** 543
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.357e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.389e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.907e-2, -1.026e-1, 1.218e-1, -1.550e-2, -2.703e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.357e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.389e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.779e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.535e-2, -1.090e-1, 1.192e-1, -1.865e-2, -2.604e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.692e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.382e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.765e-2, -1.076e-1, 1.229e-1, -2.199e-2, -2.560e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.608e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.102e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.022e-1, -1.090e-1, 1.245e-1, -2.582e-2, -2.508e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.577e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.532e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.056e-1, -1.078e-1, 1.276e-1, -2.948e-2, -2.472e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">539</td><td style="border: 1px solid #ddd; padding: 4px;">3.936e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.467e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.309e-1, -1.053e-1, 5.924e-1, -1.630e-1, -8.231e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">540</td><td style="border: 1px solid #ddd; padding: 4px;">3.931e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.063e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.302e-1, -1.055e-1, 5.931e-1, -1.634e-1, -8.264e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">541</td><td style="border: 1px solid #ddd; padding: 4px;">3.931e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.030e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.310e-1, -1.052e-1, 5.941e-1, -1.634e-1, -8.322e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">542</td><td style="border: 1px solid #ddd; padding: 4px;">3.926e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.077e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.308e-1, -1.054e-1, 5.936e-1, -1.636e-1, -8.339e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">543</td><td style="border: 1px solid #ddd; padding: 4px;">3.919e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.107e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.308e-1, -1.054e-1, 5.936e-1, -1.636e-1, -8.339e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3421.6
- **Average Gradient Evaluations per Run:** 1467.3
- **Average Iterations per Run:** 487.5
- **Average Time per Run:** 3.299s
- **Function Evaluations per Second:** 1037.0
- **Iterations per Second:** 147.8
### Resource Utilization
- **Total Function Evaluations:** 68432
- **Total Gradient Evaluations:** 29347
- **Total Computation Time:** 66.0s
- **Function/Gradient Ratio:** 2.33
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
*Detailed report for L-BFGS-Limited on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
