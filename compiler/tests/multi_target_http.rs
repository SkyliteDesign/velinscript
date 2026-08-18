//! Multi-target IR HTTP smoke (string presence; Rust has cargo E2E separately)

use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::ir::builder::IRBuilder;
use velin_compiler::parser::parser::Parser;

fn compile(src: &str, target: TargetLanguage) -> String {
    let program = Parser::parse(src).expect("parse");
    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    IRCodeGenerator::new(target).generate(&ir).expect("codegen")
}

const HELLO: &str = r#"
@GET("/hello")
fn hello(): string {
    return "ok";
}
"#;

#[test]
fn python_fastapi_route() {
    let out = compile(HELLO, TargetLanguage::Python);
    assert!(out.contains("FastAPI"), "{}", out);
    assert!(out.contains("@app.get(\"/hello\")"), "{}", out);
}

#[test]
fn php_laravel_route() {
    let out = compile(HELLO, TargetLanguage::Php);
    assert!(out.contains("Route::get"), "{}", out);
    assert!(out.contains("/hello"), "{}", out);
}

#[test]
fn javascript_express_route() {
    let out = compile(HELLO, TargetLanguage::JavaScript);
    assert!(out.contains("express"), "{}", out);
    assert!(out.contains("app.get('/hello'"), "{}", out);
}

#[test]
fn typescript_express_route() {
    let out = compile(HELLO, TargetLanguage::TypeScript);
    assert!(out.contains("express"), "{}", out);
    assert!(out.contains("app.get('/hello'"), "{}", out);
}

#[test]
fn go_gin_route() {
    let out = compile(HELLO, TargetLanguage::Go);
    assert!(out.contains("gin"), "{}", out);
    assert!(out.contains("GET(\"/hello\""), "{}", out);
}

#[test]
fn java_spring_mapping() {
    let out = compile(HELLO, TargetLanguage::Java);
    assert!(out.contains("GetMapping"), "{}", out);
    assert!(out.contains("/hello"), "{}", out);
}

#[test]
fn csharp_aspnet_httpget() {
    let out = compile(HELLO, TargetLanguage::CSharp);
    assert!(out.contains("HttpGet"), "{}", out);
    assert!(out.contains("hello"), "{}", out);
}
