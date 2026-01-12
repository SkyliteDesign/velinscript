// Document Cache für LSP

use velin_compiler::parser::parser::Parser;
use velin_compiler::parser::ast::*;
use std::collections::HashMap;

pub struct DocumentCache {
    documents: HashMap<String, DocumentInfo>,
}

pub struct DocumentInfo {
    pub uri: String,
    pub text: String,
    pub program: Option<Program>,
    pub parse_errors: Vec<String>,
}

impl DocumentCache {
    pub fn new() -> Self {
        DocumentCache {
            documents: HashMap::new(),
        }
    }
    
    pub fn update(&mut self, uri: String, text: String) {
        let program = Parser::parse(&text).ok();
        let parse_errors = if program.is_none() {
            vec!["Parse error".to_string()]
        } else {
            vec![]
        };
        
        self.documents.insert(uri.clone(), DocumentInfo {
            uri,
            text,
            program,
            parse_errors,
        });
    }
    
    pub fn get(&self, uri: &str) -> Option<&DocumentInfo> {
        self.documents.get(uri)
    }
    
    pub fn get_program(&self, uri: &str) -> Option<&Program> {
        self.documents.get(uri)?.program.as_ref()
    }
}

impl Default for DocumentCache {
    fn default() -> Self {
        Self::new()
    }
}
