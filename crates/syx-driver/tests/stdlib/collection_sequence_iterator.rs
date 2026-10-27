//! 01 Collection/Sequence/Iterator protocol stable subset.
// was: m2_s1_01_collection_sequence_iterator

use std::path::PathBuf;

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed,
    DriverOptions,
};

fn mir_op_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing MirOp override {name}"));
    rest.split("[[op]]").next().unwrap_or(rest)
}

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

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn run_native_fixture(rel: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native exit {code}: {lines:?}");
    lines
}

fn run_wasm_fixture(rel: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts()).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines
}

const ARRAY_FOR_IN_SRC: &str = r#"
func main() {
    var s: Int = 0
    for x in [1, 2, 3] {
        s = s + x
    }
    print(s)
}
"#;

#[test]
fn m2_s1_01_custom_iterator_for_in() {
    let out = run(
        r#"
class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? {
        self.n = self.n + 1
        if self.n > 3 { return nil }
        return self.n
    }
}

func main() {
    var s: Int = 0
    for x in Counter() {
        s = s + x
    }
    print(s)
}
"#,
    );
    assert_eq!(out.trim(), "6", "custom Iterator for-in must sum 1+2+3: {out}");
}

#[test]
fn m2_s1_01_array_for_in() {
    let out = run(
        r#"
func main() {
    var s: Int = 0
    for x in [1, 2, 3] {
        s = s + x
    }
    print(s)
}
"#,
    );
    assert_eq!(out.trim(), "6", "Array for-in must sum 1+2+3: {out}");
}

#[test]
fn m2_s1_01_array_for_in_native() {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = std::env::temp_dir().join(format!("syx_w200_ar_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), ARRAY_FOR_IN_SRC).unwrap();
    let (code, lines) =
        run_native_mir_file_printed(&dir.join("main.sy"), &opts()).expect("native array for-in");
    assert_eq!(code, 0);
    assert!(
        lines.iter().any(|l| l.trim() == "6"),
        "native array for-in must sum 1+2+3: {lines:?}"
    );
}

#[test]
fn m2_s1_01_custom_iterator_for_in_native() {
    let lines = run_native_fixture("parity/protocol_iterator_custom.sy");
    assert!(
        lines.iter().any(|l| l.trim() == "6"),
        "native custom Iterator for-in must sum 1+2+3: {lines:?}"
    );
}

#[test]
fn m2_s1_01_custom_iterator_for_in_wasm() {
    let lines = run_wasm_fixture("parity/protocol_iterator_custom.sy");
    assert!(
        lines.iter().any(|l| l.trim() == "6"),
        "wasm custom Iterator for-in must sum 1+2+3: {lines:?}"
    );
}

#[test]
fn m2_s1_01_mir_op_make_iterator() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "MakeIterator");
    assert!(
        block.contains("ADR-543") || block.contains("ADR-542") || block.contains("W198"),
        "MakeIterator notes must cite ADR-543: {block}"
    );
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "MakeIterator product backends must be real: {block}"
    );
    assert!(
        !block.contains("for-in still open"),
        "MakeIterator notes must not contain stale for-in debt: {block}"
    );
}

#[test]
fn m2_s1_01_mir_op_iterator_next() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "IteratorNext");
    assert!(
        block.contains("ADR-543") || block.contains("ADR-542") || block.contains("W198") || block.contains("ADR-223"),
        "IteratorNext notes must cite ADR-543 / ADR-223: {block}"
    );
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "IteratorNext product backends must be real: {block}"
    );
}

#[test]
fn m2_s1_01_mir_op_array_get_inventory_real() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "ArrayGet");
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "ArrayGet product backends must be real: {block}"
    );
    assert!(
        !block.contains("for-in still open") && !block.contains("W198+"),
        "ArrayGet notes must not contain stale for-in debt: {block}"
    );
    assert!(
        block.contains("ADR-543") || block.contains("W199"),
        "ArrayGet notes must cite ADR-543: {block}"
    );
}

#[test]
fn m2_s1_01_mir_op_object_get_inventory_real() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "ObjectGet");
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "ObjectGet product backends must be real: {block}"
    );
    assert!(
        !block.contains("for-in still open") && !block.contains("W198+"),
        "ObjectGet notes must not contain stale for-in debt: {block}"
    );
    assert!(
        block.contains("ADR-543") || block.contains("W199"),
        "ObjectGet notes must cite ADR-543: {block}"
    );
}

#[test]
fn m2_s1_01_product_protocol_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "collections.protocol");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "collections.protocol must declare native+wasm=real: {block}"
    );
    assert!(
        block.contains("ADR-544") || block.contains("W200"),
        "collections.protocol notes must cite ADR-544: {block}"
    );
    assert!(
        block.contains("suite1_array_for_in_differential")
            && block.contains("iter_native_mir_custom_counter_fixture")
            && block.contains("iter_wasm_custom_counter_fixture"),
        "tests must wire differential + native/wasm nails: {block}"
    );
}

#[test]
fn m2_s1_01_product_protocol_implemented() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "collections.protocol");
    assert!(
        block.contains("status = \"implemented\""),
        "collections.protocol must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s1_01_collection_sequence_iterator"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-223") || block.contains("M2-S1-01"),
        "notes must cite ADR-223: {block}"
    );
    assert!(
        block.contains("makeIterator") || block.contains("Collection") || block.contains("M2-S1-02"),
        "notes must keep Sequence/Collection / mutation honesty fence: {block}"
    );
}

#[test]
fn m2_s1_01_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s1_01_collection_sequence_iterator_tier"),
        "inventory must contain m2_s1_01_collection_sequence_iterator_tier"
    );
    assert!(
        inv.contains("ADR-223") || inv.contains("signed-ADR-223"),
        "inventory must note ADR-223"
    );
}

#[test]
fn m2_s1_01_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s1_01_collection_sequence_iterator"
    );
}
