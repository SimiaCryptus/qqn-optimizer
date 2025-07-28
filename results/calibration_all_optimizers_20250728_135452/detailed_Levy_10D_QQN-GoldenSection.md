# Detailed Analysis: QQN-GoldenSection on Levy_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_10D
**Optimizer:** QQN-GoldenSection
**Problem Family:** Levy
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 60
**Successful Runs:** 54 (90.0%)

### Quick Statistics
* **Best Final Value:** 9.027376e-17
* **Worst Final Value:** 1.361746e-12
* **Mean Final Value:** 1.881344e-13
* **Success Rate:** 90.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.236e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.980e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.189e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.359e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.768e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.001e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.027e-17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.982e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.421e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.119e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.946e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.703e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.223e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.900e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.681e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.777e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.126e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.907e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.515e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.411e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">644</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.792e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.638e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.711e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.387e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.768e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.978e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.556e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.762e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.330e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.071e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.999e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.718e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.977e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.258e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.596e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.808e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.782e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.536e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">518</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.726e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.639e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">596</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.350e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.952e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">607</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.443e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.893e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.525e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.449e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">607</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.164e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.141e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.241e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.443e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.223e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.694e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.213e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.775e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.646e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.501e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.834e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.363e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.575e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.009e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.819e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.838e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.012e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.043e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.362e-12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.394e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.699e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.124e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.425e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.458e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">485</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.045e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.292e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.357e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.152e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">741</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.382e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.057e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.634e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.628e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.140e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">751</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.151e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.623e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">639</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.271e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.065e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">751</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.599e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.841e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">662</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.580e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.151e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">235</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.163e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.464e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.571e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.571e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.835e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.447e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.279e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.808e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.979e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.420e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.418e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.626e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.252e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.288e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.114e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.307e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">53</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.052e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.011e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">797</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">211</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.459e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.213e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">69</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.036e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.549e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">663</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.550e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.853e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.641e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.644e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">579</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.740e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.141e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">607</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.942e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.838e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.171e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.958e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (54 out of 60)
- **Average Iterations to Convergence:** 13.5
- **Average Function Evaluations:** 531.2
- **Average Time to Convergence:** 0.009s
- **Fastest Convergence:** 11 iterations (0.007s)
- **Slowest Convergence:** 19 iterations (0.012s)
### Failed Runs (6 out of 60)

**Failure Reasons:**
- MaxFunctionEvaluations: 6 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 9.027376e-17
**Final Gradient Norm:** 1.981779e-8
**Iterations:** 13
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.630e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.010e0, 1.853e0, 2.084e0, 2.106e0, 2.058e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.625e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.630e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.071e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.792e-1, 9.717e-1, 1.212e0, 1.247e0, 1.175e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.726e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.439e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.196e0, 9.992e-1, 9.453e-1, 9.287e-1, 9.616e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.189e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.385e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.406e-1, 9.985e-1, 9.798e-1, 9.717e-1, 9.872e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.294e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.271e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.681e-1, 1.001e0, 9.812e-1, 9.754e-1, 9.866e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">8</td><td style="border: 1px solid #ddd; padding: 4px;">3.369e-7</td><td style="border: 1px solid #ddd; padding: 4px;">6.113e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">1.497e-9</td><td style="border: 1px solid #ddd; padding: 4px;">2.279e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">1.495e-11</td><td style="border: 1px solid #ddd; padding: 4px;">5.395e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">11</td><td style="border: 1px solid #ddd; padding: 4px;">5.685e-12</td><td style="border: 1px solid #ddd; padding: 4px;">3.252e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">12</td><td style="border: 1px solid #ddd; padding: 4px;">9.027e-17</td><td style="border: 1px solid #ddd; padding: 4px;">1.982e-8</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 554.2
- **Average Gradient Evaluations per Run:** 94.7
- **Average Iterations per Run:** 16.1
- **Average Time per Run:** 0.010s
- **Function Evaluations per Second:** 57322.1
- **Iterations per Second:** 1666.9
### Resource Utilization
- **Total Function Evaluations:** 33253
- **Total Gradient Evaluations:** 5682
- **Total Computation Time:** 0.6s
- **Function/Gradient Ratio:** 5.85
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Levy_10D_results.csv)
* [Convergence Plot](convergence_Levy_10D.png)
* [Log Scale Convergence Plot](convergence_Levy_10D_log.png)


---
*Detailed report for QQN-GoldenSection on Levy_10D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
