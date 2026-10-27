//! language-level ARC + VM GC smoke gates.
// was: memory_arc

use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, run_file_printed, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/memory/{name}"))
}

fn run_embed(path: &PathBuf) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = std::fs::read_to_string(path).unwrap();
    compile_and_run_embedded(&src, &opts()).unwrap().printed
}

#[test]
fn arc_embed_retain_release_fixture() {
    let lines = run_embed(&fixture("arc_retain_release.sy"));
    assert!(lines.iter().any(|l| l.trim() == "1"), "{lines:?}");
}

#[test]
fn arc_vm_relocating_cycle_fixture() {
    let lines = run_embed(&fixture("arc_relocating_vm_cycle.sy"));
    assert!(lines.iter().any(|l| l.trim() == "10"), "{lines:?}");
}

#[test]
fn arc_file_printed_retain_release() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_file_printed(&fixture("arc_retain_release.sy"), &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "1"), "{lines:?}");
}

#[test]
fn arc_native_mir_retain_release() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_native_mir_file_printed(&fixture("arc_retain_release.sy"), &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "1"), "{lines:?}");
}

#[test]
fn arc_wasm_retain_release() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_wasm_source_printed(&fixture("arc_retain_release.sy"), "main", &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "1"), "{lines:?}");
}

#[test]
fn arc_shared_alias_two_fields() {
    let src = r#"class Holder { var n: Int init(n: Int) { self.n = n } }
func main() {
    let a = Holder(n: 7)
    let b = a
    b.n = 9
    print(a.n)
}"#;
    syx_host_plugin::register_echo_plugin_in_process();
    let lines = compile_and_run_embedded(src, &opts()).unwrap().printed;
    assert!(lines.iter().any(|l| l.trim() == "9"), "{lines:?}");
}

#[test]
fn arc_class_init_refcount_smoke() {
    let src = r#"class C { var x: Int init() { self.x = 0 } }
func main() { print(C().x) }"#;
    syx_host_plugin::register_echo_plugin_in_process();
    let lines = compile_and_run_embedded(src, &opts()).unwrap().printed;
    assert!(lines.iter().any(|l| l.trim() == "0"), "{lines:?}");
}

#[test]
fn arc_multiple_locals_same_instance() {
    let src = r#"class P { var v: Int init(v: Int) { self.v = v } }
func main() {
    let p = P(v: 3)
    let q = p
    print(q.v)
}"#;
    syx_host_plugin::register_echo_plugin_in_process();
    let lines = compile_and_run_embedded(src, &opts()).unwrap().printed;
    assert!(lines.iter().any(|l| l.trim() == "3"), "{lines:?}");
}

#[test]
fn arc_cycle_vm_sum_bounded() {
    let lines = run_embed(&fixture("arc_relocating_vm_cycle.sy"));
    assert_eq!(lines.iter().filter(|l| l.trim() == "10").count(), 1, "{lines:?}");
}

#[test]
fn arc_mir_op_retain_release_absent() {
    // Wave A W544 / ADR-888: MirOp ObjectRefRetain/Release peeled.
    let src = include_str!("../../../syx-mir/src/lib.rs");
    assert!(!src.contains("\n    ObjectRefRetain,"));
    assert!(!src.contains("\n    ObjectRefRelease,"));
}

#[test]
fn arc_runtime_core_retain_api() {
    // Wave A W529+: Value::object_ref_retain API deleted; dual-track birth remains.
    let src = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !src.contains("fn object_ref_retain")
            && (src.contains("object_ref_from_map") || src.contains("ObjectRef")),
        "object_ref_retain API deleted; ObjectRef dual-track birth must remain"
    );
}

#[test]
fn arc_gate_count() {
    let src = include_str!("memory_arc.rs");
    let n = src.matches("#[test]").count();
    assert!(n >= 10, "memory_arc needs 10+ tests, got {n}");
}
