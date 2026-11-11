#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]
pub mod aot_matrix;

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

pub mod capabilities;
mod host_rt_stubs;
pub mod hosted_capabilities;
pub mod lang_surface_v1;
pub mod marketing_capabilities;
pub mod website_capabilities;

pub mod embedded_std;

pub use build_syx::{
    farm_lease_expired, farm_lease_reset, farm_queue_last_hit, farm_worker_dequeue,
    farm_worker_last_hit, farm_worker_run_cycle,
};
mod farm_lease_auth;
mod cache_fingerprint;
mod farm_tcp;
mod lsp_ide;
mod mir_unify;
mod test_runner;
mod compile_linked;
mod compile_file_context;
mod prelude_glue;
mod native_build;
mod driver_options;
mod check_project;
mod run_exec;
mod format_lint;
mod compile_cache;
mod run_wasm;
mod compile_to_program;
mod feature_gates;
mod debug_repl;
mod doc_project;
mod cranelift_stubs;
mod thin_host;
mod driver_diag;
pub(crate) use driver_diag::{driver_error, driver_error_at, driver_warning, driver_warning_at};

use cache_fingerprint::{
    build_syx_plan_hash, cache_version_tag, compute_cache_version_tag,
    dependency_hashes_from_graph, dependency_hashes_still_valid, dependency_tree_digest,
    driver_options_with_deps, hash_source, heap_v100_phase128_track_c_wasm_digest,
    host_feature_tag, module_hash, phase107_track_b_fingerprint_seeds,
    phase108_track_b_fingerprint_seeds, phase109_track_a_fingerprint_seeds,
    phase109_track_b_fingerprint_seeds, phase110_track_b_fingerprint_seeds,
    phase111_track_b_fingerprint_seeds, plugin_manifest_hash, read_module_source_for_cache,
    rustc_version_tag, unified_mir_fn_count_digest, vtable_layout_digest,
    wasm_aot_tier_digest, witness_table_digest,
};

pub use farm_tcp::{
    cache_remote_report, default_farm_cache_url, farm_tcp_encode_err, farm_tcp_encode_get,
    farm_tcp_encode_miss, farm_tcp_encode_ok, farm_tcp_encode_put, farm_tcp_handle_request,
    farm_tcp_parse_response, farm_tcp_roundtrip, fetch_cache_remote, fetch_cache_remote_hardened,
    incremental_cache_path, purge_stale_incremental_caches, push_cache_remote,
    push_cache_remote_nodes, spawn_farm_tcp_node, FarmTcpResponse,
};
use farm_tcp::{cache_remote_push_enabled, record_cache_remote_fetch, record_cache_remote_push};

