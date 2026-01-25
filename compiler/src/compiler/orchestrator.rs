use crate::compiler::context::CompilationContext;
use crate::parser::ast::*;
use anyhow::Result;
use indexmap::IndexMap;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::path::Path;

pub struct BuildOrchestrator {
    file_dependencies: HashMap<String, Vec<String>>,
}

impl BuildOrchestrator {
    pub fn new() -> Self {
        Self {
            file_dependencies: HashMap::new(),
        }
    }

    /// Analyzes all files in the project and builds a dependency graph
    pub fn analyze_project(&self, context: &CompilationContext) -> Result<DiGraph<String, ()>> {
        let mut graph = DiGraph::new();
        let mut node_map = IndexMap::new();

        // Add all files as nodes
        for (filename, _) in &context.source_map {
            if !node_map.contains_key(filename) {
                let idx = graph.add_node(filename.clone());
                node_map.insert(filename.clone(), idx);
            }
        }

        // Build edges based on use statements
        let mut file_deps = HashMap::new();
        if let Some(ref program) = context.program {
            Self::extract_file_dependencies_static(program, &context.root_file, &mut file_deps);
        }

        // Add edges to graph
        for (file, deps) in &file_deps {
            if let Some(&file_idx) = node_map.get(file) {
                for dep in deps {
                    if let Some(&dep_idx) = node_map.get(dep) {
                        graph.add_edge(dep_idx, file_idx, ());
                    }
                }
            }
        }

        Ok(graph)
    }

    /// Determines the compilation order based on dependencies
    pub fn determine_compilation_order(&self, context: &CompilationContext) -> Result<Vec<String>> {
        let graph = self.analyze_project(context)?;

        // Perform topological sort
        let sorted_indices = match toposort(&graph, None) {
            Ok(indices) => indices,
            Err(cycle) => {
                let cycle_node = graph[cycle.node_id()].clone();
                return Err(anyhow::anyhow!(
                    "Circular dependency detected between files involving: {}",
                    cycle_node
                ));
            }
        };

        // Convert indices to filenames
        let mut ordered_files = Vec::new();
        for idx in sorted_indices {
            ordered_files.push(graph[idx].clone());
        }

        Ok(ordered_files)
    }

    /// Extracts file dependencies from use statements in the program (static version)
    fn extract_file_dependencies_static(
        program: &Program,
        current_file: &str,
        file_deps: &mut HashMap<String, Vec<String>>,
    ) {
        for item in &program.items {
            match item {
                Item::Use(use_stmt) => {
                    // Extract module path from use statement
                    let module_path = use_stmt.path.join("::");

                    // Try to resolve the module path to a file
                    // For now, we'll use a simple heuristic: module paths map to files
                    // In a real implementation, this would use the module resolution system
                    if let Some(dep_file) =
                        Self::resolve_module_to_file_static(&module_path, current_file)
                    {
                        file_deps
                            .entry(current_file.to_string())
                            .or_insert_with(Vec::new)
                            .push(dep_file);
                    }
                }
                Item::Module(module) => {
                    // Recursively process nested modules
                    // Note: Nested modules are in the same file, so we use the same current_file
                    Self::extract_dependencies_from_items_static(
                        &module.items,
                        current_file,
                        file_deps,
                    );
                }
                _ => {}
            }
        }
    }

    /// Extracts dependencies from a list of items (static version)
    fn extract_dependencies_from_items_static(
        items: &[Item],
        current_file: &str,
        file_deps: &mut HashMap<String, Vec<String>>,
    ) {
        for item in items {
            match item {
                Item::Use(use_stmt) => {
                    let module_path = use_stmt.path.join("::");
                    if let Some(dep_file) =
                        Self::resolve_module_to_file_static(&module_path, current_file)
                    {
                        file_deps
                            .entry(current_file.to_string())
                            .or_insert_with(Vec::new)
                            .push(dep_file);
                    }
                }
                Item::Module(module) => {
                    Self::extract_dependencies_from_items_static(
                        &module.items,
                        current_file,
                        file_deps,
                    );
                }
                _ => {}
            }
        }
    }

    /// Resolves a module path to a file path
    /// This is a simplified implementation - in a real system, this would use proper module resolution
    fn resolve_module_to_file_static(module_path_str: &str, _current_file: &str) -> Option<String> {
        // Simple heuristic: convert module path to file path
        // e.g., "models::User" -> "models/user.velin" or "models/user.rs"
        let file_path = module_path_str.replace("::", "/");

        // Try common extensions
        for ext in &["velin", "rs", "ts", "js", "py"] {
            let candidate = format!("{}.{}", file_path, ext);
            if Path::new(&candidate).exists() {
                return Some(candidate);
            }
        }

        // If not found, return the module path as-is (will be handled by module resolution)
        // For now, we'll return None to avoid false dependencies
        None
    }

    /// Orchestrates the build process by ordering files based on dependencies
    pub fn orchestrate_build(&self, context: &CompilationContext) -> Result<Vec<String>> {
        // Determine compilation order
        let ordered_files = self.determine_compilation_order(context)?;

        // Reorder source_map entries (if needed for future processing)
        // For now, we just return the ordered list

        Ok(ordered_files)
    }
}

impl Default for BuildOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
