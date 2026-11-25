//! C-3s: thin MIR aot_export_roots name-resolve floor (fail-closed).
//! Not AOT shake rewrite · Not export GA / auto-discovery · Not ConstFnIndex.
//!
//! Floor rules (MIR019):
//! - empty `aot_export_roots` → OK (main-only / no export)
//! - each non-empty root name must exist in `functions[].name`

use crate::mir_diag::mir_error;
use crate::mir_types::MirModule;
use syx_diagnostics::SyxResult;

fn export_error(detail: &str) -> syx_diagnostics::SyxDiagnostic {
    mir_error(
        format!("MIR aot-export floor: {detail}; C-3s"),
        "MIR019",
    )
}

/// C-3s thin floor: non-empty `aot_export_roots` must resolve to `functions[].name`; else MIR019.
pub fn verify_mir_aot_export_floor(module: &MirModule) -> SyxResult<()> {
    if module.aot_export_roots.is_empty() {
        return Ok(());
    }
    for root in &module.aot_export_roots {
        if !module.functions.iter().any(|f| f.name == *root) {
            return Err(export_error(&format!(
                "aot_export_roots name '{root}' not in functions[].name"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_types::{MirFunction, MirInstr, MirOp};
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
    fn empty_roots_ok() {
        verify_mir_aot_export_floor(&MirModule::default()).expect("empty");
    }

    #[test]
    fn empty_roots_with_fns_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main")],
            aot_export_roots: vec![],
            ..Default::default()
        };
        verify_mir_aot_export_floor(&mir).expect("main-only");
    }

    #[test]
    fn resolvable_roots_ok() {
        let mir = MirModule {
            functions: vec![flat_fn("main"), flat_fn("greet")],
            aot_export_roots: vec![SmolStr::new("greet")],
            ..Default::default()
        };
        verify_mir_aot_export_floor(&mir).expect("resolvable");
    }

    #[test]
    fn unknown_root_mir019() {
        let mir = MirModule {
            functions: vec![flat_fn("main")],
            aot_export_roots: vec![SmolStr::new("missing")],
            ..Default::default()
        };
        let err = verify_mir_aot_export_floor(&mir).expect_err("unknown");
        assert_eq!(err.code.as_deref(), Some("MIR019"));
        let text = format!("{err}");
        assert!(
            text.contains("aot_export_roots")
                && text.contains("missing")
                && text.contains("C-3s")
                && text.contains("functions[].name"),
            "{text}"
        );
    }
}
