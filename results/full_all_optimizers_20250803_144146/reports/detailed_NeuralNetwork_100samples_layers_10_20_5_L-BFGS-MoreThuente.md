# Detailed Analysis: L-BFGS-MoreThuente on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 5.343640e-2
* **Worst Final Value:** 7.104395e-2
* **Mean Final Value:** 5.956650e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.483e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.475e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2824</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.779</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.180e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.936e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">322</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2830</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.819</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.977e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.448e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.811</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.861e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.604e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2836</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.958e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.583e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">327</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2835</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.780</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.421e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.380e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2831</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.135e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.521e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">336</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2844</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.864</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.398e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.112e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2835</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.777</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.942e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.282e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">320</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2822</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.805</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.359e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.065e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.834</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.953e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.346e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.887</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.158e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.205e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2836</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.842</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.525e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.511e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2836</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.398e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.505e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.810</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.399e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.641e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2835</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.344e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.539e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2835</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.815</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.144e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.166e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2829</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.815</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.104e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.887e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">326</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2831</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.814</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.897e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.283e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2834</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.496e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.008e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">323</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2827</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 16)
**Final Value:** 5.343640e-2
**Final Gradient Norm:** 1.539007e-1
**Iterations:** 330
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.306e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.430e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.101e-2, -6.257e-2, 1.239e-1, -2.244e-1, 7.964e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.306e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.430e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.933e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.670e-2, -6.640e-2, 1.224e-1, -2.282e-1, 8.040e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.726e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.451e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.897e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.807e-2, -6.412e-2, 1.209e-1, -2.236e-1, 7.533e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.599e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.932e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.999e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.222e-2, -6.599e-2, 1.205e-1, -2.245e-1, 7.470e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.561e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.569e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.990e-2, -6.717e-2, 1.187e-1, -2.229e-1, 7.048e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">326</td><td style="border: 1px solid #ddd; padding: 4px;">5.377e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.767e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.657e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.016e-2, -4.070e-1, -1.033e-1, -7.374e-1, -6.986e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">327</td><td style="border: 1px solid #ddd; padding: 4px;">5.376e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.343e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.210e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.122e-2, -4.063e-1, -1.039e-1, -7.376e-1, -8.727e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">328</td><td style="border: 1px solid #ddd; padding: 4px;">5.357e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.290e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.134e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.097e-2, -4.062e-1, -1.043e-1, -7.371e-1, -9.471e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">329</td><td style="border: 1px solid #ddd; padding: 4px;">5.349e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.395e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.105e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.081e-2, -4.076e-1, -1.051e-1, -7.373e-1, -1.086e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">330</td><td style="border: 1px solid #ddd; padding: 4px;">5.344e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.539e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.073e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.081e-2, -4.076e-1, -1.051e-1, -7.373e-1, -1.086e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2833.2
- **Average Gradient Evaluations per Run:** 2176.2
- **Average Iterations per Run:** 327.4
- **Average Time per Run:** 3.804s
- **Function Evaluations per Second:** 744.8
- **Iterations per Second:** 86.1
### Resource Utilization
- **Total Function Evaluations:** 56663
- **Total Gradient Evaluations:** 43525
- **Total Computation Time:** 76.1s
- **Function/Gradient Ratio:** 1.30
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
*Detailed report for L-BFGS-MoreThuente on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
