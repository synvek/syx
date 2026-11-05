//! EC-M2-1 Path deep contract (join / normalize / fileExtension win/unix).
// was: ec_m2_1_path_contract

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/path_tool/main.sy")
}

fn run_example() -> String {
    let (_, printed) = run_file_printed(&example_main(), &DriverOptions::default()).expect("run");
    printed.join("\n")
}

#[test]
fn ec_m2_1_path_contract_join_normalize_extension() {
    let out = run_example();
    // Wave A W89 / ADR-433 / A-03: Path join/normalize/extension product exact.
    #[cfg(unix)]
    assert_eq!(out, "a/b\na/c\nb\n\ngz", "unix path contract must exact: {out}");
    #[cfg(windows)]
    assert_eq!(out, "a\\b\na\\c\nb\n\ngz", "windows path contract must exact: {out}");
    #[cfg(not(any(unix, windows)))]
    {
        let _ = out;
        panic!("Path contract nail requires unix or windows cfg");
    }
}
