//! Plugin codegen tests (SYX 40.0).
// was: plugin_codegen

use std::path::PathBuf;
use syx_host_plugin::{generate_syx_module, PluginHostManifest, PluginManifest};

#[test]
fn codegen_emits_crypto_module() {
    let manifest = PluginManifest {
        id: "mycrypto".into(),
        api_version: 1,
        library: "libmycrypto_plugin.dylib".into(),
        hosts: vec![PluginHostManifest {
            name: "__syx_plugin_mycrypto_sha256".into(),
            syx_name: Some("sha256".into()),
            module: Some("Crypto".into()),
            return_type: "String".into(),
            params: vec!["data: String".into()],
        }],
        min_syx_version: None,
        capabilities: vec![],
        signature: None,
    };
    let src = generate_syx_module(&manifest);
    assert!(src.contains("module Crypto"));
    assert!(src.contains("func sha256"));
    assert!(src.contains("__syx_plugin_mycrypto_sha256"));
}

#[test]
fn plugin_build_writes_file() {
    let demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/plugin_demo/plugins/mycrypto/plugin.json");
    if !demo.exists() {
        return;
    }
    let manifest = PluginManifest::from_path(&demo).expect("manifest");
    let dir = std::env::temp_dir().join(format!(
        "syx_plugin_codegen_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let written = syx_host_plugin::write_codegen_module(&manifest, &dir).expect("write");
    assert!(written.exists());
    let text = std::fs::read_to_string(written).expect("read");
    assert!(text.contains("Crypto"));
}
