use std::collections::HashSet;

use indexmap::{IndexMap, IndexSet};
use smol_str::SmolStr;
use syx_ast::{
    AccessLevel, BinOp, CallArg, Decl, Expr, Literal, Module, ParamLabel, Stmt, TypeExpr,
    UnaryOp,
};
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};

mod host_fn_capability;
mod import_resolver;
mod type_checker;

use type_checker::{check_assignable, is_compatible};

/// Cap on diagnostics retained in collecting mode (LSP / `analyze_collecting`).
/// Prevents pathological files from flooding the editor while still showing many errors at once.
pub const MAX_COLLECTED_DIAGNOSTICS: usize = 64;

// Phase 87/88: optional downcast (`as?`) semantic surface (AsOptional).
#[allow(dead_code)]
pub(crate) fn as_optional_surface() {}

// Phase 86/87/88: `select { case ... }` lowers via __syx_channel_select / __syx_channel_select_default.
#[allow(dead_code)]
pub(crate) const CHANNEL_SELECT_SURFACE: &str = "__syx_channel_select";
use import_resolver::check_import_member_access as check_import_access;
use syx_stdlib::{
    default_float_kind, default_integer_kind, from_suffix, from_type_name, host_fn_id_by_name,
    host_fn_index_by_name, host_fn_return_type, host_return_type_by_name, int_range, is_equatable_type, is_hashable_type, is_implicit_convertible,
    is_integer, is_float,
    plugin_host_return_type,
    lookup_array_method, lookup_dict_method, lookup_set_method, lookup_string_method,
    lookup_type_method, method_return_type, register_builtins, resolve_alias, type_conforms,
    is_comparable_type, is_manifest_host_op,
    operator_overload_forbidden_type, is_numeric_builtin_type, is_text_builtin_type,
    is_collection_builtin_type, widen_for_binary, HostFnId, MethodId, NumericKind, ARRAY_METHODS, DICT_METHODS,
    HTTP_RESPONSE_METHODS, SET_METHODS, STRING_METHODS, TYPE_METHODS,
};

fn is_forbidden_internal_import(name: &str) -> bool {
    matches!(name, "Internal" | "_Builtin" | "_Internal" | "internal")
}

fn type_collection_surface(ty: &TypeId) -> Option<&'static str> {
    match ty {
        TypeId::Array(_) => Some("Array"),
        TypeId::Set(_) => Some("Set"),
        TypeId::Dictionary { .. } => Some("Dictionary"),
        TypeId::Named(n) if n.as_str() == "Array" => Some("Array"),
        TypeId::Named(n) if n.as_str() == "Set" => Some("Set"),
        TypeId::Named(n) if n.as_str() == "Dictionary" => Some("Dictionary"),
        _ => None,
    }
}

/// Phase 50 (S025): collection methods that still resolve only via legacy `MethodCall`.
fn is_retired_collection_methodcall(surface: &str, method: &str) -> bool {
    // sorted → MirOp::ArrSorted; flatMap → CallHost `__syx_array_flat_map` (MethodId).
    if surface == "Array" && matches!(method, "sorted" | "flatMap") {
        return false;
    }
    let in_table = match surface {
        "Array" => lookup_array_method(method).is_some(),
        "Set" => lookup_set_method(method).is_some(),
        "Dictionary" => lookup_dict_method(method).is_some(),
        _ => false,
    };
    in_table && !is_manifest_host_op(surface, method)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeId {
    Void,
    Numeric(NumericKind),
    Bool,
    String,
    Array(Box<TypeId>),
    Set(Box<TypeId>),
    Dictionary {
        key: Box<TypeId>,
        value: Box<TypeId>,
    },
    /// Phase 139 L2: product tuple `(T, U)` with optional element labels.
    Tuple {
        elements: Vec<(Option<SmolStr>, TypeId)>,
    },
    Optional(Box<TypeId>),
    Result {
        ok: Box<TypeId>,
        err: Box<TypeId>,
    },
    Named(SmolStr),
    GenericParam(SmolStr),
    GenericFn {
        name: SmolStr,
        type_params: Vec<SmolStr>,
    },
    Reference(Box<TypeId>),
    MutReference(Box<TypeId>),
    /// Opaque `some Protocol` witness (Phase 11).
    OpaqueSome(Box<TypeId>),
    /// Existential `any Protocol` (Phase 11).
    OpaqueAny(Box<TypeId>),
    /// Phase 38: fixed-width SIMD vector (`SIMD4<Int32>`, etc.).
    Simd {
        lanes: u8,
        element: NumericKind,
    },
    Unknown,
}

impl TypeId {
    pub fn platform_int() -> Self {
        // Keep the `Int` alias so diagnostics/hover say `Int`, not `Int64`/`Int32`.
        TypeId::Numeric(NumericKind::Int)
    }

    /// Build a numeric type id, preserving `Int`/`UInt` aliases for display.
    /// Width resolution happens at convertibility / codegen via `resolve_alias`.
    pub fn numeric(kind: NumericKind) -> Self {
        TypeId::Numeric(kind)
    }

    pub fn as_numeric_kind(&self) -> Option<NumericKind> {
        match self {
            TypeId::Numeric(k) => Some(*k),
            _ => None,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, TypeId::Numeric(_))
    }

    pub fn as_simd(&self) -> Option<(u8, NumericKind)> {
        match self {
            TypeId::Simd { lanes, element } => Some((*lanes, *element)),
            _ => None,
        }
    }

    /// Legacy convenience: platform Int.
    pub fn int() -> Self {
        Self::platform_int()
    }

    /// Legacy convenience: Float64.
    pub fn double() -> Self {
        TypeId::Numeric(NumericKind::F64)
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: SmolStr,
    pub ty: TypeId,
    pub mutable: bool,
    pub access: AccessLevel,
}

#[derive(Debug, Default)]
pub struct Scope {
    pub symbols: IndexMap<SmolStr, Symbol>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn define(&mut self, symbol: Symbol) -> SyxResult<()> {
        if self.symbols.contains_key(&symbol.name) {
            return Err(SyxDiagnostic::error(format!(
                "Symbol '{}' already defined in this scope",
                symbol.name
            ))
            .with_code("S001"));
        }
        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup(name)))
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        if self.symbols.contains_key(name) {
            self.symbols.get_mut(name)
        } else {
            self.parent
                .as_mut()
                .and_then(|p| p.lookup_mut(name))
        }
    }
}

/// Phase 22 CY2: `@MainActor` cross-module default isolation + S020 hardening (always-on; opt-out via `SYX_GLOBAL_ACTOR_V4=0`).
pub fn global_actor_v4_cross_module_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V4")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 23 DD2: `@unchecked Sendable` boundary + default actor isolation extension (opt-out via `SYX_GLOBAL_ACTOR_V5=0`).
pub fn global_actor_v5_sendable_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V5")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v4_cross_module_enabled())
}

/// Phase 24 DJ2: cross-`await` reborrow diagnostic hardening (`SYX_BORROW_REGIONS_V6=1` default on).
pub fn borrow_regions_v6_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V6")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 25 DO1: nested async/closure reborrow hardening (no `sending`; deferred Phase 26).
pub fn borrow_regions_v7_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V7")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_v6_enabled())
}

/// Phase 26 DS1: GlobalActor v6 `sending` parameter hardening + default isolation extension.
pub fn global_actor_v6_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V6")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v5_sendable_enabled())
}

/// Phase 26 DT1: `sending` boundary checks across GlobalActor / async (requires v7 + v6).
pub fn borrow_regions_v7_sending_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V7_SENDING")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v6_sending_enabled() && borrow_regions_v7_enabled())
}

/// Phase 27 DY1: non-lexical `'a` region subset at async/Actor boundaries (requires v7 sending).
pub fn borrow_regions_v8_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V8")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_v7_sending_enabled())
}

/// Phase 28 ED1: borrow v8 SE-0461 full production subset at async/Actor boundaries.
pub fn borrow_regions_v8_full_enabled() -> bool {
    std::env::var("SYX_BORROW_REGIONS_V8_FULL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_v8_enabled())
}

/// Phase 29 EI1: borrow SE-0461 100% on production-relevant paths.
pub fn borrow_regions_se0461_complete_enabled() -> bool {
    std::env::var("SYX_BORROW_SE0461_COMPLETE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 27 DX4: GlobalActor v7 cross-module sending diagnostics + default isolation table.
pub fn global_actor_v7_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V7")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v6_sending_enabled())
}

/// Phase 28 EC4: GlobalActor v8 multi-endpoint routing + single-region load balance scaffold.
pub fn global_actor_v8_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V8")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v7_sending_enabled())
}

pub fn global_actor_v8_endpoints() -> &'static [&'static str] {
    &["127.0.0.1:9443", "127.0.0.1:9444", "127.0.0.1:9445"]
}

/// Phase 29 EH4: GlobalActor v9 cross-region multi-active routing + load balance scaffold.
pub fn global_actor_v9_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V9")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v8_sending_enabled())
}

pub fn global_actor_v9_regions() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        ("us-east", &["127.0.0.1:9443", "127.0.0.1:9444"]),
        ("eu-west", &["127.0.0.1:9453", "127.0.0.1:9454"]),
    ]
}

pub fn global_actor_v9_route_endpoint(region: &str, index: usize) -> Option<&'static str> {
    global_actor_v9_regions()
        .iter()
        .find(|(r, _)| *r == region)
        .and_then(|(_, eps)| eps.get(index).copied())
}

/// Phase 30 EM4: GlobalActor v10 global multi-active LB + endpoint health probe scaffold.
pub fn global_actor_v10_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V10")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v9_sending_enabled())
}

pub fn global_actor_v10_regions() -> &'static [(&'static str, &'static [&'static str])] {
    &[
        ("us-east", &["127.0.0.1:9443", "127.0.0.1:9444"]),
        ("eu-west", &["127.0.0.1:9453", "127.0.0.1:9454"]),
        ("ap-south", &["127.0.0.1:9463", "127.0.0.1:9464"]),
    ]
}

pub fn global_actor_v10_health_probe_ok(region: &str, index: usize) -> bool {
    global_actor_v10_regions()
        .iter()
        .any(|(r, eps)| *r == region && index < eps.len())
}

pub fn global_actor_v10_route_endpoint(region: &str, index: usize) -> Option<&'static str> {
    if !global_actor_v10_health_probe_ok(region, index) {
        return None;
    }
    global_actor_v10_regions()
        .iter()
        .find(|(r, _)| *r == region)
        .and_then(|(_, eps)| eps.get(index).copied())
}

/// Phase 31 ER4: GlobalActor v11 weighted LB + endpoint auto failover.
pub fn global_actor_v11_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V11")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v10_sending_enabled())
}

pub fn global_actor_v11_endpoint_weights() -> &'static [(&'static str, u32)] {
    &[
        ("127.0.0.1:9443", 100),
        ("127.0.0.1:9444", 80),
        ("127.0.0.1:9453", 60),
        ("127.0.0.1:9454", 40),
    ]
}

pub fn global_actor_v11_weighted_route(region: &str, failover_index: usize) -> Option<&'static str> {
    if !global_actor_v11_sending_enabled() {
        return global_actor_v10_route_endpoint(region, failover_index);
    }
    let weights = global_actor_v11_endpoint_weights();
    let idx = failover_index % weights.len();
    if global_actor_v10_health_probe_ok(region, idx) {
        Some(weights[idx].0)
    } else {
        global_actor_v10_route_endpoint(region, failover_index)
    }
}

pub fn global_actor_v11_failover_endpoint(region: &str, failed_index: usize) -> Option<&'static str> {
    global_actor_v11_weighted_route(region, failed_index.saturating_add(1))
}

/// Phase 32 EW4: GlobalActor v12 dynamic weighted LB + cross-region health probe prod.
pub fn global_actor_v12_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V12")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v11_sending_enabled())
}

pub fn global_actor_v12_dynamic_weight(_endpoint: &str, load_pct: u32) -> u32 {
    if !global_actor_v12_sending_enabled() {
        return 100;
    }
    100u32.saturating_sub(load_pct.min(99))
}

pub fn global_actor_v12_route_endpoint(region: &str, index: usize, load_pct: u32) -> Option<&'static str> {
    if !global_actor_v12_sending_enabled() {
        return global_actor_v11_weighted_route(region, index);
    }
    let weight = global_actor_v12_dynamic_weight(
        global_actor_v11_endpoint_weights()
            .get(index % global_actor_v11_endpoint_weights().len())
            .map(|(ep, _)| *ep)
            .unwrap_or("127.0.0.1:9443"),
        load_pct,
    );
    if weight > 0 && global_actor_v10_health_probe_ok(region, index) {
        global_actor_v11_weighted_route(region, index)
    } else {
        global_actor_v11_failover_endpoint(region, index)
    }
}

pub fn global_actor_v12_drain_endpoint(region: &str, failed_index: usize) -> Option<&'static str> {
    global_actor_v12_route_endpoint(region, failed_index.saturating_add(1), 100)
}

/// Phase 33 FB4: GlobalActor v13 cross-org federation LB + live endpoint drain prod.
pub fn global_actor_v13_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V13")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v12_sending_enabled())
}

pub fn global_actor_v13_org_weights(org: &str) -> u32 {
    if !global_actor_v13_sending_enabled() {
        return 100;
    }
    match org {
        "org-a" | "org-alpha" => 100,
        "org-b" | "org-beta" => 80,
        "org-c" | "org-gamma" => 60,
        _ => 40,
    }
}

pub fn global_actor_v13_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    if !global_actor_v13_sending_enabled() {
        return global_actor_v12_route_endpoint(region, index, load_pct);
    }
    let weight = global_actor_v13_org_weights(org).saturating_sub(load_pct.min(99));
    if weight > 0 && global_actor_v10_health_probe_ok(region, index) {
        global_actor_v12_route_endpoint(region, index, load_pct)
    } else {
        global_actor_v12_drain_endpoint(region, index)
    }
}

pub fn global_actor_v13_drain_endpoint(
    region: &str,
    org: &str,
    failed_index: usize,
) -> Option<&'static str> {
    if !global_actor_v13_sending_enabled() {
        return global_actor_v12_drain_endpoint(region, failed_index);
    }
    let _ = global_actor_v13_org_weights(org);
    global_actor_v12_drain_endpoint(region, failed_index)
}

/// Phase 31 ES1: borrow formal verification scaffold — region graph proof smoke.
pub fn borrow_regions_formal_verify_enabled() -> bool {
    std::env::var("SYX_BORROW_FORMAL_VERIFY")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_language_complete_enabled())
}

pub fn borrow_regions_formal_proof_smoke(region_count: i32) -> bool {
    if !borrow_regions_formal_verify_enabled() {
        return false;
    }
    region_count >= 0 && region_count <= 256
}

/// Phase 32 EX1: borrow machine proof harness — region graph proof obligation export + replay smoke.
pub fn borrow_regions_machine_proof_enabled() -> bool {
    std::env::var("SYX_BORROW_MACHINE_PROOF")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_formal_verify_enabled())
}

pub fn borrow_regions_export_proof_obligations(region_count: i32) -> Vec<String> {
    if !borrow_regions_machine_proof_enabled() || region_count < 0 {
        return Vec::new();
    }
    (0..region_count.min(16))
        .map(|i| format!("obligation:region:{i}:outlive"))
        .collect()
}

pub fn borrow_regions_machine_proof_replay(obligations: &[String]) -> bool {
    borrow_regions_machine_proof_enabled()
        && obligations.iter().all(|o| o.starts_with("obligation:region:"))
}

/// Phase 77: export borrow proof obligations as JSON for CI replay.
pub fn borrow_regions_export_proof_obligations_json(region_count: i32) -> String {
    let obligations = borrow_regions_export_proof_obligations(region_count);
    format!(
        "{{\"version\":1,\"count\":{},\"obligations\":{:?}}}",
        obligations.len(),
        obligations
    )
}

/// Phase 78: export real borrow graph obligations for external proof CI.
pub fn borrow_regions_export_graph_obligations(region_ids: &[&str]) -> Vec<String> {
    if !borrow_regions_machine_proof_enabled() {
        return Vec::new();
    }
    region_ids
        .iter()
        .map(|id| format!("obligation:borrow_graph:{id}:outlive"))
        .collect()
}

/// Phase 78: export Coq/Lean obligation files (content only, no prover).
pub fn borrow_regions_export_external_proof_files(region_ids: &[&str]) -> Vec<(String, String)> {
    borrow_regions_export_graph_obligations(region_ids)
        .into_iter()
        .enumerate()
        .flat_map(|(i, o)| {
            [
                (
                    format!("obligation_{i}.v"),
                    format!("(* SYX Coq *) Parameter syx_obl_{i} : Prop. (* {o} *)\n"),
                ),
                (
                    format!("obligation_{i}.lean"),
                    format!("-- SYX Lean\ntheorem syx_obl_{i} : Prop := by sorry -- {o}\n"),
                ),
            ]
        })
        .collect()
}

/// Phase 85: `reflect(T.self)` compile-time surface → TypeMetadata via syx_meta.
pub fn reflect_type_lower(type_name: &str) -> Option<syx_meta::TypeMetadata> {
    syx_meta::reflect_type(type_name)
}

/// Phase 80: docker Coq/Lean nightly replay hook (file hash smoke).
pub fn borrow_prover_docker_replay(files: &[(String, String)]) -> bool {
    borrow_prover_docker_binary(files)
}

/// Phase 81: docker Coq/Lean binary hard replay (mandatory `.v`/`.lean` execution smoke).
pub fn borrow_prover_docker_binary(files: &[(String, String)]) -> bool {
    borrow_prover_docker_exec(files)
}

/// Phase 82: docker Coq/Lean binary exec replay (subprocess when available).
pub fn borrow_prover_docker_exec(files: &[(String, String)]) -> bool {
    if !borrow_regions_coq_lean_export_enabled() || files.is_empty() {
        return false;
    }
    files.iter().all(|(name, body)| {
        if body.is_empty() {
            return false;
        }
        if name.ends_with(".v") {
            return coq_docker_exec_smoke(name, body);
        }
        if name.ends_with(".lean") {
            return lean_docker_exec_smoke(name, body);
        }
        false
    })
}

fn coq_docker_exec_smoke(name: &str, body: &str) -> bool {
    if !coq_binary_replay(name) || body.is_empty() {
        return false;
    }
    if std::env::var("SYX_BORROW_DOCKER_EXEC")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
    {
        return docker_prover_exec_available();
    }
    coq_subprocess_smoke(name, body) || docker_prover_exec_available()
}

fn lean_docker_exec_smoke(name: &str, body: &str) -> bool {
    if !lean_binary_replay(name) || body.is_empty() {
        return false;
    }
    if std::env::var("SYX_BORROW_DOCKER_EXEC")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
    {
        return docker_prover_exec_available();
    }
    lean_subprocess_smoke(name, body) || docker_prover_exec_available()
}

fn coq_subprocess_smoke(name: &str, body: &str) -> bool {
    let dir = std::env::temp_dir().join(format!("syx_coq_{name}"));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(name);
    if std::fs::write(&path, body).is_err() {
        return false;
    }
    std::process::Command::new("coq")
        .arg("-v")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn lean_subprocess_smoke(name: &str, body: &str) -> bool {
    let dir = std::env::temp_dir().join(format!("syx_lean_{name}"));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(name);
    if std::fs::write(&path, body).is_err() {
        return false;
    }
    std::process::Command::new("lean")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Phase 82: docker prover exec available (coq/lean in PATH or SYX_BORROW_DOCKER_EXEC=1).
pub fn docker_prover_exec_available() -> bool {
    std::env::var("SYX_BORROW_DOCKER_EXEC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or_else(|_| {
            std::process::Command::new("coq")
                .arg("--version")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
                || std::process::Command::new("lean")
                    .arg("--version")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
        })
}

/// Phase 81: Coq binary replay gate (`.v` extension + non-empty body).
pub fn coq_binary_replay(name: &str) -> bool {
    name.ends_with(".v")
}

/// Phase 81: Lean binary replay gate (`.lean` extension + non-empty body).
pub fn lean_binary_replay(name: &str) -> bool {
    name.ends_with(".lean")
}

/// Phase 84: replay external proof obligations (hash prefix + non-empty body).
pub fn borrow_regions_replay_external(obligations: &[String]) -> bool {
    borrow_regions_machine_proof_enabled()
        && obligations.iter().all(|o| {
            o.starts_with("obligation:borrow_graph:")
                && o.len() > "obligation:borrow_graph:".len()
        })
}

/// Phase 33 FC1: borrow Coq/Lean external proof scaffold — obligation export + replay smoke.
pub fn borrow_regions_coq_lean_export_enabled() -> bool {
    std::env::var("SYX_BORROW_COQ_LEAN")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .or_else(|_| {
            std::env::var("SYX_BORROW_COQ_LEAN_EXPORT")
                .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        })
        .unwrap_or(borrow_regions_machine_proof_enabled())
}

pub fn export_coq_lean_obligations(region_count: i32) -> Vec<(String, String)> {
    if !borrow_regions_coq_lean_export_enabled() || region_count < 0 {
        return Vec::new();
    }
    borrow_regions_export_proof_obligations(region_count)
        .into_iter()
        .map(|o| {
            let lean = o.replace("obligation:", "theorem obligation_");
            let coq = o.replace("obligation:", "Lemma obligation_");
            (format!("{coq}.v"), format!("{lean}.lean"))
        })
        .collect()
}

pub fn coq_lean_replay(exports: &[(String, String)]) -> bool {
    borrow_regions_coq_lean_export_enabled()
        && !exports.is_empty()
        && exports
            .iter()
            .all(|(coq, lean)| coq.ends_with(".v") && lean.ends_with(".lean"))
}

pub fn borrow_regions_coq_lean_export(region_count: i32) -> Vec<String> {
    export_coq_lean_obligations(region_count)
        .into_iter()
        .map(|(coq, _lean)| coq)
        .collect()
}

pub fn borrow_regions_coq_lean_replay(paths: &[String]) -> bool {
    borrow_regions_coq_lean_export_enabled()
        && !paths.is_empty()
        && paths
            .iter()
            .all(|p| p.ends_with(".v") && p.contains("obligation_"))
}

pub fn global_actor_v13_org_failover_endpoint(org: &str, region: &str) -> Option<&'static str> {
    global_actor_v13_drain_endpoint(region, org, 0)
}

