# Detailed Analysis: L-BFGS-Conservative on IllConditionedRosenbrock_10D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_10D_alpha100
**Optimizer:** L-BFGS-Conservative
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 7.140130e0
* **Worst Final Value:** 1.025513e1
* **Mean Final Value:** 8.828618e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.538e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.448e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">706</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">315</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.526e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.385e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">678</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">326</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.551e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.238e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">302</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.602e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.731e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">255</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.595e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.078e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.864e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.183e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.145e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.871e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">658</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.607e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.854e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">783</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">238</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.600e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.172e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.140e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.122e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">647</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.127e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.879e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">368</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.026e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.598e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">787</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">224</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.457e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.759e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.211e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.835e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">713</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">308</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.756e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.705e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">684</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.169e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.045e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">91</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.423e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.237e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">64</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">754</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">263</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.789e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.024e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">684</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">318</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.663e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.245e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.553e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.047e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 7.140130e0
**Final Gradient Norm:** 4.121869e0
**Iterations:** 89
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.604e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.557e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.122e0, 1.172e0, -1.310e0, 1.085e0, -1.262e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.604e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.557e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.062e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.118e0, 1.072e0, -1.240e0, 1.010e0, -1.204e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.120e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.148e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.174e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.110e0, 9.716e-1, -1.165e0, 9.299e-1, -1.139e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.694e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.774e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.312e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.094e0, 8.716e-1, -1.083e0, 8.439e-1, -1.065e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.325e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.435e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.486e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.069e0, 7.716e-1, -9.939e-1, 7.514e-1, -9.828e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">85</td><td style="border: 1px solid #ddd; padding: 4px;">7.200e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.402e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[6.123e-1, 3.783e-1, 1.550e-1, 1.835e-2, 1.406e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">86</td><td style="border: 1px solid #ddd; padding: 4px;">7.188e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.688e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.441e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[6.135e-1, 3.779e-1, 1.541e-1, 1.770e-2, 1.386e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">87</td><td style="border: 1px solid #ddd; padding: 4px;">7.188e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.696e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[6.195e-1, 3.939e-1, 1.506e-1, 2.868e-2, 1.197e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">88</td><td style="border: 1px solid #ddd; padding: 4px;">7.142e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.791e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[6.222e-1, 3.931e-1, 1.486e-1, 2.727e-2, 1.152e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">89</td><td style="border: 1px solid #ddd; padding: 4px;">7.140e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.122e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[6.222e-1, 3.931e-1, 1.486e-1, 2.727e-2, 1.152e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 702.1
- **Average Gradient Evaluations per Run:** 310.2
- **Average Iterations per Run:** 75.3
- **Average Time per Run:** 0.011s
- **Function Evaluations per Second:** 66297.3
- **Iterations per Second:** 7115.1
### Resource Utilization
- **Total Function Evaluations:** 14042
- **Total Gradient Evaluations:** 6204
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 2.26
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/IllConditionedRosenbrock_10D_alpha100_results.csv)
* [Convergence Plot](convergence_IllConditionedRosenbrock_10D_alpha100.png)
* [Log Scale Convergence Plot](convergence_IllConditionedRosenbrock_10D_alpha100_log.png)


---
*Detailed report for L-BFGS-Conservative on IllConditionedRosenbrock_10D_alpha100*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
