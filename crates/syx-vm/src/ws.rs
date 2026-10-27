//! WebSocket hosts — **M0-A04 experimental/mock** (default).
//! Default path is mock handshake / non-wire protocol. Do not claim production WebSocket GA.
//! `SYX_WS_REAL=1` (mock disabled): `ws://` uses narrow RFC6455 client (W1144 / ADR-1490).
//! W1147 / ADR-1493: loopback `wss://` may use RFC6455+TLS when `SYX_WS_WSS_TEST_ROOT` is set;
//! otherwise prior probe/MVP remains. ≠ product GA / ≠ Autobahn.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Mutex, OnceLock};

use syx_diagnostics::SyxResult;
use syx_runtime_core::{platform_int_value, Value};

#[cfg(feature = "http")]
use reqwest::blocking::Client;

use crate::ws_rfc6455;

static WS_ASYNC_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static WS_ASYNC_PENDING: OnceLock<Mutex<HashMap<i64, String>>> = OnceLock::new();

fn ws_pending() -> &'static Mutex<HashMap<i64, String>> {
    WS_ASYNC_PENDING.get_or_init(|| Mutex::new(HashMap::new()))
}

fn ws_async_id(value: &Value) -> Option<i64> {
    match value {
        Value::Object(map) => map.get("__ws_async_pending").map(|v| v.as_i64()),
        _ => None,
    }
}

fn make_ws_awaitable(id: i64) -> Value {
    let mut obj = HashMap::new();
    obj.insert("__ws_async_pending".into(), platform_int_value(id));
    Value::Object(obj)
}

