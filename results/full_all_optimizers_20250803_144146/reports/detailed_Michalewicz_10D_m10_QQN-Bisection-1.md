# Detailed Analysis: QQN-Bisection-1 on Michalewicz_10D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_10D_m10
**Optimizer:** QQN-Bisection-1
**Problem Family:** Michalewicz
**Dimension:** 10
**Success Threshold:** -6.260e0
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -5.363138e0
* **Worst Final Value:** -2.144291e0
* **Mean Final Value:** -4.237019e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.625e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.125e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1576</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.512e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.597e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1260</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1367</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.944e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.839e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.063</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.344e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.115e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2240</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2763</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.078</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.654e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.448e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.068</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.259e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.173e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2724</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.827e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.404e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.379e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2225</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.065</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.752e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.145e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2658</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.142e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.406e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.064</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.363e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.950e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2585</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.942e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">85</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2562</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.066</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.865e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.821e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">67</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.297e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.174e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2676</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.071</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.073e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">581</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">596</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.054e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.387e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1938</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2238</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.363e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.083e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.204e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.144e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.627e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2424</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2607</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.069</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.815e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.137e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 11 runs
- FunctionTolerance: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 11)
**Final Value:** -5.363138e0
**Final Gradient Norm:** 1.949572e-3
**Iterations:** 73
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.867e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.319e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 9.842e-1, 7.047e-1, 9.539e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.867e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.319e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.870e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 9.866e-1, 7.047e-1, 1.005e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-3.758e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.224e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.563e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 9.871e-1, 7.047e-1, 1.003e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-3.977e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.062e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.563e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 9.875e-1, 7.047e-1, 1.001e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-4.139e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.899e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.563e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 9.880e-1, 7.047e-1, 9.999e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">69</td><td style="border: 1px solid #ddd; padding: 4px;">-5.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.712e-3</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 1.285e0, 7.051e-1, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">70</td><td style="border: 1px solid #ddd; padding: 4px;">-5.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.290e-3</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 1.285e0, 7.051e-1, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">71</td><td style="border: 1px solid #ddd; padding: 4px;">-5.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.822e-3</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 1.285e0, 7.052e-1, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">72</td><td style="border: 1px solid #ddd; padding: 4px;">-5.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.470e-3</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 1.285e0, 7.052e-1, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">73</td><td style="border: 1px solid #ddd; padding: 4px;">-5.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.950e-3</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.915e-1, 7.473e-1, 1.285e0, 7.052e-1, 9.967e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2030.0
- **Average Gradient Evaluations per Run:** 2151.8
- **Average Iterations per Run:** 67.5
- **Average Time per Run:** 0.056s
- **Function Evaluations per Second:** 35957.1
- **Iterations per Second:** 1196.5
### Resource Utilization
- **Total Function Evaluations:** 40600
- **Total Gradient Evaluations:** 43035
- **Total Computation Time:** 1.1s
- **Function/Gradient Ratio:** 0.94
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Michalewicz_10D_m10_results.csv)
* [Convergence Plot](../plots/Michalewicz_10D_m10.png)
* [Log Scale Convergence Plot](../plots/Michalewicz_10D_m10_log.png)


---
*Detailed report for QQN-Bisection-1 on Michalewicz_10D_m10*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
