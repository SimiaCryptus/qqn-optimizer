# Detailed Analysis: GD on Barrier_2D_mu0.1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Barrier_2D_mu0.1
**Optimizer:** GD
**Problem Family:** Barrier
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.995732e-1
* **Worst Final Value:** 3.995732e-1
* **Mean Final Value:** 3.995732e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.844e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">279</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">281</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.604e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.968e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.963e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">590</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.621e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">588</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.862e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.883e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">285</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.719e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.838e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.941e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">588</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.928e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">298</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.829e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.795e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.702e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">285</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.903e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">580</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.663e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.704e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">576</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.742e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.895e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.684e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.794e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">570</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.713e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.793e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.864e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.961e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.613e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.720e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.783e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.676e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.971e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.833e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.849e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">298</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.877e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.648e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.610e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.815e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.640e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.756e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">298</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.762e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">590</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.667e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- GradientTolerance: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 3.995732e-1
**Final Gradient Norm:** 9.603941e-5
**Iterations:** 290
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.971e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.661e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.130e0, 8.292e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.971e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.661e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.108e0, 8.138e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.900e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.605e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.087e0, 7.988e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.833e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.550e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.066e0, 7.840e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.769e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.496e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.046e0, 7.696e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">285</td><td style="border: 1px solid #ddd; padding: 4px;">3.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.131e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">286</td><td style="border: 1px solid #ddd; padding: 4px;">3.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.086e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">287</td><td style="border: 1px solid #ddd; padding: 4px;">3.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.042e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">288</td><td style="border: 1px solid #ddd; padding: 4px;">3.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">289</td><td style="border: 1px solid #ddd; padding: 4px;">3.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.604e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 291.9
- **Average Gradient Evaluations per Run:** 581.7
- **Average Iterations per Run:** 289.9
- **Average Time per Run:** 0.007s
- **Function Evaluations per Second:** 42134.6
- **Iterations per Second:** 41845.9
### Resource Utilization
- **Total Function Evaluations:** 11674
- **Total Gradient Evaluations:** 23268
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Barrier_2D_mu0.1_results.csv)
* [Convergence Plot](convergence_Barrier_2D_mu0.1.png)
* [Log Scale Convergence Plot](convergence_Barrier_2D_mu0.1_log.png)


---
*Detailed report for GD on Barrier_2D_mu0.1*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
