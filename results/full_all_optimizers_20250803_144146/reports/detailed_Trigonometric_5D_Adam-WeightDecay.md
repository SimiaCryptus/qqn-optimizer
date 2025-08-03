# Detailed Analysis: Adam-WeightDecay on Trigonometric_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_5D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Trigonometric
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 14 (70.0%)

### Quick Statistics
* **Best Final Value:** 3.702710e-7
* **Worst Final Value:** 8.532847e-2
* **Mean Final Value:** 1.950852e-2
* **Success Rate:** 70.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.950e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.975e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">855</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">855</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.493e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.482e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.814e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.867e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">823</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">823</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.692e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.870e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">505</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.760e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.714e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">524</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.775e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.990e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">399</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.570e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.478e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">733</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">733</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.550e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.928e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.306e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.859e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.533e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.151e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1237</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.772e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.802e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">959</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">959</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.549e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.961e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.457e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.587e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.451e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.800e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">241</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">240</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.867e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.769e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">484</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.703e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.664e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">309</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">309</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.533e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.160e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.955e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.816e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">494</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.732e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.304e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.901e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.759e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">829</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">829</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (14 out of 20)

* **Average Iterations to Convergence:** 368.6
* **Average Function Evaluations:** 740.3
* **Average Time to Convergence:** 0.017s
* **Fastest Convergence:** 156 iterations (0.007s)
* **Slowest Convergence:** 524 iterations (0.024s)

### Failed Runs (6 out of 20)

**Failure Reasons:**
- FunctionTolerance: 6 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 16)
**Final Value:** 3.702710e-7
**Final Gradient Norm:** 3.664339e-3
**Iterations:** 153
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.047e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.379e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.600e-2, 2.873e-2, 2.539e-1, 2.641e-1, 1.709e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.047e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.379e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.900e-2, 3.173e-2, 2.509e-1, 2.611e-1, 1.409e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.028e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.380e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.200e-2, 3.471e-2, 2.479e-1, 2.581e-1, 1.242e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.010e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.377e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.500e-2, 3.764e-2, 2.449e-1, 2.551e-1, 1.251e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.921e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.366e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.800e-2, 4.050e-2, 2.419e-1, 2.521e-1, 1.333e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">149</td><td style="border: 1px solid #ddd; padding: 4px;">1.786e-5</td><td style="border: 1px solid #ddd; padding: 4px;">9.134e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.133e-3, 7.335e-6, 3.425e-5, 1.134e-4, 7.161e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">150</td><td style="border: 1px solid #ddd; padding: 4px;">9.968e-6</td><td style="border: 1px solid #ddd; padding: 4px;">7.191e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.138e-3, 2.559e-6, 1.130e-6, 1.121e-4, -3.636e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">151</td><td style="border: 1px solid #ddd; padding: 4px;">4.752e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.547e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.227e-3, -2.041e-6, -2.879e-5, 1.101e-4, 2.397e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">152</td><td style="border: 1px solid #ddd; padding: 4px;">1.704e-6</td><td style="border: 1px solid #ddd; padding: 4px;">4.314e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.977e-4, -5.704e-6, -5.553e-5, 1.074e-4, 6.156e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">153</td><td style="border: 1px solid #ddd; padding: 4px;">3.703e-7</td><td style="border: 1px solid #ddd; padding: 4px;">3.664e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.977e-4, -5.704e-6, -5.553e-5, 1.074e-4, 6.156e-7]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 784.6
- **Average Gradient Evaluations per Run:** 784.3
- **Average Iterations per Run:** 390.8
- **Average Time per Run:** 0.018s
- **Function Evaluations per Second:** 43345.5
- **Iterations per Second:** 21589.9
### Resource Utilization
- **Total Function Evaluations:** 15692
- **Total Gradient Evaluations:** 15686
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Trigonometric_5D_results.csv)
* [Convergence Plot](../plots/Trigonometric_5D.png)
* [Log Scale Convergence Plot](../plots/Trigonometric_5D_log.png)


---
*Detailed report for Adam-WeightDecay on Trigonometric_5D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
