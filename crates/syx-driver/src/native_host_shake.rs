//! Native link-time host shaking: slim `libsyx_rt.a` + optional `syx-host-{sql,http,tls}`.
//!
//! Archives are **prebuilt only** — `syx build --native` never spawns `cargo` (SDK users have
//! no Rust toolchain; spawning a build tool from the linker path is also a security footgun).
//! Prebuild with `scripts/build_native_shake_libs.sh` (dev) or the release SDK packager.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use syx_diagnostics::{Span, SyxResult};
use syx_mir::{collect_call_host_indices_reachable, MirModule};
use syx_stdlib::{shake_domains_for_indices, HostShakeDomain};

use crate::{driver_error_at, BuildProfile, DriverOptions};

/// Isolated cargo target dir used by `scripts/build_native_shake_libs.sh` (dev / CI prebuild).
pub fn native_shake_target_dir(workspace: &Path) -> PathBuf {
    workspace.join("target/native-host-shake")
}

fn profile_name(release: bool) -> &'static str {
    if release {
        "release"
    } else {
        "debug"
    }
}

fn rt_archive_name() -> &'static str {
    #[cfg(windows)]
    {
        "syx_rt.lib"
    }
    #[cfg(not(windows))]
    {
        "libsyx_rt.a"
    }
}

fn host_archive_name(domain: HostShakeDomain) -> String {
    let stem = domain.archive_package().replace('-', "_");
    #[cfg(windows)]
    {
        format!("{stem}.lib")
    }
    #[cfg(not(windows))]
    {
        format!("lib{stem}.a")
    }
}

fn first_existing(candidates: &[PathBuf]) -> Option<PathBuf> {
    candidates.iter().find(|p| p.is_file()).cloned()
}

/// Search roots for prebuilt slim rt / host domain archives (never builds).
fn shake_lib_search_dirs(release: bool) -> Vec<PathBuf> {
    let profile = profile_name(release);
    let mut dirs = Vec::new();

    if let Ok(raw) = std::env::var("SYX_NATIVE_SHAKE_LIB_DIR") {
        let p = PathBuf::from(raw);
        if !p.as_os_str().is_empty() {
            dirs.push(p);
        }
    }

    // Official SDK: `bin/syx` → `../lib` (+ `../lib/hosts` for domain archives).
    if let Ok(exe) = std::env::current_exe() {
        if let Some(bin) = exe.parent() {
            let lib = bin.join("../lib");
            dirs.push(lib.join("hosts"));
            dirs.push(lib.clone());
            dirs.push(lib.join("native-host-shake").join(profile));
        }
    }

    // Dev checkout: walk cwd ancestors for `target/native-host-shake/<profile>`.
    if let Ok(cwd) = std::env::current_dir() {
        for anc in cwd.ancestors().take(8) {
            dirs.push(anc.join("target/native-host-shake").join(profile));
        }
    }

    // Binary built from this tree: baked workspace path may still exist on the build machine.
    let baked = workspace_root()
        .join("target/native-host-shake")
        .join(profile);
    dirs.push(baked);

    dirs
}

fn resolve_rt_archive(release: bool) -> Option<PathBuf> {
    if let Ok(raw) = std::env::var("SYX_RT_STATIC_LIB") {
        let p = PathBuf::from(raw);
        if p.is_file() {
            return Some(p);
        }
        // Directory containing the archive.
        let nested = p.join(rt_archive_name());
        if nested.is_file() {
            return Some(nested);
        }
    }

    if let Ok(id) = std::env::var("SYX_RT_PROFILE") {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(bin) = exe.parent() {
                let lib = bin.join("../lib");
                for c in [
                    lib.join("profiles").join(&id).join(rt_archive_name()),
                    lib.join(&id).join(rt_archive_name()),
                ] {
                    if c.is_file() {
                        return Some(c);
                    }
                }
            }
        }
    }

    let mut candidates = Vec::new();
    for dir in shake_lib_search_dirs(release) {
        candidates.push(dir.join(rt_archive_name()));
    }
    if let Some(p) = first_existing(&candidates) {
        return Some(p);
    }

    // Last resort: existing syx-rt locator (SDK flat lib / cargo target from a prior build).
    syx_rt::rt_archive_path().ok()
}

fn resolve_host_archive(domain: HostShakeDomain, release: bool, rt: &Path) -> Option<PathBuf> {
    let name = host_archive_name(domain);
    let mut candidates = Vec::new();
    if let Some(parent) = rt.parent() {
        candidates.push(parent.join(&name));
        candidates.push(parent.join("hosts").join(&name));
    }
    for dir in shake_lib_search_dirs(release) {
        candidates.push(dir.join(&name));
    }
    first_existing(&candidates)
}

fn missing_archive_hint() -> &'static str {
    "prebuild with `bash scripts/build_native_shake_libs.sh` (dev/CI), or install an SDK that ships `lib/libsyx_rt.a` (+ `lib/hosts/` for sql/http/tls); set SYX_RT_STATIC_LIB / SYX_NATIVE_SHAKE_LIB_DIR to override"
}

