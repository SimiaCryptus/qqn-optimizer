use crate::benchmarks::evaluation::{BenchmarkResults, OptimizationTrace};
use anyhow::Result;
use log::{debug, error, info, warn};
use plotters::backend::BitMapBackend;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use std::collections::HashMap;
use std::{fs, panic};

/// Check if font rendering is available
fn has_font_support() -> bool {
    info!("Checking font rendering support...");
    // Check if we're in a CI/test environment where fonts might not be available
    if std::env::var("CI").is_ok() || std::env::var("TEST_ENV").is_ok() {
        info!("Detected CI/test environment, disabling font support");
        return false;
    }

    // Try to create a simple font and actually draw text to see if it works
    debug!("Testing font rendering with sample text...");
    let result = std::panic::catch_unwind(|| {
        use plotters::prelude::*;

        // Create a small in-memory bitmap to test font rendering
        let mut buffer = vec![0u8; 100 * 100 * 3];
        let root = BitMapBackend::with_buffer(&mut buffer, (100, 100)).into_drawing_area();

        // Try to actually draw text
        root.draw(&Text::new("test", (50, 50), ("sans-serif", 12).into_font()))?;

        Ok::<(), Box<dyn std::error::Error>>(())
    });

    let ok = result.is_ok() && result.unwrap().is_ok();
    info!("Font support available: {}", ok);
    ok
}

#[derive(Debug, Clone)]
pub struct PlotConfig {
    pub width: u32,
    pub height: u32,
    pub output_format: String,
    pub color_scheme: String,
    pub enable_legends: bool,
    pub enable_grid: bool,
}
impl Default for PlotConfig {
    fn default() -> Self {
        info!("Creating default plot configuration");
        Self {
            width: 1024,
            height: 768,
            output_format: "png".to_string(),
            color_scheme: "default".to_string(),
            enable_legends: false, // todo: debug
            enable_grid: true,
        }
    }
}

/// Extended optimization trace with additional fields for plotting
#[derive(Debug, Clone)]
pub struct ExtendedOptimizationTrace {
    pub optimizer_name: String,
    pub objective_values: Vec<f64>,
    pub evaluation_counts: Vec<usize>,
}
impl ExtendedOptimizationTrace {
    /// Sanitize trace data by filtering out NaN and infinite values
    fn sanitize(&mut self) {
        debug!(
            "Sanitizing trace data for optimizer: {}",
            self.optimizer_name
        );
        let original_count = self.objective_values.len();
        let mut valid_indices = Vec::new();
        for (i, &val) in self.objective_values.iter().enumerate() {
            if val.is_finite() {
                valid_indices.push(i);
            }
        }
        if valid_indices.len() < self.objective_values.len() {
            let filtered_count = self.objective_values.len() - valid_indices.len();
            warn!(
                "Filtered {} non-finite values from trace for {} (kept {}/{})",
                filtered_count,
                self.optimizer_name,
                valid_indices.len(),
                original_count
            );
            self.objective_values = valid_indices
                .iter()
                .map(|&i| self.objective_values[i])
                .collect();
            if self.evaluation_counts.len()
                == self.objective_values.len() + valid_indices.len() - self.objective_values.len()
            {
                self.evaluation_counts = valid_indices
                    .iter()
                    .map(|&i| self.evaluation_counts[i])
                    .collect();
            }
        } else {
            debug!(
                "No non-finite values found in trace for {}",
                self.optimizer_name
            );
        }
        info!(
            "Trace sanitization complete for {}: {} valid data points",
            self.optimizer_name,
            self.objective_values.len()
        );
    }
}

impl From<&OptimizationTrace> for ExtendedOptimizationTrace {
    fn from(trace: &OptimizationTrace) -> Self {
        debug!("Converting OptimizationTrace to ExtendedOptimizationTrace");
        let mut result = Self {
            optimizer_name: "Unknown".to_string(),
            objective_values: trace
                .iterations
                .iter()
                .map(|iter| iter.function_value)
                .collect(),
            evaluation_counts: trace
                .iterations
                .iter()
                .map(|iter| iter.total_evaluations())
                .collect(),
        };
        info!(
            "Created extended trace with {} iterations",
            result.objective_values.len()
        );
        result.sanitize();
        result
    }
}

pub struct PlottingEngine {
    output_dir: String,
    width: u32,
    height: u32,
    config: PlotConfig,
    has_fonts: bool,
}

impl PlottingEngine {
    pub fn new(output_dir: String) -> Self {
        info!(
            "Initializing PlottingEngine with output directory: {}",
            output_dir
        );
        let has_fonts = has_font_support();
        if !has_fonts {
            warn!("Font support not available. Plots will be generated without text labels.");
        }
        info!(
            "PlottingEngine initialized with dimensions {}x{}, fonts: {}",
            1024, 768, has_fonts
        );
        Self {
            output_dir,
            width: 1024,
            height: 768,
            config: PlotConfig::default(),
            has_fonts,
        }
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        info!("Setting plot dimensions to {}x{}", width, height);
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self
    }
    pub fn with_config(mut self, config: PlotConfig) -> Self {
        info!("Applying custom plot configuration: {:?}", config);
        self.width = config.width;
        self.height = config.height;
        self.config = config;
        self
    }

