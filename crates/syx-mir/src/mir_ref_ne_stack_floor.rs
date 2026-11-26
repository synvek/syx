//! C-3be: thin MIR RefNe stack-height floor (fail-closed).
//! Not a change to `is_suite1_pilot_mir_op` · Not MIR004 expand · Not ArrayGet/Call · Not MakeClosure
//! · Not CallHost arity · Not ConstFnIndex (R-33).
//!
//! Floor rules (MIR057):
//! - track suite1/pilot ops **plus** `RefNe`
//! - RefNe stack Δ: pop2 push1 → (2, 1); unit op, no kind
//! - underflow **only at RefNe** → MIR057 when height is proven in-stream
//! - RefNe underflow on **non-entry** before any push in that block → abandon
//!   (incoming height unknown; ≠ cross-block φ)
//! - underflow at other trackable ops / `CallHost` / unknown Δ → abandon stream
//! - no RefNe / empty → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{is_suite1_pilot_mir_op, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn ref_ne_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR ref-ne-stack floor: {detail} in '{func_name}'; C-3be"),
        "MIR057",
    )
}

fn is_ref_ne_op(op: &MirOp) -> bool {
    matches!(op, MirOp::RefNe)
}

/// Local trackable set: suite1 pilot ∪ RefNe. Does **not** mutate `is_suite1_pilot_mir_op`.
fn is_trackable(op: &MirOp) -> bool {
    is_ref_ne_op(op) || is_suite1_pilot_mir_op(op)
}

fn stack_effect(op: &MirOp, callees: &IndexMap<SmolStr, u32>) -> Option<(u32, u32)> {
    match op {
        // pop2 push1 (VM RefNe); unit op.
        MirOp::RefNe => Some((2, 1)),
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
        MirOp::Return => None, // handled inline (Nil pad)
        MirOp::Call(name) => callees.get(name).map(|&n| (n, 1)),
        MirOp::CallHost(_) => None,
        _ => None,
    }
}

fn op_label(op: &MirOp) -> &'static str {
    match op {
        MirOp::RefNe => "RefNe",
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
        // CallHost / non-trackable: abandon (same honesty as C-3x/C-3q).
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
            // RefNe underflow: MIR057 when entry/flat, or non-entry after an
            // in-stream push (proven empty). Non-entry with no prior push →
            // abandon (incoming height unknown; ≠ cross-block φ).
            // Other underflows abandon (same honesty as C-3x).
            if is_ref_ne_op(op) {
                if !is_entry && !saw_push {
                    return Ok(());
                }
                return Err(ref_ne_error(
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

fn verify_function_ref_ne_stack_floor(
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

/// C-3be thin floor: RefNe (+pilot) stack height; RefNe underflow → MIR057.
pub fn verify_mir_ref_ne_stack_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_ref_ne_stack_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_ref_ne_stack_floor`] over an explicit function list.
pub fn verify_mir_ref_ne_stack_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let callees = callee_param_counts(funcs.iter().copied());
    for func in funcs {
        verify_function_ref_ne_stack_floor(func, &callees)?;
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
        verify_mir_ref_ne_stack_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_ref_ne_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![MirOp::ConstI32(1), MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_ref_ne_stack_floor(&mir).expect("no RefNe");
    }

    #[test]
    fn ref_ne_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![
                    MirOp::ConstI32(1),
                    MirOp::ConstI32(2),
                    MirOp::RefNe,
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_ref_ne_stack_floor(&mir).expect("RefNe ok");
    }

    #[test]
    fn ref_ne_single_value_underflow_mir057() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::ConstI32(1), MirOp::RefNe, MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_ref_ne_stack_floor(&mir).expect_err("single");
        assert_eq!(err.code.as_deref(), Some("MIR057"));
    }

    #[test]
    fn ref_ne_underflow_mir057() {
        let mir = MirModule {
            functions: vec![flat_fn("bad", vec![MirOp::RefNe, MirOp::Return])],
            ..Default::default()
        };
        let err = verify_mir_ref_ne_stack_floor(&mir).expect_err("underflow");
        assert_eq!(err.code.as_deref(), Some("MIR057"));
        let text = format!("{err}");
        assert!(
            text.contains("before RefNe") && text.contains("C-3be") && text.contains("underflow"),
            "{text}"
        );
    }

    #[test]
    fn abandon_on_non_trackable_no_false_mir057() {
        // ArrayGet is not trackable here → abandon before a later would-be underflow RefNe.
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![MirOp::ArrayGet, MirOp::RefNe, MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_ref_ne_stack_floor(&mir).expect("abandon");
    }

    #[test]
    fn abandon_store_local_underflow_not_mir057() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![MirOp::StoreLocal(0), MirOp::RefNe, MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_ref_ne_stack_floor(&mir).expect("StoreLocal abandon");
    }

    #[test]
    fn ref_ne_underflow_in_entry_block() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("bad"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![instr(MirOp::RefNe), instr(MirOp::Return)],
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
        let err = verify_mir_ref_ne_stack_floor(&mir).expect_err("entry");
        assert_eq!(err.code.as_deref(), Some("MIR057"));
    }

    #[test]
    fn abandon_ref_ne_underflow_on_non_entry_block() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("ok"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![
                    MirBlock {
                        id: 0,
                        instructions: vec![
                            instr(MirOp::ConstI32(1)),
                            instr(MirOp::Jump(1)),
                        ],
                        successors: vec![1],
                    },
                    MirBlock {
                        id: 1,
                        instructions: vec![instr(MirOp::RefNe), instr(MirOp::Return)],
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
        verify_mir_ref_ne_stack_floor(&mir).expect("non-entry RefNe abandon");
    }

    #[test]
    fn ref_ne_underflow_after_push_in_non_entry_still_mir057() {
        // Proven empty within the non-entry stream → still MIR057.
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
                        // Proven: Const → height 1; JumpIfFalse → 0; RefNe → MIR057.
                        instructions: vec![
                            instr(MirOp::ConstI32(1)),
                            instr(MirOp::JumpIfFalse(1)),
                            instr(MirOp::RefNe),
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
        let err = verify_mir_ref_ne_stack_floor(&mir).expect_err("proven");
        assert_eq!(err.code.as_deref(), Some("MIR057"));
    }
}
