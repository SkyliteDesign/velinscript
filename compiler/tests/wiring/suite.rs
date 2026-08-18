//! Generic wiring suite for core product features

use std::fs;
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::compiler::config::CompilerConfig;
use velin_compiler::compiler::VelinCompiler;
use velin_compiler::optimizer::parallelization::ParallelizationAnalyzer;
use velin_compiler::passes::codegen::CodegenPass;
use velin_compiler::passes::parser::ParserPass;
use velin_compiler::passes::security_gate::SecurityGatePass;

fn pipeline(config: CompilerConfig, src: &str) -> velin_compiler::compiler::CompilationContext {
    let use_ir = config.use_ir;
    let target = config.target;
    let framework = config.framework.clone();
    let enable_opt = config.enable_optimization;
    let mut c = VelinCompiler::new(config);
    c.add_pass(Box::new(ParserPass::new()));
    if enable_opt {
        c.add_pass(Box::new(ParallelizationAnalyzer::new()));
    }
    c.add_pass(Box::new(SecurityGatePass::new()));
    c.add_pass(Box::new(CodegenPass::new(
        None,
        false,
        target,
        framework,
    ).with_ir(use_ir)));
    c.compile("main.velin".into(), src.into()).unwrap()
}

#[test]
fn wire_http_auth_reaches_output() {
    let src = r#"
@GET("/secure")
@Auth
fn secure(): string { return "ok"; }
"#;
    let mut config = CompilerConfig::default();
    config.enable_type_check = false;
    let ctx = pipeline(config, src);
    assert!(!ctx.has_errors(), "{:?}", ctx.errors);
    let out = ctx.metadata.get("generated_code").unwrap();
    assert!(out.contains("UNAUTHORIZED") || out.contains("Authorization"), "{}", out);
}

#[test]
fn wire_security_gate_blocks_secret() {
    let src = r#"
fn bad(): string {
    let password = "supersecret123";
    return password;
}
"#;
    let mut config = CompilerConfig::default();
    config.enable_type_check = false;
    let ctx = pipeline(config, src);
    assert!(ctx.has_errors(), "security gate must error");
}

#[test]
fn wire_optimization_metadata_when_enabled() {
    let src = r#"
fn hello(): string { return "ok"; }
"#;
    let mut on = CompilerConfig::default();
    on.enable_type_check = false;
    on.enable_optimization = true;
    let ctx_on = pipeline(on, src);
    assert_eq!(
        ctx_on.metadata.get("parallelization_ran").map(|s| s.as_str()),
        Some("true")
    );

    let mut off = CompilerConfig::default();
    off.enable_type_check = false;
    off.enable_optimization = false;
    let mut c = VelinCompiler::new(off);
    c.add_pass(Box::new(ParserPass::new()));
    c.add_pass(Box::new(CodegenPass::new(None, false, TargetLanguage::Rust, None)));
    let ctx_off = c.compile("main.velin".into(), src.into()).unwrap();
    assert!(ctx_off.metadata.get("parallelization_ran").is_none());
}

#[test]
fn wire_orchestrator_sets_module_order() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("helpers.velin"),
        "fn greet(): string { return \"hi\"; }\n",
    )
    .unwrap();
    let main = dir.path().join("main.velin");
    fs::write(
        &main,
        "use helpers;\nfn hello(): string { return \"ok\"; }\n",
    )
    .unwrap();
    let code = fs::read_to_string(&main).unwrap();
    let mut config = CompilerConfig::default();
    config.enable_type_check = false;
    let mut c = VelinCompiler::new(config);
    c.add_pass(Box::new(ParserPass::new()));
    let ctx = c
        .compile(main.to_string_lossy().into(), code)
        .unwrap();
    assert!(!ctx.has_errors(), "{:?}", ctx.errors);
    let order = ctx.metadata.get("module_order").map(|s| s.as_str()).unwrap_or("");
    assert!(
        order.contains("helpers") || ctx.source_map.len() >= 2,
        "module_order={} sources={}",
        order,
        ctx.source_map.len()
    );
}

#[test]
fn wire_multi_target_python_http() {
    let src = r#"
@GET("/hello")
fn hello(): string { return "ok"; }
"#;
    let mut config = CompilerConfig::default();
    config.enable_type_check = false;
    config.target = TargetLanguage::Python;
    let ctx = pipeline(config, src);
    let out = ctx.metadata.get("generated_code").unwrap();
    assert!(out.contains("FastAPI") || out.contains("@app.get"), "{}", out);
}
