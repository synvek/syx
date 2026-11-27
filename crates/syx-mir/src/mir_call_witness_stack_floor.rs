//! C-3by: thin MIR CallWitness stack-height floor (fail-closed).
//! Not a change to `is_suite1_pilot_mir_op` · Not MIR004 expand · Not CallIndirect full arity
//! · Not CallHost arity · Not ConstFnIndex (R-33) · Not MIR015 index OOB · Not full arity semantics.
//!
//! Floor rules (MIR077):
//! - track suite1/pilot ops **plus** `CallWitness`
//! - CallWitness(_, method) stack Δ: pop `param_count(functions[method])` push1 → (n, 1);
//!   method OOB → abandon
//! - underflow **only at CallWitness** → MIR077 when height is proven in-stream
//! - CallWitness underflow on **non-entry** before any push in that block → abandon
//! - underflow at other trackable ops / `CallHost` / unknown Δ → abandon stream
//! - no CallWitness / empty → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{is_suite1_pilot_mir_op, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn call_witness_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR call-witness-stack floor: {detail} in '{func_name}'; C-3by"),
        "MIR077",
    )
}

fn is_call_witness_op(op: &MirOp) -> bool {
    matches!(op, MirOp::CallWitness(_, _))
}

/// Local trackable set: suite1 pilot ∪ CallWitness. Does **not** mutate `is_suite1_pilot_mir_op`.
fn is_trackable(op: &MirOp) -> bool {
    is_call_witness_op(op) || is_suite1_pilot_mir_op(op)
}

fn stack_effect(
    op: &MirOp,
    callees: &IndexMap<SmolStr, u32>,
    by_index: &[u32],
) -> Option<(u32, u32)> {
    match op {
        MirOp::CallWitness(_, method) => by_index.get(*method as usize).copied().map(|n| (n, 1)),
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
        MirOp::CallWitness(_, _) => "CallWitness",
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
    by_index: &[u32],
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
        let Some((pops, pushes)) = stack_effect(op, callees, by_index) else {
            return Ok(());
        };
        if height < pops as i32 {
            if is_call_witness_op(op) {
                if !is_entry && !saw_push {
                    return Ok(());
                }
                return Err(call_witness_error(
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

fn verify_function_call_witness_stack_floor(
    func: &MirFunction,
    callees: &IndexMap<SmolStr, u32>,
    by_index: &[u32],
) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            let is_entry = block.id == func.entry;
            verify_instr_stream(&func.name, &block.instructions, callees, by_index, is_entry)?;
        }
        return Ok(());
    }
    verify_instr_stream(&func.name, &func.instructions, callees, by_index, true)
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

fn param_counts_by_index<'a>(functions: &[&'a MirFunction]) -> Vec<u32> {
    functions.iter().map(|f| f.param_count).collect()
}

/// C-3by thin floor: CallWitness (+pilot) stack height; CallWitness underflow → MIR077.
pub fn verify_mir_call_witness_stack_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_call_witness_stack_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_call_witness_stack_floor`] over an explicit function list.
pub fn verify_mir_call_witness_stack_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let callees = callee_param_counts(funcs.iter().copied());
    let by_index = param_counts_by_index(&funcs);
    for func in funcs {
        verify_function_call_witness_stack_floor(func, &callees, &by_index)?;
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
        verify_mir_call_witness_stack_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_call_witness_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main", vec![MirOp::ConstI32(1), MirOp::Return])],
            ..Default::default()
        };
        verify_mir_call_witness_stack_floor(&mir).expect("no CallWitness");
    }

    #[test]
    fn call_witness_n0_ok() {
        // method index 0 → f with param_count 0
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 0, vec![MirOp::Return]),
                flat_fn(
                    "main",
                    vec![MirOp::CallWitness(0, 0), MirOp::Return],
                ),
            ],
            ..Default::default()
        };
        verify_mir_call_witness_stack_floor(&mir).expect("CallWitness n=0 ok");
    }

    #[test]
    fn call_witness_n1_ok() {
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 1, vec![MirOp::Return]),
                flat_fn(
                    "main",
                    vec![
                        MirOp::ConstI32(1),
                        MirOp::CallWitness(0, 0),
                        MirOp::Return,
                    ],
                ),
            ],
            ..Default::default()
        };
        verify_mir_call_witness_stack_floor(&mir).expect("CallWitness n=1 ok");
    }

    #[test]
    fn call_witness_underflow_mir077() {
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 1, vec![MirOp::Return]),
                flat_fn(
                    "bad",
                    vec![MirOp::CallWitness(0, 0), MirOp::Return],
                ),
            ],
            ..Default::default()
        };
        let err = verify_mir_call_witness_stack_floor(&mir).expect_err("underflow");
        assert_eq!(err.code.as_deref(), Some("MIR077"));
        let text = format!("{err}");
        assert!(
            text.contains("before CallWitness")
                && text.contains("C-3by")
                && text.contains("underflow"),
            "{text}"
        );
    }

    #[test]
    fn abandon_oob_method() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![MirOp::CallWitness(0, 99), MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_call_witness_stack_floor(&mir).expect("oob abandon");
    }

    #[test]
    fn abandon_on_non_trackable_no_false_mir077() {
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 1, vec![MirOp::Return]),
                flat_fn(
                    "ok",
                    vec![
                        MirOp::CallSuper(SmolStr::new("f")),
                        MirOp::CallWitness(0, 0),
                        MirOp::Return,
                    ],
                ),
            ],
            ..Default::default()
        };
        verify_mir_call_witness_stack_floor(&mir).expect("abandon");
    }

    #[test]
    fn abandon_store_local_underflow_not_mir077() {
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 1, vec![MirOp::Return]),
                flat_fn(
                    "ok",
                    vec![
                        MirOp::StoreLocal(0),
                        MirOp::CallWitness(0, 0),
                        MirOp::Return,
                    ],
                ),
            ],
            ..Default::default()
        };
        verify_mir_call_witness_stack_floor(&mir).expect("StoreLocal abandon");
    }

    #[test]
    fn call_witness_underflow_in_entry_block() {
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 1, vec![MirOp::Return]),
                MirFunction {
                    name: SmolStr::new("bad"),
                    param_count: 0,
                    local_count: 0,
                    entry: 0,
                    blocks: vec![MirBlock {
                        id: 0,
                        instructions: vec![
                            instr(MirOp::CallWitness(0, 0)),
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
                },
            ],
            ..Default::default()
        };
        let err = verify_mir_call_witness_stack_floor(&mir).expect_err("entry");
        assert_eq!(err.code.as_deref(), Some("MIR077"));
    }

    #[test]
    fn abandon_call_witness_underflow_on_non_entry_block() {
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 1, vec![MirOp::Return]),
                MirFunction {
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
                            instructions: vec![
                                instr(MirOp::CallWitness(0, 0)),
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
                },
            ],
            ..Default::default()
        };
        verify_mir_call_witness_stack_floor(&mir).expect("non-entry CallWitness abandon");
    }

    #[test]
    fn call_witness_underflow_after_push_in_non_entry_still_mir077() {
        let mir = MirModule {
            functions: vec![
                flat_fn_params("f", 1, vec![MirOp::Return]),
                MirFunction {
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
                                instr(MirOp::CallWitness(0, 0)),
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
                },
            ],
            ..Default::default()
        };
        let err = verify_mir_call_witness_stack_floor(&mir).expect_err("proven");
        assert_eq!(err.code.as_deref(), Some("MIR077"));
    }
}
