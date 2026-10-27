//! Language edge: deinit ordering + nil sentinel (native + wasm).

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions,
};

const HEAP_DEINIT_ORDER: &str =
    include_str!("../fixtures/memory/heap_deinit_order.sy");

const DEINIT_EXPECTED: &str = "deinit:\n1\ndone\ndeinit:\n2";

const NIL_PRINT: &str = r#"
func main() {
  print(nil)
}
"#;

const DEQUE_EMPTY_POP_NIL: &str = r#"
func main() {
  var d = Deque.new()
  print(Deque.isEmpty(items: d))
  print(Deque.popFront(items: d))
  print(Deque.popBack(items: d))
  print(Deque.count(items: d))
}
"#;

const TASK_LOCAL_MISSING: &str = r#"
func main() {
  print(__syx_task_local_get("missing"))
  return 0
}
"#;

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w277_a04_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join("main.sy");
    fs::write(&path, src).expect("write main");
    path
}

fn run_native_mir(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &DriverOptions::default()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
    lines_joined(&lines)
}

fn run_native_strict(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w277_a04_strict_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write manifest");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    assert_eq!(
        output.status.code().unwrap_or(1),
        0,
        "native strict exit: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim_end().to_string()
}

fn run_wasm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let path = write_src(label, src);
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines_joined(&lines)
}

#[test]
fn edge_deinit_scope_exit_and_arc_overwrite_native_strict() {
    let out = run_native_strict("deinit", HEAP_DEINIT_ORDER);
    assert_eq!(
        out, DEINIT_EXPECTED,
        "native strict deinit order must exact: {out}"
    );
}

#[test]
fn edge_deinit_scope_exit_and_arc_overwrite_wasm() {
    let out = run_wasm("deinit", HEAP_DEINIT_ORDER);
    assert_eq!(out, DEINIT_EXPECTED, "wasm deinit order must exact: {out}");
}

#[test]
fn edge_const_nil_print_native() {
    let out = run_native_mir("nil_print", NIL_PRINT);
    assert_eq!(out, "nil", "native ConstNil print must exact: {out}");
}

#[test]
fn edge_const_nil_print_wasm() {
    let out = run_wasm("nil_print", NIL_PRINT);
    assert_eq!(out, "nil", "wasm ConstNil print must exact: {out}");
}

#[test]
fn edge_deque_empty_pop_nil_native() {
    let out = run_native_mir("deque_nil", DEQUE_EMPTY_POP_NIL);
    assert_eq!(
        out, "true\nnil\nnil\n0",
        "native Deque empty pop nil must exact: {out}"
    );
}

#[test]
fn edge_deque_empty_pop_nil_wasm() {
    let out = run_wasm("deque_nil", DEQUE_EMPTY_POP_NIL);
    assert_eq!(
        out, "true\nnil\nnil\n0",
        "wasm Deque empty pop nil must exact: {out}"
    );
}

#[test]
fn edge_task_local_missing_nil_wasm() {
    let out = run_wasm("task_local_nil", TASK_LOCAL_MISSING);
    assert!(
        out.trim() == "nil" || out.trim() == "Nil",
        "TaskLocalGet missing must print nil: {out}"
    );
}

#[test]
fn edge_spec_doc_cites_adr621() {
    let spec = include_str!("../../../../docs/gates/gate-a04-edge-semantics.md");
    assert!(
        spec.contains("ADR-621") && spec.contains("deinit") && spec.contains("nil"),
        "a04-edge-semantics.md must cite ADR-621 deinit+nil: {spec}"
    );
    assert!(
        !spec.contains("| actor ordering | deferred")
            && !spec.contains("actor ordering | deferred"),
        "spec must not leave actor deferred"
    );
    assert!(
        spec.contains("ADR-632") && spec.contains("Q-L4 floor"),
        "spec must cite ADR-632 Q-L4 floor signed"
    );
    assert!(
        spec.contains("Not** Wave A")
            || spec.contains("**Not** Wave A")
            || spec.contains("Not Wave A"),
        "spec must remain Not Wave A exit"
    );
}

#[test]
fn edge_roadmap_meta_thin_charter() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    let a04 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-04 |"))
        .expect("A-04 row");
    assert!(
        a04.contains("| [x] |"),
        "A-04 must be checked after W288 exit charter: {a04}"
    );
    assert!(
        a04.contains("W277") || a04.contains("deinit") || a04.contains("ADR-621"),
        "A-04 row must cite W277 thin charter: {a04}"
    );
    // F-02 slim SSOT dropped `- [x] Q-L4` checklist lines; Q-L4 remains as A-04 source tag.
    assert!(
        roadmap.lines().any(|l| l.starts_with("- [x] Q-L4"))
            || (a04.contains("Q-L4") && a04.contains("| [x] |")),
        "Q-L4 must remain checked (checklist or A-04 [x] source tag)"
    );
    assert!(
        !roadmap.lines().any(|l| l.starts_with("- [ ] Q-L4")),
        "unchecked Q-L4 checklist must not remain"
    );
}
