//! Append-only plugin Host registry (ADR-071 §1.3).

use std::sync::OnceLock;
use std::sync::RwLock;

pub use syx_stdlib::plugin_host_base_index;

pub type PluginHostCallback =
    std::sync::Arc<dyn Fn(&mut Vec<String>) -> Result<(), String> + Send + Sync>;

pub type PluginHostV2Callback =
    std::sync::Arc<dyn Fn(&mut Vec<syx_runtime_core::Value>) -> Result<(), String> + Send + Sync>;

#[derive(Clone)]
pub struct LoadedPluginHost {
    pub name: String,
    pub return_type: String,
    pub plugin_id: String,
    pub index: u32,
    pub arity: u32,
    pub callback: PluginHostCallback,
    pub v2_callback: Option<PluginHostV2Callback>,
}

#[cfg(any(unix, windows))]
use libloading::Library;

static RUNTIME_HOSTS: OnceLock<RwLock<Vec<LoadedPluginHost>>> = OnceLock::new();

#[cfg(any(unix, windows))]
static LOADED_LIBRARIES: OnceLock<std::sync::Mutex<Vec<LoadedDynamicLibrary>>> = OnceLock::new();

/// Phase 103: retain `Library` handles so unload can `dlclose` (drop), not `mem::forget`.
#[cfg(any(unix, windows))]
pub struct LoadedDynamicLibrary {
    pub plugin_id: String,
    pub library: Library,
}

#[cfg(any(unix, windows))]
pub fn register_loaded_library(plugin_id: String, library: libloading::Library) {
    let store = LOADED_LIBRARIES.get_or_init(|| std::sync::Mutex::new(Vec::new()));
    if let Ok(mut g) = store.lock() {
        g.retain(|e| e.plugin_id != plugin_id);
        g.push(LoadedDynamicLibrary { plugin_id, library });
    }
}

/// Phase 103: drop the dynamic library for `plugin_id` and remove its runtime hosts.
#[cfg(any(unix, windows))]
pub fn unload_plugin_library(plugin_id: &str) -> Result<(), String> {
    {
        let store = RUNTIME_HOSTS.get_or_init(|| RwLock::new(Vec::new()));
        if let Ok(mut g) = store.write() {
            g.retain(|h| h.plugin_id != plugin_id);
        }
    }
    let store = LOADED_LIBRARIES.get_or_init(|| std::sync::Mutex::new(Vec::new()));
    let mut guard = store
        .lock()
        .map_err(|_| "plugin library lock poisoned".to_string())?;
    let pos = guard
        .iter()
        .position(|e| e.plugin_id == plugin_id)
        .ok_or_else(|| format!("H005: no loaded library for plugin '{plugin_id}'"))?;
    guard.remove(pos);
    poison_plugin_slot(plugin_id);
    Ok(())
}

#[cfg(not(any(unix, windows)))]
pub fn unload_plugin_library(_plugin_id: &str) -> Result<(), String> {
    Err("H001: dynamic plugins unsupported on this platform".into())
}

#[cfg(any(unix, windows))]
pub fn loaded_plugin_ids() -> Vec<String> {
    LOADED_LIBRARIES
        .get()
        .and_then(|s| s.lock().ok())
        .map(|g| g.iter().map(|e| e.plugin_id.clone()).collect())
        .unwrap_or_default()
}

