//! C-3r: thin MIR local debug metadata floor (fail-closed).
//! Not locals type semantics · Not φ/join · Not AOT strip behavior · Not ConstFnIndex.
//!
//! Floor rules (MIR018):
//! - if `local_names` is non-empty → `len as u32 == local_count`
//! - if `local_types` is non-empty → `len as u32 == local_count`
//! - both empty → OK (AOT/production strip path)

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn debug_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR local-debug floor: {detail} in '{func_name}'; C-3r"),
        "MIR018",
    )
}

fn verify_function_local_debug_floor(func: &MirFunction) -> SyxResult<()> {
    if !func.local_names.is_empty() && func.local_names.len() as u32 != func.local_count {
        return Err(debug_error(
            &func.name,
            &format!(
                "local_names.len() ({}) != local_count ({})",
                func.local_names.len(),
                func.local_count
            ),
        ));
    }
    if !func.local_types.is_empty() && func.local_types.len() as u32 != func.local_count {
        return Err(debug_error(
            &func.name,
            &format!(
                "local_types.len() ({}) != local_count ({})",
                func.local_types.len(),
                func.local_count
            ),
        ));
    }
    Ok(())
}

/// C-3r thin floor: non-empty local debug vectors must match `local_count`; else MIR018.
pub fn verify_mir_local_debug_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_local_debug_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_local_debug_floor`] over an explicit function list.
pub fn verify_mir_local_debug_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_local_debug_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};

    fn flat_fn(
        name: &str,
        local_count: u32,
        local_names: Vec<SmolStr>,
        local_types: Vec<SmolStr>,
    ) -> MirFunction {
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
            local_names,
            local_types,
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_ok() {
        verify_mir_local_debug_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn both_empty_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", 2, vec![], vec![])],
            ..Default::default()
        };
        verify_mir_local_debug_floor(&mir).expect("stripped");
    }

    #[test]
    fn equal_len_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                2,
                vec![SmolStr::new("a"), SmolStr::new("b")],
                vec![SmolStr::new("Int"), SmolStr::new("Int")],
            )],
            ..Default::default()
        };
        verify_mir_local_debug_floor(&mir).expect("equal");
    }

    #[test]
    fn names_mismatch_mir018() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                2,
                vec![SmolStr::new("only")],
                vec![],
            )],
            ..Default::default()
        };
        let err = verify_mir_local_debug_floor(&mir).expect_err("names");
        assert_eq!(err.code.as_deref(), Some("MIR018"));
        let text = format!("{err}");
        assert!(
            text.contains("local_names") && text.contains("C-3r") && text.contains("local_count"),
            "{text}"
        );
    }

    #[test]
    fn types_mismatch_mir018() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                1,
                vec![],
                vec![SmolStr::new("Int"), SmolStr::new("extra")],
            )],
            ..Default::default()
        };
        let err = verify_mir_local_debug_floor(&mir).expect_err("types");
        assert_eq!(err.code.as_deref(), Some("MIR018"));
        assert!(format!("{err}").contains("local_types"));
    }
}
