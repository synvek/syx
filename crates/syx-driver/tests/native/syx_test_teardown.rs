//! JUnit-shaped hooks: `@TestAfter` per-case, `@TestTeardown` once;
//! plus single-file `syx test` path.

#[test]
fn syx_test_full_hook_order_junit_shaped() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    let marker = dir.path().join("order.txt");
    let marker_str = marker.display().to_string().replace('\\', "/");
    std::fs::write(
        tests.join("suite.sy"),
        format!(
            r#"
import IO

@TestSetup
func boot() {{
    File.write("{marker_str}", "setup\n")
}}

@TestBefore
func prep() {{
    File.append("{marker_str}", "before\n")
}}

@Test
func first() {{
    File.append("{marker_str}", "first\n")
}}

@Test
func second() {{
    File.append("{marker_str}", "second\n")
}}

@TestAfter
func clean() {{
    File.append("{marker_str}", "after\n")
}}

@TestTeardown
func done() {{
    File.append("{marker_str}", "teardown\n")
}}
"#
        ),
    )
    .unwrap();
    let report = syx_driver::test_project_with_format(
        dir.path(),
        &syx_driver::DriverOptions::default(),
        None,
        syx_driver::TestOutputFormat::Json,
    )
    .expect("json format run");
    assert_eq!(report.failed, 0, "cases={:?}", report.cases);
    assert_eq!(report.passed, 2, "cases={:?}", report.cases);
    let text = std::fs::read_to_string(&marker).expect("order file");
    assert_eq!(
        text, "setup\nbefore\nfirst\nafter\nbefore\nsecond\nafter\nteardown\n",
        "order={text:?}"
    );
}

#[test]
fn syx_test_after_failure_fails_passing_case() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("bad.sy"),
        r#"
@Test
func okish() {
}

@TestAfter
func clean() {
    throw "after-boom"
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
    assert_eq!(report.cases[0].name, "bad.okish");
    let msg = report.cases[0].failure.as_deref().unwrap_or("");
    assert!(
        msg.contains("@TestAfter"),
        "expected after failure text: {msg}"
    );
}

#[test]
fn syx_test_duplicate_teardown_is_discover_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("dup.sy"),
        r#"
@TestTeardown
func a() {}

@TestTeardown
func b() {}

@Test
func t() {}
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
    assert_eq!(report.failed, 1);
    assert_eq!(report.cases[0].name, "dup");
    let msg = report.cases[0].failure.as_deref().unwrap_or("");
    assert!(
        msg.contains("@TestTeardown"),
        "expected duplicate teardown message: {msg}"
    );
}

#[test]
fn syx_test_single_file_path_without_tests_dir() {
    let dir = tempfile::tempdir().expect("tempdir");
    let file = dir.path().join("alone.sy");
    std::fs::write(
        &file,
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
        &file,
        &syx_driver::DriverOptions::default(),
        None,
        syx_driver::TestOutputFormat::Json,
    )
    .expect("single file");
    assert_eq!(report.passed, 2, "cases={:?}", report.cases);
    assert_eq!(report.failed, 0);
    assert!(report.cases.iter().any(|c| c.name == "alone.first"));
    assert!(report.cases.iter().any(|c| c.name == "alone.second"));
}
