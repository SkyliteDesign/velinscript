//! P0 Golden tests: Notizzettel-API → Default IR path → Rust/Axum surface
//!
//! Proves VelinScript 3.5.0 definition:
//! describe a small API → compile via IR → runnable Axum-oriented Rust.

use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::ir::builder::IRBuilder;
use velin_compiler::parser::parser::Parser;

fn compile_ir_rust(source: &str) -> String {
    let program = Parser::parse(source).expect("parse should succeed");
    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    let gen = IRCodeGenerator::new(TargetLanguage::Rust);
    gen.generate(&ir).expect("IR codegen should succeed")
}

#[test]
fn golden_hello_api_has_axum_route() {
    let src = r#"
@GET("/hello")
fn hello(): string {
    return "Hello, World!";
}
"#;
    let out = compile_ir_rust(src);
    assert!(
        out.contains("use axum::"),
        "expected axum imports in IR output:\n{}",
        out
    );
    assert!(
        out.contains(".route(\"/hello\", get(hello))"),
        "expected GET /hello route wiring:\n{}",
        out
    );
    assert!(
        out.contains("create_router"),
        "expected create_router():\n{}",
        out
    );
    assert!(
        out.contains("async fn hello"),
        "expected async handler hello:\n{}",
        out
    );
    assert!(
        out.contains("IntoResponse"),
        "expected IntoResponse return for HTTP handler:\n{}",
        out
    );
}

#[test]
fn golden_auth_api_has_middleware() {
    let src = r#"
@GET("/secure")
@Auth
fn secure(): string {
    return "ok";
}
"#;
    let out = compile_ir_rust(src);
    assert!(
        out.contains("velin_auth_middleware"),
        "expected auth middleware stub:\n{}",
        out
    );
    assert!(
        out.contains("from_fn(velin_auth_middleware)")
            || out.contains("layer(middleware::from_fn(velin_auth_middleware))"),
        "expected middleware layer on router:\n{}",
        out
    );
    assert!(
        out.contains(".route(\"/secure\", get(secure)")
            && out.contains("from_fn(velin_auth_middleware)"),
        "expected /secure route with per-route auth layer:\n{}",
        out
    );
    assert!(
        out.contains("UNAUTHORIZED") || out.contains("Authorization"),
        "expected auth enforcement in output:\n{}",
        out
    );
}

#[test]
fn golden_named_decorator_args_preserved_in_ir_attributes() {
    use velin_compiler::ir::ir::IRAttributeArg;
    use velin_compiler::ir::builder::IRBuilder;
    use velin_compiler::parser::parser::Parser;

    let src = r#"
@Cache(ttl = "60")
@GET("/cached")
fn cached(): string {
    return "x";
}
"#;
    let program = Parser::parse(src).expect("parse");
    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    let func = ir
        .functions
        .iter()
        .find(|f| f.name == "cached")
        .expect("cached fn");
    let cache = func
        .attributes
        .iter()
        .find(|a| a.name == "Cache")
        .expect("Cache attribute");
    match cache.args.first() {
        Some(IRAttributeArg::Named { name, value }) => {
            assert_eq!(name, "ttl");
            match value.as_ref() {
                IRAttributeArg::String(s) => assert_eq!(s, "60"),
                other => panic!("expected string value, got {:?}", other),
            }
        }
        other => panic!("expected Named arg, got {:?}", other),
    }
}

#[test]
fn golden_hello_query_and_interpolation() {
    let src = r#"
@GET("/hello")
fn hello(name: string): string {
    return "Hello {name}";
}
"#;
    let out = compile_ir_rust(src);
    assert!(
        out.contains("Query(q)") || out.contains("Query<"),
        "expected Query HashMap for GET scalar:\n{}",
        out
    );
    assert!(
        !out.contains("Query<String>"),
        "must not use Query<String>:\n{}",
        out
    );
    assert!(
        out.contains("format!"),
        "expected format! for Hello interpolation:\n{}",
        out
    );
    assert!(
        out.contains("BAD_REQUEST") || out.contains("missing query parameter"),
        "expected 400 for missing query:\n{}",
        out
    );
}
