# Detailed Analysis: QQN-CubicQuadraticInterpolation on SparseQuadratic_10D_pattern[1, 3]
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseQuadratic_10D_pattern[1, 3]
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** SparseQuadratic
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 40
**Successful Runs:** 40 (100.0%)

### Quick Statistics
* **Best Final Value:** 2.565027e-15
* **Worst Final Value:** 2.526453e-13
* **Mean Final Value:** 4.083763e-14
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.632e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.402e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.125e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.948e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.994e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.740e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.465e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.904e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.526e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.482e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.294e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.136e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.335e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.191e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.710e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.986e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.054e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.941e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.488e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.016e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.173e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.289e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.782e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.168e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.708e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.055e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.556e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.796e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.932e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.563e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.541e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.361e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.668e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.694e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.783e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.401e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.214e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.059e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.330e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.379e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.089e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.799e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.474e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.218e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.962e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.769e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.786e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.441e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.565e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.595e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.367e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.150e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.632e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.523e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.736e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.573e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.189e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.386e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.225e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.457e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.427e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.940e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.191e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.705e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.639e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.091e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.227e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.769e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.095e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.322e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.348e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.228e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.399e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.856e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.763e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.043e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.802e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.141e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.436e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.952e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">103</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (40 out of 40)
- **Average Iterations to Convergence:** 10.4
- **Average Function Evaluations:** 106.4
- **Average Time to Convergence:** 0.002s
- **Fastest Convergence:** 9 iterations (0.002s)
- **Slowest Convergence:** 11 iterations (0.003s)
## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 2.565027e-15
**Final Gradient Norm:** 9.595478e-8
**Iterations:** 11
**Convergence Reason:** GradientTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.226e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.546e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.974e-1, 1.072e0, 1.010e0, 1.017e0, 1.182e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.226e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.546e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.260e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.821e-1, -4.697e-1, -4.505e-1, -5.067e-1, -5.573e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.542e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.443e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.901e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.590e-2, -4.903e-2, -4.047e-2, -3.946e-2, -4.628e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.134e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.139e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.901e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.522e-2, -1.517e-2, -1.204e-2, -1.072e-2, -1.312e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.926e-3</td><td style="border: 1px solid #ddd; padding: 4px;">9.394e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.901e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.089e-3, -4.682e-3, -3.621e-3, -2.850e-3, -3.705e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">6</td><td style="border: 1px solid #ddd; padding: 4px;">1.739e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.766e-3</td><td style="border: 1px solid #ddd; padding: 4px;">7.802e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.400e-5, 4.290e-6, -5.779e-6, 1.691e-5, 6.957e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">7</td><td style="border: 1px solid #ddd; padding: 4px;">3.163e-9</td><td style="border: 1px solid #ddd; padding: 4px;">1.125e-4</td><td style="border: 1px solid #ddd; padding: 4px;">7.802e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.670e-6, 2.197e-6, -1.650e-6, 1.861e-6, -5.824e-8, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">8</td><td style="border: 1px solid #ddd; padding: 4px;">4.002e-11</td><td style="border: 1px solid #ddd; padding: 4px;">1.235e-5</td><td style="border: 1px solid #ddd; padding: 4px;">7.802e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.259e-7, 2.623e-7, -2.228e-7, 1.439e-7, -1.214e-7, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">2.840e-13</td><td style="border: 1px solid #ddd; padding: 4px;">1.015e-6</td><td style="border: 1px solid #ddd; padding: 4px;">7.802e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.930e-8, 2.857e-8, -2.178e-8, 1.833e-8, -1.590e-8, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">2.565e-15</td><td style="border: 1px solid #ddd; padding: 4px;">9.595e-8</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.930e-8, 2.857e-8, -2.178e-8, 1.833e-8, -1.590e-8, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 106.4
- **Average Gradient Evaluations per Run:** 107.4
- **Average Iterations per Run:** 10.4
- **Average Time per Run:** 0.002s
- **Function Evaluations per Second:** 46173.2
- **Iterations per Second:** 4524.0
### Resource Utilization
- **Total Function Evaluations:** 4256
- **Total Gradient Evaluations:** 4296
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 0.99
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](problems/SparseQuadratic_10D_pattern[1,_3]_results.csv)
* [Convergence Plot](convergence_SparseQuadratic_10D_pattern[1,_3].png)
* [Log Scale Convergence Plot](convergence_SparseQuadratic_10D_pattern[1,_3]_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on SparseQuadratic_10D_pattern[1, 3]*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
