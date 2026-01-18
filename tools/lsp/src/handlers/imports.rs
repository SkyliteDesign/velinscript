// LSP Handler für Auto-Import Management

use tower_lsp::lsp_types::{Position, Range, TextEdit};
use velin_compiler::parser::ast::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub module_path: Vec<String>,
    pub location: Option<Range>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Module,
}

pub struct SymbolTracker {
    defined_symbols: HashMap<String, Vec<SymbolInfo>>,
    used_symbols: HashMap<String, Vec<Usage>>,
}

#[derive(Debug, Clone)]
pub struct Usage {
    pub symbol: String,
    pub location: Range,
    pub context: String,
}

impl SymbolTracker {
    pub fn new() -> Self {
        SymbolTracker {
            defined_symbols: HashMap::new(),
            used_symbols: HashMap::new(),
        }
    }

    /// Trackt alle definierten Symbole in einem Program
    pub fn track_definitions(&mut self, program: &Program, module_path: &[String]) {
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    let info = SymbolInfo {
                        name: f.name.clone(),
                        kind: SymbolKind::Function,
                        module_path: module_path.to_vec(),
                        location: None,
                    };
                    self.defined_symbols
                        .entry(f.name.clone())
                        .or_insert_with(Vec::new)
                        .push(info);
                }
                Item::Struct(s) => {
                    let info = SymbolInfo {
                        name: s.name.clone(),
                        kind: SymbolKind::Struct,
                        module_path: module_path.to_vec(),
                        location: None,
                    };
                    self.defined_symbols
                        .entry(s.name.clone())
                        .or_insert_with(Vec::new)
                        .push(info);
                }
                Item::Enum(e) => {
                    let info = SymbolInfo {
                        name: e.name.clone(),
                        kind: SymbolKind::Enum,
                        module_path: module_path.to_vec(),
                        location: None,
                    };
                    self.defined_symbols
                        .entry(e.name.clone())
                        .or_insert_with(Vec::new)
                        .push(info);
                }
                Item::Trait(t) => {
                    let info = SymbolInfo {
                        name: t.name.clone(),
                        kind: SymbolKind::Trait,
                        module_path: module_path.to_vec(),
                        location: None,
                    };
                    self.defined_symbols
                        .entry(t.name.clone())
                        .or_insert_with(Vec::new)
                        .push(info);
                }
                Item::TypeAlias(ta) => {
                    let info = SymbolInfo {
                        name: ta.name.clone(),
                        kind: SymbolKind::TypeAlias,
                        module_path: module_path.to_vec(),
                        location: None,
                    };
                    self.defined_symbols
                        .entry(ta.name.clone())
                        .or_insert_with(Vec::new)
                        .push(info);
                }
                Item::Module(m) => {
                    let info = SymbolInfo {
                        name: m.name.clone(),
                        kind: SymbolKind::Module,
                        module_path: module_path.to_vec(),
                        location: None,
                    };
                    self.defined_symbols
                        .entry(m.name.clone())
                        .or_insert_with(Vec::new)
                        .push(info);
                }
                _ => {}
            }
        }
    }

    /// Trackt alle verwendeten Symbole
    pub fn track_usages(&mut self, program: &Program) {
        for item in &program.items {
            self.track_usages_in_item(item);
        }
    }
    
    /// Gibt alle verwendeten Symbole zurück
    pub fn get_used_symbols(&self) -> &HashMap<String, Vec<Usage>> {
        &self.used_symbols
    }
    
    /// Gibt alle definierten Symbole zurück
    pub fn get_defined_symbols(&self) -> &HashMap<String, Vec<SymbolInfo>> {
        &self.defined_symbols
    }

    fn track_usages_in_item(&mut self, item: &Item) {
        match item {
            Item::Function(f) => {
                self.track_usages_in_block(&f.body);
            }
            Item::Module(m) => {
                for item in &m.items {
                    self.track_usages_in_item(item);
                }
            }
            _ => {}
        }
    }

    fn track_usages_in_block(&mut self, block: &Block) {
        for statement in &block.statements {
            self.track_usages_in_statement(statement);
        }
    }

    fn track_usages_in_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => {
                self.track_usages_in_expression(&let_stmt.value);
            }
            Statement::Return(ret_stmt) => {
                if let Some(ref value) = ret_stmt.value {
                    self.track_usages_in_expression(value);
                }
            }
            Statement::Expression(expr_stmt) => {
                self.track_usages_in_expression(&expr_stmt.expression);
            }
            Statement::If(if_stmt) => {
                self.track_usages_in_expression(&if_stmt.condition);
                self.track_usages_in_block(&if_stmt.then_block);
                if let Some(ref else_block) = if_stmt.else_block {
                    self.track_usages_in_block(else_block);
                }
            }
            Statement::For(for_stmt) => {
                self.track_usages_in_expression(&for_stmt.iterable);
                self.track_usages_in_block(&for_stmt.body);
            }
            Statement::While(while_stmt) => {
                self.track_usages_in_expression(&while_stmt.condition);
                self.track_usages_in_block(&while_stmt.body);
            }
            Statement::Match(match_stmt) => {
                self.track_usages_in_expression(&match_stmt.expression);
                for arm in &match_stmt.arms {
                    self.track_usages_in_block(&arm.body);
                }
            }
            Statement::Throw(throw_stmt) => {
                self.track_usages_in_expression(&throw_stmt.expression);
            }
            Statement::Break(_) => {
            }
        }
    }

    fn track_usages_in_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier(name) => {
                let usage = Usage {
                    symbol: name.clone(),
                    location: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 0 },
                    },
                    context: String::new(),
                };
                self.used_symbols
                    .entry(name.clone())
                    .or_insert_with(Vec::new)
                    .push(usage);
            }
            Expression::Call { callee, args } => {
                self.track_usages_in_expression(callee);
                for arg in args {
                    self.track_usages_in_expression(arg);
                }
            }
            Expression::Member { object, .. } => {
                self.track_usages_in_expression(object);
            }
            Expression::BinaryOp { left, right, .. } => {
                self.track_usages_in_expression(left);
                self.track_usages_in_expression(right);
            }
            Expression::UnaryOp { expr, .. } => {
                self.track_usages_in_expression(expr);
            }
            _ => {}
        }
    }
}

