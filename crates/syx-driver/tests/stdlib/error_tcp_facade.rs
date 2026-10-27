//! TcpStream connect/read/write throw NetworkError; no legacy ok/status fields.
// was: m2_error_tcp_facade

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
        // Wave A W99 / ADR-443: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let tcp = include_str!("../../../../packages/net/tcp.sy");
    format!("{tcp}\n{body}")
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
fn m2_error_tcp_connect_throws_not_status_fail() {
    // Wave A W22 / ADR-366 / A-03: refused connect product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let s = TcpStream.connect(host: "127.0.0.1", port: 9)
    print("UNEXPECTED_OK")
    print(s.handle)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "refused connect must exact-catch: {out}");
}

#[test]
fn m2_error_tcp_facade_source_has_throws_not_status_fail_assign() {
    let src = include_str!("../../../../packages/net/tcp.sy");
    assert!(src.contains("throws"), "connect must be throws: missing throws");
    assert!(src.contains("NetworkError"), "NetworkError type required");
    assert!(
        !src.contains("status = \"fail\""),
        "must not assign status=fail on connect failure"
    );
    // ADR-151: legacy sentinel fields removed.
    assert!(
        !src.contains("var ok") && !src.contains("ok: Bool"),
        "legacy ok field must be removed: {src}"
    );
    assert!(
        !src.contains("var status") && !src.contains("status: String"),
        "legacy status field must be removed: {src}"
    );
    assert!(!src.contains("ok = true") && !src.contains("ok = false"), "no ok assignments");
    assert!(
        src.contains("func read(maxBytes: Int) -> String throws")
            || src.contains("-> String throws"),
        "read must throws"
    );
    assert!(
        src.contains("func write(data: String) -> Int throws") || src.contains("-> Int throws"),
        "write must throws Int"
    );
}

#[test]
fn m2_error_tcp_read_unknown_handle_throws() {
    // Facade wraps host Err as NetworkError; host Display is the stable R012 floor.
    // Ensure MVP path (unknown handle), not "disabled".
    let _mvp = EnvGuard::set("SYX_NET_TCP_MVP", "1");
    let err = run_err(
        r#"
func main() {
  let _ = __syx_tcp_read(999999, 8)
}
"#,
    );
    // Wave A W99 / ADR-443 / A-03: tcp read unknown-handle Display product exact.
    assert_eq!(
        err, "[R012] tcp read: unknown handle",
        "read unknown-handle must exact: {err}"
    );
}

#[test]
fn m2_error_tcp_write_unknown_handle_throws() {
    let _mvp = EnvGuard::set("SYX_NET_TCP_MVP", "1");
    let err = run_err(
        r#"
func main() {
  let _ = __syx_tcp_write(999999, "hi")
}
"#,
    );
    // Wave A W99 / ADR-443 / A-03: tcp write unknown-handle Display product exact.
    assert_eq!(
        err, "[R012] tcp write: unknown handle",
        "write unknown-handle must exact: {err}"
    );
}
