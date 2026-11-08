//! SYX semantic SemanticAnalyzer hub (struct + entry/builtins).
//! Faces peeled through S2-I258: + concurrency/IDE/infer-host.

use crate::{
    attached_conformance_protocols, default_assoc_type_for,
    global_actor_v4_cross_module_enabled, has_attr, normalize_type_arg_name, type_expr_uses_param, type_id_to_name, type_name_to_id, Scope,
    Symbol, TypeId, MAX_COLLECTED_DIAGNOSTICS,
};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::collections::HashSet;
use syx_ast::{
    AccessLevel, Decl, Module, ParamLabel, TypeExpr,
};
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_stdlib::{
    host_fn_return_type,
    lookup_array_method, lookup_dict_method, lookup_set_method, lookup_string_method,
    lookup_type_method, method_return_type, register_builtins as stdlib_register_builtins,
    HostFnId, MethodId,
};

#[derive(Debug, Default)]
pub struct SemanticAnalyzer {
    pub globals: IndexMap<SmolStr, Symbol>,
    pub scope: Scope,
    pub exports: IndexMap<SmolStr, TypeId>,
    pub imported_modules: Vec<SmolStr>,
    pub import_exports: IndexMap<SmolStr, IndexMap<SmolStr, TypeId>>,
    pub errors: Vec<SyxDiagnostic>,
    pub instantiations: HashSet<(SmolStr, Vec<SmolStr>)>,
    /// Expression span-start → inferred type (for codegen/MIR).
    pub expr_types: IndexMap<u32, TypeId>,
    /// All bindings seen during analysis (for IDE; last shadow wins).
    pub all_bindings: IndexMap<SmolStr, TypeId>,
    /// Each binding with its source offset, so completion can pick the declaration
    /// in scope at the cursor instead of the last same-named local in the file.
    pub binding_sites: Vec<(SmolStr, u32, TypeId)>,
    /// Top-level function names already collected (duplicate → S001).
    pub(crate) defined_functions: HashSet<SmolStr>,
    pub(crate) generic_functions: IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    pub(crate) generic_enums: IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    pub(crate) generic_final_classes: IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    pub(crate) enum_cases: IndexMap<SmolStr, Vec<SmolStr>>,
    pub(crate) enum_case_field_types: IndexMap<(SmolStr, SmolStr), Vec<(SmolStr, TypeExpr)>>,
    pub(crate) type_conformances: IndexMap<SmolStr, Vec<SmolStr>>,
    /// ADR-324: `extension Base: Proto where …` gated conformances (not unconditional).
    pub(crate) conditional_conformances: Vec<(SmolStr, SmolStr, Vec<syx_ast::WherePred>)>,
    /// User/local protocol names (for `class T: P` parsed as superclass → conformance).
    pub(crate) protocol_names: HashSet<SmolStr>,
    pub(crate) dataclass_names: HashSet<SmolStr>,
    pub(crate) dataclass_fields: IndexMap<SmolStr, Vec<(SmolStr, TypeId)>>,
    /// S1-I2: `(type, method)` marked `mutating` / known mutating collection ops.
    pub(crate) mutating_methods: HashSet<(SmolStr, SmolStr)>,
    pub(crate) class_names: HashSet<SmolStr>,
    /// S1-I10: classes whose designated `init?` / `init!` makes `Type(...)` return Optional.
    pub(crate) failable_inits: HashSet<SmolStr>,
    /// Computed properties with a getter but no setter (`let name { get { … } }`).
    pub(crate) getter_only_properties: HashSet<(SmolStr, SmolStr)>,
    /// Phase 121: `@Service` / `@Component` types for typed `__syx_service_resolve`.
    pub(crate) service_class_names: HashSet<SmolStr>,
    pub(crate) class_methods: IndexMap<SmolStr, Vec<(SmolStr, TypeId)>>,
    /// Phase 65: child class -> direct superclass.
    pub(crate) class_superclasses: IndexMap<SmolStr, SmolStr>,
    /// Phase 65: (class, method) marked `final`.
    pub(crate) final_methods: HashSet<(SmolStr, SmolStr)>,
    /// Phase 65: analyzing inside class/actor member.
    pub(crate) current_class_context: Option<SmolStr>,
    pub(crate) type_extension_methods: IndexMap<SmolStr, Vec<(SmolStr, TypeId)>>,
    /// Phase 41: extension method where-clauses keyed by (extended type, method name).
    pub(crate) extension_method_where: IndexMap<(SmolStr, SmolStr), Vec<syx_ast::WherePred>>,
    pub(crate) actor_types: HashSet<SmolStr>,
    pub(crate) actor_fields: IndexMap<SmolStr, HashSet<SmolStr>>,
    /// When analyzing inside an actor method/init/deinit, the enclosing actor type name.
    pub(crate) current_actor_context: Option<SmolStr>,
    /// Functions declared with `throws` (value = optional typed error).
    pub(crate) throwing_functions: IndexMap<SmolStr, Option<TypeId>>,
    pub(crate) try_depth: u32,
    pub strict_concurrency: bool,
    /// Phase 11 AV3: module default isolation from `syx.toml`.
    pub module_default_isolation: Option<SmolStr>,
    pub(crate) sendable_types: HashSet<SmolStr>,
    /// Phase 23: types marked `@unchecked` Sendable (v5 boundary).
    pub(crate) unchecked_sendable_types: HashSet<SmolStr>,
    pub(crate) main_actor_functions: HashSet<SmolStr>,
    pub(crate) global_actor_names: HashSet<SmolStr>,
    pub(crate) function_global_actor: IndexMap<SmolStr, SmolStr>,
    pub(crate) nonisolated_functions: HashSet<SmolStr>,
    /// Phase 12: `@nonisolated(nonsending)` functions.
    pub(crate) nonsending_functions: HashSet<SmolStr>,
    pub(crate) concurrent_functions: HashSet<SmolStr>,
    pub(crate) function_params: IndexMap<SmolStr, Vec<TypeId>>,
    /// External label strategy + internal name for call-site label checking.
    pub(crate) function_param_labels: IndexMap<SmolStr, Vec<(ParamLabel, SmolStr)>>,
    pub(crate) function_return_types: IndexMap<SmolStr, TypeId>,
    /// Per-function `sending` param flags (top-level functions).
    pub(crate) function_sending_params: IndexMap<SmolStr, Vec<bool>>,
    /// Actor method `sending` param flags keyed by (actor, method).
    pub(crate) actor_method_sending: IndexMap<(SmolStr, SmolStr), Vec<bool>>,
    pub(crate) mut_borrows: HashSet<SmolStr>,
    pub(crate) moved_bindings: HashSet<SmolStr>,
    /// Phase 13: `@MainActor` isolated types.
    pub(crate) main_actor_types: HashSet<SmolStr>,
    /// Phase 13: function being analyzed (for isolation checks).
    pub(crate) current_function: Option<SmolStr>,
    /// Phase 16: lifetime param on reference bindings in the current function.
    pub(crate) binding_lifetimes: IndexMap<SmolStr, SmolStr>,
    /// Phase 16: expected lifetime on function return reference type.
    pub(crate) current_return_lifetime: Option<SmolStr>,
    /// Phase 17: active local reference bindings that cannot cross `await`.
    pub(crate) local_ref_bindings: HashSet<SmolStr>,
    /// Phase 17: analyzing an `async` function body.
    pub(crate) current_function_is_async: bool,
    /// S1-I4: generic params constrained `: Equatable` / `where T: Equatable` in current fn.
    pub(crate) current_equatable_params: HashSet<SmolStr>,
    /// S1-I6: open type params of the function currently under analysis (`T`, `U`, …).
    pub(crate) current_type_params: HashSet<SmolStr>,
    /// Phase 20: `@MainActor` async mutable locals in scope.
    pub(crate) main_actor_mutable_locals: HashSet<SmolStr>,
    /// Phase 20: snapshot of mutable locals at the last `await` in `@MainActor` async fn.
    pub(crate) main_actor_frozen_at_await: HashSet<SmolStr>,
    /// Phase 20: crossed an `await` in the current `@MainActor` async function.
    pub(crate) main_actor_await_crossed: bool,
    /// Phase 20: protocol associated type requirements keyed by protocol name.
    pub(crate) protocol_assoc_types: IndexMap<SmolStr, IndexMap<SmolStr, TypeExpr>>,
    /// Phase 20: associated type bindings for conforming types.
    pub(crate) type_assoc_bindings: IndexMap<SmolStr, IndexMap<SmolStr, SmolStr>>,
    /// Phase 20: nested borrow region depth for multi-region tracking.
    pub(crate) borrow_region_depth: u32,
    /// Phase 40: top-level type aliases.
    pub(crate) type_aliases: IndexMap<SmolStr, TypeExpr>,
    /// Phase 139: dotted nested path → mangled type (`Outer.Inner` → `Outer_Inner`).
    pub(crate) nested_type_dotted: IndexMap<SmolStr, SmolStr>,
    /// Phase 40: types with user subscript getters.
    pub(crate) subscript_types: HashSet<SmolStr>,
    /// Phase 41: types with user subscript setters.
    pub(crate) subscript_set_types: HashSet<SmolStr>,
    /// Phase 40: subscript return types.
    pub(crate) subscript_return_types: IndexMap<SmolStr, TypeId>,
    pub(crate) immutable_ref_bindings: HashSet<SmolStr>,
    /// Phase 23 DE2: active lifetime labels (`'a`, `'b`) in nested borrow regions.
    pub(crate) active_lifetime_labels: HashSet<SmolStr>,
    /// Phase 20: skip S020 when analyzing async-lowering output.
    pub(crate) skip_main_actor_await_check: bool,
    /// Phase 42: skip S023 when analyzing embedded stdlib prelude modules (`class Math`, …).
    pub(crate) skip_prelude_name_shadow: bool,
    /// Phase 22: imported `(module, function)` → required global actor.
    pub(crate) import_member_actors: IndexMap<(SmolStr, SmolStr), SmolStr>,
    /// Source file for attaching diagnostic spans (line/column/filename).
    pub(crate) source: Option<SourceFile>,
    /// Decl spans for type names (class / dataclass / protocol) used by S012 etc.
    pub(crate) type_decl_spans: IndexMap<SmolStr, syx_diagnostics::Span>,
}

