use super::{ExperimentRunner, ParameterEvolution, OptimizerGenome};
use crate::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults, BenchmarkRunner, ProblemSpec, new_initial_point, DurationWrapper};
use crate::Optimizer;
use log::{info, warn};
use rand::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

/// Adaptive experiment runner that evolves optimizer parameters for each problem
pub struct AdaptiveExperimentRunner {
    base_runner: ExperimentRunner,
    population_size: usize,
    num_generations: usize,
    evaluation_runs: usize,
    output_dir: String,
    config: BenchmarkConfig,
}

impl AdaptiveExperimentRunner {
    pub fn new(
        output_dir: String,
        config: BenchmarkConfig,
        population_size: usize,
        num_generations: usize,
        evaluation_runs: usize,
        max_concurrent_tasks: Option<usize>,
    ) -> Self {
        let base_runner = ExperimentRunner::new(
            output_dir.clone(),
            config.clone(),
            max_concurrent_tasks,
        );

        Self {
            base_runner,
            population_size,
            num_generations,
            evaluation_runs,
            output_dir,
            config,
        }
    }

    /// Run adaptive parameter evolution to find best optimizer configurations for each problem
    pub async fn run_adaptive_evolution(
        &self,
        problems: Vec<ProblemSpec>,
        optimizer_types: Vec<super::parameter_evolution::OptimizerType>,
    ) -> anyhow::Result<HashMap<String, Vec<(String, Arc<dyn Optimizer>)>>> {
        info!("Starting adaptive parameter evolution");
        info!("Population size: {}, Generations: {}, Evaluation runs: {}",
              self.population_size, self.num_generations, self.evaluation_runs);

        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;
        let evolution_dir = format!("{}/evolution", self.output_dir);
        fs::create_dir_all(&evolution_dir)?;

        // Validate problems first
        self.base_runner.validate_problems(&problems).await?;

        let mut problem_best_optimizers = HashMap::new();

        // Evolve parameters for each problem independently
        for problem in &problems {
            info!("Evolving optimizer parameters for problem: {}", problem.get_name());

            let best_genomes = self.evolve_for_problem(
                problem.clone(),
                optimizer_types.clone(),
                &evolution_dir,
            ).await?;

            // Convert best genomes to optimizers
            let mut optimizers = Vec::new();
            for (i, genome) in best_genomes.iter().enumerate() {
                let name = format!("{:?}-Evolved-{}", genome.optimizer_type, i);
                optimizers.push((name, genome.to_optimizer()));
            }

            problem_best_optimizers.insert(problem.get_name(), optimizers);
        }

        // Save evolution results
        self.save_evolution_results(&problem_best_optimizers, &evolution_dir)?;

        Ok(problem_best_optimizers)
    }

    async fn evolve_for_problem(
        &self,
        problem: ProblemSpec,
        optimizer_types: Vec<super::parameter_evolution::OptimizerType>,
        evolution_dir: &str,
    ) -> anyhow::Result<Vec<OptimizerGenome>> {
        let mut evolution = ParameterEvolution::new(self.population_size, 42);
        let mut population = evolution.initialize_population(optimizer_types);

        let problem_evolution_dir = format!("{}/{}", evolution_dir, problem.get_name());
        fs::create_dir_all(&problem_evolution_dir)?;

        for generation in 0..self.num_generations {
            info!("Generation {}/{} for problem {}",
                  generation + 1, self.num_generations, problem.get_name());

            // Evaluate fitness of population
            self.evaluate_population(&mut population, &problem).await?;

            // Log best fitness
            let best_fitness = population.iter()
                .filter_map(|g| g.fitness)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(f64::INFINITY);

            info!("Best fitness in generation {}: {:.6e}", generation, best_fitness);

            // Save generation results
            self.save_generation_results(
                &population,
                generation,
                &problem_evolution_dir,
            )?;

            // Evolve to next generation (except for last generation)
            if generation < self.num_generations - 1 {
                population = evolution.evolve_generation(&mut population, generation + 1);
            }
        }

        // Return top performers
        let top_n = 5.min(self.population_size / 2);
        Ok(evolution.get_best_genomes(&population, top_n))
    }

    async fn evaluate_population(
        &self,
        population: &mut [OptimizerGenome],
        problem: &ProblemSpec,
    ) -> anyhow::Result<()> {
        let semaphore = Arc::new(Semaphore::new(8)); // Limit concurrent evaluations
        let mut tasks = Vec::new();

        for (idx, genome) in population.iter().enumerate() {
            if genome.fitness.is_some() {
                continue; // Skip already evaluated genomes (elites)
            }

            let semaphore = semaphore.clone();
            let optimizer = genome.to_optimizer();
            let problem = problem.clone();
            let config = self.config.clone();
            let evaluation_runs = self.evaluation_runs;

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                Self::evaluate_genome(optimizer, problem, config, evaluation_runs).await
                    .map(|fitness| (idx, fitness))
            });

