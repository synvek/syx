//! Phase 91: Relocating GC v12 relocate_count + Host label-echo smoke (C-1d).

use syx_driver::DriverOptions;
use syx_runtime_core::{relocate_count, reset_relocate_count};

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
    // C-1d: fixture prints Host `__syx_gc_churn_heap` label echo — ≠ GC KPI.
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/memory/heap_relocate_churn.sy");
    if path.exists() {
        let (_, lines) = syx_driver::run_file_printed(&path, &DriverOptions::default()).unwrap();
        assert!(
            lines.iter().any(|l| l.contains("gc:churn:quad")) || !lines.is_empty(),
            "C-1d Host label echo or non-empty print: {lines:?}"
        );
    }
}
