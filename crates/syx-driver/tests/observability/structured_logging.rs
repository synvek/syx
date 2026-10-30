//! ADR-292 / M3 W33: M3-D01 structured logging thin start (min-level + infoFields).
// was: m3_d01_structured_logging

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn m3_d01_structured_logging_policy_filter_and_fields() {
    let src = r#"
func main() {
  print(__syx_log_policy())
  print(__syx_log_min_level())
  let ok = __syx_log_set_min_level("WARN")
  print(ok)
  print(__syx_log_min_level())
  __syx_log_debug("x")
  print(__syx_log_take_last())
  __syx_log_error("e")
  print(__syx_log_take_last())
  let _ = __syx_log_set_min_level("DEBUG")
  let line = __syx_log_info_fields("hi", { "b": 2, "a": 1 })
  print(line)
}
"#;
    let out = run(src);
    assert!(
        out.contains("log-v1-thin") || out.contains("log-v1-fields"),
        "policy fingerprint required (thin or fields): {out}"
    );
    // Default min is DEBUG before set.
    assert!(
        out.lines().any(|l| l == "DEBUG") || out.contains("DEBUG"),
        "default or set min-level must surface DEBUG: {out}"
    );
    assert!(out.contains("true"), "setMinLevel(WARN) must succeed: {out}");
    assert!(out.contains("WARN"), "minLevel after set must be WARN: {out}");
    // After filtered debug, takeLast prints empty line; after error, [ERROR] e.
    assert!(
        out.contains("[ERROR] e"),
        "error above WARN must emit via takeLast: {out}"
    );
    assert!(
        out.contains("[INFO] hi"),
        "infoFields must emit INFO line: {out}"
    );
    let fields_line = out
        .lines()
        .find(|l| l.contains("[INFO] hi"))
        .expect("infoFields line");
    let a_pos = fields_line.find("a=").expect("a= key");
    let b_pos = fields_line.find("b=").expect("b= key");
    assert!(
        a_pos < b_pos,
        "infoFields keys must be sorted: {fields_line}"
    );
}

