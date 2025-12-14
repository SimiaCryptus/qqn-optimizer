use super::{ExperimentRunner, OptimizerGenome, ParameterEvolution};
use crate::benchmarks::evaluation::{
    is_no_threshold_mode, new_initial_point, BenchmarkConfig, BenchmarkResults, BenchmarkRunner,
    DurationWrapper, ProblemSpec,
};
use crate::Optimizer;
use itertools::Itertools;
use log::{debug, info, trace, warn};
use rand::prelude::*;
use rand::rng;
use serde::de::Unexpected::Float;
use serde_json::json;
use std::collections::HashMap;
use std::f32::INFINITY;
use std::fs;
use std::iter::Take;
use std::slice::Iter;
use std::sync::Arc;
use std::time::Duration;
use luminal::prelude::Shape;
use tokio::sync::Semaphore;

/// Detailed tracking of evolutionary events
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionaryEvent {
    pub generation: usize,
    pub event_type: EventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub individual_id: usize,
    pub parent_ids: Vec<usize>,
    pub fitness_before: Option<f32>,
    pub fitness_after: Option<f32>,
    pub parameters: HashMap<String, f32>,
    pub family: String,
    pub details: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EventType {
    Initialization,
    Selection,
    Crossover,
    Mutation,
    Evaluation,
    Elitism,
    FamilyBalancing,
}
/// Detailed individual tracking
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndividualRecord {
    pub id: usize,
    pub generation_created: usize,
    pub family: String,
    pub parameters: HashMap<String, f32>,
    pub fitness_history: Vec<(usize, f32)>, // (generation, fitness)
    pub parent_ids: Vec<usize>,
    pub offspring_ids: Vec<usize>,
    pub selection_count: usize,
    pub mutation_count: usize,
    pub crossover_count: usize,
    pub is_elite: bool,
    pub final_generation: Option<usize>,
}
/// Family representation tracker
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FamilyRepresentation {
    pub family_name: String,
    pub target_proportion: f32,
    pub current_count: usize,
    pub total_created: usize,
    pub best_fitness: Option<f32>,
    pub worst_fitness: Option<f32>,
    pub average_fitness: f32,
    pub generations_dominated: Vec<usize>,
}
/// Final selected optimizer details
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SelectedOptimizer {
    pub problem_name: String,
    pub optimizer_name: String,
    pub family: String,
    pub individual_id: usize,
    pub generation_created: usize,
    pub final_fitness: f32,
    pub parameters: HashMap<String, f32>,
    pub selection_rank: usize,
    pub parent_ids: Vec<usize>,
    pub total_evaluations: usize,
}

/// Comprehensive evolution tracker
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionTracker {
    pub problem_name: String,
    pub population_size: usize,
    pub num_generations: usize,
    pub events: Vec<EvolutionaryEvent>,
    pub individuals: HashMap<usize, IndividualRecord>,
    pub family_representations: HashMap<String, FamilyRepresentation>,
    pub generation_summaries: Vec<GenerationSummary>,
    pub next_individual_id: usize,
    pub selected_optimizers: Vec<SelectedOptimizer>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerationSummary {
    pub generation: usize,
    pub best_fitness: f32,
    pub worst_fitness: f32,
    pub average_fitness: f32,
    pub fitness_std: f32,
    pub family_counts: HashMap<String, usize>,
    pub diversity_metrics: DiversityMetrics,
    pub selection_pressure: f32,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiversityMetrics {
    pub parameter_variance: HashMap<String, f32>,
    pub fitness_diversity: f32,
    pub family_diversity: f32,
}

/// Adaptive experiment runner that evolves optimizer parameters for each problem
pub struct AdaptiveExperimentRunner {
    base_runner: ExperimentRunner,
    population_size: usize,
    num_generations: usize,
    evaluation_runs: usize,
    output_dir: String,
    config: BenchmarkConfig,
    family_proportions: HashMap<String, f32>,
    min_family_representation: usize,
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
        let base_runner =
            ExperimentRunner::new(output_dir.clone(), config.clone(), max_concurrent_tasks);
        // Initialize empty family proportions - will be set dynamically based on optimizer_types
        let family_proportions = HashMap::new();
        let min_family_representation = std::cmp::max(1, population_size / 10);

        Self {
            base_runner,
            population_size,
            num_generations,
            evaluation_runs,
            output_dir,
            config,
            family_proportions,
            min_family_representation,
        }
    }

    /// Run adaptive parameter evolution to find best optimizer configurations for each problem
    pub async fn run_adaptive_evolution<S: Shape>(
        &mut self,
        problems: Vec<ProblemSpec>,
        optimizer_types: Vec<super::parameter_evolution::OptimizerType>,
    ) -> anyhow::Result<HashMap<String, Vec<(String, Arc<dyn Optimizer<S>>)>>> {
        info!("Starting adaptive parameter evolution");
        info!(
            "Population size: {}, Generations: {}, Evaluation runs: {}",
            self.population_size, self.num_generations, self.evaluation_runs
        );

        info!(
            "Will evolve {} optimizer families independently",
            optimizer_types.len()
        );

        // Ensure output directory exists
        fs::create_dir_all(self.output_dir.to_string())?;
        let evolution_dir = format!("{}/evolution", self.output_dir);
        fs::create_dir_all(evolution_dir.to_string())?;
        info!("Created evolution directory: {}", evolution_dir);

        // Validate problems first
        info!("Validating {} problems", problems.len());
        self.base_runner.validate_problems(&problems).await?;
        info!("Problem validation completed successfully");

        // Group problems by family
        let mut problem_families: HashMap<String, Vec<ProblemSpec>> = HashMap::new();
        for problem in problems {
            problem_families
                .entry(problem.family.clone())
                .or_insert_with(Vec::new)
                .push(problem);
        }

        info!("Grouped problems into {} families", problem_families.len());
        for (family, probs) in &problem_families {
            info!("Family '{}': {} problems", family, probs.len());
        }

        let mut family_best_optimizers = HashMap::new();

        // Evolve parameters for each problem family
        for (family_idx, (family_name, family_problems)) in problem_families.iter().enumerate() {
            info!(
                "Family {}/{}: {} ({} problems)",
                family_idx + 1,
                problem_families.len(),
                family_name,
                family_problems.len()
            );

            // Evolve each optimizer family separately for this problem
            let mut all_best_genomes = Vec::new();

            for optimizer_type in &optimizer_types {
                info!(
                    "Evolving {:?} optimizer for problem family '{}'",
                    optimizer_type, family_name
                );

                let family_best = self
                    .evolve_optimizer_for_problem_family::<S>(
                        family_problems.clone(),
                        optimizer_type.clone(),
                        &evolution_dir,
                        family_name,
                    )
                    .await?;

                info!(
                    "Found {} best {:?} configurations for problem family '{}'",
                    family_best.len(),
                    optimizer_type,
                    family_name
                );
                // Add selected optimizer details
                all_best_genomes.extend(family_best.clone());

                // Save detailed selected optimizers JSON
                let outdir = format!(
                    "{}/family_{}/{}",
                    evolution_dir, family_name, optimizer_type
                );
                fs::create_dir_all(outdir.to_string());
                let selected_json_path =
                    format!("{}/all_selected_optimizers.json", outdir.to_string());
                let selected_json = serde_json::to_string_pretty(&family_best.clone())?;
                fs::write(selected_json_path.to_string(), selected_json)?;
                info!(
                    "Saved {} selected optimizers to {}",
                    family_best.len(),
                    selected_json_path
                );
            }

            info!("Evolution completed for problem family '{}', found {} total best genomes across all optimizer types",
                  family_name, all_best_genomes.len());

            // Convert best genomes to optimizers
            let mut optimizers: Vec<(String, Arc<dyn Optimizer<S>>)> = Vec::new();
            let best: Vec<OptimizerGenome> = all_best_genomes
                .iter()
                .into_group_map_by(|x| x.optimizer_type.to_string())
                .values()
                .flat_map(|genomes| {
                    let mut x1: Vec<OptimizerGenome> =
                        genomes.iter().map(|x| (*x).clone()).collect_vec();
                    x1.sort_by(|a, b| {
                        let fitness_a = a.fitness.unwrap_or(INFINITY);
                        let fitness_b = b.fitness.unwrap_or(INFINITY);
                        fitness_a
                            .partial_cmp(&fitness_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    x1.into_iter().take(1) // Take the best (1) genome from each family
                })
                .collect_vec();
            for (i, genome) in best.iter().enumerate() {
                let optimizer_family_name = format!("{:?}", genome.optimizer_type);
                let name = format!(
                    "{}-Evolved-Family-{}-{}",
                    optimizer_family_name,
                    family_name,
                    genome.id.unwrap_or(0)
                );
                info!(
                    "Created evolved optimizer '{}' with fitness: {:?}",
                    name, genome.fitness
                );
                optimizers.push((name, genome.to_optimizer::<S>()));
            }

            family_best_optimizers.insert(family_name.clone(), optimizers);
        }

        // Now map the family optimizers to each individual problem for compatibility
        let mut problem_best_optimizers = HashMap::new();
        for (family_name, family_problems) in problem_families {
            if let Some(family_optimizers) = family_best_optimizers.get(&family_name) {
                for problem in family_problems {
                    problem_best_optimizers.insert(problem.get_name(), family_optimizers.clone());
                }
            }
        }

        Ok(problem_best_optimizers)
    }
    /// Set family proportions dynamically based on the provided optimizer types
    fn set_dynamic_family_proportions(
        &mut self,
        optimizer_types: &[super::parameter_evolution::OptimizerType],
    ) {
        self.family_proportions.clear();
        if optimizer_types.is_empty() {
            return;
        }
        // Equal proportions for all provided optimizer types
        let equal_proportion = 1.0 / optimizer_types.len() as f32;
        for optimizer_type in optimizer_types {
            let family_name = format!("{:?}", optimizer_type);
            self.family_proportions
                .insert(family_name, equal_proportion);
        }
        info!(
            "Set dynamic family proportions for {} optimizer types",
            optimizer_types.len()
        );
        for (family, proportion) in &self.family_proportions {
            debug!(
                "Family '{}': {:.1}% target proportion",
                family,
                proportion * 100.0
            );
        }
    }

    async fn evolve_optimizer_for_problem_family<S: Shape>(
        &self,
        family_problems: Vec<ProblemSpec>,
        optimizer_type: super::parameter_evolution::OptimizerType,
        evolution_dir: &str,
        family_name: &str,
    ) -> anyhow::Result<Vec<OptimizerGenome>> {
        info!(
            "Starting evolution for problem family: {} with optimizer type: {:?}",
            family_name, optimizer_type
        );

        let mut evolution = ParameterEvolution::new(self.population_size, 42);
        let mut population = evolution.initialize_single_family_population(optimizer_type.clone());
        info!("Initialized population of {} individuals", population.len());

        // Initialize evolution tracker
        let optimizer_family_name = format!("{:?}", optimizer_type);
        let mut tracker = EvolutionTracker::new(
            family_name.to_string(),
            self.population_size,
            self.num_generations,
            &[optimizer_type.clone()],
            &HashMap::from([(optimizer_family_name.clone(), 1.0)]), // Single optimizer type gets 100%
        );
        debug!("Evolution tracker initialized");

        // Assign IDs to initial population
        for (idx, genome) in population.iter_mut().enumerate() {
            genome.id = Some(idx);
        }

        // Track initial population
        self.track_initial_population(&mut tracker, &population);
        debug!("Initial population tracking completed");

        let problem_evolution_dir = format!(
            "{}/family_{}/{}",
            evolution_dir, family_name, optimizer_family_name
        );
        fs::create_dir_all(problem_evolution_dir.to_string())?;
        info!(
            "Created family-specific evolution directory: {}",
            problem_evolution_dir
        );

        for generation in 0..self.num_generations {
            info!(
                "Generation {}/{} for problem family {} - {:?} optimizer",
                generation + 1,
                self.num_generations,
                family_name,
                optimizer_type
            );
            let start_time = std::time::Instant::now();

            // Evaluate fitness of population
            debug!("Starting fitness evaluation for generation {}", generation);
            self.evaluate_population_on_family::<S>(
                &mut population,
                &family_problems,
                &mut tracker,
                generation,
            )
            .await?;
            debug!("Fitness evaluation completed in {:?}", start_time.elapsed());

            // Log best fitness
            let best_fitness = population
                .iter()
                .filter_map(|g| g.fitness)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(f32::INFINITY);
            let avg_fitness =
                population.iter().filter_map(|g| g.fitness).sum::<f32>() / population.len() as f32;

            info!(
                "{:?} optimizer - Generation {} results - Best: {:.6e}, Avg: {:.6e}",
                optimizer_type, generation, best_fitness, avg_fitness
            );

            // Track generation summary
            self.track_generation_summary(&mut tracker, &population, generation);

            // Save generation results
            debug!("Saving generation {} results", generation);
            self.save_detailed_generation_results(
                &population,
                &tracker,
                generation,
                &problem_evolution_dir,
            )?;
            debug!("Generation {} results saved", generation);

            // Evolve to next generation (except for last generation)
            if generation < self.num_generations - 1 {
                debug!("Evolving to generation {}", generation + 1);
                population = self.evolve_generation_with_tracking(
                    &mut evolution,
                    &mut population,
                    &mut tracker,
                    generation + 1,
                );
                debug!("Evolution to generation {} completed", generation + 1);
            }
            info!(
                "Generation {} completed in {:?}",
                generation,
                start_time.elapsed()
            );
        }

        // Save comprehensive evolution report
        info!("Generating comprehensive evolution report");
        self.save_comprehensive_evolution_report(&tracker, &problem_evolution_dir)?;
        info!("Comprehensive evolution report saved");

        // Return top performers - ensure we always return results
        let top_n = 3.min(self.population_size / 4).max(1); // At least 1, at most population_size/4
        info!(
            "Returning top {} performers for {:?} family",
            top_n, optimizer_type
        );

        // Sort population by fitness and take top N
        // Ensure we ALWAYS return the requested number of results
        let mut sorted_population = population.clone();

        // First pass: ensure all genomes have fitness values
        let mut has_valid_fitness = false;
        for genome in sorted_population.iter_mut() {
            if genome.fitness.is_none() {
                warn!(
                    "Genome {:?} without fitness found, assigning penalty fitness",
                    genome.id
                );
                genome.fitness = Some(f32::INFINITY);
                genome.success_rate = Some(0.0);
                genome.mean_final_value = Some(f32::INFINITY);
                genome.total_evaluations = Some(usize::MAX);
            } else if genome.fitness.unwrap() < f32::INFINITY {
                has_valid_fitness = true;
            }
        }

        if !has_valid_fitness {
            warn!(
                "No genomes with valid fitness values found for {:?} family!",
                optimizer_type
            );
            // If no valid fitness values, try to re-evaluate a few random genomes
            info!("Attempting emergency evaluation of random genomes");
            for i in 0..top_n.min(3) {
                if i < sorted_population.len() {
                    let genome = &mut sorted_population[i];
                    match Self::evaluate_genome_with_metrics(
                        genome.to_optimizer::<S>(),
                        (&family_problems[0]).clone().clone(),
                        self.config.clone(),
                        1, // Just one run for emergency evaluation
                    )
                    .await
                    {
                        Ok((fitness, success_rate, mean_value, eval_count)) => {
                            info!(
                                "Emergency evaluation {} succeeded: fitness = {:.6e}",
                                i, fitness
                            );
                            genome.fitness = Some(fitness);
                            genome.success_rate = Some(success_rate);
                            genome.mean_final_value = Some(mean_value);
                            genome.total_evaluations = Some(eval_count);
                            has_valid_fitness = true;
                        }
                        Err(e) => {
                            warn!("Emergency evaluation {} failed: {}", i, e);
                            genome.fitness = Some(f32::INFINITY);
                            genome.success_rate = Some(0.0);
                            genome.mean_final_value = Some(f32::INFINITY);
                            genome.total_evaluations = Some(usize::MAX);
                        }
                    }
                }
            }
        }

        // Sort using the sophisticated logic: success rate first, then mean final value, then total evaluations
        sorted_population.sort_by(|a, b| {
            if is_no_threshold_mode() {
                // In no-threshold mode, sort by mean final value, then by total evaluations
                let mean_a = a.mean_final_value.unwrap_or(f32::INFINITY);
                let mean_b = b.mean_final_value.unwrap_or(f32::INFINITY);
                let evals_a = a.total_evaluations.unwrap_or(usize::MAX) as f32;
                let evals_b = b.total_evaluations.unwrap_or(usize::MAX) as f32;

                match mean_a.total_cmp(&mean_b) {
                    std::cmp::Ordering::Equal => evals_a.total_cmp(&evals_b),
                    ord => ord,
                }
            } else {
                // Sort by success rate first (higher is better), then by mean final value (lower is better), then by total evaluations (lower is better)
                let success_a = a.success_rate.unwrap_or(0.0);
                let success_b = b.success_rate.unwrap_or(0.0);
                let mean_a = a.mean_final_value.unwrap_or(f32::INFINITY);
                let mean_b = b.mean_final_value.unwrap_or(f32::INFINITY);
                let evals_a = a.total_evaluations.unwrap_or(usize::MAX) as f32;
                let evals_b = b.total_evaluations.unwrap_or(usize::MAX) as f32;

                match success_b.total_cmp(&success_a) {
                    // Note: reversed for success rate (higher is better)
                    std::cmp::Ordering::Equal => match mean_a.total_cmp(&mean_b) {
                        std::cmp::Ordering::Equal => evals_a.total_cmp(&evals_b),
                        ord => ord,
                    },
                    ord => ord,
                }
            }
        });

        // Take top N - but ensure we have at least top_n results
        let mut best_genomes: Vec<OptimizerGenome> =
            sorted_population.into_iter().take(top_n).collect();

        // If we somehow don't have enough genomes, create random ones as fallback
        if best_genomes.len() < top_n {
            warn!(
                "Only found {} genomes, creating {} random fallback genomes to reach target of {}",
                best_genomes.len(),
                top_n - best_genomes.len(),
                top_n
            );
            let mut rng = StdRng::seed_from_u64(42);
            while best_genomes.len() < top_n {
                let mut fallback_genome =
                    OptimizerGenome::new_random(optimizer_type.clone(), &mut rng);
                // Try to evaluate the fallback genome
                match Self::evaluate_genome(
                    fallback_genome.to_optimizer::<S>(),
                    (&family_problems[0]).clone(),
                    self.config.clone(),
                    1,
                )
                .await
                {
                    Ok(fitness) => {
                        info!(
                            "Fallback genome evaluation succeeded: fitness = {:.6e}",
                            fitness
                        );
                        fallback_genome.fitness = Some(fitness);
                    }
                    Err(e) => {
                        warn!("Fallback genome evaluation failed: {}", e);
                        fallback_genome.fitness = Some(f32::INFINITY);
                    }
                }
                fallback_genome.id = Some(tracker.next_individual_id);
                tracker.next_individual_id += 1;
                fallback_genome.generation_created = tracker.num_generations;
                fallback_genome.generation = tracker.num_generations;
                best_genomes.push(fallback_genome);
            }
        }

        info!(
            "Selected {} best genomes with fitness values: {:?}",
            best_genomes.len(),
            best_genomes
                .iter()
                .map(|g| g.fitness.unwrap_or(f32::INFINITY))
                .collect::<Vec<_>>()
        );
        // Final check: ensure we're returning exactly top_n genomes
        assert_eq!(
            best_genomes.len(),
            top_n,
            "Must return exactly {} genomes",
            top_n
        );

        // Create selected optimizer details
        for (rank, genome) in best_genomes.iter().enumerate() {
            let generation_created = genome.generation_created;
            let parent_ids = genome.parent_ids.clone();
            let total_evaluations = genome.generation;
            if let Some(individual_id) = genome.id {
                // Representative problem for naming
                let optimizer = SelectedOptimizer {
                    problem_name: (&family_problems[0]).get_family(),
                    optimizer_name: format!(
                        "{:?}-Evolved-{}-{}",
                        genome.optimizer_type,
                        (&family_problems[0]).get_family(),
                        individual_id
                    ),
                    family: format!("{:?}", genome.optimizer_type),
                    individual_id,
                    generation_created,
                    final_fitness: genome.fitness.unwrap_or(f32::INFINITY),
                    parameters: genome.get_parameters(),
                    selection_rank: rank + 1,
                    parent_ids,
                    total_evaluations,
                };
                info!(
                    "Selected optimizer: {} (ID: {}, Fitness: {:.6e}, Rank: {})",
                    optimizer.optimizer_name,
                    optimizer.individual_id,
                    optimizer.final_fitness,
                    optimizer.selection_rank
                );
                tracker.selected_optimizers.push(optimizer);
            }
        }

        // Save the selected optimizers for this family
        let selected_filename = format!("{}/selected_optimizers.json", problem_evolution_dir);
        let selected_json = serde_json::to_string_pretty(&best_genomes)?;
        fs::write(selected_filename, selected_json)?;

        Ok(best_genomes)
    }

    async fn evaluate_population_on_family<S: Shape>(
        &self,
        population: &mut [OptimizerGenome],
        family_problems: &[ProblemSpec],
        tracker: &mut EvolutionTracker,
        generation: usize,
    ) -> anyhow::Result<()> {
        // Always evaluate all individuals to ensure fair comparison
        let total_to_evaluate = population.len();
        info!(
            "Evaluating {} individuals in generation {} on {} problems",
            total_to_evaluate,
            generation,
            family_problems.len()
        );

        let semaphore = Arc::new(Semaphore::new(8)); // Limit concurrent evaluations
        let mut tasks = Vec::new();
        let mut evaluated_count = 0;

        for (idx, genome) in population.iter().enumerate() {
            evaluated_count += 1;
            debug!(
                "Queuing evaluation for individual {} ({}/{})",
                idx, evaluated_count, total_to_evaluate
            );

            // Track evaluation event
            tracker.add_event(EvolutionaryEvent {
                generation,
                event_type: EventType::Evaluation,
                timestamp: chrono::Utc::now(),
                individual_id: genome.id.unwrap_or(idx),
                parent_ids: vec![],
                fitness_before: genome.fitness,
                fitness_after: None,
                parameters: genome.get_parameters(),
                family: format!("{:?}", genome.optimizer_type),
                details: format!(
                    "Starting fitness evaluation for individual {} on {} problems",
                    idx,
                    family_problems.len()
                ),
            });

            let semaphore = semaphore.clone();
            let optimizer = genome.to_optimizer::<S>();
            let problems = family_problems.to_vec();
            let config = self.config.clone();
            let evaluation_runs = self.evaluation_runs;

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                trace!(
                    "Starting evaluation for individual {} on problem family",
                    idx
                );

                // Evaluate on all problems in the family and average the fitness
                let mut total_fitness = 0.0;
                let mut valid_evaluations = 0;
                let mut total_success_rate = 0.0;
                let mut total_mean_value = 0.0;
                let mut total_eval_count = 0;

                for problem in problems.iter() {
                    match Self::evaluate_genome_with_metrics(
                        optimizer.clone(),
                        problem.clone(),
                        config.clone(),
                        evaluation_runs,
                    )
                    .await
                    {
                        Ok((fitness, success_rate, mean_value, eval_count)) => {
                            total_fitness += fitness;
                            total_success_rate += success_rate;
                            total_mean_value += mean_value;
                            total_eval_count += eval_count;
                            valid_evaluations += 1;
                        }
                        Err(e) => {
                            warn!(
                                "Failed to evaluate on problem {}: {}",
                                problem.get_name(),
                                e
                            );
                        }
                    }
                }

                if valid_evaluations > 0 {
                    let avg_fitness = total_fitness / valid_evaluations as f32;
                    let avg_success_rate = total_success_rate / valid_evaluations as f32;
                    let avg_mean_value = total_mean_value / valid_evaluations as f32;
                    let avg_eval_count = total_eval_count / valid_evaluations;
                    Ok((
                        idx,
                        avg_fitness,
                        avg_success_rate,
                        avg_mean_value,
                        avg_eval_count,
                    ))
                } else {
                    Err(anyhow::anyhow!(
                        "No valid evaluations for individual {}",
                        idx
                    ))
                }
            });

            tasks.push(task);
        }

        info!("Waiting for {} evaluation tasks to complete", tasks.len());
        let mut completed_count = 0;
        let mut successful_evaluations = 0;

        // Collect results
        for task in tasks {
            match task.await {
                Ok(Ok((idx, fitness, success_rate, mean_value, eval_count))) => {
                    completed_count += 1;
                    successful_evaluations += 1;
                    let genome = &population[idx];
                    let old_fitness = genome.fitness;
                    debug!(
                        "Individual {} evaluation completed ({}/{}): fitness = {:.6e}",
                        idx, completed_count, total_to_evaluate, fitness
                    );

                    // Track evaluation completion
                    tracker.add_event(EvolutionaryEvent {
                        generation,
                        event_type: EventType::Evaluation,
                        timestamp: chrono::Utc::now(),
                        individual_id: genome.id.unwrap_or(idx),
                        parent_ids: vec![],
                        fitness_before: old_fitness,
                        fitness_after: Some(fitness),
                        parameters: genome.get_parameters(),
                        family: format!("{:?}", genome.optimizer_type),
                        details: format!("Completed fitness evaluation: {:.6e}", fitness),
                    });

                    // Update individual record
                    tracker.update_individual_fitness(
                        genome.id.unwrap_or(idx),
                        generation,
                        fitness,
                    );
                    population[idx].fitness = Some(fitness);
                    population[idx].success_rate = Some(success_rate);
                    population[idx].mean_final_value = Some(mean_value);
                    population[idx].total_evaluations = Some(eval_count);
                }
                Ok(Err(e)) => {
                    completed_count += 1;
                    warn!(
                        "Failed to evaluate individual ({}/{}): {}",
                        completed_count, total_to_evaluate, e
                    );
                }
                Err(e) => {
                    completed_count += 1;
                    warn!(
                        "Evaluation task {} panicked ({}/{}): {}",
                        completed_count - 1,
                        completed_count,
                        total_to_evaluate,
                        e
                    );
                }
            }
        }

        // Ensure all genomes have fitness values
        for (idx, genome) in population.iter_mut().enumerate() {
            if genome.fitness.is_none() {
                warn!(
                    "Genome {} still without fitness after evaluation, assigning penalty",
                    idx
                );
                genome.fitness = Some(f32::INFINITY);
            }
        }

        info!(
            "Population evaluation completed: {}/{} successful evaluations",
            successful_evaluations, total_to_evaluate
        );

        Ok(())
    }

    async fn evaluate_population<S: Shape>(
        &self,
        population: &mut [OptimizerGenome],
        problem: &ProblemSpec,
        tracker: &mut EvolutionTracker,
        generation: usize,
    ) -> anyhow::Result<()> {
        // Always evaluate all individuals to ensure fair comparison
        let total_to_evaluate = population.len();
        info!(
            "Evaluating {} individuals in generation {}",
            total_to_evaluate, generation
        );

        let semaphore = Arc::new(Semaphore::new(8)); // Limit concurrent evaluations
        let mut tasks = Vec::new();
        let mut evaluated_count = 0;

        for (idx, genome) in population.iter().enumerate() {
            // Always evaluate all genomes to ensure fresh fitness values
            evaluated_count += 1;
            debug!(
                "Queuing evaluation for individual {} ({}/{})",
                idx, evaluated_count, total_to_evaluate
            );

            // Track evaluation event
            tracker.add_event(EvolutionaryEvent {
                generation,
                event_type: EventType::Evaluation,
                timestamp: chrono::Utc::now(),
                individual_id: genome.id.unwrap_or(idx),
                parent_ids: vec![],
                fitness_before: genome.fitness,
                fitness_after: None,
                parameters: genome.get_parameters(),
                family: format!("{:?}", genome.optimizer_type),
                details: format!("Starting fitness evaluation for individual {}", idx),
            });

            let semaphore = semaphore.clone();
            let optimizer = genome.to_optimizer::<S>();
            let problem = problem.clone();
            let config = self.config.clone();
            let evaluation_runs = self.evaluation_runs;

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                trace!("Starting evaluation for individual {}", idx);
                Self::evaluate_genome(optimizer, problem, config, evaluation_runs)
                    .await
                    .map(|fitness| (idx, fitness))
            });

            tasks.push(task);
        }
        info!("Waiting for {} evaluation tasks to complete", tasks.len());
        let mut completed_count = 0;
        let mut successful_evaluations = 0;

        // Collect results
        for task in tasks {
            match task.await {
                Ok(Ok((idx, fitness))) => {
                    completed_count += 1;
                    successful_evaluations += 1;
                    let genome = &population[idx];
                    let old_fitness = genome.fitness;
                    debug!(
                        "Individual {} evaluation completed ({}/{}): fitness = {:.6e}",
                        idx, completed_count, total_to_evaluate, fitness
                    );

                    // Track evaluation completion
                    tracker.add_event(EvolutionaryEvent {
                        generation,
                        event_type: EventType::Evaluation,
                        timestamp: chrono::Utc::now(),
                        individual_id: genome.id.unwrap_or(idx),
                        parent_ids: vec![],
                        fitness_before: old_fitness,
                        fitness_after: Some(fitness),
                        parameters: genome.get_parameters(),
                        family: format!("{:?}", genome.optimizer_type),
                        details: format!("Completed fitness evaluation: {:.6e}", fitness),
                    });

                    // Update individual record
                    tracker.update_individual_fitness(
                        genome.id.unwrap_or(idx),
                        generation,
                        fitness,
                    );
                    population[idx].fitness = Some(fitness);
                }
                Ok(Err(e)) => {
                    completed_count += 1;
                    // Note : we can't get the idx here
                    warn!(
                        "Failed to evaluate individual ({}/{}): {}",
                        completed_count, total_to_evaluate, e
                    );
                    // Assign worst fitness to failed evaluations
                    // population[idx].fitness = Some(f32::INFINITY);
                }
                Err(e) => {
                    completed_count += 1;
                    warn!(
                        "Evaluation task {} panicked ({}/{}): {}",
                        completed_count - 1,
                        completed_count,
                        total_to_evaluate,
                        e
                    );
                    // Assign worst fitness to panicked evaluations
                    // Note: we can't get the idx here, but this is rare
                }
            }
        }
        // Ensure all genomes have fitness values
        for (idx, genome) in population.iter_mut().enumerate() {
            if genome.fitness.is_none() {
                warn!(
                    "Genome {} still without fitness after evaluation, assigning penalty",
                    idx
                );
                genome.fitness = Some(f32::INFINITY);
            }
        }

        info!(
            "Population evaluation completed: {}/{} successful evaluations",
            successful_evaluations, total_to_evaluate
        );

        Ok(())
    }
    fn track_initial_population(
        &self,
        tracker: &mut EvolutionTracker,
        population: &[OptimizerGenome],
    ) {
        info!(
            "Tracking initial population of {} individuals",
            population.len()
        );
        let mut family_counts = std::collections::HashMap::new();

        for (idx, genome) in population.iter().enumerate() {
            let family = format!("{:?}", genome.optimizer_type);
            *family_counts.entry(family.clone()).or_insert(0) += 1;

            let individual_id = tracker.next_individual_id;
            tracker.next_individual_id += 1;
            trace!(
                "Creating individual record {} for family {}",
                individual_id,
                family
            );
            // Note: We can't modify the genome here since it's borrowed immutably
            // The ID assignment should happen in evolve_for_problem after initialization

            // Create individual record
            let individual = IndividualRecord {
                id: individual_id,
                generation_created: 0,
                family: family.clone(),
                parameters: genome.get_parameters(),
                fitness_history: vec![],
                parent_ids: vec![],
                offspring_ids: vec![],
                selection_count: 0,
                mutation_count: 0,
                crossover_count: 0,
                is_elite: false,
                final_generation: None,
            };
            tracker.individuals.insert(individual_id, individual);

            // Track initialization event
            tracker.add_event(EvolutionaryEvent {
                generation: 0,
                event_type: EventType::Initialization,
                timestamp: chrono::Utc::now(),
                individual_id,
                parent_ids: vec![],
                fitness_before: None,
                fitness_after: None,
                parameters: genome.get_parameters(),
                family: family.clone(),
                details: format!(
                    "Initial individual created with {} parameters",
                    genome.get_parameters().len()
                ),
            });

            // Update family representation
            tracker
                .family_representations
                .entry(family.to_string())
                .or_insert_with(|| {
                    FamilyRepresentation::new(
                        &family,
                        self.family_proportions.get(&family).copied().unwrap_or(0.1),
                    )
                })
                .total_created += 1;
        }
        info!(
            "Initial population family distribution: {:?}",
            family_counts
        );
        debug!(
            "Initial population tracking completed with {} events",
            tracker.events.len()
        );
    }
    fn track_generation_summary(
        &self,
        tracker: &mut EvolutionTracker,
        population: &[OptimizerGenome],
        generation: usize,
    ) {
        let fitnesses: Vec<f32> = population.iter().filter_map(|g| g.fitness).collect();
        if fitnesses.is_empty() {
            return;
        }
        let best_fitness = fitnesses.iter().cloned().fold(f32::INFINITY, f32::min);
        let worst_fitness = fitnesses.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let average_fitness = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;
        let fitness_variance = fitnesses
            .iter()
            .map(|f| (f - average_fitness).powi(2))
            .sum::<f32>()
            / fitnesses.len() as f32;
        let fitness_std = fitness_variance.sqrt();
        // Count families
        let mut family_counts = HashMap::new();
        for genome in population {
            let family = format!("{:?}", genome.optimizer_type);
            *family_counts.entry(family).or_insert(0) += 1;
        }
        // Calculate diversity metrics
        let diversity_metrics = self.calculate_diversity_metrics(population);
        let summary = GenerationSummary {
            generation,
            best_fitness,
            worst_fitness,
            average_fitness,
            fitness_std,
            family_counts,
            diversity_metrics,
            selection_pressure: worst_fitness - best_fitness,
        };
        tracker.generation_summaries.push(summary);
    }
    fn calculate_diversity_metrics(&self, population: &[OptimizerGenome]) -> DiversityMetrics {
        let mut parameter_variances = HashMap::new();
        // Get all parameter names
        let all_params: std::collections::HashSet<String> = population
            .iter()
            .flat_map(|g| {
                g.get_parameters()
                    .keys()
                    .clone()
                    .map(|s| s.to_string())
                    .collect_vec()
            })
            .collect();
        // Calculate variance for each parameter
        for param_name in all_params {
            let values: Vec<f32> = population
                .iter()
                .filter_map(|g| g.get_parameters().get(&param_name).copied())
                .collect();
            if values.len() > 1 {
                let mean = values.iter().sum::<f32>() / values.len() as f32;
                let variance =
                    values.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / values.len() as f32;
                parameter_variances.insert(param_name, variance);
            }
        }
        // Calculate fitness diversity
        let fitnesses: Vec<f32> = population.iter().filter_map(|g| g.fitness).collect();
        let fitness_diversity = if fitnesses.len() > 1 {
            let mean = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;
            fitnesses.iter().map(|f| (f - mean).powi(2)).sum::<f32>() / fitnesses.len() as f32
        } else {
            0.0
        };
        // Calculate family diversity (Shannon entropy)
        let mut family_counts = HashMap::new();
        for genome in population {
            let family = format!("{:?}", genome.optimizer_type);
            *family_counts.entry(family).or_insert(0) += 1;
        }
        let total = population.len() as f32;
        let family_diversity = family_counts
            .values()
            .map(|&count| {
                let p = count as f32 / total;
                if p > 0.0 {
                    -p * p.ln()
                } else {
                    0.0
                }
            })
            .sum::<f32>();
        DiversityMetrics {
            parameter_variance: parameter_variances,
            fitness_diversity,
            family_diversity,
        }
    }
    fn maintain_family_representation(
        &self,
        population: &mut [OptimizerGenome],
        tracker: &mut EvolutionTracker,
        generation: usize,
    ) {
        // Count current family representation
        let mut family_counts = HashMap::new();
        for genome in population.iter() {
            let family = format!("{:?}", genome.optimizer_type);
            *family_counts.entry(family).or_insert(0) += 1;
        }
        // Check if any family is underrepresented
        for (family, target_proportion) in &self.family_proportions {
            let target_count = (self.population_size as f32 * target_proportion) as usize;
            let current_count = family_counts.get(family).copied().unwrap_or(0);
            let min_count = std::cmp::max(self.min_family_representation, target_count / 2);
            if current_count < min_count {
                // Force representation by replacing worst performers from over-represented families
                tracker.add_event(EvolutionaryEvent {
                    generation,
                    event_type: EventType::FamilyBalancing,
                    timestamp: chrono::Utc::now(),
                    individual_id: 0, // Will be updated
                    parent_ids: vec![],
                    fitness_before: None,
                    fitness_after: None,
                    parameters: HashMap::new(),
                    family: family.clone(),
                    details: format!(
                        "Family {} underrepresented: {}/{} (min: {})",
                        family, current_count, target_count, min_count
                    ),
                });
            }
        }
    }
    fn evolve_generation_with_tracking(
        &self,
        evolution: &mut ParameterEvolution,
        population: &mut Vec<OptimizerGenome>,
        tracker: &mut EvolutionTracker,
        generation: usize,
    ) -> Vec<OptimizerGenome> {
        debug!(
            "Starting evolution to generation {} with tracking",
            generation
        );

        // Track selection events
        debug!("Selecting parents for generation {}", generation);
        let selected_indices = evolution.select_parents(population);
        info!(
            "Selected {} parents for generation {}",
            selected_indices.len(),
            generation
        );

        for &idx in &selected_indices {
            if let Some(individual_id) = population[idx].id {
                trace!("Individual {} selected for reproduction", individual_id);
                tracker.add_event(EvolutionaryEvent {
                    generation,
                    event_type: EventType::Selection,
                    timestamp: chrono::Utc::now(),
                    individual_id,
                    parent_ids: vec![],
                    fitness_before: population[idx].fitness,
                    fitness_after: None,
                    parameters: population[idx].get_parameters(),
                    family: format!("{:?}", population[idx].optimizer_type),
                    details: "Selected for reproduction".to_string(),
                });
                if let Some(individual) = tracker.individuals.get_mut(&individual_id) {
                    individual.selection_count += 1;
                }
            }
        }
        // Perform evolution with tracking
        debug!(
            "Performing crossover and mutation for generation {}",
            generation
        );
        let mut new_population = evolution.evolve_generation(population, generation);
        info!(
            "Generated {} new individuals for generation {}",
            new_population.len(),
            generation
        );

        // Track new individuals (this is simplified - in practice you'd track crossover/mutation details)
        let mut new_individuals_count = 0;
        for (idx, genome) in new_population.iter_mut().enumerate() {
            if genome.id.is_none() {
                // This is a new individual
                new_individuals_count += 1;
                let individual_id = tracker.next_individual_id;
                tracker.next_individual_id += 1;
                genome.id = Some(individual_id); // Assign ID to the genome
                trace!(
                    "Creating new individual {} in generation {}",
                    individual_id,
                    generation
                );

                let individual = IndividualRecord {
                    id: individual_id,
                    generation_created: generation,
                    family: format!("{:?}", genome.optimizer_type),
                    parameters: genome.get_parameters(),
                    fitness_history: vec![],
                    parent_ids: vec![], // Would be filled with actual parent tracking
                    offspring_ids: vec![],
                    selection_count: 0,
                    mutation_count: 0,
                    crossover_count: 0,
                    is_elite: false,
                    final_generation: None,
                };
                tracker.individuals.insert(individual_id, individual);
            }
        }
        info!(
            "Evolution to generation {} completed: {} new individuals created",
            generation, new_individuals_count
        );
        new_population
    }

    async fn evaluate_genome_with_metrics<S: Shape>(
        optimizer: Arc<dyn Optimizer<S>>,
        problem: ProblemSpec,
        config: BenchmarkConfig,
        num_runs: usize,
    ) -> anyhow::Result<(f32, f32, f32, usize)> {
        trace!(
            "Starting genome evaluation with metrics for {} runs",
            num_runs
        );
        let runner = BenchmarkRunner::new(config.clone());
        let mut total_fitness = 0.0;
        let mut successful_runs = 0;
        let mut total_final_value = 0.0;
        let mut total_iterations = 0;
        let mut rng = StdRng::seed_from_u64(42);

        for run_id in 0..num_runs {
            trace!("Starting evaluation run {}/{}", run_id + 1, num_runs);
            let result = runner
                .run_single_benchmark(
                    &problem,
                    &mut optimizer.clone_box(),
                    run_id,
                    "eval",
                    new_initial_point(&problem, config.initial_point_noise, &mut rng),
                )
                .await?;

            total_iterations += result.iterations;

            // Fitness is combination of final value and convergence speed
            let fitness = if result.best_value.is_finite() {
                total_final_value += result.best_value;
                let value_component = result.best_value.abs().ln().max(-20.0);
                let speed_component = (result.iterations as f32) / (config.max_iterations as f32);
                value_component + 0.1 * speed_component
            } else {
                trace!("Run {} failed to converge", run_id);
                1e10 // Penalty for non-convergence
            };

            if fitness < 1e9 {
                total_fitness += fitness;
                successful_runs += 1;
                trace!("Run {} successful: fitness = {:.6e}", run_id, fitness);
            } else {
                trace!("Run {} failed: fitness = {:.6e}", run_id, fitness);
            }
        }

        // Calculate metrics
        let final_fitness = if successful_runs > 0 {
            total_fitness / (successful_runs as f32) + (num_runs - successful_runs) as f32 * 100.0
        } else {
            f32::INFINITY
        };

        let success_rate = successful_runs as f32 / num_runs as f32;
        let mean_final_value = if successful_runs > 0 {
            total_final_value / successful_runs as f32
        } else {
            f32::INFINITY
        };
        let avg_evaluations = total_iterations / num_runs.max(1);

        debug!(
            "Genome evaluation completed: {}/{} successful runs, fitness: {:.6e}, success_rate: {:.2}%, mean_value: {:.6e}",
            successful_runs, num_runs, final_fitness, success_rate * 100.0, mean_final_value
        );

        Ok((
            final_fitness,
            success_rate,
            mean_final_value,
            avg_evaluations,
        ))
    }

    async fn evaluate_genome<S: Shape>(
        optimizer: Arc<dyn Optimizer<S>>,
        problem: ProblemSpec,
        config: BenchmarkConfig,
        num_runs: usize,
    ) -> anyhow::Result<f32> {
        trace!("Starting genome evaluation with {} runs", num_runs);
        let runner = BenchmarkRunner::new(config.clone());
        let mut total_fitness = 0.0;
        let mut successful_runs = 0;
        let mut total_final_value = 0.0;
        let mut total_iterations = 0;
        let mut rng = StdRng::seed_from_u64(42);

        for run_id in 0..num_runs {
            trace!("Starting evaluation run {}/{}", run_id + 1, num_runs);
            let result = runner
                .run_single_benchmark(
                    &problem,
                    &mut optimizer.clone_box(),
                    run_id,
                    "eval",
                    new_initial_point(&problem, config.initial_point_noise, &mut rng),
                )
                .await?;
            total_iterations += result.iterations;

            // Fitness is combination of final value and convergence speed
            let fitness = if result.best_value.is_finite() {
                total_final_value += result.best_value;
                let value_component = result.best_value.abs().ln().max(-20.0);
                let speed_component = (result.iterations as f32) / (config.max_iterations as f32);
                value_component + 0.1 * speed_component
            } else {
                trace!("Run {} failed to converge", run_id);
                1e10 // Penalty for non-convergence
            };

            if fitness < 1e9 {
                total_fitness += fitness;
                successful_runs += 1;
                trace!("Run {} successful: fitness = {:.6e}", run_id, fitness);
            } else {
                trace!("Run {} failed: fitness = {:.6e}", run_id, fitness);
            }
        }

        // Return average fitness, with penalty for failed runs
        let final_fitness = if successful_runs > 0 {
            total_fitness / (successful_runs as f32) + (num_runs - successful_runs) as f32 * 100.0
        } else {
            f32::INFINITY
        };

        debug!(
            "Genome evaluation completed: {}/{} successful runs, final fitness: {:.6e}",
            successful_runs, num_runs, final_fitness
        );
        Ok(final_fitness)
    }

    fn save_detailed_generation_results(
        &self,
        population: &[OptimizerGenome],
        tracker: &EvolutionTracker,
        generation: usize,
        output_dir: &str,
    ) -> anyhow::Result<()> {
        debug!("Saving detailed results for generation {}", generation);

        // Save population state
        let pop_filename = format!(
            "{}/generation_{:03}_population.json",
            output_dir, generation
        );
        trace!("Saving population to: {}", pop_filename);
        let pop_json = serde_json::to_string_pretty(population)?;
        fs::write(pop_filename, pop_json)?;

        // Save generation summary
        if let Some(summary) = tracker.generation_summaries.get(generation) {
            let summary_filename =
                format!("{}/generation_{:03}_summary.json", output_dir, generation);
            trace!("Saving summary to: {}", summary_filename);
            let summary_json = serde_json::to_string_pretty(summary)?;
            fs::write(summary_filename, summary_json)?;
        }

        // Save events for this generation
        let generation_events: Vec<_> = tracker
            .events
            .iter()
            .filter(|e| e.generation == generation)
            .collect();
        let events_filename = format!("{}/generation_{:03}_events.json", output_dir, generation);
        trace!(
            "Saving {} events to: {}",
            generation_events.len(),
            events_filename
        );
        let events_json = serde_json::to_string_pretty(&generation_events)?;
        fs::write(events_filename, events_json)?;
        debug!("Generation {} results saved successfully", generation);

        Ok(())
    }

    fn save_comprehensive_evolution_report(
        &self,
        tracker: &EvolutionTracker,
        output_dir: &str,
    ) -> anyhow::Result<()> {
        info!("Saving comprehensive evolution report to {}", output_dir);

        // Save complete tracker data
        let tracker_filename = format!("{}/complete_evolution_data.json", output_dir);
        debug!("Saving complete tracker data to: {}", tracker_filename);
        let tracker_json = serde_json::to_string_pretty(tracker)?;
        fs::write(tracker_filename, tracker_json)?;

        // Generate detailed HTML report
        debug!("Generating detailed HTML report");
        let html_report = self.generate_detailed_evolution_html_report(tracker)?;
        let html_filename = format!("{}/detailed_evolution_report.html", output_dir);
        debug!("Saving HTML report to: {}", html_filename);
        fs::write(html_filename, html_report)?;

        // Generate CSV exports for analysis
        debug!("Generating CSV exports");
        self.save_evolution_csv_exports(tracker, output_dir)?;
        info!("Comprehensive evolution report saved successfully");

        Ok(())
    }

    fn save_selected_optimizers_csv(
        &self,
        selected_optimizers: &[SelectedOptimizer],
        output_dir: &str,
    ) -> anyhow::Result<()> {
        let mut csv_content = String::from("Problem,OptimizerName,Family,Rank,FinalFitness,GenerationCreated,TotalEvaluations,Parameters\n");
        for selected in selected_optimizers {
            let params_str = selected
                .parameters
                .iter()
                .map(|(k, v)| format!("{}={:.6}", k, v))
                .collect::<Vec<_>>()
                .join(";");
            csv_content.push_str(&format!(
                "{},{},{},{},{:.6e},{},{},\"{}\"\n",
                selected.problem_name,
                selected.optimizer_name,
                selected.family,
                selected.selection_rank,
                selected.final_fitness,
                selected.generation_created,
                selected.total_evaluations,
                params_str
            ));
        }
        let csv_path = format!("{}/selected_optimizers.csv", output_dir);
        fs::write(csv_path.to_string(), csv_content)?;
        info!("Saved selected optimizers CSV to {}", csv_path);
        Ok(())
    }

    /// Run final championship with evolved optimizers
    pub async fn run_evolved_championship<S: Shape>(
        &self,
        problems: Vec<ProblemSpec>,
        evolved_optimizers: HashMap<String, Vec<(String, Arc<dyn Optimizer<S>>)>>,
    ) -> anyhow::Result<()> {
        info!("Running championship with evolved optimizers");
        info!("Championship includes {} problems", problems.len());

        // For each problem, run benchmarks with its evolved optimizers
        // for (problem_idx, problem) in problems.iter().enumerate() {
        //     let problem_name = problem.get_name();
        //     info!(
        //         "Running championship for problem {}/{}: {}",
        //         problem_idx + 1,
        //         problems.len(),
        //         problem_name
        //     );
        //
        //     if let Some(optimizers) = evolved_optimizers.get(&problem_name) {
        //         info!(
        //             "Running championship for {} with {} evolved optimizers",
        //             problem_name,
        //             optimizers.len()
        //         );
        //         for (opt_idx, (opt_name, _)) in optimizers.iter().enumerate() {
        //             debug!(
        //                 "Championship optimizer {}/{}: {}",
        //                 opt_idx + 1,
        //                 optimizers.len(),
        //                 opt_name
        //             );
        //         }
        //
        //         // Run comparative benchmarks for this problem
        //         debug!("Starting comparative benchmarks for {}", problem_name);
        //         let mut runner: ExperimentRunner = self.base_runner.clone();
        //         runner.output_dir = format!("{}/championship/{}", self.base_runner.output_dir, problem_name.replace(" ", "_"));
        //         runner.report_generator.output_dir = runner.output_dir.clone();
        //         runner.plotting_manager.output_dir = runner.output_dir.clone();
        //         runner.run_comparative_benchmarks(vec![problem.clone()], optimizers.clone()).await?;
        //         info!("Championship completed for problem: {}", problem_name);
        //     } else {
        //         warn!("No evolved optimizers found for problem: {}", problem_name);
        //     }
        // }

        // Run comparative benchmarks for this problem
        let mut runner: ExperimentRunner = self.base_runner.clone();
        runner
            .run_comparative_benchmarks(
                problems,
                evolved_optimizers
                    .values()
                    .flatten()
                    .map(|x| (x.0.to_string(), x.1.clone()))
                    .collect_vec(),
            )
            .await?;

        info!("All championship benchmarks completed successfully");

        Ok(())
    }
    fn generate_detailed_evolution_html_report(
        &self,
        tracker: &EvolutionTracker,
    ) -> anyhow::Result<String> {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Detailed Evolution Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        .family-section { margin: 30px 0; padding: 20px; border: 1px solid #ccc; }
        .event-log { max-height: 400px; overflow-y: scroll; }
        .generation-summary { background-color: #f9f9f9; padding: 15px; margin: 10px 0; }
    </style>
</head>
<body>
"#,
        );
        html.push_str(&format!(
            r#"<h1>Detailed Evolution Report: {}</h1>
<p><strong>Population Size:</strong> {}</p>
<p><strong>Generations:</strong> {}</p>
<p><strong>Total Individuals Created:</strong> {}</p>
<p><strong>Total Events:</strong> {}</p>
"#,
            tracker.problem_name,
            tracker.population_size,
            tracker.num_generations,
            tracker.individuals.len(),
            tracker.events.len()
        ));
        // Add selected optimizers section
        if !tracker.selected_optimizers.is_empty() {
            html.push_str("<h2>Selected Optimizers</h2>");
            html.push_str("<table>");
            html.push_str("<tr><th>Rank</th><th>Name</th><th>Family</th><th>Fitness</th><th>Generation</th><th>Parameters</th></tr>");
            for selected in &tracker.selected_optimizers {
                let params_html = selected
                    .parameters
                    .iter()
                    .map(|(k, v)| format!("<b>{}:</b> {:.4}", k, v))
                    .collect::<Vec<_>>()
                    .join("<br>");
                html.push_str(&format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.2e}</td><td>{}</td><td>{}</td></tr>",
                    selected.selection_rank,
                    selected.optimizer_name,
                    selected.family,
                    selected.final_fitness,
                    selected.generation_created,
                    params_html
                ));
            }
            html.push_str("</table>");
        }

        // Family representation summary
        html.push_str("<h2>Family Representation Summary</h2>");
        html.push_str("<table><tr><th>Family</th><th>Target %</th><th>Total Created</th><th>Best Fitness</th><th>Avg Fitness</th></tr>");
        for (family_name, family_data) in &tracker.family_representations {
            html.push_str(&format!(
                "<tr><td>{}</td><td>{:.1}%</td><td>{}</td><td>{:.2e}</td><td>{:.2e}</td></tr>",
                family_name,
                family_data.target_proportion * 100.0,
                family_data.total_created,
                family_data.best_fitness.unwrap_or(f32::INFINITY),
                family_data.average_fitness
            ));
        }
        html.push_str("</table>");
        // Generation summaries
        html.push_str("<h2>Generation Summaries</h2>");
        for summary in &tracker.generation_summaries {
            html.push_str(&format!(
                r#"<div class="generation-summary">
<h3>Generation {}</h3>
<p><strong>Best Fitness:</strong> {:.2e}</p>
<p><strong>Average Fitness:</strong> {:.2e}</p>
<p><strong>Fitness Std:</strong> {:.2e}</p>
<p><strong>Family Diversity:</strong> {:.3}</p>
<p><strong>Family Counts:</strong> {}</p>
</div>"#,
                summary.generation,
                summary.best_fitness,
                summary.average_fitness,
                summary.fitness_std,
                summary.diversity_metrics.family_diversity,
                summary
                    .family_counts
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        // Individual details by family
        for (family_name, family_data) in &tracker.family_representations {
            html.push_str(&format!(
                r#"<div class="family-section">
<h2>Family: {}</h2>"#,
                family_name
            ));
            let family_individuals: Vec<_> = tracker
                .individuals
                .values()
                .filter(|ind| ind.family == *family_name)
                .collect();
            html.push_str("<table>");
            html.push_str("<tr><th>ID</th><th>Generation</th><th>Parameters</th><th>Best Fitness</th><th>Selections</th><th>Mutations</th><th>Elite</th></tr>");
            for individual in family_individuals {
                let best_fitness = individual
                    .fitness_history
                    .iter()
                    .map(|(_, f)| *f)
                    .fold(f32::INFINITY, f32::min);
                let params_str = individual
                    .parameters
                    .iter()
                    .map(|(k, v)| format!("{}: {:.3}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                html.push_str(&format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.2e}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    individual.id,
                    individual.generation_created,
                    params_str,
                    best_fitness,
                    individual.selection_count,
                    individual.mutation_count,
                    individual.is_elite
                ));
            }
            html.push_str("</table>");
            html.push_str("</div>");
        }
        // Event log
        html.push_str(r#"<h2>Event Log</h2>
<div class="event-log">
<table>
<tr><th>Generation</th><th>Event</th><th>Individual</th><th>Family</th><th>Details</th><th>Fitness Change</th></tr>"#);
        for event in &tracker.events {
            let fitness_change = match (event.fitness_before, event.fitness_after) {
                (Some(before), Some(after)) => format!("{:.2e} → {:.2e}", before, after),
                (None, Some(after)) => format!("→ {:.2e}", after),
                _ => "N/A".to_string(),
            };
            html.push_str(&format!(
                "<tr><td>{}</td><td>{:?}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                event.generation,
                event.event_type,
                event.individual_id,
                event.family,
                event.details,
                fitness_change
            ));
        }
        html.push_str("</table></div>");
        html.push_str("</body></html>");
        Ok(html)
    }
    fn save_evolution_csv_exports(
        &self,
        tracker: &EvolutionTracker,
        output_dir: &str,
    ) -> anyhow::Result<()> {
        // Export individuals
        let mut individuals_csv = String::from(
            "ID,Family,Generation,BestFitness,Selections,Mutations,Crossovers,IsElite,Parameters\n",
        );
        for individual in tracker.individuals.values() {
            let best_fitness = individual
                .fitness_history
                .iter()
                .map(|(_, f)| *f)
                .fold(f32::INFINITY, f32::min);
            let params_str = individual
                .parameters
                .iter()
                .map(|(k, v)| format!("{}:{:.6}", k, v))
                .collect::<Vec<_>>()
                .join(";");
            individuals_csv.push_str(&format!(
                "{},{},{},{:.6e},{},{},{},{},\"{}\"\n",
                individual.id,
                individual.family,
                individual.generation_created,
                best_fitness,
                individual.selection_count,
                individual.mutation_count,
                individual.crossover_count,
                individual.is_elite,
                params_str
            ));
        }
        fs::write(format!("{}/individuals.csv", output_dir), individuals_csv)?;
        // Export events
        let mut events_csv = String::from(
            "Generation,EventType,IndividualID,Family,FitnessBefore,FitnessAfter,Details\n",
        );
        for event in &tracker.events {
            events_csv.push_str(&format!(
                "{},{:?},{},{},{},{},\"{}\"\n",
                event.generation,
                event.event_type,
                event.individual_id,
                event.family,
                event
                    .fitness_before
                    .map(|f| format!("{:.6e}", f))
                    .unwrap_or("N/A".to_string()),
                event
                    .fitness_after
                    .map(|f| format!("{:.6e}", f))
                    .unwrap_or("N/A".to_string()),
                event.details
            ));
        }
        fs::write(format!("{}/events.csv", output_dir), events_csv)?;
        // Export generation summaries
        let mut summaries_csv = String::from(
            "Generation,BestFitness,AvgFitness,FitnessStd,FamilyDiversity,FamilyCounts\n",
        );
        for summary in &tracker.generation_summaries {
            let family_counts_str = summary
                .family_counts
                .iter()
                .map(|(k, v)| format!("{}:{}", k, v))
                .collect::<Vec<_>>()
                .join(";");
            summaries_csv.push_str(&format!(
                "{},{:.6e},{:.6e},{:.6e},{:.6},\"{}\"\n",
                summary.generation,
                summary.best_fitness,
                summary.average_fitness,
                summary.fitness_std,
                summary.diversity_metrics.family_diversity,
                family_counts_str
            ));
        }
        fs::write(
            format!("{}/generation_summaries.csv", output_dir),
            summaries_csv,
        )?;
        Ok(())
    }
}
impl EvolutionTracker {
    pub fn new(
        problem_name: String,
        population_size: usize,
        num_generations: usize,
        optimizer_types: &[super::parameter_evolution::OptimizerType],
        family_proportions: &HashMap<String, f32>,
    ) -> Self {
        let mut family_representations = HashMap::new();
        // Initialize family representations
        for optimizer_type in optimizer_types {
            let family_name = format!("{:?}", optimizer_type);
            let target_proportion = family_proportions.get(&family_name).copied().unwrap_or(0.1);
            family_representations.insert(
                family_name.clone(),
                FamilyRepresentation::new(&family_name, target_proportion),
            );
        }
        Self {
            problem_name,
            population_size,
            num_generations,
            events: Vec::new(),
            individuals: HashMap::new(),
            family_representations,
            generation_summaries: Vec::new(),
            next_individual_id: 0,
            selected_optimizers: Vec::new(),
        }
    }
    pub fn add_event(&mut self, event: EvolutionaryEvent) {
        self.events.push(event);
    }

    pub fn update_individual_fitness(
        &mut self,
        individual_id: usize,
        generation: usize,
        fitness: f32,
    ) {
        if let Some(individual) = self.individuals.get_mut(&individual_id) {
            individual.fitness_history.push((generation, fitness));
            // Update family representation stats
            if let Some(family_rep) = self.family_representations.get_mut(&individual.family) {
                if family_rep.best_fitness.is_none() || fitness < family_rep.best_fitness.unwrap() {
                    family_rep.best_fitness = Some(fitness);
                }
                if family_rep.worst_fitness.is_none() || fitness > family_rep.worst_fitness.unwrap()
                {
                    family_rep.worst_fitness = Some(fitness);
                }
            }
        }
    }
}
impl FamilyRepresentation {
    pub fn new(family_name: &str, target_proportion: f32) -> Self {
        Self {
            family_name: family_name.to_string(),
            target_proportion,
            current_count: 0,
            total_created: 0,
            best_fitness: None,
            worst_fitness: None,
            average_fitness: f32::INFINITY,
            generations_dominated: Vec::new(),
        }
    }
}

/// Convenience function to run adaptive evolution experiments
pub async fn run_adaptive_benchmark<S: Shape>(
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
    info!("Starting adaptive benchmark");
    info!("Parameters: max_evals={}, num_runs={}, population_size={}, num_generations={}, evaluation_runs={}", 
          max_evals, num_runs, population_size, num_generations, evaluation_runs);
    info!("Time limit: {:?}", time_limit);
    info!(
        "Problems: {}",
        problems
            .iter()
            .map(|p| p.get_name())
            .collect::<Vec<_>>()
            .join(", ")
    );
    info!("Optimizer types: {:?}", optimizer_types);

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("{report_path_prefix}adaptive_{timestamp}");
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(output_dir.to_str().unwrap().to_string())?;

    info!(
        "Creating adaptive benchmark results in: {}",
        output_dir.display()
    );

    let config = BenchmarkConfig {
        max_iterations: max_evals,
        maximum_function_calls: max_evals,
        time_limit: DurationWrapper::from(time_limit),
        num_runs,
        ..BenchmarkConfig::default()
    };
    debug!("Benchmark config created: {:?}", config);

    let mut runner = AdaptiveExperimentRunner::new(
        output_dir.to_string_lossy().to_string(),
        config,
        population_size,
        num_generations,
        evaluation_runs,
        Some(8),
    );
    info!("Adaptive experiment runner created");

    // First, evolve optimizer parameters for each problem
    info!("Starting parameter evolution phase");
    let evolved_optimizers = runner
        .run_adaptive_evolution::<S>(problems.clone(), optimizer_types)
        .await?;
    info!("Parameter evolution phase completed");

    // Then run final championship with evolved optimizers
    info!("Starting championship phase");
    runner
        .run_evolved_championship(problems, evolved_optimizers)
        .await?;
    info!("Championship phase completed");

    info!("Adaptive benchmark completed successfully");
    info!("Results saved to: {}", output_dir.display());

    Ok(())
}