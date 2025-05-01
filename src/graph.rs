// This module is responsible for converting the flight data into a directed graph of airports.
// The graph represents airports as nodes and flights as directed edges between those nodes.

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};
use crate::flight::Flight;  


pub struct GraphFlight {
    pub origin: String,
    pub destination: String,
}

// Function to build a graph of airports from the flight data
// It takes a list of flights and returns a directed graph with airports as nodes and flights as edges.
pub fn build_graph(flights: Vec<Flight>) -> DiGraph<String, i32> {
    let mut graph = DiGraph::new();
    let mut airport_map: HashMap<String, NodeIndex> = HashMap::new();

    for flight in flights {
        let origin_node = *airport_map.entry(flight.origin.clone())
            .or_insert_with(|| graph.add_node(flight.origin.clone()));

        let dest_node = *airport_map.entry(flight.destination.clone())
            .or_insert_with(|| graph.add_node(flight.destination.clone()));

        // Only add one direction for the flight (no bidirectional edges)
        graph.add_edge(origin_node, dest_node, 1);
    }

    graph
}



// Function to calculate the degree distribution of the graph
// This function computes how many airports (nodes) have a certain number of connections (degree).
pub fn calc_degree_distribution(graph: &DiGraph<String, i32>) -> HashMap<usize, usize> {
    let mut degree_distribution = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.edges(node)
            .filter(|edge| edge.source() != edge.target())
            .count();
        *degree_distribution.entry(degree).or_insert(0) += 1;
    }

    degree_distribution
}

// Function to get the top N airports with the most connections (hubs)
// This function sorts the airports by degree (number of connections) and returns the top N.
pub fn get_airport_hubs(graph: &DiGraph<String, i32>, top_n: usize) -> Vec<(String, usize)> {
    let mut airport_degrees: Vec<(String, usize)> = graph.node_indices()
        .map(|node| (graph[node].clone(), graph.edges(node).count()))
        .collect();

    airport_degrees.sort_by(|a, b| b.1.cmp(&a.1));
    airport_degrees.into_iter().take(top_n).collect()
}
