//! S1-I24: project check aggregates parse_collecting across package .sy files.

use std::path::PathBuf;

use syx_driver::{check_project_with_options, DriverOptions};

#[test]
fn s1_i24_check_project_multi_syntax_reports_ge2_p001() {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/multi_project_syntax");
    let err = check_project_with_options(&root, &DriverOptions::default())
        .expect_err("fixture project must fail check with syntax errors");
    let p001 = err
        .iter()
        .filter(|e| e.code.as_deref() == Some("P001"))
        .count();
    assert!(
        p001 >= 2,
        "project syx check must surface ≥2 P001 across files; got {p001} in {err:?}"
    );
}
