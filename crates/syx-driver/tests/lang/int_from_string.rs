//! R-32: Int(String) → Int? (parse success / failure).

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn compile_run(rel: &str) -> Result<(i32, Vec<String>), syx_diagnostics::SyxDiagnostic> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    compile_file_with_options(&path, &opts())?;
    run_file_printed(&path, &opts())
}

#[test]
fn int_from_string_success_print_and_add() {
    let (code, lines) = compile_run("lang/int_from_string.sy").expect("compile/run");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t: Vec<_> = lines.iter().map(|l| l.trim()).collect();
    assert!(
        !t.iter().any(|l| l.contains("Int {}")),
        "must not print Int {{}}, got {t:?}"
    );
    assert!(
        t.windows(3).any(|w| w == ["42", "43", "nil"]),
        "expected 42, 43, nil; got {t:?}"
    );
}

#[test]
fn int_from_string_failure_add_fail_closed() {
    let err = compile_run("lang/int_from_string_fail_add.sy");
    match err {
        Err(e) => {
            let msg = e.message.clone();
            assert!(
                e.code.as_deref() == Some("R003")
                    || msg.contains("R003")
                    || msg.to_lowercase().contains("nil")
                    || msg.to_lowercase().contains("numeric"),
                "expected fail-closed on Nil+1, got {e:?}"
            );
        }
        Ok((code, lines)) => {
            let t: Vec<_> = lines.iter().map(|l| l.trim()).collect();
            assert!(
                !t.iter().any(|l| *l == "1"),
                "Nil+1 must not silently print 1, got {t:?}"
            );
            panic!("expected runtime error for Int(\"abc\")+1, got exit={code} printed={t:?}");
        }
    }
}
