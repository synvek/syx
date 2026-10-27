//! Phase 56: freestanding macro roles (`@freestanding(...)`).

use smol_str::SmolStr;
use syx_ast::Attribute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreestandingRole {
    Expression,
    Statement,
    Declaration,
}

impl FreestandingRole {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "expression" => Some(Self::Expression),
            "statement" => Some(Self::Statement),
            "declaration" => Some(Self::Declaration),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expression => "expression",
            Self::Statement => "statement",
            Self::Declaration => "declaration",
        }
    }
}

/// Parse `@freestanding(expression|statement|declaration)` from macro attrs.
pub fn freestanding_roles_from_attrs(attrs: &[Attribute]) -> Vec<FreestandingRole> {
    let mut out = Vec::new();
    for attr in attrs {
        if attr.name.as_str() != "freestanding" {
            continue;
        }
        for arg in &attr.args {
            if let syx_ast::AttributeArg::Ident(name) = arg {
                if let Some(role) = FreestandingRole::from_str(name.as_str()) {
                    if !out.contains(&role) {
                        out.push(role);
                    }
                }
            }
        }
    }
    out
}

/// Default freestanding roles when none declared (legacy `#macro`).
pub fn default_freestanding_roles() -> Vec<FreestandingRole> {
    vec![FreestandingRole::Statement, FreestandingRole::Expression]
}

/// Phase 119 M9: empty roles means "no freestanding roles declared" — callers must fail closed.
pub fn macro_has_role(roles: &[FreestandingRole], role: FreestandingRole) -> bool {
    roles.contains(&role)
}

pub fn user_macro_roles(_name: &SmolStr, attrs: &[Attribute]) -> Vec<FreestandingRole> {
    let parsed = freestanding_roles_from_attrs(attrs);
    if parsed.is_empty() {
        // Legacy `#macro` without `@freestanding(...)` keeps statement+expression.
        // Explicit `@freestanding` with unrecognized/empty args yields empty → E003 at use site.
        if attrs.iter().any(|a| a.name.as_str() == "freestanding") {
            Vec::new()
        } else {
            default_freestanding_roles()
        }
    } else {
        parsed
    }
}

pub fn role_mismatch_diag(macro_name: &str, role: FreestandingRole) -> syx_diagnostics::SyxDiagnostic {
    syx_diagnostics::SyxDiagnostic::error(format!(
        "macro '#{macro_name}' is not a {} macro",
        role.as_str()
    ))
    .with_code("E003")
}
