# Detailed Analysis: QQN-GoldenSection on GoldsteinPrice_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** GoldsteinPrice_2D
**Optimizer:** QQN-GoldenSection
**Problem Family:** Non-Convex Unimodal
**Dimension:** 2
**Success Threshold:** 1.700e2
**Total Runs:** 60
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 8.400000e1
* **Worst Final Value:** 8.400000e1
* **Mean Final Value:** 8.400000e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.489e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.737e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">454</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.020e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">322</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.114e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">419</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.877e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.793e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.623e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.025e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">432</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.458e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.283e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.036e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.655e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.375e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.984e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.782e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.571e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">369</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.699e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.653e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">246</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.847e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.583e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.783e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.341e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.113e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">421</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.943e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.595e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">374</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.545e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">241</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.695e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.244e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">299</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.274e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.065e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.438e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.489e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.104e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.579e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.376e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.108e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">371</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.476e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">425</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.491e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">355</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.228e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">368</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.507e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.390e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.553e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.648e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.086e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">365</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">45</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.146e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.482e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">47</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.785e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.974e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.075e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.284e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.795e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.199e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">53</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.051e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">371</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.254e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.246e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.540e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">376</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.755e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">317</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.138e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.175e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.400e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.045e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (60 out of 60)

**Failure Reasons:**
- NumericalError: 60 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 13)
**Final Value:** 8.400000e1
**Final Gradient Norm:** 3.375041e-4
**Iterations:** 8
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.681e3</td><td style="border: 1px solid #ddd; padding: 4px;">8.401e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.925e-1, 9.689e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.681e3</td><td style="border: 1px solid #ddd; padding: 4px;">8.401e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.295e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.608e0, 7.194e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.806e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.914e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.637e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.791e0, 1.984e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.420e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.902e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.860e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.793e0, 1.952e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.401e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.757e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.805e0, 2.031e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">4</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.177e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">5</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.917e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">6</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.735e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">7</td><td style="border: 1px solid #ddd; padding: 4px;">8.400e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.109e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.643e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[1.800e0, 2.000e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 370.4
- **Average Gradient Evaluations per Run:** 58.3
- **Average Iterations per Run:** 9.4
- **Average Time per Run:** 0.005s
- **Function Evaluations per Second:** 68714.4
- **Iterations per Second:** 1740.5
### Resource Utilization
- **Total Function Evaluations:** 22227
- **Total Gradient Evaluations:** 3498
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 6.35
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 60
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/GoldsteinPrice_2D_results.csv)
* [Convergence Plot](convergence_GoldsteinPrice_2D.png)
* [Log Scale Convergence Plot](convergence_GoldsteinPrice_2D_log.png)


---
*Detailed report for QQN-GoldenSection on GoldsteinPrice_2D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
