use crate::tsp::TSP;
use rand::prelude::*;

// Genetic Algorithm solution for TSP
pub fn solve_with_genetic(tsp: &TSP) -> Vec<usize> {
    let population_size = 100;
    let generations = 1000;
    let mutation_rate = 0.05;

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
        // Evaluate fitness and selection
        population.sort_by(|a, b| {
            tsp.calculate_total_distance(a)
                .partial_cmp(&tsp.calculate_total_distance(b))
                .unwrap()
        });
        population.truncate(population_size / 2);

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

    // Return the best solution
    population.sort_by(|a, b| {
        tsp.calculate_total_distance(a)
            .partial_cmp(&tsp.calculate_total_distance(b))
            .unwrap()
    });

    population[0].clone()
}
