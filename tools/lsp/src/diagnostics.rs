//! Compiler diagnostics for LSP (parse + typecheck + security).

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use velin_compiler::compiler::config::CompilerConfig;
use velin_compiler::compiler::VelinCompiler;
use velin_compiler::error::CompilerError;
use velin_compiler::parser::parser::Parser;
use velin_compiler::passes::parser::ParserPass;
use velin_compiler::passes::security_gate::SecurityGatePass;
use velin_compiler::passes::type_check::TypeCheckPass;

/// Stage 1+2: parse errors and compiler pipeline diagnostics.
pub fn collect_diagnostics(uri_path: &str, source: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    // Stage 1 — Parser
    match Parser::parse(source) {
        Ok(_) => {}
        Err(e) => {
            out.push(diag(
                e.line.saturating_sub(1),
                e.column.saturating_sub(1),
                format!("Parse: {}", e.message),
                DiagnosticSeverity::ERROR,
            ));
            return out; // stop early; typecheck needs AST
        }
    }

    // Stage 2 — TypeCheck + SecurityGate via product compiler path
    let mut config = CompilerConfig::default();
    config.enable_type_check = true;
    config.enable_optimization = false;
    let mut compiler = VelinCompiler::new(config);
    compiler.add_pass(Box::new(ParserPass::new()));
    compiler.add_pass(Box::new(TypeCheckPass::new(true)));
    compiler.add_pass(Box::new(SecurityGatePass::new()));

    if let Ok(ctx) = compiler.compile(uri_path.to_string(), source.to_string()) {
        for err in &ctx.errors {
            out.push(compiler_error_to_diagnostic(err));
        }
    }

    out
}

fn compiler_error_to_diagnostic(err: &CompilerError) -> Diagnostic {
    let (line, col, msg, severity) = match err {
        CompilerError::Parse {
            message,
            line,
            column,
            ..
        } => (
            *line,
            *column,
            format!("Parse: {}", message),
            DiagnosticSeverity::ERROR,
        ),
        CompilerError::Type {
            message,
            line,
            column,
            ..
        } => (
            *line,
            *column,
            format!("Type: {}", message),
            DiagnosticSeverity::ERROR,
        ),
        other => {
            let s = other.to_string();
            let sev = if s.to_lowercase().contains("security") {
                DiagnosticSeverity::WARNING
            } else {
                DiagnosticSeverity::ERROR
            };
            (0, 0, s, sev)
        }
    };
    diag(line.saturating_sub(1), col.saturating_sub(1), msg, severity)
}

fn diag(line: usize, col: usize, message: String, severity: DiagnosticSeverity) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position {
                line: line as u32,
                character: col as u32,
            },
            end: Position {
                line: line as u32,
                character: (col + 1) as u32,
            },
        },
        severity: Some(severity),
        code: None,
        code_description: None,
        source: Some("velin".into()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage1_parse_error_diagnostic() {
        let diags = collect_diagnostics("t.velin", "fn {{{");
        assert!(!diags.is_empty());
        assert!(diags[0].message.to_lowercase().contains("parse"));
    }

    #[test]
    fn stage2_security_secret_diagnostic() {
        let src = r#"
fn bad(): string {
    let password = "supersecret123";
    return password;
}
"#;
        let diags = collect_diagnostics("t.velin", src);
        assert!(
            diags.iter().any(|d| d.message.to_lowercase().contains("security")
                || d.message.to_lowercase().contains("secret")
                || d.message.to_lowercase().contains("password")),
            "{:?}",
            diags
        );
    }
}
