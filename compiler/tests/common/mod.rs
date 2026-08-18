//! Shared helpers for compiler integration tests (not a test crate).

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;
use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::lowering::{axum_cargo_toml, axum_main_wrapper_with_port};
use velin_compiler::codegen::traits::TargetLanguage;
use velin_compiler::ir::builder::IRBuilder;
use velin_compiler::parser::parser::Parser;

pub fn compile_ir(src: &str, target: TargetLanguage) -> String {
    let program = Parser::parse(src).expect("parse");
    let mut builder = IRBuilder::new();
    let ir = builder.build_module(&program);
    IRCodeGenerator::new(target)
        .generate(&ir)
        .expect("codegen")
}

pub fn compile_ir_rust(src: &str) -> String {
    compile_ir(src, TargetLanguage::Rust)
}

pub fn http_request(
    port: u16,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: Option<&str>,
) -> (u16, String) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let mut req = format!(
        "{} {} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n",
        method, path
    );
    for (k, v) in headers {
        req.push_str(&format!("{}: {}\r\n", k, v));
    }
    if let Some(b) = body {
        req.push_str("Content-Type: application/json\r\n");
        req.push_str(&format!("Content-Length: {}\r\n", b.len()));
        req.push_str("\r\n");
        req.push_str(b);
    } else {
        req.push_str("\r\n");
    }
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

pub fn http_get(port: u16, path: &str, auth: Option<&str>) -> (u16, String) {
    let mut headers = Vec::new();
    if let Some(a) = auth {
        headers.push(("Authorization", a));
    }
    http_request(port, "GET", path, &headers, None)
}

pub fn wait_port(port: u16, child: &mut Child) {
    for _ in 0..80 {
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

pub fn find_debug_bin(root: &Path, name: &str) -> PathBuf {
    let exe_name = if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };
    let direct = root.join("target").join("debug").join(&exe_name);
    if direct.exists() {
        return direct;
    }
    if let Ok(td) = std::env::var("CARGO_TARGET_DIR") {
        let p = PathBuf::from(td).join("debug").join(&exe_name);
        if p.exists() {
            return p;
        }
    }
    panic!("binary not found for {}", name);
}

pub struct RunningServer {
    pub port: u16,
    child: Child,
    _dir: tempfile::TempDir,
}

impl Drop for RunningServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl RunningServer {
    pub fn kill(self) {
        drop(self);
    }
}

pub fn free_tcp_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind 0");
    let port = listener.local_addr().expect("addr").port();
    drop(listener);
    port
}

pub fn spawn_generated_axum(src: &str, pkg: &str, port: Option<u16>) -> RunningServer {
    let body = compile_ir_rust(src);
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("Cargo.toml"), axum_cargo_toml(pkg)).unwrap();
    fs::write(
        root.join("src").join("main.rs"),
        axum_main_wrapper_with_port(&body, 8080),
    )
    .unwrap();

    let build = Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(root.join("Cargo.toml"))
        .env("CARGO_TERM_COLOR", "never")
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_BUILD_TARGET_DIR")
        .status()
        .expect("cargo build");
    assert!(build.success(), "cargo build failed for {}", pkg);

    let port = port.unwrap_or_else(free_tcp_port);
    let exe = find_debug_bin(root, pkg);
    assert!(exe.exists(), "expected generated binary at {:?}", exe);
    let mut child = Command::new(&exe)
        .env("PORT", port.to_string())
        .env("VELIN_HOST", "127.0.0.1")
        .env_remove("CARGO_TARGET_DIR")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn server");
    wait_port(port, &mut child);
    RunningServer {
        port,
        child,
        _dir: dir,
    }
}
