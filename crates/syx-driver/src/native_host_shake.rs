//! Native link-time host shaking: slim `libsyx_rt.a` + optional `syx-host-{sql,http,tls}`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use syx_diagnostics::{Span, SyxResult};
use syx_mir::{collect_call_host_indices_reachable, MirModule};
use syx_stdlib::{shake_domains_for_indices, HostShakeDomain};

use crate::{driver_error_at, BuildProfile, DriverOptions};

/// Isolated cargo target dir so CLI feature unification cannot fatten `libsyx_rt.a`.
pub fn native_shake_target_dir(workspace: &Path) -> PathBuf {
    workspace.join("target/native-host-shake")
}

fn profile_dir(target_dir: &Path, release: bool) -> PathBuf {
    target_dir.join(if release { "release" } else { "debug" })
}

/// Ensure slim `syx-rt` (+ needed host domain staticlibs) exist under the shake target dir.
pub fn ensure_shake_archives(
    workspace: &Path,
    domains: &[HostShakeDomain],
    options: &DriverOptions,
) -> SyxResult<ShakeLinkArtifacts> {
    let target_dir = native_shake_target_dir(workspace);
    let release = options.build_profile == BuildProfile::Release;
    let mut args = vec![
        "build".to_string(),
        "-p".to_string(),
        "syx-rt".to_string(),
        "--no-default-features".to_string(),
    ];
    let mut rt_features = Vec::new();
    // Async pulls syx-async (+ rustls dist transport). Only link when explicitly requested
    // via --minimal=false AND SYX_NATIVE_ASYNC=1, or legacy non-shake rebuilds.
    let want_async = !options.native_minimal
        && std::env::var("SYX_NATIVE_ASYNC")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
    if want_async {
        rt_features.push("async-runtime");
    }
    if !options.native_no_file {
        rt_features.push("file");
    }
    // `http` on syx-rt is a cfg marker for handler install only (no syx-vm/http).
    // Only needed when the HTTP domain archive is linked (Http.serve handler bridge).
    if domains.iter().any(|d| *d == HostShakeDomain::Http) && !options.native_no_http {
        rt_features.push("http");
    }
    if !rt_features.is_empty() {
        args.push("--features".to_string());
        args.push(rt_features.join(","));
    }
    if release {
        args.push("--release".to_string());
    }
    for domain in domains {
        args.push("-p".to_string());
        args.push(domain.archive_package().to_string());
    }
    let status = Command::new("cargo")
        .args(&args)
        .current_dir(workspace)
        .env("CARGO_TARGET_DIR", &target_dir)
        .status()
        .map_err(|e| {
            driver_error_at(
                format!("Failed to build native shake archives: {e}"),
                None,
                Span::DUMMY,
                "D001",
            )
        })?;
    if !status.success() {
        return Err(driver_error_at(
            "Failed to build slim syx-rt / host domain archives for native link",
            None,
            Span::DUMMY,
            "N001",
        ));
    }
    let dir = profile_dir(&target_dir, release);
    let rt = dir.join(rt_archive_name());
    if !rt.is_file() {
        return Err(driver_error_at(
            format!("slim syx-rt archive missing: {}", rt.display()),
            None,
            Span::DUMMY,
            "N001",
        ));
    }
    let mut host_archives = Vec::new();
    for domain in domains {
        // staticlib name: libsyx_host_sql.a / syx_host_sql.lib
        let stem = domain.archive_package().replace('-', "_");
        #[cfg(windows)]
        let archive = dir.join(format!("{stem}.lib"));
        #[cfg(not(windows))]
        let archive = dir.join(format!("lib{stem}.a"));
        if !archive.is_file() {
            return Err(driver_error_at(
                format!("host domain archive missing: {}", archive.display()),
                None,
                Span::DUMMY,
                "N001",
            ));
        }
        host_archives.push(archive);
    }
    Ok(ShakeLinkArtifacts {
        rt_archive: rt,
        host_archives,
        domains: domains.to_vec(),
    })
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
    lines.push(format!("native-host-shake call_host_count={}", indices.len()));
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
    lines.join("\n")
}

/// Workspace root from syx-driver manifest dir.
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Compile `path` to MIR and print the host-shake report (doctor surface).
pub fn host_shake_report_for_file(path: &Path) -> SyxResult<String> {
    use crate::module_ref::ModuleRef;
    use crate::{
        build_unified_mir, compile_module_recursive, DriverOptions, ModuleGraph,
    };
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
