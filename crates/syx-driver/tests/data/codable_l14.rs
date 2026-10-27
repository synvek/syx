//! Phase 98: Codable L14 prost schema evolution + breaking L13 gate.

use syx_runtime_core::Value;
use syx_vm::{
    host_codable_decode_l14, host_codable_encode_l14, host_codable_l14_register_migration,
    host_codable_l14_register_rename, reset_phase98_state,
};

#[test]
fn p98_codable_l14_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("codable-l14-surface-v1:hostfn-packages-l14-names"));
}

#[test]
fn p98_codable_l14_prost_roundtrip_with_rename() {
    reset_phase98_state();
    let mut reg = vec![Value::Str("name".into()), Value::Str("User".into())];
    host_codable_l14_register_migration(&mut reg).unwrap();
    let mut ren = vec![
        Value::Str("fullName".into()),
        Value::Str("name".into()),
        Value::Str("User".into()),
    ];
    host_codable_l14_register_rename(&mut ren).unwrap();
    let mut enc = vec![Value::Str("User".into()), Value::Str("alice".into())];
    host_codable_encode_l14(&mut enc).unwrap();
    let wire = enc.pop().unwrap();
    let mut dec = vec![wire, Value::Str("User".into())];
    host_codable_decode_l14(&mut dec).unwrap();
    assert_eq!(dec.pop().unwrap().as_str(), "alice");
}

#[test]
fn p98_codable_l14_breaking_l13_prefix() {
    reset_phase98_state();
    let mut enc = vec![Value::Str("T".into()), Value::Str("x".into())];
    host_codable_encode_l14(&mut enc).unwrap();
    let wire = match enc.pop().unwrap() {
        Value::Array(bytes) => bytes,
        _ => panic!("expected array wire"),
    };
    assert_eq!(wire.first().map(|v| v.as_i64()), Some(b'L' as i64));
    assert_eq!(wire.get(1).map(|v| v.as_i64()), Some(b'1' as i64));
    assert_eq!(wire.get(2).map(|v| v.as_i64()), Some(b'4' as i64));
}
