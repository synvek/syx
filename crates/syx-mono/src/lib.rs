use std::collections::HashSet;

use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_ast::{Decl, Module, TypeExpr, WherePred};

pub fn mangle_name(base: &str, type_args: &[SmolStr]) -> SmolStr {
    if type_args.is_empty() {
        return SmolStr::new(base);
    }
    let suffix: Vec<_> = type_args.iter().map(|t| t.as_str()).collect();
    SmolStr::new(format!("{base}_{}", suffix.join("_")))
}

fn substitute_type_expr(ty: &TypeExpr, params: &[SmolStr], args: &[SmolStr]) -> TypeExpr {
    match ty {
        TypeExpr::Named { name, generics } => {
            if generics.is_empty() {
                if let Some(idx) = params.iter().position(|p| p == name) {
                    if let Some(arg) = args.get(idx) {
                        return TypeExpr::Named {
                            name: arg.clone(),
                            generics: vec![],
                        };
                    }
                }
                TypeExpr::Named {
                    name: name.clone(),
                    generics: vec![],
                }
            } else {
                TypeExpr::Named {
                    name: name.clone(),
                    generics: generics
                        .iter()
                        .map(|g| substitute_type_expr(g, params, args))
                        .collect(),
                }
            }
        }
        TypeExpr::Optional(inner) => {
            TypeExpr::Optional(Box::new(substitute_type_expr(inner, params, args)))
        }
        TypeExpr::Array(inner) => {
            TypeExpr::Array(Box::new(substitute_type_expr(inner, params, args)))
        }
        TypeExpr::Dictionary { key, value } => TypeExpr::Dictionary {
            key: Box::new(substitute_type_expr(key, params, args)),
            value: Box::new(substitute_type_expr(value, params, args)),
        },
        TypeExpr::Reference(inner, mutable, lifetime) => TypeExpr::Reference(
            Box::new(substitute_type_expr(inner, params, args)),
            *mutable,
            lifetime.clone(),
        ),
        TypeExpr::OpaqueSome(inner) => TypeExpr::OpaqueSome(Box::new(substitute_type_expr(
            inner, params, args,
        ))),
        TypeExpr::OpaqueAny(inner) => TypeExpr::OpaqueAny(Box::new(substitute_type_expr(
            inner, params, args,
        ))),
        other => other.clone(),
    }
}

fn class_static_generic_template(class_name: &SmolStr, member: &Decl) -> Option<(SmolStr, Decl)> {
    let Decl::Function {
        span,
        access,
        attrs,
        name: method,
        type_params,
        params,
        return_type,
        throws_type,
        effect,
        where_clause,
        body,
        is_async,
        ..
    } = member
    else {
        return None;
    };
    if type_params.is_empty() || !attrs.iter().any(|a| a.name.as_str() == "static") {
        return None;
    }
    let qname = SmolStr::new(format!("{class_name}_{method}"));
    Some((
        qname.clone(),
        Decl::Function {
            span: *span,
            access: *access,
            attrs: attrs.clone(),
            name: qname,
            type_params: type_params.clone(),
            params: params.clone(),
            return_type: return_type.clone(),
            throws_type: throws_type.clone(),
            effect: *effect,
            where_clause: where_clause.clone(),
            body: body.clone(),
            is_async: *is_async,
            is_override: false,
            is_final: false,
        },
    ))
}

fn collect_generic_templates(module: &Module) -> IndexMap<SmolStr, Decl> {
    let mut templates = IndexMap::new();
    for decl in &module.decls {
        match decl {
            Decl::Function {
                name,
                type_params,
                ..
            } if !type_params.is_empty() => {
                templates.insert(name.clone(), decl.clone());
            }
            Decl::DataClass {
                name,
                type_params,
                ..
            } if !type_params.is_empty() => {
                templates.insert(name.clone(), decl.clone());
            }
            Decl::Enum {
                name,
                type_params,
                ..
            } if !type_params.is_empty() => {
                templates.insert(name.clone(), decl.clone());
            }
            Decl::FinalClass {
                name,
                type_params,
                ..
            } if !type_params.is_empty() => {
                templates.insert(name.clone(), decl.clone());
            }
            Decl::Class { name, members, .. } | Decl::Actor { name, members, .. } => {
                for member in members {
                    if let Some((qname, template)) = class_static_generic_template(name, member) {
                        templates.insert(qname, template);
                    }
                }
            }
            _ => {}
        }
    }
    templates
}

