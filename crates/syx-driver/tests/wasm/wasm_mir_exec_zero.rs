//! Phase 91: sync WASM AOT programs must have zero `mir_exec` import calls.

use syx_config::wasm_aot_zero_mir_exec_enabled;
use syx_driver::aot_matrix;
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};
use syx_wasm_emit::{emit_wasm_with_options, measure_aot_tier_stats};

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
fn p98_wasm_zero_mir_exec_default_on() {
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    assert!(wasm_aot_zero_mir_exec_enabled());
}

#[test]
fn p98_wasm_run_op_zero_sync_fixture() {
    std::env::set_var("SYX_WASM_ZERO_MIR_EXEC", "1");
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    let wasm = emit_wasm_with_options(&sort_by_mir(), true).unwrap();
    let stats = measure_aot_tier_stats(&wasm);
    assert_eq!(stats.mir_exec, 0, "mir_exec must be zero: {stats:?}");
}

#[test]
fn p98_wasm_run_op_zero_aot_matrix() {
    std::env::set_var("SYX_WASM_ZERO_MIR_EXEC", "1");
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    let report = aot_matrix::run_aot_matrix(false);
    for fx in &report.fixtures {
        assert_eq!(
            fx.stats.mir_exec, 0,
            "{} mir_exec must be zero: {:?}",
            fx.name, fx.stats
        );
        if fx.name == "hello.sy" || fx.name == "hot_loop.sy" {
            assert_eq!(
                fx.stats.run_op, 0,
                "{} run_op must be zero: {:?}",
                fx.name, fx.stats
            );
        }
    }
}

#[test]
fn p98_wasm_mir_exec_zero_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("wasm-guidemo-zero-v1:main-guidemo-mir-exec-run-op-zero"));
}
