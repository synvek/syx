//! `dlopen` / `LoadLibrary` plugin loader (ADR-071 P0).

use std::env;
use std::path::{Path, PathBuf};

use crate::manifest::{verify_signature, PluginManifest};
use crate::registry::{LoadedPluginHost, PluginHostCallback, PluginHostV2Callback};

pub fn plugins_enabled() -> bool {
    !matches!(
        env::var("SYX_HOST_PLUGINS").as_deref(),
        Ok("0") | Ok("false") | Ok("FALSE")
    )
}

pub fn plugin_paths_from_env() -> Vec<PathBuf> {
    env::var("SYX_HOST_PLUGIN_PATH")
        .ok()
        .map(|v| {
            v.split(':')
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect()
        })
        .unwrap_or_default()
}

/// Install-prefix roots that hold system plugins (`bin/`) and user plugins (`bin/plugins/`).
///
/// Search order:
/// 1. Directory containing the running `syx` / `syx-cli` binary
/// 2. `./bin` under the current working directory (repo/dev installs)
pub fn plugin_install_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.to_path_buf());
        }
    }
    if let Ok(cwd) = env::current_dir() {
        let bin = cwd.join("bin");
        if bin.is_dir() {
            roots.push(bin);
        }
    }
    roots.dedup();
    roots
}

/// True when a system faerlin dylib or manifest is present under `root` (install prefix).
pub fn system_faerlin_present(root: &Path) -> bool {
    root.join("faerlin").join("plugin.json").is_file()
        || root.join("libfaerlin_plugin.dylib").is_file()
        || root.join("libfaerlin_plugin.so").is_file()
        || root.join("faerlin_plugin.dll").is_file()
}

/// Register Mat-facing in-process faer / v2probe hosts (compile + runtime).
/// Called when a system faerlin install is detected next to `syx`.
pub fn register_linalg_system_hosts() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        register_faer_from_in_process();
        register_faer_zeros_in_process();
        register_faer_mul_in_process();
        register_faer_solve_in_process();
        register_faer_add_in_process();
        register_faer_sub_in_process();
        register_faer_hadamard_in_process();
        register_faer_to_in_process();
        register_v2probe_drop_in_process();
    });
}

fn load_plugin_tree(dir: &Path) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let nested = path.join("plugin.json");
            if nested.is_file() {
                let _ = load_path(&nested);
            }
        } else if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n == "plugin.json" || n.ends_with(".plugin.json"))
        {
            let _ = load_path(&path);
        }
    }
    Ok(())
}

/// Load system plugins from install roots (`bin/`) and user plugins from `bin/plugins/`.
pub fn load_from_exe_plugin_dirs() -> Result<(), String> {
    for root in plugin_install_roots() {
        if system_faerlin_present(&root) {
            register_linalg_system_hosts();
        }
        // System: manifests under bin/<id>/plugin.json (dylibs may sit flat in bin/).
        load_plugin_tree(&root)?;
        // Also try flat dylib next to a sibling source-tree style path if present.
        for name in [
            "libfaerlin_plugin.dylib",
            "libfaerlin_plugin.so",
            "faerlin_plugin.dll",
        ] {
            let lib = root.join(name);
            let manifest = root.join("faerlin").join("plugin.json");
            if lib.is_file() && manifest.is_file() {
                let _ = load_path(&manifest);
            }
        }
        // User extensions: bin/plugins/<id>/plugin.json
        load_plugin_tree(&root.join("plugins"))?;
    }
    Ok(())
}

/// Load plugins from env paths. Each path may be a `.so`/`.dylib`/`.dll` or `plugin.json`.
/// Phase 84: when `SYX_HOST_PLUGIN_PATH` is unset, auto-load from `syx-plugin-lock.json` / `syx.lock`.
/// Also loads install-prefix plugins from `bin/` (system) and `bin/plugins/` (user).
pub fn load_from_env() -> Result<(), String> {
    if !plugins_enabled() {
        return Ok(());
    }
    let env_paths = plugin_paths_from_env();
    for path in &env_paths {
        load_path(path)?;
    }
    // Always probe install-prefix layout (bin/ + bin/plugins/).
    load_from_exe_plugin_dirs()?;
    if env_paths.is_empty() {
        load_from_project_lock(Path::new("."))?;
    }
    Ok(())
}

