# Detailed Analysis: QQN-CubicQuadraticInterpolation on SparseRosenbrock_4D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_4D
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** SparseRosenbrock
**Dimension:** 4
**Success Threshold:** 1.000e-6
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.649805e-2
* **Worst Final Value:** 7.602043e0
* **Mean Final Value:** 4.635875e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.570e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.387e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">676</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.867e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.240e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.084e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.860e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.168e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.475e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">670</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.050e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.837e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">675</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.707e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.162e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.396e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.221e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">392</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">636</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.530e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.262e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.602e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.920e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.798e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.438e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">640</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.841e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.662e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">645</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.239e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.637e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.052e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.989e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">408</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">676</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.663e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.578e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.255e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.394e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">408</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.335e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.195e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.424e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.786e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.810e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.679e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.604e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.137e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">674</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.803e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.696e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">672</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.446e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.522e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.491e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.502e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">616</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.176e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.757e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">398</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.283e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.390e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.763e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.230e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">646</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.795e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.611e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.866e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.912e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.154e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.648e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">670</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.640e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.016e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">675</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.650e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.469e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.980e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.898e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">676</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.244e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.035e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">670</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.512e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.336e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">664</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.569e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.182e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">408</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.455e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.373e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">656</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.294e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.491e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">666</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.243e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.374e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.162e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.986e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">642</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.045e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.557e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">642</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- MaxFunctionEvaluations: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 11)
**Final Value:** 1.649805e-2
**Final Gradient Norm:** 1.468723e-1
**Iterations:** 17
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.124e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.354e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.353e0, 8.883e-1, -1.388e0, 8.684e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.124e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.354e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.227e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.340e0, 1.874e0, 1.712e0, 1.976e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.247e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.842e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.777e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.351e0, 1.870e0, 1.530e0, 2.029e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.032e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.033e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.362e0, 1.866e0, 1.454e0, 2.055e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">7.176e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.861e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.365e0, 1.865e0, 1.437e0, 2.061e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">13</td><td style="border: 1px solid #ddd; padding: 4px;">6.520e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.757e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.035e0, 1.060e0, 1.075e0, 1.138e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">14</td><td style="border: 1px solid #ddd; padding: 4px;">4.953e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.845e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.096e0, 1.203e0, 1.141e0, 1.303e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">15</td><td style="border: 1px solid #ddd; padding: 4px;">2.978e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.074e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.068e0, 1.140e0, 1.110e0, 1.230e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">16</td><td style="border: 1px solid #ddd; padding: 4px;">1.665e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.972e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.068e0, 1.140e0, 1.109e0, 1.231e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">17</td><td style="border: 1px solid #ddd; padding: 4px;">1.650e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.469e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.068e0, 1.140e0, 1.109e0, 1.231e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 397.2
- **Average Gradient Evaluations per Run:** 651.0
- **Average Iterations per Run:** 13.5
- **Average Time per Run:** 0.015s
- **Function Evaluations per Second:** 27049.8
- **Iterations per Second:** 919.3
### Resource Utilization
- **Total Function Evaluations:** 15890
- **Total Gradient Evaluations:** 26041
- **Total Computation Time:** 0.6s
- **Function/Gradient Ratio:** 0.61
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/SparseRosenbrock_4D_results.csv)
* [Convergence Plot](convergence_SparseRosenbrock_4D.png)
* [Log Scale Convergence Plot](convergence_SparseRosenbrock_4D_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on SparseRosenbrock_4D*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
