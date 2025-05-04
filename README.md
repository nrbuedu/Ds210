Project Overview
•	Goal: This project aims to analyze the flight network between airports in the U.S. by constructing a graph representation of the flight routes and exploring key properties of the network. The main questions being answered are:
1.	What is the degree distribution of airports in terms of their connections (flights)?
2.	What is the average number of hops (steps) needed to travel between any two airports (average path length)?
3.	Does the degree distribution follow a power-law distribution?
4.	Which airports have the most connections?
•	Dataset: The dataset used in this project consists of flight data, with information on the airports involved in each flight route. The data is stored in a CSV file where each row contains an origin airport and a destination airport. The dataset includes over 700 airports and a large number of flight connections (routes). The dataset was sourced from OpenFlights and can be found here - https://www.kaggle.com/datasets/flashgordon/usa-airport-dataset
Data Processing
•	How the data is loaded into Rust: The dataset is loaded into the program using the csv crate. The read_ csv() function reads the data from a CSV file, parsing each row into a Flight struct containing the origin and destination airport names.
•	Cleaning and Transformation: During data loading, any rows where the origin or destination airports are empty are discarded to ensure clean data. The data is then used to construct a graph of flight routes, where each airport is a node, and each flight is an undirected edge between two airports.
The project is divided into four main files: flight.rs handles the parsing of flight data from a CSV file, graph.rs constructs the directed graph of airports and flight routes while calculating the degree distribution, utils.rs contains utility functions for calculating average path length and analyzing power-law distributions, and main.rs orchestrates the program flow by coordinating data loading, graph construction, analysis, and output generation.
Code Structure
1.	Functions:
o	Key functions include:
	read_flights_from_csv(): Loads flight data from a CSV file into a vector of Flight structs.
	build_graph(): Constructs a directed graph using petgraph::DiGraph from the flight data.
	calc_degree_distribution(): Computes how many airports have each possible degree (number of connections).
	analyze_power_law(): Analyzes whether the degree distribution follows a power-law pattern.
	calculate_average_path_length(): Uses the Floyd-Warshall algorithm to calculate the average path length (number of hops) between any two airports in the network.
o	flight struct:
	Purpose: Represents a flight between two airports.
	Fields: origin (the origin airport) and destination (the destination airport).
o	build_graph function:
	Purpose: Converts the list of flight routes into a graph where airports are nodes and flight routes are edges.
	Inputs: A vector of Flight structs.
	Outputs: A directed graph (DiGraph) of airport nodes and edges.
o	calculate_degree_distribution function:
	Purpose: Determines how many airports have each degree (number of flight routes).
	Inputs: A DiGraph representing the flight network.
	Outputs: A HashMap where keys are degrees (number of connections) and values are the count of airports with that degree.
o	analyze_power_law function:
	Purpose: Analyzes if the degree distribution follows a power-law distribution.
	Inputs: A HashMap of the degree distribution.
	Outputs: A boolean indicating if the distribution follows a power-law pattern.
o	calculate_average_path_length function:
	Purpose: Computes the average path length in terms of hops between all pairs of airports.
	Inputs: A DiGraph representing the flight network.
	Outputs: The average path length in hops.
Main Workflow
The main program flow works as follows:
1.	Data Loading: The dataset is loaded from a CSV file into a vector of Flight structs.
2.	Graph Construction: The build_graph() function constructs a graph where airports are nodes and flight routes are edges.
3.	Degree Distribution Calculation: The calculate_degree_distribution() function computes how many airports have each possible degree.
4.	Power-Law Analysis: The analyze_power_law() function checks whether the degree distribution follows a power-law.
5.	Average Path Length Calculation: The calculate_average_path_length() function computes the average path length in hops for the entire network.
6.	Output: The results are printed to the console and exported to a CSV file (degree_distribution.csv).
Tests
The project includes several unit tests to ensure correctness:
•	test_read_flights_from_csv(): Verifies that the CSV file is read correctly and the Flight structs are properly created.
•	test_build_graph(): Verifies that the graph is built with the correct number of nodes and edges.
•	test_calculate_degree_distribution(): Ensures that the degree distribution function correctly counts the degree of each airport.
•	test_analyze_power_law(): Verifies that the power-law analysis function correctly analyzes a simple degree distribution.
Test results can be run using cargo test and the output will indicate whether the tests pass or fail.
Results
The program produces the following outputs:
1.	Degree Distribution:
o	Total Airports: 727 airports are included in the analysis.
o	Maximum Connections (Degree): The airport with the highest number of connections has 333 routes.
o	Average Connections per Airport: On average, each airport has 101.02 connections.
o	Top 10 Most Connected Airports:
1.	YIP – 334 connections
2.	MSP – 292 connections
3.	MEM – 287 connections
4.	ORD – 286 connections
5.	ATL – 280 connections
6.	DFW – 271 connections
7.	MIA – 269 connections
8.	MCI – 267 connections
9.	STL – 264 connections
10.	PHX – 259 connections
o	The list shows the top 10 airports with the highest number of connections. Notably, YIP (has 334 connections, which may seem unusual at first, but it makes sense as I realized it's a cargo port which has many routes 
2.	Power-Law Analysis:
o	Based on the distribution of node degrees, the degree distribution is unlikely to follow a power-law. A power-law distribution typically shows a heavy tail, where most nodes (airports) have few connections and a few nodes have many. Since this is not the case here, the network is likely more evenly distributed, with no single or small group of airports overwhelmingly more connected than others.

3.	Average Path Length:
o	The average path length (in hops) between airports in the network is 2.05, indicating that airports are relatively well-connected across the flight network.
 • How to Build and Run the Code:
•	To build and run the project, use the following commands in your terminal:
	cargo run -- release

Citations
•	AI Assistance: This project used ChatGPT for debugging, and guidance on implementing Rust's graph algorithms and certain functions like Floyd-Warshall for path length calculation.
•	Citations:
o	Petgraph documentation: https://docs.rs/petgraph/
o	OpenFlights dataset: https://openflights.org/data.html

