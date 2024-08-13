use std::f64;

pub struct TSP {
    pub distances: Vec<Vec<f64>>,
    pub coordinates: Vec<(f64, f64)>, // Coordinates of the cities
}

impl TSP {
    // Constructor for creating a new TSP instance
    pub fn new(distances: Vec<Vec<f64>>, coordinates: Vec<(f64, f64)>) -> Self {
        TSP { distances, coordinates }
    }

    // Function for generating a distance matrix between cities
    pub fn generate_distances(coordinates: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
        let num_cities = coordinates.len();
        let mut distances = vec![vec![0.0; num_cities]; num_cities];

        for i in 0..num_cities {
            for j in (i + 1)..num_cities {
                let dx = coordinates[i].0 - coordinates[j].0;
                let dy = coordinates[i].1 - coordinates[j].1;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();
                distances[i][j] = distance;
                distances[j][i] = distance;
            }
        }

        distances
    }

    // Function for calculating the total distance of a route
    pub fn calculate_total_distance(&self, route: &Vec<usize>) -> f64 {
        let mut total_distance = 0.0;
        for i in 0..route.len() - 1 {
            let city1 = route[i];
            let city2 = route[i + 1];
            total_distance += self.distances[city1][city2];
        }
        total_distance
    }
}
