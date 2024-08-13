# Data Documentation

This document provides a detailed overview of the data used in the *Traveling Salesman Project*. It includes information on data formats, sources, preprocessing steps, and how the data is used in the project.

## Data Overview

The project uses a variety of data related to the Traveling Salesman Problem (TSP), including city coordinates and route information. The data is stored in CSV files and is processed by the Rust and Python scripts within the project.

### Data Files

The following data files are included in the `data/` directory:

1. **cities_coordinates.csv**

   - **Description:** This file contains the latitude and longitude coordinates of the cities used in the TSP simulations.
   - **Format:** CSV file with two columns: `latitude` and `longitude`.
   - **Example:**
     ```csv
     latitude,longitude
     52.5200,13.4050
     48.8566,2.3522
     51.5074,-0.1278
     40.7128,-74.0060
     ```

2. **machine_learning_model.keras**

   - **Description:** This file contains a pre-trained TensorFlow model used for initializing routes in the ML Initialization algorithm.
   - **Format:** Keras model file in the `.keras` format, which is compatible with TensorFlow.
   - **Usage:** The model is loaded by the `ml_initialization.rs` script to predict an initial route for TSP.

### Data Sources

The coordinates in `cities_coordinates.csv` can be based on real-world data or synthetically generated for testing purposes. For real-world applications, city coordinates can be sourced from geographical databases or APIs, such as OpenStreetMap or Google Maps.

### Data Preprocessing

Before using the data in algorithms, the following preprocessing steps are typically applied:

1. **Normalization:**
   - City coordinates are sometimes normalized to fit within a specific range, making them easier to process by algorithms.

2. **Distance Matrix Generation:**
   - The project automatically generates a distance matrix from the city coordinates, which is used by various TSP algorithms. The distance matrix is stored in memory during execution and is not persisted in a separate file.

3. **Data Validation:**
   - Input data is validated to ensure that coordinates are correctly formatted and within valid ranges (e.g., latitude between -90 and 90 degrees, longitude between -180 and 180 degrees).

### Data Usage in the Project

The data in this project serves several purposes:

- **Algorithm Input:**
  - City coordinates are the primary input for the TSP algorithms, which use these coordinates to calculate distances and optimize routes.

- **Model Training:**
  - For the Machine Learning Initialization, the city coordinates and corresponding routes are used to train the TensorFlow model. The model learns to predict near-optimal routes based on the input coordinates.

- **Performance Analysis:**
  - The generated routes and distances are stored in the `outputs/` directory for further analysis. These outputs are used to compare the performance of different algorithms.

### Handling Custom Data

If you want to use your own data, follow these steps:

1. **Prepare Your Data:**
   - Ensure your city coordinates are in a CSV file with the same format as `cities_coordinates.csv`.

2. **Update the File Path:**
   - Replace the default `cities_coordinates.csv` file with your custom file or update the file path in the relevant scripts.

3. **Run the Algorithms:**
   - Use the provided Rust or Python scripts to process your data and generate results.

## Conclusion

This document provides an overview of the data used in the *Traveling Salesman Project*. Understanding the data and how it is processed is crucial for correctly using the project's algorithms and models. If you need to use custom data or modify the existing data handling, this guide should serve as a useful reference.

