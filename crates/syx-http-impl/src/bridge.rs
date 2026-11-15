//! VM/native hooks so `syx-http-impl` does not depend on `syx-vm`.
//!
//! The interpreter installs a bytecode runner and program-context snapshot.
//! `syx-rt` registers a native MIR handler. WebSocket RFC6455 lives in the VM
//! and is optional: without those hooks, WS hosts stay on the mock path.

use std::sync::OnceLock;

use syx_diagnostics::{Span, SyxResult};
use syx_runtime_core::Value;

use crate::error::vm_error_at;

/// Opaque program context captured on the CallHost thread and restored on workers.
pub trait HttpCtxClone: Send {
    fn box_clone(&self) -> Box<dyn HttpCtxClone + Send>;
    fn install(&self);
}

pub struct HttpCtxToken {
    inner: Box<dyn HttpCtxClone + Send>,
}

impl Clone for HttpCtxToken {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.box_clone(),
        }
    }
}

impl HttpCtxToken {
    pub fn new(ctx: impl HttpCtxClone + 'static) -> Self {
        Self {
            inner: Box::new(ctx),
        }
    }

    pub fn install(&self) {
        self.inner.install();
    }
}

pub struct HttpVmBridge {
    pub capture_ctx: fn() -> Option<HttpCtxToken>,
    pub clear_ctx: fn(),
    pub spawn_blocking: fn(Box<dyn FnOnce() + Send + 'static>),
    /// `Some` when a bytecode program context handled the call (including errors).
    /// `None` falls through to the native MIR handler.
    pub run_bytecode: fn(&str, Value) -> Option<SyxResult<Value>>,
}

pub struct WsRfcHooks {
    pub connect_ws: fn(&str) -> Option<i64>,
    pub connect_wss: fn(&str) -> Option<i64>,
    pub send_any: fn(i64, &str) -> bool,
    pub recv: fn(i64) -> Option<String>,
    pub close: fn(i64),
}

static BRIDGE: OnceLock<HttpVmBridge> = OnceLock::new();
static WS_RFC: OnceLock<WsRfcHooks> = OnceLock::new();

pub type NativeHttpHandlerFn = fn(&str, &Value) -> SyxResult<Value>;

static NATIVE_HTTP_HANDLER: OnceLock<NativeHttpHandlerFn> = OnceLock::new();

pub fn register_native_http_handler(f: NativeHttpHandlerFn) {
    let _ = NATIVE_HTTP_HANDLER.set(f);
}

pub fn install_vm_bridge(bridge: HttpVmBridge) {
    let _ = BRIDGE.set(bridge);
}

pub fn install_ws_rfc(hooks: WsRfcHooks) {
    let _ = WS_RFC.set(hooks);
}

pub fn ws_rfc_is_installed() -> bool {
    WS_RFC.get().is_some()
}

pub(crate) fn capture_ctx() -> Option<HttpCtxToken> {
    BRIDGE.get().and_then(|b| (b.capture_ctx)())
}

pub(crate) fn clear_ctx() {
    if let Some(b) = BRIDGE.get() {
        (b.clear_ctx)();
    }
}

pub(crate) fn spawn_blocking(job: Box<dyn FnOnce() + Send + 'static>) {
    if let Some(b) = BRIDGE.get() {
        (b.spawn_blocking)(job);
    } else {
        std::thread::spawn(job);
    }
}

pub(crate) fn invoke_handler(name: &str, req: Value) -> SyxResult<Value> {
    if let Some(b) = BRIDGE.get() {
        if let Some(result) = (b.run_bytecode)(name, req.clone()) {
            return result;
        }
    }
    if let Some(handler) = NATIVE_HTTP_HANDLER.get() {
        return handler(name, &req);
    }
    Err(vm_error_at(
        "HTTP serve requires active program context",
        None,
        Span::DUMMY,
        "R012",
    ))
}

pub(crate) fn ws_rfc_connect_ws(url: &str) -> Option<i64> {
    WS_RFC.get().and_then(|h| (h.connect_ws)(url))
}

pub(crate) fn ws_rfc_connect_wss(url: &str) -> Option<i64> {
    WS_RFC.get().and_then(|h| (h.connect_wss)(url))
}

pub(crate) fn ws_rfc_send_any(id: i64, message: &str) -> bool {
    WS_RFC
        .get()
        .map(|h| (h.send_any)(id, message))
        .unwrap_or(false)
}

pub(crate) fn ws_rfc_recv(id: i64) -> Option<String> {
    WS_RFC.get().and_then(|h| (h.recv)(id))
}

pub(crate) fn ws_rfc_close(id: i64) {
    if let Some(h) = WS_RFC.get() {
        (h.close)(id);
    }
}
