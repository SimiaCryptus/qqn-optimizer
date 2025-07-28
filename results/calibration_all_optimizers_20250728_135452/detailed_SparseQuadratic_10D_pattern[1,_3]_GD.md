# Detailed Analysis: GD on SparseQuadratic_10D_pattern[1, 3]
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseQuadratic_10D_pattern[1, 3]
**Optimizer:** GD
**Problem Family:** SparseQuadratic
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.235895e-6
* **Worst Final Value:** 3.762811e-6
* **Mean Final Value:** 2.852346e-6
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.828e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.552e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.048e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.651e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.966e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.578e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.286e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.755e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.681e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.446e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.038e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.646e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.308e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.225e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.572e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.329e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.872e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.515e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.623e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.352e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.444e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.280e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.349e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.794e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.236e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.122e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.934e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.580e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.276e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.751e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.726e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.438e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.473e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.824e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.543e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.344e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.964e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.604e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.834e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.486e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.943e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.566e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.366e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.605e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.367e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.797e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.524e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.257e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.183e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.071e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.661e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.010e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.603e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.702e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.447e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.661e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.422e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.857e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.525e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.949e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.545e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.616e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.378e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.336e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.756e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.848e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.522e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.304e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.737e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.954e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.600e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.763e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.951e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.904e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.570e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.670e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.427e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.282e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.177e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- MaxFunctionEvaluations: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 13)
**Final Value:** 2.235895e-6
**Final Gradient Norm:** 3.122100e-3
**Iterations:** 332
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.827e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.014e-1, 8.715e-1, 1.008e0, 9.655e-1, 8.280e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.827e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.835e-1, 8.514e-1, 9.847e-1, 9.426e-1, 8.076e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.562e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.668e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.661e-1, 8.318e-1, 9.621e-1, 9.202e-1, 7.877e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.123e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.513e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.490e-1, 8.127e-1, 9.400e-1, 8.984e-1, 7.683e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.704e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.362e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.323e-1, 7.940e-1, 9.184e-1, 8.770e-1, 7.493e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">328</td><td style="border: 1px solid #ddd; padding: 4px;">2.668e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.414e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.590e-4, 4.387e-4, 5.069e-4, 3.598e-4, 1.455e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">329</td><td style="border: 1px solid #ddd; padding: 4px;">2.552e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.338e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.470e-4, 4.288e-4, 4.953e-4, 3.514e-4, 1.225e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">330</td><td style="border: 1px solid #ddd; padding: 4px;">2.442e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.265e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.353e-4, 4.192e-4, 4.840e-4, 3.433e-4, 1.003e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">331</td><td style="border: 1px solid #ddd; padding: 4px;">2.337e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.193e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.238e-4, 4.098e-4, 4.729e-4, 3.353e-4, 7.903e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">332</td><td style="border: 1px solid #ddd; padding: 4px;">2.236e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.122e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.238e-4, 4.098e-4, 4.729e-4, 3.353e-4, 7.903e-6, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 335.0
- **Average Gradient Evaluations per Run:** 668.0
- **Average Iterations per Run:** 332.0
- **Average Time per Run:** 0.009s
- **Function Evaluations per Second:** 36460.7
- **Iterations per Second:** 36134.2
### Resource Utilization
- **Total Function Evaluations:** 13400
- **Total Gradient Evaluations:** 26720
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/SparseQuadratic_10D_pattern[1,_3]_results.csv)
* [Convergence Plot](convergence_SparseQuadratic_10D_pattern[1,_3].png)
* [Log Scale Convergence Plot](convergence_SparseQuadratic_10D_pattern[1,_3]_log.png)


---
*Detailed report for GD on SparseQuadratic_10D_pattern[1, 3]*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
