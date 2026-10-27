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
fn p99_codable_l14_surface_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("codable-l14-only-v1:no-l9-surface"));
}

#[test]
fn p98_codable_l14_surface_packages_api_names() {
    let src = include_str!("../../../../packages/data/codable.sy");
    assert!(src.contains("__syx_codable_encode_l14"));
    assert!(src.contains("__syx_codable_decode_l14"));
    assert!(src.contains("__syx_codable_l14_register_migration"));
}

#[test]
fn p98_codable_l14_surface_host_registry() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("__syx_codable_encode_l14"));
    assert!(reg.contains("__syx_codable_decode_l14"));
    assert!(reg.contains("__syx_codable_l14_register_migration"));
}

#[test]
fn p98_codable_l14_surface_roundtrip_embedded() {
    let out = run(
        r#"func main() {
    __syx_codable_l14_register_migration("name", "User")
    let wire = __syx_codable_encode_l14("alice", "User")
    print(wire.count >= 4)
    print(wire[0])
    print(wire[1])
    print(wire[2])
}"#,
    );
    assert!(out.contains("true"), "{out}");
    let lines: Vec<_> = out.lines().collect();
    assert_eq!(lines.get(1).and_then(|s| s.parse::<i64>().ok()), Some(b'L' as i64), "{out}");
    assert_eq!(lines.get(2).and_then(|s| s.parse::<i64>().ok()), Some(b'1' as i64), "{out}");
    assert_eq!(lines.get(3).and_then(|s| s.parse::<i64>().ok()), Some(b'4' as i64), "{out}");
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
    let mut dec = vec![
        syx_runtime_core::Value::Str("User".into()),
        wire,
    ];
    syx_vm::host_codable_decode_l14(&mut dec).unwrap();
    assert_eq!(dec.pop().unwrap().as_str(), "alice");
}

#[test]
fn p98_codable_l14_surface_wire_prefix() {
    let out = run(
        r#"func main() {
    let wire = __syx_codable_encode_l14("x", "T")
    print(wire[0])
    print(wire[1])
    print(wire[2])
}"#,
    );
    let lines: Vec<_> = out.lines().collect();
    assert_eq!(lines.first().map(|s| s.parse::<i64>().ok()), Some(Some(b'L' as i64)), "{out}");
    assert_eq!(lines.get(1).map(|s| s.parse::<i64>().ok()), Some(Some(b'1' as i64)), "{out}");
    assert_eq!(lines.get(2).map(|s| s.parse::<i64>().ok()), Some(Some(b'4' as i64)), "{out}");
}

#[test]
fn p99_codable_l9_surface_removed() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(!reg.contains("__syx_codable_encode_l9"));
    assert!(!reg.contains("__syx_codable_decode_l9"));
    assert!(!reg.contains("__syx_codable_l9_register_migration"));
    assert!(!reg.contains("CodableEncodeL9"));
}

#[test]
fn p101_codable_hostfn_prune_v2_no_l5_l13_enum() {
    assert!(include_str!("../../src/lib.rs").contains("codable-hostfn-prune-v2:no-l5-l13-enum"));
    let names: Vec<&str> = syx_stdlib::HostFnId::all()
        .iter()
        .map(|id| id.name())
        .collect();
    for slot in [
        "__syx_codable_encode_l5",
        "__syx_codable_decode_l5",
        "__syx_codable_encode_l6",
        "__syx_codable_decode_l6",
        "__syx_codable_encode_l7",
        "__syx_codable_decode_l7",
        "__syx_codable_l7_register_evolution",
        "__syx_codable_encode_l8",
        "__syx_codable_decode_l8",
        "__syx_codable_l8_register_migration",
        "__syx_codable_encode_l9",
        "__syx_codable_decode_l9",
        "__syx_codable_encode_l10",
        "__syx_codable_encode_l11",
        "__syx_codable_encode_l12",
        "__syx_codable_encode_l13",
    ] {
        assert!(
            !names.contains(&slot),
            "HostFnId::all() must not contain pruned slot {slot}"
        );
    }
    assert!(names.contains(&"__syx_codable_encode_l14"));
}

#[test]
fn p102_codable_host_body_prune_no_l5_l13_impl() {
    assert!(include_str!("../../src/lib.rs").contains("codable-host-body-prune-v1:no-l5-l13-impl"));
    let vm = concat!(
        include_str!("../../../syx-vm/src/lib.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase88_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase90_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase91_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase92_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase93_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase94_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase95_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase96_hosts.rs"),
        include_str!("../../../syx-vm/src/stdlib_phase97_hosts.rs"),
    );
    for slot in 5..=13 {
        assert!(
            !vm.contains(&format!("host_codable_encode_l{slot}")),
            "active syx-vm must not define host_codable_encode_l{slot}"
        );
        assert!(
            !vm.contains(&format!("host_codable_decode_l{slot}")),
            "active syx-vm must not define host_codable_decode_l{slot}"
        );
    }
}

#[test]
fn p100_codable_l14_host_prune_no_l5_l13_public() {
    assert!(include_str!("../../src/lib.rs").contains("codable-l14-host-prune-v1:no-l5-l13-public"));
    let packages = include_str!("../../../../packages/data/codable.sy");
    for name in [
        "__syx_codable_encode_l5",
        "__syx_codable_encode_l6",
        "__syx_codable_encode_l7",
        "__syx_codable_encode_l8",
        "__syx_codable_encode_l9",
        "__syx_codable_encode_l10",
        "__syx_codable_encode_l11",
        "__syx_codable_encode_l12",
        "__syx_codable_encode_l13",
    ] {
        assert!(
            !packages.contains(name),
            "packages/data/codable.sy must not expose {name}"
        );
    }
    assert!(
        packages.contains("l14") || packages.contains("L14") || packages.contains("encode_l14"),
        "expected L14 surface in codable.sy"
    );
}
