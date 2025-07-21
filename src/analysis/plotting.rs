use crate::benchmarks::evaluation::{BenchmarkResults, OptimizationTrace};
use anyhow::Result;
use plotters::backend::BitMapBackend;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use std::collections::HashMap;
/// Check if font rendering is available
fn has_font_support() -> bool {
    // Check if we're in a CI/test environment where fonts might not be available
    if std::env::var("CI").is_ok() || std::env::var("TEST_ENV").is_ok() {
        return false;
    }

    // Try to create a simple font and actually draw text to see if it works
    let result = std::panic::catch_unwind(|| {
        use plotters::prelude::*;

        // Create a small in-memory bitmap to test font rendering
        let mut buffer = vec![0u8; 100 * 100 * 3];
        let root = BitMapBackend::with_buffer(&mut buffer, (100, 100)).into_drawing_area();

        // Try to actually draw text
        root.draw(&Text::new(
            "test",
            (50, 50),
            ("sans-serif", 12).into_font(),
        ))?;
        
        Ok::<(), Box<dyn std::error::Error>>(())
    });

    let ok = result.is_ok() && result.unwrap().is_ok();
    println!("[Plotting] Font support available: {}", ok);
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
    pub iterations: Vec<usize>,
}
impl From<&OptimizationTrace> for ExtendedOptimizationTrace {
    fn from(trace: &OptimizationTrace) -> Self {
        Self {
            optimizer_name: "Unknown".to_string(),
            objective_values: trace
                .iterations
                .iter()
                .map(|iter| iter.function_value)
                .collect(),
            iterations: trace.iterations.iter().map(|iter| iter.iteration).collect(),
        }
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
        let has_fonts = has_font_support();
        if !has_fonts {
            eprintln!("Warning: Font support not available. Plots will be generated without text labels.");
        }
        Self {
            output_dir,
            width: 1024,
            height: 768,
            config: PlotConfig::default(),
            has_fonts,
        }
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self
    }
    pub fn with_config(mut self, config: PlotConfig) -> Self {
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
        if traces.is_empty() {
            return Ok(());
        }

        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();

        root.fill(&WHITE)?;

        // Find the range of iterations and objective values
        let max_iterations = traces
            .iter()
            .map(|t| t.objective_values.len())
            .max()
            .unwrap_or(0);

        let (min_obj, max_obj) = traces
            .iter()
            .flat_map(|t| t.objective_values.iter())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });
        // Add padding to the objective value range
        let obj_range = max_obj - min_obj;
        let padded_min = min_obj - obj_range * 0.1;
        let padded_max = max_obj + obj_range * 0.1;


        // Build chart without text elements to avoid font issues
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(if self.has_fonts { 30 } else { 0 })
            .y_label_area_size(if self.has_fonts { 40 } else { 0 })
            .build_cartesian_2d(0..max_iterations, padded_min..padded_max)
            .map_err(|e| anyhow::anyhow!("Chart building error: {}", e))?;


        // Configure mesh based on font availability
        if self.has_fonts {
            chart.configure_mesh()
                .x_desc("Iterations")
                .y_desc("Objective Value")
                .draw()?;
        } else {
            chart.configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()?;
        }

        // Color palette for different optimizers
        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

        for (i, trace) in traces.iter().enumerate() {
            let color = colors[i % colors.len()];
            let series_data: Vec<(usize, f64)> = trace
                .objective_values
                .iter()
                .enumerate()
                .map(|(iter, &val)| (iter, val))
                .collect();

            // Draw series
            chart
                .draw_series(LineSeries::new(series_data.clone(), color))
                .map_err(|e| anyhow::anyhow!("Series drawing error: {}", e))?;
            // Add markers at regular intervals for better visibility
            let marker_interval = series_data.len().max(1) / 20 + 1;
            chart
                .draw_series(
                    series_data
                        .iter()
                        .step_by(marker_interval)
                        .map(|&(x, y)| Circle::new((x, y), 3, color.filled())),
                )
                .map_err(|e| anyhow::anyhow!("Marker drawing error: {}", e))?;
        }
        // Try to add legend
        if self.config.enable_legends && traces.len() > 1 && self.has_fonts {
            self.add_legend(&root, traces, &colors)?;
        }


        root.present()?;
        println!("Convergence plot saved to: {}", output_path);
        Ok(())
    }

    /// Create log-scale convergence plots for better visualization of convergence
    pub fn log_convergence_plot(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        if traces.is_empty() {
            return Ok(());
        }

        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_iterations = traces
            .iter()
            .map(|t| t.objective_values.len())
            .max()
            .unwrap_or(0);

        // Find minimum positive objective value for log scale
        let min_positive_obj = traces
            .iter()
            .flat_map(|t| t.objective_values.iter())
            .filter(|&&val| val > 0.0)
            .fold(f64::INFINITY, |min, &val| min.min(val))
            .max(1e-15); // Ensure minimum bound to prevent extreme log values

        let max_obj = traces
            .iter()
            .flat_map(|t| t.objective_values.iter())
            .fold(f64::NEG_INFINITY, |max, &val| max.max(val))
            .min(1e10); // Cap maximum to prevent overflow

        // Safely calculate log bounds with overflow protection
        let safe_min = min_positive_obj.max(1e-12).min(1e10);
        let safe_max = max_obj.max(1.0).min(1e10);

        let log_min = safe_min.log10().max(-15.0).min(15.0);
        let log_max = safe_max.log10().max(-15.0).min(15.0);

        // Ensure we have a valid range
        let (final_log_min, final_log_max) = if (log_max - log_min).abs() < 1e-10 {
            (log_min - 1.0, log_min + 1.0)
        } else {
            (log_min, log_max)
        };

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(if self.has_fonts { 30 } else { 0 })
            .y_label_area_size(if self.has_fonts { 50 } else { 0 })
            .build_cartesian_2d(0..max_iterations, final_log_min..final_log_max)?;


        // Configure mesh based on font availability
        if self.has_fonts {
            chart.configure_mesh()
                .x_desc("Iterations")
                .y_desc("Log10(Objective Value)")
                .draw()?;
        } else {
            chart.configure_mesh()
                .draw()?;
        }

        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

        for (i, trace) in traces.iter().enumerate() {
            let color = colors[i % colors.len()];
            let series_data: Vec<(usize, f64)> = trace
                .objective_values
                .iter()
                .enumerate()
                .map(|(iter, &val)| {
                    let safe_val = val.max(1e-15).min(1e10);
                    let log_val = safe_val.log10().max(-15.0).min(15.0);
                    (iter, log_val)
                })
                .collect();

            // Only draw if we have valid data points
            if !series_data.is_empty() {
                chart.draw_series(LineSeries::new(series_data.clone(), color))?;

                // Add markers for better visibility
                let marker_interval = series_data.len().max(1) / 20 + 1;
                chart.draw_series(
                    series_data
                        .iter()
                        .step_by(marker_interval)
                        .map(|&(x, y)| Circle::new((x, y), 3, color.filled())),
                )?;
            }
        }
        // Try to add legend
        if self.config.enable_legends && traces.len() > 1 && self.has_fonts {
            self.add_legend(&root, traces, &colors)?;
        }


        root.present()?;
        println!("Log convergence plot saved to: {}", output_path);
        Ok(())
    }

    /// Create performance comparison bar charts
    pub fn performance_comparison(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        if results.results.is_empty() {
            return Ok(());
        }

        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;

        // Group results by problem and optimizer
        let mut problem_results: HashMap<String, HashMap<String, Vec<f64>>> = HashMap::new();

        for result in &results.results {
            problem_results
                .entry(result.problem_name.clone())
                .or_insert_with(HashMap::new)
                .entry(result.optimizer_name.clone())
                .or_insert_with(Vec::new)
                .push(result.final_value);
        }

        // Calculate mean final values for each optimizer on each problem
        let mut chart_data: Vec<(String, Vec<(String, f64)>)> = Vec::new();

        for (problem, optimizers) in problem_results {
            let mut optimizer_means = Vec::new();
            for (optimizer, values) in optimizers {
                let mean = values.iter().sum::<f64>() / values.len() as f64;
                optimizer_means.push((optimizer, mean));
            }
            optimizer_means.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by optimizer name
            chart_data.push((problem, optimizer_means));
        }
        if chart_data.is_empty() {
            return Ok(());
        }

        // Create subplot for each problem
        let num_problems = chart_data.len();
        let subplot_height = self.height / num_problems as u32;
        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

        for (i, (problem_name, optimizer_data)) in chart_data.iter().enumerate() {
            let y_start = i as u32 * subplot_height;
            let y_end = (i + 1) as u32 * subplot_height;

            let subplot = root.margin(10, 10, y_start as i32, (self.height - y_end) as i32);

            let max_value = optimizer_data
                .iter()
                .map(|(_, val)| *val)
                .fold(f64::NEG_INFINITY, f64::max);

            let min_value = optimizer_data
                .iter()
                .map(|(_, val)| *val)
                .fold(f64::INFINITY, f64::min);
            let value_range = max_value - min_value;
            let padded_min = min_value - value_range * 0.1;
            let padded_max = max_value + value_range * 0.1;


            let mut chart = ChartBuilder::on(&subplot)
                .margin(10)
                .x_label_area_size(if self.has_fonts { 30 } else { 0 })
                .y_label_area_size(if self.has_fonts { 50 } else { 0 })
                .build_cartesian_2d(
                    0.0..(optimizer_data.len() as f64),
                    padded_min..padded_max,
                )?;


            // Configure mesh based on font availability
            if self.has_fonts {
                chart.configure_mesh()
                    .y_desc("Objective Value")
                    .draw()?;
            } else {
                chart.configure_mesh()
                    .disable_x_mesh()
                    .disable_y_mesh()
                    .draw()?;
            }

            chart.draw_series(optimizer_data.iter().enumerate().map(|(x, (name, value))| {
                let color = colors[x % colors.len()];
                Rectangle::new(
                    [(x as f64, padded_min), (x as f64 + 0.8, *value)],
                    color.filled(),
                )
            }))?;

            // Add problem name as title if fonts are available
            if self.has_fonts {
                let _ = subplot.draw(&Text::new(
                    problem_name.as_str(),
                    (subplot_height as i32 / 2, 10),
                    ("sans-serif", 20).into_font().color(&BLACK),
                ));
            }
        }

        root.present()?;
        println!("Performance comparison saved to: {}", output_path);
        Ok(())
    }

    /// Create box plots showing distribution of results
    pub fn performance_boxplot(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        if results.results.is_empty() {
            return Ok(());
        }

        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;

        // Group results by optimizer across all problems
        let mut optimizer_results: HashMap<String, Vec<f64>> = HashMap::new();

        for result in &results.results {
            optimizer_results
                .entry(result.optimizer_name.clone())
                .or_insert_with(Vec::new)
                .push(result.final_value);
        }

        // Calculate statistics for each optimizer
        let mut box_data: Vec<(String, BoxPlotData)> = Vec::new();

        for (optimizer, mut values) in optimizer_results {
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let n = values.len();

            if n > 0 {
                let q1 = values[n / 4];
                let median = values[n / 2];
                let q3 = values[3 * n / 4];
                let min = values[0];
                let max = values[n - 1];

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
            return Ok(());
        }

        let global_min = box_data
            .iter()
            .map(|(_, data)| data.min)
            .fold(f64::INFINITY, f64::min);
        let global_max = box_data
            .iter()
            .map(|(_, data)| data.max)
            .fold(f64::NEG_INFINITY, f64::max);
        let value_range = global_max - global_min;
        let padded_min = global_min - value_range * 0.1;
        let padded_max = global_max + value_range * 0.1;


        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(if self.has_fonts { 60 } else { 0 })
            .y_label_area_size(if self.has_fonts { 50 } else { 0 })
            .build_cartesian_2d(
                0.0..(box_data.len() as f64),
                padded_min..padded_max,
            )?;


        // Configure mesh based on font availability
        if self.has_fonts {
            chart.configure_mesh()
                .y_desc("Objective Value Distribution")
                .draw()?;
        } else {
            chart.configure_mesh()
                .draw()?;
        }

        // Draw box plots
        for (i, (name, data)) in box_data.iter().enumerate() {
            let x = i as f64;
            let box_width = 0.3;

            // Draw box (Q1 to Q3)
            chart.draw_series(std::iter::once(Rectangle::new(
                [
                    (x as f64 - box_width / 2.0, data.q1),
                    (x as f64 + box_width / 2.0, data.q3),
                ],
                BLUE.mix(0.3).filled(),
            )))?;

            // Draw median line
            chart.draw_series(std::iter::once(PathElement::new(
                vec![
                    (x as f64 - box_width / 2.0, data.median),
                    (x as f64 + box_width / 2.0, data.median),
                ],
                RED.stroke_width(2),
            )))?;

            // Draw whiskers
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x as f64, data.min), (x as f64, data.q1)],
                &BLACK,
            )))?;

            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x as f64, data.q3), (x as f64, data.max)],
                &BLACK,
            )))?;
            // Draw whisker caps
            let cap_width = box_width / 3.0;
            chart.draw_series(std::iter::once(PathElement::new(
                vec![
                    (x - cap_width / 2.0, data.min),
                    (x + cap_width / 2.0, data.min),
                ],
                &BLACK,
            )))?;
            chart.draw_series(std::iter::once(PathElement::new(
                vec![
                    (x - cap_width / 2.0, data.max),
                    (x + cap_width / 2.0, data.max),
                ],
                &BLACK,
            )))?;

            // Add optimizer names as x-axis labels if fonts are available
            if self.has_fonts {
                let _ = root.draw(&Text::new(
                    name.as_str(),
                    (((i as f64 + 0.5) * self.width as f64 / box_data.len() as f64) as i32, (self.height - 20) as i32),
                    ("sans-serif", 15).into_font().color(&BLACK).pos(Pos::new(HPos::Center, VPos::Top)),
                ));
            }
        }

        root.present()?;
        println!("Performance boxplot saved to: {}", output_path);
        Ok(())
    }
    /// Helper function to add legend to plots
    fn add_legend<DB: DrawingBackend>(
        &self,
        root: &DrawingArea<DB, Shift>,
        traces: &[ExtendedOptimizationTrace],
        colors: &[&RGBColor],
    ) -> Result<()> {
        if !self.has_fonts {
            return Ok(());
        }

        let legend_x = self.width as i32 - 200;
        let legend_y = 50;
        let line_height = 20;

        // Draw legend background
        let _ = root.draw(&Rectangle::new(
            [(legend_x - 10, legend_y - 10), (legend_x + 180, legend_y + traces.len() as i32 * line_height + 10)],
            WHITE.mix(0.8).filled(),
        ));

        // Draw legend border
        let _ = root.draw(&Rectangle::new(
            [(legend_x - 10, legend_y - 10), (legend_x + 180, legend_y + traces.len() as i32 * line_height + 10)],
            BLACK,
        ));

        // Draw legend entries
        for (i, trace) in traces.iter().enumerate() {
            let color = colors[i % colors.len()];
            let y_pos = legend_y + i as i32 * line_height;
            // Draw color line
            let _ = root.draw(&PathElement::new(
                vec![(legend_x, y_pos), (legend_x + 30, y_pos)],
                color.stroke_width(2),
            ));
            // Try to draw text label (ignore errors)
            let _ = root.draw(&Text::new(
                trace.optimizer_name.as_str(),
                (legend_x + 40, y_pos),
                ("sans-serif", 15).into_font().color(&BLACK).pos(Pos::new(HPos::Left, VPos::Center)),
            ));
        }

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