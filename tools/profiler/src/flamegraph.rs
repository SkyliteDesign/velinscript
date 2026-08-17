// Flame Graph Generator
// Generiert Flame Graphs aus Profiling-Daten

use crate::cpu::CpuProfileData;
use anyhow::Result;
use std::path::PathBuf;

pub struct FlameGraphGenerator;

impl FlameGraphGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, data: &CpuProfileData, output: &PathBuf) -> Result<()> {
        // Generiere vereinfachtes Flame Graph
        // In Produktion würde man inferno oder flamegraph crate nutzen
        
        let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="800">"#);
        svg.push_str(r#"<text x="10" y="20" font-family="Arial" font-size="14">Flame Graph</text>"#);
        
        // Generiere einfache Visualisierung
        let mut y = 40;
        for (i, func) in data.functions.iter().enumerate() {
            let width = (func.percentage * 10.0) as i32;
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"20\" fill=\"#{:02x}{:02x}{:02x}\"/>",
                10 + i * 120,
                y,
                width,
                (i * 50) % 255,
                (i * 100) % 255,
                (i * 150) % 255
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="10">{}</text>"#,
                10 + i * 120,
                y + 15,
                func.name
            ));
            y += 30;
        }
        
        svg.push_str("</svg>");
        
        std::fs::write(output, svg)?;
        
        Ok(())
    }
}
