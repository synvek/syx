//! ADR-278 / M3-B02: submit blocking I/O jobs onto SyxRuntime.blocking_pool when available.

use std::sync::OnceLock;

type BlockingExecuteFn = fn(Box<dyn FnOnce() + Send + 'static>);

static BLOCKING_EXECUTE: OnceLock<BlockingExecuteFn> = OnceLock::new();

/// Register the `syx-async` blocking-pool executor (called from `install_hosts`).
pub fn register_blocking_execute(f: BlockingExecuteFn) {
    let _ = BLOCKING_EXECUTE.set(f);
}

/// Run `job` on the dedicated blocking pool when registered; otherwise spawn a thread.
pub fn execute_on_blocking_pool(job: Box<dyn FnOnce() + Send + 'static>) {
    if let Some(f) = BLOCKING_EXECUTE.get() {
        f(job);
    } else {
        std::thread::spawn(move || job());
    }
}
