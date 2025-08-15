use crate::optimizers::{GDConfig, GDOptimizer, TrustRegionConfig, TrustRegionOptimizer};
use crate::{
    AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod,
    Optimizer, QQNConfig, QQNOptimizer,
};
use log::{debug, info, trace, warn};
use plotters::prelude::LogScalable;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Represents a genome for an optimizer configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptimizerGenome {
    pub optimizer_type: OptimizerType,
    pub parameters: HashMap<String, f64>,
    pub fitness: Option<f64>,
    pub id: Option<usize>,
    pub generation_created: usize,
    pub parent_ids: Vec<usize>,
    pub generation: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OptimizerType {
    QQN,
    LBFGS,
    Adam,
    GD,
    TrustRegion,
}
impl fmt::Display for OptimizerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizerType::QQN => write!(f, "QQN"),
            OptimizerType::LBFGS => write!(f, "L-BFGS"),
            OptimizerType::Adam => write!(f, "Adam"),
            OptimizerType::GD => write!(f, "GD"),
            OptimizerType::TrustRegion => write!(f, "TrustRegion"),
        }
    }
}

impl OptimizerGenome {
    pub fn get_parameters(&self) -> std::collections::HashMap<String, f64> {
        self.parameters.clone()
    }

    pub fn new_random(optimizer_type: OptimizerType, rng: &mut StdRng) -> Self {
        debug!(
            "Creating new random genome for optimizer type: {}",
            optimizer_type
        );
        let parameters = match optimizer_type {
            OptimizerType::QQN => Self::random_qqn_params(rng),
            OptimizerType::LBFGS => Self::random_lbfgs_params(rng),
            OptimizerType::Adam => Self::random_adam_params(rng),
            OptimizerType::GD => Self::random_gd_params(rng),
            OptimizerType::TrustRegion => Self::random_trust_region_params(rng),
        };
        trace!(
            "Generated parameters for {}: {:?}",
            optimizer_type,
            parameters
        );
        Self {
            optimizer_type,
            parameters,
            fitness: None,
            id: None,
            generation_created: 0,
            parent_ids: vec![],
            generation: 0,
        }
    }

