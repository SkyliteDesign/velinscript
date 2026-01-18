// Dependency Analyzer
// Analysiert use-Statements und erstellt Dependency-Graph

use crate::graph::DependencyGraph;
use velin_compiler::parser::parser::Parser;
use velin_compiler::parser::ast::{Program, Item};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

pub struct DependencyAnalyzer;

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn analyze(&self, path: &Path) -> Result<DependencyGraph> {
        let mut graph = DependencyGraph::new();
        let mut file_map: HashMap<String, PathBuf> = HashMap::new();
        
        // Sammle alle .velin Dateien
        let files = if path.is_file() {
            vec![path.to_path_buf()]
        } else {
            WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter(|e| {
                    e.path().extension()
                        .and_then(|s| s.to_str())
                        == Some("velin")
                })
                .map(|e| e.path().to_path_buf())
                .collect()
        };
        
        // Erstelle Mapping von Modul-Namen zu Dateien
        for file in &files {
            if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
                file_map.insert(stem.to_string(), file.clone());
            }
        }
        
        // Analysiere jede Datei
        for file in &files {
            let content = fs::read_to_string(file)?;
            let module_name = file.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            match Parser::parse(&content) {
                Ok(program) => {
                    graph.add_node(module_name.clone());
                    
                    // Finde alle use-Statements
                    for item in &program.items {
                        if let Item::Use(use_stmt) = item {
                            if let Some(dep_name) = use_stmt.path.first() {
                                // Prüfe ob Dependency existiert
                                if file_map.contains_key(dep_name) {
                                    graph.add_edge(module_name.clone(), dep_name.clone());
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("⚠️  Fehler beim Parsen von {}: {}", file.display(), e.message);
                }
            }
        }
        
        Ok(graph)
    }
}
