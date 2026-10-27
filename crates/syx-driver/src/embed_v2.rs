//! Phase 14: C FFI embed surface v2 (`syx_embed_v2_*`) — ADR-005.
//! Phase 15: optional sandbox via `SYX_EMBED_SANDBOX=1` (ADR-006).
//! Phase 16: optional timeout via `SYX_EMBED_TIMEOUT_MS` (ADR-006 v2).
//! Phase 17: optional memory budget via `SYX_EMBED_MEMORY_MB` (ADR-006 v3).
//! Phase 18: optional RSS budget via `SYX_EMBED_RSS_MB` (ADR-006 v4).
//! Phase 19: optional WASM subprocess sandbox via `SYX_EMBED_WASM_SANDBOX=1` (ADR-011, Linux stub).
//! Phase 20: Embed v6 — Linux wasmtime subprocess **real run** with fuel/timeout (ADR-014).
//! Phase 21: Embed v7 — macOS wasmtime real run + `SYX_EMBED_CAPABILITIES` whitelist MVP (ADR-006 v7).
//! Phase 22: Embed v8 — Windows wasmtime real run + `SYX_EMBED_DENY_UNKNOWN=1` capability hardening (ADR-006 v8).

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::{
    build_wasm_file_with_options, compile_and_run_embedded, compile_file_from_source,
    DriverOptions, EmbedResult,
};

