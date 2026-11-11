//! WASM hybrid AOT smoke tests (SYX 40.0).
// was: wasm_aot

use std::path::PathBuf;
use syx_config::SyxRuntimeConfig;
use syx_driver::{emit_wasm_bytes, DriverOptions};

use crate::runtime_config_guard::{with_aot, with_runtime_config};

fn fixture(rel: &str) -> PathBuf {
    // R-24: same hello alias as wasm_full_program_aot / smoke — real source lives
    // under `examples/hello.sy`, not `tests/fixtures/hello.sy`.
    if rel == "hello.sy" {
        return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.sy");
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel)
}

#[test]
fn wasm_emit_produces_bytes() {
    let path = fixture("hello.sy");
    let bytes = emit_wasm_bytes(&path, &DriverOptions::default()).expect("emit wasm");
    assert!(bytes.len() > 8);
    assert_eq!(&bytes[0..4], b"\0asm");
}

#[test]
fn wasm_aot_flag_changes_emit() {
    let path = fixture("hello.sy");
    let baseline = with_runtime_config(SyxRuntimeConfig::default(), || {
        emit_wasm_bytes(&path, &DriverOptions::default()).expect("baseline")
    });
    with_aot(|| {
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        let aot = emit_wasm_bytes(&path, &opts).expect("aot emit");
        assert!(!aot.is_empty());
        assert_eq!(&aot[0..4], b"\0asm");
        let _ = baseline.len();
    });
}

#[test]
fn wasm_aot_arithmetic_fixture() {
    with_aot(|| {
        let path = fixture("types/dict_hashable_key.sy");
        let bytes = emit_wasm_bytes(&path, &DriverOptions::default()).expect("emit");
        assert!(bytes.len() > 100);
    });
}

#[test]
fn wasm_aot_magic_header() {
    with_aot(|| {
        let path = fixture("hello.sy");
        let bytes = emit_wasm_bytes(&path, &DriverOptions::default()).expect("emit");
        assert_eq!(bytes[4], 1);
        assert_eq!(bytes[5], 0);
        assert_eq!(bytes[6], 0);
        assert_eq!(bytes[7], 0);
    });
}