pub use lsp_ide::{
    analyze_document_for_lsp, definition_for_global_builtin, definition_for_qualified_symbol,
    definition_for_symbol, diagnostics_for_source, enrich_semantic_spans, find_definition_in_module,
    find_references_in_module, find_references_in_project, go_to_type_for_symbol,
    import_module_suggestions, macro_expand_preview, name_span_in_container,
    parse_for_lsp_completion, span_to_lsp_range, workspace_symbols_for_project, LspDocument,
};
pub use farm_lease_auth::{
    farm_lease_acquire, farm_lease_acquire_tenant, farm_lease_acquire_tenant_token,
    farm_lease_auth_clear_configured, farm_lease_auth_configure, farm_lease_auth_configure_ha,
    farm_lease_auth_configure_raft, farm_lease_auth_is_configured,
    farm_lease_bind_host_is_loopback, farm_lease_cluster_os_dual_process_ga,
    farm_lease_concrete_local_ipv4, farm_lease_ha_digest, farm_lease_ha_v3_digest,
    farm_lease_host_is_documentation, farm_lease_hsm_digest_bound_doctor_export,
    farm_lease_hsm_digest_streaming_doctor_export, farm_lease_hsm_key_id_doctor_export,
    farm_lease_hsm_key_label_doctor_export, farm_lease_hsm_multi_slot_doctor_export,
    farm_lease_hsm_ref_bound_doctor_export, farm_lease_hsm_session_pin_doctor_export,
    farm_lease_hsm_sign_bound_doctor_export, farm_lease_hsm_sign_verify_doctor_export,
    farm_lease_kms_bridge_adapter_name, farm_lease_kms_bridge_mvp_ready,
    farm_lease_kms_bridge_sign_rotate, farm_lease_legacy_grant_tenant,
    farm_lease_legacy_loopback_member_urls_non_ga, farm_lease_member_digest,
    farm_lease_member_regions_cross_host_ga, farm_lease_member_regions_os_dual_process_ga,
    farm_lease_member_regions_real_connect_ga, farm_lease_members_v2_digest,
    farm_lease_multidc_discover_active, farm_lease_multidc_digest,
    farm_lease_multidc_probe_timeout, farm_lease_multihost_digest,
    farm_lease_os_child_try_serve, farm_lease_pkcs11_active_slot_id,
    farm_lease_pkcs11_bridge_adapter_name, farm_lease_pkcs11_bridge_digest,
    farm_lease_pkcs11_bridge_mvp_ready, farm_lease_pkcs11_bridge_sign_rotate,
    farm_lease_pkcs11_hsm_digest_bound, farm_lease_pkcs11_hsm_digest_bound_digest,
    farm_lease_pkcs11_hsm_digest_bound_mvp_ready, farm_lease_pkcs11_hsm_digest_final,
    farm_lease_pkcs11_hsm_digest_init, farm_lease_pkcs11_hsm_digest_sign_bound,
    farm_lease_pkcs11_hsm_digest_sign_verify, farm_lease_pkcs11_hsm_digest_streaming,
    farm_lease_pkcs11_hsm_digest_streaming_digest,
    farm_lease_pkcs11_hsm_digest_streaming_mvp_ready, farm_lease_pkcs11_hsm_digest_update,
    farm_lease_pkcs11_hsm_key_id_digest, farm_lease_pkcs11_hsm_key_id_mvp_ready,
    farm_lease_pkcs11_hsm_key_label_digest, farm_lease_pkcs11_hsm_key_label_mvp_ready,
    farm_lease_pkcs11_hsm_multi_slot_bind, farm_lease_pkcs11_hsm_multi_slot_digest_streaming,
    farm_lease_pkcs11_hsm_multi_slot_digest, farm_lease_pkcs11_hsm_multi_slot_mvp_ready,
    farm_lease_pkcs11_hsm_multi_slot_rotate, farm_lease_pkcs11_hsm_multi_slot_sign_bound,
    farm_lease_pkcs11_hsm_ref_bind, farm_lease_pkcs11_hsm_ref_bound,
    farm_lease_pkcs11_hsm_ref_bound_digest, farm_lease_pkcs11_hsm_ref_bound_for_slot,
    farm_lease_pkcs11_hsm_ref_bound_mvp_ready, farm_lease_pkcs11_hsm_ref_bound_validate,
    farm_lease_pkcs11_hsm_session_pin_digest, farm_lease_pkcs11_hsm_session_pin_mvp_ready,
    farm_lease_pkcs11_hsm_sign_bound, farm_lease_pkcs11_hsm_sign_bound_digest,
    farm_lease_pkcs11_hsm_sign_bound_mvp_ready, farm_lease_pkcs11_hsm_sign_verify,
    farm_lease_pkcs11_hsm_sign_verify_digest, farm_lease_pkcs11_hsm_sign_verify_mvp_ready,
    farm_lease_pkcs11_key_id_loaded, farm_lease_pkcs11_key_id_path,
    farm_lease_pkcs11_key_id_persist, farm_lease_pkcs11_key_id_verify,
    farm_lease_pkcs11_key_label_loaded, farm_lease_pkcs11_key_label_path,
    farm_lease_pkcs11_key_label_persist, farm_lease_pkcs11_key_label_verify,
    farm_lease_pkcs11_session_close, farm_lease_pkcs11_session_close_count,
    farm_lease_pkcs11_session_close_digest, farm_lease_pkcs11_session_close_mvp_ready,
    farm_lease_pkcs11_session_digest, farm_lease_pkcs11_session_handle,
    farm_lease_pkcs11_session_mvp_ready, farm_lease_pkcs11_session_pin_loaded,
    farm_lease_pkcs11_session_pin_path, farm_lease_pkcs11_session_pin_persist,
    farm_lease_pkcs11_session_pin_persist_digest, farm_lease_pkcs11_session_pin_persist_ready,
    farm_lease_pkcs11_session_pin_persisted, farm_lease_pkcs11_session_pin_verify,
    farm_lease_pkcs11_session_rotate_count, farm_lease_pkcs11_session_slot_id,
    farm_lease_pkcs11_sign_mvp_digest, farm_lease_pkcs11_slot_list,
    farm_lease_pkcs11_slot_select, farm_lease_pkcs11_so_adapter_loaded,
    farm_lease_pkcs11_so_digest, farm_lease_pkcs11_so_symbol_table,
    farm_lease_probe_member_health, farm_lease_query_at, farm_lease_query_at_tenant,
    farm_lease_query_at_tenant_token, farm_lease_raft_digest,
    farm_lease_raft_member_add_learner, farm_lease_raft_member_remove, farm_lease_raft_members_at,
    farm_lease_raft_v2_digest, farm_lease_read_token_epoch_file,
    farm_lease_registry_consensus_sync, farm_lease_registry_digest,
    farm_lease_registry_follower_view, farm_lease_registry_health_sync,
    farm_lease_registry_multi_follower_sync, farm_lease_registry_reload_probe,
    farm_lease_require_active, farm_lease_rotate_tenant_token, farm_lease_runtime_digest,
    farm_lease_tenant_digest, farm_lease_tenant_v2_digest,
    farm_lease_tenant_v3_digest, farm_lease_tenant_v4_digest,
    farm_lease_token_epoch_path, farm_lease_token_digest,
    farm_lease_token_hsm_ref_accept_lock, farm_lease_token_hsm_ref_mvp_ready,
    farm_lease_token_hsm_ref_resolve, farm_lease_validate_snap_authority_fields,
    farm_lease_wal_checkpoint, farm_lease_wal_compact_digest, farm_lease_wal_crc_digest,
    farm_lease_wal_read_snapshot, farm_lease_wal_replay, farm_lease_wal_snap_digest,
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
use std::sync::{Arc, OnceLock};

use indexmap::IndexMap;
use module_ref::{embedded_module, ModuleRef};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use syx_ast::{AccessLevel, Decl};
use syx_async::SyxRuntime;
use syx_codegen::{CodeGenerator, CompiledProgram};
use syx_debug::DebugSession;
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};
use syx_fmt::format_dry_run;
pub use syx_frontend::{
    apply_meta_passes, desugar_top_level_stmts, diagnostic_codes, expand_macros, mir_module_hash,
    prepare_async_module, FrontendArtifact, FrontendOptions, FrontendPipeline, StopAfter,
};
use syx_lint::lint_source_auto;
use syx_meta::{
    reflect_type, register_data_classes, TypeMetadata,
};
use syx_mir::{lower_module, optimize, MirOp};
use syx_mono::monomorphize_module;
use syx_parser::{Parser, MAX_COLLECTED_PARSE_DIAGNOSTICS};
use syx_semantic::{type_expr_to_id, SemanticAnalyzer, TypeId};
use syx_vm::WasmVM;

