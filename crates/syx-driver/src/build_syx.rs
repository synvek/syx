//! `build.syb` GA — compile-time build configuration (SYX 41.0+).
//!
//! Extension `.syb` means **Sy Build** (declarative DSL), not Sy source (`.sy`).
//! Do not put `func` / `class` here — only `build { ... }` / `task ...` blocks.

use indexmap::IndexMap;
use smol_str::SmolStr;
use std::path::{Path, PathBuf};

use syx_pkg::NativeConfig;

use crate::DriverOptions;

/// Canonical default build-config filename (`syb` = Sy Build).
pub const DEFAULT_BUILD_CONFIG: &str = "build.syb";

/// True if `path` is a build config (`.syb`), distinct from Sy source (`.sy`).
pub fn is_build_config_path(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "syb")
}

/// Resolve build config path: `syx.toml` `build = "..."` if set, else default `build.syb` when present.
pub fn resolve_build_config_path(
    root: &Path,
    manifest: &syx_pkg::SyxManifest,
) -> Option<PathBuf> {
    if let Some(rel) = &manifest.build {
        // Explicit path: always return it (missing file → parse error), matching prior behavior.
        return Some(root.join(rel));
    }
    let default = root.join(DEFAULT_BUILD_CONFIG);
    if default.exists() {
        Some(default)
    } else {
        None
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum OptLevel {
    #[default]
    Debug,
    Release,
}

#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    pub opt_level: OptLevel,
}

#[derive(Debug, Clone, Default)]
pub struct BuildTask {
    pub name: String,
    pub depends_on: Vec<String>,
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BuildPlan {
    pub native: NativeConfig,
    pub plugins: Vec<SmolStr>,
    pub features: IndexMap<String, bool>,
    pub compile_options: CompileOptions,
    pub tasks: Vec<BuildTask>,
    /// Phase 76: build farm URL for remote compile task.
    pub farm_url: Option<String>,
    /// Phase 77: build farm queue URL.
    pub farm_queue_url: Option<String>,
    /// Phase 78: farm worker id for queue lease.
    pub farm_worker_id: Option<String>,
}

/// Context for executing `build.syb` task targets.
pub struct BuildTaskContext<'a> {
    pub project_root: &'a Path,
    pub options: &'a mut DriverOptions,
    pub manifest: &'a mut syx_pkg::SyxManifest,
    pub executed: Vec<String>,
}

impl<'a> BuildTaskContext<'a> {
    pub fn new(
        project_root: &'a Path,
        options: &'a mut DriverOptions,
        manifest: &'a mut syx_pkg::SyxManifest,
    ) -> Self {
        Self {
            project_root,
            options,
            manifest,
            executed: Vec::new(),
        }
    }
}

/// Strip trailing ` //` comments without breaking `file://` URLs.
fn strip_build_syx_comment(line: &str) -> &str {
    if let Some(idx) = line.find(" //") {
        line[..idx].trim()
    } else {
        line.trim()
    }
}

/// Minimal line/block parser for `build.syb` (declarative DSL, not full Sy).
pub fn parse_build_syx(path: &Path) -> syx_diagnostics::SyxResult<BuildPlan> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("Failed to read build.syb: {e}"))
    })?;
    let mut plan = BuildPlan::default();
    let mut section = String::new();
    for line in text.lines() {
        let line = strip_build_syx_comment(line);
        if line.is_empty() {
            continue;
        }
        if line.ends_with('{') {
            section = line.trim_end_matches('{').trim().to_string();
            if section.starts_with("task ") {
                let task_name = section.strip_prefix("task ").unwrap_or("").trim();
                plan.tasks.push(BuildTask {
                    name: task_name.to_string(),
                    ..Default::default()
                });
            }
            continue;
        }
        if line == "}" {
            section.clear();
            continue;
        }
        if section == "compileOptions" {
            if let Some((key, val)) = line.split_once('=') {
                let key = key.trim();
                let val = val.trim().trim_end_matches(',');
                if key == "optLevel" {
                    plan.compile_options.opt_level = if val.contains("release") {
                        OptLevel::Release
                    } else {
                        OptLevel::Debug
                    };
                }
            }
            continue;
        }
        if section.starts_with("task ") {
            let task_name = section.strip_prefix("task ").unwrap_or("").trim();
            if let Some(task) = plan.tasks.iter_mut().find(|t| t.name == task_name) {
                if let Some((key, val)) = line.split_once('=') {
                    let key = key.trim();
                    let val = val.trim().trim_end_matches(',').trim_matches('"');
                    if key == "target" {
                        task.target = Some(val.to_string());
                    } else if key == "dependsOn" {
                        task.depends_on = parse_string_array(val);
                    } else if key == "farm" {
                        plan.farm_url = Some(val.to_string());
                    } else if key == "queue" {
                        plan.farm_queue_url = Some(val.to_string());
                    } else if key == "worker" {
                        plan.farm_worker_id = Some(val.to_string());
                    }
                }
            }
            continue;
        }
        if section == "plugins" {
            let s = line.trim().trim_matches('"').trim();
            if !s.is_empty() && !s.contains('=') {
                plan.plugins.push(SmolStr::new(s));
            }
            continue;
        }
        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim();
            let val = val.trim().trim_end_matches(',');
            match section.as_str() {
                "native" => match key {
                    "libs" | "link_flags" | "lib_paths" | "include_dirs" => {
                        let items = parse_string_array(val);
                        match key {
                            "libs" => plan.native.libs = items,
                            "link_flags" => plan.native.link_flags = items,
                            "lib_paths" => plan.native.lib_paths = items,
                            "include_dirs" => plan.native.include_dirs = items,
                            _ => {}
                        }
                    }
                    _ => {}
                },
                "features" => {
                    let on = val == "true";
                    plan.features.insert(key.to_string(), on);
                }
                _ => {}
            }
        }
    }
    Ok(plan)
}

