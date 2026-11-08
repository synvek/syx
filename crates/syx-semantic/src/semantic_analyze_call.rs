//! SYX semantic analyze_call face (S2-I254).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{
    expr_span, is_compatible, is_retired_collection_methodcall, reflect_type_lower,
    simd_lanes_from_type_name, type_collection_surface, type_id_to_name, type_name_to_id, Scope,
    ScopeError, Symbol, TypeId,
};
use crate::import_resolver::{
    check_import_member_access as check_import_access, ImportAccessError,
};
use crate::type_checker::check_assignable;
use smol_str::SmolStr;
use syx_ast::{
    CallArg, Expr, Literal, ParamLabel,
};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_stdlib::{
    from_suffix, host_fn_id_by_name,
    host_fn_index_by_name, host_fn_return_type, host_return_type_by_name, is_float,
    is_implicit_convertible, is_integer,
    lookup_array_method,
    lookup_type_method,
    plugin_host_return_type, MethodId, NumericKind,
};

impl SemanticAnalyzer {
    pub(crate) fn check_import_member_access(
        &self,
        module: &str,
        member: &str,
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        check_import_access(
            &self.imported_modules,
            &self.import_exports,
            module,
            member,
        )
        .map_err(|e| match e {
            ImportAccessError::NonPublic { module, member } => self.error_at(
                format!("Cannot access non-public symbol '{module}.{member}'"),
                span,
                "S005",
            ),
        })
    }

    pub(crate) fn define_in_scope(
        &mut self,
        scope: &mut Scope,
        symbol: Symbol,
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        scope.define(symbol).map_err(|e| match e {
            ScopeError::Duplicate(name) => self.error_at(
                format!("Symbol '{name}' already defined in this scope"),
                span,
                "S001",
            ),
        })
    }

    pub(crate) fn define_symbol(
        &mut self,
        symbol: Symbol,
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        let name = symbol.name.clone();
        if self.scope.symbols.contains_key(&name) {
            return Err(self.error_at(
                format!("Symbol '{name}' already defined in this scope"),
                span,
                "S001",
            ));
        }
        self.scope.symbols.insert(name, symbol);
        Ok(())
    }

    fn lookup_binding_type(&self, name: &SmolStr) -> TypeId {
        self.scope
            .lookup(name)
            .or_else(|| self.globals.get(name))
            .map(|s| s.ty.clone())
            .unwrap_or(TypeId::Unknown)
    }

    fn param_consumes_value(&self, ty: &TypeId) -> bool {
        !matches!(ty, TypeId::Reference(_) | TypeId::MutReference(_))
    }

    fn is_move_type(&self, ty: &TypeId) -> bool {
        !matches!(ty, TypeId::Reference(_) | TypeId::MutReference(_))
            && matches!(ty, TypeId::String | TypeId::Named(_))
    }

    pub(crate) fn check_prelude_name_shadow(&mut self, name: &SmolStr, span: syx_diagnostics::Span, kind: &str) {
        if self.skip_prelude_name_shadow {
            return;
        }
        if syx_config::CORE_PRELUDE_MODULES
            .iter()
            .any(|m| *m == name.as_str())
        {
            self.errors.push(self.error_at(
                format!(
                    "{kind} '{name}' shadows a core prelude module; rename or use a qualified import"
                ),
                span,
                "S023",
            ));
        } else if syx_stdlib::is_builtin_type(name.as_str()) {
            // Host types owned by an importable module (HTTPResponse → Net) may
            // be declared in that module's source without counting as a shadow.
            let owned = syx_stdlib::lookup_manifest_entry(name.as_str())
                .is_some_and(|e| e.owner_module.is_some());
            if !owned {
                self.errors.push(self.error_at(
                    format!("{kind} '{name}' shadows a builtin type name"),
                    span,
                    "S023",
                ));
            }
        }
    }

    pub(crate) fn error_at(&self, message: impl Into<String>, span: syx_diagnostics::Span, code: &str) -> SyxDiagnostic {
        let mut diag = SyxDiagnostic {
            message: message.into(),
            severity: syx_diagnostics::Severity::Error,
            code: Some(code.to_string()),
            source: None,
            primary_span: None,
        label: None,
        help: None,
        };
        if span != syx_diagnostics::Span::DUMMY {
            if let Some(ref src) = self.source {
                diag = diag.with_span(src, span, "");
            }
        }
        diag
    }

    /// Bare stored property of the enclosing struct/class (book `value = value + 1`).
    pub(crate) fn lookup_enclosing_type_field(&self, name: &SmolStr) -> Option<TypeId> {
        let type_name = self.current_class_context.as_ref()?;
        self.dataclass_fields
            .get(type_name)
            .and_then(|fields| fields.iter().find(|(f, _)| f == name).map(|(_, ty)| ty.clone()))
    }

