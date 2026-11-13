//! SYX MIR module index (AST → ModuleIndex) + type-name helpers.
//! Extracted in S2-I170 (zero semantics; crate-private; used by lowering in lib.rs).

use indexmap::{IndexMap, IndexSet};
use smol_str::SmolStr;
use syx_ast::{Decl, Expr, Literal, Module, Stmt, TypeExpr};
use syx_diagnostics::SourceFile;
use syx_stdlib::NumericKind;

#[derive(Debug, Default, Clone)]
pub(crate) struct ModuleIndex {
    pub(crate) class_types: IndexMap<SmolStr, Vec<SmolStr>>,
    /// Phase 121: stored property name → type for `self.field` method dispatch.
    pub(crate) class_field_types: IndexMap<SmolStr, IndexMap<SmolStr, SmolStr>>,
    /// Phase 65/125: inheritance + vtable slot map (class → method → mangled fn).
    pub(crate) class_superclasses: IndexMap<SmolStr, SmolStr>,
    pub(crate) class_vtables: IndexMap<SmolStr, IndexMap<SmolStr, SmolStr>>,
    pub(crate) actor_types: IndexSet<SmolStr>,
    /// Phase 101: types that declare an explicit `deinit`.
    pub(crate) types_with_deinit: IndexSet<SmolStr>,
    pub(crate) generic_templates: IndexSet<SmolStr>,
    /// S1-I5: type-parameter arity for mono mangling (`compose<A,B,C>` → 3).
    pub(crate) generic_template_arity: IndexMap<SmolStr, usize>,
    pub(crate) dataclass_names: IndexSet<SmolStr>,
    /// Field order for DataClass/Struct memberwise init (MIR pads missing args).
    pub(crate) dataclass_fields: IndexMap<SmolStr, Vec<SmolStr>>,
    /// `(type, field)` → default int literal for omitted memberwise args (Counter value=0).
    pub(crate) dataclass_field_default_i64: IndexMap<(SmolStr, SmolStr), i128>,
    pub(crate) enum_classes: IndexSet<SmolStr>,
    pub(crate) final_classes: IndexSet<SmolStr>,
    pub(crate) enum_case_tags: IndexMap<(SmolStr, SmolStr), i64>,
    pub(crate) enum_associated_fields: IndexMap<(SmolStr, SmolStr), Vec<SmolStr>>,
    pub(crate) function_params: IndexMap<SmolStr, Vec<Option<TypeExpr>>>,
    /// Top-level / free function return type names (`make` → `Array<Int>`).
    pub(crate) function_returns: IndexMap<SmolStr, SmolStr>,
    pub(crate) opaque_functions: IndexSet<SmolStr>,
    pub(crate) extension_methods: IndexSet<(SmolStr, SmolStr)>,
    /// Extension method return type names for local inference (`Person.decode` → `Person`).
    pub(crate) extension_method_returns: IndexMap<(SmolStr, SmolStr), SmolStr>,
    pub(crate) static_class_methods: IndexSet<(SmolStr, SmolStr)>,
    /// Phase 47: merged extension symbols available for `Call` (e.g. `Array_description`).
    pub(crate) callable_extensions: IndexSet<SmolStr>,
    /// Phase 126: declared protocol names for virtual instance dispatch on protocol-typed refs.
    pub(crate) protocol_names: IndexSet<SmolStr>,
    /// Phase 126: protocol name → method names (witness slot setup + dispatch).
    pub(crate) protocol_methods: IndexMap<SmolStr, Vec<SmolStr>>,
    /// S1-I6: protocol default method bodies `(protocol, method)` → `(params, body)`.
    pub(crate) protocol_default_methods:
        IndexMap<(SmolStr, SmolStr), (Vec<syx_ast::Param>, Vec<Stmt>)>,
    pub(crate) property_observers:
        IndexMap<(SmolStr, SmolStr), (Option<Vec<Stmt>>, Option<Vec<Stmt>>)>,
    /// Phase 134: `(type, field)` with desugared `__get_{field}` accessor.
    pub(crate) property_getters: IndexSet<(SmolStr, SmolStr)>,
    /// Phase 135: `(type, field)` with desugared `__set_{field}` accessor.
    pub(crate) property_setters: IndexSet<(SmolStr, SmolStr)>,
    /// Phase 134: `(type, field)` → (global name, mutable).
    pub(crate) static_vars: IndexMap<(SmolStr, SmolStr), (SmolStr, bool)>,
    /// Span-start → NumericKind from semantic contextual typing
    /// (`let x: Int8 = 8`, `let y: Double = 1`). Empty when unused.
    pub(crate) contextual_numeric_kinds: IndexMap<u32, NumericKind>,
    /// Top-level `let`/`var` annotated types (`arr: Array<Int>` → for mono mangling).
    pub(crate) global_types: IndexMap<SmolStr, SmolStr>,
    /// R-36: optional source for MIR001/C020 Display `file:line:col`.
    pub(crate) source: Option<SourceFile>,
}