impl SemanticAnalyzer {
    pub fn analyze(module: &Module) -> SyxResult<Self> {
        Self::analyze_with_imports(module, &[])
    }

    /// Phase 87: register analyzed dataclass field metadata for reflect/codable L4.
    /// ADR-231: also register field type names (Optional as `T?`) for recursive JSONEncoder.
    pub fn register_reflect_types(&self) {
        for (name, fields) in &self.dataclass_fields {
            syx_meta::register_type_with_field_types(
                name.as_str(),
                fields
                    .iter()
                    .map(|(f, ty)| (f.clone(), type_id_to_name(ty)))
                    .collect(),
            );
        }
    }

    pub fn analyze_with_imports(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
    ) -> SyxResult<Self> {
        Self::analyze_with_imports_and_generics(module, imports, &IndexMap::new())
    }

    pub fn analyze_with_imports_strict(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
        strict_concurrency: bool,
    ) -> SyxResult<Self> {
        let analyzer =
            Self::analyze_collecting_with_imports_strict(module, imports, strict_concurrency);
        if let Some(err) = analyzer.errors.first() {
            return Err(err.clone());
        }
        Ok(analyzer)
    }

    pub fn analyze_with_imports_strict_skip_main_actor(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
        strict_concurrency: bool,
        module_default_isolation: Option<SmolStr>,
        import_isolation: &[(SmolStr, IndexMap<SmolStr, SmolStr>)],
        source: Option<&SourceFile>,
        import_generic_methods: &IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    ) -> SyxResult<Self> {
        let mut analyzer = Self::analyze_collecting_with_options(
            module,
            imports,
            strict_concurrency,
            module_default_isolation,
            import_isolation,
            source,
            import_generic_methods,
            false,
        );
        analyzer.errors.retain(|e| e.code.as_deref() != Some("S020"));
        if let Some(err) = analyzer.errors.first() {
            return Err(err.clone());
        }
        Ok(analyzer)
    }

