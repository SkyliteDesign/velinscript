// Dependency Graph
// Repräsentiert Modul-Abhängigkeiten als Graph

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::kosaraju_scc;
use indexmap::IndexMap;
use std::collections::HashSet;

pub struct DependencyGraph {
    graph: Graph<String, ()>,
    node_map: IndexMap<String, NodeIndex>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: IndexMap::new(),
        }
    }
    
    pub fn add_node(&mut self, name: String) {
        if !self.node_map.contains_key(&name) {
            let idx = self.graph.add_node(name.clone());
            self.node_map.insert(name, idx);
        }
    }
    
    pub fn add_edge(&mut self, from: String, to: String) {
        self.add_node(from.clone());
        self.add_node(to.clone());
        
        let from_idx = *self.node_map.get(&from).unwrap();
        let to_idx = *self.node_map.get(&to).unwrap();
        
        // Prüfe ob Edge bereits existiert
        if !self.graph.contains_edge(from_idx, to_idx) {
            self.graph.add_edge(from_idx, to_idx, ());
        }
    }
    
    pub fn find_circular_dependencies(&self) -> Vec<Vec<String>> {
        let sccs = kosaraju_scc(&self.graph);
        
        let mut cycles = Vec::new();
        for scc in sccs {
            if scc.len() > 1 {
                // Zirkuläre Abhängigkeit gefunden
                let cycle: Vec<String> = scc.iter()
                    .map(|&idx| self.graph[idx].clone())
                    .collect();
                cycles.push(cycle);
            }
        }
        
        cycles
    }
    
    pub fn get_nodes(&self) -> Vec<String> {
        self.node_map.keys().cloned().collect()
    }
    
    pub fn get_edges(&self) -> Vec<(String, String)> {
        let mut edges = Vec::new();
        for edge_idx in self.graph.edge_indices() {
            let (a, b) = self.graph.edge_endpoints(edge_idx).unwrap();
            let from = self.graph[a].clone();
            let to = self.graph[b].clone();
            edges.push((from, to));
        }
        edges
    }
    
    pub fn get_dependencies(&self, node: &str) -> Vec<String> {
        if let Some(&node_idx) = self.node_map.get(node) {
            self.graph.neighbors_directed(node_idx, petgraph::Direction::Outgoing)
                .map(|idx| self.graph[idx].clone())
                .collect()
        } else {
            Vec::new()
        }
    }
}
