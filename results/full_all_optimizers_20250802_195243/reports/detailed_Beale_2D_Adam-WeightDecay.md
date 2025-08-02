# Detailed Analysis: Adam-WeightDecay on Beale_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Beale_2D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Beale
**Dimension:** 2
**Success Threshold:** 1.500e-2
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 1.492714e-2
* **Worst Final Value:** 1.499792e-2
* **Mean Final Value:** 1.496454e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.493e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.273e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">700</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1403</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1403</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.271e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">795</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1593</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1593</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.496e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.323e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">682</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.499e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.300e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">659</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.286e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">781</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.499e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.279e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">688</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.493e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.268e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">783</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.267e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.500e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.264e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">803</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1609</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1609</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.493e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.274e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.493e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.277e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">839</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.494e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.301e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.493e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.307e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">676</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.263e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.500e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.320e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.264e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">930</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.275e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">846</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1695</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1695</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.496e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.297e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">739</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.263e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">873</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1749</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1749</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.310e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">711</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1425</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1425</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 768.5
* **Average Function Evaluations:** 1539.9
* **Average Time to Convergence:** 0.031s
* **Fastest Convergence:** 659 iterations (0.027s)
* **Slowest Convergence:** 930 iterations (0.037s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 1.492714e-2
**Final Gradient Norm:** 1.272644e-1
**Iterations:** 700
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.099e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.421e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.175e0, 8.859e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.099e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.421e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.178e0, 8.829e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.091e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.408e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.181e0, 8.799e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.083e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.394e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.184e0, 8.769e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.075e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.381e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.187e0, 8.739e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">696</td><td style="border: 1px solid #ddd; padding: 4px;">1.524e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.289e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.736e0, 4.255e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">697</td><td style="border: 1px solid #ddd; padding: 4px;">1.516e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.285e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.737e0, 4.257e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">698</td><td style="border: 1px solid #ddd; padding: 4px;">1.508e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.281e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.738e0, 4.259e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">699</td><td style="border: 1px solid #ddd; padding: 4px;">1.501e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.277e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.738e0, 4.261e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">700</td><td style="border: 1px solid #ddd; padding: 4px;">1.493e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.273e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.738e0, 4.261e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1539.9
- **Average Gradient Evaluations per Run:** 1539.9
- **Average Iterations per Run:** 768.5
- **Average Time per Run:** 0.031s
- **Function Evaluations per Second:** 48965.5
- **Iterations per Second:** 24435.1
### Resource Utilization
- **Total Function Evaluations:** 30798
- **Total Gradient Evaluations:** 30798
- **Total Computation Time:** 0.6s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Beale_2D_results.csv)
* [Convergence Plot](../plots/Beale_2D.png)
* [Log Scale Convergence Plot](../plots/Beale_2D_log.png)


---
*Detailed report for Adam-WeightDecay on Beale_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
