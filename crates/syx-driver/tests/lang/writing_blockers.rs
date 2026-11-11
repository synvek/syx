//! R-35: writing blockers — implicit self compound assign / closure mut capture / async let S020.

use syx_driver::{
    check_file_with_options, compile_file_with_options, run_file_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

#[test]
fn compound_assign_implicit_self_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/compound_assign_implicit_self.sy");
    compile_file_with_options(&path, &opts()).expect("compile compound assign self");
    let (code, lines) = run_file_printed(&path, &opts()).expect("run");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t: Vec<_> = lines.iter().map(|l| l.trim()).collect();
    assert!(
        t.iter().any(|l| *l == "2"),
        "expected count==2 after bump+= twice, got {t:?}"
    );
}

#[test]
fn closure_mut_capture_updates_outer() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/closure_mut_capture.sy");
    compile_file_with_options(&path, &opts()).expect("compile closure mut capture");
    let (code, lines) = run_file_printed(&path, &opts()).expect("run");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t: Vec<_> = lines.iter().map(|l| l.trim()).collect();
    assert!(
        t.iter().any(|l| *l == "2"),
        "expected outer c==2 after two inc(), got {t:?}"
    );
}

#[test]
fn async_let_no_false_s020() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/async_let_no_false_s020.sy");
    match check_file_with_options(&path, &opts()) {
        Ok(_) => {}
        Err(errs) => {
            assert!(
                !errs.iter().any(|e| e.code.as_deref() == Some("S020")),
                "async let must not false-positive S020: {errs:?}"
            );
            panic!("unexpected check error: {errs:?}");
        }
    }
    compile_file_with_options(&path, &opts()).expect("compile async let");
    let (code, lines) = run_file_printed(&path, &opts()).expect("run");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t: Vec<_> = lines.iter().map(|l| l.trim()).collect();
    assert!(t.iter().any(|l| *l == "3"), "expected 1+2==3, got {t:?}");
}
