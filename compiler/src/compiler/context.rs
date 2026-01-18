use crate::parser::ast::Program;
use crate::error::CompilerError;
use crate::compiler::language::VELISCH_LANGUAGE_NAME;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CompilationContext {
    pub source_map: HashMap<String, String>, // filename -> source
    pub program: Option<Program>,
    pub errors: Vec<CompilerError>,
    pub root_file: String,
}

impl CompilationContext {
    pub fn new(root_file: String, source: String) -> Self {
        // Velisch Identity - Fingerabdruck im Context
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
        let mut source_map = HashMap::new();
        source_map.insert(root_file.clone(), source);
        
        Self {
            source_map,
            program: None,
            errors: Vec::new(),
            root_file,
        }
    }

    pub fn add_source(&mut self, filename: String, source: String) {
        self.source_map.insert(filename, source);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