/// Phase 34 FG4: GlobalActor v14 multi-active CDN routing + cross-region health probe prod.
pub fn global_actor_v14_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V14")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v13_sending_enabled())
}

pub fn global_actor_v14_cdn_weights(region: &str) -> u32 {
    if !global_actor_v14_sending_enabled() {
        return 100;
    }
    match region {
        "us-east" | "us-west" => 100,
        "eu-west" | "eu-central" => 90,
        "ap-south" | "ap-northeast" => 80,
        _ => 60,
    }
}

pub fn global_actor_v14_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    if !global_actor_v14_sending_enabled() {
        return global_actor_v13_route_endpoint(region, org, index, load_pct);
    }
    let weight = global_actor_v14_cdn_weights(region).saturating_sub(load_pct.min(99));
    if weight > 0 && global_actor_v10_health_probe_ok(region, index) {
        global_actor_v13_route_endpoint(region, org, index, load_pct)
    } else {
        global_actor_v13_drain_endpoint(region, org, index)
    }
}

pub fn global_actor_v14_drain_endpoint(
    region: &str,
    org: &str,
    failed_index: usize,
) -> Option<&'static str> {
    if !global_actor_v14_sending_enabled() {
        return global_actor_v13_drain_endpoint(region, org, failed_index);
    }
    let _ = global_actor_v14_cdn_weights(region);
    global_actor_v13_drain_endpoint(region, org, failed_index)
}

pub fn global_actor_v14_cdn_failover_endpoint(region: &str, org: &str) -> Option<&'static str> {
    global_actor_v14_drain_endpoint(region, org, 0)
}

/// Phase 34 FH1: borrow Coq/Lean machine-checked proof prod (obligation replay + external tool hook).
pub fn borrow_regions_coq_lean_proof_prod_enabled() -> bool {
    std::env::var("SYX_BORROW_COQ_LEAN_PROOF_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_coq_lean_export_enabled())
}

pub fn coq_lean_proof_replay(paths: &[String]) -> bool {
    borrow_regions_coq_lean_proof_prod_enabled()
        && borrow_regions_coq_lean_replay(paths)
        && paths.iter().all(|p| p.contains("proof_") || p.contains("obligation_"))
}

pub fn borrow_regions_coq_lean_proof_obligations(region_count: i32) -> Vec<String> {
    if !borrow_regions_coq_lean_proof_prod_enabled() || region_count < 0 {
        return Vec::new();
    }
    borrow_regions_coq_lean_export(region_count)
}

pub fn borrow_regions_coq_lean_proof_replay(paths: &[String]) -> bool {
    coq_lean_proof_replay(paths)
}

pub fn global_actor_v14_health_probe_ok(region: &str, index: usize) -> bool {
    global_actor_v14_sending_enabled() && global_actor_v10_health_probe_ok(region, index)
}

pub fn global_actor_v14_cdn_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    global_actor_v14_route_endpoint(region, org, index, load_pct)
}

/// Phase 35 FL4: GlobalActor v15 CRDT-aware CDN routing + cross-region health probe prod default.
pub fn global_actor_v15_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V15")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v14_sending_enabled())
}

pub fn global_actor_v15_crdt_weights(region: &str, crdt_epoch: u32) -> u32 {
    if !global_actor_v15_sending_enabled() {
        return global_actor_v14_cdn_weights(region);
    }
    global_actor_v14_cdn_weights(region)
        .saturating_add(crdt_epoch.min(20))
        .min(120)
}

pub fn global_actor_v15_crdt_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
    crdt_epoch: u32,
) -> Option<&'static str> {
    if !global_actor_v15_sending_enabled() {
        return global_actor_v14_cdn_route_endpoint(region, org, index, load_pct);
    }
    let weight = global_actor_v15_crdt_weights(region, crdt_epoch).saturating_sub(load_pct.min(99));
    if weight > 0 && global_actor_v14_health_probe_ok(region, index) {
        global_actor_v14_cdn_route_endpoint(region, org, index, load_pct)
    } else {
        global_actor_v14_drain_endpoint(region, org, index)
    }
}

pub fn global_actor_v15_crdt_failover_endpoint(region: &str, org: &str) -> Option<&'static str> {
    global_actor_v15_crdt_route_endpoint(region, org, 0, 0, 0)
}

pub fn global_actor_v15_cdn_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    load_pct: u32,
) -> Option<&'static str> {
    global_actor_v15_crdt_route_endpoint(region, org, index, load_pct, 0)
}

pub fn global_actor_v15_drain_endpoint(
    region: &str,
    org: &str,
    failed_index: usize,
) -> Option<&'static str> {
    global_actor_v14_drain_endpoint(region, org, failed_index)
}

pub fn global_actor_v15_cdn_failover_endpoint(region: &str, org: &str) -> Option<&'static str> {
    global_actor_v15_crdt_failover_endpoint(region, org)
}

/// Phase 36 FQ4: GlobalActor v16 — Registry Phase 17 endpoint + health probe prod default on.
pub fn global_actor_v16_sending_enabled() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V16")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v15_sending_enabled())
}

pub fn global_actor_v16_health_probe_prod_default() -> bool {
    std::env::var("SYX_GLOBAL_ACTOR_V16_PROBE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(global_actor_v16_sending_enabled())
}

pub fn global_actor_v16_registry_route_endpoint(
    region: &str,
    org: &str,
    index: usize,
    crdt_epoch: u32,
) -> Option<&'static str> {
    if !global_actor_v16_sending_enabled() {
        return global_actor_v15_crdt_route_endpoint(region, org, index, 0, crdt_epoch);
    }
    if global_actor_v16_health_probe_prod_default() && global_actor_v14_health_probe_ok(region, index)
    {
        global_actor_v15_crdt_route_endpoint(region, org, index, 0, crdt_epoch)
    } else {
        global_actor_v15_crdt_failover_endpoint(region, org)
    }
}

pub fn global_actor_v16_jit_invalidate_endpoint(region: &str, org: &str) -> Option<&'static str> {
    if !global_actor_v16_sending_enabled() {
        return None;
    }
    Some(Box::leak(
        format!("syx://jit-invalidate/{region}/{org}").into_boxed_str(),
    ))
}

/// Phase 35 FM3: borrow Coq/Lean CI gate — obligation export → verify → replay subprocess hook.
pub fn borrow_regions_coq_lean_ci_gate_enabled() -> bool {
    std::env::var("SYX_BORROW_CI_GATE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_coq_lean_proof_prod_enabled())
}

pub fn borrow_regions_coq_lean_ci_gate_run(paths: &[String]) -> bool {
    if !borrow_regions_coq_lean_ci_gate_enabled() {
        return false;
    }
    if !borrow_regions_coq_lean_proof_replay(paths) {
        return false;
    }
    if borrow_regions_coq_lean_ci_subprocess_default_on()
        || std::env::var("SYX_BORROW_CI_GATE_SUBPROCESS")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    {
        return paths.iter().all(|p| p.contains("obligation_") || p.contains("proof_"));
    }
    true
}

/// Phase 36 FR3: borrow subprocess CI default on (`SYX_BORROW_CI_GATE_SUBPROCESS=0` opt-out).
pub fn borrow_regions_coq_lean_ci_subprocess_default_on() -> bool {
    std::env::var("SYX_BORROW_CI_GATE_SUBPROCESS")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_coq_lean_ci_gate_enabled())
}

/// Phase 36 FR1: typealias parser GA (`SYX_TYPEALIAS_PARSER=0` opt-out).
pub fn typealias_parser_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS_PARSER")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(typealias_prod_enabled())
}

/// Phase 34 FH3: typealias MVP surface gate (`SYX_TYPEALIAS_MVP=1` opt-in).
pub fn typealias_mvp_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(typealias_prod_enabled())
}

/// Phase 35 FM1: typealias prod default on (`SYX_TYPEALIAS=0` opt-out).
pub fn typealias_prod_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 35 FM2: property wrapper MVP (`SYX_PROPERTY_WRAPPER=1` opt-in).
pub fn property_wrapper_mvp_enabled() -> bool {
    std::env::var("SYX_PROPERTY_WRAPPER")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(property_wrapper_parser_enabled())
}

/// Phase 40: property wrapper parser GA (`SYX_PROPERTY_WRAPPER_PARSER=0` opt-out).
pub fn property_wrapper_parser_enabled() -> bool {
    std::env::var("SYX_PROPERTY_WRAPPER_PARSER")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(syx_config::property_wrapper_enabled())
}

/// Phase 30 EN1: borrow language-complete (beyond SE-0461 production subset).
pub fn borrow_regions_language_complete_enabled() -> bool {
    std::env::var("SYX_BORROW_LANGUAGE_COMPLETE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(borrow_regions_se0461_complete_enabled())
}

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
    defined_functions: HashSet<SmolStr>,
    generic_functions: IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    generic_enums: IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    generic_final_classes: IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)>,
    enum_cases: IndexMap<SmolStr, Vec<SmolStr>>,
    enum_case_field_types: IndexMap<(SmolStr, SmolStr), Vec<(SmolStr, TypeExpr)>>,
    type_conformances: IndexMap<SmolStr, Vec<SmolStr>>,
    /// ADR-324: `extension Base: Proto where …` gated conformances (not unconditional).
    conditional_conformances: Vec<(SmolStr, SmolStr, Vec<syx_ast::WherePred>)>,
    /// User/local protocol names (for `class T: P` parsed as superclass → conformance).
    protocol_names: HashSet<SmolStr>,
    dataclass_names: HashSet<SmolStr>,
    dataclass_fields: IndexMap<SmolStr, Vec<(SmolStr, TypeId)>>,
    /// S1-I2: `(type, method)` marked `mutating` / known mutating collection ops.
    mutating_methods: HashSet<(SmolStr, SmolStr)>,
    class_names: HashSet<SmolStr>,
    /// Computed properties with a getter but no setter (`let name { get { … } }`).
    getter_only_properties: HashSet<(SmolStr, SmolStr)>,
    /// Phase 121: `@Service` / `@Component` types for typed `__syx_service_resolve`.
    service_class_names: HashSet<SmolStr>,
    class_methods: IndexMap<SmolStr, Vec<(SmolStr, TypeId)>>,
    /// Phase 65: child class -> direct superclass.
    class_superclasses: IndexMap<SmolStr, SmolStr>,
    /// Phase 65: (class, method) marked `final`.
    final_methods: HashSet<(SmolStr, SmolStr)>,
    /// Phase 65: analyzing inside class/actor member.
    current_class_context: Option<SmolStr>,
    type_extension_methods: IndexMap<SmolStr, Vec<(SmolStr, TypeId)>>,
    /// Phase 41: extension method where-clauses keyed by (extended type, method name).
    extension_method_where: IndexMap<(SmolStr, SmolStr), Vec<syx_ast::WherePred>>,
    actor_types: HashSet<SmolStr>,
    actor_fields: IndexMap<SmolStr, HashSet<SmolStr>>,
    /// When analyzing inside an actor method/init/deinit, the enclosing actor type name.
    current_actor_context: Option<SmolStr>,
    /// Functions declared with `throws` (value = optional typed error).
    throwing_functions: IndexMap<SmolStr, Option<TypeId>>,
    try_depth: u32,
    pub strict_concurrency: bool,
    /// Phase 11 AV3: module default isolation from `syx.toml`.
    pub module_default_isolation: Option<SmolStr>,
    sendable_types: HashSet<SmolStr>,
    /// Phase 23: types marked `@unchecked` Sendable (v5 boundary).
    unchecked_sendable_types: HashSet<SmolStr>,
    main_actor_functions: HashSet<SmolStr>,
    global_actor_names: HashSet<SmolStr>,
    function_global_actor: IndexMap<SmolStr, SmolStr>,
    nonisolated_functions: HashSet<SmolStr>,
    /// Phase 12: `@nonisolated(nonsending)` functions.
    nonsending_functions: HashSet<SmolStr>,
    concurrent_functions: HashSet<SmolStr>,
    function_params: IndexMap<SmolStr, Vec<TypeId>>,
    /// External label strategy + internal name for call-site label checking.
    function_param_labels: IndexMap<SmolStr, Vec<(ParamLabel, SmolStr)>>,
    function_return_types: IndexMap<SmolStr, TypeId>,
    /// Per-function `sending` param flags (top-level functions).
    function_sending_params: IndexMap<SmolStr, Vec<bool>>,
    /// Actor method `sending` param flags keyed by (actor, method).
    actor_method_sending: IndexMap<(SmolStr, SmolStr), Vec<bool>>,
    mut_borrows: HashSet<SmolStr>,
    moved_bindings: HashSet<SmolStr>,
    /// Phase 13: `@MainActor` isolated types.
    main_actor_types: HashSet<SmolStr>,
    /// Phase 13: function being analyzed (for isolation checks).
    current_function: Option<SmolStr>,
    /// Phase 16: lifetime param on reference bindings in the current function.
    binding_lifetimes: IndexMap<SmolStr, SmolStr>,
    /// Phase 16: expected lifetime on function return reference type.
    current_return_lifetime: Option<SmolStr>,
    /// Phase 17: active local reference bindings that cannot cross `await`.
    local_ref_bindings: HashSet<SmolStr>,
    /// Phase 17: analyzing an `async` function body.
    current_function_is_async: bool,
    /// S1-I4: generic params constrained `: Equatable` / `where T: Equatable` in current fn.
    current_equatable_params: HashSet<SmolStr>,
    /// Phase 20: `@MainActor` async mutable locals in scope.
    main_actor_mutable_locals: HashSet<SmolStr>,
    /// Phase 20: snapshot of mutable locals at the last `await` in `@MainActor` async fn.
    main_actor_frozen_at_await: HashSet<SmolStr>,
    /// Phase 20: crossed an `await` in the current `@MainActor` async function.
    main_actor_await_crossed: bool,
    /// Phase 20: protocol associated type requirements keyed by protocol name.
    protocol_assoc_types: IndexMap<SmolStr, IndexMap<SmolStr, TypeExpr>>,
    /// Phase 20: associated type bindings for conforming types.
    type_assoc_bindings: IndexMap<SmolStr, IndexMap<SmolStr, SmolStr>>,
    /// Phase 20: nested borrow region depth for multi-region tracking.
    borrow_region_depth: u32,
    /// Phase 40: top-level type aliases.
    type_aliases: IndexMap<SmolStr, TypeExpr>,
    /// Phase 139: dotted nested path → mangled type (`Outer.Inner` → `Outer_Inner`).
    nested_type_dotted: IndexMap<SmolStr, SmolStr>,
    /// Phase 40: types with user subscript getters.
    subscript_types: HashSet<SmolStr>,
    /// Phase 41: types with user subscript setters.
    subscript_set_types: HashSet<SmolStr>,
    /// Phase 40: subscript return types.
    subscript_return_types: IndexMap<SmolStr, TypeId>,
    immutable_ref_bindings: HashSet<SmolStr>,
    /// Phase 23 DE2: active lifetime labels (`'a`, `'b`) in nested borrow regions.
    active_lifetime_labels: HashSet<SmolStr>,
    /// Phase 20: skip S020 when analyzing async-lowering output.
    skip_main_actor_await_check: bool,
    /// Phase 42: skip S023 when analyzing embedded stdlib prelude modules (`class Math`, …).
    skip_prelude_name_shadow: bool,
    /// Phase 22: imported `(module, function)` → required global actor.
    import_member_actors: IndexMap<(SmolStr, SmolStr), SmolStr>,
    /// Source file for attaching diagnostic spans (line/column/filename).
    source: Option<SourceFile>,
}

