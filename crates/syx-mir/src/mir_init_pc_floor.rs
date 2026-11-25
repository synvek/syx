//! C-3t: thin MIR init_body_pc / init_wrap_pc bounds floor (fail-closed).
//! Not CallSuper / VM slice rewrite · Not `_init` name requirement · Not ConstFnIndex.
//!
//! Floor rules (MIR020):
//! - `init_body_pc == 0 && init_wrap_pc == 0` → OK
//! - otherwise → `init_body_pc ≤ init_wrap_pc ≤ instructions.len()`

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn init_pc_error(func_name: &SmolStr, detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR init-pc floor: {detail} in '{func_name}'; C-3t"),
        "MIR020",
    )
}

fn verify_function_init_pc_floor(func: &MirFunction) -> SyxResult<()> {
    let body = func.init_body_pc;
    let wrap = func.init_wrap_pc;
    if body == 0 && wrap == 0 {
        return Ok(());
    }
    let len = func.instructions.len() as u32;
    if body > wrap {
        return Err(init_pc_error(
            &func.name,
            &format!("init_body_pc ({body}) > init_wrap_pc ({wrap})"),
        ));
    }
    if wrap > len {
        return Err(init_pc_error(
            &func.name,
            &format!("init_wrap_pc ({wrap}) > instructions.len() ({len})"),
        ));
    }
    Ok(())
}

/// C-3t thin floor: non-zero init PCs must satisfy body ≤ wrap ≤ instructions.len(); else MIR020.
pub fn verify_mir_init_pc_floor(module: &MirModule) -> SyxResult<()> {
    verify_mir_init_pc_floor_functions(module.functions.iter())
}

/// Same as [`verify_mir_init_pc_floor`] over an explicit function list.
pub fn verify_mir_init_pc_floor_functions<'a>(
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    for func in functions {
        verify_function_init_pc_floor(func)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};

    fn flat_fn(name: &str, n_instr: usize, body: u32, wrap: u32) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![],
            instructions: (0..n_instr)
                .map(|_| MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                })
                .collect(),
            local_names: vec![],
            local_types: vec![],
            init_body_pc: body,
            init_wrap_pc: wrap,
            owner_module: None,
        }
    }

    #[test]
    fn empty_module_ok() {
        verify_mir_init_pc_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn dual_zero_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main", 3, 0, 0)],
            ..Default::default()
        };
        verify_mir_init_pc_floor(&mir).expect("dual zero");
    }

    #[test]
    fn ordered_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("Cls_init", 5, 1, 4)],
            ..Default::default()
        };
        verify_mir_init_pc_floor(&mir).expect("ordered");
    }

    #[test]
    fn body_gt_wrap_mir020() {
        let mir = MirModule {
            functions: vec![flat_fn("bad", 5, 3, 2)],
            ..Default::default()
        };
        let err = verify_mir_init_pc_floor(&mir).expect_err("body>wrap");
        assert_eq!(err.code.as_deref(), Some("MIR020"));
        let text = format!("{err}");
        assert!(
            text.contains("init_body_pc")
                && text.contains("init_wrap_pc")
                && text.contains("C-3t"),
            "{text}"
        );
    }

    #[test]
    fn wrap_gt_len_mir020() {
        let mir = MirModule {
            functions: vec![flat_fn("bad", 3, 1, 5)],
            ..Default::default()
        };
        let err = verify_mir_init_pc_floor(&mir).expect_err("wrap>len");
        assert_eq!(err.code.as_deref(), Some("MIR020"));
        assert!(format!("{err}").contains("instructions.len()"));
    }
}
