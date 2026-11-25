//! C-3o: thin MIR fn-index literal OOB floor (fail-closed).
//! Not ConstFnIndex fail-closed (R-33 `-1` sentinel kept) · Not CallIndirect
//! (arity, not fn index) · Not HostFn arity.
//!
//! Floor rules (MIR015):
//! - `CallWitness(a, b)` requires `a < fn_count` and `b < fn_count`
//! - `AwaitSuspend { resume_fn }` requires `resume_fn < fn_count`
//! - no such ops → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn fn_lit_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR fn-lit floor: {detail} in '{func_name}'; C-3o"),
        "MIR015",
    )
}

fn verify_instr_stream(
    func_name: &SmolStr,
    fn_count: u32,
    instructions: &[MirInstr],
) -> SyxResult<()> {
    for instr in instructions {
        match &instr.op {
            MirOp::CallWitness(a, b) => {
                if *a >= fn_count {
                    return Err(fn_lit_error(
                        func_name,
                        &format!("CallWitness({a}, {b}) protocol index out of range (fn_count={fn_count})"),
                    ));
                }
                if *b >= fn_count {
                    return Err(fn_lit_error(
                        func_name,
                        &format!("CallWitness({a}, {b}) method index out of range (fn_count={fn_count})"),
                    ));
                }
            }
            MirOp::AwaitSuspend { resume_fn, .. } if *resume_fn >= fn_count => {
                return Err(fn_lit_error(
                    func_name,
                    &format!(
                        "AwaitSuspend.resume_fn ({resume_fn}) out of range (fn_count={fn_count})"
                    ),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_function_fn_lit_floor(func: &MirFunction, fn_count: u32) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            verify_instr_stream(&func.name, fn_count, &block.instructions)?;
        }
    } else {
        verify_instr_stream(&func.name, fn_count, &func.instructions)?;
    }
    Ok(())
}

/// C-3o thin floor: CallWitness / AwaitSuspend.resume_fn must be `< functions.len()`; else MIR015.
pub fn verify_mir_fn_lit_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_fn_lit_floor_functions(
        module.functions.len() as u32,
        module.functions.iter(),
    )
}

/// Same as [`verify_mir_fn_lit_floor`] over an explicit function list + bound.
pub fn verify_mir_fn_lit_floor_functions<'a>(
    fn_count: u32,
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_fn_lit_floor(func, fn_count)?;
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
        verify_mir_fn_lit_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_fn_lit_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", vec![MirOp::Return])],
            ..Default::default()
        };
        verify_mir_fn_lit_floor(&mir).expect("no fn-lit");
    }

    #[test]
    fn call_witness_in_range_ok() {
        let mir = MirModule {
            functions: vec![
                flat_fn(
                    "main",
                    vec![MirOp::CallWitness(0, 0), MirOp::Return],
                ),
            ],
            ..Default::default()
        };
        verify_mir_fn_lit_floor(&mir).expect("in range");
    }

    #[test]
    fn await_resume_zero_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![
                    MirOp::AwaitSuspend {
                        resume_fn: 0,
                        state: 0,
                    },
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_fn_lit_floor(&mir).expect("resume_fn 0");
    }

    #[test]
    fn call_witness_method_oob_mir015() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::CallWitness(0, 1), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_fn_lit_floor(&mir).expect_err("method OOB");
        assert_eq!(err.code.as_deref(), Some("MIR015"));
        let text = format!("{err}");
        assert!(
            text.contains("CallWitness") && text.contains("C-3o") && text.contains("method"),
            "{text}"
        );
    }

    #[test]
    fn call_witness_protocol_oob_mir015() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::CallWitness(1, 0), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_fn_lit_floor(&mir).expect_err("protocol OOB");
        assert_eq!(err.code.as_deref(), Some("MIR015"));
    }

    #[test]
    fn await_resume_oob_mir015() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![
                    MirOp::AwaitSuspend {
                        resume_fn: 99,
                        state: 0,
                    },
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_fn_lit_floor(&mir).expect_err("resume OOB");
        assert_eq!(err.code.as_deref(), Some("MIR015"));
        let text = format!("{err}");
        assert!(
            text.contains("AwaitSuspend") && text.contains("resume_fn") && text.contains("C-3o"),
            "{text}"
        );
    }

    #[test]
    fn await_oob_in_block_mir015() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("bad"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        instr(MirOp::AwaitSuspend {
                            resume_fn: 1,
                            state: 0,
                        }),
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
        let err = verify_mir_fn_lit_floor(&mir).expect_err("block");
        assert_eq!(err.code.as_deref(), Some("MIR015"));
    }
}