    /// Create convergence plots showing objective value vs iterations
    pub fn convergence_plot(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        info!(
            "Creating convergence plot with {} traces, filename: {}",
            traces.len(),
            filename
        );
        // Wrap in panic handler
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            self.convergence_plot_impl(traces, filename)
        }));
        result.unwrap_or_else(|e| {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown panic in convergence_plot".to_string()
            };
            error!("Panic caught in convergence_plot: {}", msg);
            Err(anyhow::anyhow!("Plotting failed due to panic: {}", msg))
        })
    }
    fn convergence_plot_impl(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        if traces.is_empty() {
            warn!("No traces provided for convergence plot, skipping");
            return Ok(());
        }
        info!("Processing {} traces for convergence plot", traces.len());
        // Sanitize traces
        let mut sanitized_traces: Vec<ExtendedOptimizationTrace> = traces.to_vec();
        for trace in &mut sanitized_traces {
            trace.sanitize();
        }
        // Filter out empty traces
        let original_trace_count = sanitized_traces.len();
        sanitized_traces.retain(|t| !t.objective_values.is_empty());
        if sanitized_traces.len() < original_trace_count {
            warn!(
                "Filtered out {} empty traces",
                original_trace_count - sanitized_traces.len()
            );
        }
        if sanitized_traces.is_empty() {
            warn!("All traces were empty after sanitization, skipping plot");
            return Ok(());
        }
        info!("Using {} valid traces for plotting", sanitized_traces.len());
        // Group traces by optimizer name for consistent coloring
        let mut optimizer_traces: HashMap<String, Vec<&ExtendedOptimizationTrace>> = HashMap::new();
        for trace in &sanitized_traces {
            optimizer_traces
                .entry(trace.optimizer_name.clone())
                .or_default()
                .push(trace);
        }
        // Create a sorted list of unique optimizer names for consistent color assignment
        let mut unique_optimizers: Vec<String> = optimizer_traces.keys().cloned().collect();
        unique_optimizers.sort();
        info!(
            "Found {} unique optimizers: {:?}",
            unique_optimizers.len(),
            unique_optimizers
        );

        // Export CSV data for this plot
        info!("Exporting convergence data to CSV");
        self.export_convergence_csv(&sanitized_traces, filename)?;
        info!("Creating output directory: {}", filename);

        fs::create_dir_all(&filename)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let output_path = format!("{}/convergence.png", filename);
        info!("Generating convergence plot: {}", output_path);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();

        root.fill(&WHITE)?;

        // Find the range of evaluation counts and objective values
        let max_evaluations = sanitized_traces
            .iter()
            .map(|t| t.evaluation_counts.iter().max().unwrap_or(&0))
            .max()
            .unwrap_or(&0);

        let (min_obj, max_obj) = sanitized_traces
            .iter()
            .flat_map(|t| t.objective_values.iter())
            .filter(|&&val| val.is_finite())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });
        info!(
            "Data ranges - Evaluations: 0 to {}, Objective: {:.6e} to {:.6e}",
            max_evaluations, min_obj, max_obj
        );
        // Check if we have valid bounds
        if !min_obj.is_finite() || !max_obj.is_finite() || min_obj >= max_obj {
            warn!(
                "Invalid objective value bounds: [{:.6e}, {:.6e}], skipping plot",
                min_obj, max_obj
            );
            return Ok(());
        }

        // Add padding to the objective value range
        let obj_range = max_obj - min_obj;
        let padded_min = min_obj - obj_range * 0.1;
        let padded_max = max_obj + obj_range * 0.1;
        debug!(
            "Padded objective range: [{:.6e}, {:.6e}]",
            padded_min, padded_max
        );

        // Build chart without text elements to avoid font issues
        info!(
            "Building chart with dimensions {}x{}",
            self.width, self.height
        );
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(if self.has_fonts { 30 } else { 0 })
            .y_label_area_size(if self.has_fonts { 40 } else { 0 })
            .build_cartesian_2d(0..*max_evaluations, padded_min..padded_max)
            .map_err(|e| anyhow::anyhow!("Chart building error: {}", e))?;

        // Configure mesh based on font availability
        if self.has_fonts {
            debug!("Configuring chart with labels and descriptions");
            chart
                .configure_mesh()
                .x_desc("Max(Function, Gradient) Evaluations")
                .y_desc("Objective Value")
                .draw()?;
        } else {
            debug!("Configuring chart without labels (no font support)");
            chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()?;
        }

        // Color palette for different optimizers
        let colors: [&RGBColor; 24] = [
            &RED,
            &BLUE,
            &GREEN,
            &MAGENTA,
            &CYAN,
            &BLACK,
            &RGBColor(255, 165, 0),   // Orange
            &RGBColor(128, 0, 128),   // Purple
            &RGBColor(165, 42, 42),   // Brown
            &RGBColor(255, 192, 203), // Pink
            &RGBColor(128, 128, 128), // Grey
            &YELLOW,
            &RGBColor(173, 216, 230), // Light Blue
            &RGBColor(144, 238, 144), // Light Green
            &RGBColor(255, 182, 193), // Light Red
            &RGBColor(0, 0, 139),     // Dark Blue
            &RGBColor(0, 100, 0),     // Dark Green
            &RGBColor(139, 0, 0),     // Dark Red
            &RGBColor(0, 0, 128),     // Navy
            &RGBColor(128, 0, 0),     // Maroon
            &RGBColor(128, 128, 0),   // Olive
            &RGBColor(0, 128, 128),   // Teal
            &RGBColor(192, 192, 192), // Silver
            &RGBColor(0, 255, 0),     // Lime
        ];

        // Plot traces grouped by optimizer name
        info!("Drawing {} optimizer groups", unique_optimizers.len());
        for (optimizer_idx, optimizer_name) in unique_optimizers.iter().enumerate() {
            let color = colors[optimizer_idx % colors.len()];
            let traces_for_optimizer = &optimizer_traces[optimizer_name];
            info!(
                "Drawing {} traces/runs for optimizer: {}",
                traces_for_optimizer.len(),
                optimizer_name
            );

            for (run_idx, trace) in traces_for_optimizer.iter().enumerate() {
                let series_data: Vec<(usize, f64)> = trace
                    .evaluation_counts
                    .iter()
                    .zip(trace.objective_values.iter())
                    .filter(|(_, &obj_val)| obj_val.is_finite())
                    .map(|(&eval_count, &obj_val)| (eval_count, obj_val))
                    .collect();
                if series_data.is_empty() {
                    debug!("Skipping empty series for {}", optimizer_name);
                    continue;
                }
                info!(
                    "Drawing series with {} data points for {} (run {})",
                    series_data.len(),
                    optimizer_name,
                    run_idx
                );
                // Use slightly different line styles for multiple runs of the same optimizer
                let line_style = if run_idx == 0 {
                    (*color).to_rgba()
                } else {
                    // Make subsequent runs more transparent based on run index
                    (*color)
                        .mix(0.8 - (run_idx as f64 * 0.05).min(0.3))
                        .to_rgba()
                };

                // Draw series
                chart
                    .draw_series(LineSeries::new(series_data.clone(), &line_style))
                    .map_err(|e| anyhow::anyhow!("Series drawing error: {}", e))?;

                // Add markers at regular intervals for better visibility
                let marker_interval = series_data.len().max(1) / 20 + 1;
                debug!("Adding markers every {} points", marker_interval);
                chart
                    .draw_series(
                        series_data
                            .iter()
                            .step_by(marker_interval)
                            .map(|&(x, y)| Circle::new((x, y), 3, line_style.filled())),
                    )
                    .map_err(|e| anyhow::anyhow!("Marker drawing error: {}", e))?;
            }
        }
        // Try to add legend
        if self.config.enable_legends && unique_optimizers.len() > 1 && self.has_fonts {
            info!("Adding legend for {} optimizers", unique_optimizers.len());
            self.add_legend_for_optimizers(&root, &unique_optimizers, &colors)?;
        } else {
            debug!(
                "Skipping legend - enabled: {}, optimizers: {}, fonts: {}",
                self.config.enable_legends,
                unique_optimizers.len(),
                self.has_fonts
            );
        }

        root.present()?;
        info!("Convergence plot saved to: {}", output_path);
        Ok(())
    }

    /// Create log-scale convergence plots for better visualization of convergence
    pub fn log_convergence_plot(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        info!(
            "Creating log-scale convergence plot with {} traces, filename: {}",
            traces.len(),
            filename
        );
        // Wrap in panic handler
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            self.log_convergence_plot_impl(traces, filename)
        }));
        match result {
            Ok(inner_result) => inner_result,
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic in log_convergence_plot".to_string()
                };
                error!("Panic caught in log_convergence_plot: {}", msg);
                Err(anyhow::anyhow!("Plotting failed due to panic: {}", msg))
            }
        }
    }
    fn log_convergence_plot_impl(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        if traces.is_empty() {
            warn!("No traces provided for log convergence plot, skipping");
            return Ok(());
        }
        info!(
            "Processing {} traces for log convergence plot",
            traces.len()
        );
        // Sanitize traces
        let mut sanitized_traces: Vec<ExtendedOptimizationTrace> = traces.to_vec();
        for trace in &mut sanitized_traces {
            trace.sanitize();
        }
        // Filter out empty traces
        let original_trace_count = sanitized_traces.len();
        sanitized_traces.retain(|t| !t.objective_values.is_empty());
        if sanitized_traces.len() < original_trace_count {
            warn!(
                "Filtered out {} empty traces",
                original_trace_count - sanitized_traces.len()
            );
        }
        if sanitized_traces.is_empty() {
            warn!("All traces were empty after sanitization, skipping log plot");
            return Ok(());
        }
        info!(
            "Using {} valid traces for log plotting",
            sanitized_traces.len()
        );
        // Group traces by optimizer name for consistent coloring
        let mut optimizer_traces: HashMap<String, Vec<&ExtendedOptimizationTrace>> = HashMap::new();
        for trace in &sanitized_traces {
            optimizer_traces
                .entry(trace.optimizer_name.clone())
                .or_default()
                .push(trace);
        }
        // Create a sorted list of unique optimizer names for consistent color assignment
        let mut unique_optimizers: Vec<String> = optimizer_traces.keys().cloned().collect();
        unique_optimizers.sort();

        // Export CSV data for this plot
        info!("Exporting log convergence data to CSV");
        self.export_log_convergence_csv(&sanitized_traces, filename)?;
        info!("Creating output directory: {}", self.output_dir);

        fs::create_dir_all(&filename)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let output_path = format!("{}/log_convergence.png", filename);
        info!("`Generating log convergence plot`: {}", output_path);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_evaluations = sanitized_traces
            .iter()
            .map(|t| t.evaluation_counts.iter().max().unwrap_or(&0))
            .max()
            .unwrap_or(&0);

        // Find minimum positive objective value for log scale
        let min_positive_obj = sanitized_traces
            .iter()
            .flat_map(|t| t.objective_values.iter())
            .filter(|&&val| val > 0.0 && val.is_finite())
            .fold(f64::INFINITY, |min, &val| min.min(val))
            .max(1e-15); // Ensure minimum bound to prevent extreme log values

        let max_obj = sanitized_traces
            .iter()
            .flat_map(|t| t.objective_values.iter())
            .filter(|&&val| val.is_finite())
            .fold(f64::NEG_INFINITY, |max, &val| max.max(val))
            .min(1e10); // Cap maximum to prevent overflow
        info!(
            "Log scale data ranges - Min positive: {:.6e}, Max: {:.6e}",
            min_positive_obj, max_obj
        );
        // Check if we have any valid positive values
        if !min_positive_obj.is_finite() || !max_obj.is_finite() || min_positive_obj >= max_obj {
            warn!(
                "No valid positive values for log plot - min: {:.6e}, max: {:.6e}",
                min_positive_obj, max_obj
            );
            return Ok(());
        }

        // Safely calculate log bounds with overflow protection
        let safe_min = min_positive_obj.clamp(1e-12, 1e10);
        let safe_max = max_obj.clamp(1.0, 1e10);

        let log_min = safe_min.log10().clamp(-15.0, 15.0);
        let log_max = safe_max.log10().clamp(-15.0, 15.0);
        debug!("Log bounds - Min: {:.3}, Max: {:.3}", log_min, log_max);

        // Ensure we have a valid range
        let (final_log_min, final_log_max) = if (log_max - log_min).abs() < 1e-10 {
            debug!("Log range too small, expanding");
            (log_min - 1.0, log_min + 1.0)
        } else {
            (log_min, log_max)
        };
        info!(
            "Final log scale range: [{:.3}, {:.3}]",
            final_log_min, final_log_max
        );

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(if self.has_fonts { 30 } else { 0 })
            .y_label_area_size(if self.has_fonts { 50 } else { 0 })
            .build_cartesian_2d(0..*max_evaluations, final_log_min..final_log_max)?;

        // Configure mesh based on font availability
        if self.has_fonts {
            chart
                .configure_mesh()
                .x_desc("Max(Function, Gradient) Evaluations")
                .y_desc("Log10(Objective Value)")
                .draw()?;
        } else {
            chart.configure_mesh().draw()?;
        }

        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

        // Plot traces grouped by optimizer name
        info!(
            "Drawing {} optimizer groups for log plot",
            unique_optimizers.len()
        );
        for (optimizer_idx, optimizer_name) in unique_optimizers.iter().enumerate() {
            let color = colors[optimizer_idx % colors.len()];
            let traces_for_optimizer = &optimizer_traces[optimizer_name];
            debug!(
                "Drawing {} traces for optimizer: {} (log scale)",
                traces_for_optimizer.len(),
                optimizer_name
            );

            for (run_idx, trace) in traces_for_optimizer.iter().enumerate() {
                let series_data: Vec<(usize, f64)> = trace
                    .evaluation_counts
                    .iter()
                    .zip(trace.objective_values.iter())
                    .filter(|(_, &obj_val)| obj_val > 0.0 && obj_val.is_finite())
                    .map(|(&eval_count, &obj_val)| {
                        let safe_val = obj_val.clamp(1e-15, 1e10);
                        let log_val = safe_val.log10().clamp(-15.0, 15.0);
                        (eval_count, log_val)
                    })
                    .collect();

                // Only draw if we have valid data points
                if !series_data.is_empty() {
                    debug!(
                        "Drawing log series with {} data points for {} (run {})",
                        series_data.len(),
                        optimizer_name,
                        run_idx
                    );
                    // Use slightly different line styles for multiple runs of the same optimizer
                    let line_style = if run_idx == 0 {
                        (*color).to_rgba()
                    } else {
                        // Make subsequent runs more transparent based on run index
                        (*color)
                            .mix(0.8 - (run_idx as f64 * 0.05).min(0.3))
                            .to_rgba()
                    };

                    // Draw series
                    chart
                        .draw_series(LineSeries::new(series_data.clone(), &line_style))
                        .map_err(|e| anyhow::anyhow!("Series drawing error: {}", e))?;

                    // Add markers for better visibility
                    let marker_interval = series_data.len().max(1) / 20 + 1;
                    debug!("Adding log markers every {} points", marker_interval);
                    chart.draw_series(
                        series_data
                            .iter()
                            .step_by(marker_interval)
                            .map(|&(x, y)| Circle::new((x, y), 3, line_style.filled())),
                    )?;
                } else {
                    debug!("Skipping empty log series for {}", optimizer_name);
                }
            }
        }
        // Try to add legend
        if self.config.enable_legends && unique_optimizers.len() > 1 && self.has_fonts {
            info!(
                "Adding legend for {} optimizers (log plot)",
                unique_optimizers.len()
            );
            self.add_legend_for_optimizers(&root, &unique_optimizers, &colors)?;
        }

        root.present()?;
        info!("Log convergence plot saved to: {}", output_path);
        Ok(())
    }

    /// Create performance comparison bar charts
    pub fn performance_comparison(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        info!(
            "Creating performance comparison chart with {} results, filename: {}",
            results.results.len(),
            filename
        );
        // Wrap in panic handler
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            self.performance_comparison_impl(results, filename)
        }));
        match result {
            Ok(inner_result) => inner_result,
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic in performance_comparison".to_string()
                };
                error!("Panic caught in performance_comparison: {}", msg);
                Err(anyhow::anyhow!("Plotting failed due to panic: {}", msg))
            }
        }
    }
    fn performance_comparison_impl(
        &self,
        results: &BenchmarkResults,
        filename: &str,
    ) -> Result<()> {
        if results.results.is_empty() {
            warn!("No results provided for performance comparison, skipping");
            return Ok(());
        }
        info!(
            "Processing {} benchmark results for performance comparison",
            results.results.len()
        );
        // Export CSV data for this plot
        info!("Exporting performance comparison data to CSV");
        self.export_performance_comparison_csv(results, filename)?;
        info!("Creating output directory: {}", self.output_dir);

        fs::create_dir_all(&self.output_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        info!("Generating performance comparison plot: {}", output_path);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;

        // Group results by problem and optimizer
        let mut problem_results: HashMap<String, HashMap<String, Vec<f64>>> = HashMap::new();

        for result in &results.results {
            problem_results
                .entry(result.problem_name.clone())
                .or_default()
                .entry(result.optimizer_name.clone())
                .or_default()
                .push(if result.final_value.is_finite() {
                    result.final_value
                } else {
                    warn!(
                        "Non-finite final value for {} on {}: {:.6e}",
                        result.optimizer_name, result.problem_name, result.final_value
                    );
                    f64::NAN
                });
        }
        info!("Grouped results into {} problems", problem_results.len());

        // Calculate mean final values for each optimizer on each problem
        let mut chart_data: Vec<(String, Vec<(String, f64)>)> = Vec::new();

        for (problem, optimizers) in problem_results {
            debug!(
                "Processing problem: {} with {} optimizers",
                problem,
                optimizers.len()
            );
            let mut optimizer_means = Vec::new();
            for (optimizer, values) in optimizers {
                let finite_values: Vec<f64> =
                    values.iter().filter(|&&v| v.is_finite()).copied().collect();

                if !finite_values.is_empty() {
                    let mean = finite_values.iter().sum::<f64>() / finite_values.len() as f64;
                    if mean.is_finite() {
                        debug!(
                            "Optimizer {} on {}: mean = {:.6e} ({} runs)",
                            optimizer,
                            problem,
                            mean,
                            finite_values.len()
                        );
                        optimizer_means.push((optimizer, mean));
                    }
                }
            }
            optimizer_means.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by optimizer name
            if !optimizer_means.is_empty() {
                info!(
                    "Problem {} has {} valid optimizer results",
                    problem,
                    optimizer_means.len()
                );
                chart_data.push((problem, optimizer_means));
            }
        }
        if chart_data.is_empty() {
            warn!("No valid chart data after processing, skipping performance comparison");
            return Ok(());
        }
        info!("Created chart data for {} problems", chart_data.len());

        // Create subplot for each problem
        let num_problems = chart_data.len();
        let subplot_height = self.height / num_problems as u32;
        info!(
            "Creating {} subplots with height {} each",
            num_problems, subplot_height
        );
        let colors = [
            &RED,
            &BLUE,
            &GREEN,
            &MAGENTA,
            &CYAN,
            &BLACK,
            &RGBColor(255, 165, 0),   // Orange
            &RGBColor(128, 0, 128),   // Purple
            &RGBColor(165, 42, 42),   // Brown
            &RGBColor(255, 192, 203), // Pink
            &RGBColor(128, 128, 128), // Grey
            &YELLOW,
            &RGBColor(173, 216, 230), // Light Blue
            &RGBColor(144, 238, 144), // Light Green
            &RGBColor(255, 182, 193), // Light Red
            &RGBColor(0, 0, 139),     // Dark Blue
            &RGBColor(0, 100, 0),     // Dark Green
            &RGBColor(139, 0, 0),     // Dark Red
            &RGBColor(0, 0, 128),     // Navy
            &RGBColor(128, 0, 0),     // Maroon
            &RGBColor(128, 128, 0),   // Olive
            &RGBColor(0, 128, 128),   // Teal
            &RGBColor(192, 192, 192), // Silver
            &RGBColor(0, 255, 0),     // Lime
        ];

        for (i, (problem_name, optimizer_data)) in chart_data.iter().enumerate() {
            debug!("Drawing subplot {} for problem: {}", i, problem_name);
            let y_start = i as u32 * subplot_height;
            let y_end = (i + 1) as u32 * subplot_height;

            let subplot = root.margin(10, 10, y_start as i32, (self.height - y_end) as i32);

            let max_value = optimizer_data
                .iter()
                .map(|(_, val)| *val)
                .filter(|&val| val.is_finite())
                .fold(f64::NEG_INFINITY, f64::max);

            let min_value = optimizer_data
                .iter()
                .map(|(_, val)| *val)
                .filter(|&val| val.is_finite())
                .fold(f64::INFINITY, f64::min);
            debug!(
                "Problem {} value range: [{:.6e}, {:.6e}]",
                problem_name, min_value, max_value
            );
            if !min_value.is_finite() || !max_value.is_finite() || min_value >= max_value {
                warn!(
                    "Invalid value range for problem {}, skipping subplot",
                    problem_name
                );
                continue;
            }

            let value_range = max_value - min_value;
            let padded_min = min_value - value_range * 0.1;
            let padded_max = max_value + value_range * 0.1;

            let mut chart = ChartBuilder::on(&subplot)
                .margin(10)
                .x_label_area_size(if self.has_fonts { 30 } else { 0 })
                .y_label_area_size(if self.has_fonts { 50 } else { 0 })
                .build_cartesian_2d(0.0..(optimizer_data.len() as f64), padded_min..padded_max)?;

            // Configure mesh based on font availability
            if self.has_fonts {
                chart.configure_mesh().y_desc("Objective Value").draw()?;
            } else {
                chart
                    .configure_mesh()
                    .disable_x_mesh()
                    .disable_y_mesh()
                    .draw()?;
            }

            chart.draw_series(
                optimizer_data
                    .iter()
                    .enumerate()
                    .map(|(x, (_name, value))| {
                        let color = colors[x % colors.len()];
                        Rectangle::new(
                            [(x as f64, padded_min), (x as f64 + 0.8, *value)],
                            color.filled(),
                        )
                    }),
            )?;

            // Add problem name as title if fonts are available
            if self.has_fonts {
                debug!("Adding title for problem: {}", problem_name);
                let _ = subplot.draw(&Text::new(
                    problem_name.as_str(),
                    (subplot_height as i32 / 2, 10),
                    ("sans-serif", 20).into_font().color(&BLACK),
                ));
            }
        }

        root.present()?;
        info!("Performance comparison saved to: {}", output_path);
        Ok(())
    }

    /// Create box plots showing distribution of results
    pub fn performance_boxplot(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        info!(
            "Creating performance boxplot with {} results, filename: {}",
            results.results.len(),
            filename
        );
        // Wrap in panic handler
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            self.performance_boxplot_impl(results, filename)
        }));
        match result {
            Ok(inner_result) => inner_result,
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic in performance_boxplot".to_string()
                };
                error!("Panic caught in performance_boxplot: {}", msg);
                Err(anyhow::anyhow!("Plotting failed due to panic: {}", msg))
            }
        }
    }
    fn performance_boxplot_impl(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        if results.results.is_empty() {
            warn!("No results provided for performance boxplot, skipping");
            return Ok(());
        }
        info!(
            "Processing {} benchmark results for boxplot",
            results.results.len()
        );
        // Export CSV data for this plot
        info!("Exporting performance boxplot data to CSV");
        self.export_performance_boxplot_csv(results, filename)?;
        info!("Creating output directory: {}", self.output_dir);

        fs::create_dir_all(&self.output_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        info!("Generating performance boxplot: {}", output_path);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;

        // Group results by optimizer across all problems
        let mut optimizer_results: HashMap<String, Vec<f64>> = HashMap::new();

        for result in &results.results {
            optimizer_results
                .entry(result.optimizer_name.clone())
                .or_default()
                .push(if result.final_value.is_finite() {
                    result.final_value
                } else {
                    warn!(
                        "Non-finite final value for {}: {:.6e}",
                        result.optimizer_name, result.final_value
                    );
                    f64::NAN
                });
        }
        info!("Grouped results by {} optimizers", optimizer_results.len());

        // Calculate statistics for each optimizer
        let mut box_data: Vec<(String, BoxPlotData)> = Vec::new();

        for (optimizer, mut values) in optimizer_results {
            debug!(
                "Processing optimizer: {} with {} values",
                optimizer,
                values.len()
            );
            // Filter out non-finite values
            let original_count = values.len();
            values.retain(|&v| v.is_finite());
            if values.len() < original_count {
                debug!(
                    "Filtered {} non-finite values for {}",
                    original_count - values.len(),
                    optimizer
                );
            }
            if values.is_empty() {
                warn!("No finite values for optimizer {}, skipping", optimizer);
                continue;
            }

            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let n = values.len();

            if n > 0 {
                let q1 = values[n / 4];
                let median = values[n / 2];
                let q3 = values[3 * n / 4];
                let min = values[0];
                let max = values[n - 1];
                info!("Boxplot stats for {}: min={:.6e}, q1={:.6e}, median={:.6e}, q3={:.6e}, max={:.6e}", 
                      optimizer, min, q1, median, q3, max);

                box_data.push((
                    optimizer,
                    BoxPlotData {
                        min,
                        q1,
                        median,
                        q3,
                        max,
                    },
                ));
            }
        }

        box_data.sort_by(|a, b| a.0.cmp(&b.0));
        if box_data.is_empty() {
            warn!("No valid boxplot data after processing, skipping");
            return Ok(());
        }
        info!("Created boxplot data for {} optimizers", box_data.len());

        let global_min = box_data
            .iter()
            .map(|(_, data)| data.min)
            .filter(|&v| v.is_finite())
            .fold(f64::INFINITY, f64::min);
        let global_max = box_data
            .iter()
            .map(|(_, data)| data.max)
            .filter(|&v| v.is_finite())
            .fold(f64::NEG_INFINITY, f64::max);
        info!(
            "Global boxplot range: [{:.6e}, {:.6e}]",
            global_min, global_max
        );
        if !global_min.is_finite() || !global_max.is_finite() || global_min >= global_max {
            warn!(
                "Invalid global bounds for boxplot: [{:.6e}, {:.6e}]",
                global_min, global_max
            );
            return Ok(());
        }

        let value_range = global_max - global_min;
        let padded_min = global_min - value_range * 0.1;
        let padded_max = global_max + value_range * 0.1;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(if self.has_fonts { 60 } else { 0 })
            .y_label_area_size(if self.has_fonts { 50 } else { 0 })
            .build_cartesian_2d(0.0..(box_data.len() as f64), padded_min..padded_max)?;

        // Configure mesh based on font availability
        if self.has_fonts {
            chart
                .configure_mesh()
                .y_desc("Objective Value Distribution")
                .draw()?;
        } else {
            chart.configure_mesh().draw()?;
        }

        // Draw box plots
        info!("Drawing {} box plots", box_data.len());
        for (i, (name, data)) in box_data.iter().enumerate() {
            debug!("Drawing box plot {} for optimizer: {}", i, name);
            let x = i as f64;
            let box_width = 0.3;

            // Draw box (Q1 to Q3)
            chart.draw_series(std::iter::once(Rectangle::new(
                [
                    (x - box_width / 2.0, data.q1),
                    (x + box_width / 2.0, data.q3),
                ],
                BLUE.mix(0.3).filled(),
            )))?;

            // Draw median line
            chart.draw_series(std::iter::once(PathElement::new(
                vec![
                    (x - box_width / 2.0, data.median),
                    (x + box_width / 2.0, data.median),
                ],
                RED.stroke_width(2),
            )))?;

            // Draw whiskers
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x, data.min), (x, data.q1)],
                BLACK,
            )))?;

            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x, data.q3), (x, data.max)],
                BLACK,
            )))?;
            // Draw whisker caps
            let cap_width = box_width / 3.0;
            chart.draw_series(std::iter::once(PathElement::new(
                vec![
                    (x - cap_width / 2.0, data.min),
                    (x + cap_width / 2.0, data.min),
                ],
                BLACK,
            )))?;
            chart.draw_series(std::iter::once(PathElement::new(
                vec![
                    (x - cap_width / 2.0, data.max),
                    (x + cap_width / 2.0, data.max),
                ],
                BLACK,
            )))?;

            // Add optimizer names as x-axis labels if fonts are available
            if self.has_fonts {
                debug!("Adding x-axis label for: {}", name);
                let _ = root.draw(&Text::new(
                    name.as_str(),
                    (
                        ((i as f64 + 0.5) * self.width as f64 / box_data.len() as f64) as i32,
                        (self.height - 20) as i32,
                    ),
                    ("sans-serif", 15)
                        .into_font()
                        .color(&BLACK)
                        .pos(Pos::new(HPos::Center, VPos::Top)),
                ));
            }
        }

        root.present()?;
        info!("Performance boxplot saved to: {}", output_path);
        Ok(())
    }

    /// Export convergence plot data to CSV
    fn export_convergence_csv(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        debug!("Exporting convergence data for {} traces", traces.len());
        fs::create_dir_all(&filename)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let csv_path = format!("{}/data.csv", filename);
        info!("Writing convergence CSV to: {}", csv_path);
        let mut csv_content = String::from("Optimizer,MaxEvaluation,ObjectiveValue\n");
        let mut total_rows = 0;
        for trace in traces {
            let mut trace_rows = 0;
            for (eval_count, obj_value) in trace
                .evaluation_counts
                .iter()
                .zip(trace.objective_values.iter())
            {
                if !obj_value.is_finite() {
                    continue;
                }
                csv_content.push_str(&format!(
                    "{},{},{:.6e}\n",
                    trace.optimizer_name, eval_count, obj_value
                ));
                trace_rows += 1;
                total_rows += 1;
            }
            debug!(
                "Exported {} rows for optimizer: {}",
                trace_rows, trace.optimizer_name
            );
        }
        fs::write(&csv_path, csv_content)
            .map_err(|e| anyhow::anyhow!("Failed to write CSV file {}: {}", csv_path, e))?;
        info!(
            "Convergence data exported to: {} ({} total rows)",
            csv_path, total_rows
        );
        Ok(())
    }
    /// Export log convergence plot data to CSV
    fn export_log_convergence_csv(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        debug!("Exporting log convergence data for {} traces", traces.len());
        fs::create_dir_all(&filename)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let csv_path = format!("{}/log_data.csv", filename);
        info!("Writing log convergence CSV to: {}", csv_path);
        let mut csv_content =
            String::from("Optimizer,MaxEvaluation,ObjectiveValue,LogObjectiveValue\n");
        let mut total_rows = 0;
        for trace in traces {
            let mut trace_rows = 0;
            for (eval_count, obj_value) in trace
                .evaluation_counts
                .iter()
                .zip(trace.objective_values.iter())
            {
                if !obj_value.is_finite() || *obj_value <= 0.0 {
                    continue;
                }
                let safe_val = obj_value.clamp(1e-15, 1e10);
                let log_val = safe_val.log10().clamp(-15.0, 15.0);
                csv_content.push_str(&format!(
                    "{},{},{:.6e},{:.6e}\n",
                    trace.optimizer_name, eval_count, obj_value, log_val
                ));
                trace_rows += 1;
                total_rows += 1;
            }
            debug!(
                "Exported {} log rows for optimizer: {}",
                trace_rows, trace.optimizer_name
            );
        }
        fs::write(&csv_path, csv_content)
            .map_err(|e| anyhow::anyhow!("Failed to write CSV file {}: {}", csv_path, e))?;
        info!(
            "Log convergence data exported to: {} ({} total rows)",
            csv_path, total_rows
        );
        Ok(())
    }
    /// Export performance comparison data to CSV
    fn export_performance_comparison_csv(
        &self,
        results: &BenchmarkResults,
        filename: &str,
    ) -> Result<()> {
        debug!(
            "Exporting performance comparison data for {} results",
            results.results.len()
        );
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let csv_path = format!("{}/{}_data.csv", self.output_dir, filename);
        info!("Writing performance comparison CSV to: {}", csv_path);
        let mut csv_content = String::from("Problem,Optimizer,MeanFinalValue,StdFinalValue,MeanIterations,MeanFunctionEvals,MeanGradientEvals,SuccessRate\n");
        // Group results by problem and optimizer
        let mut problem_results: HashMap<String, HashMap<String, Vec<f64>>> = HashMap::new();
        for result in &results.results {
            problem_results
                .entry(result.problem_name.clone())
                .or_default()
                .entry(result.optimizer_name.clone())
                .or_default()
                .push(result.final_value);
        }
        let mut total_rows = 0;
        for (problem, optimizers) in problem_results {
            debug!(
                "Processing problem: {} with {} optimizers",
                problem,
                optimizers.len()
            );
            for (optimizer, values) in optimizers {
                let optimizer_results: Vec<_> = results
                    .results
                    .iter()
                    .filter(|r| r.problem_name == problem && r.optimizer_name == optimizer)
                    .collect();
                if !optimizer_results.is_empty() {
                    let finite_values: Vec<f64> =
                        values.iter().filter(|&&v| v.is_finite()).copied().collect();

                    if finite_values.is_empty() {
                        debug!(
                            "No finite values for {} on {}, skipping",
                            optimizer, problem
                        );
                        continue;
                    }

                    let mean_final = finite_values.iter().sum::<f64>() / finite_values.len() as f64;
                    let std_final = {
                        let variance = finite_values
                            .iter()
                            .map(|x| (x - mean_final).powi(2))
                            .sum::<f64>()
                            / finite_values.len() as f64;
                        variance.sqrt()
                    };
                    let mean_iterations = optimizer_results
                        .iter()
                        .map(|r| r.iterations as f64)
                        .sum::<f64>()
                        / optimizer_results.len() as f64;
                    let mean_function_evals = optimizer_results
                        .iter()
                        .map(|r| r.function_evaluations as f64)
                        .sum::<f64>()
                        / optimizer_results.len() as f64;
                    let mean_gradient_evals = optimizer_results
                        .iter()
                        .map(|r| r.gradient_evaluations as f64)
                        .sum::<f64>()
                        / optimizer_results.len() as f64;
                    let success_count = optimizer_results
                        .iter()
                        .filter(|r| r.convergence_achieved)
                        .count();
                    let success_rate = success_count as f64 / optimizer_results.len() as f64;
                    csv_content.push_str(&format!(
                        "{problem},{optimizer},{mean_final:.6e},{std_final:.6e},{mean_iterations:.1},{mean_function_evals:.1},{mean_gradient_evals:.1},{success_rate:.3}\n"
                    ));
                    total_rows += 1;
                    debug!(
                        "Added performance row for {} on {}: mean={:.6e}, success_rate={:.3}",
                        optimizer, problem, mean_final, success_rate
                    );
                }
            }
        }
        fs::write(&csv_path, csv_content)
            .map_err(|e| anyhow::anyhow!("Failed to write CSV file {}: {}", csv_path, e))?;
        info!(
            "Performance comparison data exported to: {} ({} total rows)",
            csv_path, total_rows
        );
        Ok(())
    }
    /// Export performance boxplot data to CSV
    fn export_performance_boxplot_csv(
        &self,
        results: &BenchmarkResults,
        filename: &str,
    ) -> Result<()> {
        debug!(
            "Exporting performance boxplot data for {} results",
            results.results.len()
        );
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        let csv_path = format!("{}/{}_data.csv", self.output_dir, filename);
        info!("Writing performance boxplot CSV to: {}", csv_path);
        let mut csv_content = String::from("Optimizer,Min,Q1,Median,Q3,Max,AllValues\n");
        // Group results by optimizer across all problems
        let mut optimizer_results: HashMap<String, Vec<f64>> = HashMap::new();
        for result in &results.results {
            optimizer_results
                .entry(result.optimizer_name.clone())
                .or_default()
                .push(result.final_value);
        }
        let mut total_rows = 0;
        for (optimizer, mut values) in optimizer_results {
            debug!(
                "Processing boxplot data for optimizer: {} with {} values",
                optimizer,
                values.len()
            );
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let n = values.len();
            if n > 0 {
                let q1 = values[n / 4];
                let median = values[n / 2];
                let q3 = values[3 * n / 4];
                let min = values[0];
                let max = values[n - 1];
                // Convert all values to a comma-separated string
                let all_values_str = values
                    .iter()
                    .map(|v| format!("{v:.6e}"))
                    .collect::<Vec<_>>()
                    .join(";"); // Use semicolon to avoid CSV parsing issues
                csv_content.push_str(&format!(
                    "{optimizer},{min:.6e},{q1:.6e},{median:.6e},{q3:.6e},{max:.6e},\"{all_values_str}\"\n"
                ));
                total_rows += 1;
                debug!(
                    "Added boxplot row for {}: median={:.6e}, range=[{:.6e}, {:.6e}]",
                    optimizer, median, min, max
                );
            }
        }
        fs::write(&csv_path, csv_content)
            .map_err(|e| anyhow::anyhow!("Failed to write CSV file {}: {}", csv_path, e))?;
        info!(
            "Performance boxplot data exported to: {} ({} total rows)",
            csv_path, total_rows
        );
        Ok(())
    }
    /// Helper function to add legend for unique optimizers
    fn add_legend_for_optimizers<DB: DrawingBackend>(
        &self,
        root: &DrawingArea<DB, Shift>,
        optimizer_names: &[String],
        colors: &[&RGBColor],
    ) -> Result<()> {
        if !self.has_fonts {
            debug!("Skipping legend creation - no font support");
            return Ok(());
        }
        info!("Creating legend for {} optimizers", optimizer_names.len());
        let legend_x = self.width as i32 - 200;
        let legend_y = 50;
        let line_height = 20;
        debug!(
            "Legend position: ({}, {}), line height: {}",
            legend_x, legend_y, line_height
        );
        // Draw legend background
        let _ = root.draw(&Rectangle::new(
            [
                (legend_x - 10, legend_y - 10),
                (
                    legend_x + 180,
                    legend_y + optimizer_names.len() as i32 * line_height + 10,
                ),
            ],
            WHITE.mix(0.8).filled(),
        ));
        // Draw legend border
        let _ = root.draw(&Rectangle::new(
            [
                (legend_x - 10, legend_y - 10),
                (
                    legend_x + 180,
                    legend_y + optimizer_names.len() as i32 * line_height + 10,
                ),
            ],
            BLACK,
        ));
        // Draw legend entries
        for (i, optimizer_name) in optimizer_names.iter().enumerate() {
            let color = colors[i % colors.len()];
            let y_pos = legend_y + i as i32 * line_height;
            debug!(
                "Drawing legend entry {}: {} at position ({}, {})",
                i, optimizer_name, legend_x, y_pos
            );
            // Draw color line
            let _ = root.draw(&PathElement::new(
                vec![(legend_x, y_pos), (legend_x + 30, y_pos)],
                color.stroke_width(2),
            ));
            // Try to draw text label (ignore errors)
            let _ = root.draw(&Text::new(
                optimizer_name.as_str(),
                (legend_x + 40, y_pos),
                ("sans-serif", 15)
                    .into_font()
                    .color(&BLACK)
                    .pos(Pos::new(HPos::Left, VPos::Center)),
            ));
        }
        info!("Legend creation completed");
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct BoxPlotData {
    min: f64,
    q1: f64,
    median: f64,
    q3: f64,
    max: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_plotting_engine_creation() {
        let temp_dir = tempdir().unwrap();
        let engine = PlottingEngine::new(temp_dir.path().to_string_lossy().to_string());
        assert_eq!(engine.width, 1024);
        assert_eq!(engine.height, 768);
    }

    #[test]
    fn test_plotting_engine_dimensions() {
        let temp_dir = tempdir().unwrap();
        let engine = PlottingEngine::new(temp_dir.path().to_string_lossy().to_string())
            .with_dimensions(800, 600);
        assert_eq!(engine.width, 800);
        assert_eq!(engine.height, 600);
    }

    #[test]
    fn test_box_plot_data() {
        let data = BoxPlotData {
            min: 0.0,
            q1: 1.0,
            median: 2.0,
            q3: 3.0,
            max: 4.0,
        };

        assert_eq!(data.median, 2.0);
        assert!(data.q1 < data.median);
        assert!(data.median < data.q3);
    }
}
