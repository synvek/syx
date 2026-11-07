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

pub fn type_id_to_name(ty: &TypeId) -> SmolStr {
    match ty {
        TypeId::Void => SmolStr::new("Void"),
        TypeId::Numeric(kind) => SmolStr::new(syx_stdlib::type_name_with_alias(*kind)),
        TypeId::Bool => SmolStr::new("Bool"),
        TypeId::String => SmolStr::new("String"),
        TypeId::Named(n) => n.clone(),
        TypeId::Array(inner) => SmolStr::new(format!("Array<{}>", type_id_to_name(inner))),
        TypeId::Set(inner) => SmolStr::new(format!("Set<{}>", type_id_to_name(inner))),
        TypeId::Dictionary { key, value } => SmolStr::new(format!(
            "Dictionary<{}, {}>",
            type_id_to_name(key),
            type_id_to_name(value)
        )),
        TypeId::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(|(label, ty)| {
                    if let Some(l) = label {
                        format!("{l}: {}", type_id_to_name(ty))
                    } else {
                        type_id_to_name(ty).to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            SmolStr::new(format!("({inner})"))
        }
        TypeId::Optional(inner) => SmolStr::new(format!("{}?", type_id_to_name(inner))),
        TypeId::Result { ok, err } => SmolStr::new(format!(
            "Result<{}, {}>",
            type_id_to_name(ok),
            type_id_to_name(err)
        )),
        TypeId::GenericParam(n) => n.clone(),
        TypeId::GenericFn { name, .. } => name.clone(),
        TypeId::Reference(inner) => SmolStr::new(format!("&{}", type_id_to_name(inner))),
        TypeId::MutReference(inner) => SmolStr::new(format!("&mut {}", type_id_to_name(inner))),
        TypeId::OpaqueSome(inner) => {
            SmolStr::new(format!("some {}", type_id_to_name(inner)))
        }
        TypeId::OpaqueAny(inner) => SmolStr::new(format!("any {}", type_id_to_name(inner))),
        TypeId::Simd { lanes, element } => SmolStr::new(format!(
            "SIMD{lanes}<{}>",
            syx_stdlib::type_name_with_alias(*element)
        )),
        TypeId::Unknown => SmolStr::new("Unknown"),
    }
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
