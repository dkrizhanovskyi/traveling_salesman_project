# Changelog

All notable changes to this project will be documented in this file. This project adheres to [Semantic Versioning](https://semver.org/).

## [0.3.0] - 2024-08-14

### Added
- Introduced `ml_initialization.rs` for route prediction using a machine learning model.
- Created a Python script (`generate_map.py`) for generating interactive route maps using Folium.
- Added multi-objective optimization algorithm (`multi_objective_optimization.rs`).
- Implemented detailed performance analysis in `analysis.rs` and provided a script (`analyze_performance.py`) for generating performance reports.
- Included a RESTful API implementation in `api.rs` for solving TSP instances via HTTP requests.

### Changed
- Refined the `hybrid_algorithms.rs` to improve the performance of the hybrid approach by introducing additional local search optimization.
- Updated the visualization module (`visualization.rs`) to enhance the route plotting with additional information like distances between cities.

### Fixed
- Resolved an issue in `genetic_algorithm.rs` where the crossover operation could occasionally produce invalid routes.
- Fixed a bug in `data_handling.rs` that caused incorrect parsing of city coordinates from CSV files under certain conditions.

## [0.2.0] - 2024-07-20

### Added
- Added `genetic_algorithm.rs` implementing a genetic algorithm for solving the TSP.
- Introduced `hybrid_algorithms.rs`, which combines genetic algorithms with local optimization.
- Implemented initial route visualization using the `plotters` library.
- Added a Python script (`machine_learning.py`) for training a simple neural network model to predict routes.

### Changed
- Enhanced the `solve_with_nearest_neighbor` function to improve route calculation by adding a nearest-insertion method.
- Updated the documentation with detailed descriptions of each algorithm and their respective modules.

### Fixed
- Fixed memory leak in `dynamic_programming.rs` by optimizing the memoization table management.
- Resolved an issue with incorrect distance calculation in `tsp.rs`.

## [0.1.0] - 2024-06-01

### Added
- Initial release of the project with basic TSP solutions:
  - Dynamic Programming (`solve_with_dp`).
  - Nearest Neighbor heuristic (`solve_with_nearest_neighbor`).
- Created `tsp.rs` to handle core TSP functionality including distance calculation and route management.
- Set up basic project structure with `Cargo.toml` and necessary dependencies.

### Changed
- N/A

### Fixed
- N/A

