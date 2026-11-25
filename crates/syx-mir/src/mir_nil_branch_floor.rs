//! C-3g: thin MIR JumpIfNil condition type-tag floor (fail-closed).
//! Not a full exception / BeginTry pairing verifier.
//! Aligns with mir-run: Nil → pop+jump; non-Nil → leave value on stack.

use crate::mir_diag::mir_error;
use crate::mir_types::{is_suite1_pilot_mir_op, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tag {
    I32,
    Str,
    Bool,
    Nil,
    Unknown,
}

fn is_cmp_or_logic_bool_producer(op: &MirOp) -> bool {
    matches!(
        op,
        MirOp::Eq
            | MirOp::Ne
            | MirOp::Lt { .. }
            | MirOp::Le { .. }
            | MirOp::Gt { .. }
            | MirOp::Ge { .. }
            | MirOp::And
            | MirOp::Or
            | MirOp::Not
    )
}

fn pop2(tags: &mut Vec<Tag>) -> Option<(Tag, Tag)> {
    let rhs = tags.pop()?;
    let lhs = tags.pop()?;
    Some((lhs, rhs))
}

fn nil_branch_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!(
            "MIR nil-branch floor: nil-branch condition tag mismatch before JumpIfNil in '{func_name}': {detail}; C-3g"
        ),
        "MIR007",
    )
}

fn verify_instr_stream(
    func_name: &SmolStr,
    instructions: &[MirInstr],
    callees: &IndexMap<SmolStr, u32>,
) -> SyxResult<()> {
    let mut tags: Vec<Tag> = Vec::new();
    for instr in instructions {
        let op = &instr.op;
        if matches!(op, MirOp::CallHost(_)) {
            return Ok(());
        }
        // JumpIfNil is not suite1 pilot — handle before abandon.
        if matches!(op, MirOp::JumpIfNil(_)) {
            let Some(&cond) = tags.last() else {
                return Ok(()); // height short — MIR004 owns
            };
            if matches!(cond, Tag::Bool | Tag::Str | Tag::I32) {
                return Err(nil_branch_error(
                    func_name,
                    "condition must be Nil|Unknown",
                ));
            }
            if cond == Tag::Nil {
                tags.pop(); // mir-run: pop on Nil then jump
            }
            // Unknown: leave on stack (non-Nil path → StoreLocal)
            continue;
        }
        if is_cmp_or_logic_bool_producer(op) {
            match op {
                MirOp::Not => {
                    if tags.pop().is_none() {
                        return Ok(());
                    }
                    tags.push(Tag::Bool);
                }
                _ => {
                    if pop2(&mut tags).is_none() {
                        return Ok(());
                    }
                    tags.push(Tag::Bool);
                }
            }
            continue;
        }
        if !is_suite1_pilot_mir_op(op) {
            return Ok(());
        }
        match op {
            MirOp::ConstI32(_) => tags.push(Tag::I32),
            MirOp::ConstStr(_) => tags.push(Tag::Str),
            MirOp::ConstBool(_) => tags.push(Tag::Bool),
            MirOp::ConstNil => tags.push(Tag::Nil),
            MirOp::LoadLocal(_) => tags.push(Tag::Unknown),
            MirOp::Call(name) => {
                let Some(&n) = callees.get(name) else {
                    return Ok(());
                };
                if tags.len() < n as usize {
                    return Ok(());
                }
                for _ in 0..n {
                    tags.pop();
                }
                tags.push(Tag::Unknown);
            }
            MirOp::StoreLocal(_) | MirOp::Print => {
                if tags.is_empty() {
                    return Ok(());
                }
                tags.pop();
            }
            MirOp::Jump(_) => {}
            MirOp::Return => {
                if !tags.is_empty() {
                    tags.pop();
                }
            }
            MirOp::JumpIfFalse(_) => {
                // Upstream C-3f owns MIR006 — only maintain tag height.
                if tags.pop().is_none() {
                    return Ok(());
                }
            }
            MirOp::Add { .. } => {
                let Some((lhs, rhs)) = pop2(&mut tags) else {
                    return Ok(());
                };
                if lhs == Tag::Str || rhs == Tag::Str {
                    tags.push(Tag::Str);
                } else if lhs == Tag::Unknown || rhs == Tag::Unknown {
                    tags.push(Tag::Unknown);
                } else {
                    tags.push(Tag::I32);
                }
            }
            MirOp::Sub { .. } | MirOp::Mul { .. } | MirOp::Div { .. } => {
                let Some((lhs, rhs)) = pop2(&mut tags) else {
                    return Ok(());
                };
                if lhs == Tag::Unknown || rhs == Tag::Unknown {
                    tags.push(Tag::Unknown);
                } else {
                    tags.push(Tag::I32);
                }
            }
            _ => return Ok(()),
        }
    }
    Ok(())
}

fn verify_function_nil_branch_floor(
    func: &MirFunction,
    callees: &IndexMap<SmolStr, u32>,
) -> SyxResult<()> {
    if !func.blocks.is_empty() {
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
        map.entry(f.name.clone()).or_insert(f.param_count);
    }
    map
}

/// C-3g thin floor: JumpIfNil condition tag; concrete Bool|Str|I32 → MIR007.
pub fn verify_mir_nil_branch_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_nil_branch_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_nil_branch_floor`] over an explicit function list (post C-3a unique).
pub fn verify_mir_nil_branch_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let callees = callee_param_counts(funcs.iter().copied());
    for func in funcs {
        verify_function_nil_branch_floor(func, &callees)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::MirInstr;

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
    fn rejects_i32_nil_cond() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::ConstI32(1), MirOp::JumpIfNil(0), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_nil_branch_floor(&mir).expect_err("I32");
        assert_eq!(err.code.as_deref(), Some("MIR007"));
        assert!(format!("{err}").contains("nil-branch condition tag"));
    }

    #[test]
    fn accepts_nil_and_unknown_store() {
        let mir = MirModule {
            functions: vec![
                flat_fn(
                    "nil",
                    vec![MirOp::ConstNil, MirOp::JumpIfNil(0), MirOp::Return],
                ),
                flat_fn(
                    "unk",
                    vec![
                        MirOp::LoadLocal(0),
                        MirOp::JumpIfNil(0),
                        MirOp::StoreLocal(1),
                        MirOp::Return,
                    ],
                ),
            ],
            ..Default::default()
        };
        verify_mir_nil_branch_floor(&mir).expect("ok");
    }
}
