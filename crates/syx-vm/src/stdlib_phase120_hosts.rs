//! Phase 120–129: runtime ServiceContainer (singleton + prototype + request + Qualifier/Primary/Profile + `@Bean` + `@Lazy`).
//! Fingerprints: `runtime-service-v5:with-request`, `macro-service-v5:conditional-qualifier`,
//! `macro-service-v5:conditional-profile`, `runtime-service-v7:protocol-witness`,
//! `runtime-service-v8:configuration-bean-mvp`, `runtime-service-v9:lazy-cycle-proxy-mvp`,
//! `runtime-service-v10:bean-param-autowire-mvp`

use std::collections::HashMap;

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_runtime_core::Value;

#[derive(Default, Clone)]
struct ServiceEntry {
    kind: String,
    deps: Vec<String>,
    init_slots: Vec<Option<String>>,
    init_qualifiers: Vec<Option<String>>,
    field_autowires: Vec<(String, String)>,
    field_qualifiers: Vec<Option<String>>,
    post_construct: Option<String>,
    pre_destroy: Option<String>,
    scope: String,
    module_stem: Option<String>,
    qualifier: Option<String>,
    primary: bool,
    profile: Option<String>,
    binding_aliases: Vec<String>,
    /// Phase 128: `@Lazy` deferral / cycle-break proxy.
    lazy: bool,
    bean_config: Option<String>,
    bean_method: Option<String>,
}

/// Phase 126: explicit protocol → concrete `@Service` witness entry.
#[derive(Clone)]
struct ProtocolWitness {
    impl_type: String,
    qualifier: Option<String>,
    primary: bool,
}

struct ServiceContainer {
    registry: HashMap<String, ServiceEntry>,
    order: Vec<String>,
    singletons: HashMap<String, Value>,
    /// Phase 126: protocol conformance witness table (protocol name → impl candidates).
    protocol_witnesses: HashMap<String, Vec<ProtocolWitness>>,
    /// Phase 123: per-request-frame instances for `@Scope("request")` services.
    request_frame: Option<HashMap<String, Value>>,
    sealed: bool,
    resolving: Vec<String>,
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self {
            registry: HashMap::new(),
            order: Vec::new(),
            singletons: HashMap::new(),
            protocol_witnesses: HashMap::new(),
            request_frame: None,
            sealed: false,
            resolving: Vec::new(),
        }
    }
}

// Thread-local container: parallel embed/driver tests must not share request/singleton frames.
std::thread_local! {
    static CONTAINER: std::cell::RefCell<ServiceContainer> =
        std::cell::RefCell::new(ServiceContainer::default());
}

fn with_container_mut<R>(f: impl FnOnce(&mut ServiceContainer) -> R) -> R {
    CONTAINER.with(|c| f(&mut c.borrow_mut()))
}

fn with_container<R>(f: impl FnOnce(&ServiceContainer) -> R) -> R {
    CONTAINER.with(|c| f(&c.borrow()))
}

fn entry_from_meta(meta: &syx_meta::ServiceDiEntry) -> ServiceEntry {
    ServiceEntry {
        kind: meta.kind.clone(),
        deps: meta.deps.clone(),
        init_slots: meta.init_slots.clone(),
        init_qualifiers: meta.init_qualifiers.clone(),
        field_autowires: meta.field_autowires.clone(),
        field_qualifiers: meta.field_qualifiers.clone(),
        post_construct: meta.post_construct.clone(),
        pre_destroy: meta.pre_destroy.clone(),
        scope: if meta.scope.is_empty() {
            "singleton".to_string()
        } else {
            meta.scope.clone()
        },
        module_stem: meta.module_stem.clone(),
        qualifier: meta.qualifier.clone(),
        primary: meta.primary,
        profile: meta.profile.clone(),
        binding_aliases: meta.binding_aliases.clone(),
        lazy: meta.lazy,
        bean_config: meta.bean_config.clone(),
        bean_method: meta.bean_method.clone(),
    }
}

fn entry_matches_active_profile(entry: &ServiceEntry) -> bool {
    syx_meta::service_di_profile_matches(
        entry.profile.as_deref(),
        syx_meta::service_di_active_profile().as_deref(),
    )
}

fn apply_pending_entries(c: &mut ServiceContainer, pending: &[syx_meta::ServiceDiEntry]) {
    for meta in pending {
        let entry = entry_from_meta(meta);
        if !entry_matches_active_profile(&entry) {
            continue;
        }
        if !c.registry.contains_key(&meta.type_name) {
            c.order.push(meta.type_name.clone());
        }
        c.registry
            .entry(meta.type_name.clone())
            .and_modify(|existing| merge_service_entry(existing, &entry))
            .or_insert(entry);
        register_protocol_witnesses_for_entry(c, meta);
    }
}