    fn random_qqn_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("c1".to_string(), rng.gen_range(1e-6..1e-2));
        params.insert("c2".to_string(), rng.gen_range(0.1..0.99));
        params.insert("lbfgs_history".to_string(), rng.gen_range(3.0..20.0));
        params.insert(
            "epsilon".to_string(),
            10f64.powf(rng.gen_range(-10.0..-4.0)),
        );
        params.insert("initial_step".to_string(), rng.gen_range(0.1..2.0));
        params.insert("max_iterations".to_string(), rng.gen_range(10.0..50.0));
        params.insert(
            "line_search_method".to_string(),
            rng.gen_range(0.0..6.0).as_f64().floor(),
        );
        params
    }

    fn random_lbfgs_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("history_size".to_string(), rng.gen_range(3.0..30.0));
        params.insert("c1".to_string(), rng.gen_range(1e-6..1e-2));
        params.insert("c2".to_string(), rng.gen_range(0.1..0.99));
        params.insert(
            "epsilon".to_string(),
            10f64.powf(rng.gen_range(-12.0..-6.0)),
        );
        params.insert("max_step_size".to_string(), rng.gen_range(0.5..10.0));
        params.insert("initial_step".to_string(), rng.gen_range(0.01..2.0));
        params
    }

    fn random_adam_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert(
            "learning_rate".to_string(),
            10f64.powf(rng.gen_range(-4.0..0.0)),
        );
        params.insert("beta1".to_string(), rng.gen_range(0.8..0.99));
        params.insert("beta2".to_string(), rng.gen_range(0.9..0.9999));
        params.insert(
            "epsilon".to_string(),
            10f64.powf(rng.gen_range(-10.0..-6.0)),
        );
        params.insert("weight_decay".to_string(), rng.gen_range(0.0..1e-3));
        params
    }

    fn random_gd_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert(
            "learning_rate".to_string(),
            10f64.powf(rng.gen_range(-3.0..0.0)),
        );
        params.insert("momentum".to_string(), rng.gen_range(0.0..0.99));
        params.insert("weight_decay".to_string(), rng.gen_range(0.0..1e-3));
        params.insert(
            "nesterov".to_string(),
            if rng.gen_bool(0.5) { 1.0 } else { 0.0 },
        );
        params
    }

    fn random_trust_region_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("initial_radius".to_string(), rng.gen_range(0.01..2.0));
        params.insert("max_radius".to_string(), rng.gen_range(10.0..200.0));
        params.insert("eta_1".to_string(), rng.gen_range(0.05..0.25));
        params.insert("eta_2".to_string(), rng.gen_range(0.5..0.95));
        params.insert("gamma_1".to_string(), rng.gen_range(0.1..0.5));
        params.insert("gamma_2".to_string(), rng.gen_range(1.5..4.0));
        params
    }

    pub fn to_optimizer(&self) -> Arc<dyn Optimizer> {
        debug!(
            "Converting genome (ID: {:?}) to optimizer: {}",
            self.id, self.optimizer_type
        );
        match self.optimizer_type {
            OptimizerType::QQN => self.build_qqn_optimizer(),
            OptimizerType::LBFGS => self.build_lbfgs_optimizer(),
            OptimizerType::Adam => self.build_adam_optimizer(),
            OptimizerType::GD => self.build_gd_optimizer(),
            OptimizerType::TrustRegion => self.build_trust_region_optimizer(),
        }
    }

    fn build_qqn_optimizer(&self) -> Arc<dyn Optimizer> {
        let method_idx = self
            .parameters
            .get("line_search_method")
            .copied()
            .unwrap_or(0.0) as usize;
        let method = match method_idx {
            0 => LineSearchMethod::Backtracking,
            1 => LineSearchMethod::StrongWolfe,
            2 => LineSearchMethod::GoldenSection,
            3 => LineSearchMethod::Bisection,
            4 => LineSearchMethod::CubicQuadraticInterpolation,
            5 => LineSearchMethod::MoreThuente,
            _ => LineSearchMethod::Backtracking,
        };

        Arc::new(QQNOptimizer::new(QQNConfig {
            name: format!("QQN_{}", self.id.unwrap_or(0)),
            line_search: LineSearchConfig {
                method,
                c1: self.parameters.get("c1").copied().unwrap_or(1e-4),
                c2: self.parameters.get("c2").copied().unwrap_or(0.9),
                max_iterations: self
                    .parameters
                    .get("max_iterations")
                    .copied()
                    .unwrap_or(20.0) as usize,
                initial_step: self.parameters.get("initial_step").copied().unwrap_or(1.0),
                ..Default::default()
            },
            lbfgs_history: self
                .parameters
                .get("lbfgs_history")
                .copied()
                .unwrap_or(10.0) as usize,
            epsilon: self.parameters.get("epsilon").copied().unwrap_or(1e-6),
            ..Default::default()
        }))
    }

    fn build_lbfgs_optimizer(&self) -> Arc<dyn Optimizer> {
        Arc::new(LBFGSOptimizer::new(LBFGSConfig {
            name: format!("LBFGS_{}", self.id.unwrap_or(0)),
            history_size: self.parameters.get("history_size").copied().unwrap_or(10.0) as usize,
            line_search: LineSearchConfig {
                c1: self.parameters.get("c1").copied().unwrap_or(1e-4),
                c2: self.parameters.get("c2").copied().unwrap_or(0.9),
                initial_step: self.parameters.get("initial_step").copied().unwrap_or(1.0),
                ..Default::default()
            },
            epsilon: self.parameters.get("epsilon").copied().unwrap_or(1e-8),
            max_step_size: self.parameters.get("max_step_size").copied().unwrap_or(2.0),
            ..Default::default()
        }))
    }

    fn build_adam_optimizer(&self) -> Arc<dyn Optimizer> {
        Arc::new(AdamOptimizer::new(
            format!("Adam_{}", self.id.unwrap_or(0)),
            AdamConfig {
                learning_rate: self
                    .parameters
                    .get("learning_rate")
                    .copied()
                    .unwrap_or(0.001),
                beta1: self.parameters.get("beta1").copied().unwrap_or(0.9),
                beta2: self.parameters.get("beta2").copied().unwrap_or(0.999),
                epsilon: self.parameters.get("epsilon").copied().unwrap_or(1e-8),
                weight_decay: self.parameters.get("weight_decay").copied().unwrap_or(0.0),
                ..Default::default()
            },
        ))
    }

    fn build_gd_optimizer(&self) -> Arc<dyn Optimizer> {
        Arc::new(GDOptimizer::new(GDConfig {
            name: format!("GD_{}", self.id.unwrap_or(0)),
            learning_rate: self
                .parameters
                .get("learning_rate")
                .copied()
                .unwrap_or(0.01),
            momentum: self.parameters.get("momentum").copied().unwrap_or(0.0),
            weight_decay: self.parameters.get("weight_decay").copied().unwrap_or(0.0),
            nesterov: self.parameters.get("nesterov").copied().unwrap_or(0.0) > 0.5,
            ..Default::default()
        }))
    }

    fn build_trust_region_optimizer(&self) -> Arc<dyn Optimizer> {
        Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
            name: format!("TrustRegion_{}", self.id.unwrap_or(0)),
            initial_radius: self
                .parameters
                .get("initial_radius")
                .copied()
                .unwrap_or(1.0),
            max_radius: self.parameters.get("max_radius").copied().unwrap_or(100.0),
            eta_1: self.parameters.get("eta_1").copied().unwrap_or(0.2),
            eta_2: self.parameters.get("eta_2").copied().unwrap_or(0.8),
            gamma_1: self.parameters.get("gamma_1").copied().unwrap_or(0.5),
            gamma_2: self.parameters.get("gamma_2").copied().unwrap_or(3.0),
            ..Default::default()
        }))
    }
}

