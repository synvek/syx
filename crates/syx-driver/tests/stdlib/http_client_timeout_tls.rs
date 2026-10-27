//! 02b HTTP client timeout + TLS bad-cert negative nails.
// was: m2_s4_02b_client_timeout_tls

use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use std::collections::HashMap;

use std::fs;

use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::ServerConfig;
use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};
use syx_runtime_core::Value;
use syx_vm::execute_http_request;

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run_err(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W95 / ADR-439: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn write_tmp_main(label: &str, src: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w282_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    main
}

fn run_wasm_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let main = write_tmp_main(label, src);
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let err = match run_wasm_source_printed(&main, "main", &opts) {
        Ok((0, lines)) => format!("UNEXPECTED_OK:{}", lines.join("\n")),
        Ok((_, lines)) if !lines.is_empty() => lines
            .into_iter()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n"),
        Ok(_) => String::new(),
        Err(e) => format!("{e}"),
    };
    let _ = fs::remove_dir_all(main.parent().expect("parent"));
    err
}

fn run_native_aot_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w342_timeout_native_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let _ = fs::remove_dir_all(&dir);
    let combined: String = [stdout.as_ref(), stderr.as_ref()]
        .join("\n")
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    if combined.starts_with("UNEXPECTED_OK") {
        return combined;
    }
    if !combined.is_empty() {
        return combined;
    }
    if !output.status.success() {
        return format!("[native exit {}]", output.status);
    }
    format!("UNEXPECTED_OK:{stdout}")
}

/// Accept TCP then sleep without writing an HTTP response (client should time out).
fn start_hang_loopback() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind hang loopback");
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
                    let mut buf = [0u8; 512];
                    let _ = stream.read(&mut buf);
                    // Hang: do not write a response (longer than client timeout_ms).
                    thread::sleep(Duration::from_millis(800));
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

/// Accept TCP, read request, write a fixed malformed HTTP payload, then close.
fn start_malformed_response_loopback(
    payload: &'static [u8],
) -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind malformed loopback");
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
                    let mut buf = [0u8; 1024];
                    let _ = stream.read(&mut buf);
                    let _ = stream.write_all(payload);
                    let _ = stream.flush();
                    // Drop stream → close after incomplete / garbage response.
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

/// Self-signed TLS acceptor on 127.0.0.1 (client rustls must reject).
fn start_self_signed_https() -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])
        .expect("rcgen self-signed");
    let key_der = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());
    let cert_der = CertificateDer::from(cert.cert);
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], PrivateKeyDer::Pkcs8(key_der))
        .expect("tls server config");
    let config = Arc::new(config);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind tls loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking tls listener");
    let handle = thread::spawn(move || {
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let mut conn = match rustls::ServerConnection::new(Arc::clone(&config)) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };
                    let mut tls = rustls::Stream::new(&mut conn, &mut stream);
                    let mut buf = [0u8; 1024];
                    let _ = tls.read(&mut buf);
                    let body = b"ok";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\nok",
                        body.len()
                    );
                    let _ = tls.write_all(resp.as_bytes());
                    let _ = tls.flush();
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
fn m2_s4_02b_timeout_hang_loopback() {
    let (port, stop, join) = start_hang_loopback();
    // Drive the same host path as Http.fetch/request options.timeout_ms
    // (embed Object subscript is Hashable-fragile; surface is still Http.fetch).
    let mut opts = HashMap::new();
    opts.insert("timeout_ms".into(), Value::I64(200));
    let url = format!("http://127.0.0.1:{port}/hang");
    let result = execute_http_request(&url, &Value::Object(opts));
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    // Wave A W94 / ADR-438 / A-03: HTTP timeout hang product exact Display.
    let err = match result {
        Ok(v) => format!("UNEXPECTED_OK:{v:?}"),
        Err(e) => format!("{e}"),
    };
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "timeout must not succeed: {err}"
    );
    assert_eq!(
        err, "[R012] HTTP request timed out",
        "timeout hang must exact: {err}"
    );
}

/// Wave A W282 / ADR-626 — timeout hang product exact on wasm AOT.
#[cfg(unix)]
#[test]
fn m2_s4_02b_timeout_hang_loopback_wasm() {
    let (port, stop, join) = start_hang_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{\"timeout_ms\":200}}")
  let _ = __syx_http_request("http://127.0.0.1:{port}/hang", opts)
}}
"#
    );
    let err = run_wasm_err("timeout_hang", &src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "wasm timeout must not succeed: {err}"
    );
    assert_eq!(
        err, "[R012] HTTP request timed out",
        "wasm timeout hang must exact: {err}"
    );
}