    /// Run semantic analysis and retain all diagnostics in `errors` (no early return).
    pub fn analyze_collecting(module: &Module) -> Self {
        Self::analyze_collecting_with_imports(module, &[])
    }

    pub fn analyze_collecting_with_imports(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
    ) -> Self {
        Self::analyze_collecting_with_imports_strict(module, imports, false)
    }

    pub fn analyze_collecting_with_imports_strict(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
        strict_concurrency: bool,
    ) -> Self {
        Self::analyze_collecting_with_options(
            module,
            imports,
            strict_concurrency,
            None,
            &[],
            None,
            &IndexMap::new(),
            false,
        )
    }

    pub fn analyze_with_imports_and_generics(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
        import_generic_methods: &IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    ) -> SyxResult<Self> {
        let analyzer = Self::analyze_collecting_with_imports_and_generics(
            module,
            imports,
            import_generic_methods,
        );
        if let Some(err) = analyzer.errors.first() {
            return Err(err.clone());
        }
        Ok(analyzer)
    }

    /// Analyze an embedded stdlib module (prelude shadow S023 disabled).
    pub fn analyze_embedded_stdlib(module: &Module) -> SyxResult<Self> {
        Self::analyze_embedded_stdlib_with_imports(module, &[])
    }

    /// Analyze an embedded stdlib module with prior prelude symbols in scope.
    pub fn analyze_embedded_stdlib_with_imports(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
    ) -> SyxResult<Self> {
        let analyzer = Self::analyze_collecting_with_options(
            module,
            imports,
            false,
            None,
            &[],
            None,
            &IndexMap::new(),
            true,
        );
        if let Some(err) = analyzer.errors.first() {
            return Err(err.clone());
        }
        Ok(analyzer)
    }

