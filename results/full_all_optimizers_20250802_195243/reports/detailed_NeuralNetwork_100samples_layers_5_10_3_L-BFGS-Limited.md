# Detailed Analysis: L-BFGS-Limited on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** L-BFGS-Limited
**Problem Family:** Neural Networks
**Dimension:** 93
**Success Threshold:** 1.400e-1
**Total Runs:** 20
**Successful Runs:** 9 (45.0%)

### Quick Statistics
* **Best Final Value:** 1.399191e-1
* **Worst Final Value:** 1.764119e-1
* **Mean Final Value:** 1.471216e-1
* **Success Rate:** 45.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.547e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.365e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">308</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">929</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.679e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.593e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">838</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">311</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.507e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.963e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.732</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.583e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">259</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">780</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.841</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.478e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.340e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.500e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.280e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3657</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.693</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.173e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1654</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">879</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.422e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.017e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1667</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.895e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3401</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1613</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.808</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.726e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">268</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1468</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.060e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">511</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3098</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.674</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.419e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.247e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">455</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.691</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.430e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.305e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.716</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.358e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">267</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">804</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.548e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">268</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.433e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.308e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">488</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2861</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.597</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.672e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.160e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.233e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">260</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">783</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.826</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.764e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.903e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3872</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (9 out of 20)

* **Average Iterations to Convergence:** 330.4
* **Average Function Evaluations:** 1897.3
* **Average Time to Convergence:** 1.065s
* **Fastest Convergence:** 260 iterations (0.826s)
* **Slowest Convergence:** 511 iterations (1.674s)

### Failed Runs (11 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 7 runs
- FunctionTolerance: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 7)
**Final Value:** 1.399191e-1
**Final Gradient Norm:** 6.173484e-2
**Iterations:** 292
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.243e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.234e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.613e-1, 1.721e-1, 8.121e-2, 5.501e-2, -9.830e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.243e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.234e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.691e-1, 1.723e-1, 7.574e-2, 5.501e-2, -1.067e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.296e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.508e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.745e-1, 1.644e-1, 7.309e-2, 5.501e-2, -1.147e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.219e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.397e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.824e-1, 1.639e-1, 7.054e-2, 5.501e-2, -1.225e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.199e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.153e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.899e-1, 1.636e-1, 6.810e-2, 5.501e-2, -1.297e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">288</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.239e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.319e-1, 4.598e-1, 1.530e-1, 5.501e-2, -5.881e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">289</td><td style="border: 1px solid #ddd; padding: 4px;">1.405e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.179e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.328e-1, 4.617e-1, 1.543e-1, 5.501e-2, -5.891e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">290</td><td style="border: 1px solid #ddd; padding: 4px;">1.403e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.067e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.327e-1, 4.622e-1, 1.538e-1, 5.501e-2, -5.918e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">291</td><td style="border: 1px solid #ddd; padding: 4px;">1.401e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.008e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.337e-1, 4.642e-1, 1.554e-1, 5.501e-2, -5.936e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">292</td><td style="border: 1px solid #ddd; padding: 4px;">1.399e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.173e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.337e-1, 4.642e-1, 1.554e-1, 5.501e-2, -5.936e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2641.3
- **Average Gradient Evaluations per Run:** 1109.3
- **Average Iterations per Run:** 368.6
- **Average Time per Run:** 1.307s
- **Function Evaluations per Second:** 2020.6
- **Iterations per Second:** 282.0
### Resource Utilization
- **Total Function Evaluations:** 52826
- **Total Gradient Evaluations:** 22186
- **Total Computation Time:** 26.1s
- **Function/Gradient Ratio:** 2.38
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
*Detailed report for L-BFGS-Limited on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
