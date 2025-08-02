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
* **Best Final Value:** 4.805721e-2
* **Worst Final Value:** 6.455526e-2
* **Mean Final Value:** 5.604088e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.965e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.903e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.127e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.989e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.851e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.231e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1364</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.264e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.350e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3663</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.858e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.158e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1373</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.517e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.796e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3698</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.962e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3645</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.868e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.708e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.820e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.437e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3659</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.546e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.748e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.816e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.842e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.343e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.128e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.686e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.009e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.456e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.121e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3651</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.461e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.989e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.419e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.026e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3673</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.720e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.914e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.831e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.356e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3652</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.098e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.269e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3650</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.631e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.884e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3651</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 7)
**Final Value:** 4.805721e-2
**Final Gradient Norm:** 5.962436e-2
**Iterations:** 451
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.916e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.128e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.050e-1, 6.745e-2, 1.139e-1, -9.485e-2, -2.200e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.916e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.128e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.041e-1, 6.890e-2, 1.144e-1, -9.248e-2, -2.197e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.640e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.643e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.013e-1, 7.062e-2, 1.168e-1, -9.053e-2, -2.216e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.588e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.847e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.989e-1, 7.266e-2, 1.195e-1, -8.655e-2, -2.246e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.573e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.683e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.953e-1, 7.350e-2, 1.231e-1, -8.463e-2, -2.261e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">447</td><td style="border: 1px solid #ddd; padding: 4px;">4.819e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.887e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.357e-1, -1.449e-1, 2.470e-1, -3.160e-1, -6.736e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">448</td><td style="border: 1px solid #ddd; padding: 4px;">4.816e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.933e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.352e-1, -1.451e-1, 2.472e-1, -3.160e-1, -6.742e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">449</td><td style="border: 1px solid #ddd; padding: 4px;">4.812e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.585e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.356e-1, -1.450e-1, 2.467e-1, -3.165e-1, -6.740e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">450</td><td style="border: 1px solid #ddd; padding: 4px;">4.810e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.016e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.350e-1, -1.448e-1, 2.462e-1, -3.164e-1, -6.748e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">451</td><td style="border: 1px solid #ddd; padding: 4px;">4.806e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.962e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.350e-1, -1.448e-1, 2.462e-1, -3.164e-1, -6.748e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3655.3
- **Average Gradient Evaluations per Run:** 1352.0
- **Average Iterations per Run:** 449.0
- **Average Time per Run:** 3.155s
- **Function Evaluations per Second:** 1158.6
- **Iterations per Second:** 142.3
### Resource Utilization
- **Total Function Evaluations:** 73107
- **Total Gradient Evaluations:** 27040
- **Total Computation Time:** 63.1s
- **Function/Gradient Ratio:** 2.70
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
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
