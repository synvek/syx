//! C-3z: thin MIR non-empty block-instruction floor (fail-closed).
//! Not MIR002 rewrite · Not succ==term · Not dense ids · Not expand pilot
//! · Not C-3 `[x]`.
//!
//! Floor rules (MIR026):
//! - empty `blocks` → OK (flat/stub)
//! - otherwise each `MirBlock.instructions` must be non-empty
//!   (closes MIR002 hole: empty instr blocks skip terminator check)

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn block_instr_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR block-instr floor: {detail} in '{func_name}'; C-3z"),
        "MIR026",
    )
}

fn verify_function_block_instr_floor(func: &MirFunction) -> SyxResult<()> {
    if func.blocks.is_empty() {
        return Ok(());
    }
    for block in &func.blocks {
        if block.instructions.is_empty() {
            return Err(block_instr_error(
                &func.name,
                &format!("empty instructions in MirBlock.id={}", block.id),
            ));
        }
    }
    Ok(())
}

/// C-3z thin floor: non-empty `blocks` require ≥1 instruction each; else MIR026.
pub fn verify_mir_block_instr_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_block_instr_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_block_instr_floor`] over an explicit function list.
pub fn verify_mir_block_instr_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_block_instr_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirBlock, MirInstr, MirOp};

    fn ret() -> MirInstr {
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        }
    }

    fn fn_with_blocks(name: &str, blocks: Vec<MirBlock>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks,
            instructions: vec![ret()],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_module_ok() {
        verify_mir_block_instr_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_blocks_ok() {
        let mir = MirModule {
            functions: vec![fn_with_blocks("main", vec![])],
            ..Default::default()
        };
        verify_mir_block_instr_floor(&mir).expect("empty blocks");
    }

    #[test]
    fn non_empty_instr_ok() {
        let mir = MirModule {
            functions: vec![fn_with_blocks(
                "main",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![],
                }],
            )],
            ..Default::default()
        };
        verify_mir_block_instr_floor(&mir).expect("instr");
    }

    #[test]
    fn empty_instr_block_mir026() {
        let mir = MirModule {
            functions: vec![fn_with_blocks(
                "bad",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![],
                    successors: vec![],
                }],
            )],
            ..Default::default()
        };
        let err = verify_mir_block_instr_floor(&mir).expect_err("empty instr");
        assert_eq!(err.code.as_deref(), Some("MIR026"));
        let text = format!("{err}");
        assert!(
            text.contains("empty") && text.contains("C-3z") && text.contains("instructions"),
            "{text}"
        );
    }
}
