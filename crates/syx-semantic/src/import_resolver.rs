use indexmap::IndexMap;
use smol_str::SmolStr;

use crate::TypeId;

/// Cross-module symbol visibility failure (mapped to S005 by SemanticAnalyzer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportAccessError {
    NonPublic { module: String, member: String },
}

/// Cross-module symbol visibility (S005).
pub fn check_import_member_access(
    imported_modules: &[SmolStr],
    import_exports: &IndexMap<SmolStr, IndexMap<SmolStr, TypeId>>,
    module: &str,
    member: &str,
) -> Result<(), ImportAccessError> {
    if !imported_modules.iter().any(|m| m.as_str() == module) {
        return Ok(());
    }
    let allowed = import_exports
        .get(module)
        .is_some_and(|exports| exports.contains_key(member));
    if !allowed {
        return Err(ImportAccessError::NonPublic {
            module: module.to_string(),
            member: member.to_string(),
        });
    }
    Ok(())
}
