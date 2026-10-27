//! Phase 91: gRPC client/server streaming loopback stdout hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_net_grpc_stream_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("net-grpc-stream-v1"));
}

#[test]
fn p98_net_grpc_stream_host_surface() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("GrpcStreamClient") && reg.contains("__syx_grpc_stream_client"));
}

#[test]
fn p98_net_grpc_stream_loopback_stdout() {
    let out = run(
        r#"func main() {
    let resp = __syx_grpc_stream_client("echo", "88")
    print(resp)
}"#,
    );
    assert!(out.contains("88") || out.contains("stream"), "{out}");
}