/// Manages the evolution of optimizer parameters
pub struct ParameterEvolution {
    population_size: usize,
    mutation_rate: f64,
    crossover_rate: f64,
    elite_size: usize,
    tournament_size: usize,
    rng: StdRng,
    next_id: usize,
}

impl ParameterEvolution {
    pub fn new(population_size: usize, seed: u64) -> Self {
        info!(
            "Initializing ParameterEvolution with population_size: {}, seed: {}",
            population_size, seed
        );
        Self {
            population_size,
            mutation_rate: 0.2,
            crossover_rate: 0.7,
            elite_size: (population_size / 10).max(1),
            tournament_size: 3,
            rng: StdRng::seed_from_u64(seed),
            next_id: 0,
        }
    }

    pub fn initialize_population(
        &mut self,
        optimizer_types: Vec<OptimizerType>,
    ) -> Vec<OptimizerGenome> {
        info!(
            "Initializing population with {} optimizer types",
            optimizer_types.len()
        );

        if optimizer_types.is_empty() {
            warn!("No optimizer types provided, defaulting to QQN");
            return self.initialize_single_family_population(OptimizerType::QQN);
        }

        let mut population = Vec::new();
        let genomes_per_type = self.population_size / optimizer_types.len();
        debug!("Creating {} genomes per optimizer type", genomes_per_type);

        for opt_type in optimizer_types {
            for _ in 0..genomes_per_type {
                let mut genome = OptimizerGenome::new_random(opt_type.clone(), &mut self.rng);
                genome.id = Some(self.next_id);
                self.next_id += 1;
                trace!(
                    "Created genome ID: {} for optimizer type: {}",
                    genome.id.unwrap(),
                    opt_type
                );
                population.push(genome);
            }
        }

        // Fill remaining slots if population_size is not divisible by number of types
        while population.len() < self.population_size {
            let opt_type = OptimizerType::QQN; // Default to QQN for extras
            let mut genome = OptimizerGenome::new_random(opt_type, &mut self.rng);
            genome.id = Some(self.next_id);
            self.next_id += 1;
            population.push(genome);
        }

        info!("Population initialized with {} genomes", population.len());
        population
    }

