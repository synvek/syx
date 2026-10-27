//! EC-M2-1 HTTP client narrow slice nail.
// was: ec_m2_1_http_client

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use syx_driver::{run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/http_tool/main.sy")
}

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn remove(key: &'static str) -> Self {
        let prev = std::env::var(key).ok();
        // SAFETY: tests run with --test-threads=1; restore in Drop.
        unsafe { std::env::remove_var(key) };
        Self { key, prev }
    }

    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        // SAFETY: tests run with --test-threads=1; restore in Drop.
        unsafe { std::env::set_var(key, value) };
        Self {
            key,
            prev,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        // SAFETY: tests run with --test-threads=1.
        unsafe {
            match &self.prev {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

/// Ephemeral HTTP/1.1 `/health` → 200 + body `ok` on 127.0.0.1.
fn start_health_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let handle = thread::spawn(move || {
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let mut buf = [0u8; 2048];
                    let _ = stream.read(&mut buf);
                    let body = b"ok";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\nok",
                        body.len()
                    );
                    let _ = stream.write_all(resp.as_bytes());
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });
    (port, stop, handle)
}

#[test]
fn ec_m2_1_http_client_source_uses_official_http() {
    let example = include_str!("../../../../examples/http_tool/main.sy");
    assert!(
        example.contains("import Net") && example.contains("Http.get"),
        "example must use import Net + Http.get: {example}"
    );
    assert!(
        example.contains("refused") && example.contains("SYX_EC_M2_1_HTTP_BASE"),
        "example must cover refuse + env base: {example}"
    );
    assert!(
        !example.contains("__syx_http_"),
        "example must not call bare http hosts: {example}"
    );
    let net = include_str!("../../../../packages/net/mod.sy");
    assert!(
        net.contains("public class Http") && net.contains("func get"),
        "Http facade required: {net}"
    );
}

#[test]
fn ec_m2_1_http_client_refuse_catchable() {
    // Wave A W65 / ADR-409 / A-03: Http.get refuse product exact (example unchanged).
    let _clear = EnvGuard::remove("SYX_EC_M2_1_HTTP_BASE");
    let (code, printed) =
        run_file_printed(&example_main(), &DriverOptions::default()).expect("run");
    let out = printed.join("\n");
    assert_eq!(code, 0, "refuse path should catch, not abort: {out}");
    assert_eq!(out, "refused", "http_tool refuse must exact: {out}");
}

#[test]
fn ec_m2_1_http_client_loopback_ok() {
    // Wave A W67 / ADR-411 / A-03: Http.get loopback-ok product exact (example unchanged).
    let (port, stop, handle) = start_health_loopback();
    let base = format!("http://127.0.0.1:{port}");
    let _env = EnvGuard::set("SYX_EC_M2_1_HTTP_BASE", &base);
    // Brief settle for listener thread.
    thread::sleep(Duration::from_millis(20));
    let (code, printed) =
        run_file_printed(&example_main(), &DriverOptions::default()).expect("run");
    stop.store(true, Ordering::Relaxed);
    let _ = handle.join();
    let out = printed.join("\n");
    assert_eq!(code, 0, "loopback get must succeed: {out}");
    assert_eq!(out, "refused\nok\n200\nok", "http_tool loopback-ok must exact: {out}");
}
