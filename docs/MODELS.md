# Models Documentation

This document provides a detailed overview of the machine learning models used in the *Traveling Salesman Project*. It includes information on the model architecture, training process, and how the models are integrated into the project.

## Overview

The project includes a machine learning model that is used for initializing routes in the Traveling Salesman Problem (TSP). The model is trained to predict an initial route based on city coordinates, which can then be further optimized by other algorithms.

### Model File

- **machine_learning_model.keras**
  - **Format:** Keras model file (HDF5 format) compatible with TensorFlow.
  - **Location:** Stored in the `data/` directory.

## Model Architecture

The model used in this project is a simple feedforward neural network (also known as a multilayer perceptron) designed to predict a sequence of city visits given a set of coordinates.

### Layers

1. **Input Layer:**
   - **Shape:** `(n, 2)` where `n` is the number of cities.
   - **Details:** Accepts the latitude and longitude of each city as input.

2. **Hidden Layers:**
   - **Layer 1:** Fully connected layer with 64 units and ReLU activation.
   - **Layer 2:** Fully connected layer with 64 units and ReLU activation.

3. **Output Layer:**
   - **Shape:** `(n,)` where `n` is the number of cities.
   - **Details:** Produces a softmax output that represents the probability distribution over city indices.

### Summary

```plaintext
Model: "sequential"
_________________________________________________________________
Layer (type)                 Output Shape              Param #   
=================================================================
dense (Dense)                (None, 64)                192       
_________________________________________________________________
dense_1 (Dense)              (None, 64)                4160      
_________________________________________________________________
dense_2 (Dense)              (None, n)                 n*65     
=================================================================
Total params: (n*65) + 4352
Trainable params: (n*65) + 4352
Non-trainable params: 0
_________________________________________________________________
```

## Training Process

### Data Preparation

- **Input:** Latitude and longitude coordinates of cities.
- **Output:** A sequence of city indices representing the initial route.

### Training Procedure

1. **Data Splitting:**
   - The dataset is split into training and validation sets. Typically, 80% of the data is used for training, and 20% is used for validation.

2. **Training:**
   - The model is trained using the Adam optimizer and mean squared error (MSE) as the loss function. Training is performed over 100 epochs with a batch size of 32.
   - During training, the model learns to minimize the loss, effectively learning to predict a sequence that represents an efficient route.

3. **Validation:**
   - The model's performance is monitored using the validation set, ensuring that the model does not overfit to the training data.

### Example Code

The following code snippet is used to train the model:

```python
model = tf.keras.Sequential([
    tf.keras.layers.Dense(64, activation='relu', input_shape=(2,)),
    tf.keras.layers.Dense(64, activation='relu'),
    tf.keras.layers.Dense(n, activation='softmax')
])

model.compile(optimizer='adam', loss='mse', metrics=['accuracy'])

model.fit(train_data, train_labels, epochs=100, validation_data=(val_data, val_labels))
```

## Model Integration

### Inference

The trained model is used in the `ml_initialization.rs` script to predict an initial route based on the input coordinates:

```python
# Load the trained model
model = tf.keras.models.load_model('data/machine_learning_model.keras')

# Predict the route
predicted_route = model.predict(coordinates)
```

### Performance

While the ML-based route initialization may not always produce the optimal route, it serves as a good starting point for further optimization. The model's predictions are particularly useful in scenarios where the number of cities is large and other algorithms may benefit from a pre-optimized starting route.

## Future Improvements

- **Model Complexity:** The current model is relatively simple. More complex architectures, such as recurrent neural networks (RNNs) or transformer models, could be explored to potentially improve performance.
- **Feature Engineering:** Additional features, such as city population or distance from a central point, could be incorporated to enhance the model's predictive power.
- **Transfer Learning:** Pre-trained models on similar optimization problems could be fine-tuned for the TSP, potentially improving the initialization process.

## Conclusion

This document provides an overview of the machine learning model used in the *Traveling Salesman Project*. The model plays a crucial role in initializing routes for further optimization by other algorithms. By understanding the model's architecture, training process, and integration, users can better utilize and potentially improve the model for their specific needs.