fn merge_service_entry(existing: &mut ServiceEntry, incoming: &ServiceEntry) {
    existing.kind = incoming.kind.clone();
    if !incoming.deps.is_empty() {
        existing.deps = incoming.deps.clone();
    }
    if !incoming.init_slots.is_empty() {
        existing.init_slots = incoming.init_slots.clone();
        existing.init_qualifiers = incoming.init_qualifiers.clone();
    }
    if !incoming.field_autowires.is_empty() {
        existing.field_autowires = incoming.field_autowires.clone();
        existing.field_qualifiers = incoming.field_qualifiers.clone();
    }
    existing.post_construct = incoming.post_construct.clone();
    existing.pre_destroy = incoming.pre_destroy.clone();
    if !incoming.scope.is_empty() {
        existing.scope = incoming.scope.clone();
    }
    existing.module_stem = incoming.module_stem.clone();
    existing.primary = existing.primary || incoming.primary;
    existing.profile = incoming.profile.clone();
    for alias in &incoming.binding_aliases {
        if !existing.binding_aliases.iter().any(|a| a == alias) {
            existing.binding_aliases.push(alias.clone());
        }
    }
    if existing.qualifier.is_none() {
        existing.qualifier = incoming.qualifier.clone();
    }
    existing.lazy = existing.lazy || incoming.lazy;
    if incoming.bean_config.is_some() {
        existing.bean_config = incoming.bean_config.clone();
        existing.bean_method = incoming.bean_method.clone();
    }
}

fn register_protocol_witnesses_for_entry(
    c: &mut ServiceContainer,
    meta: &syx_meta::ServiceDiEntry,
) {
    for protocol in &meta.protocol_conformances {
        let witness = ProtocolWitness {
            impl_type: meta.type_name.clone(),
            qualifier: meta.qualifier.clone(),
            primary: meta.primary,
        };
        let slot = c.protocol_witnesses.entry(protocol.clone()).or_default();
        if let Some(existing) = slot.iter_mut().find(|w| w.impl_type == witness.impl_type) {
            *existing = witness;
        } else {
            slot.push(witness);
        }
    }
}

fn resolve_protocol_witness(
    c: &ServiceContainer,
    protocol: &str,
    inject_qualifier: Option<&str>,
) -> Option<String> {
    let witnesses = c.protocol_witnesses.get(protocol)?;
    if let Some(q) = inject_qualifier {
        return witnesses
            .iter()
            .find(|w| w.qualifier.as_deref() == Some(q))
            .map(|w| w.impl_type.clone());
    }
    if witnesses.len() == 1 {
        return Some(witnesses[0].impl_type.clone());
    }
    let primaries: Vec<_> = witnesses.iter().filter(|w| w.primary).collect();
    if primaries.len() == 1 {
        return Some(primaries[0].impl_type.clone());
    }
    None
}

fn is_prototype_scope(scope: &str) -> bool {
    scope == "prototype"
}

fn is_request_scope(scope: &str) -> bool {
    scope == "request"
}

fn find_user_fn_name(type_name: &str, method: &str, module_stem: Option<&str>) -> String {
    use crate::program_ctx::with_program_ctx;

    let exact = format!("{type_name}_{method}");
    let stemmed = module_stem.map(|s| format!("{s}_{type_name}_{method}"));
    let suffix = format!("_{type_name}_{method}");

    if let Some(ctx) = with_program_ctx(|c| c.clone()) {
        if ctx.program.functions.iter().any(|f| f.name == exact) {
            return exact;
        }
        if let Some(ref name) = stemmed {
            if ctx
                .program
                .functions
                .iter()
                .any(|f| f.name.as_str() == name)
            {
                return name.clone();
            }
        }
        for f in &ctx.program.functions {
            if f.name.ends_with(suffix.as_str()) {
                return f.name.to_string();
            }
        }
    }
    exact
}

fn run_predestroy_for_instance(
    type_name: &str,
    entry: &ServiceEntry,
    instance: &Value,
) -> SyxResult<()> {
    let Some(method) = entry.pre_destroy.as_ref() else {
        return Ok(());
    };
    let fn_name = find_user_fn_name(type_name, method, entry.module_stem.as_deref());
    let _ = invoke_user_fn_allow_nil(&fn_name, &[instance.clone()])?;
    Ok(())
}

fn run_predestroy_hooks(c: &ServiceContainer) -> SyxResult<()> {
    for type_name in c.order.iter().rev() {
        let Some(entry) = c.registry.get(type_name) else {
            continue;
        };
        if is_request_scope(&entry.scope) {
            continue;
        }
        if let Some(instance) = c.singletons.get(type_name) {
            run_predestroy_for_instance(type_name, entry, instance)?;
        }
    }
    Ok(())
}

