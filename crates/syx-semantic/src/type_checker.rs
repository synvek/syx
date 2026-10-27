use syx_ast::Expr;
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_stdlib::{is_hashable_type, is_implicit_convertible, is_integer_narrowing};

use crate::{type_id_to_name, TypeId};

pub fn is_compatible(expected: &TypeId, found: &TypeId) -> bool {
    if expected == found {
        return true;
    }
    if let (TypeId::Numeric(to), TypeId::Numeric(from)) = (expected, found) {
        return is_implicit_convertible(*from, *to) || is_integer_narrowing(*from, *to);
    }
    match (expected, found) {
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
        (TypeId::Optional(inner), found) if is_compatible(inner, found) || inner.as_ref() == found => {
            true
        }
        (TypeId::Optional(_), TypeId::Unknown) => true,
        (TypeId::Optional(_), TypeId::Optional(inner)) if matches!(inner.as_ref(), TypeId::Unknown) => {
            true
        }
        (TypeId::Reference(exp), TypeId::Reference(found)) if is_compatible(exp, found) => true,
        (TypeId::Reference(exp), found) if is_compatible(exp, found) => true,
        (TypeId::MutReference(exp), TypeId::MutReference(found)) if is_compatible(exp, found) => true,
        (TypeId::MutReference(exp), found) if is_compatible(exp, found) => true,
        (TypeId::Tuple { elements: exp }, TypeId::Tuple { elements: found }) => {
            exp.len() == found.len()
                && exp
                    .iter()
                    .zip(found.iter())
                    .all(|((_, a), (_, b))| is_compatible(a, b))
        }
        (TypeId::Named(exp), TypeId::Named(found)) if exp.as_str() == "Character" && found.as_str() == "Character" => {
            true
        }
        _ => false,
    }
}

pub fn check_assignable(expected: TypeId, found: TypeId, name: &str) -> SyxResult<()> {
    if expected == found || is_compatible(&expected, &found) {
        Ok(())
    } else {
        Err(SyxDiagnostic::error(format!(
            "Type mismatch for '{name}': expected {}, found {}",
            type_id_to_name(&expected),
            type_id_to_name(&found)
        ))
        .with_code("S004"))
    }
}

pub fn check_assignable_with_literal(
    expected: TypeId,
    found: TypeId,
    name: &str,
    init: Option<&Expr>,
    check_literal_in_range: impl FnOnce(&syx_ast::Literal, &TypeId),
    push_error: impl FnOnce(SyxDiagnostic),
) -> SyxResult<()> {
    if let Some(Expr::Literal { value, .. }) = init {
        check_literal_in_range(value, &expected);
    }
    if matches!(expected, TypeId::Set(_)) {
        if let TypeId::Array(elem) = &found {
            let TypeId::Set(set_elem) = &expected else {
                return check_assignable(expected, found, name);
            };
            if !is_compatible(set_elem, elem) && !matches!(elem.as_ref(), TypeId::Unknown) {
                return check_assignable(expected, found, name);
            }
            let elem_name = type_id_to_name(set_elem);
            if !is_hashable_type(elem_name.as_str()) {
                push_error(SyxDiagnostic::error(format!(
                    "Set element type '{}' does not satisfy Hashable",
                    elem_name
                ))
                .with_code("S015"));
            }
            return Ok(());
        }
        if matches!(&found, TypeId::Named(n) if n.as_str() == "Set") {
            let TypeId::Set(set_elem) = &expected else {
                return check_assignable(expected, found, name);
            };
            let elem_name = type_id_to_name(set_elem);
            if !is_hashable_type(elem_name.as_str()) {
                push_error(SyxDiagnostic::error(format!(
                    "Set element type '{}' does not satisfy Hashable",
                    elem_name
                ))
                .with_code("S015"));
            }
            return Ok(());
        }
    }
    check_assignable(expected, found, name)
}