fn parse_string_array(val: &str) -> Vec<String> {
    val.trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .filter_map(|s| {
            let s = s.trim().trim_matches('"');
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect()
}

/// Merge `build.syb` plan into manifest and driver options (build.syb overrides toml).
pub fn apply_build_plan(
    plan: &BuildPlan,
    options: &mut DriverOptions,
    manifest: &mut syx_pkg::SyxManifest,
) {
    if !plan.native.libs.is_empty() {
        manifest.native.libs = plan.native.libs.clone();
    }
    if !plan.native.link_flags.is_empty() {
        manifest.native.link_flags = plan.native.link_flags.clone();
    }
    if !plan.native.lib_paths.is_empty() {
        manifest.native.lib_paths = plan.native.lib_paths.clone();
    }
    if !plan.native.include_dirs.is_empty() {
        manifest.native.include_dirs = plan.native.include_dirs.clone();
    }
    if plan.compile_options.opt_level == OptLevel::Release {
        options.build_profile = crate::BuildProfile::Release;
        options.wasm_aot = true;
    }
    if plan.features.get("wasm_aot").copied().unwrap_or(false) {
        options.wasm_aot = true;
    }
}

pub fn load_build_plan(
    root: &Path,
    manifest: &syx_pkg::SyxManifest,
) -> syx_diagnostics::SyxResult<Option<BuildPlan>> {
    let Some(path) = resolve_build_config_path(root, manifest) else {
        return Ok(None);
    };
    Ok(Some(parse_build_syx(&path)?))
}

/// Phase 67: go-to-def for build.syb task names and dependsOn references.
pub fn definition_for_build_syx(
    text: &str,
    line: usize,
    character: usize,
) -> Option<(usize, usize)> {
    let lines: Vec<&str> = text.lines().collect();
    let line_text = lines.get(line)?;
    let word = {
        let chars: Vec<char> = line_text.chars().collect();
        if character >= chars.len() {
            return None;
        }
        let mut start = character;
        let mut end = character + 1;
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }
        line_text[start..end].trim_matches('"').to_string()
    };
    if word.is_empty() {
        return None;
    }
    for (i, l) in lines.iter().enumerate() {
        let trimmed = l.trim();
        if trimmed.starts_with("task ") {
            let name = trimmed
                .strip_prefix("task ")
                .unwrap_or("")
                .split_whitespace()
                .next()?;
            if name == word {
                let col = l.find(name).unwrap_or(0);
                return Some((i, col));
            }
        }
        if trimmed.contains("dependsOn") && trimmed.contains(&format!("\"{word}\"")) {
            if let Some(pos) = trimmed.find(&format!("\"{word}\"")) {
                return Some((i, pos + 1));
            }
        }
    }
    None
}

/// Phase 64: topological task order; cycle → S4xx diagnostic.
pub fn order_build_tasks(plan: &BuildPlan) -> syx_diagnostics::SyxResult<Vec<String>> {
    use std::collections::{HashMap, HashSet, VecDeque};
    let names: HashSet<&str> = plan.tasks.iter().map(|t| t.name.as_str()).collect();
    for t in &plan.tasks {
        for d in &t.depends_on {
            if !names.contains(d.as_str()) {
                return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                    "build.syb task '{}' depends on unknown task '{d}'",
                    t.name
                ))
                .with_code("S401"));
            }
        }
    }
    let mut indeg: HashMap<&str, usize> = plan
        .tasks
        .iter()
        .map(|t| (t.name.as_str(), t.depends_on.len()))
        .collect();
    let mut q: VecDeque<&str> = indeg
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(n, _)| *n)
        .collect();
    let mut order = Vec::new();
    while let Some(n) = q.pop_front() {
        order.push(n.to_string());
        for t in &plan.tasks {
            if t.depends_on.iter().any(|d| d == n) {
                let e = indeg.get_mut(t.name.as_str()).unwrap();
                *e = e.saturating_sub(1);
                if *e == 0 {
                    q.push_back(t.name.as_str());
                }
            }
        }
    }
    if order.len() != plan.tasks.len() {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "build.syb task dependency cycle detected",
        )
        .with_code("S402"));
    }
    Ok(order)
}

