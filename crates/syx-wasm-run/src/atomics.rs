use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicI32, Ordering};

/// Phase 21 CT1 / Phase 22 CY1: MVP SharedArrayBuffer + Atomics host hook (single-threaded host simulation).
pub struct SharedAtomicsBuffer {
    pub cells: Vec<AtomicI32>,
}

impl SharedAtomicsBuffer {
    pub fn new(len: usize) -> Self {
        Self {
            cells: (0..len).map(|_| AtomicI32::new(0)).collect(),
        }
    }

    pub fn atomic_add_i32(&self, index: usize, delta: i32) -> i32 {
        if index >= self.cells.len() {
            return 0;
        }
        let old = self.cells[index].fetch_add(delta, Ordering::SeqCst);
        old + delta
    }
}

/// Phase 22 CY1: worker message queue entry (host-side MVP).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerMessage {
    pub worker_id: i32,
    pub payload: String,
}

static WORKER_MESSAGES: OnceLock<Mutex<VecDeque<WorkerMessage>>> = OnceLock::new();

fn worker_messages() -> &'static Mutex<VecDeque<WorkerMessage>> {
    WORKER_MESSAGES.get_or_init(|| Mutex::new(VecDeque::new()))
}

pub fn post_message(worker_id: i32, payload: impl Into<String>) {
    worker_messages()
        .lock()
        .unwrap()
        .push_back(WorkerMessage {
            worker_id,
            payload: payload.into(),
        });
}

pub fn get_message(worker_id: i32) -> Option<String> {
    let mut queue = worker_messages().lock().unwrap();
    if let Some(pos) = queue
        .iter()
        .position(|m| m.worker_id == worker_id)
    {
        return queue.remove(pos).map(|m| m.payload);
    }
    None
}

pub fn wasm_threads_mvp_enabled() -> bool {
    match std::env::var("SYX_WASM_THREADS") {
        Ok(v) if v == "0" || v.eq_ignore_ascii_case("false") => false,
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => true,
        Ok(_) => false,
        Err(_) => wasm_threads_release_default(),
    }
}

/// Phase 26 DS3: WASM threads release default on (opt-out via `SYX_WASM_THREADS=0`).
pub fn wasm_threads_release_default() -> bool {
    std::env::var("SYX_WASM_THREADS_RELEASE_DEFAULT")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 23 DD1: nightly builds default WASM threads on unless explicitly disabled.
pub fn wasm_threads_nightly_default_enabled() -> bool {
    std::env::var("SYX_NIGHTLY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 22 CY1: production opt-in path — same gate as MVP (`SYX_WASM_THREADS=1`).
pub fn wasm_threads_prod_enabled() -> bool {
    wasm_threads_mvp_enabled()
}

pub const WASM_THREADS_MVP: &str =
    "SharedArrayBuffer+Atomics MVP (Phase 21; Phase 22 prod opt-in via SYX_WASM_THREADS=1)";

/// Phase 24 DI1: release production hardening — worker fuel/timeout isolation when threads enabled.
pub fn wasm_threads_release_prod_hardened() -> bool {
    wasm_threads_mvp_enabled()
        && std::env::var("SYX_WASM_THREADS_RELEASE_PROD")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
}

static WORKER_FUEL: AtomicI32 = AtomicI32::new(1000);

pub fn wasm_thread_worker_fuel_remaining() -> i32 {
    WORKER_FUEL.load(Ordering::SeqCst)
}

pub fn wasm_thread_worker_consume_fuel(amount: i32) -> bool {
    if !wasm_threads_release_prod_hardened() {
        return true;
    }
    let cur = WORKER_FUEL.load(Ordering::SeqCst);
    if cur <= 0 {
        return false;
    }
    WORKER_FUEL.fetch_sub(amount.max(0), Ordering::SeqCst);
    true
}

/// Phase 25 DN3: evaluate release default for WASM threads (documented; still env-gated).
pub fn wasm_threads_release_default_evaluated() -> bool {
    wasm_threads_release_default() || wasm_threads_mvp_enabled()
}

pub fn wasm_thread_worker_reset_fuel() {
    WORKER_FUEL.store(1000, Ordering::SeqCst);
}
