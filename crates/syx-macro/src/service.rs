//! Phase 122–131 Macro M12–M21: `@Service` / `@Scope` / `@Qualifier` / `@Primary` / `@Profile` /
//! advice / `@Bean` / `@Lazy` / `@Pointcut`.
//! Fingerprints: `macro-service-v5:conditional-qualifier`, `macro-service-v5:conditional-profile`,
//! `macro-service-v5:di-package-ga`, `macro-service-v6:polymorphic-base-inject`,
//! `macro-service-v6:field-qualifier-metadata`, `macro-m15:service-method-interceptor-mvp`,
//! `macro-service-v7:advice-chain`, `runtime-service-v7:protocol-witness`,
//! `macro-service-v8:advice-proceed`, `macro-service-v8:configuration-bean-mvp`,
//! `macro-service-v9:lazy-cycle-proxy-mvp`,
//! `macro-service-v10:bean-param-autowire-mvp`,
//! `macro-service-v11:pointcut-name-mvp`,
//! `macro-service-v12:pointcut-execution-name-mvp`,
//! `macro-service-v13:before-after-pointcut-bind-mvp`,
//! `macro-service-v14:pointcut-within-name-mvp`,
//! `macro-service-v15:pointcut-args-name-mvp`,
//! `macro-service-v16:pointcut-and-compose-mvp`,
//! `macro-service-v17:pointcut-or-compose-mvp`,
//! `macro-service-v19:pointcut-within-wildcard-mvp`

use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_ast::{AccessLevel, Attribute, AttributeArg, Decl, Expr, Literal, Param, ParamLabel, Stmt, TypeExpr};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAutowire {
    pub field: SmolStr,
    pub type_name: SmolStr,
    /// Optional `@Qualifier` on the `@Autowired` field.
    pub qualifier: Option<SmolStr>,
}

#[derive(Debug, Clone)]
pub struct InitInject {
    pub param: SmolStr,
    pub dep_type: SmolStr,
    pub qualifier: Option<SmolStr>,
}

#[derive(Debug, Clone)]
pub struct ServiceSpec {
    pub type_name: SmolStr,
    /// All dependency types (ctor `@Inject` + field `@Autowired`) for graph checks.
    pub deps: Vec<SmolStr>,
    /// Ctor `@Inject` slots in parameter order.
    pub init_inject: Vec<InitInject>,
    pub field_autowires: Vec<FieldAutowire>,
    pub post_construct: Option<SmolStr>,
    pub pre_destroy: Option<SmolStr>,
    pub scope: SmolStr,
    pub kind: &'static str,
    /// `@Qualifier("name")` on the service type (also a resolve alias).
    pub qualifier: Option<SmolStr>,
    /// `@Primary` — preferred among unqualified multi-binding candidates.
    pub primary: bool,
    /// `@Profile("name")` — conditional registration.
    pub profile: Option<SmolStr>,
    /// Superclass + protocol conformances used as shared binding keys.
    pub binding_aliases: Vec<SmolStr>,
    /// Protocol conformances registered in the DI witness table (Phase 126).
    pub protocol_conformances: Vec<SmolStr>,
    /// Phase 128: `@Lazy` — defer factory until first resolve / proxy use.
    pub lazy: bool,
}

/// Phase 127: `@Configuration` factory `@Bean` binding.
#[derive(Debug, Clone)]
pub struct BeanSpec {
    pub config_type: SmolStr,
    pub method: SmolStr,
    pub return_type: SmolStr,
    /// Phase 128: `@Lazy` on the `@Bean` method.
    pub lazy: bool,
    /// Phase 129: `@Bean` method params resolved by type / `@Autowired` (+ optional `@Qualifier`).
    pub params: Vec<InitInject>,
}

fn attr_named(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.name.as_str() == name)
}

fn is_inject_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Inject")
        || attr_named(attrs, "inject")
        || attr_named(attrs, "Autowired")
}

fn is_autowired_field_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Autowired") || attr_named(attrs, "autowired")
}

fn scope_from_attrs(attrs: &[Attribute]) -> SyxResult<SmolStr> {
    for a in attrs {
        if !matches!(a.name.as_str(), "Scope" | "scope") {
            continue;
        }
        let scope = a.args.first().map(|arg| match arg {
            AttributeArg::String(s) => s.as_str(),
            AttributeArg::Ident(s) => s.as_str(),
        });
        return match scope {
            None | Some("singleton") => Ok(SmolStr::new("singleton")),
            Some("prototype") => Ok(SmolStr::new("prototype")),
            Some("request") => Ok(SmolStr::new("request")),
            Some(other) => Err(SyxDiagnostic::error(format!(
                "@Scope(\"{other}\") is not supported; use \"singleton\" (default), \"prototype\", or \"request\""
            ))
            .with_code("E011")),
        };
    }
    Ok(SmolStr::new("singleton"))
}

fn type_name_from_param(param: &Param) -> Option<SmolStr> {
    match &param.ty {
        Some(TypeExpr::Named { name, .. }) => Some(name.clone()),
        _ => None,
    }
}

fn attr_string_arg(attrs: &[Attribute], names: &[&str]) -> Option<SmolStr> {
    for a in attrs {
        if !names.iter().any(|n| a.name.as_str() == *n) {
            continue;
        }
        let arg = a.args.first().map(|arg| match arg {
            AttributeArg::String(s) => SmolStr::new(s.as_str()),
            AttributeArg::Ident(s) => s.clone(),
        })?;
        return Some(arg);
    }
    None
}

fn qualifier_from_attrs(attrs: &[Attribute]) -> Option<SmolStr> {
    attr_string_arg(attrs, &["Qualifier", "qualifier"])
}

fn profile_from_attrs(attrs: &[Attribute]) -> Option<SmolStr> {
    attr_string_arg(attrs, &["Profile", "profile"])
}

fn is_primary_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Primary") || attr_named(attrs, "primary")
}

fn is_lazy_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Lazy") || attr_named(attrs, "lazy")
}

fn init_inject_from_params(params: &[Param]) -> (Vec<InitInject>, Vec<SmolStr>) {
    let mut inject = Vec::new();
    let mut deps = Vec::new();
    for p in params {
        if is_inject_attr(&p.attrs) {
            if let Some(dep) = type_name_from_param(p) {
                if !deps.contains(&dep) {
                    deps.push(dep.clone());
                }
                inject.push(InitInject {
                    param: p.name.clone(),
                    dep_type: dep,
                    qualifier: qualifier_from_attrs(&p.attrs),
                });
            }
        }
    }
    (inject, deps)
}

fn field_autowires_from_members(members: &[Decl]) -> Vec<FieldAutowire> {
    let mut out = Vec::new();
    for m in members {
        let Decl::VarGlobal { name, attrs, ty, .. } = m else {
            continue;
        };
        if !is_autowired_field_attr(attrs) {
            continue;
        }
        if let Some(TypeExpr::Named { name: type_name, .. }) = ty {
            out.push(FieldAutowire {
                field: name.clone(),
                type_name: type_name.clone(),
                qualifier: qualifier_from_attrs(attrs),
            });
        }
    }
    out
}

fn post_construct_from_members(members: &[Decl]) -> Option<SmolStr> {
    for m in members {
        let Decl::Function { name, attrs, .. } = m else {
            continue;
        };
        if attr_named(attrs, "PostConstruct") || attr_named(attrs, "postConstruct") {
            return Some(name.clone());
        }
    }
    None
}

fn pre_destroy_from_members(members: &[Decl]) -> Option<SmolStr> {
    for m in members {
        let Decl::Function { name, attrs, .. } = m else {
            continue;
        };
        if attr_named(attrs, "PreDestroy") || attr_named(attrs, "preDestroy") {
            return Some(name.clone());
        }
    }
    None
}

fn merge_deps(ctor_deps: &[SmolStr], fields: &[FieldAutowire]) -> Vec<SmolStr> {
    let mut deps = ctor_deps.to_vec();
    for f in fields {
        if !deps.contains(&f.type_name) {
            deps.push(f.type_name.clone());
        }
    }
    deps
}

