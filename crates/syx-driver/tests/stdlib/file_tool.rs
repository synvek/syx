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
fn ec_m2_1_file_tool_roundtrip_ok_exists() {
    // Wave A W72 / ADR-416 / A-03: file_tool example roundtrip product exact.
    let out = run_example();
    assert_eq!(out, "ok\n3\ntrue", "file_tool roundtrip must exact: {out}");
}
