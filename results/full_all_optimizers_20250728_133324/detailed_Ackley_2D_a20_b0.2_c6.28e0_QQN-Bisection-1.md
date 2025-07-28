# Detailed Analysis: QQN-Bisection-1 on Ackley_2D_a20_b0.2_c6.28e0
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Ackley_2D_a20_b0.2_c6.28e0
**Optimizer:** QQN-Bisection-1
**Problem Family:** Highly Multimodal
**Dimension:** 2
**Success Threshold:** 2.640e0
**Total Runs:** 40
**Successful Runs:** 3 (7.5%)

### Quick Statistics
* **Best Final Value:** 1.091248e-9
* **Worst Final Value:** 3.574452e0
* **Mean Final Value:** 2.874431e0
* **Success Rate:** 7.5%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.953e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.166e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.828e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">504</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.712e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.310e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.815e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.696e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.897e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.913e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.580e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.765e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.580e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.648e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.896e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.832e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">556</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.745e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.385e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.447e-11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.092e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.828e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.872e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.224e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.570e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.342e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.829e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">496</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">562</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.162e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.113e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.078e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.091e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.828e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.580e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.348e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.452e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.718e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.505e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.776e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.455e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.081e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.271e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.744e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.049e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.575e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.829e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">543</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.056e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.504e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.506e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.249e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.293e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.384e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.574e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.849e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (3 out of 40)
- **Average Iterations to Convergence:** 7.7
- **Average Function Evaluations:** 116.7
- **Average Time to Convergence:** 0.002s
- **Fastest Convergence:** 7 iterations (0.002s)
- **Slowest Convergence:** 8 iterations (0.003s)
### Failed Runs (37 out of 40)

**Failure Reasons:**
- NumericalError: 3 runs
- GradientTolerance: 30 runs
- MaxFunctionEvaluations: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 1.091248e-9
**Final Gradient Norm:** 2.828427e0
**Iterations:** 13
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.799e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.385e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.174e0, 1.078e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.799e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.385e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.826e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.256e-1, 2.686e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.198e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.829e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.250e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.332e-1, -1.467e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.380e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.220e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.390e-2, 6.183e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.007e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.134e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.761e-2, -3.347e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">8</td><td style="border: 1px solid #ddd; padding: 4px;">1.707e-5</td><td style="border: 1px solid #ddd; padding: 4px;">2.829e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[1.487e-6, -2.986e-6]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">9.434e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.829e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[2.843e-7, -5.710e-7]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">1.804e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.828e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.384e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.627e-8, 3.267e-8]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">11</td><td style="border: 1px solid #ddd; padding: 4px;">1.032e-7</td><td style="border: 1px solid #ddd; padding: 4px;">2.828e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[2.520e-9, -5.061e-9]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">12</td><td style="border: 1px solid #ddd; padding: 4px;">1.599e-8</td><td style="border: 1px solid #ddd; padding: 4px;">2.828e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.863e-9</td><td style="border: 1px solid #ddd; padding: 4px;">[1.720e-10, -3.454e-10]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 167.4
- **Average Gradient Evaluations per Run:** 208.1
- **Average Iterations per Run:** 8.4
- **Average Time per Run:** 0.004s
- **Function Evaluations per Second:** 43084.1
- **Iterations per Second:** 2161.6
### Resource Utilization
- **Total Function Evaluations:** 6697
- **Total Gradient Evaluations:** 8322
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 0.80
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 3
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Ackley_2D_a20_b0.2_c6.28e0_results.csv)
* [Convergence Plot](convergence_Ackley_2D_a20_b0.2_c6.28e0.png)
* [Log Scale Convergence Plot](convergence_Ackley_2D_a20_b0.2_c6.28e0_log.png)


---
*Detailed report for QQN-Bisection-1 on Ackley_2D_a20_b0.2_c6.28e0*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
