//! R-25: serialize wasm tests that mutate process-global `RUNTIME_CONFIG`.
//!
//! Without a snapshot/restore guard, `install_runtime_config(mir_exec_only: true)`
//! (or simd128 toggles) leaks across parallel tests and poisons siblings
//! (`aot_dict_hashable` → Phase 100 W001).
//!
//! Re-entrant on the same thread: `with_aot(|| emit_fixture(..))` nests another
//! `with_aot` — a plain `Mutex` would deadlock.

use std::cell::Cell;
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

use syx_config::SyxRuntimeConfig;

static CONFIG_LOCK: Mutex<()> = Mutex::new(());

thread_local! {
    static LOCK_HELD: Cell<bool> = const { Cell::new(false) };
}

/// Holds the process-wide config lock (outermost enter only) and restores the
/// pre-enter snapshot on drop.
pub struct RuntimeConfigGuard {
    _lock: Option<MutexGuard<'static, ()>>,
    /// `Some` only for the outermost enter on this thread.
    snapshot: Option<SyxRuntimeConfig>,
    /// Nested enter: restore this cfg when the inner scope ends (outer keeps lock).
    nested_restore: Option<SyxRuntimeConfig>,
}

impl RuntimeConfigGuard {
    pub fn enter() -> Self {
        if LOCK_HELD.with(|h| h.get()) {
            return Self {
                _lock: None,
                snapshot: None,
                nested_restore: Some(syx_config::runtime_config()),
            };
        }
        let lock = CONFIG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        LOCK_HELD.with(|h| h.set(true));
        let snapshot = syx_config::runtime_config();
        Self {
            _lock: Some(lock),
            snapshot: Some(snapshot),
            nested_restore: None,
        }
    }
}

impl Drop for RuntimeConfigGuard {
    fn drop(&mut self) {
        if let Some(prev) = self.nested_restore.take() {
            syx_config::install_runtime_config(prev);
            return;
        }
        if let Some(snapshot) = self.snapshot.take() {
            syx_config::install_runtime_config(snapshot);
            LOCK_HELD.with(|h| h.set(false));
        }
    }
}

fn restore_env(key: &str, prev: Option<OsString>) {
    match prev {
        Some(v) => std::env::set_var(key, v),
        None => std::env::remove_var(key),
    }
}

/// Install `cfg` under the global lock; restore the prior config when `f` returns.
pub fn with_runtime_config<F, T>(cfg: SyxRuntimeConfig, f: F) -> T
where
    F: FnOnce() -> T,
{
    let _guard = RuntimeConfigGuard::enter();
    syx_config::install_runtime_config(cfg);
    f()
}

/// AOT-on / mir_exec_only-off for the duration of `f`.
pub fn with_aot<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    with_runtime_config(
        SyxRuntimeConfig {
            wasm: syx_config::WasmConfig {
                aot: true,
                mir_exec_only: false,
            },
            ..SyxRuntimeConfig::default()
        },
        f,
    )
}

/// Enable wasm simd128 + tier-3 for the duration of `f`; restore config and env on drop.
pub fn with_simd128_tier3<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let _guard = RuntimeConfigGuard::enter();
    let prev_simd = std::env::var_os("SYX_WASM_SIMD128");
    let prev_tier3 = std::env::var_os("SYX_WASM_AOT_TIER3");
    let prev_mir = std::env::var_os("SYX_WASM_MIR_EXEC_ONLY");
    std::env::set_var("SYX_WASM_SIMD128", "1");
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    std::env::remove_var("SYX_WASM_MIR_EXEC_ONLY");
    let mut cfg = syx_config::runtime_config();
    cfg.enable_wasm_simd128 = true;
    cfg.wasm.mir_exec_only = false;
    syx_config::install_runtime_config(cfg);

    struct EnvRestore {
        prev_simd: Option<OsString>,
        prev_tier3: Option<OsString>,
        prev_mir: Option<OsString>,
    }
    impl Drop for EnvRestore {
        fn drop(&mut self) {
            restore_env("SYX_WASM_SIMD128", self.prev_simd.take());
            restore_env("SYX_WASM_AOT_TIER3", self.prev_tier3.take());
            restore_env("SYX_WASM_MIR_EXEC_ONLY", self.prev_mir.take());
        }
    }
    let _env = EnvRestore {
        prev_simd,
        prev_tier3,
        prev_mir,
    };
    f()
}
