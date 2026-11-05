//! ADR-194 / M2 W50: EC-M2-4 Path/File Criterion formal-benchmark first tier (nail).
// was: ec_m2_4_path_file_criterion

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/path_microbench/main.sy")
}

#[test]
fn ec_m2_4_path_file_criterion_smoke_ok() {
    let (code, printed) =
        run_file_printed(&example_main(), &DriverOptions::default()).expect("run");
    assert_eq!(code, 0, "exit code must be 0");
    let out = printed.join("\n");
    assert!(
        out.lines().any(|l| l.trim() == "ok"),
        "must print ok: {out}"
    );
}

