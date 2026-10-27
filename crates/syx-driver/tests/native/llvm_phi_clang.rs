//! Phase 98: LLVM IR clang compile parity for parity fixtures + examples/hello.sy.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{emit_llvm_text, DriverOptions};
use syx_llvm::emit_llvm_ir;

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn clang_compile_ir(ir: &str, tag: &str) {
    let dir = std::env::temp_dir().join(format!("syx_p98_clang_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let ir_path = dir.join("out.ll");
    fs::write(&ir_path, ir).unwrap();
    let obj_path = dir.join("out.o");
    let output = Command::new("clang")
        .args(["-c", "-o"])
        .arg(&obj_path)
        .arg(&ir_path)
        .output()
        .expect("clang must be available");
    assert!(
        output.status.success(),
        "clang failed for {tag}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let _ = fs::remove_dir_all(&dir);
}

fn ir_for_path(path: &PathBuf, tag: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    emit_llvm_text(path, &opts()).unwrap_or_else(|e| panic!("emit llvm for {tag}: {e}"))
}

#[test]
fn p98_llvm_phi_clang_main_example() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.sy");
    let ir = ir_for_path(&path, "hello");
    clang_compile_ir(&ir, "hello");
}

#[test]
fn p98_llvm_phi_clang_parity_as_is_cast() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parity/as_is_cast.sy");
    let ir = ir_for_path(&path, "as_is_cast");
    clang_compile_ir(&ir, "as_is_cast");
}

#[test]
fn p98_llvm_phi_clang_parity_hof_sort() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/parity/stdlib/hof_sort_closure.sy");
    let ir = ir_for_path(&path, "hof_sort");
    clang_compile_ir(&ir, "hof_sort");
}

#[test]
fn p98_llvm_phi_clang_mir_main_declare_not_matched() {
    let ir = "declare i32 @syx_rt_run_main_fn()\n";
    assert!(!syx_llvm::llvm_ir_uses_mir_main_entry(ir));
    let ir_call = "call i32 @syx_rt_run_main_fn()\n";
    assert!(syx_llvm::llvm_ir_uses_mir_main_entry(ir_call));
}

#[test]
fn p98_llvm_phi_clang_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v5:alloca-ssa-value-ga"));
}
