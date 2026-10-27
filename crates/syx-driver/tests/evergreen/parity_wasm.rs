//! WASM parity — fixtures under tests/fixtures/parity/.
// was: parity_wasm

use std::path::PathBuf;

use syx_driver::{run_wasm_source_printed, DriverOptions};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/parity/{rel}"))
}

fn run_wasm(rel: &str) -> (i32, Vec<String>) {
    let path = fixture(rel);
    run_wasm_source_printed(&path, "main", &DriverOptions::default()).expect("wasm run")
}

#[test]
fn wasm_hof() {
    let (code, _) = run_wasm("hof.sy");
    assert_eq!(code, 0);
}

#[test]
fn wasm_generics() {
    let (code, _) = run_wasm("generics.sy");
    assert_eq!(code, 0);
}

#[test]
fn wasm_array_hof() {
    let (code, lines) = run_wasm("array_hof.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "4"));
}

#[test]
fn wasm_dataclass_synth() {
    let (code, lines) = run_wasm("dataclass_synth.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.contains("true")));
}

#[test]
fn wasm_dict_hashable_key() {
    let (code, lines) = run_wasm("dict_hashable_key.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "100"));
    assert!(lines.iter().any(|l| l.trim() == "2"));
}

#[test]
fn wasm_as_is_cast() {
    let (code, lines) = run_wasm("as_is_cast.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "cat"));
}
