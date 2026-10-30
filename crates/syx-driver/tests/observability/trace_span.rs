//! ADR-295 / M3 W36: M3-D02 Trace/Span thin start (explicit span + task/actor context).
// was: m3_d02_trace_span

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
fn m3_d02_trace_span_lifecycle_and_policy() {
    let src = r#"
func main() {
  print(__syx_trace_policy())
  let id = __syx_trace_start("demo")
  print(id)
  print(__syx_trace_current())
  print(__syx_trace_context_line())
  let ended = __syx_trace_end()
  print(ended)
  print(__syx_trace_current())
}
"#;
    let lines = printed(src);
    let out = lines.join("\n");
    assert!(
        out.contains("trace-v1-thin") || out.contains("trace-v1-logctx"),
        "policy fingerprint required (thin or logctx): {out}"
    );
    assert!(
        lines.len() >= 6,
        "expected lifecycle prints (incl. empty current): {lines:?}"
    );
    let span_id = lines
        .iter()
        .find(|l| l.starts_with('s') && l.chars().nth(1).is_some_and(|c| c.is_ascii_digit()))
        .cloned()
        .expect("span id");
    assert_eq!(
        lines.get(1).map(String::as_str),
        Some(span_id.as_str()),
        "start must print span id: {lines:?}"
    );
    assert_eq!(
        lines.get(2).map(String::as_str),
        Some(span_id.as_str()),
        "current must match start: {lines:?}"
    );
    let ctx = lines
        .iter()
        .find(|l| l.contains("span=") && l.contains("task_id=") && l.contains("actor_id="))
        .expect("contextLine");
    assert!(
        ctx.contains(&format!("span={span_id}")),
        "contextLine must include active span: {ctx}"
    );
    assert!(
        lines.iter().any(|l| l == "true"),
        "Trace.end must return true: {lines:?}"
    );
    assert_eq!(
        lines.last().map(String::as_str),
        Some(""),
        "current after end must be empty: {lines:?}"
    );
}

#[test]
fn m3_d02_trace_span_task_id_in_detached() {
    // Detached body cannot capture locals; print contextLine from worker TLS.
    // Avoid duplicate `let _` (S001) — discard via distinct names.
    let src = r#"
func main() {
  let t = Task.detached {
    let sid = __syx_trace_start("worker")
    print(sid)
    print(__syx_trace_context_line())
    let ended = __syx_trace_end()
    print(ended)
  }
  __syx_task_join(t)
}
"#;
    let out = run(src);
    let ctx = out
        .lines()
        .find(|l| l.contains("span=") && l.contains("task_id="))
        .expect("detached contextLine");
    assert!(
        ctx.contains("task_id="),
        "contextLine must include task_id=: {ctx}"
    );
    let task_part = ctx
        .split_whitespace()
        .find(|p| p.starts_with("task_id="))
        .expect("task_id field");
    let val = task_part.strip_prefix("task_id=").unwrap_or("-");
    assert_ne!(val, "-", "detached task must bind non-dash task_id: {ctx}");
    assert!(
        val.chars().all(|c| c.is_ascii_digit()),
        "task_id must be numeric: {ctx}"
    );
}

