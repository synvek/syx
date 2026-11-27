//! C-3bs: thin MIR ReturnErr stack-height floor (fail-closed).
//! Not a change to `is_suite1_pilot_mir_op` · Not MIR004 expand · Not CallIndirect/Call · Not MakeClosure
//! · Not CallHost arity · Not ConstFnIndex (R-33) · Not MIR009 / full exception.
//!
//! Floor rules (MIR071):
//! - track suite1/pilot ops **plus** `ReturnErr`
//! - ReturnErr stack Δ: pop1 push0 → (1, 0)
//! - underflow **only at ReturnErr** → MIR071 when height is proven in-stream
//! - ReturnErr underflow on **non-entry** before any push in that block → abandon
//!   (incoming height unknown; ≠ cross-block φ)
//! - underflow at other trackable ops / `CallHost` / unknown Δ → abandon stream
//! - no ReturnErr / empty → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{is_suite1_pilot_mir_op, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn return_err_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR return-err-stack floor: {detail} in '{func_name}'; C-3bs"),
        "MIR071",
    )
}

fn is_return_err_op(op: &MirOp) -> bool {
    matches!(op, MirOp::ReturnErr)
}

/// Local trackable set: suite1 pilot ∪ ReturnErr. Does **not** mutate `is_suite1_pilot_mir_op`.
fn is_trackable(op: &MirOp) -> bool {
    is_return_err_op(op) || is_suite1_pilot_mir_op(op)
}

fn stack_effect(op: &MirOp, callees: &IndexMap<SmolStr, u32>) -> Option<(u32, u32)> {
    match op {
        MirOp::ReturnErr => Some((1, 0)),

        MirOp::ConstI32(_)
        | MirOp::ConstStr(_)
        | MirOp::ConstBool(_)
        | MirOp::ConstNil
        | MirOp::LoadLocal(_) => Some((0, 1)),
        MirOp::StoreLocal(_) => Some((1, 0)),
        MirOp::Add { .. } | MirOp::Sub { .. } | MirOp::Mul { .. } | MirOp::Div { .. } => {
            Some((2, 1))
        }
        MirOp::Print => Some((1, 1)),
        MirOp::Jump(_) => Some((0, 0)),
        MirOp::JumpIfFalse(_) => Some((1, 0)),
        MirOp::Return => None,
        MirOp::Call(name) => callees.get(name).map(|&n| (n, 1)),
        MirOp::CallHost(_) => None,
        _ => None,

    }
}

fn op_label(op: &MirOp) -> &'static str {
    match op {
        MirOp::ReturnErr => "ReturnErr",

        MirOp::ConstI32(_) => "ConstI32",
        MirOp::ConstStr(_) => "ConstStr",
        MirOp::ConstBool(_) => "ConstBool",
        MirOp::ConstNil => "ConstNil",
        MirOp::LoadLocal(_) => "LoadLocal",
        MirOp::StoreLocal(_) => "StoreLocal",
        MirOp::Add { .. } => "Add",
        MirOp::Sub { .. } => "Sub",
        MirOp::Mul { .. } => "Mul",
        MirOp::Div { .. } => "Div",
        MirOp::Print => "Print",
        MirOp::Jump(_) => "Jump",
        MirOp::JumpIfFalse(_) => "JumpIfFalse",
        MirOp::Call(_) => "Call",
        MirOp::Return => "Return",
        MirOp::CallHost(_) => "CallHost",
        _ => "Op",

    }
}

fn verify_instr_stream(
    func_name: &SmolStr,
    instructions: &[MirInstr],
    callees: &IndexMap<SmolStr, u32>,
    is_entry: bool,
) -> SyxResult<()> {
    let mut height: i32 = 0;
    let mut saw_push = false;
    for instr in instructions {
        let op = &instr.op;
        if matches!(op, MirOp::CallHost(_)) || !is_trackable(op) {
            return Ok(());
        }
        if matches!(op, MirOp::Return) {
            if height >= 1 {
                height -= 1;
            }
            continue;
        }
        let Some((pops, pushes)) = stack_effect(op, callees) else {
            return Ok(());
        };
        if height < pops as i32 {
            if is_return_err_op(op) {
                if !is_entry && !saw_push {
                    return Ok(());
                }
                return Err(return_err_error(
                    func_name,
                    &format!(
                        "stack underflow before {} (height={height} pops={pops})",
                        op_label(op)
                    ),
                ));
            }
            return Ok(());
        }
        height = height - pops as i32 + pushes as i32;
        if pushes > 0 {
            saw_push = true;
        }
    }
    Ok(())
}

