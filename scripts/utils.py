import csv

def read_csv(file_path):
    """Read data from a CSV file and return a list of rows."""
    with open(file_path, mode='r') as file:
        reader = csv.reader(file)
        data = [row for row in reader]
    return data

def save_to_csv(data, file_path):
    """Save data to a CSV file."""
    with open(file_path, mode='w', newline='') as file:
        writer = csv.writer(file)
        for row in data:
            writer.writerow(row)
