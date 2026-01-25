use crate::parser::ast::*;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize)]
pub struct InsightReport {
    pub unused_structs: Vec<String>,
    pub complex_functions: Vec<String>,
    pub redundant_queries: Vec<String>,
}

pub struct InsightAnalyzer;

impl InsightAnalyzer {
    pub fn new() -> Self {
        InsightAnalyzer
    }

    pub fn analyze(&self, program: &Program) -> InsightReport {
        let mut report = InsightReport {
            unused_structs: Vec::new(),
            complex_functions: Vec::new(),
            redundant_queries: Vec::new(),
        };

        // 1. Detect Unused Structs
        let mut declared_structs = HashSet::new();
        let mut used_types = HashSet::new();

        for item in &program.items {
            if let Item::Struct(s) = item {
                declared_structs.insert(s.name.clone());
            }
            // Collect used types from functions
            if let Item::Function(f) = item {
                self.collect_types_in_function(f, &mut used_types);

                // 2. Detect Complex Functions (Heuristic: Statement Count > 20)
                if f.body.statements.len() > 20 {
                    report.complex_functions.push(f.name.clone());
                }
            }
        }

        for s in declared_structs {
            if !used_types.contains(&s) {
                // If it's public, it might be used externally, so skip warning
                // Real implementation would check visibility
                report.unused_structs.push(s);
            }
        }

        // 3. Detect Redundant Queries (Stub)
        // Would analyze call graph for repetitive db.find() calls without caching

        report
    }

    fn collect_types_in_function(&self, func: &Function, used: &mut HashSet<String>) {
        for param in &func.params {
            self.collect_type(&param.param_type, used);
        }
        if let Some(rt) = &func.return_type {
            self.collect_type(rt, used);
        }
    }

    fn collect_type(&self, ty: &Type, used: &mut HashSet<String>) {
        match ty {
            Type::Named(n) => {
                used.insert(n.clone());
            }
            Type::List(inner) => self.collect_type(inner, used),
            Type::Optional(inner) => self.collect_type(inner, used),
            _ => {}
        }
    }
}