    pub fn analyze_collecting_with_imports_and_generics(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
        import_generic_methods: &IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    ) -> Self {
        Self::analyze_collecting_with_options(
            module,
            imports,
            false,
            None,
            &[],
            None,
            import_generic_methods,
            false,
        )
    }

    pub fn register_import_generic_class_methods(
        &mut self,
        methods: &IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    ) {
        for (name, (type_params, where_clause)) in methods {
            self.generic_functions
                .insert(name.clone(), (type_params.clone(), where_clause.clone()));
        }
    }

    pub fn analyze_collecting_with_options(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
        strict_concurrency: bool,
        module_default_isolation: Option<SmolStr>,
        import_isolation: &[(SmolStr, IndexMap<SmolStr, SmolStr>)],
        source: Option<&SourceFile>,
        import_generic_methods: &IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
        skip_prelude_name_shadow: bool,
    ) -> Self {
        Self::analyze_collecting_with_value_builtins(
            module,
            imports,
            strict_concurrency,
            module_default_isolation,
            import_isolation,
            source,
            import_generic_methods,
            skip_prelude_name_shadow,
            &[],
        )
    }

    /// Like [`analyze_collecting_with_options`], but merges Core value-builtin
    /// Class/Extension **signatures** (e.g. `Int.min`) after `collect_decls` and
    /// before body analysis — no foreign method bodies, so IDE `binding_sites`
    /// stay scoped to the user buffer.
    pub fn analyze_collecting_with_value_builtins(
        module: &Module,
        imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
        strict_concurrency: bool,
        module_default_isolation: Option<SmolStr>,
        import_isolation: &[(SmolStr, IndexMap<SmolStr, SmolStr>)],
        source: Option<&SourceFile>,
        import_generic_methods: &IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
        skip_prelude_name_shadow: bool,
        value_builtin_modules: &[Module],
    ) -> Self {
        syx_meta::reset_semantic_instantiations();
        let mut analyzer = Self::default();
        analyzer.skip_prelude_name_shadow = skip_prelude_name_shadow;
        analyzer.source = source.cloned();
        analyzer.strict_concurrency = strict_concurrency;
        analyzer.module_default_isolation = module_default_isolation.clone();
        analyzer.register_builtins();
        analyzer.register_import_generic_class_methods(import_generic_methods);
        for (module_name, symbols) in imports {
            analyzer.import_exports.insert(module_name.clone(), symbols.clone());
            analyzer.globals.insert(
                module_name.clone(),
                Symbol {
                    name: module_name.clone(),
                    ty: TypeId::Named(module_name.clone()),
                    mutable: false,
                    access: AccessLevel::Public,
                },
            );
            for (name, ty) in symbols {
                analyzer.globals.insert(
                    name.clone(),
                    Symbol {
                        name: name.clone(),
                        ty: ty.clone(),
                        mutable: false,
                        access: AccessLevel::Public,
                    },
                );
                if matches!(ty, TypeId::Named(n) if n.as_str() == name.as_str()) {
                    analyzer.class_names.insert(name.clone());
                }
                if syx_meta::reflect_type(name.as_str()).is_some() {
                    analyzer.dataclass_names.insert(name.clone());
                }
            }
        }
        // Phase 122: multi-package workspace scan + queued services for typed resolve (no silent).
        for name in syx_meta::service_di_known_type_names() {
            analyzer.service_class_names.insert(SmolStr::new(name));
        }
        let _ = analyzer.collect_decls(&module.decls);
        for builtin in value_builtin_modules {
            analyzer.merge_value_builtin_surface(builtin);
        }
        // ADR-322: `class C: UserProtocol` is parsed as superclass; promote to conformance.
        analyzer.promote_protocol_superclass_conformances();
        if global_actor_v4_cross_module_enabled() {
            analyzer.apply_module_default_isolation(module);
            for (module_name, fn_actors) in import_isolation {
                for (fn_name, actor) in fn_actors {
                    analyzer.import_member_actors.insert(
                        (module_name.clone(), fn_name.clone()),
                        actor.clone(),
                    );
                }
            }
        }
        if let Err(e) = analyzer.analyze_decls(&module.decls) {
            analyzer.push_diagnostic(e);
        }
        analyzer.check_concurrency_rules(module);
        analyzer.populate_default_assoc_bindings();
        analyzer.check_assoc_type_conformances();
        analyzer.errors.truncate(MAX_COLLECTED_DIAGNOSTICS);
        syx_meta::set_semantic_instantiations(&analyzer.instantiations);
        analyzer
    }

