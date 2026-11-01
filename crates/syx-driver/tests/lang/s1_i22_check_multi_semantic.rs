//! S1-I22: single-file check uses semantic collecting (≥2 diags on multi_semantic fixture).

use std::path::PathBuf;

use syx_driver::{check_file_with_options, DriverOptions};

#[test]
fn s1_i22_check_file_multi_semantic_errors_reports_ge2() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/multi_semantic_errors.sy");
    let err = check_file_with_options(&path, &DriverOptions::default())
        .expect_err("fixture must fail check with semantic errors");
    assert!(
        err.len() >= 2,
        "syx check / check_file_with_options must surface ≥2 semantic diags; got {} in {err:?}",
        err.len()
    );
    let undef = err
        .iter()
        .filter(|e| {
            e.code.as_deref() == Some("S002")
                || e.message.contains("Undefined")
                || e.message.contains("undefined")
        })
        .count();
    assert!(
        undef >= 2,
        "expected ≥2 undefined-identifier-class diags; got {undef} in {err:?}"
    );
}

#[test]
fn s1_i22_check_file_multi_syntax_still_ge2_p001() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/multi_syntax_errors.sy");
    let err = check_file_with_options(&path, &DriverOptions::default())
        .expect_err("syntax fixture must fail");
    let p001 = err
        .iter()
        .filter(|e| e.code.as_deref() == Some("P001"))
        .count();
    assert!(p001 >= 2, "parse multi-diag must remain ≥2 P001; got {p001}");
}
