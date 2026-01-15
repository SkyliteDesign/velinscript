use velin_compiler::parser::ast::{Item, Program};

#[derive(Debug, Default)]
pub struct CodeMetrics {
    pub function_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub total_lines: usize,
    pub function_lines: usize,
}

impl CodeMetrics {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn merge(&mut self, other: &CodeMetrics) {
        self.function_count += other.function_count;
        self.struct_count += other.struct_count;
        self.enum_count += other.enum_count;
        self.total_lines += other.total_lines;
        self.function_lines += other.function_lines;
    }
    
    pub fn avg_function_length(&self) -> f64 {
        if self.function_count > 0 {
            self.function_lines as f64 / self.function_count as f64
        } else {
            0.0
        }
    }
}

pub fn analyze_program(program: &Program) -> CodeMetrics {
    let mut metrics = CodeMetrics::new();
    
    for item in &program.items {
        match item {
            Item::Function(func) => {
                metrics.function_count += 1;
                
                // Schätze Zeilen-Anzahl (vereinfacht)
                if let Some(ref body) = func.body {
                    let body_str = format!("{:?}", body);
                    let lines = body_str.matches('\n').count() + 1;
                    metrics.function_lines += lines;
                }
            }
            Item::Struct(_) => {
                metrics.struct_count += 1;
            }
            Item::Enum(_) => {
                metrics.enum_count += 1;
            }
            _ => {}
        }
    }
    
    // Schätze Gesamt-Zeilen (vereinfacht)
    // In einer echten Implementierung würde man die Source-Datei analysieren
    metrics.total_lines = metrics.function_lines + 
        (metrics.struct_count * 5) + 
        (metrics.enum_count * 3);
    
    metrics
}
