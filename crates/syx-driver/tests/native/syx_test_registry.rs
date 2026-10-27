//! `Testing.test` closures run when a file has no `@Test`.

#[test]
fn syx_test_registry_cases_pass_and_fail() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("reg.sy"),
        r#"
import Testing

func main() {
    test(name: "ok", fn: { () -> Void in assert(true) })
    test(name: "bad", fn: { () -> Void in assertEq(1, 2) })
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
    assert_eq!(report.passed, 1, "cases={:?}", report.cases);
    assert_eq!(report.failed, 1, "cases={:?}", report.cases);
    let ok = report
        .cases
        .iter()
        .find(|c| c.name == "reg.ok")
        .expect("ok case");
    assert_eq!(ok.status, syx_driver::TestCaseStatus::Passed);
    let bad = report
        .cases
        .iter()
        .find(|c| c.name == "reg.bad")
        .expect("bad case");
    assert_eq!(bad.status, syx_driver::TestCaseStatus::Failed);
    assert!(!bad.failure.as_deref().unwrap_or("").is_empty());
}
