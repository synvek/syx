//! Phase 91: sync `@main` LLVM IR must not reference MIR main runner.

use syx_llvm::{emit_llvm_ir, llvm_ir_uses_mir_main_entry, mir_function_needs_mir_entry};
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};

fn sync_main_module() -> MirModule {
    MirModule {
        functions: vec![MirFunction {
            name: "main".into(),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstI32(42),
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                successors: vec![],
            }],
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }],
        ..Default::default()
    }
}

fn async_main_module() -> MirModule {
    MirModule {
        functions: vec![MirFunction {
            name: "main".into(),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![MirInstr {
                    dest: None,
                    op: MirOp::AwaitPoll,
                    span: None,
                }],
                successors: vec![],
            }],
            instructions: vec![MirInstr {
                dest: None,
                op: MirOp::AwaitPoll,
                span: None,
            }],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }],
        ..Default::default()
    }
}

#[test]
fn p98_sync_main_ir_zero_mir_entry() {
    let mir = sync_main_module();
    let main_fn = mir.functions.first().unwrap();
    assert!(!mir_function_needs_mir_entry(main_fn));
    let mut ir = emit_llvm_ir(&mir).unwrap();
    syx_llvm::append_native_runtime_shim(&mir, &mut ir).unwrap();
    assert!(
        !llvm_ir_uses_mir_main_entry(&ir),
        "sync main IR must not call syx_rt_run_main_fn"
    );
}

#[test]
fn p98_async_main_allows_mir_entry() {
    let mir = async_main_module();
    let main_fn = mir.functions.first().unwrap();
    assert!(mir_function_needs_mir_entry(main_fn));
    let mut ir = emit_llvm_ir(&mir).unwrap();
    syx_llvm::append_native_runtime_shim(&mir, &mut ir).unwrap();
    assert!(llvm_ir_uses_mir_main_entry(&ir));
}

#[test]
fn p98_sync_main_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v5:alloca-ssa-value-ga"));
}

#[test]
fn p98_sync_main_compile_embedded_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = syx_driver::compile_and_run_embedded(
        r#"func main() { print(90) }"#,
        &syx_driver::DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains("90")), "{out:?}");
}