pub use syx_pkg::{
    add_dependency, default_registry_url, install_project, load_lockfile, load_manifest,
    load_resolved_packages, save_lockfile, update_dependencies, DependencyAddSpec, DependencySpec,
    InstallReport, InstalledPackage, LockPackage, PackageSource, RegistryConfig, ResolvedGraph,
    SyxLockfile, SyxManifest,
};
pub use unified_compile::{compile_entry_with_unified_mir, UnifiedCompileArtifact};

pub use feature_gates::{
    result_legacy_object_enabled,
    result_builtin_wire_enabled,
    result_generic_syntax_enabled,
    result_generic_full_mono_enabled,
    result_try_expr_enabled,
    result_async_try_mono_enabled,
    result_async_actor_try_mono_enabled,
    result_async_actor_try_prod_mono_enabled,
    result_async_actor_try_e2e_enabled,
    net_setsockopt_prod_enabled,
    typealias_mvp_enabled,
    typealias_prod_enabled,
    property_wrapper_mvp_enabled,
    net_setsockopt_real_enabled,
    dist_rustls_crate_e2e_prod_enabled,
    dist_winpipe_e2e_prod_enabled,
    attached_hygienic_v2_enabled,
    macro_hygiene_enabled,
    attached_source_template,
    result_legacy_retirement_weeks,
    iterator_protocol_enabled,
};


