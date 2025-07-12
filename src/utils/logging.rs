//! Logging and performance monitoring utilities
use std::time::{Duration, Instant};
use tracing::{info, debug, warn};
use serde::{Serialize, Deserialize};
/// Setup tracing for the optimization framework
pub fn setup_tracing() -> anyhow::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "qqn_optimizer=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize logging: {}", e))?;
    Ok(())
}
/// Log an optimization step
pub fn log_optimization_step(
    iteration: usize,
    function_value: f64,
    gradient_norm: f64,
    step_size: f64,
) {
    debug!(
        "Iteration {}: f = {:.6e}, ||∇f|| = {:.6e}, α = {:.6e}",
        iteration, function_value, gradient_norm, step_size
    );
}
/// Log convergence information
pub fn log_convergence_info(converged: bool, reason: &str, iterations: usize) {
    if converged {
        info!("Converged after {} iterations: {}", iterations, reason);
    } else {
        warn!("Failed to converge after {} iterations: {}", iterations, reason);
    }
}
/// Performance timer for measuring execution time
#[derive(Debug, Clone)]
pub struct PerformanceTimer {
    start_time: Instant,
    checkpoints: Vec<(String, Instant)>,
}
impl PerformanceTimer {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            checkpoints: Vec::new(),
        }
    }
    pub fn checkpoint(&mut self, name: &str) {
        self.checkpoints.push((name.to_string(), Instant::now()));
    }
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    pub fn checkpoint_duration(&self, name: &str) -> Option<Duration> {
        self.checkpoints
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, time)| time.duration_since(self.start_time))
    }
}
impl Default for PerformanceTimer {
    fn default() -> Self {
        Self::new()
    }
}
/// Optimization logger for tracking progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationLogger {
    pub entries: Vec<LogEntry>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub iteration: usize,
    pub timestamp: String,
    pub function_value: f64,
    pub gradient_norm: f64,
    pub step_size: f64,
    pub optimizer_state: String,
}
impl OptimizationLogger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn log_step(
        &mut self,
        iteration: usize,
        function_value: f64,
        gradient_norm: f64,
        step_size: f64,
        optimizer_state: &str,
    ) {
        let entry = LogEntry {
            iteration,
            timestamp: chrono::Utc::now().to_rfc3339(),
            function_value,
            gradient_norm,
            step_size,
            optimizer_state: optimizer_state.to_string(),
        };
        self.entries.push(entry);
    }
    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
impl Default for OptimizationLogger {
    fn default() -> Self {
        Self::new()
    }
}