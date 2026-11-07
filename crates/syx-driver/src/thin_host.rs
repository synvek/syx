//! SYX driver thin host / publish / reflect / mono / desugar face.
//! Extracted in S2-I235 (zero semantics).

use crate::DriverOptions;
use smol_str::SmolStr;
use std::path::Path;
use syx_diagnostics::SyxResult;
use syx_meta::{reflect_type, TypeMetadata};
use syx_pkg::{default_registry_url, load_manifest, SyxManifest};

/// Phase 96: dlopen `libsyx_gui` before visible GUI run.
pub fn ensure_gui_loaded() -> bool {
    syx_vm::ensure_gui_loaded()
}

pub fn require_gui_dylib_if_visible() {
    syx_vm::require_gui_dylib_if_visible()
}

/// Serialize farm lease + TCP integration tests.
pub fn farm_lease_test_lock() -> std::sync::MutexGuard<'static, ()> {
    crate::farm_lease_auth::farm_lease_test_lock()
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

pub(crate) fn compile_native_object(mir: &syx_mir::MirModule, options: &DriverOptions) -> SyxResult<Vec<u8>> {
    syx_llvm::compile_native_with_profile(mir, options.build_profile.is_release())
}

pub(crate) const GLOBALS_PACKAGES_REL: &str = "core/builtins/special/globals.sy";

fn is_manifest_main_file(module_key: &Path, manifest: &SyxManifest) -> bool {
    module_key.ends_with(&manifest.main)
        || module_key
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n == manifest.main.as_str())
}

pub(crate) fn should_desugar_top_level_stmts(module_key: &Path, manifest: &SyxManifest) -> bool {
    if is_manifest_main_file(module_key, manifest) {
        return true;
    }
    module_key
        .parent()
        .map(|p| !p.join("syx.toml").exists())
        .unwrap_or(true)
}
