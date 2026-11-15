//! Link-time shakeable host slots (SQL/HTTP/TLS domains for native AOT).
//!
//! Domain staticlibs register implementations here; `libsyx_rt.a` only looks up slots
//! and must not name rusqlite/hyper/rustls symbols.

use std::sync::Mutex;

use syx_diagnostics::SyxResult;
use syx_stdlib::HostFnId;

use crate::Value;

/// Same ABI as `syx_vm::HostFn` — kept here so domain crates need not depend on `syx-vm`.
pub type ShakeableHostFn = fn(&mut Vec<Value>) -> SyxResult<()>;

static SHAKEABLE_HOSTS: Mutex<Vec<Option<ShakeableHostFn>>> = Mutex::new(Vec::new());

fn ensure_len(table: &mut Vec<Option<ShakeableHostFn>>, idx: usize) {
    if table.len() <= idx {
        table.resize(idx + 1, None);
    }
}

/// Register one host implementation (called from domain `*_install`).
pub fn register_shakeable_host(idx: u32, f: ShakeableHostFn) {
    let mut table = SHAKEABLE_HOSTS.lock().unwrap_or_else(|e| e.into_inner());
    ensure_len(&mut table, idx as usize);
    table[idx as usize] = Some(f);
}

pub fn register_shakeable_host_id(id: HostFnId, f: ShakeableHostFn) {
    register_shakeable_host(syx_stdlib::host_fn_index(id), f);
}

pub fn lookup_shakeable_host(idx: u32) -> Option<ShakeableHostFn> {
    let table = SHAKEABLE_HOSTS.lock().unwrap_or_else(|e| e.into_inner());
    table.get(idx as usize).copied().flatten()
}

pub fn lookup_shakeable_host_id(id: HostFnId) -> Option<ShakeableHostFn> {
    lookup_shakeable_host(syx_stdlib::host_fn_index(id))
}

pub fn reset_shakeable_hosts() {
    let mut table = SHAKEABLE_HOSTS.lock().unwrap_or_else(|e| e.into_inner());
    table.clear();
    *HTTP_ASYNC_POLL.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *WS_ASYNC_POLL.lock().unwrap_or_else(|e| e.into_inner()) = None;
}

type AsyncPollFn = fn(&Value) -> Option<SyxResult<Value>>;

static HTTP_ASYNC_POLL: Mutex<Option<AsyncPollFn>> = Mutex::new(None);
static WS_ASYNC_POLL: Mutex<Option<AsyncPollFn>> = Mutex::new(None);

/// VM await-poll for HTTP requests created by the HTTP shake domain.
pub fn register_http_async_poll(f: AsyncPollFn) {
    *HTTP_ASYNC_POLL.lock().unwrap_or_else(|e| e.into_inner()) = Some(f);
}

pub fn poll_registered_http_async(value: &Value) -> Option<SyxResult<Value>> {
    let poll = *HTTP_ASYNC_POLL.lock().unwrap_or_else(|e| e.into_inner());
    poll.and_then(|f| f(value))
}

/// VM await-poll for WebSocket connects/sends created by the HTTP shake domain.
pub fn register_ws_async_poll(f: AsyncPollFn) {
    *WS_ASYNC_POLL.lock().unwrap_or_else(|e| e.into_inner()) = Some(f);
}

pub fn poll_registered_ws_async(value: &Value) -> Option<SyxResult<Value>> {
    let poll = *WS_ASYNC_POLL.lock().unwrap_or_else(|e| e.into_inner());
    poll.and_then(|f| f(value))
}
