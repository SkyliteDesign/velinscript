// Go-to-Definition Handler

use tower_lsp::lsp_types::*;
use velin_compiler::parser::ast::*;

pub fn find_definition(program: &Program, word: &str, text: &str) -> Option<Location> {
    // Suche nach Funktionen
    for item in &program.items {
        if let Item::Function(f) = item {
            if f.name == word {
                if let Some(pos) = find_name_position(text, &f.name, "fn") {
                    return Some(Location {
                        uri: "file:///current".to_string(),
                        range: pos,
                    });
                }
            }
        }
    }
    
    // Suche nach Structs
    for item in &program.items {
        if let Item::Struct(s) = item {
            if s.name == word {
                if let Some(pos) = find_name_position(text, &s.name, "struct") {
                    return Some(Location {
                        uri: "file:///current".to_string(),
                        range: pos,
                    });
                }
            }
        }
    }
    
    // Suche nach Enums
    for item in &program.items {
        if let Item::Enum(e) = item {
            if e.name == word {
                if let Some(pos) = find_name_position(text, &e.name, "enum") {
                    return Some(Location {
                        uri: "file:///current".to_string(),
                        range: pos,
                    });
                }
            }
        }
    }
    
    None
}

fn find_name_position(text: &str, name: &str, keyword: &str) -> Option<Range> {
    let lines: Vec<&str> = text.split('\n').collect();
    
    for (line_idx, line) in lines.iter().enumerate() {
        // Suche nach "keyword name" Pattern
        let pattern = format!("{} {}", keyword, name);
        if let Some(pos) = line.find(&pattern) {
            let char_pos = line[..pos].chars().count();
            let name_start = char_pos + keyword.len() + 1; // +1 für Space
            let name_end = name_start + name.chars().count();
            
            return Some(Range {
                start: Position {
                    line: line_idx as u32,
                    character: name_start as u32,
                },
                end: Position {
                    line: line_idx as u32,
                    character: name_end as u32,
                },
            });
        }
        
        // Alternative: Suche nur nach dem Namen (wenn er am Zeilenanfang steht)
        if line.trim().starts_with(name) {
            let trimmed = line.trim_start();
            let indent = line.len() - trimmed.len();
            let name_start = indent;
            let name_end = indent + name.len();
            
            return Some(Range {
                start: Position {
                    line: line_idx as u32,
                    character: name_start as u32,
                },
                end: Position {
                    line: line_idx as u32,
                    character: name_end as u32,
                },
            });
        }
    }
    
    None
}
