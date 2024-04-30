mod jaccard_similarity;
use jaccard_similarity::{Graph, compute_jaccard_similarity};

fn main() {
    let graph = Graph::from_file("final_data.txt").unwrap();

    let similarities = compute_jaccard_similarity(&graph);

    // Display Jaccard similarity scores
    for ((v1, v2), score) in similarities {
        println!("Jaccard similarity between {} and {}: {:.3}", v1, v2, score);
    }
}