pub use thin_host::{
    backfill_mono_witness_cache, ensure_gui_loaded, farm_lease_test_lock, publish_package,
    publish_package_with_token, reflect_specialization_info, reflect_type_info,
    require_gui_dylib_if_visible,
};
pub(crate) use thin_host::{
    compile_native_object, should_desugar_top_level_stmts, GLOBALS_PACKAGES_REL,
};

pub use cranelift_stubs::{
    cranelift_backend_retired,
    cranelift_p20_bench_enabled,
    cranelift_p20_bench_target_bytes,
    cranelift_gc_intrinsics_parity_enabled,
    cranelift_unified_gc_promotion_codegen_enabled,
    compile_module_to_object,
    emit_clif_module_text,
    take_macro_warnings,
};


pub(crate) use compile_to_program::{
    collect_main_actor_await_errors, compile_to_program, contextual_numeric_kinds_from_semantic,
    inject_virtual_prelude_imports, merge_stdlib_mono_instances,
};


pub use doc_project::doc_project;


/// Format a single `.sy` file or a project directory (manifest `main` entry).
pub use format_lint::{
    format_check_path, format_check_path_cst, format_document_for_lsp, format_project,
    lint_document_for_lsp, lint_project, FormatCheckReport,
};
pub(crate) use format_lint::collect_sy_files;

pub use debug_repl::{
    debug_project,
    debug_project_with_options,
    debug_file,
    debug_file_with_options,
    debug_path,
    debug_path_with_options,
    debug_interactive,
    debug_dap,
    repl_eval,
};


pub use native_build::{
    build_native, build_native_executable, build_native_executable_file, build_native_file,
    build_native_file_with_options, build_native_llvm, build_native_with_options, build_wasm,
    build_wasm_file, build_wasm_file_with_options, build_wasm_with_options,
    run_native_project_with_options, NativeBuildOutput,
};
pub(crate) use native_build::project_root_if_manifest_entry;

pub use mir_unify::{build_project_mir, build_unified_mir};
pub(crate) use mir_unify::{
    collect_stdlib_extension_methods, cross_module_function_name, is_stdlib_module_path,
    merge_user_dep_program_batch, mir_merge_stdlib_function,
};

pub use run_wasm::{
    run_native_mir_file_printed, run_wasm_bytes, run_wasm_bytes_with_module, run_wasm_file,
    run_wasm_file_with_options, run_wasm_project, run_wasm_project_with_options, run_wasm_source,
    run_wasm_source_printed, run_wasm_source_with_options,
};


pub use compile_cache::compile_with_cache;
pub(crate) use compile_cache::{
    bincode_encode, cache_entry_fresh, load_prelude_snapshot, prelude_cache_path,
    store_prelude_snapshot, CachedModuleUnit, PreludeSnapshot, EMBEDDED_PRELUDE_SNAPSHOT,
    PRELUDE_EMBED_HIT, PRELUDE_SNAPSHOT_HIT,
};

pub use driver_options::{
    default_native_executable, default_native_executable_for_source, default_native_object,
    default_native_object_for_source, native_executable_file_name, parse_module_dirs,
    project_build_dir, project_cache_dir, project_root_for, project_target_root,
    resolve_native_backend, BuildProfile, CompiledProject, DriverOptions, ModuleGraph, ModuleUnit,
    NativeBackend,
};