fn run_request_frame_predestroy(
    c: &ServiceContainer,
    frame: &HashMap<String, Value>,
) -> SyxResult<()> {
    for type_name in c.order.iter().rev() {
        let Some(entry) = c.registry.get(type_name) else {
            continue;
        };
        if !is_request_scope(&entry.scope) {
            continue;
        }
        if let Some(instance) = frame.get(type_name) {
            run_predestroy_for_instance(type_name, entry, instance)?;
        }
    }
    Ok(())
}

pub fn host_service_reset(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    with_container(|c| {
        if let Some(frame) = &c.request_frame {
            run_request_frame_predestroy(c, frame)?;
        }
        run_predestroy_hooks(c)
    })?;
    with_container_mut(|c| *c = ServiceContainer::default());
    Ok(())
}

pub fn host_service_destroy(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    with_container(|c| {
        if let Some(frame) = &c.request_frame {
            run_request_frame_predestroy(c, frame)?;
        }
        run_predestroy_hooks(c)
    })?;
    with_container_mut(|c| {
        c.singletons.clear();
        c.request_frame = None;
        c.resolving.clear();
        c.sealed = false;
    });
    Ok(())
}

pub fn host_service_begin_request(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    with_container_mut(|c| {
        if !c.sealed {
            return Err(SyxDiagnostic::error(
                "service container not sealed; call __syx_service_boot / __syx_service_seal first",
            ));
        }
        if c.request_frame.is_some() {
            return Err(SyxDiagnostic::error(
                "request frame already active; nested request frames are not supported in MVP",
            ));
        }
        c.request_frame = Some(HashMap::new());
        Ok(())
    })
}

pub fn host_service_end_request(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    let frame = with_container_mut(|c| {
        c.request_frame
            .take()
            .ok_or_else(|| SyxDiagnostic::error("no active request frame to end"))
    })?;
    with_container(|c| run_request_frame_predestroy(c, &frame))
}

/// Phase 124: begin request, invoke zero-arg handler by name, end request (Ui onClick pattern).
pub fn host_service_run_in_request(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handler = pop_string(stack, "handler")?;
    host_service_begin_request(&mut Vec::new())?;
    let result = invoke_user_fn_allow_nil(&handler, &[]);
    let end = host_service_end_request(&mut Vec::new());
    result?;
    end
}

/// Phase 124: set active `@Profile` name (empty clears to env/`SYX_DI_PROFILE` fallback).
pub fn host_service_set_profile(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = pop_string(stack, "profile")?;
    if name.is_empty() {
        syx_meta::service_di_set_active_profile(None);
    } else {
        syx_meta::service_di_set_active_profile(Some(name));
    }
    Ok(())
}

pub fn host_service_boot(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    let pending = syx_meta::service_di_take();
    if !pending.is_empty() {
        syx_meta::service_di_append(pending.clone());
        with_container_mut(|c| {
            c.sealed = false;
            c.registry.clear();
            c.order.clear();
            c.singletons.clear();
            c.request_frame = None;
            c.resolving.clear();
            apply_pending_entries(c, &pending);
        });
    }
    host_service_seal(&mut Vec::new())
}

pub fn host_service_register(stack: &mut Vec<Value>) -> SyxResult<()> {
    let kind = pop_string(stack, "kind")?;
    let type_name = pop_string(stack, "type")?;
    with_container_mut(|c| {
        if c.sealed {
            return Err(SyxDiagnostic::error(
                "service container sealed; cannot register",
            ));
        }
        if !c.registry.contains_key(&type_name) {
            c.order.push(type_name.clone());
        }
        c.registry
            .entry(type_name)
            .and_modify(|e| e.kind = kind.clone())
            .or_insert(ServiceEntry {
                kind,
                deps: Vec::new(),
                init_slots: Vec::new(),
                init_qualifiers: Vec::new(),
                field_autowires: Vec::new(),
                field_qualifiers: Vec::new(),
                post_construct: None,
                pre_destroy: None,
                scope: "singleton".to_string(),
                module_stem: None,
                qualifier: None,
                primary: false,
                profile: None,
                binding_aliases: Vec::new(),
                lazy: false,
                bean_config: None,
                bean_method: None,
            });
        Ok(())
    })
}