    /// Push a diagnostic, respecting [`MAX_COLLECTED_DIAGNOSTICS`].
    pub fn push_diagnostic(&mut self, diag: SyxDiagnostic) {
        if self.errors.len() < MAX_COLLECTED_DIAGNOSTICS {
            self.errors.push(diag);
        }
    }

    pub(crate) fn record_instantiation(&mut self, fn_name: SmolStr, type_args: Vec<SmolStr>) {
        // ADR-324: never record bare identifier names as type args (mangling hazard).
        // Normalize `Int`→`Int64` so MIR Call mangling matches mono instances.
        let type_args: Vec<SmolStr> = type_args
            .into_iter()
            .map(|t| {
                if t.as_str() == "Unknown"
                    || t.chars().next().is_some_and(|c| c.is_uppercase())
                    || t.contains('<')
                    || t.contains('[')
                {
                    normalize_type_arg_name(t)
                } else {
                    // Lowercase bare names are values, not types — treat as Unknown.
                    SmolStr::new("Unknown")
                }
            })
            .collect();
        self.instantiations
            .insert((fn_name.clone(), type_args.clone()));
        syx_meta::note_semantic_instantiation(fn_name.as_str(), &type_args);
    }

    pub(crate) fn check_main_actor_after_await(
        &mut self,
        name: &SmolStr,
        span: syx_diagnostics::Span,
    ) {
        if self.skip_main_actor_await_check {
            return;
        }
        if name.starts_with("__") {
            return;
        }
        if self.main_actor_await_crossed && self.main_actor_frozen_at_await.contains(name) {
            self.errors.push(self.error_at(
                format!(
                    "MainActor-isolated variable '{name}' cannot be used after crossing an await boundary"
                ),
                span,
                "S020",
            ));
        }
    }

    /// ADR-322: promote `class T: P` (parsed as superclass when P is a local protocol) → conformances.
    pub(crate) fn promote_protocol_superclass_conformances(&mut self) {
        let promote: Vec<(SmolStr, SmolStr)> = self
            .class_superclasses
            .iter()
            .filter(|(_, super_name)| self.protocol_names.contains(*super_name))
            .map(|(type_name, super_name)| (type_name.clone(), super_name.clone()))
            .collect();
        for (type_name, protocol) in promote {
            self.class_superclasses.shift_remove(&type_name);
            self.type_conformances
                .entry(type_name)
                .or_default()
                .push(protocol);
        }
    }

    /// ADR-322 / M4-L02: conforming-type member `typealias Assoc = Concrete` → bindings.
    pub(crate) fn record_member_typealias_bindings(&mut self, type_name: &SmolStr, members: &[Decl]) {
        for member in members {
            if let Decl::TypeAlias { name, ty, .. } = member {
                let TypeExpr::Named {
                    name: concrete, ..
                } = ty
                else {
                    continue;
                };
                self.type_assoc_bindings
                    .entry(type_name.clone())
                    .or_default()
                    .insert(name.clone(), concrete.clone());
            }
        }
    }

    pub(crate) fn populate_default_assoc_bindings(&mut self) {
        let conformances: Vec<(SmolStr, Vec<SmolStr>)> = self
            .type_conformances
            .iter()
            .map(|(t, c)| (t.clone(), c.clone()))
            .collect();
        for (type_name, protocols) in conformances {
            for protocol in protocols {
                let Some(requirements) = self.protocol_assoc_types.get(&protocol) else {
                    continue;
                };
                for (assoc_name, _) in requirements {
                    if self
                        .type_assoc_bindings
                        .get(&type_name)
                        .and_then(|m| m.get(assoc_name))
                        .is_some()
                    {
                        continue;
                    }
                    if let Some(default) = default_assoc_type_for(protocol.as_str(), assoc_name.as_str())
                    {
                        self.type_assoc_bindings
                            .entry(type_name.clone())
                            .or_default()
                            .insert(assoc_name.clone(), SmolStr::new(default));
                    }
                }
            }
        }
    }

