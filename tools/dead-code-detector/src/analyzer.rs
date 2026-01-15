// Dead Code Analyzer - Findet ungenutzten Code

use velin_compiler::parser::ast::*;
use velin_compiler::parser::parser::Parser;
use std::fs;
use std::collections::{HashMap, HashSet};
use walkdir::WalkDir;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DeadCode {
    pub kind: DeadCodeKind,
    pub name: String,
    pub file: String,
    pub line: Option<u32>,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)] // Variable variant reserved for future use
pub enum DeadCodeKind {
    Function,
    Variable, // Reserved for future variable dead code detection
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Import,
    Impl,
}

#[allow(dead_code)] // Fields used internally for analysis
pub struct DeadCodeAnalyzer {
    defined: HashMap<String, Definition>,
    used: HashSet<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields used indirectly via HashMap key and file parameter
struct Definition {
    name: String, // Used indirectly in find_unused via HashMap key
    kind: DeadCodeKind,
    file: String, // Used indirectly in find_unused via file parameter
    line: Option<u32>,
}

impl DeadCodeAnalyzer {
    pub fn new() -> Self {
        DeadCodeAnalyzer {
            defined: HashMap::new(),
            used: HashSet::new(),
        }
    }

    /// Analysiert ein Program auf Dead Code
    pub fn analyze_program(&mut self, program: &Program, file: &str) -> Vec<DeadCode> {
        // Reset für jedes File
        let mut local_defined = HashMap::new();
        let mut local_used = HashSet::new();
        
        // Track definitions
        Self::track_definitions(program, file, &mut local_defined);
        
        // Track usages
        Self::track_usages(program, &mut local_used);
        
        // Find unused
        Self::find_unused(&local_defined, &local_used, file)
    }

