# Detailed Analysis: L-BFGS-MoreThuente on LinearRegression_100samples_5features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LinearRegression_100samples_5features_reg0.01
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** Regression
**Dimension:** 5
**Success Threshold:** 7.150e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 7.154664e-2
* **Worst Final Value:** 7.179108e-2
* **Mean Final Value:** 7.155891e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.326e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.033e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.071e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.840</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.591e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.623e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2920</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.834</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.186e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.933e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.126e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.557e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.179e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.198e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.836</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.260e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.396e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.172e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.353e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.595e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.494e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.561e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.831</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.419e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.873e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.191e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- NumericalError: 16 runs
- MaxFunctionEvaluations: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 7.154664e-2
**Final Gradient Norm:** 8.190768e-6
**Iterations:** 17
**Convergence Reason:** NumericalError

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.913e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.697e-1, -9.115e-2, -4.524e-2, -1.515e-1, -6.080e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.913e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.850e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.958e-1, 7.230e-1, 7.273e-1, 1.521e0, 1.939e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.461e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.577e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.133e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.884e-1, 1.019e0, 1.399e0, 2.069e0, 2.483e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.899e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.848e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.148e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.895e-1, 1.017e0, 1.505e0, 2.026e0, 2.498e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">7.428e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.174e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.914e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.899e-1, 1.015e0, 1.518e0, 2.015e0, 2.497e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">12</td><td style="border: 1px solid #ddd; padding: 4px;">7.156e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.307e-3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.931e-1, 9.959e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">13</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.111e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.675e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.959e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">14</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.719e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">15</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.851e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">16</td><td style="border: 1px solid #ddd; padding: 4px;">7.155e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.421e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.756e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 9.960e-1, 1.490e0, 1.987e0, 2.492e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 796.0
- **Average Gradient Evaluations per Run:** 562.6
- **Average Iterations per Run:** 116.1
- **Average Time per Run:** 0.229s
- **Function Evaluations per Second:** 3473.1
- **Iterations per Second:** 506.6
### Resource Utilization
- **Total Function Evaluations:** 15920
- **Total Gradient Evaluations:** 11252
- **Total Computation Time:** 4.6s
- **Function/Gradient Ratio:** 1.41
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 16
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/LinearRegression_100samples_5features_reg0.01_results.csv)
* [Convergence Plot](../plots/LinearRegression_100samples_5features_reg0.01.png)
* [Log Scale Convergence Plot](../plots/LinearRegression_100samples_5features_reg0.01_log.png)


---
*Detailed report for L-BFGS-MoreThuente on LinearRegression_100samples_5features_reg0.01*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
