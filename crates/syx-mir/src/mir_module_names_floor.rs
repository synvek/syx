//! C-3aa: thin MIR module_names empty-string floor (fail-closed).
//! Not MIR008 rewrite · Not Call(ModuleId) IR · Not dense ModuleId
//! · Not expand pilot · Not C-3 `[x]`.
//!
//! Floor rules (MIR027):
//! - empty `module_names` → OK
//! - any entry `""` → MIR027
//! - (collect_module_names_from_paths already skips empty stems; this
//!   fail-closes hand-crafted / corrupt tables)

use crate::mir_diag::mir_error;
use crate::mir_types::MirModule;
use syx_diagnostics::SyxResult;

fn module_names_error(detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR module-names floor: {detail}; C-3aa"),
        "MIR027",
    )
}

/// C-3aa thin floor: every `module_names` entry must be non-empty; else MIR027.
pub fn verify_mir_module_names_floor(module: &MirModule) -> SyxResult<()> {
    for (index, name) in module.module_names.iter().enumerate() {
        if name.is_empty() {
            return Err(module_names_error(&format!(
                "empty module_names[{index}]"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;

    #[test]
    fn empty_table_ok() {
        verify_mir_module_names_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn non_empty_names_ok() {
        let mir = MirModule {
            module_names: vec![SmolStr::new("a"), SmolStr::new("b")],
            ..Default::default()
        };
        verify_mir_module_names_floor(&mir).expect("names");
    }

    #[test]
    fn empty_entry_mir027() {
        let mir = MirModule {
            module_names: vec![SmolStr::new("a"), SmolStr::new("")],
            ..Default::default()
        };
        let err = verify_mir_module_names_floor(&mir).expect_err("empty entry");
        assert_eq!(err.code.as_deref(), Some("MIR027"));
        let text = format!("{err}");
        assert!(
            text.contains("empty")
                && text.contains("C-3aa")
                && text.contains("module_names"),
            "{text}"
        );
    }
}
