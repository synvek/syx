//! C-3d: thin MIR value-stack height floor for suite1/pilot ops (fail-closed).
//! Not a full MirOp / cross-block join / type verifier.
//! Stack deltas follow mir-run SSOT (Print Δ=0).

use crate::mir_diag::mir_error;
use crate::mir_types::{is_suite1_pilot_mir_op, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

/// pops / pushes for a pilot op. `None` = skip (CallHost or unknown Call).
fn pilot_stack_effect(
    op: &MirOp,
    callees: &IndexMap<SmolStr, u32>,
) -> Option<(u32, u32)> {
    match op {
        MirOp::ConstI32(_)
        | MirOp::ConstStr(_)
        | MirOp::ConstBool(_)
        | MirOp::ConstNil
        | MirOp::LoadLocal(_) => Some((0, 1)),
        MirOp::StoreLocal(_) => Some((1, 0)),
        MirOp::Add { .. } | MirOp::Sub { .. } | MirOp::Mul { .. } | MirOp::Div { .. } => {
            Some((2, 1))
        }
        // mir-run: pop then push back → Δ 0
        MirOp::Print => Some((1, 1)),
        MirOp::Jump(_) => Some((0, 0)),
        MirOp::JumpIfFalse(_) => Some((1, 0)),
        // Return handled in verify_instr_stream (mir-run Nil pad when empty).
        MirOp::Return => None,
        MirOp::Call(name) => match callees.get(name) {
            Some(&n) => Some((n, 1)),
            None => None, // unknown: skip (MIR003 owns missing callee on codegen)
        },
        // Host arity not in MIR immediates — skip even though pilot-listed.
        MirOp::CallHost(_) => None,
        _ => None,
    }
}

fn op_label(op: &MirOp) -> &'static str {
    match op {
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
        // Non-pilot / CallHost / unresolvable Call: stack Δ unknown.
        // Abandon this stream (do not continue with a stale height — false MIR004).
        if matches!(op, MirOp::CallHost(_)) {
            return Ok(());
        }
        if !is_suite1_pilot_mir_op(op) {
            return Ok(());
        }
        // mir-run Return: `pop().unwrap_or(Nil)`. Empty = Nil pad (not MIR004).
        if matches!(op, MirOp::Return) {
            if height >= 1 {
                height -= 1;
            }
            continue;
        }
        let Some((pops, pushes)) = pilot_stack_effect(op, callees) else {
            // e.g. Call(name) with unknown callee — abandon stream.
            return Ok(());
        };
        if height < pops as i32 {
            return Err(mir_error(
                format!(
                    "MIR stack floor: stack underflow before {} in '{}'; height={height} pops={pops}; C-3d",
                    op_label(op),
                    func_name
                ),
                "MIR004",
            ));
        }
        height = height - pops as i32 + pushes as i32;
    }
    Ok(())
}

fn verify_function_stack_floor(
    func: &MirFunction,
    callees: &IndexMap<SmolStr, u32>,
) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        // Thin floor: no cross-block join φ. Non-entry blocks have unknown
        // incoming height (e.g. for-in StoreLocal of iterator value) — skip them.
        for block in &func.blocks {
            if block.id != func.entry {
                continue;
            }
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
        // First wins (C-3a unique path already collapsed conflicts).
        map.entry(f.name.clone()).or_insert(f.param_count);
    }
    map
}

/// C-3d thin floor: suite1/pilot value-stack height; underflow → MIR004.
pub fn verify_mir_stack_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_stack_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_stack_floor`] over an explicit function list (post C-3a unique).
pub fn verify_mir_stack_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let callees = callee_param_counts(funcs.iter().copied());
    for func in funcs {
        verify_function_stack_floor(func, &callees)?;
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

    fn flat_fn(name: &str, param_count: u32, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count,
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

    fn block_fn(name: &str, ops: Vec<MirOp>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: ops.into_iter().map(instr).collect(),
                successors: vec![],
            }],
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn accepts_const_add_return() {
        let mir = MirModule {
            functions: vec![block_fn(
                "ok",
                vec![
                    MirOp::ConstI32(1),
                    MirOp::ConstI32(2),
                    MirOp::Add { kind: 0 },
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_stack_floor(&mir).expect("valid");
    }

    #[test]
    fn rejects_add_underflow() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad_add",
                0,
                vec![MirOp::ConstI32(1), MirOp::Add { kind: 0 }, MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_stack_floor(&mir).expect_err("underflow");
        assert_eq!(err.code.as_deref(), Some("MIR004"));
        assert!(format!("{err}").contains("stack underflow"));
        assert!(format!("{err}").contains("Add"));
    }

    #[test]
    fn accepts_empty_return_nil_pad() {
        // mir-run pads Nil; CallHost;Return shells must not MIR004.
        let mir = MirModule {
            functions: vec![flat_fn("empty_ret", 0, vec![MirOp::Return])],
            ..Default::default()
        };
        verify_mir_stack_floor(&mir).expect("empty Return Nil pad");
    }

    #[test]
    fn rejects_store_local_underflow() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad_store",
                0,
                vec![MirOp::StoreLocal(0), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_stack_floor(&mir).expect_err("store underflow");
        assert_eq!(err.code.as_deref(), Some("MIR004"));
        assert!(format!("{err}").contains("StoreLocal"));
    }

    #[test]
    fn rejects_call_arity_underflow() {
        let callee = flat_fn("needs2", 2, vec![MirOp::ConstNil, MirOp::Return]);
        let caller = flat_fn(
            "main",
            0,
            vec![
                MirOp::Call(SmolStr::new("needs2")), // needs 2 args, height=0
                MirOp::Return,
            ],
        );
        let mir = MirModule {
            functions: vec![caller, callee],
            ..Default::default()
        };
        let err = verify_mir_stack_floor(&mir).expect_err("call underflow");
        assert_eq!(err.code.as_deref(), Some("MIR004"));
        assert!(format!("{err}").contains("Call"));
    }

    #[test]
    fn accepts_known_call_arity() {
        let callee = flat_fn("needs1", 1, vec![MirOp::ConstNil, MirOp::Return]);
        let caller = flat_fn(
            "main",
            0,
            vec![
                MirOp::ConstI32(1),
                MirOp::Call(SmolStr::new("needs1")),
                MirOp::Return,
            ],
        );
        let mir = MirModule {
            functions: vec![caller, callee],
            ..Default::default()
        };
        verify_mir_stack_floor(&mir).expect("call ok");
    }
}
