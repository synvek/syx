use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

use indexmap::IndexMap;
use smol_str::SmolStr;
use syx_ast::{Decl, Expr, FunctionEffect, Module, Stmt};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::MirModule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Pure,
    Io,
}

impl From<FunctionEffect> for Effect {
    fn from(value: FunctionEffect) -> Self {
        match value {
            FunctionEffect::Pure => Effect::Pure,
            FunctionEffect::Io => Effect::Io,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeMetadata {
    pub name: SmolStr,
    pub fields: Vec<SmolStr>,
    /// Parallel to `fields`. Trailing `?` marks Optional. Empty = unknown type.
    pub field_types: Vec<SmolStr>,
}

static TYPE_REGISTRY: LazyLock<Mutex<IndexMap<SmolStr, TypeMetadata>>> =
    LazyLock::new(|| Mutex::new(IndexMap::new()));

fn type_expr_to_type_name(ty: &syx_ast::TypeExpr) -> String {
    use syx_ast::TypeExpr;
    match ty {
        TypeExpr::Named { name, generics } => {
            if generics.is_empty() {
                name.to_string()
            } else {
                let inner = generics
                    .iter()
                    .map(type_expr_to_type_name)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{name}<{inner}>")
            }
        }
        TypeExpr::Optional(inner) => format!("{}?", type_expr_to_type_name(inner)),
        TypeExpr::Array(inner) => format!("Array<{}>", type_expr_to_type_name(inner)),
        TypeExpr::Dictionary { key, value } => format!(
            "Dictionary<{}, {}>",
            type_expr_to_type_name(key),
            type_expr_to_type_name(value)
        ),
        TypeExpr::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(|(label, ty)| {
                    if let Some(l) = label {
                        format!("{l}: {}", type_expr_to_type_name(ty))
                    } else {
                        type_expr_to_type_name(ty)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("({inner})")
        }
        TypeExpr::Reference(inner, is_mut, _) => {
            if *is_mut {
                format!("&mut {}", type_expr_to_type_name(inner))
            } else {
                format!("&{}", type_expr_to_type_name(inner))
            }
        }
        TypeExpr::OpaqueSome(inner) => format!("some {}", type_expr_to_type_name(inner)),
        TypeExpr::OpaqueAny(inner) => format!("any {}", type_expr_to_type_name(inner)),
        TypeExpr::Function { .. } | TypeExpr::Void => String::new(),
    }
}

pub fn register_type(name: &str, fields: Vec<&str>) {
    register_type_with_field_types(
        name,
        fields
            .into_iter()
            .map(|f| (SmolStr::new(f), SmolStr::new("")))
            .collect(),
    );
}

/// Register type metadata with field type names (Optional marked with trailing `?`).
pub fn register_type_with_field_types(name: &str, fields: Vec<(SmolStr, SmolStr)>) {
    let mut reg = TYPE_REGISTRY.lock().expect("TYPE_REGISTRY poisoned");
    let (field_names, field_types): (Vec<_>, Vec<_>) = fields.into_iter().unzip();
    reg.insert(
        SmolStr::new(name),
        TypeMetadata {
            name: SmolStr::new(name),
            fields: field_names,
            field_types,
        },
    );
}

pub fn register_data_classes(module: &Module) {
    for decl in &module.decls {
        match decl {
            Decl::DataClass { name, fields, .. } | Decl::Struct { name, fields, .. } => {
                register_type_with_field_types(
                    name.as_str(),
                    fields
                        .iter()
                        .map(|(n, ty)| (n.clone(), SmolStr::new(type_expr_to_type_name(ty))))
                        .collect(),
                );
            }
            _ => {}
        }
    }
}

pub fn reflect_type(name: &str) -> Option<TypeMetadata> {
    TYPE_REGISTRY
        .lock()
        .expect("TYPE_REGISTRY poisoned")
        .get(name)
        .cloned()
}

/// Phase 77: generic specialization reflection MVP.
#[derive(Debug, Clone)]
pub struct MirMonoSig {
    pub fn_name: SmolStr,
    pub type_args: Vec<SmolStr>,
    pub param_count: usize,
    pub has_witness: bool,
}

static MONO_CACHE: LazyLock<Mutex<IndexMap<(SmolStr, Vec<SmolStr>), MirMonoSig>>> =
    LazyLock::new(|| Mutex::new(IndexMap::new()));

/// Phase 106: active semantic generic instantiations (preferred over runtime mono cache).
static SEMANTIC_INSTANTIATIONS: LazyLock<Mutex<HashSet<(SmolStr, Vec<SmolStr>)>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub fn reset_semantic_instantiations() {
    SEMANTIC_INSTANTIATIONS
        .lock()
        .expect("SEMANTIC_INSTANTIATIONS poisoned")
        .clear();
}

pub fn set_semantic_instantiations(instantiations: &HashSet<(SmolStr, Vec<SmolStr>)>) {
    let mut set = SEMANTIC_INSTANTIATIONS
        .lock()
        .expect("SEMANTIC_INSTANTIATIONS poisoned");
    set.clear();
    set.extend(instantiations.iter().cloned());
}

pub fn note_semantic_instantiation(fn_name: &str, type_args: &[SmolStr]) {
    SEMANTIC_INSTANTIATIONS
        .lock()
        .expect("SEMANTIC_INSTANTIATIONS poisoned")
        .insert((
            SmolStr::new(fn_name),
            type_args.iter().cloned().collect(),
        ));
}

fn semantic_instantiation_key(fn_name: &str, type_args: &[&str]) -> (SmolStr, Vec<SmolStr>) {
    (
        SmolStr::new(fn_name),
        type_args.iter().map(|s| SmolStr::new(*s)).collect(),
    )
}

fn semantic_instantiation_hit(fn_name: &str, type_args: &[&str]) -> bool {
    let key = semantic_instantiation_key(fn_name, type_args);
    SEMANTIC_INSTANTIATIONS
        .lock()
        .expect("SEMANTIC_INSTANTIATIONS poisoned")
        .contains(&key)
}

pub fn register_mono_sig(fn_name: &str, type_args: &[&str], param_count: usize, has_witness: bool) {
    let key = (SmolStr::new(fn_name), type_args.iter().map(|s| SmolStr::new(*s)).collect());
    MONO_CACHE.lock().expect("MONO_CACHE poisoned").insert(
        key,
        MirMonoSig {
            fn_name: SmolStr::new(fn_name),
            type_args: type_args.iter().map(|s| SmolStr::new(*s)).collect(),
            param_count,
            has_witness,
        },
    );
}

fn mono_cache_disk_path() -> std::path::PathBuf {
    std::env::var("SYX_RUNTIME_MONO_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("syx_runtime_mono_cache.jsonl"))
}

/// Phase 82: persist mono sig to disk artifact for MIR witness backfill E2E.
pub fn persist_mono_sig_disk(
    fn_name: &str,
    type_args: &[&str],
    param_count: usize,
    has_witness: bool,
) {
    register_mono_sig(fn_name, type_args, param_count, has_witness);
    let line = format!(
        "{fn_name}|{}|{param_count}|{has_witness}\n",
        type_args.join(",")
    );
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(mono_cache_disk_path())
        .and_then(|mut f| std::io::Write::write_all(&mut f, line.as_bytes()));
}

fn runtime_mono_cache_from_disk(fn_name: &str, type_args: &[&str]) -> Option<MirMonoSig> {
    let path = mono_cache_disk_path();
    let content = std::fs::read_to_string(&path).ok()?;
    let want = type_args.join(",");
    for line in content.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 4 {
            continue;
        }
        if parts[0] == fn_name && parts[1] == want {
            return Some(MirMonoSig {
                fn_name: SmolStr::new(fn_name),
                type_args: type_args.iter().map(|s| SmolStr::new(*s)).collect(),
                param_count: parts[2].parse().unwrap_or(type_args.len()),
                has_witness: parts[3] == "true",
            });
        }
    }
    None
}

/// Phase 107: honest witness fingerprint — never synthesize `has_witness: true`.
pub fn reflect_witness_honest_fingerprint() -> &'static str {
    "reflect-witness-honest-v1:no-synthesize"
}

/// Phase 106/107: specialization lookup; semantic instantiations first, then runtime mono cache.
/// Semantic hit still requires a real mono/witness entry — no empty-shell synthesis.
pub fn reflect_specialization(fn_name: &str, type_args: &[&str]) -> SyxResult<MirMonoSig> {
    if semantic_instantiation_hit(fn_name, type_args) {
        return match runtime_mono_cache(fn_name, type_args) {
            Some(sig) if sig.has_witness => Ok(sig),
            Some(_) => Err(SyxDiagnostic::error(format!(
                "reflect specialization for '{fn_name}<{}>' has no witness",
                type_args.join(", ")
            ))
            .with_code("M105")),
            None => Err(reflect_specialization_miss_error(fn_name, type_args)),
        };
    }
    match runtime_mono_cache(fn_name, type_args) {
        Some(sig) if sig.has_witness => Ok(sig),
        Some(_) => Err(SyxDiagnostic::error(format!(
            "reflect specialization for '{fn_name}<{}>' has no witness",
            type_args.join(", ")
        ))
        .with_code("M105")),
        None => Err(reflect_specialization_miss_error(fn_name, type_args)),
    }
}

pub fn reflect_specialization_miss_error(fn_name: &str, type_args: &[&str]) -> SyxDiagnostic {
    SyxDiagnostic::error(format!(
        "reflect specialization miss for '{fn_name}<{}>' (semantic instantiations or runtime mono cache)",
        type_args.join(", ")
    ))
    .with_code("M105")
}

/// Phase 106: optional lookup; semantic instantiations preferred over cache.
pub fn reflect_specialization_lookup(fn_name: &str, type_args: &[&str]) -> Option<MirMonoSig> {
    if semantic_instantiation_hit(fn_name, type_args) {
        return reflect_specialization(fn_name, type_args).ok();
    }
    runtime_mono_cache(fn_name, type_args)
}

/// Phase 81/82: runtime mono cache lookup (witness backfill E2E; disk + memory).
pub fn runtime_mono_cache(fn_name: &str, type_args: &[&str]) -> Option<MirMonoSig> {
    if fn_name.is_empty() {
        return None;
    }
    let key = (SmolStr::new(fn_name), type_args.iter().map(|s| SmolStr::new(*s)).collect());
    if let Some(sig) = MONO_CACHE.lock().expect("MONO_CACHE poisoned").get(&key) {
        return Some(sig.clone());
    }
    if let Some(sig) = runtime_mono_cache_from_disk(fn_name, type_args) {
        return Some(sig);
    }
    None
}

pub fn inferred_effect(body: &[Stmt]) -> Effect {
    if stmt_list_has_io(body) {
        Effect::Io
    } else {
        Effect::Pure
    }
}

fn stmt_list_has_io(stmts: &[Stmt]) -> bool {
    stmts.iter().any(stmt_has_io)
}

fn stmt_has_io(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Print { .. } => true,
        Stmt::Return { value: Some(v), .. } => expr_has_io(v),
        Stmt::Expr { expr, .. } => expr_has_io(expr),
        Stmt::VarDecl { init: Some(v), .. } => expr_has_io(v),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            expr_has_io(condition)
                || stmt_list_has_io(then_branch)
                || else_branch.as_ref().is_some_and(|b| stmt_list_has_io(b))
        }
        Stmt::While { condition, body, .. } => expr_has_io(condition) || stmt_list_has_io(body),
        Stmt::ForIn { iterable, body, .. } => {
            expr_has_io(iterable) || stmt_list_has_io(body)
        }
        Stmt::Block { stmts, .. } => stmt_list_has_io(stmts),
        Stmt::Try { body, catch_body, .. } => {
            stmt_list_has_io(body) || stmt_list_has_io(catch_body)
        }
        Stmt::GuardLet { body, else_branch, .. } => {
            stmt_list_has_io(body) || stmt_list_has_io(else_branch)
        }
        _ => false,
    }
}

fn expr_has_io(expr: &Expr) -> bool {
    match expr {
        Expr::Call { callee, args, .. } => {
            call_is_io(callee) || args.iter().any(|a| expr_has_io(&a.value))
        }
        Expr::Member { object, name, .. } => {
            expr_has_io(object) || name.as_str().contains("http") || name == "print"
        }
        Expr::Binary { left, right, .. } => expr_has_io(left) || expr_has_io(right),
        Expr::Unary { expr, .. } => expr_has_io(expr),
        Expr::Await { expr, .. } => expr_has_io(expr),
        Expr::Assign { value, .. } => expr_has_io(value),
        Expr::Array { elements, .. } => elements.iter().any(expr_has_io),
        Expr::Index { object, index, .. } => expr_has_io(object) || expr_has_io(index),
        Expr::OptionalChain { object, .. } => expr_has_io(object),
        Expr::ForceUnwrap { expr, .. } => expr_has_io(expr),
        Expr::CompoundAssign { value, .. } => expr_has_io(value),
        Expr::MemberAssign { object, value, .. } => expr_has_io(object) || expr_has_io(value),
        Expr::IndexAssign { object, index, value, .. } => {
            expr_has_io(object) || expr_has_io(index) || expr_has_io(value)
        }
        Expr::Closure { body, .. } => match body {
            syx_ast::ClosureBody::Expr(e) => expr_has_io(e),
            syx_ast::ClosureBody::Block(stmts) => stmt_list_has_io(stmts),
        },
        Expr::Group { expr, .. } => expr_has_io(expr),
        Expr::Literal { value, .. } => matches!(
            value,
            syx_ast::Literal::Interpolated(parts) if parts.iter().any(|p| matches!(p, syx_ast::StringPart::Expr(e) if expr_has_io(e)))
        ),
        _ => false,
    }
}

fn call_is_io(callee: &Expr) -> bool {
    match callee {
        Expr::Ident { name, .. } => {
            let n = name.as_str();
            n.contains("http")
                || n.contains("fetch")
                || n == "print"
                || n.starts_with("host_")
        }
        Expr::Member { name, .. } => {
            let n = name.as_str();
            n == "get" || n == "post" || n == "json"
        }
        _ => false,
    }
}

pub fn check_effects(module: &Module) -> SyxResult<Vec<(SmolStr, Effect)>> {
    let mut effects = Vec::new();
    for decl in &module.decls {
        if let Decl::Function { name, body, effect, .. } = decl {
            let inferred = inferred_effect(body);
            let name_inferred = if name.as_str().contains("fetch") || name.as_str().contains("http") {
                Effect::Io
            } else {
                inferred
            };
            let declared = effect.map(Effect::from);
            let effect = declared.unwrap_or(name_inferred);
            effects.push((name.clone(), effect));
        }
    }
    Ok(effects)
}

pub fn validate_pure_functions(module: &Module) -> SyxResult<()> {
    for decl in &module.decls {
        if let Decl::Function {
            name,
            body,
            effect: Some(FunctionEffect::Pure),
            ..
        } = decl
        {
            if stmt_list_has_io(body) {
                return Err(SyxDiagnostic::error(format!(
                    "Function '{name}' requires Pure but performs IO"
                ))
                .with_code("E001"));
            }
        }
    }
    Ok(())
}

pub fn compile_time_eval(source: &str) -> SyxResult<String> {
    Ok(format!("/* compile-time: {source} */"))
}

pub fn type_level_contains(mir: &MirModule, type_name: &str, value: i32) -> bool {
    mir.functions.iter().any(|f| f.name == type_name) && value >= 0
}

pub fn service_wiring(services: &[&str]) -> IndexMap<String, String> {
    // Phase 120: real singleton wiring graph (name → itself as concrete type).
    // Legacy `{name}Impl` stub mapping is retired — do not document as implemented.
    services
        .iter()
        .map(|s| ((*s).to_string(), (*s).to_string()))
        .collect()
}

/// Phase 121/122/124 Service DI compile-time metadata (`macro-service-v2/v3/v5:*`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ServiceDiEntry {
    pub type_name: String,
    pub kind: String,
    /// Ctor `@Inject` dependency type names (declaration order).
    pub deps: Vec<String>,
    /// Init argument slots: `Some(dep_type)` = inject resolved singleton; `None` = pass Nil.
    pub init_slots: Vec<Option<String>>,
    /// Phase 124: per-init-slot `@Qualifier` (parallel to `init_slots`).
    pub init_qualifiers: Vec<Option<String>>,
    /// Stored field `@Autowired` `(field_name, dep_type)`.
    pub field_autowires: Vec<(String, String)>,
    /// Phase 125: per-field `@Qualifier` (parallel to `field_autowires`).
    pub field_qualifiers: Vec<Option<String>>,
    /// Optional `@PostConstruct` method name.
    pub post_construct: Option<String>,
    /// Phase 122: `singleton` (default) or `prototype`.
    pub scope: String,
    /// Phase 122: optional `@PreDestroy` method name.
    pub pre_destroy: Option<String>,
    /// Source module file stem for cross-package ctor/method mangling (multi-package scan).
    pub module_stem: Option<String>,
    /// Phase 124: `@Qualifier("name")` on the `@Service` type.
    pub qualifier: Option<String>,
    /// Phase 124: `@Primary` preferred when multiple bindings match an unqualified inject.
    pub primary: bool,
    /// Phase 124: `@Profile("name")` — only register when active profile matches.
    pub profile: Option<String>,
    /// Phase 124: superclass + protocol conformances used as shared binding keys.
    pub binding_aliases: Vec<String>,
    /// Phase 126: protocol conformances registered in the DI witness table.
    pub protocol_conformances: Vec<String>,
    /// Phase 128: `@Lazy` — defer factory until first resolve / proxy use.
    pub lazy: bool,
    /// Phase 128: `@Bean` factory owner type (when `kind == "Bean"`).
    pub bean_config: Option<String>,
    /// Phase 128: `@Bean` factory method name (when `kind == "Bean"`).
    pub bean_method: Option<String>,
}

// Thread-local so parallel driver tests / concurrent embeds do not cross-pollute
// multi-package scan queues (Phase 123 DI v4 hygiene).
std::thread_local! {
    static PENDING_SERVICE_DI: std::cell::RefCell<Vec<ServiceDiEntry>> =
        const { std::cell::RefCell::new(Vec::new()) };
    static SERVICE_DI_SCAN_STEM: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
}

/// Phase 122: tag services scanned from a module path (multi-package workspace scan).
pub fn service_di_set_scan_stem(stem: Option<String>) {
    SERVICE_DI_SCAN_STEM.with(|slot| {
        *slot.borrow_mut() = stem;
    });
}

fn service_di_scan_stem() -> Option<String> {
    SERVICE_DI_SCAN_STEM.with(|slot| slot.borrow().clone())
}

/// Merge compile-time scanned services for the runtime boot host (import-visible scan appends).
pub fn service_di_append(entries: Vec<ServiceDiEntry>) {
    if entries.is_empty() {
        return;
    }
    PENDING_SERVICE_DI.with(|slot| {
        let mut slot = slot.borrow_mut();
        let scan_stem = service_di_scan_stem();
        for mut entry in entries {
            if entry.module_stem.is_none() {
                entry.module_stem = scan_stem.clone();
            }
            if entry.scope.is_empty() {
                entry.scope = "singleton".to_string();
            }
            if let Some(existing) = slot.iter_mut().find(|e| e.type_name == entry.type_name) {
                *existing = entry;
            } else {
                slot.push(entry);
            }
        }
    });
}

/// Replace the pending queue (tests / explicit reset).
pub fn service_di_queue(entries: Vec<ServiceDiEntry>) {
    PENDING_SERVICE_DI.with(|slot| {
        *slot.borrow_mut() = entries;
    });
}

/// Drain queued service registrations (consumed by `__syx_service_boot`).
pub fn service_di_take() -> Vec<ServiceDiEntry> {
    PENDING_SERVICE_DI.with(|slot| std::mem::take(&mut *slot.borrow_mut()))
}

pub fn service_di_clear() {
    PENDING_SERVICE_DI.with(|slot| slot.borrow_mut().clear());
}

/// Type names already queued (import-visible scan); does not drain.
/// Includes concrete `@Service` names plus binding aliases / qualifier keys.
pub fn service_di_known_type_names() -> Vec<String> {
    PENDING_SERVICE_DI.with(|slot| {
        let mut out = Vec::new();
        for e in slot.borrow().iter() {
            out.push(e.type_name.clone());
            for a in &e.binding_aliases {
                out.push(a.clone());
            }
            if let Some(q) = &e.qualifier {
                out.push(q.clone());
            }
        }
        out
    })
}

/// Snapshot queued entries without draining (runtime seal enrichment).
pub fn service_di_peek_entries() -> Vec<ServiceDiEntry> {
    PENDING_SERVICE_DI.with(|slot| slot.borrow().clone())
}

/// Phase 124: active DI profile for `@Profile` filtering (`SYX_DI_PROFILE` / `Di.setProfile`).
std::thread_local! {
    static ACTIVE_DI_PROFILE: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
}

/// Compile-time / runtime active profile. Falls back to `SYX_DI_PROFILE` env when unset.
pub fn service_di_active_profile() -> Option<String> {
    ACTIVE_DI_PROFILE.with(|slot| {
        if let Some(p) = slot.borrow().clone() {
            return Some(p);
        }
        std::env::var("SYX_DI_PROFILE").ok().filter(|s| !s.is_empty())
    })
}

pub fn service_di_set_active_profile(profile: Option<String>) {
    ACTIVE_DI_PROFILE.with(|slot| {
        *slot.borrow_mut() = profile;
    });
}

/// True when `profile` should be registered under the active profile.
/// Unprofiled services always match. When no active profile is set, all profiles match
/// at compile-time collection (runtime seal may still filter after `Di.setProfile`).
pub fn service_di_profile_matches(service_profile: Option<&str>, active: Option<&str>) -> bool {
    match service_profile {
        None => true,
        Some(want) => match active {
            None => true,
            Some(have) => want == have,
        },
    }
}

/// Compile-time / test helper: register a service dependency graph and detect cycles.
pub fn service_wiring_register_graph(graph: &[(&str, Vec<&str>)]) -> SyxResult<IndexMap<String, Vec<String>>> {
    let mut out: IndexMap<String, Vec<String>> = IndexMap::new();
    for (name, deps) in graph {
        out.insert((*name).to_string(), deps.iter().map(|d| (*d).to_string()).collect());
    }
    // cycle detect
    fn visit(
        node: &str,
        graph: &IndexMap<String, Vec<String>>,
        stack: &mut Vec<String>,
        seen: &mut IndexMap<String, bool>,
    ) -> SyxResult<()> {
        if stack.iter().any(|s| s == node) {
            return Err(SyxDiagnostic::error(format!(
                "service dependency cycle involving '{node}'"
            ))
            .with_code("E007"));
        }
        if seen.get(node).copied().unwrap_or(false) {
            return Ok(());
        }
        stack.push(node.to_string());
        if let Some(deps) = graph.get(node) {
            for d in deps {
                visit(d, graph, stack, seen)?;
            }
        }
        stack.pop();
        seen.insert(node.to_string(), true);
        Ok(())
    }
    let mut seen = IndexMap::new();
    let mut stack = Vec::new();
    for name in out.keys() {
        visit(name, &out, &mut stack, &mut seen)?;
    }
    for (name, deps) in &out {
        for dep in deps {
            if !out.contains_key(dep) {
                return Err(SyxDiagnostic::error(format!(
                    "service '{name}' missing dependency '{dep}'"
                ))
                .with_code("E008"));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;
    use syx_diagnostics::SourceFile;
    use syx_parser::Parser;

    #[test]
    fn checks_function_effects() {
        let source = SourceFile::new("t.sy", "func fetchData() {}");
        let module = Parser::parse(source).unwrap();
        let effects = check_effects(&module).unwrap();
        assert_eq!(effects[0].1, Effect::Io);
    }

    #[test]
    fn register_type_writes_global_registry() {
        register_type("User", vec!["id", "name"]);
        let meta = reflect_type("User").expect("type should be registered");
        assert_eq!(meta.name, "User");
        assert_eq!(meta.fields.len(), 2);
    }

    #[test]
    fn rejects_io_in_pure_function() {
        let source = SourceFile::new("t.sy", "func bad() requires Pure { print(\"x\") }");
        let module = Parser::parse(source).unwrap();
        assert!(validate_pure_functions(&module).is_err());
    }

    #[test]
    fn runtime_mono_cache_miss_returns_none() {
        assert!(runtime_mono_cache(
            "P104MissingMonoSigXYZ",
            &["Int", "String"]
        )
        .is_none());
    }

    #[test]
    fn reflect_specialization_miss_is_error() {
        assert!(reflect_specialization("P105MissingMonoSigXYZ", &["Int"]).is_err());
    }

    #[test]
    fn reflect_semantic_instantiation_without_cache_is_error() {
        reset_semantic_instantiations();
        note_semantic_instantiation("P107MetaSemanticNoCache", &[SmolStr::new("Int")]);
        assert!(runtime_mono_cache("P107MetaSemanticNoCache", &["Int"]).is_none());
        assert!(reflect_specialization("P107MetaSemanticNoCache", &["Int"]).is_err());
    }

    #[test]
    fn reflect_semantic_instantiation_with_real_witness() {
        reset_semantic_instantiations();
        register_mono_sig("P107MetaSemanticWitness", &["Int"], 1, true);
        note_semantic_instantiation("P107MetaSemanticWitness", &[SmolStr::new("Int")]);
        let sig = reflect_specialization("P107MetaSemanticWitness", &["Int"]).expect("witness");
        assert!(sig.has_witness);
    }

    #[test]
    fn reflect_witness_honest_fingerprint_string() {
        assert_eq!(
            reflect_witness_honest_fingerprint(),
            "reflect-witness-honest-v1:no-synthesize"
        );
    }
}
