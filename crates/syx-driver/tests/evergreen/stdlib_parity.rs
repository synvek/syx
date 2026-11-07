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
fn manifest_phase_pin() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    let phase = m["phase"].as_u64().unwrap_or(0);
    assert_eq!(phase, 139, "expected phase 139, got {phase}");
    let version = m["version"].as_str().unwrap_or("");
    // S2-I264: public version follows crate semver (CARGO_PKG_VERSION).
    assert_eq!(
        version,
        env!("CARGO_PKG_VERSION"),
        "expected manifest version {}, got {version}",
        env!("CARGO_PKG_VERSION")
    );
}
