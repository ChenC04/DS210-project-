use petgraph::Undirected;
use petgraph::Graph;
use petgraph::graph::UnGraph;
use std::collections::HashMap;


// Calculating degree centrality for each genre
pub fn calculate_degree_centrality(graph: &Graph<String, u32, Undirected>) -> HashMap<String, usize> {
    graph.node_indices().map(|node_idx| {
        let degree = graph.edges(node_idx).count();  // Counting the number of edges connected to the node
        let genre_name = graph[node_idx].clone(); 
        (genre_name, degree)
    }).collect()
}
// Calculating betweenness centrality for each node in the graph 
pub fn calculate_betweenness_centrality(graph: &UnGraph<String, u32>) -> HashMap<String, f64> {
    let mut betweenness = HashMap::new();
    // initializing the betweeness score to 0
    for node in graph.node_indices() {
        betweenness.insert(graph[node].clone(), 0.0);
    }

    for s in graph.node_indices() {
        let mut short_paths = HashMap::new(); // Helps track shortest paths from starting point
        let mut short_path_counts = HashMap::new(); //counting the shortest paths through each nodes
        let mut short_distances = HashMap::new(); // finding the distances from each nodes
        let mut stack = Vec::new();

        // Initializating the variables 
        for v in graph.node_indices() {
            short_paths.insert(v, Vec::new());
            short_path_counts.insert(v, 0.0);
            short_distances.insert(v, -1);
        }
        short_path_counts.insert(s, 1.0); 
        short_distances.insert(s, 0); 

        let mut queue = vec![s];
        // Using BFS to find the shortest path
        while let Some(v) = queue.pop() {
            stack.push(v);
            for w in graph.neighbors(v) {
                // checking nodes
                if *short_distances.get(&w).unwrap() == -1 {
                    queue.push(w);
                    short_distances.insert(w, *short_distances.get(&v).unwrap() + 1);
                }
                // updating the path, if its the shortest path 
                if *short_distances.get(&w).unwrap() == *short_distances.get(&v).unwrap() + 1 {
                    let count_v = *short_path_counts.get(&v).unwrap();
                    *short_path_counts.get_mut(&w).unwrap() += count_v;
                    short_paths.get_mut(&w).unwrap().push(v);
                }
            }
        }

        // Computing the dependencies and betweenness 
        let mut dependencies = HashMap::new();
        for v in graph.node_indices() {
            dependencies.insert(v, 0.0);
        }
        while let Some(w) = stack.pop() {
            for v in short_paths.get(&w).unwrap() {
                let c = dependencies.get(v).unwrap() + (short_path_counts.get(v).unwrap() / short_path_counts.get(&w).unwrap()) * (1.0 + dependencies.get(&w).unwrap());
                dependencies.insert(*v, c);
            }
            // Updating the betweeness centrality
            if w != s {
                *betweenness.get_mut(&graph[w]).unwrap() += dependencies.get(&w).unwrap();
            }
        }
    }

    // Normalizing the centrality values
    let normalization = (graph.node_count() - 1) as f64 * (graph.node_count() - 2) as f64;
    for value in betweenness.values_mut() {
        *value /= normalization;
    }

    betweenness
}