/// Phase 126: register explicit protocol conformance witness (protocol → impl + optional qualifier).
pub fn host_service_register_witness(stack: &mut Vec<Value>) -> SyxResult<()> {
    let primary_raw = pop_string(stack, "primary")?;
    let qualifier_raw = pop_string(stack, "qualifier")?;
    let impl_type = pop_string(stack, "impl")?;
    let protocol = pop_string(stack, "protocol")?;
    let qualifier = if qualifier_raw.is_empty() {
        None
    } else {
        Some(qualifier_raw)
    };
    with_container_mut(|c| {
        if c.sealed {
            return Err(SyxDiagnostic::error(
                "service container sealed; cannot register witness",
            ));
        }
        let meta_primary = syx_meta::service_di_peek_entries()
            .iter()
            .find(|e| e.type_name == impl_type)
            .map(|e| e.primary)
            .unwrap_or(false);
        let primary = primary_raw == "1"
            || primary_raw.eq_ignore_ascii_case("true")
            || meta_primary
            || c.registry
                .get(&impl_type)
                .map(|e| e.primary)
                .unwrap_or(false);
        let witness = ProtocolWitness {
            impl_type: impl_type.clone(),
            qualifier: qualifier.clone(),
            primary,
        };
        let slot = c.protocol_witnesses.entry(protocol.clone()).or_default();
        if let Some(existing) = slot.iter_mut().find(|w| w.impl_type == impl_type) {
            *existing = witness;
        } else {
            slot.push(witness);
        }
        if let Some(entry) = c.registry.get_mut(&impl_type) {
            if !entry.binding_aliases.iter().any(|a| a == &protocol) {
                entry.binding_aliases.push(protocol);
            }
            if primary {
                entry.primary = true;
            }
            if entry.qualifier.is_none() {
                entry.qualifier = qualifier;
            }
        }
        Ok(())
    })
}

pub fn host_service_add_dep(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dep = pop_string(stack, "dep")?;
    let type_name = pop_string(stack, "type")?;
    with_container_mut(|c| {
        if c.sealed {
            return Err(SyxDiagnostic::error(
                "service container sealed; cannot add dep",
            ));
        }
        let Some(entry) = c.registry.get_mut(&type_name) else {
            return Err(SyxDiagnostic::error(format!(
                "unknown service '{type_name}' for add_dep"
            )));
        };
        if !entry.deps.contains(&dep) {
            entry.deps.push(dep);
        }
        Ok(())
    })
}

pub fn host_service_seal(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    with_container_mut(|c| {
        if c.sealed {
            return Ok(());
        }
        apply_pending_entries(c, &syx_meta::service_di_peek_entries());
        // Runtime profile filter (after Di.setProfile / SYX_DI_PROFILE).
        let drop_names: Vec<String> = c
            .registry
            .iter()
            .filter(|(_, e)| !entry_matches_active_profile(e))
            .map(|(n, _)| n.clone())
            .collect();
        for name in drop_names {
            c.registry.remove(&name);
            c.order.retain(|n| n != &name);
            c.singletons.remove(&name);
        }
        for (name, entry) in &c.registry {
            for dep in &entry.deps {
                if c.registry.contains_key(dep) {
                    continue;
                }
                if resolve_binding_key(c, dep).is_err() {
                    return Err(SyxDiagnostic::error(format!(
                        "service '{name}' missing dependency '{dep}'"
                    )));
                }
            }
        }
        let graph: HashMap<String, Vec<String>> = c
            .registry
            .iter()
            .map(|(k, v)| (k.clone(), v.deps.clone()))
            .collect();
        let lazy_map: HashMap<String, bool> = c
            .registry
            .iter()
            .map(|(k, v)| (k.clone(), v.lazy))
            .collect();
        let mut seen = HashMap::new();
        let mut stack_nodes = Vec::new();
        for name in graph.keys() {
            visit_cycle(name, &graph, &lazy_map, &mut stack_nodes, &mut seen)?;
        }
        c.sealed = true;
        Ok(())
    })
}

fn cycle_has_lazy_break(stack: &[String], back_to: &str, lazy_map: &HashMap<String, bool>) -> bool {
    let Some(start) = stack.iter().position(|s| s == back_to) else {
        return lazy_map.get(back_to).copied().unwrap_or(false);
    };
    stack[start..]
        .iter()
        .any(|n| lazy_map.get(n).copied().unwrap_or(false))
        || lazy_map.get(back_to).copied().unwrap_or(false)
}

