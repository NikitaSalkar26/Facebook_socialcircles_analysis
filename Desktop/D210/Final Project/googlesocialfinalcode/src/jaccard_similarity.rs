use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::graph::Graph;

/// Computes the Jaccard similarity score for all pairs of vertices with a shortest path of 2
pub fn compute_jaccard_similarity(graph: &Graph) -> HashMap<(usize, usize), f64> {
    let mut similarities = HashMap::new();
    let vertices: Vec<usize> = graph.adj_map.keys().copied().collect();

    for &v1 in &vertices {
        let shortest_paths = graph.bfs_shortest_paths(v1);

        for &v2 in &vertices {
            if v1 != v2 {
                if let Some(&dist) = shortest_paths.get(&v2) {
                    if dist == 2 {
                        let neighbors1: HashSet<usize> = graph.get_adjacent(v1).unwrap_or(&vec![]).iter().copied().collect();
                        let neighbors2: HashSet<usize> = graph.get_adjacent(v2).unwrap_or(&vec![]).iter().copied().collect();

                        let intersection: HashSet<_> = neighbors1.intersection(&neighbors2).copied().collect();
                        let union: HashSet<_> = neighbors1.union(&neighbors2).copied().collect();

                        let jaccard_score = intersection.len() as f64 / union.len() as f64;
                        similarities.insert((v1, v2), jaccard_score);
                    }
                }
            }
        }
    }

    similarities
}

pub fn compute_stats(similarities: &HashMap<(usize, usize), f64>) -> (f64, f64, Vec<((usize, usize), f64)>) {
    let mut total_similarity = 0.0;
    let mut similarity_count = 0;
    let mut max_similarity = 0.0;
    let mut max_pairs = Vec::new();

    for (&(v1, v2), &similarity) in similarities.iter() {
        total_similarity += similarity;
        similarity_count += 1;

        if similarity > max_similarity {
            max_similarity = similarity;
            max_pairs.clear();
            max_pairs.push(((v1, v2), similarity));
        } else if similarity == max_similarity {
            max_pairs.push(((v1, v2), similarity));
        }
    }

    let mean_similarity = if similarity_count > 0 {
        total_similarity / similarity_count as f64
    } else {
        0.0
    };

    (mean_similarity, max_similarity, max_pairs)
}

/// Computes the percentage of pairs with a Jaccard similarity score above the given thresholds
pub fn percentage_above_thresholds(similarities: &HashMap<(usize, usize), f64>) -> Vec<(f64, f64)> {
    let thresholds: Vec<f64> = (1..=10).map(|i| i as f64 / 10.0).collect();

    thresholds.iter().map(|&th| {
        let count_above = similarities.values().filter(|&&v| v > th).count();
        let percentage = count_above as f64 / similarities.len() as f64 * 100.0;

        (th, percentage)
    }).collect()
}
