//! `@TestSetup` (once) / `@TestTeardown` (once, file-level).
//! Per-case `@TestBefore` / `@TestAfter` covered in sibling modules.

#[test]
fn syx_test_setup_then_tests_then_teardown_order() {
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
    assert_eq!(text, "setup\nfirst\nsecond\nteardown\n", "order={text:?}");
}

#[test]
fn syx_test_hooks_ignored_without_at_test() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    let marker = dir.path().join("hook.txt");
    let marker_str = marker.display().to_string().replace('\\', "/");
    std::fs::write(
        tests.join("plain.sy"),
        format!(
            r#"
import IO

@TestSetup
func boot() {{
    File.write("{marker_str}", "setup-ran\n")
}}

func main() {{
    File.write("{marker_str}", "main-ran\n")
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
    assert_eq!(report.passed, 1);
    assert_eq!(report.cases[0].name, "plain");
    let text = std::fs::read_to_string(&marker).expect("marker");
    assert_eq!(text, "main-ran\n", "setup must not run: {text:?}");
}

#[test]
fn syx_test_setup_failure_marks_tests_and_skips_teardown() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    let marker = dir.path().join("teardown.txt");
    let marker_str = marker.display().to_string().replace('\\', "/");
    std::fs::write(
        tests.join("bad.sy"),
        format!(
            r#"
import IO

@TestSetup
func boot() {{
    let x: Int = "nope"
}}

@Test
func one() {{
}}

@Test
func two() {{
}}

@TestTeardown
func done() {{
    File.write("{marker_str}", "teardown-ran\n")
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
            msg.contains("@TestSetup"),
            "expected setup failure text, got {msg:?}"
        );
    }
    assert!(
        !marker.exists(),
        "teardown must not run when setup fails"
    );
}

#[test]
fn syx_test_duplicate_setup_is_discover_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("dup.sy"),
        r#"
@TestSetup
func a() {}

@TestSetup
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
        msg.contains("@TestSetup"),
        "expected duplicate setup message: {msg}"
    );
}
