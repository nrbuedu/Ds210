use petgraph::graph::DiGraph;
use petgraph::algo::floyd_warshall;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Function to analyze if the degree distribution follows a power law
pub fn analyze_power_law(degree_distribution: &HashMap<usize, usize>) -> bool {
    let total_nodes: usize = degree_distribution.values().sum();
    let max_degree = *degree_distribution.keys().max().unwrap_or(&0);
    
    let low_degree_nodes: usize = degree_distribution.iter()
        .filter(|(k, _)| **k <= 2)
        .map(|(_, v)| *v)
        .sum();
    
    let high_degree_nodes: usize = degree_distribution.iter()
        .filter(|(k, _)| **k >= (max_degree as f64 * 0.8) as usize)
        .map(|(_, v)| *v)
        .sum();
    
    low_degree_nodes > (total_nodes * 60) / 100 && high_degree_nodes < (total_nodes * 5) / 100
}

// Function to export the degree distribution to a CSV file
pub fn export_degree_distribution(distribution: &HashMap<usize, usize>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "degree,count")?;
    
    for (degree, count) in distribution {
        writeln!(file, "{},{}", degree, count)?;
    }
    
    Ok(())
}

// Function to calculate the average path length (in hops) between all pairs of nodes
pub fn calculate_average_path_length(graph: &DiGraph<String, i32>) -> f64 {
    let path_matrix = floyd_warshall(&graph, |e| *e.weight()); // Use Floyd-Warshall to get all-pairs shortest paths

    match path_matrix {
        Ok(distances) => {
            let mut total_path_length = 0; // Total path length
            let mut total_pairs = 0; // Total number of pairs of nodes

            // Count only connected pairs
            for i in graph.node_indices() {
                for j in graph.node_indices() {
                    if i != j {
                        if let Some(&distance) = distances.get(&(i, j)) { // Get the shortest path between nodes
                            total_path_length += distance;
                            total_pairs += 1;
                        }
                    }
                }
            }

            if total_pairs == 0 {
                return 0.0; // Avoid division by zero if there are no pairs
            }

            total_path_length as f64 / total_pairs as f64 // Calculate the average path length
        },
        Err(_) => 0.0, // Return 0 if there's an error calculating the paths
    }
}