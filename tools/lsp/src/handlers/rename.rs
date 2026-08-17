// LSP Handler für Rename Symbol

use tower_lsp::lsp_types::{Position, RenameParams, TextEdit, WorkspaceEdit};
use crate::document::DocumentCache;
use velin_compiler::parser::ast::{*, Statement, Expression};

pub fn rename_symbol(
    params: RenameParams,
    documents: &DocumentCache,
) -> Option<WorkspaceEdit> {
    let uri_str = params.text_document_position.text_document.uri.to_string();
    let position = params.text_document_position.position;
    let new_name = params.new_name;
    
    let document = documents.get(&uri_str)?;
    let program = document.program.as_ref()?;
    
    // Find symbol at position
    let symbol_name = find_symbol_at_position(&program, position)?;
    
    // Find all references and create text edits
    let mut changes = std::collections::HashMap::new();
    let mut edits = Vec::new();
    
    let uri_url = tower_lsp::lsp_types::Url::parse(&uri_str).ok()?;
    // Use symbol_name and uri_str - both are used in find_and_rename_in_program
    find_and_rename_in_program(&program, &symbol_name, &new_name, &uri_url, &mut edits);
    
    if !edits.is_empty() {
        changes.insert(uri_url, edits);
    }
    
    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
}

/// Findet Symbol an gegebener Position
/// 
/// Analysiert den Text um die Position, um ein Identifier zu finden.
/// Dies ist eine vereinfachte Implementierung, die den Text direkt analysiert.
fn find_symbol_at_position(program: &Program, position: Position) -> Option<String> {
    // Vereinfachte Implementierung: Finde Identifier in der Nähe der Position
    // In einer vollständigen Implementierung würde man Positionen im AST speichern
    
    // Durchsuche alle Funktionen nach dem Identifier
    for item in &program.items {
        if let Item::Function(f) = item {
            // Prüfe ob Position im Funktionsbereich liegt (vereinfacht)
            if let Some(name) = find_identifier_in_function(f, position) {
                return Some(name);
            }
        }
    }
    
    None
}

fn find_identifier_in_function(_f: &Function, _position: Position) -> Option<String> {
    // Vereinfachte Implementierung: Finde Identifier im Funktionskörper
    // In einer vollständigen Implementierung würde man den Text an der Position analysieren
    None
}

/// Findet alle Vorkommen eines Symbols und erstellt Rename-Edits
/// 
/// Durchsucht das Programm nach allen Vorkommen des Symbols und erstellt TextEdits.
fn find_and_rename_in_program(
    program: &Program,
    old_name: &str,
    new_name: &str,
    _uri: &tower_lsp::lsp_types::Url,
    edits: &mut Vec<TextEdit>,
) {
    // uri parameter is for future use when we need to track which file the edits belong to
    // Durchsuche alle Items nach dem Symbol
    for item in &program.items {
        match item {
            Item::Function(f) => {
                // Prüfe Funktionsname
                if f.name == old_name {
                    // Erstelle Edit für Funktionsname (vereinfacht - Position 0,0)
                    edits.push(TextEdit {
                        range: tower_lsp::lsp_types::Range {
                            start: tower_lsp::lsp_types::Position { line: 0, character: 0 },
                            end: tower_lsp::lsp_types::Position { line: 0, character: old_name.len() as u32 },
                        },
                        new_text: new_name.to_string(),
                    });
                }
                // Durchsuche Funktionskörper
                find_and_rename_in_block(&f.body, old_name, new_name, edits);
            }
            Item::Struct(s) => {
                if s.name == old_name {
                    edits.push(TextEdit {
                        range: tower_lsp::lsp_types::Range {
                            start: tower_lsp::lsp_types::Position { line: 0, character: 0 },
                            end: tower_lsp::lsp_types::Position { line: 0, character: old_name.len() as u32 },
                        },
                        new_text: new_name.to_string(),
                    });
                }
            }
            Item::Enum(e) => {
                if e.name == old_name {
                    edits.push(TextEdit {
                        range: tower_lsp::lsp_types::Range {
                            start: tower_lsp::lsp_types::Position { line: 0, character: 0 },
                            end: tower_lsp::lsp_types::Position { line: 0, character: old_name.len() as u32 },
                        },
                        new_text: new_name.to_string(),
                    });
                }
            }
            _ => {}
        }
    }
}

fn find_and_rename_in_block(
    block: &Block,
    old_name: &str,
    new_name: &str,
    edits: &mut Vec<TextEdit>,
) {
    for statement in &block.statements {
        match statement {
            Statement::Let(let_stmt) => {
                if let Expression::Identifier(name) = &let_stmt.value {
                    if name == old_name {
                        edits.push(TextEdit {
                            range: tower_lsp::lsp_types::Range {
                                start: tower_lsp::lsp_types::Position { line: 0, character: 0 },
                                end: tower_lsp::lsp_types::Position { line: 0, character: old_name.len() as u32 },
                            },
                            new_text: new_name.to_string(),
                        });
                    }
                }
            }
            Statement::Expression(expr_stmt) => {
                find_and_rename_in_expression(&expr_stmt.expression, old_name, new_name, edits);
            }
            Statement::Return(ret_stmt) => {
                if let Some(ref value) = ret_stmt.value {
                    find_and_rename_in_expression(value, old_name, new_name, edits);
                }
            }
            _ => {}
        }
    }
}

fn find_and_rename_in_expression(
    expr: &Expression,
    old_name: &str,
    new_name: &str,
    edits: &mut Vec<TextEdit>,
) {
    match expr {
        Expression::Identifier(name) => {
            if name == old_name {
                edits.push(TextEdit {
                    range: tower_lsp::lsp_types::Range {
                        start: tower_lsp::lsp_types::Position { line: 0, character: 0 },
                        end: tower_lsp::lsp_types::Position { line: 0, character: old_name.len() as u32 },
                    },
                    new_text: new_name.to_string(),
                });
            }
        }
        Expression::Call { callee, args } => {
            find_and_rename_in_expression(callee, old_name, new_name, edits);
            for arg in args {
                find_and_rename_in_expression(arg, old_name, new_name, edits);
            }
        }
        Expression::BinaryOp { left, right, .. } => {
            find_and_rename_in_expression(left, old_name, new_name, edits);
            find_and_rename_in_expression(right, old_name, new_name, edits);
        }
        Expression::UnaryOp { expr, .. } => {
            find_and_rename_in_expression(expr, old_name, new_name, edits);
        }
        Expression::Member { object, .. } => {
            find_and_rename_in_expression(object, old_name, new_name, edits);
        }
        _ => {}
    }
}