/// Reject `@Service` / `@Component` on non-type decls → E006.
pub fn reject_service_on_non_types(decls: &[Decl]) -> SyxResult<()> {
    for decl in decls {
        match decl {
            Decl::Function { attrs, .. }
            | Decl::OperatorFunc { attrs, .. }
            | Decl::VarGlobal { attrs, .. } => {
                if attr_named(attrs, "Service")
                    || attr_named(attrs, "Component")
                    || attr_named(attrs, "service")
                    || attr_named(attrs, "component")
                {
                    return Err(SyxDiagnostic::error(
                        "@Service/@Component may only attach to class or data class types",
                    )
                    .with_code("E006"));
                }
                if attr_named(attrs, "PreDestroy") || attr_named(attrs, "preDestroy") {
                    return Err(SyxDiagnostic::error(
                        "@PreDestroy may only attach to @Service/@Component methods",
                    )
                    .with_code("E012"));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn collect_services(module: &syx_ast::Module) -> SyxResult<Vec<ServiceSpec>> {
    reject_service_on_non_types(&module.decls)?;
    let protocol_names: std::collections::HashSet<SmolStr> = module
        .decls
        .iter()
        .filter_map(|d| match d {
            Decl::Protocol { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect();
    let active = syx_meta::service_di_active_profile();
    let mut out = Vec::new();
    for decl in &module.decls {
        let (attrs, name, members, superclass, conformances): (
            &[Attribute],
            &SmolStr,
            &[Decl],
            Option<&SmolStr>,
            &[SmolStr],
        ) = match decl {
            Decl::Class {
                attrs,
                name,
                members,
                superclass,
                conformances,
                ..
            } => (
                attrs.as_slice(),
                name,
                members.as_slice(),
                superclass.as_ref(),
                conformances.as_slice(),
            ),
            Decl::DataClass { attrs, name, .. } => (attrs.as_slice(), name, &[][..], None, &[][..]),
            Decl::Actor { name, members, .. } => {
                let _ = (name, members);
                continue;
            }
            _ => continue,
        };
        let kind = if attr_named(attrs, "Service") || attr_named(attrs, "service") {
            "Service"
        } else if attr_named(attrs, "Component") || attr_named(attrs, "component") {
            "Component"
        } else {
            continue;
        };
        let profile = profile_from_attrs(attrs);
        // Compile-time filter when SYX_DI_PROFILE / set profile is known.
        if !syx_meta::service_di_profile_matches(
            profile.as_ref().map(|s| s.as_str()),
            active.as_deref(),
        ) {
            continue;
        }
        let scope = scope_from_attrs(attrs)?;
        let mut init_inject = Vec::new();
        let mut ctor_deps = Vec::new();
        for m in members {
            if let Decl::Init { params, .. } = m {
                let (inject, deps) = init_inject_from_params(params.as_slice());
                init_inject = inject;
                ctor_deps = deps;
            }
        }
        let field_autowires = field_autowires_from_members(members);
        let post_construct = post_construct_from_members(members);
        let pre_destroy = pre_destroy_from_members(members);
        let deps = merge_deps(&ctor_deps, &field_autowires);
        let mut binding_aliases = Vec::new();
        let mut protocol_conformances = Vec::new();
        if let Some(super_name) = superclass {
            binding_aliases.push(super_name.clone());
            if protocol_names.contains(super_name) {
                protocol_conformances.push(super_name.clone());
            }
        }
        for c in conformances {
            protocol_conformances.push(c.clone());
            if !binding_aliases.contains(c) {
                binding_aliases.push(c.clone());
            }
        }
        out.push(ServiceSpec {
            type_name: name.clone(),
            deps,
            init_inject,
            field_autowires,
            post_construct,
            pre_destroy,
            scope,
            kind,
            qualifier: qualifier_from_attrs(attrs),
            primary: is_primary_attr(attrs),
            profile,
            binding_aliases,
            protocol_conformances,
            lazy: is_lazy_attr(attrs),
        });
    }
    Ok(out)
}

fn reject_bean_on_non_configuration(decls: &[Decl]) -> SyxResult<()> {
    for decl in decls {
        let members = match decl {
            Decl::Class { attrs, members, .. } if !is_configuration_attr(attrs) => members.as_slice(),
            _ => continue,
        };
        for m in members {
            if let Decl::Function { attrs, name, .. } = m {
                if is_bean_attr(attrs) {
                    return Err(SyxDiagnostic::error(format!(
                        "@Bean on '{name}' requires enclosing @Configuration type"
                    ))
                    .with_code("E015"));
                }
            }
        }
    }
    Ok(())
}

/// Collect `@Bean` factory methods declared on `@Configuration` classes.
pub fn collect_beans(module: &syx_ast::Module) -> SyxResult<Vec<BeanSpec>> {
    reject_bean_on_non_configuration(&module.decls)?;
    let mut out = Vec::new();
    for decl in &module.decls {
        let Decl::Class { attrs, name, members, .. } = decl else {
            continue;
        };
        if !is_configuration_attr(attrs) {
            continue;
        }
        for m in members {
            let Decl::Function {
                name: method,
                attrs: method_attrs,
                return_type,
                params,
                ..
            } = m
            else {
                continue;
            };
            if !is_bean_attr(method_attrs) {
                continue;
            }
            let Some(TypeExpr::Named { name: ret, .. }) = return_type else {
                return Err(SyxDiagnostic::error(format!(
                    "@Bean method '{method}' on '{name}' requires an explicit return type"
                ))
                .with_code("E015"));
            };
            // Phase 129: every `@Bean` param is a type-resolve inject point
            // (`@Autowired` optional; `@Qualifier` honored).
            let mut inject = Vec::new();
            for p in params {
                let Some(dep) = type_name_from_param(p) else {
                    return Err(SyxDiagnostic::error(format!(
                        "@Bean method '{method}' param '{}' needs an explicit named type",
                        p.name
                    ))
                    .with_code("E015"));
                };
                inject.push(InitInject {
                    param: p.name.clone(),
                    dep_type: dep,
                    qualifier: qualifier_from_attrs(&p.attrs),
                });
            }
            out.push(BeanSpec {
                config_type: name.clone(),
                method: method.clone(),
                return_type: ret.clone(),
                lazy: is_lazy_attr(method_attrs),
                params: inject,
            });
        }
    }
    Ok(out)
}

fn bean_to_service_spec(b: &BeanSpec) -> ServiceSpec {
    let deps: Vec<SmolStr> = b
        .params
        .iter()
        .map(|p| p.dep_type.clone())
        .collect::<Vec<_>>()
        .into_iter()
        .fold(Vec::new(), |mut acc, d| {
            if !acc.contains(&d) {
                acc.push(d);
            }
            acc
        });
    ServiceSpec {
        type_name: b.return_type.clone(),
        deps,
        init_inject: b.params.clone(),
        field_autowires: Vec::new(),
        post_construct: None,
        pre_destroy: None,
        scope: SmolStr::new("singleton"),
        kind: "Bean",
        qualifier: None,
        primary: false,
        profile: None,
        binding_aliases: Vec::new(),
        protocol_conformances: Vec::new(),
        lazy: b.lazy,
    }
}

fn topo_sort_beans(beans: &[BeanSpec]) -> SyxResult<Vec<usize>> {
    let bean_names: std::collections::HashSet<&str> =
        beans.iter().map(|b| b.return_type.as_str()).collect();
    let graph: IndexMap<SmolStr, Vec<SmolStr>> = beans
        .iter()
        .map(|b| {
            let deps = b
                .params
                .iter()
                .map(|p| p.dep_type.clone())
                .filter(|d| bean_names.contains(d.as_str()))
                .collect();
            (b.return_type.clone(), deps)
        })
        .collect();
    let mut order = Vec::new();
    let mut seen: IndexMap<SmolStr, bool> = IndexMap::new();
    let mut stack = Vec::new();
    fn visit(
        node: &SmolStr,
        beans: &[BeanSpec],
        graph: &IndexMap<SmolStr, Vec<SmolStr>>,
        stack: &mut Vec<SmolStr>,
        seen: &mut IndexMap<SmolStr, bool>,
        order: &mut Vec<usize>,
    ) -> SyxResult<()> {
        if stack.contains(node) {
            return Err(SyxDiagnostic::error(format!(
                "bean dependency cycle involving '{node}'"
            ))
            .with_code("E007"));
        }
        if seen.get(node).copied().unwrap_or(false) {
            return Ok(());
        }
        stack.push(node.clone());
        if let Some(deps) = graph.get(node) {
            for d in deps {
                visit(d, beans, graph, stack, seen, order)?;
            }
        }
        stack.pop();
        seen.insert(node.clone(), true);
        if let Some(idx) = beans.iter().position(|b| b.return_type == *node) {
            order.push(idx);
        }
        Ok(())
    }
    for b in beans {
        if b.lazy {
            continue;
        }
        visit(
            &b.return_type,
            beans,
            &graph,
            &mut stack,
            &mut seen,
            &mut order,
        )?;
    }
    Ok(order)
}

fn codegen_bean_factory_stmts(beans: &[BeanSpec], span: syx_diagnostics::Span) -> SyxResult<Vec<Stmt>> {
    let mut out = Vec::new();
    let mut configs_done = std::collections::HashSet::new();
    for idx in topo_sort_beans(beans)? {
        let b = &beans[idx];
        // Phase 128: `@Lazy` beans materialize on first resolve, not at boot.
        if b.lazy {
            continue;
        }
        if configs_done.insert(b.config_type.as_str()) {
            let config_temp = di_temp_name(b.config_type.as_str());
            out.push(Stmt::VarDecl {
                span,
                mutable: false,
                name: config_temp.clone(),
                ty: Some(TypeExpr::Named {
                    name: b.config_type.clone(),
                    generics: vec![],
                }),
                init: Some(Expr::Call {
                    span,
                    callee: Box::new(ident_expr(span, b.config_type.as_str())),
                    args: vec![],
                }),
            });
        }
        let config_temp = di_temp_name(b.config_type.as_str());
        let bean_temp = di_temp_name(b.return_type.as_str());
        // Phase 129: resolve `@Bean` params from already-constructed service/bean temps.
        let mut call_args = Vec::new();
        for inj in &b.params {
            call_args.push(syx_ast::CallArg {
                label: Some(inj.param.clone()),
                value: ident_expr(span, di_temp_name(inj.dep_type.as_str()).as_str()),
            });
        }
        out.push(Stmt::VarDecl {
            span,
            mutable: false,
            name: bean_temp.clone(),
            ty: Some(TypeExpr::Named {
                name: b.return_type.clone(),
                generics: vec![],
            }),
            init: Some(Expr::Cast {
                span,
                expr: Box::new(Expr::Call {
                    span,
                    callee: Box::new(Expr::Member {
                        span,
                        object: Box::new(ident_expr(span, config_temp.as_str())),
                        name: b.method.clone(),
                    }),
                    args: call_args,
                }),
                target: TypeExpr::Named {
                    name: b.return_type.clone(),
                    generics: vec![],
                },
            }),
        });
        out.push(call_host(
            span,
            "__syx_service_put",
            vec![
                string_lit(span, b.return_type.as_str()),
                ident_expr(span, bean_temp.as_str()),
            ],
        ));
    }
    Ok(out)
}

fn string_lit(span: syx_diagnostics::Span, s: &str) -> Expr {
    Expr::Literal {
        span,
        value: Literal::String(s.into()),
    }
}

fn is_around_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Around") || attr_named(attrs, "around")
}

fn is_before_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Before") || attr_named(attrs, "before")
}

fn is_after_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "After") || attr_named(attrs, "after")
}

fn is_pointcut_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Pointcut") || attr_named(attrs, "pointcut")
}

/// Phase 130 DI v11: `@Pointcut("Type.method")` narrow string pattern.
/// Phase 131 DI v12: also `execution(* Type.method(..))` / `execution(* Type.*(..))`.
/// Phase 133 DI v14: also `within(Type)` type-name pattern.
/// Phase 134 DI v15: also `args(Type)` first-param type-name pattern.
/// Phase 135 DI v16: also `within(T) && args(U)` narrow `&&` compose.
/// Fingerprint: `macro-service-v14:pointcut-within-name-mvp`,
/// `macro-service-v15:pointcut-args-name-mvp`,
/// `macro-service-v16:pointcut-and-compose-mvp`
fn pointcut_patterns_from_attrs(attrs: &[Attribute]) -> Vec<SmolStr> {
    let mut out = Vec::new();
    for a in attrs {
        if !matches!(a.name.as_str(), "Pointcut" | "pointcut") {
            continue;
        }
        if let Some(arg) = a.args.first() {
            let s = match arg {
                AttributeArg::String(s) => SmolStr::new(s.as_str()),
                AttributeArg::Ident(s) => s.clone(),
            };
            if !out.contains(&s) {
                out.push(s);
            }
        }
    }
    out
}

/// Phase 131 DI v12: parsed `@Pointcut` pattern.
/// Phase 133 DI v14: `within(Type)` name-only MVP.
/// Phase 134 DI v15: `args(Type)` first-param type-name MVP.
/// Phase 135 DI v16: `within(T) && args(U)` narrow compose MVP.
/// Phase 136 DI v17: `within(T) || args(U)` narrow compose MVP.
/// Phase 137 DI v18: `!within(Type)` negated inner pattern MVP.
/// Phase 138 DI v19: `within(*Service)` / `within(Foo*)` type-name wildcard MVP.
/// Fingerprint: `macro-service-v12:pointcut-execution-name-mvp`,
/// `macro-service-v14:pointcut-within-name-mvp`,
/// `macro-service-v15:pointcut-args-name-mvp`,
/// `macro-service-v16:pointcut-and-compose-mvp`,
/// `macro-service-v17:pointcut-or-compose-mvp`,
/// `macro-service-v18:pointcut-not-within-mvp`,
/// `macro-service-v19:pointcut-within-wildcard-mvp`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WithinWildcard {
    Exact,
    Prefix,
    Suffix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointcutPattern {
    /// Exact `"Type.method"` (Phase 130).
    Exact { type_name: SmolStr, method: SmolStr },
    /// `execution(* Type.method(..))` or `execution(* Type.*(..))` (`method == None` ⇒ `*`).
    Execution {
        type_name: SmolStr,
        method: Option<SmolStr>,
    },
    /// `within(Type)` — matches any `Type.method` join (Phase 133).
    /// Phase 138: `within(Foo*)` prefix / `within(*Service)` suffix wildcard MVP.
    Within {
        type_name: SmolStr,
        wildcard: WithinWildcard,
    },
    /// `args(Type)` — matches when the first parameter type name equals `Type` (Phase 134).
    Args { type_name: SmolStr },
    /// `within(T) && args(U)` — both name patterns must match (Phase 135).
    And {
        left: Box<PointcutPattern>,
        right: Box<PointcutPattern>,
    },
    /// `within(T) || args(U)` — either name pattern may match (Phase 136).
    Or {
        left: Box<PointcutPattern>,
        right: Box<PointcutPattern>,
    },
    /// `!within(Type)` — negated inner pattern (Phase 137).
    Not {
        inner: Box<PointcutPattern>,
    },
}

fn parse_pointcut_type_name(inner: &str) -> Option<SmolStr> {
    let type_name = inner.trim();
    if type_name.is_empty() {
        return None;
    }
    if !type_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return None;
    }
    Some(SmolStr::new(type_name))
}

/// Parse `within(...)` inner: exact name, `Foo*` prefix, or `*Service` suffix.
/// Fingerprint: `macro-service-v19:pointcut-within-wildcard-mvp`
pub fn parse_within_pattern(inner: &str) -> Option<PointcutPattern> {
    let inner = inner.trim();
    if inner.starts_with('*') {
        let suffix = inner.strip_prefix('*')?.trim();
        if suffix.is_empty() {
            return None;
        }
        let type_name = parse_pointcut_type_name(suffix)?;
        return Some(PointcutPattern::Within {
            type_name,
            wildcard: WithinWildcard::Suffix,
        });
    }
    if inner.ends_with('*') {
        let prefix = inner.strip_suffix('*')?.trim();
        if prefix.is_empty() {
            return None;
        }
        let type_name = parse_pointcut_type_name(prefix)?;
        return Some(PointcutPattern::Within {
            type_name,
            wildcard: WithinWildcard::Prefix,
        });
    }
    let type_name = parse_pointcut_type_name(inner)?;
    Some(PointcutPattern::Within {
        type_name,
        wildcard: WithinWildcard::Exact,
    })
}

/// Whether join type `jt` matches a `within(...)` pattern.
/// Fingerprint: `macro-service-v19:pointcut-within-wildcard-mvp`
pub fn within_type_name_matches(
    wildcard: &WithinWildcard,
    pattern_type: &str,
    join_type: &str,
) -> bool {
    match wildcard {
        WithinWildcard::Exact => pattern_type == join_type,
        WithinWildcard::Prefix => join_type.starts_with(pattern_type),
        WithinWildcard::Suffix => join_type.ends_with(pattern_type),
    }
}

fn type_expr_name(ty: &syx_ast::TypeExpr) -> Option<&str> {
    match ty {
        syx_ast::TypeExpr::Named { name, .. } => Some(name.as_str()),
        syx_ast::TypeExpr::Optional(inner) => type_expr_name(inner),
        _ => None,
    }
}

fn first_param_type_name(params: &[syx_ast::Param]) -> Option<&str> {
    params
        .first()
        .and_then(|p| p.ty.as_ref())
        .and_then(type_expr_name)
}

/// Parse `@Pointcut` / `@Around` string: exact name, `execution(* T.m(..))`, `within(T)`, `args(T)`,
/// narrow `within(T) && args(U)`, or narrow `within(T) || args(U)`.
pub fn parse_pointcut_pattern(raw: &str) -> Option<PointcutPattern> {
    let s = raw.trim();
    if let Some((left, right)) = s.split_once(" && ") {
        let left_pat = parse_pointcut_pattern_single(left.trim())?;
        let right_pat = parse_pointcut_pattern_single(right.trim())?;
        let within_args = matches!(
            (&left_pat, &right_pat),
            (PointcutPattern::Within { .. }, PointcutPattern::Args { .. })
                | (PointcutPattern::Args { .. }, PointcutPattern::Within { .. })
        );
        if within_args {
            return Some(PointcutPattern::And {
                left: Box::new(left_pat),
                right: Box::new(right_pat),
            });
        }
        return None;
    }
    if let Some((left, right)) = s.split_once(" || ") {
        let left_pat = parse_pointcut_pattern_single(left.trim())?;
        let right_pat = parse_pointcut_pattern_single(right.trim())?;
        let within_args = matches!(
            (&left_pat, &right_pat),
            (PointcutPattern::Within { .. }, PointcutPattern::Args { .. })
                | (PointcutPattern::Args { .. }, PointcutPattern::Within { .. })
        );
        if within_args {
            return Some(PointcutPattern::Or {
                left: Box::new(left_pat),
                right: Box::new(right_pat),
            });
        }
        return None;
    }
    parse_pointcut_pattern_single(s)
}

fn parse_pointcut_pattern_single(s: &str) -> Option<PointcutPattern> {
    if let Some(rest) = s.strip_prefix('!') {
        let inner = parse_pointcut_pattern_single(rest.trim())?;
        return Some(PointcutPattern::Not {
            inner: Box::new(inner),
        });
    }
    if let Some(inner) = s.strip_prefix("args(").and_then(|r| r.strip_suffix(')')) {
        return parse_pointcut_type_name(inner).map(|type_name| PointcutPattern::Args { type_name });
    }
    if let Some(inner) = s
        .strip_prefix("within(")
        .and_then(|r| r.strip_suffix(')'))
    {
        return parse_within_pattern(inner);
    }
    if let Some(inner) = s
        .strip_prefix("execution(")
        .and_then(|r| r.strip_suffix(')'))
    {
        let inner = inner.trim();
        // Expect: `* Type.method(..)` or `* Type.*(..)`
        let rest = inner.strip_prefix('*')?.trim_start();
        // Expect: `Type.method(..)` or `Type.*(..)`
        let (type_method, args) = rest.split_once('(')?;
        if args.trim() != "..)" {
            return None;
        }
        let type_method = type_method.trim();
        let (type_name, method) = type_method.split_once('.')?;
        let type_name = type_name.trim();
        let method = method.trim();
        if type_name.is_empty() || method.is_empty() {
            return None;
        }
        if !type_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return None;
        }
        let method = if method == "*" {
            None
        } else {
            if !method
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                return None;
            }
            Some(SmolStr::new(method))
        };
        return Some(PointcutPattern::Execution {
            type_name: SmolStr::new(type_name),
            method,
        });
    }
    let (type_name, method) = s.split_once('.')?;
    if type_name.is_empty() || method.is_empty() || method.contains('.') {
        return None;
    }
    Some(PointcutPattern::Exact {
        type_name: SmolStr::new(type_name),
        method: SmolStr::new(method),
    })
}

