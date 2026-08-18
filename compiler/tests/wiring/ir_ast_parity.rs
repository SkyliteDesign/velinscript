//! IR default vs AST-legacy semantic parity (Hello HTTP)

use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::compiler::config::CompilerConfig;
use velin_compiler::compiler::VelinCompiler;
use velin_compiler::passes::codegen::CodegenPass;
use velin_compiler::passes::parser::ParserPass;

const HELLO: &str = r#"
@GET("/hello")
fn hello(): string {
    return "ok";
}
"#;

fn compile(use_ir: bool) -> String {
    let mut config = CompilerConfig::default();
    config.use_ir = use_ir;
    config.enable_type_check = false;
    let mut compiler = VelinCompiler::new(config);
    compiler.add_pass(Box::new(ParserPass::new()));
    compiler.add_pass(Box::new(
        CodegenPass::new(None, false, TargetLanguage::Rust, None).with_ir(use_ir),
    ));
    let ctx = compiler.compile("main.velin".into(), HELLO.into()).unwrap();
    assert!(!ctx.has_errors(), "{:?}", ctx.errors);
    assert_eq!(
        ctx.metadata.get("codegen_use_ir").map(|s| s.as_str()),
        Some(if use_ir { "true" } else { "false" })
    );
    ctx.metadata
        .get("generated_code")
        .cloned()
        .expect("code")
}

fn semantic_markers(code: &str) -> (bool, bool, bool) {
    let has_get_hello = code.contains("/hello");
    let has_hello_fn = code.contains("hello") || code.contains("fn hello");
    let has_http = code.contains("Router")
        || code.contains("route")
        || code.contains("@GET")
        || code.contains("get(");
    (has_get_hello, has_hello_fn, has_http)
}

#[test]
fn default_path_is_ir() {
    let mut config = CompilerConfig::default();
    assert!(config.use_ir);
    config.use_ir = true;
    let ir = compile(true);
    assert!(
        ir.contains("create_router") || ir.contains("use axum"),
        "IR path should emit axum router: {}",
        ir
    );
}

#[test]
fn ir_and_ast_legacy_share_hello_semantics() {
    let ir = compile(true);
    let ast = compile(false);
    let (a1, a2, a3) = semantic_markers(&ir);
    let (b1, b2, b3) = semantic_markers(&ast);
    assert!(a1 && b1, "both must mention /hello\nir={}\nast={}", ir, ast);
    assert!(a2 && b2, "both must mention hello handler");
    assert!(a3 && b3, "both must show HTTP surface");
}