fn verify_function_return_err_stack_floor(
    func: &MirFunction,
    callees: &IndexMap<SmolStr, u32>,
) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            let is_entry = block.id == func.entry;
            verify_instr_stream(&func.name, &block.instructions, callees, is_entry)?;
        }
        return Ok(());
    }
    verify_instr_stream(&func.name, &func.instructions, callees, true)
}

fn callee_param_counts<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> IndexMap<SmolStr, u32> {
    let mut map = IndexMap::new();
    for f in functions {
        map.entry(f.name.clone()).or_insert(f.param_count);
    }
    map
}

/// C-3bs thin floor: ReturnErr (+pilot) stack height; ReturnErr underflow → MIR071.
pub fn verify_mir_return_err_stack_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_return_err_stack_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_return_err_stack_floor`] over an explicit function list.
pub fn verify_mir_return_err_stack_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let callees = callee_param_counts(funcs.iter().copied());
    for func in funcs {
        verify_function_return_err_stack_floor(func, &callees)?;
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

    fn flat_fn_params(name: &str, param_count: u32, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count,
            local_count: param_count,
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
        verify_mir_return_err_stack_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_return_err_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main", vec![MirOp::ConstI32(1), MirOp::Return])],
            ..Default::default()
        };
        verify_mir_return_err_stack_floor(&mir).expect("no ReturnErr");
    }

    #[test]
    fn return_err_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![MirOp::ConstI32(1), MirOp::ReturnErr, MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_return_err_stack_floor(&mir).expect("ReturnErr ok");
    }

    #[test]
    fn return_err_underflow_mir071() {
        let mir = MirModule {
            functions: vec![flat_fn("bad", vec![MirOp::ReturnErr, MirOp::Return])],
            ..Default::default()
        };
        let err = verify_mir_return_err_stack_floor(&mir).expect_err("underflow");
        assert_eq!(err.code.as_deref(), Some("MIR071"));
        let text = format!("{err}");
        assert!(
            text.contains("before ReturnErr") && text.contains("C-3bs") && text.contains("underflow"),
            "{text}"
        );
    }

    #[test]
    fn abandon_on_non_trackable_no_false_mir071() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![
                    MirOp::ActorSend(SmolStr::new("f")),
                    MirOp::ReturnErr,
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_return_err_stack_floor(&mir).expect("abandon");
    }

    #[test]
    fn abandon_store_local_underflow_not_mir071() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![MirOp::StoreLocal(0), MirOp::ReturnErr, MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_return_err_stack_floor(&mir).expect("StoreLocal abandon");
    }

    #[test]
    fn return_err_underflow_in_entry_block() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("bad"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::ReturnErr), instr(MirOp::Return)],
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
        let err = verify_mir_return_err_stack_floor(&mir).expect_err("entry");
        assert_eq!(err.code.as_deref(), Some("MIR071"));
    }

    #[test]
    fn abandon_return_err_underflow_on_non_entry_block() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("ok"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![instr(MirOp::ConstI32(1)), instr(MirOp::Jump(1))],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![instr(MirOp::ReturnErr), instr(MirOp::Return)],
                        successors: vec![],
                    },
                ],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
                owner_module: None,
            }],
            ..Default::default()
        };
        verify_mir_return_err_stack_floor(&mir).expect("non-entry ReturnErr abandon");
    }

    #[test]
    fn return_err_underflow_after_push_in_non_entry_still_mir071() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("bad"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![instr(MirOp::Jump(1))],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![
                            instr(MirOp::ConstI32(1)),
                            instr(MirOp::JumpIfFalse(1)),
                            instr(MirOp::ReturnErr),
                            instr(MirOp::Return),
                        ],
                        successors: vec![],
                    },
                ],
                instructions: vec![],
                local_names: vec![],
                local_types: vec![],
                init_body_pc: 0,
                init_wrap_pc: 0,
                owner_module: None,
            }],
            ..Default::default()
        };
        let err = verify_mir_return_err_stack_floor(&mir).expect_err("proven");
        assert_eq!(err.code.as_deref(), Some("MIR071"));
    }
}
