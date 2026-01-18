// Bundle Analyzer
// Analysiert generierten Rust-Code und berechnet Bundle-Größen

use crate::tree_shaking::TreeShakingAnalyzer;
use velin_compiler::compiler::{VelinCompiler, config::CompilerConfig};
use velin_compiler::passes::{parser::ParserPass, type_check::TypeCheckPass, codegen::CodegenPass};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleAnalysis {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_functions: usize,
    pub total_structs: usize,
    pub total_enums: usize,
    pub unused_functions: Vec<String>,
    pub unused_structs: Vec<String>,
    pub unused_enums: Vec<String>,
    pub file_sizes: Vec<FileSize>,
    pub tree_shaking_potential: Option<TreeShakingPotential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSize {
    pub file: String,
    pub lines: usize,
    pub functions: usize,
    pub structs: usize,
    pub enums: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeShakingPotential {
    pub unused_functions_count: usize,
    pub unused_structs_count: usize,
    pub unused_enums_count: usize,
    pub potential_savings_percent: f64,
}

pub struct BundleAnalyzer {
    tree_shaking: TreeShakingAnalyzer,
}

impl BundleAnalyzer {
    pub fn new() -> Self {
        Self {
            tree_shaking: TreeShakingAnalyzer::new(),
        }
    }
    
    pub fn analyze(&self, path: &Path) -> Result<BundleAnalysis> {
        let files = self.collect_velin_files(path)?;
        
        let mut total_lines = 0;
        let mut total_functions = 0;
        let mut total_structs = 0;
        let mut total_enums = 0;
        let mut file_sizes = Vec::new();
        
        for file in &files {
            let content = fs::read_to_string(file)?;
            let lines = content.lines().count();
            total_lines += lines;
            
            // Einfache Analyse der Code-Struktur
            let functions = content.matches("fn ").count();
            let structs = content.matches("struct ").count();
            let enums = content.matches("enum ").count();
            
            total_functions += functions;
            total_structs += structs;
            total_enums += enums;
            
            file_sizes.push(FileSize {
                file: file.to_string_lossy().to_string(),
                lines,
                functions,
                structs,
                enums,
            });
        }
        
        // Tree-Shaking-Analyse
        let tree_shaking_potential = if !files.is_empty() {
            Some(self.tree_shaking.analyze(&files)?)
        } else {
            None
        };
        
        let unused_functions = tree_shaking_potential.as_ref()
            .map(|t| t.unused_functions_count)
            .unwrap_or(0);
        let unused_structs = tree_shaking_potential.as_ref()
            .map(|t| t.unused_structs_count)
            .unwrap_or(0);
        let unused_enums = tree_shaking_potential.as_ref()
            .map(|t| t.unused_enums_count)
            .unwrap_or(0);
        
        // Berechne potenzielle Einsparungen
        let total_items = total_functions + total_structs + total_enums;
        let unused_items = unused_functions + unused_structs + unused_enums;
        let potential_savings_percent = if total_items > 0 {
            (unused_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };
        
        let tree_shaking_potential = Some(TreeShakingPotential {
            unused_functions_count: unused_functions,
            unused_structs_count: unused_structs,
            unused_enums_count: unused_enums,
            potential_savings_percent,
        });
        
        Ok(BundleAnalysis {
            total_files: files.len(),
            total_lines,
            total_functions,
            total_structs,
            total_enums,
            unused_functions: Vec::new(), // Wird von Tree-Shaking-Analyzer gefüllt
            unused_structs: Vec::new(),
            unused_enums: Vec::new(),
            file_sizes,
            tree_shaking_potential,
        })
    }
    
    fn collect_velin_files(&self, path: &Path) -> Result<Vec<PathBuf>> {
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
        
        Ok(files)
    }
}
