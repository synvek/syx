//! C-3k: thin MIR locals slot OOB floor (fail-closed).
//! Not full locals type · Not φ/join · Not LoadGlobal · Not Call(ModuleId) IR.
//!
//! Floor rules (MIR011):
//! - `LoadLocal(i)` / `StoreLocal(i)` require `i < local_count`
//! - empty fn / no Load·Store with any `local_count` → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn local_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR local floor: {detail} in '{func_name}'; C-3k"),
        "MIR011",
    )
}

fn verify_instr_stream(func: &MirFunction, instructions: &[MirInstr]) -> SyxResult<()> {
    let n = func.local_count;
    for instr in instructions {
        match &instr.op {
            MirOp::LoadLocal(slot) if *slot >= n => {
                return Err(local_error(
                    &func.name,
                    &format!("LoadLocal({slot}) out of range (local_count={n})"),
                ));
            }
            MirOp::StoreLocal(slot) if *slot >= n => {
                return Err(local_error(
                    &func.name,
                    &format!("StoreLocal({slot}) out of range (local_count={n})"),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_function_local_floor(func: &MirFunction) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            verify_instr_stream(func, &block.instructions)?;
        }
    } else {
        verify_instr_stream(func, &func.instructions)?;
    }
    Ok(())
}

/// C-3k thin floor: LoadLocal/StoreLocal slots must be `< local_count`; else MIR011.
pub fn verify_mir_local_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_local_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_local_floor`] over an explicit function list.
pub fn verify_mir_local_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_local_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirBlock, MirInstr};

    fn instr(op: MirOp) -> MirInstr {
        MirInstr {
            dest: None,
            op,
            span: None,
        }
    }

    fn flat_fn(name: &str, local_count: u32, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count,
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

    fn blocked_fn(name: &str, local_count: u32, blocks: Vec<(u32, Vec<MirOp>)>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count,
            entry: blocks.first().map(|b| b.0).unwrap_or(0),
            blocks: blocks
                .into_iter()
                .map(|(id, ops)| MirBlock {
                    id,
                    instructions: ops.into_iter().map(instr).collect(),
                    successors: vec![],
                })
                .collect(),
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_ok() {
        verify_mir_local_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn in_range_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                2,
                vec![
                    MirOp::ConstI32(1),
                    MirOp::StoreLocal(0),
                    MirOp::LoadLocal(0),
                    MirOp::StoreLocal(1),
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_local_floor(&mir).expect("in range");
    }

    #[test]
    fn zero_locals_no_access_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", 0, vec![MirOp::Return])],
            ..Default::default()
        };
        verify_mir_local_floor(&mir).expect("zero locals");
    }

    #[test]
    fn oob_load_mir011() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                1,
                vec![MirOp::LoadLocal(1), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_local_floor(&mir).expect_err("OOB Load");
        assert_eq!(err.code.as_deref(), Some("MIR011"));
        let text = format!("{err}");
        assert!(text.contains("LoadLocal") && text.contains("C-3k"), "{text}");
    }

    #[test]
    fn oob_store_mir011() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                0,
                vec![MirOp::ConstI32(0), MirOp::StoreLocal(0), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_local_floor(&mir).expect_err("OOB Store");
        assert_eq!(err.code.as_deref(), Some("MIR011"));
        let text = format!("{err}");
        assert!(text.contains("StoreLocal") && text.contains("C-3k"), "{text}");
    }

    #[test]
    fn oob_in_block_mir011() {
        let mir = MirModule {
            functions: vec![blocked_fn(
                "bad",
                1,
                vec![(0, vec![MirOp::LoadLocal(5), MirOp::Return])],
            )],
            ..Default::default()
        };
        let err = verify_mir_local_floor(&mir).expect_err("block OOB");
        assert_eq!(err.code.as_deref(), Some("MIR011"));
    }
}
