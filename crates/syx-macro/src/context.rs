//! Phase 56 Macro M5: expansion context.

use syx_diagnostics::Span;

/// Tracks macro expansion rounds and optional trace output.
#[derive(Debug, Clone)]
pub struct MacroContext {
    pub module_path: String,
    pub round: u32,
    pub site_span: Option<Span>,
}

impl MacroContext {
    pub fn new(module_path: impl Into<String>) -> Self {
        Self {
            module_path: module_path.into(),
            round: 0,
            site_span: None,
        }
    }

    pub fn trace_enabled() -> bool {
        std::env::var("SYX_MACRO_TRACE")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    pub fn trace(&self, msg: impl AsRef<str>) {
        if Self::trace_enabled() {
            eprintln!(
                "[syx-macro trace] module={} round={}: {}",
                self.module_path,
                self.round,
                msg.as_ref()
            );
        }
    }
}
