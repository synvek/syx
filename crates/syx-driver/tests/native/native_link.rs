//! Native link configuration smoke tests (SYX 40.0).
// was: native_link

use syx_pkg::{load_manifest, NativeConfig};
use syx_rt::LinkOptions;

#[test]
fn native_config_defaults_empty() {
    let cfg = NativeConfig::default();
    assert!(cfg.libs.is_empty());
    assert!(cfg.link_flags.is_empty());
}

#[test]
fn link_options_merge_user_libs() {
    let opts = LinkOptions {
        libs: vec!["z".into()],
        link_flags: vec!["-lm".into()],
        ..Default::default()
    };
    assert_eq!(opts.libs, vec!["z"]);
    assert_eq!(opts.link_flags, vec!["-lm"]);
}

#[test]
fn manifest_native_section_deserializes() {
    let toml = r#"
name = "link_test"
version = "0.1.0"
main = "main.sy"

[native]
libs = ["z"]
link_flags = ["-pthread"]
"#;
    let manifest: syx_pkg::SyxManifest = toml::from_str(toml).expect("parse");
    assert_eq!(manifest.native.libs, vec!["z"]);
    assert_eq!(manifest.native.link_flags, vec!["-pthread"]);
}

#[test]
fn load_manifest_with_native() {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nested_cross_module");
    let manifest = load_manifest(&root).expect("load");
    let _ = manifest.name;
}
