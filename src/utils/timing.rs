
use std::time::{Duration, Instant};

/// Simple timer for measuring elapsed time
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    /// Create a new timer with a name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Get elapsed time in seconds
    pub fn elapsed_secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    /// Log the elapsed time
    pub fn log(&self) {
        tracing::info!("{}: {:.3}s", self.name, self.elapsed_secs());
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.log();
    }
}

/// Measure the execution time of a closure
pub fn measure<F, R>(name: &str, f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}
