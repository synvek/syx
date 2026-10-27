//! Language edge: i64 wrap-around floor (native MIR + wasm AOT).

use std::fs;

use syx_driver::{run_native_mir_file_printed, run_wasm_source_printed, DriverOptions};

const ADD_WRAP: &str = r#"
func main() {
    print(9223372036854775807 + 1)
}
"#;

const ADD_WRAP_EXPECTED: &str = "-9223372036854775808";

const SUB_WRAP: &str = r#"
func main() {
    print(-9223372036854775808 - 1)
}
"#;

const SUB_WRAP_EXPECTED: &str = "9223372036854775807";

const MUL_WRAP: &str = r#"
func main() {
    print(4611686018427387904 * 2)
}
"#;

const MUL_WRAP_EXPECTED: &str = "-9223372036854775808";

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
        "syx_w283_a04_{}_{}",
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
fn edge_int_add_wrap_native_mir() {
    let out = run_native_mir("add_wrap", ADD_WRAP);
    assert_eq!(
        out, ADD_WRAP_EXPECTED,
        "native mir Int add wrap must exact: {out}"
    );
}

#[test]
fn edge_int_add_wrap_wasm() {
    let out = run_wasm("add_wrap", ADD_WRAP);
    assert_eq!(
        out, ADD_WRAP_EXPECTED,
        "wasm Int add wrap must exact: {out}"
    );
}

#[test]
fn edge_int_sub_wrap_native_mir() {
    let out = run_native_mir("sub_wrap", SUB_WRAP);
    assert_eq!(
        out, SUB_WRAP_EXPECTED,
        "native mir Int sub wrap must exact: {out}"
    );
}

#[test]
fn edge_int_sub_wrap_wasm() {
    let out = run_wasm("sub_wrap", SUB_WRAP);
    assert_eq!(
        out, SUB_WRAP_EXPECTED,
        "wasm Int sub wrap must exact: {out}"
    );
}

#[test]
fn edge_int_mul_wrap_native_mir() {
    let out = run_native_mir("mul_wrap", MUL_WRAP);
    assert_eq!(
        out, MUL_WRAP_EXPECTED,
        "native mir Int mul wrap must exact: {out}"
    );
}

#[test]
fn edge_int_mul_wrap_wasm() {
    let out = run_wasm("mul_wrap", MUL_WRAP);
    assert_eq!(
        out, MUL_WRAP_EXPECTED,
        "wasm Int mul wrap must exact: {out}"
    );
}

#[test]
fn edge_spec_doc_cites_adr627() {
    let spec = include_str!("../../../../docs/gates/gate-a04-edge-semantics.md");
    assert!(
        spec.contains("ADR-627") && spec.contains("overflow"),
        "a04-edge-semantics.md must cite ADR-627 overflow: {spec}"
    );
    assert!(
        spec.contains("wrapping") || spec.contains("wrap"),
        "overflow section must describe wrapping floor"
    );
    assert!(!spec.contains("| actor ordering | deferred") && !spec.contains("actor ordering | deferred"),
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
fn edge_roadmap_meta_overflow_slice() {
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
        a04.contains("W283") || a04.contains("overflow") || a04.contains("ADR-627"),
        "A-04 row must cite W283 overflow slice: {a04}"
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
