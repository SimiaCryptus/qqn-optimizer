# Detailed Analysis: QQN-Bisection-2 on Michalewicz_10D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_10D_m10
**Optimizer:** QQN-Bisection-2
**Problem Family:** Highly Multimodal
**Dimension:** 10
**Success Threshold:** -6.260e0
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -5.905017e0
* **Worst Final Value:** -9.512502e-1
* **Mean Final Value:** -3.502816e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.053e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.819e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">682</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.684e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.007e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">219</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.766e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.314e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">593</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-9.513e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.155e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.757e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.140e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">678</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">327</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.524e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.123e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.566e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.154e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">661</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.757e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.775e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">654</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">372</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.478e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.068e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.603e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">545</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.905e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.361e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">652</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.462e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.652e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">670</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">342</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.224e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.746e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">inf</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.612e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.708e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">363</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- NumericalError: 10 runs
- MaxFunctionEvaluations: 9 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 16)
**Final Value:** -5.905017e0
**Final Gradient Norm:** 6.360959e0
**Iterations:** 36
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-7.359e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.303e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 8.244e-1, 9.778e-1, 8.648e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-7.359e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.303e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 8.245e-1, 1.088e0, 9.634e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-3.886e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.323e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 8.245e-1, 1.110e0, 9.937e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-4.481e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 8.245e-1, 1.114e0, 9.982e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-4.511e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.724e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 8.245e-1, 1.113e0, 9.965e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">32</td><td style="border: 1px solid #ddd; padding: 4px;">-5.383e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.286e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 1.242e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">33</td><td style="border: 1px solid #ddd; padding: 4px;">-5.439e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.095e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.125e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 1.237e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">34</td><td style="border: 1px solid #ddd; padding: 4px;">-5.496e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.924e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 1.177e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">35</td><td style="border: 1px solid #ddd; padding: 4px;">-5.542e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.043e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 1.196e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">36</td><td style="border: 1px solid #ddd; padding: 4px;">-5.905e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.361e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.325e-1, 6.011e-1, 1.196e0, 1.114e0, 9.967e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 349.7
- **Average Gradient Evaluations per Run:** 196.8
- **Average Iterations per Run:** 19.1
- **Average Time per Run:** 0.009s
- **Function Evaluations per Second:** 39392.0
- **Iterations per Second:** 2145.9
### Resource Utilization
- **Total Function Evaluations:** 6994
- **Total Gradient Evaluations:** 3935
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.78
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 10
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Michalewicz_10D_m10_results.csv)
* [Convergence Plot](convergence_Michalewicz_10D_m10.png)
* [Log Scale Convergence Plot](convergence_Michalewicz_10D_m10_log.png)


---
*Detailed report for QQN-Bisection-2 on Michalewicz_10D_m10*
*Generated on: 2025-07-28 13:54:23 UTC*
*[← Back to Main Report](benchmark_report.md)*