fn pointcut_pattern_matches_join(pat: &PointcutPattern, join: &str) -> bool {
    let Some((jt, jm)) = join.split_once('.') else {
        return false;
    };
    match pat {
        PointcutPattern::Exact { type_name, method } => {
            type_name.as_str() == jt && method.as_str() == jm
        }
        PointcutPattern::Execution { type_name, method } => {
            if type_name.as_str() != jt {
                return false;
            }
            match method {
                None => true,
                Some(m) => m.as_str() == jm,
            }
        }
        PointcutPattern::Within {
            type_name,
            wildcard,
        } => within_type_name_matches(wildcard, type_name.as_str(), jt),
        PointcutPattern::Args { .. } => false,
        PointcutPattern::And { left, right } => {
            pointcut_pattern_matches_join(left, join)
                && pointcut_pattern_matches_join(right, join)
        }
        PointcutPattern::Or { left, right } => {
            pointcut_pattern_matches_join(left, join)
                || pointcut_pattern_matches_join(right, join)
        }
        PointcutPattern::Not { inner } => !pointcut_pattern_matches_join(inner, join),
    }
}

fn pointcut_pattern_matches_method(
    pat: &PointcutPattern,
    join: &str,
    params: &[syx_ast::Param],
) -> bool {
    match pat {
        PointcutPattern::Args { type_name } => {
            first_param_type_name(params) == Some(type_name.as_str())
        }
        PointcutPattern::And { left, right } => {
            pointcut_pattern_matches_method(left, join, params)
                && pointcut_pattern_matches_method(right, join, params)
        }
        PointcutPattern::Or { left, right } => {
            pointcut_pattern_matches_method(left, join, params)
                || pointcut_pattern_matches_method(right, join, params)
        }
        PointcutPattern::Not { inner } => {
            !pointcut_pattern_matches_method(inner, join, params)
        }
        _ => pointcut_pattern_matches_join(pat, join),
    }
}