/// Resolve prebuilt slim `syx-rt` (+ needed host domain staticlibs). **Never** spawns cargo.
pub fn ensure_shake_archives(
    _workspace: &Path,
    domains: &[HostShakeDomain],
    options: &DriverOptions,
) -> SyxResult<ShakeLinkArtifacts> {
    let release = options.build_profile == BuildProfile::Release;
    let rt = resolve_rt_archive(release).ok_or_else(|| {
        driver_error_at(
            format!(
                "slim libsyx_rt archive not found ({})",
                missing_archive_hint()
            ),
            None,
            Span::DUMMY,
            "N001",
        )
    })?;

    let mut host_archives = Vec::new();
    for domain in domains {
        let archive = resolve_host_archive(*domain, release, &rt).ok_or_else(|| {
            driver_error_at(
                format!(
                    "host domain archive missing for {} ({}; expected {})",
                    domain.as_str(),
                    missing_archive_hint(),
                    host_archive_name(*domain)
                ),
                None,
                Span::DUMMY,
                "N001",
            )
        })?;
        host_archives.push(archive);
    }

    Ok(ShakeLinkArtifacts {
        rt_archive: rt,
        host_archives,
        domains: domains.to_vec(),
    })
}

#[derive(Debug, Clone)]
pub struct ShakeLinkArtifacts {
    pub rt_archive: PathBuf,
    pub host_archives: Vec<PathBuf>,
    pub domains: Vec<HostShakeDomain>,
}

/// Domains required by `module`'s reachable CallHost set (main ∪ aot roots).
pub fn domains_for_module(module: &MirModule) -> Vec<HostShakeDomain> {
    let indices = collect_call_host_indices_reachable(module);
    shake_domains_for_indices(&indices)
}

/// Emit a small C object that calls domain `*_install` before main (constructor).
/// Uses host `cc` only (same requirement as native link); does not invoke cargo.
pub fn write_host_install_object(
    out_dir: &Path,
    domains: &[HostShakeDomain],
) -> SyxResult<Option<PathBuf>> {
    if domains.is_empty() {
        return Ok(None);
    }
    let _ = std::fs::create_dir_all(out_dir);
    let c_path = out_dir.join("syx_host_shake_install.c");
    let o_path = out_dir.join("syx_host_shake_install.o");
    let mut src = String::from("/* generated: native host-shake install */\n");
    for d in domains {
        src.push_str(&format!("extern void {}(void);\n", d.install_symbol()));
    }
    src.push_str(
        "\n#if defined(__GNUC__) || defined(__clang__)\n\
         __attribute__((constructor))\n\
         #endif\n\
         void syx_host_shake_autoload(void) {\n",
    );
    for d in domains {
        src.push_str(&format!("  {}();\n", d.install_symbol()));
    }
    src.push_str("}\n");
    std::fs::write(&c_path, src).map_err(|e| {
        driver_error_at(
            format!("Failed to write host install C: {e}"),
            None,
            Span::DUMMY,
            "D001",
        )
    })?;
    let status = Command::new("cc")
        .args([
            "-c",
            "-O2",
            "-fno-common",
            "-o",
            o_path.to_str().unwrap_or("syx_host_shake_install.o"),
            c_path.to_str().unwrap_or("syx_host_shake_install.c"),
        ])
        .status()
        .map_err(|e| {
            driver_error_at(
                format!("Failed to compile host install object (need cc): {e}"),
                None,
                Span::DUMMY,
                "N003",
            )
        })?;
    if !status.success() {
        return Err(driver_error_at(
            "cc failed compiling syx_host_shake_install.c",
            None,
            Span::DUMMY,
            "N003",
        ));
    }
    Ok(Some(o_path))
}

/// Human-readable shake report for doctor / diagnostics.
pub fn format_host_shake_report(module: &MirModule) -> String {
    let indices: BTreeSet<u32> = collect_call_host_indices_reachable(module);
    let domains = shake_domains_for_indices(&indices);
    let mut lines = Vec::new();
    lines.push(format!(
        "native-host-shake call_host_count={}",
        indices.len()
    ));
    if domains.is_empty() {
        lines.push("native-host-shake domains=<none>".into());
    } else {
        let names: Vec<&str> = domains.iter().map(|d| d.as_str()).collect();
        lines.push(format!("native-host-shake domains={}", names.join(",")));
    }
    for d in HostShakeDomain::all() {
        let touched = domains.contains(d);
        lines.push(format!(
            "native-host-shake {}={} archive={}",
            d.as_str(),
            if touched { "link" } else { "omit" },
            d.archive_package()
        ));
    }
    lines.push(
        "native-host-shake note=prebuilt archives only (never spawns cargo at link time)".into(),
    );
    lines.join("\n")
}

/// Workspace root from syx-driver manifest dir (dev path lookup only).
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Compile `path` to MIR and print the host-shake report (doctor surface).
pub fn host_shake_report_for_file(path: &Path) -> SyxResult<String> {
    use crate::module_ref::ModuleRef;
    use crate::{build_unified_mir, compile_module_recursive, DriverOptions, ModuleGraph};
    use syx_pkg::{load_manifest, load_resolved_packages};

    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    syx_host_plugin::register_compile_plugin_manifests(parent);
    let manifest = load_manifest(parent).unwrap_or_default();
    let mut opts = DriverOptions::default();
    opts.strict_concurrency = manifest.concurrency.strict;
    opts.default_isolation = manifest.concurrency.default_isolation.clone();
    if !manifest.dependencies.is_empty() {
        crate::ensure_project_lock(parent, &opts)?;
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
    Ok(format_host_shake_report(&mir))
}
