//! C-3y: thin MIR function-name floor (fail-closed).
//! Not Dup/MakeClosure stack · Not ConstFnIndex (R-33) · Not source_name
//! · Not expand pilot · Not C-3 `[x]`.
//!
//! Floor rules (MIR025):
//! - each `functions[i].name` must be non-empty
//! - empty module / no functions → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use syx_diagnostics::SyxResult;

fn fn_name_error(detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR fn-name floor: {detail}; C-3y"),
        "MIR025",
    )
}

fn verify_function_fn_name_floor(index: usize, func: &MirFunction) -> SyxResult<()> {
    if func.name.is_empty() {
        return Err(fn_name_error(&format!(
            "empty functions[{index}].name"
        )));
    }
    Ok(())
}

/// C-3y thin floor: every `MirFunction.name` must be non-empty; else MIR025.
pub fn verify_mir_fn_name_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_fn_name_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_fn_name_floor`] over an explicit function list.
pub fn verify_mir_fn_name_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for (index, func) in functions.into_iter().enumerate() {
        verify_function_fn_name_floor(index, func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};
    use smol_str::SmolStr;

    fn flat_fn(name: &str) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: vec![MirInstr {
                dest: None,
                op: MirOp::Return,
                span: None,
            }],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn empty_ok() {
        verify_mir_fn_name_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn non_empty_name_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main"), flat_fn("g")],
            ..Default::default()
        };
        verify_mir_fn_name_floor(&mir).expect("names");
    }

    #[test]
    fn empty_name_mir025() {
        let mir = MirModule {
            functions: vec![flat_fn("")],
            ..Default::default()
        };
        let err = verify_mir_fn_name_floor(&mir).expect_err("empty name");
        assert_eq!(err.code.as_deref(), Some("MIR025"));
        let text = format!("{err}");
        assert!(
            text.contains("empty") && text.contains("C-3y") && text.contains("name"),
            "{text}"
        );
    }
}