#[cfg(not(any(unix, windows)))]
pub fn loaded_plugin_ids() -> Vec<String> {
    Vec::new()
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PluginLockEntry {
    pub plugin_id: String,
    pub hosts: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PluginLock {
    pub plugins: Vec<PluginLockEntry>,
    pub plugin_base: u32,
    #[serde(default)]
    pub plugin_unload_epoch: u64,
}

static LOCKFILE: std::sync::Mutex<Option<PluginLock>> = std::sync::Mutex::new(None);

pub fn register_compile_hosts_from_manifest(manifest: &crate::manifest::PluginManifest) {
    let entries: Vec<(&'static str, &'static str)> = manifest
        .hosts
        .iter()
        .map(|h| {
            let name: &'static str = Box::leak(h.name.clone().into_boxed_str());
            let ret: &'static str = Box::leak(h.return_type.clone().into_boxed_str());
            (name, ret)
        })
        .collect();
    syx_stdlib::register_compile_plugin_hosts(entries);
}

pub fn register_runtime_host(entry: LoadedPluginHost) {
    let store = RUNTIME_HOSTS.get_or_init(|| RwLock::new(Vec::new()));
    if let Ok(mut g) = store.write() {
        g.push(entry);
    }
}

pub fn runtime_hosts() -> Vec<LoadedPluginHost> {
    RUNTIME_HOSTS
        .get()
        .and_then(|s| s.read().ok())
        .map(|g| g.clone())
        .unwrap_or_default()
}

pub fn dispatch_plugin_host(name: &str, stack: &mut Vec<syx_runtime_core::Value>) -> bool {
    let store = match RUNTIME_HOSTS.get() {
        Some(s) => s,
        None => return false,
    };
    let hosts = match store.read() {
        Ok(h) => h,
        Err(_) => return false,
    };
    let Some(host) = hosts.iter().find(|h| h.name == name) else {
        return false;
    };
    let mut args = Vec::new();
    for _ in 0..host.arity {
        args.push(stack.pop().unwrap_or(syx_runtime_core::Value::Nil));
    }
    args.reverse();
    if let Some(v2) = &host.v2_callback {
        if v2(&mut args).is_err() {
            return false;
        }
        for v in args {
            stack.push(v);
        }
        return true;
    }
    // Resolve HeapSlot-backed strings (ManagedHeap) before C ABI UTF-8 handoff.
    let mut str_stack: Vec<String> = args
        .into_iter()
        .map(|v| {
            let fallback = format!("{v:?}");
            syx_runtime_core::Value::materialize_str(v).unwrap_or(fallback)
        })
        .collect();
    if (host.callback)(&mut str_stack).is_err() {
        return false;
    }
    for s in str_stack {
        stack.push(syx_runtime_core::Value::Str(s));
    }
    true
}

pub fn set_plugin_lock(lock: PluginLock) {
    if let Ok(mut g) = LOCKFILE.lock() {
        *g = Some(lock);
    }
}

pub fn plugin_lock() -> Option<PluginLock> {
    LOCKFILE.lock().ok().and_then(|g| g.clone())
}

pub fn runtime_plugin_loaded(name: &str) -> bool {
    RUNTIME_HOSTS
        .get()
        .and_then(|s| s.read().ok())
        .map(|h| h.iter().any(|e| e.name == name))
        .unwrap_or(false)
}

pub fn plugin_host_index(name: &str) -> Option<u32> {
    syx_stdlib::host_fn_index_by_name(name)
}

pub fn validate_plugin_lock() -> Result<(), String> {
    let Some(lock) = plugin_lock() else {
        return Ok(());
    };
    let base = plugin_host_base_index();
    if lock.plugin_base != base {
        return Err(format!(
            "H002: plugin_base mismatch lock={} runtime={base}",
            lock.plugin_base
        ));
    }
    Ok(())
}

/// Phase 51: compile-time lock vs registered plugin hosts (H002).
pub fn validate_compile_plugin_lock() -> Result<(), String> {
    validate_plugin_lock()?;
    let Some(lock) = plugin_lock() else {
        return Ok(());
    };
    for entry in &lock.plugins {
        for host in &entry.hosts {
            if syx_stdlib::host_fn_index_by_name(host).is_none() {
                return Err(format!(
                    "H002: compile lock references unknown host '{host}' for plugin '{}'",
                    entry.plugin_id
                ));
            }
        }
    }
    Ok(())
}

pub fn register_compile_plugin_manifests(project_root: &std::path::Path) {
    #[derive(serde::Deserialize, Default)]
    struct SyxLockToml {
        #[serde(default)]
        plugins: Vec<SyxLockPluginEntry>,
    }
    #[derive(serde::Deserialize)]
    struct SyxLockPluginEntry {
        id: String,
        hosts: Vec<String>,
    }
    #[derive(serde::Deserialize, Default)]
    struct SyxTomlPlugins {
        #[serde(default)]
        plugins: PluginsSection,
    }
    #[derive(serde::Deserialize, Default)]
    struct PluginsSection {
        #[serde(default)]
        paths: Vec<String>,
    }

    fn ingest_manifest(path: &std::path::Path) {
        if let Ok(m) = crate::manifest::PluginManifest::from_path(path) {
            let _ = crate::manifest::verify_signature(&m);
            register_compile_hosts_from_manifest(&m);
        }
    }

    fn scan_plugins_dir(dir: &std::path::Path) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let nested = path.join("plugin.json");
                if nested.is_file() {
                    ingest_manifest(&nested);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
                ingest_manifest(&path);
            }
        }
    }

    let syx_lock_path = project_root.join("syx.lock");
    if syx_lock_path.exists() {
        if let Ok(text) = std::fs::read_to_string(&syx_lock_path) {
            if let Ok(lock) = toml::from_str::<SyxLockToml>(&text) {
                if !lock.plugins.is_empty() {
                    set_plugin_lock(PluginLock {
                        plugins: lock
                            .plugins
                            .into_iter()
                            .map(|e| PluginLockEntry {
                                plugin_id: e.id,
                                hosts: e.hosts,
                            })
                            .collect(),
                        plugin_base: plugin_host_base_index(),
                        plugin_unload_epoch: plugin_unload_epoch(),
                    });
                }
            }
        }
    }
    let syx_toml = project_root.join("syx.toml");
    if syx_toml.exists() {
        if let Ok(text) = std::fs::read_to_string(&syx_toml) {
            if let Ok(cfg) = toml::from_str::<SyxTomlPlugins>(&text) {
                for rel in cfg.plugins.paths {
                    let path = project_root.join(rel);
                    if path.is_file() {
                        ingest_manifest(&path);
                    }
                }
            }
        }
    }
    for manifest in crate::manifest::compile_manifests() {
        register_compile_hosts_from_manifest(&manifest);
    }
    scan_plugins_dir(&project_root.join("plugins"));
    let syx_plugins = project_root.join(".syx/plugins");
    if syx_plugins.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&syx_plugins) {
            for entry in entries.flatten() {
                let manifest_path = entry.path().join("plugin.json");
                if manifest_path.is_file() {
                    ingest_manifest(&manifest_path);
                }
            }
        }
    }
    // Install-prefix layout: system plugins in bin/, user plugins in bin/plugins/
    // (relative to running syx or ./bin under CWD).
    for root in crate::loader::plugin_install_roots() {
        if crate::loader::system_faerlin_present(&root) {
            crate::loader::register_linalg_system_hosts();
        }
        scan_plugins_dir(&root);
        scan_plugins_dir(&root.join("plugins"));
    }
}

