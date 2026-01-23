// LSP Handler für Find All References

use tower_lsp::lsp_types::{Location, Position, Range, ReferenceParams, Url};
use crate::document::DocumentCache;
use velin_compiler::parser::ast::*;

pub fn find_references(
    params: ReferenceParams,
    documents: &DocumentCache,
) -> Option<Vec<Location>> {
    let uri = params.text_document_position.text_document.uri.to_string();
    let position = params.text_document_position.position;
    
    let document = documents.get(&uri)?;
    let program = document.program.as_ref()?;
    
    // Find symbol at position
    let symbol_name = get_word_at_position(&document.text, position)?;
    
    // Find all references to this symbol
    let mut references = Vec::new();
    let uri_url = tower_lsp::lsp_types::Url::parse(&uri).ok()?;
    
    find_references_in_program(&program, &symbol_name, &uri_url, &mut references, &document.text);
    
    Some(references)
}


// Helper to find word at position from text
fn get_word_at_position(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    if position.line as usize >= lines.len() {
        return None;
    }
    
    let line = lines[position.line as usize];
    let col = position.character as usize;
    if col >= line.len() {
        return None;
    }
    
    // Find start of word
    let mut start = col;
    let chars: Vec<char> = line.chars().collect();
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }
    
    // Find end of word
    let mut end = col;
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }
    
    if start < end {
        Some(line[start..end].to_string())
    } else {
        None
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn find_references_in_program(
    _program: &Program,
    symbol: &str,
    uri: &Url,
    references: &mut Vec<Location>,
    document_text: &str,
) {
    // Naive approach: Search for the symbol in the text directly
    // This is a temporary "real logic" replacement for the AST-based tracking
    // until the AST supports span information.
    
    for (line_idx, line) in document_text.lines().enumerate() {
        let mut start_idx = 0;
        while let Some(idx) = line[start_idx..].find(symbol) {
            let actual_idx = start_idx + idx;
            
            // Check if it's a whole word
            let char_before = if actual_idx > 0 {
                line.chars().nth(actual_idx - 1)
            } else {
                None
            };
            
            let char_after = line.chars().nth(actual_idx + symbol.len());
            
            let is_start_word = char_before.map_or(true, |c| !is_word_char(c));
            let is_end_word = char_after.map_or(true, |c| !is_word_char(c));
            
            if is_start_word && is_end_word {
                references.push(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: line_idx as u32,
                            character: actual_idx as u32,
                        },
                        end: Position {
                            line: line_idx as u32,
                            character: (actual_idx + symbol.len()) as u32,
                        },
                    },
                });
            }
            
            start_idx = actual_idx + symbol.len();
        }
    }
}