    pub fn initialize_single_family_population(
        &mut self,
        optimizer_type: OptimizerType,
    ) -> Vec<OptimizerGenome> {
        info!(
            "Initializing population of {} individuals for {} family",
            self.population_size, optimizer_type
        );

        let mut population = Vec::new();
        for i in 0..self.population_size {
            let mut genome = OptimizerGenome::new_random(optimizer_type.clone(), &mut self.rng);
            genome.id = Some(self.next_id);
            self.next_id += 1;
            trace!(
                "Created genome ID: {} for {} family",
                genome.id.unwrap(),
                optimizer_type
            );
            population.push(genome);
        }

        info!(
            "Single-family population initialized with {} genomes",
            population.len()
        );
        population
    }

    pub fn select_parents(&mut self, population: &[OptimizerGenome]) -> Vec<usize> {
        // Tournament selection
        let tournament_size = 3;
        let mut selected_indices = Vec::new();
        for _ in 0..population.len() {
            let mut tournament_indices = Vec::new();
            for _ in 0..tournament_size {
                tournament_indices.push(self.rng.random_range(0..population.len()));
            }
            let best_idx = tournament_indices
                .into_iter()
                .min_by(|&a, &b| {
                    let fitness_a = population[a].fitness.unwrap_or(f64::INFINITY);
                    let fitness_b = population[b].fitness.unwrap_or(f64::INFINITY);
                    fitness_a.partial_cmp(&fitness_b).unwrap()
                })
                .unwrap();
            selected_indices.push(best_idx);
        }
        selected_indices
    }

    pub fn evolve_generation(
        &mut self,
        population: &mut Vec<OptimizerGenome>,
        generation: usize,
    ) -> Vec<OptimizerGenome> {
        info!("Evolving generation {}", generation);

        // Log fitness statistics before evolution
        let fitnesses: Vec<f64> = population.iter().filter_map(|g| g.fitness).collect();

        if !fitnesses.is_empty() {
            let best_fitness = fitnesses.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let avg_fitness = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
            let worst_fitness = fitnesses.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            info!(
                "Generation {} fitness stats - Best: {:.6}, Avg: {:.6}, Worst: {:.6}",
                generation, best_fitness, avg_fitness, worst_fitness
            );
        } else {
            warn!("No fitness values available for generation {}", generation);
        }

        // Sort by fitness (lower is better for minimization)
        population.sort_by(|a, b| {
            let a_fitness = a.fitness.unwrap_or(f64::INFINITY);
            let b_fitness = b.fitness.unwrap_or(f64::INFINITY);
            a_fitness.partial_cmp(&b_fitness).unwrap()
        });

        if !population.is_empty() {
            debug!(
                "Population sorted by fitness. Best genome ID: {:?} with fitness: {:?}",
                population[0].id, population[0].fitness
            );
        }

        let mut new_population = Vec::new();

        // Keep elite individuals
        let elite_count = self.elite_size.min(population.len());
        debug!("Keeping {} elite individuals", elite_count);
        for i in 0..elite_count {
            let mut elite = population[i].clone();
            elite.generation = generation;
            new_population.push(elite);
        }

        // Generate rest of population through crossover and mutation
        let offspring_count = self.population_size - elite_count;
        debug!(
            "Generating {} offspring through crossover and mutation",
            offspring_count
        );
        let mut crossover_count = 0;
        let mut mutation_count = 0;

        while new_population.len() < self.population_size {
            let parent1 = self.tournament_selection(population);
            let parent2 = self.tournament_selection(population);

            let mut offspring = if self.rng.gen::<f64>() < self.crossover_rate {
                crossover_count += 1;
                self.crossover(&parent1, &parent2)
            } else {
                parent1.clone()
            };

            offspring.id = Some(self.next_id.clone());
            self.next_id += 1;
            offspring.generation_created = generation;
            offspring.parent_ids = vec![parent1.id.unwrap_or(0), parent2.id.unwrap_or(0)];

            if self.rng.gen::<f64>() < self.mutation_rate {
                mutation_count += 1;
                self.mutate(&mut offspring);
            }

            offspring.generation = generation;
            offspring.fitness = None; // Reset fitness for evaluation
            new_population.push(offspring);
        }

        info!(
            "Generation {} evolution complete: {} crossovers, {} mutations",
            generation, crossover_count, mutation_count
        );
        debug!("New population size: {}", new_population.len());

        new_population
    }

