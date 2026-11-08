//! SYX semantic TypeId ↔ name / TypeExpr codec face.
//! Extracted in S2-I237 (zero semantics).

use crate::TypeId;
use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_ast::TypeExpr;
use syx_stdlib::{from_type_name, NumericKind};

pub fn type_name_to_id(name: &str) -> TypeId {
    if let Some(kind) = from_type_name(name) {
        return TypeId::numeric(kind);
    }
    if let Some(lanes) = simd_lanes_from_type_name(name) {
        return TypeId::Simd {
            lanes,
            element: NumericKind::I32,
        };
    }
    match name {
        "Bool" => TypeId::Bool,
        "String" => TypeId::String,
        "Void" => TypeId::Void,
        "Array" => TypeId::Array(Box::new(TypeId::Unknown)),
        "Set" => TypeId::Set(Box::new(TypeId::Unknown)),
        "Dictionary" => TypeId::Dictionary {
            key: Box::new(TypeId::String),
            value: Box::new(TypeId::Unknown),
        },
        "Object" => TypeId::Named(SmolStr::new("Object")),
        other => TypeId::Named(SmolStr::new(other)),
    }
}

pub fn type_expr_to_id(ty: &TypeExpr) -> TypeId {
    type_expr_to_id_with_aliases(ty, &IndexMap::new())
}

pub fn type_expr_to_id_with_aliases(
    ty: &TypeExpr,
    aliases: &IndexMap<SmolStr, TypeExpr>,
) -> TypeId {
    match ty {
        TypeExpr::Named { name, generics } => {
            if let Some(alias) = aliases.get(name) {
                return type_expr_to_id_with_aliases(alias, aliases);
            }
            if name.as_str() == "Character" {
                return TypeId::Named(SmolStr::new("Character"));
            }
            if name.as_str() == "Set" {
                let elem = generics
                    .first()
                    .map(type_expr_to_id)
                    .unwrap_or(TypeId::Unknown);
                return TypeId::Set(Box::new(elem));
            }
            if name.as_str() == "Array" {
                if generics.is_empty() {
                    return TypeId::Named(SmolStr::new("Array"));
                }
                let elem = type_expr_to_id_with_aliases(&generics[0], aliases);
                return TypeId::Array(Box::new(elem));
            }
            if name.as_str() == "Result" {
                let ok = generics
                    .first()
                    .map(type_expr_to_id)
                    .unwrap_or(TypeId::Unknown);
                let err = generics
                    .get(1)
                    .map(type_expr_to_id)
                    .unwrap_or(TypeId::Unknown);
                return TypeId::Result {
                    ok: Box::new(ok),
                    err: Box::new(err),
                };
            }
            if let Some(lanes) = simd_lanes_from_type_name(name.as_str()) {
                let element = generics
                    .first()
                    .and_then(|g| type_expr_to_id(g).as_numeric_kind())
                    .unwrap_or(NumericKind::I32);
                if !syx_config::simd_enabled() {
                    return TypeId::Unknown;
                }
                return TypeId::Simd { lanes, element };
            }
            if let Some(kind) = from_type_name(name) {
                return TypeId::numeric(kind);
            }
            match name.as_str() {
                "Bool" => TypeId::Bool,
                "String" => TypeId::String,
                "Void" => TypeId::Void,
                "Dictionary" => TypeId::Dictionary {
                    key: Box::new(
                        generics
                            .first()
                            .map(type_expr_to_id)
                            .unwrap_or(TypeId::String),
                    ),
                    value: Box::new(
                        generics
                            .get(1)
                            .map(type_expr_to_id)
                            .unwrap_or(TypeId::Unknown),
                    ),
                },
                other => {
                    let mangled = other.replace('.', "_");
                    TypeId::Named(SmolStr::new(mangled))
                }
            }
        }
        TypeExpr::Optional(inner) => TypeId::Optional(Box::new(type_expr_to_id(inner))),
        TypeExpr::Array(inner) => TypeId::Array(Box::new(type_expr_to_id(inner))),
        TypeExpr::Dictionary { key, value } => TypeId::Dictionary {
            key: Box::new(type_expr_to_id(key)),
            value: Box::new(type_expr_to_id(value)),
        },
        TypeExpr::Reference(inner, mutable, _lifetime) => {
            let inner_id = type_expr_to_id(inner);
            if *mutable {
                TypeId::MutReference(Box::new(inner_id))
            } else {
                TypeId::Reference(Box::new(inner_id))
            }
        }
        TypeExpr::Void => TypeId::Void,
        // S1-I4: function types `(T) -> U` / closures share a callable surface type.
        TypeExpr::Function { .. } => TypeId::Named(SmolStr::new("Closure")),
        TypeExpr::Tuple { elements } => TypeId::Tuple {
            elements: elements
                .iter()
                .map(|(label, ty)| (label.clone(), type_expr_to_id_with_aliases(ty, aliases)))
                .collect(),
        },
        TypeExpr::OpaqueSome(inner) => TypeId::OpaqueSome(Box::new(type_expr_to_id(inner))),
        TypeExpr::OpaqueAny(inner) => TypeId::OpaqueAny(Box::new(type_expr_to_id(inner))),
    }
}

