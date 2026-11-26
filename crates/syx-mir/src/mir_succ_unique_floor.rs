//! C-3ab: thin MIR MirBlock.successors uniqueness floor (fail-closed).
//! Not succ==term · Not MIR023 rewrite · Not dense ids · Not expand pilot
//! · Not C-3 `[x]`.
//!
//! Floor rules (MIR028):
//! - empty `blocks` / empty `successors` → OK
//! - otherwise each block's `successors` must have unique targets
//!   (`IndexSet` len == successors.len())
//! - (`LowerCtx::link` already dedups; this fail-closes hand-crafted dups)

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use indexmap::IndexSet;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn succ_unique_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR succ-unique floor: {detail} in '{func_name}'; C-3ab"),
        "MIR028",
    )
}

fn verify_function_succ_unique_floor(func: &MirFunction) -> SyxResult<()> {
    if func.blocks.is_empty() {
        return Ok(());
    }
    for block in &func.blocks {
        if block.successors.is_empty() {
            continue;
        }
        let uniq: IndexSet<u32> = block.successors.iter().copied().collect();
        if uniq.len() != block.successors.len() {
            return Err(succ_unique_error(
                &func.name,
                &format!(
                    "duplicate successors in MirBlock.id={} (unique {}, len {})",
                    block.id,
                    uniq.len(),
                    block.successors.len()
                ),
            ));
        }
    }
    Ok(())
}

/// C-3ab thin floor: MirBlock.successors must be unique within each block; else MIR028.
pub fn verify_mir_succ_unique_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_succ_unique_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_succ_unique_floor`] over an explicit function list.
pub fn verify_mir_succ_unique_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_succ_unique_floor(func)?;
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
        verify_mir_succ_unique_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_blocks_ok() {
        let mir = MirModule {
            functions: vec![fn_with_blocks("main", vec![])],
            ..Default::default()
        };
        verify_mir_succ_unique_floor(&mir).expect("empty blocks");
    }

    #[test]
    fn unique_successors_ok() {
        let mir = MirModule {
            functions: vec![fn_with_blocks(
                "ok",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![1, 2],
                }],
            )],
            ..Default::default()
        };
        verify_mir_succ_unique_floor(&mir).expect("unique");
    }

    #[test]
    fn empty_successors_ok() {
        let mir = MirModule {
            functions: vec![fn_with_blocks(
                "ok",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![],
                }],
            )],
            ..Default::default()
        };
        verify_mir_succ_unique_floor(&mir).expect("empty succ");
    }

    #[test]
    fn duplicate_successors_mir028() {
        let mir = MirModule {
            functions: vec![fn_with_blocks(
                "bad",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![1, 1],
                }],
            )],
            ..Default::default()
        };
        let err = verify_mir_succ_unique_floor(&mir).expect_err("dup");
        assert_eq!(err.code.as_deref(), Some("MIR028"));
        let text = format!("{err}");
        assert!(
            text.contains("duplicate")
                && text.contains("C-3ab")
                && text.contains("successors"),
            "{text}"
        );
    }
}