fn artifact_dir(root: &Path, options: &DriverOptions) -> PathBuf {
    root.join("target").join(options.build_profile.dir_name())
}

fn execute_build_target(
    task_name: &str,
    target: &str,
    ctx: &mut BuildTaskContext<'_>,
) -> syx_diagnostics::SyxResult<()> {
    let out_dir = artifact_dir(ctx.project_root, ctx.options);
    std::fs::create_dir_all(&out_dir).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!(
            "build task '{task_name}': failed to create {}: {e}",
            out_dir.display()
        ))
    })?;

    ctx.options.in_build_syx_task = true;
    let result: syx_diagnostics::SyxResult<()> = match target {
        "mir" => crate::emit_mir_text(ctx.project_root, ctx.options).and_then(|text| {
            std::fs::write(out_dir.join("out.mir"), &text).map_err(|e| {
                syx_diagnostics::SyxDiagnostic::error(format!(
                    "build task '{task_name}': failed to write MIR: {e}"
                ))
            })
        }),
        "wasm" => crate::build_wasm_with_options(ctx.project_root, ctx.options).and_then(|bytes| {
            std::fs::write(out_dir.join("out.wasm"), &bytes).map_err(|e| {
                syx_diagnostics::SyxDiagnostic::error(format!(
                    "build task '{task_name}': failed to write WASM: {e}"
                ))
            })
        }),
        "native" => {
            crate::build_native_with_options(ctx.project_root, ctx.options).and_then(|out| {
                std::fs::write(out_dir.join("out.o"), &out.object).map_err(|e| {
                    syx_diagnostics::SyxDiagnostic::error(format!(
                        "build task '{task_name}': failed to write native object: {e}"
                    ))
                })
            })
        }
        "link" => {
            let exe = out_dir.join(crate::native_executable_file_name("app"));
            crate::build_native_executable(ctx.project_root, ctx.options, &exe).map(|_| ())
        }
        "test" => crate::test_project(ctx.project_root, ctx.options).map(|_| ()),
        "remote" => execute_remote_farm_task(task_name, ctx),
        "farm_queue" => execute_farm_queue_task(task_name, ctx),
        "farm_worker" => execute_farm_worker_task(task_name, ctx),
        other => {
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "build.syb task '{task_name}' has unknown target '{other}'"
            ))
            .with_code("S403"));
        }
    };
    ctx.options.in_build_syx_task = false;
    result.map_err(|e| {
        if e.code.as_deref() == Some("S403") {
            e
        } else {
            syx_diagnostics::SyxDiagnostic::error(format!(
                "build.syb task '{task_name}' (target '{target}') failed: {}",
                e.message
            ))
            .with_code("S404")
        }
    })
}

/// Phase 65: execute build.syb tasks in topological order (mir → wasm → native → link → test).
pub fn run_build_tasks(
    plan: &BuildPlan,
    ctx: &mut BuildTaskContext<'_>,
) -> syx_diagnostics::SyxResult<()> {
    let order = order_build_tasks(plan)?;
    for name in order {
        let task = plan
            .tasks
            .iter()
            .find(|t| t.name == name)
            .expect("task name in topological order");
        if let Some(ref target) = task.target {
            execute_build_target(&name, target, ctx)?;
        }
        ctx.executed.push(name);
    }
    Ok(())
}