/// Whether pointcut pattern `pc` matches join point `"Type.method"`.
pub fn pointcut_matches_join(pc: &str, join: &str) -> bool {
    let Some(pat) = parse_pointcut_pattern(pc) else {
        return pc == join;
    };
    pointcut_pattern_matches_join(&pat, join)
}

/// Whether pointcut pattern `pc` matches join point `"Type.method"` with method params.
/// Phase 134 DI v15: `args(Type)` matches when the first parameter type name equals `Type`.
/// Phase 135 DI v16: `within(T) && args(U)` requires both patterns to match.
/// Phase 136 DI v17: `within(T) || args(U)` matches when either pattern matches.
/// Fingerprint: `macro-service-v15:pointcut-args-name-mvp`,
/// `macro-service-v16:pointcut-and-compose-mvp`,
/// `macro-service-v17:pointcut-or-compose-mvp`
pub fn pointcut_matches_method(pc: &str, join: &str, params: &[syx_ast::Param]) -> bool {
    let Some(pat) = parse_pointcut_pattern(pc) else {
        return pc == join;
    };
    pointcut_pattern_matches_method(&pat, join, params)
}

/// `@Around` / `@Around("Type.method")` — bare Around keeps Phase 127 behavior;
/// string form binds a registered `@Pointcut` pattern.
fn around_bindings_from_attrs(attrs: &[Attribute]) -> Vec<Option<SmolStr>> {
    advice_string_bindings_from_attrs(attrs, &["Around", "around"])
}

/// Phase 132 DI v13: `@Before` / `@Before("…")` — bare keeps Phase 127; string binds pointcut.
/// Fingerprint: `macro-service-v13:before-after-pointcut-bind-mvp`
fn before_bindings_from_attrs(attrs: &[Attribute]) -> Vec<Option<SmolStr>> {
    advice_string_bindings_from_attrs(attrs, &["Before", "before"])
}

/// Phase 132 DI v13: `@After` / `@After("…")` — bare keeps Phase 127; string binds pointcut.
fn after_bindings_from_attrs(attrs: &[Attribute]) -> Vec<Option<SmolStr>> {
    advice_string_bindings_from_attrs(attrs, &["After", "after"])
}

fn advice_string_bindings_from_attrs(
    attrs: &[Attribute],
    names: &[&str],
) -> Vec<Option<SmolStr>> {
    let mut out = Vec::new();
    for a in attrs {
        if !names.iter().any(|n| a.name.as_str() == *n) {
            continue;
        }
        let binding = a.args.first().map(|arg| match arg {
            AttributeArg::String(s) => SmolStr::new(s.as_str()),
            AttributeArg::Ident(s) => s.clone(),
        });
        out.push(binding);
    }
    out
}

fn join_point_name(type_name: &str, method: &str) -> SmolStr {
    SmolStr::new(format!("{type_name}.{method}"))
}

fn advice_print_stmt(span: syx_diagnostics::Span, prefix: &str, method: &str) -> Stmt {
    Stmt::Expr {
        span,
        expr: Expr::Call {
            span,
            callee: Box::new(Expr::Ident {
                span,
                name: SmolStr::new("print"),
            }),
            args: vec![syx_ast::CallArg {
                label: None,
                value: Expr::Binary {
                    span,
                    op: syx_ast::BinOp::Add,
                    left: Box::new(Expr::Literal {
                        span,
                        value: Literal::String(prefix.into()),
                    }),
                    right: Box::new(Expr::Literal {
                        span,
                        value: Literal::String(method.to_string()),
                    }),
                },
            }],
        },
    }
}

fn is_configuration_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Configuration") || attr_named(attrs, "configuration")
}

fn is_bean_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Bean") || attr_named(attrs, "bean")
}

fn method_has_return_value(return_type: &Option<TypeExpr>) -> bool {
    match return_type {
        None => false,
        Some(TypeExpr::Void) => false,
        Some(TypeExpr::Named { name, .. }) if name.as_str() == "Void" => false,
        Some(_) => true,
    }
}

fn assign_proceed_stmt(span: syx_diagnostics::Span, proceed: &str, value: Expr) -> Stmt {
    Stmt::Expr {
        span,
        expr: Expr::Assign {
            span,
            name: SmolStr::new(proceed),
            value: Box::new(value),
        },
    }
}

fn rewrite_returns_to_proceed(span: syx_diagnostics::Span, proceed: &str, stmts: Vec<Stmt>) -> Vec<Stmt> {
    let mut out = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Return {
                span: ret_span,
                value: Some(value),
            } => out.push(assign_proceed_stmt(ret_span, proceed, value)),
            Stmt::Return { value: None, .. } => {}
            Stmt::If {
                span: if_span,
                condition,
                then_branch,
                else_branch,
            } => {
                out.push(Stmt::If {
                    span: if_span,
                    condition,
                    then_branch: rewrite_returns_to_proceed(span, proceed, then_branch),
                    else_branch: else_branch
                        .map(|b| rewrite_returns_to_proceed(span, proceed, b)),
                });
            }
            Stmt::Block { span: block_span, stmts: inner } => {
                out.push(Stmt::Block {
                    span: block_span,
                    stmts: rewrite_returns_to_proceed(span, proceed, inner),
                });
            }
            Stmt::While {
                span: while_span,
                condition,
                body,
                label,
            } => {
                out.push(Stmt::While {
                    span: while_span,
                    condition,
                    body: rewrite_returns_to_proceed(span, proceed, body),
                    label,
                });
            }
            Stmt::Try {
                span: try_span,
                body,
                catch_name,
                catch_type,
                catch_body,
            } => {
                out.push(Stmt::Try {
                    span: try_span,
                    body: rewrite_returns_to_proceed(span, proceed, body),
                    catch_name,
                    catch_type,
                    catch_body: rewrite_returns_to_proceed(span, proceed, catch_body),
                });
            }
            other => out.push(other),
        }
    }
    out
}

fn wrap_one_around_layer(
    span: syx_diagnostics::Span,
    method: &str,
    layer: usize,
    inner: Vec<Stmt>,
    return_type: &Option<TypeExpr>,
) -> Vec<Stmt> {
    let proceed = format!("__syx_proceed_{layer}");
    let has_return = method_has_return_value(return_type);
    let mut wrapped = vec![advice_print_stmt(span, "__around__:", method)];
    if has_return {
        wrapped.push(Stmt::VarDecl {
            span,
            mutable: true,
            name: SmolStr::new(&proceed),
            ty: None,
            init: None,
        });
        wrapped.push(Stmt::Block {
            span,
            stmts: rewrite_returns_to_proceed(span, &proceed, inner),
        });
        wrapped.push(Stmt::Return {
            span,
            value: Some(ident_expr(span, &proceed)),
        });
    } else {
        wrapped.push(Stmt::Block { span, stmts: inner });
    }
    wrapped
}

