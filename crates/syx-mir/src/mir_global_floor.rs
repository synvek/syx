//! C-3l: thin MIR globals slot OOB floor (fail-closed).
//! Not global name table · Not cross-module remap · Not locals type · Not Call(ModuleId) IR.
//!
//! Floor rules (MIR012):
//! - `LoadGlobal(i)` / `StoreGlobal(i)` require `i < module.global_count`
//! - empty / no Global ops with `global_count=0` → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn global_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR global floor: {detail} in '{func_name}'; C-3l"),
        "MIR012",
    )
}

/// Max `LoadGlobal`/`StoreGlobal` slot + 1 across functions (0 if none).
/// Used when merging function-only fragments that lost `MirModule.global_count`.
pub fn mir_global_slot_bound<'a>(functions: impl IntoIterator<Item = &'a MirFunction>) -> u32 {
    let mut max_slot: Option<u32> = None;
    for func in functions {
        let streams: Box<dyn Iterator<Item = &MirInstr>> = if !func.blocks.is_empty() {
            Box::new(func.blocks.iter().flat_map(|b| b.instructions.iter()))
        } else {
            Box::new(func.instructions.iter())
        };
        for instr in streams {
            match &instr.op {
                MirOp::LoadGlobal(slot) | MirOp::StoreGlobal(slot) => {
                    max_slot = Some(max_slot.map_or(*slot, |m| m.max(*slot)));
                }
                _ => {}
            }
        }
    }
    max_slot.map(|s| s.saturating_add(1)).unwrap_or(0)
}

/// Raise `dst.global_count` to at least `bound`.
pub fn absorb_mir_global_count(dst: &mut MirModule, bound: u32) {
    if bound > dst.global_count {
        dst.global_count = bound;
    }
}

fn verify_instr_stream(
    func_name: &SmolStr,
    global_count: u32,
    instructions: &[MirInstr],
) -> SyxResult<()> {
    for instr in instructions {
        match &instr.op {
            MirOp::LoadGlobal(slot) if *slot >= global_count => {
                return Err(global_error(
                    func_name,
                    &format!("LoadGlobal({slot}) out of range (global_count={global_count})"),
                ));
            }
            MirOp::StoreGlobal(slot) if *slot >= global_count => {
                return Err(global_error(
                    func_name,
                    &format!("StoreGlobal({slot}) out of range (global_count={global_count})"),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_function_global_floor(func: &MirFunction, global_count: u32) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            verify_instr_stream(&func.name, global_count, &block.instructions)?;
        }
    } else {
        verify_instr_stream(&func.name, global_count, &func.instructions)?;
    }
    Ok(())
}

/// C-3l thin floor: LoadGlobal/StoreGlobal slots must be `< global_count`; else MIR012.
pub fn verify_mir_global_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_global_floor_functions(module.global_count, module.functions.iter())
}

/// Same as [`verify_mir_global_floor`] over an explicit function list + bound.
pub fn verify_mir_global_floor_functions<'a>(
    global_count: u32,
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_global_floor(func, global_count)?;
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
        verify_mir_global_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn in_range_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![
                    MirOp::ConstI32(1),
                    MirOp::StoreGlobal(0),
                    MirOp::LoadGlobal(0),
                    MirOp::Return,
                ],
            )],
            global_count: 1,
            ..Default::default()
        };
        verify_mir_global_floor(&mir).expect("in range");
    }

    #[test]
    fn zero_no_access_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", vec![MirOp::Return])],
            global_count: 0,
            ..Default::default()
        };
        verify_mir_global_floor(&mir).expect("zero");
    }

    #[test]
    fn oob_load_mir012() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::LoadGlobal(0), MirOp::Return],
            )],
            global_count: 0,
            ..Default::default()
        };
        let err = verify_mir_global_floor(&mir).expect_err("OOB Load");
        assert_eq!(err.code.as_deref(), Some("MIR012"));
        let text = format!("{err}");
        assert!(text.contains("LoadGlobal") && text.contains("C-3l"), "{text}");
    }

    #[test]
    fn oob_store_mir012() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::ConstI32(0), MirOp::StoreGlobal(2), MirOp::Return],
            )],
            global_count: 2,
            ..Default::default()
        };
        let err = verify_mir_global_floor(&mir).expect_err("OOB Store");
        assert_eq!(err.code.as_deref(), Some("MIR012"));
        let text = format!("{err}");
        assert!(text.contains("StoreGlobal") && text.contains("C-3l"), "{text}");
    }

    #[test]
    fn slot_bound_helper() {
        let fns = [flat_fn(
            "g",
            vec![MirOp::StoreGlobal(3), MirOp::LoadGlobal(1), MirOp::Return],
        )];
        assert_eq!(mir_global_slot_bound(fns.iter()), 4);
        assert_eq!(mir_global_slot_bound(std::iter::empty()), 0);
    }

    #[test]
    fn oob_in_block_mir012() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("bad"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::LoadGlobal(9)), instr(MirOp::Return)],
                    successors: vec![],
                }],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
                owner_module: None,
            }],
            global_count: 1,
            ..Default::default()
        };
        let err = verify_mir_global_floor(&mir).expect_err("block OOB");
        assert_eq!(err.code.as_deref(), Some("MIR012"));
    }
}
