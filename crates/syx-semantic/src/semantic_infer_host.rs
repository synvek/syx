//! SYX semantic infer/host face (S2-I258).
//! Zero-semantics peel from semantic_analyzer.rs.

use crate::semantic_analyzer::SemanticAnalyzer;
use crate::{
    expr_span_start, has_attr, normalize_type_arg_name, type_id_to_name, TypeId,
};
use crate::host_fn_capability;
use smol_str::SmolStr;
use syx_ast::{
    CallArg, Decl, Expr, Literal, Module, Stmt, TypeExpr,
};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_stdlib::{
    widen_for_binary,
    HostFnId, NumericKind,
};

impl SemanticAnalyzer {
    pub(crate) fn infer_enum_case_type_args(
        &self,
        type_params: &[SmolStr],
        enum_name: &SmolStr,
        case_name: &SmolStr,
        args: &[CallArg],
    ) -> Vec<SmolStr> {
        let mut type_args: Vec<SmolStr> = type_params
            .iter()
            .map(|_| SmolStr::new("Unknown"))
            .collect();
        if let Some(fields) = self
            .enum_case_fields(enum_name, case_name)
            .filter(|f| !f.is_empty())
        {
            for (i, (_, param_ty)) in fields.iter().enumerate() {
                if let Some(arg) = args.get(i) {
                    let arg_ty = self.infer_expr_type_name(&arg.value);
                    if let TypeExpr::Named { name, .. } = param_ty {
                        if let Some(idx) = type_params.iter().position(|p| p == name) {
                            type_args[idx] = arg_ty;
                        }
                    }
                }
            }
        }
        type_args
    }

    fn enum_case_fields(
        &self,
        enum_name: &SmolStr,
        case_name: &SmolStr,
    ) -> Option<Vec<(SmolStr, TypeExpr)>> {
        self.enum_case_field_types
            .get(&(enum_name.clone(), case_name.clone()))
            .cloned()
    }

    pub(crate) fn promote_numeric(&self, a: TypeId, b: TypeId) -> TypeId {
        let Some(ka) = a.as_numeric_kind() else {
            return TypeId::Unknown;
        };
        let Some(kb) = b.as_numeric_kind() else {
            return TypeId::Unknown;
        };
        if let Some(w) = widen_for_binary(ka, kb) {
            TypeId::Numeric(w)
        } else {
            TypeId::Unknown
        }
    }

    /// Binary arithmetic promotion; signed/unsigned mix → **S014**.
    pub(crate) fn promote_numeric_binop(
        &mut self,
        a: TypeId,
        b: TypeId,
        span: syx_diagnostics::Span,
    ) -> TypeId {
        let Some(ka) = a.as_numeric_kind() else {
            return TypeId::Unknown;
        };
        let Some(kb) = b.as_numeric_kind() else {
            return TypeId::Unknown;
        };
        if let Some(w) = widen_for_binary(ka, kb) {
            TypeId::Numeric(w)
        } else {
            self.errors.push(self.error_at(
                "cannot mix signed and unsigned integer types in binary operation; use an explicit cast",
                span,
                "S014",
            ));
            TypeId::Unknown
        }
    }

    pub(crate) fn service_resolve_type_name_from_arg(&self, arg: &Expr) -> Option<SmolStr> {
        match arg {
            Expr::Literal {
                value: syx_ast::Literal::String(type_name),
                ..
            } => Some(SmolStr::new(type_name.as_str())),
            Expr::Ident { name, .. } if self.service_class_names.contains(name) => Some(name.clone()),
            _ => None,
        }
    }

    pub(crate) fn analyze_service_resolve(
        &mut self,
        args: &[syx_ast::CallArg],
        span: syx_diagnostics::Span,
    ) -> SyxResult<TypeId> {
        let Some(arg) = args.first() else {
            self.errors.push(self.error_at(
                "Di.resolve / resolve expects a string literal or @Service/@Component type identifier",
                span,
                "E010",
            ));
            return Ok(TypeId::Unknown);
        };
        match self.service_resolve_type_name_from_arg(&arg.value) {
            Some(type_name) => {
                if !self.service_class_names.contains(&type_name) {
                    self.errors.push(self.error_at(
                        format!(
                            "service resolve: '{type_name}' is not a registered @Service/@Component type"
                        ),
                        span,
                        "E010",
                    ));
                    Ok(TypeId::Unknown)
                } else {
                    Ok(TypeId::Named(type_name))
                }
            }
            None => {
                self.errors.push(self.error_at(
                    "service resolve requires a string literal or registered type identifier; dynamic args are rejected (typed resolve harden)",
                    span,
                    "E010",
                ));
                Ok(TypeId::Unknown)
            }
        }
    }