pub fn monomorphize_module(
    module: &Module,
    instantiations: &HashSet<(SmolStr, Vec<SmolStr>)>,
) -> Module {
    let templates = collect_generic_templates(module);

    let mut decls = module.decls.clone();
    let mut insertions: Vec<(usize, Decl)> = Vec::new();
    for (base, type_args) in instantiations {
        if let Some(template) = templates.get(base) {
            let mangled = mangle_name(base.as_str(), type_args);
            if decls.iter().any(|d| decl_name(d) == Some(mangled.as_str())) {
                continue;
            }
            let insert_at = decls
                .iter()
                .position(|d| decl_name(d) == Some(base.as_str()))
                .map(|i| i + 1)
                .unwrap_or(decls.len());
            match template {
                Decl::Function {
                    span,
                    access,
                    attrs,
                    name,
                    type_params,
                    params,
                    return_type,
                    effect,
                    where_clause,
                    body,
                    is_async,
                    ..
                } => {
                    let substituted_params: Vec<_> = params
                        .iter()
                        .map(|p| syx_ast::Param {
                            name: p.name.clone(),
                            external_label: p.external_label.clone(),
                            ty: p
                                .ty
                                .as_ref()
                                .map(|t| substitute_type_expr(t, type_params, type_args)),
                            default: p.default.clone(),
                            isolated: p.isolated,
                            sending: p.sending,
                            attrs: p.attrs.clone(),
                        })
                        .collect();
                    insertions.push((
                        insert_at,
                        Decl::Function {
                            span: *span,
                            access: *access,
                            attrs: attrs.clone(),
                            name: mangled,
                            type_params: vec![],
                            params: substituted_params,
                            return_type: return_type
                                .as_ref()
                                .map(|t| substitute_type_expr(t, type_params, type_args)),
                            throws_type: None,
                            effect: *effect,
                            where_clause: substitute_where_clause(
                                where_clause,
                                type_params,
                                type_args,
                            ),
                            body: body.clone(),
                            is_async: *is_async,
                            is_override: false,
            is_final: false,
                        },
                    ));
                }
                Decl::DataClass {
                    span,
                    access,
                    attrs,
                    name,
                    type_params,
                    where_clause,
                    fields,
                    ..
                } => {
                    insertions.push((
                        insert_at,
                        Decl::DataClass {
                            span: *span,
                            access: *access,
                            attrs: attrs.clone(),
                            name: mangled,
                            type_params: vec![],
                            where_clause: substitute_where_clause(
                                where_clause,
                                type_params,
                                type_args,
                            ),
                            fields: fields
                                .iter()
                                .map(|(f, ty)| {
                                    (
                                        f.clone(),
                                        substitute_type_expr(ty, type_params, type_args),
                                    )
                                })
                                .collect(),
                        },
                    ));
                }
                Decl::Enum {
                    span,
                    access,
                    name,
                    type_params,
                    cases,
                    ..
                } => {
                    let substituted_cases = cases
                        .iter()
                        .map(|(case, value, fields)| {
                            (
                                case.clone(),
                                value.clone(),
                                fields
                                    .iter()
                                    .map(|(f, ty)| {
                                        (
                                            f.clone(),
                                            substitute_type_expr(ty, type_params, type_args),
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect();
                    insertions.push((
                        insert_at,
                        Decl::Enum {
                            span: *span,
                            access: *access,
                            attrs: vec![],
                            name: mangled,
                            type_params: vec![],
                            cases: substituted_cases,
                        },
                    ));
                }
                Decl::FinalClass {
                    span,
                    access,
                    name,
                    type_params,
                    variants,
                    ..
                } => {
                    let substituted_variants = variants
                        .iter()
                        .map(|(variant, fields)| {
                            (
                                variant.clone(),
                                fields
                                    .iter()
                                    .map(|(f, ty)| {
                                        (
                                            f.clone(),
                                            substitute_type_expr(ty, type_params, type_args),
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect();
                    insertions.push((
                        insert_at,
                        Decl::FinalClass {
                            span: *span,
                            access: *access,
                            name: mangled,
                            type_params: vec![],
                            variants: substituted_variants,
                        },
                    ));
                }
                _ => {}
            }
        }
    }
    for (idx, decl) in insertions {
        decls.insert(idx, decl);
    }
    Module {
        name: module.name.clone(),
        decls,
    }
}

fn decl_name(d: &Decl) -> Option<&str> {
    match d {
        Decl::Function { name, .. }
        | Decl::DataClass { name, .. }
        | Decl::Enum { name, .. }
        | Decl::FinalClass { name, .. } => Some(name.as_str()),
        _ => None,
    }
}

/// Phase 22: substitute generic type parameters in where-clause predicates during monomorphization.
pub fn substitute_where_clause(
    where_clause: &[WherePred],
    type_params: &[SmolStr],
    type_args: &[SmolStr],
) -> Vec<WherePred> {
    where_clause
        .iter()
        .map(|pred| WherePred {
            type_name: type_params
                .iter()
                .position(|p| p == &pred.type_name)
                .and_then(|idx| type_args.get(idx))
                .cloned()
                .unwrap_or_else(|| pred.type_name.clone()),
            constraint: propagate_witness_where(&pred.constraint, type_args, type_params),
        })
        .collect()
}

/// Phase 23 DE3: default associated type name for witness v6 monomorphization.
pub fn default_assoc_type_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    match (protocol, assoc) {
        ("Sequence", "Element") | ("Collection", "Element") => Some("Int"),
        ("Hashable", "HashValue") => Some("Int"),
        ("IteratorProtocol", "Element") => Some("Int"),
        ("WitnessProto", "Element") => Some("Int"),
        _ => None,
    }
}

/// Phase 25 DO4: witness v8 higher-order associated type default propagation.
pub fn default_higher_order_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_assoc_type_for(protocol, assoc).or(match (protocol, assoc) {
        ("Comparable", "Compared") => Some("Bool"),
        ("Equatable", "Compared") => Some("Bool"),
        _ => None,
    })
}

/// Phase 26 DT2: witness v9 — nested protocol associated type defaults.
pub fn default_witness_v9_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_higher_order_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("Hashable", "HashValue") => Some("Int"),
        ("Sequence", "Iterator") => Some("Int"),
        ("Collection", "Index") => Some("Int"),
        _ => None,
    })
}

/// Phase 27 DY3: witness v10 — stream/GC/TLS protocol associated type defaults.
pub fn default_witness_v10_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v9_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("Streamable", "Element") => Some("Int"),
        ("TlsConnectable", "Endpoint") => Some("String"),
        ("GcCollectible", "HeapSlot") => Some("Int"),
        _ => None,
    })
}

/// Phase 28 ED3: witness v11 — Result generic + GC phase28 + registry failover assoc defaults.
pub fn default_witness_v11_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v10_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("ResultGeneric", "Ok") => Some("Int"),
        ("ResultGeneric", "Err") => Some("String"),
        ("RegistryFailover", "Region") => Some("String"),
        ("GcPhase28", "SweepBudget") => Some("Int"),
        _ => None,
    })
}

/// Phase 29 EI3: witness v12 — relocating GC + multi-region GlobalActor protocol assoc defaults.
pub fn default_witness_v12_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v11_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("GcRelocating", "HeapSlot") => Some("Int"),
        ("GcRelocating", "CompactBudget") => Some("Int"),
        ("MultiRegionActor", "Region") => Some("String"),
        ("MultiRegionActor", "Endpoint") => Some("String"),
        ("NetStreamHandle", "Backpressure") => Some("Int"),
        _ => None,
    })
}

