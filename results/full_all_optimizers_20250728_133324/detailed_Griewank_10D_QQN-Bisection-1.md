# Detailed Analysis: QQN-Bisection-1 on Griewank_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Griewank_10D
**Optimizer:** QQN-Bisection-1
**Problem Family:** Griewank
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 6.160318e-10
* **Worst Final Value:** 2.578816e1
* **Mean Final Value:** 1.052857e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.190e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.022e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">512</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">541</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.415e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.523e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">525</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.027e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.164e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">532</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">483</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.113e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.831e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">577</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.580e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.058e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.122e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.089e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">558</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.161e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.663e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">500</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.870e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.059e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">350</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.496e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.565e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.007e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.014e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.237e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.413e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">512</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.059e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.326e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">483</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">524</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.017e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.629e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.160e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.326e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.575e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.352e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">498</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.376e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.513e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">585</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.259e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.118e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">434</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.843e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.830e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">498</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.946e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.905e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">507</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.017e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.574e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">556</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.922e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.774e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">496</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">568</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.575e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.584e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.003e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.550e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">515</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.578e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.572e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.576e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.588e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.415e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.989e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">516</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">532</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.574e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.506e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.569e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.565e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.574e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.569e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.574e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.525e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.578e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.912e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.050e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">454</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">595</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.577e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.575e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.737e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.381e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">514</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">499</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.579e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.541e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.576e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.544e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.814e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.895e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">527</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">508</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.573e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.557e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.577e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.505e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.578e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.530e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">522</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- MaxFunctionEvaluations: 39 runs
- FunctionTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 6.160318e-10
**Final Gradient Norm:** 1.326441e-5
**Iterations:** 21
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.600e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.509e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.002e2, 9.982e1, 1.001e2, 9.988e1, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.600e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.509e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.503e1, 9.737e1, 9.585e1, 9.483e1, 9.521e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.370e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.513e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.499e1, 9.732e1, 9.580e1, 9.478e1, 9.516e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.368e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.513e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-6</td><td style="border: 1px solid #ddd; padding: 4px;">[9.499e1, 9.732e1, 9.580e1, 9.478e1, 9.516e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.368e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.513e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.138e0, -5.450e0, -7.164e0, -4.896e0, -9.053e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">17</td><td style="border: 1px solid #ddd; padding: 4px;">4.307e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.055e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.296e-4, -5.191e-4, 2.995e-4, 2.959e-4, 6.318e-4, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">18</td><td style="border: 1px solid #ddd; padding: 4px;">1.633e-6</td><td style="border: 1px solid #ddd; padding: 4px;">6.825e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.304e-5, -1.336e-4, -1.975e-4, -1.345e-4, 2.321e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">19</td><td style="border: 1px solid #ddd; padding: 4px;">2.751e-8</td><td style="border: 1px solid #ddd; padding: 4px;">1.164e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[4.967e-5, 1.141e-5, 2.116e-4, 5.403e-5, -2.391e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">20</td><td style="border: 1px solid #ddd; padding: 4px;">1.025e-8</td><td style="border: 1px solid #ddd; padding: 4px;">8.953e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.322e-6, 5.321e-6, 1.352e-5, 5.157e-6, 8.465e-6, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">21</td><td style="border: 1px solid #ddd; padding: 4px;">6.160e-10</td><td style="border: 1px solid #ddd; padding: 4px;">1.326e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.322e-6, 5.321e-6, 1.352e-5, 5.157e-6, 8.465e-6, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 499.8
- **Average Gradient Evaluations per Run:** 526.0
- **Average Iterations per Run:** 16.2
- **Average Time per Run:** 0.013s
- **Function Evaluations per Second:** 37735.6
- **Iterations per Second:** 1227.0
### Resource Utilization
- **Total Function Evaluations:** 19991
- **Total Gradient Evaluations:** 21041
- **Total Computation Time:** 0.5s
- **Function/Gradient Ratio:** 0.95
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Griewank_10D_results.csv)
* [Convergence Plot](convergence_Griewank_10D.png)
* [Log Scale Convergence Plot](convergence_Griewank_10D_log.png)


---
*Detailed report for QQN-Bisection-1 on Griewank_10D*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