    pub(crate) fn check_host_fn_allowed(&mut self, id: HostFnId, span: syx_diagnostics::Span) {
        let cfg = syx_config::runtime_config();
        let crypto_host = matches!(
            id,
            HostFnId::CryptoSha256 | HostFnId::CryptoSha512 | HostFnId::CryptoHmacSha256
                | HostFnId::CryptoAesGcmEncrypt
        );
        if crypto_host {
            if cfg.embed.sandbox {
                self.errors.push(self.error_at(
                    "Crypto Host functions are blocked in embed sandbox",
                    span,
                    "S021",
                ));
            } else if !cfg.enable_crypto {
                self.errors.push(self.error_at(
                    "Crypto is disabled; re-enable with default settings or omit --no-crypto",
                    span,
                    "S021",
                ));
            }
        }
        let simd_host = matches!(id, HostFnId::SimdFromArray | HostFnId::SimdReduceAdd);
        if simd_host && !syx_config::simd_enabled() {
            self.errors.push(self.error_at(
                "SIMD Host functions require SIMD to be enabled",
                span,
                "S022",
            ));
        }
        // M3-E02 / ADR-310–311: opt-in permission sandbox — net + fs domains.
        let net_host = matches!(id, HostFnId::TcpConnect | HostFnId::HttpGet);
        if net_host && cfg.permission.sandbox && !cfg.permission.allow_net {
            self.errors.push(self.error_at(
                "net Host functions require permission grant; pass --allow-net (or SYX_ALLOW_NET=1) under --permission-sandbox",
                span,
                "S024",
            ));
        }
        let fs_host = matches!(id, HostFnId::FileRead | HostFnId::FileWrite);
        if fs_host && cfg.permission.sandbox && !cfg.permission.allow_fs {
            self.errors.push(self.error_at(
                "fs Host functions require permission grant; pass --allow-fs (or SYX_ALLOW_FS=1) under --permission-sandbox",
                span,
                "S024",
            ));
        }
        let env_host = matches!(id, HostFnId::EnvGet | HostFnId::EnvSet);
        if env_host && cfg.permission.sandbox && !cfg.permission.allow_env {
            self.errors.push(self.error_at(
                "env Host functions require permission grant; pass --allow-env (or SYX_ALLOW_ENV=1) under --permission-sandbox",
                span,
                "S024",
            ));
        }
        let process_host = matches!(id, HostFnId::ProcessExec | HostFnId::ProcessSpawn);
        if process_host && cfg.permission.sandbox && !cfg.permission.allow_process {
            self.errors.push(self.error_at(
                "process Host functions require permission grant; pass --allow-process (or SYX_ALLOW_PROCESS=1) under --permission-sandbox",
                span,
                "S024",
            ));
        }
        // M3-E03 / ADR-313: opt-in capability-table gate — overall status unsupported → S026.
        if cfg.ir_capability_gate {
            let cap_name = format!("{:?}", id);
            if host_fn_capability::host_fn_is_unsupported(&cap_name) {
                self.errors.push(self.error_at(
                    &format!(
                        "HostFn `{cap_name}` is capability-table status unsupported; refuse under --ir-capability-gate (or SYX_IR_CAPABILITY_GATE=1)"
                    ),
                    span,
                    "S026",
                ));
            }
        }
    }

    pub(crate) fn infer_simd_element_from_args(&self, args: &[CallArg]) -> Option<NumericKind> {
        let first = args.first()?;
        let ty = self.expr_types.get(&expr_span_start(&first.value))?;
        if let TypeId::Array(inner) = ty {
            return inner.as_numeric_kind();
        }
        None
    }

    fn known_protocol(name: &str) -> bool {
        syx_stdlib::known_protocol(name)
            || matches!(name, "Iterator" | "Sequence" | "Codable" | "Sendable")
    }

