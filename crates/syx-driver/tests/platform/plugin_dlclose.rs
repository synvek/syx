//! Phase 103: plugin unload must dlclose and allow reload.

use std::path::PathBuf;

use syx_host_plugin::{
    load_library, load_path, runtime_plugin_loaded, unload_plugin, loaded_plugin_ids,
    PluginManifest,
};

fn echo_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-host-plugin/tests/fixtures/echo_plugin")
}

fn platform_library_name() -> &'static str {
    if cfg!(target_os = "macos") {
        "libecho_plugin.dylib"
    } else if cfg!(target_os = "windows") {
        "echo_plugin.dll"
    } else {
        "libecho_plugin.so"
    }
}

fn load_echo_plugin() -> Result<(), String> {
    let dir = echo_fixture_dir();
    let json = dir.join("plugin.json");
    if json.is_file() {
        let mut manifest = PluginManifest::from_path(&json)?;
        manifest.library = platform_library_name().into();
        let lib = dir.join(&manifest.library);
        if lib.is_file() {
            return load_library(&lib, &manifest);
        }
    }
    load_path(&dir.join(platform_library_name()))
}

#[test]
fn p103_plugin_dlclose_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("plugin-dlclose-v1:unload-reload"));
    assert!(include_str!("../../src/lib.rs").contains("plugin-unload-hard-v1:fixture-required"));
}

#[test]
fn p105_plugin_unload_fixture_required() {
    let dir = echo_fixture_dir();
    let lib = dir.join(platform_library_name());
    assert!(
        lib.is_file(),
        "echo plugin dylib fixture required for CI: {}",
        lib.display()
    );
}

#[test]
fn p103_plugin_loader_no_mem_forget() {
    let src = include_str!("../../../syx-host-plugin/src/loader.rs");
    assert!(!src.contains("mem::forget(lib)"), "unload must drop Library");
    assert!(src.contains("register_loaded_library"));
}

#[test]
fn p103_plugin_macro_no_mem_forget() {
    let src = include_str!("../../../syx-host-plugin/src/macro_plugin.rs");
    assert!(!src.contains("mem::forget(lib)"), "macro plugin must retain Library for dlclose");
    assert!(src.contains("register_loaded_library"));
}

#[test]
fn p103_plugin_load_unload_reload_epoch() {
    let dir = echo_fixture_dir();
    assert!(
        dir.join(platform_library_name()).is_file(),
        "echo plugin dylib fixture required: {}",
        dir.join(platform_library_name()).display()
    );
    let host = "__syx_plugin_echo_echo";
    load_echo_plugin().expect("load echo plugin");
    assert!(runtime_plugin_loaded(host));
    assert!(loaded_plugin_ids().iter().any(|id| id == "echo"));
    unload_plugin("echo").expect("unload echo");
    assert!(!runtime_plugin_loaded(host));
    assert!(!loaded_plugin_ids().iter().any(|id| id == "echo"));
    load_echo_plugin().expect("reload echo plugin");
    assert!(runtime_plugin_loaded(host));
}
