# Detailed Analysis: Trust Region-Precise on Rosenbrock_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rosenbrock_2D
**Optimizer:** Trust Region-Precise
**Problem Family:** Rosenbrock
**Dimension:** 2
**Success Threshold:** 8.450e-3
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.756281e0
* **Worst Final Value:** 3.520108e1
* **Mean Final Value:** 7.264419e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.520e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.986e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.375e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.086e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.304e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.434e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.886e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.282e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">718</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.222e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.972e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">458</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">306</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.402e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.291e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">752</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.311e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.146e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.151e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.085e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.085e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.029e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2096</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1398</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.756e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.196e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">869</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">580</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.308e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.053e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.385e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.382e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">272</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.981e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.085e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.461e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.304e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.892e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.046e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.122e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.466e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.477e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.593e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.929e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.243e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">64</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.274e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.665e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.323e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.766e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">420</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">842</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 3 runs
- GradientTolerance: 17 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 3.756281e0
**Final Gradient Norm:** 3.196375e0
**Iterations:** 289
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.164e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.052e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.105e0, 8.072e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.164e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.052e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.046e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.105e0, 8.073e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.158e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.048e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.052e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.105e0, 8.074e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.152e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.044e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.058e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.105e0, 8.075e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.145e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.040e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.064e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.104e0, 8.076e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">284</td><td style="border: 1px solid #ddd; padding: 4px;">3.964e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.968e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.176e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.526e-1, 8.768e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">285</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.672e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.739e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.491e-1, 8.782e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">286</td><td style="border: 1px solid #ddd; padding: 4px;">3.850e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.327e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.710e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.447e-1, 8.798e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">287</td><td style="border: 1px solid #ddd; padding: 4px;">3.798e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.012e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.935e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.380e-1, 8.818e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">288</td><td style="border: 1px solid #ddd; padding: 4px;">3.756e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.196e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.380e-1, 8.818e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 946.2
- **Average Gradient Evaluations per Run:** 631.5
- **Average Iterations per Run:** 314.6
- **Average Time per Run:** 0.006s
- **Function Evaluations per Second:** 162646.0
- **Iterations per Second:** 54075.0
### Resource Utilization
- **Total Function Evaluations:** 18925
- **Total Gradient Evaluations:** 12630
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Rosenbrock_2D_results.csv)
* [Convergence Plot](../plots/Rosenbrock_2D.png)
* [Log Scale Convergence Plot](../plots/Rosenbrock_2D_log.png)


---
*Detailed report for Trust Region-Precise on Rosenbrock_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
