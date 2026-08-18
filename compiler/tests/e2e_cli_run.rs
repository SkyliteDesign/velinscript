//! CLI `velin run` E2E: process, port 8080, GET /ping and /hello?name=Velin, .vel alias.

use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static CLI_RUN_LOCK: Mutex<()> = Mutex::new(());

const HELLO: &str = r#"
@GET("/ping")
fn ping(): string {
    return "ok";
}

@GET("/hello")
fn hello(name: string): string {
    return "Hello {name}";
}

@GET("/cli-e2e")
fn marker(): string {
    return "cli-e2e-marker";
}
"#;

fn velin_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_velin"))
}

fn http_get(port: u16, path: &str) -> (u16, String) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let req = format!(
        "GET {} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n",
        path
    );
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

fn port_is_open(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}

fn free_tcp_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind 0");
    let port = listener.local_addr().expect("addr").port();
    drop(listener);
    port
}

fn wait_port(port: u16, child: &mut Child) {
    for _ in 0..120 {
        if port_is_open(port) {
            if let Ok(Some(status)) = child.try_wait() {
                panic!("velin run exited after bind race: {:?}", status);
            }
            thread::sleep(Duration::from_millis(200));
            return;
        }
        if let Ok(Some(status)) = child.try_wait() {
            panic!("velin run exited early: {:?}", status);
        }
        thread::sleep(Duration::from_millis(500));
    }
    panic!("velin run did not bind on {}", port);
}

fn http_get_retry(port: u16, path: &str) -> (u16, String) {
    let mut last = (0u16, String::new());
    for _ in 0..20 {
        last = http_get(port, path);
        if last.0 != 0 {
            return last;
        }
        thread::sleep(Duration::from_millis(200));
    }
    last
}

fn run_and_assert(filename: &str, port: u16) {
    let _guard = CLI_RUN_LOCK.lock().unwrap();
    assert!(
        !port_is_open(port),
        "port {} already in use — refusing to attach to a foreign process",
        port
    );
    let dir = tempfile::tempdir().expect("sandbox");
    let src = dir.path().join(filename);
    fs::write(&src, HELLO).unwrap();

    let mut child = Command::new(velin_bin())
        .arg("run")
        .arg(&src)
        .arg("--port")
        .arg(port.to_string())
        .arg("--host")
        .arg("127.0.0.1")
        .current_dir(dir.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("spawn velin run");

    wait_port(port, &mut child);
    assert!(
        child.try_wait().ok().flatten().is_none(),
        "velin run process must still be alive after bind"
    );

    let (s, _) = http_get_retry(port, "/ping");
    assert_eq!(s, 200, "GET /ping via velin run ({})", filename);

    let (s, b) = http_get_retry(port, "/hello?name=Velin");
    assert_eq!(s, 200, "GET /hello?name=Velin ({})", filename);
    assert!(b.contains("Hello Velin"), "body {}", b);

    let (s, b) = http_get_retry(port, "/cli-e2e");
    assert_eq!(s, 200, "unique marker via velin run ({})", filename);
    assert!(b.contains("cli-e2e-marker"), "marker body {}", b);

    let _ = child.kill();
    let _ = child.wait();
}

#[test]
fn e2e_cli_run_hello_velin_port_8080() {
    let port = match std::net::TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => {
            drop(listener);
            8080
        }
        Err(e) => {
            eprintln!(
                "Port 8080 nicht bindbar auf diesem Host ({e}); Default bleibt 8080, Runtime-E2E nutzt einen freien Port."
            );
            free_tcp_port()
        }
    };
    run_and_assert("hello.velin", port);
}

#[test]
fn e2e_cli_run_hello_vel_alias() {
    run_and_assert("hello.vel", free_tcp_port());
}

#[test]
fn gitignore_does_not_ignore_compiler_tests() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf();
    let out = Command::new("git")
        .args([
            "check-ignore",
            "-v",
            "compiler/tests/e2e_auth_route_scope.rs",
        ])
        .current_dir(&repo)
        .output()
        .expect("git check-ignore");
    let text = String::from_utf8_lossy(&out.stdout);
    // Negation patterns still "match" for check-ignore -v; the last rule must un-ignore.
    assert!(
        text.is_empty() || text.contains("!compiler/tests"),
        "compiler/tests/*.rs must not be gitignored; got: {}",
        text
    );
    let plain = Command::new("git")
        .args(["check-ignore", "compiler/tests/e2e_auth_route_scope.rs"])
        .current_dir(&repo)
        .status()
        .expect("git check-ignore status");
    assert!(
        !plain.success(),
        "git check-ignore without -v must exit unignored (non-zero) for compiler/tests/*.rs"
    );
}

#[test]
fn cli_run_is_visible_in_help() {
    let out = Command::new(velin_bin())
        .arg("--help")
        .output()
        .expect("help");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("run"), "run must be visible in --help:\n{}", text);
    assert!(text.contains("8080"), "default port 8080 must appear in help:\n{}", text);
}

#[test]
fn velin_version_is_351() {
    let out = Command::new(velin_bin())
        .arg("--version")
        .output()
        .expect("version");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(text.contains("3.5.1"), "velin --version: {}", text);
}