fn visit_cycle(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    lazy_map: &HashMap<String, bool>,
    stack: &mut Vec<String>,
    seen: &mut HashMap<String, bool>,
) -> SyxResult<()> {
    if stack.iter().any(|s| s == node) {
        if cycle_has_lazy_break(stack, node, lazy_map) {
            return Ok(());
        }
        return Err(SyxDiagnostic::error(format!(
            "service dependency cycle involving '{node}'"
        )));
    }
    if seen.get(node).copied().unwrap_or(false) {
        return Ok(());
    }
    stack.push(node.to_string());
    if let Some(deps) = graph.get(node) {
        for d in deps {
            if stack.iter().any(|s| s == d) {
                if cycle_has_lazy_break(stack, d, lazy_map) {
                    continue;
                }
                return Err(SyxDiagnostic::error(format!(
                    "service dependency cycle involving '{d}'"
                )));
            }
            visit_cycle(d, graph, lazy_map, stack, seen)?;
        }
    }
    stack.pop();
    seen.insert(node.to_string(), true);
    Ok(())
}

pub fn host_service_resolve(stack: &mut Vec<Value>) -> SyxResult<()> {
    let type_name = pop_string(stack, "type")?;
    let value = resolve_one(&type_name)?;
    stack.push(value);
    Ok(())
}

pub fn host_service_put(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut instance = stack
        .pop()
        .ok_or_else(|| SyxDiagnostic::error("service put expected instance value"))?;
    let type_name = pop_string(stack, "type")?;
    // Refresh registry from meta so field_autowires / qualifiers are present even when
    // the SYX boot only emitted bare `__syx_service_register` calls.
    with_container_mut(|c| {
        apply_pending_entries(c, &syx_meta::service_di_peek_entries());
    });
    let entry = with_container(|c| c.registry.get(&type_name).cloned());
    if let Some(entry) = entry {
        inject_autowired_fields(&mut instance, &entry)?;
    }
    with_container_mut(|c| {
        if c.sealed {
            return Err(SyxDiagnostic::error(
                "service container sealed; cannot put instance",
            ));
        }
        c.singletons.insert(type_name, instance);
        Ok(())
    })
}

const LAZY_FLAG: &str = "__syx_lazy";
const LAZY_INSTANCE: &str = "__syx_lazy_instance";

fn lazy_flag_true(v: &Value) -> bool {
    let inner = Value::dispatch_receiver_for_method(v.clone());
    matches!(
        inner.get_by_key(&Value::Str(LAZY_FLAG.into())),
        Value::Bool(true)
    )
}

fn make_lazy_proxy(type_name: &str) -> Value {
    let mut map = HashMap::new();
    map.insert("__type".into(), Value::Str(type_name.into()));
    map.insert(LAZY_FLAG.into(), Value::Bool(true));
    map.insert(LAZY_INSTANCE.into(), Value::Nil);
    Value::object_ref_from_map(map, true)
}

/// Phase 128: create or reuse a lazy proxy handle for `typeName`.
pub fn host_service_lazy_proxy(stack: &mut Vec<Value>) -> SyxResult<()> {
    let type_name = pop_string(stack, "type")?;
    let proxy = with_container_mut(|c| {
        apply_pending_entries(c, &syx_meta::service_di_peek_entries());
        if let Some(existing) = c.singletons.get(&type_name) {
            return existing.clone();
        }
        let proxy = make_lazy_proxy(&type_name);
        c.singletons.insert(type_name, proxy.clone());
        proxy
    });
    stack.push(proxy);
    Ok(())
}

/// Materialize a `@Lazy` proxy receiver before instance method dispatch.
/// Returns the real instance when `receiver` is an unmaterialized lazy proxy.
pub fn materialize_lazy_receiver(receiver: Value) -> SyxResult<Value> {
    let inner = Value::dispatch_receiver_for_method(receiver.clone());
    let cached = inner.get_by_key(&Value::Str(LAZY_INSTANCE.into()));
    if !matches!(cached, Value::Nil) && !lazy_flag_true(&cached) {
        return Ok(cached);
    }
    if !lazy_flag_true(&receiver) {
        return Ok(receiver);
    }
    let target = inner
        .object_ref_type_name()
        .ok_or_else(|| SyxDiagnostic::error("lazy proxy missing __type"))?;
    resolve_one(&target)
}

fn invoke_user_fn(fn_name: &str, args: &[Value]) -> SyxResult<Value> {
    invoke_user_fn_inner(fn_name, args, true)
}

fn invoke_user_fn_allow_nil(fn_name: &str, args: &[Value]) -> SyxResult<Value> {
    invoke_user_fn_inner(fn_name, args, false)
}

