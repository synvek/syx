//! unified managed heap retain/release parity (VM / MIR / WASM / Native).
// was: memory_unified_heap

use std::path::PathBuf;

use syx_driver::{
    compile_and_run_embedded, emit_llvm_text, run_file_printed, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

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
fn unified_heap_managed_heap_enabled() {
    let src = include_str!("../../../syx-runtime-core/src/managed_heap.rs");
    assert!(src.contains("SYX_UNIFIED_HEAP"));
}

#[test]
fn unified_heap_vm_retain_fixture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_file_printed(&fixture("unified_heap_retain.sy"), &opts()).unwrap();
    assert_eq!(lines.iter().filter(|l| l.trim() == "42").count(), 2, "{lines:?}");
}

#[test]
fn unified_heap_embed_retain_fixture() {
    let lines = run_embed(&fixture("unified_heap_retain.sy"));
    assert!(lines.iter().any(|l| l.trim() == "42"), "{lines:?}");
}

#[test]
fn unified_heap_native_mir_retain_fixture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) =
        run_native_mir_file_printed(&fixture("unified_heap_retain.sy"), &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "42"), "{lines:?}");
}

#[test]
fn unified_heap_wasm_retain_fixture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) =
        run_wasm_source_printed(&fixture("unified_heap_retain.sy"), "main", &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "42"), "{lines:?}");
}

#[test]
fn unified_heap_llvm_retain_direct_rt() {
    let ir = emit_llvm_text(&fixture("unified_heap_retain.sy"), &opts()).unwrap();
    if ir.contains("ObjectRefRetain") || ir.contains("object_ref_retain") {
        assert!(
            ir.contains("syx_rt_object_ref_retain"),
            "expected direct retain RT call"
        );
    }
}

#[test]
fn unified_heap_arc_regression() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_file_printed(&fixture("arc_retain_release.sy"), &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "1"), "{lines:?}");
}

#[test]
fn unified_heap_collect_global_smoke() {
    let src = include_str!("../../../syx-runtime-core/src/managed_heap.rs");
    assert!(src.contains("collect_global"));
}

#[test]
fn unified_heap_actor_roots_fixture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("heap_actor_roots.sy");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "7"), "{lines:?}");
}

#[test]
fn unified_heap_mir_run_slots() {
    let src = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(src.contains("heap_slots"));
}
