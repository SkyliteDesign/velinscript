// Formatting Handler

use tower_lsp::lsp_types::*;
use velin_compiler::formatter::{Formatter, FormatConfig};

pub fn format_document(text: &str) -> Option<Vec<TextEdit>> {
    let config = FormatConfig::default();
    let mut formatter = Formatter::new(config);
    
    // Parse the document
    let program = match velin_compiler::parser::parser::Parser::parse(text) {
        Ok(program) => program,
        Err(_) => return None,
    };
    
    // Format the document
    let formatted = formatter.format(&program);
    
    // Create TextEdit for the entire document
    let lines: Vec<&str> = text.split('\n').collect();
    let last_line = lines.len().saturating_sub(1);
    let last_char = lines.last().map(|l| l.len()).unwrap_or(0);
    
    Some(vec![TextEdit {
        range: Range {
            start: Position { line: 0, character: 0 },
            end: Position {
                line: last_line as u32,
                character: last_char as u32,
            },
        },
        new_text: formatted,
    }])
}
