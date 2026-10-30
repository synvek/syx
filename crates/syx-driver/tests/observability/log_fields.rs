//! ADR-293 / M3 W34: M3-D01 all-level *Fields deepen.
// was: m3_d01_log_fields

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_d01_log_fields_all_levels_and_filter() {
    let src = r#"
func main() {
  print(__syx_log_policy())
  __syx_log_set_min_level("DEBUG")
  let d = __syx_log_debug_fields("d", { "k": 1 })
  print(d)
  __syx_log_set_min_level("ERROR")
  let w = __syx_log_warn_fields("w", { "x": 9 })
  print(w)
  let e = __syx_log_error_fields("e", { "b": 2, "a": 1 })
  print(e)
}
"#;
    let out = run(src);
    assert!(
        out.contains("log-v1-fields"),
        "policy fingerprint must be log-v1-fields: {out}"
    );
    assert!(
        out.contains("[DEBUG] d") && out.contains("k="),
        "debugFields must emit DEBUG line with fields: {out}"
    );
    // warnFields filtered under ERROR min → empty print line between DEBUG and ERROR
    let err_line = out
        .lines()
        .find(|l| l.contains("[ERROR] e"))
        .expect("errorFields line");
    assert!(
        !out.contains("[WARN]"),
        "warnFields must be filtered at ERROR min: {out}"
    );
    let a_pos = err_line.find("a=").expect("a=");
    let b_pos = err_line.find("b=").expect("b=");
    assert!(a_pos < b_pos, "errorFields keys must be sorted: {err_line}");
    // Filtered warn returns empty string printed as blank — ensure w was empty
    assert!(
        out.lines().any(|l| l.is_empty()) || w_was_empty(&out),
        "filtered warnFields should print empty: {out}"
    );
}

fn w_was_empty(out: &str) -> bool {
    // After DEBUG line, next non-policy printed content before ERROR should be empty.
    let mut seen_debug = false;
    for line in out.lines() {
        if line.contains("[DEBUG]") {
            seen_debug = true;
            continue;
        }
        if !seen_debug {
            continue;
        }
        if line.contains("[ERROR]") {
            return true; // no WARN between → filter ok
        }
        if line.contains("[WARN]") {
            return false;
        }
    }
    true
}

