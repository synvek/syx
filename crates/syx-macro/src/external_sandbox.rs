//! External macro expansion: crash-containment reexec only (C-5a).
//! ≠ OS/seccomp sandbox GA — see `docs/adr/macro-embed-threat-model.md`.
//! C-5c-1: opt-in `SYX_MACRO_TIMEOUT_MS` wall clock on subprocess (kill → M040).
//! C-5c-2: opt-in `SYX_MACRO_SECCOMP=1` → Linux net-deny BPF in child; non-Linux fail-closed.
//! C-5d: opt-in `SYX_MACRO_RSS_MB` → Linux `prlimit(RLIMIT_AS)` in child.
//! C-5e: same env → Windows Job Object `PROCESS_MEMORY_LIMIT` (parent assign); else → M040.

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use syx_diagnostics::{Span, SyxResult};

use crate::external_protocol::{
    request_to_json, response_from_json, ExternalMacroExpander, InProcessMacroExpander,
    MacroExpandRequest, MacroExpandResponse,
};
use crate::jobobject_windows::try_apply_rss_hard_wall_for_pid;
use crate::macro_error_at;
use crate::rlimit_linux::try_set_as_limit_bytes;
use crate::seccomp_linux::{
    try_install_seccomp_netdeny, MODE_SECCOMP_NETDENY_WIRED, MODE_SECCOMP_UNSUPPORTED,
};

/// When true, external macros expand in-process (dev/test only).
pub fn macro_in_process_enabled() -> bool {
    std::env::var("SYX_MACRO_IN_PROCESS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn macro_seccomp_requested() -> bool {
    std::env::var("SYX_MACRO_SECCOMP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// C-5c-2: default re-exec only. `SYX_MACRO_SECCOMP=1` → Linux `seccomp-netdeny-wired`
/// (child installs BPF); non-Linux → `seccomp-unsupported-fail-closed`.
/// Never advertise bare `"seccomp"` as GA. Historical C-5a literal kept for census:
#[allow(dead_code)]
const _C5A_HISTORICAL_LABEL: &str = "seccomp-label-unwired";

pub fn macro_sandbox_mode() -> &'static str {
    if !macro_seccomp_requested() {
        return "sandbox-reexec-only";
    }
    if cfg!(target_os = "linux") {
        // C-5c-2 wired label (child installs BPF). Keep historical token in-fn for C-5a census.
        let _ = "seccomp-label-unwired";
        MODE_SECCOMP_NETDENY_WIRED
    } else {
        let _ = _C5A_HISTORICAL_LABEL;
        MODE_SECCOMP_UNSUPPORTED
    }
}

pub fn macro_external_plugin_prod_digest() -> &'static str {
    "macro-external-v2:plugin-prod"
}

/// When true, WASM target rejects external macro expansion (E004).
pub fn wasm_external_macro_disabled() -> bool {
    std::env::var("SYX_TARGET_WASM")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn select_expander() -> Box<dyn ExternalMacroExpander> {
    if macro_in_process_enabled() {
        Box::new(InProcessMacroExpander)
    } else {
        Box::new(SandboxMacroExpander)
    }
}

pub struct SandboxMacroExpander;

impl ExternalMacroExpander for SandboxMacroExpander {
    fn expand(&self, req: &MacroExpandRequest) -> SyxResult<MacroExpandResponse> {
        // C-5c-2: non-Linux + SYX_MACRO_SECCOMP=1 → fail-closed (no pretend install).
        if macro_seccomp_requested() && !cfg!(target_os = "linux") {
            return Err(macro_error_at(
                "macro seccomp requested but unsupported on this OS (C-5c-2 fail-closed)",
                None,
                Span::DUMMY,
                "M040",
            ));
        }
        // C-5d/C-5e: RSS hard wall only on Linux (prlimit) or Windows (Job Object).
        if macro_rss_limit_bytes().is_some()
            && !cfg!(any(target_os = "linux", target_os = "windows"))
        {
            return Err(macro_error_at(
                "macro RSS hard wall requested but unsupported on this OS (C-5e fail-closed)",
                None,
                Span::DUMMY,
                "M040",
            ));
        }
        expand_via_subprocess(req).map_err(|e| {
            macro_error_at(
                format!("macro sandbox subprocess failed: {e}"),
                None,
                Span::DUMMY,
                "E004",
            )
        })
    }
}

fn macro_timeout_ms() -> Option<u64> {
    std::env::var("SYX_MACRO_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
}

/// C-5d: opt-in hard AS wall for macro child (`SYX_MACRO_RSS_MB`).
fn macro_rss_limit_bytes() -> Option<u64> {
    std::env::var("SYX_MACRO_RSS_MB")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(|mb| mb.saturating_mul(1024 * 1024))
}

fn expand_via_subprocess(req: &MacroExpandRequest) -> SyxResult<MacroExpandResponse> {
    let json = request_to_json(req)?;
    let exe = std::env::current_exe()
        .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?;
    let mut child = Command::new(exe)
        .arg("__macro_sandbox_expand__")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?;
    // C-5e: Windows parent assigns child to Job Object after spawn (before stdin close/wait).
    if let Some(limit) = macro_rss_limit_bytes() {
        if cfg!(target_os = "windows") {
            try_apply_rss_hard_wall_for_pid(child.id(), limit).map_err(|e| {
                let _ = child.kill();
                let _ = child.wait();
                macro_error_at(
                    format!("macro Job Object assign failed: {e}"),
                    None,
                    Span::DUMMY,
                    "M040",
                )
            })?;
        }
    }
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(json.as_bytes())
            .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?;
    }
    let output = match macro_timeout_ms() {
        None => child
            .wait_with_output()
            .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?,
        Some(ms) => wait_child_output_deadline(child, ms).map_err(|e| {
            macro_error_at(
                format!("macro sandbox subprocess timeout/fail: {e}"),
                None,
                Span::DUMMY,
                "M040",
            )
        })?,
    };
    if !output.status.success() {
        return Err(macro_error_at(
            "macro sandbox subprocess failed",
            None,
            Span::DUMMY,
            "M040",
        ));
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?;
    response_from_json(&text)
}

/// C-5c-1: wall-clock wait; on timeout kill child and error.
fn wait_child_output_deadline(
    mut child: std::process::Child,
    timeout_ms: u64,
) -> Result<std::process::Output, String> {
    let mut stdout_pipe = child
        .stdout
        .take()
        .ok_or_else(|| "missing stdout".to_string())?;
    let out_h = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buf);
        buf
    });
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = out_h
                    .join()
                    .map_err(|_| "stdout join failed".to_string())?;
                return Ok(std::process::Output {
                    status,
                    stdout,
                    stderr: Vec::new(),
                });
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!("timeout after {timeout_ms}ms"));
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => return Err(e.to_string()),
        }
    }
}

