# Detailed Analysis: Trust Region-Adaptive on Himmelblau_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Himmelblau_2D
**Optimizer:** Trust Region-Adaptive
**Problem Family:** Himmelblau
**Dimension:** 2
**Success Threshold:** 2.500e-1
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 7.130029e-2
* **Worst Final Value:** 2.458614e-1
* **Mean Final Value:** 1.318004e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.744e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.191e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">687</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1377</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.130e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.974e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2097</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1399</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.646e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">684</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1371</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.448e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.413e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">693</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.819e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.904e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.682e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.076e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.339e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.028e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.249e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.384e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">692</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2079</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.214e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.134e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">702</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.345e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.243e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.637e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">682</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.428e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.741e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1401</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.135e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.494e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.312e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.699e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.044e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.398e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">685</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.786e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.320e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">683</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.459e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.753e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">688</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.443e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.371e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">657</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1974</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.261e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.592e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.382e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.886e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">683</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 681.7
* **Average Function Evaluations:** 2048.1
* **Average Time to Convergence:** 0.013s
* **Fastest Convergence:** 668 iterations (0.012s)
* **Slowest Convergence:** 666 iterations (0.027s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 7.130029e-2
**Final Gradient Norm:** 1.974346e0
**Iterations:** 698
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.743e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.967e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.912e-1, -1.183e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.743e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.967e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.271e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.874e-1, -1.062e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.741e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.003e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.248e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.836e-1, -9.431e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.738e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.039e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.226e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.799e-1, -8.263e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.736e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.074e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.205e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.762e-1, -7.117e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">694</td><td style="border: 1px solid #ddd; padding: 4px;">9.291e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.796e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.207e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.879e0, 2.183e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">695</td><td style="border: 1px solid #ddd; padding: 4px;">6.978e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.642e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.764e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.906e0, 2.157e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">696</td><td style="border: 1px solid #ddd; padding: 4px;">4.725e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.350e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.673e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.937e0, 2.122e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">697</td><td style="border: 1px solid #ddd; padding: 4px;">2.583e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.854e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.487e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.972e0, 2.068e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">698</td><td style="border: 1px solid #ddd; padding: 4px;">7.130e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.974e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.972e0, 2.068e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2048.1
- **Average Gradient Evaluations per Run:** 1366.4
- **Average Iterations per Run:** 681.7
- **Average Time per Run:** 0.013s
- **Function Evaluations per Second:** 155236.9
- **Iterations per Second:** 51669.8
### Resource Utilization
- **Total Function Evaluations:** 40962
- **Total Gradient Evaluations:** 27328
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Himmelblau_2D_results.csv)
* [Convergence Plot](../plots/Himmelblau_2D.png)
* [Log Scale Convergence Plot](../plots/Himmelblau_2D_log.png)


---
*Detailed report for Trust Region-Adaptive on Himmelblau_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
