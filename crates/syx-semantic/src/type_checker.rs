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
        // C-0a+3: only InferenceVar remains soft/gradual. Bare `Unknown` is no
        // longer a global escape. ErrorType never soft-compatible (falls to `_`).
        // HostFn found-Object soft arm below stays until a façade honesty window.
        (_, TypeId::InferenceVar) => true,
        // C-0b fallout: many HostFn tables still surface `Object` where façades
        // declare Bool/Int/…; treat found-Object like Unknown until HostFn façade window.
        (_, TypeId::Named(n)) if n.as_str() == "Object" => true,
        // `Any` is the unconstrained top type (soga/sgate `jStr(v: Any)`, books).
        (TypeId::Named(n), _) if n.as_str() == "Any" => true,
        // `Object` JSON/dynamic bag accepts Dictionary (typed bag shares Object ops).
        (TypeId::Named(n), TypeId::Dictionary { .. }) if n.as_str() == "Object" => true,
        // Empty array literal `[]` infers `Array<Unknown|InferenceVar>`; allow it to
        // initialize any `Array<T>` (same Unknown-elem carve-out as Set).
        (TypeId::Array(expected_elem), TypeId::Array(found_elem)) => {
            is_compatible(expected_elem, found_elem) || found_elem.is_inference_hole()
        }
        (TypeId::Set(expected_elem), TypeId::Set(found_elem)) => {
            is_compatible(expected_elem, found_elem) || found_elem.is_inference_hole()
        }
        (TypeId::Set(expected_elem), TypeId::Array(found_elem)) => {
            is_compatible(expected_elem, found_elem) || found_elem.is_inference_hole()
        }
        // Bare `Set()` constructor types as `Named("Set")`; allow under `Set<T>`.
        (TypeId::Set(_), TypeId::Named(n)) if n.as_str() == "Set" => true,
        // C-0b fallout: stdlib façades declare bare `Array` / `Dictionary` while
        // host returns `Array<T>` / typed Dictionary — allow both directions.
        (TypeId::Named(n), TypeId::Array(_)) if n.as_str() == "Array" => true,
        (TypeId::Array(_), TypeId::Named(n)) if n.as_str() == "Array" => true,
        (TypeId::Named(n), TypeId::Dictionary { .. }) if n.as_str() == "Dictionary" => true,
        (TypeId::Dictionary { .. }, TypeId::Named(n)) if n.as_str() == "Dictionary" => true,
        (TypeId::Optional(inner), found)
            if is_compatible(inner, found) || inner.as_ref() == found =>
        {
            true
        }
        // Inference hole found already matched above for bare; Optional<_> carve-out.
        (TypeId::Optional(_), TypeId::Optional(inner)) if inner.is_inference_hole() => true,
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
    // R-36 / C-0a+: inference holes covered by soft arms; ErrorType rejects here.
    Err(AssignError {
        message: format!(
            "Type mismatch for '{name}': expected {}, found {}",
            type_id_to_diag_name(&expected),
            type_id_to_diag_name(&found)
        ),
        code: "S004",
    })
}


#[cfg(test)]
mod c0ap_tests {
    use super::*;
    use syx_stdlib::NumericKind;

    #[test]
    fn soft_arm_inference_var_only_unknown_closed() {
        let int = TypeId::Numeric(NumericKind::Int);
        assert!(is_compatible(&int, &TypeId::InferenceVar));
        assert!(!is_compatible(&int, &TypeId::Unknown));
        assert!(!is_compatible(&int, &TypeId::ErrorType));
        // HostFn Object soft arm still open.
        assert!(is_compatible(
            &int,
            &TypeId::Named(smol_str::SmolStr::new("Object"))
        ));
    }
}