/// Phase 30 EN2: witness v13 — Host I/O AOT / write-barrier / global LB protocol assoc defaults.
pub fn default_witness_v13_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v12_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("HostIoAot", "Buffer") => Some("String"),
        ("GcWriteBarrier", "Slot") => Some("Int"),
        ("GlobalActorLb", "Region") => Some("String"),
        ("NetRingBuffer", "Capacity") => Some("Int"),
        ("ResultTryExpr", "Ok") => Some("Int"),
        _ => None,
    })
}

/// Phase 31 ES2: witness v14 — generational GC / perf release / global LB failover protocol assoc defaults.
pub fn default_witness_v14_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v13_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("GcGenerational", "YoungHeap") => Some("Int"),
        ("GcGenerational", "OldHeap") => Some("Int"),
        ("GcGenerational", "PromotionCount") => Some("Int"),
        ("PerfRelease", "StrictGreenWeeks") => Some("Int"),
        ("GlobalActorV11", "Weight") => Some("Int"),
        ("GlobalActorV11", "FailoverEndpoint") => Some("String"),
        ("NetFlowControl", "Window") => Some("Int"),
        ("BorrowFormal", "RegionGraph") => Some("Bool"),
        ("ResultAsyncTry", "Ok") => Some("Int"),
        _ => None,
    })
}

