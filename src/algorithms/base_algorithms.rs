use crate::tsp::TSP;
use std::collections::HashMap;

// Dynamic Programming solution for TSP
pub fn solve_with_dp(tsp: &TSP) -> Vec<usize> {
    let num_cities = tsp.coordinates.len();
    let mut memo: HashMap<(usize, usize), f64> = HashMap::new();
    let mut path: HashMap<(usize, usize), usize> = HashMap::new();

    fn visit(
        pos: usize,
        visited: usize,
        tsp: &TSP,
        memo: &mut HashMap<(usize, usize), f64>,
        path: &mut HashMap<(usize, usize), usize>,
    ) -> f64 {
        if visited == (1 << tsp.coordinates.len()) - 1 {
            return tsp.distances[pos][0]; // Return to the start city
        }

        if let Some(&cost) = memo.get(&(pos, visited)) {
            return cost;
        }

        let mut result = f64::INFINITY;
        for next in 0..tsp.coordinates.len() {
            if visited & (1 << next) == 0 {
                let new_cost = tsp.distances[pos][next]
                    + visit(next, visited | (1 << next), tsp, memo, path);
                if new_cost < result {
                    result = new_cost;
                    path.insert((pos, visited), next);
                }
            }
        }

        memo.insert((pos, visited), result);
        result
    }

    let start_city = 0;
    visit(start_city, 1 << start_city, tsp, &mut memo, &mut path);

    let mut tour = vec![start_city];
    let mut visited = 1 << start_city;
    let mut current_city = start_city;

    while visited != (1 << num_cities) - 1 {
        let next_city = *path.get(&(current_city, visited)).unwrap();
        tour.push(next_city);
        visited |= 1 << next_city;
        current_city = next_city;
    }

    tour.push(start_city); // Return to the start city
    tour
}

// Nearest Neighbor heuristic for TSP
pub fn solve_with_nearest_neighbor(tsp: &TSP) -> Vec<usize> {
    let mut visited = vec![false; tsp.coordinates.len()];
    let mut route = vec![0]; // Start with the first city
    visited[0] = true;

    while route.len() < tsp.coordinates.len() {
        let current_city = *route.last().unwrap();
        let mut next_city = None;
        let mut min_distance = f64::MAX;

        for city in 0..tsp.coordinates.len() {
            if !visited[city] && tsp.distances[current_city][city] < min_distance {
                min_distance = tsp.distances[current_city][city];
                next_city = Some(city);
            }
        }

        if let Some(city) = next_city {
            route.push(city);
            visited[city] = true;
        }
    }

    route.push(0); // Return to the start city
    route
}
