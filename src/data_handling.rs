use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use csv::{ReaderBuilder, WriterBuilder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CityCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

// Function to load city coordinates from a CSV file
pub fn load_coordinates_from_csv(file_path: &str) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut coordinates = Vec::new();

    for result in rdr.deserialize() {
        let record: CityCoordinates = result?;
        coordinates.push((record.latitude, record.longitude));
    }

    Ok(coordinates)
}

// Function to save a route to a CSV file
pub fn save_route_to_csv(route: &Vec<usize>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut writer = WriterBuilder::new().from_writer(BufWriter::new(file));

    writer.write_record(&["City Index"])?;

    for &city in route {
        writer.write_record(&[city.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}

// Function to load real-world data from a CSV file (for advanced use)
pub fn load_real_world_data(file_path: &str) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(BufReader::new(file));
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: CityCoordinates = result?;
        data.push((record.latitude, record.longitude));
    }

    Ok(data)
}
