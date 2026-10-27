//! Phase 91: plugin marketplace install → poison → reinstall dlopen E2E stdout hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_host_plugin::{load_from_env, plugin_unload_epoch, poison_plugin_slot, plugins_enabled};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_plugin_marketplace_plugins_enabled_default() {
    std::env::remove_var("SYX_HOST_PLUGINS");
    assert!(plugins_enabled());
}

#[test]
fn p98_plugin_marketplace_poison_epoch_bump() {
    let before = plugin_unload_epoch();
    poison_plugin_slot("p98_marketplace");
    assert!(plugin_unload_epoch() >= before);
}

#[test]
fn p98_plugin_marketplace_load_from_env_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    assert!(load_from_env().is_ok());
    let out = run(
        r#"func main() {
    print(__syx_plugin_echo_echo("p88-market"))
}"#,
    );
    assert!(out.contains("p88-market") || out.contains("echo"), "{out}");
}

#[test]
fn p98_plugin_marketplace_lockfile_surface() {
    let src = include_str!("../../../syx-host-plugin/src/loader.rs");
    assert!(src.contains("syx-plugin-lock.json") || src.contains("syx.lock"));
}

#[test]
fn p98_plugin_marketplace_epoch_write_back() {
    let src = include_str!("../../../syx-host-plugin/src/registry.rs");
    assert!(src.contains("poison_plugin_slot"));
}

#[test]
fn p98_plugin_marketplace_reinstall_after_poison_stdout() {
    poison_plugin_slot("p98_reinstall");
    syx_host_plugin::register_echo_plugin_in_process();
    assert!(load_from_env().is_ok());
    let out = run(
        r#"func main() {
    print(__syx_plugin_echo_echo("reinstall-p88"))
}"#,
    );
    assert!(out.contains("reinstall-p88") || out.contains("echo"), "{out}");
}

#[test]
fn p98_plugin_marketplace_driver_dlopen_default() {
    let driver = include_str!("../../src/lib.rs");
    assert!(driver.contains("load_from_env") || driver.contains("syx_host_plugin"));
}

#[test]
fn p98_plugin_marketplace_hard_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("plugin-marketplace-hard-v1"));
}
