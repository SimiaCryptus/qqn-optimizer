# Detailed Analysis: QQN-Backtracking on IllConditionedRosenbrock_2D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_2D_alpha100
**Optimizer:** QQN-Backtracking
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.114523e-5
* **Worst Final Value:** 3.186577e0
* **Mean Final Value:** 4.637709e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.115e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.472e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">717</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.627e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.961e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">307</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.787e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.022e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">712</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.286e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.195e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">717</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.130e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.166e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">734</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.187e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.169e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">726</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.044e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.181e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.779e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.253e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">702</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.659e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.572e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.105e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.873e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">722</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.347e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.035e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">745</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.345e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.076e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">722</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.172e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.788e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.316e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.234e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">744</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.444e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.595e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">710</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">307</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.729e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.799e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">748</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.103e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.121e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.580e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.295e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.969e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.588e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">712</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">307</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.275e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.105e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 3.114523e-5
**Final Gradient Norm:** 2.471693e-1
**Iterations:** 48
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.149e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.258e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.234e0, 9.181e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.149e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.258e2</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.130e0, 1.862e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.415e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.886e2</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.388e0, 1.748e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.354e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.064e2</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.290e0, 1.783e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.469e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.460e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.349e0, 1.760e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">44</td><td style="border: 1px solid #ddd; padding: 4px;">4.469e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.971e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.002e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">45</td><td style="border: 1px solid #ddd; padding: 4px;">4.086e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.838e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.002e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">46</td><td style="border: 1px solid #ddd; padding: 4px;">3.730e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.710e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.002e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">47</td><td style="border: 1px solid #ddd; padding: 4px;">3.411e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.588e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.002e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">48</td><td style="border: 1px solid #ddd; padding: 4px;">3.115e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.472e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 1.002e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 720.2
- **Average Gradient Evaluations per Run:** 292.9
- **Average Iterations per Run:** 47.6
- **Average Time per Run:** 0.020s
- **Function Evaluations per Second:** 36537.1
- **Iterations per Second:** 2417.4
### Resource Utilization
- **Total Function Evaluations:** 14404
- **Total Gradient Evaluations:** 5858
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 2.46
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/IllConditionedRosenbrock_2D_alpha100_results.csv)
* [Convergence Plot](convergence_IllConditionedRosenbrock_2D_alpha100.png)
* [Log Scale Convergence Plot](convergence_IllConditionedRosenbrock_2D_alpha100_log.png)


---
*Detailed report for QQN-Backtracking on IllConditionedRosenbrock_2D_alpha100*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
