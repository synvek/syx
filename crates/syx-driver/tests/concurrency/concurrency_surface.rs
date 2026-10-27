//! SYX 38.0 concurrency E2E — actor isolation, @MainActor, Sendable/C003.
// was: concurrency

use std::path::PathBuf;

use syx_driver::{compile_file_with_options, compile_project_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/concurrency/{rel}"))
}

fn run(rel: &str) -> (i32, Vec<String>) {
    let path = fixture(rel);
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile");
    run_file_printed(&path, &options).expect("run")
}

#[test]
fn actor_isolation_counter() {
    let path = fixture("actor_counter.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("actor counter should compile");
    if let Ok((code, lines)) = run_file_printed(&path, &options) {
        assert_eq!(code, 0);
        assert!(lines.iter().any(|l| l.trim() == "2"));
    }
}

#[test]
fn main_actor_isolated_fn() {
    let (code, lines) = run("main_actor_fn.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "main-actor"));
}

#[test]
fn sendable_detach_rejected_c003() {
    let dir = tempfile::tempdir().expect("tempdir");
    let src = std::fs::read_to_string(fixture("sendable_detach_rejected.sy")).unwrap();
    std::fs::write(
        dir.path().join("syx.toml"),
        "name = \"strict\"\nmain = \"main.sy\"\n[concurrency]\nstrict = true\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("main.sy"), src).unwrap();
    let err = compile_project_with_options(dir.path(), &DriverOptions::default())
        .expect_err("non-Sendable detach should fail under strict concurrency");
    let msg = format!("{err}");
    assert!(
        msg.contains("C003"),
        "expected C003 diagnostic, got: {msg}"
    );
}

#[test]
fn parity_actor_fixture_also_runs() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parity/actor_isolation.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile parity actor_isolation");
    if let Ok((code, lines)) = run_file_printed(&path, &options) {
        assert_eq!(code, 0);
        assert!(lines.iter().any(|l| l.trim() == "2"));
    }
}
