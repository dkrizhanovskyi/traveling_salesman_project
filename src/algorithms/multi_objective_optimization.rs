use crate::tsp::TSP;
use ndarray::{Array2, Axis};
use rand::prelude::*;
use rayon::prelude::*;

// Multi-objective Optimization for TSP
pub fn solve_with_multi_objective(tsp: &TSP) -> Vec<usize> {
    let population_size = 100;
    let generations = 500;
    let mutation_rate = 0.1;

    // Objectives: 0 - Minimize total distance, 1 - Minimize number of turns
    let num_objectives = 2;

    // Initialize the population
    let mut population: Vec<Vec<usize>> = (0..population_size)
        .map(|_| {
            let mut individual: Vec<usize> = (0..tsp.coordinates.len()).collect();
            individual.shuffle(&mut rand::thread_rng());
            individual.push(0); // Return to the start city
            individual
        })
        .collect();

    for _ in 0..generations {
        // Evaluate fitness and Pareto ranking
        let fitness_scores: Vec<Array2<f64>> = population
            .par_iter()
            .map(|individual| evaluate_fitness(tsp, individual, num_objectives))
            .collect();

        // Select the top individuals based on Pareto ranking
        population = select_pareto_front(&population, &fitness_scores);

        // Crossover and mutation
        let mut new_population = population.clone();
        while new_population.len() < population_size {
            let parent1 = &population[rand::thread_rng().gen_range(0..population.len())];
            let parent2 = &population[rand::thread_rng().gen_range(0..population.len())];

            let crossover_point = rand::thread_rng().gen_range(1..parent1.len() - 1);
            let mut child1 = parent1[0..crossover_point].to_vec();
            let mut child2 = parent2[0..crossover_point].to_vec();

            let remaining_cities1: Vec<_> = parent2.iter().filter(|&&city| !child1.contains(&city)).cloned().collect();
            let remaining_cities2: Vec<_> = parent1.iter().filter(|&&city| !child2.contains(&city)).cloned().collect();

            child1.extend(remaining_cities1);
            child2.extend(remaining_cities2);

            // Mutation
            if rand::thread_rng().gen_bool(mutation_rate) {
                let swap_indices = (rand::thread_rng().gen_range(1..child1.len() - 1), rand::thread_rng().gen_range(1..child1.len() - 1));
                child1.swap(swap_indices.0, swap_indices.1);
                child2.swap(swap_indices.0, swap_indices.1);
            }

            new_population.push(child1);
            new_population.push(child2);
        }

        population = new_population;
    }

    // Return the best solution from the final Pareto front
    population.sort_by(|a, b| {
        evaluate_fitness(tsp, a, num_objectives)
            .sum_axis(Axis(0))
            .sum()
            .partial_cmp(
                &evaluate_fitness(tsp, b, num_objectives)
                    .sum_axis(Axis(0))
                    .sum(),
            )
            .unwrap()
    });

    population[0].clone()
}

// Evaluate fitness for multiple objectives
fn evaluate_fitness(tsp: &TSP, individual: &Vec<usize>, num_objectives: usize) -> Array2<f64> {
    let total_distance = tsp.calculate_total_distance(individual);
    let num_turns = calculate_number_of_turns(individual);

    let mut fitness = Array2::<f64>::zeros((num_objectives, 1));
    fitness[[0, 0]] = total_distance;
    fitness[[1, 0]] = num_turns as f64;

    fitness
}

// Calculate the number of turns in a route
fn calculate_number_of_turns(route: &Vec<usize>) -> usize {
    // Placeholder function: Actual implementation would calculate turns based on angles or direction changes
    route.len() - 2
}

// Select individuals from the Pareto front
fn select_pareto_front(population: &Vec<Vec<usize>>, fitness_scores: &Vec<Array2<f64>>) -> Vec<Vec<usize>> {
    let mut selected = Vec::new();
    for (i, score1) in fitness_scores.iter().enumerate() {
        let mut dominated = false;
        for (j, score2) in fitness_scores.iter().enumerate() {
            if i != j && dominates(score2, score1) {
                dominated = true;
                break;
            }
        }
        if !dominated {
            selected.push(population[i].clone());
        }
    }
    selected
}

// Check if one solution dominates another
fn dominates(score1: &Array2<f64>, score2: &Array2<f64>) -> bool {
    score1.iter().zip(score2.iter()).all(|(a, b)| a <= b) && score1 != score2
}
