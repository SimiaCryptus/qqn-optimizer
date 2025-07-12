use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// Note: bincode serialization can be added later if needed
// For now, we'll focus on serde-based serialization

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    pub name: String,
    pub description: String,
    pub problems: Vec<ProblemConfig>,
    pub optimizers: Vec<OptimizerConfig>,
    pub benchmark_settings: BenchmarkConfig,
    pub analysis_settings: AnalysisConfig,
    pub output_settings: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemConfig {
    pub name: String,
    pub problem_type: ProblemType,
    pub parameters: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProblemType {
    Rosenbrock { dimension: usize },
    Rastrigin { dimension: usize },
    Sphere { dimension: usize },
    Beale,
    LogisticRegression { dataset: String },
    NeuralNetwork { architecture: NetworkConfig },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub layers: Vec<usize>,
    pub activation: String,
    pub loss: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    pub name: String,
    pub optimizer_type: OptimizerType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OptimizerType {
    QQN {
        threshold: f64,
        lbfgs_history: usize,
        line_search: LineSearchConfig,
        epsilon: Option<f64>,
    },
    LBFGS {
        history: usize,
        line_search: LineSearchConfig,
    },
    Adam {
        learning_rate: f64,
        beta1: f64,
        beta2: f64,
        epsilon: Option<f64>,
    },
    SGD {
        learning_rate: f64,
        momentum: Option<f64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSearchConfig {
    pub method: LineSearchMethod,
    pub c1: f64,
    pub c2: f64,
    pub max_iterations: usize,
    pub initial_step: f64,
    pub min_step: f64,
    pub max_step: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineSearchMethod {
    StrongWolfe,
    Backtracking,
}

impl Default for LineSearchConfig {
    fn default() -> Self {
        Self {
            method: LineSearchMethod::StrongWolfe,
            c1: 1e-4,
            c2: 0.9,
            max_iterations: 20,
            initial_step: 1.0,
            min_step: 1e-16,
            max_step: 1e16,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub max_function_evaluations: usize,
    #[serde(with = "humantime_serde")]
    pub time_limit: Duration,
    pub random_seed: u64,
    pub num_runs: usize,
    pub parallel_runs: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            tolerance: 1e-6,
            max_function_evaluations: 10000,
            time_limit: Duration::from_secs(600), // 10 minutes
            random_seed: 42,
            num_runs: 10,
            parallel_runs: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub statistical_tests: Vec<StatisticalTest>,
    pub confidence_level: f64,
    pub performance_profiles: bool,
    pub convergence_analysis: bool,
    pub robustness_analysis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatisticalTest {
    TTest,
    MannWhitney,
    Wilcoxon,
    KruskalWallis,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            statistical_tests: vec![StatisticalTest::TTest, StatisticalTest::MannWhitney],
            confidence_level: 0.95,
            performance_profiles: true,
            convergence_analysis: true,
            robustness_analysis: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub results_dir: PathBuf,
    pub generate_plots: bool,
    pub export_csv: bool,
    pub export_json: bool,
    pub latex_tables: bool,
    pub plot_formats: Vec<PlotFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotFormat {
    PNG,
    PDF,
    SVG,
    HTML,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            results_dir: PathBuf::from("results"),
            generate_plots: true,
            export_csv: true,
            export_json: true,
            latex_tables: false,
            plot_formats: vec![PlotFormat::PNG, PlotFormat::PDF],
        }
    }
}

// Configuration loading and validation
impl ExperimentConfig {
    pub fn from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        let content =
            std::fs::read_to_string(path).map_err(|e| ConfigError::FileRead(path.clone(), e))?;

        let config: ExperimentConfig =
            serde_yaml::from_str(&content).map_err(|e| ConfigError::Parse(e))?;

        config.validate()?;
        Ok(config)
    }
    pub fn default_research() -> Self {
        Self {
            name: "QQN Research Experiment".to_string(),
            description: "Default research configuration for QQN optimizer evaluation".to_string(),
            problems: vec![
                create_rosenbrock_problem(2),
                create_rosenbrock_problem(10),
                create_rastrigin_problem(2),
                create_rastrigin_problem(10),
                ProblemConfig {
                    name: "sphere_2d".to_string(),
                    problem_type: ProblemType::Sphere { dimension: 2 },
                    parameters: None,
                },
                ProblemConfig {
                    name: "beale".to_string(),
                    problem_type: ProblemType::Beale,
                    parameters: None,
                },
            ],
            optimizers: vec![
                create_qqn_config(0.01, 10),
                create_qqn_config(0.05, 10),
                create_lbfgs_config(10),
                create_adam_config(0.001),
            ],
            benchmark_settings: BenchmarkConfig::default(),
            analysis_settings: AnalysisConfig::default(),
            output_settings: OutputConfig::default(),
        }
    }

    pub fn to_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let content = serde_yaml::to_string(self).map_err(|e| ConfigError::Serialize(e))?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ConfigError::FileWrite(path.clone(), e))?;
        }

        std::fs::write(path, content).map_err(|e| ConfigError::FileWrite(path.clone(), e))?;

        Ok(())
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate problem configurations
        for problem in &self.problems {
            problem.validate()?;
        }

        // Validate optimizer configurations
        for optimizer in &self.optimizers {
            optimizer.validate()?;
        }

        // Validate benchmark settings
        self.benchmark_settings.validate()?;

        // Validate analysis settings
        self.analysis_settings.validate()?;

        // Validate output settings
        self.output_settings.validate()?;

        Ok(())
    }
}

impl ProblemConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        match &self.problem_type {
            ProblemType::Rosenbrock { dimension } => {
                if *dimension < 2 {
                    return Err(ConfigError::InvalidProblem(format!(
                        "Rosenbrock dimension must be >= 2, got {}",
                        dimension
                    )));
                }
            }
            ProblemType::Rastrigin { dimension } => {
                if *dimension < 1 {
                    return Err(ConfigError::InvalidProblem(format!(
                        "Rastrigin dimension must be >= 1, got {}",
                        dimension
                    )));
                }
            }
            ProblemType::Sphere { dimension } => {
                if *dimension < 1 {
                    return Err(ConfigError::InvalidProblem(format!(
                        "Sphere dimension must be >= 1, got {}",
                        dimension
                    )));
                }
            }
            ProblemType::Beale => {
                // Beale function is always 2D, no validation needed
            }
            ProblemType::LogisticRegression { dataset } => {
                if dataset.is_empty() {
                    return Err(ConfigError::InvalidProblem(
                        "LogisticRegression dataset cannot be empty".to_string(),
                    ));
                }
            }
            ProblemType::NeuralNetwork { architecture } => {
                if architecture.layers.len() < 2 {
                    return Err(ConfigError::InvalidProblem(
                        "Neural network must have at least 2 layers".to_string(),
                    ));
                }
                if architecture.layers.iter().any(|&size| size == 0) {
                    return Err(ConfigError::InvalidProblem(
                        "Neural network layer sizes must be > 0".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl OptimizerConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        match &self.optimizer_type {
            OptimizerType::QQN {
                threshold,
                lbfgs_history,
                ..
            } => {
                if *threshold <= 0.0 || *threshold >= 1.0 {
                    return Err(ConfigError::InvalidOptimizer(format!(
                        "QQN threshold must be in (0, 1), got {}",
                        threshold
                    )));
                }
                if *lbfgs_history == 0 {
                    return Err(ConfigError::InvalidOptimizer(
                        "QQN L-BFGS history must be > 0".to_string(),
                    ));
                }
            }
            OptimizerType::LBFGS { history, .. } => {
                if *history == 0 {
                    return Err(ConfigError::InvalidOptimizer(
                        "L-BFGS history must be > 0".to_string(),
                    ));
                }
            }
            OptimizerType::Adam {
                learning_rate,
                beta1,
                beta2,
                ..
            } => {
                if *learning_rate <= 0.0 {
                    return Err(ConfigError::InvalidOptimizer(format!(
                        "Adam learning rate must be > 0, got {}",
                        learning_rate
                    )));
                }
                if *beta1 <= 0.0 || *beta1 >= 1.0 {
                    return Err(ConfigError::InvalidOptimizer(format!(
                        "Adam beta1 must be in (0, 1), got {}",
                        beta1
                    )));
                }
                if *beta2 <= 0.0 || *beta2 >= 1.0 {
                    return Err(ConfigError::InvalidOptimizer(format!(
                        "Adam beta2 must be in (0, 1), got {}",
                        beta2
                    )));
                }
            }
            OptimizerType::SGD {
                learning_rate,
                momentum,
            } => {
                if *learning_rate <= 0.0 {
                    return Err(ConfigError::InvalidOptimizer(format!(
                        "SGD learning rate must be > 0, got {}",
                        learning_rate
                    )));
                }
                if let Some(m) = momentum {
                    if *m < 0.0 || *m >= 1.0 {
                        return Err(ConfigError::InvalidOptimizer(format!(
                            "SGD momentum must be in [0, 1), got {}",
                            m
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

impl BenchmarkConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_iterations == 0 {
            return Err(ConfigError::InvalidBenchmark(
                "max_iterations must be > 0".to_string(),
            ));
        }
        if self.tolerance <= 0.0 {
            return Err(ConfigError::InvalidBenchmark(format!(
                "tolerance must be > 0, got {}",
                self.tolerance
            )));
        }
        if self.max_function_evaluations == 0 {
            return Err(ConfigError::InvalidBenchmark(
                "max_function_evaluations must be > 0".to_string(),
            ));
        }
        if self.time_limit.is_zero() {
            return Err(ConfigError::InvalidBenchmark(
                "time_limit must be > 0".to_string(),
            ));
        }
        if self.num_runs == 0 {
            return Err(ConfigError::InvalidBenchmark(
                "num_runs must be > 0".to_string(),
            ));
        }
        Ok(())
    }
}

impl AnalysisConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.confidence_level <= 0.0 || self.confidence_level >= 1.0 {
            return Err(ConfigError::InvalidAnalysis(format!(
                "confidence_level must be in (0, 1), got {}",
                self.confidence_level
            )));
        }
        if self.statistical_tests.is_empty() {
            return Err(ConfigError::InvalidAnalysis(
                "at least one statistical test must be specified".to_string(),
            ));
        }
        Ok(())
    }
}

impl OutputConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.plot_formats.is_empty() && self.generate_plots {
            return Err(ConfigError::InvalidOutput(
                "plot_formats cannot be empty when generate_plots is true".to_string(),
            ));
        }
        Ok(())
    }
}

// Configuration builder for programmatic creation
pub struct ExperimentConfigBuilder {
    config: ExperimentConfig,
}

impl ExperimentConfigBuilder {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            config: ExperimentConfig {
                name: name.to_string(),
                description: description.to_string(),
                problems: Vec::new(),
                optimizers: Vec::new(),
                benchmark_settings: BenchmarkConfig::default(),
                analysis_settings: AnalysisConfig::default(),
                output_settings: OutputConfig::default(),
            },
        }
    }

    pub fn add_problem(mut self, problem: ProblemConfig) -> Self {
        self.config.problems.push(problem);
        self
    }

    pub fn add_optimizer(mut self, optimizer: OptimizerConfig) -> Self {
        self.config.optimizers.push(optimizer);
        self
    }

    pub fn benchmark_settings(mut self, settings: BenchmarkConfig) -> Self {
        self.config.benchmark_settings = settings;
        self
    }

    pub fn analysis_settings(mut self, settings: AnalysisConfig) -> Self {
        self.config.analysis_settings = settings;
        self
    }

    pub fn output_settings(mut self, settings: OutputConfig) -> Self {
        self.config.output_settings = settings;
        self
    }

    pub fn build(self) -> Result<ExperimentConfig, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

// Error types for configuration handling
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file {0}: {1}")]
    FileRead(PathBuf, std::io::Error),

    #[error("Failed to write config file {0}: {1}")]
    FileWrite(PathBuf, std::io::Error),

    #[error("Failed to parse config: {0}")]
    Parse(serde_yaml::Error),

    #[error("Failed to serialize config: {0}")]
    Serialize(serde_yaml::Error),

    #[error("Invalid problem configuration: {0}")]
    InvalidProblem(String),

    #[error("Invalid optimizer configuration: {0}")]
    InvalidOptimizer(String),

    #[error("Invalid benchmark configuration: {0}")]
    InvalidBenchmark(String),

    #[error("Invalid analysis configuration: {0}")]
    InvalidAnalysis(String),

    #[error("Invalid output configuration: {0}")]
    InvalidOutput(String),
}

// Helper functions for creating common configurations
pub fn create_qqn_config(threshold: f64, history: usize) -> OptimizerConfig {
    OptimizerConfig {
        name: format!("qqn_t{}_h{}", threshold, history),
        optimizer_type: OptimizerType::QQN {
            threshold,
            lbfgs_history: history,
            line_search: LineSearchConfig::default(),
            epsilon: Some(1e-8),
        },
    }
}

pub fn create_lbfgs_config(history: usize) -> OptimizerConfig {
    OptimizerConfig {
        name: format!("lbfgs_h{}", history),
        optimizer_type: OptimizerType::LBFGS {
            history,
            line_search: LineSearchConfig::default(),
        },
    }
}

pub fn create_adam_config(lr: f64) -> OptimizerConfig {
    OptimizerConfig {
        name: format!("adam_lr{}", lr),
        optimizer_type: OptimizerType::Adam {
            learning_rate: lr,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: Some(1e-8),
        },
    }
}

pub fn create_rosenbrock_problem(dimension: usize) -> ProblemConfig {
    ProblemConfig {
        name: format!("rosenbrock_{}d", dimension),
        problem_type: ProblemType::Rosenbrock { dimension },
        parameters: None,
    }
}

pub fn create_rastrigin_problem(dimension: usize) -> ProblemConfig {
    ProblemConfig {
        name: format!("rastrigin_{}d", dimension),
        problem_type: ProblemType::Rastrigin { dimension },
        parameters: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_serialization() {
        let config = ExperimentConfigBuilder::new("test_experiment", "Test experiment description")
            .add_problem(create_rosenbrock_problem(10))
            .add_optimizer(create_qqn_config(0.01, 10))
            .add_optimizer(create_lbfgs_config(10))
            .build()
            .unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Test serialization
        config.to_file(&path).unwrap();

        // Test deserialization
        let loaded_config = ExperimentConfig::from_file(&path).unwrap();

        assert_eq!(config.name, loaded_config.name);
        assert_eq!(config.description, loaded_config.description);
        assert_eq!(config.problems.len(), loaded_config.problems.len());
        assert_eq!(config.optimizers.len(), loaded_config.optimizers.len());
    }

    #[test]
    fn test_config_validation() {
        // Test invalid QQN threshold
        let invalid_config = ExperimentConfigBuilder::new("test", "test")
            .add_optimizer(OptimizerConfig {
                name: "invalid_qqn".to_string(),
                optimizer_type: OptimizerType::QQN {
                    threshold: 1.5, // Invalid: > 1.0
                    lbfgs_history: 10,
                    line_search: LineSearchConfig::default(),
                    epsilon: Some(1e-8),
                },
            })
            .build();

        assert!(invalid_config.is_err());

        // Test invalid problem dimension
        let invalid_problem = ProblemConfig {
            name: "invalid_rosenbrock".to_string(),
            problem_type: ProblemType::Rosenbrock { dimension: 1 }, // Invalid: < 2
            parameters: None,
        };

        assert!(invalid_problem.validate().is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let config = ExperimentConfigBuilder::new("builder_test", "Testing builder pattern")
            .add_problem(create_rosenbrock_problem(5))
            .add_problem(create_rastrigin_problem(10))
            .add_optimizer(create_qqn_config(0.05, 15))
            .add_optimizer(create_lbfgs_config(15))
            .add_optimizer(create_adam_config(0.001))
            .benchmark_settings(BenchmarkConfig {
                max_iterations: 500,
                num_runs: 5,
                ..Default::default()
            })
            .build()
            .unwrap();

        assert_eq!(config.problems.len(), 2);
        assert_eq!(config.optimizers.len(), 3);
        assert_eq!(config.benchmark_settings.max_iterations, 500);
        assert_eq!(config.benchmark_settings.num_runs, 5);
    }
}