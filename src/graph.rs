use petgraph::Graph;
use petgraph::prelude::*;
use std::collections::HashMap;

// creating a graph where each node represent a movie genre 
pub fn create_genre_graph(movies: &[super::Movie]) -> Graph<String, u32, Undirected> {
    let mut graph = Graph::<String, u32, Undirected>::new_undirected();
    // store each genre and corresponding node index in the graph 
    let mut genre_indices = HashMap::new();

    for movie in movies {
        let genres: Vec<&str> = movie.genres.split(',').map(str::trim).collect();
        let mut genre_nodes = Vec::new();
        
        for genre in &genres {
            let index = genre_indices.entry(genre.to_string()).or_insert_with(|| {
                graph.add_node(genre.to_string())
            });
            genre_nodes.push(*index);
        }

        // Adding edges between all pairs of genre 
        for i in 0..genre_nodes.len() {
            for j in i + 1..genre_nodes.len() {
                let weight = 1;
                // checking if an edge already exist 
                let edge = graph.find_edge(genre_nodes[i], genre_nodes[j]);
                if let Some(e) = edge {
                    let new_weight = graph.edge_weight(e).unwrap() + weight;
                    graph.update_edge(genre_nodes[i], genre_nodes[j], new_weight);
                } else {
                    graph.add_edge(genre_nodes[i], genre_nodes[j], weight);
                }
            }
        }
    }

    graph
}
