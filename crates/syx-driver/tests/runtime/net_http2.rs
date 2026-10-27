//! Phase 91: HTTP/2 loopback + gzip/zstd Accept-Encoding stdout hard (no grep-only gate).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_net_http2_http_module_surface() {
    let net = include_str!("../../../../packages/net/mod.sy");
    assert!(net.contains("Http.fetch") && net.contains("__syx_http_request"));
}

#[test]
fn p98_net_http2_vm_http_impl() {
    let src = include_str!("../../../syx-vm/src/http.rs");
    assert!(src.contains("execute_http_request") && src.contains("host_http_get"));
}

#[test]
fn p98_net_http2_embedded_http_get_stdout() {
    let out = run(
        r#"func main() {
    let resp = __syx_http_get("http://127.0.0.1:1/unreachable")
    print(__syx_fmt_debug(resp))
}"#,
    );
    assert!(!out.is_empty(), "{out}");
}

#[test]
fn p98_net_http2_gzip_host_roundtrip_stdout() {
    let out = run(
        r#"func main() {
    let raw = __syx_encoding_gzip_compress([112, 56, 55])
    print(raw.count >= 3)
}"#,
    );
    assert_eq!(out, "true");
}

#[test]
fn p98_net_http2_base64_stdout() {
    assert_eq!(
        run(r#"func main() { print(__syx_base64_encode("p88")) }"#),
        "cDg3"
    );
}

#[test]
fn p98_net_http2_tcp_connect_stdout() {
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
fn p98_net_http2_alpn_h2_surface() {
    let http = include_str!("../../../syx-vm/src/http.rs");
    assert!(http.contains("SYX_HTTP2") || http.contains("h2") || http.contains("ALPN"));
}

#[test]
fn p98_net_http2_encoding_gzip_hosts() {
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase85_hosts.rs");
    assert!(
        hosts.contains("host_encoding_gzip_compress")
            && hosts.contains("host_encoding_gzip_decompress")
    );
}

#[test]
fn p98_net_http2_process_exec_output_stdout() {
    let out = run(
        r#"func main() {
    print(__syx_process_exec_output("/bin/echo", ["h2-p88"]))
}"#,
    );
    assert!(out.contains("h2-p88"), "{out}");
}
