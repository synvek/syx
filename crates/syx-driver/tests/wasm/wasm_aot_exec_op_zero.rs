//! Phase 91: WASM AOT tier-3 — zero `__syx_aot_exec_op` in emitted module.

use syx_config::wasm_aot_tier3_enabled;
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};
use syx_wasm_emit::emit_wat;

fn sort_by_mir() -> MirModule {
    MirModule {
        functions: vec![MirFunction {
            name: "main".into(),
            param_count: 0,
            local_count: 2,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ConstI32(3),
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

#[test]
fn p98_wasm_aot_tier3_default_on() {
    std::env::remove_var("SYX_WASM_AOT_TIER3");
    assert!(wasm_aot_tier3_enabled());
}

#[test]
fn p98_wasm_aot_exec_op_zero_wat() {
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    let wat = emit_wat(&sort_by_mir()).unwrap();
    assert!(
        !wat.contains("__syx_aot_exec_op"),
        "tier-3 wasm must not import __syx_aot_exec_op"
    );
}

#[test]
fn p98_wasm_aot_tier3_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("wasm-aot-zero-run-op-v1"));
}