    fn tournament_selection(&mut self, population: &[OptimizerGenome]) -> OptimizerGenome {
        if population.is_empty() {
            panic!("Cannot perform tournament selection on empty population");
        }

        trace!(
            "Running tournament selection with tournament size: {}",
            self.tournament_size
        );
        let mut best = None;
        let mut best_fitness = f64::INFINITY;

        for _ in 0..self.tournament_size {
            let idx = self.rng.gen_range(0..population.len());
            let fitness = population[idx].fitness.unwrap_or(f64::INFINITY);
            if fitness < best_fitness {
                best_fitness = fitness;
                best = Some(&population[idx]);
            }
        }

        let selected = best.expect("Tournament selection failed to select a genome");
        trace!(
            "Tournament selection chose genome ID: {:?} with fitness: {:.6}",
            selected.id,
            best_fitness
        );
        selected.clone()
    }

    fn crossover(
        &mut self,
        parent1: &OptimizerGenome,
        parent2: &OptimizerGenome,
    ) -> OptimizerGenome {
        trace!(
            "Performing crossover between genomes {:?} and {:?}",
            parent1.id,
            parent2.id
        );

        // Handle cross-type breeding with type selection
        let offspring_type = if parent1.optimizer_type != parent2.optimizer_type {
            // Choose type randomly from parents
            if self.rng.gen_bool(0.5) {
                parent1.optimizer_type.clone()
            } else {
                parent2.optimizer_type.clone()
            }
        } else {
            parent1.optimizer_type.clone()
        };

        // Create offspring with selected type
        let mut offspring = OptimizerGenome {
            optimizer_type: offspring_type.clone(),
            parameters: HashMap::new(),
            fitness: None,
            id: None,
            generation_created: 0,
            parent_ids: vec![],
            generation: 0,
        };

        // Get valid parameters for the offspring type
        let valid_params = match offspring_type {
            OptimizerType::QQN => vec![
                "c1",
                "c2",
                "lbfgs_history",
                "epsilon",
                "initial_step",
                "max_iterations",
                "line_search_method",
            ],
            OptimizerType::LBFGS => vec![
                "history_size",
                "c1",
                "c2",
                "epsilon",
                "max_step_size",
                "initial_step",
            ],
            OptimizerType::Adam => {
                vec!["learning_rate", "beta1", "beta2", "epsilon", "weight_decay"]
            }
            OptimizerType::GD => vec!["learning_rate", "momentum", "weight_decay", "nesterov"],
            OptimizerType::TrustRegion => vec![
                "initial_radius",
                "max_radius",
                "eta_1",
                "eta_2",
                "gamma_1",
                "gamma_2",
            ],
        };

        // Perform uniform crossover for valid parameters
        let mut crossed_params = 0;
        for param in valid_params {
            let param_str = param.to_string();
            let value = if self.rng.gen_bool(0.5) {
                parent1.parameters.get(&param_str)
            } else {
                parent2.parameters.get(&param_str)
            };

            if let Some(v) = value {
                offspring.parameters.insert(param_str, *v);
                crossed_params += 1;
            } else {
                // If parameter not found in parent, generate random value
                let random_genome =
                    OptimizerGenome::new_random(offspring_type.clone(), &mut self.rng);
                if let Some(v) = random_genome.parameters.get(&param_str) {
                    offspring.parameters.insert(param_str, *v);
                }
            }
        }

        trace!(
            "Crossover completed: {} parameters set for {} offspring",
            crossed_params,
            offspring_type
        );
        offspring
    }

