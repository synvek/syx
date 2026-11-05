//! 01 Collection/Sequence/Iterator protocol stable subset.
// was: m2_s1_01_collection_sequence_iterator

use std::path::PathBuf;

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed,
    DriverOptions,
};

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

