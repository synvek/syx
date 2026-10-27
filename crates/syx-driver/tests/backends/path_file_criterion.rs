//! ADR-194 / M2 W50: EC-M2-4 Path/File Criterion formal-benchmark first tier (nail).
// was: ec_m2_4_path_file_criterion

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/path_microbench/main.sy")
}

#[test]
fn ec_m2_4_path_file_criterion_fixture_source() {
    let src = include_str!("../../../../examples/path_microbench/main.sy");
    assert!(
        src.contains("Path.join") && src.contains("10000"),
        "fixture must Path.join × 10000: {src}"
    );
    assert!(
        !src.contains("__syx_path_"),
        "fixture must not call bare path hosts: {src}"
    );
}

#[test]
fn ec_m2_4_path_file_criterion_harness_wired() {
    let bench = format!(
        "{}/benches/path_join_criterion.rs",
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
        cargo.contains("path_join_criterion"),
        "Cargo.toml must declare [[bench]] path_join_criterion: {cargo}"
    );
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

#[test]
fn ec_m2_4_path_file_criterion_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_criterion_tier"),
        "inventory must contain ec_m2_4_path_file_criterion_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-194") || inv.contains("signed-ADR-194"),
        "inventory must note ADR-194: {inv}"
    );
}

#[test]
fn ec_m2_4_path_file_criterion_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire nail test"
    );
    assert!(
        ci.contains("--bench path_join_criterion"),
        "ci.yml must wire short Criterion bench"
    );
}