/// Phase 32 EX2: witness v15 — remembered set / release blocker GA / v12 dynamic LB protocol assoc defaults.
pub fn default_witness_v15_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v14_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("GcRememberedSet", "CardTable") => Some("Int"),
        ("GcRememberedSet", "SlotCount") => Some("Int"),
        ("ReleaseBlockerGa", "GreenSeasons") => Some("Int"),
        ("GlobalActorV12", "DynamicWeight") => Some("Int"),
        ("GlobalActorV12", "DrainEndpoint") => Some("String"),
        ("NetSocketWindow", "RecvBuf") => Some("Int"),
        ("NetSocketWindow", "SendBuf") => Some("Int"),
        ("BorrowMachineProof", "Obligation") => Some("String"),
        ("ResultActorTry", "Ok") => Some("Int"),
        _ => None,
    })
}

/// Phase 33 FC2: witness v16 — native GC / inkwell default / v13 org failover protocol assoc defaults.
pub fn default_witness_v16_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v15_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("VmNativeGc", "RememberedSet") | ("NativeGc", "RememberedSet") => Some("Int"),
        ("VmNativeGc", "CollectCount") | ("NativeGc", "CardTable") => Some("Int"),
        ("InkwellDefault", "GaReady") => Some("Bool"),
        ("InkwellDefault", "GreenWeeks") => Some("Int"),
        ("InkwellDefault", "CompilePath") => Some("String"),
        ("GlobalActorV13", "OrgWeight") => Some("Int"),
        ("GlobalActorV13", "DrainEndpoint") | ("GlobalActorV13", "OrgFailover")
        | ("GlobalActorV13", "LiveEndpoint") => Some("String"),
        ("BorrowCoqLean", "CoqExport") | ("BorrowCoqLean", "LeanExport")
        | ("BorrowCoqLean", "Obligation") => Some("String"),
        ("RegistryPhase14", "EdgeCacheDir") => Some("String"),
        ("NetSetsockopt", "RecvBuf") => Some("Int"),
        ("NetSetsockopt", "SendBuf") => Some("Int"),
        ("ResultActorTryProd", "Ok") | ("ResultActorTry", "Ok") => Some("Int"),
        _ => None,
    })
}

/// Phase 34 FH2: witness v17 — unified GC / inkwell prod / v14 global CDN protocol assoc defaults.
pub fn default_witness_v17_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v16_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("UnifiedGc", "RememberedSet") | ("UnifiedGc", "CollectCount") => Some("Int"),
        ("UnifiedGc", "PromoteCount") => Some("Int"),
        ("InkwellProd", "GaReady") => Some("Bool"),
        ("InkwellProd", "GreenWeeks") => Some("Int"),
        ("InkwellProd", "CompilePath") => Some("String"),
        ("GlobalActorV14", "CdnWeight") => Some("Int"),
        ("GlobalActorV14", "DrainEndpoint") | ("GlobalActorV14", "CdnFailover")
        | ("GlobalActorV14", "LiveEndpoint") => Some("String"),
        ("BorrowCoqLeanProof", "CoqReplay") | ("BorrowCoqLeanProof", "LeanReplay") => Some("String"),
        ("RegistryPhase15", "GlobalWritePath") => Some("String"),
        ("DateTimeV2", "Iso8601") | ("DateTimeV2", "Timezone") => Some("String"),
        _ => None,
    })
}

