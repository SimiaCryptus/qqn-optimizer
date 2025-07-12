//! Serialization utilities for saving and loading results.

use crate::core::OptResult;
use crate::benchmarks::BenchmarkResults;
use crate::config::ExperimentConfig;
use serde::{Serialize, Deserialize};
use std::path::Path;
use bincode::config::standard;

/// Save benchmark results to file
pub fn save_results(results: &BenchmarkResults, path: &Path) -> OptResult<()> {
    let serializer = ResultsSerializer::new();
    serializer.save(results, path)
}

/// Load benchmark results from file
pub fn load_results(path: &Path) -> OptResult<BenchmarkResults> {
    let serializer = ResultsSerializer::new();
    serializer.load(path)
}

/// Save experiment configuration to file
pub fn save_config(config: &ExperimentConfig, path: &Path) -> OptResult<()> {
    let serializer = ConfigSerializer::new();
    serializer.save(config, path)
}

/// Load experiment configuration from file
pub fn load_config(path: &Path) -> OptResult<ExperimentConfig> {
    let serializer = ConfigSerializer::new();
    serializer.load(path)
}

/// Results serializer
pub struct ResultsSerializer {
    format: SerializationFormat,
}

impl ResultsSerializer {
    pub fn new() -> Self {
        Self {
            format: SerializationFormat::Json,
        }
    }
    
    pub fn with_format(format: SerializationFormat) -> Self {
        Self { format }
    }
    
    pub fn save(&self, results: &BenchmarkResults, path: &Path) -> OptResult<()> {
        match self.format {
            SerializationFormat::Json => {
                let json = serde_json::to_string_pretty(results)?;
                std::fs::write(path, json)?;
            }
            SerializationFormat::Bincode => {
                let encoded = bincode::serialize(results)?;
                std::fs::write(path, encoded)?;
            }
        }
        Ok(())
    }
    
    pub fn load(&self, path: &Path) -> OptResult<BenchmarkResults> {
        match self.format {
            SerializationFormat::Json => {
                let content = std::fs::read_to_string(path)?;
                let results = serde_json::from_str(&content)?;
                Ok(results)
            }
            SerializationFormat::Bincode => {
                let content = std::fs::read(path)?;
                let results = bincode::deserialize(&content)?;
                Ok(results)
            }
        }
    }
}

/// Configuration serializer
pub struct ConfigSerializer {
    format: SerializationFormat,
}

impl ConfigSerializer {
    pub fn new() -> Self {
        Self {
            format: SerializationFormat::Toml,
        }
    }
    
    pub fn with_format(format: SerializationFormat) -> Self {
        Self { format }
    }
    
    pub fn save(&self, config: &ExperimentConfig, path: &Path) -> OptResult<()> {
        match self.format {
            SerializationFormat::Json => {
                let json = serde_json::to_string_pretty(config)?;
                std::fs::write(path, json)?;
            }
            SerializationFormat::Toml => {
                let toml = toml::to_string_pretty(config)?;
                std::fs::write(path, toml)?;
            }
            SerializationFormat::Bincode => {
                let encoded = bincode::serialize(config)?;
                std::fs::write(path, encoded)?;
            }
        }
        Ok(())
    }
    
    pub fn load(&self, path: &Path) -> OptResult<ExperimentConfig> {
        match self.format {
            SerializationFormat::Json => {
                let content = std::fs::read_to_string(path)?;
                let config = serde_json::from_str(&content)?;
                Ok(config)
            }
            SerializationFormat::Toml => {
                let content = std::fs::read_to_string(path)?;
                let config = toml::from_str(&content)?;
                Ok(config)
            }
            SerializationFormat::Bincode => {
                let content = std::fs::read(path)?;
                let config = bincode::deserialize(&content)?;
                Ok(config)
            }
        }
    }
}

/// Supported serialization formats
#[derive(Debug, Clone)]
pub enum SerializationFormat {
    Json,
    Toml,
    Bincode,
}