// Graph Visualizer
// Generiert verschiedene Output-Formate für Dependency-Graphs

use crate::graph::DependencyGraph;
use anyhow::Result;
use serde_json;

pub struct GraphVisualizer;

impl GraphVisualizer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn to_dot(&self, graph: &DependencyGraph, circular_only: bool) -> Result<String> {
        let mut dot = String::from("digraph Dependencies {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");
        
        let circular_deps: std::collections::HashSet<String> = if circular_only {
            graph.find_circular_dependencies()
                .iter()
                .flat_map(|cycle| cycle.iter().cloned())
                .collect()
        } else {
            std::collections::HashSet::new()
        };
        
        let edges = graph.get_edges();
        for (from, to) in edges {
            if !circular_only || circular_deps.contains(&from) || circular_deps.contains(&to) {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", from, to));
            }
        }
        
        dot.push_str("}\n");
        Ok(dot)
    }
    
    pub fn to_svg(&self, graph: &DependencyGraph, circular_only: bool) -> Result<String> {
        // Für SVG generieren wir DOT und geben Hinweis auf dot-Befehl
        let dot = self.to_dot(graph, circular_only)?;
        
        let svg_hint = r#"# Um SVG zu generieren, installiere Graphviz und führe aus:
# dot -Tsvg -o output.svg <(velin-deps graph --format dot)

"#;
        
        Ok(format!("{}{}", svg_hint, dot))
    }
    
    pub fn to_json(&self, graph: &DependencyGraph, circular_only: bool) -> Result<String> {
        let nodes = graph.get_nodes();
        let edges = graph.get_edges();
        let circular = graph.find_circular_dependencies();
        
        let json = serde_json::json!({
            "nodes": nodes,
            "edges": edges,
            "circular_dependencies": circular,
            "total_nodes": nodes.len(),
            "total_edges": edges.len(),
            "circular_count": circular.len()
        });
        
        Ok(serde_json::to_string_pretty(&json)?)
    }
}