fn prepend_after_advice(span: syx_diagnostics::Span, method: &str, stmts: Vec<Stmt>) -> Vec<Stmt> {
    let mut out = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Return { .. } => {
                out.push(advice_print_stmt(span, "__after__:", method));
                out.push(stmt);
            }
            Stmt::If {
                span: if_span,
                condition,
                then_branch,
                else_branch,
            } => {
                out.push(Stmt::If {
                    span: if_span,
                    condition,
                    then_branch: prepend_after_advice(span, method, then_branch),
                    else_branch: else_branch.map(|b| prepend_after_advice(span, method, b)),
                });
            }
            Stmt::Block { span: block_span, stmts: inner } => {
                out.push(Stmt::Block {
                    span: block_span,
                    stmts: prepend_after_advice(span, method, inner),
                });
            }
            other => out.push(other),
        }
    }
    out
}

/// Phase 127 M17 / Phase 130 DI v11: expand `@Around` / `@Before` / `@After` on
/// `@Service`/`@Component` methods. `@Pointcut("Type.method")` registers a method-name
/// pattern; `@Around("Type.method")` binds that pointcut and wraps the matching join point
/// with the existing proceed chain. Bare `@Around` remains declaration-order self-wrap.
/// Phase 132 DI v13: `@Before("…")` / `@After("…")` bind the same pointcut patterns
/// (`macro-service-v13:before-after-pointcut-bind-mvp`); bare Before/After keep Phase 127.
/// Phase 133 DI v14: `@Pointcut("within(Type)")` type-name pattern
/// (`macro-service-v14:pointcut-within-name-mvp`); Around/Before/After string bind hits
/// when the join type matches.
/// Phase 134 DI v15: `@Pointcut("args(Type)")` first-param type-name pattern
/// (`macro-service-v15:pointcut-args-name-mvp`).
pub fn expand_advice_interceptors(module: &mut syx_ast::Module) -> SyxResult<()> {
    reject_advice_on_non_service(&module.decls)?;
    reject_pointcut_on_non_service(&module.decls)?;

    // Collect all `@Pointcut("Type.method")` registrations across service types.
    let mut pointcuts: std::collections::HashSet<SmolStr> = std::collections::HashSet::new();
    for decl in &module.decls {
        let Decl::Class { attrs, members, .. } = decl else {
            continue;
        };
        if !has_service_attr(attrs) {
            continue;
        }
        for member in members {
            if let Decl::Function { attrs: method_attrs, .. } = member {
                for p in pointcut_patterns_from_attrs(method_attrs) {
                    pointcuts.insert(p);
                }
            }
        }
    }

    for decl in &mut module.decls {
        let Decl::Class { attrs, name, members, .. } = decl else {
            continue;
        };
        if !has_service_attr(attrs) {
            continue;
        }
        let type_name = name.clone();
        for member in members.iter_mut() {
            let Decl::Function {
                name: method,
                attrs: method_attrs,
                body,
                span,
                return_type,
                params,
                ..
            } = member
            else {
                continue;
            };

            let around_bindings = around_bindings_from_attrs(method_attrs);
            let before_bindings = before_bindings_from_attrs(method_attrs);
            let after_bindings = after_bindings_from_attrs(method_attrs);
            let join = join_point_name(type_name.as_str(), method.as_str());

            let around_count = validate_advice_bindings(
                "Around",
                &around_bindings,
                &pointcuts,
                &type_name,
                method,
                &join,
                params,
            )?;
            let before_count = validate_advice_bindings(
                "Before",
                &before_bindings,
                &pointcuts,
                &type_name,
                method,
                &join,
                params,
            )?;
            let after_count = validate_advice_bindings(
                "After",
                &after_bindings,
                &pointcuts,
                &type_name,
                method,
                &join,
                params,
            )?;

            if around_count == 0 && before_count == 0 && after_count == 0 {
                // Strip Pointcut-only markers so they don't leak to codegen.
                method_attrs.retain(|a| !matches!(a.name.as_str(), "Pointcut" | "pointcut"));
                continue;
            }

            method_attrs.retain(|a| {
                !matches!(
                    a.name.as_str(),
                    "Around"
                        | "around"
                        | "Before"
                        | "before"
                        | "After"
                        | "after"
                        | "Pointcut"
                        | "pointcut"
                )
            });
            let original_body = std::mem::take(body);
            let mut stmts = original_body;
            if after_count > 0 {
                stmts = prepend_after_advice(*span, method.as_str(), stmts);
            }
            for layer in (0..around_count).rev() {
                stmts = wrap_one_around_layer(*span, method.as_str(), layer, stmts, return_type);
            }
            if before_count > 0 {
                let mut prefixed = vec![advice_print_stmt(*span, "__before__:", method.as_str())];
                prefixed.extend(stmts);
                stmts = prefixed;
            }
            *body = stmts;
        }
    }
    Ok(())
}

fn validate_advice_bindings(
    kind: &str,
    bindings: &[Option<SmolStr>],
    pointcuts: &std::collections::HashSet<SmolStr>,
    type_name: &SmolStr,
    method: &SmolStr,
    join: &SmolStr,
    params: &[syx_ast::Param],
) -> SyxResult<usize> {
    let mut count = 0usize;
    for binding in bindings {
        match binding {
            None => {
                // Bare advice — always applies to this method (Phase 127).
                count += 1;
            }
            Some(pc) => {
                if !pointcuts.contains(pc) {
                    return Err(SyxDiagnostic::error(format!(
                        "@{kind}(\"{pc}\") requires a matching @Pointcut(\"{pc}\") registration"
                    ))
                    .with_code("E016"));
                }
                if !pointcut_matches_method(pc.as_str(), join.as_str(), params) {
                    return Err(SyxDiagnostic::error(format!(
                        "@{kind}(\"{pc}\") on '{type_name}.{method}' does not match join point '{join}' (pointcut within/execution/args-name MVP)"
                    ))
                    .with_code("E016"));
                }
                count += 1;
            }
        }
    }
    Ok(count)
}

/// Back-compat alias for Phase 125 `@Around` MVP callers.
pub fn expand_around_interceptors(module: &mut syx_ast::Module) -> SyxResult<()> {
    expand_advice_interceptors(module)
}

fn reject_advice_on_non_service(decls: &[Decl]) -> SyxResult<()> {
    for decl in decls {
        let members = match decl {
            Decl::Class { attrs, members, .. } if !has_service_attr(attrs) => members.as_slice(),
            _ => continue,
        };
        for m in members {
            if let Decl::Function { attrs, name, .. } = m {
                if is_around_attr(attrs) || is_before_attr(attrs) || is_after_attr(attrs) {
                    return Err(SyxDiagnostic::error(format!(
                        "@Around/@Before/@After on '{name}' requires enclosing @Service/@Component type"
                    ))
                    .with_code("E014"));
                }
            }
        }
    }
    Ok(())
}

fn reject_pointcut_on_non_service(decls: &[Decl]) -> SyxResult<()> {
    for decl in decls {
        let members = match decl {
            Decl::Class { attrs, members, .. } if !has_service_attr(attrs) => members.as_slice(),
            Decl::Function { attrs, name, .. } if is_pointcut_attr(attrs) => {
                return Err(SyxDiagnostic::error(format!(
                    "@Pointcut on '{name}' requires enclosing @Service/@Component type"
                ))
                .with_code("E016"));
            }
            _ => continue,
        };
        for m in members {
            if let Decl::Function { attrs, name, .. } = m {
                if is_pointcut_attr(attrs) {
                    return Err(SyxDiagnostic::error(format!(
                        "@Pointcut on '{name}' requires enclosing @Service/@Component type"
                    ))
                    .with_code("E016"));
                }
            }
        }
    }
    Ok(())
}

