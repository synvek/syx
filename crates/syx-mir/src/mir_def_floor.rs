//! C-3j: thin DefId floor — optional `MirFunction.owner_module` → MIR010.
//! Not a DefId graph · Not Call(ModuleId) IR · Not full MirOp verifier.

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn def_error(detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(format!("MIR def floor: {detail}; C-3j"), "MIR010")
}

/// Stable `(owner_module, name)` → first function index for owned defs.
/// Functions with `owner_module = None` are omitted (legacy / stdlib OK).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MirDefIndex {
    by_key: IndexMap<(u32, SmolStr), u32>,
}

impl MirDefIndex {
    /// Build from module; OOB owner or duplicate `(owner, name)` → **MIR010**.
    /// All-`None` / empty → empty index (OK).
    pub fn build(module: &MirModule) -> SyxResult<Self> {
        Self::build_from_functions(module.module_names.len(), module.functions.iter())
    }

    /// Same as [`Self::build`] over an explicit function list + `module_names` length.
    pub fn build_from_functions<'a>(
        module_name_count: usize,
        functions: impl IntoIterator<Item = &'a MirFunction>,
    ) -> SyxResult<Self> {
        let mut by_key: IndexMap<(u32, SmolStr), u32> = IndexMap::new();
        for (i, func) in functions.into_iter().enumerate() {
            let Some(owner) = func.owner_module else {
                continue;
            };
            if (owner as usize) >= module_name_count {
                return Err(def_error(&format!(
                    "owner_module ModuleId({owner}) out of range (module_names.len={module_name_count}) in '{}'",
                    func.name
                )));
            }
            let key = (owner, func.name.clone());
            if let Some(prev) = by_key.insert(key, i as u32) {
                return Err(def_error(&format!(
                    "duplicate def '{name}' under owner ModuleId({owner}) (prev fn#{prev}, new fn#{i})",
                    name = func.name
                )));
            }
        }
        Ok(Self { by_key })
    }

    pub fn resolve(&self, owner: u32, name: &SmolStr) -> Option<u32> {
        self.by_key.get(&(owner, name.clone())).copied()
    }

    pub fn len(&self) -> usize {
        self.by_key.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_key.is_empty()
    }
}

/// C-3j thin floor: owned defs must be in-range and unique per `(owner, name)`.
pub fn verify_mir_def_floor(module: &MirModule) -> SyxResult<()> {
    let _ = MirDefIndex::build(module)?;
    Ok(())
}

/// Same as [`verify_mir_def_floor`] over explicit functions + `module_names` length.
pub fn verify_mir_def_floor_functions<'a>(
    module_name_count: usize,
    functions: impl IntoIterator<Item = &'a MirFunction>,
) -> SyxResult<()> {
    let _ = MirDefIndex::build_from_functions(module_name_count, functions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirInstr, MirOp};

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
    fn all_none_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("a", None), flat_fn("b", None)],
            module_names: vec![SmolStr::new("m")],
            ..Default::default()
        };
        verify_mir_def_floor(&mir).expect("all None");
        assert!(MirDefIndex::build(&mir).unwrap().is_empty());
    }

    #[test]
    fn empty_ok() {
        let mir = MirModule::default();
        verify_mir_def_floor(&mir).expect("empty");
    }

    #[test]
    fn oob_owner_mir010() {
        let mir = MirModule {
            functions: vec![flat_fn("a", Some(0))],
            module_names: vec![],
            ..Default::default()
        };
        let err = verify_mir_def_floor(&mir).expect_err("OOB");
        assert_eq!(err.code.as_deref(), Some("MIR010"));
        let text = format!("{err}");
        assert!(text.contains("out of range") && text.contains("C-3j"), "{text}");
    }

    #[test]
    fn dup_owner_name_mir010() {
        let mir = MirModule {
            functions: vec![flat_fn("dup", Some(0)), flat_fn("dup", Some(0))],
            module_names: vec![SmolStr::new("m")],
            ..Default::default()
        };
        let err = verify_mir_def_floor(&mir).expect_err("dup");
        assert_eq!(err.code.as_deref(), Some("MIR010"));
        let text = format!("{err}");
        assert!(text.contains("duplicate") && text.contains("C-3j"), "{text}");
    }

    #[test]
    fn same_name_different_owners_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("f", Some(0)), flat_fn("f", Some(1))],
            module_names: vec![SmolStr::new("a"), SmolStr::new("b")],
            ..Default::default()
        };
        let idx = MirDefIndex::build(&mir).expect("ok");
        assert_eq!(idx.len(), 2);
    }
}