fn invoke_user_fn_inner(fn_name: &str, args: &[Value], reject_nil: bool) -> SyxResult<Value> {
    use crate::program_ctx::{with_active_vm, with_program_ctx};

    let Some(ctx) = with_program_ctx(|c| c.clone()) else {
        return Err(SyxDiagnostic::error(
            "service resolve requires active program context",
        ));
    };
    let func = ctx
        .program
        .functions
        .iter()
        .find(|f| f.name == fn_name)
        .ok_or_else(|| {
            SyxDiagnostic::error(format!(
                "service constructor '{fn_name}' not found in program"
            ))
        })?;
    if let Some(result) = with_active_vm(|vm| {
        let saved_stack = std::mem::take(&mut vm.stack);
        let saved_locals = std::mem::take(&mut vm.locals);
        let run = vm.run_function_with_args(func, &ctx.program, args);
        vm.stack = saved_stack;
        vm.locals = saved_locals;
        let result = run?;
        if reject_nil && matches!(result, Value::Nil) {
            return Err(SyxDiagnostic::error(format!(
                "service constructor '{fn_name}' returned nil"
            )));
        }
        Ok(result)
    }) {
        return result;
    }
    let mut vm = crate::VmInstance::new();
    vm.runtime = ctx.runtime.clone();
    let result = vm.run_function_with_args(func, &ctx.program, args)?;
    if reject_nil && matches!(result, Value::Nil) {
        return Err(SyxDiagnostic::error(format!(
            "service constructor '{fn_name}' returned nil"
        )));
    }
    Ok(result)
}

fn build_init_args(entry: &ServiceEntry) -> SyxResult<Vec<Value>> {
    let mut args = Vec::new();
    if entry.init_slots.is_empty() {
        let field_dep_types: std::collections::HashSet<&str> = entry
            .field_autowires
            .iter()
            .map(|(_, t)| t.as_str())
            .collect();
        for dep in &entry.deps {
            if field_dep_types.contains(dep.as_str()) {
                continue;
            }
            args.push(ensure_dep_value(dep, None)?);
        }
        return Ok(args);
    }
    for (idx, slot) in entry.init_slots.iter().enumerate() {
        match slot {
            Some(dep_type) => {
                let qual = entry.init_qualifiers.get(idx).and_then(|q| q.as_deref());
                args.push(ensure_dep_value(dep_type, qual)?);
            }
            None => args.push(Value::Nil),
        }
    }
    Ok(args)
}

fn resolve_one_with_qualifier(type_name: &str, inject_qualifier: Option<&str>) -> SyxResult<Value> {
    if let Some(concrete) =
        with_container(|c| resolve_protocol_witness(c, type_name, inject_qualifier))
    {
        if with_container(|c| c.registry.contains_key(&concrete)) {
            return resolve_one(&concrete);
        }
    }
    resolve_one(type_name)
}

fn inject_autowired_fields(instance: &mut Value, entry: &ServiceEntry) -> SyxResult<()> {
    for (i, (field, dep_type)) in entry.field_autowires.iter().enumerate() {
        let key = entry
            .field_qualifiers
            .get(i)
            .and_then(|q| q.as_deref())
            .unwrap_or(dep_type.as_str());
        let dep_val = resolve_field_autowire_singleton(key, dep_type, field)?;
        *instance = Value::mut_ref_cell_set_field(instance.clone(), field, dep_val);
    }
    Ok(())
}

fn resolve_field_autowire_singleton(key: &str, dep_type: &str, field: &str) -> SyxResult<Value> {
    let dep_val = with_container(|c| {
        c.singletons.get(dep_type).cloned().or_else(|| {
            c.registry
                .iter()
                .find(|(name, e)| entry_provides(e, name, key))
                .and_then(|(name, _)| c.singletons.get(name).cloned())
        })
    });
    match dep_val {
        Some(v) => Ok(v),
        None => {
            let sealed = with_container(|c| c.sealed);
            if sealed {
                ensure_dep_value(key, None)
            } else {
                // Before seal (boot put): create lazy proxy when target is `@Lazy`.
                let concrete = with_container(|c| resolve_binding_key(c, key).ok());
                if let Some(name) = concrete {
                    let is_lazy =
                        with_container(|c| c.registry.get(&name).map(|e| e.lazy).unwrap_or(false));
                    if is_lazy {
                        let proxy = make_lazy_proxy(&name);
                        with_container_mut(|c| {
                            c.singletons.insert(name, proxy.clone());
                        });
                        return Ok(proxy);
                    }
                }
                let keys: Vec<String> = with_container(|c| c.singletons.keys().cloned().collect());
                Err(SyxDiagnostic::error(format!(
                    "field inject '{field}: {key}' missing singleton before seal (dep_type={dep_type}, have {keys:?})"
                )))
            }
        }
    }
}

fn run_post_construct(
    type_name: &str,
    method: &str,
    instance: &Value,
    module_stem: Option<&str>,
) -> SyxResult<()> {
    let fn_name = find_user_fn_name(type_name, method, module_stem);
    let _ = invoke_user_fn_allow_nil(&fn_name, &[instance.clone()])?;
    Ok(())
}