/// Child entry for subprocess macro expansion (`syx __macro_sandbox_expand__`).
pub fn macro_sandbox_child_main() -> SyxResult<()> {
    // C-5c-2: install net-deny BPF before expand when opted in (Linux only reaches here
    // with env set from a successful parent path; still fail-closed on install error).
    if macro_seccomp_requested() {
        try_install_seccomp_netdeny().map_err(|e| {
            macro_error_at(
                format!("macro seccomp install failed: {e}"),
                None,
                Span::DUMMY,
                "M040",
            )
        })?;
    }
    // C-5d: Linux child self-installs RLIMIT_AS; Windows Job Object is parent-assign (C-5e).
    if let Some(limit) = macro_rss_limit_bytes() {
        if cfg!(target_os = "linux") {
            try_set_as_limit_bytes(limit).map_err(|e| {
                macro_error_at(
                    format!("macro RLIMIT_AS install failed: {e}"),
                    None,
                    Span::DUMMY,
                    "M040",
                )
            })?;
        }
    }
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?;
    let req: MacroExpandRequest = serde_json::from_str(&input)
        .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?;
    let resp = InProcessMacroExpander.expand(&req)?;
    let out = serde_json::to_string(&resp)
        .map_err(|e| macro_error_at(e.to_string(), None, Span::DUMMY, "M039"))?;
    print!("{out}");
    Ok(())
}

pub fn is_macro_sandbox_child_argv(args: &[String]) -> bool {
    args.iter().any(|a| a == "__macro_sandbox_expand__")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c5c1_wait_child_deadline_kills_sleep() {
        let child = Command::new("sleep")
            .arg("30")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn sleep");
        let err = wait_child_output_deadline(child, 50).expect_err("must timeout");
        assert!(err.contains("timeout"), "{err}");
    }

    #[test]
    fn c5c1_macro_timeout_ms_reads_env() {
        std::env::remove_var("SYX_MACRO_TIMEOUT_MS");
        assert!(macro_timeout_ms().is_none());
        std::env::set_var("SYX_MACRO_TIMEOUT_MS", "123");
        assert_eq!(macro_timeout_ms(), Some(123));
        std::env::remove_var("SYX_MACRO_TIMEOUT_MS");
    }
}
