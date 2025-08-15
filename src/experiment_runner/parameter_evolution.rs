use crate::optimizers::{GDConfig, GDOptimizer, TrustRegionConfig, TrustRegionOptimizer};
use crate::{
    AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod,
    Optimizer, QQNConfig, QQNOptimizer,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use plotters::prelude::LogScalable;

/// Represents a genome for an optimizer configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptimizerGenome {
    pub optimizer_type: OptimizerType,
    pub parameters: HashMap<String, f64>,
    pub fitness: Option<f64>,
    pub generation: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OptimizerType {
    QQN,
    LBFGS,
    Adam,
    GD,
    TrustRegion,
    GradientDescent,
}
impl OptimizerGenome {
    pub fn new_random(optimizer_type: OptimizerType, rng: &mut StdRng) -> Self {
        let parameters = match optimizer_type {
            OptimizerType::QQN => Self::random_qqn_params(rng),
            OptimizerType::LBFGS => Self::random_lbfgs_params(rng),
            OptimizerType::Adam => Self::random_adam_params(rng),
            OptimizerType::GD => Self::random_gd_params(rng),
            OptimizerType::TrustRegion => Self::random_trust_region_params(rng),
            OptimizerType::GradientDescent => Self::random_gd_params(rng), // Alias for GD
        };
        Self {
            optimizer_type,
            parameters,
            fitness: None,
            generation: 0,
        }
    }
    fn random_qqn_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("c1".to_string(), rng.gen_range(1e-6..1e-2));
        params.insert("c2".to_string(), rng.gen_range(0.1..0.99));
        params.insert("lbfgs_history".to_string(), rng.gen_range(3.0..20.0));
        params.insert("epsilon".to_string(), 10f64.powf(rng.gen_range(-10.0..-4.0)));
        params.insert("initial_step".to_string(), rng.gen_range(0.1..2.0));
        params.insert("max_iterations".to_string(), rng.gen_range(10.0..50.0));
        params.insert("line_search_method".to_string(), rng.gen_range(0.0..6.0).as_f64().floor());
        params
    }
    fn random_lbfgs_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("history_size".to_string(), rng.gen_range(3.0..30.0));
        params.insert("c1".to_string(), rng.gen_range(1e-6..1e-2));
        params.insert("c2".to_string(), rng.gen_range(0.1..0.99));
        params.insert("epsilon".to_string(), 10f64.powf(rng.gen_range(-12.0..-6.0)));
        params.insert("max_step_size".to_string(), rng.gen_range(0.5..10.0));
        params.insert("initial_step".to_string(), rng.gen_range(0.01..2.0));
        params
    }
    fn random_adam_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("learning_rate".to_string(), 10f64.powf(rng.gen_range(-4.0..0.0)));
        params.insert("beta1".to_string(), rng.gen_range(0.8..0.99));
        params.insert("beta2".to_string(), rng.gen_range(0.9..0.9999));
        params.insert("epsilon".to_string(), 10f64.powf(rng.gen_range(-10.0..-6.0)));
        params.insert("weight_decay".to_string(), rng.gen_range(0.0..1e-3));
        params
    }
    fn random_gd_params(rng: &mut StdRng) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("learning_rate".to_string(), 10f64.powf(rng.gen_range(-3.0..0.0)));
        params.insert("momentum".to_string(), rng.gen_range(0.0..0.99));
        params.insert("weight_decay".to_string(), rng.gen_range(0.0..1e-3));
        params.insert("nesterov".to_string(), if rng.gen_bool(0.5) { 1.0 } else { 0.0 });
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
        match self.optimizer_type {
            OptimizerType::QQN => self.build_qqn_optimizer(),
            OptimizerType::LBFGS => self.build_lbfgs_optimizer(),
            OptimizerType::Adam => self.build_adam_optimizer(),
            OptimizerType::GD => self.build_gd_optimizer(),
            OptimizerType::TrustRegion => self.build_trust_region_optimizer(),
            OptimizerType::GradientDescent => self.build_gd_optimizer(),
        }
    }
    fn build_qqn_optimizer(&self) -> Arc<dyn Optimizer> {
        let method_idx = self.parameters.get("line_search_method")
            .copied().unwrap_or(0.0) as usize;
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
            line_search: LineSearchConfig {
                method,
                c1: self.parameters.get("c1").copied().unwrap_or(1e-4),
                c2: self.parameters.get("c2").copied().unwrap_or(0.9),
                max_iterations: self.parameters.get("max_iterations").copied().unwrap_or(20.0) as usize,
                initial_step: self.parameters.get("initial_step").copied().unwrap_or(1.0),
                ..Default::default()
            },
            lbfgs_history: self.parameters.get("lbfgs_history").copied().unwrap_or(10.0) as usize,
            epsilon: self.parameters.get("epsilon").copied().unwrap_or(1e-6),
            ..Default::default()
        }))
    }
    fn build_lbfgs_optimizer(&self) -> Arc<dyn Optimizer> {
        Arc::new(LBFGSOptimizer::new(LBFGSConfig {
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
        Arc::new(AdamOptimizer::new(AdamConfig {
            learning_rate: self.parameters.get("learning_rate").copied().unwrap_or(0.001),
            beta1: self.parameters.get("beta1").copied().unwrap_or(0.9),
            beta2: self.parameters.get("beta2").copied().unwrap_or(0.999),
            epsilon: self.parameters.get("epsilon").copied().unwrap_or(1e-8),
            weight_decay: self.parameters.get("weight_decay").copied().unwrap_or(0.0),
            ..Default::default()
        }))
    }
    fn build_gd_optimizer(&self) -> Arc<dyn Optimizer> {
        Arc::new(GDOptimizer::new(GDConfig {
            learning_rate: self.parameters.get("learning_rate").copied().unwrap_or(0.01),
            momentum: self.parameters.get("momentum").copied().unwrap_or(0.0),
            weight_decay: self.parameters.get("weight_decay").copied().unwrap_or(0.0),
            nesterov: self.parameters.get("nesterov").copied().unwrap_or(0.0) > 0.5,
            ..Default::default()
        }))
    }
    fn build_trust_region_optimizer(&self) -> Arc<dyn Optimizer> {
        Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
            initial_radius: self.parameters.get("initial_radius").copied().unwrap_or(1.0),
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
}
impl ParameterEvolution {
    pub fn new(population_size: usize, seed: u64) -> Self {
        Self {
            population_size,
            mutation_rate: 0.2,
            crossover_rate: 0.7,
            elite_size: population_size / 10,
            tournament_size: 3,
            rng: StdRng::seed_from_u64(seed),
        }
    }
    pub fn initialize_population(&mut self, optimizer_types: Vec<OptimizerType>) -> Vec<OptimizerGenome> {
        let mut population = Vec::new();
        let genomes_per_type = self.population_size / optimizer_types.len();
        for opt_type in optimizer_types {
            for _ in 0..genomes_per_type {
                population.push(OptimizerGenome::new_random(opt_type.clone(), &mut self.rng));
            }
        }
        // Fill remaining slots if population_size is not divisible by number of types
        while population.len() < self.population_size {
            let opt_type = OptimizerType::QQN; // Default to QQN for extras
            population.push(OptimizerGenome::new_random(opt_type, &mut self.rng));
        }
        population
    }
    pub fn evolve_generation(
        &mut self,
        population: &mut Vec<OptimizerGenome>,
        generation: usize,
    ) -> Vec<OptimizerGenome> {
        // Sort by fitness (lower is better for minimization)
        population.sort_by(|a, b| {
            let a_fitness = a.fitness.unwrap_or(f64::INFINITY);
            let b_fitness = b.fitness.unwrap_or(f64::INFINITY);
            a_fitness.partial_cmp(&b_fitness).unwrap()
        });
        let mut new_population = Vec::new();
        // Keep elite individuals
        for i in 0..self.elite_size {
            let mut elite = population[i].clone();
            elite.generation = generation;
            new_population.push(elite);
        }
        // Generate rest of population through crossover and mutation
        while new_population.len() < self.population_size {
            let parent1 = self.tournament_selection(population);
            let parent2 = self.tournament_selection(population);
            let mut offspring = if self.rng.gen::<f64>() < self.crossover_rate {
                self.crossover(&parent1, &parent2)
            } else {
                parent1.clone()
            };
            if self.rng.gen::<f64>() < self.mutation_rate {
                self.mutate(&mut offspring);
            }
            offspring.generation = generation;
            offspring.fitness = None; // Reset fitness for new evaluation
            new_population.push(offspring);
        }
        new_population
    }
    fn tournament_selection(&mut self, population: &[OptimizerGenome]) -> OptimizerGenome {
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
        best.unwrap().clone()
    }
    fn crossover(&mut self, parent1: &OptimizerGenome, parent2: &OptimizerGenome) -> OptimizerGenome {
        // Only crossover if same optimizer type
        if parent1.optimizer_type != parent2.optimizer_type {
            return if self.rng.gen_bool(0.5) {
                parent1.clone()
            } else {
                parent2.clone()
            };
        }
        let mut offspring = parent1.clone();
        // Uniform crossover for parameters
        for (key, value) in &parent2.parameters {
            if self.rng.gen_bool(0.5) {
                offspring.parameters.insert(key.clone(), *value);
            }
        }
        offspring
    }
    fn mutate(&mut self, genome: &mut OptimizerGenome) {
        let keys: Vec<String> = genome.parameters.keys().cloned().collect();
        let num_mutations = self.rng.gen_range(1..=3.min(keys.len()));
        for _ in 0..num_mutations {
            let key = &keys[self.rng.gen_range(0..keys.len())];
            if let Some(value) = genome.parameters.get_mut(key) {
                // Apply Gaussian mutation
                let mutation_strength = 0.2;
                let delta = self.rng.gen_range(-mutation_strength..mutation_strength);
                // Handle different parameter ranges
                if key.contains("epsilon") || key.contains("learning_rate") {
                    // Log scale for very small values
                    let log_val = value.ln();
                    let new_log_val = log_val + delta;
                    *value = new_log_val.exp().max(1e-12).min(1.0);
                } else if key.contains("beta") || key.contains("c1") || key.contains("c2") {
                    // Keep in [0, 1] range
                    *value = (*value + delta).max(0.0).min(1.0);
                } else {
                    // General positive values
                    *value = (*value * (1.0 + delta)).max(0.001);
                }
            }
        }
    }
    pub fn get_best_genomes(&self, population: &[OptimizerGenome], n: usize) -> Vec<OptimizerGenome> {
        let mut sorted = population.to_vec();
        sorted.sort_by(|a, b| {
            let a_fitness = a.fitness.unwrap_or(f64::INFINITY);
            let b_fitness = b.fitness.unwrap_or(f64::INFINITY);
            a_fitness.partial_cmp(&b_fitness).unwrap()
        });
        sorted.into_iter().take(n).collect()
    }
}