// Report Generator
// Generiert menschenlesbare Reports

use crate::analyzer::{BundleAnalysis, FileSize};
use anyhow::Result;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(
        &self,
        analysis: &BundleAnalysis,
        tree_shaking: bool,
        code_splitting: bool,
    ) -> Result<String> {
        let mut report = String::new();
        
        report.push_str("📦 Bundle-Analyse Report\n");
        report.push_str("=".repeat(50).as_str());
        report.push_str("\n\n");
        
        // Übersicht
        report.push_str("## Übersicht\n\n");
        report.push_str(&format!("Dateien: {}\n", analysis.total_files));
        report.push_str(&format!("Gesamt-Zeilen: {}\n", analysis.total_lines));
        report.push_str(&format!("Funktionen: {}\n", analysis.total_functions));
        report.push_str(&format!("Structs: {}\n", analysis.total_structs));
        report.push_str(&format!("Enums: {}\n", analysis.total_enums));
        report.push_str("\n");
        
        // Datei-Größen
        report.push_str("## Datei-Größen\n\n");
        for file_size in &analysis.file_sizes {
            report.push_str(&format!("  {}:\n", file_size.file));
            report.push_str(&format!("    Zeilen: {}, Funktionen: {}, Structs: {}, Enums: {}\n",
                file_size.lines, file_size.functions, file_size.structs, file_size.enums));
        }
        report.push_str("\n");
        
        // Tree-Shaking
        if tree_shaking {
            if let Some(ref potential) = analysis.tree_shaking_potential {
                report.push_str("## Tree-Shaking-Potenzial\n\n");
                report.push_str(&format!("Ungenutzte Funktionen: {}\n", potential.unused_functions_count));
                report.push_str(&format!("Ungenutzte Structs: {}\n", potential.unused_structs_count));
                report.push_str(&format!("Ungenutzte Enums: {}\n", potential.unused_enums_count));
                report.push_str(&format!("Potenzielle Einsparungen: {:.2}%\n", potential.potential_savings_percent));
                report.push_str("\n");
            }
        }
        
        // Code-Splitting-Vorschläge
        if code_splitting {
            report.push_str("## Code-Splitting-Vorschläge\n\n");
            
            // Gruppiere Dateien nach Größe
            let mut large_files: Vec<&FileSize> = analysis.file_sizes.iter()
                .filter(|f| f.lines > 200)
                .collect();
            large_files.sort_by(|a, b| b.lines.cmp(&a.lines));
            
            if !large_files.is_empty() {
                report.push_str("Große Dateien (könnten aufgeteilt werden):\n");
                for file_size in large_files.iter().take(5) {
                    report.push_str(&format!("  - {} ({} Zeilen)\n", file_size.file, file_size.lines));
                }
            } else {
                report.push_str("Keine großen Dateien gefunden. Code-Splitting nicht notwendig.\n");
            }
            report.push_str("\n");
        }
        
        Ok(report)
    }
}
