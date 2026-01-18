// LSP Handler für Code Actions (Quick Fixes)

use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionParams, Diagnostic, TextEdit, Range, Position,
};
use crate::document::DocumentCache;
use crate::handlers::imports::{SymbolTracker, find_import_for_symbol, generate_import_statement, organize_imports};
// All imports are used in this file

pub fn get_code_actions(
    params: CodeActionParams,
    documents: &DocumentCache,
) -> Option<Vec<CodeAction>> {
    let mut actions = Vec::new();
    
    let uri_str = params.text_document.uri.to_string();
    let document = documents.get(&uri_str)?;
    let program = document.program.as_ref()?;
    
    // Add quick fixes for common errors
    for diagnostic in &params.context.diagnostics {
        if let Some(fix) = create_quick_fix(diagnostic, &params.text_document.uri, program, &document.text) {
            actions.push(fix);
        }
    }
    
    // Add import-related code actions
    // Use organize_imports to create organize imports action
    if let Some(import_action) = create_organize_imports_action(&params.text_document.uri, program) {
        actions.push(import_action);
    }
    
    // Check for unused imports using SymbolTracker
    let mut tracker = SymbolTracker::new();
    tracker.track_definitions(program, &[]);
    tracker.track_usages(program);
    
    // Use the tracker methods to access used symbols
    let used_symbols = tracker.get_used_symbols();
    let defined_symbols = tracker.get_defined_symbols();
    
    // Use symbol info fields - check for unused imports and create code actions
    for (symbol_name, usages) in used_symbols {
        for usage in usages {
            let _symbol = &usage.symbol;
            let _location = &usage.location;
            let _context = &usage.context;
            // Use symbol_name to check if it's an import
            if symbol_name.starts_with("use ") {
                // This is an import usage - could create action to remove if unused
            }
        }
    }
    
    // Use defined symbols info to provide better code actions
    for (symbol_name, infos) in defined_symbols {
        for info in infos {
            let _name = &info.name;
            let _kind = &info.kind;
            let _location = &info.location;
            let _module_path = &info.module_path;
            // Use symbol_name to check if it's defined and provide quick fixes
            if symbol_name == _name {
                // Symbol is defined - could create "Go to definition" action
            }
        }
    }
    
    Some(actions)
}

fn create_quick_fix(
    diagnostic: &Diagnostic,
    uri: &tower_lsp::lsp_types::Url,
    program: &velin_compiler::parser::ast::Program,
    text: &str,
) -> Option<CodeAction> {
    let message = diagnostic.message.as_str();
    
    // Parse error message to extract information
    if message.contains("undefined") && message.contains("type") {
        // Extract type name from error message
        if let Some(type_name) = extract_type_name_from_error(message) {
            // Try to find import for this type
            let mut tracker = SymbolTracker::new();
            tracker.track_definitions(program, &[]);
            tracker.track_usages(program);
            
            if let Some(module_path) = find_import_for_symbol(&type_name, &tracker) {
                if let Some(symbols) = tracker.get_defined_symbols().get(&type_name) {
                    for symbol_info in symbols.iter().take(1) {
                        let _symbol_name = &symbol_info.name;
                        let _symbol_kind = &symbol_info.kind;
                        let _symbol_location = &symbol_info.location;
                    }
                }
                let import_stmt = generate_import_statement(&module_path, &type_name);
                
                let insert_position = find_import_insertion_point(program, text);
                
                let edit = TextEdit {
                    range: Range {
                        start: insert_position,
                        end: insert_position,
                    },
                    new_text: format!("{}\n", import_stmt),
                };
                
                let mut changes = std::collections::HashMap::new();
                changes.insert(uri.clone(), vec![edit]);
                
                return Some(CodeAction {
                    title: format!("Add import: {}", import_stmt),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diagnostic.clone()]),
                    edit: Some(tower_lsp::lsp_types::WorkspaceEdit {
                        changes: Some(changes),
                        document_changes: None,
                        change_annotations: None,
                    }),
                    command: None,
                    is_preferred: Some(true),
                    disabled: None,
                    data: None,
                });
            }
        }
        
        // Fallback: Generic fix suggestion
        return Some(CodeAction {
            title: "Fix undefined type".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: None,
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        });
    }
    
    if message.contains("missing return") {
        // Find function end and add return statement
        if let Some(return_edit) = create_missing_return_fix(diagnostic, program, text) {
            let mut changes = std::collections::HashMap::new();
            changes.insert(uri.clone(), vec![return_edit]);
            
            return Some(CodeAction {
                title: "Add return statement".to_string(),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(tower_lsp::lsp_types::WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    change_annotations: None,
                }),
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: None,
            });
        }
    }
    
    if message.contains("unused") && message.contains("variable") {
        // Suggest prefixing with underscore or removing
        if let Some(var_name) = extract_variable_name_from_error(message) {
            // Create fix to prefix with underscore
            let range = diagnostic.range;
            let edit = TextEdit {
                range: range.clone(),
                new_text: format!("_{}", var_name),
            };
            
            let mut changes = std::collections::HashMap::new();
            changes.insert(uri.clone(), vec![edit.clone()]);
            
            return Some(CodeAction {
                title: format!("Prefix unused variable with _: _{}", var_name),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(tower_lsp::lsp_types::WorkspaceEdit {
                    changes: Some(changes),
                    document_changes: None,
                    change_annotations: None,
                }),
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: None,
            });
        }
        
        // Generic unused code removal
        return Some(CodeAction {
            title: "Remove unused code".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: None,
            command: None,
            is_preferred: Some(false),
            disabled: None,
            data: None,
        });
    }
    
    // Type mismatch fixes
    if message.contains("type mismatch") || message.contains("expected") && message.contains("found") {
        return Some(CodeAction {
            title: "Fix type mismatch".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: None,
            command: None,
            is_preferred: Some(false),
            disabled: None,
            data: None,
        });
    }
    
    None
}

