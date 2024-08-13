import tensorflow as tf
import numpy as np
import csv

def load_data(file_path):
    """Load TSP data from a CSV file."""
    coordinates = []
    with open(file_path, 'r') as file:
        reader = csv.reader(file)
        next(reader)  # Skip header
        for row in reader:
            latitude = float(row[0])
            longitude = float(row[1])
            coordinates.append([latitude, longitude])
    return np.array(coordinates)

def build_model(input_shape):
    """Build a simple neural network model."""
    model = tf.keras.Sequential([
        tf.keras.layers.Dense(64, activation='relu', input_shape=input_shape),
        tf.keras.layers.Dense(64, activation='relu'),
        tf.keras.layers.Dense(input_shape[0], activation='softmax')
    ])
    model.compile(optimizer='adam', loss='mse', metrics=['accuracy'])
    return model

def train_model(model, data, epochs=100):
    """Train the model with provided data."""
    model.fit(data, data, epochs=epochs)
    return model

def save_model(model, file_path):
    """Save the trained model to a file."""
    model.save(file_path)

def load_model(file_path):
    """Load a pre-trained model from a file."""
    return tf.keras.models.load_model(file_path) 

if __name__ == "__main__":
    # Load the city coordinates
    data = load_data('data/cities_coordinates.csv') # Load the data

    # Build the model
    model = build_model((data.shape[1],)) # Build the model

    # Train the model
    trained_model = train_model(model, data, epochs=500)   # Train the model

    # Save the model to a file
    save_model(trained_model, 'data/machine_learning_model.keras')  # Save the model

    # Example of loading the model
    loaded_model = load_model('data/machine_learning_model.keras')  # Load the model
    print("Model loaded successfully.")
