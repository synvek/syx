use std::sync::{Mutex, OnceLock};
use syx_diagnostics::SyxDiagnostic;

pub(crate) fn macro_warnings_store() -> &'static Mutex<Vec<SyxDiagnostic>> {
    static STORE: OnceLock<Mutex<Vec<SyxDiagnostic>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(Vec::new()))
}

/// Phase 19: diagnostics emitted by `#warning` during the last `expand_macros` call.
pub fn take_macro_warnings() -> Vec<SyxDiagnostic> {
    macro_warnings_store()
        .lock()
        .map(|mut w| std::mem::take(&mut *w))
        .unwrap_or_default()
}

pub(crate) fn push_macro_warning(diag: SyxDiagnostic) {
    if let Ok(mut w) = macro_warnings_store().lock() {
        w.push(diag);
    }
}
