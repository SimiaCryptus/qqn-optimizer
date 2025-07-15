use crate::benchmarks::evaluation::{BenchmarkResults, OptimizationTrace};
use anyhow::Result;
use plotters::backend::BitMapBackend;
use plotters::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlotConfig {
    pub width: u32,
    pub height: u32,
    pub output_format: String,
    pub color_scheme: String,
}
impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            output_format: "png".to_string(),
            color_scheme: "default".to_string(),
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
}

impl PlottingEngine {
    pub fn new(output_dir: String) -> Self {
        Self {
            output_dir,
            width: 1024,
            height: 768,
        }
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Create convergence plots showing objective value vs iterations
    pub fn convergence_plot(
        &self,
        traces: &[ExtendedOptimizationTrace],
        filename: &str,
    ) -> Result<()> {
        // Check if we can create a simple plot first to detect font issues early
        if traces.is_empty() {
            return Ok(());
        }

        let output_path = format!("{}/{}.png", self.output_dir, filename);

        // Try to create the backend and handle font errors gracefully
        let root = match BitMapBackend::new(&output_path, (self.width, self.height)).into_drawing_area() {
            area => area,
        };

        // Try to fill with white, which might trigger font issues
        if let Err(e) = root.fill(&WHITE) {
            if e.to_string().contains("font") || e.to_string().contains("text") {
                return Err(anyhow::anyhow!("Font rendering not available: {}", e));
            }
            return Err(anyhow::anyhow!("Drawing error: {}", e));
        }

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

        // Build chart without text elements to avoid font issues
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(0)  // No labels to avoid font issues
            .y_label_area_size(0)  // No labels to avoid font issues
            .build_cartesian_2d(0..max_iterations, min_obj..max_obj)
            .map_err(|e| anyhow::anyhow!("Chart building error: {}", e))?;

        // Configure mesh without text to avoid font issues
        chart.configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()
            .map_err(|e| anyhow::anyhow!("Mesh drawing error: {}", e))?;

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
            chart.draw_series(LineSeries::new(series_data, color))
                .map_err(|e| anyhow::anyhow!("Series drawing error: {}", e))?;
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
            .fold(f64::INFINITY, |min, &val| min.min(val));

        let max_obj = traces
            .iter()
            .flat_map(|t| t.objective_values.iter())
            .fold(f64::NEG_INFINITY, |max, &val| max.max(val));

        let log_min = (min_positive_obj.max(1e-12)).log10();
        let log_max = max_obj.max(1.0).log10();

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(0..max_iterations, log_min..log_max)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];

        for (i, trace) in traces.iter().enumerate() {
            let color = colors[i % colors.len()];
            let series_data: Vec<(usize, f64)> = trace
                .objective_values
                .iter()
                .enumerate()
                .map(|(iter, &val)| (iter, val.max(1e-12).log10()))
                .collect();

            chart.draw_series(LineSeries::new(series_data, color))?;
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

        for (i, (_problem_name, optimizer_data)) in chart_data.iter().enumerate() {
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

            let mut chart = ChartBuilder::on(&subplot)
                .margin(5)
                .x_label_area_size(0)
                .y_label_area_size(0)
                .build_cartesian_2d(
                    0.0..(optimizer_data.len() as f64),
                    min_value * 0.9..max_value * 1.1,
                )?;

            chart
                .configure_mesh()
                .disable_x_mesh()
                .disable_y_mesh()
                .draw()?;

            chart.draw_series(optimizer_data.iter().enumerate().map(|(x, (_, value))| {
                Rectangle::new(
                    [(x as f64, min_value * 0.9), (x as f64 + 0.8, *value)],
                    BLUE.filled(),
                )
            }))?;
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

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(
                0.0..(box_data.len() as f64),
                global_min * 0.9..global_max * 1.1,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        // Draw box plots
        for (i, (_, data)) in box_data.iter().enumerate() {
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
                &RED,
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
        }

        root.present()?;
        println!("Performance boxplot saved to: {}", output_path);
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