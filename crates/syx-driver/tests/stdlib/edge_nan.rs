//! Language edge: NaN/Inf Display + IEEE equality (native MIR + wasm AOT).

use std::fs;

use syx_driver::{run_native_mir_file_printed, run_wasm_source_printed, DriverOptions};

const NAN_DISPLAY: &str = r#"
func main() {
    print(__syx_f64_sqrt(-1.0))
}
"#;

const NAN_DISPLAY_EXPECTED: &str = "NaN";

const INF_DISPLAY: &str = r#"
func main() {
    print(__syx_f64_pow(10.0, 400.0))
}
"#;

const INF_DISPLAY_EXPECTED: &str = "inf";

const NEG_INF_DISPLAY: &str = r#"
func main() {
    print(__syx_f64_log(0.0))
}
"#;

const NEG_INF_DISPLAY_EXPECTED: &str = "-inf";

const NAN_IEEE_EQ: &str = r#"
func main() {
    let n = __syx_f64_sqrt(-1.0)
    print(n == n)
    print(n != n)
    print(n == 1.0)
}
"#;

const NAN_IEEE_EQ_EXPECTED: &str = "false\ntrue\nfalse";

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
        "syx_w285_a04_{}_{}",
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
fn edge_nan_display_native_mir() {
    let out = run_native_mir("nan_display", NAN_DISPLAY);
    assert_eq!(
        out, NAN_DISPLAY_EXPECTED,
        "native mir NaN display must exact: {out}"
    );
}

#[test]
fn edge_nan_display_wasm() {
    let out = run_wasm("nan_display", NAN_DISPLAY);
    assert_eq!(
        out, NAN_DISPLAY_EXPECTED,
        "wasm NaN display must exact: {out}"
    );
}

#[test]
fn edge_inf_display_native_mir() {
    let out = run_native_mir("inf_display", INF_DISPLAY);
    assert_eq!(
        out, INF_DISPLAY_EXPECTED,
        "native mir inf display must exact: {out}"
    );
}

#[test]
fn edge_inf_display_wasm() {
    let out = run_wasm("inf_display", INF_DISPLAY);
    assert_eq!(
        out, INF_DISPLAY_EXPECTED,
        "wasm inf display must exact: {out}"
    );
}

#[test]
fn edge_neg_inf_display_native_mir() {
    let out = run_native_mir("neg_inf_display", NEG_INF_DISPLAY);
    assert_eq!(
        out, NEG_INF_DISPLAY_EXPECTED,
        "native mir -inf display must exact: {out}"
    );
}

#[test]
fn edge_neg_inf_display_wasm() {
    let out = run_wasm("neg_inf_display", NEG_INF_DISPLAY);
    assert_eq!(
        out, NEG_INF_DISPLAY_EXPECTED,
        "wasm -inf display must exact: {out}"
    );
}

#[test]
fn edge_nan_ieee_eq_native_mir() {
    let out = run_native_mir("nan_ieee_eq", NAN_IEEE_EQ);
    assert_eq!(
        out, NAN_IEEE_EQ_EXPECTED,
        "native mir NaN IEEE eq must exact: {out}"
    );
}

#[test]
fn edge_nan_ieee_eq_wasm() {
    let out = run_wasm("nan_ieee_eq", NAN_IEEE_EQ);
    assert_eq!(
        out, NAN_IEEE_EQ_EXPECTED,
        "wasm NaN IEEE eq must exact: {out}"
    );
}

#[test]
fn edge_spec_doc_cites_adr629() {
    let spec = include_str!("../../../../docs/gates/gate-a04-edge-semantics.md");
    assert!(
        spec.contains("ADR-629") && spec.contains("NaN"),
        "a04-edge-semantics.md must cite ADR-629 NaN: {spec}"
    );
    assert!(
        spec.contains("IEEE") || spec.contains("=="),
        "NaN section must describe IEEE eq floor"
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
fn edge_roadmap_meta_nan_slice() {
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
        a04.contains("W285") || a04.contains("NaN") || a04.contains("ADR-629"),
        "A-04 row must cite W285 NaN slice: {a04}"
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