/// Base collection type name for member dispatch (`Set`, `Dictionary`, `Array`).
pub fn type_id_collection_base(ty: &TypeId) -> Option<&'static str> {
    match ty {
        TypeId::Set(_) => Some("Set"),
        TypeId::Dictionary { .. } => Some("Dictionary"),
        TypeId::Array(_) => Some("Array"),
        _ => None,
    }
}

/// Base type name for extension / subscript dispatch (`Array<Int>` → `Array`).
pub fn type_name_dispatch_base(name: &SmolStr) -> SmolStr {
    SmolStr::new(name.split('<').next().unwrap_or(name.as_str()))
}

pub fn normalize_type_arg_name(name: SmolStr) -> SmolStr {
    match name.as_str() {
        "Int" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "Int64"
        } else {
            "Int32"
        }),
        "UInt" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "UInt64"
        } else {
            "UInt32"
        }),
        other => SmolStr::new(other),
    }
}

pub use syx_mono::{demangle_mono_name, is_mono_type_arg_segment, user_facing_runtime_type_name};

/// User-facing type display (S3-I43). Same surface as [`type_id_to_name`].
pub fn user_facing_type_name(ty: &TypeId) -> SmolStr {
    type_id_to_name(ty)
}

/// User-facing MIR / DAP function name (S3-I44).
///
/// - `Greeter_hello` → `Greeter.hello`
/// - `Foo_Int_bar` → `Foo<Int>.bar` (class demangled via [`demangle_mono_name`])
/// - `Greeter_init` → `Greeter.init`
/// - monomorphized free fn `Foo_Int` → `Foo<Int>`（not `Foo.Int`）
/// - otherwise → [`demangle_mono_name`]
pub fn user_facing_mir_fn_name(raw: &str) -> SmolStr {
    if let Some(base) = raw.strip_suffix("_init") {
        return SmolStr::new(format!("{}.init", demangle_mono_name(base)));
    }
    if let Some((class, method)) = syx_ast::parse_mangled_instance_method(raw) {
        // Only treat as Class_method when the RHS is not a mono type-arg suffix.
        if raw.contains('_') && !syx_mono::is_mono_type_arg_segment(method.as_str()) {
            return SmolStr::new(format!(
                "{}.{}",
                demangle_mono_name(class.as_str()),
                method
            ));
        }
    }
    demangle_mono_name(raw)
}

/// IR / codegen identity name: `Named` stays raw (may be mangled). Do **not** demangle.
pub fn type_id_to_ir_name(ty: &TypeId) -> SmolStr {
    type_id_to_name_core(ty, /* demangle_named */ false)
}