    /// S1-06 / S060: `let` bindings cannot invoke mutating methods (incl. `Array.append`).
    /// Distinct from MainActor `S020`.
    fn check_mutating_call_on_immutable(
        &mut self,
        object: &Expr,
        obj_ty: &TypeId,
        method: &str,
        span: syx_diagnostics::Span,
    ) {
        // Keep this list to in-place mutators. Do NOT include Core copy-returning
        // facades like `Array.reverse()` / `sorted()` (packages/core … array.sy).
        let known_collection_mutating = matches!(
            method,
            "append"
                | "insert"
                | "remove"
                | "removeAll"
                | "removeFirst"
                | "removeLast"
                | "popLast"
        );
        let marked_mutating = match obj_ty {
            TypeId::Named(type_name) => self
                .mutating_methods
                .contains(&(type_name.clone(), SmolStr::new(method))),
            _ => false,
        };
        if !known_collection_mutating && !marked_mutating {
            return;
        }
        let Some(binding) = Self::mutating_receiver_binding(object) else {
            return;
        };
        let Some(sym) = self.scope.lookup(&binding) else {
            return;
        };
        if sym.mutable {
            return;
        }
        self.errors.push(self.error_at(
            format!(
                "Cannot call mutating method '{method}' on immutable binding '{binding}'"
            ),
            span,
            "S060",
        ));
    }

    fn mutating_receiver_binding(object: &Expr) -> Option<SmolStr> {
        match object {
            Expr::Ident { name, .. } => Some(name.clone()),
            // `self.field.append(...)` — mutate through stored property; not a let-binding reject.
            _ => None,
        }
    }

    fn is_callable_function(&self, name: &SmolStr) -> bool {
        if self.function_return_types.contains_key(name)
            || self.function_params.contains_key(name)
        {
            return true;
        }
        if host_fn_id_by_name(name.as_str()).is_some() {
            return true;
        }
        if host_fn_index_by_name(name.as_str()).is_some() {
            return true;
        }
        if matches!(name.as_str(), "print" | "readLine" | "Ok" | "Err" | "reflect") {
            return true;
        }
        if self.class_names.contains(name)
            || self.dataclass_names.contains(name)
            || self.actor_types.contains(name)
        {
            return true;
        }
        if self
            .enum_cases
            .values()
            .any(|cases| cases.iter().any(|c| c == name))
        {
            return true;
        }
        if self
            .import_exports
            .values()
            .any(|exports| exports.contains_key(name))
        {
            return true;
        }
        if let Some((mod_name, member)) = name.split_once('_') {
            if member.chars().next().is_some_and(|c| c.is_uppercase())
                && self
                    .import_exports
                    .get(mod_name)
                    .is_some_and(|exports| exports.contains_key(member))
            {
                return true;
            }
        }
        if self.globals.contains_key(name) && name.as_str().starts_with("__syx_") {
            return true;
        }
        false
    }

    fn check_call_argument_labels(
        &mut self,
        callee_name: &SmolStr,
        args: &[CallArg],
        span: syx_diagnostics::Span,
    ) {
        let Some(param_labels) = self.function_param_labels.get(callee_name) else {
            return;
        };
        if param_labels.is_empty() {
            return;
        }

        let all_omitted = param_labels
            .iter()
            .all(|(ext, _)| matches!(ext, ParamLabel::Omitted));
        let all_positional = args.iter().all(|a| a.label.is_none());

        if all_positional {
            if all_omitted {
                return;
            }
            for (ext, internal) in param_labels {
                if let Some(expected) = ext.call_site_label(internal) {
                    self.errors.push(
                        self.error_at(
                            format!(
                                "Missing argument label '{expected}:' in call to '{callee_name}'"
                            ),
                            span,
                            "S042",
                        ),
                    );
                }
            }
            return;
        }

        for (i, arg) in args.iter().enumerate() {
            let Some((ext, internal)) = param_labels.get(i) else {
                continue;
            };
            match (&arg.label, ext) {
                (None, ParamLabel::Omitted) => {}
                (None, _) => {
                    if let Some(expected) = ext.call_site_label(internal) {
                        self.errors.push(
                            self.error_at(
                                format!(
                                    "Missing argument label '{expected}:' in call to '{callee_name}'"
                                ),
                                span,
                                "S042",
                            ),
                        );
                    }
                }
                (Some(_label), ParamLabel::Omitted) => {
                    self.errors.push(
                        self.error_at(
                            format!(
                                "Argument #{i} to '{callee_name}' must be positional (no label)"
                            ),
                            span,
                            "S042",
                        ),
                    );
                }
                (Some(label), ext) => {
                    let expected = ext.call_site_label(internal);
                    if expected != Some(label) {
                        let expected = expected.map(|s| s.as_str()).unwrap_or("<none>");
                        self.errors.push(
                            self.error_at(
                                format!(
                                    "Unknown argument label '{label}' in call to '{callee_name}' (expected '{expected}')"
                                ),
                                span,
                                "S042",
                            ),
                        );
                    }
                }
            }
        }
    }

