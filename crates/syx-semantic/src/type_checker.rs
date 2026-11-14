use syx_stdlib::{is_implicit_convertible, is_integer_narrowing};

use crate::{type_id_to_diag_name, TypeId};

pub fn is_compatible(expected: &TypeId, found: &TypeId) -> bool {
    if expected == found {
        return true;
    }
    if let (TypeId::Numeric(to), TypeId::Numeric(from)) = (expected, found) {
        return is_implicit_convertible(*from, *to) || is_integer_narrowing(*from, *to);
    }
    match (expected, found) {
        // Soft / gradual: Unknown found is inference noise (Object fields, host
        // returns). Needed after R-34 started assign-checking call args — without
        // this, soga/sgate `jStr(v: Any)` / `encodeJson(value: Object)` / Int
        // params reject `_` and hard-fail tools that previously type-checked green.
        (_, TypeId::Unknown) => true,
        // `Any` is the unconstrained top type (soga/sgate `jStr(v: Any)`, books).
        (TypeId::Named(n), _) if n.as_str() == "Any" => true,
        // `Object` JSON/dynamic bag accepts Dictionary (typed bag shares Object ops).
        (TypeId::Named(n), TypeId::Dictionary { .. }) if n.as_str() == "Object" => true,
        // Empty array literal `[]` infers `Array<Unknown>`; allow it to
        // initialize any `Array<T>` (same Unknown-elem carve-out as Set).
        (TypeId::Array(expected_elem), TypeId::Array(found_elem)) => {
            is_compatible(expected_elem, found_elem)
                || matches!(found_elem.as_ref(), TypeId::Unknown)
        }
        (TypeId::Set(expected_elem), TypeId::Set(found_elem)) => {
            is_compatible(expected_elem, found_elem)
                || matches!(found_elem.as_ref(), TypeId::Unknown)
        }
        (TypeId::Set(expected_elem), TypeId::Array(found_elem)) => {
            is_compatible(expected_elem, found_elem)
                || matches!(found_elem.as_ref(), TypeId::Unknown)
        }
        // Bare `Set()` constructor types as `Named("Set")`; allow under `Set<T>`.
        (TypeId::Set(_), TypeId::Named(n)) if n.as_str() == "Set" => true,
        (TypeId::Optional(inner), found)
            if is_compatible(inner, found) || inner.as_ref() == found =>
        {
            true
        }
        // Unknown found already matched above.
        (TypeId::Optional(_), TypeId::Optional(inner))
            if matches!(inner.as_ref(), TypeId::Unknown) =>
        {
            true
        }
        (TypeId::Reference(exp), TypeId::Reference(found)) if is_compatible(exp, found) => true,
        (TypeId::Reference(exp), found) if is_compatible(exp, found) => true,
        (TypeId::MutReference(exp), TypeId::MutReference(found)) if is_compatible(exp, found) => {
            true
        }
        (TypeId::MutReference(exp), found) if is_compatible(exp, found) => true,
        (TypeId::Tuple { elements: exp }, TypeId::Tuple { elements: found }) => {
            exp.len() == found.len()
                && exp
                    .iter()
                    .zip(found.iter())
                    .all(|((_, a), (_, b))| is_compatible(a, b))
        }
        (TypeId::Named(exp), TypeId::Named(found))
            if exp.as_str() == "Character" && found.as_str() == "Character" =>
        {
            true
        }
        _ => false,
    }
}

/// Assignability failure (mapped to a span-aware diagnostic by SemanticAnalyzer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignError {
    pub message: String,
    pub code: &'static str,
}

pub fn check_assignable(expected: TypeId, found: TypeId, name: &str) -> Result<(), AssignError> {
    if expected == found || is_compatible(&expected, &found) {
        return Ok(());
    }
    // R-36: Unknown is inference noise — covered by is_compatible(_, Unknown).
    Err(AssignError {
        message: format!(
            "Type mismatch for '{name}': expected {}, found {}",
            type_id_to_diag_name(&expected),
            type_id_to_diag_name(&found)
        ),
        code: "S004",
    })
}
