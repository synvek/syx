//! SYX semantic collect_decls face (S2-I250).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{has_attr, is_forbidden_internal_import, type_expr_to_id, Symbol, TypeId};
use indexmap::{IndexMap, IndexSet};
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{AccessLevel, Decl, Expr, ParamLabel, TypeExpr};
use syx_diagnostics::SyxResult;

impl SemanticAnalyzer {
    pub(crate) fn register_nested_type_path(
        &mut self,
        outer: &SmolStr,
        inner: &SmolStr,
        mangled: &SmolStr,
    ) {
        let dotted = syx_ast::nested_type_dotted_path(outer.as_str(), inner.as_str());
        self.nested_type_dotted
            .insert(dotted.clone(), mangled.clone());
        self.type_aliases.insert(
            dotted,
            TypeExpr::Named {
                name: mangled.clone(),
                generics: vec![],
            },
        );
    }

    fn collect_nested_type_decl(&mut self, outer: &SmolStr, decl: &Decl) {
        match decl {
            Decl::Class {
                name,
                span,
                access,
                members,
                conformances,
                ..
            } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.register_nested_type_path(outer, name, &qualified);
                self.class_names.insert(qualified.clone());
                self.type_decl_spans.insert(qualified.clone(), *span);
                let mut methods = Vec::new();
                for member in members {
                    if let Decl::Function {
                        name: method,
                        return_type,
                        ..
                    } = member
                    {
                        let ret = return_type
                            .as_ref()
                            .map(type_expr_to_id)
                            .unwrap_or(TypeId::Void);
                        methods.push((method.clone(), ret));
                    }
                }
                if !methods.is_empty() {
                    self.class_methods.insert(qualified.clone(), methods);
                }
                if !conformances.is_empty() {
                    self.type_conformances
                        .insert(qualified.clone(), conformances.clone());
                }
                self.record_member_typealias_bindings(&qualified, members);
                self.globals.insert(
                    qualified.clone(),
                    Symbol {
                        name: qualified.clone(),
                        ty: TypeId::Named(qualified.clone()),
                        mutable: false,
                        access: *access,
                    },
                );
                for member in members {
                    if matches!(
                        member,
                        Decl::Class { .. }
                            | Decl::Struct { .. }
                            | Decl::Enum { .. }
                            | Decl::FinalClass { .. }
                    ) {
                        self.collect_nested_type_decl(&qualified, member);
                    }
                }
            }
            Decl::Struct {
                name,
                span,
                access,
                fields,
                members,
                ..
            } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.register_nested_type_path(outer, name, &qualified);
                self.class_names.insert(qualified.clone());
                self.type_decl_spans.insert(qualified.clone(), *span);
                let mut field_types: Vec<(SmolStr, TypeId)> = fields
                    .iter()
                    .map(|(n, t)| (n.clone(), type_expr_to_id(t)))
                    .collect();
                for member in members {
                    if let Decl::VarGlobal {
                        name: field,
                        ty: Some(ty),
                        ..
                    } = member
                    {
                        field_types.push((field.clone(), type_expr_to_id(ty)));
                    }
                }
                if !field_types.is_empty() {
                    self.dataclass_fields.insert(qualified.clone(), field_types);
                }
                self.globals.insert(
                    qualified.clone(),
                    Symbol {
                        name: qualified.clone(),
                        ty: TypeId::Named(qualified.clone()),
                        mutable: false,
                        access: *access,
                    },
                );
            }
            Decl::Enum {
                name,
                access,
                type_params,
                cases,
                conformances,
                ..
            } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.register_nested_type_path(outer, name, &qualified);
                if !type_params.is_empty() {
                    self.generic_enums
                        .insert(qualified.clone(), (type_params.clone(), vec![]));
                }
                let case_names: Vec<SmolStr> = cases.iter().map(|(c, _, _)| c.clone()).collect();
                for (case, _, fields) in cases {
                    if !fields.is_empty() {
                        self.enum_case_field_types
                            .insert((qualified.clone(), case.clone()), fields.clone());
                    }
                }
                self.enum_cases.insert(qualified.clone(), case_names);
                if !conformances.is_empty() {
                    self.type_conformances
                        .insert(qualified.clone(), conformances.clone());
                }
                self.globals.insert(
                    qualified.clone(),
                    Symbol {
                        name: qualified.clone(),
                        ty: TypeId::Named(qualified.clone()),
                        mutable: false,
                        access: *access,
                    },
                );
            }
            Decl::FinalClass {
                name,
                access,
                type_params,
                variants,
                ..
            } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.register_nested_type_path(outer, name, &qualified);
                if !type_params.is_empty() {
                    self.generic_final_classes
                        .insert(qualified.clone(), (type_params.clone(), vec![]));
                }
                let variant_names: Vec<SmolStr> = variants.iter().map(|(v, _)| v.clone()).collect();
                for (variant, fields) in variants {
                    if !fields.is_empty() {
                        self.enum_case_field_types
                            .insert((qualified.clone(), variant.clone()), fields.clone());
                    }
                }
                self.enum_cases.insert(qualified.clone(), variant_names);
                self.globals.insert(
                    qualified.clone(),
                    Symbol {
                        name: qualified.clone(),
                        ty: TypeId::Named(qualified.clone()),
                        mutable: false,
                        access: *access,
                    },
                );
            }
            _ => {}
        }
    }

    pub(crate) fn collect_decls(&mut self, decls: &[Decl]) -> SyxResult<()> {
        for decl in decls {
            match decl {
                Decl::Import { span, path, .. } => {
                    if let Some(first) = path.first() {
                        if is_forbidden_internal_import(first.as_str()) {
                            self.errors.push(self.error_at(
                                format!(
                                    "Module '{}' is internal to the compiler and cannot be imported",
                                    path.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(".")
                                ),
                                *span,
                                "M003",
                            ));
                        }
                    }
                    if let Some(path) = decl.import_path_str() {
                        self.imported_modules
                            .push(SmolStr::new(path.split('.').next().unwrap_or(&path)));
                    }
                }
                Decl::Function {
                    name,
                    span,
                    access,
                    attrs,
                    type_params,
                    where_clause,
                    throws_type,
                    return_type,
                    params,
                    ..
                } => {
                    if !self.defined_functions.insert(name.clone()) {
                        self.errors.push(self.error_at(
                            format!("Function '{name}' is already defined"),
                            *span,
                            "S001",
                        ));
                    }
                    self.check_prelude_name_shadow(name, *span, "function");
                    self.collect_function_isolation(name, attrs);
                    if throws_type.is_some() {
                        self.throwing_functions
                            .insert(name.clone(), throws_type.as_ref().map(type_expr_to_id));
                    }
                    if type_params.is_empty() {
                        self.function_params.insert(
                            name.clone(),
                            params
                                .iter()
                                .map(|p| {
                                    p.ty.as_ref()
                                        .map(type_expr_to_id)
                                        .unwrap_or(TypeId::Unknown)
                                })
                                .collect(),
                        );
                        self.function_param_labels.insert(
                            name.clone(),
                            params
                                .iter()
                                .map(|p| (p.external_label.clone(), p.name.clone()))
                                .collect(),
                        );
                        self.function_sending_params
                            .insert(name.clone(), params.iter().map(|p| p.sending).collect());
                        if let Some(rt) = return_type {
                            self.function_return_types
                                .insert(name.clone(), type_expr_to_id(rt));
                        }
                        self.globals.insert(
                            name.clone(),
                            Symbol {
                                name: name.clone(),
                                ty: TypeId::Void,
                                mutable: false,
                                access: *access,
                            },
                        );
                    } else {
                        self.generic_functions
                            .insert(name.clone(), (type_params.clone(), where_clause.clone()));
                        if let Some(rt) = return_type {
                            self.function_return_types
                                .insert(name.clone(), type_expr_to_id(rt));
                        }
                        self.function_param_labels.insert(
                            name.clone(),
                            params
                                .iter()
                                .map(|p| (p.external_label.clone(), p.name.clone()))
                                .collect(),
                        );
                        self.globals.insert(
                            name.clone(),
                            Symbol {
                                name: name.clone(),
                                ty: TypeId::GenericFn {
                                    name: name.clone(),
                                    type_params: type_params.clone(),
                                },
                                mutable: false,
                                access: *access,
                            },
                        );
                    }
                }
                Decl::VarGlobal {
                    name,
                    mutable,
                    ty,
                    init,
                    access,
                    ..
                } => {
                    let inferred = if matches!(init, Some(Expr::Closure { .. })) {
                        TypeId::Named(SmolStr::new("Closure"))
                    } else {
                        TypeId::Unknown
                    };
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: ty.as_ref().map(type_expr_to_id).unwrap_or(inferred),
                            mutable: *mutable,
                            access: *access,
                        },
                    );
                }
                Decl::Class {
                    name,
                    span,
                    access,
                    attrs,
                    superclass,
                    conformances,
                    members,
                    ..
                } => {
                    self.check_prelude_name_shadow(name, *span, "class");
                    if attrs.iter().any(|a| {
                        matches!(
                            a.name.as_str(),
                            "Service" | "Component" | "service" | "component"
                        )
                    }) {
                        self.service_class_names.insert(name.clone());
                    }
                    if let Some(base) = superclass {
                        if !self.class_names.contains(base) {
                            self.errors.push(self.error_at(
                                format!("Unknown superclass '{base}' for class '{name}'"),
                                *span,
                                "S050",
                            ));
                        } else {
                            self.class_superclasses.insert(name.clone(), base.clone());
                        }
                    }
                    self.class_names.insert(name.clone());
                    self.type_decl_spans.insert(name.clone(), *span);
                    let mut methods = Vec::new();
                    let mut fields: Vec<(SmolStr, TypeId)> = Vec::new();
                    let mut prop_getters: IndexMap<SmolStr, TypeId> = IndexMap::new();
                    let mut prop_setters: HashSet<SmolStr> = HashSet::new();
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            ty,
                            computed_getter,
                            computed_setter,
                            ..
                        } = member
                        {
                            let field_ty =
                                ty.as_ref().map(type_expr_to_id).unwrap_or(TypeId::Unknown);
                            fields.push((field.clone(), field_ty));
                            if computed_getter.is_some() && computed_setter.is_none() {
                                self.getter_only_properties
                                    .insert((name.clone(), field.clone()));
                            }
                        }
                        if let Decl::Function {
                            name: method,
                            return_type,
                            is_override,
                            is_final,
                            span: mspan,
                            params,
                            ..
                        } = member
                        {
                            // Desugared accessors → surface property name for completion / assign checks.
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
                            methods.push((method.clone(), ret.clone()));
                            if *is_final {
                                self.final_methods.insert((name.clone(), method.clone()));
                            }
                            if let Some(base) = superclass {
                                if let Some(base_methods) = self.class_methods.get(base) {
                                    if let Some((_, base_ret)) =
                                        base_methods.iter().find(|(m, _)| m == method)
                                    {
                                        if !*is_override {
                                            self.errors.push(self.error_at(
                                                format!(
                                                    "Method '{method}' overrides '{base}.{method}' but is missing 'override'"
                                                ),
                                                *mspan,
                                                "S052",
                                            ));
                                        }
                                        if &ret != base_ret {
                                            self.errors.push(self.error_at(
                                                format!(
                                                    "Override of '{base}.{method}' has incompatible return type"
                                                ),
                                                *mspan,
                                                "S053",
                                            ));
                                        }
                                        if self
                                            .final_methods
                                            .contains(&(base.clone(), method.clone()))
                                        {
                                            self.errors.push(self.error_at(
                                                format!(
                                                    "Method '{base}.{method}' is final and cannot be overridden"
                                                ),
                                                *mspan,
                                                "S054",
                                            ));
                                        }
                                    } else if *is_override {
                                        self.errors.push(self.error_at(
                                            format!(
                                                "'override' method '{method}' does not override any method in '{base}'"
                                            ),
                                            *mspan,
                                            "S051",
                                        ));
                                    }
                                } else if *is_override {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "'override' method '{method}' does not override any method in '{base}'"
                                        ),
                                        *mspan,
                                        "S051",
                                    ));
                                }
                            } else if *is_override {
                                self.errors.push(self.error_at(
                                    format!(
                                        "'override' method '{method}' in '{name}' has no superclass"
                                    ),
                                    *mspan,
                                    "S051",
                                ));
                            }
                            let _ = params;
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
                    for member in members {
                        if let Decl::Init {
                            params, failable, ..
                        } = member
                        {
                            let init_name = SmolStr::new(format!("{name}_init"));
                            self.function_param_labels.insert(
                                init_name,
                                params
                                    .iter()
                                    .map(|p| (p.external_label.clone(), p.name.clone()))
                                    .collect(),
                            );
                            if *failable {
                                self.failable_inits.insert(name.clone());
                            }
                        }
                        if let Decl::Function {
                            name: method,
                            params,
                            type_params,
                            where_clause,
                            attrs,
                            ..
                        } = member
                        {
                            let method_name = SmolStr::new(format!("{name}_{method}"));
                            self.function_param_labels.insert(
                                method_name.clone(),
                                params
                                    .iter()
                                    .map(|p| (p.external_label.clone(), p.name.clone()))
                                    .collect(),
                            );
                            if !type_params.is_empty()
                                && attrs.iter().any(|a| a.name.as_str() == "static")
                            {
                                self.generic_functions.insert(
                                    method_name,
                                    (type_params.clone(), where_clause.clone()),
                                );
                            }
                        }
                    }
                    if !methods.is_empty() {
                        self.class_methods.insert(name.clone(), methods);
                    }
                    if !conformances.is_empty() {
                        self.type_conformances
                            .insert(name.clone(), conformances.clone());
                    }
                    self.record_member_typealias_bindings(name, members);
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: *access,
                        },
                    );
                    for member in members {
                        if matches!(
                            member,
                            Decl::Class { .. }
                                | Decl::Struct { .. }
                                | Decl::Enum { .. }
                                | Decl::FinalClass { .. }
                        ) {
                            self.collect_nested_type_decl(name, member);
                        }
                    }
                }
                Decl::Actor {
                    name,
                    span,
                    access,
                    conformances,
                    members,
                    ..
                } => {
                    self.check_prelude_name_shadow(name, *span, "actor");
                    self.actor_types.insert(name.clone());
                    let fields: HashSet<SmolStr> = members
                        .iter()
                        .filter_map(|m| {
                            if let Decl::VarGlobal { name: field, .. } = m {
                                Some(field.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.actor_fields.insert(name.clone(), fields);
                    for member in members {
                        if let Decl::Function {
                            name: method,
                            params,
                            ..
                        } = member
                        {
                            self.actor_method_sending.insert(
                                (name.clone(), method.clone()),
                                params.iter().map(|p| p.sending).collect(),
                            );
                        }
                    }
                    if !conformances.is_empty() {
                        self.type_conformances
                            .insert(name.clone(), conformances.clone());
                    }
                    self.record_member_typealias_bindings(name, members);
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: *access,
                        },
                    );
                }
                Decl::GlobalActor { name, access, .. } => {
                    self.global_actor_names.insert(name.clone());
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: *access,
                        },
                    );
                }
                Decl::DataClass {
                    name,
                    span,
                    access,
                    attrs,
                    type_params,
                    where_clause,
                    fields,
                    ..
                } => {
                    self.check_dataclass_codable_witness(
                        name,
                        *span,
                        attrs,
                        type_params,
                        where_clause,
                        fields,
                    );
                    self.dataclass_names.insert(name.clone());
                    self.type_decl_spans.insert(name.clone(), *span);
                    let field_types: Vec<(SmolStr, TypeId)> = fields
                        .iter()
                        .map(|(f, ty)| (f.clone(), type_expr_to_id(ty)))
                        .collect();
                    self.dataclass_fields.insert(name.clone(), field_types);
                    if has_attr(attrs, "MainActor") {
                        self.main_actor_types.insert(name.clone());
                    }
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: *access,
                        },
                    );
                }
                Decl::Struct {
                    name,
                    span,
                    access,
                    fields,
                    members,
                    ..
                } => {
                    // S1-I2: Struct joins DataClass construction / field lookup path.
                    self.dataclass_names.insert(name.clone());
                    self.type_decl_spans.insert(name.clone(), *span);
                    let mut field_types: Vec<(SmolStr, TypeId)> = fields
                        .iter()
                        .map(|(f, ty)| (f.clone(), type_expr_to_id(ty)))
                        .collect();
                    let mut methods: Vec<(SmolStr, TypeId)> = Vec::new();
                    for member in members {
                        if let Decl::VarGlobal {
                            name: field,
                            ty: Some(ty),
                            ..
                        } = member
                        {
                            let field_ty = type_expr_to_id(ty);
                            if let Some(slot) = field_types.iter_mut().find(|(f, _)| f == field) {
                                slot.1 = field_ty;
                            } else {
                                field_types.push((field.clone(), field_ty));
                            }
                        }
                        if let Decl::Function {
                            name: method,
                            return_type,
                            params,
                            attrs,
                            ..
                        } = member
                        {
                            let ret = return_type
                                .as_ref()
                                .map(type_expr_to_id)
                                .unwrap_or(TypeId::Void);
                            methods.push((method.clone(), ret.clone()));
                            if attrs.iter().any(|a| a.name.as_str() == "mutating") {
                                self.mutating_methods.insert((name.clone(), method.clone()));
                            }
                            let method_name = SmolStr::new(format!("{name}_{method}"));
                            self.function_param_labels.insert(
                                method_name.clone(),
                                params
                                    .iter()
                                    .map(|p| (p.external_label.clone(), p.name.clone()))
                                    .collect(),
                            );
                            self.function_return_types.insert(method_name, ret);
                        }
                        if let Decl::Init { params, .. } = member {
                            let init_name = SmolStr::new(format!("{name}_init"));
                            self.function_param_labels.insert(
                                init_name,
                                params
                                    .iter()
                                    .map(|p| (p.external_label.clone(), p.name.clone()))
                                    .collect(),
                            );
                        }
                    }
                    if !field_types.is_empty() {
                        self.dataclass_fields
                            .insert(name.clone(), field_types.clone());
                    }
                    // Memberwise init labels (book `Point(x:y:)` / positional).
                    if !self
                        .function_param_labels
                        .contains_key(&SmolStr::new(format!("{name}_init")))
                    {
                        let init_name = SmolStr::new(format!("{name}_init"));
                        self.function_param_labels.insert(
                            init_name,
                            field_types
                                .iter()
                                .map(|(f, _)| (ParamLabel::Same, f.clone()))
                                .collect(),
                        );
                    }
                    if !methods.is_empty() {
                        self.class_methods.insert(name.clone(), methods);
                    }
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: *access,
                        },
                    );
                }
                Decl::FinalClass {
                    name,
                    access,
                    type_params,
                    variants,
                    ..
                } => {
                    if !type_params.is_empty() {
                        self.generic_final_classes
                            .insert(name.clone(), (type_params.clone(), vec![]));
                    }
                    let variant_names: Vec<SmolStr> =
                        variants.iter().map(|(v, _)| v.clone()).collect();
                    for (variant, fields) in variants {
                        if !fields.is_empty() {
                            self.enum_case_field_types
                                .insert((name.clone(), variant.clone()), fields.clone());
                        }
                    }
                    self.enum_cases.insert(name.clone(), variant_names);
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: *access,
                        },
                    );
                }
                Decl::TypeAlias {
                    name, ty, access, ..
                } => {
                    self.type_aliases.insert(name.clone(), ty.clone());
                    let resolved = self.resolve_type_expr(ty);
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: resolved,
                            mutable: false,
                            access: *access,
                        },
                    );
                }
                Decl::Extension {
                    type_name,
                    conformances,
                    where_clause,
                    methods,
                    ..
                } => {
                    // ADR-324: where-gated protocol extensions are conditional, not unconditional.
                    if !conformances.is_empty() {
                        if where_clause.is_empty() {
                            self.type_conformances
                                .entry(type_name.clone())
                                .or_default()
                                .extend(conformances.iter().cloned());
                        } else {
                            for protocol in conformances {
                                self.conditional_conformances.push((
                                    type_name.clone(),
                                    protocol.clone(),
                                    where_clause.clone(),
                                ));
                            }
                        }
                    }
                    let mut ext_methods = Vec::new();
                    let mut ext_prop_getters: IndexMap<SmolStr, TypeId> = IndexMap::new();
                    let mut ext_prop_setters: IndexSet<SmolStr> = IndexSet::new();
                    for method in methods {
                        // S1-I12: nested types declared inside `extension Outer { struct Inner … }`.
                        if matches!(
                            method,
                            Decl::Class { .. }
                                | Decl::Struct { .. }
                                | Decl::Enum { .. }
                                | Decl::FinalClass { .. }
                        ) {
                            self.collect_nested_type_decl(type_name, method);
                            continue;
                        }
                        if let Decl::Function {
                            name,
                            access,
                            return_type,
                            ..
                        } = method
                        {
                            let ret = return_type
                                .as_ref()
                                .map(type_expr_to_id)
                                .unwrap_or(TypeId::Void);
                            // Desugared accessors → surface property name (completion / assign).
                            if let Some(field) = name.as_str().strip_prefix("__get_") {
                                let field = SmolStr::new(field);
                                ext_prop_getters.insert(field.clone(), ret.clone());
                                ext_methods.push((field.clone(), ret.clone()));
                                if !where_clause.is_empty() {
                                    self.extension_method_where
                                        .insert((type_name.clone(), field), where_clause.clone());
                                }
                            } else if let Some(field) = name.as_str().strip_prefix("__set_") {
                                ext_prop_setters.insert(SmolStr::new(field));
                            } else {
                                ext_methods.push((name.clone(), ret.clone()));
                                if !where_clause.is_empty() {
                                    self.extension_method_where.insert(
                                        (type_name.clone(), name.clone()),
                                        where_clause.clone(),
                                    );
                                }
                            }
                            if name.as_str() == "__subscript_get" {
                                self.subscript_types.insert(type_name.clone());
                                self.subscript_return_types
                                    .insert(type_name.clone(), ret.clone());
                            }
                            if name.as_str() == "__subscript_set" {
                                self.subscript_set_types.insert(type_name.clone());
                            }
                            // Keep mangled `__get_`/`__set_` symbols for Call resolution.
                            let mangled = SmolStr::new(format!("{type_name}_{name}"));
                            self.function_return_types
                                .insert(mangled.clone(), ret.clone());
                            self.globals.insert(
                                mangled,
                                Symbol {
                                    name: SmolStr::new(format!("{type_name}_{name}")),
                                    ty: ret,
                                    mutable: false,
                                    access: *access,
                                },
                            );
                        }
                    }
                    if !ext_prop_getters.is_empty() {
                        let fields = self.dataclass_fields.entry(type_name.clone()).or_default();
                        for (field, field_ty) in &ext_prop_getters {
                            if !fields.iter().any(|(f, _)| f == field) {
                                fields.push((field.clone(), field_ty.clone()));
                            }
                            if !ext_prop_setters.contains(field) {
                                self.getter_only_properties
                                    .insert((type_name.clone(), field.clone()));
                            }
                        }
                    }
                    if !ext_methods.is_empty() {
                        self.type_extension_methods
                            .entry(type_name.clone())
                            .or_default()
                            .extend(ext_methods);
                    }
                }
                Decl::Enum {
                    name,
                    access,
                    attrs,
                    type_params,
                    cases,
                    conformances,
                    ..
                } => {
                    if !type_params.is_empty() {
                        self.generic_enums
                            .insert(name.clone(), (type_params.clone(), vec![]));
                    }
                    let case_names: Vec<SmolStr> =
                        cases.iter().map(|(c, _, _)| c.clone()).collect();
                    for (case, _, fields) in cases {
                        if !fields.is_empty() {
                            self.enum_case_field_types
                                .insert((name.clone(), case.clone()), fields.clone());
                        }
                    }
                    self.enum_cases.insert(name.clone(), case_names.clone());
                    // S1-I4: `@Equatable enum` → protocol table (tag/case `==`).
                    if attrs.iter().any(|a| a.name.as_str() == "Equatable") {
                        self.type_conformances
                            .entry(name.clone())
                            .or_default()
                            .push(SmolStr::new("Equatable"));
                    }
                    // SA-2-02: `enum E: Error` (and other protocol lists) → conformance table.
                    if !conformances.is_empty() {
                        self.type_conformances
                            .entry(name.clone())
                            .or_default()
                            .extend(conformances.iter().cloned());
                    }
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: *access,
                        },
                    );
                    for case in case_names {
                        self.globals.insert(
                            case.clone(),
                            Symbol {
                                name: case.clone(),
                                ty: TypeId::Named(name.clone()),
                                mutable: false,
                                access: AccessLevel::Public,
                            },
                        );
                    }
                }
                Decl::Init { .. } | Decl::Deinit { .. } | Decl::TopLevelStmt { .. } => {}
                Decl::MacroDecl { .. }
                | Decl::MacroSiteInvoke { .. }
                | Decl::Subscript { .. }
                | Decl::OperatorFunc { .. } => {}
                Decl::Export { item, .. } => {
                    self.collect_decls(std::slice::from_ref(item.as_ref()))?;
                }
                Decl::ExportList { .. } => {}
                Decl::Protocol { name, span, .. } => {
                    // Phase 125: local protocols used as `: Protocol` superclass syntax (protocol binding).
                    self.class_names.insert(name.clone());
                    self.protocol_names.insert(name.clone());
                    self.type_decl_spans.insert(name.clone(), *span);
                    self.globals.insert(
                        name.clone(),
                        Symbol {
                            name: name.clone(),
                            ty: TypeId::Named(name.clone()),
                            mutable: false,
                            access: AccessLevel::Internal,
                        },
                    );
                }
            }
        }
        Ok(())
    }
}
