//! Recursive `tests/**/*.sy` discovery (case names use relative paths).

#[test]
fn syx_test_discovers_nested_tests_dir() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(tests.join("nested")).unwrap();
    std::fs::write(
        tests.join("top.sy"),
        r#"
@Test
func alpha() {
}
"#,
    )
    .unwrap();
    std::fs::write(
        tests.join("nested").join("deep.sy"),
        r#"
@Test
func beta() {
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
    assert_eq!(report.passed, 2, "cases={:?}", report.cases);
    assert!(
        report.cases.iter().any(|c| c.name == "top.alpha"),
        "missing top.alpha: {:?}",
        report.cases
    );
    assert!(
        report.cases.iter().any(|c| c.name == "nested/deep.beta"),
        "missing nested/deep.beta: {:?}",
        report.cases
    );
}