fn entry_provides(entry: &ServiceEntry, type_name: &str, key: &str) -> bool {
    type_name == key
        || entry.binding_aliases.iter().any(|a| a == key)
        || entry.qualifier.as_deref() == Some(key)
}

fn resolve_binding_key(c: &ServiceContainer, key: &str) -> SyxResult<String> {
    if c.registry.contains_key(key) {
        return Ok(key.to_string());
    }
    if let Some(concrete) = resolve_protocol_witness(c, key, None) {
        if c.registry.contains_key(&concrete) {
            return Ok(concrete);
        }
    }
    let candidates: Vec<(&String, &ServiceEntry)> = c
        .registry
        .iter()
        .filter(|(name, e)| entry_provides(e, name, key))
        .collect();
    if candidates.is_empty() {
        return Err(SyxDiagnostic::error(format!(
            "service '{key}' is not registered"
        )));
    }
    if candidates.len() == 1 {
        return Ok(candidates[0].0.clone());
    }
    let primaries: Vec<_> = candidates.iter().filter(|(_, e)| e.primary).collect();
    match primaries.as_slice() {
        [(name, _)] => Ok((*name).clone()),
        [] => Err(SyxDiagnostic::error(format!(
            "ambiguous service binding '{key}' among {} without @Primary or @Qualifier",
            candidates.len()
        ))),
        _ => Err(SyxDiagnostic::error(format!(
            "ambiguous @Primary services for binding '{key}'"
        ))),
    }
}

fn resolve_one(type_name: &str) -> SyxResult<Value> {
    enum Phase1 {
        Cached(Value),
        Construct {
            entry: ServiceEntry,
            concrete: String,
            prototype: bool,
            request: bool,
        },
        MaterializeProxy {
            concrete: String,
            entry: ServiceEntry,
            proxy: Value,
        },
    }
    let phase1 = with_container_mut(|c| -> SyxResult<Phase1> {
        if !c.sealed {
            return Err(SyxDiagnostic::error(
                "service container not sealed; call __syx_service_boot / __syx_service_seal first",
            ));
        }
        let concrete = resolve_binding_key(c, type_name)?;
        let entry = c.registry.get(&concrete).cloned().ok_or_else(|| {
            SyxDiagnostic::error(format!("service '{concrete}' is not registered"))
        })?;
        let prototype = is_prototype_scope(&entry.scope);
        let request = is_request_scope(&entry.scope);

        if request {
            let Some(frame) = c.request_frame.as_ref() else {
                return Err(SyxDiagnostic::error(format!(
                    "request-scoped service '{concrete}' requires an active request frame; call __syx_service_begin_request first"
                )));
            };
            if let Some(v) = frame.get(&concrete) {
                if lazy_flag_true(v) {
                    return Ok(Phase1::MaterializeProxy {
                        concrete,
                        entry,
                        proxy: v.clone(),
                    });
                }
                return Ok(Phase1::Cached(v.clone()));
            }
        } else if !prototype {
            if let Some(v) = c.singletons.get(&concrete) {
                if lazy_flag_true(v) {
                    return Ok(Phase1::MaterializeProxy {
                        concrete,
                        entry,
                        proxy: v.clone(),
                    });
                }
                return Ok(Phase1::Cached(v.clone()));
            }
            if entry.kind == "Bean" && !entry.lazy {
                return Err(SyxDiagnostic::error(format!(
                    "@Bean '{concrete}' was not materialized during boot"
                )));
            }
        }

        if c.resolving.iter().any(|s| s == &concrete) {
            // Cycle: if this node (or anything on the resolving stack) is `@Lazy`,
            // return/create a proxy instead of failing.
            let breakable = entry.lazy
                || c.resolving
                    .iter()
                    .any(|n| c.registry.get(n).map(|e| e.lazy).unwrap_or(false));
            if breakable {
                if let Some(v) = c.singletons.get(&concrete) {
                    return Ok(Phase1::Cached(v.clone()));
                }
                let proxy = make_lazy_proxy(&concrete);
                c.singletons.insert(concrete.clone(), proxy.clone());
                return Ok(Phase1::Cached(proxy));
            }
            return Err(SyxDiagnostic::error(format!(
                "service resolve cycle at '{concrete}'"
            )));
        }
        c.resolving.push(concrete.clone());
        Ok(Phase1::Construct {
            entry,
            concrete,
            prototype,
            request,
        })
    })?;

    match phase1 {
        Phase1::Cached(v) => Ok(v),
        Phase1::MaterializeProxy {
            concrete,
            entry,
            proxy,
        } => materialize_proxy_into(&concrete, &entry, proxy),
        Phase1::Construct {
            entry,
            concrete,
            prototype,
            request,
        } => construct_service(&concrete, &entry, prototype, request),
    }
}

