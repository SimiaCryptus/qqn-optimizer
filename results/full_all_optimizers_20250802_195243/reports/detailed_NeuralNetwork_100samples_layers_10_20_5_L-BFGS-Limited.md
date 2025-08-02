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
* **Best Final Value:** 4.163589e-2
* **Worst Final Value:** 8.186213e-2
* **Mean Final Value:** 5.064232e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.348e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.520e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1658</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.653e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.164e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">532</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1601</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.458e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.816e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.482e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.796e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3393</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.600</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.593e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.855e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3434</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.512</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.582e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.912e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.532</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.186e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.189e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.622e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.621e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">544</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1637</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.071e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.473e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3408</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.164e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.093e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.775e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.759e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.501e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.048e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.639e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.447e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1652</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.714e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.354e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1652</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.671</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.307e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.895e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">541</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.607</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.219e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.032e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.562</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.241e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.707e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.357e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.348e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1619</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.433e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.567e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.939e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.061e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1616</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 19 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 4.163589e-2
**Final Gradient Norm:** 6.092800e-2
**Iterations:** 546
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.492e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.536e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.246e-1, 7.823e-2, -9.284e-2, 3.645e-2, -1.452e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.492e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.536e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.163e-1, 7.570e-2, -8.846e-2, 3.759e-2, -1.274e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.724e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.327e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.175e-1, 7.721e-2, -8.863e-2, 3.575e-2, -1.279e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.609e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.011e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.174e-1, 7.563e-2, -8.820e-2, 3.542e-2, -1.242e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.582e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.382e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.174e-1, 7.368e-2, -8.706e-2, 3.534e-2, -1.219e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">542</td><td style="border: 1px solid #ddd; padding: 4px;">4.188e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.151e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.501e-1, -6.537e-2, -8.231e-2, 4.481e-1, 3.195e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">543</td><td style="border: 1px solid #ddd; padding: 4px;">4.185e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.378e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.517e-1, -6.251e-2, -8.199e-2, 4.465e-1, 3.183e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">544</td><td style="border: 1px solid #ddd; padding: 4px;">4.183e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.392e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.510e-1, -6.247e-2, -8.056e-2, 4.464e-1, 3.188e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">545</td><td style="border: 1px solid #ddd; padding: 4px;">4.175e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.109e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.516e-1, -6.289e-2, -8.042e-2, 4.464e-1, 3.187e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">546</td><td style="border: 1px solid #ddd; padding: 4px;">4.164e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.093e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.516e-1, -6.289e-2, -8.042e-2, 4.464e-1, 3.187e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3290.1
- **Average Gradient Evaluations per Run:** 1552.5
- **Average Iterations per Run:** 515.9
- **Average Time per Run:** 3.444s
- **Function Evaluations per Second:** 955.2
- **Iterations per Second:** 149.8
### Resource Utilization
- **Total Function Evaluations:** 65802
- **Total Gradient Evaluations:** 31051
- **Total Computation Time:** 68.9s
- **Function/Gradient Ratio:** 2.12
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
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
