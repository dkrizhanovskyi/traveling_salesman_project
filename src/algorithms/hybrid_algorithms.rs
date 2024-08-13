use crate::tsp::TSP;
use crate::algorithms::solve_with_genetic;
use std::error::Error;

pub fn solve_with_hybrid(tsp: &TSP) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut refined_route = solve_with_genetic(tsp);
    let n = refined_route.len();

    for i in 0..n {
        let (_, right) = refined_route.split_at_mut(i);
        let chunk = &mut right[0..n - i];
        
        // Refine the route by swapping cities for optimization
        for j in 0..chunk.len() - 1 {
            if tsp.distances[chunk[j]][chunk[j + 1]] > tsp.distances[chunk[j + 1]][chunk[j]] {
                chunk.swap(j, j + 1);
            }
        }
    }

    Ok(refined_route)
}
