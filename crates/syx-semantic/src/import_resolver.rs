use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::TypeId;

/// Cross-module symbol visibility (S005).
pub fn check_import_member_access(
    imported_modules: &[SmolStr],
    import_exports: &IndexMap<SmolStr, IndexMap<SmolStr, TypeId>>,
    module: &str,
    member: &str,
) -> SyxResult<()> {
    if !imported_modules.iter().any(|m| m.as_str() == module) {
        return Ok(());
    }
    let allowed = import_exports
        .get(module)
        .is_some_and(|exports| exports.contains_key(member));
    if !allowed {
        return Err(SyxDiagnostic::error(format!(
            "Cannot access non-public symbol '{module}.{member}'"
        ))
        .with_code("S005"));
    }
    Ok(())
}
