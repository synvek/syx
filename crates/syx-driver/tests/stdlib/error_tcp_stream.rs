//! tcp-stream smoke hosts return Int or Err — no status strings.
// was: m2_error_tcp_stream

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W96 / ADR-440: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

/// SAFETY: nails use --test-threads=1; restore in Drop.
struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        unsafe { std::env::set_var(key, value) };
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => unsafe { std::env::set_var(self.key, v) },
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
}

#[test]
fn m2_error_tcp_stream_hosts_source_no_status_paste() {
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        !hosts.contains("\"tcp-stream-ok\"")
            && !hosts.contains("\"tcp-stream-invalid\"")
            && !hosts.contains("\"tcp-stream-disabled\"")
            && !hosts.contains("\"tcp-handle-ok\"")
            && !hosts.contains("\"tcp-handle-invalid\""),
        "host_tcp_stream* must not paste status string literals"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("HostFnId::TcpStream => \"Int\"")
            && reg.contains("HostFnId::TcpStreamTls => \"Int\"")
            && reg.contains("HostFnId::TcpStreamHandle => \"Int\""),
        "TcpStream* return types must be Int"
    );
}

#[test]
fn m2_error_tcp_stream_valid_returns_int() {
    let out = run(
        r#"
func main() {
  let n = __syx_tcp_stream(8)
  print(n)
  let t = __syx_tcp_stream_tls(8)
  print(t)
  let h = __syx_tcp_stream_handle(8)
  print(h)
}
"#,
    );
    // Wave A W87 / ADR-431 / A-03: tcp stream hosts return Int product exact.
    assert_eq!(out, "8\n8\n8", "tcp stream Int smoke must exact: {out}");
}

#[test]
fn m2_error_tcp_stream_invalid_throws() {
    // Ensure smoke path (invalid nbytes), not "disabled".
    let _mvp = EnvGuard::set("SYX_NET_STREAM_MVP", "1");
    let err = run_err(
        r#"
func main() {
  let _ = __syx_tcp_stream(0)
}
"#,
    );
    // Wave A W97 / ADR-441 / A-03: tcp stream plain invalid Display product exact.
    assert_eq!(
        err, "[R003] tcp stream smoke invalid nbytes",
        "tcp stream plain invalid must exact: {err}"
    );
}

#[test]
fn m2_error_tcp_stream_tls_invalid_throws() {
    let _mvp = EnvGuard::set("SYX_NET_STREAM_TLS_MVP", "1");
    let err = run_err(
        r#"
func main() {
  let _ = __syx_tcp_stream_tls(0)
}
"#,
    );
    // Wave A W96 / ADR-440 / A-03: tcp stream tls invalid Display product exact.
    assert_eq!(
        err, "[R003] tcp stream tls smoke invalid nbytes",
        "tcp stream tls invalid must exact: {err}"
    );
}

#[test]
fn m2_error_tcp_stream_handle_invalid_throws() {
    let _prod = EnvGuard::set("SYX_NET_STREAM_HANDLE_PROD", "1");
    let err = run_err(
        r#"
func main() {
  let _ = __syx_tcp_stream_handle(0)
}
"#,
    );
    // Wave A W97 / ADR-441 / A-03: tcp stream handle invalid Display product exact.
    assert_eq!(
        err, "[R003] tcp stream handle smoke invalid nbytes",
        "tcp stream handle invalid must exact: {err}"
    );
}
