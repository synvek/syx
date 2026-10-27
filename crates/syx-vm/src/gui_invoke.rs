//! Phase 103: invoke registered SYX GUI handlers; pump errors surface as `error:…` (no swallow).
//! Fingerprint: gui-onclick-hard-v1:no-swallow-err

use std::sync::OnceLock;

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_runtime_core::Value;

use crate::program_ctx::with_program_ctx;

/// Native/MIR backend: dispatch GUI handler by name when no bytecode `ProgramCtx` is active.
pub type NativeGuiHandlerFn = fn(&str) -> SyxResult<Value>;

static NATIVE_GUI_HANDLER: OnceLock<NativeGuiHandlerFn> = OnceLock::new();

pub fn register_native_gui_handler(f: NativeGuiHandlerFn) {
    let _ = NATIVE_GUI_HANDLER.set(f);
}

/// Invoke a zero-arg SYX function registered as an onClick handler.
pub fn invoke_gui_handler(handler_name: &str) -> SyxResult<Value> {
    if let Some(ctx) = with_program_ctx(|ctx| ctx.clone()) {
        let handler_fn = ctx
            .program
            .functions
            .iter()
            .find(|f| f.name == handler_name)
            .ok_or_else(|| {
                SyxDiagnostic::error(format!("Unknown GUI handler function '{handler_name}'"))
            })?;
        let mut vm = crate::VmInstance::new();
        vm.runtime = ctx.runtime.clone();
        return vm.run_function_with_args(handler_fn, &ctx.program, &[]);
    }
    if let Some(handler) = NATIVE_GUI_HANDLER.get() {
        return handler(handler_name);
    }
    Err(
        SyxDiagnostic::error("GUI handler invoke requires active program context")
            .with_code("R012"),
    )
}

pub fn drain_pending_handlers_from_gui(win_id: &str) -> Vec<String> {
    #[cfg(feature = "gui-dlopen")]
    {
        if crate::gui_dlopen::dlopen_loaded() {
            return crate::gui_dlopen::drain_pending_handlers(win_id);
        }
    }
    let _ = win_id;
    Vec::new()
}

/// Drain queued handlers (collected during pump) and invoke each when a backend is active.
pub fn finish_pump_event(win_id: &str, default_event: &str) -> SyxResult<String> {
    let pending = drain_pending_handlers_from_gui(win_id);
    if pending.is_empty() {
        return Ok(default_event.to_string());
    }
    for name in &pending {
        if let Err(e) = invoke_gui_handler(name) {
            let msg = e.to_string();
            return Ok(format!("error:{msg}"));
        }
    }
    Ok(format!("invoked:{}", pending.join(",")))
}
