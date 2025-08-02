# Detailed Analysis: Adam on Rastrigin_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rastrigin_2D
**Optimizer:** Adam
**Problem Family:** Rastrigin
**Dimension:** 2
**Success Threshold:** 7.960e0
**Total Runs:** 20
**Successful Runs:** 11 (55.0%)

### Quick Statistics
* **Best Final Value:** 7.959684e0
* **Worst Final Value:** 1.293653e1
* **Mean Final Value:** 1.019922e1
* **Success Rate:** 55.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.793e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.619e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.462e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">434</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">871</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">871</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.104e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">280</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.264e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.971e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.205e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">427</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.418e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">505</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.673e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.275e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">314</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.239e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">855</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.196e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">424</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.174e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">784</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.205e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">432</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">866</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.242e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.333e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">717</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">717</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.288e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">749</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">748</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.324e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.960e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.095e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.198e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">836</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (11 out of 20)

* **Average Iterations to Convergence:** 374.3
* **Average Function Evaluations:** 751.5
* **Average Time to Convergence:** 0.015s
* **Fastest Convergence:** 280 iterations (0.012s)
* **Slowest Convergence:** 505 iterations (0.021s)

### Failed Runs (9 out of 20)

**Failure Reasons:**
- FunctionTolerance: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 7.959684e0
**Final Gradient Norm:** 1.323585e-1
**Iterations:** 337
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.238e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.148e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.181e0, 2.358e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.238e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.148e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.180e0, 2.357e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.226e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.152e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.179e0, 2.356e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.215e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.155e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.178e0, 2.355e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.203e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.158e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.177e0, 2.354e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">333</td><td style="border: 1px solid #ddd; padding: 4px;">7.964e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.826e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.993e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">334</td><td style="border: 1px solid #ddd; padding: 4px;">7.962e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.396e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.992e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">335</td><td style="border: 1px solid #ddd; padding: 4px;">7.961e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.672e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.991e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">336</td><td style="border: 1px solid #ddd; padding: 4px;">7.960e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.397e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.990e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">337</td><td style="border: 1px solid #ddd; padding: 4px;">7.960e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.324e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.990e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 780.5
- **Average Gradient Evaluations per Run:** 780.0
- **Average Iterations per Run:** 388.8
- **Average Time per Run:** 0.016s
- **Function Evaluations per Second:** 48704.8
- **Iterations per Second:** 24258.8
### Resource Utilization
- **Total Function Evaluations:** 15610
- **Total Gradient Evaluations:** 15601
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Rastrigin_2D_results.csv)
* [Convergence Plot](../plots/Rastrigin_2D.png)
* [Log Scale Convergence Plot](../plots/Rastrigin_2D_log.png)


---
*Detailed report for Adam on Rastrigin_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