    fn mutate(&mut self, genome: &mut OptimizerGenome) {
        debug!("Mutating genome ID: {:?}", genome.id);
        let keys: Vec<String> = genome.parameters.keys().cloned().collect();

        if keys.is_empty() {
            warn!("No parameters to mutate in genome");
            return;
        }

        let num_mutations = self.rng.gen_range(1..=3.min(keys.len()));
        debug!("Applying {} mutations to genome", num_mutations);

        for _ in 0..num_mutations {
            let key = &keys[self.rng.gen_range(0..keys.len())];
            if let Some(value) = genome.parameters.get_mut(key) {
                let old_value = *value;

                // Apply Gaussian mutation with adaptive strength
                let mutation_strength = if key.contains("epsilon") || key.contains("learning_rate")
                {
                    0.3 // Stronger mutation for log-scale parameters
                } else {
                    0.2
                };

                let delta = self.rng.gen_range(-mutation_strength..mutation_strength);

                // Handle different parameter ranges
                *value = match key.as_str() {
                    // Log scale parameters
                    "epsilon" | "learning_rate" | "weight_decay" => {
                        if *value > 0.0 {
                            let log_val = value.ln();
                            let new_log_val = log_val + delta * 2.0; // Larger changes in log space
                            new_log_val.exp().max(1e-12).min(1.0)
                        } else {
                            10f64.powf(self.rng.gen_range(-12.0..-4.0))
                        }
                    }
                    // Probability parameters [0, 1]
                    "beta1" | "beta2" | "c1" | "c2" | "eta_1" | "eta_2" | "momentum" => {
                        (*value + delta).max(0.0).min(0.999)
                    }
                    // Integer parameters
                    "lbfgs_history" | "history_size" | "max_iterations" => {
                        let int_val = (*value as i32 + (delta * 10.0) as i32).max(1);
                        int_val as f64
                    }
                    // Line search method (categorical)
                    "line_search_method" => {
                        if self.rng.gen_bool(0.3) {
                            // 30% chance to change method
                            self.rng.gen_range(0.0..6.0).as_f64().floor()
                        } else {
                            *value
                        }
                    }
                    // Boolean parameters
                    "nesterov" => {
                        if self.rng.gen_bool(0.3) {
                            // 30% chance to flip
                            1.0 - *value
                        } else {
                            *value
                        }
                    }
                    // Positive real parameters
                    _ => (*value * (1.0 + delta)).max(0.001),
                };

                trace!(
                    "Mutated parameter '{}': {:.6} -> {:.6}",
                    key,
                    old_value,
                    *value
                );
            }
        }
    }

    pub fn get_best_genomes(
        &self,
        population: &[OptimizerGenome],
        n: usize,
    ) -> Vec<OptimizerGenome> {
        debug!(
            "Selecting top {} genomes from population of {}",
            n,
            population.len()
        );

        if population.is_empty() {
            warn!("Empty population provided to get_best_genomes");
            return Vec::new();
        }

        let mut sorted = population.to_vec();
        sorted.sort_by(|a, b| {
            let fitness_a = a.fitness.unwrap_or(f64::INFINITY);
            let fitness_b = b.fitness.unwrap_or(f64::INFINITY);
            fitness_a.partial_cmp(&fitness_b).unwrap()
        });

        let best_genomes = sorted.into_iter().take(n).collect::<Vec<_>>();

        if !best_genomes.is_empty() {
            info!(
                "Best genome has fitness: {:?} (ID: {:?})",
                best_genomes[0].fitness, best_genomes[0].id
            );
        }

        best_genomes
    }

