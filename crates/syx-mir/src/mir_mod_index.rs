//! C-3h: thin module name → ModuleId index (fail-closed).
//! Not a DefId graph · Not full MirOp verifier · Not Call(ModuleId) IR.

use crate::mir_diag::mir_error;
use crate::mir_types::MirModule;
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::path::{Path, PathBuf};
use syx_diagnostics::SyxResult;

/// Ordered unique path stems for [`MirModule::module_names`].
/// Same stem → different path → **MIR008** (fail-closed).
pub fn collect_module_names_from_paths<'a>(
    paths: impl IntoIterator<Item = &'a Path>,
) -> SyxResult<Vec<SmolStr>> {
    let mut first_path: IndexMap<SmolStr, PathBuf> = IndexMap::new();
    let mut names = Vec::new();
    for path in paths {
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if stem.is_empty() {
            continue;
        }
        let name = SmolStr::new(stem);
        if let Some(prev) = first_path.get(&name) {
            if prev != path {
                return Err(mir_error(
                    format!(
                        "MIR mod index: duplicate module stem '{name}' (paths '{}' vs '{}'); C-3h",
                        prev.display(),
                        path.display()
                    ),
                    "MIR008",
                ));
            }
            continue;
        }
        first_path.insert(name.clone(), path.to_path_buf());
        names.push(name);
    }
    Ok(names)
}

/// Dense module id: equals `MirModule.module_names` index at build time.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ModuleId(pub u32);

/// Stable module-name → [`ModuleId`] table for a MIR module snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MirModIndex {
    by_name: IndexMap<SmolStr, ModuleId>,
}

impl MirModIndex {
    /// Build from [`MirModule::module_names`] order. Duplicate names → MIR008.
    pub fn build(module: &MirModule) -> SyxResult<Self> {
        Self::build_from_names(module.module_names.iter())
    }

    /// Build from ordered module names (e.g. graph stems). Empty → empty index (OK).
    pub fn build_from_names<'a>(names: impl IntoIterator<Item = &'a SmolStr>) -> SyxResult<Self> {
        let mut by_name: IndexMap<SmolStr, ModuleId> = IndexMap::new();
        for (i, name) in names.into_iter().enumerate() {
            let id = ModuleId(i as u32);
            if let Some(prev) = by_name.insert(name.clone(), id) {
                return Err(mir_error(
                    format!(
                        "MIR mod index: duplicate name '{name}' (prev ModuleId({}), new ModuleId({})); C-3h",
                        prev.0, id.0
                    ),
                    "MIR008",
                ));
            }
        }
        Ok(Self { by_name })
    }

    pub fn resolve(&self, name: &SmolStr) -> Option<ModuleId> {
        self.by_name.get(name).copied()
    }

    pub fn require(&self, name: &SmolStr) -> SyxResult<ModuleId> {
        self.resolve(name).ok_or_else(|| {
            mir_error(
                format!("MIR mod index: unknown module '{name}'; C-3h"),
                "MIR008",
            )
        })
    }

    pub fn len(&self) -> usize {
        self.by_name.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_assigns_dense_ids() {
        let mir = MirModule {
            module_names: vec![SmolStr::new("a"), SmolStr::new("b")],
            ..Default::default()
        };
        let idx = MirModIndex::build(&mir).expect("build");
        assert_eq!(idx.require(&SmolStr::new("a")).unwrap(), ModuleId(0));
        assert_eq!(idx.require(&SmolStr::new("b")).unwrap(), ModuleId(1));
    }

    #[test]
    fn empty_names_ok() {
        let mir = MirModule::default();
        let idx = MirModIndex::build(&mir).expect("empty");
        assert!(idx.is_empty());
    }

    #[test]
    fn duplicate_name_fails_mir008() {
        let mir = MirModule {
            module_names: vec![SmolStr::new("dup"), SmolStr::new("dup")],
            ..Default::default()
        };
        let err = MirModIndex::build(&mir).expect_err("dup");
        assert_eq!(err.code.as_deref(), Some("MIR008"));
        assert!(format!("{err}").contains("duplicate name"));
    }

    #[test]
    fn unknown_module_fails_mir008() {
        let mir = MirModule {
            module_names: vec![SmolStr::new("only")],
            ..Default::default()
        };
        let idx = MirModIndex::build(&mir).unwrap();
        let err = idx.require(&SmolStr::new("missing")).expect_err("missing");
        assert_eq!(err.code.as_deref(), Some("MIR008"));
        assert!(format!("{err}").contains("unknown module"));
    }
}
