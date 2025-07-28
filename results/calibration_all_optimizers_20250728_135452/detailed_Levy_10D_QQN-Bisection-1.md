# Detailed Analysis: QQN-Bisection-1 on Levy_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_10D
**Optimizer:** QQN-Bisection-1
**Problem Family:** Levy
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 40
**Successful Runs:** 40 (100.0%)

### Quick Statistics
* **Best Final Value:** 1.864837e-16
* **Worst Final Value:** 5.963526e-13
* **Mean Final Value:** 1.091327e-13
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.089e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.457e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.032e-16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.470e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.230e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.425e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">197</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.222e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.095e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.222e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.034e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.806e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.784e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.456e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.246e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.720e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.097e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.801e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.458e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.456e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.523e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.584e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.101e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.801e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.730e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.363e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.067e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.421e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.370e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.368e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.192e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.387e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.666e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.964e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.480e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">197</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.616e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.772e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.848e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.076e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.730e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.602e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.865e-16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.732e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.051e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.463e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.555e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.562e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.565e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.167e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.034e-16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.601e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.262e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.074e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">206</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.224e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.596e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.777e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.813e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.627e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.212e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.518e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.425e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">206</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.073e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.804e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.629e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.545e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.447e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.505e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.925e-16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.027e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.117e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.376e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.839e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.902e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.758e-15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.405e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.421e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.644e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.990e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.534e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.055e-14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.155e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (40 out of 40)
- **Average Iterations to Convergence:** 13.3
- **Average Function Evaluations:** 147.1
- **Average Time to Convergence:** 0.004s
- **Fastest Convergence:** 12 iterations (0.003s)
- **Slowest Convergence:** 16 iterations (0.005s)
## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 1.864837e-16
**Final Gradient Norm:** 2.731877e-8
**Iterations:** 14
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.747e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.544e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.815e0, 2.167e0, 1.841e0, 1.893e0, 2.187e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.747e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.544e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.766e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.992e-1, 1.383e0, 9.979e-1, 1.033e0, 1.418e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">5.798e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.120e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.147e0, 8.535e-1, 1.000e0, 9.983e-1, 8.332e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">7.980e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.215e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.173e-1, 9.392e-1, 9.999e-1, 1.000e0, 9.273e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.452e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.280e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.659e-1, 9.662e-1, 1.000e0, 1.000e0, 9.586e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">2.556e-7</td><td style="border: 1px solid #ddd; padding: 4px;">6.050e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">3.719e-10</td><td style="border: 1px solid #ddd; padding: 4px;">3.936e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">11</td><td style="border: 1px solid #ddd; padding: 4px;">1.755e-11</td><td style="border: 1px solid #ddd; padding: 4px;">5.909e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">12</td><td style="border: 1px solid #ddd; padding: 4px;">6.907e-13</td><td style="border: 1px solid #ddd; padding: 4px;">1.305e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">13</td><td style="border: 1px solid #ddd; padding: 4px;">1.865e-16</td><td style="border: 1px solid #ddd; padding: 4px;">2.732e-8</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0, 1.000e0, 1.000e0, 1.000e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 147.1
- **Average Gradient Evaluations per Run:** 183.4
- **Average Iterations per Run:** 13.3
- **Average Time per Run:** 0.004s
- **Function Evaluations per Second:** 41452.6
- **Iterations per Second:** 3748.6
### Resource Utilization
- **Total Function Evaluations:** 5883
- **Total Gradient Evaluations:** 7335
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 0.80
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](problems/Levy_10D_results.csv)
* [Convergence Plot](convergence_Levy_10D.png)
* [Log Scale Convergence Plot](convergence_Levy_10D_log.png)


---
*Detailed report for QQN-Bisection-1 on Levy_10D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
