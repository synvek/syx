//! C-3q: thin MIR Build stack-height floor (fail-closed).
//! Not a change to `is_suite1_pilot_mir_op` · Not MIR004 expand · Not cross-block φ
//! · Not CallHost arity · Not ConstFnIndex (R-33).
//!
//! Floor rules (MIR017):
//! - track suite1/pilot ops **plus** `ArrayBuild` / `DictBuild` / `SetBuild`
//! - Build stack Δ follows VM/mir-run: Array/Set pop `n` push 1; Dict pop `2n` push 1
//! - underflow **only at Build** → MIR017
//! - underflow at other trackable ops / `CallHost` / unknown Δ → abandon stream
//!   (non-entry blocks may start with StoreLocal of incoming values; ≠ cross-block φ)
//! - no Build / empty → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{is_suite1_pilot_mir_op, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn build_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR build-stack floor: {detail} in '{func_name}'; C-3q"),
        "MIR017",
    )
}

fn is_build_op(op: &MirOp) -> bool {
    matches!(
        op,
        MirOp::ArrayBuild(_) | MirOp::DictBuild(_) | MirOp::SetBuild(_)
    )
}

/// Local trackable set: suite1 pilot ∪ Build. Does **not** mutate `is_suite1_pilot_mir_op`.
fn is_trackable(op: &MirOp) -> bool {
    is_build_op(op) || is_suite1_pilot_mir_op(op)
}

fn stack_effect(op: &MirOp, callees: &IndexMap<SmolStr, u32>) -> Option<(u32, u32)> {
    match op {
        MirOp::ArrayBuild(n) | MirOp::SetBuild(n) => Some((*n, 1)),
        MirOp::DictBuild(n) => Some((n.saturating_mul(2), 1)),
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
        MirOp::ArrayBuild(_) => "ArrayBuild",
        MirOp::DictBuild(_) => "DictBuild",
        MirOp::SetBuild(_) => "SetBuild",
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
) -> SyxResult<()> {
    let mut height: i32 = 0;
    for instr in instructions {
        let op = &instr.op;
        // CallHost / non-trackable: abandon (same honesty as C-3d).
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
            // Only Build underflows are MIR017. Other underflows (e.g. StoreLocal
            // at start of a non-entry block) abandon — incoming height unknown.
            if is_build_op(op) {
                return Err(build_error(
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
    }
    Ok(())
}

fn verify_function_build_stack_floor(
    func: &MirFunction,
    callees: &IndexMap<SmolStr, u32>,
) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        // All blocks; each stream starts at height 0 (≠ cross-block φ).
        for block in &func.blocks {
            verify_instr_stream(&func.name, &block.instructions, callees)?;
        }
        return Ok(());
    }
    verify_instr_stream(&func.name, &func.instructions, callees)
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

/// C-3q thin floor: Build (+pilot) stack height; underflow → MIR017.
pub fn verify_mir_build_stack_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_build_stack_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_build_stack_floor`] over an explicit function list.
pub fn verify_mir_build_stack_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let callees = callee_param_counts(funcs.iter().copied());
    for func in funcs {
        verify_function_build_stack_floor(func, &callees)?;
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
        verify_mir_build_stack_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_build_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![MirOp::ConstI32(1), MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_build_stack_floor(&mir).expect("no Build");
    }

    #[test]
    fn array_build_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![
                    MirOp::ConstI32(1),
                    MirOp::ConstI32(2),
                    MirOp::ArrayBuild(2),
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_build_stack_floor(&mir).expect("ArrayBuild ok");
    }

    #[test]
    fn array_build_underflow_mir017() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::ConstI32(1), MirOp::ArrayBuild(2), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_build_stack_floor(&mir).expect_err("underflow");
        assert_eq!(err.code.as_deref(), Some("MIR017"));
        let text = format!("{err}");
        assert!(
            text.contains("ArrayBuild") && text.contains("C-3q") && text.contains("underflow"),
            "{text}"
        );
    }

    #[test]
    fn dict_build_underflow_mir017() {
        // DictBuild(1) needs 2 stack slots.
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::ConstI32(1), MirOp::DictBuild(1), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_build_stack_floor(&mir).expect_err("dict");
        assert_eq!(err.code.as_deref(), Some("MIR017"));
        assert!(format!("{err}").contains("DictBuild"));
    }

    #[test]
    fn set_build_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![MirOp::ConstI32(1), MirOp::SetBuild(1), MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_build_stack_floor(&mir).expect("SetBuild");
    }

    #[test]
    fn abandon_on_non_trackable_no_false_mir017() {
        // Dup is not trackable → abandon before a later would-be underflow Build.
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![MirOp::Dup, MirOp::ArrayBuild(9), MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_build_stack_floor(&mir).expect("abandon");
    }

    #[test]
    fn abandon_store_local_underflow_not_mir017() {
        // Non-entry / unknown incoming height: StoreLocal underflow abandons.
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![MirOp::StoreLocal(0), MirOp::ArrayBuild(1), MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_build_stack_floor(&mir).expect("StoreLocal abandon");
    }

    #[test]
    fn array_build_underflow_in_block() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("bad"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        instr(MirOp::ArrayBuild(1)),
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
        let err = verify_mir_build_stack_floor(&mir).expect_err("block");
        assert_eq!(err.code.as_deref(), Some("MIR017"));
    }
}
