mod jaccard_similarity;
mod graph;
use jaccard_similarity::{compute_jaccard_similarity, compute_stats, percentage_above_thresholds};
use std::collections::HashMap;
use crate::graph::Graph;

fn main() {
    let graph = Graph::from_file("final_data.txt").unwrap();

    let similarities = compute_jaccard_similarity(&graph);

    // Compute and display the mean, max, and max pairs of Jaccard similarities
    let (mean, max, max_pairs) = compute_stats(&similarities);
    println!("Mean Jaccard similarity: {:.3}", mean);
    println!("Max Jaccard similarity: {:.3}", max);

    println!("Pairs with the highest similarity:");
    for ((v1, v2), similarity) in max_pairs {
        println!("Pair: ({}, {}), Similarity: {:.3}", v1, v2, similarity);
    }

    // Display the percentage of pairs with similarity scores above each threshold
    let percentages = percentage_above_thresholds(&similarities);
    for (th, perc) in percentages {
        println!("Percentage of pairs with Jaccard similarity above {:.1}: {}%", th, perc);
    }
}