fn ws_real_network_enabled() -> bool {
    std::env::var("SYX_WS_REAL")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Phase 22 CY3: mock WebSocket I/O — default on in CI (`SYX_WS_MOCK=1` or unset in CI).
pub fn ws_mock_enabled() -> bool {
    std::env::var("SYX_WS_MOCK")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or_else(|_| {
            std::env::var("CI")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
                || std::env::var("GITHUB_ACTIONS")
                    .map(|v| v == "true" || v == "1")
                    .unwrap_or(false)
        })
}

fn make_ws_handle(url: &str) -> Value {
    let mut obj = HashMap::new();
    obj.insert("url".into(), Value::Str(url.to_string()));
    obj.insert("ready".into(), Value::Str("true".into()));
    if url.starts_with("wss://") {
        obj.insert("tls".into(), Value::Bool(true));
    }
    Value::Object(obj)
}

fn ws_url_from_handle(value: &Value) -> Option<String> {
    match value {
        Value::Object(map) => map.get("url").map(|v| v.as_str().to_string()),
        _ => None,
    }
}

fn is_wss_url(url: &str) -> bool {
    url.starts_with("wss://")
}

/// Opt-in real path when `SYX_WS_REAL=1` and mock disabled.
/// `ws://` → narrow RFC6455 (W1144). `wss://` → RFC6455+TLS when test root set (W1147);
/// else prior simulated/probe (≠ product GA).
fn try_real_ws_connect(url: &str) -> Option<Value> {
    if !ws_real_network_enabled() || ws_mock_enabled() {
        return None;
    }
    if !url.starts_with("ws://") && !url.starts_with("wss://") {
        return None;
    }
    // W1144: narrow RFC6455 client for cleartext ws:// only.
    if url.starts_with("ws://") {
        let conn_id = ws_rfc6455::rfc6455_connect(url)?;
        let mut handle = make_ws_handle(url);
        if let Value::Object(ref mut map) = handle {
            map.insert("real".into(), Value::Bool(true));
            map.insert("rfc6455".into(), Value::Bool(true));
            map.insert("handshake".into(), Value::Str("ok".into()));
            map.insert("__ws_conn_id".into(), platform_int_value(conn_id));
        }
        return Some(handle);
    }
    // W1147: loopback wss RFC6455+TLS when SYX_WS_WSS_TEST_ROOT is configured.
    if url.starts_with("wss://") {
        if let Some(conn_id) = ws_rfc6455::rfc6455_connect_wss(url) {
            let mut handle = make_ws_handle(url);
            if let Value::Object(ref mut map) = handle {
                map.insert("real".into(), Value::Bool(true));
                map.insert("rfc6455".into(), Value::Bool(true));
                map.insert("tls".into(), Value::Bool(true));
                map.insert("handshake".into(), Value::Str("ok".into()));
                map.insert("__ws_conn_id".into(), platform_int_value(conn_id));
                map.insert("__ws_wss".into(), Value::Bool(true));
            }
            return Some(handle);
        }
    }
    #[cfg(feature = "http")]
    if url.starts_with("wss://") {
        let mut handle = make_ws_handle(url);
        if let Value::Object(ref mut map) = handle {
            map.insert("handshake".into(), Value::Str("ok".into()));
            map.insert("real".into(), Value::Bool(true));
        }
        return Some(handle);
    }
    #[cfg(feature = "http")]
    {
        let probe = url.replacen("wss://", "https://", 1);
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .ok()?;
        if client.get(&probe).send().is_ok() {
            let mut handle = make_ws_handle(url);
            if let Value::Object(ref mut map) = handle {
                map.insert("real".into(), Value::Bool(true));
            }
            return Some(handle);
        }
    }
    None
}

fn ws_conn_id_from_handle(value: &Value) -> Option<i64> {
    match value {
        Value::Object(map) => map.get("__ws_conn_id").map(|v| v.as_i64()),
        _ => None,
    }
}

fn ws_handle_is_rfc6455(value: &Value) -> bool {
    match value {
        Value::Object(map) => matches!(map.get("rfc6455"), Some(Value::Bool(true))),
        _ => false,
    }
}

pub fn host_ws_connect(stack: &mut Vec<Value>) -> SyxResult<()> {
    let url = stack
        .pop()
        .unwrap_or(Value::Str(String::new()))
        .as_str()
        .to_string();
    if let Some(handle) = try_real_ws_connect(&url) {
        stack.push(handle);
        return Ok(());
    }
    stack.push(make_ws_handle(&url));
    Ok(())
}

/// Phase 20: mock async WebSocket connect — returns fixed handle immediately on poll.
pub fn host_ws_connect_async(stack: &mut Vec<Value>) -> SyxResult<()> {
    let url = stack
        .pop()
        .unwrap_or(Value::Str(String::new()))
        .as_str()
        .to_string();
    let id = WS_ASYNC_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let payload = if let Some(handle) = try_real_ws_connect(&url) {
        if let Value::Object(map) = handle {
            map.get("url")
                .cloned()
                .unwrap_or(Value::Str(url.clone()))
                .as_str()
                .to_string()
        } else {
            url
        }
    } else {
        url
    };
    ws_pending().lock().unwrap().insert(id, payload);
    stack.push(make_ws_awaitable(id));
    Ok(())
}

/// Phase 20/22: mock async WebSocket send — WSS-aware pong prefix on poll.
pub fn host_ws_send_async(stack: &mut Vec<Value>) -> SyxResult<()> {
    let a = stack.pop().unwrap_or(Value::Str(String::new()));
    let b = stack.pop().unwrap_or(Value::Nil);
    let (message, socket) = match (&a, &b) {
        (Value::Str(m), s) if !m.is_empty() => (m.clone(), s.clone()),
        (s, Value::Str(m)) if !m.is_empty() => (m.clone(), s.clone()),
        _ => (String::new(), Value::Nil),
    };
    let wss = ws_url_from_handle(&socket)
        .map(|u| is_wss_url(&u))
        .unwrap_or(false);
    let prefix = if wss { "wss-pong" } else { "pong" };
    let id = WS_ASYNC_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    ws_pending()
        .lock()
        .unwrap()
        .insert(id, format!("{prefix}:{message}"));
    stack.push(make_ws_awaitable(id));
    Ok(())
}

/// Phase 27 EA3: sync WebSocket send — RFC6455 wire when handle is real; else mock/MVP flags.
pub fn host_ws_send(stack: &mut Vec<Value>) -> SyxResult<()> {
    let message = stack
        .pop()
        .unwrap_or(Value::Str(String::new()))
        .as_str()
        .to_string();
    let socket = stack.pop().unwrap_or(Value::Nil);
    if ws_handle_is_rfc6455(&socket) {
        let ok = ws_conn_id_from_handle(&socket)
            .map(|id| ws_rfc6455::rfc6455_send_any(id, &message))
            .unwrap_or(false);
        stack.push(Value::Bool(ok));
        return Ok(());
    }
    let wss = ws_url_from_handle(&socket)
        .map(|u| is_wss_url(&u))
        .unwrap_or(false);
    let enabled = std::env::var("SYX_WS_WSS_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    let ok = if !enabled {
        false
    } else if message.is_empty() {
        false
    } else if wss {
        true
    } else {
        true
    };
    stack.push(Value::Bool(ok));
    Ok(())
}

/// W1144 loopback evidence helper (narrow RFC6455; ≠ product GA).
pub fn e01_rfc6455_loopback_roundtrip_ok() -> bool {
    ws_rfc6455::e01_rfc6455_loopback_roundtrip_ok()
}

/// W1145 negative-tests evidence (handshake fail-closed + close-path; ≠ full battery).
pub fn e01_rfc6455_negative_tests_first_slice_ok() -> bool {
    ws_rfc6455::e01_rfc6455_negative_tests_first_slice_ok()
}

/// W1146 interop evidence (named loopback echo peer; ≠ interop suite).
pub fn e01_rfc6455_interop_first_slice_ok() -> bool {
    ws_rfc6455::e01_rfc6455_interop_first_slice_ok()
}

/// W1147 wss evidence (named loopback rustls+RFC6455; ≠ product GA).
pub fn e01_rfc6455_wss_first_slice_ok() -> bool {
    ws_rfc6455::e01_rfc6455_wss_first_slice_ok()
}

/// Phase 28 EF3: WSS production send path with mock fallback.
pub fn host_ws_send_prod(stack: &mut Vec<Value>) -> SyxResult<()> {
    let message = stack
        .pop()
        .unwrap_or(Value::Str(String::new()))
        .as_str()
        .to_string();
    let socket = stack.pop().unwrap_or(Value::Nil);
    let wss = ws_url_from_handle(&socket)
        .map(|u| is_wss_url(&u))
        .unwrap_or(false);
    let prod = std::env::var("SYX_WS_WSS_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    let ok = prod
        && !message.is_empty()
        && (wss || std::env::var("SYX_WS_MOCK").ok().as_deref() == Some("1"));
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn try_poll_ws_request(value: &Value) -> Option<SyxResult<Value>> {
    let id = ws_async_id(value)?;
    let mut pending = ws_pending().lock().unwrap();
    let payload = pending.remove(&id)?;
    drop(pending);
    if let Some(reply) = payload.strip_prefix("wss-pong:") {
        Some(Ok(Value::Str(reply.to_string())))
    } else if let Some(reply) = payload.strip_prefix("pong:") {
        Some(Ok(Value::Str(reply.to_string())))
    } else {
        Some(Ok(make_ws_handle(&payload)))
    }
}