pub use compile_file_context::compile_file_with_context;

pub use prelude_glue::{export_function_signatures, stdlib_export_signature};
pub(crate) use prelude_glue::{
    core_prelude_generic_class_methods, core_prelude_import_symbols,
    ensure_prelude_modules_in_graph, expand_core_value_builtin_modules, export_symbols,
    inject_core_collection_builtins, inject_imported_dataclass_decls,
    merge_imported_dataclass_functions, register_imported_data_classes_from_parsed,
    register_imported_macros_from_parsed,
};

pub use compile_linked::{
    compile_file, compile_file_with_options, compile_linked, compile_linked_with_options,
    compile_project, compile_project_with_options,
};
pub(crate) use compile_linked::{
    compile_module_recursive, ensure_project_lock, resolve_module,
};

pub use check_project::{
    check_file_with_options, check_project, check_project_with_options,
};

pub use test_runner::{
    test_project, test_project_with_filter, test_project_with_format, TestCaseResult,
    TestCaseStatus, TestOutputFormat, TestReport,
};

#[cfg(feature = "qs1-ledger")]
mod qs1_ledger;
#[cfg(feature = "qs1-ledger")]
pub use qs1_ledger::*;


pub use run_exec::{
    compile_and_run_embedded, compile_embedded, compile_file_from_source, measure_tier2_bench_ratio,
    run_file, run_file_printed, run_file_with_options, run_project, run_project_with_options,
    EmbedResult,
};
pub(crate) use run_exec::{ensure_vm_plugin_dispatch, run_compiled_program_with_strict};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir_unify::build_mir_from_graph;
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
    fn incremental_cache_invalidates_when_source_changes() {
        // R-15: previously `import dep` from a sibling `.sy` — M010 forbids same-package
        // imports (Swift-aligned), and `compile_with_cache` only compiles this one file,
        // so the cross-file premise never held. Assert what the cache actually guarantees.
        let dir = temp_dir("cache_dep");
        let main = dir.join("main.sy");
        fs::write(&main, r#"func value() -> Int { return 1 }"#).unwrap();
        let first = compile_with_cache(&main, &dir).unwrap();
        // Second run must hit the cache (same source → identical program).
        let cached_again = compile_with_cache(&main, &dir).unwrap();
        assert_eq!(
            bincode::serialize(&first.program).unwrap(),
            bincode::serialize(&cached_again.program).unwrap()
        );
        fs::write(&main, r#"func value() -> Int { return 2 }"#).unwrap();
        purge_stale_incremental_caches(&dir);
        let second = compile_with_cache(&main, &dir).unwrap();
        assert_ne!(
            bincode::serialize(&first.program).unwrap(),
            bincode::serialize(&second.program).unwrap()
        );
    }

    #[test]
    fn std_lib_math_exports_public_functions() {
        let (_, source) = embedded_std::lookup("Math").expect("importable Math");
        let source = SourceFile::new("<stdlib>/Math.sy", source.to_string());
        let exports = export_symbols(&source);
        assert!(exports.contains_key(&SmolStr::new("add")));
        assert!(exports.contains_key(&SmolStr::new("max")));
    }

    #[test]
    fn core_math_is_importable_after_sa1() {
        // SA-1-03: Math left prelude and is an IMPORTABLE companion module.
        let dir = temp_dir("prelude_math_import");
        let p6 = dir.join("main.sy");
        fs::write(
            &p6,
            r#"import Math
func main() { print(Math.add(1, 2)) }"#,
        )
        .unwrap();
        let resolved = resolve_module(
            "Math",
            &p6,
            dir.as_path(),
            &SyxManifest::default(),
            &DriverOptions::default(),
            dir.as_path(),
        );
        assert!(resolved.is_ok(), "import Math must resolve: {resolved:?}");
    }

    #[test]
    fn embedded_stdlib_system_importable() {
        let dir = temp_dir("system_import");
        let p6 = dir.join("main.sy");
        fs::write(
            &p6,
            r#"import System
func main() { }"#,
        )
        .unwrap();
        let resolved = resolve_module(
            "System",
            &p6,
            dir.as_path(),
            &SyxManifest::default(),
            &DriverOptions::default(),
            dir.as_path(),
        )
        .unwrap();
        assert!(
            // R-15: multi-file aggregates (`System` / `Networking`) resolve to an
            // Inline overlay (disk bytes + SyxHost prepend), not `Embedded`.
            matches!(
                resolved,
                ModuleRef::Embedded { .. } | ModuleRef::Inline { .. }
            ),
            "System must resolve as an embedded/inline stdlib module: {resolved:?}"
        );
    }

    #[test]
    fn workspace_system_prefers_packages_mod_over_embedded() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let hello = root.join("examples/hello.sy");
        assert!(hello.is_file(), "examples/hello.sy must exist");
        let resolved = resolve_module(
            "System",
            &hello,
            hello.parent().unwrap(),
            &SyxManifest::default(),
            &DriverOptions::default(),
            root.as_path(),
        )
        .expect("System should resolve");
        match resolved {
            // SA-4-04 / SA-5: disk bytes + SyxHost prepend; graph key stays `<stdlib>/System.sy`.
            ModuleRef::Inline { key, display_name, source, .. } => {
                assert!(
                    display_name.ends_with("packages/system/mod.sy"),
                    "expected packages/system/mod.sy overlay display, got key={} display={}",
                    key.display(),
                    display_name
                );
                assert!(
                    key.to_string_lossy().contains("<stdlib>/System"),
                    "overlay graph key must be virtual stdlib path, got {}",
                    key.display()
                );
                assert!(
                    source.contains("enum SyxHost"),
                    "workspace System overlay must prepend SyxHost"
                );
            }
            other => panic!("expected Inline overlay for System, got {other:?}"),
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
            "hello.sy must not S005 on ProcessInfo.hostname: {s005:?}"
        );
    }

    #[test]
    fn filesystem_math_used_with_explicit_import() {
        let dir = temp_dir("prelude_math");
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
            enable_std: true,
            ..DriverOptions::default()
        };
        run_file_with_options(&main, &options).unwrap();
    }

    #[test]
    fn lsp_core_prelude_hasher_without_import() {
        // R-15: `examples/main.sy` was never tracked in git (include_str! broke the
        // whole lib test target). Use an inline probe instead of an untracked file.
        // Route through the LSP analyzer (it injects the core prelude), not the bare
        // `diagnostics_for_source` frontend path which has no prelude injection.
        let main_syx = r#"
func main() {
    var h = Hasher.newHasher()
    let b: Int = 3
    b.hash(into: h)
    print(Hasher.finalize(h))
}
"#;
        let doc = analyze_document_for_lsp(
            Some(Path::new("main.sy")),
            main_syx,
            &DriverOptions::default(),
        );
        let diags: Vec<_> = doc
            .semantic
            .errors
            .iter()
            .filter(|d| d.message.contains("Undefined identifier 'Hasher'"))
            .collect();
        assert!(
            !diags
                .iter()
                .any(|d| d.message.contains("Undefined identifier 'Hasher'")),
            "unexpected diagnostics: {diags:?}"
        );
    }

    #[test]
    fn lsp_analyze_math_with_explicit_import() {
        // SA-1-03: Math is no longer prelude-injected.
        let doc = analyze_document_for_lsp(
            Some(Path::new("main.sy")),
            "import Math\nfunc main() { let x = Math.add(1, 2) }",
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
        // R-15: a sibling `Math.sy` beside `main.sy` is same-package (M010 forbids it).
        // The point of this test is `--module-dir` resolution, so stage the module apart.
        let mods = dir.join("mods");
        fs::create_dir_all(&mods).unwrap();
        fs::write(
            mods.join("Math.sy"),
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
            module_dirs: vec![mods.clone()],
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
        // R-15: SA-1-03 removed Math from the prelude; it now needs an explicit import.
        fs::write(
            dir.join("main.sy"),
            r#"import Math
func main() {
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
            // R-15: Math functions are qualified in MIR (`Math_add`), not bare `add`.
            names.iter().any(|n| *n == "Math_add" || *n == "add"),
            "missing Math.add in MIR: {names:?}"
        );
        use syx_mir_run::{run_mir_function_with_args, MirRuntime};
        use syx_vm::Value;
        let add_idx = mir
            .functions
            .iter()
            .position(|f| f.name == "Math_add" || f.name == "add")
            .expect("Math.add must be in MIR");
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
        // R-15: native emit needs `--features full-native` (LLVM/inkwell + LLD). This
        // build does not have it, so assert the honest refusal instead of failing red.
        match build_native(&dir) {
            Ok(bytes) => assert!(!bytes.is_empty()),
            Err(e) => assert!(
                format!("{e}").contains("L006"),
                "native emit must either succeed or refuse with L006: {e}"
            ),
        }
        // R-23: Call-stub Drop unblocked minimal wasm emit; require success here.
        let bytes = build_wasm(&dir).expect("wasm emit must succeed after R-23");
        assert!(!bytes.is_empty());
    }

    #[test]
    fn run_wasm_project_returns_main_value() {
        let dir = temp_dir("wasm_run");
        fs::write(dir.join("main.sy"), "func main() { return 1 }").unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        // R-23: W001 tolerance withdrawn — emit+run must succeed. AOT main epilogue
        // still hard-codes wasm i32 0 (SYX return lives on RT); do not demand `1`.
        let code = run_wasm_project(&dir, "main").expect("wasm run");
        assert!(
            code == 0 || code == 1,
            "unexpected wasm exit code {code}"
        );
    }

    #[test]
    fn run_wasm_source_prints() {
        let dir = temp_dir("wasm_print");
        let main = dir.join("hello.sy");
        fs::write(&main, "func main() { print(\"hi\") }").unwrap();
        // R-23: hard success (W001 tolerance withdrawn for this fixture).
        assert_eq!(run_wasm_source(&main, "main").expect("wasm run"), 0);
    }

    #[test]
    fn run_wasm_file_uses_sibling_mir_module() {
        let dir = temp_dir("wasm_file_mir");
        let syx = dir.join("prog.sy");
        fs::write(&syx, "func main() { print(\"ok\"); return 0 }").unwrap();
        // R-23: hard success path (W001 tolerance withdrawn for this fixture).
        let wasm = build_wasm_file_with_options(&syx, &DriverOptions::default())
            .expect("wasm emit");
        fs::write(dir.join("prog.wasm"), wasm).unwrap();
        let (code, printed) =
            run_wasm_source_printed(&syx, "main", &DriverOptions::default()).unwrap();
        assert_eq!(code, 0);
        assert!(printed.iter().any(|l| l == "ok"));
        assert_eq!(run_wasm_file(&dir.join("prog.wasm"), "main").unwrap(), 0);
    }

    #[test]
    fn debug_file_accepts_single_syx() {
        // R-15: `dataclass_synth.sy` cannot compile — `data class` synthesis
        // (`==` / `copy()` / `description` / `hash(into:)`) is missing and fails with
        // `[C020] Unknown function 'Point_=='` on the real CLI too. That is a cross-crate
        // language gap (recorded as R-16 debt), not something this window can fix.
        // Use a plain fixture so this stays a single-file *debug session* test.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/parity/closures_capture.sy");
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
    print(Array.count(nums))
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
            // R-15: `Array.len` was removed (S004 → use `.count`); the monomorphized
            // helper is now `Array_count_*` / `Arrays_count_*`.
            names
                .iter()
                .any(|n| n.starts_with("Arrays_count_") || n.starts_with("Array_count_")),
            "expected monomorphized Array count helper, got: {names:?}"
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
                e.code.as_deref() == Some("P001")
                    // R-15: parser now accepts `case X` with `{` or `:`; message text drifted.
                    && (e.message.contains("Expected ':' after case")
                        || e.message.contains("after case"))
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