    /// Set mutation rate (0.0 to 1.0)
    pub fn set_mutation_rate(&mut self, rate: f64) {
        self.mutation_rate = rate.max(0.0).min(1.0);
        debug!("Mutation rate set to {:.3}", self.mutation_rate);
    }

    /// Set crossover rate (0.0 to 1.0)
    pub fn set_crossover_rate(&mut self, rate: f64) {
        self.crossover_rate = rate.max(0.0).min(1.0);
        debug!("Crossover rate set to {:.3}", self.crossover_rate);
    }

    /// Set elite size (number of best individuals to preserve)
    pub fn set_elite_size(&mut self, size: usize) {
        self.elite_size = size.min(self.population_size);
        debug!("Elite size set to {}", self.elite_size);
    }

    /// Set tournament size for selection
    pub fn set_tournament_size(&mut self, size: usize) {
        self.tournament_size = size.max(2);
        debug!("Tournament size set to {}", self.tournament_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_creation() {
        let mut rng = StdRng::seed_from_u64(42);
        let genome = OptimizerGenome::new_random(OptimizerType::QQN, &mut rng);
        assert_eq!(genome.optimizer_type, OptimizerType::QQN);
        assert!(!genome.parameters.is_empty());
        assert!(genome.fitness.is_none());
    }

    #[test]
    fn test_population_initialization() {
        let mut evolution = ParameterEvolution::new(10, 42);
        let types = vec![OptimizerType::QQN, OptimizerType::Adam];
        let population = evolution.initialize_population(types);
        assert_eq!(population.len(), 10);
    }

    #[test]
    fn test_evolution_generation() {
        let mut evolution = ParameterEvolution::new(10, 42);
        let mut population = evolution.initialize_single_family_population(OptimizerType::Adam);

        // Set some fitness values
        for (i, genome) in population.iter_mut().enumerate() {
            genome.fitness = Some((i as f64) * 0.1);
        }

        let new_population = evolution.evolve_generation(&mut population, 1);
        assert_eq!(new_population.len(), 10);
    }

    #[test]
    fn test_crossover_same_type() {
        let mut evolution = ParameterEvolution::new(10, 42);
        let mut rng = StdRng::seed_from_u64(42);

        let parent1 = OptimizerGenome::new_random(OptimizerType::Adam, &mut rng);
        let parent2 = OptimizerGenome::new_random(OptimizerType::Adam, &mut rng);

        let offspring = evolution.crossover(&parent1, &parent2);
        assert_eq!(offspring.optimizer_type, OptimizerType::Adam);
        assert!(!offspring.parameters.is_empty());
    }

    #[test]
    fn test_mutation() {
        let mut evolution = ParameterEvolution::new(10, 42);
        let mut rng = StdRng::seed_from_u64(42);
        let mut genome = OptimizerGenome::new_random(OptimizerType::LBFGS, &mut rng);

        let original_params = genome.parameters.clone();
        evolution.mutate(&mut genome);

        // At least one parameter should have changed
        let changed = original_params
            .iter()
            .any(|(k, v)| genome.parameters.get(k) != Some(v));
        assert!(changed);
    }

    #[test]
    fn test_get_best_genomes() {
        let evolution = ParameterEvolution::new(10, 42);
        let mut population = vec![];

        for i in 0..5 {
            let mut genome = OptimizerGenome {
                optimizer_type: OptimizerType::QQN,
                parameters: HashMap::new(),
                fitness: Some(i as f64),
                id: Some(i),
                generation_created: 0,
                parent_ids: vec![],
                generation: 0,
            };
            population.push(genome);
        }

        let best = evolution.get_best_genomes(&population, 2);
        assert_eq!(best.len(), 2);
        assert_eq!(best[0].fitness, Some(0.0));
        assert_eq!(best[1].fitness, Some(1.0));
    }
}
