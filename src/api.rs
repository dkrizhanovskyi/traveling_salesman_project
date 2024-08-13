use crate::tsp::TSP;
use crate::algorithms::{
    solve_with_dp, solve_with_nearest_neighbor, solve_with_genetic,
    solve_with_hybrid, solve_with_ml_initialization, solve_with_multi_objective,
};
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct TSPRequest {
    coordinates: Vec<(f64, f64)>,
    algorithm: String,
}

#[derive(Serialize)]
struct TSPResponse {
    route: Vec<usize>,
    total_distance: f64,
    execution_time: f64,
}

#[tokio::main]
async fn main() {
    let tsp_solver = Arc::new(Mutex::new(None));

    let solve_tsp = warp::post()
        .and(warp::path("solve"))
        .and(warp::body::json())
        .and(with_tsp_solver(tsp_solver.clone()))
        .and_then(handle_solve_tsp);

    let routes = solve_tsp;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_tsp_solver(
    tsp_solver: Arc<Mutex<Option<TSP>>>,
) -> impl Filter<Extract = (Arc<Mutex<Option<TSP>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tsp_solver.clone())
}

async fn handle_solve_tsp(
    req: TSPRequest,
    _tsp_solver: Arc<Mutex<Option<TSP>>>, 
) -> Result<impl warp::Reply, warp::Rejection> {
    let tsp_instance = TSP::new(TSP::generate_distances(&req.coordinates), req.coordinates.clone());

    let start = std::time::Instant::now();
    let route = match req.algorithm.as_str() {
        "dp" => solve_with_dp(&tsp_instance),
        "nearest_neighbor" => solve_with_nearest_neighbor(&tsp_instance),
        "genetic" => solve_with_genetic(&tsp_instance),
        "hybrid" => solve_with_hybrid(&tsp_instance),
        "ml" => solve_with_ml_initialization(&tsp_instance).unwrap_or_else(|_| vec![]),
        "multi_objective" => solve_with_multi_objective(&tsp_instance),
        _ => return Ok(warp::reply::json(&"Invalid algorithm")),
    };
    let execution_time = start.elapsed().as_secs_f64();
    let total_distance = tsp_instance.calculate_total_distance(&route);

    let response = TSPResponse {
        route,
        total_distance,
        execution_time,
    };

    Ok(warp::reply::json(&response))
}