/// Apply plan options and validate the task DAG without executing targets.
pub fn prepare_build_plan(
    plan: &BuildPlan,
    options: &mut DriverOptions,
    manifest: &mut syx_pkg::SyxManifest,
    project_root: &Path,
) -> syx_diagnostics::SyxResult<Vec<String>> {
    apply_build_plan(plan, options, manifest);
    let order = order_build_tasks(plan)?;
    if !plan.plugins.is_empty() {
        syx_host_plugin::register_compile_plugin_manifests(project_root);
    }
    if plan.features.get("wasm_aot").copied().unwrap_or(false) {
        options.wasm_aot = true;
    }
    Ok(order)
}

/// Phase 65: prepare + execute build.syb task DAG.
pub fn execute_build_plan(
    plan: &BuildPlan,
    options: &mut DriverOptions,
    manifest: &mut syx_pkg::SyxManifest,
    project_root: &Path,
) -> syx_diagnostics::SyxResult<Vec<String>> {
    prepare_build_plan(plan, options, manifest, project_root)?;
    let mut ctx = BuildTaskContext::new(project_root, options, manifest);
    run_build_tasks(plan, &mut ctx)?;
    Ok(ctx.executed)
}

const KNOWN_TARGETS: &[&str] = &[
    "mir",
    "wasm",
    "native",
    "link",
    "test",
    "remote",
    "farm_queue",
    "farm_worker",
];

static LAST_FARM_WORKER_HIT: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// Phase 78: last farm worker dequeue hit for doctor report.
pub fn farm_worker_last_hit() -> Option<String> {
    LAST_FARM_WORKER_HIT.lock().ok().and_then(|g| g.clone())
}

/// Phase 78: dequeue `.bin` from queue dir into farm cache dir.
pub fn farm_worker_dequeue(
    queue_dir: &std::path::Path,
    farm_dir: &std::path::Path,
) -> Option<String> {
    let _ = std::fs::create_dir_all(farm_dir);
    let _ = std::fs::create_dir_all(queue_dir);
    let mut entries: Vec<_> = std::fs::read_dir(queue_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("bin"))
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let dest = farm_dir.join(&name);
        if std::fs::rename(entry.path(), &dest).is_ok() {
            if let Ok(mut g) = LAST_FARM_WORKER_HIT.lock() {
                *g = Some(name.clone());
            }
            return Some(name);
        }
    }
    None
}

fn farm_worker_urls_need_lease(queue_url: &str, farm_url: &str) -> bool {
    queue_url.starts_with("tcp://") || farm_url.starts_with("tcp://")
}

/// Phase 78: run one farm worker cycle (queue → farm cache dir).
pub fn farm_worker_run_cycle(queue_url: &str, farm_url: &str) -> Result<Option<String>, String> {
    if farm_worker_urls_need_lease(queue_url, farm_url) {
        crate::farm_lease_auth::farm_lease_require_active()
            .map_err(|e| format!("farm lease hard reject: {e}"))?;
    }
    let farm_url = if farm_url.is_empty() {
        crate::default_farm_cache_url()
    } else {
        farm_url.to_string()
    };
    if std::env::var("SYX_FARM_BILLING_PATH").ok().as_deref() == Some("1") {
        if std::env::var("SYX_BILLING_SETTLEMENT_ACK").is_err() {
            return Err("billing farm path: settlement ack missing (exit!=0)".into());
        }
    }
    // Phase 99: queue must participate — TCP queue fetch then farm put/write.
    if queue_url.starts_with("tcp://") {
        let bytes = crate::fetch_cache_remote_hardened(queue_url)?;
        let name = queue_url
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("job.bin")
            .to_string();
        if farm_url.starts_with("tcp://") {
            crate::push_cache_remote(&farm_url, &bytes)?;
        } else {
            let farm_dir = farm_url
                .strip_prefix("file://")
                .unwrap_or(farm_url.as_str());
            std::fs::create_dir_all(farm_dir).map_err(|e| e.to_string())?;
            std::fs::write(std::path::Path::new(farm_dir).join(&name), &bytes)
                .map_err(|e| e.to_string())?;
        }
        if let Ok(mut g) = LAST_FARM_WORKER_HIT.lock() {
            *g = Some(name.clone());
        }
        if let Ok(mut g) = LAST_FARM_QUEUE_HIT.lock() {
            *g = Some(queue_url.to_string());
        }
        return Ok(Some(name));
    }
    if farm_url.starts_with("tcp://") {
        // Farm TCP without TCP queue: still require a local file:// queue.
        let queue_dir = queue_url.strip_prefix("file://").unwrap_or(queue_url);
        let bytes = {
            let q = std::path::Path::new(queue_dir);
            let entry = std::fs::read_dir(q)
                .map_err(|e| e.to_string())?
                .flatten()
                .find(|e| e.path().is_file())
                .ok_or_else(|| "farm queue empty".to_string())?;
            std::fs::read(entry.path()).map_err(|e| e.to_string())?
        };
        crate::push_cache_remote(&farm_url, &bytes)?;
        let name = farm_url
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("tenant.bin")
            .to_string();
        if let Ok(mut g) = LAST_FARM_WORKER_HIT.lock() {
            *g = Some(name.clone());
        }
        return Ok(Some(name));
    }
    let queue_dir = queue_url.strip_prefix("file://").unwrap_or(queue_url);
    let farm_dir = farm_url
        .strip_prefix("file://")
        .unwrap_or(farm_url.as_str());
    Ok(farm_worker_dequeue(
        std::path::Path::new(queue_dir),
        std::path::Path::new(farm_dir),
    ))
}

