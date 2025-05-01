
// The flight module handles reading flight data from a CSV file.
// It defines a Flight struct to represent each flight with an origin and destination airport.

use std::error::Error;
use csv::Reader;
use std::collections::HashSet;

// Flight struct represents a flight with an origin and destination airport
#[derive(Debug)]
pub struct Flight {
    pub origin: String,
    pub destination: String,
}

// Function to read the flight data from a CSV file and return a vector of Flight structs
// This function parses the CSV and constructs a list of Flight objects
pub fn read_csv(file_path: &str) -> Result<Vec<Flight>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut flights = Vec::new();
    let mut added_edges = HashSet::new();

    for result in rdr.records() {
        let record = result?;
        let origin = record[0].to_string();
        let destination = record[1].to_string();

        let edge = (origin.clone(), destination.clone());
        if !origin.is_empty() && !destination.is_empty() && !added_edges.contains(&edge) {
            flights.push(Flight { origin, destination });
            added_edges.insert(edge);
        }
    }
    Ok(flights) // Return the list of flights
}
