# Algorithms Documentation

This document provides a detailed overview of the algorithms implemented in the *Traveling Salesman Project*. Each algorithm is described with its theoretical background, implementation details, and complexity analysis.

## 1. Dynamic Programming (DP)

### Overview

Dynamic Programming is an optimization technique that solves complex problems by breaking them down into simpler subproblems. It is particularly useful for the Traveling Salesman Problem (TSP) when the number of cities is relatively small.

### Theoretical Background

The DP approach for TSP works by storing the results of subproblems in a memoization table to avoid redundant calculations. The algorithm considers all possible subsets of cities, calculating the minimum cost to visit each subset and return to the starting city.

### Implementation

The DP algorithm is implemented in the `src/algorithms/base_algorithms.rs` file. It uses a recursive function with memoization to compute the shortest possible route.

### Complexity

- **Time Complexity:** \(O(n^2 \times 2^n)\), where \(n\) is the number of cities.
- **Space Complexity:** \(O(n \times 2^n)\) due to the memoization table.

## 2. Nearest Neighbor Heuristic (NN)

### Overview

The Nearest Neighbor (NN) heuristic is a greedy algorithm that constructs a path by repeatedly visiting the nearest unvisited city. While it is fast, it does not always produce the optimal solution.

### Theoretical Background

This algorithm starts at an arbitrary city, then repeatedly selects the closest unvisited city until all cities have been visited. The simplicity of the NN heuristic makes it a popular choice for approximations, but it often falls into local optima.

### Implementation

The NN algorithm is implemented in the `src/algorithms/base_algorithms.rs` file. It iterates over the list of cities, selecting the nearest unvisited city at each step.

### Complexity

- **Time Complexity:** \(O(n^2)\), where \(n\) is the number of cities.
- **Space Complexity:** \(O(n)\) for storing the route.

## 3. Genetic Algorithm (GA)

### Overview

Genetic Algorithms are a type of evolutionary algorithm inspired by the process of natural selection. They are particularly effective for finding near-optimal solutions in large search spaces, such as TSP.

### Theoretical Background

The GA operates on a population of potential solutions, using selection, crossover, and mutation to evolve the population over successive generations. The fittest individuals are selected to create offspring, which replace the less fit individuals in the population.

### Implementation

The GA is implemented in the `src/algorithms/genetic_algorithm.rs` file. It includes components such as population initialization, fitness evaluation, selection, crossover, and mutation.

### Complexity

- **Time Complexity:** Dependent on the number of generations and population size, typically \(O(g \times p \times n)\), where \(g\) is the number of generations, \(p\) is the population size, and \(n\) is the number of cities.
- **Space Complexity:** \(O(p \times n)\) for storing the population.

## 4. Hybrid Algorithm

### Overview

The Hybrid Algorithm combines the Genetic Algorithm with local optimization techniques to improve the quality of the solution. This approach seeks to combine the global search capability of GA with the fine-tuning ability of local search.

### Theoretical Background

After generating an initial solution using GA, the Hybrid Algorithm applies a local search to refine the route by exploring neighboring solutions. This process helps to escape local optima and achieve a better overall solution.

### Implementation

The Hybrid Algorithm is implemented in the `src/algorithms/hybrid_algorithms.rs` file. It first calls the GA and then refines the solution using a local search technique.

### Complexity

- **Time Complexity:** Similar to GA, with additional overhead for the local search.
- **Space Complexity:** \(O(p \times n)\), with additional space for local search operations.

## 5. Machine Learning Initialization (ML)

### Overview

This approach uses a machine learning model to predict an initial route, which can then be optimized using other techniques. The ML model is trained on historical data to learn patterns that help in route optimization.

### Theoretical Background

The model is trained using city coordinates as input and the optimal or near-optimal routes as output. The ML model serves as a heuristic to provide a good starting point for further optimization.

### Implementation

The ML Initialization is implemented in the `src/algorithms/ml_initialization.rs` file. The model is built and trained using TensorFlow, and the resulting model is used to predict routes.

### Complexity

- **Time Complexity:** Dependent on the model's architecture and input size, typically \(O(n)\) for inference.
- **Space Complexity:** Dependent on the size of the trained model.

## 6. Multi-objective Optimization (MOO)

### Overview

Multi-objective Optimization considers multiple objectives simultaneously, such as minimizing distance and the number of turns. It is particularly useful when the TSP has more than one criterion that needs to be optimized.

### Theoretical Background

The MOO algorithm generates a set of Pareto-optimal solutions, where no solution can be improved in one objective without degrading another. The algorithm uses a population-based approach, similar to GA, but evaluates multiple objectives.

### Implementation

The MOO is implemented in the `src/algorithms/multi_objective_optimization.rs` file. It uses a modified GA to optimize multiple objectives simultaneously.

### Complexity

- **Time Complexity:** Similar to GA but with additional overhead for evaluating multiple objectives.
- **Space Complexity:** \(O(p \times n)\) for storing the population and objectives.

## Conclusion

This document provides an in-depth look at the algorithms implemented in the *Traveling Salesman Project*. Each algorithm has been chosen for its strengths in solving different aspects of the TSP. The combination of these algorithms allows for a comprehensive exploration of solutions to this classic optimization problem.

