use crate::tsp::TSP;
use tensorflow::{Graph, SessionOptions, SessionRunArgs, SavedModelBundle, Tensor};
use std::error::Error;

pub fn solve_with_ml_initialization(tsp: &TSP) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut graph = Graph::new();
    let model_dir = "data/saved_model";
    let session_bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_dir)?;

    let session = &session_bundle.session;

    let input_tensor = Tensor::new(&[tsp.coordinates.len() as u64, 2])
        .with_values(&tsp.coordinates.iter().flat_map(|&coord| vec![coord.0 as f32, coord.1 as f32]).collect::<Vec<f32>>())?;

    let mut args = SessionRunArgs::new();
    args.add_feed(&graph.operation_by_name_required("serve_input_layer")?, 0, &input_tensor);
    let output_fetch_token = args.request_fetch(&graph.operation_by_name_required("StatefulPartitionedCall")?, 0);

    session.run(&mut args)?;

    let output_tensor: Tensor<f32> = args.fetch(output_fetch_token)?;

    // Debug output tensor
    println!("Output tensor: {:?}", output_tensor);

    // Process the output to convert continuous values into city indices
    let mut route = Vec::new();
    let mut visited = vec![false; tsp.coordinates.len()];
    let output = output_tensor.to_vec();

    for i in 0..tsp.coordinates.len() {
        let mut min_distance = f64::MAX;
        let mut city = 0;
        for j in 0..2 {
            let index = i * 2 + j; // Calculate the correct index in the flattened array
            if (output[index] as f64) < min_distance && !visited[j] {
                min_distance = output[index] as f64;
                city = j;
            }
        }
        route.push(city);
        visited[city] = true;
    }

    // If all cities are not included, return an error
    if route.len() != tsp.coordinates.len() {
        return Err("Not all cities were included in the route.".into());
    }

    route.push(route[0]); // Return to the start city
    Ok(route)
}
