mod graph;
fn main() {
    match graph::Graph::from_file("facebook_combined.txt") {
        Ok(graph) => {
            if let Some(adj) = graph.get_adjacent(1) {
                println!("Adjacent to 1: {:?}", adj);
            } else {
                println!("No adjacent vertices found for 1.");
            }
        },
        Err(e) => println!("Failed to create graph: {}", e),
    }
}
