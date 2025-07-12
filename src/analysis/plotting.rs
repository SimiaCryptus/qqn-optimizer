use crate::analysis::statistics::PerformanceProfiles;
use crate::benchmarks::evaluation::{BenchmarkResults, OptimizationTrace};
use crate::core::qqn::QQNTrace;
use anyhow::{Context, Result};
use plotters::backend::{BitMapBackend, SVGBackend};
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
pub trait ConvergencePlot {
    fn plot_convergence(&self, traces: &[ExtendedOptimizationTrace], filename: &str) -> Result<()>;
}
pub trait PerformancePlot {
    fn plot_performance(&self, results: &BenchmarkResults, filename: &str) -> Result<()>;
}
pub trait MagnitudeRatioPlot {
    fn plot_magnitude_ratios(&self, traces: &[QQNTrace], filename: &str) -> Result<()>;
}
pub trait StatisticalPlot {
    fn plot_statistics(&self, analysis: &crate::analysis::statistics::StatisticalAnalysis, filename: &str) -> Result<()>;
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
            objective_values: trace.iterations.iter().map(|iter| iter.function_value).collect(),
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
    pub fn convergence_plot(&self, traces: &[ExtendedOptimizationTrace], filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height))
            .into_drawing_area();
        root.fill(&WHITE)?;

        // Find the range of iterations and objective values
        let max_iterations = traces.iter()
            .map(|t| t.objective_values.len())
            .max()
            .unwrap_or(0);
        
        let (min_obj, max_obj) = traces.iter()
            .flat_map(|t| t.objective_values.iter())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });

        let mut chart = ChartBuilder::on(&root)
            .caption("Convergence Comparison", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(70)
            .build_cartesian_2d(0..max_iterations, min_obj..max_obj)?;

        chart.configure_mesh()
            .x_desc("Iterations")
            .y_desc("Objective Value")
            .draw()?;

        // Color palette for different optimizers
        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];
        
        for (i, trace) in traces.iter().enumerate() {
            let color = colors[i % colors.len()];
            let series_data: Vec<(usize, f64)> = trace.objective_values.iter()
                .enumerate()
                .map(|(iter, &val)| (iter, val))
                .collect();

            chart.draw_series(LineSeries::new(series_data, color))?
                .label(&trace.optimizer_name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], color));
        }

        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        println!("Convergence plot saved to: {}", output_path);
        Ok(())
    }

    /// Create log-scale convergence plots for better visualization of convergence
    pub fn log_convergence_plot(&self, traces: &[ExtendedOptimizationTrace], filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height))
            .into_drawing_area();
        root.fill(&WHITE)?;

        let max_iterations = traces.iter()
            .map(|t| t.objective_values.len())
            .max()
            .unwrap_or(0);

        // Find minimum positive objective value for log scale
        let min_positive_obj = traces.iter()
            .flat_map(|t| t.objective_values.iter())
            .filter(|&&val| val > 0.0)
            .fold(f64::INFINITY, |min, &val| min.min(val));

        let max_obj = traces.iter()
            .flat_map(|t| t.objective_values.iter())
            .fold(f64::NEG_INFINITY, |max, &val| max.max(val));

        let log_min = (min_positive_obj.max(1e-12)).log10();
        let log_max = max_obj.max(1.0).log10();

        let mut chart = ChartBuilder::on(&root)
            .caption("Convergence Comparison (Log Scale)", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(70)
            .build_cartesian_2d(0..max_iterations, log_min..log_max)?;

        chart.configure_mesh()
            .x_desc("Iterations")
            .y_desc("Log10(Objective Value)")
            .draw()?;

        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];
        
        for (i, trace) in traces.iter().enumerate() {
            let color = colors[i % colors.len()];
            let series_data: Vec<(usize, f64)> = trace.objective_values.iter()
                .enumerate()
                .map(|(iter, &val)| (iter, val.max(1e-12).log10()))
                .collect();

            chart.draw_series(LineSeries::new(series_data, color))?
                .label(&trace.optimizer_name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], color));
        }

        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        println!("Log convergence plot saved to: {}", output_path);
        Ok(())
    }

    /// Create performance comparison bar charts
    pub fn performance_comparison(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height))
            .into_drawing_area();
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

        // Create subplot for each problem
        let num_problems = chart_data.len();
        let subplot_height = self.height / num_problems as u32;

        for (i, (problem_name, optimizer_data)) in chart_data.iter().enumerate() {
            let y_start = i as u32 * subplot_height;
            let y_end = (i + 1) as u32 * subplot_height;
            
            let subplot = root.margin(10, 10, y_start as i32, (self.height - y_end) as i32);
            
            let max_value = optimizer_data.iter()
                .map(|(_, val)| *val)
                .fold(f64::NEG_INFINITY, f64::max);
            
            let min_value = optimizer_data.iter()
                .map(|(_, val)| *val)
                .fold(f64::INFINITY, f64::min);

            let mut chart = ChartBuilder::on(&subplot)
                .caption(&format!("Performance on {}", problem_name), ("sans-serif", 20))
                .x_label_area_size(40)
                .y_label_area_size(60)
                .build_cartesian_2d(
                    0..optimizer_data.len(),
                    min_value * 0.9..max_value * 1.1
                )?;

            chart.configure_mesh()
                .x_desc("Optimizer")
                .y_desc("Final Objective Value")
                .x_label_formatter(&|x| {
                    optimizer_data.get(*x)
                        .map(|(name, _)| name.clone())
                        .unwrap_or_default()
                })
                .draw()?;

            chart.draw_series(
                optimizer_data.iter().enumerate().map(|(x, (_, value))| {
                    Rectangle::new([(x, min_value * 0.9), (x, *value)], BLUE.filled())
                })
            )?;
        }

        root.present()?;
        println!("Performance comparison saved to: {}", output_path);
        Ok(())
    }

    /// Create box plots showing distribution of results
    pub fn performance_boxplot(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height))
            .into_drawing_area();
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
                
                box_data.push((optimizer, BoxPlotData {
                    min, q1, median, q3, max
                }));
            }
        }

        box_data.sort_by(|a, b| a.0.cmp(&b.0));

        let global_min = box_data.iter().map(|(_, data)| data.min).fold(f64::INFINITY, f64::min);
        let global_max = box_data.iter().map(|(_, data)| data.max).fold(f64::NEG_INFINITY, f64::max);

        let mut chart = ChartBuilder::on(&root)
            .caption("Performance Distribution", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(60)
            .y_label_area_size(70)
            .build_cartesian_2d(
                0..box_data.len(),
                global_min * 0.9..global_max * 1.1
            )?;

        chart.configure_mesh()
            .x_desc("Optimizer")
            .y_desc("Final Objective Value")
            .x_label_formatter(&|x| {
                box_data.get(*x)
                    .map(|(name, _)| name.clone())
                    .unwrap_or_default()
            })
            .draw()?;

        // Draw box plots
        for (i, (_, data)) in box_data.iter().enumerate() {
            let x = i;
            let box_width = 0.3;
            
            // Draw box (Q1 to Q3)
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x as f64 - box_width, data.q1), (x as f64 + box_width, data.q3)],
                BLUE.mix(0.3).filled()
            )))?;
            
            // Draw median line
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x as f64 - box_width, data.median), (x as f64 + box_width, data.median)],
                &RED
            )))?;
            
            // Draw whiskers
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x as f64, data.min), (x as f64, data.q1)],
                &BLACK
            )))?;
            
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x as f64, data.q3), (x as f64, data.max)],
                &BLACK
            )))?;
        }

        root.present()?;
        println!("Performance boxplot saved to: {}", output_path);
        Ok(())
    }

    /// Create performance profiles showing fraction of problems solved within tolerance
    pub fn performance_profiles(&self, profiles: &PerformanceProfiles, filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height))
            .into_drawing_area();
        root.fill(&WHITE)?;

        let max_ratio = profiles.ratios.iter().fold(0.0, |max, &r| max.max(r));

        let mut chart = ChartBuilder::on(&root)
            .caption("Performance Profiles", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(70)
            .build_cartesian_2d(1.0..max_ratio, 0.0..1.0)?;

        chart.configure_mesh()
            .x_desc("Performance Ratio")
            .y_desc("Fraction of Problems Solved")
            .draw()?;

        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];
        
        for (i, (optimizer, profile_data)) in profiles.optimizer_profiles.iter().enumerate() {
            let color = colors[i % colors.len()];
            
            chart.draw_series(LineSeries::new(
                profile_data.iter().enumerate().map(|(j, &fraction)| {
                    (profiles.ratios[j], fraction)
                }),
                color
            ))?
            .label(optimizer)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], color));
        }

        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        println!("Performance profiles saved to: {}", output_path);
        Ok(())
    }

    /// Create QQN-specific analysis plots showing magnitude ratios and switching behavior
    pub fn magnitude_ratio_analysis(&self, qqn_traces: &[QQNTrace], filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width, self.height))
            .into_drawing_area();
        root.fill(&WHITE)?;

        // Split into two subplots: histogram and time series
        let (upper, lower) = root.split_evenly(false);

        // Upper plot: Histogram of magnitude ratios
        {
            let all_ratios: Vec<f64> = qqn_traces.iter()
                .flat_map(|trace| trace.magnitude_ratios.iter())
                .cloned()
                .collect();

            if !all_ratios.is_empty() {
                let min_ratio = all_ratios.iter().fold(f64::INFINITY, |min, &r| min.min(r));
                let max_ratio = all_ratios.iter().fold(f64::NEG_INFINITY, |max, &r| max.max(r));
                
                let num_bins = 50;
                let bin_width = (max_ratio - min_ratio) / num_bins as f64;
                let mut histogram = vec![0; num_bins];
                
                for &ratio in &all_ratios {
                    let bin = ((ratio - min_ratio) / bin_width).floor() as usize;
                    let bin = bin.min(num_bins - 1);
                    histogram[bin] += 1;
                }

                let max_count = histogram.iter().max().unwrap_or(&0);

                let mut chart = ChartBuilder::on(&upper)
                    .caption("Distribution of Magnitude Ratios (ρ)", ("sans-serif", 30))
                    .margin(5)
                    .x_label_area_size(40)
                    .y_label_area_size(50)
                    .build_cartesian_2d(min_ratio..max_ratio, 0..*max_count)?;

                chart.configure_mesh()
                    .x_desc("Magnitude Ratio (ρ)")
                    .y_desc("Frequency")
                    .draw()?;

                chart.draw_series(
                    histogram.iter().enumerate().map(|(i, &count)| {
                        let x_start = min_ratio + i as f64 * bin_width;
                        let x_end = x_start + bin_width;
                        Rectangle::new([(x_start, 0), (x_end, count)], BLUE.mix(0.7).filled())
                    })
                )?;

                // Draw threshold line
                let threshold = 0.01; // Default QQN threshold
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![(threshold, 0.0), (threshold, *max_count as f64)],
                    &RED.stroke_width(2)
                )))?
                .label("Threshold (τ)")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

                chart.configure_series_labels().draw()?;
            }
        }

        // Lower plot: Time series of switching behavior
        {
            if let Some(trace) = qqn_traces.first() {
                let max_iterations = trace.magnitude_ratios.len();
                let threshold = 0.01;

                let mut chart = ChartBuilder::on(&lower)
                    .caption("QQN Switching Behavior Over Time", ("sans-serif", 30))
                    .margin(5)
                    .x_label_area_size(40)
                    .y_label_area_size(50)
                    .build_cartesian_2d(0..max_iterations, 0.0..1.0)?;

                chart.configure_mesh()
                    .x_desc("Iteration")
                    .y_desc("Mode (0=L-BFGS, 1=Quadratic)")
                    .draw()?;

                // Convert magnitude ratios to binary switching signal
                let switching_signal: Vec<(usize, f64)> = trace.magnitude_ratios.iter()
                    .enumerate()
                    .map(|(i, &ratio)| (i, if ratio > threshold { 1.0 } else { 0.0 }))
                    .collect();

                chart.draw_series(LineSeries::new(switching_signal, &GREEN.stroke_width(2)))?
                    .label("QQN Mode")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &GREEN));

                chart.configure_series_labels().draw()?;
            }
        }

        root.present()?;
        println!("Magnitude ratio analysis saved to: {}", output_path);
        Ok(())
    }

    /// Create comprehensive comparison dashboard
    pub fn create_dashboard(&self, results: &BenchmarkResults, filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.png", self.output_dir, filename);
        let root = BitMapBackend::new(&output_path, (self.width * 2, self.height * 2))
            .into_drawing_area();
        root.fill(&WHITE)?;

        // Split into 4 quadrants
        let (upper, lower) = root.split_evenly((false, true));
        let (upper_left, upper_right) = upper.split_evenly((true, false));
        let (lower_left, lower_right) = lower.split_evenly((true, false));

        // Upper left: Convergence comparison
        if let Some(trace) = results.results.first() {
            // This would need actual trace data - simplified for now
            let mut chart = ChartBuilder::on(&upper_left)
                .caption("Convergence", ("sans-serif", 20))
                .margin(5)
                .build_cartesian_2d(0..100, 0.0..1.0)?;
            chart.configure_mesh().draw()?;
        }

        // Upper right: Performance comparison
        {
            let mut chart = ChartBuilder::on(&upper_right)
                .caption("Final Performance", ("sans-serif", 20))
                .margin(5)
                .build_cartesian_2d(0..5, 0.0..1.0)?;
            chart.configure_mesh().draw()?;
        }

        // Lower left: Success rate
        {
            let mut chart = ChartBuilder::on(&lower_left)
                .caption("Success Rate", ("sans-serif", 20))
                .margin(5)
                .build_cartesian_2d(0..5, 0.0..1.0)?;
            chart.configure_mesh().draw()?;
        }

        // Lower right: Execution time
        {
            let mut chart = ChartBuilder::on(&lower_right)
                .caption("Execution Time", ("sans-serif", 20))
                .margin(5)
                .build_cartesian_2d(0..5, 0.0..1.0)?;
            chart.configure_mesh().draw()?;
        }

        root.present()?;
        println!("Dashboard saved to: {}", output_path);
        Ok(())
    }

    /// Export plots in different formats
    pub fn export_svg(&self, traces: &[ExtendedOptimizationTrace], filename: &str) -> Result<()> {
        let output_path = format!("{}/{}.svg", self.output_dir, filename);
        let root = SVGBackend::new(&output_path, (self.width, self.height))
            .into_drawing_area();
        
        // Reuse convergence plot logic with SVG backend
        self.draw_convergence_plot_on_backend(traces, root)?;
        
        println!("SVG plot saved to: {}", output_path);
        Ok(())
    }

    fn draw_convergence_plot_on_backend<DB: DrawingBackend>(
        &self,
        traces: &[ExtendedOptimizationTrace],
        root: DrawingArea<DB, plotters::coord::Shift>
    ) -> Result<()>
    where
        DB::ErrorType: 'static,
    {
        root.fill(&WHITE)?;

        let max_iterations = traces.iter()
            .map(|t| t.objective_values.len())
            .max()
            .unwrap_or(0);
        
        let (min_obj, max_obj) = traces.iter()
            .flat_map(|t| t.objective_values.iter())
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &val| {
                (min.min(val), max.max(val))
            });

        let mut chart = ChartBuilder::on(&root)
            .caption("Convergence Comparison", ("sans-serif", 40))
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(70)
            .build_cartesian_2d(0..max_iterations, min_obj..max_obj)?;

        chart.configure_mesh()
            .x_desc("Iterations")
            .y_desc("Objective Value")
            .draw()?;

        let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];
        
        for (i, trace) in traces.iter().enumerate() {
            let color = colors[i % colors.len()];
            let series_data: Vec<(usize, f64)> = trace.objective_values.iter()
                .enumerate()
                .map(|(iter, &val)| (iter, val))
                .collect();

            chart.draw_series(LineSeries::new(series_data, color))?
                .label(&trace.optimizer_name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], color));
        }

        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
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