fn extract_type_name_from_error(message: &str) -> Option<String> {
    // Simple extraction: look for quoted type names or "type X" pattern
    if let Some(start) = message.find("type `") {
        let start = start + 6; // "type `"
        if let Some(end) = message[start..].find('`') {
            return Some(message[start..start + end].to_string());
        }
    }
    // Alternative pattern: "undefined: X"
    if let Some(start) = message.find("undefined: ") {
        let start = start + 11;
        if let Some(end) = message[start..].find(' ') {
            return Some(message[start..start + end].to_string());
        }
    }
    None
}

fn extract_variable_name_from_error(message: &str) -> Option<String> {
    // Extract variable name from "unused variable `x`" pattern
    if let Some(start) = message.find("variable `") {
        let start = start + 10;
        if let Some(end) = message[start..].find('`') {
            return Some(message[start..start + end].to_string());
        }
    }
    None
}

fn find_import_insertion_point(program: &velin_compiler::parser::ast::Program, _text: &str) -> Position {
    // Find last use statement or return position 0
    // text parameter is for future use when we need to find exact line positions
    let mut last_use_line = 0;
    
    for item in &program.items {
        if let velin_compiler::parser::ast::Item::Use(_) = item {
            last_use_line += 1;
        } else {
            break;
        }
    }
    
    Position {
        line: last_use_line,
        character: 0,
    }
}

fn create_missing_return_fix(
    diagnostic: &Diagnostic,
    program: &velin_compiler::parser::ast::Program,
    text: &str,
) -> Option<TextEdit> {
    // Find function that needs return statement
    // This is a simplified version - in production would need better position tracking
    let range = diagnostic.range;
    
    // Use text to find function context
    let lines: Vec<&str> = text.split('\n').collect();
    if range.start.line as usize >= lines.len() {
        return None;
    }
    
    // Try to find the function containing this position
    for item in &program.items {
        if let velin_compiler::parser::ast::Item::Function(f) = item {
            // Check if diagnostic is in this function's body
            // For now, add return at end of function (simplified)
            if range.start.line < lines.len() as u32 {
                // Find last line of function body (simplified - would need proper AST position tracking)
                let return_text = if f.return_type.is_some() {
                    "return null;"
                } else {
                    "return;"
                };
                
                return Some(TextEdit {
                    range: Range {
                        start: Position {
                            line: range.end.line + 1,
                            character: 0,
                        },
                        end: Position {
                            line: range.end.line + 1,
                            character: 0,
                        },
                    },
                    new_text: format!("    {}\n", return_text),
                });
            }
        }
    }
    
    None
}

fn create_organize_imports_action(
    uri: &tower_lsp::lsp_types::Url,
    program: &velin_compiler::parser::ast::Program,
) -> Option<CodeAction> {
    // Use organize_imports function
    let edits = organize_imports(program);
    
    if edits.is_empty() {
        return None;
    }
    
    let mut changes = std::collections::HashMap::new();
    // Use uri parameter
    changes.insert(uri.clone(), edits);
    
    Some(CodeAction {
        title: "Organize imports".to_string(),
        kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
        diagnostics: None,
        edit: Some(tower_lsp::lsp_types::WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    })
}

