//! R-34: Array<Shape> protocol witness dispatch (file-path).

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

#[test]
fn protocol_shape_array_total_and_describe() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/protocol_shape_array.sy");
    compile_file_with_options(&path, &opts()).expect("compile Array<Shape>");
    let (code, lines) = run_file_printed(&path, &opts()).expect("run");
    assert_eq!(code, 0, "exit: {lines:?}");
    let t: Vec<_> = lines.iter().map(|l| l.trim()).collect();
    assert!(
        t.iter().any(|l| *l == "circle"),
        "expected describe circle, got {t:?}"
    );
    assert!(
        t.iter().any(|l| *l == "square"),
        "expected describe square, got {t:?}"
    );
    // 3.14159*4 + 9 = 21.56636 → *100 → 2156.636 (print Double; Int(Double) is "Int {}")
    assert!(
        t.iter().any(|l| l.starts_with("2156")),
        "expected total*100≈2156.x (not silent 0), got {t:?}"
    );
    assert!(
        !t.iter().any(|l| *l == "0" && t.last() == Some(l)),
        "total must not silently be 0, got {t:?}"
    );
}

#[test]
fn protocol_shape_array_nonconform_s014() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("lang/protocol_shape_array_bad.sy");
    match compile_file_with_options(&path, &opts()) {
        Err(e) => {
            assert_eq!(e.code.as_deref(), Some("S014"), "expected S014, got {e:?}");
            assert!(
                e.message.contains("Shape")
                    || e.message.contains("Box")
                    || e.message.contains("conform")
                    || e.message.to_lowercase().contains("protocol"),
                "S014 should mention Shape/Box/conform: {}",
                e.message
            );
        }
        Ok(_) => panic!("expected S014 for Box under Array<Shape>"),
    }
}
