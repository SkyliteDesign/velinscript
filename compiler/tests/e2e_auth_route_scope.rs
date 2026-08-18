//! E2E: Auth middleware only on @Auth routes (Header-Presence scope, not JWT).
//!
//! Scope: generated Axum middleware checks Authorization header *presence* only.
//! `Bearer test` succeeds because a header is present — not because JWT is validated.

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::lowering::{axum_cargo_toml, axum_main_wrapper};
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::ir::builder::IRBuilder;
use velin_compiler::parser::parser::Parser;

const SRC: &str = r#"
@GET("/public")
fn public_ep(): string {
    return "pub-ok";
}

@GET("/secure")
@Auth
fn secure_ep(): string {
    return "sec-ok";
}

@GET("/public2")
fn public_ep2(): string {
    return "pub2-ok";
}

@GET("/secure2")
@Auth
fn secure_ep2(): string {
    return "sec2-ok";
}
"#;

fn http_get(port: u16, path: &str, auth: Option<&str>) -> (u16, String) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .ok();
    let mut req = format!(
        "GET {} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n",
        path
    );
    if let Some(a) = auth {
        req.push_str(&format!("Authorization: {}\r\n", a));
    }
    req.push_str("\r\n");
    stream.write_all(req.as_bytes()).expect("write");
    let mut buf = String::new();
    stream.read_to_string(&mut buf).ok();
    let status = buf
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let body = buf
        .split("\r\n\r\n")
        .nth(1)
        .unwrap_or("")
        .trim()
        .to_string();
    (status, body)
}

fn wait_port(port: u16, child: &mut Child) {
    for _ in 0..40 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        if let Ok(Some(status)) = child.try_wait() {
            panic!("server exited early: {:?}", status);
        }
        thread::sleep(Duration::from_millis(250));
    }
    panic!("server did not bind on {}", port);
}

#[test]
fn e2e_auth_route_scope_runtime() {
    let program = Parser::parse(SRC).expect("parse");
    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    let body = IRCodeGenerator::new(TargetLanguage::Rust)
        .generate(&ir)
        .expect("codegen");

    assert!(
        body.contains("public.merge(protected)")
            || body.contains(".layer(middleware::from_fn"),
        "expected split public/protected router or per-route layer"
    );
    assert!(
        body.contains("velin_auth_middleware"),
        "auth middleware missing"
    );
    assert!(
        (body.contains("let public = Router::new()") && body.contains("let protected = Router::new()"))
            || body.contains("get(secure_ep).layer(middleware::from_fn(velin_auth_middleware))")
            || body.contains("get(secure).layer(middleware::from_fn(velin_auth_middleware))"),
        "expected separate public/protected routers or per-route auth layer; got leak risk"
    );

    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("Cargo.toml"), axum_cargo_toml("e2e_auth_scope")).unwrap();
    fs::write(
        root.join("src").join("main.rs"),
        axum_main_wrapper(&body),
    )
    .unwrap();

    let check = Command::new("cargo")
        .args(["check", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .env("CARGO_TERM_COLOR", "never")
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_BUILD_TARGET_DIR")
        .status()
        .expect("cargo check");
    assert!(check.success(), "cargo check failed");

    let build = Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .env("CARGO_TERM_COLOR", "never")
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_BUILD_TARGET_DIR")
        .status()
        .expect("cargo build");
    assert!(build.success(), "cargo build failed");

    let exe = root
        .join("target")
        .join("debug")
        .join(if cfg!(windows) {
            "e2e_auth_scope.exe"
        } else {
            "e2e_auth_scope"
        });
    // cargo may use CARGO_TARGET_DIR
    let exe = if exe.exists() {
        exe
    } else if let Ok(td) = std::env::var("CARGO_TARGET_DIR") {
        let p = std::path::PathBuf::from(td)
            .join("debug")
            .join(if cfg!(windows) {
                "e2e_auth_scope.exe"
            } else {
                "e2e_auth_scope"
            });
        assert!(p.exists(), "binary not found at {:?}", p);
        p
    } else {
        panic!("binary not found");
    };

    let port: u16 = {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.local_addr().unwrap().port()
    };
    let mut child = Command::new(&exe)
        .env("PORT", port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn server");
    wait_port(port, &mut child);

    // A. public without header
    let (s, b) = http_get(port, "/public", None);
    assert_eq!(s, 200, "A public no header");
    assert!(b.contains("pub-ok"), "A body {}", b);

    // B. protected without header → 401
    let (s, _) = http_get(port, "/secure", None);
    assert_eq!(s, 401, "B secure no header");

    // C. protected with Authorization present (Header-Presence scope, NOT JWT validation)
    let (s, b) = http_get(port, "/secure", Some("Bearer test"));
    assert_eq!(s, 200, "C secure with header presence");
    assert!(b.contains("sec-ok"), "C body {}", b);

    // D. interleaved public2 / secure2
    let (s, _) = http_get(port, "/public2", None);
    assert_eq!(s, 200, "D public2");
    let (s, _) = http_get(port, "/secure2", None);
    assert_eq!(s, 401, "D secure2 no header");
    let (s, _) = http_get(port, "/secure2", Some("Bearer test"));
    assert_eq!(s, 200, "D secure2 with header");

    // E. public + Authorization must NOT be altered by auth middleware
    let (s, b) = http_get(port, "/public", Some("Bearer test"));
    assert_eq!(s, 200, "E public with Authorization header");
    assert!(b.contains("pub-ok"), "E body must stay pub-ok, got {}", b);

    let _ = child.kill();
    let _ = child.wait();
}