    pub(crate) fn check_assoc_type_conformances(&mut self) {
        let conformances: Vec<(SmolStr, Vec<SmolStr>)> = self
            .type_conformances
            .iter()
            .map(|(t, c)| (t.clone(), c.clone()))
            .collect();
        for (type_name, protocols) in conformances {
            for protocol in protocols {
                let Some(requirements) = self.protocol_assoc_types.get(&protocol) else {
                    continue;
                };
                for (assoc_name, constraint) in requirements {
                    let resolved = self
                        .type_assoc_bindings
                        .get(&type_name)
                        .and_then(|m| m.get(assoc_name))
                        .cloned()
                        .unwrap_or_else(|| SmolStr::new("Int"));
                    if let TypeExpr::Named { name: constraint_name, .. } = constraint {
                        if !self.type_satisfies_protocol(resolved.as_str(), constraint_name.as_str())
                        {
                            let span = self
                                .type_decl_spans
                                .get(&type_name)
                                .copied()
                                .unwrap_or(syx_diagnostics::Span::DUMMY);
                            self.errors.push(self.error_at(
                                format!(
                                    "Associated type '{assoc_name}' = '{resolved}' on '{type_name}' does not satisfy '{constraint_name}' required by protocol '{protocol}'"
                                ),
                                span,
                                "S012",
                            ));
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn check_dataclass_codable_witness(
        &mut self,
        name: &SmolStr,
        span: syx_diagnostics::Span,
        attrs: &[syx_ast::Attribute],
        type_params: &[SmolStr],
        where_clause: &[syx_ast::WherePred],
        fields: &[(SmolStr, syx_ast::TypeExpr)],
    ) {
        if type_params.is_empty() {
            return;
        }
        let wants_codable = has_attr(attrs, "Codable")
            || attached_conformance_protocols(attrs)
                .iter()
                .any(|p| p.as_str() == "Codable");
        if !wants_codable {
            return;
        }
        for (field_name, field_ty) in fields {
            let Some(param) = type_expr_uses_param(field_ty, type_params) else {
                continue;
            };
            let satisfied = where_clause.iter().any(|pred| {
                pred.type_name == param && pred.constraint.as_str() == "Codable"
            });
            if !satisfied {
                self.errors.push(self.error_at(
                    format!(
                        "Data class '{name}' field '{field_name}' uses type parameter '{param}' which must conform to Codable (add `where {param}: Codable`)"
                    ),
                    span,
                    "E005",
                ));
            }
        }
    }

    pub(crate) fn track_binding(&mut self, name: SmolStr, ty: TypeId, at: u32) {
        self.all_bindings.insert(name.clone(), ty.clone());
        self.binding_sites.push((name, at, ty));
    }

    /// Type of `name` from the latest binding whose span starts at or before `at`.
    pub fn binding_type_before(&self, name: &str, at: u32) -> Option<TypeId> {
        self.binding_sites
            .iter()
            .rev()
            .find(|(n, start, _)| n.as_str() == name && *start <= at)
            .map(|(_, _, ty)| ty.clone())
    }

    pub(crate) fn deref_type(&self, ty: &TypeId) -> TypeId {
        match ty {
            TypeId::MutReference(inner) | TypeId::Reference(inner) => inner.as_ref().clone(),
            other => other.clone(),
        }
    }

    pub(crate) fn assign_expected_type(&self, ty: &TypeId) -> TypeId {
        match ty {
            TypeId::MutReference(inner) => inner.as_ref().clone(),
            other => other.clone(),
        }
    }

    pub(crate) fn register_builtins(&mut self) {
        for entry in stdlib_register_builtins() {
            self.globals.insert(
                SmolStr::new(entry.name),
                Symbol {
                    name: SmolStr::new(entry.name),
                    ty: type_name_to_id(entry.type_name),
                    mutable: false,
                    access: AccessLevel::Public,
                },
            );
        }
        for id in HostFnId::all() {
            let name = id.name();
            self.globals.insert(
                SmolStr::new(name),
                Symbol {
                    name: SmolStr::new(name),
                    ty: type_name_to_id(host_fn_return_type(*id)),
                    mutable: false,
                    access: AccessLevel::Public,
                },
            );
        }
        self.enum_cases.insert(
            SmolStr::new("Result"),
            vec![SmolStr::new("Ok"), SmolStr::new("Err")],
        );
        for cn in ["Result", "BigInt"] {
            self.class_names.insert(SmolStr::new(cn));
        }
        for case in ["Ok", "Err"] {
            self.globals.insert(
                SmolStr::new(case),
                Symbol {
                    name: SmolStr::new(case),
                    ty: TypeId::Named(SmolStr::new("Result")),
                    mutable: false,
                    access: AccessLevel::Public,
                },
            );
        }
    }

    /// Builtin collections/String: unknown members must fail in semantic (LSP),
    /// not wait for codegen C020 (`Dictionary_append`, etc.).
    pub(crate) fn check_known_collection_member(
        &mut self,
        object_ty: &TypeId,
        method: &str,
        span: syx_diagnostics::Span,
    ) {
        let check = matches!(
            object_ty,
            TypeId::Array(_) | TypeId::Set(_) | TypeId::Dictionary { .. } | TypeId::String
        );
        if !check {
            return;
        }
        if self
            .members_for_type(object_ty)
            .iter()
            .any(|(m, _)| m.as_str() == method)
        {
            return;
        }
        // Embed / pipeline paths may analyze before Core `array.sy` extensions are
        // injected into `type_extension_methods`; still accept MethodId + known
        // extension-only Array APIs (append, sortBy, …).
        let known = match object_ty {
            TypeId::Array(_) => {
                lookup_array_method(method).is_some()
                    || matches!(
                        method,
                        "append"
                            | "sortBy"
                            | "binarySearch"
                            | "chunked"
                            | "zip"
                            | "flatMap"
                    )
            }
            TypeId::Set(_) => lookup_set_method(method).is_some(),
            TypeId::Dictionary { .. } => lookup_dict_method(method).is_some(),
            TypeId::String => {
                lookup_string_method(method).is_some()
                    // S1-I11: book `source.isEmpty` (extension property; not in STRING_METHODS).
                    || method == "isEmpty"
                    // S1-I14: Core extensions (no HostFn+1): hasSuffix / replacingOccurrences.
                    || method == "hasSuffix"
                    || method == "replacingOccurrences"
            }
            _ => false,
        };
        if known {
            return;
        }
        let ty_name = type_id_to_name(object_ty);
        self.errors.push(self.error_at(
            format!("Type '{ty_name}' has no member '{method}'"),
            span,
            "S002",
        ));
    }

    pub(crate) fn infer_member_type(&self, object_ty: &TypeId, method: &str) -> TypeId {
        if method == "copy" {
            if let TypeId::Named(n) = object_ty {
                if self.dataclass_names.contains(n) {
                    return TypeId::Named(n.clone());
                }
            }
        }
        if method == "description" {
            if self.type_id_satisfies_protocol(object_ty, "CustomStringConvertible") {
                return TypeId::String;
            }
        }
        if method == "debugDescription" {
            if self.type_id_satisfies_protocol(object_ty, "CustomDebugStringConvertible") {
                return TypeId::String;
            }
        }
        if method == "hash" {
            if self.type_id_satisfies_protocol(object_ty, "Hashable") {
                return TypeId::Void;
            }
        }
        if matches!(object_ty, TypeId::String) {
            if method == "isEmpty" || method == "hasSuffix" {
                return TypeId::Bool;
            }
            if method == "replacingOccurrences" {
                return TypeId::String;
            }
            if let Some(m) = lookup_string_method(method) {
                return type_name_to_id(method_return_type(m));
            }
        }
        if let TypeId::Array(elem) = object_ty {
            if let Some(m) = lookup_array_method(method) {
                // sorted/reverse/filter preserve element type (≠ Array<Unknown>).
                if matches!(
                    m,
                    MethodId::ArrSorted | MethodId::ArrReverse | MethodId::ArrFilter
                ) {
                    return TypeId::Array(elem.clone());
                }
                return type_name_to_id(method_return_type(m));
            }
            // Extension-only Array APIs: facade still bare `Array` / `Array<Array>`.
            match method {
                "sortBy" => return TypeId::Array(elem.clone()),
                "chunked" => {
                    return TypeId::Array(Box::new(TypeId::Array(elem.clone())));
                }
                // Prefer MethodCall path (sees `other`); without args stay conservative.
                // Heterogeneous pairs → Array<Array<Object>> (≠ Unknown; avoids Array carve-out).
                "zip" => {
                    return TypeId::Array(Box::new(TypeId::Array(Box::new(TypeId::Named(
                        SmolStr::new("Object"),
                    )))));
                }
                _ => {}
            }
        }
        if let TypeId::Tuple { elements } = object_ty {
            if let Ok(idx) = method.parse::<usize>() {
                if let Some((_, ty)) = elements.get(idx) {
                    return ty.clone();
                }
            }
            if let Some((_, ty)) = elements
                .iter()
                .find(|(label, _)| label.as_ref().is_some_and(|l| l.as_str() == method))
            {
                return ty.clone();
            }
        }
        if matches!(object_ty, TypeId::Set(_)) {
            if let Some(m) = lookup_set_method(method) {
                if method_return_type(m) == "Set" {
                    if let TypeId::Set(inner) = object_ty {
                        return TypeId::Set(inner.clone());
                    }
                }
                return type_name_to_id(method_return_type(m));
            }
        }
        if let TypeId::Dictionary { key, value } = object_ty {
            if let Some(m) = lookup_dict_method(method) {
                // keys/values keep the parameters sema already stored.
                // Bare "Array" from the method table would become Array<Unknown>.
                return match m {
                    MethodId::DictKeys => TypeId::Array(key.clone()),
                    MethodId::DictValues => TypeId::Array(value.clone()),
                    _ => type_name_to_id(method_return_type(m)),
                };
            }
        }
        if let TypeId::Named(n) = object_ty {
            // Typed Object shares Dictionary bag ops; Object-only APIs fall through
            // to `type_extension_methods` (injected from object.sy).
            if n.as_str() == "Object" {
                if let Some(m) = lookup_dict_method(method) {
                    // Object keys are always String; values are type-erased Object (≠ bare Array).
                    return match m {
                        MethodId::DictKeys => TypeId::Array(Box::new(TypeId::String)),
                        MethodId::DictValues => {
                            TypeId::Array(Box::new(TypeId::Named(SmolStr::new("Object"))))
                        }
                        _ => type_name_to_id(method_return_type(m)),
                    };
                }
                if matches!(method, "valueForKey" | "put") {
                    return type_name_to_id("Object");
                }
                if method == "isEmpty" {
                    return TypeId::Bool;
                }
            }
            if let Some(exports) = self.import_exports.get(n) {
                if let Some(ty) = exports.get(method) {
                    return ty.clone();
                }
            }
            if n.as_str() == "Result" {
                return match method {
                    "ok" | "err" => TypeId::Result {
                        ok: Box::new(TypeId::Unknown),
                        err: Box::new(TypeId::Unknown),
                    },
                    "isOk" | "isErr" => TypeId::Bool,
                    _ => TypeId::Unknown,
                };
            }
            if self
                .enum_cases
                .get(n)
                .is_some_and(|cases| cases.iter().any(|c| c.as_str() == method))
            {
                return TypeId::Named(n.clone());
            }
            if let Some(tm) = lookup_type_method(n, method) {
                if let Some(host) = tm.host {
                    return type_name_to_id(host_fn_return_type(host));
                }
                if let Some(m) = tm.instance {
                    return type_name_to_id(method_return_type(m));
                }
            }
            if let Some(ty) = self.lookup_surface_member_type(n, method) {
                return ty;
            }
        }
        // `Int` / `Int8` bare names are `TypeId::Numeric` (stdlib globals), not `Named`.
        if let TypeId::Numeric(kind) = object_ty {
            let surface = SmolStr::new(syx_stdlib::type_name_with_alias(*kind));
            if let Some(ty) = self.lookup_surface_member_type(&surface, method) {
                return ty;
            }
        }
        // S1-I8: `any P` / `some P` member lookup via protocol requirements.
        if let TypeId::OpaqueAny(inner) | TypeId::OpaqueSome(inner) = object_ty {
            if let TypeId::Named(n) = inner.as_ref() {
                if let Some(ty) = self.lookup_surface_member_type(n, method) {
                    return ty;
                }
            }
        }
        if method == "json" {
            return type_name_to_id("Object");
        }
        TypeId::Unknown
    }

    pub(crate) fn lookup_surface_member_type(&self, surface: &SmolStr, method: &str) -> Option<TypeId> {
        if let Some(fields) = self.dataclass_fields.get(surface) {
            if let Some((_, ty)) = fields.iter().find(|(f, _)| f.as_str() == method) {
                return Some(ty.clone());
            }
        }
        if let Some(methods) = self.class_methods.get(surface) {
            if let Some((_, ty)) = methods.iter().find(|(m, _)| m.as_str() == method) {
                return Some(ty.clone());
            }
        }
        if let Some(methods) = self.type_extension_methods.get(surface) {
            if let Some((_, ty)) = methods.iter().find(|(m, _)| m.as_str() == method) {
                return Some(ty.clone());
            }
        }
        None
    }

    pub(crate) fn push_surface_members(&self, surface: &SmolStr, out: &mut Vec<(SmolStr, SmolStr)>) {
        if let Some(fields) = self.dataclass_fields.get(surface) {
            for (field, ty) in fields {
                out.push((field.clone(), SmolStr::new(type_id_to_name(ty))));
            }
        }
        if let Some(methods) = self.class_methods.get(surface) {
            for (method, ty) in methods {
                if let Some(field) = method.as_str().strip_prefix("__get_") {
                    out.push((SmolStr::new(field), SmolStr::new(type_id_to_name(ty))));
                } else if method.as_str().starts_with("__set_") {
                    continue;
                } else {
                    out.push((method.clone(), SmolStr::new(type_id_to_name(ty))));
                }
            }
        }
        if let Some(methods) = self.type_extension_methods.get(surface) {
            for (method, ty) in methods {
                if let Some(field) = method.as_str().strip_prefix("__get_") {
                    out.push((SmolStr::new(field), SmolStr::new(type_id_to_name(ty))));
                } else if method.as_str().starts_with("__set_") {
                    continue;
                } else {
                    out.push((method.clone(), SmolStr::new(type_id_to_name(ty))));
                }
            }
        }
    }
}


