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
    let symbol_name = find_symbol_at_position(&program, position)?;
    
    // Find all references to this symbol
    let mut references = Vec::new();
    let uri_url = tower_lsp::lsp_types::Url::parse(&uri).ok()?;
    // Use symbol_name and uri - both are used in find_references_in_program
    find_references_in_program(&program, &symbol_name, &uri_url, &mut references);
    
    Some(references)
}

/// Findet Symbol an gegebener Position
/// 
/// **Status**: Benötigt Position-Tracking im AST
/// 
/// In zukünftigen Versionen wird dies:
/// - Position-Informationen im AST speichern
/// - Symbol an exakter Position identifizieren
/// - Scope-Auflösung für korrekte Symbol-Identifikation
fn find_symbol_at_position(_program: &Program, _position: Position) -> Option<String> {
    // TODO: Implementiere Position-Tracking im AST
    // - Erweitere AST-Nodes um Position-Informationen
    // - Identifiziere Symbol an gegebener Position
    // - Berücksichtige Scope für korrekte Auflösung
    None
}

fn find_references_in_program(
    program: &Program,
    symbol: &str,
    uri: &Url,
    references: &mut Vec<Location>,
) {
    // Find all references to the symbol
    for item in &program.items {
        find_references_in_item(item, symbol, uri, references);
    }
}

fn find_references_in_item(
    item: &Item,
    symbol: &str,
    uri: &Url,
    references: &mut Vec<Location>,
) {
    match item {
        Item::Function(f) => {
            if f.name == symbol {
                // This is a definition, not a reference
            }
            find_references_in_block(&f.body, symbol, uri, references);
        }
        Item::Struct(s) => {
            if s.name == symbol {
                // This is a definition
            }
            for field in &s.fields {
                if field.name == symbol {
                    // Field definition
                }
            }
        }
        Item::Enum(e) => {
            if e.name == symbol {
                // This is a definition
            }
        }
        Item::Trait(t) => {
            if t.name == symbol {
                // This is a definition
            }
        }
        Item::Impl(i) => {
            if i.trait_name == symbol {
                // Trait reference
            }
        }
        _ => {}
    }
}

fn find_references_in_block(
    block: &Block,
    symbol: &str,
    uri: &Url,
    references: &mut Vec<Location>,
) {
    for statement in &block.statements {
        find_references_in_statement(statement, symbol, uri, references);
    }
}

fn find_references_in_statement(
    statement: &Statement,
    symbol: &str,
    uri: &Url,
    references: &mut Vec<Location>,
) {
    match statement {
        Statement::Let(let_stmt) => {
            if let Expression::Identifier(name) = &let_stmt.value {
                if name == symbol {
                    // TODO: Position-Tracking für exakte Referenz-Position
                    // Aktuell wird Position (0,0) verwendet als Placeholder
                }
            }
        }
        Statement::Return(ret_stmt) => {
            if let Some(ref value) = ret_stmt.value {
                find_references_in_expression(value, symbol, uri, references);
            }
        }
        Statement::Expression(expr_stmt) => {
            find_references_in_expression(&expr_stmt.expression, symbol, uri, references);
        }
        Statement::If(if_stmt) => {
            find_references_in_expression(&if_stmt.condition, symbol, uri, references);
            find_references_in_block(&if_stmt.then_block, symbol, uri, references);
            if let Some(ref else_block) = if_stmt.else_block {
                find_references_in_block(else_block, symbol, uri, references);
            }
        }
        Statement::For(for_stmt) => {
            find_references_in_expression(&for_stmt.iterable, symbol, uri, references);
            find_references_in_block(&for_stmt.body, symbol, uri, references);
        }
        Statement::While(while_stmt) => {
            find_references_in_expression(&while_stmt.condition, symbol, uri, references);
            find_references_in_block(&while_stmt.body, symbol, uri, references);
        }
        Statement::Match(match_stmt) => {
            find_references_in_expression(&match_stmt.expression, symbol, uri, references);
            for arm in &match_stmt.arms {
                find_references_in_block(&arm.body, symbol, uri, references);
            }
        }
    }
}

fn find_references_in_expression(
    expr: &Expression,
    symbol: &str,
    uri: &Url,
    references: &mut Vec<Location>,
) {
    match expr {
        Expression::Identifier(name) => {
            if name == symbol {
                // TODO: Position-Tracking für exakte Referenz-Position
                // Aktuell wird Position (0,0) verwendet als Placeholder
                // In zukünftigen Versionen wird die exakte Position aus dem AST verwendet
                references.push(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 0 },
                    },
                });
            }
        }
        Expression::Call { callee, args } => {
            find_references_in_expression(callee, symbol, uri, references);
            for arg in args {
                find_references_in_expression(arg, symbol, uri, references);
            }
        }
        Expression::Member { object, .. } => {
            find_references_in_expression(object, symbol, uri, references);
        }
        Expression::BinaryOp { left, right, .. } => {
            find_references_in_expression(left, symbol, uri, references);
            find_references_in_expression(right, symbol, uri, references);
        }
        Expression::UnaryOp { expr, .. } => {
            find_references_in_expression(expr, symbol, uri, references);
        }
        _ => {}
    }
}