    /// Phase 22: user-defined module protocols satisfy where-clause constraints.
    pub(crate) fn is_protocol_constraint(&self, name: &str) -> bool {
        let n = SmolStr::new(name);
        // S1-I8: `extension Array where Element: User` — class/data class as type bound.
        Self::known_protocol(name)
            || self.protocol_assoc_types.contains_key(&n)
            || self.protocol_names.contains(&n)
            || self.class_names.contains(&n)
            || self.dataclass_names.contains(&n)
    }

    pub(crate) fn apply_module_default_isolation(&mut self, module: &Module) {
        let Some(default) = self.module_default_isolation.clone() else {
            return;
        };
        for decl in &module.decls {
            let Decl::Function { name, attrs, .. } = decl else {
                continue;
            };
            if self.function_global_actor.contains_key(name)
                || self.nonisolated_functions.contains(name)
                || has_attr(attrs, "MainActor")
                || has_attr(attrs, "nonisolated")
            {
                continue;
            }
            match default.as_str() {
                "MainActor" => {
                    self.main_actor_functions.insert(name.clone());
                    self.function_global_actor
                        .insert(name.clone(), SmolStr::new("MainActor"));
                }
                "nonisolated" => {
                    self.nonisolated_functions.insert(name.clone());
                }
                custom if self.global_actor_names.contains(&SmolStr::new(custom)) => {
                    self.function_global_actor
                        .insert(name.clone(), SmolStr::new(custom));
                }
                _ => {}
            }
        }
    }

    pub(crate) fn check_opaque_protocol_assignable(
        &mut self,
        expected: &TypeId,
        found: &TypeId,
        name: &str,
    ) -> bool {
        let protocol = match expected {
            TypeId::OpaqueAny(inner) | TypeId::OpaqueSome(inner) => inner.as_ref(),
            _ => return false,
        };
        let TypeId::Named(protocol_name) = protocol else {
            return false;
        };
        if let TypeId::OpaqueAny(found_inner) | TypeId::OpaqueSome(found_inner) = found {
            if found_inner.as_ref() == protocol {
                return true;
            }
        }
        if self.type_id_satisfies_protocol(found, protocol_name.as_str()) {
            return true;
        }
        self.errors.push(
            SyxDiagnostic::error(format!(
                "Type '{}' does not conform to protocol '{}' for '{name}'",
                type_id_to_name(found),
                protocol_name
            ))
            .with_code("S014"),
        );
        true
    }

    /// Element type is still an open formal (`T`/`U`/GenericParam) — skip concrete checks.
    pub(crate) fn is_open_generic_elem(&self, ty: &TypeId) -> bool {
        match ty {
            TypeId::Unknown | TypeId::GenericParam(_) => true,
            TypeId::Named(n) => self.current_type_params.contains(n),
            _ => false,
        }
    }

    pub(crate) fn infer_class_method_type_args(
        &mut self,
        type_param_count: usize,
        args: &[CallArg],
    ) -> Vec<SmolStr> {
        for arg in args {
            if let Ok(TypeId::Array(elem)) = self.analyze_expr(&arg.value) {
                let elem_name = normalize_type_arg_name(type_id_to_name(&elem));
                if elem_name.as_str() != "Unknown" {
                    return vec![elem_name; type_param_count];
                }
            }
        }
        self.infer_call_type_args(type_param_count, args)
    }

