//! S1-I21: single-file check uses parse_collecting (≥2× P001 on multi_syntax fixture).

use std::path::PathBuf;

use syx_driver::{check_file_with_options, DriverOptions};

#[test]
fn s1_i21_check_file_multi_syntax_errors_reports_ge2_p001() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/multi_syntax_errors.sy");
    let err = check_file_with_options(&path, &DriverOptions::default())
        .expect_err("fixture must fail check with syntax errors");
    let p001 = err
        .iter()
        .filter(|e| e.code.as_deref() == Some("P001"))
        .count();
    assert!(
        p001 >= 2,
        "syx check / check_file_with_options must surface ≥2 P001; got {p001} in {err:?}"
    );
}
