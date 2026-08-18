//! Interpolation matrix: 8 targets × string/int/float/bool/multi/expr/empty/special/JSON-like.

#[path = "common/mod.rs"]
mod common;

use common::compile_ir;
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::parser::lexer::{Lexer, Token};

const SRC: &str = r#"
@GET("/s")
fn i_str(name: string): string {
    return "Hello {name}";
}

@GET("/i")
fn i_int(n: int): string {
    return "n={n}";
}

@GET("/f")
fn i_float(x: float): string {
    return "x={x}";
}

@GET("/b")
fn i_bool(on: bool): string {
    return "on={on}";
}

@GET("/m")
fn i_multi(a: string, b: string): string {
    return "{a}-{b}";
}

@GET("/e")
fn i_empty(): string {
    let name = "";
    return "A{name}B";
}

@GET("/sp")
fn i_special(name: string): string {
    return "[{name}]";
}

@GET("/j")
fn i_json(): string {
    return '{"k":"v"}';
}

@GET("/expr")
fn i_expr(n: int): string {
    return "n={n + 1}";
}
"#;

fn assert_not_rust_string_plus(out: &str) {
    assert!(
        out.contains("format!"),
        "Rust interpolation must use format!, got:\n{}",
        out
    );
    assert!(
        !out.contains("let tmp_0 = \"Hello \" + "),
        "must not emit string + for interpolation:\n{}",
        out
    );
}

#[test]
fn interpolation_rust_uses_format() {
    let out = compile_ir(SRC, TargetLanguage::Rust);
    assert_not_rust_string_plus(&out);
    assert!(out.contains("Hello {name}") || out.contains("format!"), "{}", out);
}

#[test]
fn interpolation_all_eight_targets_compile_matrix() {
    for target in [
        TargetLanguage::Rust,
        TargetLanguage::Php,
        TargetLanguage::Python,
        TargetLanguage::TypeScript,
        TargetLanguage::JavaScript,
        TargetLanguage::Go,
        TargetLanguage::Java,
        TargetLanguage::CSharp,
    ] {
        let out = compile_ir(SRC, target.clone());
        assert!(
            !out.is_empty(),
            "{:?} produced empty interpolation output",
            target
        );
        assert!(
            out.contains("Hello") || out.contains("format") || out.contains("Sprintf") || out.contains("Concat"),
            "{:?} missing interpolated hello surface:\n{}",
            target,
            out
        );
    }
}

#[test]
fn interpolation_php_concat() {
    let out = compile_ir(SRC, TargetLanguage::Php);
    assert!(out.contains(" . ") || out.contains("Hello"), "php:\n{}", out);
}

#[test]
fn interpolation_python_str_concat() {
    let out = compile_ir(SRC, TargetLanguage::Python);
    assert!(out.contains("str(") || out.contains(" + "), "python:\n{}", out);
}

#[test]
fn interpolation_typescript_string() {
    let out = compile_ir(SRC, TargetLanguage::TypeScript);
    assert!(out.contains("String(") || out.contains(" + "), "ts:\n{}", out);
}

#[test]
fn interpolation_javascript_string() {
    let out = compile_ir(SRC, TargetLanguage::JavaScript);
    assert!(out.contains("String(") || out.contains(" + "), "js:\n{}", out);
}

#[test]
fn interpolation_go_sprintf() {
    let out = compile_ir(SRC, TargetLanguage::Go);
    assert!(out.contains("fmt.Sprintf") || out.contains(" + "), "go:\n{}", out);
}

#[test]
fn interpolation_java_valueof() {
    let out = compile_ir(SRC, TargetLanguage::Java);
    assert!(
        out.contains("String.valueOf") || out.contains(" + "),
        "java:\n{}",
        out
    );
}

#[test]
fn interpolation_csharp_concat() {
    let out = compile_ir(SRC, TargetLanguage::CSharp);
    assert!(
        out.contains("string.Concat") || out.contains(" + "),
        "csharp:\n{}",
        out
    );
}

#[test]
fn lexer_single_quote_and_json_not_format() {
    let mut lexer = Lexer::new("'Hello {name}'");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::String(s) if s == "Hello {name}"));

    let mut lexer = Lexer::new("\"Hello {name}\"");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0], Token::FormatString(_)));

    let mut lexer = Lexer::new("\"{\"");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::String(s) if s == "{"));

    let mut lexer = Lexer::new("\"{}\"");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(&tokens[0], Token::String(s) if s == "{}"));
}

#[test]
fn interpolation_rust_runtime_matrix() {
    use common::{http_get, spawn_generated_axum};
    let srv = spawn_generated_axum(SRC, "e2e_interp_rt", None);
    let port = srv.port;

    let (s, b) = http_get(port, "/s?name=Velin", None);
    assert_eq!(s, 200);
    assert!(b.contains("Hello Velin"), "str {}", b);

    let (s, b) = http_get(port, "/i?n=7", None);
    assert_eq!(s, 200);
    assert!(b.contains("n=7"), "int {}", b);

    let (s, b) = http_get(port, "/f?x=1.5", None);
    assert_eq!(s, 200);
    assert!(b.contains("1.5"), "float {}", b);

    let (s, b) = http_get(port, "/b?on=true", None);
    assert_eq!(s, 200);
    assert!(b.contains("on=true"), "bool {}", b);

    let (s, b) = http_get(port, "/m?a=x&b=y", None);
    assert_eq!(s, 200);
    assert!(b.contains("x-y"), "multi {}", b);

    let (s, b) = http_get(port, "/e", None);
    assert_eq!(s, 200);
    assert!(b.contains("AB"), "empty {}", b);

    let (s, b) = http_get(port, "/sp?name=%3C%26%3E", None);
    assert_eq!(s, 200);
    assert_eq!(b.trim(), "[\u{3c}&\u{3e}]");

    let (s, b) = http_get(port, "/j", None);
    assert_eq!(s, 200);
    assert!(b.contains("k") && b.contains("v"), "json {}", b);

    let (s, b) = http_get(port, "/expr?n=3", None);
    assert_eq!(s, 200);
    assert!(b.contains("n=4"), "expr {}", b);

    srv.kill();
}
