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
fn ec_m2_1_path_contract_facade_and_registry() {
    let src = include_str!("../../../../packages/core/path.sy");
    for needle in ["func join", "func normalize", "func exists", "func fileExtension"] {
        assert!(
            src.contains(needle),
            "Path facade must expose {needle}: {src}"
        );
    }

    let example = include_str!("../../../../examples/path_tool/main.sy");
    assert!(
        example.contains("Path.join")
            && example.contains("Path.normalize")
            && example.contains("Path.fileExtension"),
        "example must use Path facade APIs: {example}"
    );
    assert!(
        !example.contains("__syx_path_"),
        "example must not call bare path hosts: {example}"
    );

    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("method: \"fileExtension\"") && reg.contains("HostFnId::PathExtension"),
        "registry must TypeMethod-bind fileExtension → PathExtension"
    );
    assert!(
        reg.contains("method: \"join\"")
            && reg.contains("method: \"normalize\"")
            && reg.contains("method: \"exists\""),
        "registry must keep Path join/normalize/exists"
    );
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