/// Collect `@Pointcut("Type.method")` patterns registered on `@Service`/`@Component` methods.
pub fn collect_pointcuts(module: &syx_ast::Module) -> Vec<SmolStr> {
    let mut out = Vec::new();
    for decl in &module.decls {
        let Decl::Class { attrs, members, .. } = decl else {
            continue;
        };
        if !has_service_attr(attrs) {
            continue;
        }
        for member in members {
            if let Decl::Function { attrs: method_attrs, .. } = member {
                for p in pointcut_patterns_from_attrs(method_attrs) {
                    if !out.contains(&p) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
}

fn call_host(span: syx_diagnostics::Span, name: &str, args: Vec<Expr>) -> Stmt {
    Stmt::Expr {
        span,
        expr: Expr::Call {
            span,
            callee: Box::new(Expr::Ident {
                span,
                name: SmolStr::new(name),
            }),
            args: args
                .into_iter()
                .map(|value| syx_ast::CallArg { label: None, value })
                .collect(),
        },
    }
}

pub fn spec_to_meta_entry(s: &ServiceSpec) -> syx_meta::ServiceDiEntry {
    syx_meta::ServiceDiEntry {
        type_name: s.type_name.to_string(),
        kind: s.kind.to_string(),
        deps: s.deps.iter().map(|d| d.to_string()).collect(),
        init_slots: s
            .init_inject
            .iter()
            .map(|inj| Some(inj.dep_type.to_string()))
            .collect(),
        init_qualifiers: s
            .init_inject
            .iter()
            .map(|inj| inj.qualifier.as_ref().map(|q| q.to_string()))
            .collect(),
        field_autowires: s
            .field_autowires
            .iter()
            .map(|f| (f.field.to_string(), f.type_name.to_string()))
            .collect(),
        field_qualifiers: s
            .field_autowires
            .iter()
            .map(|f| f.qualifier.as_ref().map(|q| q.to_string()))
            .collect(),
        post_construct: s.post_construct.as_ref().map(|m| m.to_string()),
        scope: s.scope.to_string(),
        pre_destroy: s.pre_destroy.as_ref().map(|m| m.to_string()),
        module_stem: None,
        qualifier: s.qualifier.as_ref().map(|q| q.to_string()),
        primary: s.primary,
        profile: s.profile.as_ref().map(|p| p.to_string()),
        binding_aliases: s.binding_aliases.iter().map(|a| a.to_string()).collect(),
        protocol_conformances: s.protocol_conformances.iter().map(|a| a.to_string()).collect(),
        lazy: s.lazy,
        bean_config: None,
        bean_method: None,
    }
}

fn bean_to_meta_entry(b: &BeanSpec) -> syx_meta::ServiceDiEntry {
    let mut e = spec_to_meta_entry(&bean_to_service_spec(b));
    e.bean_config = Some(b.config_type.to_string());
    e.bean_method = Some(b.method.to_string());
    e
}

/// Queue `@Service` metadata without boot codegen (multi-package workspace scan).
pub fn queue_services_from_module(module: &syx_ast::Module) -> SyxResult<()> {
    let services = collect_services(module)?;
    if services.is_empty() {
        return Ok(());
    }
    syx_meta::service_di_append(services.iter().map(spec_to_meta_entry).collect());
    Ok(())
}

fn spec_from_meta_entry(e: &syx_meta::ServiceDiEntry) -> ServiceSpec {
    let mut init_inject = Vec::new();
    for (idx, slot) in e.init_slots.iter().enumerate() {
        if let Some(dep) = slot {
            let qualifier = e
                .init_qualifiers
                .get(idx)
                .and_then(|q| q.as_ref().map(|s| SmolStr::new(s)));
            init_inject.push(InitInject {
                param: SmolStr::new(format!("arg{idx}")),
                dep_type: SmolStr::new(dep),
                qualifier,
            });
        }
    }
    ServiceSpec {
        type_name: SmolStr::new(&e.type_name),
        deps: e.deps.iter().map(|d| SmolStr::new(d)).collect(),
        init_inject,
        field_autowires: e
            .field_autowires
            .iter()
            .enumerate()
            .map(|(idx, (f, t))| FieldAutowire {
                field: SmolStr::new(f),
                type_name: SmolStr::new(t),
                qualifier: e
                    .field_qualifiers
                    .get(idx)
                    .and_then(|q| q.as_ref().map(|s| SmolStr::new(s))),
            })
            .collect(),
        post_construct: e.post_construct.as_ref().map(|m| SmolStr::new(m)),
        pre_destroy: e.pre_destroy.as_ref().map(|m| SmolStr::new(m)),
        scope: if e.scope.is_empty() {
            SmolStr::new("singleton")
        } else {
            SmolStr::new(&e.scope)
        },
        kind: match e.kind.as_str() {
            "Component" => "Component",
            "Bean" => "Bean",
            _ => "Service",
        },
        qualifier: e.qualifier.as_ref().map(|q| SmolStr::new(q)),
        primary: e.primary,
        profile: e.profile.as_ref().map(|p| SmolStr::new(p)),
        binding_aliases: e.binding_aliases.iter().map(|a| SmolStr::new(a)).collect(),
        protocol_conformances: e
            .protocol_conformances
            .iter()
            .map(|a| SmolStr::new(a))
            .collect(),
        lazy: e.lazy,
    }
}

fn service_provides(s: &ServiceSpec, key: &str) -> bool {
    s.type_name.as_str() == key
        || s.protocol_conformances.iter().any(|a| a.as_str() == key)
        || s.binding_aliases.iter().any(|a| a.as_str() == key)
        || s.qualifier.as_ref().is_some_and(|q| q.as_str() == key)
}

/// Pick a concrete service for binding key `want` with optional inject-site qualifier.
pub fn pick_binding<'a>(
    services: &'a [ServiceSpec],
    want: &str,
    inject_qualifier: Option<&str>,
) -> SyxResult<&'a ServiceSpec> {
    let candidates: Vec<_> = services
        .iter()
        .filter(|s| service_provides(s, want))
        .collect();
    if candidates.is_empty() {
        // Exact type may still be valid as a registered service name even when not in slice
        // (cross-package); callers check known set separately.
        return Err(SyxDiagnostic::error(format!(
            "no @Service/@Component provides binding '{want}'"
        ))
        .with_code("E008"));
    }
    if let Some(q) = inject_qualifier {
        let matched: Vec<_> = candidates
            .iter()
            .copied()
            .filter(|s| {
                s.qualifier.as_ref().is_some_and(|sq| sq.as_str() == q)
                    || s.type_name.as_str() == q
            })
            .collect();
        return match matched.as_slice() {
            [one] => Ok(*one),
            [] => Err(SyxDiagnostic::error(format!(
                "no @Service/@Component matches @Qualifier(\"{q}\") for binding '{want}'"
            ))
            .with_code("E013")),
            many => Err(SyxDiagnostic::error(format!(
                "ambiguous @Qualifier(\"{q}\") for binding '{want}': {}",
                many.iter()
                    .map(|s| s.type_name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .with_code("E013")),
        };
    }
    if candidates.len() == 1 {
        return Ok(candidates[0]);
    }
    // Exact type_name match wins when present (concrete resolve of the service itself).
    if let Some(exact) = candidates.iter().find(|s| s.type_name.as_str() == want) {
        return Ok(*exact);
    }
    let primaries: Vec<_> = candidates.iter().copied().filter(|s| s.primary).collect();
    match primaries.as_slice() {
        [one] => Ok(*one),
        [] => Err(SyxDiagnostic::error(format!(
            "ambiguous service binding '{want}' among {} without @Primary or @Qualifier: {}",
            candidates.len(),
            candidates
                .iter()
                .map(|s| s.type_name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ))
        .with_code("E013")),
        many => Err(SyxDiagnostic::error(format!(
            "ambiguous @Primary services for binding '{want}': {}",
            many.iter()
                .map(|s| s.type_name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ))
        .with_code("E013")),
    }
}

fn rewrite_inject_bindings(services: &mut [ServiceSpec]) -> SyxResult<()> {
    // Include import-scanned meta so cross-package bindings participate.
    let snapshot = boot_construction_specs(services);
    for s in services.iter_mut() {
        for inj in &mut s.init_inject {
            let concrete = pick_binding(
                &snapshot,
                inj.dep_type.as_str(),
                inj.qualifier.as_ref().map(|q| q.as_str()),
            )?;
            inj.dep_type = concrete.type_name.clone();
        }
        for field in &mut s.field_autowires {
            let concrete = pick_binding(
                &snapshot,
                field.type_name.as_str(),
                field.qualifier.as_ref().map(|q| q.as_str()),
            )?;
            field.type_name = concrete.type_name.clone();
        }
        s.deps = merge_deps(
            &s.init_inject
                .iter()
                .map(|i| i.dep_type.clone())
                .collect::<Vec<_>>(),
            &s.field_autowires,
        );
    }
    Ok(())
}

/// Phase 129: rewrite `@Bean` params to concrete `@Service`/`@Bean` types (qualifier / primary).
fn rewrite_bean_params(beans: &mut [BeanSpec], services: &[ServiceSpec]) -> SyxResult<()> {
    let mut snapshot = boot_construction_specs(services);
    for b in beans.iter() {
        // Bean products are valid inject targets for later beans.
        snapshot.push(bean_to_service_spec(b));
    }
    for b in beans.iter_mut() {
        for inj in &mut b.params {
            let concrete = pick_binding(
                &snapshot,
                inj.dep_type.as_str(),
                inj.qualifier.as_ref().map(|q| q.as_str()),
            )?;
            inj.dep_type = concrete.type_name.clone();
        }
    }
    Ok(())
}

fn boot_construction_specs(local: &[ServiceSpec]) -> Vec<ServiceSpec> {
    let mut out: IndexMap<SmolStr, ServiceSpec> = IndexMap::new();
    for e in syx_meta::service_di_peek_entries() {
        out.insert(SmolStr::new(&e.type_name), spec_from_meta_entry(&e));
    }
    for s in local {
        out.insert(s.type_name.clone(), s.clone());
    }
    out.into_values().collect()
}

/// Phase 123: user-facing `resolve(typeName)` wraps internal `__syx_service_resolve` (ceremony sugar).
/// Prefer `import Di` + `Di.resolve` / `Di.beginRequest` for application code; free `resolve` stays as short sugar.
fn inject_resolve_sugar(module: &mut syx_ast::Module) {
    let span = syx_diagnostics::Span::DUMMY;
    module.decls.retain(|d| {
        !matches!(d, Decl::Function { name, .. } if name.as_str() == "resolve")
    });
    module.decls.push(Decl::Function {
        span,
        access: AccessLevel::Public,
        attrs: vec![],
        name: SmolStr::new("resolve"),
        type_params: vec![],
        params: vec![Param {
            name: SmolStr::new("typeName"),
            external_label: ParamLabel::Same,
            ty: Some(TypeExpr::Named {
                name: SmolStr::new("String"),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }],
        return_type: Some(TypeExpr::Named {
            name: SmolStr::new("Object"),
            generics: vec![],
        }),
        throws_type: None,
        effect: None,
        where_clause: vec![],
        body: vec![Stmt::Return {
            span,
            value: Some(Expr::Call {
                span,
                callee: Box::new(Expr::Ident {
                    span,
                    name: SmolStr::new("__syx_service_resolve_dynamic"),
                }),
                args: vec![syx_ast::CallArg {
                    label: None,
                    value: Expr::Ident {
                        span,
                        name: SmolStr::new("typeName"),
                    },
                }],
            }),
        }],
        is_async: false,
        is_override: false,
        is_final: false,
    });
}

fn di_temp_name(type_name: &str) -> SmolStr {
    SmolStr::new(format!("__syx_di_{type_name}"))
}

fn ident_expr(span: syx_diagnostics::Span, name: &str) -> Expr {
    Expr::Ident {
        span,
        name: SmolStr::new(name),
    }
}

fn topo_sort_services(services: &[ServiceSpec]) -> SyxResult<Vec<usize>> {
    let mut order = Vec::new();
    let mut seen = IndexMap::new();
    let mut stack = Vec::new();
    let lazy_names: std::collections::HashSet<&str> = services
        .iter()
        .filter(|s| s.lazy)
        .map(|s| s.type_name.as_str())
        .collect();
    let graph: IndexMap<SmolStr, Vec<SmolStr>> = services
        .iter()
        .map(|s| {
            // Phase 128: skip edges into `@Lazy` nodes so eager boot topo can proceed;
            // lazy deps are satisfied via runtime proxies.
            let deps = s
                .deps
                .iter()
                .filter(|d| !lazy_names.contains(d.as_str()))
                .cloned()
                .collect();
            (s.type_name.clone(), deps)
        })
        .collect();
    fn visit(
        node: &SmolStr,
        services: &[ServiceSpec],
        graph: &IndexMap<SmolStr, Vec<SmolStr>>,
        stack: &mut Vec<SmolStr>,
        seen: &mut IndexMap<SmolStr, bool>,
        order: &mut Vec<usize>,
    ) -> SyxResult<()> {
        if stack.contains(node) {
            return Err(SyxDiagnostic::error(format!(
                "service dependency cycle involving '{node}'"
            ))
            .with_code("E007"));
        }
        if seen.get(node).copied().unwrap_or(false) {
            return Ok(());
        }
        stack.push(node.clone());
        if let Some(deps) = graph.get(node) {
            for d in deps {
                visit(d, services, graph, stack, seen, order)?;
            }
        }
        stack.pop();
        seen.insert(node.clone(), true);
        if let Some(idx) = services.iter().position(|s| s.type_name == *node) {
            order.push(idx);
        }
        Ok(())
    }
    for s in services {
        // Lazy / prototype / request are not eagerly constructed at boot.
        if s.lazy || matches!(s.scope.as_str(), "prototype" | "request") {
            continue;
        }
        visit(
            &s.type_name,
            services,
            &graph,
            &mut stack,
            &mut seen,
            &mut order,
        )?;
    }
    Ok(order)
}

fn codegen_real_construction_stmts(
    services: &[ServiceSpec],
    span: syx_diagnostics::Span,
) -> SyxResult<Vec<Stmt>> {
    let mut out = Vec::new();
    let lazy_names: std::collections::HashSet<&str> = services
        .iter()
        .filter(|s| s.lazy)
        .map(|s| s.type_name.as_str())
        .collect();
    for idx in topo_sort_services(services)? {
        let s = &services[idx];
        if s.kind == "Bean" || s.lazy {
            continue;
        }
        let temp = di_temp_name(s.type_name.as_str());
        let mut ctor_args = Vec::new();
        for inj in &s.init_inject {
            let dep_expr = if lazy_names.contains(inj.dep_type.as_str()) {
                // Phase 128: inject lazy proxy handle for `@Lazy` ctor deps.
                Expr::Call {
                    span,
                    callee: Box::new(Expr::Ident {
                        span,
                        name: SmolStr::new("__syx_service_lazy_proxy"),
                    }),
                    args: vec![syx_ast::CallArg {
                        label: None,
                        value: string_lit(span, inj.dep_type.as_str()),
                    }],
                }
            } else {
                ident_expr(span, di_temp_name(inj.dep_type.as_str()).as_str())
            };
            ctor_args.push(syx_ast::CallArg {
                label: Some(inj.param.clone()),
                value: dep_expr,
            });
        }
        out.push(Stmt::VarDecl {
            span,
            mutable: false,
            name: temp.clone(),
            ty: Some(TypeExpr::Named {
                name: s.type_name.clone(),
                generics: vec![],
            }),
            init: Some(Expr::Call {
                span,
                callee: Box::new(ident_expr(span, s.type_name.as_str())),
                args: ctor_args,
            }),
        });
        for field in &s.field_autowires {
            let dep_expr = if lazy_names.contains(field.type_name.as_str()) {
                Expr::Call {
                    span,
                    callee: Box::new(Expr::Ident {
                        span,
                        name: SmolStr::new("__syx_service_lazy_proxy"),
                    }),
                    args: vec![syx_ast::CallArg {
                        label: None,
                        value: string_lit(span, field.type_name.as_str()),
                    }],
                }
            } else {
                ident_expr(span, di_temp_name(field.type_name.as_str()).as_str())
            };
            out.push(Stmt::Expr {
                span,
                expr: Expr::MemberAssign {
                    span,
                    object: Box::new(ident_expr(span, temp.as_str())),
                    name: field.field.clone(),
                    value: Box::new(dep_expr),
                },
            });
        }
        if let Some(method) = &s.post_construct {
            out.push(Stmt::Expr {
                span,
                expr: Expr::Call {
                    span,
                    callee: Box::new(Expr::Member {
                        span,
                        object: Box::new(ident_expr(span, temp.as_str())),
                        name: method.clone(),
                    }),
                    args: vec![],
                },
            });
        }
        if !matches!(s.scope.as_str(), "prototype" | "request") {
            out.push(call_host(
                span,
                "__syx_service_put",
                vec![string_lit(span, s.type_name.as_str()), ident_expr(span, temp.as_str())],
            ));
        }
    }
    Ok(out)
}

fn service_boot_impl_name() -> SmolStr {
    SmolStr::new("__syx_service_boot_impl")
}

fn inject_boot_call_into_main(module: &mut syx_ast::Module) {
    let span = syx_diagnostics::Span::DUMMY;
    let boot_impl = service_boot_impl_name();
    let call = Stmt::Expr {
        span,
        expr: Expr::Call {
            span,
            callee: Box::new(Expr::Ident {
                span,
                name: boot_impl.clone(),
            }),
            args: vec![],
        },
    };
    for decl in &mut module.decls {
        if let Decl::Function { name, body, .. } = decl {
            if name.as_str() == "main" {
                let already = body.iter().any(|s| {
                    matches!(
                        s,
                        Stmt::Expr {
                            expr: Expr::Call { callee, .. },
                            ..
                        } if match callee.as_ref() {
                            Expr::Ident { name, .. } if name.as_str() == boot_impl.as_str() => true,
                            Expr::Ident { name, .. } if name.as_str() == "__syx_service_boot" => {
                                true
                            }
                            _ => false,
                        }
                    )
                });
                if !already {
                    // After explicit reset / setProfile when present so boot sees the active profile.
                    let insert_at = body
                        .iter()
                        .enumerate()
                        .filter_map(|(i, s)| {
                            let is_prelude = matches!(
                                s,
                                Stmt::Expr {
                                    expr: Expr::Call { callee, .. },
                                    ..
                                } if match callee.as_ref() {
                                    Expr::Ident { name, .. } => matches!(
                                        name.as_str(),
                                        "__syx_service_reset"
                                            | "__syx_service_set_profile"
                                    ),
                                    Expr::Member { object, name, .. }
                                        if matches!(name.as_str(), "reset" | "setProfile")
                                            && matches!(
                                                object.as_ref(),
                                                Expr::Ident { name: obj, .. }
                                                    if obj.as_str() == "Di"
                                            ) =>
                                    {
                                        true
                                    }
                                    _ => false,
                                }
                            );
                            is_prelude.then_some(i)
                        })
                        .max()
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    body.insert(insert_at, call);
                }
                return;
            }
        }
    }
}

/// Detect cycles in service dep graph → E007.
/// Phase 128: cycles that include at least one `@Lazy` node are allowed (lazy proxy break).
pub fn check_service_cycles(services: &[ServiceSpec]) -> SyxResult<()> {
    let mut graph: IndexMap<SmolStr, Vec<SmolStr>> = IndexMap::new();
    let mut lazy_set: IndexMap<SmolStr, bool> = IndexMap::new();
    for s in services {
        graph.insert(s.type_name.clone(), s.deps.clone());
        lazy_set.insert(s.type_name.clone(), s.lazy);
    }
    fn cycle_has_lazy(
        stack: &[SmolStr],
        back_to: &SmolStr,
        lazy_set: &IndexMap<SmolStr, bool>,
    ) -> bool {
        let Some(start) = stack.iter().position(|s| s == back_to) else {
            return lazy_set.get(back_to).copied().unwrap_or(false);
        };
        stack[start..]
            .iter()
            .any(|n| lazy_set.get(n).copied().unwrap_or(false))
            || lazy_set.get(back_to).copied().unwrap_or(false)
    }
    fn visit(
        node: &SmolStr,
        graph: &IndexMap<SmolStr, Vec<SmolStr>>,
        lazy_set: &IndexMap<SmolStr, bool>,
        stack: &mut Vec<SmolStr>,
        seen: &mut IndexMap<SmolStr, bool>,
    ) -> SyxResult<()> {
        if stack.contains(node) {
            if cycle_has_lazy(stack, node, lazy_set) {
                return Ok(());
            }
            return Err(SyxDiagnostic::error(format!(
                "service dependency cycle involving '{node}'"
            ))
            .with_code("E007"));
        }
        if seen.get(node).copied().unwrap_or(false) {
            return Ok(());
        }
        stack.push(node.clone());
        if let Some(deps) = graph.get(node) {
            for d in deps {
                if stack.contains(d) {
                    if cycle_has_lazy(stack, d, lazy_set) {
                        continue;
                    }
                    return Err(SyxDiagnostic::error(format!(
                        "service dependency cycle involving '{d}'"
                    ))
                    .with_code("E007"));
                }
                visit(d, graph, lazy_set, stack, seen)?;
            }
        }
        stack.pop();
        seen.insert(node.clone(), true);
        Ok(())
    }
    let mut seen = IndexMap::new();
    let mut stack = Vec::new();
    for s in services {
        visit(&s.type_name, &graph, &lazy_set, &mut stack, &mut seen)?;
    }
    Ok(())
}

pub fn expand_service_macros(module: &mut syx_ast::Module) -> SyxResult<()> {
    expand_advice_interceptors(module)?;
    let mut services = collect_services(module)?;
    let mut beans = collect_beans(module)?;
    rewrite_inject_bindings(&mut services)?;
    rewrite_bean_params(&mut beans, &services)?;
    check_service_cycles(&services)?;
    let mut known: std::collections::HashSet<SmolStr> = services
        .iter()
        .map(|s| s.type_name.clone())
        .collect();
    for b in &beans {
        known.insert(b.return_type.clone());
    }
    for name in syx_meta::service_di_known_type_names() {
        known.insert(SmolStr::new(name));
    }
    // Binding aliases (superclass / protocol / qualifier) are valid inject targets.
    for s in &services {
        for a in &s.binding_aliases {
            known.insert(a.clone());
        }
        if let Some(q) = &s.qualifier {
            known.insert(q.clone());
        }
    }
    for s in &services {
        for dep in &s.deps {
            if !known.contains(dep) {
                return Err(SyxDiagnostic::error(format!(
                    "service '{}' requires injected dependency '{}' which is not a @Service/@Component",
                    s.type_name, dep
                ))
                .with_code("E008"));
            }
        }
    }
    for b in &beans {
        for inj in &b.params {
            if !known.contains(&inj.dep_type) {
                return Err(SyxDiagnostic::error(format!(
                    "@Bean '{}.{}' requires injected dependency '{}' which is not a @Service/@Component/@Bean",
                    b.config_type, b.method, inj.dep_type
                ))
                .with_code("E008"));
            }
        }
    }
    // Re-queue with rewritten concrete deps so runtime metadata matches codegen.
    if !services.is_empty() {
        // Drop previous local entries of the same types then append rewritten.
        syx_meta::service_di_append(services.iter().map(spec_to_meta_entry).collect());
    }
    if !beans.is_empty() {
        syx_meta::service_di_append(beans.iter().map(bean_to_meta_entry).collect());
    }
    codegen_service_registry_from_specs(module, &services, &beans)?;
    desugar_service_resolve_idents(module, &services)?;
    Ok(())
}

/// Emit `__syx_service_boot()` peer + queue runtime registrations for the boot host.
pub fn codegen_service_registry(module: &mut syx_ast::Module) -> SyxResult<()> {
    let mut services = collect_services(module)?;
    let mut beans = collect_beans(module)?;
    rewrite_inject_bindings(&mut services)?;
    rewrite_bean_params(&mut beans, &services)?;
    codegen_service_registry_from_specs(module, &services, &beans)
}

fn codegen_service_registry_from_specs(
    module: &mut syx_ast::Module,
    services: &[ServiceSpec],
    beans: &[BeanSpec],
) -> SyxResult<()> {
    if services.is_empty() && beans.is_empty() {
        return Ok(());
    }

    // Runtime host `__syx_service_boot` consumes merged queue (multi-package scan appends).
    if !services.is_empty() {
        syx_meta::service_di_append(services.iter().map(spec_to_meta_entry).collect());
    }
    if !beans.is_empty() {
        syx_meta::service_di_append(beans.iter().map(bean_to_meta_entry).collect());
    }

    let has_main = module.decls.iter().any(|d| {
        matches!(d, Decl::Function { name, .. } if name.as_str() == "main")
    });
    if !has_main {
        return Ok(());
    }

    let mut boot_services = boot_construction_specs(services);
    for b in beans {
        let spec = bean_to_service_spec(b);
        if !boot_services.iter().any(|s| s.type_name == spec.type_name) {
            boot_services.push(spec);
        }
    }

    let span = syx_diagnostics::Span::DUMMY;
    let mut body = Vec::new();
    for s in &boot_services {
        body.push(call_host(
            span,
            "__syx_service_register",
            vec![
                string_lit(span, s.type_name.as_str()),
                string_lit(span, s.kind),
            ],
        ));
        for protocol in &s.protocol_conformances {
            body.push(call_host(
                span,
                "__syx_service_register_witness",
                vec![
                    string_lit(span, protocol.as_str()),
                    string_lit(span, s.type_name.as_str()),
                    string_lit(
                        span,
                        s.qualifier.as_ref().map(|q| q.as_str()).unwrap_or(""),
                    ),
                    string_lit(span, if s.primary { "1" } else { "" }),
                ],
            ));
        }
        for dep in &s.deps {
            body.push(call_host(
                span,
                "__syx_service_add_dep",
                vec![
                    string_lit(span, s.type_name.as_str()),
                    string_lit(span, dep.as_str()),
                ],
            ));
        }
    }
    body.extend(codegen_real_construction_stmts(&boot_services, span)?);
    body.extend(codegen_bean_factory_stmts(beans, span)?);
    body.push(call_host(span, "__syx_service_seal", vec![]));

    module.decls.retain(|d| {
        !matches!(d, Decl::Function { name, .. } if name.as_str() == service_boot_impl_name().as_str())
    });

    module.decls.push(Decl::Function {
        span,
        access: AccessLevel::Public,
        attrs: vec![],
        name: service_boot_impl_name(),
        type_params: vec![],
        params: vec![],
        return_type: Some(TypeExpr::Void),
        throws_type: None,
        effect: None,
        where_clause: vec![],
        body,
        is_async: false,
        is_override: false,
        is_final: false,
    });

    inject_boot_call_into_main(module);
    inject_resolve_sugar(module);
    Ok(())
}

fn desugar_service_resolve_idents(
    module: &mut syx_ast::Module,
    services: &[ServiceSpec],
) -> SyxResult<()> {
    use syx_ast::{Expr, Stmt};
    let mut known: std::collections::HashSet<SmolStr> = services
        .iter()
        .map(|s| s.type_name.clone())
        .chain(
            syx_meta::service_di_known_type_names()
                .into_iter()
                .map(SmolStr::new),
        )
        .collect();
    for s in services {
        for a in &s.binding_aliases {
            known.insert(a.clone());
        }
        if let Some(q) = &s.qualifier {
            known.insert(q.clone());
        }
    }

    fn rewrite_expr(expr: &mut Expr, known: &std::collections::HashSet<SmolStr>) {
        match expr {
            Expr::Call { callee, args, .. } => {
                let is_service_resolve = match callee.as_ref() {
                    Expr::Ident { name, .. } => matches!(
                        name.as_str(),
                        "__syx_service_resolve" | "resolve"
                    ),
                    Expr::Member { object, name, .. }
                        if name.as_str() == "resolve"
                            && matches!(
                                object.as_ref(),
                                Expr::Ident { name: obj, .. } if obj.as_str() == "Di"
                            ) =>
                    {
                        true
                    }
                    _ => false,
                };
                rewrite_expr(callee, known);
                for a in args.iter_mut() {
                    rewrite_expr(&mut a.value, known);
                }
                if is_service_resolve {
                    if let Some(arg) = args.first_mut() {
                        if let Expr::Ident { name, span } = &arg.value {
                            if known.contains(name) {
                                arg.value = Expr::Literal {
                                    span: *span,
                                    value: Literal::String(name.to_string()),
                                };
                            }
                        }
                    }
                }
            }
            Expr::Cast { expr, .. }
            | Expr::OptionalCast { expr, .. }
            | Expr::Await { expr, .. }
            | Expr::Borrow { expr, .. }
            | Expr::Unary { expr, .. }
            | Expr::Member { object: expr, .. }
            | Expr::Group { expr, .. } => rewrite_expr(expr, known),
            Expr::Binary { left, right, .. } => {
                rewrite_expr(left, known);
                rewrite_expr(right, known);
            }
            Expr::Array { elements, .. } => {
                for e in elements {
                    rewrite_expr(e, known);
                }
            }
            _ => {}
        }
    }

    fn rewrite_stmts(stmts: &mut [Stmt], known: &std::collections::HashSet<SmolStr>) {
        for stmt in stmts {
            match stmt {
                Stmt::Expr { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
                    rewrite_expr(expr, known);
                }
                Stmt::VarDecl { init: Some(expr), .. } => rewrite_expr(expr, known),
                Stmt::If {
                    then_branch,
                    else_branch,
                    ..
                } => {
                    rewrite_stmts(then_branch, known);
                    if let Some(b) = else_branch {
                        rewrite_stmts(b, known);
                    }
                }
                Stmt::Block { stmts, .. } => rewrite_stmts(stmts, known),
                _ => {}
            }
        }
    }

    fn rewrite_decls(decls: &mut [Decl], known: &std::collections::HashSet<SmolStr>) {
        for decl in decls {
            match decl {
                Decl::Function { body, .. } => rewrite_stmts(body, known),
                Decl::Class { members, .. } => rewrite_decls(members, known),
                _ => {}
            }
        }
    }

    rewrite_decls(&mut module.decls, &known);
    Ok(())
}

pub fn has_service_attr(attrs: &[Attribute]) -> bool {
    attr_named(attrs, "Service")
        || attr_named(attrs, "Component")
        || attr_named(attrs, "service")
        || attr_named(attrs, "component")
}
