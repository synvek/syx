//! ADR-296 / M3 W37: M3-D02 nested span + Log.*Fields context inject deepen.
// was: m3_d02_trace_logctx

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn printed(src: &str) -> Vec<String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
}

fn run(src: &str) -> String {
    printed(src).join("\n")
}

#[test]
fn m3_d02_trace_logctx_nested_and_policy() {
    let src = r#"
func main() {
  print(__syx_trace_policy())
  let outer = __syx_trace_start("outer")
  print(outer)
  let inner = __syx_trace_start("inner")
  print(inner)
  print(__syx_trace_current())
  print(__syx_trace_context_line())
  let e1 = __syx_trace_end()
  print(e1)
  print(__syx_trace_current())
  let e2 = __syx_trace_end()
  print(e2)
  print(__syx_trace_current())
}
"#;
    let lines = printed(src);
    let out = lines.join("\n");
    assert!(
        out.contains("trace-v1-logctx"),
        "policy fingerprint must be trace-v1-logctx: {out}"
    );
    assert!(
        lines.len() >= 9,
        "expected nested lifecycle prints: {lines:?}"
    );
    let outer = lines[1].clone();
    let inner = lines[2].clone();
    assert!(
        outer.starts_with('s') && inner.starts_with('s') && outer != inner,
        "outer/inner span ids must differ: {lines:?}"
    );
    assert_eq!(
        lines[3], inner,
        "current under nested must be inner: {lines:?}"
    );
    let ctx = lines
        .iter()
        .find(|l| l.contains("span=") && l.contains("task_id="))
        .expect("contextLine");
    assert!(
        ctx.contains(&format!("span={inner}")),
        "contextLine must show inner span: {ctx}"
    );
    // After first end: current == outer
    let after_e1 = lines
        .iter()
        .position(|l| l == "true")
        .and_then(|i| lines.get(i + 1))
        .map(String::as_str)
        .unwrap_or("");
    assert_eq!(
        after_e1, outer.as_str(),
        "current after inner end must restore outer: {lines:?}"
    );
    assert_eq!(
        lines.last().map(String::as_str),
        Some(""),
        "current after both ends must be empty: {lines:?}"
    );
}

#[test]
fn m3_d02_trace_logctx_log_fields_inject() {
    // Empty `{}` is S042 (Unknown key); use a caller field and assert auto-inject still runs.
    let src = r#"
func main() {
  let sid = __syx_trace_start("log")
  print(sid)
  let line = __syx_log_info_fields("hi", { "k": 1 })
  print(line)
  let ended = __syx_trace_end()
  print(ended)
}
"#;
    let out = run(src);
    let line = out
        .lines()
        .find(|l| l.contains("[INFO] hi"))
        .expect("infoFields line");
    assert!(
        line.contains("span=") && line.contains("task_id=") && line.contains("actor_id="),
        "infoFields must auto-inject correlation: {line}"
    );
    assert!(line.contains("k=1"), "caller field must remain: {line}");
    let sid = out
        .lines()
        .find(|l| l.starts_with('s') && l.chars().nth(1).is_some_and(|c| c.is_ascii_digit()))
        .expect("span id");
    assert!(
        line.contains(&format!("span={sid}")),
        "injected span must match active span: {line}"
    );
}

#[test]
fn m3_d02_trace_logctx_ci_inventory_and_fence() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test observability"),
        "ci.yml must wire m3_d02_trace_logctx"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m3_d02_trace_logctx = \"signed-ADR-296\""),
        "inventory must note signed-ADR-296"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-296")
            || guide.contains("m3_d02_trace_logctx")
            || guide.contains("trace-v1-logctx"),
        "UsageGuide must document M3-D02 Log context deepen"
    );
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let d02 = roadmap
        .lines()
        .find(|l| l.starts_with("| M3-D02 |"))
        .expect("M3-D02 row");
    assert!(
        d02.contains("| [~] |") || d02.contains("| [x] |"),
        "M3-D02 may be [~] or [x] after later windows: {d02}"
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
        (roadmap.contains("W39")
            && (roadmap.contains("D03")
                || roadmap.contains("metrics")
                || roadmap.contains("thin start")))
            || (roadmap.contains("W38")
                && (roadmap.contains("closeout")
                    || roadmap.contains("D02")
                    || roadmap.contains("leaf")
                    || roadmap.contains("ADR-297")))
            || (roadmap.contains("W37") && roadmap.contains("ADR-296")),
        "roadmap may advance W38 done / W39 = D03 thin start"
    );
    assert!(
        guide.contains("≠ OTel")
            || guide.contains("非 OTel")
            || d02.contains("≠ OTel"),
        "docs must honestly exclude OTel GA"
    );
}
