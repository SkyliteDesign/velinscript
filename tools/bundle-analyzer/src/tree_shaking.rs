// Tree Shaking Analyzer
// Analysiert ungenutzten Code für Tree-Shaking

use crate::analyzer::TreeShakingPotential;
use velin_compiler::parser::parser::Parser;
use velin_compiler::parser::ast::{Program, Item};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use std::collections::HashSet;

pub struct TreeShakingAnalyzer;

impl TreeShakingAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn analyze(&self, files: &[PathBuf]) -> Result<TreeShakingPotential> {
        let mut all_functions = HashSet::new();
        let mut all_structs = HashSet::new();
        let mut all_enums = HashSet::new();
        let mut used_items = HashSet::new();
        
        // Sammle alle definierten Items
        for file in files {
            let content = fs::read_to_string(file)?;
            match Parser::parse(&content) {
                Ok(program) => {
                    for item in &program.items {
                        match item {
                            Item::Function(f) => {
                                all_functions.insert(f.name.clone());
                            }
                            Item::Struct(s) => {
                                all_structs.insert(s.name.clone());
                            }
                            Item::Enum(e) => {
                                all_enums.insert(e.name.clone());
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        
        // Analysiere Verwendung (vereinfachte Analyse)
        // In einer vollständigen Implementierung würde man den AST traversieren
        // und alle Referenzen sammeln
        for file in files {
            let content = fs::read_to_string(file)?;
            
            // Einfache String-basierte Analyse
            for func_name in &all_functions {
                if content.contains(func_name) && content.matches(func_name).count() > 1 {
                    used_items.insert(format!("function:{}", func_name));
                }
            }
            
            for struct_name in &all_structs {
                if content.contains(struct_name) && content.matches(struct_name).count() > 1 {
                    used_items.insert(format!("struct:{}", struct_name));
                }
            }
            
            for enum_name in &all_enums {
                if content.contains(enum_name) && content.matches(enum_name).count() > 1 {
                    used_items.insert(format!("enum:{}", enum_name));
                }
            }
        }
        
        // Zähle ungenutzte Items
        let unused_functions = all_functions.iter()
            .filter(|f| !used_items.contains(&format!("function:{}", f)))
            .count();
        
        let unused_structs = all_structs.iter()
            .filter(|s| !used_items.contains(&format!("struct:{}", s)))
            .count();
        
        let unused_enums = all_enums.iter()
            .filter(|e| !used_items.contains(&format!("enum:{}", e)))
            .count();
        
        let total_items = all_functions.len() + all_structs.len() + all_enums.len();
        let unused_items = unused_functions + unused_structs + unused_enums;
        let potential_savings_percent = if total_items > 0 {
            (unused_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(TreeShakingPotential {
            unused_functions_count: unused_functions,
            unused_structs_count: unused_structs,
            unused_enums_count: unused_enums,
            potential_savings_percent,
        })
    }
}
