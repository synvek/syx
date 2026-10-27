//! ADR-195 / M2 W51: EC-M2-4 HTTP Criterion formal-benchmark first tier (nail).
// was: ec_m2_4_http_criterion

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/http_microbench/main.sy")
}

#[test]
fn ec_m2_4_http_criterion_fixture_source() {
    let src = include_str!("../../../../examples/http_microbench/main.sy");
    assert!(
        src.contains("Http.response") && src.contains("10000"),
        "fixture must Http.response × 10000: {src}"
    );
    assert!(
        !src.contains("__syx_http_"),
        "fixture must not call bare http hosts: {src}"
    );
}

#[test]
fn ec_m2_4_http_criterion_harness_wired() {
    let bench = format!(
        "{}/benches/http_response_criterion.rs",
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
        cargo.contains("http_response_criterion"),
        "Cargo.toml must declare [[bench]] http_response_criterion: {cargo}"
    );
}

#[test]
fn ec_m2_4_http_criterion_smoke_ok() {
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
fn ec_m2_4_http_criterion_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_http_criterion_tier"),
        "inventory must contain ec_m2_4_http_criterion_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-195") || inv.contains("signed-ADR-195"),
        "inventory must note ADR-195: {inv}"
    );
}

#[test]
fn ec_m2_4_http_criterion_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire nail test"
    );
    assert!(
        ci.contains("--bench http_response_criterion"),
        "ci.yml must wire short Criterion bench"
    );
}
