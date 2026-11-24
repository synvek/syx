//! C-3b: thin MIR CFG / terminator floor (fail-closed). Not a full MirOp verifier.

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule, MirOp};
use syx_diagnostics::SyxResult;

fn is_terminator_floor(op: &MirOp) -> bool {
    matches!(
        op,
        MirOp::Return
            | MirOp::ReturnErr
            | MirOp::Jump(_)
            | MirOp::Throw
            | MirOp::JumpIfFalse(_)
            | MirOp::JumpIfNil(_)
    )
}

fn branch_target(op: &MirOp) -> Option<u32> {
    match op {
        MirOp::Jump(t)
        | MirOp::JumpIfFalse(t)
        | MirOp::JumpIfNil(t)
        | MirOp::BeginTry(t)
        | MirOp::BindResult(t) => Some(*t),
        _ => None,
    }
}

fn verify_function_cfg_floor(func: &MirFunction) -> SyxResult<()> {
    if func.blocks.is_empty() {
        // Flat-instruction / stub shells: no CFG surface to check.
        return Ok(());
    }
    let ids: indexmap::IndexSet<u32> = func.blocks.iter().map(|b| b.id).collect();
    if !ids.contains(&func.entry) {
        return Err(mir_error(
            format!(
                "MIR CFG floor: bad entry {} in '{}'; C-3b",
                func.entry, func.name
            ),
            "MIR002",
        ));
    }
    for block in &func.blocks {
        for &succ in &block.successors {
            if !ids.contains(&succ) {
                return Err(mir_error(
                    format!(
                        "MIR CFG floor: oob successor {} in '{}'; block {}; C-3b",
                        succ, func.name, block.id
                    ),
                    "MIR002",
                ));
            }
        }
        for instr in &block.instructions {
            if let Some(t) = branch_target(&instr.op) {
                if !ids.contains(&t) {
                    return Err(mir_error(
                        format!(
                            "MIR CFG floor: oob jump {} in '{}'; block {}; C-3b",
                            t, func.name, block.id
                        ),
                        "MIR002",
                    ));
                }
            }
        }
        if let Some(last) = block.instructions.last() {
            if !is_terminator_floor(&last.op) {
                return Err(mir_error(
                    format!(
                        "MIR CFG floor: missing terminator in '{}'; block {}; C-3b",
                        func.name, block.id
                    ),
                    "MIR002",
                ));
            }
        }
    }
    Ok(())
}

/// C-3b thin floor: entry / successors / branch targets bounded; non-empty blocks end in a terminator.
pub fn verify_mir_cfg_floor(module: &MirModule) -> SyxResult<()> {
    for func in &module.functions {
        verify_function_cfg_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirBlock, MirInstr};

    fn block(id: u32, ops: Vec<MirOp>, successors: Vec<u32>) -> MirBlock {
        MirBlock {
            id,
            instructions: ops
                .into_iter()
                .map(|op| MirInstr {
                    dest: None,
                    op,
                    span: None,
                })
                .collect(),
            successors,
        }
    }

    fn func(name: &str, entry: u32, blocks: Vec<MirBlock>) -> MirFunction {
        MirFunction {
            name: smol_str::SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry,
            blocks,
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }
    }

    #[test]
    fn accepts_minimal_return() {
        let mir = MirModule {
            functions: vec![func(
                "ok",
                0,
                vec![block(0, vec![MirOp::ConstNil, MirOp::Return], vec![])],
            )],
            ..Default::default()
        };
        verify_mir_cfg_floor(&mir).expect("valid floor");
    }

    #[test]
    fn rejects_bad_entry() {
        let mir = MirModule {
            functions: vec![func(
                "bad_entry",
                9,
                vec![block(0, vec![MirOp::Return], vec![])],
            )],
            ..Default::default()
        };
        let err = verify_mir_cfg_floor(&mir).expect_err("bad entry");
        assert_eq!(err.code.as_deref(), Some("MIR002"));
        assert!(format!("{err}").contains("bad entry"));
    }

    #[test]
    fn rejects_oob_jump() {
        let mir = MirModule {
            functions: vec![func(
                "oob",
                0,
                vec![block(0, vec![MirOp::Jump(99)], vec![])],
            )],
            ..Default::default()
        };
        let err = verify_mir_cfg_floor(&mir).expect_err("oob jump");
        assert_eq!(err.code.as_deref(), Some("MIR002"));
        assert!(format!("{err}").contains("oob jump"));
    }

    #[test]
    fn rejects_missing_terminator() {
        let mir = MirModule {
            functions: vec![func(
                "no_term",
                0,
                vec![block(0, vec![MirOp::ConstNil], vec![])],
            )],
            ..Default::default()
        };
        let err = verify_mir_cfg_floor(&mir).expect_err("missing terminator");
        assert_eq!(err.code.as_deref(), Some("MIR002"));
        assert!(format!("{err}").contains("missing terminator"));
    }
}
