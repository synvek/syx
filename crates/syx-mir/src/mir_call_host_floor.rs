//! C-3m: thin MIR CallHost absolute-index floor (fail-closed).
//! Not HostFn arity/semantics · Not Call→FuncId · Not bytecode trim dense
//! `host_fn_names` remap (program_trim may remap indices after this floor).
//!
//! Floor rules (MIR013):
//! - `CallHost(i)` requires `syx_stdlib::host_fn_name_for_index(i).is_some()`
//! - no CallHost → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn call_host_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR call-host floor: {detail} in '{func_name}'; C-3m"),
        "MIR013",
    )
}

fn verify_instr_stream(func_name: &SmolStr, instructions: &[MirInstr]) -> SyxResult<()> {
    for instr in instructions {
        if let MirOp::CallHost(idx) = instr.op {
            if syx_stdlib::host_fn_name_for_index(idx).is_none() {
                return Err(call_host_error(
                    func_name,
                    &format!("CallHost({idx}) unresolved in absolute host space"),
                ));
            }
        }
    }
    Ok(())
}

fn verify_function_call_host_floor(func: &MirFunction) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            verify_instr_stream(&func.name, &block.instructions)?;
        }
    } else {
        verify_instr_stream(&func.name, &func.instructions)?;
    }
    Ok(())
}

/// C-3m thin floor: absolute CallHost indices must resolve; else MIR013.
pub fn verify_mir_call_host_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_call_host_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_call_host_floor`] over an explicit function list.
pub fn verify_mir_call_host_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_call_host_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::MirBlock;
    use syx_stdlib::{host_fn_index, HostFnId};

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
        verify_mir_call_host_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_call_host_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", vec![MirOp::Return])],
            ..Default::default()
        };
        verify_mir_call_host_floor(&mir).expect("no CallHost");
    }

    #[test]
    fn known_index_ok() {
        let idx = host_fn_index(HostFnId::FmtPrint);
        let mir = MirModule {
            functions: vec![flat_fn("ok", vec![MirOp::CallHost(idx), MirOp::Return])],
            ..Default::default()
        };
        verify_mir_call_host_floor(&mir).expect("known index");
    }

    #[test]
    fn unresolved_index_mir013() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::CallHost(u32::MAX), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_call_host_floor(&mir).expect_err("unresolved");
        assert_eq!(err.code.as_deref(), Some("MIR013"));
        let text = format!("{err}");
        assert!(
            text.contains("CallHost") && text.contains("C-3m") && text.contains("unresolved"),
            "{text}"
        );
    }

    #[test]
    fn unresolved_in_block_mir013() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("bad"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        instr(MirOp::CallHost(u32::MAX - 1)),
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
        let err = verify_mir_call_host_floor(&mir).expect_err("block");
        assert_eq!(err.code.as_deref(), Some("MIR013"));
    }
}
