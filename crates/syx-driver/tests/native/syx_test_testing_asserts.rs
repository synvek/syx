//! `@Test` + explicit `import Testing` assertEq pass/fail.

#[test]
fn syx_test_assert_eq_mismatch_fails_case() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("bad.sy"),
        r#"
import Testing

@Test
func boom() {
    assertEq(1, 2)
}
"#,
    )
    .unwrap();
    let report = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        None,
        syx_driver::TestOutputFormat::Json,
    )
    .expect("json");
    assert_eq!(report.failed, 1, "cases={:?}", report.cases);
    assert_eq!(report.passed, 0);
    let case = report
        .cases
        .iter()
        .find(|c| c.name == "bad.boom")
        .expect("case");
    assert_eq!(case.status, syx_driver::TestCaseStatus::Failed);
    let msg = case.failure.as_deref().unwrap_or("");
    assert!(!msg.is_empty(), "failure text must be recorded");
}

#[test]
fn syx_test_assert_eq_match_passes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("ok.sy"),
        r#"
import Testing

@Test
func good() {
    assertEq(2, 2)
    assert(true)
}
"#,
    )
    .unwrap();
    let report = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        None,
        syx_driver::TestOutputFormat::Json,
    )
    .expect("json");
    assert_eq!(report.failed, 0, "cases={:?}", report.cases);
    assert_eq!(report.passed, 1);
    assert_eq!(report.cases[0].name, "ok.good");
}
