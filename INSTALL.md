# Installation Guide

This document provides detailed instructions on how to install and set up the *Traveling Salesman Project* on your local machine.

## Prerequisites

Before you begin, ensure that you have the following software installed on your system:

- **Rust (Edition 2021)**
  - Installation: Follow the instructions on the official [Rust website](https://www.rust-lang.org/learn/get-started) to install Rust and Cargo.
  
- **Python 3.8+**
  - Installation: Download and install Python from the official [Python website](https://www.python.org/downloads/).

- **TensorFlow 2.x**
  - Installation: TensorFlow can be installed using pip:
    ```sh
    pip install tensorflow
    ```

- **Folium**
  - Installation: Folium can be installed using pip:
    ```sh
    pip install folium
    ```

- **Pandas and Matplotlib**
  - Installation: Install both using pip:
    ```sh
    pip install pandas matplotlib
    ```

## Setting Up the Project

### Step 1: Clone the Repository

Clone the repository from GitHub to your local machine:

```sh
git clone https://github.com/dkrizhanovskyi/traveling_salesman_project.git
cd traveling_salesman_project
```

### Step 2: Install Rust Dependencies

Compile and install the Rust dependencies using Cargo:

```sh
cargo build --release
```

This command will download and compile all the necessary Rust libraries as specified in the `Cargo.toml` file.

### Step 3: Install Python Dependencies

Install the required Python packages by running:

```sh
pip install -r requirements.txt
```

This command installs all the necessary Python libraries listed in the `requirements.txt` file, including TensorFlow, Folium, Pandas, and Matplotlib.

## Verifying the Installation

After installing the dependencies, you can verify the installation by running the following commands:

1. **Check Rust Version:**
   ```sh
   rustc --version
   cargo --version
   ```

2. **Check Python Version:**
   ```sh
   python --version
   pip --version
   ```

3. **Run the Project:**
   ```sh
   cargo run --release
   ```

   If everything is set up correctly, the project should compile and run without errors.

## Troubleshooting

- **Rust Installation Issues:** Ensure that your Rust installation is up-to-date. You can update Rust by running:
  ```sh
  rustup update
  ```

- **Python Dependency Errors:** If you encounter issues installing Python packages, ensure that your pip is updated:
  ```sh
  pip install --upgrade pip
  ```

- **TensorFlow Compatibility:** Ensure that your Python version is compatible with the version of TensorFlow you are installing. TensorFlow 2.x generally supports Python 3.6 and above.

For additional help, refer to the [official documentation](https://www.tensorflow.org/install) of TensorFlow, Rust, and other tools.

## Conclusion

After following these steps, you should have a fully functioning environment ready to explore the *Traveling Salesman Project*. If you encounter any issues, please refer to the Troubleshooting section or contact the project maintainer.