/// Findet mögliche Import-Pfade für ein Symbol
pub fn find_import_for_symbol(
    symbol: &str,
    tracker: &SymbolTracker,
) -> Option<Vec<String>> {
    if let Some(infos) = tracker.defined_symbols.get(symbol) {
        if let Some(info) = infos.first() {
            return Some(info.module_path.clone());
        }
    }
    None
}

/// Generiert ein `use` Statement
pub fn generate_import_statement(module_path: &[String], symbol: &str) -> String {
    if module_path.is_empty() {
        format!("use {};", symbol)
    } else {
        let path_str = module_path.join("::");
        format!("use {}::{};", path_str, symbol)
    }
}

/// Organisiert Imports in einem Program
pub fn organize_imports(program: &Program) -> Vec<TextEdit> {
    let mut imports = Vec::new();
    let mut other_items = Vec::new();
    let mut first_import_index = None;

    // Trenne Imports von anderen Items
    for (index, item) in program.items.iter().enumerate() {
        match item {
            Item::Use(use_stmt) => {
                if first_import_index.is_none() {
                    first_import_index = Some(index);
                }
                let path_str = use_stmt.path.join("::");
                let import_str = if let Some(alias) = &use_stmt.alias {
                    format!("use {} as {};", path_str, alias)
                } else {
                    format!("use {};", path_str)
                };
                imports.push(import_str);
            }
            _ => {
                other_items.push(item.clone());
            }
        }
    }

    // Sortiere Imports alphabetisch
    imports.sort();

    // Gruppiere Imports (std, extern, local)
    let (std_imports, extern_imports, local_imports) = group_imports(&imports);

    // Erstelle TextEdit für Import-Reorganisation
    if let Some(start_index) = first_import_index {
        let mut new_imports = Vec::new();
        new_imports.extend(std_imports.clone());
        if !std_imports.is_empty() && !extern_imports.is_empty() {
            new_imports.push(String::new()); // Leerzeile zwischen Gruppen
        }
        new_imports.extend(extern_imports.clone());
        if !extern_imports.is_empty() && !local_imports.is_empty() {
            new_imports.push(String::new());
        }
        new_imports.extend(local_imports.clone());

        // Finde letzte Import-Zeile
        let end_index = start_index + imports.len();

        vec![TextEdit {
            range: Range {
                start: Position {
                    line: start_index as u32,
                    character: 0,
                },
                end: Position {
                    line: end_index as u32,
                    character: 0,
                },
            },
            new_text: new_imports.join("\n"),
        }]
    } else {
        Vec::new()
    }
}

fn group_imports(imports: &[String]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut std_imports = Vec::new();
    let mut extern_imports = Vec::new();
    let mut local_imports = Vec::new();

    for import in imports {
        if import.starts_with("use std::") || import.starts_with("use core::") {
            std_imports.push(import.clone());
        } else if import.contains("::") && !import.starts_with("use crate::") {
            extern_imports.push(import.clone());
        } else {
            local_imports.push(import.clone());
        }
    }

    (std_imports, extern_imports, local_imports)
}

/// Findet ungenutzte Imports
pub fn find_unused_imports(
    program: &Program,
    tracker: &SymbolTracker,
) -> Vec<String> {
    let mut unused = Vec::new();
    let used_symbols: HashSet<String> = tracker.used_symbols.keys().cloned().collect();

    for item in &program.items {
        if let Item::Use(use_stmt) = item {
            let symbol = if let Some(alias) = &use_stmt.alias {
                alias.clone()
            } else {
                use_stmt.path.last().cloned().unwrap_or_default()
            };

            if !used_symbols.contains(&symbol) {
                unused.push(symbol);
            }
        }
    }

    unused
}
