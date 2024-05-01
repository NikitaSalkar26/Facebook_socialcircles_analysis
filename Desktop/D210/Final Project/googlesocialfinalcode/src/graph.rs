use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Graph {
    pub adj_map: HashMap<usize, Vec<usize>>,
}

impl Graph {
    /// Creates a new empty graph
    pub fn new() -> Self {
        Graph {
            adj_map: HashMap::new(),
        }
    }

    /// Adds a vertex to the graph
    pub fn add_vertex(&mut self, vertex: usize) {
        self.adj_map.entry(vertex).or_insert_with(Vec::new);
    }

    /// Adds an edge from `src` to `dst`
    pub fn add_edge(&mut self, src: usize, dst: usize) {
        self.adj_map.entry(src).or_insert_with(Vec::new).push(dst);
    }

    /// Reads a graph from a file and constructs the graph object
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut graph = Graph::new();

        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() == 2 {
                let src = tokens[0].parse::<usize>().unwrap();
                let dst = tokens[1].parse::<usize>().unwrap();
                graph.add_vertex(src);
                graph.add_vertex(dst);
                graph.add_edge(src, dst);
            }
        }
        Ok(graph)
    }

    /// Returns the vertices connected to a given vertex
    pub fn get_adjacent(&self, vertex: usize) -> Option<&Vec<usize>> {
        self.adj_map.get(&vertex)
    }

    /// Finds the shortest paths from a given vertex using BFS
    pub fn bfs_shortest_paths(&self, start: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        distances.insert(start, 0);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            let current_dist = *distances.get(&current).unwrap();

            for &neighbor in self.get_adjacent(current).unwrap_or(&vec![]) {
                if visited.insert(neighbor) {
                    distances.insert(neighbor, current_dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        distances
    }
}