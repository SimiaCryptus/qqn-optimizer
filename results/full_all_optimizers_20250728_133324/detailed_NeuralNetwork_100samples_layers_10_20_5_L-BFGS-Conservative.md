# Detailed Analysis: L-BFGS-Conservative on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** L-BFGS-Conservative
**Problem Family:** ML Neural Networks
**Dimension:** 325
**Success Threshold:** 3.540e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 5.337035e-2
* **Worst Final Value:** 8.431209e-2
* **Mean Final Value:** 6.446557e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.441e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.581e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">568</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.908e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.957e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">434</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.765</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.397e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.381e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.751</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.393e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.162e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">567</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.337e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.366e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.807e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.679e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.760</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.187e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.221e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.741</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.354e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.259e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">579</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.068e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.323e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">564</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.762</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.278e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.098e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.761</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.008e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.025e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.763</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.431e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.532e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.717</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.619e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.032e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.758</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.407e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.327e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.818e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.114e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.761</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.127e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.002e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.761</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.822e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.004e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.756</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.490e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.533e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.212e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.097e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">557</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.758</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.828e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.226e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">561</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 5.337035e-2
**Final Gradient Norm:** 1.366254e-1
**Iterations:** 108
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.479e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.600e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.003e-1, 2.861e-2, 1.053e-1, -5.523e-2, -1.645e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.479e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.600e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.821e-2, 2.958e-2, 1.057e-1, -5.622e-2, -1.642e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.046e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.117e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.620e-2, 3.055e-2, 1.061e-1, -5.711e-2, -1.639e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.832e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.003e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.544e-2, 3.093e-2, 1.062e-1, -5.742e-2, -1.638e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.801e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.473e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.512e-2, 3.109e-2, 1.063e-1, -5.753e-2, -1.637e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">104</td><td style="border: 1px solid #ddd; padding: 4px;">5.502e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.103e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.498e-1, 6.570e-1, 6.169e-1, -3.145e-1, -1.453e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">105</td><td style="border: 1px solid #ddd; padding: 4px;">5.483e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.089e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.513e-1, 6.594e-1, 6.179e-1, -3.141e-1, -1.461e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">106</td><td style="border: 1px solid #ddd; padding: 4px;">5.466e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.047e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.561e-1, 6.671e-1, 6.214e-1, -3.127e-1, -1.494e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">107</td><td style="border: 1px solid #ddd; padding: 4px;">5.412e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.033e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.647e-1, 6.821e-1, 6.281e-1, -3.100e-1, -1.558e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">108</td><td style="border: 1px solid #ddd; padding: 4px;">5.337e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.366e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.647e-1, 6.821e-1, 6.281e-1, -3.100e-1, -1.558e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 568.8
- **Average Gradient Evaluations per Run:** 436.9
- **Average Iterations per Run:** 107.7
- **Average Time per Run:** 0.757s
- **Function Evaluations per Second:** 751.4
- **Iterations per Second:** 142.3
### Resource Utilization
- **Total Function Evaluations:** 11375
- **Total Gradient Evaluations:** 8738
- **Total Computation Time:** 15.1s
- **Function/Gradient Ratio:** 1.30
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](convergence_NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](convergence_NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for L-BFGS-Conservative on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
