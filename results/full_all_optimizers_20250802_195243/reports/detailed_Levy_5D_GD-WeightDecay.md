# Detailed Analysis: GD-WeightDecay on Levy_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_5D
**Optimizer:** GD-WeightDecay
**Problem Family:** Levy
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.926157e-7
* **Worst Final Value:** 9.999347e-7
* **Mean Final Value:** 9.958344e-7
* **Success Rate:** 100.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.939e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.331e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1479</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2955</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.942e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.332e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1432</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.926e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.328e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2897</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.954e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.335e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.975e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.340e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2869</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.976e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.340e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.929e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.329e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1425</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.944e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.332e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.997e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.345e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2873</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.940e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.331e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.990e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.343e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.959e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.336e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2899</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.999e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.345e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.954e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.335e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2855</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.950e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.334e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1455</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1458</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2913</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.997e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.345e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.938e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.331e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2951</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.926e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.328e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2887</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.975e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.340e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1462</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.959e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.336e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 1445.8
* **Average Function Evaluations:** 1448.8
* **Average Time to Convergence:** 0.048s
* **Fastest Convergence:** 1426 iterations (0.046s)
* **Slowest Convergence:** 1476 iterations (0.050s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 9.926157e-7
**Final Gradient Norm:** 5.328281e-4
**Iterations:** 1442
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.916e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.650e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.894e0, 2.165e0, 1.897e0, 1.826e0, 1.939e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.916e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.650e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.885e0, 2.160e0, 1.892e0, 1.820e0, 1.938e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.881e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.647e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.868e0, 2.151e0, 1.882e0, 1.811e0, 1.936e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.818e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.639e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.845e0, 2.139e0, 1.868e0, 1.798e0, 1.933e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.733e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.817e0, 2.123e0, 1.852e0, 1.782e0, 1.929e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1438</td><td style="border: 1px solid #ddd; padding: 4px;">1.023e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.399e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1439</td><td style="border: 1px solid #ddd; padding: 4px;">1.015e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.381e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1440</td><td style="border: 1px solid #ddd; padding: 4px;">1.008e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.363e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1441</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.346e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1442</td><td style="border: 1px solid #ddd; padding: 4px;">9.926e-7</td><td style="border: 1px solid #ddd; padding: 4px;">5.328e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 1.004e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1448.8
- **Average Gradient Evaluations per Run:** 2894.6
- **Average Iterations per Run:** 1445.8
- **Average Time per Run:** 0.048s
- **Function Evaluations per Second:** 30254.3
- **Iterations per Second:** 30191.6
### Resource Utilization
- **Total Function Evaluations:** 28976
- **Total Gradient Evaluations:** 57892
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Levy_5D_results.csv)
* [Convergence Plot](../plots/Levy_5D.png)
* [Log Scale Convergence Plot](../plots/Levy_5D_log.png)


---
*Detailed report for GD-WeightDecay on Levy_5D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
