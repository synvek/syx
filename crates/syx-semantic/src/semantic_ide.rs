//! SYX semantic IDE face (S2-I257).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{
    expr_span, type_expr_to_id, type_id_to_name, TypeId,
};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{
    Decl, Expr, Literal, Module,
};
use syx_stdlib::{
    host_fn_return_type, method_return_type, ARRAY_METHODS, DICT_METHODS, HTTP_RESPONSE_METHODS, SET_METHODS, STRING_METHODS, TYPE_METHODS,
};

impl SemanticAnalyzer {
    /// Resolve a symbol's type from lexical scope, tracked bindings, or globals (for IDE).
    /// Local / catch / let bindings win over globals so names like `e` are not
    /// reported as the Math host constant `Float64`.
    pub fn symbol_type(&self, name: &str) -> Option<TypeId> {
        self.scope
            .lookup(name)
            .map(|s| s.ty.clone())
            .or_else(|| self.all_bindings.get(name).cloned())
            .or_else(|| self.globals.get(name).map(|s| s.ty.clone()))
    }

    /// Best-effort type of an expression without mutating analyzer state (for IDE).
    pub fn type_of_expr(&self, expr: &Expr) -> TypeId {
        if let Some(ty) = self.type_of_expr_from_cache(expr) {
            if !matches!(ty, TypeId::Unknown) {
                return ty;
            }
        }
        match expr {
            Expr::Ident { name, .. } => self.symbol_type(name).unwrap_or(TypeId::Unknown),
            Expr::Member { object, name, .. } => {
                let obj_ty = self.type_of_expr(object);
                self.infer_member_type(&obj_ty, name)
            }
            Expr::Literal { value, .. } => match value {
                Literal::BigInt(_) => TypeId::Named(SmolStr::new("BigInt")),
                Literal::Integer { .. } | Literal::Double { .. } => self.literal_numeric_type(value),
                Literal::Bool(_) => TypeId::Bool,
                Literal::String(_) | Literal::Interpolated(_) | Literal::Regex(_) => TypeId::String,
                Literal::Nil => TypeId::Optional(Box::new(TypeId::Unknown)),
                Literal::Dict(_) => TypeId::Dictionary {
                    key: Box::new(TypeId::String),
                    value: Box::new(TypeId::Unknown),
                },
            },
            Expr::Array { elements, .. } => {
                let elem_types: Vec<TypeId> =
                    elements.iter().map(|el| self.type_of_expr(el)).collect();
                let elem = if elem_types.is_empty() {
                    TypeId::Unknown
                } else if elem_types.iter().all(|t| *t == elem_types[0]) {
                    elem_types[0].clone()
                } else {
                    TypeId::Unknown
                };
                TypeId::Array(Box::new(elem))
            }
            Expr::Call { callee, .. } => self.type_of_call_callee(callee),
            Expr::Index { object, .. } => {
                let obj_ty = self.type_of_expr(object);
                match obj_ty {
                    TypeId::Array(inner) => *inner,
                    TypeId::String => TypeId::String,
                    TypeId::Dictionary { value, .. } => *value,
                    _ => TypeId::Unknown,
                }
            }
            Expr::Borrow { expr, .. } | Expr::Group { expr, .. } => self.type_of_expr(expr),
            _ => TypeId::Unknown,
        }
    }

    fn type_of_expr_from_cache(&self, expr: &Expr) -> Option<TypeId> {
        let start = expr_span(expr).start;
        // Synthetic completion receivers use Span::DUMMY (start=0); never trust cache there.
        if start == 0 && expr_span(expr).len == 0 {
            return None;
        }
        self.expr_types.get(&start).cloned()
    }

    fn type_of_call_callee(&self, callee: &Expr) -> TypeId {
        match callee {
            Expr::Ident { name, .. } => self
                .function_return_types
                .get(name)
                .cloned()
                .unwrap_or_else(|| {
                    self.globals
                        .get(name)
                        .map(|s| s.ty.clone())
                        .unwrap_or(TypeId::Unknown)
                }),
            Expr::Member { object, name, .. } => {
                let obj_ty = self.type_of_expr(object);
                self.infer_member_type(&obj_ty, name)
            }
            _ => TypeId::Unknown,
        }
    }

    /// Type inferred at a source byte offset from the semantic expression cache.
    pub fn type_of_expr_at_offset(&self, offset: u32) -> Option<TypeId> {
        self.expr_types
            .iter()
            .filter(|(start, _)| **start <= offset)
            .max_by_key(|(start, _)| *start)
            .map(|(_, ty)| ty.clone())
    }

    /// Public member-type lookup for IDE hover (`Int.min` → `Int`).
    pub fn type_of_member(&self, object_ty: &TypeId, name: &str) -> TypeId {
        self.infer_member_type(object_ty, name)
    }

