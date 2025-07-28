# Detailed Analysis: QQN-Backtracking on SVM_200samples_10features_C1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SVM_200samples_10features_C1
**Optimizer:** QQN-Backtracking
**Problem Family:** ML Classification
**Dimension:** 10
**Success Threshold:** 6.860e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 6.861329e-1
* **Worst Final Value:** 6.861414e-1
* **Mean Final Value:** 6.861383e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.319</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.319</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.486e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">675</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.319</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.462e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">725</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.620e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">708</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.472e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">750</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.318</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.527e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">687</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.314</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.319</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.517e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">723</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.528e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">640</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">259</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.502e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">732</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">235</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.304</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.457e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">717</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.327</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.456e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.312</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.632e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">723</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.472e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.322</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.613e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">693</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">319</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.319</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 11 runs
- MaxFunctionEvaluations: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 6.861329e-1
**Final Gradient Norm:** 1.485654e-2
**Iterations:** 41
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.021e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.098e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-7.359e-2, -1.365e-1, 1.599e-2, 1.916e-1, -2.424e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.021e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.098e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.111e-2, 1.081e-1, 7.155e-2, 6.129e-2, 1.702e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">7.644e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.172e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.392e-2, 4.850e-3, 5.695e-2, 1.453e-1, 6.099e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">7.477e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.910e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.387e-2, 5.573e-2, 6.166e-2, 1.024e-1, 1.169e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.893e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.547e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.763e-2, 5.416e-2, 4.145e-2, 7.341e-2, 1.232e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">36</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.486e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.907e-6</td><td style="border: 1px solid #ddd; padding: 4px;">[5.768e-2, 4.402e-2, 3.175e-2, 6.482e-2, 1.197e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">37</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.301e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[5.768e-2, 4.402e-2, 3.175e-2, 6.482e-2, 1.197e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">38</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.413e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.768e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[5.768e-2, 4.402e-2, 3.175e-2, 6.482e-2, 1.197e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">39</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.486e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.768e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[5.768e-2, 4.402e-2, 3.175e-2, 6.482e-2, 1.197e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">40</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.301e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.768e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[5.768e-2, 4.402e-2, 3.175e-2, 6.482e-2, 1.197e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 704.7
- **Average Gradient Evaluations per Run:** 285.1
- **Average Iterations per Run:** 46.9
- **Average Time per Run:** 0.318s
- **Function Evaluations per Second:** 2219.3
- **Iterations per Second:** 147.7
### Resource Utilization
- **Total Function Evaluations:** 14094
- **Total Gradient Evaluations:** 5702
- **Total Computation Time:** 6.4s
- **Function/Gradient Ratio:** 2.47
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/SVM_200samples_10features_C1_results.csv)
* [Convergence Plot](convergence_SVM_200samples_10features_C1.png)
* [Log Scale Convergence Plot](convergence_SVM_200samples_10features_C1_log.png)


---
*Detailed report for QQN-Backtracking on SVM_200samples_10features_C1*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
