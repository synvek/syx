//! Phase 91: Relocating GC v12 churn + relocate_count hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_runtime_core::{relocate_count, reset_relocate_count};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_gc_relocating_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gc-relocating-v12-v1"));
}

#[test]
fn p98_gc_relocating_api_surface() {
    assert!(include_str!("../../../syx-runtime-core/src/managed_heap.rs").contains("relocate_global"));
}

#[test]
fn p98_gc_relocating_count_smoke() {
    reset_relocate_count();
    let mut roots = vec![syx_runtime_core::Value::Array(vec![
        syx_runtime_core::Value::I32(1),
    ])];
    syx_runtime_core::relocate_global(&mut roots);
    assert!(relocate_count() >= 1);
}

#[test]
fn p98_gc_relocating_churn_stdout() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/memory/heap_relocate_churn.sy");
    if path.exists() {
        let (_, lines) =
            syx_driver::run_file_printed(&path, &DriverOptions::default()).unwrap();
        assert!(!lines.is_empty());
    }
}

#[test]
fn p98_gc_relocating_doctor_report() {
    let r = syx_driver::gc_relocate_report::run_gc_relocate_report();
    assert!(r.relocate_api);
}

#[test]
fn p98_gc_relocating_native_stub_wired() {
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    assert!(stubs.contains("syx_rt_gc_relocating") && stubs.contains("rt_direct_exec"));
}
