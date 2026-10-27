//! ADR-196 / M2 W52: EC-M2-4 Sql Criterion formal-benchmark first tier (nail).
// was: ec_m2_4_sql_criterion

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

const RUN_STACK: usize = 16 * 1024 * 1024;

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/sql_microbench/main.sy")
}

#[test]
fn ec_m2_4_sql_criterion_fixture_source() {
    let src = include_str!("../../../../examples/sql_microbench/main.sy");
    assert!(
        src.contains("Sql.execOn") && src.contains(":memory:") && src.contains("10000"),
        "fixture must Sql.execOn × 10000 on :memory:: {src}"
    );
    assert!(
        !src.contains("__syx_sql_"),
        "fixture must not call bare sql hosts: {src}"
    );
    assert!(
        !src.contains("while "),
        "fixture must not use while (while+Sql control-flow broken): {src}"
    );
}

#[test]
fn ec_m2_4_sql_criterion_harness_wired() {
    let bench = format!(
        "{}/benches/sql_exec_criterion.rs",
        env!("CARGO_MANIFEST_DIR")
    );
    assert!(
        std::path::Path::new(&bench).is_file(),
        "missing Criterion bench harness: {bench}"
    );
    let cargo = include_str!("../../Cargo.toml");
    assert!(
        cargo.contains("criterion"),
        "Cargo.toml must list criterion: {cargo}"
    );
    assert!(
        cargo.contains("sql_exec_criterion"),
        "Cargo.toml must declare [[bench]] sql_exec_criterion: {cargo}"
    );
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

#[test]
fn ec_m2_4_sql_criterion_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_sql_criterion_tier"),
        "inventory must contain ec_m2_4_sql_criterion_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-196") || inv.contains("signed-ADR-196"),
        "inventory must note ADR-196: {inv}"
    );
}

#[test]
fn ec_m2_4_sql_criterion_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire nail test"
    );
    assert!(
        ci.contains("--bench sql_exec_criterion"),
        "ci.yml must wire short Criterion bench"
    );
}
