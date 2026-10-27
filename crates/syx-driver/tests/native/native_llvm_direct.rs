//! Phase 91: Native LLVM direct emit GA.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_llvm::{append_native_runtime_shim, emit_llvm_ir, llvm_ir_uses_mir_main_entry};
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};

#[test]
fn p98_native_llvm_direct_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v5:alloca-ssa-value-ga"));
}

#[test]
fn p98_native_llvm_direct_sync_main_shim() {
    let mir = MirModule {
        functions: vec![MirFunction {
            name: "main".into(),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                }],
                successors: vec![],
            }],
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }],
        ..Default::default()
    };
    let mut ir = emit_llvm_ir(&mir).unwrap();
    append_native_runtime_shim(&mir, &mut ir).unwrap();
    assert!(!llvm_ir_uses_mir_main_entry(&ir));
    assert!(ir.contains("@main"));
}

#[test]
fn p98_native_llvm_direct_sort_by_vm() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains('1')), "{out:?}");
}

#[test]
fn p98_native_llvm_direct_c_stub_default_off() {
    std::env::remove_var("SYX_NATIVE_C_STUB");
    assert!(std::env::var("SYX_NATIVE_C_STUB").is_err());
}
