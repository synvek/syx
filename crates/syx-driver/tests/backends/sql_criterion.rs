//! ADR-196 / M2 W52: EC-M2-4 Sql Criterion formal-benchmark first tier (nail).
// was: ec_m2_4_sql_criterion

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

const RUN_STACK: usize = 16 * 1024 * 1024;

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/sql_microbench/main.sy")
}

#[test]
fn ec_m2_4_sql_criterion_smoke_ok() {
    let handle = std::thread::Builder::new()
        .name("ec_m2_4_sql_criterion_smoke".into())
        .stack_size(RUN_STACK)
        .spawn(|| run_file_printed(&example_main(), &DriverOptions::default()))
        .expect("spawn smoke thread");
    let (code, printed) = handle.join().expect("join smoke thread").expect("run");
    assert_eq!(code, 0, "exit code must be 0");
    let out = printed.join("\n");
    assert!(
        out.lines().any(|l| l.trim() == "ok"),
        "must print ok: {out}"
    );
}