/// Phase 84: load plugins declared in project lock files (default-on when plugins enabled).
pub fn load_from_project_lock(project_root: &Path) -> Result<(), String> {
    let json_lock = project_root.join("syx-plugin-lock.json");
    if json_lock.exists() {
        if let Ok(text) = std::fs::read_to_string(&json_lock) {
            #[derive(serde::Deserialize)]
            struct JsonLock {
                #[serde(default)]
                plugins: Vec<JsonPlugin>,
            }
            #[derive(serde::Deserialize)]
            struct JsonPlugin {
                id: String,
                library: Option<String>,
                path: Option<String>,
            }
            if let Ok(parsed) = serde_json::from_str::<JsonLock>(&text) {
                for p in parsed.plugins {
                    if let Some(path) = p.path.or(p.library) {
                        let lib = project_root.join(path);
                        if lib.exists() {
                            let _ = load_path(&lib);
                        }
                    }
                }
            }
        }
    }
    let syx_lock = project_root.join("syx.lock");
    if syx_lock.exists() {
        if let Ok(text) = std::fs::read_to_string(&syx_lock) {
            #[derive(serde::Deserialize)]
            struct SyxLockToml {
                #[serde(default)]
                plugins: Vec<PluginEntry>,
            }
            #[derive(serde::Deserialize)]
            struct PluginEntry {
                id: String,
                #[serde(default)]
                library: Option<String>,
            }
            if let Ok(parsed) = toml::from_str::<SyxLockToml>(&text) {
                for p in parsed.plugins {
                    if let Some(lib) = p.library {
                        let path = project_root.join("plugins").join(&p.id).join(&lib);
                        if path.exists() {
                            let _ = load_path(&path);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn load_path(path: &Path) -> Result<(), String> {
    if path.extension().and_then(|e| e.to_str()) == Some("json") {
        let manifest = PluginManifest::from_path(path)?;
        crate::manifest::verify_signature(&manifest)?;
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let lib = dir.join(&manifest.library);
        return load_library(&lib, &manifest);
    }
    if path.extension().is_some() {
        let manifest = PluginManifest {
            id: path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("plugin")
                .to_string(),
            api_version: 1,
            library: path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string(),
            hosts: vec![crate::manifest::PluginHostManifest {
                name: format!(
                    "__syx_plugin_{}_echo",
                    path.file_stem().and_then(|s| s.to_str()).unwrap_or("p")
                ),
                syx_name: Some("echo".into()),
                module: None,
                return_type: "String".into(),
                params: vec!["data: String".into()],
            }],
            min_syx_version: None,
            capabilities: vec![],
            signature: None,
        };
        return load_library(path, &manifest);
    }
    Err(format!("H001: unsupported plugin path {}", path.display()))
}

pub fn load_library(path: &Path, manifest: &PluginManifest) -> Result<(), String> {
    if !path.exists() {
        return Err(format!(
            "H001: plugin library not found: {}",
            path.display()
        ));
    }
    crate::manifest::verify_signature(manifest)?;
    load_library_dynamic(path, manifest)
}

fn load_library_dynamic(path: &Path, manifest: &PluginManifest) -> Result<(), String> {
    #[cfg(any(unix, windows))]
    {
        load_library_unix(path, manifest)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (path, manifest);
        Err("H001: dynamic plugins unsupported on this platform".into())
    }
}

#[cfg(any(unix, windows))]
fn load_library_unix(path: &Path, manifest: &PluginManifest) -> Result<(), String> {
    use libloading::{Library, Symbol};
    use std::ffi::CStr;

    type InitFn = unsafe extern "C" fn() -> *const crate::abi::SyxPluginInfo;

    let lib = unsafe { Library::new(path).map_err(|e| format!("H001: dlopen failed: {e}"))? };
    let init: Symbol<InitFn> = unsafe {
        lib.get(b"syx_plugin_init")
            .map_err(|e| format!("H001: dlsym syx_plugin_init: {e}"))?
    };
    let info = unsafe { init() };
    if info.is_null() {
        return Err("H001: syx_plugin_init returned null".into());
    }
    let info = unsafe { &*info };
    let version = info.api_version;
    if version != crate::abi::SYX_PLUGIN_API_VERSION
        && version != crate::abi::SYX_PLUGIN_API_VERSION_V2
        && version != crate::abi::SYX_PLUGIN_API_VERSION_V2M
    {
        return Err("H003: plugin API version mismatch".into());
    }
    let plugin_id = unsafe {
        CStr::from_ptr(info.plugin_id as *const _)
            .to_string_lossy()
            .into_owned()
    };
    if plugin_id != manifest.id {
        return Err(format!(
            "H002: plugin id mismatch {plugin_id} vs {}",
            manifest.id
        ));
    }
    crate::registry::register_compile_hosts_from_manifest(manifest);
    register_v1_hosts_from_info(info, &plugin_id)?;
    if version == crate::abi::SYX_PLUGIN_API_VERSION_V2
        || version == crate::abi::SYX_PLUGIN_API_VERSION_V2M
    {
        let info_v2 = unsafe {
            &*(info as *const crate::abi::SyxPluginInfo as *const crate::abi::SyxPluginInfoV2)
        };
        register_v2_hosts_from_info(info_v2, &plugin_id)?;
    }
    if version == crate::abi::SYX_PLUGIN_API_VERSION_V2M {
        let info_v2m = unsafe {
            &*(info as *const crate::abi::SyxPluginInfo as *const crate::abi::SyxPluginInfoV2m)
        };
        register_v2m_hosts_from_info(info_v2m, &plugin_id)?;
    }
    crate::registry::register_loaded_library(plugin_id, lib);
    Ok(())
}

#[cfg(any(unix, windows))]
fn register_v1_hosts_from_info(
    info: &crate::abi::SyxPluginInfo,
    plugin_id: &str,
) -> Result<(), String> {
    use std::ffi::CStr;
    let count = info.host_count as usize;
    if count == 0 || info.hosts.is_null() {
        return Ok(());
    }
    let hosts = unsafe { std::slice::from_raw_parts(info.hosts, count) };
    for host in hosts.iter() {
        let name = unsafe {
            CStr::from_ptr(host.name as *const _)
                .to_string_lossy()
                .into_owned()
        };
        let return_type = unsafe {
            CStr::from_ptr(host.return_type as *const _)
                .to_string_lossy()
                .into_owned()
        };
        let idx = syx_stdlib::host_fn_index_by_name(&name)
            .ok_or_else(|| format!("H002: compile host not registered for '{name}'"))?;
        let func = host.func;
        let cb: PluginHostCallback = std::sync::Arc::new(move |stack: &mut Vec<String>| {
            let input = stack.pop().unwrap_or_default();
            let mut out = vec![0u8; 8192];
            if let Some(f) = func {
                let c_in = std::ffi::CString::new(input.as_str()).unwrap_or_default();
                unsafe {
                    f(c_in.as_ptr() as *const u8, out.as_mut_ptr(), out.len());
                }
                let nul = out.iter().position(|&b| b == 0).unwrap_or(out.len());
                let result = String::from_utf8_lossy(&out[..nul]).into_owned();
                stack.push(result);
            } else {
                stack.push(input);
            }
            Ok(())
        });
        crate::registry::register_runtime_host(LoadedPluginHost {
            name,
            return_type,
            plugin_id: plugin_id.to_string(),
            index: idx,
            arity: host.arity,
            callback: cb,
            v2_callback: None,
        });
    }
    Ok(())
}

#[cfg(any(unix, windows))]
fn register_v2_hosts_from_info(
    info: &crate::abi::SyxPluginInfoV2,
    plugin_id: &str,
) -> Result<(), String> {
    use std::ffi::CStr;
    use syx_runtime_core::Value;
    let count = info.v2_host_count as usize;
    if count == 0 || info.v2_hosts.is_null() {
        return Ok(());
    }
    let hosts = unsafe { std::slice::from_raw_parts(info.v2_hosts, count) };
    for host in hosts.iter() {
        let name = unsafe {
            CStr::from_ptr(host.name as *const _)
                .to_string_lossy()
                .into_owned()
        };
        let return_type = unsafe {
            CStr::from_ptr(host.return_type as *const _)
                .to_string_lossy()
                .into_owned()
        };
        let idx = syx_stdlib::host_fn_index_by_name(&name)
            .ok_or_else(|| format!("H002: compile host not registered for '{name}'"))?;
        let func = host.func;
        let cb: PluginHostV2Callback = std::sync::Arc::new(move |stack: &mut Vec<Value>| {
            let arg = stack.pop().unwrap_or(Value::Nil);
            let arr = Value::materialize_array(arg)
                .ok_or_else(|| "H004: v2 host expects Array".to_string())?;
            let packed = crate::v2::pack_f64_le(&arr)?;
            let in_buf = crate::abi::SyxPluginBuf {
                kind: crate::abi::SYX_PLUGIN_BUF_F64LE,
                ptr: packed.as_ptr() as *mut u8,
                len: packed.len(),
            };
            let mut out = vec![0u8; crate::abi::SYX_PLUGIN_V2_OUT_CAP];
            let mut out_buf = crate::abi::SyxPluginBuf {
                kind: crate::abi::SYX_PLUGIN_BUF_F64LE,
                ptr: out.as_mut_ptr(),
                len: 0,
            };
            let rc = if let Some(f) = func {
                unsafe { f(&in_buf, &mut out_buf, out.len()) }
            } else {
                return Err("H004: v2 host missing fn".into());
            };
            if rc != 0 {
                return Err("H004: v2 host buffer overflow or error".into());
            }
            if out_buf.len > out.len() {
                return Err("H004: v2 host wrote past cap (no silent truncate)".into());
            }
            let result = crate::v2::unpack_f64_le(&out[..out_buf.len])?;
            stack.push(Value::Array(result));
            Ok(())
        });
        let dummy_v1: PluginHostCallback = std::sync::Arc::new(|_| Ok(()));
        crate::registry::register_runtime_host(LoadedPluginHost {
            name,
            return_type,
            plugin_id: plugin_id.to_string(),
            index: idx,
            arity: host.arity,
            callback: dummy_v1,
            v2_callback: Some(cb),
        });
    }
    Ok(())
}

#[cfg(any(unix, windows))]
fn register_v2m_hosts_from_info(
    info: &crate::abi::SyxPluginInfoV2m,
    plugin_id: &str,
) -> Result<(), String> {
    use std::ffi::CStr;
    use syx_runtime_core::Value;
    let count = info.v2m_host_count as usize;
    if count == 0 || info.v2m_hosts.is_null() {
        return Ok(());
    }
    let hosts = unsafe { std::slice::from_raw_parts(info.v2m_hosts, count) };
    for host in hosts.iter() {
        let name = unsafe {
            CStr::from_ptr(host.name as *const _)
                .to_string_lossy()
                .into_owned()
        };
        let return_type = unsafe {
            CStr::from_ptr(host.return_type as *const _)
                .to_string_lossy()
                .into_owned()
        };
        let idx = syx_stdlib::host_fn_index_by_name(&name)
            .ok_or_else(|| format!("H002: compile host not registered for '{name}'"))?;
        let func = host.func;
        let arity = host.arity;
        let cb: PluginHostV2Callback = std::sync::Arc::new(move |stack: &mut Vec<Value>| {
            let mut packed_slots: Vec<Vec<u8>> = Vec::new();
            let mut in_kinds: Vec<u32> = Vec::new();
            let mut in_bufs: Vec<crate::abi::SyxPluginBuf> = Vec::new();
            for _ in 0..arity {
                let arg = stack.pop().unwrap_or(Value::Nil);
                if let Ok(id) = crate::handles::value_as_handle(&arg) {
                    packed_slots.push(crate::handles::pack_handle(id).to_vec());
                    in_kinds.push(crate::abi::SYX_PLUGIN_BUF_HANDLE);
                } else {
                    let arr = Value::materialize_array(arg)
                        .ok_or_else(|| "H004: v2m host expects Array or handle".to_string())?;
                    packed_slots.push(crate::v2::pack_f64_le(&arr)?);
                    in_kinds.push(crate::abi::SYX_PLUGIN_BUF_F64LE);
                }
            }
            packed_slots.reverse();
            in_kinds.reverse();
            for (slot, kind) in packed_slots.iter_mut().zip(in_kinds.iter()) {
                in_bufs.push(crate::abi::SyxPluginBuf {
                    kind: *kind,
                    ptr: slot.as_mut_ptr(),
                    len: slot.len(),
                });
            }
            let in_iov = crate::abi::SyxPluginIov {
                count: in_bufs.len() as u32,
                bufs: in_bufs.as_mut_ptr(),
            };
            let mut ops = crate::v2::HostOpsCtx {
                out: vec![0u8; crate::abi::SYX_PLUGIN_V2M_GROW_INITIAL_CAP],
                borrows: Vec::new(),
            };
            let mut out_bufs = [crate::abi::SyxPluginBuf {
                kind: crate::abi::SYX_PLUGIN_BUF_F64LE,
                ptr: ops.out.as_mut_ptr(),
                len: 0,
            }];
            let mut out_iov = crate::abi::SyxPluginIov {
                count: 1,
                bufs: out_bufs.as_mut_ptr(),
            };
            let mut alloc = crate::abi::SyxPluginAlloc {
                ctx: (&mut ops as *mut crate::v2::HostOpsCtx).cast(),
                realloc_fn: Some(crate::v2::host_realloc),
                borrow_fn: Some(crate::v2::host_borrow),
                pin_fn: Some(crate::v2::host_pin),
                drop_fn: Some(crate::v2::host_drop),
            };
            let rc = if let Some(f) = func {
                unsafe { f(&in_iov, &mut out_iov, &mut alloc) }
            } else {
                return Err("H004: v2m host missing fn".into());
            };
            if rc != 0 {
                return Err("H004: v2m host buffer overflow or error".into());
            }
            let written = unsafe { (*out_iov.bufs).len };
            if written > ops.out.len() {
                return Err("H004: v2m host wrote past cap (no silent truncate)".into());
            }
            let out_kind = unsafe { (*out_iov.bufs).kind };
            if out_kind == crate::abi::SYX_PLUGIN_BUF_HANDLE {
                let id = crate::handles::unpack_handle_out(&ops.out[..written])?;
                stack.push(Value::I64(id as i64));
            } else {
                let result = crate::v2::unpack_f64_le(&ops.out[..written])?;
                stack.push(Value::Array(result));
            }
            Ok(())
        });
        let dummy_v1: PluginHostCallback = std::sync::Arc::new(|_| Ok(()));
        crate::registry::register_runtime_host(LoadedPluginHost {
            name,
            return_type,
            plugin_id: plugin_id.to_string(),
            index: idx,
            arity,
            callback: dummy_v1,
            v2_callback: Some(cb),
        });
    }
    Ok(())
}

/// Phase 103: unload dynamic library (`dlclose`) and poison epoch for stale symbols.
pub fn unload_plugin(plugin_id: &str) -> Result<(), String> {
    crate::registry::unload_plugin_library(plugin_id)
}

/// In-process mycrypto host for tests (Phase 54 real sha256).
pub fn register_mycrypto_plugin_in_process() {
    use sha2::{Digest, Sha256};
    let name = "__syx_plugin_mycrypto_sha256";
    syx_stdlib::register_compile_plugin_hosts(vec![(name, "String")]);
    let cb: PluginHostCallback = std::sync::Arc::new(|stack: &mut Vec<String>| {
        let s = stack.pop().unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let hex: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        stack.push(hex);
        Ok(())
    });
    let idx = syx_stdlib::host_fn_index_by_name(name)
        .unwrap_or(crate::registry::plugin_host_base_index());
    crate::registry::register_runtime_host(LoadedPluginHost {
        name: name.into(),
        return_type: "String".into(),
        plugin_id: "mycrypto".into(),
        index: idx,
        arity: 1,
        callback: cb,
        v2_callback: None,
    });
}

/// In-process echo host for tests when dlopen is unavailable.
pub fn register_echo_plugin_in_process() {
    let name = "__syx_plugin_echo_echo";
    syx_stdlib::register_compile_plugin_hosts(vec![(name, "String")]);
    let cb: PluginHostCallback = std::sync::Arc::new(|stack: &mut Vec<String>| {
        let s = stack.pop().unwrap_or_default();
        stack.push(s);
        Ok(())
    });
    let idx = syx_stdlib::host_fn_index_by_name(name)
        .unwrap_or(crate::registry::plugin_host_base_index());
    crate::registry::register_runtime_host(LoadedPluginHost {
        name: name.into(),
        return_type: "String".into(),
        plugin_id: "echo".into(),
        index: idx,
        arity: 1,
        callback: cb,
        v2_callback: None,
    });
}

/// In-process ABI v2 f64-sum probe (W1045 / ADR-1390). ≠ faer.
pub fn register_v2probe_sum_in_process() {
    use syx_runtime_core::Value;
    let name = "__syx_plugin_v2probe_sum";
    syx_stdlib::register_compile_plugin_hosts(vec![(name, "Array")]);
    let cb: PluginHostV2Callback = std::sync::Arc::new(|stack: &mut Vec<Value>| {
        let arg = stack.pop().unwrap_or(Value::Nil);
        let arr = Value::materialize_array(arg)
            .ok_or_else(|| "H004: v2probe expects Array".to_string())?;
        let sum = crate::v2::sum_f64_array(&arr)?;
        stack.push(Value::Array(vec![Value::F64(sum)]));
        Ok(())
    });
    let dummy_v1: PluginHostCallback = std::sync::Arc::new(|_| Ok(()));
    let idx = syx_stdlib::host_fn_index_by_name(name)
        .unwrap_or(crate::registry::plugin_host_base_index());
    crate::registry::register_runtime_host(LoadedPluginHost {
        name: name.into(),
        return_type: "Array".into(),
        plugin_id: "v2probe".into(),
        index: idx,
        arity: 1,
        callback: dummy_v1,
        v2_callback: Some(cb),
    });
}

/// In-process ABI v2m two-slot f64 dot product (W1046 / ADR-1391). ≠ faer.
pub fn register_v2probe_dot_in_process() {
    use syx_runtime_core::Value;
    let name = "__syx_plugin_v2probe_dot";
    syx_stdlib::register_compile_plugin_hosts(vec![(name, "Array")]);
    let cb: PluginHostV2Callback = std::sync::Arc::new(|stack: &mut Vec<Value>| {
        let b = stack.pop().unwrap_or(Value::Nil);
        let a = stack.pop().unwrap_or(Value::Nil);
        let aa = Value::materialize_array(a)
            .ok_or_else(|| "H004: v2probe_dot expects Array".to_string())?;
        let bb = Value::materialize_array(b)
            .ok_or_else(|| "H004: v2probe_dot expects Array".to_string())?;
        let dot = crate::v2::dot_f64_arrays(&aa, &bb)?;
        stack.push(Value::Array(vec![Value::F64(dot)]));
        Ok(())
    });
    let dummy_v1: PluginHostCallback = std::sync::Arc::new(|_| Ok(()));
    let idx = syx_stdlib::host_fn_index_by_name(name)
        .unwrap_or(crate::registry::plugin_host_base_index());
    crate::registry::register_runtime_host(LoadedPluginHost {
        name: name.into(),
        return_type: "Array".into(),
        plugin_id: "v2probe".into(),
        index: idx,
        arity: 2,
        callback: dummy_v1,
        v2_callback: Some(cb),
    });
}

/// In-process ABI v2m growable out (W1046 / ADR-1391). Initial cap 8, realloc if needed.
pub fn register_v2probe_grow_in_process() {
    use syx_runtime_core::Value;
    let name = "__syx_plugin_v2probe_grow";
    syx_stdlib::register_compile_plugin_hosts(vec![(name, "Array")]);
    let cb: PluginHostV2Callback = std::sync::Arc::new(|stack: &mut Vec<Value>| {
        let arg = stack.pop().unwrap_or(Value::Nil);
        let arr = Value::materialize_array(arg)
            .ok_or_else(|| "H004: v2probe_grow expects Array".to_string())?;
        let grown = crate::v2::grow_copy_f64(&arr, crate::abi::SYX_PLUGIN_V2M_GROW_INITIAL_CAP)?;
        let out = crate::v2::unpack_f64_le(&grown)?;
        stack.push(Value::Array(out));
        Ok(())
    });
    let dummy_v1: PluginHostCallback = std::sync::Arc::new(|_| Ok(()));
    let idx = syx_stdlib::host_fn_index_by_name(name)
        .unwrap_or(crate::registry::plugin_host_base_index());
    crate::registry::register_runtime_host(LoadedPluginHost {
        name: name.into(),
        return_type: "Array".into(),
        plugin_id: "v2probe".into(),
        index: idx,
        arity: 1,
        callback: dummy_v1,
        v2_callback: Some(cb),
    });
}

fn register_v2probe_named(
    name: &'static str,
    return_type: &'static str,
    arity: u32,
    cb: PluginHostV2Callback,
) {
    syx_stdlib::register_compile_plugin_hosts(vec![(name, return_type)]);
    let dummy_v1: PluginHostCallback = std::sync::Arc::new(|_| Ok(()));
    let idx = syx_stdlib::host_fn_index_by_name(name)
        .unwrap_or(crate::registry::plugin_host_base_index());
    crate::registry::register_runtime_host(LoadedPluginHost {
        name: name.into(),
        return_type: return_type.into(),
        plugin_id: "v2probe".into(),
        index: idx,
        arity,
        callback: dummy_v1,
        v2_callback: Some(cb),
    });
}

/// In-process handle pin (W1047 / ADR-1392). Array f64 → I64. ≠ faer.
pub fn register_v2probe_pin_in_process() {
    use syx_runtime_core::Value;
    register_v2probe_named(
        "__syx_plugin_v2probe_pin",
        "Int",
        1,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let arg = stack.pop().unwrap_or(Value::Nil);
            let arr = Value::materialize_array(arg)
                .ok_or_else(|| "H004: v2probe_pin expects Array".to_string())?;
            let id = crate::handles::pin_f64_values(&arr)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process handle sum (W1047 / ADR-1392). I64 handle → Array of one f64.
pub fn register_v2probe_sumh_in_process() {
    use syx_runtime_core::Value;
    register_v2probe_named(
        "__syx_plugin_v2probe_sumh",
        "Array",
        1,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let arg = stack.pop().unwrap_or(Value::Nil);
            let id = crate::handles::value_as_handle(&arg)?;
            let sum = crate::handles::sum_handle(id)?;
            stack.push(Value::Array(vec![Value::F64(sum)]));
            Ok(())
        }),
    );
}

/// In-process handle drop (W1047 / ADR-1392). Unknown / double drop fails.
pub fn register_v2probe_drop_in_process() {
    use syx_runtime_core::Value;
    register_v2probe_named(
        "__syx_plugin_v2probe_drop",
        "Int",
        1,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let arg = stack.pop().unwrap_or(Value::Nil);
            let id = crate::handles::value_as_handle(&arg)?;
            crate::handles::drop_handle(id)?;
            stack.push(Value::I64(0));
            Ok(())
        }),
    );
}

fn register_faer_named(
    name: &'static str,
    return_type: &'static str,
    arity: u32,
    cb: PluginHostV2Callback,
) {
    syx_stdlib::register_compile_plugin_hosts(vec![(name, return_type)]);
    let dummy_v1: PluginHostCallback = std::sync::Arc::new(|_| Ok(()));
    let idx = syx_stdlib::host_fn_index_by_name(name)
        .unwrap_or(crate::registry::plugin_host_base_index());
    crate::registry::register_runtime_host(LoadedPluginHost {
        name: name.into(),
        return_type: return_type.into(),
        plugin_id: "faerlin".into(),
        index: idx,
        arity,
        callback: dummy_v1,
        v2_callback: Some(cb),
    });
}

/// In-process faer from (W1048 / ADR-1393). Array + rows + cols → I64.
pub fn register_faer_from_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_from",
        "Int",
        3,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let cols = crate::handles::value_as_dim(&stack.pop().unwrap_or(Value::Nil))?;
            let rows = crate::handles::value_as_dim(&stack.pop().unwrap_or(Value::Nil))?;
            let arr = Value::materialize_array(stack.pop().unwrap_or(Value::Nil))
                .ok_or_else(|| "H004: faer_from expects Array".to_string())?;
            let id = crate::handles::pin_shaped(&arr, rows, cols)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process faer zeros (W1048 / ADR-1393).
pub fn register_faer_zeros_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_zeros",
        "Int",
        2,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let cols = crate::handles::value_as_dim(&stack.pop().unwrap_or(Value::Nil))?;
            let rows = crate::handles::value_as_dim(&stack.pop().unwrap_or(Value::Nil))?;
            let id = crate::handles::pin_zeros(rows, cols)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process faer mul (W1048 / ADR-1393). Requires feature p-faer.
pub fn register_faer_mul_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_mul",
        "Int",
        2,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let b = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let a = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let id = crate::handles::mul_handles(a, b)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process faer solve (W1049 / ADR-1394). Requires feature p-faer.
pub fn register_faer_solve_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_solve",
        "Int",
        2,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let b = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let a = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let id = crate::handles::solve_handles(a, b)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process faer add (W1103 / ADR-1449). Requires feature p-faer.
pub fn register_faer_add_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_add",
        "Int",
        2,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let b = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let a = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let id = crate::handles::add_handles(a, b)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process faer sub (W1103 / ADR-1449). Requires feature p-faer.
pub fn register_faer_sub_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_sub",
        "Int",
        2,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let b = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let a = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let id = crate::handles::sub_handles(a, b)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process faer hadamard / element-wise mul (W1108 / ADR-1454). Requires feature p-faer.
pub fn register_faer_hadamard_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_hadamard",
        "Int",
        2,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let b = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let a = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let id = crate::handles::hadamard_handles(a, b)?;
            stack.push(Value::I64(id));
            Ok(())
        }),
    );
}

/// In-process faer to-array (W1048 / ADR-1393).
pub fn register_faer_to_in_process() {
    use syx_runtime_core::Value;
    register_faer_named(
        "__syx_plugin_faer_to",
        "Array",
        1,
        std::sync::Arc::new(|stack: &mut Vec<Value>| {
            let id = crate::handles::value_as_handle(&stack.pop().unwrap_or(Value::Nil))?;
            let out = crate::handles::buf_to_values(id)?;
            stack.push(Value::Array(out));
            Ok(())
        }),
    );
}

/// Phase 52: install plugin manifest + library into `.syx/plugins/<id>/`.
pub fn install_plugin(src: &Path, project_root: &Path) -> Result<String, String> {
    use crate::manifest::PluginManifest;
    let manifest = PluginManifest::from_path(src)?;
    crate::manifest::verify_signature(&manifest)?;
    let dest = project_root.join(".syx/plugins").join(&manifest.id);
    std::fs::create_dir_all(&dest).map_err(|e| format!("install mkdir: {e}"))?;
    std::fs::copy(src, dest.join("plugin.json"))
        .map_err(|e| format!("install copy manifest: {e}"))?;
    let lib_src = src
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(&manifest.library);
    if lib_src.exists() {
        std::fs::copy(&lib_src, dest.join(&manifest.library))
            .map_err(|e| format!("install copy library: {e}"))?;
    }
    write_plugin_lock(project_root, &manifest)?;
    Ok(manifest.id)
}

fn write_plugin_lock(project_root: &Path, manifest: &PluginManifest) -> Result<(), String> {
    use crate::registry::{PluginLock, PluginLockEntry};
    let base = crate::registry::plugin_host_base_index();
    let entry = PluginLockEntry {
        plugin_id: manifest.id.clone(),
        hosts: manifest.hosts.iter().map(|h| h.name.clone()).collect(),
    };
    let mut lock = PluginLock {
        plugins: vec![entry],
        plugin_base: base,
        plugin_unload_epoch: crate::registry::plugin_unload_epoch(),
    };
    if let Ok(text) = std::fs::read_to_string(project_root.join("syx.lock")) {
        #[derive(serde::Deserialize)]
        struct SyxLockToml {
            #[serde(default)]
            plugins: Vec<PluginLockTomlEntry>,
        }
        #[derive(serde::Deserialize)]
        struct PluginLockTomlEntry {
            id: String,
            hosts: Vec<String>,
        }
        if let Ok(parsed) = toml::from_str::<SyxLockToml>(&text) {
            lock.plugins = parsed
                .plugins
                .into_iter()
                .filter(|e| e.id != manifest.id)
                .map(|e| PluginLockEntry {
                    plugin_id: e.id,
                    hosts: e.hosts,
                })
                .collect();
            lock.plugins.push(PluginLockEntry {
                plugin_id: manifest.id.clone(),
                hosts: manifest.hosts.iter().map(|h| h.name.clone()).collect(),
            });
        }
    }
    crate::registry::set_plugin_lock(lock);

    #[derive(serde::Serialize, serde::Deserialize)]
    struct PluginLockTomlEntry {
        id: String,
        hosts: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        plugin_base: Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default)]
    struct SyxLockToml {
        #[serde(default)]
        packages: toml::value::Table,
        #[serde(default)]
        plugins: Vec<PluginLockTomlEntry>,
    }
    let syx_lock_path = project_root.join("syx.lock");
    let mut syx_lock = if syx_lock_path.exists() {
        std::fs::read_to_string(&syx_lock_path)
            .ok()
            .and_then(|t| toml::from_str::<SyxLockToml>(&t).ok())
            .unwrap_or_default()
    } else {
        SyxLockToml::default()
    };
    syx_lock.plugins.retain(|e| e.id != manifest.id);
    syx_lock.plugins.push(PluginLockTomlEntry {
        id: manifest.id.clone(),
        hosts: manifest.hosts.iter().map(|h| h.name.clone()).collect(),
        plugin_base: Some(base),
    });
    let syx_text =
        toml::to_string_pretty(&syx_lock).map_err(|e| format!("syx.lock encode: {e}"))?;
    std::fs::write(&syx_lock_path, syx_text).map_err(|e| format!("syx.lock write: {e}"))?;
    Ok(())
}