fn type_id_to_name_core(ty: &TypeId, demangle_named: bool) -> SmolStr {
    match ty {
        TypeId::Void => SmolStr::new("Void"),
        TypeId::Numeric(kind) => SmolStr::new(syx_stdlib::type_name_with_alias(*kind)),
        TypeId::Bool => SmolStr::new("Bool"),
        TypeId::String => SmolStr::new("String"),
        TypeId::Named(n) => {
            if demangle_named {
                demangle_mono_name(n.as_str())
            } else {
                n.clone()
            }
        }
        TypeId::Array(inner) => SmolStr::new(format!(
            "Array<{}>",
            type_id_to_name_core(inner, demangle_named)
        )),
        TypeId::Set(inner) => SmolStr::new(format!(
            "Set<{}>",
            type_id_to_name_core(inner, demangle_named)
        )),
        TypeId::Dictionary { key, value } => SmolStr::new(format!(
            "Dictionary<{}, {}>",
            type_id_to_name_core(key, demangle_named),
            type_id_to_name_core(value, demangle_named)
        )),
        TypeId::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(|(label, ty)| {
                    if let Some(l) = label {
                        format!("{l}: {}", type_id_to_name_core(ty, demangle_named))
                    } else {
                        type_id_to_name_core(ty, demangle_named).to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            SmolStr::new(format!("({inner})"))
        }
        TypeId::Optional(inner) => SmolStr::new(format!(
            "{}?",
            type_id_to_name_core(inner, demangle_named)
        )),
        TypeId::Result { ok, err } => SmolStr::new(format!(
            "Result<{}, {}>",
            type_id_to_name_core(ok, demangle_named),
            type_id_to_name_core(err, demangle_named)
        )),
        TypeId::GenericParam(n) => n.clone(),
        TypeId::GenericFn { name, .. } => name.clone(),
        TypeId::Reference(inner) => SmolStr::new(format!(
            "&{}",
            type_id_to_name_core(inner, demangle_named)
        )),
        TypeId::MutReference(inner) => SmolStr::new(format!(
            "&mut {}",
            type_id_to_name_core(inner, demangle_named)
        )),
        TypeId::OpaqueSome(inner) => SmolStr::new(format!(
            "some {}",
            type_id_to_name_core(inner, demangle_named)
        )),
        TypeId::OpaqueAny(inner) => SmolStr::new(format!(
            "any {}",
            type_id_to_name_core(inner, demangle_named)
        )),
        TypeId::Simd { lanes, element } => SmolStr::new(format!(
            "SIMD{lanes}<{}>",
            syx_stdlib::type_name_with_alias(*element)
        )),
        TypeId::Unknown => SmolStr::new("Unknown"),
    }
}

/// User-facing type name (demangles monomorph `Named` forms). Prefer this / call sites of diagnostics.
pub fn type_id_to_name(ty: &TypeId) -> SmolStr {
    type_id_to_name_core(ty, /* demangle_named */ true)
}

pub fn simd_lanes_from_type_name(name: &str) -> Option<u8> {
    match name {
        "SIMD2" => Some(2),
        "SIMD4" => Some(4),
        "SIMD8" => Some(8),
        "SIMD16" => Some(16),
        _ => None,
    }
}

#[cfg(test)]
mod demangle_tests {
    use super::*;

    #[test]
    fn demangle_mono_generic_form() {
        assert_eq!(demangle_mono_name("Foo_Int").as_str(), "Foo<Int>");
        assert_eq!(
            demangle_mono_name("Pair_Int_String").as_str(),
            "Pair<Int, String>"
        );
        assert_eq!(demangle_mono_name("Foo").as_str(), "Foo");
    }

    #[test]
    fn demangle_nested_dotted_heuristic() {
        assert_eq!(demangle_mono_name("Outer_Inner").as_str(), "Outer.Inner");
    }

    #[test]
    fn type_id_to_name_demangles_named() {
        let ty = TypeId::Named(SmolStr::new("Foo_Int"));
        assert_eq!(type_id_to_name(&ty).as_str(), "Foo<Int>");
        assert_eq!(user_facing_type_name(&ty).as_str(), "Foo<Int>");
        assert_eq!(type_id_to_ir_name(&ty).as_str(), "Foo_Int");
    }

    #[test]
    fn user_facing_mir_fn_name_instance_and_init() {
        assert_eq!(
            user_facing_mir_fn_name("Greeter_hello").as_str(),
            "Greeter.hello"
        );
        assert_eq!(
            user_facing_mir_fn_name("Greeter_init").as_str(),
            "Greeter.init"
        );
        assert_eq!(
            user_facing_mir_fn_name("Foo_Int_bar").as_str(),
            "Foo<Int>.bar"
        );
        assert_eq!(user_facing_mir_fn_name("Foo_Int").as_str(), "Foo<Int>");
        assert_eq!(user_facing_mir_fn_name("main").as_str(), "main");
    }
}
