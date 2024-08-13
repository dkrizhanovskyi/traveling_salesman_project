# Traveling Salesman Project

## Project Overview

The *Traveling Salesman Project* is a comprehensive solution to the Traveling Salesman Problem (TSP), implemented using Rust and Python. This project explores various algorithms and advanced techniques, including dynamic programming, genetic algorithms, multi-objective optimization, and machine learning, to find the most efficient route that visits a series of cities and returns to the starting point.

This repository is intended for researchers, developers, and enthusiasts who are interested in optimization algorithms, algorithmic research, and machine learning applications in combinatorial problems.

## Features

- **Multiple Algorithms:** Includes dynamic programming, nearest neighbor heuristic, genetic algorithms, hybrid algorithms, machine learning initialization, and multi-objective optimization.
- **Machine Learning Integration:** Utilizes TensorFlow for route initialization based on machine learning models.
- **Parallel Processing:** Leverages Rust's concurrency capabilities through the Rayon library for enhanced performance.
- **API Access:** Provides a RESTful API for solving TSP instances with different algorithms using Warp and Tokio.
- **Data Visualization:** Supports route visualization with `plotters` and map generation using Folium.
- **Performance Analysis:** Includes scripts for detailed performance analysis of each algorithm, with HTML report generation.

## Installation

### Prerequisites

Ensure you have the following installed:

- Rust (Edition 2021)
- Python 3.8+
- TensorFlow 2.x
- Folium
- Pandas, Matplotlib
- Cargo and required Rust dependencies (see `Cargo.toml`)

### Setup

1. Clone the repository:

   ```sh
   git clone https://github.com/dkrizhanovskyi/traveling_salesman_project.git
   cd traveling_salesman_project
   ```

2. Install Rust dependencies:

   ```sh
   cargo build --release
   ```

3. Install Python dependencies:

   ```sh
   pip install -r requirements.txt
   ```

## Usage

### Running the Project

1. **Command Line Interface (CLI):**

   Use the CLI to run the TSP algorithms:

   ```sh
   cargo run --release
   ```

2. **Python Scripts:**

   Analyze the performance of different algorithms using the provided Python scripts:

   ```sh
   python scripts/analyze_performance.py
   ```

3. **API:**

   Start the API server to solve TSP instances via HTTP:

   ```sh
   cargo run --bin tsp_api
   ```

   Then, send a POST request to `http://127.0.0.1:3030/solve` with the coordinates and selected algorithm.

4. **Visualization:**

   Generate visualizations of the routes:

   ```sh
   python scripts/generate_map.py
   ```

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

## Contact Information

For any inquiries or questions, feel free to reach out:

- **Author:** Daniil Krizhanovskyi
- **Email:** daniil.krizhanovskyi@hotmail.com
- **GitHub:** [dkrizhanovskyi](https://github.com/dkrizhanovskyi)