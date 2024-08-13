# Supplementary Materials

This document provides additional resources and materials that complement the main content of the *Traveling Salesman Project*. These materials include extended data, code snippets, examples, and any other content that might be useful for deeper understanding or further experimentation.

## 1. Extended Data

### 1.1. Additional City Coordinates

- **Description:** A supplementary set of city coordinates not included in the main datasets. These can be used for additional testing or experimentation.
- **File:** `data/extra_cities_coordinates.csv`
- **Format:** CSV file with latitude and longitude columns.

### 1.2. Benchmark Results

- **Description:** Extended results from the performance benchmarks conducted in the project, including raw data and additional metrics not covered in the main performance analysis.
- **File:** `outputs/benchmark_extended_results.csv`
- **Format:** CSV file with columns for each algorithm, dataset size, total distance, execution time, and additional metrics like memory usage.

## 2. Code Snippets

### 2.1. Custom Algorithm Integration

- **Objective:** Example code for integrating a new custom algorithm into the existing framework.
- **Snippet:**

```rust
// In src/algorithms/custom_algorithm.rs

pub fn solve_with_custom(tsp: &TSP) -> Vec<usize> {
    // Initialize your custom algorithm here
    let mut route = Vec::new();
    
    // Implement your logic
    // For example, a basic random shuffle:
    let mut cities: Vec<usize> = (0..tsp.coordinates.len()).collect();
    cities.shuffle(&mut rand::thread_rng());
    
    route.push(0); // Starting city
    route.extend(cities);
    route.push(0); // Return to the start
    
    route
}
```

- **Usage:** To use this custom algorithm, add a reference in `src/algorithms/mod.rs` and update the main entry point in `src/main.rs`.

### 2.2. Visualizing Custom Data

- **Objective:** Example Python code for visualizing custom TSP routes using Matplotlib.
- **Snippet:**

```python
import matplotlib.pyplot as plt

def visualize_custom_route(coordinates, route):
    x = [coordinates[i][0] for i in route]
    y = [coordinates[i][1] for i in route]
    
    plt.figure(figsize=(10, 6))
    plt.plot(x, y, 'o-', markersize=10, color='blue', label='Route')
    plt.title('Custom TSP Route Visualization')
    plt.xlabel('Latitude')
    plt.ylabel('Longitude')
    plt.grid(True)
    plt.legend()
    plt.show()

# Example usage
coordinates = [(52.5200, 13.4050), (48.8566, 2.3522), (51.5074, -0.1278)]
route = [0, 2, 1, 0]
visualize_custom_route(coordinates, route)
```

- **Usage:** Replace the `coordinates` and `route` with your own data to visualize custom routes.

## 3. Example Scenarios

### 3.1. Real-World Logistics Problem

- **Scenario:** A company needs to optimize delivery routes across major European cities.
- **Data:** Use `data/real_world_europe_cities.csv`.
- **Suggested Algorithm:** Start with `ml_initialization.rs` for initial route prediction, followed by `hybrid_algorithms.rs` for optimization.
- **Expected Outcome:** A balanced route that minimizes travel distance and time, with the flexibility to add or remove cities as needed.

### 3.2. Academic Research Extension

- **Scenario:** Extending the project for academic research on hybrid optimization methods.
- **Approach:** Implement and compare additional hybrid techniques, such as combining Ant Colony Optimization with Genetic Algorithms.
- **Metrics:** Evaluate performance using both traditional metrics (total distance, execution time) and new ones (e.g., algorithm convergence speed, robustness across datasets).

## 4. Additional Visualizations

- **Description:** A collection of additional visualizations generated during the project, including:
  - 3D route visualizations for geographic data.
  - Interactive maps with additional layers (e.g., traffic data).
  - Comparative plots of algorithm performance across different datasets.

- **Files:** 
  - `outputs/3d_visualizations/`
  - `outputs/interactive_maps/`

## 5. Links to Further Reading

### 5.1. Books

- **"The Traveling Salesman Problem: A Computational Study" by David L. Applegate, Robert E. Bixby, Vasek Chvátal, and William J. Cook**
  - An in-depth exploration of the TSP, including theoretical and practical aspects.

- **"Combinatorial Optimization: Algorithms and Complexity" by Christos H. Papadimitriou, Kenneth Steiglitz**
  - A foundational text on combinatorial optimization, covering a range of problems including the TSP.

### 5.2. Online Resources

- **GeeksforGeeks: Introduction to Genetic Algorithms**
  - [Link](https://www.geeksforgeeks.org/genetic-algorithms/)
  - A beginner-friendly guide to understanding and implementing Genetic Algorithms.

- **Rust Book**
  - [Link](https://doc.rust-lang.org/book/)
  - The official guide to learning Rust, useful for anyone contributing to the project.

## Conclusion

The supplementary materials provided here are intended to support deeper exploration and application of the *Traveling Salesman Project*. Whether you are conducting further research, developing new features, or simply exploring the project's capabilities, these materials should serve as a valuable resource.
