//! C-3ac: thin MIR aot_export_roots uniqueness floor (fail-closed).
//! Not MIR019 rewrite · Not export GA / auto-discovery · Not ConstFnIndex
//! · Not expand pilot · Not C-3 `[x]`.
//!
//! Floor rules (MIR029):
//! - empty `aot_export_roots` → OK
//! - otherwise unique root names (`IndexSet` len == roots.len())
//! - (lower already dedupes; this fail-closes hand-crafted dups)

use crate::mir_diag::mir_error;
use crate::mir_types::MirModule;
use indexmap::IndexSet;
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

fn export_unique_error(detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR aot-export-unique floor: {detail}; C-3ac"),
        "MIR029",
    )
}

/// C-3ac thin floor: `aot_export_roots` names must be unique; else MIR029.
pub fn verify_mir_aot_export_unique_floor(module: &MirModule) -> SyxResult<()> {
    if module.aot_export_roots.is_empty() {
        return Ok(());
    }
    let uniq: IndexSet<&SmolStr> = module.aot_export_roots.iter().collect();
    if uniq.len() != module.aot_export_roots.len() {
        return Err(export_unique_error(&format!(
            "duplicate aot_export_roots (unique {}, len {})",
            uniq.len(),
            module.aot_export_roots.len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirFunction, MirInstr, MirOp};

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
    fn empty_roots_ok() {
        verify_mir_aot_export_unique_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn unique_roots_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main"), flat_fn("greet")],
            aot_export_roots: vec![SmolStr::new("greet"), SmolStr::new("main")],
            ..Default::default()
        };
        verify_mir_aot_export_unique_floor(&mir).expect("unique");
    }

    #[test]
    fn duplicate_roots_mir029() {
        let mir = MirModule {
            functions: vec![flat_fn("main"), flat_fn("greet")],
            aot_export_roots: vec![SmolStr::new("greet"), SmolStr::new("greet")],
            ..Default::default()
        };
        let err = verify_mir_aot_export_unique_floor(&mir).expect_err("dup");
        assert_eq!(err.code.as_deref(), Some("MIR029"));
        let text = format!("{err}");
        assert!(
            text.contains("duplicate")
                && text.contains("C-3ac")
                && text.contains("aot_export_roots"),
            "{text}"
        );
    }
}
