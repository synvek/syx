//! SYX driver native/wasm build face.
//! Extracted in S2-I223 (zero semantics).

use crate::cache_fingerprint::{cache_version_tag, driver_options_with_deps};
use crate::module_ref::ModuleRef;
use crate::{
    build_unified_mir, compile_module_recursive, compile_native_object, compile_project_with_options,
    ensure_project_lock, native_executable_file_name, project_cache_dir, BuildProfile,
    DriverOptions, ModuleGraph, NativeBackend,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_pkg::{load_manifest, load_resolved_packages};

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

pub(crate) fn project_root_if_manifest_entry(path: &Path) -> Option<PathBuf> {
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

    // Prefer in-process / prefix LLD when full-native wired syx-lld into the CLI.
    // LLD must NOT be linked into libsyx_rt.a (that made every native app link pull LLVM).
    #[cfg(feature = "full-native")]
    {
        let force_cc = std::env::var("SYX_FORCE_CC_LINK")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let disable_lld = std::env::var("SYX_DISABLE_LLD")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if !force_cc && !disable_lld && syx_lld::available() {
            let rt = if merged.rt_profile.is_empty() {
                syx_rt::rt_archive_path()
            } else {
                syx_rt::rt_archive_path_for_profile(&merged.rt_profile)
            }
            .map_err(|e| SyxDiagnostic::error(format!("Failed to locate syx-rt archive: {e}")))?;
            match syx_rt::native_lld_argv(obj_path, output, &merged, &rt)
                .and_then(|args| syx_lld::run_lld(&args))
            {
                Ok(()) => return Ok(()),
                Err(lld_err) => {
                    let force_lld = std::env::var("SYX_FORCE_LLD")
                        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                        .unwrap_or(false);
                    if force_lld {
                        return Err(SyxDiagnostic::error(format!(
                            "Failed to link native executable with LLD: {lld_err}"
                        )));
                    }
                    // Fall through to host cc / link.exe.
                }
            }
        }
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

fn native_input_digest(path: &Path, options: &DriverOptions) -> String {
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
        native_input_digest(source, options),
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
        fp == native_input_digest(source, options)
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
