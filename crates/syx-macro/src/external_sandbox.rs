//! Phase 57: seccomp subprocess sandbox for external macro expansion.

use std::io::{Read, Write};
use std::process::{Command, Stdio};

use syx_diagnostics::SyxResult;

use crate::external_protocol::{
    ExternalMacroExpander, InProcessMacroExpander, MacroExpandRequest, MacroExpandResponse,
    request_to_json, response_from_json,
};

/// When true, external macros expand in-process (dev/test only).
pub fn macro_in_process_enabled() -> bool {
    std::env::var("SYX_MACRO_IN_PROCESS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 119 honesty: sandbox is re-exec by default; Linux seccomp hard-fail only when enabled.
/// Fingerprint claim: `macro-external-v2:plugin-prod` must not advertise seccomp GA on Darwin.
pub fn macro_sandbox_mode() -> &'static str {
    if cfg!(target_os = "linux")
        && std::env::var("SYX_MACRO_SECCOMP")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    {
        "seccomp"
    } else {
        "sandbox-reexec-only"
    }
}

pub fn macro_external_plugin_prod_fingerprint() -> &'static str {
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
        expand_via_subprocess(req).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(format!("macro sandbox subprocess failed: {e}"))
                .with_code("E004")
        })
    }
}

fn expand_via_subprocess(req: &MacroExpandRequest) -> SyxResult<MacroExpandResponse> {
    let json = request_to_json(req)?;
    let exe = std::env::current_exe().map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let mut child = Command::new(exe)
        .arg("__macro_sandbox_expand__")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(json.as_bytes())
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    if !output.status.success() {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "macro sandbox subprocess failed",
        ));
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    response_from_json(&text)
}

/// Child entry for subprocess macro expansion (`syx __macro_sandbox_expand__`).
pub fn macro_sandbox_child_main() -> SyxResult<()> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let req: MacroExpandRequest = serde_json::from_str(&input)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let resp = InProcessMacroExpander.expand(&req)?;
    let out = serde_json::to_string(&resp)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    print!("{out}");
    Ok(())
}

pub fn is_macro_sandbox_child_argv(args: &[String]) -> bool {
    args.iter().any(|a| a == "__macro_sandbox_expand__")
}