/// Phase 35 FM4: witness v18 — inkwell object / GC promotion / formal borrow CI protocol assoc defaults.
pub fn default_witness_v18_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v17_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("InkwellObject", "GaReady") => Some("Bool"),
        ("InkwellObject", "ObjectPath") => Some("String"),
        ("GcPromotion", "PromoteCount") | ("GcPromotion", "CollectCount") => Some("Int"),
        ("FormalBorrow", "CiGate") | ("FormalBorrow", "CoqReplay") | ("FormalBorrow", "LeanReplay") => {
            Some("String")
        }
        ("GlobalActorV15", "CrdtWeight") => Some("Int"),
        ("GlobalActorV15", "CrdtEndpoint") | ("GlobalActorV15", "CrdtFailover") => Some("String"),
        ("RegistryPhase16", "CrdtWritePath") => Some("String"),
        ("PropertyWrapper", "WrappedType") => Some("String"),
        ("TypealiasProd", "AliasName") => Some("String"),
        _ => None,
    })
}

/// Phase 36 FR4: witness v19 — JIT tier-2 / GC compaction / typealias GA protocol assoc defaults.
pub fn default_witness_v19_assoc_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    default_witness_v18_assoc_for(protocol, assoc).or(match (protocol, assoc) {
        ("InkwellCrate", "GaReady") => Some("Bool"),
        ("InkwellCrate", "ModulePath") => Some("String"),
        ("JitTier2", "NativeReady") | ("JitTier2", "DeoptVersion") => Some("Int"),
        ("GcCompaction", "CompactCount") => Some("Int"),
        ("TypealiasGa", "MonoName") => Some("String"),
        ("BorrowSubprocess", "CiGate") => Some("String"),
        ("GlobalActorV16", "RegistryEndpoint") | ("GlobalActorV16", "JitInvalidate") => {
            Some("String")
        }
        ("RegistryPhase17", "CrdtResolvePath") => Some("String"),
        ("PropertyWrapperParser", "WrapperName") => Some("String"),
        _ => None,
    })
}

/// Phase 24 DJ4 / Phase 25 v8: parameterized associated type default for witness monomorphization.
pub fn default_param_assoc_type_for(protocol: &str, assoc: &str, type_param: &str) -> Option<&'static str> {
    if let Some(base) = default_witness_v19_assoc_for(protocol, assoc) {
        let _ = type_param;
        return Some(base);
    }
    if let Some(base) = default_witness_v17_assoc_for(protocol, assoc) {
        let _ = type_param;
        return Some(base);
    }
    if let Some(base) = default_witness_v16_assoc_for(protocol, assoc) {
        let _ = type_param;
        return Some(base);
    }
    match (protocol, assoc, type_param) {
        ("Sequence", "Element", "T") => Some("Int"),
        ("IteratorProtocol", "Element", "T") => Some("Int"),
        ("Collection", "Element", "T") => Some("Int"),
        ("WitnessProto", "Element", "T") => Some("Int"),
        _ => None,
    }
}

