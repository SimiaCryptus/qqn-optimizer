# Detailed Analysis: L-BFGS-Aggressive on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** Neural Networks
**Dimension:** 93
**Success Threshold:** 1.400e-1
**Total Runs:** 20
**Successful Runs:** 1 (5.0%)

### Quick Statistics
* **Best Final Value:** 1.398963e-1
* **Worst Final Value:** 1.619824e-1
* **Mean Final Value:** 1.503538e-1
* **Success Rate:** 5.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.561e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.926e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">812</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.973</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.489e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.413e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">437</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3496</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.579e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.853e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">989</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.424e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.511</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.620e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.282e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">304</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">914</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.470e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.217e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3659</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.675</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.618e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.790e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.935</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.474e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.320e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.509e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.433e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">463</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.695</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.424e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.135e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.562e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.270e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">833</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.560e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.936e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">266</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">800</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.948</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.447e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.068e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">393</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.511e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.086e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">255</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.506e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.523e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1238</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.452e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.933e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3642</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.685</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.503e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.267e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.533e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.235e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1800</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.451e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.755e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.403e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.068e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">463</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3613</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.700</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (1 out of 20)

* **Average Iterations to Convergence:** 416.0
* **Average Function Evaluations:** 3131.0
* **Average Time to Convergence:** 1.511s
* **Fastest Convergence:** 416 iterations (1.511s)
* **Slowest Convergence:** 416 iterations (1.511s)

### Failed Runs (19 out of 20)

**Failure Reasons:**
- FunctionTolerance: 14 runs
- MaxFunctionEvaluations: 5 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 1.398963e-1
**Final Gradient Norm:** 4.424201e-2
**Iterations:** 416
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.615e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.392e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.332e-2, -1.878e-1, 4.533e-2, -1.359e-1, 7.184e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.615e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.392e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.999e-2, -1.863e-1, 4.833e-2, -1.392e-1, 5.806e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.247e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.437e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.596e-2, -1.861e-1, 4.750e-2, -1.341e-1, 5.835e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.206e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.497e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.485e-2, -1.848e-1, 4.742e-2, -1.280e-1, 5.802e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.195e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.062e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.751e-2, -1.827e-1, 4.662e-2, -1.233e-1, 5.820e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">412</td><td style="border: 1px solid #ddd; padding: 4px;">1.404e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.814e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.063e-1, 3.241e-1, 2.849e-1, -5.013e-1, -1.329e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">413</td><td style="border: 1px solid #ddd; padding: 4px;">1.403e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.001e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.058e-1, 3.240e-1, 2.860e-1, -5.017e-1, -1.321e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">414</td><td style="border: 1px solid #ddd; padding: 4px;">1.402e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.803e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.104e-1, 3.246e-1, 2.877e-1, -5.002e-1, -1.245e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">415</td><td style="border: 1px solid #ddd; padding: 4px;">1.402e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.232e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.099e-1, 3.236e-1, 2.892e-1, -5.004e-1, -1.243e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">416</td><td style="border: 1px solid #ddd; padding: 4px;">1.399e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.424e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.099e-1, 3.236e-1, 2.892e-1, -5.004e-1, -1.243e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2776.7
- **Average Gradient Evaluations per Run:** 1087.3
- **Average Iterations per Run:** 361.5
- **Average Time per Run:** 1.320s
- **Function Evaluations per Second:** 2103.1
- **Iterations per Second:** 273.8
### Resource Utilization
- **Total Function Evaluations:** 55533
- **Total Gradient Evaluations:** 21746
- **Total Computation Time:** 26.4s
- **Function/Gradient Ratio:** 2.55
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_5_10_3_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3_log.png)


---
*Detailed report for L-BFGS-Aggressive on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
