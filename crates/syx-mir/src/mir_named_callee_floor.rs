//! C-3p: thin MIR named-callee resolve floor (fail-closed).
//! Not ConstFnIndex (owned by C-3cd / MIR080) · Not CallIndirect
//! · Not HostFn arity · Not Call(ModuleId) IR.
//!
//! Floor rules (MIR016):
//! - `Call` / `CallSuper` / `ActorCall` / `ActorSend` / `MakeClosure` / `TaskSpawn`
//!   names must resolve in [`MirFnIndex`]
//! - `ConstFnIndex` is **not** scanned here (see C-3cd)
//! - no named-callee ops → OK

use crate::mir_diag::mir_error;
use crate::mir_fn_index::MirFnIndex;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn callee_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR named-callee floor: {detail} in '{func_name}'; C-3p"),
        "MIR016",
    )
}

fn verify_instr_stream(
    func_name: &SmolStr,
    index: &MirFnIndex,
    instructions: &[MirInstr],
) -> SyxResult<()> {
    for instr in instructions {
        let name = match &instr.op {
            MirOp::Call(n)
            | MirOp::CallSuper(n)
            | MirOp::ActorCall(n)
            | MirOp::ActorSend(n)
            | MirOp::TaskSpawn(n) => Some(n),
            MirOp::MakeClosure(n, _) => Some(n),
            // ConstFnIndex deliberately skipped (R-33).
            _ => None,
        };
        if let Some(name) = name {
            if index.resolve(name).is_none() {
                return Err(callee_error(
                    func_name,
                    &format!("unknown callee '{name}'"),
                ));
            }
        }
    }
    Ok(())
}

fn verify_function_named_callee_floor(func: &MirFunction, index: &MirFnIndex) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            verify_instr_stream(&func.name, index, &block.instructions)?;
        }
    } else {
        verify_instr_stream(&func.name, index, &func.instructions)?;
    }
    Ok(())
}

/// C-3p thin floor: named callees must resolve in module [`MirFnIndex`]; else MIR016.
pub fn verify_mir_named_callee_floor(module: &MirModule) -> SyxResult<()> {
    let index = MirFnIndex::build(module)?;
    verify_mir_named_callee_floor_functions(&index, module.functions.iter())
}

/// Same as [`verify_mir_named_callee_floor`] over an explicit index + function list.
pub fn verify_mir_named_callee_floor_functions<'a>(
    index: &MirFnIndex,
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_named_callee_floor(func, index)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::MirBlock;

    fn instr(op: MirOp) -> MirInstr {
        MirInstr {
            dest: None,
            op,
            span: None,
        }
    }

    fn flat_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: ops.into_iter().map(instr).collect(),
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_ok() {
        verify_mir_named_callee_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_named_callee_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main", vec![MirOp::Return])],
            ..Default::default()
        };
        verify_mir_named_callee_floor(&mir).expect("no callee");
    }

    #[test]
    fn known_call_ok() {
        let mir = MirModule {
            functions: vec![
                flat_fn("helper", vec![MirOp::Return]),
                flat_fn(
                    "main",
                    vec![MirOp::Call(SmolStr::new("helper")), MirOp::Return],
                ),
            ],
            ..Default::default()
        };
        verify_mir_named_callee_floor(&mir).expect("known Call");
    }

    #[test]
    fn unknown_call_mir016() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![MirOp::Call(SmolStr::new("missing")), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_named_callee_floor(&mir).expect_err("unknown");
        assert_eq!(err.code.as_deref(), Some("MIR016"));
        let text = format!("{err}");
        assert!(
            text.contains("missing") && text.contains("C-3p") && text.contains("unknown callee"),
            "{text}"
        );
    }

    #[test]
    fn unknown_actor_send_mir016() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![MirOp::ActorSend(SmolStr::new("gone")), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_named_callee_floor(&mir).expect_err("ActorSend");
        assert_eq!(err.code.as_deref(), Some("MIR016"));
    }

    #[test]
    fn const_fn_index_missing_not_scanned() {
        // R-33: ConstFnIndex missing names stay codegen ConstI32(-1); floor must not MIR016.
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![
                    MirOp::ConstFnIndex("Int8_equals".into()),
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_named_callee_floor(&mir).expect("ConstFnIndex not scanned");
    }

    #[test]
    fn unknown_call_in_block_mir016() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("main"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        instr(MirOp::Call(SmolStr::new("nope"))),
                        instr(MirOp::Return),
                    ],
                    successors: vec![],
                }],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
                owner_module: None,
            }],
            ..Default::default()
        };
        let err = verify_mir_named_callee_floor(&mir).expect_err("block");
        assert_eq!(err.code.as_deref(), Some("MIR016"));
    }
}
