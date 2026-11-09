//! ADR-185 / M2 W41: EC-M2-4 Path/File micro-benchmark first tier (FilePath.join×N Instant).
// was: ec_m2_4_path_file_microbench

use std::path::PathBuf;
use std::time::Instant;

use syx_driver::{run_file_printed, DriverOptions};

const N: u32 = 10_000;

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/path_microbench/main.sy")
}

#[test]
fn ec_m2_4_path_file_microbench_join_instant() {
    let t0 = Instant::now();
    let (code, printed) =
        run_file_printed(&example_main(), &DriverOptions::default()).expect("run");
    let elapsed = t0.elapsed();

    assert_eq!(code, 0, "exit code must be 0");
    let out = printed.join("\n");
    assert!(
        out.lines().any(|l| l.trim() == "ok"),
        "must print ok: {out}"
    );
    assert!(
        elapsed.as_nanos() > 0,
        "elapsed must be > 0 (got {elapsed:?})"
    );

    let per_op = elapsed / N;
    eprintln!(
        "ec_m2_4_path_file_microbench: FilePath.join×{N} compile+run in {elapsed:?} (~{per_op:?}/op)"
    );
}