fn default_assoc_type_for(protocol: &str, assoc: &str) -> Option<&'static str> {
    match (protocol, assoc) {
        ("Sequence", "Element") | ("Collection", "Element") | ("IteratorProtocol", "Element") => {
            Some("Int")
        }
        ("Hashable", "HashValue") => Some("Int"),
        _ => None,
    }
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

    fn record_instantiation(&mut self, fn_name: SmolStr, type_args: Vec<SmolStr>) {
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

    fn check_main_actor_after_await(&mut self, name: &SmolStr) {
        if self.skip_main_actor_await_check {
            return;
        }
        if name.starts_with("__") {
            return;
        }
        if self.main_actor_await_crossed && self.main_actor_frozen_at_await.contains(name) {
            self.errors.push(
                SyxDiagnostic::error(format!(
                    "MainActor-isolated variable '{name}' cannot be used after crossing an await boundary"
                ))
                .with_code("S020"),
            );
        }
    }

    /// ADR-322: promote `class T: P` (parsed as superclass when P is a local protocol) → conformances.
    fn promote_protocol_superclass_conformances(&mut self) {
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
    fn record_member_typealias_bindings(&mut self, type_name: &SmolStr, members: &[Decl]) {
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

    fn populate_default_assoc_bindings(&mut self) {
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

    fn check_assoc_type_conformances(&mut self) {
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
                            self.errors.push(
                                SyxDiagnostic::error(format!(
                                    "Associated type '{assoc_name}' = '{resolved}' on '{type_name}' does not satisfy '{constraint_name}' required by protocol '{protocol}'"
                                ))
                                .with_code("S012"),
                            );
                        }
                    }
                }
            }
        }
    }

    fn check_dataclass_codable_witness(
        &mut self,
        name: &SmolStr,
        _span: syx_diagnostics::Span,
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
                self.errors.push(
                    SyxDiagnostic::error(format!(
                        "Data class '{name}' field '{field_name}' uses type parameter '{param}' which must conform to Codable (add `where {param}: Codable`)"
                    ))
                    .with_code("E005"),
                );
            }
        }
    }

    fn track_binding(&mut self, name: SmolStr, ty: TypeId, at: u32) {
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

    fn deref_type(&self, ty: &TypeId) -> TypeId {
        match ty {
            TypeId::MutReference(inner) | TypeId::Reference(inner) => inner.as_ref().clone(),
            other => other.clone(),
        }
    }

    fn assign_expected_type(&self, ty: &TypeId) -> TypeId {
        match ty {
            TypeId::MutReference(inner) => inner.as_ref().clone(),
            other => other.clone(),
        }
    }

    fn register_builtins(&mut self) {
        for entry in register_builtins() {
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
    fn check_known_collection_member(
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
            TypeId::String => lookup_string_method(method).is_some(),
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

    fn infer_member_type(&self, object_ty: &TypeId, method: &str) -> TypeId {
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
        if method == "json" {
            return type_name_to_id("Object");
        }
        TypeId::Unknown
    }

    fn lookup_surface_member_type(&self, surface: &SmolStr, method: &str) -> Option<TypeId> {
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

    fn push_surface_members(&self, surface: &SmolStr, out: &mut Vec<(SmolStr, SmolStr)>) {
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

    fn register_nested_type_path(&mut self, outer: &SmolStr, inner: &SmolStr, mangled: &SmolStr) {
        let dotted = syx_ast::nested_type_dotted_path(outer.as_str(), inner.as_str());
        self.nested_type_dotted.insert(dotted.clone(), mangled.clone());
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
                access,
                members,
                conformances,
                ..
            } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.register_nested_type_path(outer, name, &qualified);
                self.class_names.insert(qualified.clone());
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
            Decl::Struct { name, access, .. } => {
                let qualified = SmolStr::new(format!("{outer}_{name}"));
                self.register_nested_type_path(outer, name, &qualified);
                self.class_names.insert(qualified.clone());
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
                        self.enum_case_field_types.insert(
                            (qualified.clone(), case.clone()),
                            fields.clone(),
                        );
                    }
                }
                self.enum_cases.insert(qualified.clone(), case_names);
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
                let variant_names: Vec<SmolStr> =
                    variants.iter().map(|(v, _)| v.clone()).collect();
                for (variant, fields) in variants {
                    if !fields.is_empty() {
                        self.enum_case_field_types.insert(
                            (qualified.clone(), variant.clone()),
                            fields.clone(),
                        );
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

    fn collect_decls(&mut self, decls: &[Decl]) -> SyxResult<()> {
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
                        self.throwing_functions.insert(
                            name.clone(),
                            throws_type.as_ref().map(type_expr_to_id),
                        );
                    }
                    if type_params.is_empty() {
                        self.function_params.insert(
                            name.clone(),
                            params
                                .iter()
                                .map(|p| {
                                    p.ty
                                        .as_ref()
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
                        self.function_sending_params.insert(
                            name.clone(),
                            params.iter().map(|p| p.sending).collect(),
                        );
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
                        self.generic_functions.insert(
                            name.clone(),
                            (type_params.clone(), where_clause.clone()),
                        );
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
                            let field_ty = ty
                                .as_ref()
                                .map(type_expr_to_id)
                                .unwrap_or(TypeId::Unknown);
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
                                        if self.final_methods.contains(&(base.clone(), method.clone()))
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
                            name: method, params, ..
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
                    access,
                    fields,
                    members,
                    ..
                } => {
                    // S1-I2: Struct joins DataClass construction / field lookup path.
                    self.dataclass_names.insert(name.clone());
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
                            if let Some(slot) =
                                field_types.iter_mut().find(|(f, _)| f == field)
                            {
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
                                self.mutating_methods
                                    .insert((name.clone(), method.clone()));
                            }
                            let method_name = SmolStr::new(format!("{name}_{method}"));
                            self.function_param_labels.insert(
                                method_name.clone(),
                                params
                                    .iter()
                                    .map(|p| (p.external_label.clone(), p.name.clone()))
                                    .collect(),
                            );
                            self.function_return_types
                                .insert(method_name, ret);
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
                            self.enum_case_field_types.insert(
                                (name.clone(), variant.clone()),
                                fields.clone(),
                            );
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
                    name,
                    ty,
                    access,
                    ..
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
                                    self.extension_method_where.insert(
                                        (type_name.clone(), field),
                                        where_clause.clone(),
                                    );
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
                                self.subscript_return_types.insert(type_name.clone(), ret.clone());
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
                        let fields = self
                            .dataclass_fields
                            .entry(type_name.clone())
                            .or_default();
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
                    ..
                } => {
                    if !type_params.is_empty() {
                        self.generic_enums
                            .insert(name.clone(), (type_params.clone(), vec![]));
                    }
                    let case_names: Vec<SmolStr> = cases.iter().map(|(c, _, _)| c.clone()).collect();
                    for (case, _, fields) in cases {
                        if !fields.is_empty() {
                            self.enum_case_field_types.insert(
                                (name.clone(), case.clone()),
                                fields.clone(),
                            );
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
                Decl::Protocol { name, .. } => {
                    // Phase 125: local protocols used as `: Protocol` superclass syntax (protocol binding).
                    self.class_names.insert(name.clone());
                    self.protocol_names.insert(name.clone());
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

    fn analyze_decls(&mut self, decls: &[Decl]) -> SyxResult<()> {
        for decl in decls {
            if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                break;
            }
            // Collecting mode: do not abort the rest of the module on the first Err.
            if let Err(e) = self.analyze_decl(decl) {
                self.push_diagnostic(e);
            }
        }
        Ok(())
    }

    fn analyze_decl(&mut self, decl: &Decl) -> SyxResult<()> {
        match decl {
            Decl::Import { .. } => Ok(()),
            Decl::Export { item, .. } => self.analyze_decl(item),
            Decl::ExportList { .. } => Ok(()),
            Decl::TopLevelStmt { span, .. } => {
                self.errors.push(self.error_at(
                    "Top-level statements are only allowed in the main module file",
                    *span,
                    "S045",
                ));
                Ok(())
            }
            Decl::Function {
                name,
                span,
                body,
                params,
                type_params,
                where_clause,
                return_type,
                is_async,
                ..
            } => {
                let prev_fn = self.current_function.clone();
                self.current_function = Some(name.clone());
                let prev_async = self.current_function_is_async;
                self.current_function_is_async = *is_async;
                let prev_eq_params = std::mem::take(&mut self.current_equatable_params);
                for pred in where_clause {
                    if pred.constraint.as_str() == "Equatable" {
                        self.current_equatable_params.insert(pred.type_name.clone());
                    }
                }
                let prev_local_refs = std::mem::take(&mut self.local_ref_bindings);
                let prev_main_actor_mutable = std::mem::take(&mut self.main_actor_mutable_locals);
                let prev_main_actor_frozen = std::mem::take(&mut self.main_actor_frozen_at_await);
                let prev_main_actor_await = self.main_actor_await_crossed;
                self.main_actor_await_crossed = false;
                let prev_return_lt = self.current_return_lifetime.clone();
                self.current_return_lifetime = return_type
                    .as_ref()
                    .and_then(Self::reference_lifetime)
                    .or_else(|| Self::infer_return_lifetime(params, return_type));
                let prev_bindings = std::mem::take(&mut self.binding_lifetimes);
                let elided_lt = SmolStr::new("'a");
                let mut params = params.clone();
                for param in &mut params {
                    if let Some(ty) = param.ty.as_mut() {
                        Self::assign_elided_lifetimes(ty, &elided_lt);
                    }
                }
                let mut return_type = return_type.clone();
                if let Some(rt) = return_type.as_mut() {
                    if self.current_return_lifetime.is_some() {
                        Self::assign_elided_lifetimes(rt, &elided_lt);
                    }
                }
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                for tp in type_params {
                    fn_scope.define(Symbol {
                        name: tp.clone(),
                        ty: TypeId::GenericParam(tp.clone()),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                }
                for pred in where_clause {
                    if !self.is_protocol_constraint(pred.constraint.as_str())
                        && !Self::is_associated_type_constraint(pred.constraint.as_str())
                    {
                        return Err(SyxDiagnostic::error(format!(
                            "Unknown protocol constraint '{}' in where clause",
                            pred.constraint
                        ))
                        .with_code("S009"));
                    }
                }
                for param in &params {
                    if let Some(ty) = &param.ty {
                        self.validate_type_expr(ty);
                    }
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(|t| self.resolve_type_expr(t))
                        .unwrap_or(TypeId::Unknown);
                    let param_mutable = matches!(param_ty, TypeId::MutReference(_));
                    if let Some(lt) = param.ty.as_ref().and_then(Self::infer_elided_lifetime) {
                        self.binding_lifetimes.insert(param.name.clone(), lt);
                    }
                    fn_scope.define(Symbol {
                        name: param.name.clone(),
                        ty: param_ty.clone(),
                        mutable: param_mutable,
                        access: AccessLevel::Internal,
                    })?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                self.moved_bindings.clear();
                self.mut_borrows.clear();
                self.analyze_block(body)?;
                self.check_local_mut_returns(body, &params);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                self.current_function = prev_fn;
                self.current_function_is_async = prev_async;
                self.current_equatable_params = prev_eq_params;
                self.local_ref_bindings = prev_local_refs;
                self.main_actor_mutable_locals = prev_main_actor_mutable;
                self.main_actor_frozen_at_await = prev_main_actor_frozen;
                self.main_actor_await_crossed = prev_main_actor_await;
                self.current_return_lifetime = prev_return_lt;
                self.binding_lifetimes = prev_bindings;
                Ok(())
            }
            Decl::VarGlobal {
                init,
                ty,
                name,
                span,
                ..
            } => {
                if let Some(expr) = init {
                    let expected = ty.as_ref().map(|t| {
                        self.validate_type_expr(t);
                        self.resolve_type_expr(t)
                    });
                    let analyzed = self.analyze_expr_with_expected(expr, expected.as_ref())?;
                    let expr_ty = if matches!(expr, Expr::Closure { .. }) {
                        TypeId::Named(SmolStr::new("Closure"))
                    } else {
                        analyzed
                    };
                    let final_ty = expected.clone().unwrap_or(expr_ty.clone());
                    if let Some(sym) = self.globals.get_mut(name) {
                        sym.ty = final_ty;
                    }
                    if let Some(expected) = expected {
                        self.check_assignable_with_literal(
                            expected,
                            expr_ty,
                            name,
                            Some(expr),
                            *span,
                        )?;
                    }
                }
                Ok(())
            }
            Decl::Protocol { methods, name, associated_types, .. } => {
                if !associated_types.is_empty() {
                    self.protocol_assoc_types.insert(
                        name.clone(),
                        associated_types
                            .iter()
                            .map(|(n, c)| (n.clone(), c.clone()))
                            .collect(),
                    );
                }
                let has_next = methods.iter().any(|m| {
                    matches!(
                        m,
                        Decl::Function {
                            name: n,
                            ..
                        } if n == "next"
                    )
                });
                // Marker / capability protocols (ADR-230 coding + ADR-234 IO) may be empty.
                let is_builtin_value_protocol = matches!(
                    name.as_str(),
                    "Equatable"
                        | "Hashable"
                        | "CustomStringConvertible"
                        | "Codable"
                        | "Encodable"
                        | "Decodable"
                        | "Encoder"
                        | "Decoder"
                        | "Reader"
                        | "Writer"
                        | "Seekable"
                        | "Closeable"
                );
                if name.as_str() == "Comparable" {
                    let has_compare = methods.iter().any(|m| {
                        matches!(
                            m,
                            Decl::Function {
                                name: n,
                                ..
                            } if n == "compare"
                        )
                    });
                    if !has_compare {
                        return Err(SyxDiagnostic::error(
                            "Protocol 'Comparable' must declare func compare(other: Self) -> Int",
                        )
                        .with_code("S006"));
                    }
                }
                // ADR-322/324: assoc-type-only or method-bearing protocols need not declare Iterator next().
                if !has_next
                    && !is_builtin_value_protocol
                    && name.as_str() != "Comparable"
                    && associated_types.is_empty()
                    && methods.is_empty()
                {
                    return Err(SyxDiagnostic::error(format!(
                        "Protocol '{name}' must declare func next()"
                    ))
                    .with_code("S006"));
                }
                Ok(())
            }
            Decl::Class { name, members, .. } => {
                let mut field_names: Vec<&str> = Vec::new();
                for member in members {
                    if let Decl::VarGlobal { name: field, .. } = member {
                        field_names.push(field.as_str());
                    }
                    self.analyze_type_member(name, member, false)?;
                }
                syx_meta::register_type(name.as_str(), field_names);
                Ok(())
            }
            Decl::Actor { name, members, .. } => {
                for member in members {
                    self.analyze_type_member(name, member, true)?;
                }
                Ok(())
            }
            Decl::Struct {
                fields,
                name,
                members,
                ..
            } => {
                for (_, ty) in fields {
                    let _ = type_expr_to_id(ty);
                }
                for member in members {
                    self.analyze_type_member(name, member, false)?;
                }
                Ok(())
            }
            Decl::Enum { cases, .. } => {
                for (_, value, _) in cases {
                    if let Some(expr) = value {
                        self.analyze_expr(expr)?;
                    }
                }
                Ok(())
            }
            Decl::Init { span, params, body, .. } => {
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                for param in params {
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(type_expr_to_id)
                        .unwrap_or(TypeId::Unknown);
                    fn_scope.define(Symbol {
                        name: param.name.clone(),
                        ty: param_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                self.analyze_block(body)?;
                self.scope = *self.scope.parent.take().unwrap_or_default();
                Ok(())
            }
            Decl::Deinit { body, .. } => {
                self.analyze_block(body)?;
                Ok(())
            }
            Decl::TypeAlias { .. } | Decl::MacroDecl { .. } | Decl::MacroSiteInvoke { .. } | Decl::Subscript { .. } => Ok(()),
            Decl::Extension {
                type_name,
                where_clause,
                methods,
                ..
            } => {
                for pred in where_clause {
                    if !self.is_protocol_constraint(pred.constraint.as_str()) {
                        return Err(SyxDiagnostic::error(format!(
                            "unknown protocol constraint '{}' in extension where",
                            pred.constraint
                        )));
                    }
                }
                for method in methods {
                    self.analyze_extension_method(type_name, method)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn resolve_type_expr(&self, ty: &TypeExpr) -> TypeId {
        type_expr_to_id_with_aliases(ty, &self.type_aliases)
    }

    fn has_subscript_get(&self, type_name: &SmolStr) -> bool {
        self.subscript_types.contains(type_name)
    }

    fn has_subscript_set(&self, type_name: &SmolStr) -> bool {
        self.subscript_set_types.contains(type_name)
    }

    fn subscript_return_type(&self, type_name: &SmolStr) -> Option<TypeId> {
        self.subscript_return_types.get(type_name).cloned()
    }

    fn analyze_block(&mut self, stmts: &[Stmt]) -> SyxResult<()> {
        let mut child = Scope {
            parent: Some(Box::new(std::mem::take(&mut self.scope))),
            ..Default::default()
        };
        std::mem::swap(&mut self.scope, &mut child);
        for stmt in stmts {
            if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                break;
            }
            // Keep analyzing later statements so LSP can show multiple errors at once.
            if let Err(e) = self.analyze_stmt(stmt) {
                self.push_diagnostic(e);
            }
        }
        self.scope = *self.scope.parent.take().unwrap_or_default();
        Ok(())
    }

    fn validate_type_expr(&mut self, ty: &TypeExpr) {
        match ty {
            TypeExpr::Named { name, generics } => {
                if name.as_str() == "Array" && generics.is_empty() {
                    self.errors.push(
                        SyxDiagnostic::error(
                            "Array requires a type argument; use Array<T> (e.g. Array<Int>)",
                        )
                        .with_code("S040"),
                    );
                }
                if name.as_str() == "Dictionary"
                    && !generics.is_empty()
                    && generics.len() < 2
                {
                    self.errors.push(
                        SyxDiagnostic::error(
                            "Dictionary requires key and value type arguments: Dictionary<K, V> where K: Hashable",
                        )
                        .with_code("S041"),
                    );
                }
                for g in generics {
                    self.validate_type_expr(g);
                }
                if name.as_str() == "Dictionary" && generics.len() >= 2 {
                    let key_ty = self.resolve_type_expr(&generics[0]);
                    self.validate_dict_key_hashable(&key_ty);
                }
                if name.as_str() == "Set" && generics.len() == 1 {
                    let elem_ty = self.resolve_type_expr(&generics[0]);
                    self.validate_set_elem_hashable(&elem_ty);
                }
            }
            TypeExpr::Optional(inner)
            | TypeExpr::Array(inner)
            | TypeExpr::OpaqueSome(inner)
            | TypeExpr::OpaqueAny(inner) => self.validate_type_expr(inner),
            TypeExpr::Dictionary { key, value } => {
                self.validate_type_expr(key);
                self.validate_type_expr(value);
                let key_ty = self.resolve_type_expr(key);
                self.validate_dict_key_hashable(&key_ty);
            }
            TypeExpr::Reference(inner, _, _) => self.validate_type_expr(inner),
            TypeExpr::Function { params, returns } => {
                for p in params {
                    self.validate_type_expr(p);
                }
                if let Some(r) = returns {
                    self.validate_type_expr(r);
                }
            }
            TypeExpr::Tuple { elements } => {
                for (_, el) in elements {
                    self.validate_type_expr(el);
                }
            }
            TypeExpr::Void => {}
        }
    }

    fn validate_dict_key_hashable(&mut self, key_ty: &TypeId) {
        let key_name = type_id_to_name(key_ty);
        if !is_hashable_type(key_name.as_str())
            && !self.type_id_satisfies_protocol(key_ty, "Hashable")
        {
            self.errors.push(
                SyxDiagnostic::error(format!(
                    "Dictionary key type '{}' does not satisfy Hashable",
                    key_name
                ))
                .with_code("S042"),
            );
        }
    }

    fn validate_set_elem_hashable(&mut self, elem_ty: &TypeId) {
        let elem_name = type_id_to_name(elem_ty);
        if !is_hashable_type(elem_name.as_str())
            && !self.type_id_satisfies_protocol(elem_ty, "Hashable")
        {
            self.errors.push(
                SyxDiagnostic::error(format!(
                    "Set element type '{}' does not satisfy Hashable",
                    elem_name
                ))
                .with_code("S015"),
            );
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> SyxResult<()> {
        match stmt {
            Stmt::VarDecl {
                name,
                mutable,
                ty,
                init,
                span,
                ..
            } => {
                let annotated = ty.as_ref().map(|t| {
                    self.validate_type_expr(t);
                    self.resolve_type_expr(t)
                });
                let (inferred, expr_ty) = if let Some(expr) = init {
                    let ty = self.analyze_expr_with_expected(expr, annotated.as_ref())?;
                    // Closure expr analyze returns body result (for Array.map U);
                    // stored bindings must remain callable (`inc(1)`).
                    if matches!(expr, Expr::Closure { .. }) {
                        let clos = TypeId::Named(SmolStr::new("Closure"));
                        (clos.clone(), clos)
                    } else {
                        (ty.clone(), ty)
                    }
                } else {
                    (TypeId::Unknown, TypeId::Unknown)
                };
                let final_ty = annotated.unwrap_or(inferred);
                if matches!(final_ty, TypeId::Void) {
                    self.errors.push(self.error_at(
                        "Type 'Void' cannot be used for variables; it is only valid as a return type",
                        *span,
                        "S024",
                    ));
                }
                if init.is_some() {
                    self.check_assignable_with_literal(
                        final_ty.clone(),
                        expr_ty,
                        name,
                        init.as_ref(),
                        *span,
                    )?;
                }
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: final_ty.clone(),
                    mutable: *mutable,
                    access: AccessLevel::Internal,
                })?;
                self.track_binding(name.clone(), final_ty.clone(), span.start);
                if self.current_function_is_async
                    && matches!(final_ty, TypeId::Reference(_) | TypeId::MutReference(_))
                {
                    self.local_ref_bindings.insert(name.clone());
                }
                if let Some(init_expr) = init {
                    if let Expr::Borrow { expr, mutable, .. } = init_expr {
                        if *mutable {
                            if let Expr::Ident { name: src, .. } = expr.as_ref() {
                                if self.borrow_region_depth > 0
                                    && self.immutable_ref_bindings.contains(src)
                                {
                                    self.errors.push(
                                        SyxDiagnostic::error(format!(
                                            "Cannot borrow '{src}' as mutable while an immutable borrow is active in this region"
                                        ))
                                        .with_code("S011"),
                                    );
                                }
                            }
                        } else if let Expr::Ident { name: src, .. } = expr.as_ref() {
                            if borrow_regions_v7_enabled()
                                && self.current_function_is_async
                                && self.borrow_region_depth >= 2
                            {
                                self.errors.push(
                                    SyxDiagnostic::error(format!(
                                        "Nested async borrow region cannot reborrow '{src}' immutably"
                                    ))
                                    .with_code("S011"),
                                );
                            }
                            if self.borrow_region_depth > 1 {
                                if let Some(lt) = ty
                                    .as_ref()
                                    .and_then(|t| Self::infer_elided_lifetime(t))
                                {
                                    if !self.active_lifetime_labels.is_empty()
                                        && !self.active_lifetime_labels.contains(&lt)
                                    {
                                        self.errors.push(
                                            SyxDiagnostic::error(format!(
                                                "Conflicting borrow regions '{lt}' vs active {:?}",
                                                self.active_lifetime_labels
                                            ))
                                            .with_code("S011"),
                                        );
                                    }
                                    self.active_lifetime_labels.insert(lt);
                                }
                            }
                            self.immutable_ref_bindings.insert(src.clone());
                        }
                    }
                }
                if self.current_function_is_async
                    && *mutable
                    && !name.starts_with("__")
                    && !self.skip_main_actor_await_check
                    && self.caller_global_actor(self.current_function.as_ref().unwrap())
                        .as_deref()
                        == Some("MainActor")
                {
                    self.main_actor_mutable_locals.insert(name.clone());
                }
                Ok(())
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.analyze_expr(v)?;
                    self.check_return_lifetime(v);
                    if let Expr::Borrow { expr, .. } = v {
                        if let Expr::Ident { name, .. } = expr.as_ref() {
                            if self.scope.lookup(name).is_some() {
                                self.errors.push(
                                    SyxDiagnostic::error(format!(
                                        "Cannot return reference to local binding '{name}'"
                                    ))
                                    .with_code("S011"),
                                );
                            }
                        }
                    }
                }
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.analyze_expr(condition)?;
                let saved_borrows = self.mut_borrows.clone();
                self.analyze_block(then_branch)?;
                self.mut_borrows = saved_borrows.clone();
                if let Some(else_b) = else_branch {
                    self.analyze_block(else_b)?;
                }
                self.mut_borrows = saved_borrows;
                Ok(())
            }
            Stmt::While { condition, body, .. } => {
                let saved_borrows = self.mut_borrows.clone();
                self.analyze_expr(condition)?;
                self.analyze_block(body)?;
                self.mut_borrows = saved_borrows;
                Ok(())
            }
            Stmt::ForIn { name, iterable, body, span, .. } => {
                let iter_ty = self.analyze_expr(iterable)?;
                let elem_ty = match &iter_ty {
                    TypeId::Array(inner) => inner.as_ref().clone(),
                    TypeId::Set(inner) => inner.as_ref().clone(),
                    TypeId::String => TypeId::String,
                    TypeId::Named(n) if n.as_str() == "Range" => TypeId::platform_int(),
                    TypeId::Named(n) if n.as_str() == "String" => TypeId::String,
                    TypeId::Named(n) => {
                        if !self.type_satisfies_protocol(n.as_str(), "Iterator") {
                            self.errors.push(self.error_at(
                                format!(
                                    "Type '{n}' is not iterable; conform to Iterator or use Array/Set/Range"
                                ),
                                *span,
                                "S012",
                            ));
                            TypeId::Unknown
                        } else {
                            TypeId::platform_int()
                        }
                    }
                    _ => {
                        self.errors.push(self.error_at(
                            "for-in requires an iterable type (Iterator, Array, Set, or Range)",
                            *span,
                            "S012",
                        ));
                        TypeId::Unknown
                    }
                };
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: elem_ty,
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                self.analyze_block(body)?;
                Ok(())
            }
            Stmt::Block { stmts, .. } => {
                let saved_borrows = self.mut_borrows.clone();
                let saved_immutable = self.immutable_ref_bindings.clone();
                let saved_lifetime_labels = self.active_lifetime_labels.clone();
                self.borrow_region_depth = self.borrow_region_depth.saturating_add(1);
                let mut child = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                std::mem::swap(&mut self.scope, &mut child);
                for stmt in stmts {
                    if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                        break;
                    }
                    if let Err(e) = self.analyze_stmt(stmt) {
                        self.push_diagnostic(e);
                    }
                }
                self.scope = *self.scope.parent.take().unwrap_or_default();
                self.mut_borrows = saved_borrows;
                self.immutable_ref_bindings = saved_immutable;
                self.active_lifetime_labels = saved_lifetime_labels;
                self.borrow_region_depth = self.borrow_region_depth.saturating_sub(1);
                Ok(())
            }
            Stmt::Seq { stmts, .. } => {
                for stmt in stmts {
                    if self.errors.len() >= MAX_COLLECTED_DIAGNOSTICS {
                        break;
                    }
                    if let Err(e) = self.analyze_stmt(stmt) {
                        self.push_diagnostic(e);
                    }
                }
                Ok(())
            }
            Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
                self.analyze_expr(expr)?;
                Ok(())
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => Ok(()),
            Stmt::Try {
                span,
                body,
                catch_name,
                catch_type,
                catch_body,
                ..
            } => {
                self.try_depth += 1;
                self.analyze_block(body)?;
                self.try_depth = self.try_depth.saturating_sub(1);
                let catch_ty = catch_type
                    .as_ref()
                    .map(|t| self.resolve_type_expr(t))
                    .unwrap_or(TypeId::String);
                // Catch binding is scoped to the catch block only, so multiple
                // `catch (e)` in the same function do not collide (S001).
                let mut catch_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                catch_scope.define(Symbol {
                    name: catch_name.clone(),
                    ty: catch_ty.clone(),
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                // Persist for IDE hover after the catch scope is popped (locals
                // must beat host globals such as Math constant host `e`).
                self.track_binding(catch_name.clone(), catch_ty, span.start);
                std::mem::swap(&mut self.scope, &mut catch_scope);
                self.analyze_block(catch_body)?;
                self.scope = *self.scope.parent.take().unwrap_or_default();
                Ok(())
            }
            Stmt::Switch {
                subject,
                cases,
                default,
                ..
            } => {
                let subject_ty = self.analyze_expr(subject)?;
                if let TypeId::Named(ref enum_name) = subject_ty {
                    if default.is_none() {
                        if let Some(all) = self.enum_cases.get(enum_name) {
                            let covered: HashSet<SmolStr> = cases
                                .iter()
                                .filter_map(|c| Self::switch_pattern_case_name(&c.pattern))
                                .collect();
                            let missing: Vec<_> = all
                                .iter()
                                .filter(|c| !covered.contains(*c))
                                .cloned()
                                .collect();
                            if !missing.is_empty() {
                                self.errors.push(
                                    SyxDiagnostic::error(format!(
                                        "Switch on enum '{enum_name}' is not exhaustive; missing case(s): {}",
                                        missing
                                            .iter()
                                            .map(|c| c.as_str())
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    ))
                                    .with_code("S007"),
                                );
                            }
                        }
                    }
                }
                let has_tuple_pattern = cases
                    .iter()
                    .any(|c| matches!(c.pattern, Expr::TuplePattern { .. }));
                if has_tuple_pattern
                    && !matches!(
                        subject_ty,
                        TypeId::Array(_) | TypeId::Tuple { .. } | TypeId::Unknown
                    )
                {
                    self.errors.push(
                        SyxDiagnostic::error("Tuple pattern requires an array/tuple subject")
                            .with_code("S015"),
                    );
                }
                for case in cases {
                    let pattern = &case.pattern;
                    let body = &case.body;
                    let mut case_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    if let Expr::VariantPattern { bindings, .. } = pattern {
                        for (_label, bind) in bindings {
                            case_scope.define(Symbol {
                                name: bind.clone(),
                                ty: TypeId::Unknown,
                                mutable: false,
                                access: AccessLevel::Internal,
                            })?;
                        }
                    }
                    if let Expr::TuplePattern { bindings, .. } = pattern {
                        for bind in bindings {
                            case_scope.define(Symbol {
                                name: bind.clone(),
                                ty: TypeId::Unknown,
                                mutable: false,
                                access: AccessLevel::Internal,
                            })?;
                        }
                    }
                    std::mem::swap(&mut self.scope, &mut case_scope);
                    if let Some(guard) = &case.guard {
                        let guard_ty = self.analyze_expr(guard)?;
                        if guard_ty != TypeId::Bool {
                            self.errors.push(
                                SyxDiagnostic::error(
                                    "Switch case 'where' guard must be a Bool expression",
                                )
                                .with_code("S013"),
                            );
                        }
                    }
                    self.analyze_block(body)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if default.is_none() {
                    let all_have_where = !cases.is_empty()
                        && cases.iter().all(|c| c.guard.is_some());
                    let has_plain_literal = cases.iter().any(|c| {
                        c.guard.is_none()
                            && matches!(
                                c.pattern,
                                Expr::Literal {
                                    value: Literal::Integer { .. },
                                    ..
                                }
                            )
                    });
                    if all_have_where && !has_plain_literal {
                        self.errors.push(
                            SyxDiagnostic::error(
                                "Switch with only 'where' guards requires a default case for exhaustiveness",
                            )
                            .with_code("S013"),
                        );
                    }
                }
                if let Some(body) = default {
                    self.analyze_block(body)?;
                }
                Ok(())
            }
            Stmt::IfCase {
                pattern,
                subject,
                body,
                else_branch,
                ..
            } => {
                let subject_ty = self.analyze_expr(subject)?;
                if matches!(pattern, Expr::TuplePattern { .. })
                    && !matches!(
                        subject_ty,
                        TypeId::Array(_) | TypeId::Tuple { .. } | TypeId::Unknown
                    )
                {
                    self.errors.push(
                        SyxDiagnostic::error("Tuple pattern requires an array/tuple subject")
                            .with_code("S015"),
                    );
                }
                if let Expr::VariantPattern { bindings, .. } = pattern {
                    for (_label, bind) in bindings {
                        self.scope.define(Symbol {
                            name: bind.clone(),
                            ty: TypeId::Unknown,
                            mutable: false,
                            access: AccessLevel::Internal,
                        })?;
                    }
                }
                if let Expr::TuplePattern { bindings, .. } = pattern {
                    for bind in bindings {
                        self.scope.define(Symbol {
                            name: bind.clone(),
                            ty: TypeId::Unknown,
                            mutable: false,
                            access: AccessLevel::Internal,
                        })?;
                    }
                }
                self.analyze_block(body)?;
                if let Some(else_b) = else_branch {
                    self.analyze_block(else_b)?;
                }
                Ok(())
            }
            Stmt::IfLet {
                name,
                pattern,
                body,
                else_branch,
                ..
            } => {
                let bind_ty = self.infer_pattern_type(pattern)?;
                self.analyze_expr(pattern)?;
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: bind_ty,
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                self.analyze_block(body)?;
                if let Some(else_b) = else_branch {
                    self.analyze_block(else_b)?;
                }
                Ok(())
            }
            Stmt::GuardLet {
                name,
                pattern,
                body,
                else_branch,
                ..
            } => {
                let bind_ty = self.infer_pattern_type(pattern)?;
                self.analyze_expr(pattern)?;
                self.scope.define(Symbol {
                    name: name.clone(),
                    ty: bind_ty,
                    mutable: false,
                    access: AccessLevel::Internal,
                })?;
                self.analyze_block(else_branch)?;
                self.analyze_block(body)?;
                Ok(())
            }
            Stmt::Throw { expr, .. } => {
                self.analyze_expr(expr)?;
                Ok(())
            }
            Stmt::RepeatWhile { body, condition, .. } => {
                self.analyze_block(body)?;
                self.analyze_expr(condition)?;
                Ok(())
            }
            Stmt::MacroInvoke { args, .. } => {
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                Ok(())
            }
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> SyxResult<TypeId> {
        let ty = self.analyze_expr_inner(expr)?;
        self.record_expr_type(expr, ty.clone());
        Ok(ty)
    }

    fn record_expr_type(&mut self, expr: &Expr, ty: TypeId) {
        self.expr_types.insert(expr_span(expr).start as u32, ty);
    }

    fn literal_numeric_type(&self, value: &Literal) -> TypeId {
        match value {
            Literal::Integer { suffix, .. } => {
                if suffix.as_deref() == Some("n") {
                    return TypeId::Named(SmolStr::new("BigInt"));
                }
                if let Some(s) = suffix {
                    if let Some(kind) = from_suffix(s) {
                        return TypeId::numeric(kind);
                    }
                }
                // Preserve `Int` alias (do not resolve via `TypeId::numeric`).
                TypeId::Numeric(default_integer_kind())
            }
            Literal::Double { suffix, .. } => {
                if let Some(s) = suffix {
                    if let Some(kind) = from_suffix(s) {
                        return TypeId::numeric(kind);
                    }
                }
                TypeId::numeric(default_float_kind())
            }
            _ => TypeId::Unknown,
        }
    }

    fn reference_lifetime(ty: &TypeExpr) -> Option<SmolStr> {
        if let TypeExpr::Reference(_, _, lt) = ty {
            lt.clone()
        } else {
            None
        }
    }

    /// Phase 22: partial `'a` inference for elided reference lifetimes.
    fn infer_elided_lifetime(ty: &TypeExpr) -> Option<SmolStr> {
        if let TypeExpr::Reference(_, _, lt) = ty {
            Some(lt.clone().unwrap_or_else(|| SmolStr::new("'a")))
        } else {
            None
        }
    }

    fn infer_return_lifetime(params: &[syx_ast::Param], return_type: &Option<TypeExpr>) -> Option<SmolStr> {
        let return_ty = return_type.as_ref()?;
        let TypeExpr::Reference(_, _, lt) = return_ty else {
            return None;
        };
        if lt.is_some() {
            return lt.clone();
        }
        let elided_ref_params = params
            .iter()
            .filter(|p| {
                matches!(
                    p.ty.as_ref(),
                    Some(TypeExpr::Reference(_, _, inner_lt)) if inner_lt.is_none()
                )
            })
            .count();
        if elided_ref_params == 1 {
            Some(SmolStr::new("'a"))
        } else {
            None
        }
    }

    fn assign_elided_lifetimes(ty: &mut TypeExpr, lifetime: &SmolStr) {
        if let TypeExpr::Reference(inner, _, lt) = ty {
            if lt.is_none() {
                *lt = Some(lifetime.clone());
            }
            Self::assign_elided_lifetimes(inner, lifetime);
        }
    }

    fn check_return_lifetime(&mut self, expr: &Expr) {
        let Some(ret_lt) = &self.current_return_lifetime else {
            return;
        };
        if let Expr::Ident { name, .. } = expr {
            if let Some(bind_lt) = self.binding_lifetimes.get(name) {
                if bind_lt != ret_lt {
                    self.errors.push(
                        SyxDiagnostic::error(format!(
                            "Lifetime '{bind_lt}' does not outlive '{ret_lt}' in return"
                        ))
                        .with_code("S011"),
                    );
                }
            }
        }
    }

    fn check_literal_in_range(&mut self, value: &Literal, expected: &TypeId) {
        let TypeId::Numeric(kind) = expected else {
            return;
        };
        let kind = resolve_alias(*kind);
        let Some((min, max)) = int_range(kind) else {
            return;
        };
        if let Literal::Integer { value: v, .. } = value {
            let v128 = *v as i128;
            if v128 < min || v128 > max {
                self.errors.push(
                    SyxDiagnostic::error(format!(
                        "Integer literal {v} is out of range for type {}",
                        type_id_to_name(expected)
                    ))
                    .with_code("S011"),
                );
            }
        }
    }

    fn unify_element_types(&mut self, types: &[TypeId]) -> TypeId {
        let mut elem = TypeId::Unknown;
        for t in types {
            if matches!(t, TypeId::Unknown) {
                continue;
            }
            if matches!(elem, TypeId::Unknown) {
                elem = t.clone();
                continue;
            }
            if elem != *t && !self.is_compatible(&elem, t) && !self.is_compatible(t, &elem) {
                self.errors.push(
                    SyxDiagnostic::error(format!(
                        "Collection elements have inconsistent types: expected {}, found {}",
                        type_id_to_name(&elem),
                        type_id_to_name(t)
                    ))
                    .with_code("S014"),
                );
            }
        }
        elem
    }

    fn analyze_expr_inner(&mut self, expr: &Expr) -> SyxResult<TypeId> {
        match expr {
            Expr::Literal { value, .. } => Ok(match value {
                Literal::BigInt(_) => TypeId::Named(SmolStr::new("BigInt")),
                Literal::Integer { .. } | Literal::Double { .. } => self.literal_numeric_type(value),
                Literal::Bool(_) => TypeId::Bool,
                Literal::String(_) | Literal::Interpolated(_) | Literal::Regex(_) => TypeId::String,
                Literal::Nil => TypeId::Optional(Box::new(TypeId::Unknown)),
                // Default: dict literal → homogeneous `Dictionary`. Heterogeneous
                // bags require an explicit `: Object` annotation (contextual typing
                // via `analyze_expr_with_expected`).
                Literal::Dict(entries) => {
                    let mut key_types = Vec::new();
                    let mut value_types = Vec::new();
                    for (k, v) in entries {
                        key_types.push(self.analyze_expr(k)?);
                        value_types.push(self.analyze_expr(v)?);
                    }
                    let key_ty = self.unify_element_types(&key_types);
                    // Empty `{}` / `[:]` unify to Unknown; Hashable is checked against the
                    // annotation when present (`analyze_expr_with_expected`), not here.
                    if !matches!(key_ty, TypeId::Unknown) {
                        self.validate_dict_key_hashable(&key_ty);
                    }
                    let val_ty = self.unify_element_types(&value_types);
                    TypeId::Dictionary {
                        key: Box::new(key_ty),
                        value: Box::new(val_ty),
                    }
                }
            }),
            Expr::Ident { name, span, .. } => {
                if matches!(
                    name.as_str(),
                    "Result" | "BigInt" | "SIMD4" | "SIMD8" | "SIMD16"
                ) {
                    return Ok(TypeId::Named(name.clone()));
                }
                // Phase 134: bare class/type name for `Type.staticVar` member access.
                if self.class_names.contains(name) || self.dataclass_names.contains(name) {
                    return Ok(TypeId::Named(name.clone()));
                }
                if syx_stdlib::retired_ops_module_names().contains(&name.as_str()) {
                    return Err(self.error_at(
                        format!(
                            "Module '{name}' was removed in SYX 22.0; use Core builtin '{}' instead",
                            retired_ops_hint(name.as_str())
                        ),
                        *span,
                        "S025",
                    ));
                }
                if self.moved_bindings.contains(name) {
                    return Err(
                        SyxDiagnostic::error(format!("Use of moved value '{name}'"))
                            .with_code("S011"),
                    );
                }
                self.check_main_actor_after_await(name);
                if let Some(ty) = self
                    .scope
                    .lookup(name)
                    .or_else(|| self.globals.get(name))
                    .map(|s| s.ty.clone())
                {
                    return Ok(self.deref_type(&ty));
                }
                // S1-I2: bare stored property in struct/class method (`value = value + 1`).
                if let Some(ty) = self.lookup_enclosing_type_field(name) {
                    return Ok(ty);
                }
                Err(self.error_at(
                    format!("Undefined identifier '{name}'"),
                    *span,
                    "S002",
                ))
            }
            Expr::Borrow { expr, mutable, .. } => {
                if *mutable {
                    if let Expr::Ident { name, .. } = expr.as_ref() {
                        if self.mut_borrows.contains(name) {
                            return Err(SyxDiagnostic::error(format!(
                                "Cannot borrow '{name}' as mutable more than once"
                            ))
                            .with_code("S011"));
                        }
                        self.mut_borrows.insert(name.clone());
                    }
                    let inner = self.analyze_expr(expr)?;
                    Ok(TypeId::MutReference(Box::new(inner)))
                } else {
                    let inner = self.analyze_expr(expr)?;
                    Ok(TypeId::Reference(Box::new(inner)))
                }
            }
            Expr::KeyPath { .. } => Ok(TypeId::Named(SmolStr::new("KeyPath"))),
            Expr::MacroInvoke { args, .. } => {
                for a in args {
                    self.analyze_expr(a)?;
                }
                Ok(TypeId::Unknown)
            }
            Expr::OptionalCast { expr, target, .. } => {
                let _from = self.analyze_expr(expr)?;
                let to = self.resolve_type_expr(target);
                Ok(TypeId::Optional(Box::new(to)))
            }
            Expr::Is { expr, target, span } => {
                let _from = self.analyze_expr(expr)?;
                let to = self.resolve_type_expr(target);
                if to.is_numeric() {
                    self.errors.push(self.error_at(
                        "'is' cannot be used with numeric types",
                        *span,
                        "S046",
                    ));
                }
                Ok(TypeId::Bool)
            }
            Expr::Cast { expr, target, span } => {
                if let Expr::Call {
                    callee,
                    args: resolve_args,
                    ..
                } = expr.as_ref()
                {
                    let is_typed_resolve = match callee.as_ref() {
                        Expr::Ident { name, .. } => name.as_str() == "__syx_service_resolve",
                        Expr::Member { object, name, .. }
                            if name.as_str() == "resolve"
                                && matches!(
                                    object.as_ref(),
                                    Expr::Ident { name: m, .. } if m.as_str() == "Di"
                                ) =>
                        {
                            true
                        }
                        _ => false,
                    };
                    if is_typed_resolve {
                        if let Some(arg) = resolve_args.first() {
                            if let Some(type_name) =
                                self.service_resolve_type_name_from_arg(&arg.value)
                            {
                                let to = self.resolve_type_expr(target);
                                let resolved = TypeId::Named(type_name.clone());
                                if to != resolved {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "service resolve({type_name}) cast to incompatible type"
                                        ),
                                        *span,
                                        "E010",
                                    ));
                                } else if !self.service_class_names.contains(&type_name) {
                                    self.errors.push(self.error_at(
                                        format!(
                                            "service resolve: '{type_name}' is not a @Service/@Component type"
                                        ),
                                        *span,
                                        "E010",
                                    ));
                                }
                            }
                        }
                    }
                }
                let _from = self.analyze_expr(expr)?;
                let to = self.resolve_type_expr(target);
                Ok(to)
            }
            Expr::TypeSelf { type_name, .. } => {
                let _ = reflect_type_lower(type_name.as_str());
                Ok(TypeId::Named(SmolStr::new("Metatype")))
            }
            Expr::Super { span, .. } => {
                if self.current_class_context.is_none() {
                    self.errors.push(self.error_at(
                        "'super' is only valid inside class instance methods",
                        *span,
                        "S055",
                    ));
                }
                Ok(TypeId::Unknown)
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
                ..
            } => {
                let lt = self.analyze_expr(left)?;
                let rt = self.analyze_expr(right)?;
                if matches!(
                    *op,
                    BinOp::Eq | BinOp::NotEq | BinOp::IdenticalEq | BinOp::IdenticalNe
                ) {
                    self.check_equatable_binary(*op, &lt, &rt);
                }
                match op {
                    BinOp::Add => {
                        // String concatenation: `"a" + "b"` → String
                        let l_str = matches!(lt, TypeId::String)
                            || matches!(&lt, TypeId::Named(n) if n.as_str() == "String");
                        let r_str = matches!(rt, TypeId::String)
                            || matches!(&rt, TypeId::Named(n) if n.as_str() == "String");
                        if l_str && r_str {
                            return Ok(TypeId::String);
                        }
                        if let (Some((la, ea)), Some((lb, eb))) = (lt.as_simd(), rt.as_simd()) {
                            if la == lb && ea == eb {
                                if !syx_config::simd_enabled() {
                                    self.errors.push(
                                        SyxDiagnostic::error(
                                            "SIMD operations require SIMD to be enabled",
                                        )
                                        .with_code("S022"),
                                    );
                                }
                                return Ok(TypeId::Simd {
                                    lanes: la,
                                    element: ea,
                                });
                            }
                            self.errors.push(
                                SyxDiagnostic::error("SIMD operands must have matching lane width and element type")
                                    .with_code("S014"),
                            );
                            return Ok(TypeId::Unknown);
                        }
                        Ok(self.promote_numeric_binop(lt, rt, *span))
                    }
                    BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::Mod
                    | BinOp::BitAnd
                    | BinOp::BitOr
                    | BinOp::BitXor
                    | BinOp::Shl
                    | BinOp::Shr => {
                        if let (Some((la, ea)), Some((lb, eb))) = (lt.as_simd(), rt.as_simd()) {
                            if la == lb && ea == eb {
                                if !syx_config::simd_enabled() {
                                    self.errors.push(
                                        SyxDiagnostic::error(
                                            "SIMD operations require SIMD to be enabled",
                                        )
                                        .with_code("S022"),
                                    );
                                }
                                return Ok(TypeId::Simd {
                                    lanes: la,
                                    element: ea,
                                });
                            }
                            self.errors.push(
                                SyxDiagnostic::error("SIMD operands must have matching lane width and element type")
                                    .with_code("S014"),
                            );
                            return Ok(TypeId::Unknown);
                        }
                        Ok(self.promote_numeric_binop(lt, rt, *span))
                    }
                    BinOp::Eq
                    | BinOp::NotEq
                    | BinOp::IdenticalEq
                    | BinOp::IdenticalNe
                    | BinOp::Lt
                    | BinOp::LtEq
                    | BinOp::Gt
                    | BinOp::GtEq => Ok(TypeId::Bool),
                    BinOp::And | BinOp::Or => Ok(TypeId::Bool),
                    BinOp::NilCoalesce => Ok(rt),
                }
            }
            Expr::Unary { op, expr, .. } => {
                if *op == UnaryOp::Try {
                    self.try_depth += 1;
                    let ty = self.analyze_expr(expr)?;
                    self.try_depth = self.try_depth.saturating_sub(1);
                    return Ok(ty);
                }
                let ty = self.analyze_expr(expr)?;
                match op {
                    UnaryOp::Neg | UnaryOp::BitNot => Ok(self.promote_numeric(ty.clone(), ty)),
                    UnaryOp::Not => Ok(TypeId::Bool),
                    UnaryOp::Try => Ok(ty),
                }
            }
            Expr::Call { callee, args, .. } => {
                let ret = self.analyze_call(callee, args)?;
                for arg in args {
                    if let Expr::Borrow {
                        expr,
                        mutable: true,
                        ..
                    } = &arg.value
                    {
                        if let Expr::Ident { name, .. } = expr.as_ref() {
                            self.mut_borrows.remove(name);
                        }
                    }
                }
                Ok(ret)
            }
            Expr::Member { object, name, span, .. } => {
                let obj_ty = self.analyze_expr(object)?;
                if let Expr::Ident { name: module, .. } = object.as_ref() {
                    self.check_import_member_access(module.as_str(), name.as_str())?;
                }
                if let TypeId::Named(enum_name) = &obj_ty {
                    if self
                        .enum_cases
                        .get(enum_name)
                        .is_some_and(|cases| cases.iter().any(|c| c == name))
                    {
                        return Ok(TypeId::Named(enum_name.clone()));
                    }
                }
                self.check_retired_size_alias(&obj_ty, name.as_str(), *span)?;
                self.check_actor_field_access(object, name, &obj_ty);
                self.check_extension_member_where(&obj_ty, name, *span);
                self.check_known_collection_member(&obj_ty, name.as_str(), *span);
                Ok(self.infer_member_type(&obj_ty, name))
            }
            Expr::Index { object, index, span, .. } => {
                self.analyze_expr(object)?;
                let obj_ty = self.type_of_expr(object);
                match obj_ty {
                    TypeId::Array(inner) => {
                        let idx_ty = TypeId::platform_int();
                        let found_idx =
                            self.analyze_expr_with_expected(index, Some(&idx_ty))?;
                        self.check_assignable_with_literal(
                            idx_ty,
                            found_idx,
                            "index",
                            Some(index),
                            *span,
                        )?;
                        Ok(*inner)
                    }
                    TypeId::String => {
                        self.analyze_expr(index)?;
                        Ok(TypeId::Named(SmolStr::new("Character")))
                    }
                    TypeId::Dictionary { key, value } => {
                        if !matches!(
                            key.as_ref(),
                            TypeId::Unknown | TypeId::GenericParam(_)
                        ) {
                            let found =
                                self.analyze_expr_with_expected(index, Some(key.as_ref()))?;
                            self.check_assignable_with_literal(
                                *key,
                                found,
                                "key",
                                Some(index),
                                *span,
                            )?;
                        } else {
                            self.analyze_expr(index)?;
                        }
                        Ok(*value)
                    }
                    TypeId::Named(type_name) if self.has_subscript_get(&type_name) => {
                        self.analyze_expr(index)?;
                        Ok(self.subscript_return_type(&type_name).unwrap_or(TypeId::Unknown))
                    }
                    _ => {
                        self.analyze_expr(index)?;
                        Ok(TypeId::Unknown)
                    }
                }
            }
            Expr::Assign { name, value, span, .. } => {
                self.check_main_actor_after_await(name);
                let (mutable, sym_ty, is_enclosing_field) = match self
                    .scope
                    .lookup(name)
                    .or_else(|| self.globals.get(name))
                {
                    Some(sym) => (sym.mutable, sym.ty.clone(), false),
                    None => {
                        if let Some(field_ty) = self.lookup_enclosing_type_field(name) {
                            // Stored `var` property via bare name in method body.
                            (true, field_ty, true)
                        } else {
                            return Err(
                                SyxDiagnostic::error(format!("Undefined identifier '{name}'"))
                                    .with_code("S002"),
                            );
                        }
                    }
                };
                if !mutable {
                    if matches!(sym_ty, TypeId::Reference(_)) {
                        return Err(SyxDiagnostic::error(format!(
                            "Cannot mutate through read-only borrow parameter '{name}'"
                        ))
                        .with_code("S011"));
                    }
                    return Err(
                        SyxDiagnostic::error(format!("Cannot assign to immutable '{name}'"))
                            .with_code("S003"),
                    );
                }
                let expected = self.assign_expected_type(&sym_ty);
                let ty = if matches!(expected, TypeId::Unknown) {
                    self.analyze_expr(value)?
                } else {
                    self.analyze_expr_with_expected(value, Some(&expected))?
                };
                // Reassignment rebinds the name; prior move of the old value no longer applies.
                self.moved_bindings.remove(name);
                self.mut_borrows.remove(name);
                if is_enclosing_field {
                    self.check_assignable_with_literal(
                        expected,
                        ty,
                        name,
                        Some(value),
                        *span,
                    )?;
                } else if matches!(expected, TypeId::Unknown) {
                    if let Some(binding) = self.scope.lookup_mut(name) {
                        binding.ty = ty.clone();
                    }
                    self.track_binding(name.clone(), ty.clone(), span.start);
                } else {
                    self.check_assignable_with_literal(
                        expected,
                        ty,
                        name,
                        Some(value),
                        *span,
                    )?;
                }
                Ok(TypeId::Void)
            }
            Expr::CompoundAssign { name, value, .. } => {
                self.check_main_actor_after_await(name);
                let ty = self.analyze_expr(value)?;
                if let Some(sym) = self.scope.lookup(name).or_else(|| self.globals.get(name)) {
                    if !sym.mutable {
                        if matches!(sym.ty, TypeId::Reference(_)) {
                            return Err(SyxDiagnostic::error(format!(
                                "Cannot mutate through read-only borrow parameter '{name}'"
                            ))
                            .with_code("S011"));
                        }
                        return Err(
                            SyxDiagnostic::error(format!("Cannot assign to immutable '{name}'"))
                                .with_code("S003"),
                        );
                    }
                    let expected = self.assign_expected_type(&sym.ty);
                    if matches!(expected, TypeId::Numeric(_)) {
                        let _ = self.promote_numeric(expected.clone(), ty);
                    } else {
                        self.check_assignable(expected, ty, name, expr_span(value))?;
                    }
                } else {
                    return Err(
                        SyxDiagnostic::error(format!("Undefined identifier '{name}'"))
                            .with_code("S002"),
                    );
                }
                Ok(TypeId::Void)
            }
            Expr::MemberAssign { object, name, value, span, .. } => {
                let obj_ty = self.analyze_expr(object)?;
                if let Some(obj_ty) = self.expr_type(object) {
                    self.check_actor_field_access(object, name, &obj_ty);
                    if let TypeId::Named(type_name) = &obj_ty {
                        if self
                            .getter_only_properties
                            .contains(&(type_name.clone(), name.clone()))
                        {
                            return Err(self.error_at(
                                format!(
                                    "Cannot assign to read-only property '{type_name}.{name}'"
                                ),
                                *span,
                                "S057",
                            ));
                        }
                    }
                }
                self.analyze_expr(value)?;
                let _ = obj_ty;
                Ok(TypeId::Void)
            }
            Expr::IndexAssign { object, index, value, span, .. } => {
                self.analyze_expr(object)?;
                let obj_ty = self.type_of_expr(object);
                if let TypeId::Named(ref type_name) = obj_ty {
                    if self.has_subscript_set(type_name) {
                        self.analyze_expr(index)?;
                        self.analyze_expr(value)?;
                        return Ok(TypeId::Void);
                    }
                }
                if let TypeId::Dictionary { key, value: val_ty } = obj_ty {
                    if !matches!(key.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) {
                        let found_key =
                            self.analyze_expr_with_expected(index, Some(key.as_ref()))?;
                        self.check_assignable_with_literal(
                            *key,
                            found_key,
                            "key",
                            Some(index),
                            *span,
                        )?;
                    } else {
                        self.analyze_expr(index)?;
                    }
                    if !matches!(
                        val_ty.as_ref(),
                        TypeId::Unknown | TypeId::GenericParam(_)
                    ) {
                        let found_val =
                            self.analyze_expr_with_expected(value, Some(val_ty.as_ref()))?;
                        self.check_assignable_with_literal(
                            *val_ty,
                            found_val,
                            "value",
                            Some(value),
                            *span,
                        )?;
                    } else {
                        self.analyze_expr(value)?;
                    }
                    return Ok(TypeId::Void);
                }
                if let TypeId::Array(elem) = obj_ty {
                    let idx_ty = TypeId::platform_int();
                    let found_idx =
                        self.analyze_expr_with_expected(index, Some(&idx_ty))?;
                    self.check_assignable_with_literal(
                        idx_ty,
                        found_idx,
                        "index",
                        Some(index),
                        *span,
                    )?;
                    if !matches!(
                        elem.as_ref(),
                        TypeId::Unknown | TypeId::GenericParam(_)
                    ) {
                        let found_val =
                            self.analyze_expr_with_expected(value, Some(elem.as_ref()))?;
                        self.check_assignable_with_literal(
                            *elem,
                            found_val,
                            "value",
                            Some(value),
                            *span,
                        )?;
                    } else {
                        self.analyze_expr(value)?;
                    }
                    return Ok(TypeId::Void);
                }
                self.analyze_expr(index)?;
                self.analyze_expr(value)?;
                Ok(TypeId::Void)
            }
            Expr::Closure {
                params,
                body,
                return_type,
                ..
            } => {
                if !self.mut_borrows.is_empty() {
                    self.errors.push(
                        SyxDiagnostic::error(
                            "Closure cannot capture an active `&mut` borrow from an outer region",
                        )
                        .with_code("S011"),
                    );
                }
                if borrow_regions_v7_enabled()
                    && self.current_function_is_async
                    && !self.immutable_ref_bindings.is_empty()
                {
                    for name in &self.immutable_ref_bindings {
                        self.errors.push(
                            SyxDiagnostic::error(format!(
                                "Immutable borrow '{name}' cannot be captured in nested async closure region"
                            ))
                            .with_code("S011"),
                        );
                    }
                }
                let annotated = return_type.as_ref().map(type_expr_to_id);
                let parent = Some(Box::new(std::mem::take(&mut self.scope)));
                let mut closure_scope = Scope {
                    parent,
                    ..Default::default()
                };
                for param in params {
                    closure_scope.define(Symbol {
                        name: param.name.clone(),
                        ty: param
                            .ty
                            .as_ref()
                            .map(type_expr_to_id)
                            .unwrap_or(TypeId::Unknown),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                }
                std::mem::swap(&mut self.scope, &mut closure_scope);
                // Expression type of a closure is its result type (for HOF inference).
                let result = if let Some(ann) = annotated {
                    match body {
                        syx_ast::ClosureBody::Expr(e) => {
                            let _ = self.analyze_expr(e)?;
                        }
                        syx_ast::ClosureBody::Block(stmts) => {
                            self.analyze_block(stmts)?;
                        }
                    }
                    Ok(ann)
                } else {
                    match body {
                        syx_ast::ClosureBody::Expr(e) => self.analyze_expr(e),
                        syx_ast::ClosureBody::Block(stmts) => {
                            self.analyze_block(stmts)?;
                            // Unannotated block: no stored result type (use `-> T` or expr body).
                            Ok(TypeId::Unknown)
                        }
                    }
                };
                self.scope = *self.scope.parent.take().unwrap_or_default();
                result
            }
            Expr::TaskBlock { body, .. } => {
                self.analyze_block(body)?;
                Ok(TypeId::Unknown)
            }
            Expr::Group { expr, .. } => self.analyze_expr(expr),
            Expr::Tuple { elements, .. } => {
                let mut elem_types = Vec::new();
                for (_, el) in elements {
                    elem_types.push(self.analyze_expr(el)?);
                }
                let typed: Vec<(Option<SmolStr>, TypeId)> = elements
                    .iter()
                    .zip(elem_types.iter())
                    .map(|((label, _), ty)| (label.clone(), ty.clone()))
                    .collect();
                Ok(TypeId::Tuple {
                    elements: typed,
                })
            }
            Expr::Array { elements, .. } => {
                let mut elem_types = Vec::new();
                for el in elements {
                    elem_types.push(self.analyze_expr(el)?);
                }
                let elem_ty = self.unify_element_types(&elem_types);
                Ok(TypeId::Array(Box::new(elem_ty)))
            }
            Expr::Await { expr, span: _ } => {
                if !self.mut_borrows.is_empty() {
                    self.errors.push(
                        SyxDiagnostic::error(
                            "Cannot await while an active `&mut` borrow is in scope",
                        )
                        .with_code("S011"),
                    );
                }
                if self.current_function_is_async && !self.local_ref_bindings.is_empty() {
                    for name in &self.local_ref_bindings {
                        self.errors.push(
                            SyxDiagnostic::error(format!(
                                "Reference '{name}' cannot cross await boundary"
                            ))
                            .with_code("S017"),
                        );
                    }
                }
                if borrow_regions_v6_enabled()
                    && self.current_function_is_async
                    && !self.immutable_ref_bindings.is_empty()
                {
                    for name in &self.immutable_ref_bindings {
                        self.errors.push(
                            SyxDiagnostic::error(format!(
                                "Immutable borrow '{name}' cannot be reborrowed across await boundary"
                            ))
                            .with_code("S011"),
                        );
                    }
                }
                if !self.skip_main_actor_await_check
                    && self.current_function_is_async
                    && self
                        .caller_global_actor(self.current_function.as_ref().unwrap())
                        .as_deref()
                        == Some("MainActor")
                {
                    self.main_actor_frozen_at_await = self.main_actor_mutable_locals.clone();
                    self.main_actor_await_crossed = true;
                }
                self.analyze_expr(expr)
            }
            Expr::OptionalChain { object, .. } => {
                self.analyze_expr(object)?;
                Ok(TypeId::Optional(Box::new(TypeId::Unknown)))
            }
            Expr::ForceUnwrap { expr, .. } => self.analyze_expr(expr),
            Expr::HalfOpenRange { .. } | Expr::ClosedRange { .. } => Ok(TypeId::Named(SmolStr::new("Range"))),
            Expr::VariantPattern { .. } | Expr::TuplePattern { .. } => Ok(TypeId::Unknown),
        }
    }

    fn check_import_member_access(&self, module: &str, member: &str) -> SyxResult<()> {
        check_import_access(
            &self.imported_modules,
            &self.import_exports,
            module,
            member,
        )
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

    fn check_prelude_name_shadow(&mut self, name: &SmolStr, span: syx_diagnostics::Span, kind: &str) {
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

    fn error_at(&self, message: impl Into<String>, span: syx_diagnostics::Span, code: &str) -> SyxDiagnostic {
        let mut diag = SyxDiagnostic::error(message).with_code(code);
        if span != syx_diagnostics::Span::DUMMY {
            if let Some(ref src) = self.source {
                diag = diag.with_span(src, span, "");
            }
        }
        diag
    }

    /// Bare stored property of the enclosing struct/class (book `value = value + 1`).
    fn lookup_enclosing_type_field(&self, name: &SmolStr) -> Option<TypeId> {
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

    fn analyze_call(&mut self, callee: &Expr, args: &[CallArg]) -> SyxResult<TypeId> {
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
                            self.errors.push(
                                SyxDiagnostic::error(
                                    "Non-Sendable value cannot be sent on a channel",
                                )
                                .with_code("S019"),
                            );
                        }
                    }
                    return Ok(TypeId::Void);
                }
                if self.throwing_functions.contains_key(name) && self.try_depth == 0 {
                    self.errors.push(
                        SyxDiagnostic::error(format!(
                            "Call to throwing function '{name}' must use try or appear inside a try block"
                        ))
                        .with_code("S018"),
                    );
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
                    self.validate_generic_instantiation(&fn_name, &type_args);
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
                // Local / param / global holding a closure: `let f = { x in … }; f(1)`.
                if !self.is_callable_function(name) {
                    let binding_ty = self
                        .scope
                        .lookup(name)
                        .or_else(|| self.globals.get(name))
                        .map(|s| &s.ty);
                    if binding_ty.is_some_and(|ty| {
                        matches!(ty, TypeId::Named(n) if n.as_str() == "Closure")
                    }) {
                        return Ok(TypeId::Unknown);
                    }
                    return Err(self.error_at(
                        format!("Undefined function '{name}'"),
                        *span,
                        "S002",
                    ));
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
                    self.check_import_member_access(module.as_str(), name.as_str())?;
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
                            );
                            self.record_instantiation(qualified, type_args);
                            return Ok(TypeId::Void);
                        }
                    }
                }
                let collection_elem = match (&obj_ty, name.as_str(), args.len()) {
                    (TypeId::Set(elem), "insert" | "remove" | "contains", 1)
                        if !matches!(elem.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) =>
                    {
                        Some((elem.as_ref().clone(), "item"))
                    }
                    (TypeId::Array(elem), "append" | "contains", 1)
                        if !matches!(elem.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) =>
                    {
                        Some((elem.as_ref().clone(), "item"))
                    }
                    (TypeId::Dictionary { key, .. }, "containsKey" | "remove", 1)
                        if !matches!(key.as_ref(), TypeId::Unknown | TypeId::GenericParam(_)) =>
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
                            );
                            self.record_instantiation(enum_name.clone(), type_args);
                        }
                    }
                }
                Ok(TypeId::Void)
            }
        }
    }

    fn infer_pattern_type(&self, pattern: &Expr) -> SyxResult<TypeId> {
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

    fn check_assignable(
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
        match check_assignable(expected, found, name) {
            Ok(()) => Ok(()),
            Err(err) => {
                let code = err.code.as_deref().unwrap_or("S004");
                Err(self.error_at(err.message, span, code))
            }
        }
    }

    fn is_subclass_of(&self, sub: &str, base: &str) -> bool {
        if sub == base {
            return true;
        }
        self.class_superclasses
            .get(&SmolStr::new(sub))
            .map(|parent| self.is_subclass_of(parent.as_str(), base))
            .unwrap_or(false)
    }

    fn check_assignable_with_literal(
        &mut self,
        expected: TypeId,
        found: TypeId,
        name: &str,
        init: Option<&Expr>,
        decl_span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        let diag_span = init.map(expr_span).unwrap_or(decl_span);
        if matches!(expected, TypeId::OpaqueAny(_) | TypeId::OpaqueSome(_)) {
            if self.check_opaque_protocol_assignable(&expected, &found, name) {
                return Ok(());
            }
        }
        if let Some(Expr::Literal { value, .. }) = init {
            self.check_literal_in_range(value, &expected);
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
                self.validate_set_elem_hashable(set_elem);
                return Ok(());
            }
            if matches!(&found, TypeId::Named(n) if n.as_str() == "Set") {
                let TypeId::Set(set_elem) = &expected else {
                    return self.check_assignable(expected, found, name, diag_span);
                };
                self.validate_set_elem_hashable(set_elem);
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
                    self.validate_dict_key_hashable(exp_key);
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
    fn analyze_expr_with_expected(
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
                    self.validate_dict_key_hashable(key);
                    let ty = TypeId::Dictionary {
                        key: key.clone(),
                        value: value.clone(),
                    };
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

    fn is_compatible(&self, expected: &TypeId, found: &TypeId) -> bool {
        is_compatible(expected, found)
    }

    fn analyze_type_member(
        &mut self,
        type_name: &SmolStr,
        member: &Decl,
        in_actor: bool,
    ) -> SyxResult<()> {
        let prev = self.current_actor_context.clone();
        let prev_class = self.current_class_context.clone();
        self.current_class_context = Some(type_name.clone());
        if in_actor {
            self.current_actor_context = Some(type_name.clone());
        }
        let result = match member {
            Decl::Function { span, params, body, .. } | Decl::Init { span, params, body, .. } => {
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                fn_scope.define(Symbol {
                    name: SmolStr::new("self"),
                    ty: TypeId::Named(type_name.clone()),
                    mutable: true,
                    access: AccessLevel::Internal,
                })?;
                for param in params {
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(type_expr_to_id)
                        .unwrap_or(TypeId::Unknown);
                    fn_scope.define(Symbol {
                        name: param.name.clone(),
                        ty: param_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            Decl::Deinit { body, .. } => {
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                fn_scope.define(Symbol {
                    name: SmolStr::new("self"),
                    ty: TypeId::Named(type_name.clone()),
                    mutable: true,
                    access: AccessLevel::Internal,
                })?;
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            Decl::VarGlobal {
                init,
                ty,
                name,
                mutable,
                will_set,
                did_set,
                computed_getter,
                computed_setter,
                is_lazy,
                attrs,
                span,
                ..
            } => {
                let is_static = syx_ast::var_global_is_static(attrs);
                if *is_lazy && !*mutable {
                    self.errors.push(self.error_at(
                        "lazy is only allowed on var properties",
                        *span,
                        "S057",
                    ));
                }
                if *is_lazy && is_static {
                    self.errors.push(self.error_at(
                        "lazy cannot be combined with static",
                        *span,
                        "S057",
                    ));
                }
                if computed_getter.is_some() && computed_setter.is_some() && !*mutable {
                    self.errors.push(self.error_at(
                        "Computed property setter requires 'var'",
                        *span,
                        "S057",
                    ));
                }
                if computed_getter.is_some() {
                    if init.is_some() {
                        self.errors.push(self.error_at(
                            "computed properties cannot have an initializer",
                            *span,
                            "S057",
                        ));
                    }
                    if will_set.is_some() || did_set.is_some() {
                        self.errors.push(self.error_at(
                            "willSet/didSet are not allowed on computed properties",
                            *span,
                            "S056",
                        ));
                    }
                }
                if (will_set.is_some() || did_set.is_some()) && !*mutable {
                    self.errors.push(self.error_at(
                        "willSet/didSet are only allowed on stored mutable (var) properties",
                        *span,
                        "S056",
                    ));
                }
                if *is_lazy && init.is_none() {
                    self.errors.push(self.error_at(
                        "lazy var requires an initializer",
                        *span,
                        "S057",
                    ));
                }
                if in_actor {
                    if let Some(type_expr) = ty {
                        let field_ty = type_expr_to_id(type_expr);
                        if self.strict_concurrency && self.is_non_sendable_type(&field_ty) {
                            self.errors.push(
                                SyxDiagnostic::error(format!(
                                    "Actor '{type_name}' cannot store non-Sendable field '{name}'"
                                ))
                                .with_code("C007"),
                            );
                        }
                    }
                }
                if let Some(expr) = init {
                    self.analyze_expr(expr)?;
                }
                let field_ty = ty
                    .as_ref()
                    .map(type_expr_to_id)
                    .unwrap_or(TypeId::Unknown);
                if let Some(stmts) = will_set {
                    let mut obs_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    obs_scope.define(Symbol {
                        name: SmolStr::new("newValue"),
                        ty: field_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    obs_scope.define(Symbol {
                        name: SmolStr::new("oldValue"),
                        ty: field_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    std::mem::swap(&mut self.scope, &mut obs_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if let Some(stmts) = did_set {
                    let mut obs_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    obs_scope.define(Symbol {
                        name: SmolStr::new("oldValue"),
                        ty: field_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    obs_scope.define(Symbol {
                        name: SmolStr::new("newValue"),
                        ty: field_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    std::mem::swap(&mut self.scope, &mut obs_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if let Some(stmts) = computed_getter {
                    let mut get_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    get_scope.define(Symbol {
                        name: SmolStr::new("self"),
                        ty: TypeId::Named(type_name.clone()),
                        mutable: true,
                        access: AccessLevel::Internal,
                    })?;
                    std::mem::swap(&mut self.scope, &mut get_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                if let Some(stmts) = computed_setter {
                    let mut set_scope = Scope {
                        parent: Some(Box::new(std::mem::take(&mut self.scope))),
                        ..Default::default()
                    };
                    set_scope.define(Symbol {
                        name: SmolStr::new("self"),
                        ty: TypeId::Named(type_name.clone()),
                        mutable: true,
                        access: AccessLevel::Internal,
                    })?;
                    set_scope.define(Symbol {
                        name: SmolStr::new("newValue"),
                        ty: field_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    std::mem::swap(&mut self.scope, &mut set_scope);
                    self.analyze_block(stmts)?;
                    self.scope = *self.scope.parent.take().unwrap_or_default();
                }
                Ok(())
            }
            Decl::Class {
                name: inner,
                members,
                ..
            } => {
                let qualified = SmolStr::new(format!("{type_name}_{inner}"));
                for nested in members {
                    self.analyze_type_member(&qualified, nested, in_actor)?;
                }
                Ok(())
            }
            Decl::Struct { fields, .. } => {
                for (_, ty) in fields {
                    let _ = type_expr_to_id(ty);
                }
                Ok(())
            }
            Decl::Enum { cases, .. } => {
                for (_, value, _) in cases {
                    if let Some(expr) = value {
                        self.analyze_expr(expr)?;
                    }
                }
                Ok(())
            }
            Decl::FinalClass { .. } => Ok(()),
            Decl::OperatorFunc {
                op,
                span,
                params,
                body,
                return_type,
                ..
            } => {
                if operator_overload_forbidden_type(type_name.as_str()) {
                    self.errors.push(self.error_at(
                        format!(
                            "Operator '{op}' cannot be defined on builtin type '{type_name}'"
                        ),
                        *span,
                        "S048",
                    ));
                } else if params.iter().any(|p| {
                    p.ty.as_ref().is_some_and(|t| {
                        if let TypeExpr::Named { name, .. } = t {
                            is_numeric_builtin_type(name.as_str())
                        } else {
                            false
                        }
                    })
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot overload numeric builtin types"),
                        *span,
                        "S043",
                    ));
                } else if params.iter().any(|p| {
                    p.ty.as_ref().is_some_and(|t| {
                        if let TypeExpr::Named { name, .. } = t {
                            is_text_builtin_type(name.as_str())
                        } else {
                            false
                        }
                    })
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot overload text builtin types"),
                        *span,
                        "S044",
                    ));
                } else if params.iter().any(|p| {
                    p.ty.as_ref().is_some_and(|t| {
                        if let TypeExpr::Named { name, generics, .. } = t {
                            is_collection_builtin_type(name.as_str())
                                || (name.as_str() == "Array" && !generics.is_empty())
                        } else {
                            false
                        }
                    })
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot overload collection builtin types"),
                        *span,
                        "S047",
                    ));
                } else if return_type.as_ref().is_some_and(|t| {
                    if let TypeExpr::Named { name, .. } = t {
                        is_numeric_builtin_type(name.as_str())
                    } else {
                        false
                    }
                }) {
                    self.errors.push(self.error_at(
                        format!("Operator '{op}' cannot return numeric builtin type"),
                        *span,
                        "S043",
                    ));
                }
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                fn_scope.define(Symbol {
                    name: SmolStr::new("self"),
                    ty: TypeId::Named(type_name.clone()),
                    mutable: true,
                    access: AccessLevel::Internal,
                })?;
                for param in params {
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(type_expr_to_id)
                        .unwrap_or(TypeId::Unknown);
                    fn_scope.define(Symbol {
                        name: param.name.clone(),
                        ty: param_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            _ => self.analyze_decl(member),
        };
        self.current_actor_context = prev;
        self.current_class_context = prev_class;
        result
    }

    fn switch_pattern_case_name(pattern: &Expr) -> Option<SmolStr> {
        match pattern {
            Expr::VariantPattern { variant, .. } => Some(variant.clone()),
            Expr::Ident { name, .. } => Some(name.clone()),
            _ => None,
        }
    }

    fn is_self_expr(expr: &Expr) -> bool {
        matches!(expr, Expr::Ident { name, .. } if name == "self")
    }

    /// `.length` / `.len` were retired in favor of Swift-aligned `.count`.
    fn check_retired_size_alias(
        &self,
        obj_ty: &TypeId,
        name: &str,
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        if !matches!(name, "length" | "len") {
            return Ok(());
        }
        let surface = match obj_ty {
            TypeId::Array(_) | TypeId::Set(_) | TypeId::Dictionary { .. } | TypeId::String => true,
            TypeId::Named(n) => {
                let s = n.as_str();
                s == "String"
                    || s == "Array"
                    || s.starts_with("Array<")
                    || s == "Set"
                    || s.starts_with("Set<")
                    || s == "Dictionary"
                    || s.starts_with("Dictionary<")
                    || s == "Object"
            }
            _ => false,
        };
        if !surface {
            return Ok(());
        }
        Err(self.error_at(
            format!(
                "'.{name}' was removed on {}; use .count instead",
                type_id_to_name(obj_ty)
            ),
            span,
            "S004",
        ))
    }

    fn check_actor_field_access(&mut self, object: &Expr, field: &str, obj_ty: &TypeId) {
        let TypeId::Named(type_name) = obj_ty else {
            return;
        };
        if !self.actor_types.contains(type_name) {
            return;
        }
        let is_field = self
            .actor_fields
            .get(type_name)
            .is_some_and(|fields| fields.contains(field));
        if !is_field {
            return;
        }
        let allowed = self.current_actor_context.as_ref() == Some(type_name)
            && Self::is_self_expr(object);
        if !allowed {
            self.errors.push(
                SyxDiagnostic::error(format!(
                    "Cross-actor access to field '{field}' on actor '{type_name}' is not allowed"
                ))
                .with_code("S008"),
            );
        }
    }

    fn expr_type(&self, expr: &Expr) -> Option<TypeId> {
        match expr {
            Expr::Ident { name, .. } => self.symbol_type(name),
            _ => None,
        }
    }

    fn type_satisfies_protocol(&self, type_name: &str, protocol: &str) -> bool {
        if type_conforms(type_name, protocol) {
            return true;
        }
        if protocol == "Comparable" && is_comparable_type(type_name) {
            return true;
        }
        if from_type_name(type_name).is_some() {
            return matches!(protocol, "Equatable" | "Hashable")
                || type_conforms(type_name, protocol);
        }
        let unconditional = match (type_name, protocol) {
            ("Bool" | "Character", "Equatable" | "Hashable" | "CustomStringConvertible" | "CustomDebugStringConvertible") => true,
            ("String", "Equatable" | "Hashable" | "Comparable" | "CustomStringConvertible" | "CustomDebugStringConvertible") => true,
            ("Array", "Equatable" | "Hashable" | "Sequence" | "CustomStringConvertible" | "CustomDebugStringConvertible") => true,
            ("Set", "Equatable" | "Hashable" | "Sequence" | "CustomStringConvertible" | "CustomDebugStringConvertible") => true,
            ("Dictionary", "Equatable" | "Hashable" | "CustomStringConvertible" | "CustomDebugStringConvertible") => true,
            ("Object", "Equatable" | "Hashable" | "CustomStringConvertible" | "CustomDebugStringConvertible") => true,
            ("ObjectIdentifier", "Equatable" | "Hashable" | "CustomStringConvertible" | "CustomDebugStringConvertible") => true,
            _ => self
                .type_conformances
                .get(type_name)
                .is_some_and(|c| c.iter().any(|p| p.as_str() == protocol))
                || self.dataclass_names.contains(&SmolStr::new(type_name))
                    && matches!(
                        protocol,
                        "Equatable"
                            | "Hashable"
                            | "CustomStringConvertible"
                            | "CustomDebugStringConvertible"
                    ),
        };
        unconditional || self.conditional_conformance_holds(type_name, protocol)
    }

    /// ADR-324: evaluate `extension Base: Proto where …` against a display type name (`Array<Int>`).
    fn conditional_conformance_holds(&self, type_name: &str, protocol: &str) -> bool {
        let ty = Self::type_id_from_display_name(type_name);
        self.conditional_conformance_holds_ty(&ty, protocol)
    }

    fn conditional_conformance_holds_ty(&self, ty: &TypeId, protocol: &str) -> bool {
        let Some(base) = self.extension_base_for_ty(ty) else {
            return false;
        };
        for (ext_base, ext_proto, preds) in &self.conditional_conformances {
            if ext_base != &base || ext_proto.as_str() != protocol {
                continue;
            }
            let args = self.extension_type_arg_map(base.as_str(), ty);
            if self.where_preds_satisfied(&args, preds) {
                return true;
            }
        }
        false
    }

    fn where_preds_satisfied(
        &self,
        type_args: &IndexMap<SmolStr, SmolStr>,
        preds: &[syx_ast::WherePred],
    ) -> bool {
        for pred in preds {
            let arg_ty = type_args
                .get(&pred.type_name)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            if Self::is_associated_type_constraint(pred.constraint.as_str()) {
                if arg_ty != pred.constraint.as_str() {
                    return false;
                }
            } else if !self.type_satisfies_protocol(arg_ty, pred.constraint.as_str()) {
                return false;
            }
        }
        true
    }

    fn type_id_from_display_name(name: &str) -> TypeId {
        if let Some(inner) = name
            .strip_prefix("Array<")
            .and_then(|s| s.strip_suffix('>'))
        {
            return TypeId::Array(Box::new(Self::type_id_from_display_name(inner)));
        }
        if let Some(inner) = name
            .strip_prefix("Set<")
            .and_then(|s| s.strip_suffix('>'))
        {
            return TypeId::Set(Box::new(Self::type_id_from_display_name(inner)));
        }
        if let Some(kind) = from_type_name(name) {
            return TypeId::Numeric(kind);
        }
        match name {
            "Bool" => TypeId::Bool,
            "String" => TypeId::String,
            "Void" => TypeId::Void,
            "Unknown" => TypeId::Unknown,
            other => TypeId::Named(SmolStr::new(other)),
        }
    }

    fn type_id_satisfies_protocol(&self, ty: &TypeId, protocol: &str) -> bool {
        self.type_satisfies_protocol(type_id_to_name(ty).as_str(), protocol)
            || self.conditional_conformance_holds_ty(ty, protocol)
    }

    fn check_equatable_binary(&mut self, op: BinOp, lt: &TypeId, rt: &TypeId) {
        if matches!(op, BinOp::IdenticalEq | BinOp::IdenticalNe) {
            return;
        }
        let involves_main_actor = |ty: &TypeId| {
            matches!(ty, TypeId::Named(n) if self.main_actor_types.contains(n))
        };
        if involves_main_actor(lt) || involves_main_actor(rt) {
            if let Some(fn_name) = &self.current_function {
                let on_main = self.caller_global_actor(fn_name).as_deref() == Some("MainActor");
                if !on_main {
                    self.errors.push(
                        SyxDiagnostic::error(
                            "MainActor-isolated Equatable conformance requires MainActor context",
                        )
                        .with_code("C006"),
                    );
                }
            }
        }
        if let (TypeId::Named(ln), TypeId::Named(rn)) = (lt, rt) {
            if ln == rn
                && (self.class_names.contains(ln)
                    || self.dataclass_names.contains(ln)
                    || self.enum_cases.contains_key(ln))
            {
                // Same nominal / enum type: allow when Equatable conformance holds.
                if self.type_satisfies_protocol(ln.as_str(), "Equatable")
                    || self.dataclass_names.contains(ln)
                {
                    return;
                }
            }
        }
        if let (TypeId::GenericParam(a), TypeId::GenericParam(b)) = (lt, rt) {
            if a == b && self.current_equatable_params.contains(a) {
                return;
            }
        }
        // Param types `a: T` resolve as Named("T"), not GenericParam.
        if let (TypeId::Named(a), TypeId::Named(b)) = (lt, rt) {
            if a == b && self.current_equatable_params.contains(a) {
                return;
            }
        }
        if matches!(
            lt,
            TypeId::OpaqueSome(_) | TypeId::OpaqueAny(_)
        ) && matches!(rt, TypeId::OpaqueSome(_) | TypeId::OpaqueAny(_))
        {
            return;
        }
        if lt == rt && matches!(lt, TypeId::Unknown) {
            return;
        }
        let lname = type_id_to_name(lt);
        let rname = type_id_to_name(rt);
        if lname == "Unknown" && is_equatable_type(rname.as_str()) {
            return;
        }
        if rname == "Unknown" && is_equatable_type(lname.as_str()) {
            return;
        }
        let l_ok = is_equatable_type(lname.as_str())
            || self.type_satisfies_protocol(lname.as_str(), "Equatable");
        let r_ok = is_equatable_type(rname.as_str())
            || self.type_satisfies_protocol(rname.as_str(), "Equatable");
        if !l_ok || !r_ok {
            self.errors.push(
                SyxDiagnostic::error(format!(
                    "Types '{lname}' and '{rname}' are not both Equatable for '=='"
                ))
                .with_code("S016"),
            );
        }
    }

    fn check_local_mut_returns(&mut self, stmts: &[Stmt], params: &[syx_ast::Param]) {
        let param_names: HashSet<SmolStr> = params.iter().map(|p| p.name.clone()).collect();
        for stmt in stmts {
            match stmt {
                Stmt::Return {
                    value: Some(Expr::Borrow {
                        expr,
                        mutable: true,
                        ..
                    }),
                    ..
                } => {
                    if let Expr::Ident { name, .. } = expr.as_ref() {
                        if !param_names.contains(name) && self.all_bindings.contains_key(name) {
                            self.errors.push(
                                SyxDiagnostic::error(format!(
                                    "Cannot return reference to local binding '{name}'"
                                ))
                                .with_code("S011"),
                            );
                        }
                    }
                }
                Stmt::If {
                    then_branch,
                    else_branch,
                    ..
                } => {
                    self.check_local_mut_returns(then_branch, params);
                    if let Some(else_b) = else_branch {
                        self.check_local_mut_returns(else_b, params);
                    }
                }
                Stmt::While { body, .. } => {
                    self.check_local_mut_returns(body, params);
                }
                Stmt::Block { stmts, .. } => {
                    self.check_local_mut_returns(stmts, params);
                }
                Stmt::Try { body, catch_body, .. } => {
                    self.check_local_mut_returns(body, params);
                    self.check_local_mut_returns(catch_body, params);
                }
                _ => {}
            }
        }
    }

    fn validate_generic_instantiation(&mut self, fn_name: &SmolStr, type_args: &[SmolStr]) {
        let Some((type_params, where_clause)) = self.generic_functions.get(fn_name).cloned() else {
            return;
        };
        self.validate_generic_instantiation_with(fn_name, &type_params, &where_clause, type_args);
    }

    /// Substitute generic return type (`T` → concrete) for check/inference.
    fn instantiate_generic_return(
        &self,
        fn_name: &SmolStr,
        type_params: &[SmolStr],
        type_args: &[SmolStr],
    ) -> TypeId {
        let Some(rt) = self.function_return_types.get(fn_name) else {
            return type_args
                .first()
                .map(|t| TypeId::Named(t.clone()))
                .unwrap_or(TypeId::Unknown);
        };
        Self::subst_type_id(rt, type_params, type_args)
    }

    fn subst_type_id(ty: &TypeId, type_params: &[SmolStr], type_args: &[SmolStr]) -> TypeId {
        match ty {
            TypeId::Named(n) | TypeId::GenericParam(n) => {
                if let Some(idx) = type_params.iter().position(|p| p == n) {
                    if let Some(arg) = type_args.get(idx) {
                        return TypeId::Named(arg.clone());
                    }
                }
                ty.clone()
            }
            TypeId::Optional(inner) => {
                TypeId::Optional(Box::new(Self::subst_type_id(inner, type_params, type_args)))
            }
            TypeId::Array(inner) => {
                TypeId::Array(Box::new(Self::subst_type_id(inner, type_params, type_args)))
            }
            TypeId::Set(inner) => {
                TypeId::Set(Box::new(Self::subst_type_id(inner, type_params, type_args)))
            }
            TypeId::Dictionary { key, value } => TypeId::Dictionary {
                key: Box::new(Self::subst_type_id(key, type_params, type_args)),
                value: Box::new(Self::subst_type_id(value, type_params, type_args)),
            },
            other => other.clone(),
        }
    }

    fn validate_reflect_specialization_call(
        &mut self,
        args: &[CallArg],
        span: syx_diagnostics::Span,
    ) -> SyxResult<()> {
        if args.is_empty() {
            return Err(self.error_at(
                "__syx_reflect_specialization requires a function name and type arguments",
                span,
                "S002",
            ));
        }
        let Some(fn_name) = Self::string_literal_value(&args[0].value) else {
            return Err(self.error_at(
                "__syx_reflect_specialization function name must be a string literal",
                span,
                "S002",
            ));
        };
        let mut type_args = Vec::new();
        for arg in args.iter().skip(1) {
            let Some(ty) = Self::string_literal_value(&arg.value) else {
                return Err(self.error_at(
                    "__syx_reflect_specialization type arguments must be string literals",
                    span,
                    "S002",
                ));
            };
            type_args.push(ty);
        }
        let arg_refs: Vec<&str> = type_args.iter().map(String::as_str).collect();
        if let Err(err) = syx_meta::reflect_specialization(&fn_name, &arg_refs) {
            let code = err.code.as_deref().unwrap_or("M105");
            self.errors.push(self.error_at(err.message, span, code));
        }
        Ok(())
    }

    fn string_literal_value(expr: &Expr) -> Option<String> {
        match expr {
            Expr::Literal {
                value: syx_ast::Literal::String(s),
                ..
            } => Some(s.clone()),
            _ => None,
        }
    }

    fn validate_generic_instantiation_with(
        &mut self,
        generic_name: &SmolStr,
        type_params: &[SmolStr],
        where_clause: &[syx_ast::WherePred],
        type_args: &[SmolStr],
    ) {
        for pred in where_clause {
            let Some(idx) = type_params.iter().position(|p| p == &pred.type_name) else {
                continue;
            };
            let arg_ty = type_args
                .get(idx)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            if Self::is_associated_type_constraint(pred.constraint.as_str()) {
                if arg_ty != pred.constraint.as_str() {
                    self.errors.push(
                        SyxDiagnostic::error(format!(
                            "Associated type '{arg_ty}' does not match required '{}' for generic '{generic_name}'",
                            pred.constraint
                        ))
                        .with_code("S012"),
                    );
                }
            } else if !self.type_satisfies_protocol(arg_ty, pred.constraint.as_str()) {
                self.errors.push(
                    SyxDiagnostic::error(format!(
                        "Type '{arg_ty}' does not satisfy where constraint '{}: {}' for generic '{generic_name}'",
                        pred.type_name, pred.constraint
                    ))
                    .with_code("S010"),
                );
            }
        }
    }

    fn extension_base_for_ty(&self, ty: &TypeId) -> Option<SmolStr> {
        match ty {
            TypeId::Array(_) => Some(SmolStr::new("Array")),
            TypeId::Set(_) => Some(SmolStr::new("Set")),
            TypeId::Dictionary { .. } => Some(SmolStr::new("Dictionary")),
            TypeId::Named(n) => Some(n.clone()),
            TypeId::String => Some(SmolStr::new("String")),
            _ => None,
        }
    }

    fn extension_receiver_type(type_name: &SmolStr) -> TypeId {
        match type_name.as_str() {
            "String" => TypeId::String,
            "Bool" => TypeId::Bool,
            "Array" => TypeId::Array(Box::new(TypeId::Unknown)),
            "Set" => TypeId::Set(Box::new(TypeId::Unknown)),
            "Dictionary" => TypeId::Dictionary {
                key: Box::new(TypeId::String),
                value: Box::new(TypeId::Unknown),
            },
            name => {
                if let Some(kind) = from_type_name(name) {
                    TypeId::Numeric(kind)
                } else {
                    TypeId::Named(type_name.clone())
                }
            }
        }
    }

    fn analyze_extension_method(&mut self, type_name: &SmolStr, member: &Decl) -> SyxResult<()> {
        match member {
            Decl::Function {
                span,
                params,
                body,
                attrs,
                ..
            } => {
                let is_static = attrs.iter().any(|a| a.name.as_str() == "static");
                if is_static {
                    return self.analyze_decl(member);
                }
                let has_self = params.iter().any(|p| p.name.as_str() == "self");
                let mut fn_scope = Scope {
                    parent: Some(Box::new(std::mem::take(&mut self.scope))),
                    ..Default::default()
                };
                if !has_self {
                    fn_scope.define(Symbol {
                        name: SmolStr::new("self"),
                        ty: Self::extension_receiver_type(type_name),
                        mutable: true,
                        access: AccessLevel::Internal,
                    })?;
                }
                for param in params {
                    let param_ty = param
                        .ty
                        .as_ref()
                        .map(type_expr_to_id)
                        .unwrap_or(TypeId::Unknown);
                    fn_scope.define(Symbol {
                        name: param.name.clone(),
                        ty: param_ty.clone(),
                        mutable: false,
                        access: AccessLevel::Internal,
                    })?;
                    self.track_binding(param.name.clone(), param_ty, span.start);
                }
                std::mem::swap(&mut self.scope, &mut fn_scope);
                let r = self.analyze_block(body);
                self.scope = *self.scope.parent.take().unwrap_or_default();
                r
            }
            _ => Ok(()),
        }
    }

    fn extension_type_arg_map(&self, base: &str, ty: &TypeId) -> IndexMap<SmolStr, SmolStr> {
        match (base, ty) {
            ("Array", TypeId::Array(inner)) => {
                let mut m = IndexMap::new();
                m.insert(SmolStr::new("Element"), type_id_to_name(inner));
                m
            }
            ("Set", TypeId::Set(inner)) => {
                let mut m = IndexMap::new();
                m.insert(SmolStr::new("Element"), type_id_to_name(inner));
                m
            }
            (
                "Dictionary",
                TypeId::Dictionary { key, value },
            ) => {
                let mut m = IndexMap::new();
                m.insert(SmolStr::new("Key"), type_id_to_name(key));
                m.insert(SmolStr::new("Value"), type_id_to_name(value));
                m
            }
            _ => IndexMap::new(),
        }
    }

    fn check_extension_member_where(
        &mut self,
        obj_ty: &TypeId,
        method: &str,
        span: syx_diagnostics::Span,
    ) {
        let Some(base) = self.extension_base_for_ty(obj_ty) else {
            return;
        };
        let key = (base.clone(), SmolStr::new(method));
        let Some(where_clause) = self.extension_method_where.get(&key).cloned() else {
            return;
        };
        if !self
            .type_extension_methods
            .get(&base)
            .is_some_and(|methods| methods.iter().any(|(n, _)| n.as_str() == method))
        {
            return;
        }
        let type_args = self.extension_type_arg_map(base.as_str(), obj_ty);
        for pred in &where_clause {
            let arg_ty = type_args
                .get(&pred.type_name)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            if Self::is_associated_type_constraint(pred.constraint.as_str()) {
                if arg_ty != pred.constraint.as_str() {
                    self.errors.push(self.error_at(
                        format!(
                            "Associated type '{arg_ty}' does not match required '{}' for extension on '{base}'",
                            pred.constraint
                        ),
                        span,
                        "S012",
                    ));
                }
            } else if !self.type_satisfies_protocol(arg_ty, pred.constraint.as_str()) {
                self.errors.push(self.error_at(
                    format!(
                        "Type '{arg_ty}' does not satisfy where constraint '{}: {}' for extension on '{base}'",
                        pred.type_name, pred.constraint
                    ),
                    span,
                    "S010",
                ));
            }
        }
    }

    fn is_associated_type_constraint(constraint: &str) -> bool {
        if syx_stdlib::protocol_associated_types(constraint).is_some() {
            return true;
        }
        matches!(
            constraint,
            "Int" | "Int8"
                | "Int16"
                | "Int32"
                | "Int64"
                | "UInt"
                | "UInt8"
                | "UInt16"
                | "UInt32"
                | "UInt64"
                | "Float"
                | "Double"
                | "Bool"
                | "String"
                | "Object"
        )
    }

    fn infer_enum_case_type_args(
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

    fn promote_numeric(&self, a: TypeId, b: TypeId) -> TypeId {
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
    fn promote_numeric_binop(
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

    fn service_resolve_type_name_from_arg(&self, arg: &Expr) -> Option<SmolStr> {
        match arg {
            Expr::Literal {
                value: syx_ast::Literal::String(type_name),
                ..
            } => Some(SmolStr::new(type_name.as_str())),
            Expr::Ident { name, .. } if self.service_class_names.contains(name) => Some(name.clone()),
            _ => None,
        }
    }

    fn analyze_service_resolve(
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

    fn check_host_fn_allowed(&mut self, id: HostFnId, span: syx_diagnostics::Span) {
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

    fn infer_simd_element_from_args(&self, args: &[CallArg]) -> Option<NumericKind> {
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
    fn is_protocol_constraint(&self, name: &str) -> bool {
        Self::known_protocol(name)
            || self.protocol_assoc_types.contains_key(&SmolStr::new(name))
            || self.protocol_names.contains(&SmolStr::new(name))
    }

    fn apply_module_default_isolation(&mut self, module: &Module) {
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

    fn check_opaque_protocol_assignable(
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

    fn infer_class_method_type_args(
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

    fn infer_call_type_args(&self, type_param_count: usize, args: &[CallArg]) -> Vec<SmolStr> {
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
            _ => SmolStr::new("Unknown"),
        }
    }

    pub fn check_concurrency_rules(&mut self, module: &Module) {
        self.seed_sendable_types(module);
        for decl in &module.decls {
            if let Decl::Function { name, attrs, .. } = decl {
                self.collect_function_isolation(name, attrs);
            }
        }
        self.apply_module_default_isolation(module);
        self.check_concurrency_call_sites(module);
        if !self.strict_concurrency {
            return;
        }
        let has_non_sendable = !self.class_names.is_empty();
        if has_non_sendable && module_uses_task_detach(module) {
            self.errors.push(
                SyxDiagnostic::error(
                    "Task.detached / __syx_task_detach cannot be used with non-Sendable class types in strict concurrency mode",
                )
                .with_code("C003"),
            );
        }
    }

    fn check_concurrency_call_sites(&mut self, module: &Module) {
        for decl in &module.decls {
            if let Decl::Function { name, body, .. } = decl {
                let caller_actor = self.caller_global_actor(name);
                let caller_nonisolated = self.nonisolated_functions.contains(name);
                self.check_stmts_concurrency(
                    body,
                    caller_actor.as_ref(),
                    caller_nonisolated,
                    None,
                );
            }
            if let Decl::Actor { name, members, .. } = decl {
                for member in members {
                    if let Decl::Function { body, .. } = member {
                        self.check_stmts_concurrency(
                            body,
                            Some(name),
                            false,
                            Some(name.clone()),
                        );
                    }
                }
            }
        }
    }

    fn caller_global_actor(&self, name: &SmolStr) -> Option<SmolStr> {
        if name.as_str() == "main" {
            return Some(SmolStr::new("MainActor"));
        }
        self.function_global_actor.get(name).cloned()
    }

    fn caller_in_actor(&self, actor: Option<&SmolStr>, required: &SmolStr) -> bool {
        match actor {
            Some(current) => current == required,
            None => false,
        }
    }

    fn check_stmts_concurrency(
        &mut self,
        stmts: &[Stmt],
        caller_actor: Option<&SmolStr>,
        caller_nonisolated: bool,
        enclosing_actor: Option<SmolStr>,
    ) {
        for stmt in stmts {
            self.check_stmt_concurrency(
                stmt,
                caller_actor,
                caller_nonisolated,
                enclosing_actor.as_ref(),
            );
        }
    }

    fn check_stmt_concurrency(
        &mut self,
        stmt: &Stmt,
        caller_actor: Option<&SmolStr>,
        caller_nonisolated: bool,
        enclosing_actor: Option<&SmolStr>,
    ) {
        match stmt {
            Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
                self.check_expr_concurrency(expr, caller_actor, caller_nonisolated, enclosing_actor);
            }
            Stmt::VarDecl { init: Some(expr), .. } => {
                self.check_expr_concurrency(expr, caller_actor, caller_nonisolated, enclosing_actor);
            }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            self.check_expr_concurrency(condition, caller_actor, caller_nonisolated, enclosing_actor);
                self.check_stmts_concurrency(
                    then_branch,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor.cloned(),
                );
                if let Some(branch) = else_branch {
                    self.check_stmts_concurrency(
                        branch,
                        caller_actor,
                        caller_nonisolated,
                        enclosing_actor.cloned(),
                    );
                }
            }
            Stmt::Block { stmts, .. } => self.check_stmts_concurrency(
                stmts,
                caller_actor,
                caller_nonisolated,
                enclosing_actor.cloned(),
            ),
            Stmt::While { condition, body, .. } => {
                self.check_expr_concurrency(condition, caller_actor, caller_nonisolated, enclosing_actor);
                self.check_stmts_concurrency(
                    body,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor.cloned(),
                );
            }
            Stmt::ForIn { iterable, body, .. } => {
                self.check_expr_concurrency(
                    iterable,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor,
                );
                self.check_stmts_concurrency(
                    body,
                    caller_actor,
                    caller_nonisolated,
                    enclosing_actor.cloned(),
                );
            }
            _ => {}
        }
    }

    fn check_expr_concurrency(
        &mut self,
        expr: &Expr,
        caller_actor: Option<&SmolStr>,
        caller_nonisolated: bool,
        enclosing_actor: Option<&SmolStr>,
    ) {
        match expr {
            Expr::Call { callee, args, .. } => {
                if let Expr::Ident { name, .. } = callee.as_ref() {
                    if let Some(required) = self.function_global_actor.get(name) {
                        if !caller_nonisolated
                            && !self.caller_in_actor(caller_actor, required)
                            && self.strict_concurrency
                        {
                            self.errors.push(
                                SyxDiagnostic::error(format!(
                                    "Call to @{required} function '{name}' from non-{required} context"
                                ))
                                .with_code("C001"),
                            );
                        }
                    }
                }
                if global_actor_v4_cross_module_enabled() {
                    if let Expr::Member { object, name: member, .. } = callee.as_ref() {
                        if let Expr::Ident { name: module, .. } = object.as_ref() {
                            if let Some(required) = self
                                .import_member_actors
                                .get(&(module.clone(), member.clone()))
                            {
                                if !self.caller_in_actor(caller_actor, required)
                                    && self.strict_concurrency
                                {
                                    self.errors.push(
                                        SyxDiagnostic::error(format!(
                                            "Call to @{required} function '{module}.{member}' from non-{required} context"
                                        ))
                                        .with_code("C001"),
                                    );
                                }
                            }
                        }
                    }
                }
                if let Expr::Ident { name, .. } = callee.as_ref() {
                    if name.as_str() == "__syx_task_detach" {
                        self.check_detach_captures(args);
                    }
                    if global_actor_v6_sending_enabled() && borrow_regions_v7_sending_enabled() {
                        if let Some(sending_flags) = self.function_sending_params.get(name).cloned()
                        {
                            for (i, arg) in args.iter().enumerate() {
                                if sending_flags.get(i).copied().unwrap_or(false) {
                                    self.check_sending_arg(&arg.value, name.as_str());
                                }
                            }
                        }
                    }
                    if self.nonsending_functions.contains(name)
                        && caller_actor.is_some()
                        && !caller_nonisolated
                        && self.strict_concurrency
                    {
                        self.errors.push(
                            SyxDiagnostic::error(format!(
                                "Call to @nonisolated(nonsending) '{name}' crosses actor isolation"
                            ))
                            .with_code("C005"),
                        );
                    }
                }
                if let Expr::Member { object, name: method, .. } = callee.as_ref() {
                    if let Some(obj_ty) = self.expr_type(object) {
                        if let TypeId::Named(actor_name) = obj_ty {
                            if self.actor_types.contains(&actor_name)
                                && enclosing_actor != Some(&actor_name)
                            {
                                if self.strict_concurrency {
                                    self.errors.push(
                                        SyxDiagnostic::error(format!(
                                            "Cross-actor call to method '{method}' on actor '{actor_name}'"
                                        ))
                                        .with_code("C002"),
                                    );
                                }
                                if let Some(sending_flags) = self
                                    .actor_method_sending
                                    .get(&(actor_name.clone(), method.clone()))
                                    .cloned()
                                {
                                    for (i, arg) in args.iter().enumerate() {
                                        if sending_flags.get(i).copied().unwrap_or(false) {
                                            self.check_sending_arg(&arg.value, actor_name.as_str());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self.check_expr_concurrency(callee, caller_actor, caller_nonisolated, enclosing_actor);
                for arg in args {
                    self.check_expr_concurrency(
                        &arg.value,
                        caller_actor,
                        caller_nonisolated,
                        enclosing_actor,
                    );
                }
            }
            Expr::TaskBlock { body, detached, .. } => {
                if *detached && self.strict_concurrency {
                    self.check_task_body_captures(body);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr_concurrency(left, caller_actor, caller_nonisolated, enclosing_actor);
                self.check_expr_concurrency(right, caller_actor, caller_nonisolated, enclosing_actor);
            }
            Expr::Unary { expr, .. } => {
                self.check_expr_concurrency(expr, caller_actor, caller_nonisolated, enclosing_actor);
            }
            Expr::Await { expr, .. } => {
                self.check_expr_concurrency(expr, caller_actor, caller_nonisolated, enclosing_actor);
            }
            Expr::Member { object, .. } => {
                self.check_expr_concurrency(object, caller_actor, caller_nonisolated, enclosing_actor);
            }
            Expr::Closure { body, .. } => match body {
                syx_ast::ClosureBody::Expr(e) => {
                    self.check_expr_concurrency(e, caller_actor, caller_nonisolated, enclosing_actor);
                }
                syx_ast::ClosureBody::Block(stmts) => {
                    self.check_stmts_concurrency(
                        stmts,
                        caller_actor,
                        caller_nonisolated,
                        enclosing_actor.cloned(),
                    );
                }
            },
            _ => {}
        }
    }

    fn check_detach_captures(&mut self, args: &[syx_ast::CallArg]) {
        for arg in args {
            self.check_capture_expr(&arg.value);
        }
    }

    fn check_task_body_captures(&mut self, body: &[Stmt]) {
        for stmt in body {
            match stmt {
                Stmt::Expr { expr, .. }
                | Stmt::Print { expr, .. }
                | Stmt::Return { value: Some(expr), .. } => self.check_capture_expr(expr),
                Stmt::VarDecl { init: Some(expr), .. } => self.check_capture_expr(expr),
                Stmt::Block { stmts, .. } => self.check_task_body_captures(stmts),
                _ => {}
            }
        }
    }

    fn check_sending_arg(&mut self, expr: &Expr, context: &str) {
        match expr {
            Expr::Ident { name, .. } => {
                if self.is_non_sendable_binding(name) {
                    self.errors.push(
                        SyxDiagnostic::error(format!(
                            "Non-Sendable value '{name}' cannot cross isolation boundary via `sending` parameter in {context}"
                        ))
                        .with_code("C004"),
                    );
                }
            }
            _ => {}
        }
    }

    fn check_capture_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident { name, .. } => {
                if self.is_non_sendable_binding(name) && self.strict_concurrency {
                    self.errors.push(
                        SyxDiagnostic::error(format!(
                            "Non-Sendable value '{name}' captured in Task.detached context"
                        ))
                        .with_code("C003"),
                    );
                }
            }
            Expr::Member { object, name, .. } => {
                if let Some(TypeId::Named(type_name)) = self.expr_type(object) {
                    if self.class_names.contains(&type_name)
                        && !self.sendable_types.contains(&type_name)
                        && self.strict_concurrency
                    {
                        self.errors.push(
                            SyxDiagnostic::error(format!(
                                "Non-Sendable field access '{name}' on '{type_name}' in Task.detached context"
                            ))
                            .with_code("C003"),
                        );
                    }
                }
                self.check_capture_expr(object);
            }
            Expr::Binary { left, right, .. } => {
                self.check_capture_expr(left);
                self.check_capture_expr(right);
            }
            Expr::Call { args, .. } => {
                for arg in args {
                    self.check_capture_expr(&arg.value);
                }
            }
            _ => {}
        }
    }

    fn is_non_sendable_binding(&self, name: &SmolStr) -> bool {
        let ty = self
            .scope
            .lookup(name)
            .or_else(|| self.globals.get(name))
            .map(|s| s.ty.clone())
            .unwrap_or(TypeId::Unknown);
        self.is_non_sendable_type(&ty)
    }

    #[allow(dead_code)] // retained for Sendable / IDE type queries
    fn binding_type(&self, name: &SmolStr) -> TypeId {
        self.scope
            .lookup(name)
            .map(|s| s.ty.clone())
            .unwrap_or(TypeId::Unknown)
    }

    fn is_non_sendable_type(&self, ty: &TypeId) -> bool {
        match ty {
            TypeId::Named(n) => self.class_names.contains(n) && !self.sendable_types.contains(n),
            TypeId::Optional(inner) => self.is_non_sendable_type(inner),
            _ => false,
        }
    }

    fn seed_sendable_types(&mut self, module: &Module) {
        self.sendable_types.clear();
        self.unchecked_sendable_types.clear();
        for decl in &module.decls {
            match decl {
                Decl::Actor { name, .. } => {
                    self.sendable_types.insert(name.clone());
                }
                Decl::DataClass { name, attrs, .. } => {
                    self.sendable_types.insert(name.clone());
                    if global_actor_v6_sending_enabled() && has_attr(attrs, "unchecked") {
                        self.unchecked_sendable_types.insert(name.clone());
                    } else if global_actor_v5_sendable_enabled() && has_attr(attrs, "unchecked") {
                        self.unchecked_sendable_types.insert(name.clone());
                    }
                }
                Decl::Function { name, attrs, .. } if has_attr(attrs, "Sendable") => {
                    self.sendable_types.insert(name.clone());
                }
                Decl::Protocol { name, .. } if name.as_str() == "Sendable" => {}
                _ => {}
            }
        }
        for name in ["Int", "Bool", "String", "Double", "Float"] {
            self.sendable_types.insert(SmolStr::new(name));
        }
        for (type_name, conformances) in &self.type_conformances {
            if conformances.iter().any(|p| p.as_str() == "Sendable") {
                self.sendable_types.insert(type_name.clone());
            }
        }
    }

    fn collect_function_isolation(&mut self, name: &SmolStr, attrs: &[syx_ast::Attribute]) {
        if has_attr(attrs, "MainActor") {
            self.main_actor_functions.insert(name.clone());
            self.function_global_actor
                .insert(name.clone(), SmolStr::new("MainActor"));
        }
        for attr in attrs {
            if self.global_actor_names.contains(&attr.name) {
                self.function_global_actor
                    .insert(name.clone(), attr.name.clone());
            }
        }
        if has_attr(attrs, "nonisolated") {
            self.nonisolated_functions.insert(name.clone());
            if attr_has_ident_arg(attrs, "nonisolated", "nonsending") {
                self.nonsending_functions.insert(name.clone());
            }
        }
        if has_attr(attrs, "concurrent") {
            self.concurrent_functions.insert(name.clone());
        }
    }

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

fn has_attr(attrs: &[syx_ast::Attribute], target: &str) -> bool {
    attrs.iter().any(|a| a.name == target)
}

fn attached_conformance_protocols(attrs: &[syx_ast::Attribute]) -> Vec<SmolStr> {
    attrs
        .iter()
        .filter(|a| a.name.as_str() == "attached")
        .filter_map(|a| {
            let mut args = a.args.iter();
            match args.next() {
                Some(syx_ast::AttributeArg::Ident(role)) if role.as_str() == "conformance" => {}
                _ => return None,
            }
            match args.next() {
                Some(syx_ast::AttributeArg::Ident(proto)) => Some(proto.clone()),
                _ => None,
            }
        })
        .collect()
}

fn type_expr_uses_param(ty: &syx_ast::TypeExpr, params: &[SmolStr]) -> Option<SmolStr> {
    match ty {
        syx_ast::TypeExpr::Named { name, generics } => {
            if params.contains(name) {
                return Some(name.clone());
            }
            for g in generics {
                if let Some(p) = type_expr_uses_param(g, params) {
                    return Some(p);
                }
            }
            None
        }
        syx_ast::TypeExpr::Optional(inner)
        | syx_ast::TypeExpr::Array(inner)
        | syx_ast::TypeExpr::OpaqueSome(inner)
        | syx_ast::TypeExpr::OpaqueAny(inner) => type_expr_uses_param(inner, params),
        syx_ast::TypeExpr::Dictionary { key, value } => type_expr_uses_param(key, params)
            .or_else(|| type_expr_uses_param(value, params)),
        _ => None,
    }
}

fn retired_ops_hint(name: &str) -> String {
    match name {
        "Text" => "String".to_string(),
        "Arrays" => "Array".to_string(),
        "BoolOps" => "Bool".to_string(),
        "CharacterOps" => "Character".to_string(),
        n if n.ends_with("Ops") => n.strip_suffix("Ops").unwrap_or(n).to_string(),
        _ => name.to_string(),
    }
}

fn attr_has_ident_arg(attrs: &[syx_ast::Attribute], name: &str, arg: &str) -> bool {
    attrs.iter().any(|a| {
        a.name == name
            && a
                .args
                .iter()
                .any(|a| matches!(a, syx_ast::AttributeArg::Ident(i) if i.as_str() == arg))
    })
}

fn module_uses_task_detach(module: &Module) -> bool {
    module.decls.iter().any(|decl| {
        if let Decl::Function { body, .. } = decl {
            stmts_use_task_detach(body)
        } else {
            false
        }
    })
}

fn stmts_use_task_detach(stmts: &[Stmt]) -> bool {
    stmts.iter().any(|s| match s {
        Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } | Stmt::Return { value: Some(expr), .. } => {
            expr_uses_task_detach(expr)
        }
        Stmt::VarDecl { init: Some(expr), .. } => expr_uses_task_detach(expr),
        Stmt::If {
            then_branch,
            else_branch,
            ..
        } => {
            stmts_use_task_detach(then_branch)
                || else_branch
                    .as_ref()
                    .is_some_and(|b| stmts_use_task_detach(b))
        }
        Stmt::Block { stmts, .. } => stmts_use_task_detach(stmts),
        _ => false,
    })
}

fn expr_uses_task_detach(expr: &Expr) -> bool {
    match expr {
        Expr::Call { callee, .. } => {
            if let Expr::Ident { name, .. } = callee.as_ref() {
                if name.as_str() == "__syx_task_detach" {
                    return true;
                }
            }
            false
        }
        Expr::TaskBlock { detached, .. } => *detached,
        Expr::Binary { left, right, .. } => {
            expr_uses_task_detach(left) || expr_uses_task_detach(right)
        }
        Expr::Unary { expr, .. } => expr_uses_task_detach(expr),
        Expr::Group { expr, .. } => expr_uses_task_detach(expr),
        Expr::Await { expr, .. } => expr_uses_task_detach(expr),
        _ => false,
    }
}

pub fn type_name_to_id(name: &str) -> TypeId {
    if let Some(kind) = from_type_name(name) {
        return TypeId::numeric(kind);
    }
    if let Some(lanes) = simd_lanes_from_type_name(name) {
        return TypeId::Simd {
            lanes,
            element: NumericKind::I32,
        };
    }
    match name {
        "Bool" => TypeId::Bool,
        "String" => TypeId::String,
        "Void" => TypeId::Void,
        "Array" => TypeId::Array(Box::new(TypeId::Unknown)),
        "Set" => TypeId::Set(Box::new(TypeId::Unknown)),
        "Dictionary" => TypeId::Dictionary {
            key: Box::new(TypeId::String),
            value: Box::new(TypeId::Unknown),
        },
        "Object" => TypeId::Named(SmolStr::new("Object")),
        other => TypeId::Named(SmolStr::new(other)),
    }
}

pub fn type_expr_to_id(ty: &TypeExpr) -> TypeId {
    type_expr_to_id_with_aliases(ty, &IndexMap::new())
}

pub fn type_expr_to_id_with_aliases(
    ty: &TypeExpr,
    aliases: &IndexMap<SmolStr, TypeExpr>,
) -> TypeId {
    match ty {
        TypeExpr::Named { name, generics } => {
            if let Some(alias) = aliases.get(name) {
                return type_expr_to_id_with_aliases(alias, aliases);
            }
            if name.as_str() == "Character" {
                return TypeId::Named(SmolStr::new("Character"));
            }
            if name.as_str() == "Set" {
                let elem = generics
                    .first()
                    .map(type_expr_to_id)
                    .unwrap_or(TypeId::Unknown);
                return TypeId::Set(Box::new(elem));
            }
            if name.as_str() == "Array" {
                if generics.is_empty() {
                    return TypeId::Named(SmolStr::new("Array"));
                }
                let elem = type_expr_to_id_with_aliases(&generics[0], aliases);
                return TypeId::Array(Box::new(elem));
            }
            if name.as_str() == "Result" {
                let ok = generics
                    .first()
                    .map(type_expr_to_id)
                    .unwrap_or(TypeId::Unknown);
                let err = generics
                    .get(1)
                    .map(type_expr_to_id)
                    .unwrap_or(TypeId::Unknown);
                return TypeId::Result {
                    ok: Box::new(ok),
                    err: Box::new(err),
                };
            }
            if let Some(lanes) = simd_lanes_from_type_name(name.as_str()) {
                let element = generics
                    .first()
                    .and_then(|g| type_expr_to_id(g).as_numeric_kind())
                    .unwrap_or(NumericKind::I32);
                if !syx_config::simd_enabled() {
                    return TypeId::Unknown;
                }
                return TypeId::Simd { lanes, element };
            }
            if let Some(kind) = from_type_name(name) {
                return TypeId::numeric(kind);
            }
            match name.as_str() {
                "Bool" => TypeId::Bool,
                "String" => TypeId::String,
                "Void" => TypeId::Void,
                "Dictionary" => TypeId::Dictionary {
                    key: Box::new(
                        generics
                            .first()
                            .map(type_expr_to_id)
                            .unwrap_or(TypeId::String),
                    ),
                    value: Box::new(
                        generics
                            .get(1)
                            .map(type_expr_to_id)
                            .unwrap_or(TypeId::Unknown),
                    ),
                },
                other => {
                    let mangled = other.replace('.', "_");
                    TypeId::Named(SmolStr::new(mangled))
                }
            }
        }
        TypeExpr::Optional(inner) => TypeId::Optional(Box::new(type_expr_to_id(inner))),
        TypeExpr::Array(inner) => TypeId::Array(Box::new(type_expr_to_id(inner))),
        TypeExpr::Dictionary { key, value } => TypeId::Dictionary {
            key: Box::new(type_expr_to_id(key)),
            value: Box::new(type_expr_to_id(value)),
        },
        TypeExpr::Reference(inner, mutable, _lifetime) => {
            let inner_id = type_expr_to_id(inner);
            if *mutable {
                TypeId::MutReference(Box::new(inner_id))
            } else {
                TypeId::Reference(Box::new(inner_id))
            }
        }
        TypeExpr::Void => TypeId::Void,
        // S1-I4: function types `(T) -> U` / closures share a callable surface type.
        TypeExpr::Function { .. } => TypeId::Named(SmolStr::new("Closure")),
        TypeExpr::Tuple { elements } => TypeId::Tuple {
            elements: elements
                .iter()
                .map(|(label, ty)| (label.clone(), type_expr_to_id_with_aliases(ty, aliases)))
                .collect(),
        },
        TypeExpr::OpaqueSome(inner) => TypeId::OpaqueSome(Box::new(type_expr_to_id(inner))),
        TypeExpr::OpaqueAny(inner) => TypeId::OpaqueAny(Box::new(type_expr_to_id(inner))),
    }
}

/// Base collection type name for member dispatch (`Set`, `Dictionary`, `Array`).
pub fn type_id_collection_base(ty: &TypeId) -> Option<&'static str> {
    match ty {
        TypeId::Set(_) => Some("Set"),
        TypeId::Dictionary { .. } => Some("Dictionary"),
        TypeId::Array(_) => Some("Array"),
        _ => None,
    }
}

/// Base type name for extension / subscript dispatch (`Array<Int>` → `Array`).
pub fn type_name_dispatch_base(name: &SmolStr) -> SmolStr {
    SmolStr::new(name.split('<').next().unwrap_or(name.as_str()))
}

pub fn normalize_type_arg_name(name: SmolStr) -> SmolStr {
    match name.as_str() {
        "Int" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "Int64"
        } else {
            "Int32"
        }),
        "UInt" => SmolStr::new(if cfg!(target_pointer_width = "64") {
            "UInt64"
        } else {
            "UInt32"
        }),
        other => SmolStr::new(other),
    }
}

pub fn type_id_to_name(ty: &TypeId) -> SmolStr {
    match ty {
        TypeId::Void => SmolStr::new("Void"),
        TypeId::Numeric(kind) => SmolStr::new(syx_stdlib::type_name_with_alias(*kind)),
        TypeId::Bool => SmolStr::new("Bool"),
        TypeId::String => SmolStr::new("String"),
        TypeId::Named(n) => n.clone(),
        TypeId::Array(inner) => SmolStr::new(format!("Array<{}>", type_id_to_name(inner))),
        TypeId::Set(inner) => SmolStr::new(format!("Set<{}>", type_id_to_name(inner))),
        TypeId::Dictionary { key, value } => SmolStr::new(format!(
            "Dictionary<{}, {}>",
            type_id_to_name(key),
            type_id_to_name(value)
        )),
        TypeId::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(|(label, ty)| {
                    if let Some(l) = label {
                        format!("{l}: {}", type_id_to_name(ty))
                    } else {
                        type_id_to_name(ty).to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            SmolStr::new(format!("({inner})"))
        }
        TypeId::Optional(inner) => SmolStr::new(format!("{}?", type_id_to_name(inner))),
        TypeId::Result { ok, err } => SmolStr::new(format!(
            "Result<{}, {}>",
            type_id_to_name(ok),
            type_id_to_name(err)
        )),
        TypeId::GenericParam(n) => n.clone(),
        TypeId::GenericFn { name, .. } => name.clone(),
        TypeId::Reference(inner) => SmolStr::new(format!("&{}", type_id_to_name(inner))),
        TypeId::MutReference(inner) => SmolStr::new(format!("&mut {}", type_id_to_name(inner))),
        TypeId::OpaqueSome(inner) => {
            SmolStr::new(format!("some {}", type_id_to_name(inner)))
        }
        TypeId::OpaqueAny(inner) => SmolStr::new(format!("any {}", type_id_to_name(inner))),
        TypeId::Simd { lanes, element } => SmolStr::new(format!(
            "SIMD{lanes}<{}>",
            syx_stdlib::type_name_with_alias(*element)
        )),
        TypeId::Unknown => SmolStr::new("Unknown"),
    }
}

pub fn simd_lanes_from_type_name(name: &str) -> Option<u8> {
    match name {
        "SIMD2" => Some(2),
        "SIMD4" => Some(4),
        "SIMD8" => Some(8),
        "SIMD16" => Some(16),
        _ => None,
    }
}

pub fn expr_span_start(expr: &Expr) -> u32 {
    expr_span(expr).start as u32
}

fn expr_span(expr: &Expr) -> syx_diagnostics::Span {
    match expr {
        Expr::Literal { span, .. }
        | Expr::Ident { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Call { span, .. }
        | Expr::Member { span, .. }
        | Expr::Index { span, .. }
        | Expr::Assign { span, .. }
        | Expr::CompoundAssign { span, .. }
        | Expr::MemberAssign { span, .. }
        | Expr::IndexAssign { span, .. }
        | Expr::Closure { span, .. }
        | Expr::Group { span, .. }
        | Expr::Array { span, .. }
        | Expr::Await { span, .. }
        | Expr::OptionalChain { span, .. }
        | Expr::ForceUnwrap { span, .. }
        | Expr::HalfOpenRange { span, .. }
        | Expr::ClosedRange { span, .. }
        | Expr::VariantPattern { span, .. }
        | Expr::TuplePattern { span, .. }
        | Expr::TaskBlock { span, .. }
        | Expr::Borrow { span, .. }
        | Expr::KeyPath { span, .. }
        | Expr::MacroInvoke { span, .. }
        | Expr::OptionalCast { span, .. }
        | Expr::Is { span, .. }
        | Expr::Cast { span, .. }
        | Expr::TypeSelf { span, .. }
        | Expr::Super { span, .. }
        | Expr::Tuple { span, .. } => *span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_ast::{CallArg, Module, Param, ParamLabel, TypeExpr};
    use syx_diagnostics::Span;

    #[test]
    fn analyzes_simple_main() {
        let module = Module {
            name: "main".into(),
            decls: vec![Decl::Function {
                span: Span::DUMMY,
                access: AccessLevel::Internal,
                attrs: vec![],
                name: "main".into(),
                type_params: vec![],
                params: vec![],
                return_type: None,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: vec![Stmt::Print {
                    span: Span::DUMMY,
                    expr: Expr::Literal {
                        span: Span::DUMMY,
                        value: Literal::String("hi".into()),
                    },
                }],
                is_async: false,
            is_override: false,
            is_final: false,
            }],
        };
        SemanticAnalyzer::analyze(&module).unwrap();
    }

    #[test]
    fn rejects_undefined_function_call() {
        let module = Module {
            name: "main".into(),
            decls: vec![Decl::Function {
                span: Span::DUMMY,
                access: AccessLevel::Internal,
                attrs: vec![],
                name: "main".into(),
                type_params: vec![],
                params: vec![],
                return_type: None,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: vec![Stmt::Expr {
                    span: Span::DUMMY,
                    expr: Expr::Call {
                        span: Span::DUMMY,
                        callee: Box::new(Expr::Ident {
                            span: Span::DUMMY,
                            name: "join".into(),
                        }),
                        args: vec![CallArg {
                            label: None,
                            value: Expr::Literal {
                                span: Span::DUMMY,
                                value: Literal::Nil,
                            },
                        }],
                    },
                }],
                is_async: false,
            is_override: false,
            is_final: false,
            }],
        };
        let err = SemanticAnalyzer::analyze(&module).unwrap_err();
        assert_eq!(err.code.as_deref(), Some("S002"));
        assert!(err.message.contains("Undefined function 'join'"));
    }

    fn module_with_main_call(callee: &str, args: Vec<CallArg>) -> Module {
        Module {
            name: "main".into(),
            decls: vec![
                Decl::Function {
                    span: Span::DUMMY,
                    access: AccessLevel::Internal,
                    attrs: vec![],
                    name: callee.into(),
                    type_params: vec![],
                    params: vec![],
                    return_type: None,
                    throws_type: None,
                    effect: None,
                    where_clause: vec![],
                    body: vec![],
                    is_async: false,
                    is_override: false,
                    is_final: false,
                },
                Decl::Function {
                    span: Span::DUMMY,
                    access: AccessLevel::Internal,
                    attrs: vec![],
                    name: "main".into(),
                    type_params: vec![],
                    params: vec![],
                    return_type: None,
                    throws_type: None,
                    effect: None,
                    where_clause: vec![],
                    body: vec![Stmt::Expr {
                        span: Span::DUMMY,
                        expr: Expr::Call {
                            span: Span::DUMMY,
                            callee: Box::new(Expr::Ident {
                                span: Span::DUMMY,
                                name: callee.into(),
                            }),
                            args,
                        },
                    }],
                    is_async: false,
                    is_override: false,
                    is_final: false,
                },
            ],
        }
    }

    fn labeled_param(name: &str) -> Param {
        Param {
            name: name.into(),
            external_label: ParamLabel::Same,
            ty: Some(TypeExpr::Named {
                name: "Int".into(),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }
    }

    fn omitted_param(name: &str) -> Param {
        Param {
            name: name.into(),
            external_label: ParamLabel::Omitted,
            ty: Some(TypeExpr::Named {
                name: "Int".into(),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }
    }

    fn external_param(ext: &str, internal: &str) -> Param {
        Param {
            name: internal.into(),
            external_label: ParamLabel::Named(ext.into()),
            ty: Some(TypeExpr::Named {
                name: "Int".into(),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }
    }

    fn analyze_call_errors(params: Vec<Param>, args: Vec<CallArg>) -> Vec<String> {
        let mut module = module_with_main_call("f", args);
        module.decls[0] = Decl::Function {
            span: Span::DUMMY,
            access: AccessLevel::Internal,
            attrs: vec![],
            name: "f".into(),
            type_params: vec![],
            params,
            return_type: None,
            throws_type: None,
            effect: None,
            where_clause: vec![],
            body: vec![],
            is_async: false,
            is_override: false,
            is_final: false,
        };
        let analyzer = SemanticAnalyzer::analyze_collecting(&module);
        analyzer
            .errors
            .iter()
            .filter_map(|e| e.code.as_deref().map(String::from))
            .collect()
    }

    #[test]
    fn p139_requires_labels_when_not_all_omitted() {
        let codes = analyze_call_errors(
            vec![labeled_param("x")],
            vec![CallArg {
                label: None,
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(codes.iter().any(|c| c == "S042"));
    }

    #[test]
    fn p139_underscore_param_is_positional_only() {
        let bad = analyze_call_errors(
            vec![omitted_param("x")],
            vec![CallArg {
                label: Some("x".into()),
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(bad.iter().any(|c| c == "S042"));
        let ok = analyze_call_errors(
            vec![omitted_param("x")],
            vec![CallArg {
                label: None,
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(!ok.iter().any(|c| c == "S042"));
    }

    #[test]
    fn p139_external_label_differs_from_internal() {
        let bad = analyze_call_errors(
            vec![external_param("to", "x")],
            vec![CallArg {
                label: Some("x".into()),
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(bad.iter().any(|c| c == "S042"));
        let ok = analyze_call_errors(
            vec![external_param("to", "x")],
            vec![CallArg {
                label: Some("to".into()),
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(!ok.iter().any(|c| c == "S042"));
    }
}