    /// Surface name used for class/extension member tables (`Numeric(Int)` → `"Int"`).
    pub fn member_lookup_surface(ty: &TypeId) -> Option<SmolStr> {
        match ty {
            TypeId::Named(n) => Some(n.clone()),
            TypeId::Numeric(kind) => Some(SmolStr::new(syx_stdlib::type_name_with_alias(*kind))),
            TypeId::Bool => Some(SmolStr::new("Bool")),
            TypeId::String => Some(SmolStr::new("String")),
            _ => None,
        }
    }

    /// True when `name` is a stored/computed field on `ty` (not a method).
    pub fn is_instance_field(&self, ty: &TypeId, name: &str) -> bool {
        let Some(surface) = Self::member_lookup_surface(ty) else {
            return false;
        };
        self.dataclass_fields
            .get(&surface)
            .is_some_and(|fields| fields.iter().any(|(f, _)| f.as_str() == name))
    }

    /// Merge Class / Extension members from an already-expanded Core value builtin
    /// (e.g. `Int.sy`) without injecting the Class into the user module (avoids S023).
    pub fn merge_value_builtin_surface(&mut self, module: &Module) {
        for decl in &module.decls {
            match decl {
                Decl::Class {
                    name, members, ..
                }
                | Decl::Actor {
                    name, members, ..
                } => {
                    // Metatype name for `Int.min` Ident resolution prefer-path stays via
                    // globals → Numeric; members are keyed by surface name below.
                    let mut methods = self.class_methods.swap_remove(name).unwrap_or_default();
                    let mut fields = self.dataclass_fields.swap_remove(name).unwrap_or_default();
                    let mut prop_getters: IndexMap<SmolStr, TypeId> = IndexMap::new();
                    let mut prop_setters: HashSet<SmolStr> = HashSet::new();
                    for member in members {
                        match member {
                            Decl::VarGlobal {
                                name: field,
                                ty,
                                computed_getter,
                                computed_setter,
                                ..
                            } => {
                                let field_ty = ty
                                    .as_ref()
                                    .map(type_expr_to_id)
                                    .unwrap_or(TypeId::Unknown);
                                if !fields.iter().any(|(f, _)| f == field) {
                                    fields.push((field.clone(), field_ty));
                                }
                                if computed_getter.is_some() && computed_setter.is_none() {
                                    self.getter_only_properties
                                        .insert((name.clone(), field.clone()));
                                }
                            }
                            Decl::Function {
                                name: method,
                                return_type,
                                ..
                            } => {
                                if let Some(field) = method.strip_prefix("__get_") {
                                    let field = SmolStr::new(field);
                                    let ret = return_type
                                        .as_ref()
                                        .map(type_expr_to_id)
                                        .unwrap_or(TypeId::Unknown);
                                    prop_getters.insert(field, ret);
                                    continue;
                                }
                                if let Some(field) = method.strip_prefix("__set_") {
                                    prop_setters.insert(SmolStr::new(field));
                                    continue;
                                }
                                let ret = return_type
                                    .as_ref()
                                    .map(type_expr_to_id)
                                    .unwrap_or(TypeId::Void);
                                if !methods.iter().any(|(m, _)| m == method) {
                                    methods.push((method.clone(), ret));
                                }
                            }
                            _ => {}
                        }
                    }
                    for (field, field_ty) in &prop_getters {
                        if !fields.iter().any(|(f, _)| f == field) {
                            fields.push((field.clone(), field_ty.clone()));
                        }
                        if !prop_setters.contains(field) {
                            self.getter_only_properties
                                .insert((name.clone(), field.clone()));
                        }
                    }
                    if !fields.is_empty() {
                        self.dataclass_fields.insert(name.clone(), fields);
                    }
                    if !methods.is_empty() {
                        self.class_methods.insert(name.clone(), methods);
                    }
                }
                Decl::Extension {
                    type_name, methods, ..
                } => {
                    let mut ext = self
                        .type_extension_methods
                        .swap_remove(type_name)
                        .unwrap_or_default();
                    let mut prop_getters: IndexMap<SmolStr, TypeId> = IndexMap::new();
                    let mut prop_setters: HashSet<SmolStr> = HashSet::new();
                    for method in methods {
                        if let Decl::Function {
                            name,
                            return_type,
                            ..
                        } = method
                        {
                            let ret = return_type
                                .as_ref()
                                .map(type_expr_to_id)
                                .unwrap_or(TypeId::Void);
                            if let Some(field) = name.strip_prefix("__get_") {
                                let field = SmolStr::new(field);
                                prop_getters.insert(field.clone(), ret.clone());
                                if !ext.iter().any(|(m, _)| m == &field) {
                                    ext.push((field, ret));
                                }
                                continue;
                            }
                            if let Some(field) = name.strip_prefix("__set_") {
                                prop_setters.insert(SmolStr::new(field));
                                continue;
                            }
                            if !ext.iter().any(|(m, _)| m == name) {
                                ext.push((name.clone(), ret));
                            }
                        }
                    }
                    if !prop_getters.is_empty() {
                        let fields = self
                            .dataclass_fields
                            .entry(type_name.clone())
                            .or_default();
                        for (field, field_ty) in &prop_getters {
                            if !fields.iter().any(|(f, _)| f == field) {
                                fields.push((field.clone(), field_ty.clone()));
                            }
                            if !prop_setters.contains(field) {
                                self.getter_only_properties
                                    .insert((type_name.clone(), field.clone()));
                            }
                        }
                    }
                    if !ext.is_empty() {
                        self.type_extension_methods
                            .insert(type_name.clone(), ext);
                    }
                }
                _ => {}
            }
        }
    }

