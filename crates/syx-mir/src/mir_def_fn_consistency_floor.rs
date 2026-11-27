//! C-3cf: thin MirDefIndex ↔ MirFnIndex consistency floor (fail-closed).
//! Not Call(ModuleId) IR · Not full DefId graph · Not multi-module required owner
//! · Not HostFn arity · Not C-3 `[x]`.
//!
//! Floor rules (MIR082):
//! - build [`MirDefIndex`] from the function list (OOB/dup still MIR010 via build)
//! - for each `owner_module=Some(o)` function at index `i` named `N`:
//!   - [`MirFnIndex`] must resolve `N` → FuncId(`i`); else MIR082
//!   - [`MirDefIndex`] must resolve `(o, N)` → `i`; else MIR082
//! - all-`None` / empty / no owned defs → OK

use crate::mir_def_floor::MirDefIndex;
use crate::mir_diag::mir_error;
use crate::mir_fn_index::{FuncId, MirFnIndex};
use crate::mir_types::{MirFunction, MirModule};
use syx_diagnostics::SyxResult;

fn consistency_error(detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR def-fn consistency floor: {detail}; C-3cf"),
        "MIR082",
    )
}

/// C-3cf thin floor: owned defs must agree with [`MirFnIndex`] same-name same-slot.
pub fn verify_mir_def_fn_consistency_floor(module: &MirModule) -> SyxResult<()> {
    let index = MirFnIndex::build(module)?;
    verify_mir_def_fn_consistency_floor_functions(
        &index,
        module.module_names.len(),
        module.functions.iter(),
    )
}

/// Same as [`verify_mir_def_fn_consistency_floor`] over an explicit index + function list.
pub fn verify_mir_def_fn_consistency_floor_functions<'a>(
    index: &MirFnIndex,
    module_name_count: usize,
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let funcs: Vec<&MirFunction> = functions.into_iter().collect();
    let def_index = MirDefIndex::build_from_functions(module_name_count, funcs.iter().copied())?;
    for (i, func) in funcs.iter().enumerate() {
        let Some(owner) = func.owner_module else {
            continue;
        };
        let i_u = i as u32;
        match index.resolve(&func.name) {
            Some(FuncId(id)) if id == i_u => {}
            Some(FuncId(id)) => {
                return Err(consistency_error(&format!(
                    "owned def '{}' ModuleId({owner}) at fn#{i_u} but MirFnIndex resolves to FuncId({id})",
                    func.name
                )));
            }
            None => {
                return Err(consistency_error(&format!(
                    "owned def '{}' ModuleId({owner}) at fn#{i_u} missing from MirFnIndex",
                    func.name
                )));
            }
        }
        match def_index.resolve(owner, &func.name) {
            Some(id) if id == i_u => {}
            Some(id) => {
                return Err(consistency_error(&format!(
                    "owned def '{}' ModuleId({owner}) at fn#{i_u} but MirDefIndex resolves to fn#{id}",
                    func.name
                )));
            }
            None => {
                return Err(consistency_error(&format!(
                    "owned def '{}' ModuleId({owner}) at fn#{i_u} missing from MirDefIndex",
                    func.name
                )));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};
    use smol_str::SmolStr;

    fn flat_fn(name: &str, owner: Option<u32>) -> MirFunction {
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
            owner_module: owner,
        }
    }

    #[test]
    fn empty_ok() {
        verify_mir_def_fn_consistency_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn all_none_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("a", None), flat_fn("b", None)],
            module_names: vec![SmolStr::new("m")],
            ..Default::default()
        };
        verify_mir_def_fn_consistency_floor(&mir).expect("all None");
    }

    #[test]
    fn owned_matching_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main", Some(0)), flat_fn("helper", Some(0))],
            module_names: vec![SmolStr::new("app")],
            ..Default::default()
        };
        verify_mir_def_fn_consistency_floor(&mir).expect("owned ok");
    }

    #[test]
    fn stale_fn_index_missing_mir082() {
        let mir = MirModule {
            functions: vec![flat_fn("owned", Some(0))],
            module_names: vec![SmolStr::new("app")],
            ..Default::default()
        };
        // Empty / unrelated FnIndex → owned name missing.
        let stale = MirFnIndex::build_from_names([SmolStr::new("other")].iter()).expect("stale");
        let err = verify_mir_def_fn_consistency_floor_functions(
            &stale,
            mir.module_names.len(),
            mir.functions.iter(),
        )
        .expect_err("missing");
        assert_eq!(err.code.as_deref(), Some("MIR082"));
        let text = format!("{err}");
        assert!(
            text.contains("missing from MirFnIndex") && text.contains("C-3cf"),
            "{text}"
        );
    }

    #[test]
    fn stale_fn_index_wrong_slot_mir082() {
        // Two names in FnIndex, but owned function list order differs from index slots.
        let funcs = vec![flat_fn("a", Some(0)), flat_fn("b", Some(0))];
        // Build FnIndex as if order were b, a → a resolves to FuncId(1) not 0.
        let swapped = MirFnIndex::build_from_names(
            [SmolStr::new("b"), SmolStr::new("a")].iter(),
        )
        .expect("swapped");
        let err = verify_mir_def_fn_consistency_floor_functions(&swapped, 1, funcs.iter())
            .expect_err("slot");
        assert_eq!(err.code.as_deref(), Some("MIR082"));
        let text = format!("{err}");
        assert!(
            text.contains("MirFnIndex resolves to FuncId") && text.contains("C-3cf"),
            "{text}"
        );
    }

    #[test]
    fn oob_owner_still_mir010() {
        let mir = MirModule {
            functions: vec![flat_fn("a", Some(0))],
            module_names: vec![],
            ..Default::default()
        };
        let err = verify_mir_def_fn_consistency_floor(&mir).expect_err("OOB");
        assert_eq!(err.code.as_deref(), Some("MIR010"));
    }
}
