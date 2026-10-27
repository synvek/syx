//! Phase 91: multi-fn sync LLVM IR must not reference `syx_rt_call_fn`.

use syx_llvm::{emit_llvm_ir, llvm_ir_uses_mir_call_fn, llvm_ir_uses_mir_main_entry};
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};

fn multi_fn_sync_module() -> MirModule {
    MirModule {
        functions: vec![
            MirFunction {
                name: "helper".into(),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstI32(7),
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
            },
            MirFunction {
                name: "main".into(),
                param_count: 0,
                local_count: 1,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        MirInstr {
                            dest: Some(0),
                            op: MirOp::Call("helper".into()),
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
            },
        ],
        ..Default::default()
    }
}

#[test]
fn p98_native_user_fn_ir_zero_mir_call_fn() {
    let mir = multi_fn_sync_module();
    let ir = emit_llvm_ir(&mir).unwrap();
    assert!(
        !llvm_ir_uses_mir_call_fn(&ir),
        "sync multi-fn IR must not call syx_rt_call_fn"
    );
    assert!(ir.contains("@helper"));
    assert!(ir.contains("call i32 @helper"));
}

#[test]
fn p98_native_user_fn_main_direct() {
    let mir = multi_fn_sync_module();
    let mut ir = emit_llvm_ir(&mir).unwrap();
    syx_llvm::append_native_runtime_shim(&mir, &mut ir).unwrap();
    assert!(!llvm_ir_uses_mir_main_entry(&ir));
}

#[test]
fn p98_native_user_fn_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v5:alloca-ssa-value-ga"));
}
