//! SystemGenerator smoke: REST fixture → Axum cargo check

use std::fs;
use std::process::Command;
use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::lowering::{axum_cargo_toml, axum_main_wrapper};
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::codegen::{APICall, SystemGenerator};
use velin_compiler::ir::builder::IRBuilder;
use velin_compiler::parser::ast::Item;
use velin_compiler::parser::parser::Parser;

#[test]
fn systemgen_rest_scaffold_cargo_check() {
    let src = r#"
@GET("/api/items")
fn list_items(): string {
    return "[]";
}

@POST("/api/items")
fn create_item(): string {
    return "created";
}
"#;
    let program = Parser::parse(src).expect("parse");
    let sys = SystemGenerator::new(None);
    let mut generated_any = false;
    for item in &program.items {
        if let Item::Function(f) = item {
            if !f.decorators.is_empty() {
                let api = APICall::from_ast(f);
                let generated = sys.generate_system(&api).expect("systemgen");
                assert!(!generated.components.is_empty());
                generated_any = true;
            }
        }
    }
    assert!(generated_any);

    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    let body = IRCodeGenerator::new(TargetLanguage::Rust)
        .generate(&ir)
        .expect("codegen");

    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("Cargo.toml"), axum_cargo_toml("sysgen_rest")).unwrap();
    fs::write(root.join("src").join("main.rs"), axum_main_wrapper(&body)).unwrap();

    let status = Command::new("cargo")
        .args(["check", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .env("CARGO_TERM_COLOR", "never")
        .status()
        .expect("cargo");
    assert!(status.success());
}

#[test]
fn systemgen_auth_db_ai_fixtures_emit_components() {
    let fixtures = [
        r#"
@GET("/secure")
@Auth
fn secure(): string { return "ok"; }
"#,
        r#"
@GET("/users")
fn users(): string { return "[]"; }
"#,
        r#"
@POST("/chat")
fn chat(): string { return "hi"; }
"#,
    ];
    let sys = SystemGenerator::new(None);
    for src in fixtures {
        let program = Parser::parse(src).expect("parse");
        for item in &program.items {
            if let Item::Function(f) = item {
                let api = APICall::from_ast(f);
                let generated = sys.generate_system(&api).expect("systemgen");
                assert!(!generated.components.is_empty(), "src={}", src);
            }
        }
    }
}
