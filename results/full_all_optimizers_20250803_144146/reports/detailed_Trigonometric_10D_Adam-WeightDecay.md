# Detailed Analysis: Adam-WeightDecay on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 14 (70.0%)

### Quick Statistics
* **Best Final Value:** 8.693556e-7
* **Worst Final Value:** 5.200158e-1
* **Mean Final Value:** 4.247609e-2
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
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.641e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.424e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.511e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.841e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.963e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.171e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.817e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.815e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.316e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.818e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.741e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.890e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.200e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.409e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">704</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.935e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.755e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.671e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.864e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.694e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.138e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2309</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2309</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.874e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.573e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">304</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.665e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.838e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">683</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">683</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.884e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.828e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">793</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">793</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.126e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.479e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">468</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">938</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.744e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.895e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">468</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.220e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.627e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.717e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.390e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">205</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.717e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.098e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">297</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.393e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.947e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.939e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.925e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (14 out of 20)

* **Average Iterations to Convergence:** 467.2
* **Average Function Evaluations:** 937.4
* **Average Time to Convergence:** 0.026s
* **Fastest Convergence:** 209 iterations (0.012s)
* **Slowest Convergence:** 1153 iterations (0.063s)

### Failed Runs (6 out of 20)

**Failure Reasons:**
- FunctionTolerance: 6 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 8.693556e-7
**Final Gradient Norm:** 2.138210e-3
**Iterations:** 1153
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.748e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.186e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.105e-2, 3.550e-1, 2.991e-1, 9.586e-2, 2.597e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.748e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.186e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.405e-2, 3.580e-1, 3.021e-1, 9.886e-2, 2.567e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.727e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.197e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.706e-2, 3.610e-1, 3.051e-1, 1.019e-1, 2.537e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.705e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.208e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.006e-2, 3.640e-1, 3.081e-1, 1.049e-1, 2.507e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.684e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.219e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.306e-2, 3.670e-1, 3.111e-1, 1.078e-1, 2.477e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1149</td><td style="border: 1px solid #ddd; padding: 4px;">2.497e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.159e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.691e-1, 2.354e-1, 1.483e-1, 1.087e-1, 8.584e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1150</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.930e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.689e-1, 2.352e-1, 1.483e-1, 1.086e-1, 8.581e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1151</td><td style="border: 1px solid #ddd; padding: 4px;">1.509e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.594e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.688e-1, 2.351e-1, 1.482e-1, 1.086e-1, 8.578e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1152</td><td style="border: 1px solid #ddd; padding: 4px;">1.152e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.355e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.687e-1, 2.349e-1, 1.481e-1, 1.086e-1, 8.575e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1153</td><td style="border: 1px solid #ddd; padding: 4px;">8.694e-7</td><td style="border: 1px solid #ddd; padding: 4px;">2.138e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.687e-1, 2.349e-1, 1.481e-1, 1.086e-1, 8.575e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 803.8
- **Average Gradient Evaluations per Run:** 803.5
- **Average Iterations per Run:** 400.4
- **Average Time per Run:** 0.022s
- **Function Evaluations per Second:** 35734.7
- **Iterations per Second:** 17800.6
### Resource Utilization
- **Total Function Evaluations:** 16076
- **Total Gradient Evaluations:** 16070
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Trigonometric_10D_results.csv)
* [Convergence Plot](../plots/Trigonometric_10D.png)
* [Log Scale Convergence Plot](../plots/Trigonometric_10D_log.png)


---
*Detailed report for Adam-WeightDecay on Trigonometric_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
