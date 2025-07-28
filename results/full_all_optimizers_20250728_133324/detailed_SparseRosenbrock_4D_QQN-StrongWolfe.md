# Detailed Analysis: QQN-StrongWolfe on SparseRosenbrock_4D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_4D
**Optimizer:** QQN-StrongWolfe
**Problem Family:** SparseRosenbrock
**Dimension:** 4
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.799517e-4
* **Worst Final Value:** 6.777555e0
* **Mean Final Value:** 1.942755e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.018e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.865e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">567</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.644e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.723e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">455</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.119e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.438e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.402e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.122e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">585</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.778e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.717e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">614</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.800e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.758e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.898e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.737e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.334e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.188e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">598</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.274e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.755e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.464e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.623e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.482e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.664e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.346e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.599e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">458</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.667e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.696e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">598</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.395e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.139e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.797e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.270e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.378e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.522e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">611</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.999e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.686e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">398</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.558e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.789e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.168e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.625e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">616</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.468e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.042e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 6)
**Final Value:** 2.799517e-4
**Final Gradient Norm:** 4.757633e-2
**Iterations:** 39
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.869e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.089e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.260e0, 8.996e-1, -1.213e0, 8.274e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.869e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.089e2</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.484e0, 1.975e0, 1.259e0, 1.832e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.168e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.968e2</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.351e0, 2.019e0, 1.380e0, 1.784e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.522e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.326e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.408e0, 1.999e0, 1.337e0, 1.800e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.158e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.413e0, 1.997e0, 1.341e0, 1.798e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">35</td><td style="border: 1px solid #ddd; padding: 4px;">4.734e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.949e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.011e0, 1.022e0, 9.873e-1, 9.748e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">36</td><td style="border: 1px solid #ddd; padding: 4px;">2.834e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.846e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.011e0, 1.022e0, 9.873e-1, 9.747e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">37</td><td style="border: 1px solid #ddd; padding: 4px;">2.831e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.508e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.011e0, 1.022e0, 9.871e-1, 9.744e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">38</td><td style="border: 1px solid #ddd; padding: 4px;">2.821e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.462e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.010e0, 1.021e0, 9.870e-1, 9.741e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">39</td><td style="border: 1px solid #ddd; padding: 4px;">2.800e-4</td><td style="border: 1px solid #ddd; padding: 4px;">4.758e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.010e0, 1.021e0, 9.870e-1, 9.741e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 583.1
- **Average Gradient Evaluations per Run:** 442.6
- **Average Iterations per Run:** 28.1
- **Average Time per Run:** 0.016s
- **Function Evaluations per Second:** 35696.4
- **Iterations per Second:** 1720.2
### Resource Utilization
- **Total Function Evaluations:** 11662
- **Total Gradient Evaluations:** 8853
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.32
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/SparseRosenbrock_4D_results.csv)
* [Convergence Plot](convergence_SparseRosenbrock_4D.png)
* [Log Scale Convergence Plot](convergence_SparseRosenbrock_4D_log.png)


---
*Detailed report for QQN-StrongWolfe on SparseRosenbrock_4D*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
