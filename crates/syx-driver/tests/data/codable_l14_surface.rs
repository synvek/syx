//! Phase 98: Codable L14 surface names — packages API roundtrip + wire prefix L14.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_codable_l14_surface_roundtrip_embedded() {
    let out = run(r#"func main() {
    __syx_codable_l14_register_migration("name", "User")
    let wire = __syx_codable_encode_l14("alice", "User")
    print(wire.count >= 4)
    print(wire[0])
    print(wire[1])
    print(wire[2])
}"#);
    assert!(out.contains("true"), "{out}");
    let lines: Vec<_> = out.lines().collect();
    assert_eq!(
        lines.get(1).and_then(|s| s.parse::<i64>().ok()),
        Some(b'L' as i64),
        "{out}"
    );
    assert_eq!(
        lines.get(2).and_then(|s| s.parse::<i64>().ok()),
        Some(b'1' as i64),
        "{out}"
    );
    assert_eq!(
        lines.get(3).and_then(|s| s.parse::<i64>().ok()),
        Some(b'4' as i64),
        "{out}"
    );
    syx_vm::reset_phase98_state();
    let mut reg = vec![
        syx_runtime_core::Value::Str("name".into()),
        syx_runtime_core::Value::Str("User".into()),
    ];
    syx_vm::host_codable_l14_register_migration(&mut reg).unwrap();
    let mut enc = vec![
        syx_runtime_core::Value::Str("User".into()),
        syx_runtime_core::Value::Str("alice".into()),
    ];
    syx_vm::host_codable_encode_l14(&mut enc).unwrap();
    let wire = enc.pop().unwrap();
    let mut dec = vec![syx_runtime_core::Value::Str("User".into()), wire];
    syx_vm::host_codable_decode_l14(&mut dec).unwrap();
    assert_eq!(dec.pop().unwrap().as_str(), "alice");
}

#[test]
fn p98_codable_l14_surface_wire_prefix() {
    let out = run(r#"func main() {
    let wire = __syx_codable_encode_l14("x", "T")
    print(wire[0])
    print(wire[1])
    print(wire[2])
}"#);
    let lines: Vec<_> = out.lines().collect();
    assert_eq!(
        lines.first().map(|s| s.parse::<i64>().ok()),
        Some(Some(b'L' as i64)),
        "{out}"
    );
    assert_eq!(
        lines.get(1).map(|s| s.parse::<i64>().ok()),
        Some(Some(b'1' as i64)),
        "{out}"
    );
    assert_eq!(
        lines.get(2).map(|s| s.parse::<i64>().ok()),
        Some(Some(b'4' as i64)),
        "{out}"
    );
}