/// Phase 22: propagate higher-order witness where constraints to concrete type names.
pub fn propagate_witness_where(
    constraint: &SmolStr,
    type_args: &[SmolStr],
    type_params: &[SmolStr],
) -> SmolStr {
    if type_params
        .iter()
        .zip(type_args.iter())
        .any(|(param, arg)| param.as_str() == constraint.as_str())
    {
        return type_args
            .iter()
            .zip(type_params.iter())
            .find_map(|(arg, param)| {
                if param.as_str() == constraint.as_str() {
                    Some(arg.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| constraint.clone());
    }
    constraint.clone()
}

/// Phase 21: substitute associated types and generic witness type parameters during monomorphization.
pub fn substitute_assoc_types(
    ty: &TypeExpr,
    bindings: &indexmap::IndexMap<SmolStr, SmolStr>,
) -> TypeExpr {
    substitute_assoc_types_with_params(ty, bindings, &[])
}

pub fn substitute_assoc_types_with_params(
    ty: &TypeExpr,
    assoc_bindings: &indexmap::IndexMap<SmolStr, SmolStr>,
    type_params: &[(&[SmolStr], &[SmolStr])],
) -> TypeExpr {
    match ty {
        TypeExpr::Named { name, generics } => {
            if generics.is_empty() {
                if let Some(concrete) = assoc_bindings.get(name) {
                    return TypeExpr::Named {
                        name: concrete.clone(),
                        generics: vec![],
                    };
                }
                for (params, args) in type_params {
                    if let Some(idx) = params.iter().position(|p| p == name) {
                        if let Some(arg) = args.get(idx) {
                            return TypeExpr::Named {
                                name: arg.clone(),
                                generics: vec![],
                            };
                        }
                    }
                }
            }
            TypeExpr::Named {
                name: name.clone(),
                generics: generics
                    .iter()
                    .map(|g| substitute_assoc_types_with_params(g, assoc_bindings, type_params))
                    .collect(),
            }
        }
        TypeExpr::Optional(inner) => TypeExpr::Optional(Box::new(substitute_assoc_types_with_params(
            inner,
            assoc_bindings,
            type_params,
        ))),
        TypeExpr::Array(inner) => TypeExpr::Array(Box::new(substitute_assoc_types_with_params(
            inner,
            assoc_bindings,
            type_params,
        ))),
        TypeExpr::Dictionary { key, value } => TypeExpr::Dictionary {
            key: Box::new(substitute_assoc_types_with_params(key, assoc_bindings, type_params)),
            value: Box::new(substitute_assoc_types_with_params(
                value,
                assoc_bindings,
                type_params,
            )),
        },
        TypeExpr::Reference(inner, mutable, lifetime) => TypeExpr::Reference(
            Box::new(substitute_assoc_types_with_params(
                inner,
                assoc_bindings,
                type_params,
            )),
            *mutable,
            lifetime.clone(),
        ),
        TypeExpr::OpaqueSome(inner) => TypeExpr::OpaqueSome(Box::new(
            substitute_assoc_types_with_params(inner, assoc_bindings, type_params),
        )),
        TypeExpr::OpaqueAny(inner) => TypeExpr::OpaqueAny(Box::new(
            substitute_assoc_types_with_params(inner, assoc_bindings, type_params),
        )),
        other => other.clone(),
    }
}

/// Phase 56: macro witness constraint matrix — map type params to protocol requirements.
pub fn witness_constraint_matrix(
    type_params: &[smol_str::SmolStr],
    protocols: &[&str],
) -> Vec<syx_ast::WherePred> {
    type_params
        .iter()
        .flat_map(|tp| {
            protocols.iter().map(move |proto| syx_ast::WherePred {
                type_name: tp.clone(),
                constraint: smol_str::SmolStr::new(*proto),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;

    #[test]
    fn substitute_where_clause_propagates_type_args() {
        let preds = vec![WherePred {
            type_name: SmolStr::new("T"),
            constraint: SmolStr::new("WitnessProto"),
        }];
        let params = vec![SmolStr::new("T")];
        let args = vec![SmolStr::new("IntWitness")];
        let out = substitute_where_clause(&preds, &params, &args);
        assert_eq!(out[0].type_name.as_str(), "IntWitness");
        assert_eq!(out[0].constraint.as_str(), "WitnessProto");
    }

    #[test]
    fn propagate_witness_where_resolves_generic_param() {
        let params = vec![SmolStr::new("T")];
        let args = vec![SmolStr::new("IntBox")];
        let out = propagate_witness_where(&SmolStr::new("T"), &args, &params);
        assert_eq!(out.as_str(), "IntBox");
    }
}