fn materialize_proxy_into(concrete: &str, entry: &ServiceEntry, proxy: Value) -> SyxResult<Value> {
    // Avoid re-entrant materialization of the same proxy.
    if with_container(|c| c.resolving.iter().any(|s| s == concrete)) {
        return Ok(proxy);
    }
    with_container_mut(|c| {
        c.resolving.push(concrete.to_string());
    });
    let real = construct_service_body(concrete, entry)?;
    with_container_mut(|c| {
        c.resolving.retain(|s| s != concrete);
        c.singletons.insert(concrete.to_string(), real.clone());
    });
    let unwrapped = Value::dispatch_receiver_for_method(proxy);
    let _ = Value::mut_ref_cell_set_field(unwrapped.clone(), LAZY_FLAG, Value::Bool(false));
    let _ = Value::mut_ref_cell_set_field(unwrapped, LAZY_INSTANCE, real.clone());
    Ok(real)
}

fn construct_service(
    concrete: &str,
    entry: &ServiceEntry,
    prototype: bool,
    request: bool,
) -> SyxResult<Value> {
    let instance = construct_service_body(concrete, entry)?;
    with_container_mut(|c| {
        c.resolving.retain(|s| s != concrete);
        if request {
            if let Some(frame) = c.request_frame.as_mut() {
                frame.insert(concrete.to_string(), instance.clone());
            }
        } else if !prototype {
            c.singletons.insert(concrete.to_string(), instance.clone());
        }
    });
    Ok(instance)
}

fn construct_service_body(concrete: &str, entry: &ServiceEntry) -> SyxResult<Value> {
    if entry.kind == "Bean" {
        return construct_bean(concrete, entry);
    }
    let init_args = build_init_args(entry)?;
    let init_name = find_user_fn_name(concrete, "init", entry.module_stem.as_deref());
    let mut instance = invoke_user_fn(&init_name, &init_args)?;
    inject_autowired_fields(&mut instance, entry)?;
    if let Some(method) = &entry.post_construct {
        run_post_construct(concrete, method, &instance, entry.module_stem.as_deref())?;
    }
    Ok(instance)
}

fn construct_bean(concrete: &str, entry: &ServiceEntry) -> SyxResult<Value> {
    let config = entry.bean_config.as_deref().ok_or_else(|| {
        SyxDiagnostic::error(format!(
            "@Bean '{concrete}' missing factory config metadata for lazy resolve"
        ))
    })?;
    let method = entry.bean_method.as_deref().ok_or_else(|| {
        SyxDiagnostic::error(format!(
            "@Bean '{concrete}' missing factory method metadata for lazy resolve"
        ))
    })?;
    let config_init = find_user_fn_name(config, "init", entry.module_stem.as_deref());
    let config_inst = invoke_user_fn(&config_init, &[])?;
    let factory = find_user_fn_name(config, method, entry.module_stem.as_deref());
    // Phase 129: `@Bean` method params resolved like ctor `@Inject` (type / qualifier).
    let mut args = vec![config_inst];
    args.extend(build_init_args(entry)?);
    invoke_user_fn(&factory, &args)
}

fn ensure_dep_value(type_name: &str, inject_qualifier: Option<&str>) -> SyxResult<Value> {
    let concrete = with_container(|c| {
        if let Some(q) = inject_qualifier {
            if let Some(name) = resolve_protocol_witness(c, type_name, Some(q)) {
                return Ok(name);
            }
        }
        resolve_binding_key(c, type_name)
    })?;
    let entry_lazy = with_container(|c| c.registry.get(&concrete).map(|e| e.lazy).unwrap_or(false));
    let in_resolving = with_container(|c| c.resolving.iter().any(|s| s == &concrete));
    if entry_lazy || in_resolving {
        // Prefer proxy when target is `@Lazy` or would re-enter a resolve cycle.
        let existing = with_container(|c| c.singletons.get(&concrete).cloned());
        if let Some(v) = existing {
            return Ok(v);
        }
        if entry_lazy || in_resolving {
            let proxy = make_lazy_proxy(&concrete);
            with_container_mut(|c| {
                c.singletons.insert(concrete.clone(), proxy.clone());
            });
            return Ok(proxy);
        }
    }
    resolve_one_with_qualifier(type_name, inject_qualifier)
}

fn pop_string(stack: &mut Vec<Value>, label: &str) -> SyxResult<String> {
    let Some(v) = stack.pop() else {
        return Err(SyxDiagnostic::error(format!(
            "service host expected String {label}, got empty stack"
        )));
    };
    Value::materialize_str(v)
        .ok_or_else(|| SyxDiagnostic::error(format!("service host expected String {label}")))
}
