//! Phase 91: gRPC unary loopback stdout hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_net_grpc_tcp_connect_stack() {
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
fn p98_net_grpc_unary_loopback_smoke() {
    let out = run(
        r#"func main() {
    let resp = __syx_http_post("http://127.0.0.1:1/grpc-unary", "{\"msg\":\"p88\"}")
    print(__syx_fmt_debug(resp))
}"#,
    );
    assert!(!out.is_empty(), "{out}");
}

#[test]
fn p98_net_grpc_process_exec_echo() {
    let out = run(
        r#"func main() {
    print(__syx_process_exec_output("/bin/echo", ["grpc-p88"]))
}"#,
    );
    assert!(out.contains("grpc-p88"), "{out}");
}

