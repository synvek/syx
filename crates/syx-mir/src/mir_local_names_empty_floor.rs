//! C-3ae: thin MIR local_names empty-string-slot floor (fail-closed).
//! Not MIR018 rewrite · Not locals type/φ semantics · Not expand pilot
//! · Not C-3 `[x]`.
//!
//! Floor rules (MIR031):
//! - empty `local_names` → OK (AOT strip; same honesty as C-3r/C-3ad)
//! - non-empty: any slot `""` → MIR031
//! - (lower fills `local_{i}`; this fail-closes hand-crafted empty slots)

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn local_names_empty_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR local-names-empty floor: {detail} in '{func_name}'; C-3ae"),
        "MIR031",
    )
}

fn verify_function_local_names_empty_floor(func: &MirFunction) -> SyxResult<()> {
    if func.local_names.is_empty() {
        return Ok(());
    }
    for (index, name) in func.local_names.iter().enumerate() {
        if name.is_empty() {
            return Err(local_names_empty_error(
                &func.name,
                &format!("empty local_names[{index}]"),
            ));
        }
    }
    Ok(())
}

/// C-3ae thin floor: non-empty `local_names` must not contain `""`; else MIR031.
pub fn verify_mir_local_names_empty_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_local_names_empty_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_local_names_empty_floor`] over an explicit function list.
pub fn verify_mir_local_names_empty_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_local_names_empty_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};

    fn flat_fn(name: &str, local_count: u32, local_names: Vec<SmolStr>) -> MirFunction {
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
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_module_ok() {
        verify_mir_local_names_empty_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_vector_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", 2, vec![])],
            ..Default::default()
        };
        verify_mir_local_names_empty_floor(&mir).expect("stripped");
    }

    #[test]
    fn local_i_sentinels_ok() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "ok",
                2,
                vec![SmolStr::new("local_0"), SmolStr::new("x")],
            )],
            ..Default::default()
        };
        verify_mir_local_names_empty_floor(&mir).expect("sentinels");
    }

    #[test]
    fn empty_slot_mir031() {
        let mir = MirModule {
            functions: vec![flat_fn(
                "bad",
                2,
                vec![SmolStr::new("local_0"), SmolStr::new("")],
            )],
            ..Default::default()
        };
        let err = verify_mir_local_names_empty_floor(&mir).expect_err("empty slot");
        assert_eq!(err.code.as_deref(), Some("MIR031"));
        let text = format!("{err}");
        assert!(
            text.contains("empty")
                && text.contains("C-3ae")
                && text.contains("local_names"),
            "{text}"
        );
    }
}
