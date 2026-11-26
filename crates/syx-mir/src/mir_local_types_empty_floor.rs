//! C-3ad: thin MIR local_types empty-string-slot floor (fail-closed).
//! Not MIR018 rewrite · Not locals type/φ semantics · Not local_names empty slots
//! · Not expand pilot · Not C-3 `[x]`.
//!
//! Floor rules (MIR030):
//! - empty `local_types` → OK (AOT strip; same honesty as C-3r)
//! - non-empty: any slot `""` → MIR030
//! - (lower fills `"Unknown"`; this fail-closes hand-crafted empty slots)

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn local_types_empty_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR local-types-empty floor: {detail} in '{func_name}'; C-3ad"),
        "MIR030",
    )
}

fn verify_function_local_types_empty_floor(func: &MirFunction) -> SyxResult<()> {
    if func.local_types.is_empty() {
        return Ok(());
    }
    for (index, ty) in func.local_types.iter().enumerate() {
        if ty.is_empty() {
            return Err(local_types_empty_error(
                &func.name,
                &format!("empty local_types[{index}]"),
            ));
        }
    }
    Ok(())
}

/// C-3ad thin floor: non-empty `local_types` must not contain `""`; else MIR030.
pub fn verify_mir_local_types_empty_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_local_types_empty_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_local_types_empty_floor`] over an explicit function list.
pub fn verify_mir_local_types_empty_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_local_types_empty_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};

    fn flat_fn(name: &str, local_count: u32, local_types: Vec<SmolStr>) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count,
            entry: 0,
            blocks: vec![],
            instructions: vec![MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            }],
            local_names: vec![],
            local_types,
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_module_ok() {
        verify_mir_local_types_empty_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_vector_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", 2, vec![])],
            ..Default::default()
        };
        verify_mir_local_types_empty_floor(&mir).expect("stripped");
    }

    #[test]
    fn unknown_sentinels_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                2,
                vec![SmolStr::new("Unknown"), SmolStr::new("Int")],
            )],
            ..Default::default()
        };
        verify_mir_local_types_empty_floor(&mir).expect("sentinels");
    }

    #[test]
    fn empty_slot_mir030() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                2,
                vec![SmolStr::new("Unknown"), SmolStr::new("")],
            )],
            ..Default::default()
        };
        let err = verify_mir_local_types_empty_floor(&mir).expect_err("empty slot");
        assert_eq!(err.code.as_deref(), Some("MIR030"));
        let text = format!("{err}");
        assert!(
            text.contains("empty")
                && text.contains("C-3ad")
                && text.contains("local_types"),
            "{text}"
        );
    }
}
