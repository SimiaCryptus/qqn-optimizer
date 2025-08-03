# Detailed Analysis: GD-Momentum on StyblinskiTang_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** StyblinskiTang_10D
**Optimizer:** GD-Momentum
**Problem Family:** StyblinskiTang
**Dimension:** 10
**Success Threshold:** -3.780e2
**Total Runs:** 20
**Successful Runs:** 3 (15.0%)

### Quick Statistics
* **Best Final Value:** -3.815191e2
* **Worst Final Value:** -1.981942e2
* **Mean Final Value:** -3.585182e2
* **Success Rate:** 15.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.545e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.881e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.599e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.508e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.815e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.770e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.671e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.084e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.506e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.519e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.982e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.147e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.963e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">422</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.719e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.805e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.785e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.364e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.553e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.540e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.692e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.676e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.681e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.945e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.434e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.034e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.688e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.605e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.279e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.548e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.618e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.795e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.080e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.730e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.859e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.744e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.634e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.543e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">197</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">200</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (3 out of 20)

* **Average Iterations to Convergence:** 40.7
* **Average Function Evaluations:** 43.7
* **Average Time to Convergence:** 0.001s
* **Fastest Convergence:** 39 iterations (0.001s)
* **Slowest Convergence:** 45 iterations (0.002s)

### Failed Runs (17 out of 20)

**Failure Reasons:**
- FunctionTolerance: 17 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** -3.815191e2
**Final Gradient Norm:** 2.770220e1
**Iterations:** 45
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.113e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.894e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.667e-1, -2.970e-2, -1.155e-1, 2.487e-2, 1.002e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.113e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.894e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.928e-1, -4.473e-2, -1.375e-1, 1.424e-2, 9.564e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-1.628e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.069e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.423e-1, -7.330e-2, -1.792e-1, -5.948e-3, 8.701e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-2.714e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.218e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.129e-1, -1.141e-1, -2.388e-1, -3.477e-2, 7.469e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-4.509e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.430e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.025e-1, -1.659e-1, -3.143e-1, -7.139e-2, 5.903e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">41</td><td style="border: 1px solid #ddd; padding: 4px;">-3.625e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.471e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.678e0, -2.750e0, -2.668e0, -2.947e0, -3.257e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">42</td><td style="border: 1px solid #ddd; padding: 4px;">-3.678e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.497e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.698e0, -2.707e0, -2.671e0, -2.869e0, -3.298e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">43</td><td style="border: 1px solid #ddd; padding: 4px;">-3.729e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.405e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.725e0, -2.677e0, -2.684e0, -2.801e0, -3.311e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">44</td><td style="border: 1px solid #ddd; padding: 4px;">-3.776e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.156e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.758e0, -2.662e0, -2.706e0, -2.745e0, -3.296e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">45</td><td style="border: 1px solid #ddd; padding: 4px;">-3.815e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.770e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.758e0, -2.662e0, -2.706e0, -2.745e0, -3.296e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 86.2
- **Average Gradient Evaluations per Run:** 168.4
- **Average Iterations per Run:** 83.2
- **Average Time per Run:** 0.003s
- **Function Evaluations per Second:** 32792.6
- **Iterations per Second:** 31650.7
### Resource Utilization
- **Total Function Evaluations:** 1723
- **Total Gradient Evaluations:** 3369
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 0.51
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/StyblinskiTang_10D_results.csv)
* [Convergence Plot](../plots/StyblinskiTang_10D.png)
* [Log Scale Convergence Plot](../plots/StyblinskiTang_10D_log.png)


---
*Detailed report for GD-Momentum on StyblinskiTang_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
