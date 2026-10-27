//! EC-M2-1 CLI file tool + Path narrow slice nail.
// was: ec_m2_1_file_tool

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/file_tool/main.sy")
}

fn run_example() -> String {
    let (_, printed) = run_file_printed(&example_main(), &DriverOptions::default()).expect("run");
    printed.join("\n")
}

#[test]
fn ec_m2_1_file_tool_source_uses_path_and_filehandle_throws() {
    let example = include_str!("../../../../examples/file_tool/main.sy");
    assert!(
        example.contains("Path.join") && example.contains("FileHandle.open"),
        "example must use Path.join + FileHandle.open: {example}"
    );
    assert!(
        example.contains("try") && example.contains("catch"),
        "example must try/catch IO path: {example}"
    );
    assert!(
        !example.contains("__syx_file_") && !example.contains("__syx_path_"),
        "example must not call bare file/path hosts: {example}"
    );

    let fh = include_str!("../../../../packages/io/file_handle.sy");
    assert!(
        fh.contains("IOError") && fh.contains("throws"),
        "FileHandle facade throws IOError"
    );
    let path = include_str!("../../../../packages/core/path.sy");
    assert!(
        path.contains("func join")
            && path.contains("func normalize")
            && path.contains("func exists"),
        "Path facade required: {path}"
    );
}

#[test]
fn ec_m2_1_file_tool_roundtrip_ok_exists() {
    // Wave A W72 / ADR-416 / A-03: file_tool example roundtrip product exact.
    let out = run_example();
    assert_eq!(
        out, "ok\n3\ntrue",
        "file_tool roundtrip must exact: {out}"
    );
}
