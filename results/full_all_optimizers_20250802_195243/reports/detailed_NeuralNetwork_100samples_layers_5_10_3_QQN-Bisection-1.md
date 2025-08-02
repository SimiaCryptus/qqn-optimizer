# Detailed Analysis: QQN-Bisection-1 on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** QQN-Bisection-1
**Problem Family:** Neural Networks
**Dimension:** 93
**Success Threshold:** 1.400e-1
**Total Runs:** 20
**Successful Runs:** 19 (95.0%)

### Quick Statistics
* **Best Final Value:** 1.375249e-1
* **Worst Final Value:** 1.429247e-1
* **Mean Final Value:** 1.396784e-1
* **Success Rate:** 95.0%


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
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.243e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.756e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1231</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.395e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.613e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">667</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">673</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.591</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.398e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.002e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">600</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.395e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.325e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.484</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.395e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.192e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">673</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.385e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.799e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">532</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.369e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">795</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.694</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.395e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.112e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">534</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.834e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2520</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.217</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.393e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.362e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">701</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.172e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">730</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">733</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.640</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.671e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">601</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">613</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.185e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">701</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">714</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.306e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">732</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.644</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.375e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.166e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">579</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.395e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.256e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">484</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.178e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">697</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">703</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.614</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.398e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.609e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.384e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.165e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (19 out of 20)

* **Average Iterations to Convergence:** 51.2
* **Average Function Evaluations:** 632.8
* **Average Time to Convergence:** 0.561s
* **Fastest Convergence:** 30 iterations (0.328s)
* **Slowest Convergence:** 97 iterations (1.067s)

### Failed Runs (1 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 16)
**Final Value:** 1.375249e-1
**Final Gradient Norm:** 2.166333e-1
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.650e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.531e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.355e-1, 1.761e-1, 1.509e-1, 1.125e-1, 1.035e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.650e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.531e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.809e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.188e-1, 1.739e-1, 1.602e-1, 1.048e-1, 9.410e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.209e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.846e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.149e-1, 1.873e-1, 1.652e-1, 9.945e-2, 9.684e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.186e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.094e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.061e-1, 2.223e-1, 1.779e-1, 8.789e-2, 1.032e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.143e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.581e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[7.824e-2, 3.456e-1, 2.196e-1, 4.151e-2, 1.306e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">37</td><td style="border: 1px solid #ddd; padding: 4px;">1.467e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.706e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[7.961e-1, 1.244e-1, 4.480e-1, 4.376e-1, 6.266e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">38</td><td style="border: 1px solid #ddd; padding: 4px;">1.457e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.801e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.288e-1, 1.451e-1, 4.653e-1, 4.752e-1, 6.507e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">39</td><td style="border: 1px solid #ddd; padding: 4px;">1.440e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.004e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.369e-1, 2.323e-1, 5.337e-1, 6.265e-1, 7.284e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">40</td><td style="border: 1px solid #ddd; padding: 4px;">1.408e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.215e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.237e-1, 2.375e-1, 5.455e-1, 6.233e-1, 7.322e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">41</td><td style="border: 1px solid #ddd; padding: 4px;">1.375e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.166e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.237e-1, 2.375e-1, 5.455e-1, 6.233e-1, 7.322e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 727.2
- **Average Gradient Evaluations per Run:** 739.0
- **Average Iterations per Run:** 56.6
- **Average Time per Run:** 0.643s
- **Function Evaluations per Second:** 1130.2
- **Iterations per Second:** 88.0
### Resource Utilization
- **Total Function Evaluations:** 14544
- **Total Gradient Evaluations:** 14781
- **Total Computation Time:** 12.9s
- **Function/Gradient Ratio:** 0.98
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 10)**
- Final Value: 1.429247e-1
- Final Gradient Norm: 1.834106e-2
- Iterations: 160
- Function Evaluations: 2520
- Reason: MaxFunctionEvaluations



## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_5_10_3_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3_log.png)


---
*Detailed report for QQN-Bisection-1 on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
