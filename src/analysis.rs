use crate::tsp::TSP;
use crate::algorithms::{solve_with_dp, solve_with_nearest_neighbor, solve_with_genetic, solve_with_hybrid, solve_with_ml_initialization, solve_with_multi_objective};
use std::time::Instant;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn analyze_performance(tsp: &TSP) -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();

    // Dynamic Programming
    let start = Instant::now();
    let dp_route = solve_with_dp(tsp);
    let dp_time = start.elapsed().as_secs_f64();
    let dp_distance = tsp.calculate_total_distance(&dp_route);
    results.push(("Dynamic Programming", dp_distance, dp_time));

    // Nearest Neighbor
    let start = Instant::now();
    let nn_route = solve_with_nearest_neighbor(tsp);
    let nn_time = start.elapsed().as_secs_f64();
    let nn_distance = tsp.calculate_total_distance(&nn_route);
    results.push(("Nearest Neighbor", nn_distance, nn_time));

    // Genetic Algorithm
    let start = Instant::now();
    let ga_route = solve_with_genetic(tsp);
    let ga_time = start.elapsed().as_secs_f64();
    let ga_distance = tsp.calculate_total_distance(&ga_route);
    results.push(("Genetic Algorithm", ga_distance, ga_time));

    // Hybrid Algorithm
    let start = Instant::now();
    let hybrid_route = solve_with_hybrid(tsp)?;
    let hybrid_time = start.elapsed().as_secs_f64();
    let hybrid_distance = tsp.calculate_total_distance(&hybrid_route);
    results.push(("Hybrid Algorithm", hybrid_distance, hybrid_time));

    // ML Initialization
    let start = Instant::now();
    let ml_route = solve_with_ml_initialization(tsp)?;
    let ml_time = start.elapsed().as_secs_f64();
    let ml_distance = tsp.calculate_total_distance(&ml_route);
    results.push(("ML Initialization", ml_distance, ml_time));

    // Multi-objective Optimization
    let start = Instant::now();
    let mo_route = solve_with_multi_objective(tsp);
    let mo_time = start.elapsed().as_secs_f64();
    let mo_distance = tsp.calculate_total_distance(&mo_route);
    results.push(("Multi-objective Optimization", mo_distance, mo_time));

    // Save the results
    save_performance_results(&results, "outputs/performance_results.csv")?;

    Ok(())
}

fn save_performance_results(results: &Vec<(&str, f64, f64)>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    writeln!(file, "Algorithm,Total Distance,Execution Time (s)")?;
    for (algorithm, distance, time) in results {
        writeln!(file, "{},{:.2},{:.6}", algorithm, distance, time)?;
    }
    Ok(())
}
