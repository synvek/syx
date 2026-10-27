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

#[test]
fn m3_d01_structured_logging_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test observability"),
        "ci.yml must wire m3_d01_structured_logging"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_d01_structured_logging = \"signed-ADR-292\""),
        "inventory must note signed-ADR-292"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-292")
            || guide.contains("m3_d01_structured_logging")
            || guide.contains("log-v1-thin"),
        "UsageGuide must document M3-D01 structured logging thin"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let d01 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-D01 |"))
        .expect("M3-D01 row");
    assert!(
        d01.contains("| [~] |") || d01.contains("| [x] |"),
        "M3-D01 may be [~] or [x] after later windows: {d01}"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-1") || roadmap.contains("- [ ] EC-M3-1"),
        "EC-M3-1 exit condition must remain documented (may be checked after ADR-318)"
    );
    assert!(
        roadmap.contains("- [x] EC-M3-4") || roadmap.contains("- [ ] EC-M3-4"),
        "EC-M3-4 exit condition must remain documented (may be checked after ADR-305)"
    );
    assert!(
        (roadmap.contains("W36")
            && (roadmap.contains("D02")
                || roadmap.contains("trace")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W35")
                && (roadmap.contains("closeout")
                    || roadmap.contains("ADR-294")
                    || roadmap.contains("D01")))
            || (roadmap.contains("W34")
                && (roadmap.contains("deepen")
                    || roadmap.contains("fields")
                    || roadmap.contains("ADR-293")
                    || roadmap.contains("D01")))
            || (roadmap.contains("W33") && roadmap.contains("ADR-292")),
        "roadmap may advance W33–W35 done / W36 = D02 thin start"
    );
}
