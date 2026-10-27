//! 02 Array/Set/Dictionary mutation semantics.
//! native/wasm behavioral nails for collections.mutation.
//! collections.dict product backends_real (reuse Dict.remove nails).
// was: m2_s1_02_collection_mutation

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed,
    DriverOptions,
};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run_native_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = std::env::temp_dir().join(format!(
        "syx_w201_{}_{}",
        label,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    std::fs::write(&path, src).unwrap();
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native exit {code}: {lines:?}");
    lines
}

fn run_wasm_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = std::env::temp_dir().join(format!(
        "syx_w201_wasm_{}_{}",
        label,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    std::fs::write(&path, src).unwrap();
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts()).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines
}

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

const DICT_REMOVE_SRC: &str = r#"
func main() {
  var d: [String: Int] = ["a": 1, "b": 2]
  print(d.remove("a"))
  print(d.containsKey("a"))
  print(d.containsKey("b"))
  print(d.count)
  print(d.remove("missing"))
  print(d.containsKey("b"))
}
"#;

const SET_MUT_SRC: &str = r#"
func main() {
  var s: Set<Int> = [1, 2]
  print(s.insert(3))
  print(s.insert(3))
  print(s.contains(3))
  print(s.count)
  print(s.remove(3))
  print(s.contains(3))
  print(s.count)
}
"#;

const ARRAY_SUB_SRC: &str = r#"
func main() {
  var a: [Int] = [1, 2, 3]
  a[1] = 9
  print(a[0])
  print(a[1])
  print(a[2])
}
"#;

const ARRAY_SET_AT_SRC: &str = r#"
func main() {
  var a: [Int] = [1, 2, 3]
  a[1] = 8
  print(a[0])
  print(a[1])
  print(a[2])
  print(__syx_array_set_at(a, 0, 7)[0])
  print(a[0])
}
"#;

#[test]
fn m2_s1_02_dict_remove_persists() {
    let out = run(DICT_REMOVE_SRC);
    // Wave A W80 / ADR-424 / A-03: Dict remove persists product exact.
    assert_eq!(
        out,
        "true\nfalse\ntrue\n1\nfalse\ntrue",
        "Dict remove must exact: {out}"
    );
}

#[test]
fn m2_s1_02_dict_remove_persists_native() {
    let lines = run_native_src("dict_rm", DICT_REMOVE_SRC);
    assert_eq!(
        lines_joined(&lines),
        "true\nfalse\ntrue\n1\nfalse\ntrue",
        "native Dict remove must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_dict_remove_persists_wasm() {
    let lines = run_wasm_src("dict_rm", DICT_REMOVE_SRC);
    assert_eq!(
        lines_joined(&lines),
        "true\nfalse\ntrue\n1\nfalse\ntrue",
        "wasm Dict remove must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_set_insert_remove_persists() {
    let out = run(SET_MUT_SRC);
    // Wave A W80 / ADR-424 / A-03: Set insert/remove persists product exact.
    assert_eq!(
        out,
        "true\nfalse\ntrue\n3\ntrue\nfalse\n2",
        "Set insert/remove must exact: {out}"
    );
}

#[test]
fn m2_s1_02_set_insert_remove_persists_native() {
    let lines = run_native_src("set_mut", SET_MUT_SRC);
    assert_eq!(
        lines_joined(&lines),
        "true\nfalse\ntrue\n3\ntrue\nfalse\n2",
        "native Set insert/remove must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_set_insert_remove_persists_wasm() {
    let lines = run_wasm_src("set_mut", SET_MUT_SRC);
    assert_eq!(
        lines_joined(&lines),
        "true\nfalse\ntrue\n3\ntrue\nfalse\n2",
        "wasm Set insert/remove must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_array_subscript_assign() {
    let out = run(ARRAY_SUB_SRC);
    // Wave A W81 / ADR-425 / A-03: Array subscript assign product exact.
    assert_eq!(out, "1\n9\n3", "Array subscript assign must exact: {out}");
}

#[test]
fn m2_s1_02_array_subscript_assign_native() {
    let lines = run_native_src("arr_sub", ARRAY_SUB_SRC);
    assert_eq!(
        lines_joined(&lines),
        "1\n9\n3",
        "native Array subscript assign must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_array_subscript_assign_wasm() {
    let lines = run_wasm_src("arr_sub", ARRAY_SUB_SRC);
    assert_eq!(
        lines_joined(&lines),
        "1\n9\n3",
        "wasm Array subscript assign must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_array_set_at_persists() {
    // Static Array.setAt monomorphizes poorly in embed; host write-back path matches subscript.
    let out = run(ARRAY_SET_AT_SRC);
    // Wave A W81 / ADR-425 / A-03: Array setAt writeback product exact.
    assert_eq!(out, "1\n8\n3\n7\n7", "Array setAt must exact: {out}");
}

#[test]
fn m2_s1_02_array_set_at_persists_native() {
    let lines = run_native_src("arr_set", ARRAY_SET_AT_SRC);
    assert_eq!(
        lines_joined(&lines),
        "1\n8\n3\n7\n7",
        "native Array setAt must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_array_set_at_persists_wasm() {
    let lines = run_wasm_src("arr_set", ARRAY_SET_AT_SRC);
    assert_eq!(
        lines_joined(&lines),
        "1\n8\n3\n7\n7",
        "wasm Array setAt must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_02_product_mutation_implemented() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "collections.mutation");
    assert!(
        block.contains("status = \"implemented\""),
        "collections.mutation must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s1_02_collection_mutation"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-224") || block.contains("M2-S1-02") || block.contains("ADR-545"),
        "notes must cite ADR-224 or ADR-545: {block}"
    );
    assert!(
        block.contains("Deque") || block.contains("M2-S1-04") || block.contains("subscript-insert"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s1_02_product_mutation_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "collections.mutation");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "collections.mutation native/wasm must be real: {block}"
    );
    assert!(
        block.contains("ADR-545") || block.contains("W201"),
        "collections.mutation notes must cite ADR-545: {block}"
    );
    assert!(
        block.contains("m2_s1_02_dict_remove_persists_native")
            && block.contains("m2_s1_02_array_subscript_assign_native"),
        "tests must list native nails: {block}"
    );
}

#[test]
fn m2_s1_02_dict_product_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "collections.dict");
    assert!(
        block.contains("native = \"real\""),
        "collections.dict must declare native=real (ADR-564): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "collections.dict must declare wasm=real (ADR-564): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "collections.dict must keep status=implemented: {block}"
    );
    assert!(
        block.contains("ADR-564"),
        "notes must cite ADR-564: {block}"
    );
    assert!(
        block.contains("m2_s1_02_dict_remove_persists_native"),
        "tests must include native mir nail: {block}"
    );
}

#[test]
fn m2_s1_02_set_surface_exposes_mutation() {
    let src = include_str!("../../../../packages/core/builtins/collections/set.sy");
    assert!(
        src.contains("__syx_set_insert") && src.contains("__syx_set_remove"),
        "set.sy must expose insert/remove hosts: {src}"
    );
}

#[test]
fn m2_s1_02_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s1_02_collection_mutation_tier"),
        "inventory must contain m2_s1_02_collection_mutation_tier"
    );
    assert!(
        inv.contains("ADR-224") || inv.contains("signed-ADR-224"),
        "inventory must note ADR-224"
    );
}

#[test]
fn m2_s1_02_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s1_02_collection_mutation"
    );
}
