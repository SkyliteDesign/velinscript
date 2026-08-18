//! Framework wire: CLI/Config → Context → CodegenPass → IRCodeGenerator → Lowering → Output

use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::lowering::{self, is_actix_framework};
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::compiler::config::CompilerConfig;
use velin_compiler::compiler::{VelinCompiler, CompilationContext};
use velin_compiler::ir::builder::IRBuilder;
use velin_compiler::parser::parser::Parser;
use velin_compiler::passes::codegen::CodegenPass;
use velin_compiler::passes::parser::ParserPass;

const HELLO: &str = r#"
@GET("/hello")
fn hello(): string {
    return "ok";
}
"#;

fn config_with_framework(fw: Option<&str>) -> CompilerConfig {
    let mut c = CompilerConfig::default();
    c.framework = fw.map(|s| s.to_string());
    c
}

#[test]
fn stage_config_rust_framework_default_axum() {
    let c = CompilerConfig::default();
    assert_eq!(c.rust_framework(), "axum");
    let c2 = config_with_framework(Some("actix"));
    assert_eq!(c2.rust_framework(), "actix");
}

#[test]
fn stage_config_to_context() {
    let config = config_with_framework(Some("actix"));
    let compiler = VelinCompiler::new(config);
    let ctx = compiler
        .compile("t.velin".into(), "fn x(): string { return \"a\"; }".into())
        .unwrap();
    // Parser may fail or succeed; framework must be on context either way
    assert_eq!(ctx.framework.as_deref(), Some("actix"));
    assert_eq!(ctx.metadata.get("framework").map(|s| s.as_str()), Some("actix"));
}

#[test]
fn stage_context_to_codegen_pass_effective_framework() {
    let mut ctx = CompilationContext::new("t.velin".into(), HELLO.into());
    ctx.framework = Some("actix".into());
    let pass = CodegenPass::new(None, false, TargetLanguage::Rust, Some("axum".into()));
    assert_eq!(
        pass.effective_framework(&ctx).as_deref(),
        Some("actix"),
        "context must override pass constructor"
    );
}

#[test]
fn stage_ir_codegen_receives_framework() {
    let gen = IRCodeGenerator::new(TargetLanguage::Rust).with_framework(Some("actix".into()));
    assert!(is_actix_framework(gen.framework()));
    let gen2 = IRCodeGenerator::new(TargetLanguage::Rust);
    assert!(!is_actix_framework(gen2.framework()));
}

#[test]
fn stage_lowering_actix_vs_axum_markers() {
    let routes = lowering::collect_routes(&{
        let program = Parser::parse(HELLO).unwrap();
        let mut b = IRBuilder::new();
        b.build_module(&program)
    });
    let axum = lowering::generate_axum_router(&routes);
    let actix = lowering::generate_actix_router(&routes);
    assert!(axum.contains("create_router") && axum.contains("Router"));
    assert!(actix.contains("configure_routes") && actix.contains("create_app"));
    assert!(!actix.contains("create_router() -> Router"));
}

#[test]
fn stage_output_diff_axum_vs_actix() {
    let program = Parser::parse(HELLO).unwrap();
    let mut b = IRBuilder::new();
    let ir = b.build_module(&program);
    let axum = IRCodeGenerator::new(TargetLanguage::Rust)
        .with_framework(Some("axum".into()))
        .generate(&ir)
        .unwrap();
    let actix = IRCodeGenerator::new(TargetLanguage::Rust)
        .with_framework(Some("actix".into()))
        .generate(&ir)
        .unwrap();
    assert!(axum.contains("use axum::"), "{}", axum);
    assert!(axum.contains("create_router"), "{}", axum);
    assert!(actix.contains("actix_web"), "{}", actix);
    assert!(actix.contains("configure_routes"), "{}", actix);
    assert!(!actix.contains("use axum::"), "actix path must not emit axum imports");
}

#[test]
fn stage_full_pipeline_context_framework_reaches_output() {
    let mut config = config_with_framework(Some("actix"));
    config.enable_type_check = false;
    let mut compiler = VelinCompiler::new(config);
    compiler.add_pass(Box::new(ParserPass::new()));
    compiler.add_pass(Box::new(CodegenPass::new(
        None,
        false,
        TargetLanguage::Rust,
        None,
    )));
    let ctx = compiler.compile("main.velin".into(), HELLO.into()).unwrap();
    assert!(!ctx.has_errors(), "{:?}", ctx.errors);
    assert_eq!(ctx.metadata.get("codegen_framework").map(|s| s.as_str()), Some("actix"));
    let out = ctx.metadata.get("generated_code").expect("generated_code");
    assert!(out.contains("actix_web"), "{}", out);
    assert!(out.contains("configure_routes"), "{}", out);
}
