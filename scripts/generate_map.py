import folium
import csv

def generate_map(route_csv, output_html):
    # Create a map object centered on a general location
    map_ = folium.Map(location=[50.0, 10.0], zoom_start=4)

    # Read the route data from CSV
    route = []
    with open(route_csv, 'r') as file:
        reader = csv.reader(file)
        next(reader)  # Skip the header
        for row in reader:
            city_index = int(row[0])
            latitude = float(row[1])
            longitude = float(row[2])
            route.append((latitude, longitude, city_index))

    # Add the route to the map
    folium.PolyLine([(lat, lon) for lat, lon, _ in route], color="blue", weight=2.5, opacity=1).add_to(map_)

    # Add markers for the cities
    for lat, lon, idx in route:
        folium.Marker([lat, lon], popup=f'City {idx}').add_to(map_)

    # Save the map to an HTML file
    map_.save(output_html)

if __name__ == "__main__":
    generate_map("outputs/ga_route.csv", "outputs/tsp_route_map.html")
