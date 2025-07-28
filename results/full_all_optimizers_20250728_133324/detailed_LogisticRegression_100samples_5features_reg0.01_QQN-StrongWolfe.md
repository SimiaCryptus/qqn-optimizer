# Detailed Analysis: QQN-StrongWolfe on LogisticRegression_100samples_5features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LogisticRegression_100samples_5features_reg0.01
**Optimizer:** QQN-StrongWolfe
**Problem Family:** ML Regression
**Dimension:** 5
**Success Threshold:** 3.150e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.153026e-1
* **Worst Final Value:** 3.153026e-1
* **Mean Final Value:** 3.153026e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.231e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.115e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.523e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">67</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.090e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.751e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.806e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.527e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.177e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.152e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.457e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.472e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.712e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.102e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.600e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.714e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.305e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.292e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.647e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.969e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.040e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 11)
**Final Value:** 3.153026e-1
**Final Gradient Norm:** 1.471932e-7
**Iterations:** 11
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.947e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.380e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.099e-1, -2.302e-2, -1.611e-1, 1.528e-1, 1.065e-3]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.947e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.380e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.009e-1, 3.908e-2, -2.408e-2, 2.849e-1, 1.294e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.408e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.155e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.431e-2, 9.503e-2, 9.941e-2, 4.037e-1, 2.474e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.965e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.955e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.992e-1, 6.214e-1, 1.254e0, 1.498e0, 1.604e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.508e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.105e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.714e-1, 8.575e-1, 1.622e0, 1.910e0, 2.081e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">6</td><td style="border: 1px solid #ddd; padding: 4px;">3.154e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.712e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.914e-1, 1.111e0, 1.976e0, 2.315e0, 2.583e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">7</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.785e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.846e-1, 1.109e0, 1.973e0, 2.314e0, 2.568e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">8</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.074e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.848e-1, 1.109e0, 1.973e0, 2.314e0, 2.568e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.103e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.849e-1, 1.109e0, 1.973e0, 2.314e0, 2.568e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.472e-7</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.849e-1, 1.109e0, 1.973e0, 2.314e0, 2.568e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 73.7
- **Average Gradient Evaluations per Run:** 81.6
- **Average Iterations per Run:** 10.9
- **Average Time per Run:** 0.031s
- **Function Evaluations per Second:** 2409.3
- **Iterations per Second:** 358.2
### Resource Utilization
- **Total Function Evaluations:** 1473
- **Total Gradient Evaluations:** 1632
- **Total Computation Time:** 0.6s
- **Function/Gradient Ratio:** 0.90
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/LogisticRegression_100samples_5features_reg0.01_results.csv)
* [Convergence Plot](convergence_LogisticRegression_100samples_5features_reg0.01.png)
* [Log Scale Convergence Plot](convergence_LogisticRegression_100samples_5features_reg0.01_log.png)


---
*Detailed report for QQN-StrongWolfe on LogisticRegression_100samples_5features_reg0.01*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
