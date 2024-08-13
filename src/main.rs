mod tsp;
mod algorithms;
mod visualization;
mod analysis;
mod data_handling;

use crate::tsp::TSP;
use crate::algorithms::{solve_with_dp, solve_with_nearest_neighbor, solve_with_genetic, solve_with_hybrid, solve_with_ml_initialization, solve_with_multi_objective};
use crate::visualization::visualize_route;
use crate::analysis::analyze_performance;
use crate::data_handling::{load_coordinates_from_csv, save_route_to_csv};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coordinates = load_coordinates_from_csv("data/cities_coordinates.csv")?;
    let distances = TSP::generate_distances(&coordinates);
    let tsp_instance = TSP::new(distances, coordinates);

    let dp_route = solve_with_dp(&tsp_instance);
    visualize_route(&tsp_instance, &dp_route, "outputs/dp_route.png")?;
    save_route_to_csv(&dp_route, "outputs/dp_route.csv")?;

    let nn_route = solve_with_nearest_neighbor(&tsp_instance);
    visualize_route(&tsp_instance, &nn_route, "outputs/nn_route.png")?;
    save_route_to_csv(&nn_route, "outputs/nn_route.csv")?;

    let ga_route = solve_with_genetic(&tsp_instance);
    visualize_route(&tsp_instance, &ga_route, "outputs/ga_route.png")?;
    save_route_to_csv(&ga_route, "outputs/ga_route.csv")?;

    let hybrid_route = solve_with_hybrid(&tsp_instance)?;
    visualize_route(&tsp_instance, &hybrid_route, "outputs/hybrid_route.png")?;
    save_route_to_csv(&hybrid_route, "outputs/hybrid_route.csv")?;

    let ml_route = solve_with_ml_initialization(&tsp_instance)?;
    visualize_route(&tsp_instance, &ml_route, "outputs/ml_route.png")?;
    save_route_to_csv(&ml_route, "outputs/ml_route.csv")?;

    let mo_route = solve_with_multi_objective(&tsp_instance);
    visualize_route(&tsp_instance, &mo_route, "outputs/mo_route.png")?;
    save_route_to_csv(&mo_route, "outputs/mo_route.csv")?;

    analyze_performance(&tsp_instance)?;

    Ok(())
}
