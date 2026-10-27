//! EC-M2-1 HTTP basic server narrow slice nail.
// was: ec_m2_1_http_server

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{run_file_printed, DriverOptions};

fn example_template() -> &'static str {
    include_str!("../../../../examples/http_server_tool/main.sy")
}

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        // SAFETY: tests run with --test-threads=1; restore in Drop.
        unsafe { std::env::set_var(key, value) };
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.prev {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("bind ephemeral")
        .local_addr()
        .expect("addr")
        .port()
}

fn wait_listening(port: u16, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return true;
        }
        thread::sleep(Duration::from_millis(20));
    }
    false
}

fn http_get_health(port: u16) -> (u16, String) {
    let mut stream =
        TcpStream::connect(("127.0.0.1", port)).expect("connect health");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .ok();
    let req = format!(
        "GET /health HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(req.as_bytes()).expect("write request");
    let mut buf = Vec::new();
    let _ = stream.read_to_end(&mut buf);
    let text = String::from_utf8_lossy(&buf).into_owned();
    let status = text
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    (status, text)
}

#[test]
fn ec_m2_1_http_server_source_uses_official_serve() {
    let example = example_template();
    assert!(
        example.contains("import Net")
            && example.contains("Http.serve")
            && example.contains("/health"),
        "example must use Http.serve + /health: {example}"
    );
    assert!(
        !example.contains("__syx_http_"),
        "example must not call bare http hosts: {example}"
    );
    let net = include_str!("../../../../packages/net/mod.sy");
    assert!(
        net.contains("func serve") && net.contains("Http"),
        "Http.serve facade required"
    );
    let host = include_str!("../../../syx-vm/src/http.rs");
    assert!(
        host.contains("SYX_HTTP_SERVE_MAX_REQUESTS"),
        "host must honor SYX_HTTP_SERVE_MAX_REQUESTS"
    );
}

#[test]
fn ec_m2_1_http_server_health_loopback() {
    let port = free_port();
    let _max_env = EnvGuard::set("SYX_HTTP_SERVE_MAX_REQUESTS", "1");

    let src = example_template().replace("18765", &port.to_string());
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/http_server_tool/.tmp_nail");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("main_{port}.sy"));
    std::fs::write(&path, src).expect("write temp example");

    let (tx, rx) = mpsc::channel();
    let path_clone = path.clone();
    let handle = thread::spawn(move || {
        let result = run_file_printed(&path_clone, &DriverOptions::default());
        let _ = tx.send(result);
    });

    assert!(
        wait_listening(port, Duration::from_secs(30)),
        "server did not listen on {port}"
    );
    let (status, body) = http_get_health(port);
    assert_eq!(status, 200, "health status: {body}");
    assert!(
        body.contains("ok"),
        "health body must contain ok: {body}"
    );

    let run_result = rx
        .recv_timeout(Duration::from_secs(30))
        .expect("serve thread result")
        .expect("run_file_printed");
    let _ = handle.join();
    let (code, printed) = run_result;
    let out = printed.join("\n");
    assert_eq!(code, 0, "serve must exit cleanly: {out}");
    assert!(out.contains("listening"), "must print listening: {out}");
    let _ = std::fs::remove_file(&path);
}
