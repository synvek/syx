//! C-3e: thin MIR value-stack type-tag floor for suite1/pilot ops (fail-closed).
//! Not a full type system / cross-block join / locals slot verifier.
//! Rules follow mir-run SSOT (Add string concat; Nil in numeric → reject).

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

fn op_label(op: &MirOp) -> &'static str {
    match op {
        MirOp::Add { .. } => "Add",
        MirOp::Sub { .. } => "Sub",
        MirOp::Mul { .. } => "Mul",
        MirOp::Div { .. } => "Div",
        _ => "Op",
    }
}

fn type_tag_error(func_name: &SmolStr, op: &MirOp, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!(
            "MIR type floor: type tag mismatch before {} in '{}': {detail}; C-3e",
            op_label(op),
            func_name
        ),
        "MIR005",
    )
}

fn pop2(tags: &mut Vec<Tag>) -> Option<(Tag, Tag)> {
    let rhs = tags.pop()?;
    let lhs = tags.pop()?;
    Some((lhs, rhs))
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
                    return Ok(()); // unknown callee — abandon (MIR003 owns)
                };
                if tags.len() < n as usize {
                    return Ok(()); // height short — abandon (MIR004 owns)
                }
                for _ in 0..n {
                    tags.pop();
                }
                tags.push(Tag::Unknown);
            }
            MirOp::StoreLocal(_) | MirOp::Print | MirOp::JumpIfFalse(_) => {
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
            MirOp::Add { .. } => {
                let Some((lhs, rhs)) = pop2(&mut tags) else {
                    return Ok(());
                };
                if lhs == Tag::Nil || rhs == Tag::Nil {
                    return Err(type_tag_error(
                        func_name,
                        op,
                        "Nil cannot participate in Add",
                    ));
                }
                // mir-run: any Str side → concat
                if lhs == Tag::Str || rhs == Tag::Str {
                    tags.push(Tag::Str);
                } else if lhs == Tag::Unknown || rhs == Tag::Unknown {
                    tags.push(Tag::Unknown);
                } else {
                    // I32/Bool mixes: numeric path (Bool coerced at runtime) → I32
                    tags.push(Tag::I32);
                }
            }
            MirOp::Sub { .. } | MirOp::Mul { .. } | MirOp::Div { .. } => {
                let Some((lhs, rhs)) = pop2(&mut tags) else {
                    return Ok(());
                };
                for t in [lhs, rhs] {
                    if matches!(t, Tag::Str | Tag::Bool | Tag::Nil) {
                        return Err(type_tag_error(
                            func_name,
                            op,
                            "non-numeric concrete tag on numeric op",
                        ));
                    }
                }
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

fn verify_function_type_floor(
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

/// C-3e thin floor: suite1/pilot coarse type tags; category mismatch → MIR005.
pub fn verify_mir_type_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_type_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_type_floor`] over an explicit function list (post C-3a unique).
pub fn verify_mir_type_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let callees = callee_param_counts(funcs.iter().copied());
    for func in funcs {
        verify_function_type_floor(func, &callees)?;
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
    fn rejects_nil_add() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![
                    MirOp::ConstNil,
                    MirOp::ConstI32(1),
                    MirOp::Add { kind: 2 },
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_type_floor(&mir).expect_err("Nil Add");
        assert_eq!(err.code.as_deref(), Some("MIR005"));
        assert!(format!("{err}").contains("type tag"));
    }

    #[test]
    fn rejects_str_sub() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![
                    MirOp::ConstStr("a".into()),
                    MirOp::ConstI32(1),
                    MirOp::Sub { kind: 2 },
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_type_floor(&mir).expect_err("Str Sub");
        assert_eq!(err.code.as_deref(), Some("MIR005"));
    }

    #[test]
    fn accepts_i32_add_and_str_concat() {
        let mir = MirModule {
            functions: vec![
                flat_fn(
                    "num",
                    vec![
                        MirOp::ConstI32(1),
                        MirOp::ConstI32(2),
                        MirOp::Add { kind: 2 },
                        MirOp::Return,
                    ],
                ),
                flat_fn(
                    "cat",
                    vec![
                        MirOp::ConstStr("a".into()),
                        MirOp::ConstStr("b".into()),
                        MirOp::Add { kind: 2 },
                        MirOp::Return,
                    ],
                ),
            ],
            ..Default::default()
        };
        verify_mir_type_floor(&mir).expect("valid");
    }

    #[test]
    fn accepts_unknown_add() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                vec![
                    MirOp::LoadLocal(0),
                    MirOp::ConstI32(1),
                    MirOp::Add { kind: 2 },
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_type_floor(&mir).expect("Unknown Add");
    }
}
