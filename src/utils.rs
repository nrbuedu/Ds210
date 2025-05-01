// The utils module contains auxiliary functions for analyzing the degree distribution,
// checking if the distribution follows a power law, and exporting the data to a CSV file.
use petgraph::graph::DiGraph;
use petgraph::algo::floyd_warshall;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Function to analyze if the degree distribution follows a power law
// It checks if more than 60% of nodes have low degrees and fewer than 5% have high degrees.
pub fn analyze_power_law(degree_distribution: &HashMap<usize, usize>) -> bool {
    let total_nodes: usize = degree_distribution.values().sum();
    let max_degree = *degree_distribution.keys().max().unwrap_or(&0);
    
    // Explicit type for low_degree_nodes
    let low_degree_nodes: usize = degree_distribution.iter()
        .filter(|(k, _)| **k <= 2)
        .map(|(_, v)| *v)
        .sum();
    
    // Explicit type for high_degree_nodes
    let high_degree_nodes: usize = degree_distribution.iter()
        .filter(|(k, _)| **k >= (max_degree as f64 * 0.8) as usize)
        .map(|(_, v)| *v)
        .sum();
    
    low_degree_nodes > (total_nodes * 60) / 100 && high_degree_nodes < (total_nodes * 5) / 100
}

// Function to export the degree distribution to a CSV file
// This function writes the degree distribution (degree, count) pairs to a CSV file.
pub fn export_degree_distribution(distribution: &HashMap<usize, usize>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "degree,count")?;
    for (degree, count) in distribution {
        writeln!(file, "{},{}", degree, count)?;
    }
    Ok(())
}

// Function to calculate the average path length (in hops) between all pairs of nodes
// It calculates the average number of hops required to travel between all pairs of nodes.
// utils.rs

pub fn calculate_average_path_length(graph: &DiGraph<String, i32>) -> f64 {
    let path_matrix = floyd_warshall(&graph, |e| *e.weight());

    match path_matrix {
        Ok(distances) => {
            let mut total_path_length = 0u64; // Use a larger type to avoid overflow
            let mut total_pairs = 0;

            for i in graph.node_indices() {
                for j in graph.node_indices() {
                    if i != j {
                        if let Some(&distance) = distances.get(&(i, j)) {
                            total_path_length = total_path_length.saturating_add(distance as u64); // Prevent overflow
                            total_pairs += 1;
                        }
                    }
                }
            }

            if total_pairs == 0 {
                return 0.0;
            }

            total_path_length as f64 / total_pairs as f64 // Calculate average
        },
        Err(_) => 0.0,
    }
}