            tasks.push(task);
        }

        // Collect results
        for task in tasks {
            match task.await {
                Ok(Ok((idx, fitness))) => {
                    population[idx].fitness = Some(fitness);
                }
                Ok(Err(e)) => {
                    warn!("Failed to evaluate genome: {}", e);
                    // Assign worst fitness to failed evaluations
                    population[0].fitness = Some(f64::INFINITY);
                }
                Err(e) => {
                    warn!("Evaluation task panicked: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn evaluate_genome(
        optimizer: Arc<dyn Optimizer>,
        problem: ProblemSpec,
        config: BenchmarkConfig,
        num_runs: usize,
    ) -> anyhow::Result<f64> {
        let runner = BenchmarkRunner::new(config.clone());
        let mut total_fitness = 0.0;
        let mut successful_runs = 0;
        let mut rng = StdRng::seed_from_u64(42);

        for run_id in 0..num_runs {
            let result = runner.run_single_benchmark(
                &problem,
                &mut optimizer.clone_box(),
                run_id,
                "eval",
                new_initial_point(&problem, config.initial_point_noise, &mut rng),
            ).await?;

            // Fitness is combination of final value and convergence speed
            let fitness = if result.best_value.is_finite() {
                let value_component = result.best_value.abs().ln().max(-20.0);
                let speed_component = (result.iterations as f64) / (config.max_iterations as f64);
                value_component + 0.1 * speed_component
            } else {
                1e10 // Penalty for non-convergence
            };

            if fitness < 1e9 {
                total_fitness += fitness;
                successful_runs += 1;
            }
        }

        // Return average fitness, with penalty for failed runs
        if successful_runs > 0 {
            Ok(total_fitness / (successful_runs as f64) + (num_runs - successful_runs) as f64 * 100.0)
        } else {
            Ok(f64::INFINITY)
        }
    }

    fn save_generation_results(
        &self,
        population: &[OptimizerGenome],
        generation: usize,
        output_dir: &str,
    ) -> anyhow::Result<()> {
        let filename = format!("{}/generation_{:03}.json", output_dir, generation);
        let json = serde_json::to_string_pretty(population)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn save_evolution_results(
        &self,
        results: &HashMap<String, Vec<(String, Arc<dyn Optimizer>)>>,
        output_dir: &str,
    ) -> anyhow::Result<()> {
        let mut summary = String::new();
        summary.push_str("# Adaptive Evolution Results\n\n");

        for (problem_name, optimizers) in results {
            summary.push_str(&format!("## Problem: {}\n\n", problem_name));
            summary.push_str(&format!("Found {} evolved optimizer configurations:\n", optimizers.len()));

            for (name, _) in optimizers {
                summary.push_str(&format!("- {}\n", name));
            }
            summary.push_str("\n");
        }

        fs::write(format!("{}/evolution_summary.md", output_dir), summary)?;
        Ok(())
    }

    /// Run final championship with evolved optimizers
    pub async fn run_evolved_championship(
        &self,
        problems: Vec<ProblemSpec>,
        evolved_optimizers: HashMap<String, Vec<(String, Arc<dyn Optimizer>)>>,
    ) -> anyhow::Result<()> {
        info!("Running championship with evolved optimizers");

        // For each problem, run benchmarks with its evolved optimizers
        for problem in problems {
            let problem_name = problem.get_name();

            if let Some(optimizers) = evolved_optimizers.get(&problem_name) {
                info!("Running championship for {} with {} evolved optimizers",
                      problem_name, optimizers.len());

                // Run comparative benchmarks for this problem
                self.base_runner.run_comparative_benchmarks(
                    vec![problem],
                    optimizers.clone(),
                ).await?;
            }
        }

        Ok(())
    }
}

/// Convenience function to run adaptive evolution experiments
pub async fn run_adaptive_benchmark(
    report_path_prefix: &str,
    max_evals: usize,
    num_runs: usize,
    time_limit: Duration,
    population_size: usize,
    num_generations: usize,
    evaluation_runs: usize,
    problems: Vec<ProblemSpec>,
    optimizer_types: Vec<super::parameter_evolution::OptimizerType>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("{report_path_prefix}adaptive_{timestamp}");
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir)?;

    println!("Creating adaptive benchmark results in: {}", output_dir.display());

    let config = BenchmarkConfig {
        max_iterations: max_evals,
        maximum_function_calls: max_evals,
        time_limit: DurationWrapper::from(time_limit),
        num_runs,
        ..BenchmarkConfig::default()
    };

    let runner = AdaptiveExperimentRunner::new(
        output_dir.to_string_lossy().to_string(),
        config,
        population_size,
        num_generations,
        evaluation_runs,
        Some(8),
    );

    // First, evolve optimizer parameters for each problem
    let evolved_optimizers = runner.run_adaptive_evolution(
        problems.clone(),
        optimizer_types,
    ).await?;

    // Then run final championship with evolved optimizers
    runner.run_evolved_championship(problems, evolved_optimizers).await?;

    println!("Adaptive benchmark completed successfully");
    println!("Results saved to: {}", output_dir.display());

    Ok(())
}