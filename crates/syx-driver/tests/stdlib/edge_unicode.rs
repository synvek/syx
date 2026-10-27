//! Language edge: Unicode NFC/NFD / three-view / Character scalar honesty.

use std::fs;

use syx_driver::{run_native_mir_file_printed, run_wasm_source_printed, DriverOptions};

const UTF8_NEQ_NFC_EQ: &str = r#"
func main() {
    let d = __syx_str_normalized_nfd("é")
    print(d == "é")
    print(__syx_str_normalized_nfc(d) == "é")
}
"#;

const UTF8_NEQ_NFC_EQ_EXPECTED: &str = "false\ntrue";

const COMBINING_THREE_VIEW: &str = r#"
func main() {
    let s = __syx_str_normalized_nfd("é")
    print(String.count(s))
    print(__syx_str_unicode_scalar_count(s))
    print(__syx_str_utf8_count(s))
}
"#;

const COMBINING_THREE_VIEW_EXPECTED: &str = "1\n2\n3";

const IS_SINGLE_SCALAR_HONEST: &str = r#"
func main() {
    print(__syx_character_is_single_scalar("a"))
    print(__syx_character_is_single_scalar("👨‍👩‍👧"))
    print(__syx_character_is_single_scalar("ab"))
}
"#;

const IS_SINGLE_SCALAR_HONEST_EXPECTED: &str = "true\nfalse\nfalse";

const CASE_MAP_HONESTY: &str = r#"
func main() {
    print(__syx_str_lowercased("ß"))
    print(__syx_str_case_folded("ß"))
}
"#;

const CASE_MAP_HONESTY_EXPECTED: &str = "ß\nss";

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
        "syx_w286_a04_{}_{}",
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
fn edge_utf8_neq_nfc_eq_native_mir() {
    let out = run_native_mir("utf8_neq_nfc", UTF8_NEQ_NFC_EQ);
    assert_eq!(
        out, UTF8_NEQ_NFC_EQ_EXPECTED,
        "native mir UTF-8≠NFC eq must exact: {out}"
    );
}

#[test]
fn edge_utf8_neq_nfc_eq_wasm() {
    let out = run_wasm("utf8_neq_nfc", UTF8_NEQ_NFC_EQ);
    assert_eq!(
        out, UTF8_NEQ_NFC_EQ_EXPECTED,
        "wasm UTF-8≠NFC eq must exact: {out}"
    );
}

#[test]
fn edge_combining_three_view_native_mir() {
    let out = run_native_mir("combining_3view", COMBINING_THREE_VIEW);
    assert_eq!(
        out, COMBINING_THREE_VIEW_EXPECTED,
        "native mir combining three-view must exact: {out}"
    );
}

#[test]
fn edge_combining_three_view_wasm() {
    let out = run_wasm("combining_3view", COMBINING_THREE_VIEW);
    assert_eq!(
        out, COMBINING_THREE_VIEW_EXPECTED,
        "wasm combining three-view must exact: {out}"
    );
}

#[test]
fn edge_is_single_scalar_honest_native_mir() {
    let out = run_native_mir("is_single_scalar", IS_SINGLE_SCALAR_HONEST);
    assert_eq!(
        out, IS_SINGLE_SCALAR_HONEST_EXPECTED,
        "native mir isSingleScalar honesty must exact: {out}"
    );
}

#[test]
fn edge_is_single_scalar_honest_wasm() {
    let out = run_wasm("is_single_scalar", IS_SINGLE_SCALAR_HONEST);
    assert_eq!(
        out, IS_SINGLE_SCALAR_HONEST_EXPECTED,
        "wasm isSingleScalar honesty must exact: {out}"
    );
}

#[test]
fn edge_case_map_honesty_native_mir() {
    let out = run_native_mir("case_map", CASE_MAP_HONESTY);
    assert_eq!(
        out, CASE_MAP_HONESTY_EXPECTED,
        "native mir case-map honesty must exact: {out}"
    );
}

#[test]
fn edge_case_map_honesty_wasm() {
    let out = run_wasm("case_map", CASE_MAP_HONESTY);
    assert_eq!(
        out, CASE_MAP_HONESTY_EXPECTED,
        "wasm case-map honesty must exact: {out}"
    );
}

#[test]
fn edge_spec_doc_cites_adr630() {
    let spec = include_str!("../../../../docs/gates/gate-a04-edge-semantics.md");
    assert!(
        spec.contains("ADR-630") && spec.contains("Unicode"),
        "a04-edge-semantics.md must cite ADR-630 Unicode: {spec}"
    );
    assert!(
        spec.contains("UTF-8") || spec.contains("NFC"),
        "Unicode section must describe UTF-8≠NFC / NFC floor"
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
fn edge_roadmap_meta_unicode_slice() {
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
        a04.contains("W286") || a04.contains("Unicode") || a04.contains("ADR-630"),
        "A-04 row must cite W286 Unicode slice: {a04}"
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