    fn track_definitions(program: &Program, file: &str, defined: &mut HashMap<String, Definition>) {
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    defined.insert(
                        f.name.clone(),
                        Definition {
                            name: f.name.clone(),
                            kind: DeadCodeKind::Function,
                            file: file.to_string(),
                            line: None,
                        },
                    );
                }
                Item::Struct(s) => {
                    defined.insert(
                        s.name.clone(),
                        Definition {
                            name: s.name.clone(),
                            kind: DeadCodeKind::Struct,
                            file: file.to_string(),
                            line: None,
                        },
                    );
                }
                Item::Enum(e) => {
                    defined.insert(
                        e.name.clone(),
                        Definition {
                            name: e.name.clone(),
                            kind: DeadCodeKind::Enum,
                            file: file.to_string(),
                            line: None,
                        },
                    );
                }
                Item::Trait(t) => {
                    defined.insert(
                        t.name.clone(),
                        Definition {
                            name: t.name.clone(),
                            kind: DeadCodeKind::Trait,
                            file: file.to_string(),
                            line: None,
                        },
                    );
                }
                Item::TypeAlias(ta) => {
                    defined.insert(
                        ta.name.clone(),
                        Definition {
                            name: ta.name.clone(),
                            kind: DeadCodeKind::TypeAlias,
                            file: file.to_string(),
                            line: None,
                        },
                    );
                }
                Item::Use(use_stmt) => {
                    let symbol = use_stmt.path.last().cloned().unwrap_or_default();
                    defined.insert(
                        symbol.clone(),
                        Definition {
                            name: symbol,
                            kind: DeadCodeKind::Import,
                            file: file.to_string(),
                            line: None,
                        },
                    );
                }
                Item::Impl(i) => {
                    // Track impl blocks
                    let impl_name = format!("impl {} for {}", i.trait_name, i.for_type.to_string());
                    defined.insert(
                        impl_name.clone(),
                        Definition {
                            name: impl_name,
                            kind: DeadCodeKind::Impl,
                            file: file.to_string(),
                            line: None,
                        },
                    );
                }
                _ => {}
            }
        }
    }

    fn track_usages(program: &Program, used: &mut HashSet<String>) {
        for item in &program.items {
            Self::track_usages_in_item(item, used);
        }
    }

    fn track_usages_in_item(item: &Item, used: &mut HashSet<String>) {
        match item {
            Item::Function(f) => {
                // Function name is used when called
                // Parameters are used
                for param in &f.params {
                    used.insert(param.name.clone());
                }
                Self::track_usages_in_block(&f.body, used);
            }
            Item::Struct(s) => {
                // Struct name is used when referenced as type
                // Fields are used
                for field in &s.fields {
                    used.insert(field.name.clone());
                }
            }
            Item::Enum(e) => {
                // Enum name is used when referenced
                // Variants are used
                for variant in &e.variants {
                    used.insert(variant.name.clone());
                }
            }
            Item::Trait(t) => {
                // Trait name is used in impl blocks
                // Methods are used
                for method in &t.methods {
                    used.insert(method.name.clone());
                }
            }
            Item::Module(m) => {
                for item in &m.items {
                    Self::track_usages_in_item(item, used);
                }
            }
            _ => {}
        }
    }

    fn track_usages_in_block(block: &Block, used: &mut HashSet<String>) {
        for statement in &block.statements {
            Self::track_usages_in_statement(statement, used);
        }
    }

    fn track_usages_in_statement(statement: &Statement, used: &mut HashSet<String>) {
        match statement {
            Statement::Let(let_stmt) => {
                // Variable name is defined, not used here
                Self::track_usages_in_expression(&let_stmt.value, used);
            }
            Statement::Return(ret_stmt) => {
                if let Some(ref value) = ret_stmt.value {
                    Self::track_usages_in_expression(value, used);
                }
            }
            Statement::Expression(expr_stmt) => {
                Self::track_usages_in_expression(&expr_stmt.expression, used);
            }
            Statement::If(if_stmt) => {
                Self::track_usages_in_expression(&if_stmt.condition, used);
                Self::track_usages_in_block(&if_stmt.then_block, used);
                if let Some(ref else_block) = if_stmt.else_block {
                    Self::track_usages_in_block(else_block, used);
                }
            }
            Statement::For(for_stmt) => {
                Self::track_usages_in_expression(&for_stmt.iterable, used);
                Self::track_usages_in_block(&for_stmt.body, used);
            }
            Statement::While(while_stmt) => {
                Self::track_usages_in_expression(&while_stmt.condition, used);
                Self::track_usages_in_block(&while_stmt.body, used);
            }
            Statement::Match(match_stmt) => {
                Self::track_usages_in_expression(&match_stmt.expression, used);
                for arm in &match_stmt.arms {
                    Self::track_usages_in_block(&arm.body, used);
                }
            }
        }
    }

    fn track_usages_in_expression(expr: &Expression, used: &mut HashSet<String>) {
        match expr {
            Expression::Identifier(name) => {
                used.insert(name.clone());
            }
            Expression::Call { callee, args } => {
                Self::track_usages_in_expression(callee, used);
                for arg in args {
                    Self::track_usages_in_expression(arg, used);
                }
            }
            Expression::Member { object, member: _ } => {
                Self::track_usages_in_expression(object, used);
            }
            Expression::BinaryOp { left, right, .. } => {
                Self::track_usages_in_expression(left, used);
                Self::track_usages_in_expression(right, used);
            }
            Expression::UnaryOp { expr, .. } => {
                Self::track_usages_in_expression(expr, used);
            }
            _ => {}
        }
    }

    fn find_unused(
        defined: &HashMap<String, Definition>,
        used: &HashSet<String>,
        file: &str,
    ) -> Vec<DeadCode> {
        let mut unused = Vec::new();

        for (name, def) in defined {
            if !used.contains(name) {
                let suggestion = match def.kind {
                    DeadCodeKind::Function => format!("Entferne Funktion '{}' oder verwende sie", name),
                    DeadCodeKind::Variable => format!("Entferne Variable '{}' oder prefixe mit '_'", name),
                    DeadCodeKind::Struct => format!("Entferne Struct '{}' oder verwende ihn", name),
                    DeadCodeKind::Enum => format!("Entferne Enum '{}' oder verwende ihn", name),
                    DeadCodeKind::Trait => format!("Entferne Trait '{}' oder verwende ihn", name),
                    DeadCodeKind::TypeAlias => format!("Entferne Type Alias '{}' oder verwende ihn", name),
                    DeadCodeKind::Import => format!("Entferne ungenutzten Import '{}'", name),
                    DeadCodeKind::Impl => format!("Entferne ungenutzte Impl '{}'", name),
                };

                unused.push(DeadCode {
                    kind: def.kind.clone(),
                    name: name.clone(),
                    file: file.to_string(),
                    line: def.line,
                    suggestion,
                });
            }
        }

        unused
    }
}

/// Scannt ein Verzeichnis auf Dead Code
pub fn scan_directory(path: &str) -> Result<Vec<DeadCode>> {
    let mut all_findings = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "velin")
                .unwrap_or(false)
        })
    {
        let file_path = entry.path();
        let content = fs::read_to_string(file_path)?;

            match Parser::parse(&content) {
                Ok(program) => {
                    let file_str = file_path.to_string_lossy().to_string();
                    let mut local_analyzer = DeadCodeAnalyzer::new();
                    let findings = local_analyzer.analyze_program(&program, &file_str);
                    all_findings.extend(findings);
                }
            Err(_) => {
                // Parse errors werden ignoriert (werden vom Compiler behandelt)
            }
        }
    }

    Ok(all_findings)
}

/// Druckt einen Report
pub fn print_report(findings: &[DeadCode]) {
    println!("VelinScript Dead Code Report");
    println!("============================");
    println!("Gefundener Dead Code: {}\n", findings.len());

    for finding in findings {
        let kind_str = match finding.kind {
            DeadCodeKind::Function => "Function",
            DeadCodeKind::Variable => "Variable",
            DeadCodeKind::Struct => "Struct",
            DeadCodeKind::Enum => "Enum",
            DeadCodeKind::Trait => "Trait",
            DeadCodeKind::TypeAlias => "TypeAlias",
            DeadCodeKind::Import => "Import",
            DeadCodeKind::Impl => "Impl",
        };

        println!("[{}] {}", kind_str, finding.name);
        println!("  File: {}", finding.file);
        if let Some(line) = finding.line {
            println!("  Line: {}", line);
        }
        println!("  Suggestion: {}", finding.suggestion);
        println!();
    }
}
