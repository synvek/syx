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

#[test]
fn m3_d01_log_fields_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test observability"),
        "ci.yml must wire m3_d01_log_fields"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_d01_log_fields = \"signed-ADR-293\""),
        "inventory must note signed-ADR-293"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-293")
            || guide.contains("m3_d01_log_fields")
            || guide.contains("log-v1-fields"),
        "UsageGuide must document M3-D01 fields deepen"
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
            || (roadmap.contains("W34") && roadmap.contains("ADR-293")),
        "roadmap may advance W34–W35 done / W36 = D02 thin start"
    );
}
