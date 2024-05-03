mod jaccard_similarity;
mod graph;
use jaccard_similarity::{compute_jaccard_similarity, compute_stats, percentage_above_thresholds};
use std::collections::HashMap;
use crate::graph::Graph;

fn main() {
    let graph = Graph::from_file("final_data.txt").unwrap(); //load graph into file

    let similarities = compute_jaccard_similarity(&graph); //calc Jacc similarity for all pairs of vertices in the graph 

    println!("Jaccard Similarity Scores for Pairs of Vertices with Shortest Path of 2:"); //Displaying Jaccard similarity scores where the shortest path between nodes is 2
    for ((source_vertex, vertex), similarity) in &similarities {
        println!("Jaccard Similarity between {} and {}: {:.4}",source_vertex, vertex, similarity);
    }

    //Example usage of BFS to find all shortest points from source vertex 
    let source_vertex = 1; 
    let destination_vertex = 5;
    let shortest_paths = graph.bfs_shortest_paths(source_vertex);

    // Compute and display the mean, max, and max pairs of Jaccard similarities
    let (mean, max, max_pairs) = compute_stats(&similarities);
    println!("Mean Jaccard similarity: {:.3}", mean);
    println!("Max Jaccard similarity: {:.3}", max);

    //Print out pairs with highest Jacc sim
    println!("Pairs with the highest similarity:");
    for ((v1, v2), similarity) in max_pairs {
        println!("Pair: ({}, {}), Similarity: {:.3}", v1, v2, similarity);
    }

    // Display the percentage of pairs with similarity scores above each threshold
    let percentages = percentage_above_thresholds(&similarities);
    for (th, perc) in percentages {
        println!("Percentage of pairs with Jaccard similarity above {:.1}: {}%", th, perc);
    }


//Test cases to verify the functionality of the Graph and Jaccard similarity calculations

}

#[test]
    fn test_read_graph() {
        let graph = Graph::from_file("test_data.txt").unwrap();
        assert_eq!(graph.num_nodes(), 7); // Made a txt file with 7 nodes to check graph correctly reads the nodes from file
    }

#[test]
    fn test_compute_jaccard_similarity() {
        let graph = Graph::from_file("test_data.txt").unwrap();
        let similarities = compute_jaccard_similarity(&graph);
        assert_eq!(similarities[&(3, 2)], 0.2500); // Example assertion to check based on nodes in my test dataset

    }

#[test]    
    fn test_find_mean_max_similarity() {
        let graph = Graph::from_file("test_data.txt").unwrap();
        let similarities = compute_jaccard_similarity(&graph);
        let (mean, max, max_pairs) = compute_stats(&similarities);
        assert_eq!(mean, 0.075); //asserting the mean that I found manually using the code - for the test data file that I computed by hand
        assert_eq!(max, 0.25); //asserting the max that I found manually using the code - for the test data file that I computed by hand

    }

#[test]
    fn test_bfs_shortest_path() {
        let graph = Graph::from_file("test_data.txt").unwrap();
        let source_vertex = 1;
        let destination_vertex = 5;
        let shortest_paths = graph.bfs_shortest_paths(source_vertex);
    
         // Print shortest path for destination_vertex
         match shortest_paths.get(&destination_vertex) {
            Some(distance) => assert_eq!(*distance, 1) ,
            None => println!("No path found from {} to {}", source_vertex, destination_vertex),
        }

    }

#[test]
    fn test_compute_similarity_percentage() {
        // Read the graph from the test file
        let graph = Graph::from_file("test_data.txt").unwrap();
    
        // Compute Jaccard similarity scores for the graph
        let jaccard_similarities = compute_jaccard_similarity(&graph);
    
        // Compute similarity percentages
        let percentages = percentage_above_thresholds(&jaccard_similarities);
    
        // Check the correctness of the computed percentages
        assert_eq!(percentages.len(), 10); // 10 thresholds from 0.1 to 1.0
        assert_eq!(percentages[0], (0.1, 35.714285714285715)); 
        assert_eq!(percentages[1], (0.2, 7.142857142857142)); 
    
    }