    /// Member names and return types for completion on a receiver type.
    pub fn members_for_type(&self, ty: &TypeId) -> Vec<(SmolStr, SmolStr)> {
        let mut out = Vec::new();
        let push_map = |map: &indexmap::IndexMap<&'static str, syx_stdlib::MethodId>,
                        out: &mut Vec<(SmolStr, SmolStr)>| {
            for (name, id) in map {
                out.push((
                    SmolStr::new(*name),
                    SmolStr::new(method_return_type(*id)),
                ));
            }
        };
        let push_extension_methods =
            |methods: &[(SmolStr, TypeId)], out: &mut Vec<(SmolStr, SmolStr)>| {
                for (method, ty) in methods {
                    // Belt-and-suspenders: never surface desugared `__get_`/`__set_` names.
                    if let Some(field) = method.as_str().strip_prefix("__get_") {
                        out.push((SmolStr::new(field), SmolStr::new(type_id_to_name(ty))));
                    } else if method.as_str().starts_with("__set_") {
                        continue;
                    } else {
                        out.push((method.clone(), SmolStr::new(type_id_to_name(ty))));
                    }
                }
            };
        let push_protocol_members = |ty: &TypeId, out: &mut Vec<(SmolStr, SmolStr)>| {
            if self.type_id_satisfies_protocol(ty, "CustomStringConvertible") {
                out.push((SmolStr::new("description"), SmolStr::new("String")));
            }
            if self.type_id_satisfies_protocol(ty, "CustomDebugStringConvertible") {
                out.push((
                    SmolStr::new("debugDescription"),
                    SmolStr::new("String"),
                ));
            }
            if self.type_id_satisfies_protocol(ty, "Hashable") {
                out.push((SmolStr::new("hash"), SmolStr::new("Void")));
            }
        };
        match ty {
            TypeId::String => push_map(&STRING_METHODS, &mut out),
            TypeId::Array(_) => {
                push_map(&ARRAY_METHODS, &mut out);
                if let Some(methods) = self.type_extension_methods.get(&SmolStr::new("Array")) {
                    push_extension_methods(methods, &mut out);
                }
                push_protocol_members(ty, &mut out);
            }
            TypeId::Set(_) => {
                push_map(&SET_METHODS, &mut out);
                if let Some(methods) = self.type_extension_methods.get(&SmolStr::new("Set")) {
                    push_extension_methods(methods, &mut out);
                }
                push_protocol_members(ty, &mut out);
            }
            TypeId::Dictionary { .. } => {
                push_map(&DICT_METHODS, &mut out);
                if let Some(methods) = self.type_extension_methods.get(&SmolStr::new("Dictionary"))
                {
                    push_extension_methods(methods, &mut out);
                }
                push_protocol_members(ty, &mut out);
            }
            TypeId::Bool | TypeId::Numeric(_) => {
                if let Some(surface) = Self::member_lookup_surface(ty) {
                    self.push_surface_members(&surface, &mut out);
                }
                push_protocol_members(ty, &mut out);
            }
            TypeId::Named(n) => {
                if n.as_str() == "HTTPResponse" {
                    push_map(&HTTP_RESPONSE_METHODS, &mut out);
                }
                if let Some(exports) = self.import_exports.get(n) {
                    for (name, ty) in exports {
                        out.push((name.clone(), SmolStr::new(type_id_to_name(ty))));
                    }
                }
                for tm in TYPE_METHODS.iter() {
                    if tm.type_name == n.as_str() {
                        let ret = if let Some(h) = tm.host {
                            host_fn_return_type(h)
                        } else if let Some(m) = tm.instance {
                            method_return_type(m)
                        } else {
                            "Void"
                        };
                        out.push((SmolStr::new(tm.method), SmolStr::new(ret)));
                    }
                }
                self.push_surface_members(n, &mut out);
                if self.dataclass_names.contains(n) {
                    out.push((SmolStr::new("copy"), n.clone()));
                }
                push_protocol_members(ty, &mut out);
            }
            _ => {}
        }
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out.dedup_by(|a, b| a.0 == b.0);
        out
    }
}
