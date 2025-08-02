# Detailed Analysis: Adam-WeightDecay on Levy_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_10D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Levy
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.414957e-6
* **Worst Final Value:** 5.427938e-6
* **Mean Final Value:** 2.797253e-6
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.809e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.913e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">930</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.807e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.025e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">970</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.373e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.035e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.428e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.319e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1875</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.116e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.784e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">896</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1795</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1794</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.315e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.613e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.957e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.410e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1900</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.752e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.018e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1875</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.641e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.785e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2069</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2068</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.011e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.081e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">998</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1998</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.994e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.119e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">968</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1938</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.400e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.052e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">897</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1797</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1796</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.362e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.041e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">894</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1791</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1790</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.944e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.119e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1817</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.168e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.821e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1805</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1804</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.376e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.307e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">934</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1871</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1870</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.672e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.378e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.415e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.788e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">925</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.263e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.255e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">966</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1935</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1934</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.141e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.101e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">967</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 1.414957e-6
**Final Gradient Norm:** 1.788328e-3
**Iterations:** 925
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.829e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.607e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.032e0, 2.121e0, 2.142e0, 2.002e0, 2.059e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.829e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.607e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.029e0, 2.118e0, 2.139e0, 1.999e0, 2.056e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.796e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.610e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.026e0, 2.115e0, 2.136e0, 1.996e0, 2.053e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.764e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.613e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.023e0, 2.112e0, 2.133e0, 1.993e0, 2.050e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.731e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.616e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.020e0, 2.109e0, 2.130e0, 1.990e0, 2.047e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">920</td><td style="border: 1px solid #ddd; padding: 4px;">1.396e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.766e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e-1, 1.000e0, 1.000e0, 9.994e-1, 9.997e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">921</td><td style="border: 1px solid #ddd; padding: 4px;">1.398e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.770e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e-1, 1.000e0, 1.000e0, 9.994e-1, 9.997e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">922</td><td style="border: 1px solid #ddd; padding: 4px;">1.401e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.774e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.996e-1, 1.000e0, 1.000e0, 9.994e-1, 9.997e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">923</td><td style="border: 1px solid #ddd; padding: 4px;">1.404e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.779e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.996e-1, 1.000e0, 1.000e0, 9.994e-1, 9.997e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">924</td><td style="border: 1px solid #ddd; padding: 4px;">1.409e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.783e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.500e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.996e-1, 1.000e0, 1.000e0, 9.994e-1, 9.997e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1897.2
- **Average Gradient Evaluations per Run:** 1896.2
- **Average Iterations per Run:** 947.1
- **Average Time per Run:** 0.047s
- **Function Evaluations per Second:** 40734.2
- **Iterations per Second:** 20334.9
### Resource Utilization
- **Total Function Evaluations:** 37944
- **Total Gradient Evaluations:** 37924
- **Total Computation Time:** 0.9s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Levy_10D_results.csv)
* [Convergence Plot](../plots/Levy_10D.png)
* [Log Scale Convergence Plot](../plots/Levy_10D_log.png)


---
*Detailed report for Adam-WeightDecay on Levy_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
