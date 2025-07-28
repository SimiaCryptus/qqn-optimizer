# Detailed Analysis: Trust Region-Standard on Rastrigin_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rastrigin_10D
**Optimizer:** Trust Region-Standard
**Problem Family:** Highly Multimodal
**Dimension:** 10
**Success Threshold:** 4.180e1
**Total Runs:** 20
**Successful Runs:** 7 (35.0%)

### Quick Statistics
* **Best Final Value:** 3.400681e1
* **Worst Final Value:** 6.842408e1
* **Mean Final Value:** 4.590364e1
* **Success Rate:** 35.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.482e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.094e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">401</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">268</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.692e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.008e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">274</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.985e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.371e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.896e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.094e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.680e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.688e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">228</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.482e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.928e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.401e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.187e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.488e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.073e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">392</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.195e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.115e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.688e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.750e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.979e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.013e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.706e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.528e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.477e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.217e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">258</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.682e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.327e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.494e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.179e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.137e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">234</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.487e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.558e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.842e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.125e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.975e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.780e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">266</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.998e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.339e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">228</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (7 out of 20)
- **Average Iterations to Convergence:** 124.7
- **Average Function Evaluations:** 376.1
- **Average Time to Convergence:** 0.003s
- **Fastest Convergence:** 109 iterations (0.002s)
- **Slowest Convergence:** 142 iterations (0.003s)
### Failed Runs (13 out of 20)

**Failure Reasons:**
- GradientTolerance: 12 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 7)
**Final Value:** 3.400681e1
**Final Gradient Norm:** 1.187467e1
**Iterations:** 142
**Convergence Reason:** GradientTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.739e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.864e0, 2.418e0, 2.272e0, 2.238e0, 1.481e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.739e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.721e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.866e0, 2.417e0, 2.269e0, 2.235e0, 1.481e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.729e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.491e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.707e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.868e0, 2.415e0, 2.266e0, 2.232e0, 1.480e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.719e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.494e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.693e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.870e0, 2.414e0, 2.263e0, 2.229e0, 1.480e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.709e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.497e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.680e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.872e0, 2.412e0, 2.260e0, 2.226e0, 1.479e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">137</td><td style="border: 1px solid #ddd; padding: 4px;">3.744e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.110e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.957e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.987e0, 2.017e0, 1.999e0, 1.997e0, 1.097e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">138</td><td style="border: 1px solid #ddd; padding: 4px;">3.651e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.455e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.245e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.987e0, 2.012e0, 1.997e0, 1.996e0, 1.078e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">139</td><td style="border: 1px solid #ddd; padding: 4px;">3.560e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.663e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.730e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.988e0, 2.005e0, 1.995e0, 1.994e0, 1.054e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">140</td><td style="border: 1px solid #ddd; padding: 4px;">3.473e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.650e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.773e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.989e0, 1.997e0, 1.992e0, 1.992e0, 1.021e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">141</td><td style="border: 1px solid #ddd; padding: 4px;">3.401e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.187e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.989e0, 1.997e0, 1.992e0, 1.992e0, 1.021e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 366.4
- **Average Gradient Evaluations per Run:** 244.9
- **Average Iterations per Run:** 121.5
- **Average Time per Run:** 0.003s
- **Function Evaluations per Second:** 142829.5
- **Iterations per Second:** 47343.4
### Resource Utilization
- **Total Function Evaluations:** 7328
- **Total Gradient Evaluations:** 4898
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Rastrigin_10D_results.csv)
* [Convergence Plot](convergence_Rastrigin_10D.png)
* [Log Scale Convergence Plot](convergence_Rastrigin_10D_log.png)


---
*Detailed report for Trust Region-Standard on Rastrigin_10D*
*Generated on: 2025-07-28 13:54:23 UTC*
*[← Back to Main Report](benchmark_report.md)*
