import pandas as pd
import matplotlib.pyplot as plt

def analyze_performance(csv_file):
    # Read data from CSV
    df = pd.read_csv(csv_file)

    # Visualization of execution time
    plt.figure(figsize=(10, 6))
    plt.bar(df['Algorithm'], df['Execution Time (s)'], color='skyblue')
    plt.xlabel('Algorithm')
    plt.ylabel('Execution Time (s)')
    plt.title('Performance Analysis')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

    # Visualization of total distance
    plt.figure(figsize=(10, 6))
    plt.bar(df['Algorithm'], df['Total Distance'], color='lightgreen')
    plt.xlabel('Algorithm')
    plt.ylabel('Total Distance')
    plt.title('Distance Analysis')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    analyze_performance("outputs/performance_results.csv")
