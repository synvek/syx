//! `@TestBefore` per-case hook.

#[test]
fn syx_test_before_runs_between_setup_and_each_test() {
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
        text, "setup\nbefore\nfirst\nbefore\nsecond\nteardown\n",
        "order={text:?}"
    );
}

#[test]
fn syx_test_before_failure_skips_body_continues_next() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    let marker = dir.path().join("side.txt");
    let marker_str = marker.display().to_string().replace('\\', "/");
    std::fs::write(
        tests.join("bad.sy"),
        format!(
            r#"
import IO

@TestBefore
func prep() {{
    let x: Int = "nope"
}}

@Test
func one() {{
    File.write("{marker_str}", "one-ran\n")
}}

@Test
func two() {{
    File.append("{marker_str}", "two-ran\n")
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
    .expect("json continues on fail");
    assert_eq!(report.failed, 2, "cases={:?}", report.cases);
    assert_eq!(report.passed, 0);
    for c in &report.cases {
        assert_eq!(c.status, syx_driver::TestCaseStatus::Failed);
        let msg = c.failure.as_deref().unwrap_or("");
        assert!(
            msg.contains("@TestBefore"),
            "expected before failure text, got {msg:?}"
        );
    }
    assert!(
        !marker.exists(),
        "test bodies must not run when before fails"
    );
}

#[test]
fn syx_test_duplicate_before_is_discover_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("dup.sy"),
        r#"
@TestBefore
func a() {}

@TestBefore
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
        msg.contains("@TestBefore"),
        "expected duplicate before message: {msg}"
    );
}
