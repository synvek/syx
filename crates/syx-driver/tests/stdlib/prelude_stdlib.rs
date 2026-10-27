//! SYX 38.0 stdlib E2E — IO binary paths use `Array<UInt8>`.
// was: stdlib (top-level)

use std::path::PathBuf;

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/stdlib/{rel}"))
}

fn run(rel: &str) -> (i32, Vec<String>) {
    let path = fixture(rel);
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile");
    run_file_printed(&path, &options).expect("run")
}

#[test]
fn io_read_write_bytes_roundtrip() {
    let (code, lines) = run("io_bytes_roundtrip.sy");
    assert_eq!(code, 0);
    // Wave A W89 / ADR-433 / A-03: IO bytes roundtrip product exact.
    let out = lines.join("\n");
    assert_eq!(out, "65\n66\n67", "io bytes roundtrip must exact: {out}");
}

#[test]
fn file_handle_read_bytes() {
    let (code, lines) = run("file_handle_bytes.sy");
    assert_eq!(code, 0);
    // Wave A W90 / ADR-434 / A-03: FileHandle read bytes product exact.
    let out = lines.join("\n");
    assert_eq!(out, "10\n20\n30", "file handle bytes must exact: {out}");
}

#[test]
fn manifest_io_uses_typed_array() {
    let io = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packages/io/mod.sy"),
    )
    .unwrap();
    assert!(io.contains("Array<UInt8>"));
    let handle = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packages/io/file_handle.sy"),
    )
    .unwrap();
    assert!(handle.contains("Array<UInt8>"));
}
