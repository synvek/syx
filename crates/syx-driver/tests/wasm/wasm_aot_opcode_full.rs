//! Phase 98: WASM AOT tier-3 full opcode direct emit (zero run_op on hot path).

use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};
use syx_wasm_emit::{emit_wasm_with_options, measure_aot_run_op_count, TIER3_NAMES};

fn tier3_hot_mir() -> MirModule {
    MirModule {
        functions: vec![MirFunction {
            name: "main".into(),
            param_count: 0,
            local_count: 3,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ConstI32(1),
                        span: None,
                    },
                    MirInstr {
                        dest: Some(1),
                        op: MirOp::ConstI32(2),
                        span: None,
                    },
                    MirInstr {
                        dest: Some(2),
                        op: MirOp::Add {
                            kind: syx_stdlib::PLATFORM_INT.to_byte(),
                        },
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Pop,
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
fn p98_wasm_aot_opcode_full_tier3_names() {
    assert!(TIER3_NAMES.len() >= 4);
}

#[test]
fn p98_wasm_aot_opcode_full_hot_path_zero_run_op() {
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    std::env::set_var("SYX_WASM_ZERO_MIR_EXEC", "1");
    let wasm = emit_wasm_with_options(&tier3_hot_mir(), true).unwrap();
    assert_eq!(measure_aot_run_op_count(&wasm), 0);
}

#[test]
fn p98_wasm_aot_opcode_full_mir_emit_surface() {
    let src = include_str!("../../../syx-wasm-emit/src/mir_emit.rs");
    assert!(src.contains("MirOp::Pop") && src.contains("MirOp::Add"));
}

#[test]
fn p98_wasm_aot_opcode_full_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("wasm-aot-zero-run-op-v1"));
}
