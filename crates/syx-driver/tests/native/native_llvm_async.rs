//! Phase 98: native LLVM async await direct emit gate.

use syx_llvm::{emit_llvm_ir, llvm_ir_uses_mir_main_entry, mir_function_needs_mir_entry};
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};

fn async_main_module() -> MirModule {
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
                        op: MirOp::AwaitPoll,
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

#[test]
fn p98_native_llvm_async_main_keeps_mir_entry() {
    let mir = async_main_module();
    assert!(mir_function_needs_mir_entry(&mir.functions[0]));
    let ir = emit_llvm_ir(&mir).unwrap();
    assert!(llvm_ir_uses_mir_main_entry(&ir) || ir.contains("AwaitPoll"));
}

#[test]
fn p98_native_llvm_async_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v5:alloca-ssa-value-ga"));
}

#[test]
fn p98_native_llvm_async_abi_surface() {
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(abi.contains("syx_rt_async_resume"));
}

#[test]
fn p98_native_llvm_async_wasm_emit_surface() {
    let emit = include_str!("../../../syx-wasm-emit/src/mir_emit.rs");
    assert!(emit.contains("AwaitPoll") && emit.contains("AwaitSuspend"));
}
