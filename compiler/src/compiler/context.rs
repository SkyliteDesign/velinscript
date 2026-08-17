use crate::parser::ast::Program;
use crate::error::CompilerError;
use crate::compiler::language::VELISCH_LANGUAGE_NAME;
use std::collections::HashMap;

/// Metadaten für KI-basierte Code-Analyse
#[derive(Debug, Clone, Default)]
pub struct SemanticMetadata {
    pub context_type: Option<String>, // "api", "service", "library", etc.
    pub dependencies: Vec<String>,
    pub security_requirements: Vec<String>,
    pub missing_components: Vec<String>,
}

#[derive(Debug)]
pub struct CompilationContext {
    pub source_map: HashMap<String, String>, // filename -> source
    pub program: Option<Program>,
    pub errors: Vec<CompilerError>,
    pub root_file: String,
    /// KI-basierte semantische Metadaten
    pub semantic_metadata: SemanticMetadata,
    /// Pass-/Build-Metadaten (z. B. compilation_order)
    pub metadata: HashMap<String, String>,
    /// Framework for HTTP lowering (from CompilerConfig); default axum when unset
    pub framework: Option<String>,
    /// Whether optimization analyzer should run (mirrors config)
    pub enable_optimization: bool,
}

impl CompilationContext {
    pub fn new(root_file: String, source: String) -> Self {
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
        let mut source_map = HashMap::new();
        source_map.insert(root_file.clone(), source);
        
        Self {
            source_map,
            program: None,
            errors: Vec::new(),
            root_file,
            semantic_metadata: SemanticMetadata::default(),
            metadata: HashMap::new(),
            framework: None,
            enable_optimization: true,
        }
    }

    pub fn add_source(&mut self, filename: String, source: String) {
        self.source_map.insert(filename, source);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn rust_framework(&self) -> String {
        self.framework
            .as_deref()
            .map(|f| f.to_lowercase())
            .filter(|f| !f.is_empty())
            .unwrap_or_else(|| "axum".to_string())
    }
}
