//! C-3c: thin name → FuncId index (fail-closed). Not a ModuleId graph.

use crate::mir_diag::mir_error;
use crate::mir_types::{MirFunction, MirModule};
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

/// Dense function id: equals `MirModule.functions` index at build time.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FuncId(pub u32);

/// Stable name → [`FuncId`] table for a MIR module snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MirFnIndex {
    by_name: IndexMap<SmolStr, FuncId>,
}

impl MirFnIndex {
    /// Build from module function order. Duplicate names → MIR003.
    pub fn build(module: &MirModule) -> SyxResult<Self> {
        Self::build_from_functions(module.functions.iter())
    }

    /// Build from an ordered function list (e.g. post C-3a unique bodies).
    pub fn build_from_functions<'a>(
        functions: impl IntoIterator<Item = &'a MirFunction>,
    ) -> SyxResult<Self> {
        let mut by_name: IndexMap<SmolStr, FuncId> = IndexMap::new();
        for (i, func) in functions.into_iter().enumerate() {
            let id = FuncId(i as u32);
            if let Some(prev) = by_name.insert(func.name.clone(), id) {
                return Err(mir_error(
                    format!(
                        "MIR fn index: duplicate name '{}' (prev FuncId({}), new FuncId({})); C-3c",
                        func.name, prev.0, id.0
                    ),
                    "MIR003",
                ));
            }
        }
        Ok(Self { by_name })
    }

    /// Build from ordered names (e.g. `CompiledProgram.functions`).
    pub fn build_from_names<'a>(names: impl IntoIterator<Item = &'a SmolStr>) -> SyxResult<Self> {
        let mut by_name: IndexMap<SmolStr, FuncId> = IndexMap::new();
        for (i, name) in names.into_iter().enumerate() {
            let id = FuncId(i as u32);
            if let Some(prev) = by_name.insert(name.clone(), id) {
                return Err(mir_error(
                    format!(
                        "MIR fn index: duplicate name '{name}' (prev FuncId({}), new FuncId({})); C-3c",
                        prev.0, id.0
                    ),
                    "MIR003",
                ));
            }
        }
        Ok(Self { by_name })
    }

    pub fn resolve(&self, name: &SmolStr) -> Option<FuncId> {
        self.by_name.get(name).copied()
    }

    pub fn require(&self, name: &SmolStr) -> SyxResult<FuncId> {
        self.resolve(name).ok_or_else(|| {
            mir_error(
                format!("MIR fn index: unknown callee '{name}'; C-3c"),
                "MIR003",
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
    use crate::mir_types::{MirBlock, MirInstr, MirOp};

    fn mir_fn(name: &str) -> MirFunction {
        MirFunction {
            name: SmolStr::new(name),
            param_count: 0,
            local_count: 0,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                }],
                successors: vec![],
            }],
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
            owner_module: None,
        }
    }

    #[test]
    fn build_assigns_dense_ids() {
        let mir = MirModule {
            functions: vec![mir_fn("a"), mir_fn("b")],
            ..Default::default()
        };
        let idx = MirFnIndex::build(&mir).expect("build");
        assert_eq!(idx.require(&SmolStr::new("a")).unwrap(), FuncId(0));
        assert_eq!(idx.require(&SmolStr::new("b")).unwrap(), FuncId(1));
    }

    #[test]
    fn duplicate_name_fails_mir003() {
        let mir = MirModule {
            functions: vec![mir_fn("dup"), mir_fn("dup")],
            ..Default::default()
        };
        let err = MirFnIndex::build(&mir).expect_err("dup");
        assert_eq!(err.code.as_deref(), Some("MIR003"));
        assert!(format!("{err}").contains("duplicate name"));
    }

    #[test]
    fn unknown_callee_fails_mir003() {
        let mir = MirModule {
            functions: vec![mir_fn("only")],
            ..Default::default()
        };
        let idx = MirFnIndex::build(&mir).unwrap();
        let err = idx.require(&SmolStr::new("missing")).expect_err("missing");
        assert_eq!(err.code.as_deref(), Some("MIR003"));
        assert!(format!("{err}").contains("unknown callee"));
    }
}
