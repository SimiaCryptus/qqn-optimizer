# Detailed Analysis: L-BFGS-Aggressive on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 4.810593e-2
* **Worst Final Value:** 6.449521e-2
* **Mean Final Value:** 5.552090e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.450e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.756e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">437</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3695</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1316</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.143</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.336e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.123e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.954e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.060e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3658</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.677e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.565e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3636</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.209</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.512e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.122e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3684</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1322</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.562e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.337e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.033e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.822e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3667</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.240</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.276e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.071e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3658</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.329e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.624e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3667</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.118e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.340e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3633</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.301</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.811e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.009e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3644</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.241</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.854e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.216e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.239</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.071e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.057e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.690e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.179e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">455</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3632</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1370</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.201</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.870e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.218e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.345e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.412e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.221e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.629e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3641</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.200</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.620e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.977e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.347e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.698e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3657</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.218</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.967e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.216e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3670</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.218</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 11)
**Final Value:** 4.810593e-2
**Final Gradient Norm:** 7.009421e-2
**Iterations:** 453
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.303e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.352e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.007e-1, 5.213e-3, 1.850e-1, 1.590e-1, -1.107e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.303e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.352e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.032e-1, 8.965e-3, 1.782e-1, 1.560e-1, -1.063e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.652e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.127e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.050e-1, 7.167e-3, 1.793e-1, 1.557e-1, -9.942e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.582e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.637e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.120e-1, 9.143e-3, 1.811e-1, 1.539e-1, -8.674e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.001e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.155e-1, 5.156e-3, 1.843e-1, 1.526e-1, -7.581e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">449</td><td style="border: 1px solid #ddd; padding: 4px;">4.832e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.944e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.793e-1, -2.262e-1, 5.342e-1, 3.827e-1, -3.076e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">450</td><td style="border: 1px solid #ddd; padding: 4px;">4.830e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.029e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.796e-1, -2.260e-1, 5.333e-1, 3.832e-1, -3.077e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">451</td><td style="border: 1px solid #ddd; padding: 4px;">4.821e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.003e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.812e-1, -2.271e-1, 5.339e-1, 3.828e-1, -3.075e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">452</td><td style="border: 1px solid #ddd; padding: 4px;">4.815e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.062e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.824e-1, -2.269e-1, 5.342e-1, 3.831e-1, -3.073e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">453</td><td style="border: 1px solid #ddd; padding: 4px;">4.811e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.009e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.824e-1, -2.269e-1, 5.342e-1, 3.831e-1, -3.073e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3657.3
- **Average Gradient Evaluations per Run:** 1349.9
- **Average Iterations per Run:** 448.3
- **Average Time per Run:** 3.210s
- **Function Evaluations per Second:** 1139.2
- **Iterations per Second:** 139.6
### Resource Utilization
- **Total Function Evaluations:** 73146
- **Total Gradient Evaluations:** 26998
- **Total Computation Time:** 64.2s
- **Function/Gradient Ratio:** 2.71
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for L-BFGS-Aggressive on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
