# Detailed Analysis: GD-Momentum on LogisticRegression_100samples_5features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LogisticRegression_100samples_5features_reg0.01
**Optimizer:** GD-Momentum
**Problem Family:** Regression
**Dimension:** 5
**Success Threshold:** 3.150e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.153154e-1
* **Worst Final Value:** 3.153170e-1
* **Mean Final Value:** 3.153164e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.177e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.187e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.280e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.062e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.270e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.908</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.217e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.317e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.436e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.414e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.209e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.903</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.246e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.168e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.140e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.282e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.343e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.336e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.261e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.426e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.295e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.974e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 3.153154e-1
**Final Gradient Norm:** 7.973619e-4
**Iterations:** 1665
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.266e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.096e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.046e-1, 1.001e-1, 1.740e-1, 1.306e-1, 1.940e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.266e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.096e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.045e-1, 1.006e-1, 1.751e-1, 1.320e-1, 1.952e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.262e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.094e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.044e-1, 1.016e-1, 1.771e-1, 1.345e-1, 1.973e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.253e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.091e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.042e-1, 1.029e-1, 1.800e-1, 1.380e-1, 2.003e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.241e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.085e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.040e-1, 1.046e-1, 1.837e-1, 1.425e-1, 2.042e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1661</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.055e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 1.098e0, 1.960e0, 2.297e0, 2.547e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1662</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.035e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 1.098e0, 1.960e0, 2.297e0, 2.547e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1663</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.014e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 1.098e0, 1.960e0, 2.297e0, 2.547e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1664</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.994e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 1.098e0, 1.960e0, 2.297e0, 2.547e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1665</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.974e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 1.098e0, 1.960e0, 2.297e0, 2.547e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1668.0
- **Average Gradient Evaluations per Run:** 3334.0
- **Average Iterations per Run:** 1665.0
- **Average Time per Run:** 0.906s
- **Function Evaluations per Second:** 1841.9
- **Iterations per Second:** 1838.6
### Resource Utilization
- **Total Function Evaluations:** 33360
- **Total Gradient Evaluations:** 66680
- **Total Computation Time:** 18.1s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/LogisticRegression_100samples_5features_reg0.01_results.csv)
* [Convergence Plot](../plots/LogisticRegression_100samples_5features_reg0.01.png)
* [Log Scale Convergence Plot](../plots/LogisticRegression_100samples_5features_reg0.01_log.png)


---
*Detailed report for GD-Momentum on LogisticRegression_100samples_5features_reg0.01*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