    pub(crate) fn analyze_call(&mut self, callee: &Expr, args: &[CallArg]) -> SyxResult<TypeId> {
        match callee {
            Expr::Ident { name, span, .. } => {
                if name.as_str() == "reflect" {
                    if let Some(arg) = args.first() {
                        if let Expr::TypeSelf { type_name, .. } = &arg.value {
                            let _ = reflect_type_lower(type_name.as_str());
                            return Ok(TypeId::Named(SmolStr::new("Object")));
                        }
                    }
                    return Err(self.error_at(
                        "reflect() expects a type metatype (e.g. reflect(Point.self))",
                        *span,
                        "S002",
                    ));
                }
                if name.as_str() == "__syx_reflect_specialization" {
                    self.validate_reflect_specialization_call(args, *span)?;
                    return Ok(TypeId::Named(SmolStr::new("Object")));
                }
                if name.as_str() == "__syx_service_resolve" {
                    return self.analyze_service_resolve(args, *span);
                }
                if name.as_str() == "__syx_service_resolve_dynamic" {
                    if let Some(arg) = args.first() {
                        self.analyze_expr(&arg.value)?;
                    }
                    return Ok(TypeId::Named(SmolStr::new("Object")));
                }
                if name.as_str() == "resolve" {
                    if let Some(arg) = args.first() {
                        self.analyze_expr(&arg.value)?;
                    }
                    return Ok(TypeId::Named(SmolStr::new("Object")));
                }
                if name.as_str() == "__syx_channel_send" {
                    for arg in args {
                        self.analyze_expr(&arg.value)?;
                    }
                    if let Some(arg) = args.get(1) {
                        let non_sendable = match &arg.value {
                            Expr::Ident { name: var, .. } => self.is_non_sendable_binding(var),
                            other => self
                                .analyze_expr(other)
                                .map(|ty| self.is_non_sendable_type(&ty))
                                .unwrap_or(false),
                        };
                        if non_sendable {
                            self.errors.push(self.error_at(
                                "Non-Sendable value cannot be sent on a channel",
                                *span,
                                "S019",
                            ));
                        }
                    }
                    return Ok(TypeId::Void);
                }
                if self.throwing_functions.contains_key(name) && self.try_depth == 0 {
                    self.errors.push(self.error_at(
                        format!(
                            "Call to throwing function '{name}' must use try or appear inside a try block"
                        ),
                        *span,
                        "S018",
                    ));
                }
                for arg in args {
                    self.analyze_expr(&arg.value)?;
                }
                if self.function_param_labels.contains_key(name) {
                    self.check_call_argument_labels(name, args, *span);
                }
                if self.class_names.contains(name) {
                    let init_name = SmolStr::new(format!("{name}_init"));
                    self.check_call_argument_labels(&init_name, args, *span);
                } else if self.dataclass_names.contains(name) {
                    // Struct/DataClass: empty call uses memberwise defaults (Counter());
                    // labeled/partial calls still check labels when args present.
                    if !args.is_empty() {
                        let init_name = SmolStr::new(format!("{name}_init"));
                        self.check_call_argument_labels(&init_name, args, *span);
                    }
                }
                if let Some(params) = self.function_params.get(name).cloned() {
                    for (arg, param_ty) in args.iter().zip(params.iter()) {
                        if let Expr::Ident { name: var, .. } = &arg.value {
                            if self.param_consumes_value(param_ty)
                                && self.is_move_type(&self.lookup_binding_type(var))
                            {
                                self.moved_bindings.insert(var.clone());
                            }
                        }
                    }
                }
                if let Some(TypeId::GenericFn {
                    name: fn_name,
                    type_params,
                }) = self.globals.get(name).map(|s| s.ty.clone())
                {
                    let type_args = self.infer_call_type_args(type_params.len(), args);
                    self.validate_generic_instantiation(&fn_name, &type_args, *span);
                    let ret = self.instantiate_generic_return(&fn_name, &type_params, &type_args);
                    self.record_instantiation(fn_name, type_args);
                    return Ok(ret);
                }
                if let Some(rt) = self.function_return_types.get(name) {
                    return Ok(rt.clone());
                }
                if let Some((mod_name, member)) = name.split_once('_') {
                    if member.chars().next().is_some_and(|c| c.is_uppercase())
                        && self.import_exports.contains_key(&SmolStr::new(mod_name))
                    {
                        return Ok(TypeId::Named(SmolStr::new(member)));
                    }
                }
                if name.as_str().starts_with("__syx_plugin_") {
                    let cfg = syx_config::runtime_config();
                    if cfg.permission.sandbox && !cfg.permission.allow_plugin {
                        return Err(self.error_at(
                            "plugin Host functions require permission grant; pass --allow-plugin (or SYX_ALLOW_PLUGIN=1) under --permission-sandbox",
                            *span,
                            "S024",
                        ));
                    }
                }
                // Local / param / global holding a closure or KeyPath: `f(1)` / `kp(root)`.
                if !self.is_callable_function(name) {
                    let binding_ty = self
                        .scope
                        .lookup(name)
                        .or_else(|| self.globals.get(name))
                        .map(|s| &s.ty);
                    if binding_ty.is_some_and(|ty| {
                        matches!(
                            ty,
                            TypeId::Named(n) if matches!(n.as_str(), "Closure" | "KeyPath")
                        )
                    }) {
                        for arg in args {
                            self.analyze_expr(&arg.value)?;
                        }
                        return Ok(TypeId::Unknown);
                    }
                    return Err(self.error_at(
                        format!("Undefined function '{name}'"),
                        *span,
                        "S002",
                    ));
                }
                // S1-I10: `init?` / `init!` → `Type(...)` yields Optional<Type>.
                if self.class_names.contains(name) || self.actor_types.contains(name) {
                    let named = TypeId::Named(name.clone());
                    if self.failable_inits.contains(name) {
                        return Ok(TypeId::Optional(Box::new(named)));
                    }
                    return Ok(named);
                }
                if self.dataclass_names.contains(name) {
                    return Ok(TypeId::Named(name.clone()));
                }
                let global_ty = self.globals.get(name).map(|sym| sym.ty.clone());
                if let Some(ty) = global_ty {
                    if let Some(id) = host_fn_id_by_name(name.as_str()) {
                        self.check_host_fn_allowed(id, *span);
                    }
                    Ok(ty)
                } else if let Some(id) = host_fn_id_by_name(name.as_str()) {
                    self.check_host_fn_allowed(id, *span);
                    Ok(type_name_to_id(host_fn_return_type(id)))
                } else if host_fn_index_by_name(name.as_str()).is_some() {
                    let ty = host_return_type_by_name(name.as_str())
                        .or_else(|| plugin_host_return_type(name.as_str()))
                        .unwrap_or("Void");
                    Ok(type_name_to_id(ty))
                } else {
                    Ok(TypeId::Void)
                }
            }
            Expr::Member { object, name, span, .. } => {
                if name.as_str() == "resolve"
                    && matches!(
                        object.as_ref(),
                        Expr::Ident { name: module, .. } if module.as_str() == "Di"
                    )
                {
                    if let Some(arg) = args.first() {
                        self.analyze_expr(&arg.value)?;
                    }
                    return Ok(TypeId::Named(SmolStr::new("Object")));
                }
                let obj_ty = self.analyze_expr(object)?;
                self.check_retired_size_alias(&obj_ty, name.as_str(), *span)?;
                if let Expr::Ident { name: module, .. } = object.as_ref() {
                    self.check_import_member_access(module.as_str(), name.as_str(), *span)?;
                    if name.chars().next().is_some_and(|c| c.is_uppercase()) {
                        for arg in args {
                            self.analyze_expr(&arg.value)?;
                        }
                        let init_name = SmolStr::new(format!("{module}_{name}_init"));
                        self.check_call_argument_labels(&init_name, args, *span);
                        return Ok(TypeId::Named(name.clone()));
                    }
                    if module.as_str() == "Crypto" {
                        if let Some(tm) = lookup_type_method("Crypto", name.as_str()) {
                            if let Some(id) = tm.host {
                                self.check_host_fn_allowed(id, *span);
                            }
                        }
                    }
                    let qualified = SmolStr::new(format!("{module}_{name}"));
                    let imported_ctor = name.chars().next().is_some_and(|c| c.is_uppercase())
                        && self.import_exports.contains_key(module);
                    if !imported_ctor {
                        if let Some((type_params, where_clause)) =
                            self.generic_functions.get(&qualified).cloned()
                        {
                            for arg in args {
                                self.analyze_expr(&arg.value)?;
                            }
                            let type_args =
                                self.infer_class_method_type_args(type_params.len(), args);
                            self.validate_generic_instantiation_with(
                                &qualified,
                                &type_params,
                                &where_clause,
                                &type_args,
                                *span,
                            );
                            self.record_instantiation(qualified, type_args);
                            return Ok(TypeId::Void);
                        }
                    }
                }
                let collection_elem = match (&obj_ty, name.as_str(), args.len()) {
                    (TypeId::Set(elem), "insert" | "remove" | "contains", 1)
                        if !self.is_open_generic_elem(elem.as_ref()) =>
                    {
                        Some((elem.as_ref().clone(), "item"))
                    }
                    (TypeId::Array(elem), "append" | "contains", 1)
                        if !self.is_open_generic_elem(elem.as_ref()) =>
                    {
                        Some((elem.as_ref().clone(), "item"))
                    }
                    (TypeId::Dictionary { key, .. }, "containsKey" | "remove", 1)
                        if !self.is_open_generic_elem(key.as_ref()) =>
                    {
                        Some((key.as_ref().clone(), "key"))
                    }
                    _ => None,
                };
                // S1-06: reject mutating methods / Array.append on `let` bindings.
                self.check_mutating_call_on_immutable(object, &obj_ty, name.as_str(), *span);
                if let Some((elem, role)) = collection_elem {
                    let found =
                        self.analyze_expr_with_expected(&args[0].value, Some(&elem))?;
                    self.check_assignable_with_literal(
                        elem,
                        found,
                        role,
                        Some(&args[0].value),
                        *span,
                    )?;
                } else {
                    for arg in args {
                        self.analyze_expr(&arg.value)?;
                    }
                }
                if name.as_str() == "sorted" {
                    if let TypeId::Array(elem) = &obj_ty {
                        match elem.as_ref() {
                            TypeId::Unknown | TypeId::GenericParam(_) => {}
                            _ => {
                                let elem_name = type_id_to_name(elem);
                                if elem_name == "T" || elem_name == "Unknown" {
                                    // generic Array.sorted<T>() in Core extension
                                } else if !self.type_satisfies_protocol(elem_name.as_str(), "Comparable") {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "Array.sorted() requires element type '{elem_name}' to conform to Comparable"
                                        ),
                                        *span,
                                        "S057",
                                    ));
                                }
                            }
                        }
                    }
                }
                if let Some(surface) = type_collection_surface(&obj_ty) {
                    if is_retired_collection_methodcall(surface, name.as_str()) {
                        return Err(self.error_at(
                            format!(
                                "MethodCall on '{surface}.{name}' is retired; use manifest host_ops or an extension (S025)"
                            ),
                            *span,
                            "S025",
                        ));
                    }
                }
                if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                    if matches!(name.as_str(), "fromArray" | "loadFromArray") {
                        if let Some(lanes) = simd_lanes_from_type_name(type_name.as_str()) {
                            if !syx_config::simd_enabled() {
                                self.errors.push(self.error_at(
                                    "SIMD types require SIMD to be enabled",
                                    *span,
                                    "S022",
                                ));
                            }
                            let element = self
                                .infer_simd_element_from_args(args)
                                .unwrap_or(NumericKind::I32);
                            return Ok(TypeId::Simd { lanes, element });
                        }
                    }
                    if name.as_str() == "storeToArray" {
                        if simd_lanes_from_type_name(type_name.as_str()).is_some() {
                            return Ok(TypeId::Array(Box::new(TypeId::Unknown)));
                        }
                    }
                }
                if let TypeId::Named(type_name) = &obj_ty {
                    if self.class_names.contains(type_name) {
                        let method_name = SmolStr::new(format!("{type_name}_{name}"));
                        self.check_call_argument_labels(&method_name, args, *span);
                    }
                }
                self.check_extension_member_where(&obj_ty, name, *span);
                self.check_known_collection_member(&obj_ty, name.as_str(), *span);
                if let Expr::Ident { name: module, .. } = object.as_ref() {
                    if let Some(exports) = self.import_exports.get(module) {
                        if let Some(ty) = exports.get(name) {
                            return Ok(ty.clone());
                        }
                    }
                }
                // Array.map: result Array<U> from transform closure result type.
                if matches!(lookup_array_method(name.as_str()), Some(MethodId::ArrMap)) {
                    if matches!(obj_ty, TypeId::Array(_)) {
                        let u = match args.first() {
                            // Re-analyze: Closure expr type is its result (annotation / expr body).
                            Some(arg) => self.analyze_expr(&arg.value)?,
                            None => TypeId::Unknown,
                        };
                        return Ok(TypeId::Array(Box::new(u)));
                    }
                }
                // Array.flatMap: unwrap one Array layer from transform result (else map-like).
                if name.as_str() == "flatMap" {
                    if matches!(obj_ty, TypeId::Array(_)) {
                        let r = match args.first() {
                            Some(arg) => self.analyze_expr(&arg.value)?,
                            None => TypeId::Unknown,
                        };
                        return Ok(match r {
                            TypeId::Array(inner) => TypeId::Array(inner),
                            other => TypeId::Array(Box::new(other)),
                        });
                    }
                }
                // Array.reduce: result U from initial (args[0]); facade still Int/Function.
                if matches!(lookup_array_method(name.as_str()), Some(MethodId::ArrReduce)) {
                    if matches!(obj_ty, TypeId::Array(_)) {
                        let u = match args.first() {
                            Some(arg) => self.analyze_expr(&arg.value)?,
                            None => TypeId::Unknown,
                        };
                        return Ok(u);
                    }
                }
                // Array.zip: runtime pairs are Array<[left, right]>; same elem → Array<Array<T>>.
                // Heterogeneous → Array<Array<Object>> (not Unknown; Unknown would assign to Array<T>).
                if name.as_str() == "zip" {
                    if let TypeId::Array(elem) = &obj_ty {
                        let other = match args.first() {
                            Some(arg) => self.analyze_expr(&arg.value)?,
                            None => TypeId::Unknown,
                        };
                        let pair_elem = match other {
                            TypeId::Array(elem2)
                                if is_compatible(elem.as_ref(), elem2.as_ref())
                                    && is_compatible(elem2.as_ref(), elem.as_ref()) =>
                            {
                                TypeId::Array(elem.clone())
                            }
                            _ => TypeId::Array(Box::new(TypeId::Named(SmolStr::new("Object")))),
                        };
                        return Ok(TypeId::Array(Box::new(pair_elem)));
                    }
                }
                Ok(self.infer_member_type(&obj_ty, name))
            }
            _ => {
                self.analyze_expr(callee)?;
                for arg in args {
                    self.analyze_expr(&arg.value)?;
                }
                if let Expr::Member {
                    object,
                    name: member,
                    ..
                } = callee
                {
                    if let Expr::Ident { name: enum_name, .. } = object.as_ref() {
                        if let Some((type_params, where_clause)) =
                            self.generic_enums.get(enum_name).cloned()
                        {
                            let type_args = self.infer_enum_case_type_args(
                                &type_params,
                                enum_name,
                                member,
                                args,
                            );
                            self.validate_generic_instantiation_with(
                                enum_name,
                                &type_params,
                                &where_clause,
                                &type_args,
                                expr_span(callee),
                            );
                            self.record_instantiation(enum_name.clone(), type_args);
                        } else if let Some((type_params, where_clause)) =
                            self.generic_final_classes.get(enum_name).cloned()
                        {
                            let type_args = self.infer_enum_case_type_args(
                                &type_params,
                                enum_name,
                                member,
                                args,
                            );
                            self.validate_generic_instantiation_with(
                                enum_name,
                                &type_params,
                                &where_clause,
                                &type_args,
                                expr_span(callee),
                            );
                            self.record_instantiation(enum_name.clone(), type_args);
                        }
                    }
                }
                Ok(TypeId::Void)
            }
        }
    }

    pub(crate) fn infer_pattern_type(&self, pattern: &Expr) -> SyxResult<TypeId> {
        match pattern {
            Expr::Ident { name, .. } => Ok(self
                .scope
                .lookup(name)
                .or_else(|| self.globals.get(name))
                .map(|s| s.ty.clone())
                .unwrap_or(TypeId::Unknown)),
            Expr::Member { object, name, .. } => {
                if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
                    if let Some(sym) = self.scope.lookup(obj_name).or_else(|| self.globals.get(obj_name)) {
                        if let TypeId::Optional(inner) = &sym.ty {
                            return Ok(inner.as_ref().clone());
                        }
                        if let TypeId::Named(type_name) = &sym.ty {
                            if let Some(exports) = self.globals.get(type_name) {
                                let _ = exports;
                            }
                        }
                    }
                }
                let obj_ty = match object.as_ref() {
                    Expr::Ident { name, .. } => self
                        .scope
                        .lookup(name)
                        .or_else(|| self.globals.get(name))
                        .map(|s| s.ty.clone())
                        .unwrap_or(TypeId::Unknown),
                    _ => TypeId::Unknown,
                };
                if matches!(obj_ty, TypeId::Optional(_)) {
                    Ok(self.infer_member_type(&obj_ty, name))
                } else {
                    Ok(TypeId::Unknown)
                }
            }
            _ => Ok(TypeId::Unknown),
        }
    }

    pub(crate) fn check_assignable(
        &self,
        expected: TypeId,
        found: TypeId,
        name: &str,
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        if let (TypeId::Named(base), TypeId::Named(sub)) = (&expected, &found) {
            if self.is_subclass_of(sub.as_str(), base.as_str()) {
                return Ok(());
            }
        }
        // S1-I8: bare protocol type as annotation (`Array<Shape>`, `Shape`).
        if self.type_satisfies_as_protocol_bound(&expected, &found) {
            return Ok(());
        }
        if let (TypeId::Array(exp_e), TypeId::Array(found_e)) = (&expected, &found) {
            if self.type_satisfies_as_protocol_bound(exp_e, found_e) {
                return Ok(());
            }
        }
        match check_assignable(expected, found, name) {
            Ok(()) => Ok(()),
            Err(err) => Err(self.error_at(err.message, span, err.code)),
        }
    }

    /// `found` may be used where `expected` is a (local) protocol requirement
    /// or `any`/`some` existential of that protocol.
    pub(crate) fn type_satisfies_as_protocol_bound(&self, expected: &TypeId, found: &TypeId) -> bool {
        let proto_name = match expected {
            TypeId::Named(proto) if self.protocol_names.contains(proto) => proto.as_str(),
            TypeId::OpaqueAny(inner) | TypeId::OpaqueSome(inner) => {
                match inner.as_ref() {
                    TypeId::Named(proto) if self.protocol_names.contains(proto) => proto.as_str(),
                    _ => return false,
                }
            }
            _ => return false,
        };
        self.type_id_satisfies_protocol(found, proto_name)
    }

    pub(crate) fn is_subclass_of(&self, sub: &str, base: &str) -> bool {
        if sub == base {
            return true;
        }
        self.class_superclasses
            .get(&SmolStr::new(sub))
            .map(|parent| self.is_subclass_of(parent.as_str(), base))
            .unwrap_or(false)
    }

    pub(crate) fn check_assignable_with_literal(
        &mut self,
        expected: TypeId,
        found: TypeId,
        name: &str,
        init: Option<&Expr>,
        decl_span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        let diag_span = init.map(expr_span).unwrap_or(decl_span);
        if matches!(expected, TypeId::OpaqueAny(_) | TypeId::OpaqueSome(_)) {
            if self.check_opaque_protocol_assignable(&expected, &found, name, diag_span) {
                return Ok(());
            }
        }
        if let Some(Expr::Literal { value, .. }) = init {
            self.check_literal_in_range(value, &expected, diag_span);
            if matches!(expected, TypeId::Named(ref n) if n.as_str() == "Character") {
                if let Literal::String(s) = value {
                    if s.chars().count() != 1 {
                        self.errors.push(self.error_at(
                            format!(
                                "Type mismatch for '{name}': expected Character, found multi-scalar String literal"
                            ),
                            diag_span,
                            "S004",
                        ));
                        return Ok(());
                    }
                    return Ok(());
                }
            }
            // Integer suffix under a float annotation is not contextual (`1i16` → Float).
            // Suffixed integers also must not use assign-site narrowing (`8i16` → Int8 → S004).
            if let (TypeId::Numeric(exp), Literal::Integer { suffix: Some(suf), .. }) =
                (&expected, value)
            {
                if let Some(found_kind) = from_suffix(suf) {
                    if is_float(*exp) && is_integer(found_kind) {
                        self.errors.push(self.error_at(
                            format!(
                                "Type mismatch for '{name}': expected {}, found {}",
                                type_id_to_name(&expected),
                                type_id_to_name(&TypeId::numeric(found_kind))
                            ),
                            diag_span,
                            "S004",
                        ));
                        return Ok(());
                    }
                    if is_integer(*exp)
                        && is_integer(found_kind)
                        && !is_implicit_convertible(found_kind, *exp)
                    {
                        self.errors.push(self.error_at(
                            format!(
                                "Type mismatch for '{name}': expected {}, found {}",
                                type_id_to_name(&expected),
                                type_id_to_name(&TypeId::numeric(found_kind))
                            ),
                            diag_span,
                            "S004",
                        ));
                        return Ok(());
                    }
                }
            }
        }
        if matches!(expected, TypeId::Set(_)) {
            if let TypeId::Array(elem) = &found {
                let TypeId::Set(set_elem) = &expected else {
                    return self.check_assignable(expected, found, name, diag_span);
                };
                if !self.is_compatible(set_elem, elem)
                    && !matches!(elem.as_ref(), TypeId::Unknown)
                {
                    return self.check_assignable(expected, found, name, diag_span);
                }
                self.validate_set_elem_hashable(set_elem, diag_span);
                return Ok(());
            }
            if matches!(&found, TypeId::Named(n) if n.as_str() == "Set") {
                let TypeId::Set(set_elem) = &expected else {
                    return self.check_assignable(expected, found, name, diag_span);
                };
                self.validate_set_elem_hashable(set_elem, diag_span);
                return Ok(());
            }
        }
        // Explicit `: Object` + dict literal (possibly heterogeneous).
        if matches!(&expected, TypeId::Named(n) if n.as_str() == "Object") {
            if let Some(Expr::Literal {
                value: Literal::Dict(_),
                ..
            }) = init
            {
                return Ok(());
            }
        }
        // Annotated `Dictionary`/`[K:V]` ← dict literal: check key/value vs declared.
        if let TypeId::Dictionary {
            key: exp_key,
            value: exp_val,
        } = &expected
        {
            if let Some(Expr::Literal {
                value: Literal::Dict(entries),
                ..
            }) = init
            {
                if matches!(&found, TypeId::Dictionary { .. }) {
                    self.validate_dict_key_hashable(exp_key, diag_span);
                    for (k, v) in entries {
                        let kt = self.type_of_expr(k);
                        let vt = self.type_of_expr(v);
                        if !matches!(kt, TypeId::Unknown)
                            && kt != **exp_key
                            && !self.is_compatible(exp_key, &kt)
                            && !self.is_compatible(&kt, exp_key)
                        {
                            self.errors.push(self.error_at(
                                format!(
                                    "Collection elements have inconsistent types: expected {}, found {}",
                                    type_id_to_name(exp_key),
                                    type_id_to_name(&kt)
                                ),
                                expr_span(k),
                                "S014",
                            ));
                        }
                        if !matches!(vt, TypeId::Unknown)
                            && vt != **exp_val
                            && !self.is_compatible(exp_val, &vt)
                            && !self.is_compatible(&vt, exp_val)
                        {
                            self.errors.push(self.error_at(
                                format!(
                                    "Collection elements have inconsistent types: expected {}, found {}",
                                    type_id_to_name(exp_val),
                                    type_id_to_name(&vt)
                                ),
                                expr_span(v),
                                "S014",
                            ));
                        }
                    }
                    return Ok(());
                }
            }
        }
        self.check_assignable(expected, found, name, diag_span)
    }

    /// Analyze `expr`, applying contextual typing when `expected` is known.
    /// Dict literals become `Object` only under an explicit `Object` expectation.
    pub(crate) fn analyze_expr_with_expected(
        &mut self,
        expr: &Expr,
        expected: Option<&TypeId>,
    ) -> SyxResult<TypeId> {
        if let Some(TypeId::Named(n)) = expected {
            if n.as_str() == "Object" {
                if let Expr::Literal {
                    value: Literal::Dict(entries),
                    ..
                } = expr
                {
                    for (k, v) in entries {
                        self.analyze_expr(k)?;
                        self.analyze_expr(v)?;
                    }
                    let ty = TypeId::Named(SmolStr::new("Object"));
                    self.record_expr_type(expr, ty.clone());
                    return Ok(ty);
                }
            }
        }
        // `Set()` under `: Set<T>` → `Set<T>` (bare ctor is `Named("Set")`).
        if let Some(TypeId::Set(elem)) = expected {
            if let Expr::Call { callee, args, .. } = expr {
                if args.is_empty() {
                    if let Expr::Ident { name, .. } = callee.as_ref() {
                        if name.as_str() == "Set" {
                            let _ = self.analyze_expr(expr)?;
                            let ty = TypeId::Set(elem.clone());
                            self.record_expr_type(expr, ty.clone());
                            return Ok(ty);
                        }
                    }
                }
            }
        }
        // Empty `{}` / `[:]` under `: Dictionary<K, V>` → Dictionary<K, V>
        // (same idea as empty `[]` under Array/Set annotations).
        if let Some(TypeId::Dictionary { key, value }) = expected {
            if let Expr::Literal {
                value: Literal::Dict(entries),
                ..
            } = expr
            {
                if entries.is_empty() {
                    self.validate_dict_key_hashable(key, expr_span(expr));
                    let ty = TypeId::Dictionary {
                        key: key.clone(),
                        value: value.clone(),
                    };
                    self.record_expr_type(expr, ty.clone());
                    return Ok(ty);
                }
            }
        }
        // S1-I8/I10: `[Circle(), Square()]` under `: Array<Shape>` (protocol or class).
        if let Some(TypeId::Array(elem_expected)) = expected {
            if let Expr::Array { elements, span, .. } = expr {
                let is_proto_elem = match elem_expected.as_ref() {
                    TypeId::Named(n) if self.protocol_names.contains(n) => true,
                    TypeId::OpaqueAny(inner) | TypeId::OpaqueSome(inner) => {
                        matches!(inner.as_ref(), TypeId::Named(n) if self.protocol_names.contains(n))
                    }
                    _ => false,
                };
                let is_class_elem = matches!(
                    elem_expected.as_ref(),
                    TypeId::Named(n) if self.class_names.contains(n)
                );
                // S1-I12: `Array<Any>` / `Array<Object>` mixed literals (book Ch05).
                let is_erasure_elem = matches!(
                    elem_expected.as_ref(),
                    TypeId::Named(n) if matches!(n.as_str(), "Any" | "Object")
                );
                if is_proto_elem || is_class_elem || is_erasure_elem || elements.is_empty() {
                    for el in elements {
                        let found = self.analyze_expr(el)?;
                        if is_erasure_elem || matches!(found, TypeId::Unknown) {
                            continue;
                        }
                        let ok = found == **elem_expected
                            || is_compatible(elem_expected, &found)
                            || self.type_satisfies_as_protocol_bound(elem_expected, &found)
                            || matches!(
                                (elem_expected.as_ref(), &found),
                                (TypeId::Named(base), TypeId::Named(sub))
                                    if self.is_subclass_of(sub.as_str(), base.as_str())
                            );
                        if !ok {
                            self.errors.push(self.error_at(
                                format!(
                                    "Collection elements have inconsistent types: expected {}, found {}",
                                    type_id_to_name(elem_expected),
                                    type_id_to_name(&found)
                                ),
                                *span,
                                "S014",
                            ));
                        }
                    }
                    let ty = TypeId::Array(elem_expected.clone());
                    self.record_expr_type(expr, ty.clone());
                    return Ok(ty);
                }
            }
        }
        // Unsuffixed integer literal under an integer or float annotation → contextual type
        // (e.g. `let x: Int8 = 8`, `let y: Double = 1`). Suffixed literals keep literal_numeric_type.
        if let Some(TypeId::Numeric(kind)) = expected {
            if is_integer(*kind) || is_float(*kind) {
                if let Expr::Literal {
                    value: Literal::Integer { suffix: None, .. },
                    ..
                } = expr
                {
                    let ty = expected.cloned().unwrap();
                    self.record_expr_type(expr, ty.clone());
                    return Ok(ty);
                }
            }
        }
        self.analyze_expr(expr)
    }

    pub(crate) fn is_compatible(&self, expected: &TypeId, found: &TypeId) -> bool {
        if is_compatible(expected, found) {
            return true;
        }
        if self.type_satisfies_as_protocol_bound(expected, found) {
            return true;
        }
        if let (TypeId::Named(base), TypeId::Named(sub)) = (expected, found) {
            if self.is_subclass_of(sub.as_str(), base.as_str()) {
                return true;
            }
        }
        if let (TypeId::Array(a), TypeId::Array(b)) = (expected, found) {
            return self.is_compatible(a, b)
                || self.type_satisfies_as_protocol_bound(a, b);
        }
        false
    }
}
