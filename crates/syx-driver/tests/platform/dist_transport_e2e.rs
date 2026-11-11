//! Dist transport prod E2E — Windows Named Pipe + rustls cross-process.

use syx_async::{
    dist_rustls_e2e_handshake_smoke, dist_rustls_e2e_prod_default_enabled,
    dist_winpipe_e2e_frame_smoke, dist_winpipe_e2e_prod_default_enabled,
};
use syx_driver::{dist_rustls_crate_e2e_prod_enabled, dist_winpipe_e2e_prod_enabled};

#[test]
fn dist_transport_e2e_winpipe_frame_stdout() {
    assert!(dist_winpipe_e2e_frame_smoke(1, 88));
}

#[test]
fn dist_transport_e2e_rustls_handshake_stdout() {
    assert!(dist_rustls_e2e_handshake_smoke(
        "sha256:phase98-p88-dist-pin"
    ));
}

#[test]
fn dist_transport_e2e_driver_prod_flags() {
    let _ = dist_rustls_crate_e2e_prod_enabled();
    let _ = dist_winpipe_e2e_prod_enabled();
    assert!(dist_rustls_e2e_prod_default_enabled() || dist_winpipe_e2e_prod_default_enabled());
}

#[test]
fn dist_transport_e2e_process_exec_cross_process_smoke() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = syx_driver::compile_and_run_embedded(
        r#"func main() {
    print(__syx_process_exec_output("/bin/echo", ["dist-p88"]))
}"#,
        &syx_driver::DriverOptions::default(),
    )
    .unwrap()
    .printed
    .join("\n");
    assert!(out.contains("dist-p88"), "{out}");
}
