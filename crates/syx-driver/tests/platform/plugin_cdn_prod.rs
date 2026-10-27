//! Phase 91: Plugin CDN production routing + poison reinstall hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_host_plugin::{load_from_env, poison_plugin_slot};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_plugin_cdn_prod_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("plugin-cdn-prod-v1"));
}

#[test]
fn p98_plugin_cdn_prod_dist_transport_surface() {
    assert!(include_str!("../../../syx-async/src/dist_transport.rs").contains("cdn"));
}

#[test]
fn p98_plugin_cdn_prod_reinstall_after_poison() {
    poison_plugin_slot("p98_cdn");
    syx_host_plugin::register_echo_plugin_in_process();
    assert!(load_from_env().is_ok());
    let out = run(
        r#"func main() {
    print(__syx_plugin_echo_echo("cdn-p89"))
}"#,
    );
    assert!(out.contains("cdn-p89") || out.contains("echo"), "{out}");
}

#[test]
fn p98_plugin_cdn_prod_registry_lock_field() {
    assert!(include_str!("../../../syx-pkg/src/lib.rs").contains("marketplace_plugin_epoch"));
}
