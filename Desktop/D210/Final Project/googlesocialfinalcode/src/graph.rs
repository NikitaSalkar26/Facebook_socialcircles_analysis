// graph.rs

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Represents a graph using an adjacency list
pub struct Graph {
    adj_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    /// Creates a new empty graph
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    /// Adds a vertex to the graph
    fn add_vertex(&mut self, vertex: u32) {
        self.adj_list.entry(vertex).or_insert_with(Vec::new);
    }

    /// Adds an edge from `src` to `dst`
    fn add_edge(&mut self, src: u32, dst: u32) {
        self.adj_list.entry(src).or_insert_with(Vec::new).push(dst);
        // Uncomment the next line if the graph is undirected
        // self.adj_list.entry(dst).or_insert_with(Vec::new).push(src);
    }

    /// Reads edges from a file and constructs the graph
    pub fn from_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut graph = Graph::new();

        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() == 2 {
                let src = tokens[0].parse::<u32>().unwrap();
                let dst = tokens[1].parse::<u32>().unwrap();
                graph.add_vertex(src);
                graph.add_vertex(dst);
                graph.add_edge(src, dst);
            }
        }
        Ok(graph)
    }

    /// Returns the vertices connected to a given vertex
    pub fn get_adjacent(&self, vertex: u32) -> Option<&Vec<u32>> {
        self.adj_list.get(&vertex)
    }
}