/// Phase 104: cross-node lease authority (TCP grant/renew); not process-local Mutex GA.
pub fn farm_lease_acquire(ttl_secs: u32) -> Result<(), String> {
    crate::farm_lease_auth::farm_lease_acquire(ttl_secs)
}

pub fn farm_lease_expired() -> bool {
    crate::farm_lease_auth::farm_lease_expired()
}

pub fn farm_lease_reset() {
    crate::farm_lease_auth::farm_lease_reset();
}

static LAST_FARM_QUEUE_HIT: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// Phase 77: last farm queue hit for doctor report.
pub fn farm_queue_last_hit() -> Option<String> {
    LAST_FARM_QUEUE_HIT.lock().ok().and_then(|g| g.clone())
}

/// Phase 77: enqueue cache artifact to farm queue dir and record hit.
fn execute_farm_queue_task(
    task_name: &str,
    ctx: &mut BuildTaskContext<'_>,
) -> syx_diagnostics::SyxResult<()> {
    let queue_url = std::env::var("SYX_BUILD_FARM_QUEUE")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            parse_build_syx(&ctx.project_root.join(DEFAULT_BUILD_CONFIG))
                .ok()
                .and_then(|p| p.farm_queue_url)
        })
        .ok_or_else(|| {
            syx_diagnostics::SyxDiagnostic::error(format!(
                "build task '{task_name}': farm_queue requires queue URL (build.syb queue= or SYX_BUILD_FARM_QUEUE)"
            ))
            .with_code("S406")
        })?;
    execute_remote_farm_task(task_name, ctx)?;
    if let Ok(mut g) = LAST_FARM_QUEUE_HIT.lock() {
        *g = Some(queue_url);
    }
    Ok(())
}

/// Phase 78: farm worker target — dequeue queue artifacts into farm cache dir.
fn execute_farm_worker_task(
    task_name: &str,
    ctx: &mut BuildTaskContext<'_>,
) -> syx_diagnostics::SyxResult<()> {
    let queue_url = std::env::var("SYX_BUILD_FARM_QUEUE")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            parse_build_syx(&ctx.project_root.join(DEFAULT_BUILD_CONFIG))
                .ok()
                .and_then(|p| p.farm_queue_url)
        })
        .ok_or_else(|| {
            syx_diagnostics::SyxDiagnostic::error(format!(
                "build task '{task_name}': farm_worker requires queue URL"
            ))
            .with_code("S407")
        })?;
    let farm_url = std::env::var("SYX_BUILD_FARM_URL")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            parse_build_syx(&ctx.project_root.join(DEFAULT_BUILD_CONFIG))
                .ok()
                .and_then(|p| p.farm_url)
        })
        .unwrap_or_else(crate::default_farm_cache_url);
    farm_worker_run_cycle(&queue_url, &farm_url)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("farm worker failed: {e}")))?;
    Ok(())
}

