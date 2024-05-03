use csv::Reader;
use serde::Deserialize;
use std::error::Error;

mod graph;  
mod centrality;
#[derive(Debug, Deserialize)]
pub struct Movie {  
    pub title: String,  
    pub genres: String,
    pub imdb_score: Option<f64>,
}

pub fn read_movies_from_csv(file_path: &str) -> Result<Vec<Movie>, Box<dyn Error>> {  
    let mut rdr = Reader::from_path(file_path)?;
    let mut movies = Vec::new();

    for result in rdr.deserialize() {
        let movie: Movie = result?;
        let genres: Vec<String> = movie.genres.split(',').map(String::from).collect();
        println!("Movie: {}, Genres: {:?}", movie.title, genres);
        movies.push(movie);
    }

    Ok(movies)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/imdb_movies_shows.csv"; // function to read csv
    let movies = read_movies_from_csv(file_path)?;
    let genre_graph = graph::create_genre_graph(&movies);
    let centrality = centrality::calculate_degree_centrality(&genre_graph);
    let betweenness_centrality = centrality::calculate_betweenness_centrality(&genre_graph);

// printing dataset 
    for movie in &movies {
        let genres: Vec<String> = movie.genres.split(',').map(String::from).collect();
        let genres_str = genres.join(",");
        println!("Movie Title: {}, Genres: {}, IMDb Score:{:?}", movie.title, genres_str, movie.imdb_score);
    }
    for (genre, degree)in centrality.iter(){
        println!("Genre:{}, Degree Centrality:{}", genre, degree);
    }
    for (genre, centrality) in &betweenness_centrality{
        println!("Genre: {}: betweenness centrality: {}",genre, centrality);
    }
    if let Some((max_genre, max_degree)) = centrality.iter().max_by_key(|entry|entry.1){
        println! ("The most popular genre based on degree centrality is {} with a centrality score of {}", max_genre,max_degree);
    }
    if let Some((max_genre, max_score)) = betweenness_centrality.iter().max_by(|a,b|a.1.partial_cmp(b.1).unwrap()){
        println! ("The genre with highest betweenness centrality is '{}' with a score of {:.4}", max_genre, max_score);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use self::centrality::calculate_betweenness_centrality;

    use super::*;
    use crate::Movie;
    use petgraph::Graph;
    use petgraph::Undirected;
    use centrality::calculate_degree_centrality;
    use std::collections::HashMap;

// test to helps handle the incomplete/missing data set details 
#[test]
fn test_empty_genre(){
    let movies = vec![
        Movie {
            title: String::from("A Movie Without Genre"),
            genres: String:: from(""),
            imdb_score:Some(6.0)
        },
    ];
    let graph = crate::graph::create_genre_graph(&movies);
    assert_eq!(graph.node_count(),0, "Graph should have no nodes when there is no genres provided");
    assert_eq!(graph.edge_count(),0,"Graph should have no edges when there is no genres provided")
}

// testing centrality 
fn create_test_graph() -> Graph<String, u32, Undirected>{
    let mut graph = Graph::<String, u32, Undirected>::new_undirected();
    let action = graph.add_node("Action".to_string());
    let comedy = graph.add_node("Comedy".to_string());
    let drama = graph.add_node("Drama".to_string());
    let romance = graph.add_node("Romance".to_string());

    // Adding edges 
    graph.add_edge(action, comedy,1);
    graph.add_edge(comedy,drama, 1);
    graph.add_edge(drama, action, 1);
    graph.add_edge(romance, drama,1);

    graph

}
#[test]
fn test_cal_degree_centrality(){
    let graph = create_test_graph();
    let centrality = calculate_degree_centrality(&graph);

    // Checking the centrality values 
    assert_eq!(centrality["Action"],2);
    assert_eq!(centrality["Comedy"],2);
    assert_eq!(centrality["Drama"],2);
    assert_eq!(centrality["Romance"],2);
}
// testing betweeness of centrality 
#[test]
fn test_betweenness(){
    let graph = create_test_graph();
    let centrality = calculate_betweenness_centrality(&graph);

    let expected = HashMap::from([
        ("Comedy".to_string(),0.0),
        ("Drama".to_string(),3.0),
        ("Action".to_string(),0.0),
        ("Romance".to_string(),0.0),
    ]);

    for (genre, &expected_centrality) in expected.iter() {
        assert!(
            (centrality.get(genre).unwrap() - expected_centrality).abs() < 0.01,
            "Centrality for {} should be close to {}", genre, expected_centrality
        );
    }
}

}
