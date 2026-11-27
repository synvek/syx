//! C-3ch: thin MIR BindResult mid-block successor floor (fail-closed).
//! Not C-3w terminator Jump* · Not C-3b block-id OOB · Not AwaitPoll · Not full exception.
//!
//! Floor rules (MIR083):
//! - empty `blocks` / flat `instructions` only → OK (no successors metadata; abandon)
//! - for each non-empty block: any mid-stream `BindResult(t)` with `t ∉ successors` → MIR083
//! - `BindResult(t)` with `t ∈ successors` / no BindResult → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn bind_result_succ_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR bind-result-succ floor: {detail} in '{func_name}'; C-3ch"),
        "MIR083",
    )
}

fn verify_block_bind_result_succ(
    func_name: &SmolStr,
    block_id: u32,
    instructions: &[MirInstr],
    successors: &[u32],
) -> SyxResult<()> {
    for instr in instructions {
        if let MirOp::BindResult(t) = instr.op {
            if !successors.contains(&t) {
                return Err(bind_result_succ_error(
                    func_name,
                    &format!(
                        "BindResult({t}) target not in successors of block {block_id}"
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn verify_function_bind_result_succ_floor(func: &MirFunction) -> SyxResult<()> {
    // Flat / empty-blocks path: no successors metadata → abandon (OK).
    if func.blocks.is_empty() {
        return Ok(());
    }
    for block in &func.blocks {
        if block.instructions.is_empty() {
            continue;
        }
        verify_block_bind_result_succ(
            &func.name,
            block.id,
            &block.instructions,
            &block.successors,
        )?;
    }
    Ok(())
}

/// C-3ch thin floor: mid-block `BindResult(t)` must be in `block.successors`; else MIR083.
pub fn verify_mir_bind_result_succ_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_bind_result_succ_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_bind_result_succ_floor`] over an explicit function list.
pub fn verify_mir_bind_result_succ_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_bind_result_succ_floor(func)?;
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

    fn mk(name: &str, blocks: Vec<MirBlock>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks,
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    fn flat(name: &str, ops: Vec<MirOp>) -> MirFunction {
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
        verify_mir_bind_result_succ_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn flat_instructions_abandon_ok() {
        let mir = MirModule {
            functions: vec![flat(
                "main",
                vec![MirOp::BindResult(9), MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_bind_result_succ_floor(&mir).expect("flat abandon");
    }

    #[test]
    fn bind_result_in_successors_ok() {
        let mir = MirModule {
            functions: vec![mk(
                "ok",
                vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![
                            instr(MirOp::BindResult(1)),
                            instr(MirOp::Jump(2)),
                        ],
                        successors: vec![1, 2],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![instr(MirOp::Return)],
                        successors: vec![],
                    },
                    MirBlock {
                        id: 2,
                        instructions: vec![instr(MirOp::Return)],
                        successors: vec![],
                    },
                ],
            )],
            ..Default::default()
        };
        verify_mir_bind_result_succ_floor(&mir).expect("linked");
    }

    #[test]
    fn no_bind_result_ok() {
        let mir = MirModule {
            functions: vec![mk(
                "ok",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::Return)],
                    successors: vec![],
                }],
            )],
            ..Default::default()
        };
        verify_mir_bind_result_succ_floor(&mir).expect("no BindResult");
    }

    #[test]
    fn bind_result_missing_succ_mir083() {
        // Block 2 exists but is not listed in successors of block 0.
        let mir = MirModule {
            functions: vec![mk(
                "bad",
                vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![
                            instr(MirOp::BindResult(2)),
                            instr(MirOp::Jump(1)),
                        ],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![instr(MirOp::Return)],
                        successors: vec![],
                    },
                    MirBlock {
                        id: 2,
                        instructions: vec![instr(MirOp::Return)],
                        successors: vec![],
                    },
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_bind_result_succ_floor(&mir).expect_err("missing succ");
        assert_eq!(err.code.as_deref(), Some("MIR083"));
        let text = format!("{err}");
        assert!(
            text.contains("BindResult(2)") && text.contains("C-3ch") && text.contains("successors"),
            "{text}"
        );
    }
}