impl ModuleIndex {
    pub(crate) fn lookup_property_observer(
        &self,
        ty: &SmolStr,
        field: &SmolStr,
    ) -> Option<(Option<Vec<Stmt>>, Option<Vec<Stmt>>)> {
        let mut cur = Some(ty.clone());
        while let Some(t) = cur {
            if let Some(obs) = self.property_observers.get(&(t.clone(), field.clone())) {
                return Some(obs.clone());
            }
            cur = self.class_superclasses.get(&t).cloned();
        }
        None
    }

    /// Stored property declared on `ty` itself (not inherited).
    pub(crate) fn has_own_stored_field(&self, ty: &SmolStr, field: &SmolStr) -> bool {
        self.dataclass_fields
            .get(ty)
            .is_some_and(|fs| fs.iter().any(|f| f == field))
            || self
                .class_types
                .get(ty)
                .is_some_and(|fs| fs.iter().any(|f| f == field))
            || self
                .class_field_types
                .get(ty)
                .is_some_and(|m| m.contains_key(field))
    }

    pub(crate) fn property_getter_fn(&self, ty: &SmolStr, field: &SmolStr) -> Option<SmolStr> {
        // R-35: `class C { var count }` must not resolve to `Object.count` getter.
        if self.has_own_stored_field(ty, field)
            && !self.property_getters.contains(&(ty.clone(), field.clone()))
        {
            return None;
        }
        let mut cur = Some(ty.clone());
        while let Some(t) = cur {
            if self.property_getters.contains(&(t.clone(), field.clone())) {
                // S1-I8: `4.isEven` types as Int64 but getter mangles as `Int___get_isEven`.
                let raw = SmolStr::new(format!("{t}_{}", syx_ast::var_global_getter_method(field)));
                return Some(remap_platform_int_extension_call(&raw));
            }
            cur = self.class_superclasses.get(&t).cloned();
        }
        None
    }

    pub(crate) fn property_setter_fn(&self, ty: &SmolStr, field: &SmolStr) -> Option<SmolStr> {
        if self.has_own_stored_field(ty, field)
            && !self.property_setters.contains(&(ty.clone(), field.clone()))
        {
            return None;
        }
        let mut cur = Some(ty.clone());
        while let Some(t) = cur {
            if self.property_setters.contains(&(t.clone(), field.clone())) {
                let raw = SmolStr::new(format!("{t}_{}", syx_ast::var_global_setter_method(field)));
                return Some(remap_platform_int_extension_call(&raw));
            }
            cur = self.class_superclasses.get(&t).cloned();
        }
        None
    }

    pub(crate) fn static_var_global(
        &self,
        ty: &SmolStr,
        field: &SmolStr,
    ) -> Option<(SmolStr, bool)> {
        self.static_vars.get(&(ty.clone(), field.clone())).cloned()
    }

    pub(crate) fn has_deinit(&self, type_name: &SmolStr) -> bool {
        self.types_with_deinit.contains(type_name)
    }

