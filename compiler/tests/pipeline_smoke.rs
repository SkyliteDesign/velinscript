//! Pipeline / security / multi-file smoke tests

use std::fs;
use velin_compiler::compiler::{VelinCompiler, config::CompilerConfig};
use velin_compiler::passes::{
    parser::ParserPass,
    desugar::DesugaringPass,
    code_order::CodeOrderingPass,
    type_check::TypeCheckPass,
    security_gate::SecurityGatePass,
};

fn compiler_with_security() -> VelinCompiler {
    let config = CompilerConfig::default();
    let mut c = VelinCompiler::new(config);
    c.add_pass(Box::new(ParserPass::new()));
    c.add_pass(Box::new(DesugaringPass::new()));
    c.add_pass(Box::new(CodeOrderingPass::new()));
    c.add_pass(Box::new(TypeCheckPass::new(true)));
    c.add_pass(Box::new(SecurityGatePass::new()));
    c
}

#[test]
fn pipeline_hello_ok() {
    let src = r#"
@GET("/hello")
fn hello(): string {
    return "ok";
}
"#;
    let ctx = compiler_with_security()
        .compile("main.velin".into(), src.into())
        .expect("compile");
    assert!(!ctx.has_errors(), "{:?}", ctx.errors);
}

#[test]
fn security_gate_flags_hardcoded_secret() {
    let src = r#"
fn bad(): string {
    let api_key = "sk-1234567890abcdefghijklmnop";
    return api_key;
}
"#;
    let ctx = compiler_with_security()
        .compile("main.velin".into(), src.into())
        .expect("compile");
    let has_signal = ctx.has_errors();
    assert!(has_signal, "expected security errors for hardcoded secret");
}

#[test]
fn path_traversal_module_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let main = root.join("main.velin");
    fs::write(
        &main,
        r#"
use ../evil;
fn hello(): string { return "x"; }
"#,
    )
    .unwrap();
    let code = fs::read_to_string(&main).unwrap();
    let ctx = compiler_with_security()
        .compile(main.to_string_lossy().into(), code)
        .expect("compile");
    assert!(ctx.has_errors(), "path traversal must fail");
    let blob = format!("{:?}", ctx.errors).to_lowercase();
    assert!(
        blob.contains("traversal")
            || blob.contains("invalid module")
            || blob.contains("dotdot")
            || blob.contains("expected identifier"),
        "{}",
        blob
    );
}

#[test]
fn multi_file_use_loads_module() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(
        root.join("helpers.velin"),
        r#"
fn greet(): string {
    return "hi";
}
"#,
    )
    .unwrap();
    let main = root.join("main.velin");
    fs::write(
        &main,
        r#"
use helpers;
@GET("/hello")
fn hello(): string {
    return "ok";
}
"#,
    )
    .unwrap();
    let code = fs::read_to_string(&main).unwrap();
    let ctx = compiler_with_security()
        .compile(main.to_string_lossy().into(), code)
        .expect("compile");
    assert!(!ctx.has_errors(), "{:?}", ctx.errors);
    assert!(
        ctx.source_map.len() >= 2 || ctx.metadata.contains_key("compilation_order"),
        "expected multi-file sources or order metadata"
    );
}
