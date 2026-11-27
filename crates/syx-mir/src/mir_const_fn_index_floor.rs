//! C-3cd: thin MIR ConstFnIndex resolve floor (fail-closed / R-33).
//! Not CallIndirect arity · Not HostFn arity · Not Call(ModuleId) IR · Not C-3 `[x]`.
//!
//! Floor rules (MIR080):
//! - `ConstFnIndex(name)` must resolve in [`MirFnIndex`]
//! - no ConstFnIndex ops → OK
//!
//! Optional missing shells (Equatable/Comparable) must demote to `ConstI32(-1)`
//! via [`demote_unresolved_const_fn_index`] before this floor — not silent
//! codegen `-1` after claiming `ConstFnIndex`.

use crate::mir_diag::mir_error;
use crate::mir_fn_index::MirFnIndex;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_diagnostics::SyxResult;

fn cfi_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR ConstFnIndex floor: {detail} in '{func_name}'; C-3cd"),
        "MIR080",
    )
}

fn verify_instr_stream(
    func_name: &SmolStr,
    index: &MirFnIndex,
    instructions: &[MirInstr],
) -> SyxResult<()> {
    for instr in instructions {
        if let MirOp::ConstFnIndex(name) = &instr.op {
            let key = SmolStr::new(name.as_str());
            if index.resolve(&key).is_none() {
                return Err(cfi_error(
                    func_name,
                    &format!("unknown ConstFnIndex '{name}'"),
                ));
            }
        }
    }
    Ok(())
}

fn verify_function_const_fn_index_floor(func: &MirFunction, index: &MirFnIndex) -> SyxResult<()> {
    if !func.blocks.is_empty() {
        for block in &func.blocks {
            verify_instr_stream(&func.name, index, &block.instructions)?;
        }
    } else {
        verify_instr_stream(&func.name, index, &func.instructions)?;
    }
    Ok(())
}

/// C-3cd thin floor: ConstFnIndex names must resolve in [`MirFnIndex`]; else MIR080.
pub fn verify_mir_const_fn_index_floor(module: &MirModule) -> SyxResult<()> {
    let index = MirFnIndex::build(module)?;
    verify_mir_const_fn_index_floor_functions(&index, module.functions.iter())
}

/// Same as [`verify_mir_const_fn_index_floor`] over an explicit index + function list.
pub fn verify_mir_const_fn_index_floor_functions<'a>(
    index: &MirFnIndex,
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_const_fn_index_floor(func, index)?;
    }
    Ok(())
}

fn demote_ops(ops: &mut [MirInstr], known: &HashSet<SmolStr>) {
    for instr in ops {
        if let MirOp::ConstFnIndex(name) = &instr.op {
            let key = SmolStr::new(name.as_str());
            if !known.contains(&key) {
                instr.op = MirOp::ConstI32(-1);
            }
        }
    }
}

/// C-3cd: rewrite unresolved `ConstFnIndex` → `ConstI32(-1)` (R-33 optional-shell sentinel)
/// so the ConstFnIndex floor only sees claimed fn indices.
pub fn demote_unresolved_const_fn_index(module: &mut MirModule) {
    let known: HashSet<SmolStr> = module.functions.iter().map(|f| f.name.clone()).collect();
    for func in &mut module.functions {
        if !func.blocks.is_empty() {
            for block in &mut func.blocks {
                demote_ops(&mut block.instructions, &known);
            }
        } else {
            demote_ops(&mut func.instructions, &known);
        }
    }
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
        verify_mir_const_fn_index_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn no_const_fn_index_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main", vec![MirOp::Return])],
            ..Default::default()
        };
        verify_mir_const_fn_index_floor(&mir).expect("no ConstFnIndex");
    }

    #[test]
    fn known_const_fn_index_ok() {
        let mir = MirModule {
            functions: vec![
                flat_fn("helper", vec![MirOp::Return]),
                flat_fn(
                    "main",
                    vec![
                        MirOp::ConstFnIndex("helper".into()),
                        MirOp::Return,
                    ],
                ),
            ],
            ..Default::default()
        };
        verify_mir_const_fn_index_floor(&mir).expect("known");
    }

    #[test]
    fn unknown_const_fn_index_mir080() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![
                    MirOp::ConstFnIndex("Int8_equals".into()),
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        let err = verify_mir_const_fn_index_floor(&mir).expect_err("unknown");
        assert_eq!(err.code.as_deref(), Some("MIR080"));
        let text = format!("{err}");
        assert!(
            text.contains("Int8_equals")
                && text.contains("C-3cd")
                && text.contains("unknown ConstFnIndex"),
            "{text}"
        );
    }

    #[test]
    fn demote_then_floor_ok() {
        let mut mir = MirModule {
            functions: vec![flat_fn(
                "main",
                vec![
                    MirOp::ConstFnIndex("Int8_equals".into()),
                    MirOp::Return,
                ],
            )],
            ..Default::default()
        };
        demote_unresolved_const_fn_index(&mut mir);
        assert!(matches!(
            mir.functions[0].instructions[0].op,
            MirOp::ConstI32(-1)
        ));
        verify_mir_const_fn_index_floor(&mir).expect("demoted");
    }

    #[test]
    fn unknown_in_block_mir080() {
        let mir = MirModule {
            functions: vec![MirFunction {
                name: SmolStr::new("main"),
                param_count: 0,
                local_count: 0,
                entry: 0,
                blocks: vec![MirBlock {
                    id: 0,
                    instructions: vec![
                        instr(MirOp::ConstFnIndex("gone".into())),
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
        let err = verify_mir_const_fn_index_floor(&mir).expect_err("block");
        assert_eq!(err.code.as_deref(), Some("MIR080"));
    }
}
