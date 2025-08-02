# Detailed Analysis: L-BFGS-Conservative on SparseRosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_10D
**Optimizer:** L-BFGS-Conservative
**Problem Family:** SparseRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.962709e-2
* **Worst Final Value:** 2.102430e8
* **Mean Final Value:** 1.051216e7
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.421e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.051e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3737</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1267</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.754e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.394e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">240</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">972</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.963e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.865e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">272</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3910</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1097</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.053e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.808e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">229</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">923</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.551e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.884e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">246</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">994</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.077e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.339e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.888e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.936e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.315e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.439e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.052e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.425e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.329e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.513e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3690</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.039e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.260e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.706e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.137e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">996</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.512e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.363e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">244</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.587e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.471e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3824</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.049e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.414e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3753</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.102e8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.144e7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">218</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.667e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.233e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3947</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.812e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.512e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">257</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3973</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.817e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.395e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">232</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">935</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.729e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.380e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3966</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 17 runs
- FunctionTolerance: 3 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 2.962709e-2
**Final Gradient Norm:** 3.864541e-1
**Iterations:** 272
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.579e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.439e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.360e0, 1.009e0, -1.107e0, 8.151e-1, -1.216e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.579e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.439e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.396e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.260e0, 1.046e0, -1.067e0, 8.329e-1, -1.149e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.453e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.136e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.493e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.160e0, 1.085e0, -1.018e0, 8.550e-1, -1.075e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.875e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.065e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.654e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.060e0, 1.126e0, -9.557e-1, 8.841e-1, -9.888e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.079e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.203e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.563e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.056e0, 1.125e0, -9.321e-1, 8.933e-1, -9.676e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">268</td><td style="border: 1px solid #ddd; padding: 4px;">2.963e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.879e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.883e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.070e0, 1.145e0, 8.501e-1, 7.223e-1, 1.043e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">269</td><td style="border: 1px solid #ddd; padding: 4px;">2.963e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.678e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.221e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.070e0, 1.145e0, 8.501e-1, 7.223e-1, 1.043e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">270</td><td style="border: 1px solid #ddd; padding: 4px;">2.963e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.871e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.883e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.070e0, 1.144e0, 8.500e-1, 7.222e-1, 1.043e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">271</td><td style="border: 1px solid #ddd; padding: 4px;">2.963e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.630e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.221e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.070e0, 1.144e0, 8.501e-1, 7.223e-1, 1.043e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">272</td><td style="border: 1px solid #ddd; padding: 4px;">2.963e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.865e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.441e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.070e0, 1.144e0, 8.501e-1, 7.223e-1, 1.043e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3686.9
- **Average Gradient Evaluations per Run:** 974.5
- **Average Iterations per Run:** 241.4
- **Average Time per Run:** 0.039s
- **Function Evaluations per Second:** 95525.8
- **Iterations per Second:** 6254.6
### Resource Utilization
- **Total Function Evaluations:** 73738
- **Total Gradient Evaluations:** 19489
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 3.78
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/SparseRosenbrock_10D_results.csv)
* [Convergence Plot](../plots/SparseRosenbrock_10D.png)
* [Log Scale Convergence Plot](../plots/SparseRosenbrock_10D_log.png)


---
*Detailed report for L-BFGS-Conservative on SparseRosenbrock_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
