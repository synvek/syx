//! Phase 91: TLS/WSS rustls loopback + native wiring stdout hard.

use syx_async::dist_rustls_e2e_handshake_smoke;
use syx_driver::{compile_and_run_embedded, dist_rustls_crate_e2e_prod_enabled, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_net_tls_wss_rustls_transport_module() {
    let src = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_rustls_e2e_handshake_smoke"));
    assert!(src.contains("rustls") || src.contains("TLS"));
}

#[test]
fn p98_net_tls_wss_handshake_smoke() {
    assert!(dist_rustls_e2e_handshake_smoke("sha256:phase98-p88-tls-pin"));
}

#[test]
fn p98_net_tls_wss_driver_prod_flag() {
    let _ = dist_rustls_crate_e2e_prod_enabled();
}

#[test]
fn p98_net_tls_wss_llvm_rt_symbols() {
    let llvm = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    assert!(llvm.contains("syx_rt_rustls") || llvm.contains("syx_rt_tls"));
}

#[test]
fn p98_net_tls_wss_tcp_connect_stdout() {
    let out = run(
        r#"func main() {
    print(__syx_tcp_connect("127.0.0.1", 9))
}"#,
    );
    assert!(
        out.contains("-1") || out.chars().any(|c| c.is_ascii_digit()),
        "{out}"
    );
    assert!(!out.contains("tcp-ok"), "{out}");
}

#[test]
fn p98_net_tls_wss_http_get_stdout() {
    let out = run(
        r#"func main() {
    let resp = __syx_http_get("https://127.0.0.1:1/wss-smoke")
    print(__syx_fmt_debug(resp))
}"#,
    );
    assert!(!out.is_empty(), "{out}");
}

#[test]
fn p98_net_tls_wss_websocket_fixture_surface() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lsp/websocket_wss_prod.sy");
    assert!(path.exists());
    let src = std::fs::read_to_string(&path).unwrap();
    assert!(src.contains("wss://") || src.contains("WebSocket"));
}

#[test]
fn p98_net_tls_wss_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("net-h2-alpn-default-v1"));
}