/// Phase 76: upload compiled cache artifact to build farm stub (`file://` or env URL).
fn execute_remote_farm_task(
    task_name: &str,
    ctx: &mut BuildTaskContext<'_>,
) -> syx_diagnostics::SyxResult<()> {
    let farm_url = std::env::var("SYX_BUILD_FARM_URL")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            parse_build_syx(&ctx.project_root.join(DEFAULT_BUILD_CONFIG))
                .ok()
                .and_then(|p| p.farm_url)
        })
        .ok_or_else(|| {
            syx_diagnostics::SyxDiagnostic::error(format!(
                "build task '{task_name}': remote target requires farm URL (build.syb farm= or SYX_BUILD_FARM_URL)"
            ))
            .with_code("S405")
        })?;
    let main = ctx.project_root.join("main.sy");
    if !main.exists() {
        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
            "build task '{task_name}': main.sy not found"
        )));
    }
    let _ = crate::compile_with_cache(&main, ctx.project_root)?;
    let cache = crate::incremental_cache_path(ctx.project_root, &main);
    let bytes = std::fs::read(&cache).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("farm cache read failed: {e}"))
    })?;
    let fname = cache
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("module.bin");
    let dest = format!("{}/{}", farm_url.trim_end_matches('/'), fname);
    crate::push_cache_remote(&dest, &bytes)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("farm upload failed: {e}")))?;
    if let Ok(queue_url) = std::env::var("SYX_BUILD_FARM_QUEUE") {
        if !queue_url.is_empty() {
            let qdest = format!("{}/{}", queue_url.trim_end_matches('/'), fname);
            let _ = crate::push_cache_remote(&qdest, &bytes);
        }
    }
    Ok(())
}

/// Phase 66: collect build.syb diagnostics for LSP / analyze (unknown dep, cycle, unknown target).
pub fn diagnose_build_syx(
    source: &syx_diagnostics::SourceFile,
) -> Vec<syx_diagnostics::SyxDiagnostic> {
    let text = source.contents.as_ref();
    let mut diags = Vec::new();
    let mut tasks: Vec<BuildTask> = Vec::new();
    let mut section = String::new();
    for (line_no, line) in text.lines().enumerate() {
        let raw = line;
        let line = line.split("//").next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if line.ends_with('{') {
            section = line.trim_end_matches('{').trim().to_string();
            if section.starts_with("task ") {
                let task_name = section.strip_prefix("task ").unwrap_or("").trim();
                tasks.push(BuildTask {
                    name: task_name.to_string(),
                    ..Default::default()
                });
            }
            continue;
        }
        if line == "}" {
            section.clear();
            continue;
        }
        if section.starts_with("task ") {
            let task_name = section.strip_prefix("task ").unwrap_or("").trim();
            if let Some(task) = tasks.iter_mut().find(|t| t.name == task_name) {
                if let Some((key, val)) = line.split_once('=') {
                    let key = key.trim();
                    let val = val.trim().trim_end_matches(',').trim_matches('"');
                    if key == "target" {
                        task.target = Some(val.to_string());
                        if !KNOWN_TARGETS.contains(&val) {
                            let col = raw.find(val).unwrap_or(0);
                            let offset = text
                                .lines()
                                .take(line_no)
                                .map(|l| l.len() + 1)
                                .sum::<usize>()
                                + col;
                            diags.push(
                                syx_diagnostics::SyxDiagnostic::error(format!(
                                    "build.syb task '{task_name}' has unknown target '{val}'"
                                ))
                                .with_code("S403")
                                .with_span(
                                    source,
                                    syx_diagnostics::Span::new(offset, val.len()),
                                    "unknown target",
                                ),
                            );
                        }
                    } else if key == "dependsOn" {
                        task.depends_on = parse_string_array(val);
                    }
                }
            }
        }
    }

    let plan = BuildPlan {
        tasks,
        ..Default::default()
    };
    let names: std::collections::HashSet<&str> =
        plan.tasks.iter().map(|t| t.name.as_str()).collect();
    for t in &plan.tasks {
        for dep in &t.depends_on {
            if !names.contains(dep.as_str()) {
                diags.push(
                    syx_diagnostics::SyxDiagnostic::error(format!(
                        "build.syb task '{}' depends on unknown task '{dep}'",
                        t.name
                    ))
                    .with_code("S401"),
                );
            }
        }
    }
    if let Err(e) = order_build_tasks(&plan) {
        if e.code.as_deref() == Some("S402") {
            diags.push(e);
        }
    }
    diags
}
