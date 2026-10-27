//! `@Test` discovery: each top-level zero-arg `@Test` fn is a case;
//! files without `@Test` still run as a whole-file entry.

#[test]
fn syx_test_two_at_test_fns_are_separate_cases() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("suite.sy"),
        r#"
@Test
func first() {
}

@Test
func second() {
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
    .expect("json format run");
    assert_eq!(report.passed, 2, "cases={:?}", report.cases);
    assert_eq!(report.failed, 0);
    assert_eq!(report.cases.len(), 2);
    assert!(report.cases.iter().any(|c| c.name == "suite.first"));
    assert!(report.cases.iter().any(|c| c.name == "suite.second"));
}

#[test]
fn syx_test_file_without_at_test_stays_whole_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(tests.join("plain.sy"), "func main() {\n}\n").unwrap();
    let report = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        None,
        syx_driver::TestOutputFormat::Json,
    )
    .expect("json format run");
    assert_eq!(report.passed, 1);
    assert_eq!(report.cases.len(), 1);
    assert_eq!(report.cases[0].name, "plain");
}

#[test]
fn syx_test_failure_text_in_json_and_junit() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("bad.sy"),
        r#"
@Test
func boom() {
    let x: Int = "nope"
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
    .expect("json continues on fail");
    assert_eq!(report.failed, 1, "cases={:?}", report.cases);
    let case = report.cases.iter().find(|c| c.name == "bad.boom").expect("case");
    assert_eq!(case.status, syx_driver::TestCaseStatus::Failed);
    let msg = case.failure.as_deref().unwrap_or("");
    assert!(
        !msg.is_empty(),
        "failure text must be recorded, got empty"
    );
    let json = report.to_json_string().expect("serialize");
    assert!(json.contains("failure") || json.contains(msg.split_whitespace().next().unwrap_or("")));
    let xml = report.to_junit_xml_string();
    assert!(xml.contains("<failure>"));
    assert!(!xml.contains("<failure/>"));
}
