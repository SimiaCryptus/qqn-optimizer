# Detailed Analysis: L-BFGS-Limited on GoldsteinPrice_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** GoldsteinPrice_2D
**Optimizer:** L-BFGS-Limited
**Problem Family:** GoldsteinPrice
**Dimension:** 2
**Success Threshold:** 8.400e1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 8.400000e1
* **Worst Final Value:** 8.537260e1
* **Mean Final Value:** 8.413342e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.401e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.671e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">734</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.400e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">242</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.958e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">255</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4232</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.425e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">241</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">728</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.868e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.424e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.897e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.537e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.120e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4258</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.406e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.473e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.401e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.566e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">755</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.310e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">746</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.412e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.137e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">749</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.559e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">259</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4231</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">782</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.402e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.945e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4252</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">752</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.614e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">240</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">725</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.150e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">240</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">725</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.478e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.505e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">280</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.420e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">734</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.623e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.405e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.206e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">256</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4247</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">773</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.158e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">241</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">728</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 17)
**Final Value:** 8.400000e1
**Final Gradient Norm:** 4.420191e-2
**Iterations:** 243
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.941e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.256e-1, 9.564e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.941e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e4</td><td style="border: 1px solid #ddd; padding: 4px;">6.153e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.284e0, 4.564e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.985e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.661e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.475e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.784e0, 2.333e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.551e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.806e0, 2.008e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.409e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.770e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.883e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.803e0, 2.048e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">239</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.887e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.104e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">240</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.195e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.221e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">241</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.491e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.104e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">242</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.458e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.221e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">243</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.420e-2</td><td style="border: 1px solid #ddd; padding: 4px;">6.104e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 4259.4
- **Average Gradient Evaluations per Run:** 752.8
- **Average Iterations per Run:** 249.2
- **Average Time per Run:** 0.038s
- **Function Evaluations per Second:** 113095.0
- **Iterations per Second:** 6618.1
### Resource Utilization
- **Total Function Evaluations:** 85188
- **Total Gradient Evaluations:** 15055
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 5.66
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/GoldsteinPrice_2D_results.csv)
* [Convergence Plot](../plots/GoldsteinPrice_2D.png)
* [Log Scale Convergence Plot](../plots/GoldsteinPrice_2D_log.png)


---
*Detailed report for L-BFGS-Limited on GoldsteinPrice_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
