//! Phase 74: opcode matrix Native=yes rows must not emit `@syx_rt_exec_op` in sample IR.
// was: native_tier2_full_matrix

use std::path::PathBuf;

use syx_driver::{emit_llvm_text, DriverOptions};
use syx_llvm::llvm_ir_exec_op_count;

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn assert_fixture_zero(rel: &str) {
    let path = fixture(rel);
    let ir = emit_llvm_text(&path, &DriverOptions::default()).unwrap_or_default();
    assert!(
        llvm_ir_exec_op_count(&ir) == 0,
        "{rel} should have zero exec_op, got {}",
        llvm_ir_exec_op_count(&ir)
    );
}

#[test]
fn native_tier2_matrix_doc_no_partial() {
    let doc = include_str!("../../../../docs/wasm_native_opcode_matrix.md");
    assert!(!doc.contains("| partial |"));
    assert!(doc.contains("Heap"));
}

#[test]
fn native_tier2_heap_retain_zero() {
    assert_fixture_zero("tests/fixtures/memory/unified_heap_retain.sy");
}

#[test]
fn native_tier2_arc_retain_zero() {
    assert_fixture_zero("tests/fixtures/memory/arc_retain_release.sy");
}

#[test]
fn native_tier2_actor_cluster_zero() {
    let root = fixture("tests/fixtures/parity/actor_cluster_cross_module");
    let main = syx_driver::compile_project_with_options(&root, &DriverOptions::default())
        .unwrap()
        .manifest
        .main;
    let path = root.join(main.as_str());
    let ir = emit_llvm_text(&path, &DriverOptions::default()).unwrap_or_default();
    assert_eq!(llvm_ir_exec_op_count(&ir), 0);
}
