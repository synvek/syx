//! C-3v: thin MIR flat↔blocks instruction-count floor (fail-closed).
//! Not instruction/PC semantic equality · Not flatten/resync rewrite · Not dense ids · Not ConstFnIndex.
//!
//! Floor rules (MIR022):
//! - empty `blocks` OR empty `instructions` → OK
//! - both non-empty → `sum(block.instructions.len()) == instructions.len()`

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn flat_blocks_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR flat-blocks floor: {detail} in '{func_name}'; C-3v"),
        "MIR022",
    )
}

fn verify_function_flat_blocks_floor(func: &MirFunction) -> SyxResult<()> {
    if func.blocks.is_empty() || func.instructions.is_empty() {
        return Ok(());
    }
    let block_sum: usize = func.blocks.iter().map(|b| b.instructions.len()).sum();
    let flat_len = func.instructions.len();
    if block_sum != flat_len {
        return Err(flat_blocks_error(
            &func.name,
            &format!(
                "sum(block.instructions) ({block_sum}) != instructions.len() ({flat_len})"
            ),
        ));
    }
    Ok(())
}

/// C-3v thin floor: when both flat and blocks are non-empty, instruction counts must match; else MIR022.
pub fn verify_mir_flat_blocks_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_flat_blocks_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_flat_blocks_floor`] over an explicit function list.
pub fn verify_mir_flat_blocks_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_flat_blocks_floor(func)?;
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

    fn mk(
        name: &str,
        blocks: Vec<MirBlock>,
        instructions: Vec<MirInstr>,
    ) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks,
            instructions,
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_module_ok() {
        verify_mir_flat_blocks_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_blocks_ok() {
        let mir = MirModule {
            functions: vec![mk("main", vec![], vec![ret()])],
            ..Default::default()
        };
        verify_mir_flat_blocks_floor(&mir).expect("empty blocks");
    }

    #[test]
    fn empty_flat_ok() {
        let mir = MirModule {
            functions: vec![mk(
                "main",
                vec![MirBlock {
                    id: 0,
                    instructions: vec![ret()],
                    successors: vec![],
                }],
                vec![],
            )],
            ..Default::default()
        };
        verify_mir_flat_blocks_floor(&mir).expect("empty flat");
    }

    #[test]
    fn equal_len_ok() {
        let mir = MirModule {
            functions: vec![mk(
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
                vec![ret(), ret()],
            )],
            ..Default::default()
        };
        verify_mir_flat_blocks_floor(&mir).expect("equal");
    }

    #[test]
    fn mismatch_mir022() {
        let mir = MirModule {
            functions: vec![mk(
                "bad",
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
                vec![ret()],
            )],
            ..Default::default()
        };
        let err = verify_mir_flat_blocks_floor(&mir).expect_err("mismatch");
        assert_eq!(err.code.as_deref(), Some("MIR022"));
        let text = format!("{err}");
        assert!(
            text.contains("sum(block.instructions)")
                && text.contains("instructions.len()")
                && text.contains("C-3v"),
            "{text}"
        );
    }
}
