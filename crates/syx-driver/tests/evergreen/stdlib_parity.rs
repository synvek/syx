//! Stdlib unified MIR parity smoke.
// was: stdlib_parity

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn stdlib_string_length_host() {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = r#"
func main() {
    print(String.count("abc"))
}
"#;
    let r = compile_and_run_embedded(src, &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "3"));
}

#[test]
fn cache_tag_present_in_driver() {
    let src = include_str!("../../src/lib.rs");
    assert!(src.contains("syx-cache-v"));
}

#[test]
fn unified_mir_pipeline_enabled() {
    let src = include_str!("../../src/lib.rs");
    assert!(src.contains("use_mir_pipeline()"));
}


#[test]
fn manifest_phase_pin() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    let phase = m["phase"].as_u64().unwrap_or(0);
    assert_eq!(phase, 139, "expected phase 139, got {phase}");
    let version = m["version"].as_str().unwrap_or("");
    assert_eq!(version, "117.0", "expected manifest version 117.0, got {version}");
}
