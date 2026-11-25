//! C-3n: thin MIR param_count ≤ local_count metadata floor (fail-closed).
//! Not locals type · Not φ/join · Not ConstFnIndex fail-closed · Not HostFn arity.
//!
//! Floor rules (MIR014):
//! - each `MirFunction` requires `param_count <= local_count`
//! - empty module / `param_count == 0` → OK

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn param_local_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR param-local floor: {detail} in '{func_name}'; C-3n"),
        "MIR014",
    )
}

fn verify_function_param_local_floor(func: &MirFunction) -> SyxResult<()> {
    if func.param_count > func.local_count {
        return Err(param_local_error(
            &func.name,
            &format!(
                "param_count ({}) > local_count ({})",
                func.param_count, func.local_count
            ),
        ));
    }
    Ok(())
}

/// C-3n thin floor: `param_count` must be `<= local_count`; else MIR014.
pub fn verify_mir_param_local_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_param_local_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_param_local_floor`] over an explicit function list.
pub fn verify_mir_param_local_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_param_local_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};

    fn flat_fn(name: &str, param_count: u32, local_count: u32) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count,
            local_count,
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
        verify_mir_param_local_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn zero_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", 0, 0)],
            ..Default::default()
        };
        verify_mir_param_local_floor(&mir).expect("zero");
    }

    #[test]
    fn param_le_local_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("ok", 2, 2), flat_fn("ok2", 1, 3)],
            ..Default::default()
        };
        verify_mir_param_local_floor(&mir).expect("param <= local");
    }

    #[test]
    fn param_gt_local_mir014() {
        let mir = MirModule {
            functions: vec![flat_fn("bad", 2, 1)],
            ..Default::default()
        };
        let err = verify_mir_param_local_floor(&mir).expect_err("param > local");
        assert_eq!(err.code.as_deref(), Some("MIR014"));
        let text = format!("{err}");
        assert!(
            text.contains("param_count")
                && text.contains("local_count")
                && text.contains("C-3n"),
            "{text}"
        );
    }
}