static POISONED_PLUGINS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

static PLUGIN_UNLOAD_EPOCH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Phase 78: current plugin unload epoch (written to lockfile on poison).
pub fn plugin_unload_epoch() -> u64 {
    PLUGIN_UNLOAD_EPOCH.load(std::sync::atomic::Ordering::Relaxed)
}

/// Phase 57 H005: mark plugin/macro slot poison after hot-unload.
pub fn poison_plugin_slot(name: &str) {
    if let Ok(mut g) = POISONED_PLUGINS.lock() {
        g.push(name.to_string());
    }
    PLUGIN_UNLOAD_EPOCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let epoch = plugin_unload_epoch();
    std::env::set_var("SYX_PLUGIN_UNLOAD_EPOCH", epoch.to_string());
    write_plugin_unload_epoch_lockfile(epoch);
}

fn write_plugin_unload_epoch_lockfile(epoch: u64) {
    if let Ok(mut guard) = LOCKFILE.lock() {
        if let Some(ref mut lock) = *guard {
            lock.plugin_unload_epoch = epoch;
        }
    }
    let cwd = std::env::current_dir().unwrap_or_default();
    for name in ["syx-plugin-lock.json", "syx.lock"] {
        let path = cwd.join(name);
        if !path.is_file() {
            continue;
        }
        if name.ends_with(".json") {
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(mut val) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(obj) = val.as_object_mut() {
                        obj.insert(
                            "plugin_unload_epoch".into(),
                            serde_json::Value::Number(epoch.into()),
                        );
                        if let Ok(out) = serde_json::to_string_pretty(&val) {
                            let _ = std::fs::write(&path, out);
                        }
                    }
                }
            }
        } else if let Ok(text) = std::fs::read_to_string(&path) {
            let marker = format!("\nplugin_unload_epoch = {epoch}\n");
            if text.contains("plugin_unload_epoch") {
                let re = text
                    .lines()
                    .map(|line| {
                        if line.trim_start().starts_with("plugin_unload_epoch") {
                            format!("plugin_unload_epoch = {epoch}")
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                let _ = std::fs::write(&path, re);
            } else {
                let _ = std::fs::write(&path, format!("{text}{marker}"));
            }
        }
    }
}

pub fn plugin_slot_poisoned(name: &str) -> bool {
    POISONED_PLUGINS
        .lock()
        .ok()
        .map(|g| g.iter().any(|n| n == name))
        .unwrap_or(false)
}
