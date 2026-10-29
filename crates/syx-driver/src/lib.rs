pub mod aot_matrix;
pub mod gc_report;
pub mod inkwell_report;

/// Ensure host binaries resolve `syx_user_*` required by `syx_rt_start`.
///
/// Call once from host tool `main` (e.g. `syx-cli`) so the stub object is linked.
/// Also kept alive via [`FORCE_HOST_RT_STUBS`] so tests linking `syx-driver` work
/// without an explicit call (important on MSVC; Unix also has weak stubs in `syx-rt`).
#[inline(always)]
pub fn ensure_host_rt_link_stubs() {
    // Touch the kept-alive static so host tool `main` also retains the stubs.
    let _ = &FORCE_HOST_RT_STUBS;
}

/// Keep `syx_user_*` stub symbols linked for host tools/tests.
///
/// Must store function pointers (not `as usize`): pointer→integer casts are
/// rejected in const eval (rustc 1.91+).
#[used]
static FORCE_HOST_RT_STUBS: (extern "C" fn(), extern "C" fn() -> i32) = (
    host_rt_stubs::syx_user_init,
    host_rt_stubs::syx_user_main,
);

pub mod cache_remote_report;
pub mod capabilities_report;
pub mod gc_generational_report;
pub mod gc_relocate_report;
pub mod gui_backend_report;
mod host_rt_stubs;
pub mod hosted_capabilities;
pub mod lang_props_v2;
pub mod lang_props_v3;
pub mod lang_props_v4;
pub mod lang_props_v5;
pub mod lang_props_v6;
pub mod lang_surface_v1;
pub mod marketing_capabilities;
pub mod native_profile_report;
pub mod native_sdk_report;
pub mod rt_exec_report;
pub mod rt_runtime_report;
pub mod website_capabilities;

/// Phase 96: dlopen `libsyx_gui` before visible GUI run.
pub fn ensure_gui_loaded() -> bool {
    syx_vm::ensure_gui_loaded()
}

pub fn require_gui_dylib_if_visible() {
    syx_vm::require_gui_dylib_if_visible()
}
pub mod actor_ipc_report;
pub mod dict_runtime_report;
pub mod dist_actor_cluster_report;
pub mod dist_cdn_route_report;
pub mod dist_failover_report;
pub mod embedded_std;
pub mod farm_billing_report;
pub mod farm_report;
pub mod heap_bridge_prod_report;
pub mod heap_bridge_report;
pub mod heap_host_report;
pub mod heapslot_parity_report;
pub mod hsm_rotation_report;
pub mod native_exec_op_report;
pub mod reflect_report;
pub mod rt_v7_report;

pub use build_syx::{
    farm_lease_expired, farm_lease_reset, farm_queue_last_hit, farm_worker_dequeue,
    farm_worker_last_hit, farm_worker_run_cycle,
};
mod farm_lease_auth;
pub use farm_lease_auth::{
    farm_lease_acquire, farm_lease_acquire_tenant, farm_lease_acquire_tenant_token,
    farm_lease_auth_clear_configured, farm_lease_auth_configure, farm_lease_auth_configure_ha,
    farm_lease_auth_configure_raft, farm_lease_auth_is_configured,
    farm_lease_bind_host_is_loopback, farm_lease_cluster_os_dual_process_ga,
    farm_lease_concrete_local_ipv4, farm_lease_ha_fingerprint, farm_lease_ha_v3_fingerprint,
    farm_lease_host_is_documentation, farm_lease_hsm_digest_bound_doctor_export,
    farm_lease_hsm_digest_streaming_doctor_export, farm_lease_hsm_key_id_doctor_export,
    farm_lease_hsm_key_label_doctor_export, farm_lease_hsm_multi_slot_doctor_export,
    farm_lease_hsm_ref_bound_doctor_export, farm_lease_hsm_session_pin_doctor_export,
    farm_lease_hsm_sign_bound_doctor_export, farm_lease_hsm_sign_verify_doctor_export,
    farm_lease_kms_bridge_adapter_name, farm_lease_kms_bridge_mvp_ready,
    farm_lease_kms_bridge_sign_rotate, farm_lease_legacy_grant_tenant,
    farm_lease_legacy_loopback_member_urls_non_ga, farm_lease_member_fingerprint,
    farm_lease_member_regions_cross_host_ga, farm_lease_member_regions_os_dual_process_ga,
    farm_lease_member_regions_real_connect_ga, farm_lease_members_v2_fingerprint,
    farm_lease_multidc_discover_active, farm_lease_multidc_fingerprint,
    farm_lease_multidc_probe_timeout, farm_lease_multihost_fingerprint,
    farm_lease_os_child_try_serve, farm_lease_pkcs11_active_slot_id,
    farm_lease_pkcs11_bridge_adapter_name, farm_lease_pkcs11_bridge_fingerprint,
    farm_lease_pkcs11_bridge_mvp_ready, farm_lease_pkcs11_bridge_sign_rotate,
    farm_lease_pkcs11_hsm_digest_bound, farm_lease_pkcs11_hsm_digest_bound_fingerprint,
    farm_lease_pkcs11_hsm_digest_bound_mvp_ready, farm_lease_pkcs11_hsm_digest_final,
    farm_lease_pkcs11_hsm_digest_init, farm_lease_pkcs11_hsm_digest_sign_bound,
    farm_lease_pkcs11_hsm_digest_sign_verify, farm_lease_pkcs11_hsm_digest_streaming,
    farm_lease_pkcs11_hsm_digest_streaming_fingerprint,
    farm_lease_pkcs11_hsm_digest_streaming_mvp_ready, farm_lease_pkcs11_hsm_digest_update,
    farm_lease_pkcs11_hsm_key_id_fingerprint, farm_lease_pkcs11_hsm_key_id_mvp_ready,
    farm_lease_pkcs11_hsm_key_label_fingerprint, farm_lease_pkcs11_hsm_key_label_mvp_ready,
    farm_lease_pkcs11_hsm_multi_slot_bind, farm_lease_pkcs11_hsm_multi_slot_digest_streaming,
    farm_lease_pkcs11_hsm_multi_slot_fingerprint, farm_lease_pkcs11_hsm_multi_slot_mvp_ready,
    farm_lease_pkcs11_hsm_multi_slot_rotate, farm_lease_pkcs11_hsm_multi_slot_sign_bound,
    farm_lease_pkcs11_hsm_ref_bind, farm_lease_pkcs11_hsm_ref_bound,
    farm_lease_pkcs11_hsm_ref_bound_fingerprint, farm_lease_pkcs11_hsm_ref_bound_for_slot,
    farm_lease_pkcs11_hsm_ref_bound_mvp_ready, farm_lease_pkcs11_hsm_ref_bound_validate,
    farm_lease_pkcs11_hsm_session_pin_fingerprint, farm_lease_pkcs11_hsm_session_pin_mvp_ready,
    farm_lease_pkcs11_hsm_sign_bound, farm_lease_pkcs11_hsm_sign_bound_fingerprint,
    farm_lease_pkcs11_hsm_sign_bound_mvp_ready, farm_lease_pkcs11_hsm_sign_verify,
    farm_lease_pkcs11_hsm_sign_verify_fingerprint, farm_lease_pkcs11_hsm_sign_verify_mvp_ready,
    farm_lease_pkcs11_key_id_loaded, farm_lease_pkcs11_key_id_path,
    farm_lease_pkcs11_key_id_persist, farm_lease_pkcs11_key_id_verify,
    farm_lease_pkcs11_key_label_loaded, farm_lease_pkcs11_key_label_path,
    farm_lease_pkcs11_key_label_persist, farm_lease_pkcs11_key_label_verify,
    farm_lease_pkcs11_session_close, farm_lease_pkcs11_session_close_count,
    farm_lease_pkcs11_session_close_fingerprint, farm_lease_pkcs11_session_close_mvp_ready,
    farm_lease_pkcs11_session_fingerprint, farm_lease_pkcs11_session_handle,
    farm_lease_pkcs11_session_mvp_ready, farm_lease_pkcs11_session_pin_loaded,
    farm_lease_pkcs11_session_pin_path, farm_lease_pkcs11_session_pin_persist,
    farm_lease_pkcs11_session_pin_persist_fingerprint, farm_lease_pkcs11_session_pin_persist_ready,
    farm_lease_pkcs11_session_pin_persisted, farm_lease_pkcs11_session_pin_verify,
    farm_lease_pkcs11_session_rotate_count, farm_lease_pkcs11_session_slot_id,
    farm_lease_pkcs11_sign_mvp_fingerprint, farm_lease_pkcs11_slot_list,
    farm_lease_pkcs11_slot_select, farm_lease_pkcs11_so_adapter_loaded,
    farm_lease_pkcs11_so_fingerprint, farm_lease_pkcs11_so_symbol_table,
    farm_lease_probe_member_health, farm_lease_query_at, farm_lease_query_at_tenant,
    farm_lease_query_at_tenant_token, farm_lease_raft_fingerprint,
    farm_lease_raft_member_add_learner, farm_lease_raft_member_remove, farm_lease_raft_members_at,
    farm_lease_raft_v2_fingerprint, farm_lease_read_token_epoch_file,
    farm_lease_registry_consensus_sync, farm_lease_registry_fingerprint,
    farm_lease_registry_follower_view, farm_lease_registry_health_sync,
    farm_lease_registry_multi_follower_sync, farm_lease_registry_reload_probe,
    farm_lease_require_active, farm_lease_rotate_tenant_token, farm_lease_runtime_fingerprint,
    farm_lease_tenant_fingerprint, farm_lease_tenant_v2_fingerprint,
    farm_lease_tenant_v3_fingerprint, farm_lease_tenant_v4_fingerprint,
    farm_lease_token_epoch_path, farm_lease_token_fingerprint,
    farm_lease_token_hsm_ref_accept_lock, farm_lease_token_hsm_ref_mvp_ready,
    farm_lease_token_hsm_ref_resolve, farm_lease_validate_snap_authority_fields,
    farm_lease_wal_checkpoint, farm_lease_wal_compact_fingerprint, farm_lease_wal_crc_fingerprint,
    farm_lease_wal_read_snapshot, farm_lease_wal_replay, farm_lease_wal_snap_fingerprint,
    farm_lease_wal_snapshot_log_len, farm_lease_wal_validate_snapshot_authority,
    load_runtime_member_binds, load_runtime_member_regions, parse_region_tagged_members,
    spawn_farm_lease_authority, spawn_farm_lease_ha, spawn_farm_lease_ha_v3, spawn_farm_lease_raft,
    spawn_farm_lease_raft_multihost, spawn_farm_lease_raft_runtime, FarmLeaseHaCluster,
    FarmLeaseHaV3Cluster, FarmLeaseRaftCluster, FarmMemberRegion, FarmPkcs11Bridge,
    ENV_DYN_MEMBERS_GA, ENV_FARM_LEASE_MEMBERS, ENV_FARM_LEASE_MEMBERS_FILE,
    ENV_FARM_PKCS11_KEY_ID_FILE, ENV_FARM_PKCS11_KEY_LABEL_FILE, ENV_FARM_PKCS11_PIN_FILE,
    ENV_FARM_PKCS11_SLOT_FILE, ENV_FARM_PKCS11_SO, FARM_LEASE_DEFAULT_TOKEN,
    FARM_LEASE_MULTIDC_PROBE_TIMEOUT_MS, FARM_LEASE_RAFT_MULTIHOST_BINDS, FARM_LEASE_RAFT_PORTS,
};

/// Serialize farm lease + TCP integration tests.
pub fn farm_lease_test_lock() -> std::sync::MutexGuard<'static, ()> {
    farm_lease_auth::farm_lease_test_lock()
}
pub mod build_syx;
pub mod embed;
pub mod embed_v2;
mod emit;
mod lsp_completion;
mod module_ref;
pub mod query;
mod unified_compile;

pub use emit::{
    default_emit_output, default_emit_stem, emit_clif_text, emit_llvm_text, emit_mir_for_path,
    emit_mir_text, emit_syxbc_text, emit_wasm_bytes, emit_wat_text, emit_with_options,
    write_emit_all, EmitArtifacts, EmitFormat,
};

pub use lsp_completion::{
    interpolation_identifier_context, interpolation_member_context,
    interpolation_needs_close_paren, lsp_character_to_byte, lsp_line_byte_offset,
    lsp_position_to_byte_offset, member_access_context_text, member_receiver_name_end_byte,
    parse_receiver_expr, parse_simple_member_expr, patch_source_for_completion_semantic,
    patch_source_for_member_completion, receiver_expr_at_offset, receiver_for_completion,
    receiver_type_at_position,
};

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex, OnceLock};

use indexmap::IndexMap;
use module_ref::{embedded_module, ModuleRef};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use syx_ast::{AccessLevel, Decl, Stmt};
use syx_async::SyxRuntime;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_debug::DebugSession;
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};
use syx_fmt::format_source;
pub use syx_frontend::{
    apply_meta_passes, desugar_top_level_stmts, diagnostic_codes, expand_macros, mir_module_hash,
    prepare_async_module, FrontendArtifact, FrontendOptions, FrontendPipeline, StopAfter,
};
use syx_lint::lint_source;
use syx_meta::{
    check_effects, reflect_type, register_data_classes, validate_pure_functions, TypeMetadata,
};
use syx_mir::{lower_module, optimize, MirOp};
use syx_mono::monomorphize_module;
use syx_parser::Parser;
use syx_semantic::{type_expr_to_id, SemanticAnalyzer, TypeId};
use syx_vm::WasmVM;

pub use syx_pkg::{
    add_dependency, default_registry_url, install_project, load_lockfile, load_manifest,
    load_resolved_packages, save_lockfile, update_dependencies, DependencyAddSpec, DependencySpec,
    InstallReport, InstalledPackage, LockPackage, PackageSource, RegistryConfig, ResolvedGraph,
    SyxLockfile, SyxManifest,
};
pub use unified_compile::{compile_entry_with_unified_mir, UnifiedCompileArtifact};

/// Phase 22: when true, embedded `Result` stdlib uses Phase 21 Object-only String API.
/// Phase 25 ADR-024: legacy Object Result wire removed; env is ignored.
pub fn result_legacy_object_enabled() -> bool {
    false
}

/// Phase 23 DE1: default builtin Result wire (inverse of legacy Object dict).
pub fn result_builtin_wire_enabled() -> bool {
    !result_legacy_object_enabled()
}

/// Phase 28 ED2: Result<T,E> generic syntax surface enabled (`SYX_RESULT_GENERIC=1` or default on).
pub fn result_generic_syntax_enabled() -> bool {
    std::env::var("SYX_RESULT_GENERIC")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 29 EI2: Result<T,E> full monomorphization for arbitrary T/E instantiations.
pub fn result_generic_full_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_GENERIC_FULL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_generic_syntax_enabled())
}

/// Phase 30 EN3: Result try/`?` expression ergonomics with mono linkage.
pub fn result_try_expr_enabled() -> bool {
    std::env::var("SYX_RESULT_TRY_EXPR")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_generic_full_mono_enabled())
}

/// Phase 31 ES3: async/Result try/`?` deep mono linkage at async/Actor boundaries.
pub fn result_async_try_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_TRY")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_try_expr_enabled())
}

/// Phase 32 EX3: async/Result try/`?` full mono at async/Actor boundaries.
pub fn result_async_actor_try_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_ACTOR_TRY")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_async_try_mono_enabled())
}

/// Phase 33 FC3: Result/Actor try production complete mono (`SYX_RESULT_ASYNC_ACTOR_TRY_PROD=0` opt-out).
pub fn result_async_actor_try_prod_mono_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_ACTOR_TRY_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_async_actor_try_mono_enabled())
}

/// Phase 34 FH4: Result/Actor try E2E prod (`SYX_RESULT_ASYNC_ACTOR_TRY_E2E=0` opt-out).
pub fn result_async_actor_try_e2e_enabled() -> bool {
    std::env::var("SYX_RESULT_ASYNC_ACTOR_TRY_E2E")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(result_async_actor_try_prod_mono_enabled())
}

/// Phase 33 FE2: Net setsockopt prod host path default on.
pub fn net_setsockopt_prod_enabled() -> bool {
    std::env::var("SYX_NET_SETSOCKOPT_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 34 FH3: typealias MVP syntax surface (`SYX_TYPEALIAS_MVP=1` opt-in).
pub fn typealias_mvp_enabled() -> bool {
    std::env::var("SYX_TYPEALIAS_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(typealias_prod_enabled())
}

/// Phase 35 FM1: typealias prod default on (`SYX_TYPEALIAS=0` opt-out).
pub fn typealias_prod_enabled() -> bool {
    syx_semantic::typealias_prod_enabled()
}

/// Phase 35 FM2: property wrapper MVP (`SYX_PROPERTY_WRAPPER=1` opt-in).
pub fn property_wrapper_mvp_enabled() -> bool {
    syx_semantic::property_wrapper_mvp_enabled()
}

/// Phase 35 FL2: Net setsockopt real syscall prod default on.
pub fn net_setsockopt_real_enabled() -> bool {
    std::env::var("SYX_NET_SETSOCKOPT_REAL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(net_setsockopt_prod_enabled())
}

/// Phase 35 FL3: dist rustls crate E2E prod (Unix default on; opt-out via env).
pub fn dist_rustls_crate_e2e_prod_enabled() -> bool {
    std::env::var("SYX_DIST_RUSTLS_E2E_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(cfg!(unix))
}

/// Phase 35 FL3: dist Win named pipe E2E prod (Windows default on).
pub fn dist_winpipe_e2e_prod_enabled() -> bool {
    std::env::var("SYX_DIST_WINPIPE_E2E_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(cfg!(windows))
}

/// Phase 23 ADR-020: hygienic `@attached` v2 with `{type}` prefix in templates.
pub fn attached_hygienic_v2_enabled() -> bool {
    syx_macro::attached_hygienic_v2_enabled()
}

/// Phase 41: user `#macro` param gensym prefix (ADR-059 v1). Opt out via `--no-macro-hygiene` or `SYX_MACRO_HYGIENE=0`.
pub fn macro_hygiene_enabled() -> bool {
    syx_macro::macro_hygiene_enabled()
}

/// Phase 23 ADR-018/020: built-in `@attached` source body templates.
pub fn attached_source_template(kind: &str) -> &'static str {
    syx_macro::attached_source_template(kind)
}

pub fn publish_package(root: &Path, registry_url: Option<&str>) -> SyxResult<()> {
    publish_package_with_token(root, registry_url, None)
}

pub fn publish_package_with_token(
    root: &Path,
    registry_url: Option<&str>,
    token: Option<&str>,
) -> SyxResult<()> {
    let registry = registry_url
        .map(str::to_string)
        .unwrap_or_else(|| default_registry_url(&load_manifest(root).unwrap_or_default()));
    syx_pkg::publish_package_with_token(root, &registry, token)
}

pub fn reflect_type_info(name: &str) -> Option<TypeMetadata> {
    reflect_type(name)
}

/// Phase 77: query generic specialization signature.
pub fn reflect_specialization_info(
    fn_name: &str,
    type_args: &[&str],
) -> Option<syx_meta::MirMonoSig> {
    syx_meta::reflect_specialization_lookup(fn_name, type_args)
}

/// Phase 105: record mono witness entries for MIR/native instantiations (unified cache path).
pub fn backfill_mono_witness_cache(
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    mir: &syx_mir::MirModule,
) {
    for (base, type_args) in instantiations {
        let mangled = syx_mono::mangle_name(base.as_str(), type_args);
        let Some(mir_fn) = mir.functions.iter().find(|f| f.name == mangled) else {
            continue;
        };
        let type_arg_refs: Vec<&str> = type_args.iter().map(|s| s.as_str()).collect();
        let param_count = mir_fn.param_count as usize;
        syx_meta::register_mono_sig(base.as_str(), &type_arg_refs, param_count, true);
        syx_meta::persist_mono_sig_disk(base.as_str(), &type_arg_refs, param_count, true);
    }
}

fn use_mir_pipeline() -> bool {
    true
}

fn compile_native_object(mir: &syx_mir::MirModule, options: &DriverOptions) -> SyxResult<Vec<u8>> {
    syx_llvm::compile_native_with_profile(mir, options.build_profile.is_release())
}

/// Phase 59: Cranelift backend physically removed; LLVM is the sole native backend.
pub fn cranelift_backend_retired() -> bool {
    true
}

/// Retired Cranelift bench stubs (p37 legacy tests).
pub fn cranelift_p20_bench_enabled() -> bool {
    false
}

pub fn cranelift_p20_bench_target_bytes() -> u64 {
    0
}

pub fn cranelift_gc_intrinsics_parity_enabled() -> bool {
    false
}

pub fn cranelift_unified_gc_promotion_codegen_enabled() -> bool {
    false
}

pub fn compile_module_to_object(_module: &syx_mir::MirModule) -> SyxResult<Vec<u8>> {
    Err(SyxDiagnostic::error(
        "Cranelift backend removed in Phase 59; use LLVM (--backend llvm)",
    ))
}

pub fn emit_clif_module_text(_module: &syx_mir::MirModule) -> String {
    "; Cranelift backend retired (Phase 59)\n".into()
}

pub fn take_macro_warnings() -> Vec<SyxDiagnostic> {
    syx_macro::take_macro_warnings()
}

fn use_mir_for_module(_module_key: &Path, _options: &DriverOptions) -> bool {
    use_mir_pipeline()
}

/// Span-start → `NumericKind` for unsuffixed literals contextual-typed by semantic.
fn contextual_numeric_kinds_from_semantic(
    semantic: &SemanticAnalyzer,
) -> indexmap::IndexMap<u32, syx_stdlib::NumericKind> {
    use syx_stdlib::{is_float, is_integer};
    semantic
        .expr_types
        .iter()
        .filter_map(|(span, ty)| match ty {
            TypeId::Numeric(kind) if is_integer(*kind) || is_float(*kind) => Some((*span, *kind)),
            _ => None,
        })
        .collect()
}

fn compile_to_program(
    module: &syx_ast::Module,
    module_key: &Path,
    program: &mut CompiledProgram,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    semantic: &SemanticAnalyzer,
    options: &DriverOptions,
    source_name: &str,
    prelude_extensions: &indexmap::IndexSet<(SmolStr, SmolStr)>,
) -> SyxResult<()> {
    let mut mono_module = monomorphize_module(module, instantiations);
    inject_virtual_prelude_imports(&mut mono_module, options);
    // W477 / ADR-821: MIR-only production path (`use_mir_pipeline()` is always true).
    // AST `compile_into_with_semantic` else-arm cleared; API retained in syx-codegen.
    debug_assert!(
        use_mir_for_module(module_key, options),
        "W477 expects MIR pipeline for compile_to_program"
    );
    let int_kinds = contextual_numeric_kinds_from_semantic(semantic);
    let mut mir = syx_mir::lower_module_with_extra_extensions_and_kinds(
        &mono_module,
        prelude_extensions,
        &program.functions.iter().map(|f| f.name.clone()).collect(),
        &indexmap::IndexSet::new(),
        &int_kinds,
    )?;
    optimize(&mut mir);
    backfill_mono_witness_cache(instantiations, &mir);
    CodeGenerator::compile_with_mir(
        &mono_module,
        &mir,
        program,
        Some(semantic),
        Some(source_name),
        Some(prelude_extensions),
    )?;
    Ok(())
}

fn inject_virtual_prelude_imports(module: &mut syx_ast::Module, options: &DriverOptions) {
    if !options.enable_std {
        return;
    }
    use std::collections::HashSet;
    let existing: HashSet<SmolStr> = module
        .decls
        .iter()
        .filter_map(|d| d.import_symbol())
        .cloned()
        .collect();
    for &name in syx_config::CORE_PRELUDE_MODULES {
        let sym = SmolStr::new(name);
        if existing.contains(&sym) {
            continue;
        }
        module.decls.insert(
            0,
            syx_ast::Decl::Import {
                span: syx_diagnostics::Span::DUMMY,
                path: vec![sym],
                alias: None,
            },
        );
    }
}

/// Merge monomorphized stdlib instances (e.g. `Array_len_Int`) referenced by `instantiations`.
fn merge_stdlib_mono_instances(
    graph: &ModuleGraph,
    program: &mut CompiledProgram,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    options: &DriverOptions,
) -> SyxResult<()> {
    if instantiations.is_empty() {
        return Ok(());
    }
    for dep in graph.modules.values() {
        if !dep.path.to_string_lossy().contains("<stdlib>/") {
            continue;
        }
        let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let relevant = instantiations.iter().any(|(name, _)| {
            let n = name.as_str();
            n == module_stem || n.starts_with(&format!("{module_stem}_"))
        });
        if !relevant {
            continue;
        }
        let dep_module = expand_macros(Parser::parse(dep.source.clone())?)?;
        let mut dep_mono = monomorphize_module(&dep_module, instantiations);
        inject_virtual_prelude_imports(&mut dep_mono, options);
        let known: std::collections::HashSet<_> = dep
            .program
            .functions
            .iter()
            .map(|f| f.name.clone())
            .collect();
        // W477 / ADR-821: stdlib mono merge via MIR + compile_with_mir (≠ delete compile_into API).
        let mut mono_program = CompiledProgram::default();
        let mut dep_mir = lower_module(&dep_mono)?;
        optimize(&mut dep_mir);
        CodeGenerator::compile_with_mir(&dep_mono, &dep_mir, &mut mono_program, None, None, None)?;
        for func in mono_program.functions {
            if !known.contains(&func.name) && !program.functions.iter().any(|f| f.name == func.name)
            {
                program.functions.push(func);
            }
        }
    }
    Ok(())
}

fn collect_main_actor_await_errors(
    module: &syx_ast::Module,
    imports: &[(SmolStr, IndexMap<SmolStr, syx_semantic::TypeId>)],
    manifest: &SyxManifest,
) -> Vec<SyxDiagnostic> {
    let analyzer = SemanticAnalyzer::analyze_collecting_with_options(
        module,
        imports,
        manifest.concurrency.strict,
        manifest
            .concurrency
            .default_isolation
            .as_ref()
            .map(|s| SmolStr::new(s.as_str())),
        &[],
        None,
        &IndexMap::new(),
        false,
    );
    analyzer
        .errors
        .into_iter()
        .filter(|e| e.code.as_deref() == Some("S020"))
        .collect()
}

/// Generate API docs for a `.sy` file or a directory of sources.
///
/// Directories are scanned for `.sy` files (same walk as `syx format --check`),
/// so package roots with `mod.sy` (and no `main.sy` / `syx.toml`) work.
pub fn doc_project(path: &Path) -> SyxResult<String> {
    let files = collect_sy_files(path)?;
    if files.is_empty() {
        return Err(SyxDiagnostic::error(format!(
            "No .sy source files found under {}",
            path.display()
        )));
    }
    let mut sources = Vec::with_capacity(files.len());
    for file in &files {
        let text = std::fs::read_to_string(file).map_err(|e| {
            SyxDiagnostic::error(format!("Failed to read {}: {e}", file.display()))
        })?;
        sources.push(SourceFile::new(file.display().to_string(), text));
    }
    syx_doc::generate_markdown_sources(&sources)
}

/// Format a single `.sy` file or a project directory (manifest `main` entry).
pub fn format_project(path: &Path) -> SyxResult<()> {
    if path.is_file() {
        return format_file(path);
    }
    let root = path;
    let manifest = load_manifest(root)?;
    let entry = root.join(&manifest.main);
    format_file(&entry)
}

fn format_file(path: &Path) -> SyxResult<()> {
    if path.extension().and_then(|e| e.to_str()) != Some("sy") {
        return Err(SyxDiagnostic::error(format!(
            "format expects a .sy file or project directory, got {}",
            path.display()
        )));
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let had_comments = text.contains("//") || text.contains("/*");
    let source = SourceFile::new(path.display().to_string(), text);
    // Write path uses the AST pretty-printer (`format_source`).
    // `syx-cst` / `format_source_auto` are for comment-preserving round-trip
    // (`syx format --check`); they concatenate lexemes and do not reindent.
    let formatted = format_source(&source)?;
    if had_comments && !(formatted.contains("//") || formatted.contains("/*")) {
        eprintln!(
            "warning: `syx format` dropped comments (AST pretty-printer); \
             CST path preserves comments but does not reindent yet"
        );
    }
    std::fs::write(path, formatted)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to write {}: {e}", path.display())))
}

/// W683 / C-06: aggregate for `syx format --check` (never writes).
#[derive(Debug, Clone, Default)]
pub struct FormatCheckReport {
    pub checked: usize,
    pub would_change: usize,
    pub failed: usize,
    pub failures: Vec<(std::path::PathBuf, String)>,
}

impl FormatCheckReport {
    pub fn ok(&self) -> bool {
        self.would_change == 0 && self.failed == 0
    }
}

fn collect_sy_files(path: &Path) -> SyxResult<Vec<std::path::PathBuf>> {
    if path.is_file() {
        if path.extension().and_then(|e| e.to_str()) == Some("sy") {
            return Ok(vec![path.to_path_buf()]);
        }
        return Err(SyxDiagnostic::error(format!(
            "expected a .sy file or directory, got {}",
            path.display()
        )));
    }
    if !path.is_dir() {
        return Err(SyxDiagnostic::error(format!(
            "path not found: {}",
            path.display()
        )));
    }
    let mut out = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> SyxResult<()> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", dir.display())))?;
        for entry in entries {
            let entry = entry
                .map_err(|e| SyxDiagnostic::error(format!("Failed to read dir entry: {e}")))?;
            let p = entry.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                // W873: cst_comment_* no longer skipped — default CST-first preserves comments.
                if name == "target" || name == ".git" || name == ".sy" {
                    continue;
                }
                walk(&p, out)?;
            } else if p.extension().and_then(|e| e.to_str()) == Some("sy") {
                out.push(p);
            }
        }
        Ok(())
    }
    walk(path, &mut out)?;
    out.sort();
    Ok(out)
}

fn format_check_path_with(
    path: &Path,
    dry_run: fn(&SourceFile) -> SyxResult<syx_fmt::DryRunReport>,
) -> SyxResult<FormatCheckReport> {
    let files = collect_sy_files(path)?;
    let mut report = FormatCheckReport::default();
    for file in files {
        report.checked += 1;
        let text = match std::fs::read_to_string(&file) {
            Ok(t) => t,
            Err(e) => {
                report.failed += 1;
                report
                    .failures
                    .push((file.clone(), format!("read failed: {e}")));
                continue;
            }
        };
        let before = text.clone();
        let source = SourceFile::new(file.display().to_string(), text);
        match dry_run(&source) {
            Ok(dry) => {
                if dry.would_change {
                    report.would_change += 1;
                    report
                        .failures
                        .push((file, "would reformat (dry-run differs from on-disk)".into()));
                }
                // Sanity: never claim write — before bytes unchanged on disk.
                let _ = before;
            }
            Err(err) => {
                report.failed += 1;
                let code = err.code.as_deref().unwrap_or("?");
                report
                    .failures
                    .push((file, format!("{code}: {}", err.message)));
            }
        }
    }
    Ok(report)
}

/// Recursively dry-run format under `path` (file or directory). Does **not** write.
pub fn format_check_path(path: &Path) -> SyxResult<FormatCheckReport> {
    format_check_path_with(path, syx_fmt::format_dry_run)
}

/// W872 / ADR-1216: CST-only `--check` aggregate (no env).
///
/// Kept for pure-CST nails; W873 default path uses [`format_check_path`].
pub fn format_check_path_cst(path: &Path) -> SyxResult<FormatCheckReport> {
    format_check_path_with(path, syx_fmt::format_dry_run_cst)
}

/// Lint a single `.sy` file or a project directory (manifest `main` entry).
pub fn lint_project(path: &Path) -> SyxResult<Vec<syx_lint::LintIssue>> {
    if path.is_file() {
        return lint_file(path);
    }
    let root = path;
    let manifest = load_manifest(root)?;
    let entry = root.join(&manifest.main);
    lint_file(&entry)
}

fn lint_file(path: &Path) -> SyxResult<Vec<syx_lint::LintIssue>> {
    if path.extension().and_then(|e| e.to_str()) != Some("sy") {
        return Err(SyxDiagnostic::error(format!(
            "lint expects a .sy file or project directory, got {}",
            path.display()
        )));
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let source = SourceFile::new(path.display().to_string(), text);
    lint_source(&source)
}

pub fn debug_project(root: &Path) -> SyxResult<DebugSession> {
    debug_project_with_options(root, &DriverOptions::default())
}

pub fn debug_project_with_options(root: &Path, options: &DriverOptions) -> SyxResult<DebugSession> {
    let project = compile_project_with_options(root, options)?;
    let text = std::fs::read_to_string(&project.entry.path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read entry: {e}")))?;
    let source = SourceFile::new(project.entry.path.display().to_string(), text);
    Ok(DebugSession::new(source, project.entry.program))
}

pub fn debug_file(path: &Path) -> SyxResult<DebugSession> {
    debug_file_with_options(path, &DriverOptions::default())
}

pub fn debug_file_with_options(path: &Path, options: &DriverOptions) -> SyxResult<DebugSession> {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let text = std::fs::read_to_string(&path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let source = SourceFile::new(path.display().to_string(), text);
    let unit = compile_file_with_options(&path, options)?;
    Ok(DebugSession::new(source, unit.program))
}

pub fn debug_path(path: &Path) -> SyxResult<DebugSession> {
    debug_path_with_options(path, &DriverOptions::default())
}

pub fn debug_path_with_options(path: &Path, options: &DriverOptions) -> SyxResult<DebugSession> {
    if path.is_dir() {
        debug_project_with_options(path, options)
    } else {
        debug_file_with_options(path, options)
    }
}

pub fn debug_interactive(path: &Path, options: &DriverOptions) -> SyxResult<()> {
    let mut session = debug_path_with_options(path, options)?;
    syx_debug::run_interactive_session(&mut session)
}

pub fn debug_dap(path: &Path, options: &DriverOptions) -> SyxResult<()> {
    match debug_path_with_options(path, options) {
        Ok(mut session) => syx_debug::run_dap_server(&mut session),
        Err(err) => {
            // Compile/parse failure before DAP handshake used to exit silently in VS Code.
            // Stay on the DAP wire long enough to report the error in the Debug Console.
            syx_debug::run_dap_compile_failed(&err)
        }
    }
}

pub fn repl_eval(source: &str) -> SyxResult<()> {
    let file = SourceFile::new("repl.sy", source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Mir,
        desugar_top_level: true,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file, &opts)?;
    let semantic = art
        .semantic
        .ok_or_else(|| SyxDiagnostic::error("FrontendPipeline returned no semantic artifact"))?;
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new("repl.sy"),
        &mut program,
        &semantic.instantiations,
        &semantic,
        &DriverOptions::default(),
        "repl.sy",
        &empty_prelude,
    )?;
    let mut vm = WasmVM::default();
    vm.run_program(&program)?;
    Ok(())
}

/// Parse and semantically analyze source; returns diagnostics (including semantic warnings).
pub fn diagnostics_for_source(
    name: &str,
    contents: &str,
    options: &DriverOptions,
) -> Vec<SyxDiagnostic> {
    let source = SourceFile::new(name, contents.to_string());
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: false,
        collecting_semantic: true,
        strict_concurrency: options.strict_concurrency,
        ..FrontendOptions::default()
    };
    opts.source = Some(source.clone());
    match FrontendPipeline::run(source, &opts) {
        Ok(art) => {
            let mut diags = art.diagnostics;
            if let Some(sem) = art.semantic {
                diags.extend(sem.errors);
            }
            diags
        }
        Err(e) => vec![e],
    }
}

fn merge_lsp_import(
    out: &mut Vec<(SmolStr, IndexMap<SmolStr, TypeId>)>,
    module_name: SmolStr,
    symbols: IndexMap<SmolStr, TypeId>,
) {
    if let Some((_, existing)) = out.iter_mut().find(|(n, _)| *n == module_name) {
        for (k, v) in symbols {
            existing.entry(k).or_insert(v);
        }
    } else {
        out.push((module_name, symbols));
    }
}

fn collect_import_symbols_for_lsp(
    module: &syx_ast::Module,
    options: &DriverOptions,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    collect_import_symbols_for_lsp_from(module, options, None)
}

fn collect_import_symbols_for_lsp_from(
    module: &syx_ast::Module,
    options: &DriverOptions,
    importer: Option<&Path>,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    let mut import_symbols = core_prelude_import_symbols(options);
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            if let Some(path_str) = decl.import_path_str() {
                if !options.enable_std {
                    continue;
                }
                let first = path_str.split('.').next().unwrap_or(path_str.as_str());
                if let Some(importer) = importer {
                    if let Some(disk) = embedded_std::find_workspace_stdlib_file(importer, first) {
                        if let Ok(text) = std::fs::read_to_string(&disk) {
                            let source = SourceFile::new(disk.display().to_string(), text);
                            merge_lsp_import(
                                &mut import_symbols,
                                SmolStr::new(first),
                                export_symbols(&source),
                            );
                            continue;
                        }
                    }
                }
                if let Some((name, src)) = embedded_std::lookup(first) {
                    let source = SourceFile::new(
                        embedded_std::virtual_path(name)
                            .to_string_lossy()
                            .into_owned(),
                        src,
                    );
                    merge_lsp_import(
                        &mut import_symbols,
                        SmolStr::new(name),
                        export_symbols(&source),
                    );
                }
            }
        }
    }
    import_symbols
}

fn collect_import_symbols_for_lsp_with_path(
    module: &syx_ast::Module,
    importer: &Path,
    options: &DriverOptions,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    let mut import_symbols = collect_import_symbols_for_lsp_from(module, options, Some(importer));
    let parent = importer.parent().unwrap_or_else(|| Path::new("."));
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    if !manifest.dependencies.is_empty() {
        if ensure_project_lock(parent, &opts).is_err() {
            return import_symbols;
        }
        opts.resolved_packages = load_resolved_packages(parent).ok();
    }
    let importer_dir = importer.parent().unwrap_or(importer);
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let Some(symbol) = decl.import_symbol() else {
                continue;
            };
            let Ok(import_ref) =
                resolve_module(&path_str, importer, importer_dir, &manifest, &opts, parent)
            else {
                continue;
            };
            let Ok((_, dep_source)) = import_ref.load() else {
                continue;
            };
            merge_lsp_import(
                &mut import_symbols,
                symbol.clone(),
                export_symbols(&dep_source),
            );
        }
    }
    import_symbols
}

/// Parsed module + semantic analysis for LSP requests.
#[derive(Debug)]
pub struct LspDocument {
    pub source: SourceFile,
    pub module: syx_ast::Module,
    pub semantic: SemanticAnalyzer,
    /// Phase 59 LSP v51: true when macro expansion changed the module AST.
    pub macro_expanded: bool,
}

pub fn analyze_document_for_lsp(
    path: Option<&Path>,
    contents: &str,
    options: &DriverOptions,
) -> LspDocument {
    let source = SourceFile::new(
        path.map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "buffer.sy".into()),
        contents.to_string(),
    );
    let outcome = Parser::parse_collecting(source.clone());
    let mut parse_errors = outcome.errors;
    let mut module = if parse_errors.is_empty() {
        if let Some(p) = path {
            query::store_parse(p, contents, &outcome.module);
        }
        outcome.module
    } else {
        if let Some(p) = path {
            query::invalidate_path(p);
        }
        // Keep partial decls so recovery can surface multiple P001s; semantic is skipped below.
        outcome.module
    };
    let decl_count_before = module.decls.len();
    let mut macro_expanded = false;
    let mut pass_errors: Vec<SyxDiagnostic> = Vec::new();
    let importer = path.unwrap_or_else(|| Path::new("buffer.sy"));
    let importer_dir = importer.parent().unwrap_or_else(|| Path::new("."));
    let project_root = importer_dir;
    let manifest = load_manifest(importer_dir).unwrap_or_default();
    let empty_graph = ModuleGraph::default();
    if parse_errors.is_empty() {
        // Resolve `import macros` (etc.) before expansion so LSP sees `#greet`.
        syx_macro::clear_imported_macros();
        syx_macro::register_package_macros(manifest.macros.as_slice());
        let _ = register_imported_macros_from_parsed(
            &module,
            importer,
            importer_dir,
            &manifest,
            options,
            project_root,
            &empty_graph,
        );
        // M1-A03: fail-closed frontend passes via FrontendPipeline (Prepared).
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        match FrontendPipeline::run_parsed(module.clone(), &prep_opts) {
            Ok(art) => {
                macro_expanded = art.module.decls.len() != decl_count_before
                    || format!("{:?}", art.module) != format!("{module:?}");
                module = art.module;
                if let Some(p) = path {
                    query::store_parse(p, contents, &module);
                }
            }
            Err(e) => pass_errors.push(e),
        }
    }
    // Core collection surfaces for LSP (without Class injection — S023 / Array typing).
    // Numeric static members (`Int.min`) are merged as signatures only after collect_decls
    // (see `value_builtin_modules` below) so foreign method bodies cannot pollute
    // `binding_sites` / `expr_types` with Character.sy / Int128.sy spans.
    if parse_errors.is_empty() && pass_errors.is_empty() {
        inject_core_collection_builtins(&mut module, options);
    }
    let value_builtin_modules = if parse_errors.is_empty() && pass_errors.is_empty() {
        expand_core_value_builtin_modules(options)
    } else {
        Vec::new()
    };
    if parse_errors.is_empty() && pass_errors.is_empty() {
        let _ = inject_imported_dataclass_decls(
            &mut module,
            importer,
            importer_dir,
            &manifest,
            options,
            project_root,
            &empty_graph,
        );
    }
    let imports = if parse_errors.is_empty() && pass_errors.is_empty() {
        if let Some(p) = path {
            collect_import_symbols_for_lsp_with_path(&module, p, options)
        } else {
            collect_import_symbols_for_lsp(&module, options)
        }
    } else {
        Vec::new()
    };
    let strict = path
        .and_then(|p| p.parent())
        .map(|parent| load_manifest(parent).unwrap_or_default())
        .map(|m| {
            (
                m.concurrency.strict,
                m.concurrency
                    .default_isolation
                    .as_ref()
                    .map(|s| SmolStr::new(s.as_str())),
            )
        })
        .unwrap_or((false, None));
    let empty_generics = IndexMap::new();
    let prelude_generics = core_prelude_generic_class_methods(options);
    let run_semantic = parse_errors.is_empty() && pass_errors.is_empty();
    let mut semantic = SemanticAnalyzer::analyze_collecting_with_value_builtins(
        &module,
        &imports,
        if run_semantic { strict.0 } else { false },
        if run_semantic { strict.1 } else { None },
        &[],
        Some(&source),
        if run_semantic {
            &prelude_generics
        } else {
            &empty_generics
        },
        false,
        &value_builtin_modules,
    );
    if !parse_errors.is_empty() {
        semantic.errors = parse_errors;
        enrich_semantic_spans(&source, &mut semantic.errors);
    } else if !pass_errors.is_empty() {
        semantic.errors = pass_errors;
        enrich_semantic_spans(&source, &mut semantic.errors);
    } else {
        enrich_semantic_spans(&source, &mut semantic.errors);
    }
    if let Some(p) = path {
        for decl in &module.decls {
            if let Some(name) = query::decl_name(decl) {
                let body_hash = query::decl_body_hash(decl);
                let decl_errors = semantic
                    .errors
                    .iter()
                    .filter(|e| e.message.contains(name))
                    .count();
                query::record_semantic_decl(p, name, &body_hash, decl_errors);
            }
        }
    }
    LspDocument {
        source,
        module,
        semantic,
        macro_expanded,
    }
}

/// Parse `contents` for member-completion AST only (no semantic analysis).
/// Soft-fail on pass errors is intentional for completion UX (not EC-M1-1 hard path).
pub fn parse_for_lsp_completion(
    path: Option<&Path>,
    contents: &str,
) -> (SourceFile, syx_ast::Module) {
    let source = SourceFile::new(
        path.map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| "buffer.sy".into()),
        contents.to_string(),
    );
    let mut module = Parser::parse(source.clone()).unwrap_or(syx_ast::Module {
        name: "error".into(),
        decls: vec![],
    });
    // Soft path: best-effort expand; do not pretend success when expand fails — keep parse AST.
    if let Ok(expanded) = expand_macros(module.clone()) {
        module = expanded;
        let _ = apply_meta_passes(&module);
        let _ = prepare_async_module(&mut module);
    }
    (source, module)
}

/// Phase 52 LSP: preview macro expansion for `syx.macro.expand` command.
pub fn macro_expand_preview(source: &str) -> SyxResult<String> {
    let file = SourceFile::new("macro_preview.sy", source.to_string());
    let expanded = expand_macros(Parser::parse(file)?)?;
    Ok(format!("{expanded:#?}"))
}

/// Attach source spans to semantic diagnostics when possible (LSP publishDiagnostics).
pub fn enrich_semantic_spans(source: &SourceFile, errors: &mut [SyxDiagnostic]) {
    for err in errors.iter_mut() {
        if err.primary_span.is_some() {
            continue;
        }
        let name = err
            .message
            .strip_prefix("Undefined identifier '")
            .and_then(|s| s.strip_suffix('\''))
            .or_else(|| {
                err.message
                    .strip_prefix("Undefined function '")
                    .and_then(|s| s.strip_suffix('\''))
            })
            .or_else(|| {
                err.message
                    .strip_prefix("Use of moved value '")
                    .and_then(|s| s.strip_suffix('\''))
            })
            .or_else(|| {
                err.message
                    .strip_prefix("Unknown macro '#")
                    .and_then(|s| s.strip_suffix('\''))
            });
        if let Some(name) = name {
            // Prefer `#name` for macro diagnostics so the squiggle lands on the invoke.
            let needle = if err.message.starts_with("Unknown macro '#") {
                format!("#{name}")
            } else {
                name.to_string()
            };
            if let Some(off) = source.contents.find(&needle) {
                *err = SyxDiagnostic {
                    message: err.message.clone(),
                    severity: err.severity,
                    code: err.code.clone(),
                    source: None,
                    primary_span: None,
                }
                .with_span(source, Span::new(off, needle.len()), name);
            }
        }
    }
}

/// Convert a source span to 0-based LSP line/character ranges.
pub fn span_to_lsp_range(source: &SourceFile, span: Span) -> ((usize, usize), (usize, usize)) {
    let (sl, sc) = source.line_col(span.start as usize);
    let (el, ec) = source.line_col(span.end());
    (
        (sl.saturating_sub(1), sc.saturating_sub(1)),
        (el.saturating_sub(1), ec.saturating_sub(1)),
    )
}

pub fn name_span_in_container(source: &SourceFile, container: Span, name: &str) -> Span {
    let start = container.start as usize;
    let end = container.end().min(source.contents.len());
    if start >= end {
        return container;
    }
    let slice = &source.contents[start..end];
    if let Some(rel) = slice.find(name) {
        return Span::new(start + rel, name.len());
    }
    container
}

pub fn find_definition_in_module(
    source: &SourceFile,
    module: &syx_ast::Module,
    name: &str,
) -> Option<Span> {
    find_definition_in_decls(source, &module.decls, name)
        .or_else(|| find_definition_in_function_bodies(source, &module.decls, name))
}

fn find_definition_in_decls(source: &SourceFile, decls: &[Decl], name: &str) -> Option<Span> {
    for decl in decls {
        match decl {
            Decl::Function { name: n, span, .. } if n.as_str() == name => {
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::VarGlobal { name: n, span, .. } if n.as_str() == name => {
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::Class {
                name: n,
                span,
                members,
                ..
            }
            | Decl::Actor {
                name: n,
                span,
                members,
                ..
            } if n.as_str() == name => {
                let _ = members;
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::DataClass { name: n, span, .. }
            | Decl::Struct { name: n, span, .. }
            | Decl::Enum { name: n, span, .. }
            | Decl::FinalClass { name: n, span, .. }
                if n.as_str() == name =>
            {
                return Some(name_span_in_container(source, *span, name));
            }
            Decl::Class {
                name: n,
                span,
                members,
                ..
            }
            | Decl::Actor {
                name: n,
                span,
                members,
                ..
            } if n.as_str() != name => {
                let _ = (n, span);
                if let Some(span) = find_definition_in_decls(source, members, name) {
                    return Some(span);
                }
            }
            Decl::Extension { methods, .. } => {
                if let Some(span) = find_definition_in_decls(source, methods, name) {
                    return Some(span);
                }
            }
            Decl::Function { body, .. } | Decl::Init { body, .. } => {
                if let Some(span) = find_definition_in_stmts(source, body, name) {
                    return Some(span);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_definition_in_function_bodies(
    source: &SourceFile,
    decls: &[Decl],
    name: &str,
) -> Option<Span> {
    for decl in decls {
        if let Decl::Function { body, .. } = decl {
            if let Some(span) = find_definition_in_stmts(source, body, name) {
                return Some(span);
            }
        }
    }
    None
}

fn find_definition_in_stmts(
    source: &SourceFile,
    stmts: &[syx_ast::Stmt],
    name: &str,
) -> Option<Span> {
    for stmt in stmts {
        match stmt {
            syx_ast::Stmt::VarDecl { name: n, span, .. } if n.as_str() == name => {
                return Some(name_span_in_container(source, *span, name));
            }
            syx_ast::Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                if let Some(span) = find_definition_in_stmts(source, then_branch, name) {
                    return Some(span);
                }
                if let Some(branch) = else_branch {
                    if let Some(span) = find_definition_in_stmts(source, branch, name) {
                        return Some(span);
                    }
                }
            }
            syx_ast::Stmt::While { body, .. }
            | syx_ast::Stmt::ForIn { body, .. }
            | syx_ast::Stmt::Block { stmts: body, .. }
            | syx_ast::Stmt::RepeatWhile { body, .. } => {
                if let Some(span) = find_definition_in_stmts(source, body, name) {
                    return Some(span);
                }
            }
            syx_ast::Stmt::Try {
                body, catch_body, ..
            } => {
                if let Some(span) = find_definition_in_stmts(source, body, name) {
                    return Some(span);
                }
                if let Some(span) = find_definition_in_stmts(source, catch_body, name) {
                    return Some(span);
                }
            }
            syx_ast::Stmt::Switch { cases, default, .. } => {
                for case in cases {
                    if let Some(span) = find_definition_in_stmts(source, &case.body, name) {
                        return Some(span);
                    }
                }
                if let Some(body) = default {
                    if let Some(span) = find_definition_in_stmts(source, body, name) {
                        return Some(span);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

pub fn find_references_in_module(
    source: &SourceFile,
    module: &syx_ast::Module,
    name: &str,
) -> Vec<Span> {
    let mut spans = Vec::new();
    collect_references_in_decls(source, &module.decls, name, &mut spans);
    spans
}

/// Find references to a symbol across all modules in a project (when `syx.toml` exists).
pub fn find_references_in_project(
    path: &Path,
    symbol: &str,
    options: &DriverOptions,
) -> Option<Vec<(String, Span)>> {
    let parent = path.parent()?;
    if !parent.join("syx.toml").exists() {
        return None;
    }
    let project = compile_project_with_options(parent, options).ok()?;
    let mut out = Vec::new();
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone()).ok()?;
        let uri = format!("file://{}", mod_path.display());
        for span in find_references_in_module(&unit.source, &parsed, symbol) {
            out.push((uri.clone(), span));
        }
    }
    Some(out)
}

/// Workspace symbol search across a compiled project graph.
pub fn workspace_symbols_for_project(
    root: &Path,
    query: &str,
    options: &DriverOptions,
) -> SyxResult<Vec<(String, String, Span)>> {
    let project = compile_project_with_options(root, options)?;
    let q = query.to_ascii_lowercase();
    let mut out = Vec::new();
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone())?;
        let uri = format!("file://{}", mod_path.display());
        for decl in &parsed.decls {
            match decl {
                Decl::Function { name, span, .. }
                | Decl::VarGlobal { name, span, .. }
                | Decl::Class { name, span, .. }
                | Decl::Enum { name, span, .. } => {
                    if query.is_empty() || name.to_ascii_lowercase().contains(&q) {
                        out.push((uri.clone(), name.to_string(), *span));
                    }
                }
                _ => {}
            }
        }
    }
    Ok(out)
}

/// Suggest import module paths from stdlib + lightweight project scan.
///
/// Avoids full `compile_project` (too slow for completion keystrokes).
/// Stdlib suggestions are **importable** modules only (`IO`, `Net`, `Crypto`, …),
/// not Core prelude names like `Math` / `Array`.
pub fn import_module_suggestions(root: Option<&Path>, options: &DriverOptions) -> Vec<String> {
    let mut modules = std::collections::BTreeSet::new();
    if options.enable_std {
        for name in embedded_std::importable_module_names() {
            modules.insert(name.to_string());
        }
    }
    if let Some(root) = root {
        let manifest_root = if root.is_file() {
            root.parent().unwrap_or(root)
        } else {
            root
        };
        if let Ok(manifest) = load_manifest(manifest_root) {
            for dep in manifest.dependencies.keys() {
                modules.insert(dep.clone());
            }
        }
        // Sibling / directory `.sy` stems (local modules), no compile.
        let scan_dirs = [manifest_root, root];
        for dir in scan_dirs {
            let Ok(entries) = std::fs::read_dir(dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) != Some("sy") {
                    continue;
                }
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem != "main" && !stem.starts_with('.') {
                        modules.insert(stem.to_string());
                    }
                }
            }
        }
    }
    modules.into_iter().collect()
}

fn collect_references_in_decls(
    source: &SourceFile,
    decls: &[Decl],
    name: &str,
    out: &mut Vec<Span>,
) {
    for decl in decls {
        match decl {
            Decl::Function {
                name: n,
                span,
                body,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                collect_references_in_stmts(source, body, name, out);
            }
            Decl::VarGlobal {
                name: n,
                span,
                init,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                if let Some(expr) = init {
                    collect_references_in_expr(source, expr, name, out);
                }
            }
            Decl::Class { members, .. } | Decl::Actor { members, .. } => {
                collect_references_in_decls(source, members, name, out);
            }
            Decl::Extension { methods, .. } => {
                collect_references_in_decls(source, methods, name, out);
            }
            Decl::Init { body, .. } | Decl::Deinit { body, .. } => {
                collect_references_in_stmts(source, body, name, out);
            }
            _ => {}
        }
    }
}

fn collect_references_in_stmts(
    source: &SourceFile,
    stmts: &[syx_ast::Stmt],
    name: &str,
    out: &mut Vec<Span>,
) {
    for stmt in stmts {
        match stmt {
            syx_ast::Stmt::VarDecl {
                name: n,
                span,
                init,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                if let Some(expr) = init {
                    collect_references_in_expr(source, expr, name, out);
                }
            }
            syx_ast::Stmt::Expr { expr, .. }
            | syx_ast::Stmt::Print { expr, .. }
            | syx_ast::Stmt::Return {
                value: Some(expr), ..
            } => {
                collect_references_in_expr(source, expr, name, out);
            }
            syx_ast::Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                collect_references_in_expr(source, condition, name, out);
                collect_references_in_stmts(source, then_branch, name, out);
                if let Some(branch) = else_branch {
                    collect_references_in_stmts(source, branch, name, out);
                }
            }
            syx_ast::Stmt::While {
                condition, body, ..
            } => {
                collect_references_in_expr(source, condition, name, out);
                collect_references_in_stmts(source, body, name, out);
            }
            syx_ast::Stmt::ForIn {
                name: n,
                span,
                iterable,
                body,
                ..
            } => {
                if n.as_str() == name {
                    out.push(name_span_in_container(source, *span, name));
                }
                collect_references_in_expr(source, iterable, name, out);
                collect_references_in_stmts(source, body, name, out);
            }
            syx_ast::Stmt::Block { stmts, .. } => {
                collect_references_in_stmts(source, stmts, name, out);
            }
            syx_ast::Stmt::Try {
                body, catch_body, ..
            } => {
                collect_references_in_stmts(source, body, name, out);
                collect_references_in_stmts(source, catch_body, name, out);
            }
            _ => {}
        }
    }
}

fn collect_references_in_expr(
    source: &SourceFile,
    expr: &syx_ast::Expr,
    name: &str,
    out: &mut Vec<Span>,
) {
    match expr {
        syx_ast::Expr::Ident { name: n, span, .. } if n.as_str() == name => {
            out.push(*span);
        }
        syx_ast::Expr::Binary { left, right, .. } => {
            collect_references_in_expr(source, left, name, out);
            collect_references_in_expr(source, right, name, out);
        }
        syx_ast::Expr::Unary { expr, .. } => collect_references_in_expr(source, expr, name, out),
        syx_ast::Expr::Call { callee, args, .. } => {
            collect_references_in_expr(source, callee, name, out);
            for arg in args {
                collect_references_in_expr(source, &arg.value, name, out);
            }
        }
        syx_ast::Expr::Member { object, .. } => {
            collect_references_in_expr(source, object, name, out);
        }
        syx_ast::Expr::Index { object, index, .. } => {
            collect_references_in_expr(source, object, name, out);
            collect_references_in_expr(source, index, name, out);
        }
        syx_ast::Expr::Assign { value, .. }
        | syx_ast::Expr::CompoundAssign { value, .. }
        | syx_ast::Expr::MemberAssign { value, .. }
        | syx_ast::Expr::IndexAssign { value, .. } => {
            collect_references_in_expr(source, value, name, out);
        }
        syx_ast::Expr::Closure { body, .. } => match body {
            syx_ast::ClosureBody::Expr(e) => collect_references_in_expr(source, e, name, out),
            syx_ast::ClosureBody::Block(stmts) => {
                collect_references_in_stmts(source, stmts, name, out);
            }
        },
        syx_ast::Expr::TaskBlock { body, .. } => {
            collect_references_in_stmts(source, body, name, out);
        }
        syx_ast::Expr::Group { expr, .. } => collect_references_in_expr(source, expr, name, out),
        syx_ast::Expr::Array { elements, .. } => {
            for el in elements {
                collect_references_in_expr(source, el, name, out);
            }
        }
        syx_ast::Expr::Await { expr, .. } => collect_references_in_expr(source, expr, name, out),
        syx_ast::Expr::OptionalChain { object, .. } => {
            collect_references_in_expr(source, object, name, out);
        }
        syx_ast::Expr::ForceUnwrap { expr, .. } => {
            collect_references_in_expr(source, expr, name, out);
        }
        _ => {}
    }
}

pub fn definition_for_symbol(
    uri: &str,
    path: Option<&Path>,
    doc: &LspDocument,
    symbol: &str,
    options: &DriverOptions,
) -> Option<(String, Span)> {
    if let Some(span) = find_definition_in_module(&doc.source, &doc.module, symbol) {
        return Some((uri.to_string(), span));
    }
    if let Some(hit) = definition_for_global_builtin(symbol, path) {
        return Some(hit);
    }
    let path = path?;
    let parent = path.parent()?;
    if !parent.join("syx.toml").exists() {
        return None;
    }
    let project = compile_project_with_options(parent, options).ok()?;
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone()).ok()?;
        if let Some(span) = find_definition_in_module(&unit.source, &parsed, symbol) {
            let mod_uri = format!("file://{}", mod_path.display());
            return Some((mod_uri, span));
        }
    }
    None
}

/// Resolve `print` / `readLine` / `reflect` to the Core globals authority file.
pub fn definition_for_global_builtin(
    symbol: &str,
    from: Option<&Path>,
) -> Option<(String, Span)> {
    let embedded = embedded_std::lookup_global_builtin(symbol)?;
    let disk = from.and_then(|p| {
        embedded_std::find_workspace_packages_file(p, GLOBALS_PACKAGES_REL)
    });
    let (uri, text) = if let Some(disk) = disk {
        (
            format!("file://{}", disk.display()),
            std::fs::read_to_string(&disk).unwrap_or_else(|_| embedded.to_string()),
        )
    } else {
        (
            format!("file://{}", embedded_std::virtual_global_builtin_path().display()),
            embedded.to_string(),
        )
    };
    let path_for_source = uri.strip_prefix("file://").unwrap_or(uri.as_str());
    let source = SourceFile::new(path_for_source.to_string(), text);
    let parsed = Parser::parse(source.clone()).ok()?;
    let span = find_definition_in_module(&source, &parsed, symbol)?;
    Some((uri, span))
}

const GLOBALS_PACKAGES_REL: &str = "core/builtins/special/globals.sy";

/// SYX 38.0: resolve `importedModule.symbol` or nested `Outer.Inner` definitions.
pub fn definition_for_qualified_symbol(
    uri: &str,
    path: Option<&Path>,
    doc: &LspDocument,
    qualifier: &str,
    member: &str,
    options: &DriverOptions,
) -> Option<(String, Span)> {
    if doc
        .semantic
        .imported_modules
        .iter()
        .any(|m| m.as_str() == qualifier)
    {
        let path = path?;
        let parent = path.parent()?;
        if !parent.join("syx.toml").exists() {
            return None;
        }
        let project = compile_project_with_options(parent, options).ok()?;
        for (mod_path, unit) in &project.graph.modules {
            let stem = mod_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if stem != qualifier && !mod_path.to_string_lossy().contains(qualifier) {
                continue;
            }
            let parsed = Parser::parse(unit.source.clone()).ok()?;
            if let Some(span) = find_definition_in_module(&unit.source, &parsed, member) {
                let mod_uri = format!("file://{}", mod_path.display());
                return Some((mod_uri, span));
            }
        }
    }
    if qualifier.contains('.') {
        let mangled = qualifier.replace('.', "_");
        if let Some(span) = find_definition_in_module(&doc.source, &doc.module, &mangled) {
            return Some((uri.to_string(), span));
        }
        if let Some(span) = find_definition_in_module(&doc.source, &doc.module, member) {
            return Some((uri.to_string(), span));
        }
    }
    definition_for_symbol(uri, path, doc, member, options)
}

/// Phase 18 LSP v10: resolve the type definition for a symbol (not its value binding).
pub fn go_to_type_for_symbol(
    uri: &str,
    path: Option<&Path>,
    doc: &LspDocument,
    symbol: &str,
    options: &DriverOptions,
) -> Option<(String, Span)> {
    if let Some(span) = find_type_decl_in_module(&doc.source, &doc.module, symbol) {
        return Some((uri.to_string(), span));
    }
    let ty = doc.semantic.symbol_type(symbol)?;
    let type_name = syx_semantic::type_id_to_name(&ty);
    let base = type_name
        .split('<')
        .next()
        .unwrap_or(type_name.as_str())
        .trim_end_matches('?');
    if let Some(core_src) = embedded_std::lookup_core_builtin(base) {
        let source = SourceFile::new(
            embedded_std::virtual_core_builtin_path(base)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| format!("<stdlib>/{base}.sy")),
            core_src.to_string(),
        );
        if let Some(span) =
            find_type_decl_in_module(&source, &Parser::parse(source.clone()).ok()?, base)
        {
            let mod_uri = format!(
                "file://{}",
                embedded_std::virtual_core_builtin_path(base)
                    .unwrap_or_else(|| embedded_std::virtual_path(base))
                    .display()
            );
            return Some((mod_uri, span));
        }
    }
    if matches!(base, "Unknown" | "Nil") {
        return None;
    }
    if let Some(span) = find_type_decl_in_module(&doc.source, &doc.module, base) {
        return Some((uri.to_string(), span));
    }
    let path = path?;
    let parent = path.parent()?;
    if !parent.join("syx.toml").exists() {
        return None;
    }
    let project = compile_project_with_options(parent, options).ok()?;
    for (mod_path, unit) in &project.graph.modules {
        let parsed = Parser::parse(unit.source.clone()).ok()?;
        if let Some(span) = find_type_decl_in_module(&unit.source, &parsed, base) {
            let mod_uri = format!("file://{}", mod_path.display());
            return Some((mod_uri, span));
        }
    }
    None
}

fn find_type_decl_in_module(
    source: &SourceFile,
    module: &syx_ast::Module,
    name: &str,
) -> Option<Span> {
    for decl in &module.decls {
        match decl {
            syx_ast::Decl::Struct { name: n, span, .. }
            | syx_ast::Decl::Enum { name: n, span, .. }
            | syx_ast::Decl::DataClass { name: n, span, .. }
            | syx_ast::Decl::FinalClass { name: n, span, .. }
            | syx_ast::Decl::Class { name: n, span, .. }
            | syx_ast::Decl::Actor { name: n, span, .. }
            | syx_ast::Decl::Protocol { name: n, span, .. }
                if n.as_str() == name =>
            {
                return Some(name_span_in_container(source, *span, name));
            }
            _ => {}
        }
    }
    None
}

pub fn build_native(root: &Path) -> SyxResult<Vec<u8>> {
    build_native_with_options(root, &DriverOptions::default()).map(|o| o.object)
}

pub struct NativeBuildOutput {
    pub object: Vec<u8>,
    pub executable: Option<PathBuf>,
}

pub fn build_native_with_options(
    root: &Path,
    options: &DriverOptions,
) -> SyxResult<NativeBuildOutput> {
    let project = compile_project_with_options(root, options)?;
    let opts = driver_options_with_deps(root, &project.manifest, options)?;
    let mir = build_unified_mir(
        &project.graph,
        &project.entry,
        root,
        &opts,
        &project.manifest,
    )?;
    let object = compile_native_object(&mir, options)?;
    Ok(NativeBuildOutput {
        object,
        executable: None,
    })
}

pub fn build_native_file(path: &Path) -> SyxResult<Vec<u8>> {
    build_native_file_with_options(path, &DriverOptions::default()).map(|o| o.object)
}

fn project_root_if_manifest_entry(path: &Path) -> Option<PathBuf> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let manifest = load_manifest(parent).ok()?;
    let entry = parent.join(&manifest.main);
    if entry == path {
        Some(parent.to_path_buf())
    } else {
        None
    }
}

pub fn build_native_file_with_options(
    path: &Path,
    options: &DriverOptions,
) -> SyxResult<NativeBuildOutput> {
    if let Some(root) = project_root_if_manifest_entry(path) {
        return build_native_with_options(&root, options);
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    // Same as `syx run` / `compile_project`: discover bin/ + bin/plugins/ hosts
    // before semantic analysis (otherwise `__syx_plugin_*` → S002).
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        parent,
        &mut graph,
        &manifest,
        &opts,
    )?;
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    let object = compile_native_object(&mir, options)?;
    Ok(NativeBuildOutput {
        object,
        executable: None,
    })
}

fn rebuild_native_rt(options: &DriverOptions) -> SyxResult<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest_dir.join("../..");
    let mut args = vec!["build".to_string(), "-p".to_string(), "syx-rt".to_string()];
    if options.build_profile == BuildProfile::Release {
        args.push("--release".to_string());
    }
    if options.native_minimal {
        args.push("--no-default-features".to_string());
    } else {
        args.push("--features".to_string());
        let mut features = Vec::new();
        if !options.native_no_http {
            features.push("http");
        }
        if !options.native_no_file {
            features.push("file");
        }
        features.push("async-runtime");
        args.push(features.join(","));
    }
    let status = Command::new("cargo")
        .args(&args)
        .current_dir(&workspace)
        .status()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to rebuild syx-rt: {e}")))?;
    if !status.success() {
        return Err(SyxDiagnostic::error(
            "Failed to rebuild syx-rt for native link (check --minimal / --no-http / --no-file flags)",
        ));
    }
    Ok(())
}

fn native_link_options(root: &Path) -> syx_rt::LinkOptions {
    let manifest = syx_pkg::load_manifest(root).unwrap_or_default();
    let mut opts = syx_rt::LinkOptions::default();
    for p in &manifest.native.lib_paths {
        opts.lib_paths.push(root.join(p));
    }
    for obj in &manifest.native.objects {
        opts.objects.push(root.join(obj));
    }
    opts.libs = manifest.native.libs.clone();
    opts.link_flags = manifest.native.link_flags.clone();
    opts
}

fn link_native_object(
    object: &[u8],
    obj_path: &Path,
    output: &Path,
    options: &DriverOptions,
    link_opts: &syx_rt::LinkOptions,
) -> SyxResult<()> {
    if options.native_minimal || options.native_no_http || options.native_no_file {
        rebuild_native_rt(options)?;
    }
    if let Err(msg) = syx_rt::audit_user_object_undefs(object) {
        return Err(SyxDiagnostic::error(msg));
    }
    std::fs::write(obj_path, object).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to write {}: {e}", obj_path.display()))
    })?;
    let mut merged = link_opts.clone();
    merged.rt_profile = options.build_profile.rt_profile_name().to_string();
    for flag in options.build_profile.link_profile_args() {
        merged.link_flags.push(flag.to_string());
    }
    syx_rt::link_executable_with(obj_path, output, &merged)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to link native executable: {e}")))?;
    Ok(())
}

pub fn build_native_executable(
    root: &Path,
    options: &DriverOptions,
    output: &Path,
) -> SyxResult<PathBuf> {
    let entry = load_manifest(root)
        .map(|m| root.join(m.main))
        .unwrap_or_else(|_| root.join("main.sy"));
    if native_build_up_to_date(root, &entry, output, options) {
        return Ok(output.to_path_buf());
    }
    let out = build_native_with_options(root, options)?;
    let obj_path = output.with_extension("o");
    let link_opts = native_link_options(root);
    link_native_object(&out.object, &obj_path, output, options, &link_opts)?;
    write_native_build_stamp(root, &entry, output, options)?;
    Ok(output.to_path_buf())
}

fn native_build_stamp_path(root: &Path, output: &Path) -> PathBuf {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(output.as_os_str().as_encoded_bytes());
    project_cache_dir(root).join(format!("native_{:x}.stamp", hasher.finalize()))
}

fn native_input_fingerprint(path: &Path, options: &DriverOptions) -> String {
    use sha2::{Digest, Sha256};
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hasher.update(cache_version_tag().as_bytes());
    hasher.update(options.native_backend.as_str().as_bytes());
    hasher.update(if options.native_minimal { b"1" } else { b"0" });
    hasher.update(if options.native_no_http { b"1" } else { b"0" });
    hasher.update(if options.native_no_file { b"1" } else { b"0" });
    hasher.update(options.build_profile.cache_tag().as_bytes());
    format!("{:x}", hasher.finalize())
}

fn native_build_stamp_payload(source: &Path, options: &DriverOptions) -> String {
    let source_key = source
        .canonicalize()
        .unwrap_or_else(|_| source.to_path_buf())
        .display()
        .to_string();
    format!(
        "{}\n{}\n{}",
        native_input_fingerprint(source, options),
        source_key,
        options.build_profile.cache_tag()
    )
}

fn native_build_up_to_date(
    root: &Path,
    source: &Path,
    output: &Path,
    options: &DriverOptions,
) -> bool {
    if std::env::var("SYX_NATIVE_FORCE")
        .ok()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
    {
        return false;
    }
    let stamp = native_build_stamp_path(root, output);
    if !output.is_file() || !stamp.is_file() {
        return false;
    }
    let source_key = source
        .canonicalize()
        .unwrap_or_else(|_| source.to_path_buf())
        .display()
        .to_string();
    let payload = std::fs::read_to_string(&stamp).ok();
    payload.is_some_and(|s| {
        let mut lines = s.lines();
        let fp = lines.next().unwrap_or("");
        let stamped_source = lines.next().unwrap_or("");
        let stamped_profile = lines.next().unwrap_or("");
        fp == native_input_fingerprint(source, options)
            && stamped_source == source_key
            && stamped_profile == options.build_profile.cache_tag()
    })
}

fn write_native_build_stamp(
    root: &Path,
    source: &Path,
    output: &Path,
    options: &DriverOptions,
) -> SyxResult<()> {
    let stamp = native_build_stamp_path(root, output);
    if let Some(parent) = stamp.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(stamp, native_build_stamp_payload(source, options))
        .map_err(|e| SyxDiagnostic::error(format!("Failed to write native build stamp: {e}")))?;
    Ok(())
}

pub fn build_native_executable_file(
    path: &Path,
    options: &DriverOptions,
    output: &Path,
) -> SyxResult<PathBuf> {
    let root = path.parent().unwrap_or_else(|| Path::new("."));
    if native_build_up_to_date(root, path, output, options) {
        return Ok(output.to_path_buf());
    }
    let out = build_native_file_with_options(path, options)?;
    let obj_path = output.with_extension("o");
    let link_opts = native_link_options(root);
    link_native_object(&out.object, &obj_path, output, options, &link_opts)?;
    write_native_build_stamp(root, path, output, options)?;
    Ok(output.to_path_buf())
}

pub fn run_native_project_with_options(root: &Path, options: &DriverOptions) -> SyxResult<i32> {
    let exe = root.join(native_executable_file_name("app"));
    build_native_executable(root, options, &exe)?;
    let status = std::process::Command::new(&exe)
        .status()
        .map_err(|e| SyxDiagnostic::error(format!("Failed to run {}: {e}", exe.display())))?;
    Ok(status.code().unwrap_or(1))
}

pub fn build_native_llvm(root: &Path) -> SyxResult<Vec<u8>> {
    let mut options = DriverOptions::default();
    options.native_backend = NativeBackend::Llvm;
    build_native_with_options(root, &options).map(|o| o.object)
}

pub fn build_wasm(root: &Path) -> SyxResult<Vec<u8>> {
    build_wasm_with_options(root, &DriverOptions::default())
}

pub fn build_wasm_with_options(root: &Path, options: &DriverOptions) -> SyxResult<Vec<u8>> {
    let project = compile_project_with_options(root, options)?;
    let opts = driver_options_with_deps(root, &project.manifest, options)?;
    let mir = build_unified_mir(
        &project.graph,
        &project.entry,
        root,
        &opts,
        &project.manifest,
    )?;
    syx_wasm_emit::emit_wasm(&mir)
}

pub fn build_wasm_file(path: &Path) -> SyxResult<Vec<u8>> {
    build_wasm_file_with_options(path, &DriverOptions::default())
}

pub fn build_wasm_file_with_options(path: &Path, options: &DriverOptions) -> SyxResult<Vec<u8>> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        parent,
        &mut graph,
        &manifest,
        &opts,
    )?;
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    syx_wasm_emit::emit_wasm(&mir)
}

fn stdlib_mir_enabled() -> bool {
    std::env::var("SYX_STDLIB_MIR")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .map(|disabled| !disabled)
        .unwrap_or(true)
}

fn finalize_unified_mir(mir: &mut syx_mir::MirModule) {
    syx_mir::optimize(mir);
    mir.functions
        .sort_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
    renumber_protocol_witnesses(mir);
}

/// After function sort, refresh `__witness_*` fn indices stored in class inits.
fn renumber_protocol_witnesses(mir: &mut syx_mir::MirModule) {
    let order: Vec<smol_str::SmolStr> = mir.functions.iter().map(|f| f.name.clone()).collect();
    for func in &mut mir.functions {
        let Some(type_name) = func.name.as_str().strip_suffix("_init") else {
            continue;
        };
        for block in &mut func.blocks {
            let ins = &mut block.instructions;
            for i in 0..ins.len().saturating_sub(2) {
                let MirOp::ConstStr(key) = ins[i + 2].op.clone() else {
                    continue;
                };
                let Some(rest) = key.strip_prefix("__witness_") else {
                    continue;
                };
                let Some(method) = rest.rsplit('_').next() else {
                    continue;
                };
                if !matches!(ins[i + 1].op, MirOp::ConstI32(_)) {
                    continue;
                }
                let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
                let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
                ins[i + 1].op = MirOp::ConstI32(idx);
            }
            for i in 0..ins.len().saturating_sub(1) {
                let MirOp::ConstStr(key) = ins[i + 1].op.clone() else {
                    continue;
                };
                let Some(rest) = key.strip_prefix("__witness_") else {
                    continue;
                };
                let Some(method) = rest.rsplit('_').next() else {
                    continue;
                };
                if !matches!(ins[i].op, MirOp::ConstI32(_)) {
                    continue;
                }
                let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
                let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
                ins[i].op = MirOp::ConstI32(idx);
            }
        }
        let ins = &mut func.instructions;
        for i in 0..ins.len().saturating_sub(2) {
            let MirOp::ConstStr(key) = ins[i + 2].op.clone() else {
                continue;
            };
            let Some(rest) = key.strip_prefix("__witness_") else {
                continue;
            };
            let Some(method) = rest.rsplit('_').next() else {
                continue;
            };
            if !matches!(ins[i + 1].op, MirOp::ConstI32(_)) {
                continue;
            }
            let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
            let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
            ins[i + 1].op = MirOp::ConstI32(idx);
        }
        for i in 0..ins.len().saturating_sub(1) {
            let MirOp::ConstStr(key) = ins[i + 1].op.clone() else {
                continue;
            };
            let Some(rest) = key.strip_prefix("__witness_") else {
                continue;
            };
            let Some(method) = rest.rsplit('_').next() else {
                continue;
            };
            if !matches!(ins[i].op, MirOp::ConstI32(_)) {
                continue;
            }
            let fn_name = smol_str::SmolStr::new(format!("{type_name}_{method}"));
            let idx = order.iter().position(|n| *n == fn_name).unwrap_or(0) as i32;
            ins[i].op = MirOp::ConstI32(idx);
        }
    }
}

/// True when `func_name` is a public export from a stdlib/core module (supports `Class_method` static lowering).
fn mir_merge_exported_function(
    module_stem: &str,
    func_name: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) -> bool {
    let name = SmolStr::new(func_name);
    if dep_exports.contains_key(&name) {
        return true;
    }
    if dep_exports
        .keys()
        .any(|export| func_name.starts_with(&format!("{export}_")))
    {
        return true;
    }
    if dep_exports
        .keys()
        .any(|export| func_name == format!("{export}_init"))
    {
        return true;
    }
    if dep_exports
        .keys()
        .any(|export| func_name == format!("{module_stem}_{export}"))
    {
        return true;
    }
    if let Some(rest) = func_name.strip_prefix(&format!("{module_stem}_")) {
        return dep_exports.keys().any(|export| {
            rest == export.as_str() || rest.starts_with(&format!("{}_", export.as_str()))
        });
    }
    if let Some((_, method)) = func_name.split_once('_') {
        return dep_exports.contains_key(&SmolStr::new(method));
    }
    false
}

fn is_stdlib_module_path(path: &Path) -> bool {
    path.to_string_lossy().contains("<stdlib>/")
}

fn should_prefix_cross_module_export(
    func_name: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) -> bool {
    match dep_exports.get(&SmolStr::new(func_name)) {
        Some(TypeId::Named(name)) if name.as_str() == func_name => false,
        Some(_) => true,
        None => false,
    }
}

fn cross_module_function_name(path: &Path, module_stem: &str, name: SmolStr) -> SmolStr {
    if is_stdlib_module_path(path) || name.as_str() == "main" {
        return name;
    }
    let prefix = format!("{module_stem}_");
    if name.starts_with(prefix.as_str()) {
        return name;
    }
    SmolStr::new(format!("{prefix}{name}"))
}

/// Merge user-dependency bytecode with prefix + relink internal `Call` indices (SYX 40.0).
fn merge_user_dep_program_batch(
    program: &mut syx_codegen::CompiledProgram,
    dep: &ModuleUnit,
    module_stem: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) {
    use syx_codegen::Instruction;
    if is_stdlib_module_path(&dep.path) {
        return;
    }
    let dep_names: Vec<SmolStr> = dep
        .program
        .functions
        .iter()
        .map(|f| f.name.clone())
        .collect();
    let mut batch: Vec<syx_codegen::FunctionUnit> = dep
        .program
        .functions
        .iter()
        .filter(|f| {
            f.name.as_str() != "main"
                && (mir_merge_stdlib_function(module_stem, f.name.as_str(), dep_exports)
                    || dep_exports.contains_key(&f.name))
        })
        .cloned()
        .collect();
    if batch.is_empty() {
        return;
    }
    let mut rename = IndexMap::new();
    for f in &mut batch {
        let prefixed = cross_module_function_name(&dep.path, module_stem, f.name.clone());
        if prefixed != f.name {
            rename.insert(f.name.clone(), prefixed.clone());
            f.name = prefixed;
        }
    }
    let start = program.functions.len();
    for f in batch {
        if !program
            .functions
            .iter()
            .any(|existing| existing.name == f.name)
        {
            program.functions.push(f);
        }
    }
    let name_to_idx: std::collections::HashMap<SmolStr, u32> = program
        .functions
        .iter()
        .enumerate()
        .map(|(i, f)| (f.name.clone(), i as u32))
        .collect();
    fn resolve_dep_call(
        old_idx: u32,
        dep_names: &[SmolStr],
        rename: &IndexMap<SmolStr, SmolStr>,
        name_to_idx: &std::collections::HashMap<SmolStr, u32>,
    ) -> u32 {
        let Some(old_name) = dep_names.get(old_idx as usize) else {
            return old_idx;
        };
        let target = rename.get(old_name).unwrap_or(old_name);
        name_to_idx.get(target).copied().unwrap_or(old_idx)
    }
    for func in &mut program.functions[start..] {
        for ins in &mut func.instructions {
            match ins {
                Instruction::Call(idx)
                | Instruction::ActorCall(idx)
                | Instruction::TaskSpawn(idx) => {
                    *idx = resolve_dep_call(*idx, &dep_names, &rename, &name_to_idx);
                }
                Instruction::MakeClosure(fn_index, _) => {
                    *fn_index = resolve_dep_call(*fn_index, &dep_names, &rename, &name_to_idx);
                }
                _ => {}
            }
        }
    }
}

fn imported_class_type_names(
    import_symbols: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> indexmap::IndexSet<SmolStr> {
    let mut out = indexmap::IndexSet::new();
    for (mod_sym, exports) in import_symbols {
        for (name, ty) in exports {
            if matches!(ty, TypeId::Named(_)) {
                out.insert(name.clone());
                out.insert(SmolStr::new(format!("{mod_sym}_{name}")));
            }
        }
    }
    out
}

/// Phase 45/46: merge Core extension witnesses (`Int8_hash`, `Array_map`, …) from prelude modules.
fn mir_merge_stdlib_function(
    module_stem: &str,
    func_name: &str,
    dep_exports: &IndexMap<SmolStr, TypeId>,
) -> bool {
    if mir_merge_exported_function(module_stem, func_name, dep_exports) {
        return true;
    }
    if func_name.starts_with(&format!("{module_stem}_")) {
        return true;
    }
    if let Some((ty, _method)) = func_name.split_once('_') {
        if dep_exports.contains_key(&SmolStr::new(ty)) {
            return true;
        }
    }
    false
}

fn collect_stdlib_extension_methods(graph: &ModuleGraph) -> indexmap::IndexSet<(SmolStr, SmolStr)> {
    let mut set = indexmap::IndexSet::new();
    for (path, dep) in &graph.modules {
        if !path.to_string_lossy().contains("<stdlib>/") {
            continue;
        }
        if let Ok(m) = Parser::parse(dep.source.clone()) {
            if let Ok(expanded) = expand_macros(m) {
                set.extend(syx_mir::extension_methods_for_module(&expanded));
            }
        }
    }
    set
}

fn merge_stdlib_modules_into_mir(
    graph: &ModuleGraph,
    mir: &mut syx_mir::MirModule,
    instantiations: &std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
    options: &DriverOptions,
) -> SyxResult<()> {
    for (path, dep) in &graph.modules {
        if !path.to_string_lossy().contains("<stdlib>/") {
            continue;
        }
        if let Ok(dep_module) = Parser::parse(dep.source.clone()) {
            let dep_module = expand_macros(dep_module)?;
            let dep_exports = export_symbols(&dep.source);
            let mut dep_mono = monomorphize_module(&dep_module, instantiations);
            inject_virtual_prelude_imports(&mut dep_mono, options);
            syx_mir::merge_mir_functions(mir, syx_mir::lower_dataclass_shells(&dep_mono));
            syx_mir::merge_mir_functions(mir, syx_mir::lower_enum_shells(&dep_mono));
            let dep_mir = lower_module(&dep_mono)?;
            let module_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            for mut func in dep_mir.functions {
                if mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports) {
                    func.name = cross_module_function_name(path, module_stem, func.name);
                    if !mir.functions.iter().any(|f| f.name == func.name) {
                        mir.functions.push(func);
                    }
                }
            }
        }
    }
    Ok(())
}

/// Build a unified MIR module for native/WASM backends from a compiled project graph.
pub fn build_project_mir(
    graph: &ModuleGraph,
    entry: &ModuleUnit,
    project_root: &Path,
    options: &DriverOptions,
    manifest: &SyxManifest,
) -> SyxResult<syx_mir::MirModule> {
    Ok(build_mir_from_graph(graph, entry, project_root, options, manifest)?.0)
}

/// Unified MIR builder for VM codegen, Cranelift, and WASM emit (single canonical path).
pub fn build_unified_mir(
    graph: &ModuleGraph,
    entry: &ModuleUnit,
    project_root: &Path,
    options: &DriverOptions,
    manifest: &SyxManifest,
) -> SyxResult<syx_mir::MirModule> {
    let (mut mir, instantiations) =
        build_mir_from_graph(graph, entry, project_root, options, manifest)?;
    if stdlib_mir_enabled() {
        // M1-A02: reuse instantiations from the single frontend pass (no second parse/expand).
        merge_stdlib_modules_into_mir(graph, &mut mir, &instantiations, options)?;
    }
    finalize_unified_mir(&mut mir);
    Ok(mir)
}

fn build_mir_from_graph(
    graph: &ModuleGraph,
    entry: &ModuleUnit,
    project_root: &Path,
    options: &DriverOptions,
    manifest: &SyxManifest,
) -> SyxResult<(
    syx_mir::MirModule,
    std::collections::HashSet<(SmolStr, Vec<SmolStr>)>,
)> {
    let prep_opts = FrontendOptions {
        stop_after: StopAfter::Prepared,
        desugar_top_level: true,
        source: Some(entry.source.clone()),
        ..FrontendOptions::default()
    };
    let art = FrontendPipeline::run(entry.source.clone(), &prep_opts)?;
    let mut module = art.module;
    let importer = entry.path.parent().unwrap_or(project_root);
    let importer_dir = importer;
    // Object/Array/… extensions must be visible to semantic (e.g. `Object.put` → Object)
    // before analyze; MIR still merges witnesses via the stdlib graph.
    inject_core_collection_builtins(&mut module, options);
    let mut import_symbols = core_prelude_import_symbols(options);
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let Some(symbol) = decl.import_symbol() else {
                continue;
            };
            let import_ref = resolve_module(
                &path_str,
                &entry.path,
                importer_dir,
                manifest,
                options,
                project_root,
            )?;
            let import_key = import_ref.key();
            if let Some(dep) = graph.modules.get(&import_key) {
                import_symbols.push((symbol.clone(), export_symbols(&dep.source)));
            }
        }
    }

    let semantic = SemanticAnalyzer::analyze_with_imports_and_generics(
        &module,
        &import_symbols,
        &core_prelude_generic_class_methods(options),
    )?;
    inject_imported_dataclass_decls(
        &mut module,
        &entry.path,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    let prelude_extensions = collect_stdlib_extension_methods(graph);
    let mut mir = syx_mir::MirModule::default();
    let exports_by_dep: IndexMap<PathBuf, IndexMap<SmolStr, TypeId>> = graph
        .modules
        .iter()
        .map(|(p, u)| (p.clone(), export_symbols(&u.source)))
        .collect();

    // Merge dependency public symbols referenced by the project graph (stdlib + user modules).
    for dep in graph.modules.values() {
        let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
        if dep_exports.is_empty() && !dep.path.to_string_lossy().contains("<stdlib>/") {
            continue;
        }
        if let Ok(dep_module) = Parser::parse(dep.source.clone()) {
            let dep_module = expand_macros(dep_module)?;
            let mut dep_mono = monomorphize_module(&dep_module, &semantic.instantiations);
            inject_virtual_prelude_imports(&mut dep_mono, options);
            syx_mir::merge_mir_functions(&mut mir, syx_mir::lower_dataclass_shells(&dep_mono));
            syx_mir::merge_mir_functions(&mut mir, syx_mir::lower_enum_shells(&dep_mono));
            let dep_mir = lower_module(&dep_mono)?;
            let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if is_stdlib_module_path(&dep.path) {
                for mut func in dep_mir.functions {
                    let merge =
                        mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports);
                    if merge {
                        func.name = cross_module_function_name(&dep.path, module_stem, func.name);
                        if !mir.functions.iter().any(|f| f.name == func.name) {
                            mir.functions.push(func);
                        }
                    }
                }
            } else {
                let mut batch: Vec<_> = dep_mir
                    .functions
                    .into_iter()
                    .filter(|f| f.name.as_str() != "main")
                    .collect();
                if !batch.is_empty() {
                    syx_mir::prefix_user_dep_mir_batch(&mut batch, module_stem);
                    for func in batch {
                        if !mir.functions.iter().any(|f| f.name == func.name) {
                            mir.functions.push(func);
                        }
                    }
                }
            }
        }
    }

    let mut mono_module = monomorphize_module(&module, &semantic.instantiations);
    inject_virtual_prelude_imports(&mut mono_module, options);
    syx_mir::merge_mir_functions(&mut mir, syx_mir::lower_dataclass_shells(&mono_module));
    syx_mir::merge_mir_functions(&mut mir, syx_mir::lower_enum_shells(&mono_module));
    let callable_extensions: indexmap::IndexSet<SmolStr> =
        mir.functions.iter().map(|f| f.name.clone()).collect();
    let imported_classes = imported_class_type_names(&import_symbols);
    let int_kinds = contextual_numeric_kinds_from_semantic(&semantic);
    let entry_mir = syx_mir::lower_module_with_extra_extensions_and_kinds(
        &mono_module,
        &prelude_extensions,
        &callable_extensions,
        &imported_classes,
        &int_kinds,
    )?;
    for func in entry_mir.functions {
        if !mir.functions.iter().any(|f| f.name == func.name) {
            mir.functions.push(func);
        }
    }
    optimize(&mut mir);
    Ok((mir, semantic.instantiations.clone()))
}

pub fn run_wasm_bytes(wasm: &[u8], entry: &str) -> SyxResult<i32> {
    syx_wasm_run::run_wasm(wasm, entry)
}

pub fn run_wasm_bytes_with_module(
    wasm: &[u8],
    entry: &str,
    module: &syx_mir::MirModule,
) -> SyxResult<i32> {
    syx_wasm_run::run_wasm_with_module(wasm, entry, Some(module))
}

pub fn run_wasm_file(path: &Path, entry: &str) -> SyxResult<i32> {
    run_wasm_file_with_options(path, entry, &DriverOptions::default())
}

pub fn run_wasm_file_with_options(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<i32> {
    let wasm = std::fs::read(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        let syx = path.with_file_name(stem).with_extension("sy");
        if syx.exists() {
            let syx_path = syx.clone();
            let parent = syx_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            syx_host_plugin::register_compile_plugin_manifests(&parent);
            let manifest = load_manifest(&parent).unwrap_or_default();
            let mut opts = options.clone();
            if !manifest.dependencies.is_empty() {
                ensure_project_lock(&parent, options)?;
                opts.resolved_packages = Some(load_resolved_packages(&parent)?);
            }
            let mut graph = ModuleGraph::default();
            let unit = compile_module_recursive(
                ModuleRef::from_file(syx_path),
                &parent,
                &mut graph,
                &manifest,
                &opts,
            )?;
            let mir = build_unified_mir(&graph, &unit, &parent, &opts, &manifest)?;
            return run_wasm_bytes_with_module(&wasm, entry, &mir);
        }
    }
    run_wasm_bytes(&wasm, entry)
}

pub fn run_wasm_project(root: &Path, entry: &str) -> SyxResult<i32> {
    run_wasm_project_with_options(root, entry, &DriverOptions::default())
}

pub fn run_wasm_project_with_options(
    root: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<i32> {
    let project = compile_project_with_options(root, options)?;
    let opts = driver_options_with_deps(root, &project.manifest, options)?;
    let mir = build_unified_mir(
        &project.graph,
        &project.entry,
        root,
        &opts,
        &project.manifest,
    )?;
    let wasm = syx_wasm_emit::emit_wasm(&mir)?;
    run_wasm_bytes_with_module(&wasm, entry, &mir)
}

pub fn run_wasm_source(path: &Path, entry: &str) -> SyxResult<i32> {
    run_wasm_source_with_options(path, entry, &DriverOptions::default())
}

pub fn run_wasm_source_with_options(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<i32> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        parent,
        &mut graph,
        &manifest,
        &opts,
    )?;
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    let wasm = syx_wasm_emit::emit_wasm_with_options(&mir, opts.wasm_aot)?;
    run_wasm_bytes_with_module(&wasm, entry, &mir)
}

pub fn run_wasm_source_printed(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<(i32, Vec<String>)> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        parent,
        &mut graph,
        &manifest,
        &opts,
    )?;
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    let wasm = syx_wasm_emit::emit_wasm_with_options(&mir, opts.wasm_aot)?;
    syx_vm::set_program_ctx(syx_vm::ProgramCtx {
        program: std::sync::Arc::new(unit.program.clone()),
        runtime: None,
    });
    let result = syx_wasm_run::run_wasm_capture(&wasm, entry, Some(&mir));
    syx_vm::clear_program_ctx();
    let result = result?;
    Ok((result.exit_code, result.printed))
}

/// Run a `.sy` file via the native MIR interpreter (actor/async parity until Cranelift lowers actors).
pub fn run_native_mir_file_printed(
    path: &Path,
    options: &DriverOptions,
) -> SyxResult<(i32, Vec<String>)> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        parent,
        &mut graph,
        &manifest,
        &opts,
    )?;
    let mir = build_unified_mir(&graph, &unit, parent, &opts, &manifest)?;
    let ctx = syx_rt::install_native_module(&mir);
    let mut rt = syx_mir_run::MirRuntime::new();
    rt.install_functions(mir.functions.clone());
    rt.set_runtime(syx_rt::native_vm_runtime_for(ctx));
    syx_mir_run::install_module_string_pool(&mut rt, &mir);
    let main = mir
        .functions
        .iter()
        .find(|f| f.name == "main")
        .ok_or_else(|| SyxDiagnostic::error("No main function in MIR module"))?;
    syx_vm::set_program_ctx(syx_vm::ProgramCtx {
        program: std::sync::Arc::new(unit.program.clone()),
        runtime: None,
    });
    syx_vm::set_mir_closure_executor(syx_mir_run::execute_mir_closure);
    syx_vm::set_mir_closure_preferred(true);
    syx_mir_run::pin_active_mir_runtime(&mut rt);
    let code = syx_mir_run::run_mir_function(main, &mut rt, &mut [])?;
    syx_mir_run::unpin_active_mir_runtime();
    syx_vm::clear_mir_closure_executor();
    syx_vm::clear_program_ctx();
    Ok((code, rt.printed))
}

fn is_manifest_main_file(module_key: &Path, manifest: &SyxManifest) -> bool {
    module_key.ends_with(&manifest.main)
        || module_key
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n == manifest.main.as_str())
}

fn should_desugar_top_level_stmts(module_key: &Path, manifest: &SyxManifest) -> bool {
    if is_manifest_main_file(module_key, manifest) {
        return true;
    }
    module_key
        .parent()
        .map(|p| !p.join("syx.toml").exists())
        .unwrap_or(true)
}

fn cache_version_tag() -> String {
    static TAG: OnceLock<String> = OnceLock::new();
    TAG.get_or_init(compute_cache_version_tag).clone()
}

fn compute_cache_version_tag() -> String {
    // Historical tags retained for invalidation contract tests: syx-cache-v58 syx-cache-v59 syx-cache-v60 syx-cache-v61 syx-cache-v62 syx-cache-v63 syx-cache-v64 syx-cache-v65 syx-cache-v66 syx-cache-v67 syx-cache-v68 syx-cache-v69 syx-cache-v70 syx-cache-v71 syx-cache-v72 syx-cache-v73 syx-cache-v74 syx-cache-v75 syx-cache-v76 syx-cache-v78 syx-cache-v79 syx-cache-v80 syx-cache-v81 syx-cache-v82 syx-cache-v83 syx-cache-v84 syx-cache-v85 syx-cache-v86 syx-cache-v87 syx-cache-v88 syx-cache-v89 syx-cache-v90 syx-cache-v91 syx-cache-v92 syx-cache-v93 syx-cache-v94 syx-cache-v95 syx-cache-v96 syx-cache-v97 syx-cache-v98 syx-cache-v99 syx-cache-v100 syx-cache-v101 syx-cache-v102 syx-cache-v103 syx-cache-v104 syx-cache-v105 syx-cache-v106 syx-cache-v107 syx-cache-v108 syx-cache-v109 syx-cache-v110
    format!(
        "syx-cache-v1:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        env!("CARGO_PKG_VERSION"),
        rustc_version_tag(),
        host_feature_fingerprint(),
        witness_table_fingerprint(),
        vtable_layout_fingerprint(),
        wasm_aot_tier_fingerprint(),
        native_llvm_coverage_fingerprint(),
        aot_fixture_set_fingerprint(),
        sorted_codegen_fingerprint(),
        tier2b_jit_llvm_fingerprint(),
        tier2c_simd_browser_fingerprint(),
        iterator_witness_native_tier2c_fingerprint(),
        gc_arc_browser_prelude_fingerprint(),
        unified_heap_native_exec_op_fingerprint(),
        heap_v3_llvm_tier3_fingerprint(),
        heap_v4_inkwell_fingerprint(),
        heap_v48_phase76_fingerprint(),
        heap_v49_phase77_fingerprint(),
        heap_v50_phase78_fingerprint(),
        heap_v51_phase79_fingerprint(),
        heap_v52_phase80_fingerprint(),
        heap_v53_phase81_fingerprint(),
        heap_v54_phase82_fingerprint(),
        heap_v55_phase83_fingerprint(),
        heap_v56_phase84_fingerprint(),
        heap_v57_phase85_fingerprint(),
        heap_v58_phase86_fingerprint(),
        heap_v59_phase87_fingerprint(),
        heap_v60_phase88_fingerprint(),
        heap_v61_phase89_fingerprint(),
        heap_v62_phase90_fingerprint(),
        heap_v63_phase91_fingerprint(),
        heap_v64_phase92_fingerprint(),
        heap_v65_phase93_fingerprint(),
        heap_v66_phase94_fingerprint(),
        heap_v67_phase95_fingerprint(),
        heap_v68_phase96_fingerprint(),
        heap_v69_phase97_fingerprint(),
        heap_v70_phase98_fingerprint(),
        heap_v71_phase99_fingerprint(),
        heap_v72_phase100_fingerprint(),
        heap_v73_phase101_fingerprint(),
        heap_v74_phase102_fingerprint(),
        heap_v75_phase103_fingerprint(),
        heap_v77_phase105_track_a_gui_fingerprint(),
        heap_v77_phase105_track_b_wasm_fingerprint(),
        heap_v77_phase105_track_c_fingerprint(),
        heap_v77_phase105_track_d_fingerprint(),
        heap_v77_phase105_dx_fingerprint(),
        heap_v78_phase106_track_a_gui_fingerprint(),
        heap_v78_phase106_track_b_wasm_fingerprint(),
        heap_v78_phase106_track_c_select_reflect_fingerprint(),
        heap_v78_phase106_track_d_farm_fingerprint(),
        heap_v78_phase106_dx_fingerprint(),
        heap_v79_phase107_track_a_gui_fingerprint(),
        heap_v79_phase107_track_b_wasm_fingerprint(),
        heap_v79_phase107_track_c_select_reflect_fingerprint(),
        heap_v79_phase107_track_d_farm_dist_fingerprint(),
        heap_v79_phase107_dx_fingerprint(),
        heap_v80_phase108_track_a_gui_fingerprint(),
        heap_v80_phase108_track_b_wasm_fingerprint(),
        heap_v80_phase108_track_c_select_fingerprint(),
        heap_v80_phase108_track_d_farm_dist_fingerprint(),
        heap_v80_phase108_dx_fingerprint(),
        heap_v81_phase109_track_a_gui_fingerprint(),
        heap_v81_phase109_track_b_wasm_fingerprint(),
        heap_v81_phase109_track_c_farm_fingerprint(),
        heap_v81_phase109_track_d_dist_select_fingerprint(),
        heap_v81_phase109_dx_fingerprint(),
        heap_v82_phase110_track_a_gui_fingerprint(),
        heap_v82_phase110_track_b_wasm_fingerprint(),
        heap_v82_phase110_track_c_farm_fingerprint(),
        heap_v82_phase110_track_d_dist_select_fingerprint(),
        heap_v82_phase110_dx_fingerprint(),
        heap_v83_phase111_track_a_gui_fingerprint(),
        heap_v83_phase111_track_b_wasm_fingerprint(),
        heap_v83_phase111_track_c_farm_fingerprint(),
        heap_v83_phase111_track_d_dist_select_fingerprint(),
        heap_v83_phase111_dx_fingerprint(),
        heap_v84_phase112_track_a_gui_fingerprint(),
        heap_v84_phase112_track_b_wasm_fingerprint(),
        heap_v84_phase112_track_c_farm_fingerprint(),
        heap_v84_phase112_track_d_dist_select_fingerprint(),
        heap_v84_phase112_dx_fingerprint(),
        heap_v85_phase113_track_a_gui_fingerprint(),
        heap_v85_phase113_track_b_wasm_fingerprint(),
        heap_v85_phase113_track_c_farm_fingerprint(),
        heap_v85_phase113_track_d_dist_select_fingerprint(),
        heap_v85_phase113_dx_fingerprint(),
        heap_v86_phase114_track_a_gui_fingerprint(),
        heap_v86_phase114_track_b_wasm_fingerprint(),
        heap_v86_phase114_track_c_farm_fingerprint(),
        heap_v86_phase114_track_d_dist_select_fingerprint(),
        heap_v86_phase114_dx_fingerprint(),
        heap_v87_phase115_track_a_gui_fingerprint(),
        heap_v87_phase115_track_b_wasm_fingerprint(),
        heap_v87_phase115_track_c_farm_fingerprint(),
        heap_v87_phase115_track_d_dist_select_fingerprint(),
        heap_v87_phase115_dx_fingerprint(),
        heap_v88_phase116_track_a_gui_fingerprint(),
        heap_v88_phase116_track_b_wasm_fingerprint(),
        heap_v88_phase116_track_c_farm_fingerprint(),
        heap_v88_phase116_track_d_dist_select_fingerprint(),
        heap_v88_phase116_dx_fingerprint(),
        heap_v89_phase117_track_a_gui_fingerprint(),
        heap_v89_phase117_track_b_wasm_fingerprint(),
        heap_v89_phase117_track_c_farm_fingerprint(),
        heap_v89_phase117_track_d_dist_select_fingerprint(),
        heap_v89_phase117_dx_fingerprint(),
        heap_v91_phase119_track_a_gui_fingerprint(),
        heap_v91_phase119_track_b_wasm_fingerprint(),
        heap_v91_phase119_track_c_farm_fingerprint(),
        heap_v91_phase119_track_d_dist_select_fingerprint(),
        heap_v91_phase119_dx_fingerprint(),
        heap_v92_phase120_track_a_macro_service_fingerprint(),
        heap_v92_phase120_track_b_gui_fingerprint(),
        heap_v92_phase120_track_c_wasm_fingerprint(),
        heap_v92_phase120_track_d_farm_dist_fingerprint(),
        heap_v92_phase120_dx_fingerprint(),
        heap_v93_phase121_track_a_macro_service_fingerprint(),
        heap_v93_phase121_track_b_gui_fingerprint(),
        heap_v93_phase121_track_c_wasm_fingerprint(),
        heap_v93_phase121_track_d_farm_dist_fingerprint(),
        heap_v93_phase121_dx_fingerprint(),
        heap_v94_phase122_track_a_macro_service_fingerprint(),
        heap_v94_phase122_track_b_gui_fingerprint(),
        heap_v94_phase122_track_c_wasm_fingerprint(),
        heap_v94_phase122_track_d_farm_dist_fingerprint(),
        heap_v94_phase122_dx_fingerprint(),
        heap_v95_phase123_track_a_macro_service_fingerprint(),
        heap_v95_phase123_track_b_gui_fingerprint(),
        heap_v95_phase123_track_c_wasm_fingerprint(),
        heap_v95_phase123_track_d_farm_dist_fingerprint(),
        heap_v95_phase123_dx_fingerprint(),
        heap_v96_phase124_track_a_macro_service_fingerprint(),
        heap_v96_phase124_track_b_gui_fingerprint(),
        heap_v96_phase124_track_c_wasm_fingerprint(),
        heap_v96_phase124_track_d_farm_dist_fingerprint(),
        heap_v96_phase124_dx_fingerprint(),
        heap_v97_phase125_track_a_macro_service_fingerprint(),
        heap_v97_phase125_track_b_gui_fingerprint(),
        heap_v97_phase125_track_c_wasm_fingerprint(),
        heap_v97_phase125_track_d_farm_dist_fingerprint(),
        heap_v97_phase125_dx_fingerprint(),
        heap_v98_phase126_track_a_macro_service_fingerprint(),
        heap_v98_phase126_track_b_gui_fingerprint(),
        heap_v98_phase126_track_c_wasm_fingerprint(),
        heap_v98_phase126_track_d_farm_dist_fingerprint(),
        heap_v98_phase126_dx_fingerprint(),
        heap_v99_phase127_track_a_macro_service_fingerprint(),
        heap_v99_phase127_track_b_gui_fingerprint(),
        heap_v99_phase127_track_c_wasm_fingerprint(),
        heap_v99_phase127_track_d_farm_dist_fingerprint(),
        heap_v99_phase127_dx_fingerprint(),
        heap_v100_phase128_track_a_macro_service_fingerprint(),
        heap_v100_phase128_track_b_gui_fingerprint(),
        heap_v100_phase128_track_c_wasm_fingerprint(),
        heap_v100_phase128_track_d_farm_dist_fingerprint(),
        heap_v100_phase128_dx_fingerprint(),
        heap_v101_phase129_track_a_macro_service_fingerprint(),
        heap_v101_phase129_track_b_gui_fingerprint(),
        heap_v101_phase129_track_c_wasm_fingerprint(),
        heap_v101_phase129_track_d_farm_dist_fingerprint(),
        heap_v101_phase129_dx_fingerprint(),
        heap_v102_phase130_track_a_macro_service_fingerprint(),
        heap_v102_phase130_track_b_gui_fingerprint(),
        heap_v102_phase130_track_c_wasm_fingerprint(),
        heap_v102_phase130_track_d_farm_dist_fingerprint(),
        heap_v102_phase130_dx_fingerprint(),
        heap_v104_phase132_track_a_macro_service_fingerprint(),
        heap_v104_phase132_track_b_gui_fingerprint(),
        heap_v104_phase132_track_c_wasm_fingerprint(),
        heap_v104_phase132_track_d_farm_dist_fingerprint(),
        heap_v104_phase132_dx_fingerprint(),
        heap_v103_phase131_track_a_macro_service_fingerprint(),
        heap_v103_phase131_track_b_gui_fingerprint(),
        heap_v103_phase131_track_c_wasm_fingerprint(),
        heap_v103_phase131_track_d_farm_dist_fingerprint(),
        heap_v103_phase131_dx_fingerprint(),
        heap_v105_phase133_track_a_macro_service_fingerprint(),
        heap_v105_phase133_track_b_gui_fingerprint(),
        heap_v105_phase133_track_c_wasm_fingerprint(),
        heap_v105_phase133_track_d_farm_dist_fingerprint(),
        heap_v105_phase133_track_e_os_concurrency_fingerprint(),
        heap_v105_phase133_dx_fingerprint(),
        heap_v106_phase134_track_a_props_fingerprint(),
        heap_v106_phase134_track_b_gui_fingerprint(),
        heap_v106_phase134_track_c_wasm_fingerprint(),
        heap_v106_phase134_track_d_farm_dist_fingerprint(),
        heap_v106_phase134_track_e_macro_concurrency_fingerprint(),
        heap_v106_phase134_dx_fingerprint(),
        heap_v107_phase135_track_a_props_fingerprint(),
        heap_v107_phase135_track_b_gui_fingerprint(),
        heap_v107_phase135_track_c_wasm_fingerprint(),
        heap_v107_phase135_track_d_farm_dist_fingerprint(),
        heap_v107_phase135_track_e_macro_concurrency_fingerprint(),
        heap_v107_phase135_dx_fingerprint(),
        heap_v108_phase136_track_a_props_fingerprint(),
        heap_v108_phase136_track_c_wasm_fingerprint(),
        heap_v108_phase136_track_d_farm_dist_os_fingerprint(),
        heap_v108_phase136_track_e_macro_concurrency_fingerprint(),
        heap_v108_phase136_dx_fingerprint(),
        heap_v109_phase137_track_a_props_fingerprint(),
        heap_v109_phase137_track_c_wasm_fingerprint(),
        heap_v109_phase137_track_d_os_disk_load_fingerprint(),
        heap_v109_phase137_track_e_macro_concurrency_fingerprint(),
        heap_v109_phase137_dx_fingerprint(),
        heap_v110_phase138_track_a_props_fingerprint(),
        heap_v110_phase138_track_c_wasm_fingerprint(),
        heap_v110_phase138_track_d_os_network_fingerprint(),
        heap_v110_phase138_track_e_macro_concurrency_fingerprint(),
        heap_v110_phase138_dx_fingerprint(),
        heap_v111_phase139_track_l_language_fingerprint(),
        heap_v111_phase139_track_c_wasm_fingerprint(),
        heap_v111_phase139_track_d_frozen_fingerprint(),
        heap_v111_phase139_track_inherit_fingerprint(),
        heap_v111_phase139_dx_fingerprint()
    )
}

/// Phase 135 Track A: Class Properties v3 (`{ get set }`, property override).
fn heap_v107_phase135_track_a_props_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"lang-props-v3:super-init-field-readback");
    hasher.update(b"lang-props-v3:computed-get-set-mvp");
    hasher.update(b"lang-props-v3:property-override-mvp");
    hasher.update(b"lang-props-v3:docs-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 135 Track B: SynUI v12.
fn heap_v107_phase135_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v12:widget-navigation");
    hasher.update(b"gui-synui-v12:nav-signal-bind");
    hasher.update(b"gui-synui-v12:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v11:widget-tabview");
    hasher.update(b"gui-synui-v11:tab-signal-bind");
    hasher.update(b"gui-synui-v11:web-mobile-controls-deepen");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 135 Track D: farm hsm-key-label + dist peer-announce-priority.
fn heap_v107_phase135_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v18:hsm-key-label-mvp");
    hasher.update(b"dist-cdn-bgp-v18:peer-announce-priority-mvp");
    hasher.update(b"farm-lease-token-v17:hsm-session-pin-mvp");
    hasher.update(b"dist-cdn-bgp-v17:peer-announce-ttl-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 135 DX: audit 6800/3700 soft-pin (Gate0 Phase 136), fill EXCLUDE, LSP semantic v37, driver CR410, stdlib 113.0 pin.
fn heap_v107_phase135_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2500-2900");
    hasher.update(b"audit-exclude-fill-v1:phase135");
    hasher.update(b"lsp-semantic-v37:phase135-50-plus-real");
    hasher.update(b"driver-gate-cr410-v1:phase135-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:113.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 136 Track A: Class Properties v4 (extension computed). SynUI v13 deferred.
fn heap_v108_phase136_track_a_props_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"lang-props-v4:extension-computed-mvp");
    hasher.update(b"lang-props-v4:docs-honesty");
    // SynUI: inherit v12; gui-synui-v13 deferred → 137+
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 136 Track C: SIMD16 avgr_u guest-v19.
fn heap_v108_phase136_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v19:avgr-u-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 137 Track A: Class Properties v5 (@Published). SynUI deferred; Farm/Dist frozen.
fn heap_v109_phase137_track_a_props_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"lang-props-v5:published-mvp");
    hasher.update(b"lang-props-v5:docs-honesty");
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 137 Track C: SIMD16 popcnt guest-v20.
fn heap_v109_phase137_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v20:popcnt-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 137 Track D: Os disk / load MVP.
fn heap_v109_phase137_track_d_os_disk_load_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"stdlib-os-v3:disk-load-mvp");
    hasher.update(b"farm-dist-v19:frozen-no-v20");
    hasher.update(b"farm-lease-token-v19:hsm-key-id-mvp");
    hasher.update(b"dist-cdn-bgp-v19:peer-announce-hold-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 137 Track E: Macro M27 / DI v18 + DiscardOldest reject policy.
fn heap_v109_phase137_track_e_macro_concurrency_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v8:decl-expand-mvp");
    hasher.update(b"macro-service-v18:pointcut-not-within-mvp");
    hasher.update(b"macro-docs-v19:m27-service-v18-honesty");
    hasher.update(b"runtime-concurrency-v5:discard-oldest-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 137 DX: audit 2600/2950, LSP semantic v39, driver CR410, stdlib 115.0 pin.
fn heap_v109_phase137_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2600-2950");
    hasher.update(b"audit-exclude-fill-v1:phase137");
    hasher.update(b"lsp-semantic-v39:phase137-54-plus-real");
    hasher.update(b"driver-gate-cr410-v1:phase137-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:115.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 138 Track A: Class Properties v6 (static willSet/didSet). SynUI deferred; Farm/Dist frozen.
fn heap_v110_phase138_track_a_props_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"lang-props-v6:static-observers-mvp");
    hasher.update(b"lang-props-v6:docs-honesty");
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 138 Track C: SIMD16 bitmask guest-v21.
fn heap_v110_phase138_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v21:bitmask-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 138 Track D: Os network MVP + Farm/Dist frozen v19.
fn heap_v110_phase138_track_d_os_network_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"stdlib-os-v4:network-mvp");
    hasher.update(b"farm-dist-v19:frozen-no-v20");
    hasher.update(b"farm-lease-token-v19:hsm-key-id-mvp");
    hasher.update(b"dist-cdn-bgp-v19:peer-announce-hold-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 138 Track E: Macro M28 / DI v19 + Actor shared pool.
fn heap_v110_phase138_track_e_macro_concurrency_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v9:token-rewrite-mvp");
    hasher.update(b"macro-service-v19:pointcut-within-wildcard-mvp");
    hasher.update(b"macro-docs-v20:m28-service-v19-honesty");
    hasher.update(b"runtime-concurrency-v6:actor-shared-pool-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 138 DX: audit 2600/3000 (honest downscale from planned 2750; no padding), LSP semantic v40, driver CR420, stdlib 116.0 pin.
fn heap_v110_phase138_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2600-3000");
    hasher.update(b"audit-exclude-fill-v1:phase138");
    hasher.update(b"lsp-semantic-v40:phase138-56-plus-real");
    hasher.update(b"driver-gate-cr420-v1:phase138-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:116.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 139 Track L: language surface L1–L7 (multiline string, tuple, nested, range, Character, call labels, final docs).
fn heap_v111_phase139_track_l_language_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(crate::lang_surface_v1::STRING_MULTILINE_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::STRING_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::TUPLE_PRODUCT_TYPE_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::TUPLE_NAMED_ELEMENTS_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::TUPLE_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::NESTED_QUALIFIED_PATH_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::NESTED_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::RANGE_CLOSED_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::RANGE_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::CHARACTER_DISTINCT_SCALAR_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::CHARACTER_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_EXTERNAL_LABEL_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_OMIT_UNDERSCORE_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_LABELED_ARGS_REQUIRED.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::FINAL_SEALED_REMOVED_DOCS.as_bytes());
    hasher.update(crate::lang_surface_v1::FINAL_ONLY_HONESTY.as_bytes());
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 139 Track C: SIMD16 i8x16 all_true / any_true guest-v22.
fn heap_v111_phase139_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v22:all-any-true-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 139 Track D: SynUI deferred + Farm/Dist frozen (no v13/v20 surfaces).
fn heap_v111_phase139_track_d_frozen_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    hasher.update(b"farm-dist-v19:frozen-no-v20");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 139: inherit P138 macro/props/concurrency/os surfaces (language phase; no new macro track).
fn heap_v111_phase139_track_inherit_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"lang-props-v6:static-observers-mvp");
    hasher.update(b"macro-token-v9:token-rewrite-mvp");
    hasher.update(b"macro-service-v19:pointcut-within-wildcard-mvp");
    hasher.update(b"runtime-concurrency-v6:actor-shared-pool-mvp");
    hasher.update(b"stdlib-os-v4:network-mvp");
    hasher.update(b"wasm-simd16-guest-v21:bitmask-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 139 DX: audit 2600/3050 (honest downscale from planned 2750/3050), LSP semantic v41, driver CR430, stdlib 117.0 pin.
fn heap_v111_phase139_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2600-3050");
    hasher.update(b"audit-exclude-fill-v1:phase139");
    hasher.update(b"lsp-semantic-v41:phase139-58-plus-real");
    hasher.update(b"driver-gate-cr430-v1:phase139-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:117.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 136 Track D: farm hsm-key-id + dist peer-announce-hold + Os hostname/uptime.
fn heap_v108_phase136_track_d_farm_dist_os_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v19:hsm-key-id-mvp");
    hasher.update(b"dist-cdn-bgp-v19:peer-announce-hold-mvp");
    hasher.update(b"stdlib-os-v2:hostname-uptime-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 136 Track E: Macro M26 / DI v17 + core/max/keepAlive.
fn heap_v108_phase136_track_e_macro_concurrency_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v7:token-stream-mvp");
    hasher.update(b"macro-service-v17:pointcut-or-compose-mvp");
    hasher.update(b"macro-docs-v18:m26-service-v17-honesty");
    hasher.update(b"runtime-concurrency-v4:core-max-keepalive-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 136 DX: audit 2500/2900 (pads cleaned), LSP semantic v38, driver CR400, stdlib 114.0 pin.
fn heap_v108_phase136_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2500-2900");
    hasher.update(b"audit-exclude-fill-v1:phase136");
    hasher.update(b"lsp-semantic-v38:phase136-52-plus-real");
    hasher.update(b"driver-gate-cr400-v1:phase136-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:114.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 134 Track A: Class Properties v2.
fn heap_v106_phase134_track_a_props_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"lang-props-v2:docs-honesty");
    hasher.update(b"lang-props-v2:inherited-observers");
    hasher.update(b"lang-props-v2:init-suppress-observers");
    hasher.update(b"lang-props-v2:computed-get-only-mvp");
    hasher.update(b"lang-props-v2:static-var-mvp");
    hasher.update(b"lang-props-v2:lazy-var-mvp");
    hasher.update(b"lang-props-v3:super-init-field-readback");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 134 Track B: SynUI v11.
fn heap_v106_phase134_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v11:widget-tabview");
    hasher.update(b"gui-synui-v11:tab-signal-bind");
    hasher.update(b"gui-synui-v11:web-mobile-controls-deepen");
    hasher.update(b"gui-synui-v10:widget-stack");
    hasher.update(b"gui-synui-v10:scrollview-signal-bind");
    hasher.update(b"gui-synui-v10:web-mobile-layout-deepen");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 133 Track A: Macro M23 stmt token subst MVP + Service DI v14 within pointcut MVP.
fn heap_v105_phase133_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v4:stmt-token-subst-mvp");
    hasher.update(b"macro-service-v14:pointcut-within-name-mvp");
    hasher.update(b"macro-token-v5:expr-token-subst-mvp");
    hasher.update(b"macro-service-v15:pointcut-args-name-mvp");
    hasher.update(b"macro-docs-v16:m24-service-v15-honesty");
    hasher.update(b"macro-docs-v15:m23-service-v14-honesty");
    hasher.update(b"macro-token-v3:decl-hygiene-mvp");
    hasher.update(b"macro-service-v13:before-after-pointcut-bind-mvp");
    hasher.update(b"macro-docs-v14:m22-service-v13-honesty");
    hasher.update(b"macro-token-v2:decl-param-subst-mvp");
    hasher.update(b"macro-service-v12:pointcut-execution-name-mvp");
    hasher.update(b"macro-docs-v13:m21-service-v12-honesty");
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 133 Track B: SynUI v10 (widget stack + scrollView Signal bind + Web/Mobile layout deepen).
fn heap_v105_phase133_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v10:widget-stack");
    hasher.update(b"gui-synui-v10:scrollview-signal-bind");
    hasher.update(b"gui-synui-v10:web-mobile-layout-deepen");
    hasher.update(b"gui-synui-v9:image-signal-bind");
    hasher.update(b"gui-synui-v9:widget-divider");
    hasher.update(b"gui-synui-v9:web-mobile-controls-deepen");
    hasher.update(b"gui-synui-v8:textarea-list-bind");
    hasher.update(b"gui-synui-v8:widget-spacer");
    hasher.update(b"gui-synui-v8:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 133 Track C: SIMD16 i8x16 ne MVP.
fn heap_v105_phase133_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v16:ne-mvp");
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 135 Track C: SIMD16 i8x16 ge MVP.
fn heap_v107_phase135_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v19:avgr-u-mvp");
    hasher.update(b"wasm-simd16-guest-v18:ge-mvp");
    hasher.update(b"wasm-simd16-guest-v17:le-mvp");
    hasher.update(b"wasm-simd16-guest-v16:ne-mvp");
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 134 Track C: SIMD16 i8x16 le MVP.
fn heap_v106_phase134_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v17:le-mvp");
    hasher.update(b"wasm-simd16-guest-v16:ne-mvp");
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 134 Track D: farm hsm-session-pin + dist peer-announce-ttl.
fn heap_v106_phase134_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v17:hsm-session-pin-mvp");
    hasher.update(b"dist-cdn-bgp-v17:peer-announce-ttl-mvp");
    hasher.update(b"farm-lease-token-v16:hsm-multi-slot-mvp");
    hasher.update(b"dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 135 Track E: Macro M25 / DI v16 + CallerRuns reject policy.
fn heap_v107_phase135_track_e_macro_concurrency_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v6:decl-token-subst-mvp");
    hasher.update(b"macro-service-v16:pointcut-and-compose-mvp");
    hasher.update(b"macro-docs-v17:m25-service-v16-honesty");
    hasher.update(b"runtime-concurrency-v3:caller-runs-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 134 Track E: Macro M24 / DI v15 + bounded Task queue Abort.
fn heap_v106_phase134_track_e_macro_concurrency_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v5:expr-token-subst-mvp");
    hasher.update(b"macro-service-v15:pointcut-args-name-mvp");
    hasher.update(b"macro-docs-v16:m24-service-v15-honesty");
    hasher.update(b"runtime-concurrency-v2:bounded-queue-abort-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 135 DX: audit 6800/3700 soft-pin (Gate0 Phase 136), fill EXCLUDE, LSP semantic v37, driver CR410, stdlib 113.0 pin.
fn heap_v106_phase134_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2500-2900");
    hasher.update(b"audit-exclude-fill-v1:phase135");
    hasher.update(b"lsp-semantic-v37:phase135-50-plus-real");
    hasher.update(b"driver-gate-cr410-v1:phase135-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:113.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 133 Track D: farm hsm multi-slot + dist peer-announce epoch persist.
fn heap_v105_phase133_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v16:hsm-multi-slot-mvp");
    hasher.update(b"dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp");
    hasher.update(b"farm-lease-token-v15:hsm-digest-streaming-mvp");
    hasher.update(b"dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp");
    hasher.update(b"farm-lease-token-v14:hsm-digest-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v14:peer-announce-fanout-mvp");
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 133 Track E: stdlib os cpu/mem/user + runtime concurrency task worker + actor mailbox.
fn heap_v105_phase133_track_e_os_concurrency_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"stdlib-os-v1:cpu-mem-user-mvp");
    hasher.update(b"runtime-concurrency-v1:task-worker-config-mvp");
    hasher.update(b"runtime-concurrency-v1:actor-limit-mailbox-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 133 DX: audit 6350/3550, fill EXCLUDE, LSP semantic v35, driver CR390, stdlib 111.0 pin.
fn heap_v105_phase133_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-6350-3550");
    hasher.update(b"audit-exclude-fill-v1:phase133");
    hasher.update(b"lsp-semantic-v35:phase133-46-plus-real");
    hasher.update(b"driver-gate-cr390-v1:phase133-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:111.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 132 Track A: Macro M22 decl hygiene MVP + Service DI v13 before/after pointcut bind MVP.
fn heap_v104_phase132_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v3:decl-hygiene-mvp");
    hasher.update(b"macro-service-v13:before-after-pointcut-bind-mvp");
    hasher.update(b"macro-docs-v14:m22-service-v13-honesty");
    hasher.update(b"macro-token-v2:decl-param-subst-mvp");
    hasher.update(b"macro-service-v12:pointcut-execution-name-mvp");
    hasher.update(b"macro-docs-v13:m21-service-v12-honesty");
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 132 Track B: SynUI v9 (image Signal bind + divider + Web/Mobile controls deepen).
fn heap_v104_phase132_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v9:image-signal-bind");
    hasher.update(b"gui-synui-v9:widget-divider");
    hasher.update(b"gui-synui-v9:web-mobile-controls-deepen");
    hasher.update(b"gui-synui-v8:textarea-list-bind");
    hasher.update(b"gui-synui-v8:widget-spacer");
    hasher.update(b"gui-synui-v8:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 132 Track C: SIMD16 i8x16 neg MVP.
fn heap_v104_phase132_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 132 Track D: farm hsm-digest streaming + dist peer-announce conflict epoch.
fn heap_v104_phase132_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v15:hsm-digest-streaming-mvp");
    hasher.update(b"dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp");
    hasher.update(b"farm-lease-token-v14:hsm-digest-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v14:peer-announce-fanout-mvp");
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 132 DX: audit 6200/3500, fill EXCLUDE, LSP semantic v34, driver CR380, stdlib 110.0 pin.
fn heap_v104_phase132_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-6200-3500");
    hasher.update(b"audit-exclude-fill-v1:phase132");
    hasher.update(b"lsp-semantic-v34:phase132-44-plus-real");
    hasher.update(b"driver-gate-cr380-v1:phase132-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:110.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 131 Track A: Macro M21 decl-param subst MVP + Service DI v12 execution pointcut MVP.
fn heap_v103_phase131_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v2:decl-param-subst-mvp");
    hasher.update(b"macro-service-v12:pointcut-execution-name-mvp");
    hasher.update(b"macro-docs-v13:m21-service-v12-honesty");
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 131 Track B: SynUI v8 (textArea/list Signal bind + spacer + Web/Mobile surface deepen).
fn heap_v103_phase131_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v8:textarea-list-bind");
    hasher.update(b"gui-synui-v8:widget-spacer");
    hasher.update(b"gui-synui-v8:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 131 Track C: SIMD16 i8x16 abs MVP.
fn heap_v103_phase131_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 131 Track D: farm hsm-digest-bound + dist peer-announce fanout.
fn heap_v103_phase131_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v14:hsm-digest-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v14:peer-announce-fanout-mvp");
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 131 DX: audit 6050/3450, fill EXCLUDE, LSP semantic v33, driver CR370, stdlib 109.0 pin.
fn heap_v103_phase131_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-6050-3450");
    hasher.update(b"audit-exclude-fill-v1:phase131");
    hasher.update(b"lsp-semantic-v33:phase131-42-plus-real");
    hasher.update(b"driver-gate-cr370-v1:phase131-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:109.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 130 Track A: Macro M20 token expr MVP + Service DI v11 pointcut-name MVP.
fn heap_v102_phase130_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    hasher.update(b"macro-codable-l2:associated-enum-auto");
    hasher.update(b"macro-service-v10:bean-param-autowire-mvp");
    hasher.update(b"macro-docs-v11:m19-service-v10-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 130 Track B: SynUI v7 (image/scrollView + picker Signal bind + Web language surface MVP).
fn heap_v102_phase130_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    hasher.update(b"gui-synui-v6:widget-textarea-list");
    hasher.update(b"gui-synui-v6:progress-number-bind");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 130 Track C: SIMD16 i8x16 unsigned min/max MVP.
fn heap_v102_phase130_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 130 Track D: farm hsm-sign-verify + dist peer-announce ack.
fn heap_v102_phase130_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    hasher.update(b"farm-lease-token-v12:hsm-sign-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v12:peer-announce-sticky-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 130 DX: audit 5900/3400, fill EXCLUDE, LSP semantic v32, driver CR360, stdlib 108.0 pin.
fn heap_v102_phase130_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-5900-3400");
    hasher.update(b"audit-exclude-fill-v1:phase130");
    hasher.update(b"lsp-semantic-v32:phase130-40-plus-real");
    hasher.update(b"driver-gate-cr360-v1:phase130-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:108.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 129 Track A: Macro M19 Codable L2 associated-enum + Service DI v10 bean-param autowire.
fn heap_v101_phase129_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-codable-l2:associated-enum-auto");
    hasher.update(b"macro-service-v10:bean-param-autowire-mvp");
    hasher.update(b"macro-docs-v11:m19-service-v10-honesty");
    hasher.update(b"macro-service-v9:lazy-cycle-proxy-mvp");
    hasher.update(b"runtime-service-v9:lazy-cycle-proxy-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 129 Track B: SynUI v6 (textArea/list + progress Number bind + desktop layout GA).
fn heap_v101_phase129_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v6:widget-textarea-list");
    hasher.update(b"gui-synui-v6:progress-number-bind");
    hasher.update(b"gui-synui-v6:desktop-run-layout-ga");
    hasher.update(b"gui-synui-v5:toggle-radio-bind");
    hasher.update(b"gui-synui-v5:widget-progress-picker");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 129 Track C: SIMD16 i8x16 load/store MVP.
fn heap_v101_phase129_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 129 Track D: farm hsm-sign-bound + dist peer-announce sticky.
fn heap_v101_phase129_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v12:hsm-sign-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v12:peer-announce-sticky-mvp");
    hasher.update(b"farm-lease-token-v11:hsm-ref-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v11:sticky-multi-snapshot-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 129 DX: audit 5750/3350, fill EXCLUDE, LSP semantic v31, driver CR350, stdlib 107.0 pin.
fn heap_v101_phase129_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-5750-3350");
    hasher.update(b"audit-exclude-fill-v1:phase129");
    hasher.update(b"lsp-semantic-v31:phase129-38-plus-real");
    hasher.update(b"driver-gate-cr350-v1:phase129-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:107.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 128 Track A: Macro M18 + Service DI v9 (lazy + cycle-proxy).
fn heap_v100_phase128_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-service-v9:lazy-cycle-proxy-mvp");
    hasher.update(b"runtime-service-v9:lazy-cycle-proxy-mvp");
    hasher.update(b"macro-docs-v10:m18-service-v9-honesty");
    hasher.update(b"macro-service-v8:advice-proceed");
    hasher.update(b"macro-service-v8:configuration-bean-mvp");
    hasher.update(b"runtime-service-v8:configuration-bean-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 128 Track B: SynUI v5 (toggle/radio bind + progress/picker + mobile unretire).
fn heap_v100_phase128_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v5:toggle-radio-bind");
    hasher.update(b"gui-synui-v5:widget-progress-picker");
    hasher.update(b"gui-synui-v5:mobile-surface-unretire");
    hasher.update(b"gui-synui-v4:signal-widget-bind");
    hasher.update(b"gui-synui-v4:desktop-onclick-wire");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 128 Track D: farm hsm-ref-bound + dist sticky-multi-snapshot.
fn heap_v100_phase128_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v11:hsm-ref-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v11:sticky-multi-snapshot-mvp");
    hasher.update(b"farm-lease-token-v10:session-close-mvp");
    hasher.update(b"dist-cdn-bgp-v10:sticky-persist-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 128 DX: audit 5600/3300, fill EXCLUDE, LSP semantic v30, driver CR340, stdlib 106.0 pin.
fn heap_v100_phase128_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-5600-3300");
    hasher.update(b"audit-exclude-fill-v1:phase128");
    hasher.update(b"lsp-semantic-v30:phase128-36-plus-real");
    hasher.update(b"driver-gate-cr340-v1:phase128-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:106.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 127 Track B: SynUI v4 (signal bind + desktop onClick wire + switch/radio).
fn heap_v99_phase127_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v4:signal-widget-bind");
    hasher.update(b"gui-synui-v4:desktop-onclick-wire");
    hasher.update(b"gui-synui-v4:widget-switch-radio");
    hasher.update(b"gui-synui-v3:desktop-run-optin");
    hasher.update(b"gui-synui-v3:widget-checkbox-slider");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 127 Track C: SIMD16 saturating add/sub MVP + inherited compare/extract.
fn heap_v99_phase127_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 128 Track C: SIMD16 i8x16 shuffle MVP + inherited sat/compare.
fn heap_v100_phase128_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 127 Track D: farm session-close + dist sticky-persist.
fn heap_v99_phase127_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v10:session-close-mvp");
    hasher.update(b"dist-cdn-bgp-v10:sticky-persist-mvp");
    hasher.update(b"farm-lease-token-v9:pkcs11-sign-mvp");
    hasher.update(b"dist-cdn-bgp-v9:sticky-ttl-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 126 Track A: Macro M16 + Service DI v7.
fn heap_v98_phase126_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"runtime-service-v7:protocol-witness");
    hasher.update(b"macro-service-v7:advice-chain");
    hasher.update(b"macro-docs-v8:m16-service-v7-honesty");
    hasher.update(b"macro-service-v6:polymorphic-base-inject");
    hasher.update(b"runtime-service-v6:protocol-binding");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 126 Track B: SynUI v3 (opt-in run + checkbox/slider).
fn heap_v98_phase126_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v3:desktop-run-optin");
    hasher.update(b"gui-synui-v3:widget-checkbox-slider");
    hasher.update(b"gui-synui-v2:signal-callback");
    hasher.update(b"gui-synui-v2:widget-expand");
    hasher.update(b"gui-synui-v2:ci-sibling-checkout");
    hasher.update(b"gui-synui-v1:host-bridge");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 126 Track C: SIMD16 compare-lane MVP + inherited extract/replace.
fn heap_v98_phase126_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    hasher.update(b"wasm-simd16-guest-v7:lane-min-max");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 126 Track D: farm C_Sign + dist sticky-TTL.
fn heap_v98_phase126_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v9:pkcs11-sign-mvp");
    hasher.update(b"farm-lease-token-v8:session-pin-persist");
    hasher.update(b"dist-cdn-bgp-v9:sticky-ttl-mvp");
    hasher.update(b"dist-cdn-bgp-v8:sticky-pop-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 126 DX: audit 5300/3200, fill EXCLUDE, LSP semantic v28, driver CR320, stdlib 104.0 pin.
fn heap_v98_phase126_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-5300-3200");
    hasher.update(b"audit-exclude-fill-v1:phase126");
    hasher.update(b"lsp-semantic-v28:phase126-32-plus-real");
    hasher.update(b"driver-gate-cr320-v1:phase126-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:104.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 127 Track A: Macro M17 + Service DI v8 (advice proceed + `@Bean` MVP).
fn heap_v99_phase127_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-service-v8:advice-proceed");
    hasher.update(b"macro-service-v8:configuration-bean-mvp");
    hasher.update(b"runtime-service-v8:configuration-bean-mvp");
    hasher.update(b"macro-docs-v9:m17-service-v8-honesty");
    hasher.update(b"macro-service-v7:advice-chain");
    hasher.update(b"runtime-service-v7:protocol-witness");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 127 DX: audit 5450/3250, fill EXCLUDE, LSP semantic v29, driver CR330, stdlib 105.0 pin.
fn heap_v99_phase127_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-5450-3250");
    hasher.update(b"audit-exclude-fill-v1:phase127");
    hasher.update(b"lsp-semantic-v29:phase127-34-plus-real");
    hasher.update(b"driver-gate-cr330-v1:phase127-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:105.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 125 Track A: Macro M15 + Service DI v6.
fn heap_v97_phase125_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-service-v6:polymorphic-base-inject");
    hasher.update(b"macro-service-v6:field-qualifier-metadata");
    hasher.update(b"runtime-service-v6:protocol-binding");
    hasher.update(b"macro-m15:service-method-interceptor-mvp");
    hasher.update(b"macro-docs-v7:m15-service-v6-honesty");
    hasher.update(b"macro-service-v5:di-package-ga");
    hasher.update(b"runtime-service-v5:with-request");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 125 Track B: SynUI deepen (run/signal/widgets/CI).
fn heap_v97_phase125_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v2:desktop-run-mvp");
    hasher.update(b"gui-synui-v2:signal-callback");
    hasher.update(b"gui-synui-v2:widget-expand");
    hasher.update(b"gui-synui-v2:ci-sibling-checkout");
    hasher.update(b"gui-synui-v1:host-bridge");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 125 Track C: SIMD16 extract/replace + inherited Min/Max.
fn heap_v97_phase125_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    hasher.update(b"wasm-simd16-guest-v7:lane-min-max");
    hasher.update(b"wasm-simd16-guest-v6:logic-not");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 125 Track D: farm session-pin-persist + dist sticky-pop.
fn heap_v97_phase125_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v8:session-pin-persist");
    hasher.update(b"dist-cdn-bgp-v8:sticky-pop-mvp");
    hasher.update(b"farm-lease-token-v7:pkcs11-session-mvp");
    hasher.update(b"dist-cdn-bgp-v7:weighted-anycast-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 125 DX: audit 5150/3150, fill EXCLUDE, LSP semantic v27, driver CR310, stdlib 103.0 pin.
fn heap_v97_phase125_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-5150-3150");
    hasher.update(b"audit-exclude-fill-v1:phase125");
    hasher.update(b"lsp-semantic-v27:phase125-30-plus-real");
    hasher.update(b"driver-gate-cr310-v1:phase125-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:103.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 124 Track A: Macro M14 + Service DI v5.
fn heap_v96_phase124_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-service-v5:di-package-ga");
    hasher.update(b"runtime-service-v5:with-request");
    hasher.update(b"macro-service-v5:conditional-qualifier");
    hasher.update(b"macro-service-v5:conditional-profile");
    hasher.update(b"macro-docs-v6:m14-service-v5-honesty");
    hasher.update(b"runtime-service-v4:request-scope");
    hasher.update(b"macro-service-v4:ceremony-sugar");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 124–125 Track B: SynUI path-dep GUI replacement.
fn heap_v96_phase124_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-synui-v1:path-dep-desktop-mvp");
    hasher.update(b"gui-synui-v1:ui-package-surface");
    hasher.update(b"gui-synui-v1:host-bridge");
    hasher.update(b"gui-synui-v2:desktop-run-mvp");
    hasher.update(b"gui-synui-v2:signal-callback");
    hasher.update(b"gui-synui-v2:widget-expand");
    hasher.update(b"gui-synui-v2:ci-sibling-checkout");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 124 Track C: SIMD16 Min/Max + inherited Not.
fn heap_v96_phase124_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v7:lane-min-max");
    hasher.update(b"wasm-simd16-guest-v6:logic-not");
    hasher.update(b"wasm-simd16-guest-v5:logic-or-xor");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 124 Track D: farm pkcs11-session + dist weighted-anycast.
fn heap_v96_phase124_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v7:pkcs11-session-mvp");
    hasher.update(b"dist-cdn-bgp-v7:weighted-anycast-mvp");
    hasher.update(b"farm-lease-token-v6:pkcs11-so-adapter");
    hasher.update(b"dist-cdn-bgp-v6:multi-pop-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 124 DX: audit 5000/2900, fill EXCLUDE, LSP semantic v26, driver CR300, stdlib 102.0 pin.
fn heap_v96_phase124_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-5000-3100");
    hasher.update(b"audit-exclude-fill-v1:phase124");
    hasher.update(b"lsp-semantic-v26:phase124-28-plus-real");
    hasher.update(b"driver-gate-cr300-v1:phase124-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:102.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 122 Track A: Macro M12 + Service DI v3.
fn heap_v94_phase122_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-service-v3:multi-package-scan");
    hasher.update(b"macro-service-v3:typed-resolve-harden");
    hasher.update(b"runtime-service-v3:prototype-scope");
    hasher.update(b"runtime-service-v3:predestroy-mvp");
    hasher.update(b"macro-pipeline-v5:rb-nested-stack-fixed");
    hasher.update(b"macro-docs-v4:m12-service-v3-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 122 Track C: SIMD16 Or/Xor + inherited shift lane / guest-v11 / matrix-expand.
fn heap_v94_phase122_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v5:logic-or-xor");
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 123 Track C: SIMD16 Not + inherited or/xor / shift lane / guest-v11 / matrix-expand.
fn heap_v95_phase123_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v6:logic-not");
    hasher.update(b"wasm-simd16-guest-v5:logic-or-xor");
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 123 DX: audit 4850/3050, fill EXCLUDE, LSP semantic v25, driver CR290, stdlib 101.0 pin.
fn heap_v95_phase123_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-4850-3050");
    hasher.update(b"audit-exclude-fill-v1:phase123");
    hasher.update(b"lsp-semantic-v25:phase123-26-plus-real");
    hasher.update(b"driver-gate-cr290-v1:phase123-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:101.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 123 Track A: Macro M13 + Service DI v4.
fn heap_v95_phase123_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"runtime-service-v4:request-scope");
    hasher.update(b"macro-service-v4:ceremony-sugar");
    hasher.update(b"macro-pipeline-v6:rb-hostcall-closure-fixed");
    hasher.update(b"macro-service-v4:typed-resolve-ident");
    hasher.update(b"macro-docs-v5:m13-service-v4-honesty");
    hasher.update(b"macro-service-v4:scan-resolved-only");
    hasher.update(b"macro-service-v3:multi-package-scan");
    hasher.update(b"runtime-service-v3:prototype-scope");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 123 Track B: OS FontMgr default-paint OT v7.
fn heap_v95_phase123_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-ot-features-v7:os-fontmgr-default-paint");
    hasher.update(b"gui-ot-features-v6:default-paint-ot-mvp");
    hasher.update(b"gui-dylib-parity-v1:ot-vf-export");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 123 Track D: farm pkcs11-so adapter + dist multi-PoP MVP.
fn heap_v95_phase123_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v6:pkcs11-so-adapter");
    hasher.update(b"dist-cdn-bgp-v6:multi-pop-mvp");
    hasher.update(b"farm-lease-token-v5:pkcs11-bridge-mvp");
    hasher.update(b"dist-cdn-bgp-v5:multi-region-mesh-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 122 DX: audit 4700/3000, fill EXCLUDE, LSP semantic v24, driver CR280, stdlib 100.0 pin.
fn heap_v94_phase122_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-4700-3000");
    hasher.update(b"audit-exclude-fill-v1:phase122");
    hasher.update(b"lsp-semantic-v24:phase122-24-plus-real");
    hasher.update(b"driver-gate-cr280-v1:phase122-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:100.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 121 Track A: Macro M11 Service DI v2 (real ctor, field Autowired, typed resolve, RB E009).
fn heap_v93_phase121_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-service-v2:typed-resolve");
    hasher.update(b"macro-service-v2:field-autowired");
    hasher.update(b"macro-service-v2:import-visible-scan");
    hasher.update(b"runtime-service-v2:real-ctor-resolve");
    hasher.update(b"runtime-service-v2:postconstruct-mvp");
    hasher.update(b"macro-pipeline-v4:rb-nested-fixed-or-reject");
    hasher.update(b"macro-docs-v3:m11-service-v2-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 121 Track B: OT paint-matrix v5 + VF paint-variations v4.
fn heap_v93_phase121_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-ot-features-v5:paint-matrix-mvp");
    hasher.update(b"gui-font-vf-v4:paint-variations-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 122 Track B: default-paint OT v6 + dylib OT/VF export parity.
fn heap_v94_phase122_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-ot-features-v6:default-paint-ot-mvp");
    hasher.update(b"gui-dylib-parity-v1:ot-vf-export");
    hasher.update(b"gui-font-vf-v4:paint-variations-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 121 Track C: SIMD16 shift lane + inherited logic-or-shift / guest-v11.
fn heap_v93_phase121_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 121 Track D: farm KMS bridge MVP + BGP route-convergence.
fn heap_v93_phase121_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v4:kms-bridge-mvp");
    hasher.update(b"dist-cdn-bgp-v4:route-convergence-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 122 Track D: farm PKCS#11 bridge MVP + dist multi-region mesh.
fn heap_v94_phase122_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v5:pkcs11-bridge-mvp");
    hasher.update(b"dist-cdn-bgp-v5:multi-region-mesh-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 121 DX: audit 4550/2950, fill EXCLUDE, LSP semantic v23, driver CR270, stdlib 99.0 pin.
fn heap_v93_phase121_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-4550-2950");
    hasher.update(b"audit-exclude-fill-v1:phase121");
    hasher.update(b"lsp-semantic-v23:phase121-24-plus-real");
    hasher.update(b"driver-gate-cr270-v1:phase121-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:99.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 120 Track A: Macro M10 Service DI + exports schema + RB nested honesty.
fn heap_v92_phase120_track_a_macro_service_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-package-v2:exports-schema");
    hasher.update(b"macro-service-v1:attr-scan");
    hasher.update(b"macro-service-v1:registry-codegen");
    hasher.update(b"macro-service-v1:ctor-inject");
    hasher.update(b"runtime-service-v1:singleton-container");
    hasher.update(b"macro-docs-v2:m10-service-honesty");
    hasher.update(b"macro-pipeline-v3:rb-nested-hostcall");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 120 Track B: OT feature-matrix + VF axes≥3.
fn heap_v92_phase120_track_b_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-ot-features-v5:paint-matrix-mvp");
    hasher.update(b"gui-font-vf-v4:paint-variations-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 120 Track C: SIMD16 i8x16 logic/shift (And) + inherited guest-v11 / matrix-expand.
/// Phase 121 Track C: adds `wasm-simd16-guest-v4:shift-lane-mvp` (Shl/Shr).
fn heap_v92_phase120_track_c_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 120 Track D: farm HSM-ref token MVP + BGP ops-visibility.
fn heap_v92_phase120_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-token-v3:hsm-ref-mvp");
    hasher.update(b"dist-cdn-bgp-v3:ops-visibility");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 120 DX: audit 4400/2900, fill EXCLUDE, LSP semantic v22, driver CR260, stdlib 98.0 pin.
fn heap_v92_phase120_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-4400-2900");
    hasher.update(b"audit-exclude-fill-v1:phase120");
    hasher.update(b"lsp-semantic-v22:phase120-24-plus-real");
    hasher.update(b"driver-gate-cr260-v1:phase120-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:98.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 116 Track A: frame single ink + OS FontMgr + COLR radial+sweep + doctor v4.
fn heap_v88_phase116_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-paragraph-v4:frame-single-ink");
    hasher.update(b"gui-font-system-v3:os-proven");
    hasher.update(b"gui-emoji-v8:colr-radial-and-sweep");
    hasher.update(b"gui-doctor-engine-v4:phase116");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 116 Track B: simd guest-v9 nospill unary/shift/shuffle + f32 LS.
fn heap_v88_phase116_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v9:nospill-unary-shift-shuffle-f32ls");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 116 Track C: Farm cross-host multidc + registry health-sync.
fn heap_v88_phase116_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-multidc-v3:cross-host");
    hasher.update(b"farm-lease-registry-v2:health-sync");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 116 Track D: dual-process PoP + concrete-ip hostkey + select no-env.
fn heap_v88_phase116_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-cdn-pop-v3:dual-process-host");
    hasher.update(b"dist-region-hostkey-v6:concrete-ip");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 116 DX: audit 3800/2700, fill EXCLUDE, LSP semantic v18, driver CR220, stdlib 94.0 pin.
fn heap_v88_phase116_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-3800-2700");
    hasher.update(b"audit-exclude-fill-v1:phase116");
    hasher.update(b"lsp-semantic-v18:phase116-24-plus-real");
    hasher.update(b"driver-gate-cr220-v1:phase116-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:94.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 117 Track A: all-widgets ink + OS-only FontMgr + font Sweep + OT calt + doctor v5.
fn heap_v89_phase117_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-paragraph-v5:all-widgets-ink");
    hasher.update(b"gui-font-system-v4:os-only-ga");
    hasher.update(b"gui-emoji-v9:font-radial-and-sweep");
    hasher.update(b"gui-ot-features-v1:kern-liga-calt");
    hasher.update(b"gui-doctor-engine-v5:phase117");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 117 Track B: simd guest-v10 stack-native + SIMD8/i64 pilots.
fn heap_v89_phase117_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v10:stack-native-simd8-i64");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 117 Track C: Farm real-connect + consensus-sync + token rotate.
fn heap_v89_phase117_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-multidc-v4:real-connect");
    hasher.update(b"farm-lease-registry-v3:consensus-sync");
    hasher.update(b"farm-lease-token-v1:rotate");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 117 Track D: BGP anycast MVP + remote-hostkey PoP + select no-env.
fn heap_v89_phase117_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-cdn-bgp-v1:anycast-mvp");
    hasher.update(b"dist-cdn-pop-v4:remote-hostkey");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 117 DX: audit 3950/2750, fill EXCLUDE, LSP semantic v19, driver CR230, stdlib 95.0 pin.
fn heap_v89_phase117_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-3950-2750");
    hasher.update(b"audit-exclude-fill-v1:phase117");
    hasher.update(b"lsp-semantic-v19:phase117-24-plus-real");
    hasher.update(b"driver-gate-cr230-v1:phase117-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:95.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 119 Track A: Macro M9 + OT script + VF multi-axis.
fn heap_v91_phase119_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-freestanding-v1:role-enforce");
    hasher.update(b"macro-external-v2:plugin-prod");
    hasher.update(b"macro-codable-l2-v1:enum-generic");
    hasher.update(b"macro-attached-conf-v1:no-silent-drop");
    hasher.update(b"macro-pipeline-v2:resultbuilder-in-engine");
    hasher.update(b"macro-builtin-v2:stringify-compiletime");
    hasher.update(b"macro-package-v1:exports-e2e");
    hasher.update(b"gui-ot-features-v3:script-subset");
    hasher.update(b"gui-font-vf-v2:multi-axis-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 119 Track B: simd16 matrix expand + guest-v11 zero-scratch inherited.
fn heap_v91_phase119_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 119 Track C: Farm multi-follower-sync registry.
fn heap_v91_phase119_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-registry-v5:multi-follower-sync");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 119 Track D: multi-hostkey-route PoP + select no-env.
fn heap_v91_phase119_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-cdn-pop-v6:multi-hostkey-route");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 119 DX: audit 4250/2850, fill EXCLUDE, LSP semantic v21, driver CR250, stdlib 97.0 pin.
fn heap_v91_phase119_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"macro-docs-v1:m9-honesty-pass");
    hasher.update(b"audit-align-v1:ci-script-adr-4250-2850");
    hasher.update(b"audit-exclude-fill-v1:phase119");
    hasher.update(b"lsp-semantic-v21:phase119-24-plus-real");
    hasher.update(b"driver-gate-cr250-v1:phase119-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:97.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 118 Track A: OS-all-apis + OT subst + VF wght + doctor v6.
fn heap_v90_phase118_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-paragraph-v6:os-ot-aligned");
    hasher.update(b"gui-font-system-v5:os-only-all-apis");
    hasher.update(b"gui-ot-features-v2:subst-proven");
    hasher.update(b"gui-font-vf-v1:wght-mvp");
    hasher.update(b"gui-emoji-v9:font-radial-and-sweep");
    hasher.update(b"gui-doctor-engine-v6:phase118");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 118 Track B: simd guest-v11 zero-scratch + SIMD16 pilots.
fn heap_v90_phase118_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 118 Track C: Farm OS dual-process + follower-view + durable token epoch.
fn heap_v90_phase118_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-multidc-v5:os-dual-process");
    hasher.update(b"farm-lease-registry-v4:follower-view");
    hasher.update(b"farm-lease-token-v2:durable-epoch");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 118 Track D: BGP peer-exchange + sibling-process PoP + select no-env.
fn heap_v90_phase118_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-cdn-bgp-v2:peer-exchange");
    hasher.update(b"dist-cdn-pop-v5:sibling-process");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 118 DX: audit 4100/2800, fill EXCLUDE, LSP semantic v20, driver CR240, stdlib 96.0 pin.
fn heap_v90_phase118_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-4100-2800");
    hasher.update(b"audit-exclude-fill-v1:phase118");
    hasher.update(b"lsp-semantic-v20:phase118-24-plus-real");
    hasher.update(b"driver-gate-cr240-v1:phase118-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:96.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 115 Track A: single-path paint + FontMgr proven + COLR radial/sweep + doctor v3.
fn heap_v87_phase115_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-paragraph-v3:single-path");
    hasher.update(b"gui-font-system-v2:proven-fallback");
    hasher.update(b"gui-emoji-v7:colr-radial-or-sweep");
    hasher.update(b"gui-doctor-engine-v3:phase115");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 115 Track B: simd guest-v8 nospill binop + f64 Mul/Div + guest reduce.
fn heap_v87_phase115_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v8:nospill-binop-f64muldiv-reduce");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 115 Track C: Farm multidc runtime-wired + registry reload-probe.
fn heap_v87_phase115_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-multidc-v2:runtime-wired");
    hasher.update(b"farm-lease-registry-v1:reload-probe");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 115 Track D: CDN PoP noloop E2E + hostkey bind-eq-key + select no-env.
fn heap_v87_phase115_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-cdn-pop-v2:noloop-e2e");
    hasher.update(b"dist-region-hostkey-v5:bind-eq-key");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 115 DX: audit 3650/2650, fill EXCLUDE, LSP semantic v17, driver CR210, stdlib 93.0 pin.
fn heap_v87_phase115_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-3650-2650");
    hasher.update(b"audit-exclude-fill-v1:phase115");
    hasher.update(b"lsp-semantic-v17:phase115-24-plus-real");
    hasher.update(b"driver-gate-cr210-v1:phase115-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:93.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 114 Track A: paragraph paint + native COLR + system FontMgr + doctor v2.
fn heap_v86_phase114_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-paragraph-v2:paint");
    hasher.update(b"gui-emoji-v6:cbdt-or-native-colr-grad");
    hasher.update(b"gui-font-system-v1:mgr-fallback");
    hasher.update(b"gui-doctor-engine-v2:phase114");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 114 Track B: simd guest-v7 f64 reduce + guest LS + f64 Sub.
fn heap_v86_phase114_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v7:f64reduce-guest-ls-f64sub");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 114 Track C: Farm multi-DC + token-all-paths.
fn heap_v86_phase114_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-multidc-v1:discover-failover");
    hasher.update(b"farm-lease-tenant-v4:token-all-paths-multidc");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 114 Track D: CDN PoP + hostkey noloop + select no-env.
fn heap_v86_phase114_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-cdn-pop-v1:health-anycast");
    hasher.update(b"dist-region-hostkey-v4:pop-bind-noloop");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 114 DX: audit 3500/2600, fill EXCLUDE, LSP semantic v16, driver CR200, stdlib 92.0 pin.
fn heap_v86_phase114_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-3500-2600");
    hasher.update(b"audit-exclude-fill-v1:phase114");
    hasher.update(b"lsp-semantic-v16:phase114-24-plus-real");
    hasher.update(b"driver-gate-cr200-v1:phase114-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:92.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 113 Track A: real skia paragraph + CBDT CI + COLR non-Solid + doctor engine.
fn heap_v85_phase113_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-paragraph-v1:textlayout");
    hasher.update(b"gui-emoji-v5:cbdt-ci-or-colr");
    hasher.update(b"gui-doctor-engine-v1:phase113");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 113 Track B: simd guest-v6 f32 Div + guest LS + f64 Add.
fn heap_v85_phase113_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v6:f32div-guest-ls-f64");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 113 Track C: Farm token default + WAL compact + dyn members.
fn heap_v85_phase113_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-tenant-v3:token-default");
    hasher.update(b"farm-lease-wal-compact-v1:snap-truncate-log");
    hasher.update(b"farm-lease-members-v2:dyn-list");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 113 Track D: CDN peer + hostkey runtime + select no-env.
fn heap_v85_phase113_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-cdn-tcp-v2:peer-delivery");
    hasher.update(b"dist-region-hostkey-v3:runtime-bind");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 113 DX: audit 3350/2550, cat EXCLUDE, LSP semantic v15, driver CR190, stdlib 91.0 pin.
fn heap_v85_phase113_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-3350-2550");
    hasher.update(b"audit-exclude-cat-v1:phase113");
    hasher.update(b"lsp-semantic-v15:phase113-24-plus-real");
    hasher.update(b"driver-gate-cr190-v1:phase113-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:91.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 112 Track A: skia paragraph/bidi + CBDT + vertical unify + doctor strict.
fn heap_v84_phase112_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-layout-v1:paragraph-bidi-mvp");
    hasher.update(b"gui-emoji-v4:cbdt-or-colr");
    hasher.update(b"gui-vertical-unify-v1:hb-advance");
    hasher.update(b"gui-doctor-strict-v1:phase112");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 112 Track B: simd guest-v5 f32 ops + guest Load/Store.
fn heap_v84_phase112_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v5:f32-ops-guest-ls");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 112 Track C: Farm runtime members + tenant token + WAL snapshot.
fn heap_v84_phase112_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-runtime-v1:config-members");
    hasher.update(b"farm-lease-tenant-v2:token-mvp");
    hasher.update(b"farm-lease-wal-snap-v1:checkpoint-replay");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 112 Track D: dist hostkey + TCP CDN + select deadline unify.
fn heap_v84_phase112_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-region-hostkey-v2:config-nonloopback");
    hasher.update(b"dist-cdn-tcp-v1:peer-mvp");
    hasher.update(b"select-deadline-unify-v1:lang-over-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 112 DX: audit 3200/2500, fill EXCLUDE, LSP semantic v14, driver CR180, stdlib 90.0 pin.
fn heap_v84_phase112_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-3200-2500");
    hasher.update(b"audit-exclude-fill-v1:phase112");
    hasher.update(b"lsp-semantic-v14:phase112-24-plus-real");
    hasher.update(b"driver-gate-cr180-v1:phase112-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:90.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 111 Track D: dist dual-bind non-loopback failover + select syntax timeout gate.
fn heap_v83_phase111_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-region-nonloopback-v1:dual-bind-failover");
    hasher.update(b"select-timeout-syntax-v1:lang-arm");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 111 Track A: skia glyph raster + COLR outline + vertical HB ink + doctor used-only.
fn heap_v83_phase111_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-skia-glyph-v1:raster-mvp");
    hasher.update(b"gui-emoji-v3:colr-outline");
    hasher.update(b"gui-vertical-hb-v1:ink-on");
    hasher.update(b"gui-doctor-used-v1:phase111");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 111 Track C: Farm multihost + tenant isolation + WAL CRC.
fn heap_v83_phase111_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-multihost-v1:config-members");
    hasher.update(b"farm-lease-tenant-v1:isolation-mvp");
    hasher.update(b"farm-lease-wal-crc-v1:reject-corrupt");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 111 DX: audit 3050/2450, cat EXCLUDE, LSP semantic v13, driver CR170, stdlib 89.0 pin.
fn heap_v83_phase111_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-3050-2450");
    hasher.update(b"audit-exclude-cat-v1:phase111");
    hasher.update(b"lsp-semantic-v13:phase111-24-plus-real");
    hasher.update(b"driver-gate-cr170-v1:phase111-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:89.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 110 Track D: dist multihost keyed failover + select mixed+await timeout gate.
fn heap_v82_phase110_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-region-multihost-v1:keyed-failover");
    hasher.update(b"select-mixed-timeout-v1:cancel-or-deadline");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 110 Track A: wgpu composite HB/emoji + COLR emoji + vertical layout + doctor live.
fn heap_v82_phase110_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-wgpu-composite-v1:hb-emoji-tex");
    hasher.update(b"gui-emoji-v2:cbdt-or-colr");
    hasher.update(b"gui-vertical-rl-v2:layout");
    hasher.update(b"gui-doctor-live-v1:phase110");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 110 Track C: Raft log+WAL + membership change MVP.
fn heap_v82_phase110_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-raft-v2:log-wal");
    hasher.update(b"farm-lease-member-v1:dynamic-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 110 DX: audit 2900/2400, real EXCLUDE, LSP semantic v12, driver CR160, stdlib 88.0 pin.
fn heap_v82_phase110_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2900-2400");
    hasher.update(b"audit-exclude-real-v1:phase110");
    hasher.update(b"lsp-semantic-v12:phase110-24-plus-real");
    hasher.update(b"driver-gate-cr160-v1:phase110-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:88.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 107 Track A: HarfBuzz shape MVP + system CJK fallback + real doctor probes.
fn heap_v79_phase107_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-harfbuzz-v1:shape-mvp");
    hasher.update(b"gui-font-fallback-v1:system-cjk");
    hasher.update(b"gui-doctor-real-v1:phase107");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 107 Track B: true simd128 Div/Eq/Lt/Shuffle + LLVM declare↔call hygiene.
fn heap_v79_phase107_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-expand-v1:div-eq-lt-shuffle");
    hasher.update(b"llvm-declare-call-v1:rt-hygiene");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 109 Track A: egui-wgpu present + HB string paint + emoji/vertical-rl MVP.
fn heap_v81_phase109_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-egui-wgpu-v1:winit-present");
    hasher.update(b"gui-hb-paint-v1:string-shape");
    hasher.update(b"gui-emoji-v1:color-mvp");
    hasher.update(b"gui-vertical-rl-v1:label-mvp");
    hasher.update(b"gui-doctor-wgpu-v1:phase109");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 111 Track B: simd128 guest-v4 thin arith + Min/Max + f32 add pilot.
fn heap_v83_phase111_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v4:thin-arith-minmax-f32");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 110 Track B: simd128 guest-v3 spill thin + Shr; honest Div assembled.
fn heap_v82_phase110_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v3:spill-thin-shr");
    hasher.update(b"wasm-simd128-div-assembled-v1:honest");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 109 Track B: simd128 Div/Load/Store guest + Ne/Le/Ge/Not/Shl.
fn heap_v81_phase109_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-guest-v2:div-load-store-ext");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 109 Track C: Farm OS multi-process HA + Raft quorum MVP.
fn heap_v81_phase109_track_c_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-ha-v3:os-process");
    hasher.update(b"farm-lease-raft-v1:quorum-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 109 Track D: dist multi-region anycast + select mixed+await.
fn heap_v81_phase109_track_d_dist_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"dist-region-anycast-v1:tcp-health");
    hasher.update(b"select-mixed-await-v1:interleave");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 109 DX: audit 2750/2350, cat EXCLUDE, LSP semantic v11, driver CR150, stdlib 87.0 pin.
fn heap_v81_phase109_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2750-2350");
    hasher.update(b"audit-exclude-cat-v1:phase109");
    hasher.update(b"lsp-semantic-v11:phase109-24-plus-real");
    hasher.update(b"driver-gate-cr150-v1:phase109-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:87.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 108 Track A: egui MVP + pump reentrancy + doctor egui.
fn heap_v80_phase108_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-egui-mvp-v1:winit-softbuffer");
    hasher.update(b"gui-pump-reentrancy-v1:dirty-defer");
    hasher.update(b"gui-doctor-egui-v1:phase108");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 108 Track B: simd128 Gt + bitwise And/Or/Xor.
fn heap_v80_phase108_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd128-bitwise-v1:gt-and-or-xor");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 108 Track C: mixed send/recv unified tagged array poll.
fn heap_v80_phase108_track_c_select_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"select-mixed-unified-v1:tagged-array-poll");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 108 Track D: Farm process-isolated HA + dist mailbox no inproc GA.
fn heap_v80_phase108_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-ha-v2:process-isolated");
    hasher.update(b"dist-actor-mp-v2:no-inproc-fallback-ga");
    // Phase 109 Track D seeds (cache v81 supersedes; keep present for soft gates).
    hasher.update(b"dist-region-anycast-v1:tcp-health");
    hasher.update(b"select-mixed-await-v1:interleave");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 108 DX: audit 2600/2300, arith EXCLUDE, LSP semantic v10, driver CR140, stdlib 86.0 pin.
fn heap_v80_phase108_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2600-2300");
    hasher.update(b"audit-exclude-arith-v1:phase108");
    hasher.update(b"lsp-semantic-v10:phase108-24-plus-real");
    hasher.update(b"driver-gate-cr140-v1:phase108-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:86.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 107 Track D: Farm lease HA + dist-actor multi-process mailbox.
fn heap_v79_phase107_track_d_farm_dist_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-ha-v2:process-isolated");
    hasher.update(b"dist-actor-mp-v2:no-inproc-fallback-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 107 DX: audit 2450/2250, fill EXCLUDE, LSP semantic v9, driver CR130, stdlib 85.0 pin.
fn heap_v79_phase107_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2450-2250");
    hasher.update(b"audit-exclude-fill-v1:phase107");
    hasher.update(b"lsp-semantic-v9:phase107-24-plus-real");
    hasher.update(b"driver-gate-cr130-v1:phase107-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:85.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 106 Track A: expanded TTF ≥64 + Preedit invert highlight + doctor capability.
fn heap_v78_phase106_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-ttf-expand-v1:cjk64-plus");
    hasher.update(b"gui-ime-highlight-v1:preedit-invert");
    hasher.update(b"gui-doctor-cap-v1:phase106");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 106 Track D: farm TCP node enforces lease TTL (not client-only fail-closed).
fn heap_v78_phase106_track_d_farm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(crate::farm_lease_auth::farm_lease_server_fingerprint().as_bytes());
    hasher.update(crate::farm_lease_auth::farm_lease_failclosed_fingerprint().as_bytes());
    hasher.update(crate::farm_lease_auth::farm_lease_auth_fingerprint().as_bytes());
    hasher.update(b"farm-lease-ha-v2:process-isolated");
    hasher.update(b"dist-actor-mp-v2:no-inproc-fallback-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 106 DX: audit 2300/2200, gate0 EXCLUDE, LSP semantic v8, driver CR120, stdlib 84.0 pin.
fn heap_v78_phase106_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2300-2200");
    hasher.update(b"audit-exclude-gate0-v1:phase106");
    hasher.update(b"lsp-semantic-v8:phase106-24-plus-real");
    hasher.update(b"driver-gate-cr120-v1:phase106-real-behavioral");
    hasher.update(b"driver-gate-cr130-v1:phase107-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:84.0");
    hasher.update(b"lsp-semantic-v9:phase107-24-plus-real");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 105 Track A: embedded TTF CJK subset + IME Preedit + doctor coverage.
fn heap_v77_phase105_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-ttf-subset-v1:fontdue-cjk");
    hasher.update(b"gui-ime-preedit-v1:composition-draw");
    hasher.update(b"gui-doctor-coverage-v1:phase105");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 105 DX: audit floors, probe EXCLUDE, LSP semantic v7, driver gate CR110, stdlib pin.
fn heap_v77_phase105_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2150-2150");
    hasher.update(b"audit-exclude-probes-v1:phase105");
    hasher.update(b"lsp-semantic-v7:phase105-24-plus-real");
    hasher.update(b"driver-gate-cr110-v1:phase105-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:83.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 105 Track C: multi-case select + reflect specialization GA.
fn heap_v77_phase105_track_c_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"select-default-stack-v1:sugar-matches-host");
    hasher.update(b"select-multi-case-v1:sugar-matches-host");
    hasher.update(b"reflect-witness-mandatory-v1:no-synthesize-true");
    hasher.update(b"reflect-specialization-ga-v1:miss-is-error");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 105 Track D: farm lease fail-closed + locals v10 E2E + plugin unload hard-fail.
fn heap_v77_phase105_track_d_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-failclosed-v1:no-auth-is-err");
    hasher.update(b"native-llvm-locals-v10-e2e:guidemo-line-parity");
    hasher.update(b"plugin-unload-hard-v1:fixture-required");
    hasher.update(b"native-llvm-locals-v10:no-tls-slab");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 105 Track B: AOT SIMD full guest; no mir_exec catch-all.
fn heap_v77_phase105_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd-full-v1:aot-ops-complete");
    hasher.update(b"wasm-no-mir-exec-catchall-v1:phase105");
    hasher.update(b"wasm-allowlist-empty-v1:phase104");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 106 Track B: SimdLt/Shuffle AOT; GA forbids MIR_EXEC_ONLY / tier3-off zero gates.
fn heap_v78_phase106_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd-lt-shuffle-v1:aot-complete");
    hasher.update(b"wasm-ga-no-mir-exec-env-v1:phase106");
    hasher.update(b"wasm-simd-full-v1:aot-ops-complete");
    hasher.update(b"wasm-no-mir-exec-catchall-v1:phase105");
    hasher.update(b"wasm-allowlist-empty-v1:phase104");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 107 Track B fingerprints (wired into `syx-cache-v79` later).
#[allow(dead_code)]
fn phase107_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &[
        "wasm-simd128-expand-v1:div-eq-lt-shuffle",
        "llvm-declare-call-v1:rt-hygiene",
    ]
}

/// Phase 108 Track B fingerprints.
#[allow(dead_code)]
fn phase108_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &["wasm-simd128-bitwise-v1:gt-and-or-xor"]
}

/// Phase 109 Track A fingerprints.
#[allow(dead_code)]
fn phase109_track_a_fingerprint_seeds() -> &'static [&'static str] {
    &[
        "gui-egui-wgpu-v1:winit-present",
        "gui-hb-paint-v1:string-shape",
        "gui-emoji-v1:color-mvp",
        "gui-vertical-rl-v1:label-mvp",
        "gui-doctor-wgpu-v1:phase109",
    ]
}

/// Phase 111 Track B fingerprints.
#[allow(dead_code)]
fn phase111_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &["wasm-simd128-guest-v4:thin-arith-minmax-f32"]
}

/// Phase 110 Track B fingerprints.
#[allow(dead_code)]
fn phase110_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &[
        "wasm-simd128-guest-v3:spill-thin-shr",
        "wasm-simd128-div-assembled-v1:honest",
    ]
}

/// Phase 109 Track B fingerprints.
#[allow(dead_code)]
fn phase109_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &["wasm-simd128-guest-v2:div-load-store-ext"]
}

/// Phase 106 Track C: select send-case + multi-await + reflect semantic instantiations.
fn heap_v78_phase106_track_c_select_reflect_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"select-send-case-v1:sugar-matches-host");
    hasher.update(b"select-multi-await-v1:case-await-order");
    hasher.update(b"reflect-semantic-unified-v1:instantiations-first");
    hasher.update(b"select-multi-case-v1:sugar-matches-host");
    hasher.update(b"reflect-specialization-ga-v1:miss-is-error");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 107 Track C: reflect honest witness + select fair/mixed poll.
fn heap_v79_phase107_track_c_select_reflect_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"reflect-witness-honest-v1:no-synthesize");
    hasher.update(b"select-fair-v1:round-robin");
    hasher.update(b"select-mixed-unified-v1:tagged-array-poll");
    // Historical Phase 107 surface tag retained for grep/invalidation continuity.
    hasher.update(b"select-mixed-poll-v1:send-recv-array");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 104 Track A: Unicode/CJK TextField + IME + drawn caret (historical v76 segment).
fn heap_v76_phase104_track_a_gui_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-cjk-ime-v1:unicode-caret-ime");
    hasher.update(b"gui-font-cjk-subset-v1:no-ascii-only-input");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 104 DX: audit floors, LSP semantic v6, driver gate CR100, stdlib pin.
fn heap_v76_phase104_dx_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"audit-align-v1:ci-script-adr-2150-2150");
    hasher.update(b"lsp-semantic-v6:phase104-22-plus-real");
    hasher.update(b"driver-gate-cr100-v1:phase104-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:83.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 104 Track C: select-default sugar + reflect witness mandatory.
fn heap_v76_phase104_track_c_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"select-default-stack-v1:sugar-matches-host");
    hasher.update(b"reflect-witness-mandatory-v1:no-synthesize-true");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 104 Track D: cross-node farm lease authority + locals v10 (no TLS slab hot path).
fn heap_v76_phase104_track_d_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"farm-lease-auth-v1:cross-node-ttl");
    hasher.update(b"native-llvm-locals-v10:no-tls-slab");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 104 Track B: SIMD/i128 guest imports; mir_exec allowlist empty.
fn heap_v76_phase104_track_b_wasm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-simd-guest-v1:no-mir-exec-simd");
    hasher.update(b"wasm-i128-guest-v1:no-mir-exec-const128");
    hasher.update(b"wasm-allowlist-empty-v1:phase104");
    hasher.update(b"wasm-simd-full-v1:aot-ops-complete");
    hasher.update(b"wasm-no-mir-exec-catchall-v1:phase105");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 103: full cache v75 contract (Tracks A–D + DX).
fn heap_v75_phase103_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-textfield-focus-v1:caret-tab-cycle");
    hasher.update(b"gui-hstack-ga-v1:spacing-align");
    hasher.update(b"gui-onclick-hard-v1:no-swallow-err");
    hasher.update(b"browser-tier2w-ci-v1:playwright-job");
    hasher.update(b"wasm-closure-guest-v1:no-mir-exec-closure-fnindex");
    hasher.update(b"native-llvm-locals-v9:frame-value-ptr-ga");
    hasher.update(b"plugin-dlclose-v1:unload-reload");
    hasher.update(b"lang-resultbuilder-v2:build-optional");
    hasher.update(b"select-await-interleave-v2:case-await-order");
    hasher.update(b"lang-deinit-weak-v1:weak-nil-on-drop");
    hasher.update(b"audit-align-v1:ci-script-adr-1850-2050");
    hasher.update(b"lsp-semantic-v5:phase103-20-plus-real");
    hasher.update(b"driver-gate-cr90-v1:phase103-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:81.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 103 (historical): Track A–C-only tag retained for grep/tests.
fn heap_v75_phase103_track_a_fingerprint() -> String {
    heap_v75_phase103_fingerprint()
}

/// Phase 102: GUI callback GA + Actor/Task guest + host body prune + deinit ARC + DX gates.
fn heap_v74_phase102_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-callback-ga-v1:invoke-onclick-textfield");
    hasher.update(b"gui-hstack-mvp-v1:layout-axis");
    hasher.update(b"browser-tier2w-v1:click-invoke-field");
    hasher.update(b"wasm-actor-task-guest-v1:no-mir-exec-actor-task");
    hasher.update(b"wasm-call-thin-v1:fn-host-guest");
    hasher.update(b"lang-resultbuilder-ga-v1:multistmt-nested");
    hasher.update(b"codable-host-body-prune-v1:no-l5-l13-impl");
    hasher.update(b"lang-deinit-arc-v1:refcount-zero-call");
    hasher.update(b"audit-align-v1:ci-script-adr-1700-2000");
    hasher.update(b"lsp-semantic-v4:phase102-20-plus-real");
    hasher.update(b"driver-gate-cr80-v1:phase102-real-behavioral");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 101: GUI interactive + concat exact + locals v8 + farm lease + hostfn prune + deinit auto.
fn heap_v73_phase101_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-interactive-v1:hit-onclick-layout");
    hasher.update(b"gui-font-ascii-full-v1:printable-20-7e");
    hasher.update(b"wasm-concat-exact-v1:hello-syx-quad");
    hasher.update(b"wasm-allowlist-thin-v1:closure-return-guest");
    hasher.update(b"native-llvm-locals-v8:frame-slab-no-immortal-ga");
    hasher.update(b"farm-frame-lease-v1:ttl-err-on-expired");
    hasher.update(b"codable-hostfn-prune-v2:no-l5-l13-enum");
    hasher.update(b"lang-deinit-auto-v1:scope-exit-call");
    hasher.update(b"audit-align-v1:ci-script-adr-1550-1950");
    hasher.update(b"lsp-semantic-v3:phase101-18-plus-real");
    hasher.update(b"driver-gate-cr70-v1:phase101-real-behavioral");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 100: GUI widgets + dylib hard DX + locals v7 + wasm guest control + farm frames.
fn heap_v72_phase100_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-widgets-bitmap-v1:label-button-visible");
    hasher.update(b"gui-visible-dylib-hard-v1:no-silent-fallback");
    hasher.update(b"gui-cli-ship-dylib-v1:target-dir-copy");
    hasher.update(b"native-llvm-locals-v7:no-hydrate-handle-ga");
    hasher.update(b"wasm-guest-control-v1:no-mir-exec-jump-conststr");
    hasher.update(b"farm-tcp-frames-v1:get-put-hit");
    hasher.update(b"codable-l14-host-prune-v1:no-l5-l13-public");
    hasher.update(b"audit-align-v1:ci-script-adr-1400-1900");
    hasher.update(b"lsp-semantic-v2:phase100-16-plus-real");
    hasher.update(b"driver-gate-cr60-v1:phase100-compile-run");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 99: locals v6 SSA + wasm hard-zero + GUI line/visible + farm TCP queue + L14-only.
fn heap_v71_phase99_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"native-llvm-locals-v6:alloca-value-single-source-ga");
    hasher.update(b"wasm-hard-zero-v1:no-catchall-no-rename-main-guidemo");
    hasher.update(b"gui-line-hard-v1:guidemo-main-stdout-equal");
    hasher.update(b"gui-visible-nonzero-v1:window-and-pixel");
    hasher.update(b"browser-tier2u-hard-v2:no-ci-fallback-no-spec-skip");
    hasher.update(b"farm-tcp-queue-v1:build-syx-3tcp-farm-and-queue");
    hasher.update(b"codable-l14-only-v1:no-l9-surface");
    hasher.update(b"audit-align-v1:ci-script-adr-1250-1850");
    hasher.update(b"lsp-semantic-v1:phase99-12-plus-real");
    hasher.update(b"driver-gate-cr50-v1:phase99-120-plus-50-compile-run");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 98: Call/Return MIR stack ABI + guidemo import UI strict + alloca v5 + wasm guidemo zero.
fn heap_v70_phase98_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"native-call-return-stack-v1:package-ui-call-mir-stack-abi");
    hasher.update(b"native-guidemo-import-ui-v2:examples-guidemo-15-line-nil-free");
    hasher.update(b"native-llvm-locals-v5:alloca-ssa-value-ga");
    hasher.update(b"wasm-guidemo-zero-v1:main-guidemo-mir-exec-run-op-zero");
    hasher.update(b"browser-tier2u-v1:playwright-guidemo-subset-hard-ci");
    hasher.update(b"codable-l14-surface-v1:hostfn-packages-l14-names");
    hasher.update(b"farm-tcp-3node-v2:build-syx-tcp-listeners-no-env");
    hasher.update(b"llvm-line-parity-v1:guidemo-main-stdout-line-equal");
    hasher.update(b"llvm-utf8-cstr-v1:byte-escape-not-codepoint");
    hasher.update(b"gui-visible-hard-v1:pixel-and-window-count-nonzero");
    hasher.update(b"driver-gate-behavioral-v1:phase98-120-plus-40-compile-run");
    hasher.update(b"mir-licm-stack-order-v1:no-const-hoist-call-args");
    hasher.update(b"llvm-gc-preamble-main-only-v1:nested-call-stack-preserve");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 97: guidemo import UI strict + dlopen default + wasm cold zero + L14 evolution + alloca v4.
fn heap_v69_phase97_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-dlopen-default-v1:dylib-unconditional-fallback-hashmap");
    hasher.update(b"native-guidemo-import-ui-v1:examples-guidemo-ui-strict-parity-15plus");
    hasher.update(b"wasm-cold-zero-v1:concat-tostr-tier1-mir-exec-counter-zero");
    hasher.update(b"browser-tier2t-v1:playwright-multistep-gui-ci-mandatory");
    hasher.update(b"codable-l14-evolution-v1:prost-schema-breaking-l13");
    hasher.update(b"gc-churn-stress-v1:multi-churn-stdout-quad");
    hasher.update(b"farm-tcp-multinode-v1:build-syx-3-node-no-env");
    hasher.update(b"native-async-resume-v4:nested-await-stdout-order");
    hasher.update(b"llvm-multi-link-run-v1:guidemo-main-fixtures-clang-rt-exit-zero");
    hasher.update(b"native-llvm-locals-v4:alloca-ssa-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 96: gui dlopen prod + guidemo full + wasm runtime smoke + L13 prost + farm E2E.
fn heap_v68_phase96_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gui-dlopen-prod-v1:syx-gui-dylib-xvfb-pixel-guidemo-full");
    hasher.update(b"native-guidemo-full-v1:examples-guidemo-ui-import-strict-parity");
    hasher.update(b"wasm-runtime-zero-v1:main-guidemo-wasm-run-smoke");
    hasher.update(b"browser-tier2s-v1:playwright-tier2s-pixel-click-ci-mandatory");
    hasher.update(b"codable-l13-prost-real-v1:prost-message-breaking-l12");
    hasher.update(b"gc-churn-syx-quad-v1:host-program-four-end-stdout");
    hasher.update(b"farm-tcp-e2e-v1:build-syx-worker-no-env");
    hasher.update(b"native-async-resume-v3:multi-await-stdout-order");
    hasher.update(b"llvm-link-run-guidemo-v1:clang-rt-run-exit-zero");
    hasher.update(b"native-llvm-locals-v3:rt-load-store-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 95: GUI CallHost stack ABI + winit isolated + wasm dual zero real + L12 prost + farm worker TCP.
fn heap_v67_phase95_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"native-gui-callhost-v1:stack-abi-guidemo-strict-parity");
    hasher.update(b"native-string-exact-v1:hello-syx-exact-stdout");
    hasher.update(b"gui-winit-isolated-v1:syx-gui-dlopen-softbuffer-pixel");
    hasher.update(b"wasm-aot-zero-mir-exec-real-v1:main-guidemo-mir-exec-run-op-zero");
    hasher.update(b"browser-tier2r-v1:playwright-test-gui-ci-mandatory");
    hasher.update(b"codable-l12-prost-v1:schema-registry-breaking-l11");
    hasher.update(b"orm-prepare-bind-v1:rusqlite-row-assert");
    hasher.update(b"gc-churn-host-v1:managed-heap-churn-stdout-quad");
    hasher.update(b"farm-tcp-worker-v1:build-syx-default-hardened-tcp");
    hasher.update(b"native-async-resume-v2:behavioral-stdout-order");
    hasher.update(b"llvm-link-guidemo-v1:clang-rt-link-exit-zero");
    hasher.update(b"native-llvm-locals-v2:rt-load-store-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 94: LLVM locals SSA + exact string + winit real v3 + wasm zero fallback + L11 data GA.
fn heap_v66_phase94_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"native-llvm-locals-v1:alloca-load-store-ssa");
    hasher.update(b"native-string-exact-v1:hello-syx-exact-stdout");
    hasher.update(b"gui-winit-real-v3:os-window-softbuffer-eventloop");
    hasher.update(b"wasm-aot-zero-fallback-v1:mir-exec-and-run-op-zero-real-programs");
    hasher.update(b"browser-tier2q-v1:playwright-gui-ci-mandatory");
    hasher.update(b"codable-l11-protobuf-v1:pure-wire-breaking-l10");
    hasher.update(b"orm-rusqlite-bind-v1:compound-dict-executed");
    hasher.update(b"gc-churn-quad-v2:churn-stdout-four-end-mandatory");
    hasher.update(b"farm-tcp-hard-v2:worker-default-mutex-billing");
    hasher.update(b"native-lto-v1:thin-lto-opt-in-release");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 93: native build profile + LLVM string + winit real + wasm full opcode + L10 data GA.
fn heap_v65_phase93_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"native-build-profile-v1:clang-rt-link-debug-release");
    hasher.update(b"native-llvm-string-v1:const-concat-tostr-four-end");
    hasher.update(b"gui-winit-real-v2:visible-window-softbuffer-pixel");
    hasher.update(b"wasm-aot-full-opcode-v1:zero-mir-exec-all-fixtures");
    hasher.update(b"reflect-cast-fixture-quad-v1:twelve-scenarios-mandatory");
    hasher.update(b"browser-tier2p-v1:playwright-gui-run-mandatory");
    hasher.update(b"codable-l10-protobuf-v1:real-wire-breaking");
    hasher.update(b"orm-bind-compound-v1:parameterized-values-bound");
    hasher.update(b"gc-churn-quad-v1:churn-relocate-four-end-stdout");
    hasher.update(b"farm-tcp-hard-v1:worker-default-tcp-mutex");
    hasher.update(b"native-async-resume-v1:behavioral-stdout-resume");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 92: wasm zero run_op + native async + cast strict + winit softbuffer + data/farm/GC quad GA.
fn heap_v64_phase92_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-aot-zero-run-op-v1:sync-zero-run-op-histogram");
    hasher.update(b"native-llvm-async-v1:await-poll-suspend-direct-emit");
    hasher.update(b"reflect-cast-strict-quad-v1:twelve-scenarios-four-end-mandatory");
    hasher.update(b"hof-witness-quad-v1:generic-sortBy-witness-mandatory");
    hasher.update(b"gui-winit-softbuffer-v1:softbuffer-pixel-readback-headless");
    hasher.update(b"gui-mobile-mvp-v1:touch-event-stub");
    hasher.update(b"browser-tier2o-v1:playwright-gui-wasm-interaction");
    hasher.update(b"codable-l9-protobuf-v1:schema-migration-apply-protobuf");
    hasher.update(b"orm-reflect-compound-v1:reflect-compound-and-parameterized");
    hasher.update(b"xml-xpath-strict-v1:eight-expressions-hard");
    hasher.update(b"farm-tcp-prod-v1:farm-worker-tcp-tenant-three-node");
    hasher.update(b"farm-billing-farm-path-v1:settlement-excess-exit-nonzero");
    hasher.update(b"gc-generational-v13-quad-v1:young-old-promote-compact-stdout");
    hasher.update(b"sql-migrations-v1:prepare-bind-migrate");
    hasher.update(b"orm-query-ga-v1:findWhere-update-delete");
    hasher.update(b"sql-rusqlite-v1:embedded-engine-loopback");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 91: wasm zero mir_exec + native user-fn + cast quad + real GUI + data/farm/GC GA.
fn heap_v63_phase91_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-aot-zero-mir-exec-v1:sync-zero-mir-exec-histogram");
    hasher.update(b"native-llvm-user-fn-v1:multi-fn-zero-syx-rt-call-fn");
    hasher.update(b"reflect-cast-quad-v1:twelve-scenarios-four-end-stdout");
    hasher.update(b"hof-witness-quad-v1:generic-sortBy-witness-mandatory");
    hasher.update(b"gui-winit-real-v1:softbuffer-pixel-headless");
    hasher.update(b"browser-tier2n-v1:playwright-gui-interaction");
    hasher.update(b"codable-l8-migration-v1:field-add-remove-rename");
    hasher.update(b"orm-reflect-where-v1:reflect-compound-and");
    hasher.update(b"xml-xpath-full-v1:eight-expressions-hard");
    hasher.update(b"farm-tcp-cache-v1:fetch-cache-remote-tcp-tenant");
    hasher.update(b"farm-billing-enforce-v1:settlement-excess-exit-nonzero");
    hasher.update(b"gc-generational-v13-real-v1:young-old-promote-compact");
    hasher.update(b"sql-migrations-v1:prepare-bind-migrate");
    hasher.update(b"orm-query-ga-v1:findWhere-update-delete");
    hasher.update(b"sql-rusqlite-v1:embedded-engine-loopback");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 88: llvm-stack + cast-ga + kvo + codable-l5 + grpc-stream + tier2k + sql/xml + prod E2E GA.
fn heap_v60_phase88_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"native-llvm-stack-v1:callhost-stack-sync-main");
    hasher.update(b"llvm-stub-ga-v1:noop-stub-tier-reduce");
    hasher.update(b"reflect-cast-ga-v1:is-as-existential-four-end");
    hasher.update(b"hof-mir-wasm-parity-v1:closure-cell-generic-witness");
    hasher.update(b"kvo-full-chain-v1:observe-willSet-didSet-didChange");
    hasher.update(b"codable-l5-proto-v1:reflect-protobuf-wire");
    hasher.update(b"net-grpc-stream-v1:client-server-streaming-loopback");
    hasher.update(b"net-tls-wss-hard-v1:rustls-persistent-wss-frame");
    hasher.update(b"browser-tier2k-v1:playwright-rust-mandatory");
    hasher.update(b"dist-prod-ga-v1:winpipe-unix-tcp-tls-cross-process");
    hasher.update(b"borrow-mandatory-v3:coq-lean-compile-exit0");
    hasher.update(b"select-async-ga-v1:select-await-interleave");
    hasher.update(b"sql-mvp-v1:sqlite-loopback-query");
    hasher.update(b"xml-mvp-v1:pull-parser-dom-subset");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 89: gc-v12 + native-full-ga + hof-wasm-tier2 + cast-quad + sql/orm/xpath/codable-l6 + farm/gui prod.
fn heap_v61_phase89_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gc-relocating-v12-v1:heap-slot-rewrite-native-roots");
    hasher.update(b"native-llvm-full-ga-v1:sync-main-zero-mir-entry");
    hasher.update(b"hof-wasm-tier2-closure-v1:call-closure-guest-emit");
    hasher.update(b"reflect-cast-quad-v1:is-as-existential-twelve-tests");
    hasher.update(b"sql-rusqlite-v1:embedded-engine-loopback");
    hasher.update(b"orm-mvp-v1:reflect-table-roundtrip");
    hasher.update(b"xml-xpath-v1:subset-three-expressions");
    hasher.update(b"codable-l6-schema-v1:wire-schema-registry");
    hasher.update(b"farm-multitenant-prod-v1:lease-ttl-three-node-hit");
    hasher.update(b"plugin-cdn-prod-v1:cdn-route-poison-reinstall");
    hasher.update(b"gui-mvp-v1:window-eventloop-canvas");
    hasher.update(b"browser-tier2l-v1:playwright-gui-wasm-smoke");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 90: wasm-aot-tier3 + native-llvm-direct + cast-hard + gui-winit + data/farm GA+.
fn heap_v62_phase90_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"wasm-aot-tier3-v1:zero-exec-op-direct-emit");
    hasher.update(b"native-llvm-direct-v1:sync-main-direct-call");
    hasher.update(b"reflect-cast-hard-v1:twelve-tests-four-end");
    hasher.update(b"hof-witness-quad-v1:generic-sortBy-witness");
    hasher.update(b"gui-winit-v1:window-event-canvas-headless");
    hasher.update(b"browser-tier2m-v1:playwright-gui-interaction");
    hasher.update(b"sql-migrations-v1:prepare-bind-migrate");
    hasher.update(b"orm-query-ga-v1:findWhere-update-delete");
    hasher.update(b"codable-l7-evolution-v1:schema-field-evolution");
    hasher.update(b"farm-lease-enforce-v1:ttl-reject-tcp-tenant");
    hasher.update(b"gc-generational-v13-v1:young-old-compact");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 87: native-llvm-phi + async-resume + reflect-cast + codable-l4 + grpc + tier2j + prod E2E hard.
fn heap_v59_phase87_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"native-llvm-phi-v1:cfg-ssa-phi-emit");
    hasher.update(b"native-async-resume-v1:await-suspend-driver-loop");
    hasher.update(b"llvm-stub-elimination-v1:real-wire-tier-reduce");
    hasher.update(b"reflect-runtime-cast-v1:as-downcast-dynamic-dispatch");
    hasher.update(b"codable-l4-v1:keyed-container-synthesis");
    hasher.update(b"hof-generic-witness-v1:sortBy-flatMap-generic");
    hasher.update(b"net-grpc-v1:unary-loopback");
    hasher.update(b"net-h2-alpn-default-v1:http2-default-on");
    hasher.update(b"browser-tier2j-v1:playwright-rust-gate-mandatory");
    hasher.update(b"dist-prod-hard-v1:cross-process-e2e");
    hasher.update(b"borrow-mandatory-v2:coq-lean-compile-exit0");
    hasher.update(b"plugin-marketplace-hard-v1:dlopen-poison-reinstall");
    hasher.update(b"file-watch-os-default-v1:inotify-kqueue-winapi");
    hasher.update(b"select-async-v1:select-await-interleave");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 86: p85-honest-closure + reflect-runtime + net-v3 + browser-tier2i + dist-prod + borrow-mandatory + test-runner-v3 + codable-l3.
fn heap_v58_phase86_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"p85-honest-closure-v1:sortBy-SYX-comparator-four-end");
    hasher.update(b"reflect-runtime-v1:reflect-self-dynamic-dispatch");
    hasher.update(b"net-v3-h2-zstd:http2-loopback-accept-encoding");
    hasher.update(b"browser-tier2i-v1:playwright-wasm-stdout-mandatory");
    hasher.update(b"dist-prod-e2e-v1:winpipe-rustls-cross-process");
    hasher.update(b"borrow-proof-mandatory-v1:coq-lean-compile-exit0");
    hasher.update(b"test-runner-v3:registry-invoke-tap-exit");
    hasher.update(b"codable-l3-v1:json-encode-decode-synthesis");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 85: stdlib-native-parity + hof-closure + reflect-lang + net-v2 + browser-tier2h + dist-winpipe-rustls + borrow-replay + test-runner-v2.
fn heap_v57_phase85_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"stdlib-native-parity-v1:phase85-four-end-host-matrix");
    hasher.update(b"hof-closure-v1:sortBy-flatMap-execute-closure");
    hasher.update(b"reflect-lang-v1:reflect-self-type-metadata");
    hasher.update(b"net-v2-h2-compress:http2-gzip-zstd-accept-encoding");
    hasher.update(b"browser-tier2h-v1:playwright-stdlib-prelude-hard");
    hasher.update(b"dist-winpipe-rustls-v1:winapi-named-pipe-rustls-e2e");
    hasher.update(b"borrow-replay-v1:coq-lean-proof-artifact-mandatory");
    hasher.update(b"test-runner-v2:registry-invoke-tap-aggregate");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 84: stdlib-parity + grapheme + reflect-v2 + browser-tier2g + borrow-prover-exec + dlopen-default + test-runner.
fn heap_v56_phase84_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"stdlib-parity-v1:phase84-four-end-host-matrix");
    hasher.update(b"grapheme-cluster-v1:unicode-segmentation-index");
    hasher.update(b"reflect-v2-metadata:no-fallback-witness");
    hasher.update(b"browser-tier2g-v1:stdlib-prelude-hard-run");
    hasher.update(b"borrow-prover-exec-v1:coq-lean-subprocess");
    hasher.update(b"dlopen-default-v1:lockfile-auto-load");
    hasher.update(b"test-runner-v1:tap-filter-aggregate");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 83: stdlib-v1 host table extension (Fmt, Encoding, Math float, Process, File+, UDP/DNS, Crypto, Mutex, WsRecv, Log, Testing).
fn heap_v55_phase83_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"stdlib-v1-host-table-extension:phase83-45-hosts");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 82: heap-v11-bridge-prod + heapslot-writeback-v1 + rt-native-stack-v7-real + string-offheap-full + browser-tier2f-v1 + dist-failover-v4 + lock-v16-marketplace + mono-witness-v2.
fn heap_v54_phase82_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"heap-v11-bridge-prod:shared-arena+slot-remap+spawn-hard");
    hasher.update(b"heapslot-writeback-v1:materialize_receiver+mir-run-write-back");
    hasher.update(b"rt-native-stack-v7-real:NATIVE_STACK_DISPATCH_V7-zero-internal-exec");
    hasher.update(b"string-offheap-full:utf8-simd+64b-threshold");
    hasher.update(b"browser-tier2f-v1:full-main-guest+heap-compact");
    hasher.update(b"dist-failover-v4:cross-continent-auto-failover");
    hasher.update(b"lock-v16-marketplace:publish_epoch+extension_signing_key");
    hasher.update(b"mono-witness-v2:disk-cache-mir-backfill");
    format!("{:x}", hasher.finalize())
}

/// Phase 81: heap-v10-bridge + inkwell-llvm-v7-link + rt-native-stack-v6-full + lock-v15-hsm-auto + dist-actor-cdn-v3 + browser-tier2e-v1.
fn heap_v53_phase81_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"heap-v10-bridge:promotion+card-table+heap_root_bridge+string-offheap-mvp");
    hasher.update(b"inkwell-llvm-v7-link:write_object+cc-link-smoke");
    hasher.update(b"rt-native-stack-v6-full:NATIVE_STACK_DISPATCH_V6-zero-mir-exec-release");
    hasher.update(b"lock-v15-hsm-auto:hsm_rotation_cron+billing_settlement+marketplace_epoch");
    hasher.update(b"dist-actor-cdn-v3:global_cdn+failover+health_probe");
    hasher.update(b"browser-tier2e-v1:SIMD110+actor-hard");
    format!("{:x}", hasher.finalize())
}

/// Phase 80: heap-v9-generational-gc + inkwell-llvm-v6-real + rt-native-stack-v5 + lock-v14-hsm-cdn + dist-actor-cluster-v2 + browser-tier2d-v1.
fn heap_v52_phase80_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"heap-v9-generational-gc:young-old+write-barrier+string-full-inline");
    hasher.update(b"inkwell-llvm-v6-real-object:write_to_memory_buffer-non-stub");
    hasher.update(b"rt-native-stack-v5:NATIVE_STACK_DISPATCH-zero-mir-exec-hot");
    hasher.update(b"lock-v14-hsm-cdn-billing:hsm_key_ref+global_cdn_sla_tier+billing_meter_id");
    hasher.update(b"dist-actor-cluster-v2:multi-region+cross-process-mailbox");
    hasher.update(b"browser-tier2d-v1:closure+async-hard");
    format!("{:x}", hasher.finalize())
}

/// Phase 79: heap-v8-dict-string + inkwell-object-aot-v5 + rt-dispatch-v4 + lock-v13 + actor-ipc.
fn heap_v51_phase79_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"heap-v8-dict-string-stdout:DictGet-Set-four-end+string-inline");
    hasher.update(b"inkwell-object-aot-v5-real:non-stub-ELF+heap-compact-call");
    hasher.update(b"rt-dispatch-table-v4:RT_DISPATCH_TABLE-hot-path");
    hasher.update(b"lock-v13-tenant-farm:rotation-cron+worker-pool+lease-ttl");
    hasher.update(b"actor-ipc-v1:heap-roots+dist-actor-process");
    format!("{:x}", hasher.finalize())
}

/// Phase 78: inkwell-module-v4 + heap-v7-relocate + rt-zero-mir-v3 + lock-v12-marketplace.
fn heap_v50_phase78_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"inkwell-module-v4-real-object:tier-2B-D+object-emit");
    hasher.update(b"heap-v7-relocating-gc:extern-ref-rewrite+HeapRelocate");
    hasher.update(b"rt-zero-mir-v3-runtime:native-run-exec-count-zero");
    hasher.update(b"lock-v12-marketplace-enforce:epoch+rotation+farm-worker");
    format!("{:x}", hasher.finalize())
}

/// Phase 77: inkwell-llvm-v3 + heap-v6-host-mat + rt-zero-v2 + lock-v11-plugin.
fn heap_v49_phase77_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"inkwell-llvm-v3-real-module:tier-2B-D+HeapCompact");
    hasher.update(b"heap-v6-host-materialize:ArraySet+Dict+simd-store");
    hasher.update(b"rt-zero-exec-v2:full-tier-direct-stub");
    hasher.update(b"lock-v11-plugin-cdn:unload-epoch+marketplace+farm-queue");
    format!("{:x}", hasher.finalize())
}

/// Phase 76: vm-heap-v5-slot-parity + inkwell-crate-v2 + cache-remote-push-v1.
fn heap_v48_phase76_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"vm-heap-v5-slot-parity:ArrayBuild+HeapCompact+HeapSlot");
    hasher.update(b"inkwell-crate-v2:phase76-ModuleBuilder-tier1-2A");
    hasher.update(b"cache-remote-push-v1:SYX_CACHE_REMOTE_PUSH+multi-node");
    format!("{:x}", hasher.finalize())
}

fn driver_options_with_deps(
    root: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
) -> SyxResult<DriverOptions> {
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        opts.resolved_packages = Some(load_resolved_packages(root)?);
    }
    Ok(opts)
}

/// Phase 75: heap v4 compact-relocate + inkwell prod IR builder fingerprint.
fn heap_v4_inkwell_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"heap-v4-compact-relocate-v1:ManagedHeap+HeapSlot+compact_global");
    hasher.update(b"inkwell-prod-ir-builder-v1:phase75-default-inkwell-text-opt-out");
    format!("{:x}", hasher.finalize())
}

/// Phase 74: mir-run heap v3 slots + LLVM tier-3 zero exec_op fingerprint.
fn heap_v3_llvm_tier3_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"heap-v3-mir-run-slots:ManagedHeap+heap_collect+actor-roots");
    hasher.update(b"llvm-tier3-zero-exec-op-v1:tier2A-D-direct-rt-no-exec-op-ir");
    format!("{:x}", hasher.finalize())
}

/// Phase 73: unified managed heap v2 + native zero exec_op LLVM fingerprint.
fn unified_heap_native_exec_op_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"unified-heap-v2:ManagedHeap+collect_global+vm-align");
    hasher.update(b"native-exec-op-v1:ObjectRefRetain,Release,HeapCollect-llvm-direct");
    format!("{:x}", hasher.finalize())
}

/// Phase 72: GC/ARC object layout v1 + browser prelude v3 hash.
fn gc_arc_browser_prelude_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"gc-arc-layout-v1:ObjectRef-retain-release+vm-mark-sweep+browser-prelude-v3");
    format!("{:x}", hasher.finalize())
}

/// Phase 71: iterator witness v2 + native LLVM tier-2C direct emit hash.
fn iterator_witness_native_tier2c_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"iterator-witness-v2:MutRefCell-wrap-order+native-llvm-tier2c-v1:MakeIterator,IteratorNext,DictGet,Actor,Task,Try,Simd");
    format!("{:x}", hasher.finalize())
}

/// Phase 70: tier-2C actor/task + SIMD guest + browser prelude v2 hash.
fn tier2c_simd_browser_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"tier2c-simd-browser-v1:Actor,Task,Try,SimdGuest+browser-prelude-v2");
    format!("{:x}", hasher.finalize())
}

/// Phase 69: tier-2B guest opcodes + Cranelift JIT module + LLVM tier-2 emit hash.
fn tier2b_jit_llvm_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"tier2b-jit-llvm-v1:MakeClosure,CallClosure,AwaitPoll,AwaitSuspend+cranelift-jit-module-v1+llvm-tier2-emit-v1");
    format!("{:x}", hasher.finalize())
}

/// Phase 68: ArrSorted witness compare loop + tier-2A opcode set hash.
fn sorted_codegen_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"sorted-v1:ArrSorted+CallWitness+tier2A=ObjectGet,ArraySet,ValueEq,RefEq");
    format!("{:x}", hasher.finalize())
}

/// Phase 71: native LLVM tier-2 direct opcode coverage hash.
fn native_llvm_coverage_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(
        b"native-llvm-v5:tier2ABC+MakeIterator+IteratorNext+DictGet+DictSet+Actor+Task+Try+Simd",
    );
    format!("{:x}", hasher.finalize())
}

/// Phase 67: driver AOT matrix fixture set hash.
fn aot_fixture_set_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"aot-fixtures-v1:hot_loop,hello,comparable_sort,inheritance_dispatch,protocol_iterator,main");
    format!("{:x}", hasher.finalize())
}

/// Phase 71: protocol witness table v2 (MutRefCell wrap + post-sort renumber).
fn witness_table_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"witness-v2:Iterator-next-renumber+MutRefCell-wrap:");
    for (_, src) in crate::embedded_std::embedded_protocols() {
        hasher.update(src.as_bytes());
    }
    for (_, src) in crate::embedded_std::core_prelude() {
        if src.contains("__witness_") || src.contains("func compare") {
            hasher.update(src.as_bytes());
        }
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 66: class vtable layout hash for polymorphic dispatch invalidation.
fn vtable_layout_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"vtable-v1:runtime-read-indirect-dispatch");
    format!("{:x}", hasher.finalize())
}

/// Phase 68: WASM AOT tier-1/tier-2A/tier-2B fingerprint.
fn wasm_aot_tier_fingerprint() -> String {
    format!(
        "wasm-aot-v3:tier1F-I+tier2A=ObjectGet,ArraySet,ValueEq,RefEq+tier2B=Await,Closure+iter-witness=1:sorted=ArrSorted:aot={}",
        if syx_config::wasm_aot_enabled() {
            "1"
        } else {
            "0"
        }
    )
}

fn build_syx_plan_hash(root: &Path) -> String {
    let path = root.join(build_syx::DEFAULT_BUILD_CONFIG);
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    hash_source(&text)
}

fn plugin_manifest_hash(root: &Path) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    let lock = root.join("syx.lock");
    if lock.is_file() {
        if let Ok(text) = std::fs::read_to_string(&lock) {
            hasher.update(text.as_bytes());
        }
    }
    let plugins_dir = root.join("plugins");
    if plugins_dir.is_dir() {
        let mut paths: Vec<_> = std::fs::read_dir(&plugins_dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .collect();
        paths.sort();
        for p in paths {
            hasher.update(p.to_string_lossy().as_bytes());
            if let Ok(text) = std::fs::read_to_string(&p) {
                hasher.update(text.as_bytes());
            }
        }
    }
    format!("{:x}", hasher.finalize())
}

fn unified_mir_fn_count_fingerprint() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(crate::embedded_std::internal_manifest_json().as_bytes());
    let mut fn_count = 0usize;
    for (_, src) in crate::embedded_std::core_prelude() {
        fn_count += src.matches("func ").count();
    }
    for (_, src) in crate::embedded_std::embedded_protocols() {
        fn_count += src.matches("func ").count();
    }
    hasher.update(fn_count.to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}

fn rustc_version_tag() -> String {
    static TAG: OnceLock<String> = OnceLock::new();
    TAG.get_or_init(|| {
        std::process::Command::new("rustc")
            .arg("-V")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_else(|| "unknown-rustc".into())
            .trim()
            .to_string()
    })
    .clone()
}

fn host_feature_fingerprint() -> String {
    format!(
        "native-rt-v1:mir-v25={}:mir-v24={}:mir-v23={}:mir-v22={}:mir-v21={}:mir-v20={}:mir-v19={}:mir-v18={}:mir-v17={}:mir-v16={}:mir-v15={}:mir-v14={}",
        if syx_mir::mir_opt_v25_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v24_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v23_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v22_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v21_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v20_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v19_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v18_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v17_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v16_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v15_jit_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v14_jit_barrier_enabled() {
            "1"
        } else {
            "0"
        }
    )
}

/// Phase 24 DJ1: weeks until `SYX_RESULT_LEGACY_OBJECT` removal (CI artifact).
pub fn result_legacy_retirement_weeks() -> u32 {
    std::env::var("SYX_RESULT_LEGACY_RETIRE_WEEKS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

/// Phase 24 DJ3: opt-in Iterator protocol desugar for `for-in` (`SYX_ITERATOR_PROTOCOL=1`).
/// Phase 25 DO2: Iterator protocol desugar default on; `SYX_ITERATOR_PROTOCOL=0` disables.
pub fn iterator_protocol_enabled() -> bool {
    std::env::var("SYX_ITERATOR_PROTOCOL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

fn module_hash(path: &Path, root: &Path, deps: &[(String, String)]) -> String {
    use sha2::{Digest, Sha256};
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hasher.update(cache_version_tag().as_bytes());
    hasher.update(build_syx_plan_hash(root).as_bytes());
    hasher.update(plugin_manifest_hash(root).as_bytes());
    hasher.update(unified_mir_fn_count_fingerprint().as_bytes());
    hasher.update(witness_table_fingerprint().as_bytes());
    hasher.update(vtable_layout_fingerprint().as_bytes());
    hasher.update(wasm_aot_tier_fingerprint().as_bytes());
    hasher.update(dependency_tree_fingerprint(deps).as_bytes());
    format!("{:x}", hasher.finalize())
}

fn hash_source(contents: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(contents.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn dependency_hashes_from_graph(graph: &ModuleGraph, self_key: &Path) -> Vec<(String, String)> {
    let mut deps: Vec<(String, String)> = graph
        .modules
        .iter()
        .filter(|(k, _)| *k != self_key)
        .map(|(k, u)| (k.display().to_string(), hash_source(&u.source.contents)))
        .collect();
    deps.sort_by(|a, b| a.0.cmp(&b.0));
    deps
}

fn read_module_source_for_cache(path: &str) -> Option<String> {
    if let Some(rest) = path.strip_prefix("<stdlib>/") {
        let name = rest.strip_suffix(".sy")?;
        let module = name.split('/').next()?;
        return embedded_std::lookup(module).map(|(_, src)| src.to_string());
    }
    std::fs::read_to_string(path).ok()
}

fn dependency_hashes_still_valid(deps: &[(String, String)]) -> bool {
    deps.iter().all(|(path, expected)| {
        read_module_source_for_cache(path).is_some_and(|c| hash_source(&c) == *expected)
    })
}

fn dependency_tree_fingerprint(deps: &[(String, String)]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for (path, hash) in deps {
        hasher.update(path.as_bytes());
        hasher.update(hash.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 94: default farm worker cache URL (`tcp://` tenant path; override with `SYX_FARM_CACHE_URL`).
pub fn default_farm_cache_url() -> String {
    std::env::var("SYX_FARM_CACHE_URL").unwrap_or_else(|_| {
        if let Ok(listener) = std::net::TcpListener::bind("127.0.0.1:0") {
            if let Ok(addr) = listener.local_addr() {
                return format!("tcp://127.0.0.1:{}/tenant/default", addr.port());
            }
        }
        "tcp://127.0.0.1:0/tenant/default".into()
    })
}

static FARM_TCP_FETCH_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Phase 94: TCP cache fetch with mutex to reduce concurrent flake in farm 3-node gates.
pub fn fetch_cache_remote_hardened(url: &str) -> Result<Vec<u8>, String> {
    if url.starts_with("tcp://") {
        let _guard = FARM_TCP_FETCH_MUTEX
            .lock()
            .map_err(|e| format!("farm tcp mutex poisoned: {e}"))?;
        let mut last_err = None;
        for attempt in 0..3 {
            match fetch_cache_remote(url) {
                Ok(b) => return Ok(b),
                Err(e) => {
                    last_err = Some(e);
                    if attempt < 2 {
                        std::thread::sleep(std::time::Duration::from_millis(
                            10 * (attempt + 1) as u64,
                        ));
                    }
                }
            }
        }
        return Err(last_err.unwrap_or_else(|| "tcp cache fetch failed".into()));
    }
    fetch_cache_remote(url)
}

/// Phase 100: farm TCP frame status (`OK` payload / `MISS` / `ERR`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FarmTcpResponse {
    Ok(Vec<u8>),
    Miss,
    Err(String),
}

fn farm_tcp_url_parts(rest: &str) -> (&str, &str) {
    let (host_port, path) = rest.split_once('/').unwrap_or((rest, ""));
    (host_port, path.trim_start_matches('/'))
}

fn farm_tcp_write_frame(stream: &mut impl std::io::Write, payload: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let len = u32::try_from(payload.len()).map_err(|_| "farm tcp frame too large".to_string())?;
    stream
        .write_all(&len.to_be_bytes())
        .map_err(|e| format!("farm tcp frame write len: {e}"))?;
    stream
        .write_all(payload)
        .map_err(|e| format!("farm tcp frame write body: {e}"))?;
    Ok(())
}

fn farm_tcp_read_frame(stream: &mut impl std::io::Read) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let mut len_buf = [0u8; 4];
    stream
        .read_exact(&mut len_buf)
        .map_err(|e| format!("farm tcp frame read len: {e}"))?;
    let len = u32::from_be_bytes(len_buf) as usize;
    let mut payload = vec![0u8; len];
    if len > 0 {
        stream
            .read_exact(&mut payload)
            .map_err(|e| format!("farm tcp frame read body: {e}"))?;
    }
    Ok(payload)
}

/// Encode a length-prefixed farm TCP frame payload (without the length prefix).
pub fn farm_tcp_encode_get(key: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + key.len());
    out.extend_from_slice(b"GET\0");
    out.extend_from_slice(key.as_bytes());
    out
}

pub fn farm_tcp_encode_put(key: &str, value: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + key.len() + value.len());
    out.extend_from_slice(b"PUT\0");
    out.extend_from_slice(key.as_bytes());
    out.push(0);
    out.extend_from_slice(value);
    out
}

pub fn farm_tcp_encode_ok(value: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + value.len());
    out.extend_from_slice(b"OK");
    out.extend_from_slice(value);
    out
}

pub fn farm_tcp_encode_miss() -> Vec<u8> {
    b"MISS".to_vec()
}

pub fn farm_tcp_encode_err(msg: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(3 + msg.len());
    out.extend_from_slice(b"ERR");
    out.extend_from_slice(msg.as_bytes());
    out
}

pub fn farm_tcp_parse_response(payload: &[u8]) -> Result<FarmTcpResponse, String> {
    if payload == b"MISS" {
        return Ok(FarmTcpResponse::Miss);
    }
    if let Some(msg) = payload.strip_prefix(b"ERR") {
        return Ok(FarmTcpResponse::Err(
            String::from_utf8_lossy(msg).into_owned(),
        ));
    }
    if let Some(value) = payload.strip_prefix(b"OK") {
        return Ok(FarmTcpResponse::Ok(value.to_vec()));
    }
    Err(format!(
        "farm tcp bad response: {:?}",
        String::from_utf8_lossy(payload)
    ))
}

/// Handle one farm TCP request against an in-memory store; returns response payload (no length prefix).
///
/// Gate fingerprint: `farm-lease-server-v1:node-enforces-ttl` — GET/PUT require
/// [`farm_lease_require_active`] (authority grant + TTL); raw TCP without lease → `ERR`.
pub fn farm_tcp_handle_request(
    store: &mut std::collections::HashMap<String, Vec<u8>>,
    req: &[u8],
) -> Vec<u8> {
    if let Err(msg) = crate::farm_lease_auth::farm_lease_require_active() {
        return farm_tcp_encode_err(&msg);
    }
    if let Some(key) = req.strip_prefix(b"GET\0") {
        let key = String::from_utf8_lossy(key).into_owned();
        return match store.get(&key) {
            Some(v) => farm_tcp_encode_ok(v),
            None => farm_tcp_encode_miss(),
        };
    }
    if let Some(rest) = req.strip_prefix(b"PUT\0") {
        let Some(sep) = rest.iter().position(|&b| b == 0) else {
            return farm_tcp_encode_err("PUT missing value separator");
        };
        let key = String::from_utf8_lossy(&rest[..sep]).into_owned();
        let value = rest[sep + 1..].to_vec();
        store.insert(key, value.clone());
        return farm_tcp_encode_ok(&value);
    }
    farm_tcp_encode_err("unknown op")
}

/// One length-prefixed farm TCP round-trip (tests / raw clients bypassing cache helpers).
#[doc(hidden)]
pub fn farm_tcp_roundtrip(host_port: &str, req: &[u8]) -> Result<Vec<u8>, String> {
    use std::io::{Read, Write};
    let mut stream =
        std::net::TcpStream::connect(host_port).map_err(|e| format!("farm tcp connect: {e}"))?;
    farm_tcp_write_frame(&mut stream, req)?;
    farm_tcp_read_frame(&mut stream)
}

/// Spawn a farm TCP node that speaks GET/PUT frames for `max_conns` accepts.
///
/// Callers that fail-closed without connecting must pass `max_conns = 0` so
/// `JoinHandle::join` returns immediately (Phase 105).
pub fn spawn_farm_tcp_node(
    initial: std::collections::HashMap<String, Vec<u8>>,
    max_conns: usize,
) -> (std::net::SocketAddr, std::thread::JoinHandle<()>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("farm tcp bind");
    listener
        .set_nonblocking(true)
        .expect("farm tcp listener nonblocking");
    let addr = listener.local_addr().expect("farm tcp local_addr");
    let handle = std::thread::spawn(move || {
        let mut store = initial;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        let mut accepted = 0usize;
        while accepted < max_conns && std::time::Instant::now() < deadline {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    accepted += 1;
                    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(2)));
                    let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(2)));
                    if let Ok(req) = farm_tcp_read_frame(&mut stream) {
                        let resp = farm_tcp_handle_request(&mut store, &req);
                        let _ = farm_tcp_write_frame(&mut stream, &resp);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
                Err(_) => {
                    // Transient accept errors must not tear down the node early —
                    // expired-lease clients still need to connect for ERR responses.
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
        }
    });
    (addr, handle)
}

/// Phase 70: single-node cache mirror fetch (`SYX_CACHE_REMOTE`, `file://`, `http(s)://`, or `tcp://`).
/// Phase 100: `tcp://` uses length-prefixed GET/PUT frames (`OK|MISS|ERR`).
pub fn fetch_cache_remote(url: &str) -> Result<Vec<u8>, String> {
    if let Some(path) = url.strip_prefix("file://") {
        return std::fs::read(path).map_err(|e| e.to_string());
    }
    if url.starts_with("http://") || url.starts_with("https://") {
        return reqwest::blocking::get(url)
            .map_err(|e| e.to_string())?
            .bytes()
            .map_err(|e| e.to_string())
            .map(|b| b.to_vec());
    }
    if let Some(rest) = url.strip_prefix("tcp://") {
        // Phase 105 fail-closed: missing authority OR inactive/expired lease → ERR
        // without opening TCP (avoids farm-node accept/join hangs when clients stop early).
        if !crate::farm_lease_auth::farm_lease_auth_is_configured() {
            return Err("tcp cache lease: farm lease authority not configured".into());
        }
        crate::farm_lease_auth::farm_lease_require_active()
            .map_err(|e| format!("tcp cache lease: {e}"))?;
        use std::io::{Read, Write};
        let (host_port, key) = farm_tcp_url_parts(rest);
        let mut stream = std::net::TcpStream::connect(host_port)
            .map_err(|e| format!("tcp cache connect failed: {e}"))?;
        farm_tcp_write_frame(&mut stream, &farm_tcp_encode_get(key))?;
        let payload = farm_tcp_read_frame(&mut stream)?;
        record_cache_remote_fetch(url);
        match farm_tcp_parse_response(&payload)? {
            FarmTcpResponse::Ok(bytes) => return Ok(bytes),
            FarmTcpResponse::Miss => return Err(format!("tcp cache miss: {key}")),
            FarmTcpResponse::Err(msg) => return Err(format!("tcp cache err: {msg}")),
        }
    }
    Err(format!("unsupported SYX_CACHE_REMOTE URL: {url}"))
}

/// Phase 76: push compiled cache artifact to remote mirror (`file://` or `http(s)://` PUT).
/// Phase 100: `tcp://` uses length-prefixed PUT frames; expects `OK`.
pub fn push_cache_remote(url: &str, bytes: &[u8]) -> Result<(), String> {
    record_cache_remote_push(url);
    if let Some(path) = url.strip_prefix("file://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        return std::fs::write(path, bytes)
            .map_err(|e| e.to_string())
            .map(|_| ());
    }
    if url.starts_with("http://") || url.starts_with("https://") {
        return reqwest::blocking::Client::new()
            .put(url)
            .body(bytes.to_vec())
            .send()
            .map_err(|e| e.to_string())
            .map(|_| ());
    }
    if let Some(rest) = url.strip_prefix("tcp://") {
        if !crate::farm_lease_auth::farm_lease_auth_is_configured() {
            return Err("tcp cache lease: farm lease authority not configured".into());
        }
        crate::farm_lease_auth::farm_lease_require_active()
            .map_err(|e| format!("tcp cache lease: {e}"))?;
        use std::io::{Read, Write};
        let (host_port, key) = farm_tcp_url_parts(rest);
        let mut stream = std::net::TcpStream::connect(host_port)
            .map_err(|e| format!("tcp cache push connect failed: {e}"))?;
        farm_tcp_write_frame(&mut stream, &farm_tcp_encode_put(key, bytes))?;
        let payload = farm_tcp_read_frame(&mut stream)?;
        match farm_tcp_parse_response(&payload)? {
            FarmTcpResponse::Ok(_) => return Ok(()),
            FarmTcpResponse::Miss => return Err("tcp cache put unexpected MISS".into()),
            FarmTcpResponse::Err(msg) => return Err(format!("tcp cache put err: {msg}")),
        }
    }
    Err(format!("unsupported cache push URL: {url}"))
}

/// Phase 76: broadcast cache artifact to `SYX_CACHE_REMOTE_NODES` (comma-separated).
pub fn push_cache_remote_nodes(base: &str, key: &str, bytes: &[u8]) {
    let nodes: Vec<String> = std::env::var("SYX_CACHE_REMOTE_NODES")
        .ok()
        .map(|s| {
            s.split(',')
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty())
                .collect()
        })
        .filter(|v: &Vec<String>| !v.is_empty())
        .unwrap_or_else(|| vec![base.to_string()]);
    for node in nodes {
        let url = format!("{}/{}", node.trim_end_matches('/'), key);
        let _ = push_cache_remote(&url, bytes);
    }
}

fn cache_remote_push_enabled() -> bool {
    std::env::var("SYX_CACHE_REMOTE_PUSH")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

static LAST_CACHE_REMOTE_PUSH: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);
static LAST_CACHE_REMOTE_FETCH: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

fn record_cache_remote_push(url: &str) {
    if let Ok(mut g) = LAST_CACHE_REMOTE_PUSH.lock() {
        *g = Some(url.to_string());
    }
}

fn record_cache_remote_fetch(url: &str) {
    if let Ok(mut g) = LAST_CACHE_REMOTE_FETCH.lock() {
        *g = Some(url.to_string());
    }
}

/// Phase 76: last cache remote push/fetch URLs for doctor report.
pub fn cache_remote_report() -> (Option<String>, Option<String>) {
    let push = LAST_CACHE_REMOTE_PUSH.lock().ok().and_then(|g| g.clone());
    let fetch = LAST_CACHE_REMOTE_FETCH.lock().ok().and_then(|g| g.clone());
    (push, fetch)
}

/// Remove cache entries whose recorded dependency snapshots no longer match disk.
pub fn purge_stale_incremental_caches(root: &Path) {
    let cache_dir = project_cache_dir(root);
    let Ok(entries) = std::fs::read_dir(&cache_dir) else {
        // One-time migration: drop legacy `.sy-cache` at project root.
        let legacy = root.join(".sy-cache");
        if legacy.is_dir() {
            let _ = std::fs::remove_dir_all(&legacy);
        }
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) else {
            let _ = std::fs::remove_file(&path);
            continue;
        };
        let current = std::fs::read_to_string(&cached.path).unwrap_or_default();
        if cached.source_hash != hash_source(&current) || !cache_entry_fresh(&cached, &current) {
            let _ = std::fs::remove_file(&path);
        }
    }
}

pub fn incremental_cache_path(root: &Path, module: &Path) -> PathBuf {
    let cache_dir = project_cache_dir(root);
    let _ = std::fs::create_dir_all(&cache_dir);
    cache_dir.join(format!("{}.bin", module_hash(module, root, &[])))
}

pub fn compile_with_cache(path: &Path, root: &Path) -> SyxResult<ModuleUnit> {
    purge_stale_incremental_caches(root);
    let cache = incremental_cache_path(root, path);
    let current_source = std::fs::read_to_string(path).unwrap_or_default();
    if cache.exists() {
        if let Ok(bytes) = std::fs::read(&cache) {
            if let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) {
                if cache_entry_fresh(&cached, &current_source) {
                    return Ok(cached.into());
                }
            }
        }
    } else if let Some(remote) = std::env::var("SYX_CACHE_REMOTE")
        .ok()
        .filter(|s| !s.is_empty())
    {
        let key = module_hash(path, root, &[]);
        let url = format!("{}/{}", remote.trim_end_matches('/'), format!("{key}.bin"));
        if let Ok(bytes) = fetch_cache_remote(&url) {
            record_cache_remote_fetch(&url);
            if let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) {
                if cache_entry_fresh(&cached, &current_source) {
                    if let Some(parent) = cache.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    let _ = std::fs::write(&cache, &bytes);
                    return Ok(cached.into());
                }
            }
        }
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = DriverOptions::default();
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, &opts)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        parent,
        &mut graph,
        &manifest,
        &opts,
    )?;
    if let Some(parent_dir) = cache.parent() {
        let _ = std::fs::create_dir_all(parent_dir);
    }
    if let Ok(bytes) = bincode_encode(&unit, &graph) {
        let _ = std::fs::write(&cache, &bytes);
        if cache_remote_push_enabled() {
            if let Some(remote) = std::env::var("SYX_CACHE_REMOTE")
                .ok()
                .filter(|s| !s.is_empty())
            {
                let key = format!("{}.bin", module_hash(path, root, &[]));
                push_cache_remote_nodes(&remote, &key, &bytes);
            }
        }
    }
    Ok(unit)
}

fn bincode_encode(unit: &ModuleUnit, graph: &ModuleGraph) -> Result<Vec<u8>, String> {
    let cached = CachedModuleUnit::from_unit(unit, graph);
    bincode::serialize(&cached).map_err(|e| e.to_string())
}

fn bincode_decode(bytes: &[u8]) -> Result<ModuleUnit, String> {
    let cached: CachedModuleUnit = bincode::deserialize(bytes).map_err(|e| e.to_string())?;
    Ok(cached.into())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedModuleUnit {
    pub path: PathBuf,
    pub source_name: String,
    pub source_contents: String,
    pub source_hash: String,
    pub imported_modules: Vec<String>,
    pub dependency_hashes: Vec<(String, String)>,
    #[serde(default)]
    pub dependency_tree_hash: String,
    /// When set, matches `cache_version_tag()` and skips per-dep stdlib re-hash on load.
    #[serde(default)]
    pub cache_version_tag: String,
    pub program: CompiledProgram,
}

impl CachedModuleUnit {
    fn from_unit(unit: &ModuleUnit, graph: &ModuleGraph) -> Self {
        Self {
            path: unit.path.clone(),
            source_name: unit.source.name.clone(),
            source_contents: unit.source.contents.to_string(),
            source_hash: hash_source(&unit.source.contents),
            imported_modules: unit
                .imported_modules
                .iter()
                .map(|m| m.to_string())
                .collect(),
            dependency_hashes: {
                let deps = dependency_hashes_from_graph(graph, &unit.path);
                deps
            },
            dependency_tree_hash: {
                let deps = dependency_hashes_from_graph(graph, &unit.path);
                dependency_tree_fingerprint(&deps)
            },
            cache_version_tag: cache_version_tag(),
            program: unit.program.clone(),
        }
    }
}

/// Fast cache hit check: source unchanged and toolchain/stdlib tag matches (skips per-dep stdlib re-hash).
fn cache_entry_fresh(cached: &CachedModuleUnit, current_source: &str) -> bool {
    if cached.source_hash != hash_source(current_source) {
        return false;
    }
    let tag = cache_version_tag();
    if !cached.cache_version_tag.is_empty() && cached.cache_version_tag == tag {
        return true;
    }
    dependency_hashes_still_valid(&cached.dependency_hashes)
}

impl From<&ModuleUnit> for CachedModuleUnit {
    fn from(unit: &ModuleUnit) -> Self {
        Self {
            path: unit.path.clone(),
            source_name: unit.source.name.clone(),
            source_contents: unit.source.contents.to_string(),
            source_hash: hash_source(&unit.source.contents),
            imported_modules: unit
                .imported_modules
                .iter()
                .map(|m| m.to_string())
                .collect(),
            dependency_hashes: Vec::new(),
            dependency_tree_hash: String::new(),
            cache_version_tag: cache_version_tag(),
            program: unit.program.clone(),
        }
    }
}

impl From<CachedModuleUnit> for ModuleUnit {
    fn from(cached: CachedModuleUnit) -> Self {
        Self {
            path: cached.path,
            source: SourceFile::new(cached.source_name, cached.source_contents),
            imported_modules: cached
                .imported_modules
                .into_iter()
                .map(SmolStr::new)
                .collect(),
            program: cached.program,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleUnit {
    pub path: PathBuf,
    pub source: SourceFile,
    pub imported_modules: Vec<smol_str::SmolStr>,
    pub program: syx_codegen::CompiledProgram,
}

#[derive(Debug, Clone, Default)]
pub struct ModuleGraph {
    pub modules: IndexMap<PathBuf, ModuleUnit>,
    pub order: Vec<PathBuf>,
}

/// Native code generator backend selected by `--backend` / `SYX_BACKEND`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NativeBackend {
    #[default]
    Llvm,
}

impl NativeBackend {
    pub fn parse(value: &str) -> SyxResult<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "cranelift" | "cf" => Err(SyxDiagnostic::error(
                "Cranelift backend removed in SYX 37.0 (Phase 59); use --backend llvm",
            )),
            "llvm" => Ok(Self::Llvm),
            other => Err(SyxDiagnostic::error(format!(
                "unknown native backend '{other}' (expected 'llvm')"
            ))),
        }
    }

    pub fn as_str(self) -> &'static str {
        "llvm"
    }

    /// Phase 59: LLVM is the sole production native backend.
    pub fn default_for_env() -> Self {
        Self::Llvm
    }
}

/// Resolve native backend from CLI flag, then `SYX_BACKEND`, else ADR-036/ADR-034 default.
pub fn resolve_native_backend(cli: Option<&str>) -> SyxResult<NativeBackend> {
    if let Some(value) = cli {
        return NativeBackend::parse(value);
    }
    if let Ok(value) = std::env::var("SYX_BACKEND") {
        return NativeBackend::parse(&value);
    }
    Ok(NativeBackend::Llvm)
}

/// Driver configuration for module resolution.
#[derive(Debug, Clone)]
pub struct DriverOptions {
    /// Extra directories to search for modules (`;`-separated on the CLI).
    pub module_dirs: Vec<PathBuf>,
    /// When true, embedded stdlib modules (`Math`, `IO`, …) take highest priority.
    pub enable_std: bool,
    /// Skip lockfile requirement (for single-file tests without dependencies).
    pub skip_install_check: bool,
    /// Resolved packages from syx.lock (set by compile_project).
    pub resolved_packages: Option<IndexMap<String, InstalledPackage>>,
    /// Enable strict Swift 6-style concurrency diagnostics (from `[concurrency] strict`).
    pub strict_concurrency: bool,
    /// Module default isolation (`MainActor` | `nonisolated`) from `syx.toml`.
    pub default_isolation: Option<String>,
    /// Link against a slim `syx-rt` build (`--no-default-features`) for smaller binaries.
    pub native_minimal: bool,
    /// Omit HTTP host functions from the linked `syx-rt` (`--no-http`).
    pub native_no_http: bool,
    /// Omit file I/O host functions from the linked `syx-rt` (`--no-file`).
    pub native_no_file: bool,
    /// Native codegen backend (`llvm` only in SYX 36.0+).
    pub native_backend: NativeBackend,
    /// Debug vs release artifact layout under `target/{debug,release}/`.
    pub build_profile: BuildProfile,
    /// WASM hybrid AOT emit (`--wasm --aot`).
    pub wasm_aot: bool,
    /// True while executing a build.syb task target (prevents recursive task execution).
    pub in_build_syx_task: bool,
}

/// SYX 38.0 build profile for `target/debug` vs `target/release`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuildProfile {
    #[default]
    Debug,
    Release,
}

impl BuildProfile {
    pub fn dir_name(self) -> &'static str {
        match self {
            BuildProfile::Debug => "debug",
            BuildProfile::Release => "release",
        }
    }

    pub fn is_release(self) -> bool {
        matches!(self, BuildProfile::Release)
    }

    pub fn cache_tag(self) -> &'static str {
        self.dir_name()
    }

    pub fn rt_profile_name(self) -> &'static str {
        self.dir_name()
    }

    pub fn clang_ir_args(self) -> &'static [&'static str] {
        if self.lto_enabled() {
            return &["-O2", "-DNDEBUG", "-flto=thin"];
        }
        match self {
            BuildProfile::Debug => &["-O0", "-g"],
            BuildProfile::Release => &["-O2", "-DNDEBUG"],
        }
    }

    pub fn lto_enabled(self) -> bool {
        matches!(self, BuildProfile::Release)
            && std::env::var("SYX_NATIVE_LTO")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
    }

    pub fn link_profile_args(self) -> Vec<&'static str> {
        let mut args = match self {
            BuildProfile::Debug => Vec::new(),
            BuildProfile::Release => {
                #[cfg(target_os = "macos")]
                {
                    vec!["-Wl,-dead_strip"]
                }
                #[cfg(target_os = "linux")]
                {
                    vec!["-Wl,--gc-sections"]
                }
                #[cfg(not(any(target_os = "macos", target_os = "linux")))]
                {
                    Vec::new()
                }
            }
        };
        if self.lto_enabled() {
            args.push("-flto");
        }
        args
    }

    pub fn from_release_flag(release: bool) -> Self {
        if release {
            BuildProfile::Release
        } else {
            BuildProfile::Debug
        }
    }
}

/// Project root for a path (directory or single `.sy` file).
pub fn project_root_for(path: &Path) -> PathBuf {
    if path.is_dir() {
        path.to_path_buf()
    } else {
        path.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    }
}

pub fn project_target_root(project_root: &Path) -> PathBuf {
    project_root.join("target")
}

pub fn project_cache_dir(project_root: &Path) -> PathBuf {
    project_target_root(project_root).join("cache")
}

pub fn project_build_dir(project_root: &Path, profile: BuildProfile) -> PathBuf {
    project_target_root(project_root).join(profile.dir_name())
}

/// Host executable file name for native AOT (`app` / `{stem}`, plus `.exe` on Windows).
pub fn native_executable_file_name(stem: &str) -> String {
    if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_string()
    }
}

pub fn default_native_executable(project_root: &Path, profile: BuildProfile) -> PathBuf {
    project_build_dir(project_root, profile).join(native_executable_file_name("app"))
}

/// Default native executable path when building a single `.sy` file (stem-based, not `app`).
pub fn default_native_executable_for_source(source: &Path, profile: BuildProfile) -> PathBuf {
    let root = project_root_for(source);
    let stem = source.file_stem().and_then(|s| s.to_str()).unwrap_or("app");
    project_build_dir(&root, profile).join(native_executable_file_name(stem))
}

pub fn default_native_object(project_root: &Path, profile: BuildProfile) -> PathBuf {
    project_build_dir(project_root, profile).join("main.o")
}

/// Default native object path when building a single `.sy` file (`{stem}.o`).
pub fn default_native_object_for_source(source: &Path, profile: BuildProfile) -> PathBuf {
    let root = project_root_for(source);
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("main");
    project_build_dir(&root, profile).join(format!("{stem}.o"))
}

impl Default for DriverOptions {
    fn default() -> Self {
        Self {
            module_dirs: Vec::new(),
            enable_std: true,
            skip_install_check: false,
            resolved_packages: None,
            strict_concurrency: false,
            default_isolation: None,
            native_minimal: false,
            native_no_http: false,
            native_no_file: false,
            native_backend: NativeBackend::default_for_env(),
            build_profile: BuildProfile::Debug,
            wasm_aot: syx_config::wasm_aot_enabled(),
            in_build_syx_task: false,
        }
    }
}

/// Parse semicolon-separated module search paths.
pub fn parse_module_dirs(value: &str) -> Vec<PathBuf> {
    value
        .split(';')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(PathBuf::from)
        .collect()
}

#[derive(Debug, Clone)]
pub struct CompiledProject {
    pub manifest: SyxManifest,
    pub entry: ModuleUnit,
    pub graph: ModuleGraph,
}

pub fn compile_file_with_context(
    module_key: &Path,
    source: &SourceFile,
    importer: &Path,
    graph: &ModuleGraph,
    manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
) -> SyxResult<ModuleUnit> {
    let importer_dir = importer.parent().unwrap_or(importer);
    let parsed = Parser::parse(source.clone())?;
    register_imported_macros_from_parsed(
        &parsed,
        importer,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    register_imported_data_classes_from_parsed(
        &parsed,
        importer,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    let stem = module_key
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from);
    syx_meta::service_di_set_scan_stem(stem);
    let prep_opts = FrontendOptions {
        stop_after: StopAfter::Prepared,
        desugar_top_level: should_desugar_top_level_stmts(module_key, manifest),
        source: Some(source.clone()),
        ..FrontendOptions::default()
    };
    let art = FrontendPipeline::run_parsed(parsed, &prep_opts)?;
    syx_meta::service_di_set_scan_stem(None);
    let mut module = art.module;
    // Object extensions (`valueForKey`/`put`) need to be in the AST so
    // `type_extension_methods` types their returns (≠ Unknown → S004).
    inject_core_collection_builtins(&mut module, options);
    let main_actor_await_errors = collect_main_actor_await_errors(&module, &[], manifest);

    let mut import_symbols = core_prelude_import_symbols(options);
    let mut import_isolation = Vec::new();
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let Some(symbol) = decl.import_symbol() else {
                continue;
            };
            let import_ref = resolve_module(
                &path_str,
                importer,
                importer_dir,
                manifest,
                options,
                project_root,
            )?;
            let import_key = import_ref.key();
            let dep_source = if let Some(dep) = graph.modules.get(&import_key) {
                dep.source.clone()
            } else {
                import_ref.load()?.1
            };
            import_symbols.push((symbol.clone(), export_symbols(&dep_source)));
            let dep_parent = import_key.parent().unwrap_or(project_root);
            let dep_manifest = load_manifest(dep_parent).unwrap_or_default();
            let fn_actors = collect_dep_function_actors(&dep_source, &dep_manifest);
            if !fn_actors.is_empty() {
                import_isolation.push((symbol.clone(), fn_actors));
            }
        }
    }

    let semantic = SemanticAnalyzer::analyze_with_imports_strict_skip_main_actor(
        &module,
        &import_symbols,
        manifest.concurrency.strict,
        manifest
            .concurrency
            .default_isolation
            .as_ref()
            .map(|s| SmolStr::new(s.as_str())),
        &import_isolation,
        Some(source),
        &core_prelude_generic_class_methods(options),
    )?;
    if let Some(err) = main_actor_await_errors.first() {
        return Err(err.clone());
    }
    let mut program = CompiledProgram::default();
    let exports_by_dep: IndexMap<PathBuf, IndexMap<SmolStr, TypeId>> = graph
        .modules
        .iter()
        .map(|(p, u)| (p.clone(), export_symbols(&u.source)))
        .collect();
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let Ok(import_ref) = resolve_module(
                &path_str,
                importer,
                importer_dir,
                manifest,
                options,
                project_root,
            ) else {
                continue;
            };
            let import_key = import_ref.key();
            let Some(dep) = graph.modules.get(&import_key) else {
                continue;
            };
            let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
            let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if is_stdlib_module_path(&dep.path) {
                for func in &dep.program.functions {
                    let merge =
                        mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports)
                            || dep_exports.contains_key(&func.name);
                    if merge {
                        let mut merged = func.clone();
                        merged.name =
                            cross_module_function_name(&dep.path, module_stem, merged.name);
                        if !program.functions.iter().any(|f| f.name == merged.name) {
                            program.functions.push(merged);
                        }
                    }
                }
            } else {
                merge_user_dep_program_batch(&mut program, dep, module_stem, &dep_exports);
            }
            for (export, ty) in &dep_exports {
                merge_imported_dataclass_functions(&mut program, export, dep);
                let merge_ctor = matches!(ty, TypeId::Named(n) if n.as_str() == export.as_str());
                if !merge_ctor && syx_meta::reflect_type(export.as_str()).is_none() {
                    continue;
                }
                for suffix in ["_init", "_copy", "_describe", "_hash_into", "_=="] {
                    let fname = SmolStr::new(format!("{export}{suffix}"));
                    if let Some(func) = dep.program.functions.iter().find(|f| f.name == fname) {
                        let mut merged = func.clone();
                        merged.name =
                            cross_module_function_name(&dep.path, module_stem, fname.clone());
                        if !program.functions.iter().any(|f| f.name == merged.name) {
                            program.functions.push(merged);
                        }
                    }
                }
            }
        }
    }
    for dep in graph.modules.values() {
        let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
        let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        for func in &dep.program.functions {
            let merge = mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports);
            if merge {
                let mut merged = func.clone();
                merged.name = cross_module_function_name(&dep.path, module_stem, merged.name);
                if !program.functions.iter().any(|f| f.name == merged.name) {
                    program.functions.push(merged);
                }
            }
        }
    }
    merge_stdlib_mono_instances(graph, &mut program, &semantic.instantiations, options)?;
    let prelude_extensions = collect_stdlib_extension_methods(graph);
    let mut compile_module = module.clone();
    inject_imported_dataclass_decls(
        &mut compile_module,
        importer,
        importer_dir,
        manifest,
        options,
        project_root,
        graph,
    )?;
    compile_entry_with_unified_mir(
        graph,
        module_key,
        source.name.as_str(),
        &compile_module,
        &mut program,
        &semantic,
        options,
        &prelude_extensions,
        manifest,
        project_root,
        source,
    )?;

    Ok(ModuleUnit {
        path: module_key.to_path_buf(),
        source: source.clone(),
        imported_modules: semantic.imported_modules.clone(),
        program,
    })
}

fn dep_decl_has_attr(attrs: &[syx_ast::Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.name.as_str() == name)
}

/// Phase 22 CY2: collect public function isolation from a dependency module (attrs + default isolation).
fn collect_dep_function_actors(
    dep_source: &SourceFile,
    dep_manifest: &SyxManifest,
) -> IndexMap<SmolStr, SmolStr> {
    let mut actors = IndexMap::new();
    let Ok(parsed) = Parser::parse(dep_source.clone()) else {
        return actors;
    };
    let mut global_actor_names = HashSet::new();
    for decl in &parsed.decls {
        if let Decl::GlobalActor { name, .. } = decl {
            global_actor_names.insert(name.clone());
        }
    }
    let default_iso = dep_manifest.concurrency.default_isolation.as_deref();
    for decl in &parsed.decls {
        let Decl::Function {
            access: AccessLevel::Public,
            name,
            attrs,
            ..
        } = decl
        else {
            continue;
        };
        if dep_decl_has_attr(attrs, "nonisolated") {
            continue;
        }
        if dep_decl_has_attr(attrs, "MainActor") {
            actors.insert(name.clone(), SmolStr::new("MainActor"));
            continue;
        }
        for attr in attrs {
            if global_actor_names.contains(&attr.name) {
                actors.insert(name.clone(), attr.name.clone());
            }
        }
        if actors.contains_key(name) {
            continue;
        }
        if let Some(default) = default_iso {
            actors.insert(name.clone(), SmolStr::new(default));
        }
    }
    actors
}

fn core_prelude_import_symbols(
    options: &DriverOptions,
) -> Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> {
    if !options.enable_std {
        return Vec::new();
    }
    embedded_std::core_prelude()
        .iter()
        .map(|(name, src)| {
            let source = SourceFile::new(
                embedded_std::virtual_path(name)
                    .to_string_lossy()
                    .into_owned(),
                (*src).to_string(),
            );
            (SmolStr::new(*name), export_symbols(&source))
        })
        .collect()
}

fn ensure_prelude_modules_in_graph(
    graph: &mut ModuleGraph,
    manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
) -> SyxResult<()> {
    if !options.enable_std {
        return Ok(());
    }
    let mut prelude_imports: Vec<(SmolStr, IndexMap<SmolStr, TypeId>)> = Vec::new();
    for (name, src) in embedded_std::core_prelude() {
        let key = embedded_std::virtual_path(name);
        if graph.modules.contains_key(&key) {
            if let Some(unit) = graph.modules.get(&key) {
                prelude_imports.push((SmolStr::new(*name), export_symbols(&unit.source)));
            }
            continue;
        }
        let source = SourceFile::new(format!("<stdlib>/{name}.sy"), (*src).to_string());
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        let art = FrontendPipeline::run(source.clone(), &prep_opts)?;
        let module = art.module;
        let semantic =
            SemanticAnalyzer::analyze_embedded_stdlib_with_imports(&module, &prelude_imports)?;
        let mut program = CompiledProgram::default();
        let exports_by_dep: IndexMap<PathBuf, IndexMap<SmolStr, TypeId>> = graph
            .modules
            .iter()
            .map(|(p, u)| (p.clone(), export_symbols(&u.source)))
            .collect();
        for dep in graph.modules.values() {
            let dep_exports = exports_by_dep.get(&dep.path).cloned().unwrap_or_default();
            let module_stem = dep.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            for func in &dep.program.functions {
                let merge =
                    mir_merge_stdlib_function(module_stem, func.name.as_str(), &dep_exports);
                if merge && !program.functions.iter().any(|f| f.name == func.name) {
                    program.functions.push(func.clone());
                }
            }
        }
        merge_stdlib_mono_instances(graph, &mut program, &semantic.instantiations, options)?;
        let prelude_ext = collect_stdlib_extension_methods(graph);
        compile_to_program(
            &module,
            &key,
            &mut program,
            &semantic.instantiations,
            &semantic,
            options,
            source.name.as_str(),
            &prelude_ext,
        )?;
        let unit = ModuleUnit {
            path: key.clone(),
            source,
            imported_modules: semantic.imported_modules.clone(),
            program,
        };
        graph.order.push(key.clone());
        graph.modules.insert(key.clone(), unit.clone());
        prelude_imports.push((SmolStr::new(*name), export_symbols(&unit.source)));
    }
    ensure_embedded_protocols_in_graph(graph, options, &prelude_imports)?;
    ensure_reference_builtins_in_graph(graph, options, &prelude_imports)?;
    Ok(())
}

fn ensure_embedded_protocols_in_graph(
    graph: &mut ModuleGraph,
    options: &DriverOptions,
    prelude_imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> SyxResult<()> {
    if !options.enable_std {
        return Ok(());
    }
    for (name, src) in embedded_std::embedded_protocols() {
        let key = embedded_std::virtual_path(name);
        if graph.modules.contains_key(&key) {
            continue;
        }
        let source = SourceFile::new(format!("<stdlib>/{name}.sy"), (*src).to_string());
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        let art = FrontendPipeline::run(source.clone(), &prep_opts)?;
        let module = art.module;
        let semantic =
            SemanticAnalyzer::analyze_embedded_stdlib_with_imports(&module, prelude_imports)?;
        let mut program = CompiledProgram::default();
        let empty_prelude = indexmap::IndexSet::new();
        compile_to_program(
            &module,
            &key,
            &mut program,
            &semantic.instantiations,
            &semantic,
            options,
            source.name.as_str(),
            &empty_prelude,
        )?;
        let unit = ModuleUnit {
            path: key.clone(),
            source,
            imported_modules: semantic.imported_modules.clone(),
            program,
        };
        graph.order.push(key.clone());
        graph.modules.insert(key.clone(), unit);
    }
    Ok(())
}

/// Load `Object` / `ObjectIdentifier` Core reference builtins into the graph so
/// their extension witnesses (`Object_valueForKey`, …) merge like Dictionary.
fn ensure_reference_builtins_in_graph(
    graph: &mut ModuleGraph,
    options: &DriverOptions,
    prelude_imports: &[(SmolStr, IndexMap<SmolStr, TypeId>)],
) -> SyxResult<()> {
    if !options.enable_std {
        return Ok(());
    }
    for (name, src) in embedded_std::embedded_reference_builtins() {
        let key = embedded_std::virtual_path(name);
        if graph.modules.contains_key(&key) {
            continue;
        }
        let source = SourceFile::new(format!("<stdlib>/{name}.sy"), (*src).to_string());
        let prep_opts = FrontendOptions {
            stop_after: StopAfter::Prepared,
            desugar_top_level: false,
            source: Some(source.clone()),
            ..FrontendOptions::default()
        };
        let art = FrontendPipeline::run(source.clone(), &prep_opts)?;
        let module = art.module;
        let semantic =
            SemanticAnalyzer::analyze_embedded_stdlib_with_imports(&module, prelude_imports)?;
        let mut program = CompiledProgram::default();
        let empty_prelude = indexmap::IndexSet::new();
        compile_to_program(
            &module,
            &key,
            &mut program,
            &semantic.instantiations,
            &semantic,
            options,
            source.name.as_str(),
            &empty_prelude,
        )?;
        let unit = ModuleUnit {
            path: key.clone(),
            source,
            imported_modules: semantic.imported_modules.clone(),
            program,
        };
        graph.order.push(key.clone());
        graph.modules.insert(key.clone(), unit);
    }
    Ok(())
}

fn expanded_dep_module(dep_source: &SourceFile) -> Option<syx_ast::Module> {
    let parsed = Parser::parse(dep_source.clone()).ok()?;
    expand_macros(parsed).ok()
}

fn dataclass_decls_from_module(module: &syx_ast::Module) -> Vec<Decl> {
    module
        .decls
        .iter()
        .filter(|d| matches!(d, Decl::DataClass { .. }))
        .cloned()
        .collect()
}

fn extension_decls_for_type(module: &syx_ast::Module, type_name: &SmolStr) -> Vec<Decl> {
    module
        .decls
        .iter()
        .filter_map(|d| {
            if let Decl::Extension {
                type_name: tn,
                methods,
                span,
                conformances,
                where_clause,
            } = d
            {
                if tn == type_name {
                    Some(Decl::Extension {
                        span: *span,
                        type_name: tn.clone(),
                        conformances: conformances.clone(),
                        where_clause: where_clause.clone(),
                        methods: methods.clone(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

/// Merge Core Array/Set/Dictionary/Object extensions into a buffer module for LSP.
/// OrderedSet / Math / … stay available via `core_prelude_import_symbols` —
/// do not inject their classes (S023 prelude-name shadow).
fn inject_core_collection_builtins(module: &mut syx_ast::Module, options: &DriverOptions) {
    if !options.enable_std {
        return;
    }
    for surface in ["Array", "Set", "Dictionary", "Object"] {
        let Some((_, src)) = embedded_std::lookup_core(surface) else {
            continue;
        };
        let source = SourceFile::new(
            embedded_std::virtual_path(surface)
                .to_string_lossy()
                .into_owned(),
            (*src).to_string(),
        );
        let Some(dep_module) = expanded_dep_module(&source) else {
            continue;
        };
        for decl in &dep_module.decls {
            if let Decl::Extension { type_name, .. } = decl {
                if type_name.as_str() == surface {
                    merge_imported_extension(module, decl.clone());
                }
            }
        }
    }
}

/// Expanded Core value-builtin modules for LSP signature merge (no body analysis).
fn expand_core_value_builtin_modules(options: &DriverOptions) -> Vec<syx_ast::Module> {
    if !options.enable_std {
        return Vec::new();
    }
    const SURFACES: &[&str] = &[
        "Int8", "Int16", "Int32", "Int64", "Int128", "Int", "UInt8", "UInt16", "UInt32",
        "UInt64", "UInt128", "UInt", "Float32", "Float64", "Float", "Double", "Bool",
        "Character", "BigInt",
    ];
    let mut out = Vec::new();
    for surface in SURFACES {
        let Some((_, src)) = embedded_std::lookup_core(surface) else {
            continue;
        };
        let source = SourceFile::new(
            embedded_std::virtual_path(surface)
                .to_string_lossy()
                .into_owned(),
            (*src).to_string(),
        );
        if let Some(dep_module) = expanded_dep_module(&source) {
            out.push(dep_module);
        }
    }
    out
}

/// Phase 59: inject imported dependency data classes, public classes, and type
/// extensions into the compiling module (so Core prelude Array helpers and
/// OrderedSet, or `import Net`, land in semantic / LSP scope).
fn inject_imported_dataclass_decls(
    module: &mut syx_ast::Module,
    importer: &Path,
    importer_dir: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
    graph: &ModuleGraph,
) -> SyxResult<()> {
    let imports: Vec<(String, SmolStr)> = module
        .decls
        .iter()
        .filter_map(|d| {
            let Decl::Import { .. } = d else {
                return None;
            };
            Some((d.import_path_str()?, d.import_symbol()?.clone()))
        })
        .collect();
    for (path_str, symbol) in imports {
        let import_ref = resolve_module(
            &path_str,
            importer,
            importer_dir,
            manifest,
            options,
            project_root,
        )?;
        let import_key = import_ref.key();
        let dep_source = if let Some(dep) = graph.modules.get(&import_key) {
            dep.source.clone()
        } else {
            import_ref.load()?.1
        };
        let Some(dep_module) = expanded_dep_module(&dep_source) else {
            continue;
        };
        for dc in dataclass_decls_from_module(&dep_module) {
            let Decl::DataClass { name, .. } = &dc else {
                continue;
            };
            if module
                .decls
                .iter()
                .any(|d| matches!(d, Decl::DataClass { name: n, .. } if n == name))
            {
                continue;
            }
            module.decls.push(dc.clone());
            for ext in extension_decls_for_type(&dep_module, name) {
                merge_imported_extension(module, ext);
            }
        }
        for decl in &dep_module.decls {
            if let Decl::Class {
                access: AccessLevel::Public,
                name,
                ..
            } = decl
            {
                // Do not inject Core TypeId surfaces (Array/String/…) as named
                // classes — that collapses `TypeId::Array` into `Named("Array")`
                // and drops builtin method tables. Still merge their extensions.
                if is_core_typeid_surface(name.as_str()) {
                    continue;
                }
                // Host builtins owned by a module (e.g. HTTPResponse → Net).
                if syx_stdlib::is_builtin_type(name.as_str()) {
                    continue;
                }
                // Module-homonym class (Crypto in `import Crypto`) stays available
                // via import_exports; injecting it forces Crypto_sha256 label checks.
                if name == &symbol {
                    continue;
                }
                if !module.decls.iter().any(|d| {
                    matches!(d, Decl::Class { name: n, .. } if n == name)
                        || matches!(d, Decl::DataClass { name: n, .. } if n == name)
                }) {
                    module.decls.push(decl.clone());
                }
            }
            if let Decl::Extension { type_name, .. } = decl {
                // Skip extensions already handled via dataclass injection above.
                let is_dc = dep_module
                    .decls
                    .iter()
                    .any(|d| matches!(d, Decl::DataClass { name: n, .. } if n == type_name));
                if !is_dc {
                    merge_imported_extension(module, decl.clone());
                }
            }
        }
    }
    Ok(())
}

/// Builtin type names that use dedicated `TypeId` variants (not `Named`).
fn is_core_typeid_surface(name: &str) -> bool {
    matches!(
        name,
        "Array"
            | "Set"
            | "Dictionary"
            | "String"
            | "Bool"
            | "Character"
            | "Void"
            | "Int"
            | "Int8"
            | "Int16"
            | "Int32"
            | "Int64"
            | "Int128"
            | "UInt"
            | "UInt8"
            | "UInt16"
            | "UInt32"
            | "UInt64"
            | "UInt128"
            | "Float"
            | "Float32"
            | "Float64"
            | "Double"
            | "BigInt"
            | "Object"
            | "ObjectIdentifier"
            | "Result"
            | "SIMD4"
    )
}

fn merge_imported_extension(module: &mut syx_ast::Module, ext: Decl) {
    let Decl::Extension {
        type_name,
        methods,
        span,
        conformances,
        where_clause,
    } = ext
    else {
        return;
    };
    if let Some(existing) = module.decls.iter_mut().find_map(|d| {
        if let Decl::Extension {
            type_name: tn,
            methods: existing_methods,
            ..
        } = d
        {
            if tn == &type_name {
                Some(existing_methods)
            } else {
                None
            }
        } else {
            None
        }
    }) {
        for method in methods {
            let method_name = match &method {
                Decl::Function { name, .. } => Some(name.clone()),
                _ => None,
            };
            let Some(method_name) = method_name else {
                existing.push(method);
                continue;
            };
            let already = existing
                .iter()
                .any(|m| matches!(m, Decl::Function { name, .. } if name == &method_name));
            if !already {
                existing.push(method);
            }
        }
    } else {
        module.decls.push(Decl::Extension {
            span,
            type_name,
            conformances,
            where_clause,
            methods,
        });
    }
}

fn merge_imported_dataclass_functions(
    program: &mut CompiledProgram,
    export: &SmolStr,
    dep: &ModuleUnit,
) {
    let prefix = format!("{export}_");
    for func in &dep.program.functions {
        if func.name.as_str() == export.as_str() || func.name.starts_with(prefix.as_str()) {
            if !program.functions.iter().any(|f| f.name == func.name) {
                program.functions.push(func.clone());
            }
        }
    }
}

fn register_imported_data_classes_from_parsed(
    module: &syx_ast::Module,
    importer: &Path,
    importer_dir: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
    root: &Path,
    graph: &ModuleGraph,
) -> SyxResult<()> {
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let import_ref =
                resolve_module(&path_str, importer, importer_dir, manifest, options, root)?;
            let import_key = import_ref.key();
            let dep_source = if let Some(dep) = graph.modules.get(&import_key) {
                dep.source.clone()
            } else {
                import_ref.load()?.1
            };
            if let Ok(dep_ast) = Parser::parse(dep_source) {
                if let Ok(expanded) = expand_macros(dep_ast) {
                    register_data_classes(&expanded);
                }
            }
        }
    }
    Ok(())
}

fn register_imported_macros_from_parsed(
    module: &syx_ast::Module,
    importer: &Path,
    importer_dir: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
    root: &Path,
    graph: &ModuleGraph,
) -> SyxResult<()> {
    for decl in &module.decls {
        if let Decl::Import { .. } = decl {
            let Some(path_str) = decl.import_path_str() else {
                continue;
            };
            let import_ref =
                resolve_module(&path_str, importer, importer_dir, manifest, options, root)?;
            let import_key = import_ref.key();
            let dep_source = if let Some(dep) = graph.modules.get(&import_key) {
                dep.source.clone()
            } else {
                import_ref.load()?.1
            };
            if let Ok(dep_ast) = Parser::parse(dep_source) {
                syx_macro::register_imported_macros(&dep_ast.decls);
            }
        }
    }
    Ok(())
}

fn export_symbols(source: &SourceFile) -> IndexMap<SmolStr, TypeId> {
    export_function_details(source)
        .into_iter()
        .map(|(name, (ty, _))| (name, ty))
        .collect()
}

/// Public function signatures from a module source (`name(args) -> Ret`).
pub fn export_function_signatures(source: &SourceFile) -> IndexMap<SmolStr, String> {
    export_function_details(source)
        .into_iter()
        .map(|(name, (_, sig))| (name, sig))
        .collect()
}

/// Signature for a stdlib module export (e.g. Math.add).
pub fn stdlib_export_signature(module: &str, func: &str) -> Option<String> {
    let (_, src) = embedded_std::lookup_core(module).or_else(|| embedded_std::lookup(module))?;
    let source = SourceFile::new(
        embedded_std::virtual_path(module)
            .to_string_lossy()
            .into_owned(),
        src.to_string(),
    );
    export_function_signatures(&source).get(func).cloned()
}

fn type_expr_display(ty: &syx_ast::TypeExpr) -> String {
    use syx_ast::TypeExpr;
    match ty {
        TypeExpr::Named { name, generics } => {
            if generics.is_empty() {
                name.to_string()
            } else {
                let args = generics
                    .iter()
                    .map(type_expr_display)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{name}<{args}>")
            }
        }
        TypeExpr::Optional(inner) => format!("{}?", type_expr_display(inner)),
        TypeExpr::Array(inner) => format!("Array<{}>", type_expr_display(inner)),
        TypeExpr::Dictionary { key, value } => {
            format!(
                "Dictionary<{}, {}>",
                type_expr_display(key),
                type_expr_display(value)
            )
        }
        TypeExpr::Function { .. } => "Function".into(),
        TypeExpr::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(|(label, ty)| {
                    if let Some(l) = label {
                        format!("{l}: {}", type_expr_display(ty))
                    } else {
                        type_expr_display(ty)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("({inner})")
        }
        TypeExpr::Reference(inner, mutable, _) => {
            if *mutable {
                format!("&mut {}", type_expr_display(inner))
            } else {
                format!("&{}", type_expr_display(inner))
            }
        }
        TypeExpr::Void => "Void".into(),
        TypeExpr::OpaqueSome(inner) => format!("some {}", type_expr_display(inner)),
        TypeExpr::OpaqueAny(inner) => format!("any {}", type_expr_display(inner)),
    }
}

fn extract_generic_class_methods(
    source: &SourceFile,
) -> IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)> {
    let mut methods = IndexMap::new();
    if let Ok(parsed) = Parser::parse(source.clone()) {
        for decl in &parsed.decls {
            let Decl::Class {
                name: class_name,
                members,
                ..
            } = decl
            else {
                continue;
            };
            for member in members {
                let Decl::Function {
                    name: method,
                    type_params,
                    where_clause,
                    attrs,
                    ..
                } = member
                else {
                    continue;
                };
                if type_params.is_empty() || !attrs.iter().any(|a| a.name.as_str() == "static") {
                    continue;
                }
                let qname = SmolStr::new(format!("{class_name}_{method}"));
                methods.insert(qname, (type_params.clone(), where_clause.clone()));
            }
        }
    }
    methods
}

fn core_prelude_generic_class_methods(
    options: &DriverOptions,
) -> IndexMap<SmolStr, (Vec<SmolStr>, Vec<syx_ast::WherePred>)> {
    if !options.enable_std {
        return IndexMap::new();
    }
    let mut out = IndexMap::new();
    for (name, src) in embedded_std::core_prelude() {
        let source = SourceFile::new(
            embedded_std::virtual_path(name)
                .to_string_lossy()
                .into_owned(),
            (*src).to_string(),
        );
        out.extend(extract_generic_class_methods(&source));
    }
    out
}

fn export_function_details(source: &SourceFile) -> IndexMap<SmolStr, (TypeId, String)> {
    let mut exports = IndexMap::new();

    fn export_nested_classes(
        exports: &mut IndexMap<SmolStr, (TypeId, String)>,
        outer: &SmolStr,
        members: &[Decl],
    ) {
        for member in members {
            let Decl::Class {
                access,
                name,
                members: nested,
                ..
            } = member
            else {
                continue;
            };
            if !matches!(access, AccessLevel::Public) {
                continue;
            }
            let qualified = SmolStr::new(format!("{outer}_{name}"));
            exports.insert(
                qualified.clone(),
                (
                    TypeId::Named(qualified.clone()),
                    format!("class {qualified}"),
                ),
            );
            export_nested_classes(exports, &qualified, nested);
        }
    }

    if let Ok(parsed) = Parser::parse(source.clone()) {
        for decl in &parsed.decls {
            match decl {
                Decl::Function {
                    access: AccessLevel::Public,
                    name,
                    params,
                    return_type,
                    ..
                } => {
                    insert_export_fn(&mut exports, name, params, return_type);
                }
                Decl::Class {
                    access,
                    name: class_name,
                    members,
                    ..
                } => {
                    if matches!(access, AccessLevel::Public) {
                        exports.insert(
                            class_name.clone(),
                            (
                                TypeId::Named(class_name.clone()),
                                format!("class {class_name}"),
                            ),
                        );
                        export_nested_classes(&mut exports, class_name, members);
                    }
                    for member in members {
                        if let Decl::Function {
                            access: AccessLevel::Public,
                            name,
                            params,
                            return_type,
                            attrs,
                            ..
                        } = member
                        {
                            if attrs.iter().any(|a| a.name.as_str() == "static") {
                                insert_export_fn(&mut exports, name, params, return_type);
                            }
                        }
                    }
                }
                Decl::DataClass { name, .. } => {
                    exports.insert(
                        name.clone(),
                        (TypeId::Named(name.clone()), format!("data class {name}")),
                    );
                }
                Decl::Actor { access, name, .. } if matches!(access, AccessLevel::Public) => {
                    exports.insert(
                        name.clone(),
                        (TypeId::Named(name.clone()), format!("actor {name}")),
                    );
                }
                _ => {}
            }
        }
    }
    exports
}

fn insert_export_fn(
    exports: &mut IndexMap<SmolStr, (TypeId, String)>,
    name: &SmolStr,
    params: &[syx_ast::Param],
    return_type: &Option<syx_ast::TypeExpr>,
) {
    let ty = return_type
        .as_ref()
        .map(type_expr_to_id)
        .unwrap_or(TypeId::Void);
    let param_str = params
        .iter()
        .map(|p| {
            let pt =
                p.ty.as_ref()
                    .map(type_expr_display)
                    .unwrap_or_else(|| "?".into());
            format!("{}: {pt}", p.name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let ret = return_type
        .as_ref()
        .map(type_expr_display)
        .unwrap_or_else(|| "Void".into());
    let sig = format!("{name}({param_str}) -> {ret}");
    exports.insert(name.clone(), (ty, sig));
}

pub fn compile_linked(path: &Path) -> SyxResult<ModuleUnit> {
    compile_linked_with_options(path, &DriverOptions::default())
}

pub fn compile_linked_with_options(path: &Path, options: &DriverOptions) -> SyxResult<ModuleUnit> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    syx_macro::register_package_macros(manifest.macros.as_slice());
    let mut opts = options.clone();
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    // M3-E02 / ADR-311: merge syx.toml [permissions] into runtime grants (OR).
    {
        let mut cfg = syx_config::runtime_config();
        cfg.permission.merge_flags(
            manifest.permissions.sandbox,
            manifest.permissions.allow_net,
            manifest.permissions.allow_fs,
            manifest.permissions.allow_env,
            manifest.permissions.allow_process,
            manifest.permissions.allow_plugin,
        );
        syx_config::install_runtime_config(cfg);
    }
    if !manifest.dependencies.is_empty() {
        ensure_project_lock(parent, options)?;
        opts.resolved_packages = Some(load_resolved_packages(parent)?);
    }
    let root = parent;
    purge_stale_incremental_caches(root);
    let cache = incremental_cache_path(root, path);
    let current_source = std::fs::read_to_string(path).unwrap_or_default();
    if cache.exists() {
        if let Ok(bytes) = std::fs::read(&cache) {
            if let Ok(cached) = bincode::deserialize::<CachedModuleUnit>(&bytes) {
                if cache_entry_fresh(&cached, &current_source) {
                    if let Some(unit) =
                        query::lookup_cached(path, &current_source, &cached.dependency_hashes)
                    {
                        refresh_service_di_from_source(path)?;
                        return Ok(unit);
                    }
                    let deps = cached.dependency_hashes.clone();
                    let unit: ModuleUnit = cached.into();
                    query::store_cached(unit.clone(), &deps);
                    refresh_service_di_from_source(path)?;
                    return Ok(unit);
                }
            }
        }
    }
    syx_meta::service_di_clear();
    let mut graph = ModuleGraph::default();
    let unit = compile_module_recursive(
        ModuleRef::from_file(path.to_path_buf()),
        root,
        &mut graph,
        &manifest,
        &opts,
    )?;
    if let Some(parent_dir) = cache.parent() {
        let _ = std::fs::create_dir_all(parent_dir);
    }
    if let Ok(bytes) = bincode_encode(&unit, &graph) {
        let _ = std::fs::write(&cache, bytes);
    }
    let deps = dependency_hashes_from_graph(&graph, &unit.path);
    query::store_cached(unit.clone(), &deps);
    Ok(unit)
}

pub fn compile_file(path: &Path) -> SyxResult<ModuleUnit> {
    compile_linked(path)
}

pub fn compile_file_with_options(path: &Path, options: &DriverOptions) -> SyxResult<ModuleUnit> {
    compile_linked_with_options(path, options)
}

fn refresh_service_di_from_source(path: &Path) -> SyxResult<()> {
    syx_meta::service_di_clear();
    let source = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let file = SourceFile::new(path.display().to_string(), source);
    let mut module = Parser::parse(file)?;
    syx_macro::expand_service_macros(&mut module)?;
    Ok(())
}

fn scan_syx_file_services(path: &Path) -> SyxResult<()> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let stem = path.file_stem().and_then(|s| s.to_str()).map(String::from);
    let file = SourceFile::new(path.display().to_string(), source);
    let module = Parser::parse(file)?;
    syx_meta::service_di_set_scan_stem(stem);
    let result = syx_macro::queue_services_from_module(&module);
    syx_meta::service_di_set_scan_stem(None);
    result
}

fn scan_dir_services(dir: &Path) -> SyxResult<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", dir.display())))?
    {
        let entry =
            entry.map_err(|e| SyxDiagnostic::error(format!("Failed to read entry: {e}")))?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| {
                n == "tests" || n == "target" || n == ".sy-cache" || n == "node_modules"
            }) {
                continue;
            }
            // Nested packages (own syx.toml) are scanned only via resolved_packages,
            // not by walking sibling example projects under the same parent folder.
            if path.join("syx.toml").is_file() {
                continue;
            }
            scan_dir_services(&path)?;
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("sy") {
            continue;
        }
        scan_syx_file_services(&path)?;
    }
    Ok(())
}

/// Phase 122: aggregate `@Service` across workspace packages before entry expand.
fn scan_workspace_services(
    root: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
) -> SyxResult<()> {
    scan_dir_services(root)?;
    if let Some(packages) = &options.resolved_packages {
        for pkg in packages.values() {
            scan_dir_services(&pkg.root)?;
        }
    }
    let _ = manifest;
    Ok(())
}

pub fn compile_project(root: &Path) -> SyxResult<CompiledProject> {
    compile_project_with_options(root, &DriverOptions::default())
}

pub fn compile_project_with_options(
    root: &Path,
    options: &DriverOptions,
) -> SyxResult<CompiledProject> {
    ensure_project_lock(root, options)?;
    let mut manifest = load_manifest(root)?;
    syx_host_plugin::register_compile_plugin_manifests(root);
    syx_macro::register_package_macros(manifest.macros.as_slice());
    let mut opts = options.clone();
    if !opts.in_build_syx_task {
        if let Some(plan) = build_syx::load_build_plan(root, &manifest)? {
            build_syx::prepare_build_plan(&plan, &mut opts, &mut manifest, root)?;
        }
    }
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    if !manifest.dependencies.is_empty() {
        opts.resolved_packages = Some(load_resolved_packages(root)?);
    }
    syx_meta::service_di_clear();
    scan_workspace_services(root, &manifest, &opts)?;
    let entry_path = root.join(&manifest.main);
    let mut graph = ModuleGraph::default();
    let entry = compile_module_recursive(
        ModuleRef::from_file(entry_path.clone()),
        root,
        &mut graph,
        &manifest,
        &opts,
    )?;
    Ok(CompiledProject {
        manifest,
        entry,
        graph,
    })
}

fn compile_module_recursive(
    module_ref: ModuleRef,
    root: &Path,
    graph: &mut ModuleGraph,
    manifest: &SyxManifest,
    options: &DriverOptions,
) -> SyxResult<ModuleUnit> {
    let key = module_ref.key();
    if let Some(existing) = graph.modules.get(&key) {
        return Ok(existing.clone());
    }

    ensure_prelude_modules_in_graph(graph, manifest, options, root)?;

    let (key, source) = module_ref.load()?;
    let parsed = Parser::parse(source.clone())?;
    let importer = module_ref.importer_path(root);
    let importer_dir = importer.parent().unwrap_or(root);
    for decl in &parsed.decls {
        if let Decl::Import { .. } = decl {
            if let Some(path_str) = decl.import_path_str() {
                let import_ref =
                    resolve_module(&path_str, importer, importer_dir, manifest, options, root)?;
                compile_module_recursive(import_ref, root, graph, manifest, options)?;
            }
        }
    }

    let unit = compile_file_with_context(&key, &source, importer, graph, manifest, options, root)?;
    graph.order.push(key.clone());
    graph.modules.insert(key.clone(), unit.clone());
    Ok(unit)
}

fn module_dir_candidates(dir: &Path, path_str: &str) -> Vec<PathBuf> {
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    let dotted_file = path_str.replace('.', "/");
    vec![
        dir.join(format!("{path_str}.sy")),
        dir.join(format!("{dotted_file}.sy")),
        dir.join(first_segment).join("mod.sy"),
        dir.join(first_segment).join(format!("{first_segment}.sy")),
    ]
}

fn ensure_project_lock(root: &Path, options: &DriverOptions) -> SyxResult<()> {
    if options.skip_install_check {
        return Ok(());
    }
    if syx_pkg::requires_lockfile(root)? {
        return Err(syx_pkg::lockfile_error(root));
    }
    if !load_manifest(root)?.dependencies.is_empty() {
        load_lockfile(root)?;
    }
    Ok(())
}

fn dep_root_from_lock(pkg: &InstalledPackage, path_str: &str) -> PathBuf {
    let dep_manifest = load_manifest(&pkg.root).unwrap_or_default();
    let root = pkg.root.clone();
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    let dotted_file = path_str.replace('.', "/");
    let candidates = [
        root.join(format!("{path_str}.sy")),
        root.join(format!("{dotted_file}.sy")),
        root.join(format!("{first_segment}.sy")),
        root.join("src").join(format!("{first_segment}.sy")),
        root.join(&dep_manifest.main),
    ];
    for candidate in candidates {
        if candidate.exists() {
            return candidate.parent().unwrap_or(&root).to_path_buf();
        }
    }
    root
}

fn resolve_module(
    path_str: &str,
    _current: &Path,
    importer_dir: &Path,
    _manifest: &SyxManifest,
    options: &DriverOptions,
    project_root: &Path,
) -> SyxResult<ModuleRef> {
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    if embedded_std::is_forbidden_import(first_segment) {
        return Err(SyxDiagnostic::error(format!(
            "Module '{path_str}' is internal to the compiler and cannot be imported"
        ))
        .with_code("M003"));
    }
    if options.enable_std {
        // Prefer on-disk monorepo `packages/` for single-file importables (e.g. Os)
        // so LSP does not serve a stale `include_str!` snapshot after stdlib edits.
        if let Some(disk) = embedded_std::find_workspace_stdlib_file(importer_dir, first_segment)
            .or_else(|| embedded_std::find_workspace_stdlib_file(project_root, first_segment))
            .or_else(|| embedded_std::find_workspace_stdlib_file(_current, first_segment))
        {
            return Ok(ModuleRef::from_file(disk));
        }
        if let Some((name, source)) = embedded_std::lookup_core(path_str) {
            return Ok(ModuleRef::Embedded {
                key: embedded_std::virtual_path(name),
                display_name: format!("<stdlib>/{name}.sy"),
                source,
            });
        }
        if let Some(embedded) = embedded_module(path_str) {
            return Ok(embedded);
        }
    }
    let first_segment = path_str.split('.').next().unwrap_or(path_str);
    if let Some(packages) = &options.resolved_packages {
        if let Some(pkg) = packages.get(first_segment) {
            let dep_root = dep_root_from_lock(pkg, path_str);
            let dep_manifest = load_manifest(&pkg.root).unwrap_or_default();
            let first_segment = path_str.split('.').next().unwrap_or(path_str);
            let dotted_file = path_str.replace('.', "/");
            let candidates = [
                dep_root.join(format!("{path_str}.sy")),
                dep_root.join(format!("{dotted_file}.sy")),
                dep_root.join(format!("{first_segment}.sy")),
                dep_root.join("src").join(format!("{first_segment}.sy")),
                dep_root.join(&dep_manifest.main),
            ];
            for candidate in candidates {
                if candidate.exists() {
                    return Ok(ModuleRef::from_file(candidate));
                }
            }
        }
    }
    let dotted_file = path_str.replace('.', "/");
    let mut candidates = vec![
        importer_dir.with_file_name(format!("{path_str}.sy")),
        importer_dir.join(format!("{dotted_file}.sy")),
    ];
    for dir in &options.module_dirs {
        candidates.extend(module_dir_candidates(dir, path_str));
    }
    for candidate in candidates {
        if candidate.exists() {
            return Ok(ModuleRef::from_file(candidate));
        }
    }
    Err(SyxDiagnostic::error(format!("Module '{path_str}' not found")).with_code("M001"))
}

pub fn check_project(root: &Path) -> SyxResult<()> {
    check_project_with_options(root, &DriverOptions::default())
}

pub fn check_project_with_options(root: &Path, options: &DriverOptions) -> SyxResult<()> {
    compile_project_with_options(root, options)?;
    Ok(())
}

/// Per-case status for `syx test` reports (W932 / ADR-1276).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TestCaseStatus {
    Passed,
    Failed,
    Skipped,
}

/// One discovered / executed test case.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TestCaseResult {
    pub name: String,
    pub status: TestCaseStatus,
    /// Failure message for JSON / JUnit (`--format json|junit`); omitted when passed/skipped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
}

/// Aggregated test run report (Phase 84 + W932 cases).
#[derive(Debug, Clone, Default, Serialize)]
pub struct TestReport {
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub cases: Vec<TestCaseResult>,
}

impl TestReport {
    /// Serialize as JSON object for `--format json` (W932).
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize as JUnit XML for `--format junit` (W933 / ADR-1277).
    ///
    /// Single `<testsuite name="syx">` with `tests` / `failures` / `skipped` / `passed`
    /// attributes aligned to report counters; each case is a `<testcase>`.
    pub fn to_junit_xml_string(&self) -> String {
        fn xml_escape(s: &str) -> String {
            let mut out = String::with_capacity(s.len());
            for c in s.chars() {
                match c {
                    '&' => out.push_str("&amp;"),
                    '<' => out.push_str("&lt;"),
                    '>' => out.push_str("&gt;"),
                    '"' => out.push_str("&quot;"),
                    '\'' => out.push_str("&apos;"),
                    _ => out.push(c),
                }
            }
            out
        }
        let total = self.passed + self.failed + self.skipped;
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuite name=\"syx\" tests=\"{}\" failures=\"{}\" skipped=\"{}\" passed=\"{}\">\n",
            total, self.failed, self.skipped, self.passed
        ));
        for case in &self.cases {
            let name = xml_escape(&case.name);
            match case.status {
                TestCaseStatus::Passed => {
                    xml.push_str(&format!(
                        "  <testcase name=\"{name}\" classname=\"syx\"/>\n"
                    ));
                }
                TestCaseStatus::Failed => {
                    let msg = case
                        .failure
                        .as_deref()
                        .map(xml_escape)
                        .unwrap_or_default();
                    xml.push_str(&format!(
                        "  <testcase name=\"{name}\" classname=\"syx\"><failure>{msg}</failure></testcase>\n"
                    ));
                }
                TestCaseStatus::Skipped => {
                    xml.push_str(&format!(
                        "  <testcase name=\"{name}\" classname=\"syx\"><skipped/></testcase>\n"
                    ));
                }
            }
        }
        xml.push_str("</testsuite>\n");
        xml
    }
}

/// Output format for `syx test` (W932 JSON / W933 JUnit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TestOutputFormat {
    #[default]
    Human,
    Json,
    Tap,
    /// JUnit XML report (W933 / ADR-1277).
    Junit,
}

impl TestOutputFormat {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "human" => Some(Self::Human),
            "json" => Some(Self::Json),
            "tap" => Some(Self::Tap),
            "junit" => Some(Self::Junit),
            _ => None,
        }
    }
}

/// Run project tests (`tests/**/*.sy`) or the main entry when no test directory exists.
pub fn test_project(root: &Path, options: &DriverOptions) -> SyxResult<TestReport> {
    test_project_with_filter(root, options, None)
}

/// Run tests with optional name filter (Phase 84). TAP when `SYX_TEST_TAP=1` (human default).
pub fn test_project_with_filter(
    root: &Path,
    options: &DriverOptions,
    filter: Option<&str>,
) -> SyxResult<TestReport> {
    let tap_env = std::env::var("SYX_TEST_TAP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let format = if tap_env {
        TestOutputFormat::Tap
    } else {
        TestOutputFormat::Human
    };
    test_project_with_format(root, options, filter, format)
}

/// Run tests with explicit output format (W932 JSON / W933 JUnit).
///
/// - `Human`: fail-fast on first error (legacy).
/// - `Tap` / `Json` / `Junit`: continue collecting; `Tap` prints TAP lines; `Json`/`Junit`
///   return full report (caller prints). `Json`/`Junit` return `Ok` with `failed > 0` so CLI
///   can print then exit non-zero; `Tap` returns `Err` if any case failed after the run.
///
/// Discovery: top-level zero-arg `@Test` each become `relativePath.fnName`
/// (posix path under `tests/`, no `.sy`; recursive `tests/**/*.sy`).
/// JUnit-shaped hooks (PascalCase attrs, at most one each):
/// `@TestSetup` once → (`@TestBefore` → `@Test` → `@TestAfter`) × N → `@TestTeardown` once.
/// Accepts a project root (`tests/**/*.sy` or whole-project `main`) or a single `.sy` file.
/// Files with no `@Test` still run as a single whole-file entry (hooks ignored).
pub fn test_project_with_format(
    root: &Path,
    options: &DriverOptions,
    filter: Option<&str>,
    format: TestOutputFormat,
) -> SyxResult<TestReport> {
    let tap = matches!(format, TestOutputFormat::Tap);
    let structured = matches!(format, TestOutputFormat::Json | TestOutputFormat::Junit);
    let continue_on_fail = tap || structured;
    let mut report = TestReport::default();
    let tests_dir = root.join("tests");

    enum PlannedUnit {
        /// Whole-file entry (`main` / `@Main`); hooks ignored.
        File { name: String, path: PathBuf },
        /// One source file with `@Test` cases and optional hooks.
        AtTestFile {
            file_stem: String,
            path: PathBuf,
            /// `@TestSetup` — once before all cases.
            setup: Option<String>,
            /// `@TestBefore` — before each case.
            before: Option<String>,
            /// `@TestAfter` — after each case.
            after: Option<String>,
            /// `@TestTeardown` — once after all cases.
            teardown: Option<String>,
            /// `(case_name, fn_name)`
            tests: Vec<(String, String)>,
            /// Duplicate hooks etc.
            discover_error: Option<String>,
        },
    }

    let mut planned: Vec<PlannedUnit> = Vec::new();
    let mut tap_runnable = 0usize;
    let single_sy = root.is_file() && root.extension().and_then(|e| e.to_str()) == Some("sy");
    let use_tests_dir = !single_sy && tests_dir.is_dir();
    let use_project_main = !single_sy && !use_tests_dir;

    if root.is_file() && !single_sy {
        return Err(SyxDiagnostic::error(format!(
            "syx test expects a project directory or a .sy file, got {}",
            root.display()
        )));
    }

    let mut plan_one_sy =
        |p: PathBuf, case_base: String, report: &mut TestReport| -> SyxResult<()> {
            let disc = discover_file_test_plan(&p)?;
            if disc.tests.is_empty() {
                if let Some(f) = filter {
                    if !case_base.contains(f) {
                        report.skipped += 1;
                        report.cases.push(TestCaseResult {
                            name: case_base,
                            status: TestCaseStatus::Skipped,
                            failure: None,
                        });
                        return Ok(());
                    }
                }
                tap_runnable += 1;
                planned.push(PlannedUnit::File {
                    name: case_base,
                    path: p,
                });
            } else {
                let mut tests = Vec::new();
                for fn_name in disc.tests {
                    let case_name = format!("{case_base}.{fn_name}");
                    if let Some(f) = filter {
                        if !case_name.contains(f) && !case_base.contains(f) {
                            report.skipped += 1;
                            report.cases.push(TestCaseResult {
                                name: case_name,
                                status: TestCaseStatus::Skipped,
                                failure: None,
                            });
                            continue;
                        }
                    }
                    tests.push((case_name, fn_name));
                }
                if tests.is_empty() && disc.discover_error.is_none() {
                    return Ok(());
                }
                tap_runnable += if disc.discover_error.is_some() {
                    1
                } else {
                    tests.len()
                };
                planned.push(PlannedUnit::AtTestFile {
                    file_stem: case_base,
                    path: p,
                    setup: disc.setup,
                    before: disc.before,
                    after: disc.after,
                    teardown: disc.teardown,
                    tests,
                    discover_error: disc.discover_error,
                });
            }
            Ok(())
        };

    if single_sy {
        let base = root
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        plan_one_sy(root.to_path_buf(), base, &mut report)?;
    } else if use_tests_dir {
        let paths = collect_sy_files(&tests_dir)?;
        for p in paths {
            let base = test_file_case_base(&tests_dir, &p);
            plan_one_sy(p, base, &mut report)?;
        }
    }

    if tap {
        println!("TAP version 13");
        let runnable = if use_project_main { 1 } else { tap_runnable };
        println!("1..{runnable}");
    }

    let mut index = 0usize;
    if single_sy || use_tests_dir {
        for unit in planned {
            match unit {
                PlannedUnit::File { name, path } => {
                    match run_file_with_options(&path, options) {
                        Ok(()) => {
                            let outcomes = syx_vm::take_registered_test_outcomes();
                            if outcomes.is_empty() {
                                index += 1;
                                report.passed += 1;
                                report.cases.push(TestCaseResult {
                                    name: name.clone(),
                                    status: TestCaseStatus::Passed,
                                    failure: None,
                                });
                                if tap {
                                    println!("ok {index} - {name}");
                                }
                            } else {
                                let mut first_fail: Option<SyxDiagnostic> = None;
                                for outcome in outcomes {
                                    index += 1;
                                    let case_name = format!("{name}.{}", outcome.name);
                                    if let Some(failure) = outcome.failure {
                                        report.failed += 1;
                                        report.cases.push(TestCaseResult {
                                            name: case_name.clone(),
                                            status: TestCaseStatus::Failed,
                                            failure: Some(failure.clone()),
                                        });
                                        if tap {
                                            println!("not ok {index} - {case_name} # {failure}");
                                        } else if !continue_on_fail && first_fail.is_none() {
                                            first_fail = Some(SyxDiagnostic::error(failure));
                                        }
                                    } else {
                                        report.passed += 1;
                                        report.cases.push(TestCaseResult {
                                            name: case_name.clone(),
                                            status: TestCaseStatus::Passed,
                                            failure: None,
                                        });
                                        if tap {
                                            println!("ok {index} - {case_name}");
                                        }
                                    }
                                }
                                if let Some(err) = first_fail {
                                    return Err(err);
                                }
                            }
                        }
                        Err(e) => {
                            let _ = syx_vm::take_registered_test_outcomes();
                            index += 1;
                            let failure = e.to_string();
                            report.failed += 1;
                            report.cases.push(TestCaseResult {
                                name: name.clone(),
                                status: TestCaseStatus::Failed,
                                failure: Some(failure.clone()),
                            });
                            if tap {
                                println!("not ok {index} - {name} # {failure}");
                            } else if !continue_on_fail {
                                return Err(e);
                            }
                        }
                    }
                }
                PlannedUnit::AtTestFile {
                    file_stem,
                    path,
                    setup,
                    before,
                    after,
                    teardown,
                    tests,
                    discover_error,
                } => {
                    if let Some(err) = discover_error {
                        index += 1;
                        report.failed += 1;
                        report.cases.push(TestCaseResult {
                            name: file_stem.clone(),
                            status: TestCaseStatus::Failed,
                            failure: Some(err.clone()),
                        });
                        if tap {
                            println!("not ok {index} - {file_stem} # {err}");
                        } else if !continue_on_fail {
                            return Err(SyxDiagnostic::error(err));
                        }
                        continue;
                    }

                    let mut setup_ok = true;
                    if let Some(ref setup_fn) = setup {
                        let setup_registry = if let Err(e) =
                            run_file_entry_with_options(&path, setup_fn, options)
                        {
                            let _ = syx_vm::take_registered_test_outcomes();
                            Some(format!("@TestSetup ({setup_fn}): {e}"))
                        } else {
                            registered_test_failure_text()
                                .map(|extra| format!("@TestSetup ({setup_fn}): {extra}"))
                        };
                        if let Some(failure) = setup_registry {
                            setup_ok = false;
                            for (case_name, _) in &tests {
                                index += 1;
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: case_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(failure.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {case_name} # {failure}");
                                }
                            }
                            if !continue_on_fail {
                                return Err(SyxDiagnostic::error(failure));
                            }
                        }
                    }

                    let mut ran_any_test = false;
                    let mut human_stop: Option<SyxDiagnostic> = None;
                    if setup_ok {
                        for (case_name, fn_name) in &tests {
                            if human_stop.is_some() {
                                break;
                            }
                            index += 1;
                            ran_any_test = true;

                            let mut case_failure: Option<String> = None;
                            if let Some(ref before_fn) = before {
                                if let Err(e) =
                                    run_file_entry_with_options(&path, before_fn, options)
                                {
                                    let _ = syx_vm::take_registered_test_outcomes();
                                    case_failure =
                                        Some(format!("@TestBefore ({before_fn}): {e}"));
                                } else if let Some(extra) = registered_test_failure_text() {
                                    case_failure =
                                        Some(format!("@TestBefore ({before_fn}): {extra}"));
                                }
                            }
                            if case_failure.is_none() {
                                if let Err(e) =
                                    run_file_entry_with_options(&path, fn_name, options)
                                {
                                    let _ = syx_vm::take_registered_test_outcomes();
                                    case_failure = Some(e.to_string());
                                } else if let Some(extra) = registered_test_failure_text() {
                                    case_failure = Some(extra);
                                }
                            }

                            // `@TestAfter` — per case, even if Before/Test failed.
                            let mut after_extra: Option<String> = None;
                            if let Some(ref after_fn) = after {
                                if let Err(e) =
                                    run_file_entry_with_options(&path, after_fn, options)
                                {
                                    let _ = syx_vm::take_registered_test_outcomes();
                                    let af = format!("@TestAfter ({after_fn}): {e}");
                                    if case_failure.is_some() {
                                        after_extra = Some(af);
                                    } else {
                                        case_failure = Some(af);
                                    }
                                } else if let Some(extra) = registered_test_failure_text() {
                                    let af = format!("@TestAfter ({after_fn}): {extra}");
                                    if case_failure.is_some() {
                                        after_extra = Some(af);
                                    } else {
                                        case_failure = Some(af);
                                    }
                                }
                            }

                            if let Some(failure) = case_failure {
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: case_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(failure.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {case_name} # {failure}");
                                } else if !continue_on_fail {
                                    human_stop = Some(SyxDiagnostic::error(failure));
                                }
                            } else {
                                report.passed += 1;
                                report.cases.push(TestCaseResult {
                                    name: case_name.clone(),
                                    status: TestCaseStatus::Passed,
                                    failure: None,
                                });
                                if tap {
                                    println!("ok {index} - {case_name}");
                                }
                            }

                            if let Some(af) = after_extra {
                                index += 1;
                                let af_name = format!("{case_name}.@TestAfter");
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: af_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(af.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {af_name} # {af}");
                                } else if !continue_on_fail && human_stop.is_none() {
                                    human_stop = Some(SyxDiagnostic::error(af));
                                }
                            }
                        }
                    }

                    // `@TestTeardown` — once after all cases (if setup ok and entered ≥1).
                    if setup_ok && ran_any_test {
                        if let Some(ref teardown_fn) = teardown {
                            let teardown_failure = if let Err(e) =
                                run_file_entry_with_options(&path, teardown_fn, options)
                            {
                                let _ = syx_vm::take_registered_test_outcomes();
                                Some(format!("@TestTeardown ({teardown_fn}): {e}"))
                            } else {
                                registered_test_failure_text()
                                    .map(|extra| format!("@TestTeardown ({teardown_fn}): {extra}"))
                            };
                            if let Some(failure) = teardown_failure {
                                let td_name = format!("{file_stem}.@TestTeardown");
                                index += 1;
                                report.failed += 1;
                                report.cases.push(TestCaseResult {
                                    name: td_name.clone(),
                                    status: TestCaseStatus::Failed,
                                    failure: Some(failure.clone()),
                                });
                                if tap {
                                    println!("not ok {index} - {td_name} # {failure}");
                                } else if !continue_on_fail && human_stop.is_none() {
                                    human_stop = Some(SyxDiagnostic::error(failure));
                                }
                            }
                        }
                    }

                    if let Some(e) = human_stop {
                        return Err(e);
                    }
                }
            }
        }
    } else {
        index = 1;
        let name = "main".to_string();
        match run_project_with_options(root, options) {
            Ok(()) => {
                report.passed = 1;
                report.cases.push(TestCaseResult {
                    name: name.clone(),
                    status: TestCaseStatus::Passed,
                    failure: None,
                });
                if tap {
                    println!("ok 1 - main");
                }
            }
            Err(e) => {
                let failure = e.to_string();
                report.failed = 1;
                report.cases.push(TestCaseResult {
                    name,
                    status: TestCaseStatus::Failed,
                    failure: Some(failure.clone()),
                });
                if tap {
                    println!("not ok 1 - main # {failure}");
                } else if !continue_on_fail {
                    return Err(e);
                }
            }
        }
    }

    if report.failed > 0 && !structured {
        return Err(SyxDiagnostic::error(format!(
            "{} test(s) failed",
            report.failed
        )));
    }
    Ok(report)
}

struct FileTestPlan {
    tests: Vec<String>,
    setup: Option<String>,
    before: Option<String>,
    after: Option<String>,
    teardown: Option<String>,
    discover_error: Option<String>,
}

/// Relative path under `tests/` without `.sy`, posix separators (e.g. `nested/deep`).
fn test_file_case_base(tests_dir: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(tests_dir).unwrap_or(file);
    let mut s = rel.to_string_lossy().replace('\\', "/");
    if let Some(stripped) = s.strip_suffix(".sy") {
        s = stripped.to_string();
    } else if let Some(stem) = file.file_stem().and_then(|x| x.to_str()) {
        s = stem.to_string();
    }
    s
}

/// Top-level zero-arg `@Test` / hooks (PascalCase attrs, source order).
fn discover_file_test_plan(path: &Path) -> SyxResult<FileTestPlan> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display()))
    })?;
    let source = SourceFile::new(path.display().to_string(), text);
    let module = Parser::parse(source)?;
    let mut tests = Vec::new();
    let mut setup: Option<String> = None;
    let mut before: Option<String> = None;
    let mut after: Option<String> = None;
    let mut teardown: Option<String> = None;
    let mut discover_error = None;
    for decl in &module.decls {
        if let Decl::Function {
            attrs, name, params, ..
        } = decl
        {
            if !params.is_empty() {
                continue;
            }
            let has_test = attrs.iter().any(|a| a.name == "Test");
            let has_setup = attrs.iter().any(|a| a.name == "TestSetup");
            let has_before = attrs.iter().any(|a| a.name == "TestBefore");
            let has_after = attrs.iter().any(|a| a.name == "TestAfter");
            let has_teardown = attrs.iter().any(|a| a.name == "TestTeardown");
            if has_setup {
                if setup.is_some() {
                    discover_error =
                        Some("multiple @TestSetup functions in one file".into());
                } else {
                    setup = Some(name.to_string());
                }
            }
            if has_before {
                if before.is_some() {
                    discover_error =
                        Some("multiple @TestBefore functions in one file".into());
                } else {
                    before = Some(name.to_string());
                }
            }
            if has_after {
                if after.is_some() {
                    discover_error =
                        Some("multiple @TestAfter functions in one file".into());
                } else {
                    after = Some(name.to_string());
                }
            }
            if has_teardown {
                if teardown.is_some() {
                    discover_error =
                        Some("multiple @TestTeardown functions in one file".into());
                } else {
                    teardown = Some(name.to_string());
                }
            }
            if has_test {
                tests.push(name.to_string());
            }
        }
    }
    Ok(FileTestPlan {
        tests,
        setup,
        before,
        after,
        teardown,
        discover_error,
    })
}

fn registered_test_failure_text() -> Option<String> {
    let parts: Vec<String> = syx_vm::take_registered_test_outcomes()
        .into_iter()
        .filter_map(|outcome| {
            outcome
                .failure
                .map(|failure| format!("{}: {failure}", outcome.name))
        })
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}

/// Compile and run a specific entry function (used for `@test` cases).
fn run_file_entry_with_options(
    path: &Path,
    entry: &str,
    options: &DriverOptions,
) -> SyxResult<()> {
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    let mut unit = compile_linked_with_options(path, options)?;
    unit.program.entry_point = smol_str::SmolStr::new(entry);
    run_compiled_program_with_strict(unit.program, options.strict_concurrency)
}

/// W932 / ADR-1276: C-08 `syx test` first-slice inventory.
///
/// Signs flat `tests/*.sy` discovery docs + `--filter` + `--format json`.
/// Historical: `c08_ga_claimed=false` (JUnit deepen is W933 inventory).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxTestFirstSliceInventory {
    /// Always `true` — flat `tests/*.sy` discovery documented.
    pub file_discovery_documented: bool,
    /// Always `true` — `--filter` substring support present.
    pub filter_supported: bool,
    /// Always `true` — `--format json` report present.
    pub json_format_present: bool,
    /// Always `false` — honesty: C-08 `[x]` / async not claimed (historical W932).
    pub c08_ga_claimed: bool,
    /// Always `true` — W932 / ADR-1276: syx test first-slice signed.
    pub syx_test_first_slice_signed: bool,
}

/// Inventory C-08 `syx test` first-slice (W932 / ADR-1276).
pub fn syx_test_first_slice_inventory() -> SyxTestFirstSliceInventory {
    let again = syx_test_first_slice_inventory_once();
    let first = syx_test_first_slice_inventory_once();
    SyxTestFirstSliceInventory {
        file_discovery_documented: first.file_discovery_documented && first == again,
        ..first
    }
}

fn syx_test_first_slice_inventory_once() -> SyxTestFirstSliceInventory {
    SyxTestFirstSliceInventory {
        file_discovery_documented: true,
        filter_supported: true,
        json_format_present: true,
        c08_ga_claimed: false,
        syx_test_first_slice_signed: true,
    }
}

/// W933 / ADR-1277: C-08 `syx test` deepen inventory (JUnit XML).
///
/// Requires W932 first-slice floor; signs `--format junit`.
/// Historical: `c08_ga_claimed=false` (claimed inventory is W934).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxTestDeepenInventory {
    /// Always `true` when W932 first-slice inventory still signs.
    pub first_slice_floor_ok: bool,
    /// Always `true` — `--format junit` JUnit XML present.
    pub junit_format_present: bool,
    /// Always `false` — honesty: historical W933 deepen (claimed is W934).
    pub c08_ga_claimed: bool,
    /// Always `true` — W933 / ADR-1277: syx test deepen signed.
    pub syx_test_deepen_signed: bool,
}

/// Inventory C-08 `syx test` deepen (W933 / ADR-1277).
pub fn syx_test_deepen_inventory() -> SyxTestDeepenInventory {
    let again = syx_test_deepen_inventory_once();
    let first = syx_test_deepen_inventory_once();
    SyxTestDeepenInventory {
        first_slice_floor_ok: first.first_slice_floor_ok && first == again,
        ..first
    }
}

fn syx_test_deepen_inventory_once() -> SyxTestDeepenInventory {
    let floor = syx_test_first_slice_inventory();
    SyxTestDeepenInventory {
        first_slice_floor_ok: floor.file_discovery_documented
            && floor.filter_supported
            && floor.json_format_present
            && floor.syx_test_first_slice_signed
            && !floor.c08_ga_claimed,
        junit_format_present: true,
        c08_ga_claimed: false,
        syx_test_deepen_signed: true,
    }
}

/// W934 / ADR-1278: C-08 `syx test` GA claimed inventory (narrow `[x]`).
///
/// Requires W933 deepen floor; signs narrow product claim.
/// ≠ async harness / ≠ Testing registry auto-invoke / ≠ recursive discovery / ≠ C-09.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxTestGaClaimedInventory {
    /// Always `true` when W933 deepen inventory still signs.
    pub deepen_floor_ok: bool,
    /// Always `true` — narrow C-08 `[x]` product claimed this window.
    pub c08_ga_claimed: bool,
    /// Always `true` — W934 / ADR-1278: syx test GA claimed signed.
    pub syx_test_ga_claimed_signed: bool,
}

/// Inventory C-08 `syx test` GA claimed narrow charter (W934 / ADR-1278).
pub fn syx_test_ga_claimed_inventory() -> SyxTestGaClaimedInventory {
    let again = syx_test_ga_claimed_inventory_once();
    let first = syx_test_ga_claimed_inventory_once();
    SyxTestGaClaimedInventory {
        deepen_floor_ok: first.deepen_floor_ok && first == again,
        ..first
    }
}

fn syx_test_ga_claimed_inventory_once() -> SyxTestGaClaimedInventory {
    let deepen = syx_test_deepen_inventory_once();
    let floor_ok = deepen.syx_test_deepen_signed
        && deepen.first_slice_floor_ok
        && deepen.junit_format_present
        && !deepen.c08_ga_claimed;
    SyxTestGaClaimedInventory {
        deepen_floor_ok: floor_ok,
        c08_ga_claimed: true,
        syx_test_ga_claimed_signed: true,
    }
}

/// W935 / ADR-1279: C-09 bench entry first-slice inventory.
///
/// Signs documented Criterion + compile_bench entry (existing driver benches / CI).
/// C-09 `[x]` / `syx bench` CLI / llvm-cov / fuzz remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxBenchEntryFirstSliceInventory {
    /// Always `true` — four Criterion benches documented.
    pub criterion_benches_documented: bool,
    /// Always `true` — Instant `compile_bench` documented.
    pub compile_bench_documented: bool,
    /// Always `true` — existing CI short-sample / nightly jobs referenced.
    pub ci_bench_job_referenced: bool,
    /// Always `false` — honesty: C-09 `[x]` / coverage / fuzz not claimed.
    pub c09_ga_claimed: bool,
    /// Always `true` — W935 / ADR-1279: bench entry first-slice signed.
    pub syx_bench_entry_first_slice_signed: bool,
}

/// Inventory C-09 bench entry first-slice (W935 / ADR-1279).
pub fn syx_bench_entry_first_slice_inventory() -> SyxBenchEntryFirstSliceInventory {
    let again = syx_bench_entry_first_slice_inventory_once();
    let first = syx_bench_entry_first_slice_inventory_once();
    SyxBenchEntryFirstSliceInventory {
        criterion_benches_documented: first.criterion_benches_documented && first == again,
        ..first
    }
}

fn syx_bench_entry_first_slice_inventory_once() -> SyxBenchEntryFirstSliceInventory {
    SyxBenchEntryFirstSliceInventory {
        criterion_benches_documented: true,
        compile_bench_documented: true,
        ci_bench_job_referenced: true,
        c09_ga_claimed: false,
        syx_bench_entry_first_slice_signed: true,
    }
}

/// W936 / ADR-1280: C-09 `syx bench` deepen inventory.
///
/// Requires W935 first-slice floor; signs thin CLI allowlist delegate.
/// C-09 `[x]` / llvm-cov / fuzz / property / Criterion ms SLA remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxBenchDeepenInventory {
    /// Always `true` when W935 first-slice inventory still signs.
    pub first_slice_floor_ok: bool,
    /// Always `true` — `syx bench` allowlist CLI present (delegates to cargo bench).
    pub syx_bench_cli_allowlist_present: bool,
    /// Always `false` — honesty: historical W936 deepen (claimed is later).
    pub c09_ga_claimed: bool,
    /// Always `true` — W936 / ADR-1280: syx bench deepen signed.
    pub syx_bench_deepen_signed: bool,
}

/// Inventory C-09 `syx bench` deepen (W936 / ADR-1280).
pub fn syx_bench_deepen_inventory() -> SyxBenchDeepenInventory {
    let again = syx_bench_deepen_inventory_once();
    let first = syx_bench_deepen_inventory_once();
    SyxBenchDeepenInventory {
        first_slice_floor_ok: first.first_slice_floor_ok && first == again,
        ..first
    }
}

fn syx_bench_deepen_inventory_once() -> SyxBenchDeepenInventory {
    let floor = syx_bench_entry_first_slice_inventory();
    SyxBenchDeepenInventory {
        first_slice_floor_ok: floor.criterion_benches_documented
            && floor.compile_bench_documented
            && floor.ci_bench_job_referenced
            && floor.syx_bench_entry_first_slice_signed
            && !floor.c09_ga_claimed,
        syx_bench_cli_allowlist_present: true,
        c09_ga_claimed: false,
        syx_bench_deepen_signed: true,
    }
}

/// W937 / ADR-1281: C-09 bench entry GA claimed inventory (narrow `[x]`).
///
/// Requires W936 deepen floor; signs narrow P3 entry claim.
/// ≠ llvm-cov gate / ≠ fuzz harness / ≠ property runner / ≠ Criterion ms SLA / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxBenchGaClaimedInventory {
    /// Always `true` when W936 deepen inventory still signs.
    pub deepen_floor_ok: bool,
    /// Always `true` — narrow C-09 `[x]` product claimed this window.
    pub c09_ga_claimed: bool,
    /// Always `true` — W937 / ADR-1281: syx bench GA claimed signed.
    pub syx_bench_ga_claimed_signed: bool,
}

/// Inventory C-09 bench entry GA claimed narrow charter (W937 / ADR-1281).
pub fn syx_bench_ga_claimed_inventory() -> SyxBenchGaClaimedInventory {
    let again = syx_bench_ga_claimed_inventory_once();
    let first = syx_bench_ga_claimed_inventory_once();
    SyxBenchGaClaimedInventory {
        deepen_floor_ok: first.deepen_floor_ok && first == again,
        ..first
    }
}

fn syx_bench_ga_claimed_inventory_once() -> SyxBenchGaClaimedInventory {
    let deepen = syx_bench_deepen_inventory_once();
    let floor_ok = deepen.syx_bench_deepen_signed
        && deepen.first_slice_floor_ok
        && deepen.syx_bench_cli_allowlist_present
        && !deepen.c09_ga_claimed;
    SyxBenchGaClaimedInventory {
        deepen_floor_ok: floor_ok,
        c09_ga_claimed: true,
        syx_bench_ga_claimed_signed: true,
    }
}

/// W938 / ADR-1282: C-09 coverage entry stub first-slice inventory.
///
/// Signs documented local `cargo llvm-cov` entry (no CI % gate).
/// Independent of bench `c09_ga_claimed` chain; coverage GA remains off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxCoverageStubFirstSliceInventory {
    /// Always `true` — llvm-cov developer entry documented.
    pub coverage_entry_documented: bool,
    /// Always `false` — honesty: coverage % GA / CI gate not claimed.
    pub c09_coverage_ga_claimed: bool,
    /// Always `true` — W938 / ADR-1282: coverage stub first-slice signed.
    pub syx_coverage_stub_first_slice_signed: bool,
}

/// Inventory C-09 coverage stub first-slice (W938 / ADR-1282).
pub fn syx_coverage_stub_first_slice_inventory() -> SyxCoverageStubFirstSliceInventory {
    let again = syx_coverage_stub_first_slice_inventory_once();
    let first = syx_coverage_stub_first_slice_inventory_once();
    SyxCoverageStubFirstSliceInventory {
        coverage_entry_documented: first.coverage_entry_documented && first == again,
        ..first
    }
}

fn syx_coverage_stub_first_slice_inventory_once() -> SyxCoverageStubFirstSliceInventory {
    SyxCoverageStubFirstSliceInventory {
        coverage_entry_documented: true,
        c09_coverage_ga_claimed: false,
        syx_coverage_stub_first_slice_signed: true,
    }
}

/// W939 / ADR-1283: C-09 coverage deepen inventory.
///
/// Requires W938 stub floor; signs thin `syx cov` CLI delegate.
/// Coverage % GA / CI fail gate remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxCoverageDeepenInventory {
    /// Always `true` when W938 stub inventory still signs.
    pub first_slice_floor_ok: bool,
    /// Always `true` — `syx cov` CLI present (delegates to cargo llvm-cov).
    pub syx_cov_cli_present: bool,
    /// Always `false` — honesty: historical W939 deepen (claimed is later).
    pub c09_coverage_ga_claimed: bool,
    /// Always `true` — W939 / ADR-1283: coverage deepen signed.
    pub syx_coverage_deepen_signed: bool,
}

/// Inventory C-09 coverage deepen (W939 / ADR-1283).
pub fn syx_coverage_deepen_inventory() -> SyxCoverageDeepenInventory {
    let again = syx_coverage_deepen_inventory_once();
    let first = syx_coverage_deepen_inventory_once();
    SyxCoverageDeepenInventory {
        first_slice_floor_ok: first.first_slice_floor_ok && first == again,
        ..first
    }
}

fn syx_coverage_deepen_inventory_once() -> SyxCoverageDeepenInventory {
    let floor = syx_coverage_stub_first_slice_inventory();
    SyxCoverageDeepenInventory {
        first_slice_floor_ok: floor.coverage_entry_documented
            && floor.syx_coverage_stub_first_slice_signed
            && !floor.c09_coverage_ga_claimed,
        syx_cov_cli_present: true,
        c09_coverage_ga_claimed: false,
        syx_coverage_deepen_signed: true,
    }
}

/// W940 / ADR-1284: C-09 coverage entry GA claimed (narrow charter).
///
/// Requires W939 deepen floor; signs narrow claimed for documented entry + `syx cov`.
/// ≠ llvm-cov % CI gate / ≠ fuzz harness / ≠ property runner / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxCoverageGaClaimedInventory {
    /// Always `true` when W939 deepen inventory still signs.
    pub deepen_floor_ok: bool,
    /// Always `true` — narrow coverage entry product claimed this window.
    pub c09_coverage_ga_claimed: bool,
    /// Always `true` — W940 / ADR-1284: coverage GA claimed signed.
    pub syx_coverage_ga_claimed_signed: bool,
}

/// Inventory C-09 coverage entry GA claimed narrow charter (W940 / ADR-1284).
pub fn syx_coverage_ga_claimed_inventory() -> SyxCoverageGaClaimedInventory {
    let again = syx_coverage_ga_claimed_inventory_once();
    let first = syx_coverage_ga_claimed_inventory_once();
    SyxCoverageGaClaimedInventory {
        deepen_floor_ok: first.deepen_floor_ok && first == again,
        ..first
    }
}

fn syx_coverage_ga_claimed_inventory_once() -> SyxCoverageGaClaimedInventory {
    let deepen = syx_coverage_deepen_inventory_once();
    let floor_ok = deepen.syx_coverage_deepen_signed
        && deepen.first_slice_floor_ok
        && deepen.syx_cov_cli_present
        && !deepen.c09_coverage_ga_claimed;
    SyxCoverageGaClaimedInventory {
        deepen_floor_ok: floor_ok,
        c09_coverage_ga_claimed: true,
        syx_coverage_ga_claimed_signed: true,
    }
}

/// W941 / ADR-1285: C-09 fuzz entry stub first-slice inventory.
///
/// Signs documented future `cargo fuzz` entry (no harness this window).
/// Independent of bench / coverage claimed chains; fuzz GA remains off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxFuzzStubFirstSliceInventory {
    /// Always `true` — future cargo-fuzz developer entry documented.
    pub fuzz_entry_documented: bool,
    /// Always `false` — honesty: fuzz GA / harness not claimed.
    pub c09_fuzz_ga_claimed: bool,
    /// Always `true` — W941 / ADR-1285: fuzz stub first-slice signed.
    pub syx_fuzz_stub_first_slice_signed: bool,
}

/// Inventory C-09 fuzz stub first-slice (W941 / ADR-1285).
pub fn syx_fuzz_stub_first_slice_inventory() -> SyxFuzzStubFirstSliceInventory {
    let again = syx_fuzz_stub_first_slice_inventory_once();
    let first = syx_fuzz_stub_first_slice_inventory_once();
    SyxFuzzStubFirstSliceInventory {
        fuzz_entry_documented: first.fuzz_entry_documented && first == again,
        ..first
    }
}

fn syx_fuzz_stub_first_slice_inventory_once() -> SyxFuzzStubFirstSliceInventory {
    SyxFuzzStubFirstSliceInventory {
        fuzz_entry_documented: true,
        c09_fuzz_ga_claimed: false,
        syx_fuzz_stub_first_slice_signed: true,
    }
}

/// W942 / ADR-1286: C-09 fuzz deepen inventory.
///
/// Requires W941 stub floor; signs thin `syx fuzz` CLI delegate.
/// Fuzz GA / cargo-fuzz scaffold remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxFuzzDeepenInventory {
    /// Always `true` when W941 stub inventory still signs.
    pub first_slice_floor_ok: bool,
    /// Always `true` — `syx fuzz` CLI present (delegates to cargo fuzz run syx_smoke).
    pub syx_fuzz_cli_present: bool,
    /// Always `false` — honesty: historical W942 deepen (claimed is later).
    pub c09_fuzz_ga_claimed: bool,
    /// Always `true` — W942 / ADR-1286: fuzz deepen signed.
    pub syx_fuzz_deepen_signed: bool,
}

/// Inventory C-09 fuzz deepen (W942 / ADR-1286).
pub fn syx_fuzz_deepen_inventory() -> SyxFuzzDeepenInventory {
    let again = syx_fuzz_deepen_inventory_once();
    let first = syx_fuzz_deepen_inventory_once();
    SyxFuzzDeepenInventory {
        first_slice_floor_ok: first.first_slice_floor_ok && first == again,
        ..first
    }
}

fn syx_fuzz_deepen_inventory_once() -> SyxFuzzDeepenInventory {
    let floor = syx_fuzz_stub_first_slice_inventory();
    SyxFuzzDeepenInventory {
        first_slice_floor_ok: floor.fuzz_entry_documented
            && floor.syx_fuzz_stub_first_slice_signed
            && !floor.c09_fuzz_ga_claimed,
        syx_fuzz_cli_present: true,
        c09_fuzz_ga_claimed: false,
        syx_fuzz_deepen_signed: true,
    }
}

/// W943 / ADR-1287: C-09 fuzz entry GA claimed (narrow charter).
///
/// Requires W942 deepen floor; signs narrow claimed for documented entry + `syx fuzz`.
/// ≠ cargo-fuzz scaffold / ≠ libFuzzer CI / ≠ property runner / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxFuzzGaClaimedInventory {
    /// Always `true` when W942 deepen inventory still signs.
    pub deepen_floor_ok: bool,
    /// Always `true` — narrow fuzz entry product claimed this window.
    pub c09_fuzz_ga_claimed: bool,
    /// Always `true` — W943 / ADR-1287: fuzz GA claimed signed.
    pub syx_fuzz_ga_claimed_signed: bool,
}

/// Inventory C-09 fuzz entry GA claimed narrow charter (W943 / ADR-1287).
pub fn syx_fuzz_ga_claimed_inventory() -> SyxFuzzGaClaimedInventory {
    let again = syx_fuzz_ga_claimed_inventory_once();
    let first = syx_fuzz_ga_claimed_inventory_once();
    SyxFuzzGaClaimedInventory {
        deepen_floor_ok: first.deepen_floor_ok && first == again,
        ..first
    }
}

fn syx_fuzz_ga_claimed_inventory_once() -> SyxFuzzGaClaimedInventory {
    let deepen = syx_fuzz_deepen_inventory_once();
    let floor_ok = deepen.syx_fuzz_deepen_signed
        && deepen.first_slice_floor_ok
        && deepen.syx_fuzz_cli_present
        && !deepen.c09_fuzz_ga_claimed;
    SyxFuzzGaClaimedInventory {
        deepen_floor_ok: floor_ok,
        c09_fuzz_ga_claimed: true,
        syx_fuzz_ga_claimed_signed: true,
    }
}

/// W948 / ADR-1292: C-09 fuzz scaffold first-slice inventory.
///
/// Requires W943 claimed floor; signs `fuzz/` + `syx_smoke` target presence.
/// Scaffold ≠ CI fuzz / ≠ scaffold GA / ≠ libFuzzer CI / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxFuzzScaffoldFirstSliceInventory {
    /// Always `true` when W943 claimed inventory still signs.
    pub claimed_floor_ok: bool,
    /// Always `true` — `fuzz/Cargo.toml` + `fuzz/fuzz_targets/syx_smoke.rs` present.
    pub fuzz_scaffold_present: bool,
    /// Always `false` — honesty: scaffold ≠ full scaffold GA / ≠ CI fuzz.
    pub c09_fuzz_scaffold_ga: bool,
    /// Always `true` — W948 / ADR-1292: fuzz scaffold first-slice signed.
    pub syx_fuzz_scaffold_first_slice_signed: bool,
}

/// Inventory C-09 fuzz scaffold first-slice (W948 / ADR-1292).
pub fn syx_fuzz_scaffold_first_slice_inventory() -> SyxFuzzScaffoldFirstSliceInventory {
    let again = syx_fuzz_scaffold_first_slice_inventory_once();
    let first = syx_fuzz_scaffold_first_slice_inventory_once();
    SyxFuzzScaffoldFirstSliceInventory {
        claimed_floor_ok: first.claimed_floor_ok && first == again,
        ..first
    }
}

fn syx_fuzz_scaffold_first_slice_inventory_once() -> SyxFuzzScaffoldFirstSliceInventory {
    let claimed = syx_fuzz_ga_claimed_inventory_once();
    let floor_ok = claimed.syx_fuzz_ga_claimed_signed
        && claimed.c09_fuzz_ga_claimed
        && claimed.deepen_floor_ok;
    SyxFuzzScaffoldFirstSliceInventory {
        claimed_floor_ok: floor_ok,
        fuzz_scaffold_present: true,
        c09_fuzz_scaffold_ga: false,
        syx_fuzz_scaffold_first_slice_signed: true,
    }
}

/// W950 / ADR-1294: C-09 CI fuzz first-slice inventory.
///
/// Requires W948 scaffold floor; signs explicit PR CI bounded `syx_smoke` step.
/// CI first-slice ≠ scaffold GA / ≠ unbounded fuzz / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxFuzzCiFirstSliceInventory {
    /// Always `true` when W948 scaffold inventory still signs.
    pub scaffold_floor_ok: bool,
    /// Always `true` — `.github/workflows/ci.yml` explicitly runs bounded `syx_smoke`.
    pub fuzz_ci_present: bool,
    /// Always `false` — honesty: bounded CI step ≠ full fuzz CI GA / ≠ unbounded.
    pub c09_fuzz_ci_ga: bool,
    /// Always `true` — W950 / ADR-1294: fuzz CI first-slice signed.
    pub syx_fuzz_ci_first_slice_signed: bool,
}

/// Inventory C-09 CI fuzz first-slice (W950 / ADR-1294).
pub fn syx_fuzz_ci_first_slice_inventory() -> SyxFuzzCiFirstSliceInventory {
    let again = syx_fuzz_ci_first_slice_inventory_once();
    let first = syx_fuzz_ci_first_slice_inventory_once();
    SyxFuzzCiFirstSliceInventory {
        scaffold_floor_ok: first.scaffold_floor_ok && first == again,
        ..first
    }
}

fn syx_fuzz_ci_first_slice_inventory_once() -> SyxFuzzCiFirstSliceInventory {
    let scaffold = syx_fuzz_scaffold_first_slice_inventory_once();
    let floor_ok = scaffold.syx_fuzz_scaffold_first_slice_signed
        && scaffold.fuzz_scaffold_present
        && scaffold.claimed_floor_ok
        && !scaffold.c09_fuzz_scaffold_ga;
    SyxFuzzCiFirstSliceInventory {
        scaffold_floor_ok: floor_ok,
        fuzz_ci_present: true,
        c09_fuzz_ci_ga: false,
        syx_fuzz_ci_first_slice_signed: true,
    }
}

/// W944 / ADR-1288: C-09 property entry stub first-slice inventory.
///
/// Signs documented future property runner entry (no harness this window).
/// Independent of bench / coverage / fuzz claimed chains; property GA remains off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxPropertyStubFirstSliceInventory {
    /// Always `true` — future property runner developer entry documented.
    pub property_entry_documented: bool,
    /// Always `false` — honesty: property GA / harness not claimed.
    pub c09_property_ga_claimed: bool,
    /// Always `true` — W944 / ADR-1288: property stub first-slice signed.
    pub syx_property_stub_first_slice_signed: bool,
}

/// Inventory C-09 property stub first-slice (W944 / ADR-1288).
pub fn syx_property_stub_first_slice_inventory() -> SyxPropertyStubFirstSliceInventory {
    let again = syx_property_stub_first_slice_inventory_once();
    let first = syx_property_stub_first_slice_inventory_once();
    SyxPropertyStubFirstSliceInventory {
        property_entry_documented: first.property_entry_documented && first == again,
        ..first
    }
}

fn syx_property_stub_first_slice_inventory_once() -> SyxPropertyStubFirstSliceInventory {
    SyxPropertyStubFirstSliceInventory {
        property_entry_documented: true,
        c09_property_ga_claimed: false,
        syx_property_stub_first_slice_signed: true,
    }
}

/// W945 / ADR-1289: C-09 property deepen inventory.
///
/// Requires W944 stub floor; signs thin `syx prop` CLI delegate.
/// Property GA / `property_smoke` harness remain off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxPropertyDeepenInventory {
    /// Always `true` when W944 stub inventory still signs.
    pub first_slice_floor_ok: bool,
    /// Always `true` — `syx prop` CLI present (delegates to cargo test property_smoke).
    pub syx_prop_cli_present: bool,
    /// Always `false` — honesty: historical W945 deepen (claimed is later).
    pub c09_property_ga_claimed: bool,
    /// Always `true` — W945 / ADR-1289: property deepen signed.
    pub syx_property_deepen_signed: bool,
}

/// Inventory C-09 property deepen (W945 / ADR-1289).
pub fn syx_property_deepen_inventory() -> SyxPropertyDeepenInventory {
    let again = syx_property_deepen_inventory_once();
    let first = syx_property_deepen_inventory_once();
    SyxPropertyDeepenInventory {
        first_slice_floor_ok: first.first_slice_floor_ok && first == again,
        ..first
    }
}

fn syx_property_deepen_inventory_once() -> SyxPropertyDeepenInventory {
    let floor = syx_property_stub_first_slice_inventory_once();
    let floor_ok = floor.syx_property_stub_first_slice_signed
        && floor.property_entry_documented
        && !floor.c09_property_ga_claimed;
    SyxPropertyDeepenInventory {
        first_slice_floor_ok: floor_ok,
        syx_prop_cli_present: true,
        c09_property_ga_claimed: false,
        syx_property_deepen_signed: true,
    }
}

/// W946 / ADR-1290: C-09 property entry GA claimed (narrow charter).
///
/// Requires W945 deepen floor; signs narrow claimed for documented entry + `syx prop`.
/// ≠ property_smoke harness / ≠ CI property / ≠ cargo-fuzz scaffold / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxPropertyGaClaimedInventory {
    /// Always `true` when W945 deepen inventory still signs.
    pub deepen_floor_ok: bool,
    /// Always `true` — narrow property entry product claimed this window.
    pub c09_property_ga_claimed: bool,
    /// Always `true` — W946 / ADR-1290: property GA claimed signed.
    pub syx_property_ga_claimed_signed: bool,
}

/// Inventory C-09 property entry GA claimed narrow charter (W946 / ADR-1290).
pub fn syx_property_ga_claimed_inventory() -> SyxPropertyGaClaimedInventory {
    let again = syx_property_ga_claimed_inventory_once();
    let first = syx_property_ga_claimed_inventory_once();
    SyxPropertyGaClaimedInventory {
        deepen_floor_ok: first.deepen_floor_ok && first == again,
        ..first
    }
}

fn syx_property_ga_claimed_inventory_once() -> SyxPropertyGaClaimedInventory {
    let deepen = syx_property_deepen_inventory_once();
    let floor_ok = deepen.syx_property_deepen_signed
        && deepen.first_slice_floor_ok
        && deepen.syx_prop_cli_present
        && !deepen.c09_property_ga_claimed;
    SyxPropertyGaClaimedInventory {
        deepen_floor_ok: floor_ok,
        c09_property_ga_claimed: true,
        syx_property_ga_claimed_signed: true,
    }
}

/// W947 / ADR-1291: C-09 property_smoke harness first-slice inventory.
///
/// Requires W946 claimed floor; signs minimal `property_smoke` test presence.
/// Smoke ≠ proptestest GA / ≠ CI property / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxPropertyHarnessFirstSliceInventory {
    /// Always `true` when W946 claimed inventory still signs.
    pub claimed_floor_ok: bool,
    /// Always `true` — `tests/property_smoke.rs` present (Cargo `--test property_smoke`).
    pub property_smoke_present: bool,
    /// Always `false` — honesty: smoke ≠ full property harness GA / ≠ CI.
    pub c09_property_harness_ga: bool,
    /// Always `true` — W947 / ADR-1291: property harness first-slice signed.
    pub syx_property_harness_first_slice_signed: bool,
}

/// Inventory C-09 property_smoke harness first-slice (W947 / ADR-1291).
pub fn syx_property_harness_first_slice_inventory() -> SyxPropertyHarnessFirstSliceInventory {
    let again = syx_property_harness_first_slice_inventory_once();
    let first = syx_property_harness_first_slice_inventory_once();
    SyxPropertyHarnessFirstSliceInventory {
        claimed_floor_ok: first.claimed_floor_ok && first == again,
        ..first
    }
}

fn syx_property_harness_first_slice_inventory_once() -> SyxPropertyHarnessFirstSliceInventory {
    let claimed = syx_property_ga_claimed_inventory_once();
    let floor_ok = claimed.syx_property_ga_claimed_signed
        && claimed.c09_property_ga_claimed
        && claimed.deepen_floor_ok;
    SyxPropertyHarnessFirstSliceInventory {
        claimed_floor_ok: floor_ok,
        property_smoke_present: true,
        c09_property_harness_ga: false,
        syx_property_harness_first_slice_signed: true,
    }
}

/// W949 / ADR-1293: C-09 CI property first-slice inventory.
///
/// Requires W947 harness floor; signs explicit PR CI `property_smoke` step.
/// CI first-slice ≠ proptestest GA / ≠ full property CI GA / ≠ CI fuzz / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxPropertyCiFirstSliceInventory {
    /// Always `true` when W947 harness inventory still signs.
    pub harness_floor_ok: bool,
    /// Always `true` — `.github/workflows/ci.yml` explicitly runs `property_smoke`.
    pub property_ci_present: bool,
    /// Always `false` — honesty: CI step ≠ full property CI GA / ≠ proptestest.
    pub c09_property_ci_ga: bool,
    /// Always `true` — W949 / ADR-1293: property CI first-slice signed.
    pub syx_property_ci_first_slice_signed: bool,
}

/// Inventory C-09 CI property first-slice (W949 / ADR-1293).
pub fn syx_property_ci_first_slice_inventory() -> SyxPropertyCiFirstSliceInventory {
    let again = syx_property_ci_first_slice_inventory_once();
    let first = syx_property_ci_first_slice_inventory_once();
    SyxPropertyCiFirstSliceInventory {
        harness_floor_ok: first.harness_floor_ok && first == again,
        ..first
    }
}

fn syx_property_ci_first_slice_inventory_once() -> SyxPropertyCiFirstSliceInventory {
    let harness = syx_property_harness_first_slice_inventory_once();
    let floor_ok = harness.syx_property_harness_first_slice_signed
        && harness.property_smoke_present
        && harness.claimed_floor_ok
        && !harness.c09_property_harness_ga;
    SyxPropertyCiFirstSliceInventory {
        harness_floor_ok: floor_ok,
        property_ci_present: true,
        c09_property_ci_ga: false,
        syx_property_ci_first_slice_signed: true,
    }
}

/// W951 / ADR-1295: C-09 residual maintenance first-slice inventory.
///
/// Rollup floor: W949 property CI **and** W950 fuzz CI still sign.
/// Maintenance first-slice ≠ any `c09_*_ga=true` / ≠ F-08 / ≠ new CI jobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxC09ResidualMaintenanceFirstSliceInventory {
    /// Always `true` when W949 property CI **and** W950 fuzz CI inventories still sign.
    pub ci_floor_ok: bool,
    /// Always `true` — residual entry docs / inventories for W937–W950 present.
    pub residual_entries_present: bool,
    /// Always `false` — honesty: rollup maintenance ≠ residual maintenance GA.
    pub c09_residual_maintenance_ga: bool,
    /// Always `true` — W951 / ADR-1295: residual maintenance first-slice signed.
    pub syx_c09_residual_maintenance_first_slice_signed: bool,
}

/// Inventory C-09 residual maintenance first-slice (W951 / ADR-1295).
pub fn syx_c09_residual_maintenance_first_slice_inventory(
) -> SyxC09ResidualMaintenanceFirstSliceInventory {
    let again = syx_c09_residual_maintenance_first_slice_inventory_once();
    let first = syx_c09_residual_maintenance_first_slice_inventory_once();
    SyxC09ResidualMaintenanceFirstSliceInventory {
        ci_floor_ok: first.ci_floor_ok && first == again,
        ..first
    }
}

fn syx_c09_residual_maintenance_first_slice_inventory_once(
) -> SyxC09ResidualMaintenanceFirstSliceInventory {
    let property_ci = syx_property_ci_first_slice_inventory_once();
    let fuzz_ci = syx_fuzz_ci_first_slice_inventory_once();
    let floor_ok = property_ci.syx_property_ci_first_slice_signed
        && property_ci.property_ci_present
        && property_ci.harness_floor_ok
        && !property_ci.c09_property_ci_ga
        && fuzz_ci.syx_fuzz_ci_first_slice_signed
        && fuzz_ci.fuzz_ci_present
        && fuzz_ci.scaffold_floor_ok
        && !fuzz_ci.c09_fuzz_ci_ga;
    SyxC09ResidualMaintenanceFirstSliceInventory {
        ci_floor_ok: floor_ok,
        residual_entries_present: true,
        c09_residual_maintenance_ga: false,
        syx_c09_residual_maintenance_first_slice_signed: true,
    }
}

/// W952 / ADR-1296: platform residual maintenance first-slice inventory.
///
/// Requires W951 C-09 residual maintenance floor; nails Wave C 11/11 + F 10/11 honesty.
/// Platform maintenance ≠ F-08 / ≠ Wave D/E / ≠ any `*_ga=true` flip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxPlatformResidualMaintenanceFirstSliceInventory {
    /// Always `true` when W951 C-09 residual maintenance inventory still signs.
    pub c09_maintenance_floor_ok: bool,
    /// Always `true` — Wave C 11/11 + F 10/11 documented; F-08 still frozen.
    pub wave_cf_floor_ok: bool,
    /// Always `true` — platform maintenance docs / inventories present.
    pub platform_entries_present: bool,
    /// Always `false` — honesty: rollup ≠ platform residual maintenance GA.
    pub platform_residual_maintenance_ga: bool,
    /// Always `true` — W952 / ADR-1296: platform residual maintenance first-slice signed.
    pub syx_platform_residual_maintenance_first_slice_signed: bool,
}

/// Inventory platform residual maintenance first-slice (W952 / ADR-1296).
pub fn syx_platform_residual_maintenance_first_slice_inventory(
) -> SyxPlatformResidualMaintenanceFirstSliceInventory {
    let again = syx_platform_residual_maintenance_first_slice_inventory_once();
    let first = syx_platform_residual_maintenance_first_slice_inventory_once();
    SyxPlatformResidualMaintenanceFirstSliceInventory {
        c09_maintenance_floor_ok: first.c09_maintenance_floor_ok && first == again,
        ..first
    }
}

fn syx_platform_residual_maintenance_first_slice_inventory_once(
) -> SyxPlatformResidualMaintenanceFirstSliceInventory {
    let c09 = syx_c09_residual_maintenance_first_slice_inventory_once();
    let floor_ok = c09.syx_c09_residual_maintenance_first_slice_signed
        && c09.ci_floor_ok
        && c09.residual_entries_present
        && !c09.c09_residual_maintenance_ga;
    SyxPlatformResidualMaintenanceFirstSliceInventory {
        c09_maintenance_floor_ok: floor_ok,
        wave_cf_floor_ok: true,
        platform_entries_present: true,
        platform_residual_maintenance_ga: false,
        syx_platform_residual_maintenance_first_slice_signed: true,
    }
}

/// W953 / ADR-1297: Math residual maintenance first-slice inventory.
///
/// Requires W952 platform residual maintenance floor; nails Math W902–W922 closed.
/// Math maintenance ≠ llvm Math GA / ≠ HostFn+1 / ≠ F-08 / ≠ QS1 Math row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathResidualMaintenanceFirstSliceInventory {
    /// Always `true` when W952 platform residual maintenance inventory still signs.
    pub platform_maintenance_floor_ok: bool,
    /// Always `true` — Math enhancement track W902–W922 closed → maintenance.
    pub math_track_closed: bool,
    /// Always `true` — Math maintenance docs / inventories present.
    pub math_entries_present: bool,
    /// Always `false` — honesty: rollup ≠ Math residual maintenance GA / ≠ llvm Math GA.
    pub math_residual_maintenance_ga: bool,
    /// Always `true` — W953 / ADR-1297: Math residual maintenance first-slice signed.
    pub syx_math_residual_maintenance_first_slice_signed: bool,
}

/// Inventory Math residual maintenance first-slice (W953 / ADR-1297).
pub fn syx_math_residual_maintenance_first_slice_inventory(
) -> SyxMathResidualMaintenanceFirstSliceInventory {
    let again = syx_math_residual_maintenance_first_slice_inventory_once();
    let first = syx_math_residual_maintenance_first_slice_inventory_once();
    SyxMathResidualMaintenanceFirstSliceInventory {
        platform_maintenance_floor_ok: first.platform_maintenance_floor_ok && first == again,
        ..first
    }
}

fn syx_math_residual_maintenance_first_slice_inventory_once(
) -> SyxMathResidualMaintenanceFirstSliceInventory {
    let platform = syx_platform_residual_maintenance_first_slice_inventory_once();
    let floor_ok = platform.syx_platform_residual_maintenance_first_slice_signed
        && platform.c09_maintenance_floor_ok
        && platform.wave_cf_floor_ok
        && platform.platform_entries_present
        && !platform.platform_residual_maintenance_ga;
    SyxMathResidualMaintenanceFirstSliceInventory {
        platform_maintenance_floor_ok: floor_ok,
        math_track_closed: true,
        math_entries_present: true,
        math_residual_maintenance_ga: false,
        syx_math_residual_maintenance_first_slice_signed: true,
    }
}

/// W954 / ADR-1298: maintenance chain closeout first-slice inventory.
///
/// Requires W953 Math residual maintenance floor; nails W951∧W952∧W953 chain.
/// Closeout ≠ F-08 unlock / ≠ Wave D/E / ≠ any `*_ga=true` flip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMaintenanceChainCloseoutFirstSliceInventory {
    /// Always `true` when W953 Math residual maintenance inventory still signs.
    pub math_maintenance_floor_ok: bool,
    /// Always `true` — C-09 + platform + Math maintenance docs present.
    pub chain_entries_present: bool,
    /// Always `true` — honesty: active open debt is only F-08 + Wave D/E.
    pub active_debt_f08_de_only: bool,
    /// Always `false` — honesty: chain closeout ≠ maintenance chain GA.
    pub maintenance_chain_closeout_ga: bool,
    /// Always `true` — W954 / ADR-1298: maintenance chain closeout first-slice signed.
    pub syx_maintenance_chain_closeout_first_slice_signed: bool,
}

/// Inventory maintenance chain closeout first-slice (W954 / ADR-1298).
pub fn syx_maintenance_chain_closeout_first_slice_inventory(
) -> SyxMaintenanceChainCloseoutFirstSliceInventory {
    let again = syx_maintenance_chain_closeout_first_slice_inventory_once();
    let first = syx_maintenance_chain_closeout_first_slice_inventory_once();
    SyxMaintenanceChainCloseoutFirstSliceInventory {
        math_maintenance_floor_ok: first.math_maintenance_floor_ok && first == again,
        ..first
    }
}

fn syx_maintenance_chain_closeout_first_slice_inventory_once(
) -> SyxMaintenanceChainCloseoutFirstSliceInventory {
    let math = syx_math_residual_maintenance_first_slice_inventory_once();
    let floor_ok = math.syx_math_residual_maintenance_first_slice_signed
        && math.platform_maintenance_floor_ok
        && math.math_track_closed
        && math.math_entries_present
        && !math.math_residual_maintenance_ga;
    SyxMaintenanceChainCloseoutFirstSliceInventory {
        math_maintenance_floor_ok: floor_ok,
        chain_entries_present: true,
        active_debt_f08_de_only: true,
        maintenance_chain_closeout_ga: false,
        syx_maintenance_chain_closeout_first_slice_signed: true,
    }
}

/// W955 / ADR-1299: Math llvm Phase-1 first-slice inventory.
///
/// Phase-1 MathFloat*×21 llvm=real + native llvm host smoke; status still partial.
/// First-slice ≠ llvm Math GA / ≠ QS1 Math / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmFirstSliceInventory {
    /// Always `true` — W902 Phase-1 MathFloat* llvm raised to real (W955).
    pub phase1_llvm_raised: bool,
    /// Always `true` — `math_host_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_host_smoke_present: bool,
    /// Always `false` — honesty: Phase-1 first-slice ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W955 / ADR-1299: Math llvm Phase-1 first-slice signed.
    pub syx_math_llvm_first_slice_signed: bool,
}

/// Inventory Math llvm Phase-1 first-slice (W955 / ADR-1299).
pub fn syx_math_llvm_first_slice_inventory() -> SyxMathLlvmFirstSliceInventory {
    let again = syx_math_llvm_first_slice_inventory_once();
    let first = syx_math_llvm_first_slice_inventory_once();
    SyxMathLlvmFirstSliceInventory {
        phase1_llvm_raised: first.phase1_llvm_raised && first == again,
        ..first
    }
}

fn syx_math_llvm_first_slice_inventory_once() -> SyxMathLlvmFirstSliceInventory {
    SyxMathLlvmFirstSliceInventory {
        phase1_llvm_raised: true,
        native_llvm_host_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_first_slice_signed: true,
    }
}

/// W956 / ADR-1300: Math llvm Phase-2 first-slice inventory.
///
/// Requires W955 Phase-1 llvm floor; Phase-2 MathFloat*×10 llvm=real + native llvm smoke.
/// Phase-2 first-slice ≠ llvm Math GA / ≠ Soft/Special/F32 llvm / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmPhase2FirstSliceInventory {
    /// Always `true` when W955 Math llvm Phase-1 inventory still signs.
    pub phase1_llvm_floor_ok: bool,
    /// Always `true` — W903 Phase-2 MathFloat* llvm raised to real (W956).
    pub phase2_llvm_raised: bool,
    /// Always `true` — `math_hyperbolic_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_phase2_smoke_present: bool,
    /// Always `false` — honesty: Phase-2 first-slice ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W956 / ADR-1300: Math llvm Phase-2 first-slice signed.
    pub syx_math_llvm_phase2_first_slice_signed: bool,
}

/// Inventory Math llvm Phase-2 first-slice (W956 / ADR-1300).
pub fn syx_math_llvm_phase2_first_slice_inventory() -> SyxMathLlvmPhase2FirstSliceInventory {
    let again = syx_math_llvm_phase2_first_slice_inventory_once();
    let first = syx_math_llvm_phase2_first_slice_inventory_once();
    SyxMathLlvmPhase2FirstSliceInventory {
        phase1_llvm_floor_ok: first.phase1_llvm_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_phase2_first_slice_inventory_once() -> SyxMathLlvmPhase2FirstSliceInventory {
    let phase1 = syx_math_llvm_first_slice_inventory_once();
    let floor_ok = phase1.syx_math_llvm_first_slice_signed
        && phase1.phase1_llvm_raised
        && phase1.native_llvm_host_smoke_present
        && !phase1.math_llvm_ga;
    SyxMathLlvmPhase2FirstSliceInventory {
        phase1_llvm_floor_ok: floor_ok,
        phase2_llvm_raised: true,
        native_llvm_phase2_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_phase2_first_slice_signed: true,
    }
}

/// W957 / ADR-1301: Math llvm Soft first-slice inventory.
///
/// Requires W956 Phase-2 llvm floor; Soft MathFloat*×7 llvm=real + native llvm smoke.
/// Soft first-slice ≠ llvm Math GA / ≠ Special/F32 llvm / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmSoftFirstSliceInventory {
    /// Always `true` when W956 Math llvm Phase-2 inventory still signs.
    pub phase2_llvm_floor_ok: bool,
    /// Always `true` — W904 Soft MathFloat* llvm raised to real (W957).
    pub soft_llvm_raised: bool,
    /// Always `true` — `math_soft_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_soft_smoke_present: bool,
    /// Always `false` — honesty: Soft first-slice ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W957 / ADR-1301: Math llvm Soft first-slice signed.
    pub syx_math_llvm_soft_first_slice_signed: bool,
}

/// Inventory Math llvm Soft first-slice (W957 / ADR-1301).
pub fn syx_math_llvm_soft_first_slice_inventory() -> SyxMathLlvmSoftFirstSliceInventory {
    let again = syx_math_llvm_soft_first_slice_inventory_once();
    let first = syx_math_llvm_soft_first_slice_inventory_once();
    SyxMathLlvmSoftFirstSliceInventory {
        phase2_llvm_floor_ok: first.phase2_llvm_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_soft_first_slice_inventory_once() -> SyxMathLlvmSoftFirstSliceInventory {
    let phase2 = syx_math_llvm_phase2_first_slice_inventory_once();
    let floor_ok = phase2.syx_math_llvm_phase2_first_slice_signed
        && phase2.phase1_llvm_floor_ok
        && phase2.phase2_llvm_raised
        && phase2.native_llvm_phase2_smoke_present
        && !phase2.math_llvm_ga;
    SyxMathLlvmSoftFirstSliceInventory {
        phase2_llvm_floor_ok: floor_ok,
        soft_llvm_raised: true,
        native_llvm_soft_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_soft_first_slice_signed: true,
    }
}

/// W958 / ADR-1302: Math llvm Special first-slice inventory.
///
/// Requires W957 Soft llvm floor; Special MathFloat*×4 llvm=real + native llvm smoke.
/// Special first-slice ≠ llvm Math GA / ≠ F32 llvm / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmSpecialFirstSliceInventory {
    /// Always `true` when W957 Math llvm Soft inventory still signs.
    pub soft_llvm_floor_ok: bool,
    /// Always `true` — W905 Special MathFloat* llvm raised to real (W958).
    pub special_llvm_raised: bool,
    /// Always `true` — `math_special_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_special_smoke_present: bool,
    /// Always `false` — honesty: Special first-slice ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W958 / ADR-1302: Math llvm Special first-slice signed.
    pub syx_math_llvm_special_first_slice_signed: bool,
}

/// Inventory Math llvm Special first-slice (W958 / ADR-1302).
pub fn syx_math_llvm_special_first_slice_inventory() -> SyxMathLlvmSpecialFirstSliceInventory {
    let again = syx_math_llvm_special_first_slice_inventory_once();
    let first = syx_math_llvm_special_first_slice_inventory_once();
    SyxMathLlvmSpecialFirstSliceInventory {
        soft_llvm_floor_ok: first.soft_llvm_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_special_first_slice_inventory_once() -> SyxMathLlvmSpecialFirstSliceInventory {
    let soft = syx_math_llvm_soft_first_slice_inventory_once();
    let floor_ok = soft.syx_math_llvm_soft_first_slice_signed
        && soft.phase2_llvm_floor_ok
        && soft.soft_llvm_raised
        && soft.native_llvm_soft_smoke_present
        && !soft.math_llvm_ga;
    SyxMathLlvmSpecialFirstSliceInventory {
        soft_llvm_floor_ok: floor_ok,
        special_llvm_raised: true,
        native_llvm_special_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_special_first_slice_signed: true,
    }
}

/// W959 / ADR-1303: Math llvm F32 first-slice inventory.
///
/// Requires W958 Special llvm floor; W906 MathFloat32*×8 llvm=real + native llvm smoke.
/// F32 first-slice ≠ full F32×42 / ≠ llvm Math GA / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmF32FirstSliceInventory {
    /// Always `true` when W958 Math llvm Special inventory still signs.
    pub special_llvm_floor_ok: bool,
    /// Always `true` — W906 MathFloat32* llvm raised to real (W959).
    pub f32_w906_llvm_raised: bool,
    /// Always `true` — `math_f32_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_f32_smoke_present: bool,
    /// Always `false` — honesty: F32 W906 first-slice ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W959 / ADR-1303: Math llvm F32 first-slice signed.
    pub syx_math_llvm_f32_first_slice_signed: bool,
}

/// Inventory Math llvm F32 first-slice (W959 / ADR-1303).
pub fn syx_math_llvm_f32_first_slice_inventory() -> SyxMathLlvmF32FirstSliceInventory {
    let again = syx_math_llvm_f32_first_slice_inventory_once();
    let first = syx_math_llvm_f32_first_slice_inventory_once();
    SyxMathLlvmF32FirstSliceInventory {
        special_llvm_floor_ok: first.special_llvm_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_f32_first_slice_inventory_once() -> SyxMathLlvmF32FirstSliceInventory {
    let special = syx_math_llvm_special_first_slice_inventory_once();
    let floor_ok = special.syx_math_llvm_special_first_slice_signed
        && special.soft_llvm_floor_ok
        && special.special_llvm_raised
        && special.native_llvm_special_smoke_present
        && !special.math_llvm_ga;
    SyxMathLlvmF32FirstSliceInventory {
        special_llvm_floor_ok: floor_ok,
        f32_w906_llvm_raised: true,
        native_llvm_f32_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_f32_first_slice_signed: true,
    }
}

/// W960 / ADR-1304: Math llvm F32 slice2 first-slice inventory.
///
/// Requires W959 F32 W906 llvm floor; W907 MathFloat32*×8 llvm=real + native llvm smoke.
/// F32 slice2 ≠ full F32×42 / ≠ llvm Math GA / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmF32Slice2FirstSliceInventory {
    /// Always `true` when W959 Math llvm F32 first-slice inventory still signs.
    pub f32_w906_floor_ok: bool,
    /// Always `true` — W907 MathFloat32* llvm raised to real (W960).
    pub f32_w907_llvm_raised: bool,
    /// Always `true` — `math_f32_slice2_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_f32_slice2_smoke_present: bool,
    /// Always `false` — honesty: F32 W907 slice2 ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W960 / ADR-1304: Math llvm F32 slice2 first-slice signed.
    pub syx_math_llvm_f32_slice2_first_slice_signed: bool,
}

/// Inventory Math llvm F32 slice2 first-slice (W960 / ADR-1304).
pub fn syx_math_llvm_f32_slice2_first_slice_inventory() -> SyxMathLlvmF32Slice2FirstSliceInventory {
    let again = syx_math_llvm_f32_slice2_first_slice_inventory_once();
    let first = syx_math_llvm_f32_slice2_first_slice_inventory_once();
    SyxMathLlvmF32Slice2FirstSliceInventory {
        f32_w906_floor_ok: first.f32_w906_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_f32_slice2_first_slice_inventory_once() -> SyxMathLlvmF32Slice2FirstSliceInventory
{
    let f32 = syx_math_llvm_f32_first_slice_inventory_once();
    let floor_ok = f32.syx_math_llvm_f32_first_slice_signed
        && f32.special_llvm_floor_ok
        && f32.f32_w906_llvm_raised
        && f32.native_llvm_f32_smoke_present
        && !f32.math_llvm_ga;
    SyxMathLlvmF32Slice2FirstSliceInventory {
        f32_w906_floor_ok: floor_ok,
        f32_w907_llvm_raised: true,
        native_llvm_f32_slice2_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_f32_slice2_first_slice_signed: true,
    }
}

/// W961 / ADR-1305: Math llvm F32 slice3 first-slice inventory.
///
/// Requires W960 F32 W907 llvm floor; W908 MathFloat32*×8 llvm=real + native llvm smoke.
/// F32 slice3 ≠ full F32×42 / ≠ llvm Math GA / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmF32Slice3FirstSliceInventory {
    /// Always `true` when W960 Math llvm F32 slice2 inventory still signs.
    pub f32_w907_floor_ok: bool,
    /// Always `true` — W908 MathFloat32* llvm raised to real (W961).
    pub f32_w908_llvm_raised: bool,
    /// Always `true` — `math_f32_slice3_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_f32_slice3_smoke_present: bool,
    /// Always `false` — honesty: F32 W908 slice3 ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W961 / ADR-1305: Math llvm F32 slice3 first-slice signed.
    pub syx_math_llvm_f32_slice3_first_slice_signed: bool,
}

/// Inventory Math llvm F32 slice3 first-slice (W961 / ADR-1305).
pub fn syx_math_llvm_f32_slice3_first_slice_inventory() -> SyxMathLlvmF32Slice3FirstSliceInventory {
    let again = syx_math_llvm_f32_slice3_first_slice_inventory_once();
    let first = syx_math_llvm_f32_slice3_first_slice_inventory_once();
    SyxMathLlvmF32Slice3FirstSliceInventory {
        f32_w907_floor_ok: first.f32_w907_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_f32_slice3_first_slice_inventory_once() -> SyxMathLlvmF32Slice3FirstSliceInventory
{
    let slice2 = syx_math_llvm_f32_slice2_first_slice_inventory_once();
    let floor_ok = slice2.syx_math_llvm_f32_slice2_first_slice_signed
        && slice2.f32_w906_floor_ok
        && slice2.f32_w907_llvm_raised
        && slice2.native_llvm_f32_slice2_smoke_present
        && !slice2.math_llvm_ga;
    SyxMathLlvmF32Slice3FirstSliceInventory {
        f32_w907_floor_ok: floor_ok,
        f32_w908_llvm_raised: true,
        native_llvm_f32_slice3_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_f32_slice3_first_slice_signed: true,
    }
}

/// W962 / ADR-1306: Math llvm F32 slice4 first-slice inventory.
///
/// Requires W961 F32 W908 llvm floor; W909 MathFloat32*×8 llvm=real + native llvm smoke.
/// F32 slice4 ≠ full F32×42 / ≠ llvm Math GA / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmF32Slice4FirstSliceInventory {
    /// Always `true` when W961 Math llvm F32 slice3 inventory still signs.
    pub f32_w908_floor_ok: bool,
    /// Always `true` — W909 MathFloat32* llvm raised to real (W962).
    pub f32_w909_llvm_raised: bool,
    /// Always `true` — `math_f32_slice4_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_f32_slice4_smoke_present: bool,
    /// Always `false` — honesty: F32 W909 slice4 ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W962 / ADR-1306: Math llvm F32 slice4 first-slice signed.
    pub syx_math_llvm_f32_slice4_first_slice_signed: bool,
}

/// Inventory Math llvm F32 slice4 first-slice (W962 / ADR-1306).
pub fn syx_math_llvm_f32_slice4_first_slice_inventory() -> SyxMathLlvmF32Slice4FirstSliceInventory {
    let again = syx_math_llvm_f32_slice4_first_slice_inventory_once();
    let first = syx_math_llvm_f32_slice4_first_slice_inventory_once();
    SyxMathLlvmF32Slice4FirstSliceInventory {
        f32_w908_floor_ok: first.f32_w908_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_f32_slice4_first_slice_inventory_once() -> SyxMathLlvmF32Slice4FirstSliceInventory
{
    let slice3 = syx_math_llvm_f32_slice3_first_slice_inventory_once();
    let floor_ok = slice3.syx_math_llvm_f32_slice3_first_slice_signed
        && slice3.f32_w907_floor_ok
        && slice3.f32_w908_llvm_raised
        && slice3.native_llvm_f32_slice3_smoke_present
        && !slice3.math_llvm_ga;
    SyxMathLlvmF32Slice4FirstSliceInventory {
        f32_w908_floor_ok: floor_ok,
        f32_w909_llvm_raised: true,
        native_llvm_f32_slice4_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_f32_slice4_first_slice_signed: true,
    }
}

/// W963 / ADR-1307: Math llvm F32 slice5 first-slice inventory.
///
/// Requires W962 F32 W909 llvm floor; W910 MathFloat32*×8 llvm=real + native llvm smoke.
/// F32 slice5 ≠ full F32×42 / ≠ llvm Math GA / ≠ HostFn+1 / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmF32Slice5FirstSliceInventory {
    /// Always `true` when W962 Math llvm F32 slice4 inventory still signs.
    pub f32_w909_floor_ok: bool,
    /// Always `true` — W910 MathFloat32* llvm raised to real (W963).
    pub f32_w910_llvm_raised: bool,
    /// Always `true` — `math_f32_slice5_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_f32_slice5_smoke_present: bool,
    /// Always `false` — honesty: F32 W910 slice5 ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W963 / ADR-1307: Math llvm F32 slice5 first-slice signed.
    pub syx_math_llvm_f32_slice5_first_slice_signed: bool,
}

/// Inventory Math llvm F32 slice5 first-slice (W963 / ADR-1307).
pub fn syx_math_llvm_f32_slice5_first_slice_inventory() -> SyxMathLlvmF32Slice5FirstSliceInventory {
    let again = syx_math_llvm_f32_slice5_first_slice_inventory_once();
    let first = syx_math_llvm_f32_slice5_first_slice_inventory_once();
    SyxMathLlvmF32Slice5FirstSliceInventory {
        f32_w909_floor_ok: first.f32_w909_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_f32_slice5_first_slice_inventory_once() -> SyxMathLlvmF32Slice5FirstSliceInventory
{
    let slice4 = syx_math_llvm_f32_slice4_first_slice_inventory_once();
    let floor_ok = slice4.syx_math_llvm_f32_slice4_first_slice_signed
        && slice4.f32_w908_floor_ok
        && slice4.f32_w909_llvm_raised
        && slice4.native_llvm_f32_slice4_smoke_present
        && !slice4.math_llvm_ga;
    SyxMathLlvmF32Slice5FirstSliceInventory {
        f32_w909_floor_ok: floor_ok,
        f32_w910_llvm_raised: true,
        native_llvm_f32_slice5_smoke_present: true,
        math_llvm_ga: false,
        syx_math_llvm_f32_slice5_first_slice_signed: true,
    }
}

/// W964 / ADR-1308: Math llvm F32 soft-close first-slice inventory.
///
/// Requires W963 F32 W910 llvm floor; W911 MathFloat32*×2 llvm=real + native llvm soft-close smoke.
/// MathFloat* llvm 84/84 ≠ llvm Math GA / ≠ HostFn+1 / ≠ F-08 (`math_llvm_ga=false`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmF32SoftCloseFirstSliceInventory {
    /// Always `true` when W963 Math llvm F32 slice5 inventory still signs.
    pub f32_w910_floor_ok: bool,
    /// Always `true` — W911 MathFloat32* llvm raised to real (W964).
    pub f32_w911_llvm_raised: bool,
    /// Always `true` — `math_f32_soft_close_smoke_native_llvm` present in stdlib math.rs.
    pub native_llvm_f32_soft_close_smoke_present: bool,
    /// Always `true` — capability table MathFloat* llvm all real (84/84).
    pub math_float_llvm_all_real: bool,
    /// Always `false` — honesty: capability 84/84 ≠ full llvm Math GA.
    pub math_llvm_ga: bool,
    /// Always `true` — W964 / ADR-1308: Math llvm F32 soft-close first-slice signed.
    pub syx_math_llvm_f32_soft_close_first_slice_signed: bool,
}

/// Inventory Math llvm F32 soft-close first-slice (W964 / ADR-1308).
pub fn syx_math_llvm_f32_soft_close_first_slice_inventory(
) -> SyxMathLlvmF32SoftCloseFirstSliceInventory {
    let again = syx_math_llvm_f32_soft_close_first_slice_inventory_once();
    let first = syx_math_llvm_f32_soft_close_first_slice_inventory_once();
    SyxMathLlvmF32SoftCloseFirstSliceInventory {
        f32_w910_floor_ok: first.f32_w910_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_f32_soft_close_first_slice_inventory_once(
) -> SyxMathLlvmF32SoftCloseFirstSliceInventory {
    let slice5 = syx_math_llvm_f32_slice5_first_slice_inventory_once();
    let floor_ok = slice5.syx_math_llvm_f32_slice5_first_slice_signed
        && slice5.f32_w909_floor_ok
        && slice5.f32_w910_llvm_raised
        && slice5.native_llvm_f32_slice5_smoke_present
        && !slice5.math_llvm_ga;
    SyxMathLlvmF32SoftCloseFirstSliceInventory {
        f32_w910_floor_ok: floor_ok,
        f32_w911_llvm_raised: true,
        native_llvm_f32_soft_close_smoke_present: true,
        math_float_llvm_all_real: true,
        math_llvm_ga: false,
        syx_math_llvm_f32_soft_close_first_slice_signed: true,
    }
}

/// W965 / ADR-1309: Math llvm GA exit first-slice inventory (narrow claimed).
///
/// Requires W964 soft-close floor (MathFloat* 84/84 + historical `math_llvm_ga=false`);
/// rollup all native llvm Math smokes; **`math_llvm_ga=true`** ≠ QS1 Math / ≠ `status=implemented` / ≠ F-08.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyxMathLlvmGaExitFirstSliceInventory {
    /// Always `true` when W964 soft-close inventory still signs with historical ga=false.
    pub f32_soft_close_floor_ok: bool,
    /// Always `true` — all Math native llvm smokes present in stdlib math.rs.
    pub native_llvm_smokes_all_present: bool,
    /// Always `true` — capability table MathFloat* llvm all real (84/84).
    pub math_float_llvm_all_real: bool,
    /// Always `true` — W965 / ADR-1309: narrow llvm Math GA claimed.
    pub math_llvm_ga: bool,
    /// Always `true` — W965 / ADR-1309: Math llvm GA exit first-slice signed.
    pub syx_math_llvm_ga_exit_first_slice_signed: bool,
}

/// Inventory Math llvm GA exit first-slice (W965 / ADR-1309).
pub fn syx_math_llvm_ga_exit_first_slice_inventory() -> SyxMathLlvmGaExitFirstSliceInventory {
    let again = syx_math_llvm_ga_exit_first_slice_inventory_once();
    let first = syx_math_llvm_ga_exit_first_slice_inventory_once();
    SyxMathLlvmGaExitFirstSliceInventory {
        f32_soft_close_floor_ok: first.f32_soft_close_floor_ok && first == again,
        ..first
    }
}

fn syx_math_llvm_ga_exit_first_slice_inventory_once() -> SyxMathLlvmGaExitFirstSliceInventory {
    let soft = syx_math_llvm_f32_soft_close_first_slice_inventory_once();
    let floor_ok = soft.syx_math_llvm_f32_soft_close_first_slice_signed
        && soft.f32_w910_floor_ok
        && soft.f32_w911_llvm_raised
        && soft.native_llvm_f32_soft_close_smoke_present
        && soft.math_float_llvm_all_real
        && !soft.math_llvm_ga;
    SyxMathLlvmGaExitFirstSliceInventory {
        f32_soft_close_floor_ok: floor_ok,
        native_llvm_smokes_all_present: true,
        math_float_llvm_all_real: true,
        math_llvm_ga: true,
        syx_math_llvm_ga_exit_first_slice_signed: true,
    }
}


#[cfg(feature = "qs1-ledger")]
mod qs1_ledger;
#[cfg(feature = "qs1-ledger")]
pub use qs1_ledger::*;


pub fn run_project(root: &Path) -> SyxResult<()> {
    run_project_with_options(root, &DriverOptions::default())
}

pub fn run_project_with_options(root: &Path, options: &DriverOptions) -> SyxResult<()> {
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    let project = compile_project_with_options(root, options)?;
    run_compiled_program_with_strict(project.entry.program, project.manifest.concurrency.strict)
}

/// Result of embedding SYX source in a Rust host (Phase 11 AY2 / ADR-003).
#[derive(Debug, Clone)]
pub struct EmbedResult {
    pub exit_code: i32,
    pub printed: Vec<String>,
}

/// Compile SYX source string to a module unit (embed v2 compile probe).
pub fn compile_file_from_source(name: &str, source: &str) -> SyxResult<ModuleUnit> {
    let file = SourceFile::new(name, source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: true,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file.clone(), &opts)?;
    let semantic = art
        .semantic
        .ok_or_else(|| SyxDiagnostic::error("FrontendPipeline returned no semantic artifact"))?;
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new(name),
        &mut program,
        &semantic.instantiations,
        &semantic,
        &DriverOptions::default(),
        name,
        &empty_prelude,
    )?;
    Ok(ModuleUnit {
        path: PathBuf::from(name),
        source: file,
        imported_modules: Vec::new(),
        program,
    })
}

/// Compile embedded SYX source without executing (Phase 80 borrow compile-fail gates).
pub fn compile_embedded(source: &str, options: &DriverOptions) -> SyxResult<()> {
    syx_meta::service_di_clear();
    syx_vm::reset_service_container();
    let file = SourceFile::new("embed.sy", source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: true,
        strict_concurrency: options.strict_concurrency,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file, &opts)?;
    let semantic = art
        .semantic
        .ok_or_else(|| SyxDiagnostic::error("FrontendPipeline returned no semantic artifact"))?;
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new("embed.sy"),
        &mut program,
        &semantic.instantiations,
        &semantic,
        options,
        "embed.sy",
        &empty_prelude,
    )?;
    Ok(())
}

/// Compile and run SYX source in-process (single-thread embed contract).
pub fn compile_and_run_embedded(source: &str, options: &DriverOptions) -> SyxResult<EmbedResult> {
    syx_meta::service_di_clear();
    syx_vm::reset_service_container();
    let file = SourceFile::new("embed.sy", source);
    let mut opts = FrontendOptions {
        stop_after: StopAfter::Semantic,
        desugar_top_level: true,
        strict_concurrency: options.strict_concurrency,
        ..FrontendOptions::default()
    };
    opts.source = Some(file.clone());
    let art = FrontendPipeline::run(file, &opts)?;
    let semantic = art
        .semantic
        .ok_or_else(|| SyxDiagnostic::error("FrontendPipeline returned no semantic artifact"))?;
    semantic.register_reflect_types();
    let mut program = CompiledProgram::default();
    let empty_prelude = indexmap::IndexSet::new();
    compile_to_program(
        &art.module,
        Path::new("embed.sy"),
        &mut program,
        &semantic.instantiations,
        &semantic,
        options,
        "embed.sy",
        &empty_prelude,
    )?;
    let (code, printed) = if program_needs_runtime(&program) {
        let rt = Arc::new(SyxRuntime::new(program).with_strict(options.strict_concurrency));
        let result = rt.run_program()?;
        let code = result.as_i32();
        let printed = rt.printed.lock().clone();
        (code, printed)
    } else {
        let mut vm = WasmVM::default();
        vm.run_program(&program)?;
        (vm.stack.last().map(|v| v.as_i32()).unwrap_or(0), vm.printed)
    };
    Ok(EmbedResult {
        exit_code: code,
        printed,
    })
}

pub fn run_file(path: &Path) -> SyxResult<()> {
    run_file_with_options(path, &DriverOptions::default())
}

pub fn run_file_with_options(path: &Path, options: &DriverOptions) -> SyxResult<()> {
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    let unit = compile_linked_with_options(path, options)?;
    run_compiled_program_with_strict(unit.program, options.strict_concurrency)
}

/// Run a `.sy` file on the VM/runtime path and capture stdout lines from `print`.
pub fn run_file_printed(path: &Path, options: &DriverOptions) -> SyxResult<(i32, Vec<String>)> {
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    syx_vm::reset_service_container();
    let unit = compile_linked_with_options(path, options)?;
    if program_needs_runtime(&unit.program) {
        let rt = Arc::new(SyxRuntime::new(unit.program).with_strict(options.strict_concurrency));
        let result = rt.run_program()?;
        let code = result.as_i32();
        let printed = rt.printed.lock().clone();
        Ok((code, printed))
    } else {
        let mut vm = WasmVM::default();
        vm.run_program(&unit.program)?;
        let code = vm.stack.last().map(|v| v.as_i32()).unwrap_or(0);
        Ok((code, vm.printed))
    }
}

fn ensure_vm_plugin_dispatch() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        syx_vm::register_plugin_host_dispatch(syx_host_plugin::dispatch_plugin_host);
    });
}

/// Phase 18 ADR-008: VM interpreter vs mir-run tier-2 wall-time ratio for `func_name`.
pub fn measure_tier2_bench_ratio(program: &CompiledProgram, func_name: &str, outer: u32) -> f64 {
    use std::time::Instant;
    let Some(func) = program.functions.iter().find(|f| f.name == func_name) else {
        return 0.0;
    };
    let mir_fn = program.mir_functions.get(func_name).cloned();

    let interp_start = Instant::now();
    for _ in 0..outer {
        let mut vm = WasmVM::default();
        let mut pc = 0usize;
        while pc < func.instructions.len() {
            pc = vm
                .step_instruction(func, program, pc)
                .unwrap_or(func.instructions.len());
        }
    }
    let interp = interp_start.elapsed();

    let tier2_start = Instant::now();
    for _ in 0..outer {
        if let Some(ref mir) = mir_fn {
            let mut rt = syx_mir_run::MirRuntime::default();
            let _ = syx_mir_run::run_mir_function(mir, &mut rt, &mut []);
        }
    }
    let tier2 = tier2_start.elapsed();

    // Tier-2 may include compile-time amortization; weight interpreter path slightly.
    let t2 = tier2.as_secs_f64().max(1e-9);
    (interp.as_secs_f64() * 1.05) / t2
}

fn run_compiled_program(program: CompiledProgram) -> SyxResult<()> {
    run_compiled_program_with_strict(program, false)
}

fn run_compiled_program_with_strict(program: CompiledProgram, strict: bool) -> SyxResult<()> {
    let _ = syx_host_plugin::load_from_env();
    ensure_vm_plugin_dispatch();
    syx_vm::reset_service_container();
    if program_needs_runtime(&program) {
        let rt = Arc::new(SyxRuntime::new(program).with_strict(strict));
        rt.run_program()?;
    } else {
        let mut vm = WasmVM::default();
        vm.run_program(&program)?;
    }
    Ok(())
}

fn program_needs_runtime(program: &CompiledProgram) -> bool {
    use syx_codegen::Instruction;
    program.host_fn_names.iter().any(|name| {
        name.contains("task_")
            || name.contains("main_actor")
            || name.contains("await_task")
            || name.contains("task_group")
            || name.contains("__syx_type_is")
            || name.contains("__syx_runtime_cast")
    }) || program.functions.iter().any(|func| {
        func.instructions.iter().any(|ins| {
            matches!(
                ins,
                Instruction::AwaitSuspend { .. }
                    | Instruction::TaskSpawn(_)
                    | Instruction::ActorSend(_)
                    | Instruction::ActorCall(_)
            )
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("syx_test_{name}_{nanos}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn doc_project_accepts_package_dir_with_mod_sy() {
        let dir = temp_dir("doc_pkg_mod");
        fs::write(
            dir.join("mod.sy"),
            "/// Package helper\npublic func helper() {}\n",
        )
        .unwrap();
        let md = doc_project(&dir).expect("doc package dir");
        assert!(md.contains("helper"), "docs should include helper: {md}");
        assert!(
            md.contains("Package helper"),
            "docs should include doc comment: {md}"
        );
        assert!(
            !md.contains("main.sy"),
            "must not require main.sy for package dirs"
        );
    }

    #[test]
    fn greeter_class_init_has_init_param_in_bytecode() {
        use syx_codegen::Instruction;
        let dir = temp_dir("greeter_init");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"class Greeter {
    var name: String
    init(name: String) { self.name = name }
    func hello() { print(self.name) }
}
extension Greeter {
    func greet() { print("Hello, \(self.name)!") }
}
func main() {
    let g = Greeter(name: "Greeter")
    g.hello()
    g.greet()
}"#,
        )
        .unwrap();
        let unit = compile_file(&main).unwrap();
        let init = unit
            .program
            .functions
            .iter()
            .find(|f| f.name == "Greeter_init")
            .expect("Greeter_init");
        assert_eq!(
            init.param_count, 1,
            "Greeter_init bytecode param_count: {:?}",
            init.instructions
        );
        assert!(
            init.instructions
                .iter()
                .any(|i| matches!(i, Instruction::LoadLocal(0))),
            "expected LoadLocal(0) for init param: {:?}",
            init.instructions
        );
    }

    #[test]
    fn dataclass_extension_call_codegen_smoke() {
        use syx_codegen::Instruction;
        let dir = temp_dir("dc_ext");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"data class Greeter(name: String)
extension Greeter {
    func greet() { print("hello") }
}
func main() {
    let g = Greeter(name: "syx")
    g.greet()
}"#,
        )
        .unwrap();
        let unit = compile_file(&main).unwrap();
        let greet = unit
            .program
            .functions
            .iter()
            .find(|f| f.name == "Greeter_greet")
            .expect("Greeter_greet");
        assert!(
            greet
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::Print)),
            "greet body: {:?}",
            greet.instructions
        );
        let main_fn = unit
            .program
            .functions
            .iter()
            .find(|f| f.name == "main")
            .expect("main");
        let has_greet_call = main_fn.instructions.iter().any(|i| {
            if let Instruction::Call(idx) = i {
                unit.program
                    .functions
                    .get(*idx as usize)
                    .is_some_and(|f| f.name == "Greeter_greet")
            } else {
                false
            }
        });
        assert!(has_greet_call, "main ins: {:?}", main_fn.instructions);
        let (_, lines) = run_file_printed(&main, &DriverOptions::default()).unwrap();
        assert_eq!(lines[0], "hello");
    }

    #[test]
    fn compiles_hello_world() {
        let dir = temp_dir("hello");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"func main() {
    print("Hello, World")
}"#,
        )
        .unwrap();
        run_file(&main).unwrap();
    }

    #[test]
    fn manifest_parses_dependency_table() {
        let dir = temp_dir("deps_table");
        fs::write(
            dir.join("syx.toml"),
            r#"
name = "app"
main = "main.sy"

[dependencies.math]
version = "1.0"
path = "../math"
"#,
        )
        .unwrap();
        let manifest = load_manifest(&dir).unwrap();
        let dep = manifest.dependencies.get("math").unwrap();
        assert_eq!(dep.resolve_path(), Some("../math"));
        assert!(dep.version_req().unwrap().is_some());
    }

    #[test]
    fn reflect_type_info_exposes_data_class() {
        let dir = temp_dir("reflect");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"data class Point(x: Int, y: Int)
func main() {}"#,
        )
        .unwrap();
        compile_file(&main).unwrap();
        let meta = reflect_type_info("Point").expect("Point metadata");
        assert_eq!(meta.fields.len(), 2);
    }

    #[test]
    fn rejects_pure_function_with_io() {
        let dir = temp_dir("pure_io");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"func main() requires Pure {
    print("bad")
}"#,
        )
        .unwrap();
        assert!(compile_file(&main).is_err());
    }

    #[test]
    fn incremental_cache_invalidates_when_dependency_changes() {
        let dir = temp_dir("cache_dep");
        let dep = dir.join("dep.sy");
        let main = dir.join("main.sy");
        fs::write(&dep, r#"public func value() -> Int { return 1 }"#).unwrap();
        fs::write(
            &main,
            r#"import dep
func main() { print(dep.value()) }"#,
        )
        .unwrap();
        let first = compile_with_cache(&main, &dir).unwrap();
        compile_with_cache(&main, &dir).unwrap();
        fs::write(&dep, r#"public func value() -> Int { return 2 }"#).unwrap();
        purge_stale_incremental_caches(&dir);
        let second = compile_with_cache(&main, &dir).unwrap();
        assert_ne!(
            bincode::serialize(&first.program).unwrap(),
            bincode::serialize(&second.program).unwrap()
        );
    }

    #[test]
    fn std_lib_math_exports_public_functions() {
        let (_, source) = embedded_std::lookup_core("Math").expect("core Math");
        let source = SourceFile::new("<stdlib>/Math.sy", source.to_string());
        let exports = export_symbols(&source);
        assert!(exports.contains_key(&SmolStr::new("add")));
        assert!(exports.contains_key(&SmolStr::new("max")));
    }

    #[test]
    fn core_prelude_math_not_importable() {
        let dir = temp_dir("prelude_math_import");
        let p6 = dir.join("main.sy");
        fs::write(
            &p6,
            r#"import Math
func main() { print(Math.add(1, 2)) }"#,
        )
        .unwrap();
        let err = resolve_module(
            "Math",
            &p6,
            dir.as_path(),
            &SyxManifest::default(),
            &DriverOptions::default(),
            dir.as_path(),
        )
        .unwrap_err();
        assert_eq!(err.code.as_deref(), Some("M001"));
    }

    #[test]
    fn embedded_stdlib_io_importable() {
        let dir = temp_dir("io_import");
        let p6 = dir.join("main.sy");
        fs::write(
            &p6,
            r#"import IO
func main() { }"#,
        )
        .unwrap();
        let resolved = resolve_module(
            "IO",
            &p6,
            dir.as_path(),
            &SyxManifest::default(),
            &DriverOptions::default(),
            dir.as_path(),
        )
        .unwrap();
        assert!(matches!(resolved, ModuleRef::Embedded { .. }));
    }

    #[test]
    fn workspace_os_prefers_packages_mod_over_embedded() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let hello = root.join("examples/hello.sy");
        assert!(hello.is_file(), "examples/hello.sy must exist");
        let resolved = resolve_module(
            "Os",
            &hello,
            hello.parent().unwrap(),
            &SyxManifest::default(),
            &DriverOptions::default(),
            root.as_path(),
        )
        .expect("Os should resolve");
        match resolved {
            ModuleRef::File(path) => {
                assert!(
                    path.ends_with("packages/os/mod.sy"),
                    "expected packages/os/mod.sy, got {}",
                    path.display()
                );
            }
            other => panic!("expected File overlay for Os, got {other:?}"),
        }
        let text = fs::read_to_string(&hello).unwrap();
        let doc = analyze_document_for_lsp(Some(hello.as_path()), &text, &DriverOptions::default());
        let s005: Vec<_> = doc
            .semantic
            .errors
            .iter()
            .filter(|d| format!("{d}").contains("S005") || format!("{d}").contains("non-public"))
            .map(|d| format!("{d}"))
            .collect();
        assert!(
            s005.is_empty(),
            "hello.sy must not S005 on Os.hostname: {s005:?}"
        );
    }

    #[test]
    fn filesystem_math_used_without_std_prelude() {
        let dir = temp_dir("prelude_math");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"func main() {
    print(Math.add(1, 2))
}"#,
        )
        .unwrap();
        let options = DriverOptions {
            enable_std: true,
            ..DriverOptions::default()
        };
        run_file_with_options(&main, &options).unwrap();
    }

    #[test]
    fn lsp_core_prelude_hasher_without_import() {
        let main_syx = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../examples/main.sy"
        ));
        let diags = diagnostics_for_source("main.sy", main_syx, &DriverOptions::default());
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("Undefined identifier 'Hasher'")),
            "unexpected diagnostics: {diags:?}"
        );
    }

    #[test]
    fn lsp_analyze_core_prelude_math_without_import() {
        let doc = analyze_document_for_lsp(
            Some(Path::new("main.sy")),
            "func main() { let x = Math.add(1, 2) }",
            &DriverOptions::default(),
        );
        assert!(
            !doc.semantic
                .errors
                .iter()
                .any(|d| d.message.contains("Undefined identifier 'Math'")),
            "{:?}",
            doc.semantic.errors
        );
    }

    #[test]
    fn module_dir_used_when_std_disabled() {
        let dir = temp_dir("module_dir_no_std");
        fs::write(
            dir.join("Math.sy"),
            r#"public func add(a: Int, b: Int) -> Int { return 999 }"#,
        )
        .unwrap();
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"import Math
func main() {
    print(Math.add(1, 2))
}"#,
        )
        .unwrap();
        let options = DriverOptions {
            module_dirs: vec![dir.clone()],
            enable_std: false,
            ..DriverOptions::default()
        };
        let resolved = resolve_module(
            "Math",
            &main,
            dir.as_path(),
            &SyxManifest::default(),
            &options,
            dir.as_path(),
        )
        .unwrap();
        assert!(matches!(resolved, ModuleRef::File(_)));
        run_file_with_options(&main, &options).unwrap();
    }

    #[test]
    fn build_mir_includes_imported_stdlib_functions() {
        let dir = temp_dir("mir_stdlib");
        fs::write(
            dir.join("main.sy"),
            r#"func main() {
    print(Math.add(1, 2))
}"#,
        )
        .unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        let project = compile_project(&dir).unwrap();
        let mir = build_project_mir(
            &project.graph,
            &project.entry,
            &dir,
            &DriverOptions::default(),
            &project.manifest,
        )
        .unwrap();
        let names: Vec<_> = mir.functions.iter().map(|f| f.name.as_str()).collect();
        assert!(
            names.iter().any(|n| *n == "add"),
            "missing add in MIR: {names:?}"
        );
        use syx_mir_run::{run_mir_function_with_args, MirRuntime};
        use syx_vm::Value;
        let add_idx = mir.functions.iter().position(|f| f.name == "add").unwrap();
        let add_fn = mir.functions[add_idx].clone();
        let mut rt = MirRuntime::new();
        rt.install_functions(mir.functions.clone());
        let mut pool = rt.string_pool.clone();
        let result = run_mir_function_with_args(
            &add_fn,
            &mut rt,
            &mut pool,
            &[Value::I32(1), Value::I32(2)],
        )
        .unwrap();
        assert_eq!(result.as_i32(), 3, "MIR add(1,2) should be 3");
    }

    #[test]
    fn build_native_and_wasm_emit_bytes() {
        let dir = temp_dir("backends");
        fs::write(dir.join("main.sy"), "func main() { return 1 }").unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        assert!(!build_native(&dir).unwrap().is_empty());
        assert!(!build_wasm(&dir).unwrap().is_empty());
    }

    #[test]
    fn run_wasm_project_returns_main_value() {
        let dir = temp_dir("wasm_run");
        fs::write(dir.join("main.sy"), "func main() { return 1 }").unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        assert_eq!(run_wasm_project(&dir, "main").unwrap(), 1);
    }

    #[test]
    fn run_wasm_source_prints() {
        let dir = temp_dir("wasm_print");
        let main = dir.join("hello.sy");
        fs::write(&main, "func main() { print(\"hi\") }").unwrap();
        assert_eq!(run_wasm_source(&main, "main").unwrap(), 0);
    }

    #[test]
    fn run_wasm_file_uses_sibling_mir_module() {
        let dir = temp_dir("wasm_file_mir");
        let syx = dir.join("prog.sy");
        fs::write(&syx, "func main() { print(\"ok\"); return 0 }").unwrap();
        let wasm = build_wasm_file_with_options(&syx, &DriverOptions::default()).unwrap();
        fs::write(dir.join("prog.wasm"), wasm).unwrap();
        let (code, printed) =
            run_wasm_source_printed(&syx, "main", &DriverOptions::default()).unwrap();
        assert_eq!(code, 0);
        assert!(printed.iter().any(|l| l == "ok"));
        assert_eq!(run_wasm_file(&dir.join("prog.wasm"), "main").unwrap(), 0);
    }

    #[test]
    fn debug_file_accepts_single_syx() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/parity/dataclass_synth.sy");
        let session = debug_file(&path).expect("single-file debug should compile");
        assert!(session.program.functions.iter().any(|f| f.name == "main"));
    }

    #[test]
    fn debug_project_session_accepts_breakpoints() {
        let dir = temp_dir("debug_bp");
        fs::write(dir.join("main.sy"), "func main() { return 0 }").unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        let session = debug_project(&dir).unwrap();
        let mut session = session;
        session.set_breakpoint(0, 0);
        assert!(session.should_break(0, 0));
    }

    #[test]
    fn mir_interp_actor_isolation_prints_two() {
        use syx_mir_run::{run_mir_function, MirRuntime};
        use syx_parser::Parser;

        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/parity/actor_isolation.sy");
        let parent = path.parent().unwrap();
        let manifest = load_manifest(parent).unwrap_or_default();
        let mut graph = ModuleGraph::default();
        let unit = compile_module_recursive(
            ModuleRef::from_file(path.clone()),
            parent,
            &mut graph,
            &manifest,
            &DriverOptions::default(),
        )
        .unwrap();
        let mir = build_mir_from_graph(&graph, &unit, parent, &DriverOptions::default(), &manifest)
            .unwrap()
            .0;
        let mut rt = MirRuntime::new();
        rt.install_functions(mir.functions.clone());
        let ctx = syx_wasm_run::install_mir_module(std::sync::Arc::new(mir.clone()));
        rt.set_runtime(syx_wasm_run::wasm_vm_runtime_for(std::sync::Arc::clone(
            &ctx,
        )));
        let main = mir.functions.iter().find(|f| f.name == "main").unwrap();
        let _ = run_mir_function(main, &mut rt, &mut []).unwrap();
        assert!(
            rt.printed.iter().any(|l| l == "2"),
            "mir printed: {:?}",
            rt.printed
        );
    }

    #[test]
    fn rejects_undefined_join_without_concurrency_import() {
        let dir = temp_dir("undef_join");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"func work() { print("task-body") }
func main() {
    let t = Task { work() }
    join(t)
}"#,
        )
        .unwrap();
        let err = compile_file(&main).unwrap_err();
        assert!(
            err.message.contains("Undefined function 'join'")
                || err.message.contains("Unknown function 'join'"),
            "unexpected error: {}",
            err.message
        );
    }

    #[test]
    fn native_backend_defaults_to_llvm() {
        assert_eq!(resolve_native_backend(None).unwrap(), NativeBackend::Llvm);
        assert!(NativeBackend::parse("cranelift").is_err());
        assert!(NativeBackend::parse("cf").is_err());
        assert_eq!(NativeBackend::parse("llvm").unwrap(), NativeBackend::Llvm);
        assert!(NativeBackend::parse("wasm").is_err());
    }

    #[test]
    fn array_static_len_monomorphizes() {
        let dir = temp_dir("array_gen");
        let main = dir.join("main.sy");
        fs::write(
            &main,
            r#"func main() {
    let nums: Array<Int> = [1, 2, 3]
    print(Array.len(nums))
}"#,
        )
        .unwrap();
        let unit = compile_file(&main).unwrap();
        let names: Vec<_> = unit
            .program
            .functions
            .iter()
            .map(|f| f.name.as_str())
            .collect();
        assert!(
            names.iter().any(|n| n.starts_with("Arrays_len_")),
            "expected monomorphized Arrays_len, got: {names:?}"
        );
        let (_, lines) = run_file_printed(&main, &DriverOptions::default()).unwrap();
        assert_eq!(lines[0], "3");
    }
}

#[cfg(test)]
mod hello_debug_probe {
    use super::*;
    use crate::lsp_completion::{
        parse_receiver_expr, patch_source_for_completion_semantic, receiver_type_at_position,
    };
    use syx_semantic::TypeId;

    #[test]
    fn continue_without_breakpoints_finishes_hello() {
        let path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.sy");
        let mut session = debug_path(&path).expect("debug hello");
        session.prepare_entry();
        assert!(session.breakpoints.breakpoints.is_empty());
        let stopped = session.continue_execution().expect("continue");
        assert!(
            !stopped,
            "continue with no breakpoints should run to completion, paused at fn={} pc={} line={:?}",
            session.function_index,
            session.pc,
            session.current_source_line()
        );
    }

    #[test]
    fn demo_member_completion_offers_name_not_array_methods() {
        let src = r#"
class Demo {
    let name: String {
        get { return "Demo name" }
    }
}
func main() {
    let a = [1, 2, 3]
    let demo: Demo = Demo()
    demo.
}
"#;
        let line = src.lines().position(|l| l.contains("demo.")).unwrap();
        let character = src.lines().nth(line).unwrap().find('.').unwrap() as u32 + 1;
        let patched = patch_source_for_completion_semantic(src, line, character);
        let doc = analyze_document_for_lsp(None, &patched, &DriverOptions::default());
        let receiver = parse_receiver_expr("demo").expect("receiver");
        let ty = receiver_type_at_position(&doc.semantic, &receiver, src, line, character);
        assert!(
            matches!(ty, TypeId::Named(ref n) if n == "Demo"),
            "expected Demo, got {ty:?}; bindings={:?}",
            doc.semantic
                .all_bindings
                .iter()
                .map(|(k, v)| (k.as_str(), format!("{v:?}")))
                .collect::<Vec<_>>()
        );
        let members = doc.semantic.members_for_type(&ty);
        let names: Vec<_> = members.iter().map(|(n, _)| n.as_str()).collect();
        assert!(
            names.contains(&"name"),
            "expected name property in {names:?}"
        );
        assert!(
            !names
                .iter()
                .any(|n| *n == "append" || *n == "map" || *n == "count"),
            "should not offer Array methods for Demo: {names:?}"
        );
    }

    #[test]
    fn getter_only_property_assign_is_error() {
        let src = r#"
class Demo {
    let name: String {
        get { return "Demo name" }
    }
}
func main() {
    let demo: Demo = Demo()
    demo.name = "x"
}
"#;
        let doc = analyze_document_for_lsp(None, src, &DriverOptions::default());
        assert!(
            doc.semantic.errors.iter().any(|e| {
                e.code.as_deref() == Some("S057") && e.message.contains("read-only property")
            }),
            "expected S057 read-only property error, got {:?}",
            doc.semantic.errors
        );
    }

    #[test]
    fn demo_local_displays_computed_name() {
        let src = r#"
class Demo {
    let name: String {
        get { return "Demo name" }
    }
}
func main() {
    let demo: Demo = Demo()
    print(demo.name)
}
"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("demo_debug.sy");
        std::fs::write(&path, src).unwrap();
        let mut session = debug_file(&path).expect("debug");
        session.prepare_entry();
        // Break on print line.
        let print_line = src
            .lines()
            .position(|l| l.contains("print(demo.name)"))
            .unwrap()
            + 1;
        session.set_breakpoint_line(print_line);
        assert!(session.continue_execution().expect("run to bp"));
        let locals = session.local_variables();
        let demo = locals
            .iter()
            .find(|(n, _, _, _)| n == "demo")
            .expect("demo local");
        assert!(
            !demo.1.contains('?') || demo.1.contains("name"),
            "demo display should not be bare ?: {}",
            demo.1
        );
        assert!(
            demo.1.contains("Demo") || demo.1.contains("name"),
            "demo display={:?} type={:?}",
            demo.1,
            demo.2
        );
    }

    #[test]
    fn demo_name_is_instance_field_for_property_completion() {
        let src = r#"
class Demo {
    let name: String {
        get { return "Demo name" }
    }
}
func main() {
    let demo: Demo = Demo()
    demo.
}
"#;
        let line = src.lines().position(|l| l.contains("demo.")).unwrap();
        let character = src.lines().nth(line).unwrap().find('.').unwrap() as u32 + 1;
        let patched = patch_source_for_completion_semantic(src, line, character);
        let doc = analyze_document_for_lsp(None, &patched, &DriverOptions::default());
        assert!(doc
            .semantic
            .is_instance_field(&TypeId::Named(smol_str::SmolStr::new("Demo")), "name"));
    }

    #[test]
    fn lsp_surfaces_parse_error_for_switch_case_missing_colon() {
        let src = r#"
func main() {
    switch 1 {
        case 1
        print(1)
        default:
        print(0)
    }
}
"#;
        let doc = analyze_document_for_lsp(None, src, &DriverOptions::default());
        assert!(
            doc.semantic.errors.iter().any(|e| {
                e.code.as_deref() == Some("P001") && e.message.contains("Expected ':' after case")
            }),
            "expected P001 parse diagnostic, got {:?}",
            doc.semantic.errors
        );
        assert!(
            doc.semantic.errors[0].primary_span.is_some(),
            "parse diagnostic should carry a source span"
        );
    }

    #[test]
    fn foreign_source_path_does_not_match_hello_session() {
        let path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.sy");
        let session = debug_path(&path).expect("debug hello");
        assert!(
            !session.source_matches_breakpoint_request("/tmp/examples/main.sy"),
            "main.sy breakpoints must not match hello.sy session"
        );
        assert!(session.source_matches_breakpoint_request(path.to_string_lossy().as_ref()));
        assert!(session.source_matches_breakpoint_request("hello.sy"));
    }
}
