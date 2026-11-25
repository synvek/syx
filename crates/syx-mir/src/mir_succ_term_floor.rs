//! C-3w: thin MIR successor⊇terminator jump-target floor (fail-closed).
//! Not succ==term · Not mid-block BeginTry ∈ successors · Not dense ids · Not ConstFnIndex.
//!
//! Floor rules (MIR023):
//! - empty `blocks` → OK
//! - for each non-empty block: if last op is Jump(t)/JumpIfFalse(t)/JumpIfNil(t)
//!   and `t ∉ successors` → MIR023
//! - Return / Throw / ReturnErr do not require empty successors (extra exception edges OK)

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn succ_term_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR succ-term floor: {detail} in '{func_name}'; C-3w"),
        "MIR023",
    )
}

fn terminator_jump_target(op: &MirOp) -> Option<u32> {
    match op {
        MirOp::Jump(t) | MirOp::JumpIfFalse(t) | MirOp::JumpIfNil(t) => Some(*t),
        _ => None,
    }
}

fn verify_function_succ_term_floor(func: &MirFunction) -> SyxResult<()> {
    if func.blocks.is_empty() {
        return Ok(());
    }
    for block in &func.blocks {
        let Some(last) = block.instructions.last() else {
            continue;
        };
        let Some(t) = terminator_jump_target(&last.op) else {
            continue;
        };
        if !block.successors.contains(&t) {
            return Err(succ_term_error(
                &func.name,
                &format!(
                    "terminator jump target {t} not in successors of block {}",
                    block.id
                ),
            ));
        }
    }
    Ok(())
}

/// C-3w thin floor: Jump*/JumpIfFalse/JumpIfNil terminator targets must be in successors; else MIR023.
pub fn verify_mir_succ_term_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_succ_term_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_succ_term_floor`] over an explicit function list.
pub fn verify_mir_succ_term_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_succ_term_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirBlock, MirInstr};

    fn ret() -> MirInstr {
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }
    }

    fn jump(t: u32) -> MirInstr {
        MirInstr {
            dest: None,
            op: MirOp::Jump(t),
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

    #[test]
    fn empty_module_ok() {
        verify_mir_succ_term_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_blocks_ok() {
        let mir = MirModule {
            functions: vec![mk("main", vec![])],
            ..Default::default()
        };
        verify_mir_succ_term_floor(&mir).expect("empty blocks");
    }

    #[test]
    fn jump_in_successors_ok() {
        let mir = MirModule {
            functions: vec![mk(
                "ok",
                vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![jump(1)],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![ret()],
                        successors: vec![],
                    },
                ],
            )],
            ..Default::default()
        };
        verify_mir_succ_term_floor(&mir).expect("linked");
    }

    #[test]
    fn return_with_extra_succ_ok() {
        let mir = MirModule {
            functions: vec![mk(
                "ok",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![9],
                }],
            )],
            ..Default::default()
        };
        verify_mir_succ_term_floor(&mir).expect("extra edge");
    }

    #[test]
    fn jump_missing_succ_mir023() {
        let mir = MirModule {
            functions: vec![mk(
                "bad",
                vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![jump(1)],
                        successors: vec![],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![ret()],
                        successors: vec![],
                    },
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_succ_term_floor(&mir).expect_err("missing");
        assert_eq!(err.code.as_deref(), Some("MIR023"));
        let text = format!("{err}");
        assert!(
            text.contains("terminator jump target")
                && text.contains("successors")
                && text.contains("C-3w"),
            "{text}"
        );
    }
}