    pub(crate) fn infer_call_type_args(&self, type_param_count: usize, args: &[CallArg]) -> Vec<SmolStr> {
        let mut type_args = Vec::with_capacity(type_param_count);
        for _ in 0..type_param_count {
            type_args.push(SmolStr::new("Unknown"));
        }
        for arg in args {
            if let Expr::Call { callee, args: ctor_args, .. } = &arg.value {
                if let Expr::Member { object, name, .. } = callee.as_ref() {
                    if let Expr::Ident { name: type_name, .. } = object.as_ref() {
                        let template = self
                            .generic_enums
                            .get(type_name)
                            .or_else(|| self.generic_final_classes.get(type_name));
                        if let Some((type_params, _)) = template {
                            let inferred = self.infer_enum_case_type_args(
                                type_params,
                                type_name,
                                name,
                                ctor_args,
                            );
                            for (i, t) in inferred.into_iter().enumerate() {
                                if t.as_str() != "Unknown" && i < type_args.len() {
                                    type_args[i] = t;
                                }
                            }
                        }
                    }
                }
            }
        }
        for (i, arg) in args.iter().enumerate() {
            if i < type_args.len() && type_args[i].as_str() == "Unknown" {
                if let Some(ty) = self.expr_types.get(&expr_span_start(&arg.value)) {
                    // Prefer element of `Array<T>` / `Set<T>` when binding a lone type param.
                    type_args[i] = match ty {
                        TypeId::Array(inner) | TypeId::Set(inner) => type_id_to_name(inner),
                        other => type_id_to_name(other),
                    };
                } else {
                    type_args[i] = self.infer_expr_type_name(&arg.value);
                }
            }
        }
        // S1-I5: `compose(f: {…}, g: {…})` — closure args leave all params Unknown;
        // fill with platform Int so mono mangling matches (numeric book samples).
        if type_args.iter().all(|t| t.as_str() == "Unknown")
            && !args.is_empty()
            && args.iter().all(|a| matches!(a.value, Expr::Closure { .. }))
        {
            let int_name = SmolStr::new(if cfg!(target_pointer_width = "64") {
                "Int64"
            } else {
                "Int32"
            });
            for slot in &mut type_args {
                *slot = int_name.clone();
            }
        }
        // S1-I6/I8: fill Unknown slots — prefer each closure arg's result type
        // (`zipMap` C from `{ n, s in "…" }` → String) before copying A/Int.
        if type_args.iter().any(|t| t.as_str() == "Unknown") {
            for (i, arg) in args.iter().enumerate() {
                if i < type_args.len()
                    && type_args[i].as_str() == "Unknown"
                    && matches!(arg.value, Expr::Closure { .. })
                {
                    let inferred = self.infer_expr_type_name(&arg.value);
                    if inferred.as_str() != "Unknown" {
                        type_args[i] = inferred;
                    }
                }
            }
        }
        if type_args.iter().any(|t| t.as_str() == "Unknown")
            && args.iter().any(|a| matches!(a.value, Expr::Closure { .. }))
        {
            let int_name = SmolStr::new(if cfg!(target_pointer_width = "64") {
                "Int64"
            } else {
                "Int32"
            });
            let fill = type_args
                .iter()
                .find(|t| t.as_str() != "Unknown")
                .cloned()
                .unwrap_or(int_name);
            for slot in &mut type_args {
                if slot.as_str() == "Unknown" {
                    *slot = fill.clone();
                }
            }
        }
        type_args
            .into_iter()
            .map(normalize_type_arg_name)
            .collect()
    }

    fn infer_expr_type_name(&self, expr: &Expr) -> SmolStr {
        match expr {
            Expr::Literal { value, .. } => match value {
                Literal::Integer { .. } | Literal::Double { .. } => {
                    SmolStr::new(type_id_to_name(&self.literal_numeric_type(value)).as_str())
                }
                Literal::Bool(_) => SmolStr::new("Bool"),
                Literal::String(_) | Literal::Interpolated(_) | Literal::Regex(_) => {
                    SmolStr::new("String")
                }
                _ => SmolStr::new("Unknown"),
            },
            Expr::Ident { name, .. } => self
                .scope
                .lookup(name)
                .or_else(|| self.globals.get(name))
                .map(|s| type_id_to_name(&s.ty))
                .unwrap_or_else(|| SmolStr::new("Unknown")),
            Expr::Array { elements, .. } => {
                let elem = elements
                    .first()
                    .map(|e| self.infer_expr_type_name(e))
                    .unwrap_or_else(|| SmolStr::new("Unknown"));
                SmolStr::new(format!("Array<{elem}>"))
            }
            // S1-I6: closure result feeds `map<T,U>` U (and similar) type-arg slots.
            Expr::Closure { body, .. } => match body {
                syx_ast::ClosureBody::Expr(e) => self.infer_expr_type_name(e),
                syx_ast::ClosureBody::Block(stmts) => stmts
                    .iter()
                    .rev()
                    .find_map(|s| match s {
                        Stmt::Return { value: Some(v), .. } | Stmt::Expr { expr: v, .. } => {
                            Some(self.infer_expr_type_name(v))
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| SmolStr::new("Unknown")),
            },
            Expr::Binary { left, right, .. } => {
                let l = self.infer_expr_type_name(left);
                if l.as_str() != "Unknown" {
                    l
                } else {
                    self.infer_expr_type_name(right)
                }
            }
            _ => SmolStr::new("Unknown"),
        }
    }
}
