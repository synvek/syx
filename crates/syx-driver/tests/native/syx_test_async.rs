//! `@Test async` + await → JSON pass (≠ auto-import).

#[test]
fn syx_test_async_await_assert_eq_passes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(
        tests.join("async_ok.sy"),
        r#"
import Testing

async func inner() {
    __syx_task_sleep(0)
}

@Test
async func good() {
    await inner()
    assert(true)
    assertEq(1, 1)
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
    assert_eq!(report.cases[0].name, "async_ok.good");
    assert_eq!(report.cases[0].status, syx_driver::TestCaseStatus::Passed);
}
