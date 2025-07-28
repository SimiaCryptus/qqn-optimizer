# Detailed Analysis: GD-Nesterov on Rosenbrock_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rosenbrock_2D
**Optimizer:** GD-Nesterov
**Problem Family:** Non-Convex Unimodal
**Dimension:** 2
**Success Threshold:** 7.550e-2
**Total Runs:** 20
**Successful Runs:** 10 (50.0%)

### Quick Statistics
* **Best Final Value:** 4.982270e-2
* **Worst Final Value:** 5.061087e0
* **Mean Final Value:** 1.080280e0
* **Success Rate:** 50.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.122e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.373e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.653e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.782e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.061e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.960e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.172e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.591e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.804e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.496e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.880e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.045e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.165e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.192e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.982e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.244e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.001e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.408e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.834e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.559e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.941e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.472e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.578e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.501e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.835e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.981e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.468e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.061e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.007e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.303e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.776e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">53</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.782e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.171e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.278e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.226e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.542e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.751e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.892e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.154e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (10 out of 20)
- **Average Iterations to Convergence:** 57.3
- **Average Function Evaluations:** 60.3
- **Average Time to Convergence:** 0.002s
- **Fastest Convergence:** 48 iterations (0.002s)
- **Slowest Convergence:** 70 iterations (0.002s)
### Failed Runs (10 out of 20)

**Failure Reasons:**
- FunctionTolerance: 10 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 8)
**Final Value:** 4.982270e-2
**Final Gradient Norm:** 6.243886e0
**Iterations:** 57
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.458e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.647e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.180e0, 1.079e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.458e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.647e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.134e0, 1.098e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.068e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.685e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.009e0, 1.150e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.792e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.612e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.021e0, 1.134e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.922e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.802e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.076e0, 1.096e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">52</td><td style="border: 1px solid #ddd; padding: 4px;">2.095e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.776e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.334e-1, 6.598e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">53</td><td style="border: 1px solid #ddd; padding: 4px;">1.483e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.321e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.565e-1, 7.259e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">54</td><td style="border: 1px solid #ddd; padding: 4px;">2.421e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.615e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.051e-1, 7.098e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">55</td><td style="border: 1px solid #ddd; padding: 4px;">4.188e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.373e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.923e-1, 6.706e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">56</td><td style="border: 1px solid #ddd; padding: 4px;">1.589e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.119e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.495e-1, 7.052e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 51.0
- **Average Gradient Evaluations per Run:** 98.0
- **Average Iterations per Run:** 48.0
- **Average Time per Run:** 0.002s
- **Function Evaluations per Second:** 32838.2
- **Iterations per Second:** 30906.5
### Resource Utilization
- **Total Function Evaluations:** 1020
- **Total Gradient Evaluations:** 1960
- **Total Computation Time:** 0.0s
- **Function/Gradient Ratio:** 0.52
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Rosenbrock_2D_results.csv)
* [Convergence Plot](convergence_Rosenbrock_2D.png)
* [Log Scale Convergence Plot](convergence_Rosenbrock_2D_log.png)


---
*Detailed report for GD-Nesterov on Rosenbrock_2D*
*Generated on: 2025-07-28 13:54:23 UTC*
*[← Back to Main Report](benchmark_report.md)*
