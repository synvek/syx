//! C-3i: thin MIR try/catch pairing floor (fail-closed).
//! Not a full exception / dominator / DefId verifier.
//!
//! Honest vs lower:
//! - `Stmt::Try` emits `EndTry` **before** the `Catch` pad (catch block).
//! - Unary `?` emits `BeginTry`/`EndTry` **without** `Catch`.
//! - `throw` inside `throws` may propagate with **no** local try (depth-0 `Throw` OK).
//!
//! Floor rules (MIR009):
//! - `BeginTry` → depth +1
//! - `EndTry` → depth must be ≥1 then −1
//! - `Catch` → orphan (no prior `BeginTry` in fn) or excess (`catches > begins`) → MIR009
//! - exit depth must be 0 (unclosed try)
//! - `Throw` is **not** gated by depth (propagation)

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn try_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR try floor: {detail} in '{func_name}'; C-3i"),
        "MIR009",
    )
}

fn verify_instr_stream(func_name: &SmolStr, instructions: &[MirInstr], depth: &mut i32, begins: &mut u32, catches: &mut u32) -> SyxResult<()> {
    for instr in instructions {
        match &instr.op {
            MirOp::BeginTry(_) => {
                *depth += 1;
                *begins += 1;
            }
            MirOp::EndTry => {
                if *depth < 1 {
                    return Err(try_error(func_name, "EndTry with no open BeginTry"));
                }
                *depth -= 1;
            }
            MirOp::Catch => {
                *catches += 1;
                if *begins == 0 {
                    return Err(try_error(func_name, "orphan Catch (no BeginTry)"));
                }
                if *catches > *begins {
                    return Err(try_error(
                        func_name,
                        "excess Catch (more Catch than BeginTry)",
                    ));
                }
            }
            // Throw at any depth is legal (throws propagation). Pairing owns EndTry/Catch.
            MirOp::Throw => {}
            _ => {}
        }
    }
    Ok(())
}

fn verify_function_try_floor(func: &MirFunction) -> SyxResult<()> {
    let mut depth: i32 = 0;
    let mut begins: u32 = 0;
    let mut catches: u32 = 0;
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            verify_instr_stream(
                &func.name,
                &block.instructions,
                &mut depth,
                &mut begins,
                &mut catches,
            )?;
        }
    } else {
        verify_instr_stream(
            &func.name,
            &func.instructions,
            &mut depth,
            &mut begins,
            &mut catches,
        )?;
    }
    if depth != 0 {
        return Err(try_error(
            &func.name,
            &format!("unclosed BeginTry (exit depth={depth})"),
        ));
    }
    Ok(())
}

/// C-3i thin floor: BeginTry/EndTry/Catch pairing; imbalance → MIR009.
pub fn verify_mir_try_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_try_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_try_floor`] over an explicit function list (post C-3a unique).
pub fn verify_mir_try_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_try_floor(func)?;
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

    fn blocked_fn(name: &str, blocks: Vec<(u32, Vec<MirOp>)>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: blocks.first().map(|b| b.0).unwrap_or(0),
            blocks: blocks
                .into_iter()
                .map(|(id, ops)| MirBlock {
                    id,
                    instructions: ops.into_iter().map(instr).collect(),
                    successors: vec![],
                })
                .collect(),
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn accepts_try_catch_lower_shape() {
        // Stmt::Try: EndTry before Catch pad.
        let mir = MirModule {
            functions: vec![blocked_fn(
                "ok",
                vec![
                    (
                        0,
                        vec![
                            MirOp::BeginTry(1),
                            MirOp::ConstI32(1),
                            MirOp::EndTry,
                            MirOp::Jump(2),
                        ],
                    ),
                    (1, vec![MirOp::Catch, MirOp::Return]),
                    (2, vec![MirOp::Return]),
                ],
            )],
            ..Default::default()
        };
        verify_mir_try_floor(&mir).expect("try/catch lower shape");
    }

    #[test]
    fn accepts_unary_try_without_catch() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "qmark",
                vec![
                    MirOp::BeginTry(1),
                    MirOp::ConstI32(1),
                    MirOp::EndTry,
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        verify_mir_try_floor(&mir).expect("unary ?");
    }

    #[test]
    fn accepts_throw_without_local_try() {
        // throws propagation — depth-0 Throw must not MIR009.
        let mir = MirModule {
            functions: vec![flat_fn(
                "boom",
                vec![MirOp::ConstStr("x".into()), MirOp::Throw, MirOp::Return],
            )],
            ..Default::default()
        };
        verify_mir_try_floor(&mir).expect("propagating Throw");
    }

    #[test]
    fn rejects_endtry_without_begin() {
        let mir = MirModule {
            functions: vec![flat_fn("bad", vec![MirOp::EndTry, MirOp::Return])],
            ..Default::default()
        };
        let err = verify_mir_try_floor(&mir).expect_err("EndTry");
        assert_eq!(err.code.as_deref(), Some("MIR009"));
        assert!(format!("{err}").contains("EndTry"));
    }

    #[test]
    fn rejects_unclosed_begin() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![MirOp::BeginTry(0), MirOp::Return],
            )],
            ..Default::default()
        };
        let err = verify_mir_try_floor(&mir).expect_err("unclosed");
        assert_eq!(err.code.as_deref(), Some("MIR009"));
        assert!(format!("{err}").contains("unclosed"));
    }

    #[test]
    fn rejects_orphan_catch() {
        let mir = MirModule {
            functions: vec![flat_fn("bad", vec![MirOp::Catch, MirOp::Return])],
            ..Default::default()
        };
        let err = verify_mir_try_floor(&mir).expect_err("orphan Catch");
        assert_eq!(err.code.as_deref(), Some("MIR009"));
        assert!(format!("{err}").contains("orphan Catch"));
    }

    #[test]
    fn rejects_excess_catch() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                vec![
                    MirOp::BeginTry(0),
                    MirOp::EndTry,
                    MirOp::Catch,
                    MirOp::Catch,
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_try_floor(&mir).expect_err("excess Catch");
        assert_eq!(err.code.as_deref(), Some("MIR009"));
        assert!(format!("{err}").contains("excess Catch"));
    }
}
