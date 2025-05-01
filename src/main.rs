// main.rs

mod flight;
mod graph;
mod utils;

use std::error::Error;
use std::collections::HashMap;  // Import HashMap here
use crate::flight::{read_csv, Flight};  // Import Flight struct here
use crate::graph::{build_graph, calc_degree_distribution, get_airport_hubs};
use crate::utils::{analyze_power_law, export_degree_distribution, calculate_average_path_length};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "airports2.csv";
    let flights = read_csv(file_path)?;

    println!("Read {} flights from CSV", flights.len());

    let graph = build_graph(flights); // Build the graph from the flight data
    println!("Built graph with {} airports and {} routes", graph.node_count(), graph.edge_count());

    let degree_distribution = calc_degree_distribution(&graph); // Calculate the degree distribution
    println!("\n--- Degree Distribution Analysis ---");

    let total_airports = graph.node_count();
    let max_degree = degree_distribution.keys().max().unwrap_or(&0);
    let avg_degree: f64 = graph.edge_count() as f64 * 2.0 / total_airports as f64;

    println!("Total airports: {}", total_airports);
    println!("Maximum connections (degree): {}", max_degree);
    println!("Average connections per airport: {:.2}", avg_degree);

    println!("\n--- Top 10 Most Connected Airports ---");
    let top_airports = get_airport_hubs(&graph, 10); // Get the top 10 most connected airports
    for (i, (airport, degree)) in top_airports.iter().enumerate() {
        println!("{}. {} - {} connections", i+1, airport, degree);
    }

    let export_file = "degree_distribution.csv";
    export_degree_distribution(&degree_distribution, export_file)?; // Export the degree distribution
    println!("\nExported degree distribution to {} for plotting", export_file);

    let is_power_law = analyze_power_law(&degree_distribution); // Check if the degree distribution follows a power-law
    println!("\nDoes the degree distribution follow a power law? {}", 
             if is_power_law { "Likely yes" } else { "Likely no" });

    println!("\nCalculating average path length...");
    let average_path_length = calculate_average_path_length(&graph); // Calculate the average path length
    println!("Average Path Length (in hops): {:.2}", average_path_length);

    Ok(())
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::error::Error;
    use crate::flight::Flight;  

    fn create_test_csv() -> Result<(), Box<dyn Error>> {
        let mut file = File::create("test_airports.csv")?;
        writeln!(file, "origin,destination")?;
        writeln!(file, "A,B")?;
        writeln!(file, "B,C")?;
        writeln!(file, "A,C")?;
        Ok(())
    }

    #[test]
    fn test_read_flights_from_csv() {
        // Create a test CSV file before running the test
        create_test_csv().unwrap();

        let flights = read_csv("test_airports.csv").unwrap(); // Test reading flights from the CSV file
        assert_eq!(flights.len(), 3); // Ensure there are 3 flights
        assert_eq!(flights[0].origin, "A"); // Ensure the first flight has the correct origin
        assert_eq!(flights[0].destination, "B"); // Ensure the first flight has the correct destination
    }

    #[test]
    fn test_build_graph() {
        let flights = vec![
            Flight { origin: "A".to_string(), destination: "B".to_string() },
            Flight { origin: "B".to_string(), destination: "C".to_string() },
            Flight { origin: "A".to_string(), destination: "C".to_string() },
        ];

        let graph = build_graph(flights); // Test building the graph
        assert_eq!(graph.node_count(), 3); // Ensure there are 3 nodes
        assert_eq!(graph.edge_count(), 3); // Ensure there are 3 edges (directed graph)
    }

    #[test]
    fn test_calculate_degree_distribution() {
        let flights = vec![
            Flight { origin: "A".to_string(), destination: "B".to_string() },
            Flight { origin: "B".to_string(), destination: "C".to_string() },
            Flight { origin: "A".to_string(), destination: "C".to_string() },
        ];

        let graph = build_graph(flights);
        let degree_distribution = calc_degree_distribution(&graph);

        // In your implementation, degree counts outgoing edges only
        // A has 2 outgoing edges (to B and C)
        // B has 1 outgoing edge (to C)
        // C has 0 outgoing edges
        assert_eq!(degree_distribution.get(&2), Some(&1)); // 1 airport with 2 outgoing connections
        assert_eq!(degree_distribution.get(&1), Some(&1)); // 1 airport with 1 outgoing connection
        assert_eq!(degree_distribution.get(&0), Some(&1)); // 1 airport with 0 outgoing connections
    }

    #[test]
    fn test_analyze_power_law() {
        let mut degree_distribution = HashMap::new();

        degree_distribution.insert(1, 2);
        degree_distribution.insert(2, 3);
        degree_distribution.insert(3, 1);

        let result = analyze_power_law(&degree_distribution); // Test if the degree distribution follows a power-law
        assert!(!result); // The distribution does not follow a power-law
    }

    #[test]
    fn test_calculate_average_path_length() {
        let flights = vec![
            Flight { origin: "A".to_string(), destination: "B".to_string() },
            Flight { origin: "B".to_string(), destination: "C".to_string() },
        ];

        let graph = build_graph(flights);
        
       
        let average_path_length = calculate_average_path_length(&graph);
        
       
        assert!(average_path_length >= 0.0); 
    }
}