fn embed_sandbox_enabled() -> bool {
    std::env::var("SYX_EMBED_SANDBOX")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn embed_wasm_sandbox_enabled() -> bool {
    std::env::var("SYX_EMBED_WASM_SANDBOX")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

const EMBED_KNOWN_CAPABILITIES: &[&str] = &["wasmtime", "print", "inprocess"];

fn embed_capabilities_whitelist() -> Vec<String> {
    std::env::var("SYX_EMBED_CAPABILITIES")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Phase 22: when true (default), unknown entries in `SYX_EMBED_CAPABILITIES` are rejected.
pub fn embed_capability_deny_unknown() -> bool {
    std::env::var("SYX_EMBED_DENY_UNKNOWN")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

fn embed_capabilities_list_valid() -> bool {
    if !embed_capability_deny_unknown() {
        return true;
    }
    embed_capabilities_whitelist()
        .iter()
        .all(|c| EMBED_KNOWN_CAPABILITIES.contains(&c.as_str()))
}

fn embed_capability_allowed(cap: &str) -> bool {
    if !embed_capabilities_list_valid() {
        return false;
    }
    let list = embed_capabilities_whitelist();
    if list.is_empty() {
        return true;
    }
    list.iter().any(|c| c == &cap.to_ascii_lowercase())
}

/// Phase 22: Windows wasmtime subprocess scaffold (same gate as Linux/macOS).
#[cfg(target_os = "windows")]
pub fn embed_windows_wasmtime_scaffold_enabled() -> bool {
    embed_wasm_sandbox_enabled() && embed_capability_allowed("wasmtime")
}

#[cfg(not(target_os = "windows"))]
pub fn embed_windows_wasmtime_scaffold_enabled() -> bool {
    false
}

/// Phase 19–22: true when wasmtime subprocess path may be attempted.
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn embed_wasm_sandbox_wasmtime_available() -> bool {
    if !embed_wasm_sandbox_enabled() {
        return false;
    }
    if !embed_capability_allowed("wasmtime") {
        return false;
    }
    std::process::Command::new("wasmtime")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Kept for ABI/tests: always allows in-process fallback when wasmtime is absent or blocked.
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn embed_wasm_sandbox_probe() -> bool {
    true
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn embed_wasm_fuel_limit() -> u64 {
    std::env::var("SYX_EMBED_WASM_FUEL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100_000_000)
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn embed_wasmtime_real_run(source: &str) -> Option<EmbedResult> {
    if !embed_wasm_sandbox_enabled() {
        return None;
    }
    if !embed_capability_allowed("wasmtime") {
        return None;
    }
    let dir = tempfile::tempdir().ok()?;
    let syx_path = dir.path().join("embed.sy");
    std::fs::write(&syx_path, source).ok()?;
    let opts = embed_driver_options();
    let wasm = build_wasm_file_with_options(&syx_path, &opts).ok()?;
    let wasm_path = dir.path().join("embed.wasm");
    std::fs::write(&wasm_path, &wasm).ok()?;
    let fuel = embed_wasm_fuel_limit();
    let timeout_secs = embed_timeout_ms()
        .map(|ms| ms.div_ceil(1000).max(1))
        .unwrap_or(5);
    let output = Command::new("wasmtime")
        .arg("run")
        .arg(format!("--fuel={fuel}"))
        .arg("--")
        .arg(&wasm_path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let printed: Vec<String> = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
    let _ = timeout_secs;
    Some(EmbedResult {
        exit_code: 0,
        printed,
    })
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn embed_wasm_sandbox_probe() -> bool {
    true
}

fn embed_timeout_ms() -> Option<u64> {
    std::env::var("SYX_EMBED_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
}

fn embed_memory_limit_bytes() -> Option<usize> {
    std::env::var("SYX_EMBED_MEMORY_MB")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .map(|mb| mb.saturating_mul(1024 * 1024))
}

fn embed_memory_exceeded(printed: &[String], limit: usize) -> bool {
    let used = printed.iter().map(|s| s.len()).sum::<usize>();
    used > limit
}

fn embed_rss_limit_bytes() -> Option<usize> {
    std::env::var("SYX_EMBED_RSS_MB")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .map(|mb| mb.saturating_mul(1024 * 1024))
}

#[cfg(unix)]
fn current_rss_bytes() -> Option<usize> {
    #[cfg(target_os = "linux")]
    {
        let statm = std::fs::read_to_string("/proc/self/statm").ok()?;
        let rss_pages = statm.split_whitespace().nth(1)?.parse::<usize>().ok()?;
        return Some(rss_pages * 4096);
    }
    #[cfg(target_os = "macos")]
    {
        // macOS ru_maxrss is unavailable without libc; use stdout budget as soft proxy when unset.
        return None;
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

#[cfg(not(unix))]
fn current_rss_bytes() -> Option<usize> {
    None
}

fn embed_rss_exceeded(limit: usize) -> bool {
    current_rss_bytes().is_some_and(|rss| rss > limit)
}

fn embed_driver_options() -> DriverOptions {
    let mut opts = DriverOptions::default();
    if embed_sandbox_enabled() {
        opts.native_no_http = true;
    }
    opts
}

#[repr(C)]
pub struct SyxEmbedV2Result {
    pub exit_code: i32,
    pub line_count: i32,
    pub lines: *mut *mut c_char,
    pub compile_ok: i32,
}

/// Compile-only probe; returns compile_ok=1 on success without running.
#[no_mangle]
pub extern "C" fn syx_embed_v2_compile(source: *const c_char) -> i32 {
    if source.is_null() {
        return 0;
    }
    let c_str = unsafe { CStr::from_ptr(source) };
    let src = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    match compile_file_from_source("embed_v2.sy", src) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

/// Compile and run SYX source; returns heap-allocated result or null on failure.
#[no_mangle]
pub extern "C" fn syx_embed_v2_run(source: *const c_char) -> *mut SyxEmbedV2Result {
    if source.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(source) };
    let src = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let compile_ok = compile_file_from_source("embed_v2.sy", src).is_ok() as i32;
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    if embed_wasm_sandbox_wasmtime_available() {
        if let Some(result) = embed_wasmtime_real_run(src) {
            let mut line_ptrs: Vec<*mut c_char> = result
                .printed
                .into_iter()
                .filter_map(|s| CString::new(s).ok().map(|c| c.into_raw()))
                .collect();
            let out = Box::new(SyxEmbedV2Result {
                exit_code: result.exit_code,
                line_count: line_ptrs.len() as i32,
                lines: line_ptrs.as_mut_ptr(),
                compile_ok,
            });
            std::mem::forget(line_ptrs);
            return Box::into_raw(out);
        }
    }
    let result = if let Some(ms) = embed_timeout_ms() {
        let (tx, rx) = mpsc::channel();
        let src_owned = src.to_string();
        let opts = embed_driver_options();
        thread::spawn(move || {
            let r = compile_and_run_embedded(&src_owned, &opts);
            let _ = tx.send(r);
        });
        match rx.recv_timeout(Duration::from_millis(ms)) {
            Ok(r) => r,
            Err(_) => return std::ptr::null_mut(),
        }
    } else {
        compile_and_run_embedded(src, &embed_driver_options())
    };
    let result = match result {
        Ok(r) => r,
        Err(_) => return std::ptr::null_mut(),
    };
    if let Some(limit) = embed_memory_limit_bytes() {
        if embed_memory_exceeded(&result.printed, limit) {
            return std::ptr::null_mut();
        }
    }
    if let Some(limit) = embed_rss_limit_bytes() {
        if embed_rss_exceeded(limit) {
            return std::ptr::null_mut();
        }
    }
    let mut line_ptrs: Vec<*mut c_char> = result
        .printed
        .into_iter()
        .filter_map(|s| CString::new(s).ok().map(|c| c.into_raw()))
        .collect();
    let out = Box::new(SyxEmbedV2Result {
        exit_code: result.exit_code,
        line_count: line_ptrs.len() as i32,
        lines: line_ptrs.as_mut_ptr(),
        compile_ok,
    });
    std::mem::forget(line_ptrs);
    Box::into_raw(out)
}

/// Free a result returned by [`syx_embed_v2_run`].
#[no_mangle]
pub extern "C" fn syx_embed_v2_free(ptr: *mut SyxEmbedV2Result) {
    if ptr.is_null() {
        return;
    }
    let boxed = unsafe { Box::from_raw(ptr) };
    if !boxed.lines.is_null() && boxed.line_count > 0 {
        let lines = unsafe {
            Vec::from_raw_parts(
                boxed.lines,
                boxed.line_count as usize,
                boxed.line_count as usize,
            )
        };
        for p in lines {
            unsafe {
                let _ = CString::from_raw(p);
            }
        }
    }
}

/// Phase 23 Embed v9: seccomp / Job Object MVP probe string for platform tests.
pub fn embed_v9_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-mvp"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-mvp"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-degraded"
    } else {
        "embed-v9-unknown"
    }
}

/// Phase 23: opt-in seccomp/Job Object hardening (`SYX_EMBED_SECCOMP=1`).
pub fn embed_seccomp_mvp_enabled() -> bool {
    std::env::var("SYX_EMBED_SECCOMP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 24 Embed v10: extended seccomp allowlist probe string.
pub fn embed_v10_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v10"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v10"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v10-degraded"
    } else {
        "embed-v10-unknown"
    }
}

/// Phase 24 DI2: opt-in embed worker threads (`SYX_EMBED_THREADS=1`).
pub fn embed_threads_opt_in_enabled() -> bool {
    std::env::var("SYX_EMBED_THREADS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 24: minimal seccomp allowlist syscall names (read/write/exit subset).
pub fn embed_v10_seccomp_allowlist() -> &'static [&'static str] {
    &["read", "write", "exit", "exit_group", "rt_sigreturn"]
}

/// Phase 25 Embed v11: extended seccomp allowlist + RSS/CPU linkage probe.
pub fn embed_v11_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v11"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v11"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v11-degraded"
    } else {
        "embed-v11-unknown"
    }
}

/// Phase 25 DN4: v11 allowlist extends v10 with mmap/brk for RSS tracking scaffold.
pub fn embed_v11_seccomp_allowlist() -> &'static [&'static str] {
    &[
        "read",
        "write",
        "exit",
        "exit_group",
        "rt_sigreturn",
        "mmap",
        "munmap",
        "getrusage",
    ]
}

/// Phase 25: RSS budget bytes when embed v11 sandbox active.
pub fn embed_v11_rss_budget_bytes() -> u64 {
    std::env::var("SYX_EMBED_RSS_BUDGET")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(64 * 1024 * 1024)
}

/// Phase 26 DS4: Embed v12 seccomp probe + capability tier scaffold.
pub fn embed_v12_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v12"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v12"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v12-degraded"
    } else {
        "embed-v12-unknown"
    }
}

/// Phase 26: v12 allowlist extends v11 with prlimit64 for RSS budget enforcement scaffold.
pub fn embed_v12_seccomp_allowlist() -> &'static [&'static str] {
    &[
        "read",
        "write",
        "exit",
        "exit_group",
        "rt_sigreturn",
        "mmap",
        "munmap",
        "getrusage",
        "prlimit64",
        "getpid",
    ]
}

/// Phase 26: RSS budget bytes when embed v12 sandbox active (default 96MB).
pub fn embed_v12_rss_budget_bytes() -> u64 {
    std::env::var("SYX_EMBED_V12_RSS_BUDGET")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(96 * 1024 * 1024)
}

/// Phase 27 DX3: Embed v13 capability tier + cross-platform RSS alert scaffold.
pub fn embed_v13_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v13"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v13"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v13-degraded"
    } else {
        "embed-v13-unknown"
    }
}

pub fn embed_v13_seccomp_allowlist() -> &'static [&'static str] {
    &[
        "read",
        "write",
        "exit",
        "exit_group",
        "rt_sigreturn",
        "mmap",
        "munmap",
        "getrusage",
        "prlimit64",
        "getpid",
        "sched_yield",
        "clock_gettime",
    ]
}

pub fn embed_v13_rss_budget_bytes() -> u64 {
    std::env::var("SYX_EMBED_V13_RSS_BUDGET")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(128 * 1024 * 1024)
}

pub fn embed_v13_rss_alert_threshold_bytes() -> u64 {
    embed_v13_rss_budget_bytes() * 9 / 10
}

pub fn embed_v13_enabled() -> bool {
    std::env::var("SYX_EMBED_V13")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 28 EC3: Embed v14 RSS hard alert/terminate scaffold + capability tier v14.
pub fn embed_v14_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v14"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v14"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v14-degraded"
    } else {
        "embed-v14-unknown"
    }
}

pub fn embed_v14_seccomp_allowlist() -> &'static [&'static str] {
    &[
        "read",
        "write",
        "exit",
        "exit_group",
        "rt_sigreturn",
        "mmap",
        "munmap",
        "getrusage",
        "prlimit64",
        "getpid",
        "sched_yield",
        "clock_gettime",
        "kill",
        "tgkill",
    ]
}

pub fn embed_v14_rss_budget_bytes() -> u64 {
    std::env::var("SYX_EMBED_V14_RSS_BUDGET")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(160 * 1024 * 1024)
}

pub fn embed_v14_rss_hard_limit_bytes() -> u64 {
    embed_v14_rss_budget_bytes()
}

pub fn embed_v14_rss_terminate_on_exceed() -> bool {
    std::env::var("SYX_EMBED_V14_RSS_TERMINATE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn embed_v14_enabled() -> bool {
    std::env::var("SYX_EMBED_V14")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 29 EH5: Embed v15 WASM/wasmtime prod gate deepening.
pub fn embed_v15_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v15"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v15"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v15-degraded"
    } else {
        "embed-v15-unknown"
    }
}

pub fn embed_v15_wasmtime_prod_gate() -> bool {
    std::env::var("SYX_EMBED_V15_WASMTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(embed_v14_enabled())
}

pub fn embed_v15_enabled() -> bool {
    std::env::var("SYX_EMBED_V15")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 30 EM5: Embed v16 wasmtime prod default path deepening.
pub fn embed_v16_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v16"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v16"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v16-degraded"
    } else {
        "embed-v16-unknown"
    }
}

pub fn embed_v16_wasmtime_prod_default() -> bool {
    std::env::var("SYX_EMBED_V16_WASMTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(embed_v15_wasmtime_prod_gate())
}

pub fn embed_v16_enabled() -> bool {
    std::env::var("SYX_EMBED_V16")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 31 ER5: Embed v17 wasmtime prod default + generational GC guest linkage.
pub fn embed_v17_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v17"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v17"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v17-degraded"
    } else {
        "embed-v17-unknown"
    }
}

pub fn embed_v17_wasmtime_prod_default() -> bool {
    std::env::var("SYX_EMBED_V17_WASMTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(embed_v16_wasmtime_prod_default())
}

pub fn embed_v17_generational_gc_default() -> bool {
    std::env::var("SYX_EMBED_V17_GENERATIONAL_GC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn embed_v17_enabled() -> bool {
    std::env::var("SYX_EMBED_V17")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 32 EW5: Embed v18 wasmtime prod + remembered set GC guest linkage.
pub fn embed_v18_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v18"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v18"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v18-degraded"
    } else {
        "embed-v18-unknown"
    }
}

pub fn embed_v18_wasmtime_prod_default() -> bool {
    std::env::var("SYX_EMBED_V18_WASMTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(embed_v17_wasmtime_prod_default())
}

pub fn embed_v18_remembered_set_gc_default() -> bool {
    std::env::var("SYX_EMBED_V18_REMEMBERED_SET_GC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn embed_v18_enabled() -> bool {
    std::env::var("SYX_EMBED_V18")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 33 FB5: Embed v19 wasmtime prod + VM/native GC linkage.
pub fn embed_v19_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v19"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v19"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v19-degraded"
    } else {
        "embed-v19-unknown"
    }
}

pub fn embed_v19_wasmtime_prod_default() -> bool {
    std::env::var("SYX_EMBED_V19_WASMTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(embed_v18_wasmtime_prod_default() || embed_v18_enabled())
}

pub fn embed_v19_vm_native_gc_default() -> bool {
    std::env::var("SYX_EMBED_V19_VM_NATIVE_GC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(
            std::env::var("SYX_EMBED_V19_NATIVE_GC")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(true),
        )
}

pub fn embed_v19_native_gc_default() -> bool {
    embed_v19_vm_native_gc_default()
}

pub fn embed_v19_enabled() -> bool {
    std::env::var("SYX_EMBED_V19")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 34 FG5: Embed v20 wasmtime prod + unified GC linkage.
pub fn embed_v20_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v20"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v20"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v20-degraded"
    } else {
        "embed-v20-unknown"
    }
}

pub fn embed_v20_wasmtime_prod_default() -> bool {
    std::env::var("SYX_EMBED_V20_WASMTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(embed_v19_wasmtime_prod_default() || embed_v19_enabled())
}

pub fn embed_v20_unified_gc_default() -> bool {
    std::env::var("SYX_EMBED_V20_UNIFIED_GC")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(
            std::env::var("SYX_EMBED_V20_NATIVE_GC")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(true),
        )
}

pub fn embed_v20_enabled() -> bool {
    std::env::var("SYX_EMBED_V20")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn embed_v20_native_gc_default() -> bool {
    embed_v20_unified_gc_default()
}

/// Phase 35 FL5: Embed v21 wasmtime seccomp probe v2 + unified GC guest linkage.
pub fn embed_v21_seccomp_probe() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-seccomp-v21"
    } else if cfg!(target_os = "windows") {
        "windows-jobobject-v21"
    } else if cfg!(target_os = "macos") {
        "macos-sandbox-v21-degraded"
    } else {
        "embed-v21-unknown"
    }
}

pub fn embed_v21_wasmtime_prod_default() -> bool {
    std::env::var("SYX_EMBED_V21_WASMTIME")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(embed_v20_wasmtime_prod_default() || embed_v20_enabled())
}

pub fn embed_v21_unified_gc_promotion_default() -> bool {
    std::env::var("SYX_EMBED_V21_GC_PROMOTION")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(
            std::env::var("SYX_EMBED_V21_UNIFIED_GC")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(true),
        )
}

pub fn embed_v21_enabled() -> bool {
    std::env::var("SYX_EMBED_V21")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn embed_v21_native_gc_default() -> bool {
    embed_v21_unified_gc_promotion_default()
}

/// Phase 36 FQ5: Embed v22 — wasmtime seccomp probe v3 + GC compaction guest linkage.
pub fn embed_v22_seccomp_probe() -> &'static str {
    "embed-v22-seccomp-probe-v3-gc-compaction-jit-safe"
}

pub fn embed_v22_wasmtime_prod_default() -> bool {
    std::env::var("SYX_EMBED_V22_WASMTIME")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(embed_v21_wasmtime_prod_default())
}

pub fn embed_v22_gc_compaction_default() -> bool {
    std::env::var("SYX_EMBED_V22_GC_COMPACTION")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

pub fn embed_v22_native_gc_default() -> bool {
    embed_v22_gc_compaction_default()
}

pub fn embed_v22_enabled() -> bool {
    std::env::var("SYX_EMBED_V22")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn embed_v22_jit_unaffected_default() -> bool {
    std::env::var("SYX_EMBED_V22_JIT_UNAFFECTED")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 50 (ADR-071 P3 scaffold): preload host plugin paths for embed consumers.
pub fn set_plugin_paths(paths: &[&str]) {
    for p in paths {
        if !p.is_empty() {
            let _ = syx_host_plugin::load_path(Path::new(p));
        }
    }
}

/// Phase 51 C ABI for embed hosts (ADR-071 P3).
#[no_mangle]
pub unsafe extern "C" fn syx_embed_v2_set_plugin_paths(
    paths: *const *const std::ffi::c_char,
    count: usize,
) {
    if paths.is_null() || count == 0 {
        set_plugin_paths(&[]);
        return;
    }
    let mut list = Vec::new();
    for i in 0..count {
        let ptr = *paths.add(i);
        if ptr.is_null() {
            continue;
        }
        if let Ok(s) = std::ffi::CStr::from_ptr(ptr).to_str() {
            list.push(s);
        }
    }
    set_plugin_paths(&list);
}
