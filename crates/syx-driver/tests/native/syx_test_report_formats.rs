//! `syx test` JSON + JUnit report behavioral floor.

#[test]
fn syx_test_json_format_fixture() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(tests.join("alpha.sy"), "func main() {\n}\n").unwrap();
    std::fs::write(tests.join("beta.sy"), "func main() {\n}\n").unwrap();
    let report = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        None,
        syx_driver::TestOutputFormat::Json,
    )
    .expect("json format run");
    assert_eq!(report.passed, 2);
    assert_eq!(report.failed, 0);
    assert_eq!(report.skipped, 0);
    assert_eq!(report.cases.len(), 2);
    let json = report.to_json_string().expect("serialize");
    assert!(json.contains("\"passed\":2") || json.contains("\"passed\": 2"));
    assert!(json.contains("alpha") && json.contains("beta"));
    assert!(json.contains("passed"));
    let filtered = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        Some("alpha"),
        syx_driver::TestOutputFormat::Json,
    )
    .expect("filtered json");
    assert_eq!(filtered.passed, 1);
    assert_eq!(filtered.skipped, 1);
    assert!(filtered
        .cases
        .iter()
        .any(|c| c.name == "beta" && c.status == syx_driver::TestCaseStatus::Skipped));
}

#[test]
fn syx_test_junit_format_fixture() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(tests.join("alpha.sy"), "func main() {\n}\n").unwrap();
    std::fs::write(tests.join("beta.sy"), "func main() {\n}\n").unwrap();
    let report = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        None,
        syx_driver::TestOutputFormat::Junit,
    )
    .expect("junit format run");
    assert_eq!(report.passed, 2);
    assert_eq!(report.failed, 0);
    assert_eq!(report.skipped, 0);
    let xml = report.to_junit_xml_string();
    assert!(xml.contains("<testsuite name=\"syx\""));
    assert!(xml.contains("tests=\"2\""));
    assert!(xml.contains("failures=\"0\""));
    assert!(xml.contains("passed=\"2\""));
    assert!(xml.contains("<testcase name=\"alpha\""));
    assert!(xml.contains("<testcase name=\"beta\""));
    let filtered = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        Some("alpha"),
        syx_driver::TestOutputFormat::Junit,
    )
    .expect("filtered junit");
    assert_eq!(filtered.passed, 1);
    assert_eq!(filtered.skipped, 1);
    let fxml = filtered.to_junit_xml_string();
    assert!(fxml.contains("<skipped/>"));
    assert!(fxml.contains("skipped=\"1\""));
}