    fn register_nested_type(&mut self, outer: &SmolStr, decl: &Decl) {
        match decl {
            Decl::Class { name, members, .. } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                let fields: Vec<SmolStr> = members
                    .iter()
                    .filter_map(|m| syx_ast::var_global_stored_field_name(m))
                    .collect();
                self.class_types.insert(qualified.clone(), fields);
                for member in members {
                    if let Decl::VarGlobal {
                        name: field,
                        ty: Some(TypeExpr::Named { name: field_ty, .. }),
                        ..
                    } = member
                    {
                        self.class_field_types
                            .entry(qualified.clone())
                            .or_default()
                            .insert(field.clone(), field_ty.clone());
                    }
                    if matches!(
                        member,
                        Decl::Class { .. }
                            | Decl::Struct { .. }
                            | Decl::Enum { .. }
                            | Decl::FinalClass { .. }
                    ) {
                        self.register_nested_type(&qualified, member);
                    }
                }
            }
            Decl::Struct { name, .. } | Decl::Enum { name, .. } | Decl::FinalClass { name, .. } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.class_types.insert(qualified, vec![]);
            }
            _ => {}
        }
    }

    pub(crate) fn from_module(module: &Module) -> Self {
        let mut idx = Self {
            dataclass_names: IndexSet::new(),
            ..Self::default()
        };
        for decl in &module.decls {
            match decl {
                Decl::DataClass { name, fields, .. } => {
                    idx.dataclass_names.insert(name.clone());
                    idx.dataclass_fields.insert(
                        name.clone(),
                        fields.iter().map(|(f, _)| f.clone()).collect(),
                    );
                }
                Decl::Struct {
                    name,
                    fields,
                    members,
                    ..
                } => {
                    // U-0: register instance method return types so
                    // `let p = x.joining(...); p.normalized()` keeps recv typed
                    // (else MIR falls through to ObjectGet of the method name).
                    idx.dataclass_names.insert(name.clone());
                    let mut field_names: Vec<SmolStr> =
                        fields.iter().map(|(f, _)| f.clone()).collect();
                    for member in members {
                        if let Some(fname) = syx_ast::var_global_stored_field_name(member) {
                            if !field_names.iter().any(|f| f == &fname) {
                                field_names.push(fname);
                            }
                        }
                        if let Decl::VarGlobal {
                            name: field,
                            init:
                                Some(Expr::Literal {
                                    value: Literal::Integer { value, .. },
                                    ..
                                }),
                            ..
                        } = member
                        {
                            idx.dataclass_field_default_i64
                                .insert((name.clone(), field.clone()), *value);
                        }
                        if let Decl::Function {
                            name: method,
                            return_type,
                            attrs,
                            ..
                        } = member
                        {
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                idx.static_class_methods
                                    .insert((name.clone(), method.clone()));
                            }
                            if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                                idx.extension_method_returns
                                    .insert((name.clone(), method.clone()), ret.clone());
                                let mangled = SmolStr::new(format!("{name}_{method}"));
                                idx.function_returns.insert(mangled, ret);
                            }
                        }
                    }
                    idx.dataclass_fields.insert(name.clone(), field_names);
                }
                Decl::VarGlobal {
                    attrs,
                    name,
                    mutable,
                    will_set,
                    did_set,
                    ty,
                    ..
                } => {
                    if let Some(type_name) = ty.as_ref().and_then(mir_type_expr_name) {
                        idx.global_types.insert(name.clone(), type_name);
                    }
                    if let Some(attr) = attrs.iter().find(|a| a.name.as_str() == "syx_static_var") {
                        if attr.args.len() >= 2 {
                            let type_key = match &attr.args[0] {
                                syx_ast::AttributeArg::Ident(s) => s.clone(),
                                syx_ast::AttributeArg::String(s) => SmolStr::new(s.as_str()),
                            };
                            let field_key = match &attr.args[1] {
                                syx_ast::AttributeArg::Ident(s) => s.clone(),
                                syx_ast::AttributeArg::String(s) => SmolStr::new(s.as_str()),
                            };
                            idx.static_vars.insert(
                                (type_key.clone(), field_key.clone()),
                                (name.clone(), *mutable),
                            );
                            if will_set.is_some() || did_set.is_some() {
                                idx.property_observers.insert(
                                    (type_key, field_key),
                                    (will_set.clone(), did_set.clone()),
                                );
                            }
                        }
                    }
                }
                Decl::Function {
                    name,
                    params,
                    type_params,
                    return_type,
                    ..
                } if type_params.is_empty() => {
                    idx.function_params
                        .insert(name.clone(), params.iter().map(|p| p.ty.clone()).collect());
                    if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                        idx.function_returns.insert(name.clone(), ret);
                    }
                    if matches!(return_type, Some(TypeExpr::OpaqueSome(_))) {
                        idx.opaque_functions.insert(name.clone());
                    }
                }
                Decl::Function {
                    name, type_params, ..
                } if !type_params.is_empty() => {
                    idx.generic_templates.insert(name.clone());
                    idx.generic_template_arity
                        .insert(name.clone(), type_params.len());
                }
                Decl::Protocol { name, methods, .. } => {
                    idx.protocol_names.insert(name.clone());
                    let mut method_names: Vec<SmolStr> = Vec::new();
                    for m in methods {
                        if let Decl::Function {
                            name: method,
                            params,
                            body,
                            ..
                        } = m
                        {
                            if !method_names.iter().any(|n| n == method) {
                                method_names.push(method.clone());
                            }
                            // Prefer the declaration that carries a default body.
                            if !body.is_empty() {
                                idx.protocol_default_methods.insert(
                                    (name.clone(), method.clone()),
                                    (params.clone(), body.clone()),
                                );
                            }
                        }
                    }
                    idx.protocol_methods.insert(name.clone(), method_names);
                }
                Decl::Class { name, members, .. } | Decl::Actor { name, members, .. } => {
                    let fields: Vec<SmolStr> = members
                        .iter()
                        .filter_map(syx_ast::var_global_stored_field_name)
                        .collect();
                    idx.class_types.insert(name.clone(), fields);
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            ty: Some(field_ty),
                            init,
                            ..
                        } = member
                        {
                            let key = syx_ast::var_global_stored_field_name(member)
                                .unwrap_or_else(|| field.clone());
                            if let Some(ty_name) = mir_type_expr_name(field_ty) {
                                idx.class_field_types
                                    .entry(name.clone())
                                    .or_default()
                                    .insert(key.clone(), ty_name);
                            }
                            // R-35: `var count: Int = 0` defaults for empty `init() {}`.
                            if let Some(Expr::Literal {
                                value: Literal::Integer { value, .. },
                                ..
                            }) = init
                            {
                                idx.dataclass_field_default_i64
                                    .insert((name.clone(), key), *value);
                            }
                        }
                    }
                    if members.iter().any(|m| matches!(m, Decl::Deinit { .. })) {
                        idx.types_with_deinit.insert(name.clone());
                    }
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            will_set,
                            did_set,
                            ..
                        } = member
                        {
                            if will_set.is_some() || did_set.is_some() {
                                idx.property_observers.insert(
                                    (name.clone(), field.clone()),
                                    (will_set.clone(), did_set.clone()),
                                );
                            }
                        }
                        if let Decl::Function {
                            name: method,
                            type_params,
                            attrs,
                            return_type,
                            ..
                        } = member
                        {
                            if let Some(field) = method.as_str().strip_prefix("__get_") {
                                idx.property_getters
                                    .insert((name.clone(), SmolStr::new(field)));
                            }
                            if let Some(field) = method.as_str().strip_prefix("__set_") {
                                idx.property_setters
                                    .insert((name.clone(), SmolStr::new(field)));
                            }
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                idx.static_class_methods
                                    .insert((name.clone(), method.clone()));
                            }
                            if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                                idx.extension_method_returns
                                    .insert((name.clone(), method.clone()), ret);
                            }
                            if !type_params.is_empty()
                                && attrs.iter().any(|a| a.name.as_str() == "static")
                            {
                                idx.generic_templates
                                    .insert(SmolStr::new(format!("{name}_{method}")));
                            }
                        }
                        if let Decl::OperatorFunc {
                            op, return_type, ..
                        } = member
                        {
                            if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name) {
                                idx.extension_method_returns
                                    .insert((name.clone(), op.clone()), ret);
                            }
                        }
                    }
                    if matches!(decl, Decl::Actor { .. }) {
                        idx.actor_types.insert(name.clone());
                    }
                    for member in members {
                        if matches!(
                            member,
                            Decl::Class { .. }
                                | Decl::Struct { .. }
                                | Decl::Enum { .. }
                                | Decl::FinalClass { .. }
                        ) {
                            idx.register_nested_type(name, member);
                        }
                    }
                }
                Decl::Enum { name, cases, .. } => {
                    idx.enum_classes.insert(name.clone());
                    let mut next_tag = 0i64;
                    for (case, value, fields) in cases {
                        let tag = if let Some(expr) = value {
                            match expr {
                                Expr::Literal {
                                    value: Literal::Integer { value: v, .. },
                                    ..
                                } => Some(*v as i64),
                                _ => None,
                            }
                        } else {
                            None
                        };
                        let tag_val = tag.unwrap_or(next_tag);
                        next_tag = tag_val + 1;
                        idx.enum_case_tags
                            .insert((name.clone(), case.clone()), tag_val);
                        if !fields.is_empty() {
                            idx.enum_associated_fields.insert(
                                (name.clone(), case.clone()),
                                fields.iter().map(|(f, _)| f.clone()).collect(),
                            );
                        }
                    }
                }
                Decl::FinalClass { name, variants, .. } => {
                    idx.final_classes.insert(name.clone());
                    for (variant, fields) in variants {
                        if !fields.is_empty() {
                            idx.enum_associated_fields.insert(
                                (name.clone(), variant.clone()),
                                fields.iter().map(|(f, _)| f.clone()).collect(),
                            );
                        }
                    }
                }
                Decl::Extension {
                    type_name, methods, ..
                } => {
                    for method in methods {
                        if let Decl::Function {
                            name,
                            return_type,
                            attrs,
                            ..
                        } = method
                        {
                            // S1-I7: `extension Int` also answers `Int64`/`Int32` receivers.
                            for ty_alias in platform_int_extension_aliases(type_name) {
                                idx.extension_methods
                                    .insert((ty_alias.clone(), name.clone()));
                                if let Some(ret) = return_type.as_ref().and_then(mir_type_expr_name)
                                {
                                    idx.extension_method_returns
                                        .insert((ty_alias.clone(), name.clone()), ret);
                                }
                                if attrs.iter().any(|a| a.name.as_str() == "static") {
                                    idx.static_class_methods
                                        .insert((ty_alias.clone(), name.clone()));
                                }
                                if let Some(field) = name.as_str().strip_prefix("__get_") {
                                    idx.property_getters
                                        .insert((ty_alias.clone(), SmolStr::new(field)));
                                }
                                if let Some(field) = name.as_str().strip_prefix("__set_") {
                                    idx.property_setters
                                        .insert((ty_alias.clone(), SmolStr::new(field)));
                                }
                            }
                        }
                        // S1-I7: `extension Point { init() { self.init(x: 0, y: 0) } }`
                        // → memberwise defaults so `Point()` pads 0,0 (shell init stays).
                        if let Decl::Init { params, body, .. } = method {
                            if params.is_empty() {
                                register_extension_init_literal_defaults(&mut idx, type_name, body);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        for decl in &module.decls {
            let Decl::Class {
                name,
                members,
                superclass,
                ..
            } = decl
            else {
                continue;
            };
            if let Some(base) = superclass {
                idx.class_superclasses.insert(name.clone(), base.clone());
            }
            let mut vtable = idx
                .class_superclasses
                .get(name)
                .and_then(|base| idx.class_vtables.get(base))
                .cloned()
                .unwrap_or_default();
            for member in members {
                let Decl::Function {
                    name: method,
                    type_params,
                    attrs,
                    ..
                } = member
                else {
                    continue;
                };
                if !type_params.is_empty() || attrs.iter().any(|a| a.name.as_str() == "static") {
                    continue;
                }
                vtable.insert(method.clone(), SmolStr::new(format!("{name}_{method}")));
            }
            idx.class_vtables.insert(name.clone(), vtable);
        }
        idx
    }
}

/// S1-I7: `self.init(x: 0, y: 0)` → ordered `(field, value)` pairs.
pub(crate) fn extension_init_literal_fields(body: &[Stmt]) -> Option<Vec<(SmolStr, i128)>> {
    for stmt in body {
        let Stmt::Expr { expr, .. } = stmt else {
            continue;
        };
        let Expr::Call { callee, args, .. } = expr else {
            continue;
        };
        let Expr::Member { object, name, .. } = callee.as_ref() else {
            continue;
        };
        if name.as_str() != "init" {
            continue;
        }
        let is_self = matches!(
            object.as_ref(),
            Expr::Ident { name, .. } if name.as_str() == "self"
        );
        if !is_self {
            continue;
        }
        let mut out = Vec::new();
        for arg in args {
            let label = arg.label.as_ref()?;
            let Expr::Literal {
                value: Literal::Integer { value, .. },
                ..
            } = &arg.value
            else {
                return None;
            };
            out.push((label.clone(), *value));
        }
        if !out.is_empty() {
            return Some(out);
        }
    }
    None
}

/// S1-I7: pull `self.init(x: 0, y: 0)` literals into memberwise default pad table.
pub(crate) fn register_extension_init_literal_defaults(
    idx: &mut ModuleIndex,
    type_name: &SmolStr,
    body: &[Stmt],
) {
    if let Some(fields) = extension_init_literal_fields(body) {
        for (field, value) in fields {
            idx.dataclass_field_default_i64
                .insert((type_name.clone(), field), value);
        }
    }
}

/// S1-I7: `extension Int` method tables also key under platform Int64/Int32.
pub(crate) fn platform_int_extension_aliases(type_name: &SmolStr) -> Vec<SmolStr> {
    match type_name.as_str() {
        "Int" => vec![
            type_name.clone(),
            SmolStr::new("Int64"),
            SmolStr::new("Int32"),
        ],
        "Int64" | "Int32" => vec![type_name.clone(), SmolStr::new("Int")],
        _ => vec![type_name.clone()],
    }
}

/// Literal receivers type as Int64/Int32; `extension Int` mangles as `Int_*`.
pub(crate) fn remap_platform_int_extension_call(name: &SmolStr) -> SmolStr {
    for (from, to) in [("Int64_", "Int_"), ("Int32_", "Int_")] {
        if let Some(rest) = name.as_str().strip_prefix(from) {
            return SmolStr::new(format!("{to}{rest}"));
        }
    }
    name.clone()
}

pub(crate) fn mir_type_expr_name(ty: &TypeExpr) -> Option<SmolStr> {
    match ty {
        TypeExpr::Named { name, generics } if !generics.is_empty() => {
            let mut parts = Vec::with_capacity(generics.len());
            for arg in generics {
                parts.push(mir_type_expr_name(arg)?.to_string());
            }
            Some(SmolStr::new(format!(
                "{}<{}>",
                name.as_str(),
                parts.join(", ")
            )))
        }
        TypeExpr::Named { name, .. } => Some(name.clone()),
        TypeExpr::Array(inner) => Some(SmolStr::new(format!(
            "Array<{}>",
            mir_type_expr_name(inner)?.as_str()
        ))),
        TypeExpr::Dictionary { key, value } => {
            match (mir_type_expr_name(key), mir_type_expr_name(value)) {
                (Some(key_name), Some(value_name)) => Some(SmolStr::new(format!(
                    "Dictionary<{key_name}, {value_name}>"
                ))),
                _ => Some(SmolStr::new("Dictionary")),
            }
        }
        // S1-I7: `(Int) -> Void` stored properties dispatch via CallClosure.
        TypeExpr::Function { .. } => Some(SmolStr::new("Closure")),
        _ => None,
    }
}
