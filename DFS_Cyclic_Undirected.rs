use std::collections::{HashMap, HashSet};

type Vertex = u32;

struct Graph {
    adjacency_list: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: Vertex, to: Vertex) {
        self.adjacency_list
            .entry(from)
            .or_insert(Vec::new())
            .push(to);

        // Since the graph is undirected, add the reverse edge as well.
        self.adjacency_list
            .entry(to)
            .or_insert(Vec::new())
            .push(from);
    }

    fn has_cycle(&self) -> bool {
        let mut visited = HashMap::new();
        let mut parent = HashMap::new();

        for &node in self.adjacency_list.keys() {
            if !visited.contains_key(&node) {
                if self.detect_cycle(node, &mut visited, &mut parent) {
                    return true;
                }
            }
        }

        false
    }

    fn detect_cycle(
        &self,
        node: Vertex,
        visited: &mut HashMap<Vertex, bool>,
        parent: &mut HashMap<Vertex, Vertex>,
    ) -> bool {
        visited.insert(node, true);

        if let Some(neighbors) = self.adjacency_list.get(&node) {
            for &neighbor in neighbors.iter() {
                if !visited.contains_key(&neighbor) {
                    parent.insert(neighbor, node);

                    if self.detect_cycle(neighbor, visited, parent) {
                        return true;
                    }
                } else if let Some(&parent_node) = parent.get(&node) {
                    if parent_node != neighbor {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn depth_first_search(&self, start: Vertex) {
        let mut visited = HashMap::new();
        self.dfs(start, &mut visited);
    }

    fn dfs(&self, start: Vertex, visited: &mut HashMap<Vertex, bool>) {
        if visited.contains_key(&start) {
            return; // Already visited this node
        }

        visited.insert(start, true);
        println!("Visited node: {}", start);

        if let Some(neighbors) = self.adjacency_list.get(&start) {
            for &neighbor in neighbors.iter() {
                self.dfs(neighbor, visited);
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    graph.depth_first_search(2);
    if graph.has_cycle() {
        println!("The graph has a cycle.");
    } else {
        println!("The graph is acyclic.");
    }
}
