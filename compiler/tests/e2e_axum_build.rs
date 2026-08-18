//! E2E: Velin Hello → IR Rust/Axum → cargo check

use std::fs;
use std::process::Command;
use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::lowering::{axum_cargo_toml, axum_main_wrapper};
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::ir::builder::IRBuilder;
use velin_compiler::parser::parser::Parser;

#[test]
fn e2e_hello_axum_cargo_check() {
    let src = r#"
@GET("/hello")
fn hello(): string {
    return "Hello, World!";
}
"#;
    let program = Parser::parse(src).expect("parse");
    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    let body = IRCodeGenerator::new(TargetLanguage::Rust)
        .generate(&ir)
        .expect("codegen");
    assert!(body.contains("create_router"));
    assert!(body.contains("use axum::"));

    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("Cargo.toml"), axum_cargo_toml("e2e_hello")).unwrap();
    fs::write(root.join("src").join("main.rs"), axum_main_wrapper(&body)).unwrap();

    let status = Command::new("cargo")
        .args(["check", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .env("CARGO_TERM_COLOR", "never")
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_BUILD_TARGET_DIR")
        .status()
        .expect("cargo check");
    assert!(status.success(), "cargo check failed for generated Axum project");
}

#[test]
fn e2e_auth_requires_authorization_code() {
    let src = r#"
@GET("/secure")
@Auth
fn secure(): string {
    return "ok";
}
"#;
    let program = Parser::parse(src).expect("parse");
    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    let body = IRCodeGenerator::new(TargetLanguage::Rust)
        .generate(&ir)
        .expect("codegen");
    assert!(body.contains("UNAUTHORIZED"));
    assert!(body.contains("AUTHORIZATION"));
}
