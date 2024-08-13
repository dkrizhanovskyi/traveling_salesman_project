# Experiments Documentation

This document provides a detailed overview of the experiments conducted as part of the *Traveling Salesman Project*. It includes the experimental setup, the procedures followed, and the results obtained. The goal of these experiments was to evaluate the performance of different algorithms on various datasets and to explore potential optimizations.

## Experimental Setup

### Hardware and Software Environment

- **CPU:** Intel Core i7
- **RAM:** 32 GB
- **Operating System:** Linux Mint 20.2
- **Rust Version:** 1.73.0
- **Python Version:** 3.12.0
- **TensorFlow Version:** 2.x

### Datasets

- **Synthetic Data:** Datasets were generated with city coordinates randomly distributed in a 2D plane. The number of cities varied from 10 to 100 to evaluate the scalability of the algorithms.
- **Real-World Data:** Datasets with actual geographical coordinates of cities in various countries (e.g., major cities in Europe, the United States) were used to test the algorithms on more realistic scenarios.

### Algorithms Tested

The following algorithms were evaluated:

1. **Dynamic Programming (DP)**
2. **Nearest Neighbor Heuristic (NN)**
3. **Genetic Algorithm (GA)**
4. **Hybrid Algorithm**
5. **Machine Learning Initialization (ML)**
6. **Multi-objective Optimization (MOO)**

## Experimental Procedures

### Experiment 1: Baseline Performance

**Objective:** To establish a baseline performance for each algorithm on small datasets (10-20 cities).

- **Procedure:**
  1. Run each algorithm on datasets with 10, 15, and 20 cities.
  2. Record the total distance and execution time for each run.
  3. Repeat each run 5 times to account for variability and average the results.

- **Results:** 
  - Dynamic Programming provided the optimal solution for all cases but was slow for datasets with more than 15 cities.
  - The Nearest Neighbor heuristic was the fastest but produced the least optimal routes.

### Experiment 2: Scalability Testing

**Objective:** To test the scalability of the algorithms as the number of cities increases.

- **Procedure:**
  1. Gradually increase the number of cities from 20 to 100 in increments of 20.
  2. Run each algorithm and measure execution time and total distance.
  3. Identify the point at which each algorithm's performance begins to degrade significantly.

- **Results:** 
  - The Genetic Algorithm and Hybrid Algorithm scaled better than Dynamic Programming, maintaining reasonable execution times and near-optimal routes up to 100 cities.
  - Machine Learning Initialization demonstrated very low execution times but with some variability in route quality.

### Experiment 3: Multi-objective Optimization

**Objective:** To evaluate the effectiveness of Multi-objective Optimization (MOO) in balancing multiple criteria (e.g., minimizing both total distance and number of turns).

- **Procedure:**
  1. Define two objectives: minimizing total distance and minimizing the number of turns in the route.
  2. Run the MOO algorithm on a dataset with 50 cities.
  3. Analyze the Pareto front to identify trade-offs between the two objectives.

- **Results:** 
  - MOO successfully generated a set of Pareto-optimal solutions, showing a clear trade-off between distance and number of turns.
  - The results suggest that MOO is particularly useful in scenarios where multiple criteria are important.

### Experiment 4: Real-World Data Testing

**Objective:** To evaluate the algorithms on real-world geographical data.

- **Procedure:**
  1. Use datasets containing the coordinates of major cities in Europe and the United States.
  2. Run each algorithm and compare the results with known optimal or near-optimal solutions from the literature.

- **Results:** 
  - The Hybrid Algorithm performed well on real-world data, providing routes close to known optimal solutions.
  - Machine Learning Initialization showed potential for large-scale real-world applications, especially when combined with further optimization techniques.

## Discussion

### Key Findings

- **Algorithm Efficiency:** Dynamic Programming is only practical for small datasets due to its exponential time complexity. For larger datasets, heuristic and evolutionary algorithms like the Genetic Algorithm and Hybrid Algorithm are more effective.
- **Scalability:** Machine Learning Initialization provides a scalable solution, especially when used as a starting point for more complex algorithms.
- **Multi-objective Trade-offs:** The Multi-objective Optimization approach is valuable in applications where multiple criteria must be balanced, offering a set of solutions that allow for flexibility in decision-making.

### Limitations

- **Computational Resources:** The experiments were limited by available computational resources, particularly for large datasets and complex algorithms.
- **Generalization:** While the algorithms performed well on the tested datasets, further research is needed to generalize these results to other TSP instances and variations.

### Future Work

- **Parameter Tuning:** Future experiments could focus on optimizing the parameters of the Genetic Algorithm and Hybrid Algorithm to further improve their performance.
- **Algorithmic Enhancements:** Incorporating advanced techniques, such as metaheuristics or hybridizing additional algorithms, could lead to better results.
- **Real-Time Applications:** Testing the algorithms in real-time applications, such as logistics or robotics, would provide valuable insights into their practical utility.

## Conclusion

The experiments conducted in the *Traveling Salesman Project* provide a comprehensive evaluation of various algorithms for solving the TSP. The results highlight the strengths and weaknesses of each approach and suggest several avenues for future research and optimization. By understanding the experimental procedures and outcomes, researchers and developers can build upon this work to further advance the field of optimization algorithms.