/// Wave A W342 / ADR-686 — timeout hang product exact on native AOT.
#[cfg(unix)]
#[test]
fn m2_s4_02b_timeout_hang_loopback_native_aot() {
    let (port, stop, join) = start_hang_loopback();
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  try {{
    let opts = __syx_json_decode("{{\"timeout_ms\":200}}")
    let _ = __syx_http_request("http://127.0.0.1:{port}/hang", opts)
    print("UNEXPECTED_OK")
  }} catch (err) {{
    print("[" + err.code + "] " + err.message)
  }}
}}
"#
    );
    let err = run_native_aot_err("timeout_hang", &src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "native AOT timeout must not succeed: {err}"
    );
    assert_eq!(
        err, "[R012] HTTP request timed out",
        "native AOT timeout hang must exact: {err}"
    );
}

#[test]
fn m2_s4_02b_http_fetch_surface_timeout_ms() {
    let net = include_str!("../../../../packages/net/mod.sy");
    assert!(
        net.contains("Http.fetch") && net.contains("__syx_http_request"),
        "Http.fetch must delegate to __syx_http_request"
    );
    let http = include_str!("../../../syx-vm/src/http.rs");
    assert!(
        http.contains("timeout_ms") && http.contains("e.is_timeout()"),
        "host must honor timeout_ms + is_timeout R012"
    );
}

#[test]
fn m2_s4_02b_bad_cert_https_fail_closed() {
    let (port, stop, join) = start_self_signed_https();
    let src = format!(
        r#"
func main() {{
  print(__syx_http_get("https://127.0.0.1:{port}/"))
}}
"#
    );
    let err = run_err(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "self-signed HTTPS must fail: {err}"
    );
    // Wave A W95 / ADR-439 / A-03: bad-cert HTTPS fail-closed product exact.
    assert_eq!(
        err, "[R003] HTTP request failed",
        "bad-cert HTTPS must exact: {err}"
    );
}

/// Wave A W130 / ADR-474 / A-03: Http connect-refused host Display exact.
#[test]
fn m2_error_http_get_connect_refused_display_throws() {
    let err = run_err(
        r#"
func main() {
  print(__syx_http_get("http://127.0.0.1:1/x"))
}
"#,
    );
    assert_eq!(
        err, "[R003] HTTP request failed",
        "Http connect-refused Display must exact: {err}"
    );
}

#[test]
fn m2_error_http_body_stream_open_opts_timeout_throws() {
    let (port, stop, join) = start_hang_loopback();
    let err = run_err(&format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{\"timeout_ms\":200}}")
  let _ = __syx_http_body_stream_open_opts("http://127.0.0.1:{port}/hang", opts)
}}
"#
    ));
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    // Wave A W126 / ADR-470 / A-03: HttpBodyStream openWithOptions timeout Display product exact.
    assert_eq!(
        err, "[R012] HTTP request timed out",
        "HttpBodyStream openWithOptions timeout must exact: {err}"
    );
}

#[test]
fn m2_error_http_body_stream_open_opts_tls_fail_throws() {
    let (port, stop, join) = start_self_signed_https();
    let err = run_err(&format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{}}")
  let _ = __syx_http_body_stream_open_opts("https://127.0.0.1:{port}/", opts)
}}
"#
    ));
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    // Wave A W126 / ADR-470 / A-03: HttpBodyStream openWithOptions TLS fail-closed Display product exact.
    assert_eq!(
        err, "[R003] HTTP request failed",
        "HttpBodyStream openWithOptions TLS fail-closed must exact: {err}"
    );
}

#[test]
fn m2_error_http_request_invalid_method_throws() {
    let err = run_err(
        r#"
func main() {
  let opts = __syx_json_decode("{\"method\":\"BAD METHOD\"}")
  let _ = __syx_http_request("http://127.0.0.1", opts)
}
"#,
    );
    // Wave A W120 / ADR-464 / A-03: Invalid HTTP method Display product exact.
    // Note: "ZZZ" is a valid extension method in reqwest; use token-invalid "BAD METHOD".
    assert_eq!(
        err, "[R003] Invalid HTTP method 'BAD METHOD'",
        "invalid HTTP method must exact: {err}"
    );
}

#[test]
fn m2_error_http_request_options_not_object_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_http_request("http://127.0.0.1", 0)
}
"#,
    );
    // Wave A W120 / ADR-464 / A-03: HTTP request options must be object Display product exact.
    assert_eq!(
        err, "[R003] HTTP request options must be an object",
        "HTTP options non-object must exact: {err}"
    );
}

