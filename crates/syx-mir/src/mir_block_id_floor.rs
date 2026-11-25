//! C-3u: thin MIR MirBlock.id uniqueness floor (fail-closed).
//! Not dense 0..n-1 · Not C-3b CFG rewrite · Not flatten/LLVM rewrite · Not ConstFnIndex.
//!
//! Floor rules (MIR021):
//! - empty `blocks` → OK
//! - otherwise → unique `block.id` (`IndexSet` len == blocks.len())

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use indexmap::IndexSet;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn block_id_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR block-id floor: {detail} in '{func_name}'; C-3u"),
        "MIR021",
    )
}

fn verify_function_block_id_floor(func: &MirFunction) -> SyxResult<()> {
    if func.blocks.is_empty() {
        return Ok(());
    }
    let ids: IndexSet<u32> = func.blocks.iter().map(|b| b.id).collect();
    if ids.len() != func.blocks.len() {
        return Err(block_id_error(
            &func.name,
            &format!(
                "duplicate MirBlock.id (unique {}, blocks {})",
                ids.len(),
                func.blocks.len()
            ),
        ));
    }
    Ok(())
}

/// C-3u thin floor: MirBlock.id values must be unique within a function; else MIR021.
pub fn verify_mir_block_id_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_block_id_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_block_id_floor`] over an explicit function list.
pub fn verify_mir_block_id_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_block_id_floor(func)?;
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

    fn flat_fn(name: &str, blocks: Vec<MirBlock>) -> MirFunction {
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
        verify_mir_block_id_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_blocks_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main", vec![])],
            ..Default::default()
        };
        verify_mir_block_id_floor(&mir).expect("empty blocks");
    }

    #[test]
    fn unique_ids_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![ret()],
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
        verify_mir_block_id_floor(&mir).expect("unique");
    }

    #[test]
    fn duplicate_id_mir021() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![
                    MirBlock {
                        id: 7,
                        instructions: vec![ret()],
                        successors: vec![],
                    },
                    MirBlock {
                        id: 7,
                        instructions: vec![ret()],
                        successors: vec![],
                    },
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_block_id_floor(&mir).expect_err("dup");
        assert_eq!(err.code.as_deref(), Some("MIR021"));
        let text = format!("{err}");
        assert!(
            text.contains("MirBlock.id") && text.contains("C-3u") && text.contains("duplicate"),
            "{text}"
        );
    }
}
