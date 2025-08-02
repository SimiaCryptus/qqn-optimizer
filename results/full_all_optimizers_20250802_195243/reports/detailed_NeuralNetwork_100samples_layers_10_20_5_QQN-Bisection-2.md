# Detailed Analysis: QQN-Bisection-2 on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** QQN-Bisection-2
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 3.770847e-2
* **Worst Final Value:** 3.819232e-2
* **Mean Final Value:** 3.801128e-2
* **Success Rate:** 100.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.816e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.461e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">682</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.777e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.034e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">593</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.796e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.098e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">590</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">512</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.873</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.789e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.343e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">763</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.302</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.813e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.458e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">67</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">741</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.815e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.601e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">800</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.771e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.428e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">67</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">741</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.595e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1097</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.793e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.211e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">925</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.799e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.801e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1071</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">916</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.782e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.397e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">682</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.773e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.617e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">728</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.797e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.103e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">898</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.810e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.188e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">700</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.811e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.718e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.489</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.817e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.686e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">657</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.978</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.813e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.028e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1227</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.041e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">931</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">791</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.421e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.808e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.802e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">818</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">709</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.212</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 78.8
* **Average Function Evaluations:** 883.2
* **Average Time to Convergence:** 1.296s
* **Fastest Convergence:** 52 iterations (0.873s)
* **Slowest Convergence:** 117 iterations (1.895s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 7)
**Final Value:** 3.770847e-2
**Final Gradient Norm:** 8.427683e-2
**Iterations:** 67
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.603e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.606e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.935e-2, 1.125e-1, -1.109e-1, -9.224e-2, -1.919e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.603e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.606e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.990e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.454e-2, 1.133e-1, -1.024e-1, -9.438e-2, -1.833e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.604e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.986e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.630e-2, 1.186e-1, -1.041e-1, -1.016e-1, -1.699e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.576e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.716e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.226e-2, 1.189e-1, -1.053e-1, -1.053e-1, -1.617e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.544e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.763e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[7.843e-2, 1.143e-1, -1.137e-1, -1.296e-1, -1.204e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">63</td><td style="border: 1px solid #ddd; padding: 4px;">3.928e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.665e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.709e-1, -2.580e-2, -4.275e-1, 2.678e-1, 4.810e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">64</td><td style="border: 1px solid #ddd; padding: 4px;">3.891e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.028e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.750e-1, -2.322e-2, -4.297e-1, 2.659e-1, 4.834e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">65</td><td style="border: 1px solid #ddd; padding: 4px;">3.862e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.704e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.749e-1, -2.197e-2, -4.350e-1, 2.634e-1, 4.856e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">66</td><td style="border: 1px solid #ddd; padding: 4px;">3.822e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.129e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.816e-1, -1.830e-2, -4.423e-1, 2.602e-1, 4.912e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">67</td><td style="border: 1px solid #ddd; padding: 4px;">3.771e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.428e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.816e-1, -1.830e-2, -4.423e-1, 2.602e-1, 4.912e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 883.2
- **Average Gradient Evaluations per Run:** 753.1
- **Average Iterations per Run:** 78.8
- **Average Time per Run:** 1.296s
- **Function Evaluations per Second:** 681.6
- **Iterations per Second:** 60.8
### Resource Utilization
- **Total Function Evaluations:** 17665
- **Total Gradient Evaluations:** 15063
- **Total Computation Time:** 25.9s
- **Function/Gradient Ratio:** 1.17
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for QQN-Bisection-2 on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