/// Wave A W344 / ADR-688 — truncated HTTP response fail-closed (wire malformed floor).
#[cfg(unix)]
#[test]
fn m2_s4_a06_malformed_truncated_response_loopback() {
    let (port, stop, join) =
        start_malformed_response_loopback(b"HTTP/1.1 200 OK\r\n");
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{}}")
  let _ = __syx_http_request("http://127.0.0.1:{port}/trunc", opts)
}}
"#
    );
    let err = run_err(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "truncated response must not succeed: {err}"
    );
    assert_eq!(
        err, "[R003] HTTP request failed",
        "truncated HTTP response must exact R003: {err}"
    );
}

/// Wave A W344 / ADR-688 — bad status line fail-closed (wire malformed floor).
#[cfg(unix)]
#[test]
fn m2_s4_a06_malformed_bad_status_line_loopback() {
    let (port, stop, join) =
        start_malformed_response_loopback(b"NOT-HTTP garbage\r\n\r\n");
    thread::sleep(Duration::from_millis(20));
    let src = format!(
        r#"
func main() {{
  let opts = __syx_json_decode("{{}}")
  let _ = __syx_http_request("http://127.0.0.1:{port}/badstatus", opts)
}}
"#
    );
    let err = run_err(&src);
    stop.store(true, Ordering::Relaxed);
    let _ = join.join();
    assert!(
        !err.starts_with("UNEXPECTED_OK"),
        "bad status line must not succeed: {err}"
    );
    assert_eq!(
        err, "[R003] HTTP request failed",
        "bad status line must exact R003: {err}"
    );
}

#[test]
fn m2_s4_02b_r012_is_true_timeout_only() {
    let http = include_str!("../../../syx-vm/src/http.rs");
    assert!(
        http.contains("e.is_timeout()") && http.contains("ADR-235"),
        "R012 must be gated on is_timeout (ADR-235)"
    );
}

#[test]
fn m2_s4_02b_docs_and_capability() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-235")
            && usage.contains("timeout_ms")
            && (usage.contains("bad cert")
                || usage.contains("self-signed")
                || usage.contains("verify")),
        "UsageGuide must document timeout + TLS verify / ADR-235"
    );
    assert!(
        usage.contains("M2-S4-05") || usage.contains("EC-M2-4-Z") || usage.contains("server TLS"),
        "UsageGuide must fence S4-05 / server TLS / EC-M2-4-Z"
    );

    let net = include_str!("../../../../docs/11_NetworkStdlib.md");
    assert!(
        net.contains("ADR-235") || net.contains("timeout_ms"),
        "NetworkStdlib must mention timeout / ADR-235"
    );
    assert!(
        net.contains("verify") || net.contains("self-signed") || net.contains("fail-closed"),
        "NetworkStdlib must document HTTPS verify fail-closed"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "net.http.timeout_tls");
    assert!(
        block.contains("status = \"implemented\""),
        "net.http.timeout_tls must be implemented: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "net.http.timeout_tls must declare wasm=real (ADR-626): {block}"
    );
    assert!(
        block.contains("native = \"real\""),
        "net.http.timeout_tls must declare native=real (ADR-687): {block}"
    );
    assert!(
        block.contains("ADR-626") && block.contains("W282"),
        "notes must cite ADR-626+W282: {block}"
    );
    assert!(
        block.contains("ADR-687") && block.contains("W343"),
        "notes must cite ADR-687+W343 native raise: {block}"
    );
    assert!(
        block.contains("ADR-235") || block.contains("M2-S4-02b"),
        "notes must cite ADR-235: {block}"
    );
    assert!(
        block.contains("m2_s4_02b_client_timeout_tls")
            && block.contains("m2_s4_02b_timeout_hang_loopback_wasm")
            && block.contains("m2_s4_02b_timeout_hang_loopback_native_aot"),
        "tests must list VM+wasm+native AOT nails: {block}"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("signed-ADR-235") || inv.contains("ADR-235"),
        "inventory must note ADR-235"
    );

    let tls_server = product_block(caps, "net.tls.server");
    assert!(
        tls_server.contains("status = \"implemented\"") && tls_server.contains("ADR-779"),
        "net.tls.server product status raise is W435/ADR-779 (M2 S4-02b deferred server): {tls_server}"
    );
}

#[test]
fn m2_s4_02b_fence_no_s4_05_or_m2_exit() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M2-S4-02b") && roadmap.contains("ADR-235"),
        "roadmap must list M2-S4-02b / ADR-235"
    );
}
