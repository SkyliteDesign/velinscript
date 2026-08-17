//! Security gate: secrets in source + SQL concat heuristics.

use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::error::{CompilerError, ErrorLocation};
use crate::parser::ast::{Expression, Item, Program, Statement};
use anyhow::Result;
use regex::Regex;

pub struct SecurityGatePass;

impl SecurityGatePass {
    pub fn new() -> Self {
        Self
    }

    fn scan_source_secrets(&self, source: &str, context: &mut CompilationContext) {
        let patterns = [
            (
                r#"(?i)(api[_-]?key|secret|password|token)\s*=\s*["'][^"']{8,}["']"#,
                "Possible hardcoded secret",
            ),
            (r"sk-[a-zA-Z0-9]{20,}", "Possible API key material"),
        ];
        for (pat, msg) in patterns {
            if let Ok(re) = Regex::new(pat) {
                if re.is_match(source) {
                    context.errors.push(CompilerError::parse_error(
                        format!("Security: {}", msg),
                        ErrorLocation::new(0, 0),
                    ));
                }
            }
        }
    }

    fn walk_expr_sql(&self, expr: &Expression, warnings: &mut Vec<String>) {
        match expr {
            Expression::BinaryOp {
                left,
                op,
                right,
            } => {
                let op_s = format!("{:?}", op).to_lowercase();
                if op_s.contains("add") {
                    let s = format!("{:?} {:?}", left, right).to_lowercase();
                    if s.contains("select") || s.contains("insert") || s.contains("query") {
                        warnings.push(
                            "Possible SQL string concatenation — use parameterized queries"
                                .into(),
                        );
                    }
                }
                self.walk_expr_sql(left, warnings);
                self.walk_expr_sql(right, warnings);
            }
            Expression::Call { callee, args } => {
                self.walk_expr_sql(callee, warnings);
                for a in args {
                    self.walk_expr_sql(a, warnings);
                }
            }
            Expression::Member { object, .. } => self.walk_expr_sql(object, warnings),
            _ => {}
        }
    }

    fn scan_block_stmts(&self, stmts: &[Statement], warnings: &mut Vec<String>) {
        for stmt in stmts {
            match stmt {
                Statement::Expression(e) => self.walk_expr_sql(&e.expression, warnings),
                Statement::Return(r) => {
                    if let Some(e) = &r.value {
                        self.walk_expr_sql(e, warnings);
                    }
                }
                Statement::Let(l) => self.walk_expr_sql(&l.value, warnings),
                Statement::If(i) => {
                    self.walk_expr_sql(&i.condition, warnings);
                    self.scan_block_stmts(&i.then_block.statements, warnings);
                    if let Some(eb) = &i.else_block {
                        self.scan_block_stmts(&eb.statements, warnings);
                    }
                }
                _ => {}
            }
        }
    }

    fn scan_program(&self, program: &Program, warnings: &mut Vec<String>) {
        for item in &program.items {
            if let Item::Function(f) = item {
                self.scan_block_stmts(&f.body.statements, warnings);
            }
        }
    }
}

impl Default for SecurityGatePass {
    fn default() -> Self {
        Self::new()
    }
}

impl Pass for SecurityGatePass {
    fn name(&self) -> &str {
        "SecurityGate"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        let sources: Vec<String> = context.source_map.values().cloned().collect();
        for src in sources {
            self.scan_source_secrets(&src, context);
        }

        let mut warnings = Vec::new();
        if let Some(program) = &context.program {
            self.scan_program(program, &mut warnings);
        }
        for w in warnings {
            eprintln!("⚠️  Security: {}", w);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_hardcoded_password() {
        let mut ctx = CompilationContext::new(
            "t.velin".into(),
            r#"let password = "supersecret123""#.into(),
        );
        let pass = SecurityGatePass::new();
        pass.run(&mut ctx).unwrap();
        assert!(!ctx.errors.is_empty());
    }
}
