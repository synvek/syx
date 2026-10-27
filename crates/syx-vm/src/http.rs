use std::collections::HashMap;
use std::convert::Infallible;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::pin::Pin;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::header::{HeaderName, HeaderValue};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use reqwest::blocking::RequestBuilder;
use reqwest::Method;
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_stdlib::MethodId;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, ReadBuf};
use tokio::net::TcpListener;
use tokio::sync::watch;

use crate::program_ctx::{clear_program_ctx, set_program_ctx, with_program_ctx, ProgramCtx};
use crate::{platform_int_value, Value};

/// Native/MIR backend: dispatch HTTP handler by name when no bytecode `ProgramCtx` is active.
pub type NativeHttpHandlerFn = fn(&str, &Value) -> SyxResult<Value>;

static NATIVE_HTTP_HANDLER: OnceLock<NativeHttpHandlerFn> = OnceLock::new();

pub fn register_native_http_handler(f: NativeHttpHandlerFn) {
    let _ = NATIVE_HTTP_HANDLER.set(f);
}

fn invoke_http_handler(handler_name: &str, req_obj: Value) -> SyxResult<Value> {
    if let Some(ctx) = with_program_ctx(|ctx| ctx.clone()) {
        let handler_fn = ctx
            .program
            .functions
            .iter()
            .find(|f| f.name == handler_name)
            .ok_or_else(|| {
                SyxDiagnostic::error(format!("Unknown handler function '{handler_name}'"))
            })?;
        let mut vm = crate::VmInstance::new();
        vm.runtime = ctx.runtime.clone();
        return vm.run_function_with_args(handler_fn, &ctx.program, &[req_obj]);
    }
    if let Some(handler) = NATIVE_HTTP_HANDLER.get() {
        return handler(handler_name, &req_obj);
    }
    Err(SyxDiagnostic::error("HTTP serve requires active program context").with_code("R012"))
}

/// Run the SYX/MIR handler off the serve/`CallHost` thread.
///
/// Hyper awaits (e.g. body collect) resume on the same thread that is mid
/// `Http.serve` CallHost; nesting MIR host calls there corrupts NativeRt stack
/// (empty `Http.response` bodies / missing MIR fns). A side thread keeps
/// ProgramCtx TLS when present and uses process-global native MIR ctx otherwise.
fn invoke_http_handler_isolated(
    handler_name: &str,
    req_obj: Value,
    ctx_override: Option<ProgramCtx>,
) -> SyxResult<Value> {
    let name = handler_name.to_string();
    // Prefer explicit ctx: multi-thread tokio may resume after await on a
    // different OS thread, so thread-local ProgramCtx alone is not enough.
    let saved_ctx: Option<ProgramCtx> =
        ctx_override.or_else(|| with_program_ctx(|ctx| ctx.clone()));
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        if let Some(ctx) = saved_ctx {
            set_program_ctx(ctx);
        }
        let result = invoke_http_handler(&name, req_obj);
        clear_program_ctx();
        let _ = tx.send(result);
    });
    rx.recv().unwrap_or_else(|_| {
        Err(SyxDiagnostic::error("HTTP handler thread disconnected").with_code("R012"))
    })
}

/// ADR-237 / M2-S4-05: optional diagnostic trust-anchor PEM **file path**.
/// Appends to the default rustls root store (≠ skip-verify / ≠ danger_accept_invalid_certs).
const EXTRA_ROOT_ENV: &str = "SYX_HTTP_EXTRA_ROOT_PEM";

fn http2_enabled() -> bool {
    std::env::var("SYX_HTTP2")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

fn extra_root_pem_path() -> Option<String> {
    std::env::var(EXTRA_ROOT_ENV).ok().filter(|p| !p.is_empty())
}

/// Apply CI/diagnostic extra root when `SYX_HTTP_EXTRA_ROOT_PEM` is set.
fn apply_extra_root(
    mut builder: reqwest::blocking::ClientBuilder,
) -> reqwest::blocking::ClientBuilder {
    let Some(path) = extra_root_pem_path() else {
        return builder;
    };
    match std::fs::read(&path) {
        Ok(pem) => match reqwest::Certificate::from_pem(&pem) {
            Ok(cert) => builder.add_root_certificate(cert),
            Err(_) => builder,
        },
        Err(_) => builder,
    }
}

fn base_http_client_builder() -> reqwest::blocking::ClientBuilder {
    let mut builder = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .gzip(true)
        .deflate(true);
    if http2_enabled() {
        builder = builder.http2_adaptive_window(true);
    }
    apply_extra_root(builder)
}

static HTTP_CLIENT: LazyLock<reqwest::blocking::Client> =
    LazyLock::new(|| base_http_client_builder().build().expect("HTTP client"));

fn http_client_for_request(timeout_ms: Option<i64>) -> SyxResult<reqwest::blocking::Client> {
    // When a diagnostic extra root is set, always build a fresh client so tests that
    // set the env after process start are not stuck with the LazyLock default client.
    if timeout_ms.is_some() || extra_root_pem_path().is_some() {
        let mut builder = reqwest::blocking::Client::builder()
            .gzip(true)
            .deflate(true);
        if http2_enabled() {
            builder = builder.http2_adaptive_window(true);
        }
        builder = apply_extra_root(builder);
        if let Some(ms) = timeout_ms {
            builder = builder.timeout(Duration::from_millis(ms.max(0) as u64));
        } else {
            builder = builder.timeout(Duration::from_secs(30));
        }
        builder
            .build()
            .map_err(|e| SyxDiagnostic::error(format!("HTTP client error: {e}")).with_code("R011"))
    } else {
        Ok(HTTP_CLIENT.clone())
    }
}

pub fn http_response(status: i32, body: String, headers: HashMap<String, Value>) -> Value {
    let mut obj = HashMap::new();
    obj.insert("status".into(), platform_int_value(status as i64));
    obj.insert("body".into(), Value::Str(body));
    obj.insert("headers".into(), Value::Object(headers));
    obj.insert("__http_response".into(), Value::Bool(true));
    Value::Object(obj)
}

pub fn is_http_response(v: &Value) -> bool {
    match v {
        Value::Object(map) => {
            map.get("__http_response").is_some_and(|b| b.as_bool())
                || map.contains_key("status") && map.contains_key("body")
        }
        _ => false,
    }
}

fn response_headers(resp: &reqwest::blocking::Response) -> HashMap<String, Value> {
    resp.headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                Value::Str(v.to_str().unwrap_or("").to_string()),
            )
        })
        .collect()
}

fn object_field_str(map: &HashMap<String, Value>, key: &str) -> Option<String> {
    map.get(key).map(|v| v.as_str())
}

fn object_field_i64(map: &HashMap<String, Value>, key: &str) -> Option<i64> {
    map.get(key).map(|v| v.as_i64())
}

fn apply_headers(req: RequestBuilder, headers: &Value) -> RequestBuilder {
    let (Value::Object(map) | Value::Dict(map)) = headers else {
        return req;
    };
    let mut builder = req;
    for (k, v) in map {
        builder = builder.header(k.as_str(), v.as_str());
    }
    builder
}

fn append_query(url: &str, query: &Value) -> SyxResult<String> {
    let (Value::Object(map) | Value::Dict(map)) = query else {
        return Ok(url.to_string());
    };
    if map.is_empty() {
        return Ok(url.to_string());
    }
    let pairs: Vec<String> = map
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                urlencoding_encode(k),
                urlencoding_encode(&v.as_str())
            )
        })
        .collect();
    let sep = if url.contains('?') { '&' } else { '?' };
    Ok(format!("{url}{sep}{}", pairs.join("&")))
}

fn urlencoding_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

fn body_for_request(body: &Value) -> (String, bool) {
    match body {
        Value::Object(map) | Value::Dict(map) => (
            serde_json::to_string(&value_to_json(&Value::Object(map.clone())))
                .unwrap_or_else(|_| "{}".into()),
            true,
        ),
        other => (other.as_str(), false),
    }
}

pub fn execute_http_request(url: &str, options: &Value) -> SyxResult<Value> {
    let opts = match options {
        Value::Object(m) | Value::Dict(m) => m,
        Value::Nil => &HashMap::new(),
        _ => {
            return Err(
                SyxDiagnostic::error("HTTP request options must be an object").with_code("R003"),
            );
        }
    };

    let method_str = object_field_str(opts, "method").unwrap_or_else(|| "GET".into());
    let method = Method::from_bytes(method_str.to_uppercase().as_bytes()).map_err(|_| {
        SyxDiagnostic::error(format!("Invalid HTTP method '{method_str}'")).with_code("R003")
    })?;

    let final_url = if let Some(query) = opts.get("query") {
        append_query(url, query)?
    } else {
        url.to_string()
    };

    let timeout_ms = object_field_i64(opts, "timeout_ms");
    // ADR-237: honor SYX_HTTP_EXTRA_ROOT_PEM on all client builds (incl. timeout_ms).
    let client = http_client_for_request(timeout_ms)?;

    let mut req = client.request(method.clone(), &final_url);
    if let Some(headers) = opts.get("headers") {
        req = apply_headers(req, headers);
    }

    if method != Method::GET && method != Method::HEAD {
        if let Some(body) = opts.get("body") {
            let (body_str, is_json) = body_for_request(body);
            if is_json {
                req = req.header("Content-Type", "application/json");
            }
            req = req.body(body_str);
        }
    }

    let response = match req.send() {
        Ok(resp) => resp,
        Err(e) => {
            // M0-B06: transport failure must not masquerade as status=0 success.
            // ADR-235 / M2-S4-02b: R012 only for true timeouts (not every error when timeout_ms set).
            // Wave A W94 / ADR-438: stabilize Display (no URL/port from reqwest `{e}`).
            if e.is_timeout() {
                return Err(
                    SyxDiagnostic::error("HTTP request timed out".to_string()).with_code("R012")
                );
            }
            // Wave A W95 / ADR-439: stabilize Display (no reqwest `{e}` / cert detail).
            return Err(SyxDiagnostic::error("HTTP request failed".to_string()).with_code("R003"));
        }
    };
    let status = response.status().as_u16() as i32;
    let headers = response_headers(&response);
    let bytes = response
        .bytes()
        .map_err(|e| SyxDiagnostic::error(format!("HTTP read failed: {e}")).with_code("R003"))?;
    let body_bytes = decompress_response_body(&bytes, &headers);
    let text = String::from_utf8_lossy(&body_bytes).into_owned();
    Ok(http_response(
        status,
        text,
        headers_for_decoded_body(headers),
    ))
}

/// ADR-239 / M2-S4-04: client response body stream session (≠ `response.bytes()` path).
struct HttpBodyStreamSession {
    response: reqwest::blocking::Response,
}

static HTTP_BODY_STREAM_SESSIONS: LazyLock<Mutex<HashMap<i64, HttpBodyStreamSession>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static HTTP_BODY_STREAM_NEXT_ID: AtomicI64 = AtomicI64::new(1);

fn send_http_request_keeping_response(
    url: &str,
    options: &Value,
) -> SyxResult<(reqwest::blocking::Response, i32, HashMap<String, Value>)> {
    let opts = match options {
        Value::Object(m) | Value::Dict(m) => m,
        Value::Nil => &HashMap::new(),
        _ => {
            return Err(
                SyxDiagnostic::error("HTTP request options must be an object").with_code("R003"),
            );
        }
    };

    let method_str = object_field_str(opts, "method").unwrap_or_else(|| "GET".into());
    let method = Method::from_bytes(method_str.to_uppercase().as_bytes()).map_err(|_| {
        SyxDiagnostic::error(format!("Invalid HTTP method '{method_str}'")).with_code("R003")
    })?;

    let final_url = if let Some(query) = opts.get("query") {
        append_query(url, query)?
    } else {
        url.to_string()
    };

    let timeout_ms = object_field_i64(opts, "timeout_ms");
    let client = http_client_for_request(timeout_ms)?;

    let mut req = client.request(method.clone(), &final_url);
    if let Some(headers) = opts.get("headers") {
        req = apply_headers(req, headers);
    }

    if method != Method::GET && method != Method::HEAD {
        if let Some(body) = opts.get("body") {
            let (body_str, is_json) = body_for_request(body);
            if is_json {
                req = req.header("Content-Type", "application/json");
            }
            req = req.body(body_str);
        }
    }

    let response = match req.send() {
        Ok(resp) => resp,
        Err(e) => {
            // Wave A W94 / ADR-438: stabilize Display (no URL/port from reqwest `{e}`).
            if e.is_timeout() {
                return Err(
                    SyxDiagnostic::error("HTTP request timed out".to_string()).with_code("R012")
                );
            }
            // Wave A W95 / ADR-439: stabilize Display (no reqwest `{e}` / cert detail).
            return Err(SyxDiagnostic::error("HTTP request failed".to_string()).with_code("R003"));
        }
    };
    let status = response.status().as_u16() as i32;
    let headers = response_headers(&response);
    Ok((response, status, headers))
}

fn push_body_stream_open_result(
    stack: &mut Vec<Value>,
    url: &str,
    options: &Value,
) -> SyxResult<()> {
    let (response, status, _headers) = send_http_request_keeping_response(url, options)?;
    let id = HTTP_BODY_STREAM_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    HTTP_BODY_STREAM_SESSIONS
        .lock()
        .expect("HTTP_BODY_STREAM_SESSIONS poisoned")
        .insert(id, HttpBodyStreamSession { response });
    let mut out = HashMap::new();
    out.insert("handle".into(), platform_int_value(id));
    out.insert("status".into(), platform_int_value(status as i64));
    stack.push(Value::Object(out));
    Ok(())
}

/// ADR-239: `HttpBodyStream.open(url) -> Object{handle,status}` (GET, empty options).
/// Keeps Response for incremental `Read` — does **not** call `response.bytes()`.
pub fn host_http_body_stream_open(stack: &mut Vec<Value>) -> SyxResult<()> {
    let url = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    push_body_stream_open_result(stack, &url, &Value::Nil)
}

/// ADR-239: `HttpBodyStream.openWithOptions(url, options) -> Object{handle,status}`
pub fn host_http_body_stream_open_opts(stack: &mut Vec<Value>) -> SyxResult<()> {
    let options = stack.pop().unwrap_or(Value::Nil);
    let url = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    push_body_stream_open_result(stack, &url, &options)
}

/// ADR-239: `HttpBodyStream.readChunk(handle, maxBytes) -> String` (empty = EOF).
pub fn host_http_body_stream_read_chunk(stack: &mut Vec<Value>) -> SyxResult<()> {
    let max_bytes = stack.pop().unwrap_or(Value::I32(0)).as_i64().max(0) as usize;
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    if max_bytes == 0 {
        stack.push(Value::Str(String::new()));
        return Ok(());
    }
    let mut sessions = HTTP_BODY_STREAM_SESSIONS
        .lock()
        .expect("HTTP_BODY_STREAM_SESSIONS poisoned");
    let Some(session) = sessions.get_mut(&handle) else {
        return Err(
            SyxDiagnostic::error(format!("HttpBodyStream invalid handle {handle}"))
                .with_code("R003"),
        );
    };
    let mut buf = vec![0u8; max_bytes];
    let n = std::io::Read::read(&mut session.response, &mut buf).map_err(|e| {
        SyxDiagnostic::error(format!("HTTP body stream read failed: {e}")).with_code("R003")
    })?;
    buf.truncate(n);
    stack.push(Value::Str(String::from_utf8_lossy(&buf).into_owned()));
    Ok(())
}

/// ADR-239 / Wave A W174: `HttpBodyStream.close(handle)` — miss → R003 (facade wraps R007).
pub fn host_http_body_stream_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let removed = HTTP_BODY_STREAM_SESSIONS
        .lock()
        .expect("HTTP_BODY_STREAM_SESSIONS poisoned")
        .remove(&handle)
        .is_some();
    if !removed {
        return Err(
            SyxDiagnostic::error(format!("HttpBodyStream invalid handle {handle}"))
                .with_code("R003"),
        );
    }
    stack.push(Value::Nil);
    Ok(())
}

/// M2 W110 / ADR-254: live `HTTP_BODY_STREAM_SESSIONS` count (test/ops only).
pub fn http_body_stream_live_count() -> usize {
    HTTP_BODY_STREAM_SESSIONS
        .lock()
        .expect("HTTP_BODY_STREAM_SESSIONS poisoned")
        .len()
}

/// Test cleanup: drop a live HttpBodyStream session by id.
pub fn http_body_stream_force_close(id: i64) -> bool {
    HTTP_BODY_STREAM_SESSIONS
        .lock()
        .expect("HTTP_BODY_STREAM_SESSIONS poisoned")
        .remove(&id)
        .is_some()
}

fn decompress_response_body(bytes: &[u8], headers: &HashMap<String, Value>) -> Vec<u8> {
    let encoding = headers
        .get("content-encoding")
        .or_else(|| headers.get("Content-Encoding"))
        .map(|v| v.as_str().to_ascii_lowercase())
        .unwrap_or_default();
    if encoding.contains("zstd") {
        return zstd::stream::decode_all(bytes).unwrap_or_else(|_| bytes.to_vec());
    }
    if encoding.contains("gzip") {
        use flate2::read::GzDecoder;
        use std::io::Read;
        let mut decoder = GzDecoder::new(bytes);
        let mut out = Vec::new();
        if decoder.read_to_end(&mut out).is_ok() {
            return out;
        }
    }
    bytes.to_vec()
}

/// Drop hop-by-hop / framing headers after the body is materialized as plain text.
/// Needed when reverse-proxying `HTTPResponse` as-is (Content-Encoding: gzip +
/// already-decoded body would make browsers gunzip twice).
fn headers_for_decoded_body(headers: HashMap<String, Value>) -> HashMap<String, Value> {
    headers
        .into_iter()
        .filter(|(k, _)| {
            let lower = k.to_ascii_lowercase();
            !matches!(
                lower.as_str(),
                "content-encoding"
                    | "content-length"
                    | "transfer-encoding"
                    | "connection"
                    | "keep-alive"
                    | "proxy-connection"
                    | "trailer"
                    | "upgrade"
            )
        })
        .collect()
}

fn response_body(v: &Value) -> String {
    match v {
        Value::Object(map) => map.get("body").map(|b| b.as_str()).unwrap_or_default(),
        Value::Str(s) => s.clone(),
        _ => String::new(),
    }
}

fn response_status(v: &Value) -> i32 {
    match v {
        Value::Object(map) => map.get("status").map(|s| s.as_i64() as i32).unwrap_or(0),
        _ => 0,
    }
}

fn response_headers_map(v: &Value) -> HashMap<String, Value> {
    match v {
        Value::Object(map) => map
            .get("headers")
            .and_then(|h| match h {
                Value::Object(m) => Some(m.clone()),
                _ => None,
            })
            .unwrap_or_default(),
        _ => HashMap::new(),
    }
}

pub fn parse_response_json(v: &Value) -> SyxResult<Value> {
    let body = response_body(v);
    let parsed: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| SyxDiagnostic::error(format!("JSON parse failed: {e}")).with_code("R005"))?;
    Ok(json_to_value(&parsed))
}

pub fn call_http_response_method(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    match method {
        MethodId::HttpResponseJson => parse_response_json(receiver).unwrap_or(Value::Nil),
        MethodId::HttpResponseStatus => platform_int_value(response_status(receiver) as i64),
        MethodId::HttpResponseText | MethodId::HttpResponseBody => {
            Value::Str(response_body(receiver))
        }
        MethodId::HttpResponseOk => {
            let s = response_status(receiver);
            Value::Bool((200..300).contains(&s))
        }
        MethodId::HttpResponseHeader => {
            let name = args.first().map(|v| v.as_str()).unwrap_or_default();
            let headers = response_headers_map(receiver);
            headers
                .get(&name)
                .or_else(|| {
                    headers
                        .iter()
                        .find(|(k, _)| k.eq_ignore_ascii_case(&name))
                        .map(|(_, v)| v)
                })
                .cloned()
                .unwrap_or(Value::Nil)
        }
        MethodId::HttpResponseHeaders => Value::Object(response_headers_map(receiver)),
        _ => Value::Nil,
    }
}

pub fn host_http_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let url = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    stack.push(execute_http_request(&url, &Value::Nil)?);
    Ok(())
}

pub fn host_http_post(stack: &mut Vec<Value>) -> SyxResult<()> {
    let body_val = stack.pop().unwrap_or(Value::Str(String::new()));
    let url = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let mut opts = HashMap::new();
    opts.insert("method".into(), Value::Str("POST".into()));
    opts.insert("body".into(), body_val);
    stack.push(execute_http_request(&url, &Value::Object(opts))?);
    Ok(())
}

pub fn host_http_request(stack: &mut Vec<Value>) -> SyxResult<()> {
    let options = stack.pop().unwrap_or(Value::Nil);
    let url = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    stack.push(execute_http_request(&url, &options)?);
    Ok(())
}

static HTTP_ASYNC_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static HTTP_ASYNC_PENDING: LazyLock<
    Mutex<HashMap<i64, std::sync::mpsc::Receiver<SyxResult<Value>>>>,
> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn http_async_pending_id(value: &Value) -> Option<i64> {
    match value {
        Value::Object(map) => map.get("__http_async_pending").map(|v| v.as_i64()),
        _ => None,
    }
}

pub fn make_http_request_awaitable(id: i64) -> Value {
    let mut obj = HashMap::new();
    obj.insert("__http_async_pending".into(), platform_int_value(id));
    Value::Object(obj)
}

/// Phase 18 / ADR-278: non-blocking HTTP — request runs on blocking_pool; await polls the result.
pub fn host_http_request_async(stack: &mut Vec<Value>) -> SyxResult<()> {
    let options = stack.pop().unwrap_or(Value::Nil);
    let url = stack
        .pop()
        .unwrap_or(Value::Str(String::new()))
        .as_str()
        .to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    let opts = options.clone();
    crate::blocking_execute::execute_on_blocking_pool(Box::new(move || {
        let _ = tx.send(execute_http_request(&url, &opts));
    }));
    let id = HTTP_ASYNC_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    HTTP_ASYNC_PENDING.lock().unwrap().insert(id, rx);
    stack.push(make_http_request_awaitable(id));
    Ok(())
}

pub fn try_poll_http_request(value: &Value) -> Option<SyxResult<Value>> {
    let id = http_async_pending_id(value)?;
    let rx = HTTP_ASYNC_PENDING.lock().unwrap().remove(&id)?;
    match rx.try_recv() {
        Ok(result) => Some(result),
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            HTTP_ASYNC_PENDING.lock().unwrap().insert(id, rx);
            None
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Err(SyxDiagnostic::error(
            "HTTP async request worker dropped",
        )
        .with_code("R003"))),
    }
}

pub fn host_http_response_json(stack: &mut Vec<Value>) -> SyxResult<()> {
    let response = stack.pop().unwrap_or(Value::Nil);
    stack.push(parse_response_json(&response)?);
    Ok(())
}

pub fn host_http_response_status(stack: &mut Vec<Value>) -> SyxResult<()> {
    let response = stack.pop().unwrap_or(Value::Nil);
    stack.push(platform_int_value(response_status(&response) as i64));
    Ok(())
}

pub fn host_http_response_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    let response = stack.pop().unwrap_or(Value::Nil);
    stack.push(Value::Str(response_body(&response)));
    Ok(())
}

pub fn host_http_response_ok(stack: &mut Vec<Value>) -> SyxResult<()> {
    let response = stack.pop().unwrap_or(Value::Nil);
    let s = response_status(&response);
    stack.push(Value::Bool((200..300).contains(&s)));
    Ok(())
}

pub fn host_http_response_header(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let response = stack.pop().unwrap_or(Value::Nil);
    let val =
        call_http_response_method(MethodId::HttpResponseHeader, &response, &[Value::Str(name)]);
    stack.push(val);
    Ok(())
}

pub fn host_http_response_headers(stack: &mut Vec<Value>) -> SyxResult<()> {
    let response = stack.pop().unwrap_or(Value::Nil);
    stack.push(Value::Object(response_headers_map(&response)));
    Ok(())
}

pub fn http_make_response(status: i32, headers: HashMap<String, Value>, body: String) -> Value {
    http_response(status, body, headers)
}

pub fn host_http_make_response(stack: &mut Vec<Value>) -> SyxResult<()> {
    let top = stack.pop().unwrap_or(Value::Nil);
    let bottom = stack.pop().unwrap_or(Value::Nil);
    let (status, body_val, headers) = match (&top, &bottom) {
        (
            Value::Object(body_map) | Value::Dict(body_map),
            Value::Object(opts) | Value::Dict(opts),
        ) => {
            let status = object_field_i64(opts, "status").unwrap_or(200) as i32;
            let headers = opts
                .get("headers")
                .map(|h| match h {
                    Value::Object(m) | Value::Dict(m) => m.clone(),
                    _ => HashMap::new(),
                })
                .unwrap_or_default();
            let body = body_to_response_string(body_map);
            (status, body, headers)
        }
        (body, Value::Object(opts) | Value::Dict(opts)) => {
            let status = object_field_i64(opts, "status").unwrap_or(200) as i32;
            let body = value_to_response_string(body);
            let headers = opts
                .get("headers")
                .map(|h| match h {
                    Value::Object(m) | Value::Dict(m) => m.clone(),
                    _ => HashMap::new(),
                })
                .unwrap_or_default();
            (status, body, headers)
        }
        (body, status_val) => {
            let status = status_val.as_i64() as i32;
            let body = value_to_response_string(body);
            (status, body, HashMap::new())
        }
    };
    stack.push(http_make_response(status, headers, body_val));
    Ok(())
}

fn body_to_response_string(map: &HashMap<String, Value>) -> String {
    serde_json::to_string(&value_to_json(&Value::Object(map.clone())))
        .unwrap_or_else(|_| "{}".into())
}

fn value_to_response_string(v: &Value) -> String {
    match v {
        Value::Str(s) => s.clone(),
        Value::Object(map) | Value::Dict(map) => body_to_response_string(map),
        other => other.as_str(),
    }
}

pub fn http_request_object(
    method: &str,
    path: &str,
    query: &str,
    headers: HashMap<String, Value>,
    body: String,
) -> Value {
    let mut obj = HashMap::new();
    obj.insert("method".into(), Value::Str(method.to_string()));
    obj.insert("path".into(), Value::Str(path.to_string()));
    obj.insert("query".into(), Value::Str(query.to_string()));
    obj.insert("headers".into(), Value::Object(headers));
    obj.insert("body".into(), Value::Str(body));
    Value::Object(obj)
}

type RespBody = Full<Bytes>;

/// ADR-238 / M2-S4-03b / W1176: active `Http.serve` cancel for `Http.shutdown`
/// (watch sender; ≠ tiny_http `Server::unblock`).
static ACTIVE_HTTP_SHUTDOWN: Mutex<Option<watch::Sender<bool>>> = Mutex::new(None);

fn register_active_http_shutdown(tx: watch::Sender<bool>) {
    if let Ok(mut slot) = ACTIVE_HTTP_SHUTDOWN.lock() {
        *slot = Some(tx);
    }
}

fn clear_active_http_shutdown() {
    if let Ok(mut slot) = ACTIVE_HTTP_SHUTDOWN.lock() {
        *slot = None;
    }
}

/// ADR-238: cancel the active serve accept loop (no-op if none). Completes the in-flight request first.
pub fn host_http_serve_shutdown(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    if let Ok(slot) = ACTIVE_HTTP_SHUTDOWN.lock() {
        if let Some(tx) = slot.as_ref() {
            let _ = tx.send(true);
        }
    }
    Ok(())
}

pub fn host_http_serve(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handler = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let port = stack.pop().unwrap_or(Value::I32(8080)).as_i32() as u16;
    let addr = stack
        .pop()
        .unwrap_or(Value::Str("127.0.0.1".into()))
        .as_str();

    let handler_name = handler.clone();

    // M2 W33 / ADR-177: test/ops override — exit after N successful responses (unset = block forever).
    // ≠ ADR-238 graceful shutdown (`Http.shutdown` / watch cancel).
    let max_requests = std::env::var("SYX_HTTP_SERVE_MAX_REQUESTS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|&n| n > 0);

    // Multi-thread: connection tasks must not share one thread with blocking
    // handler isolate recv (W1178 overlap; ≠ current_thread serialization).
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .map_err(|e| {
            SyxDiagnostic::error(format!("HTTP serve runtime failed: {e}")).with_code("R012")
        })?;
    rt.block_on(http_serve_loop(addr, port, handler_name, max_requests))
}

/// Prepend already-read bytes so hyper can parse a connection after header peek.
struct PrefixedStream<S> {
    prefix: std::io::Cursor<Vec<u8>>,
    inner: S,
}

impl<S: AsyncRead + Unpin> AsyncRead for PrefixedStream<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if self.prefix.position() < self.prefix.get_ref().len() as u64 {
            let pos = self.prefix.position() as usize;
            let data = &self.prefix.get_ref()[pos..];
            let n = data.len().min(buf.remaining());
            buf.put_slice(&data[..n]);
            self.prefix.set_position((pos + n) as u64);
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for PrefixedStream<S> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

/// W1179 / ADR-1525: diagnostic PEM paths for in-process HTTPS terminate.
const SERVE_TLS_CERT_ENV: &str = "SYX_HTTP_SERVE_TLS_CERT";
const SERVE_TLS_KEY_ENV: &str = "SYX_HTTP_SERVE_TLS_KEY";

#[cfg(not(target_arch = "wasm32"))]
fn load_http_serve_tls_acceptor() -> Option<tokio_rustls::TlsAcceptor> {
    use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
    use rustls::ServerConfig;

    let cert_path = std::env::var(SERVE_TLS_CERT_ENV).ok().filter(|s| !s.is_empty())?;
    let key_path = std::env::var(SERVE_TLS_KEY_ENV).ok().filter(|s| !s.is_empty())?;
    let certs = CertificateDer::pem_file_iter(std::path::Path::new(&cert_path))
        .ok()?
        .collect::<Result<Vec<_>, _>>()
        .ok()
        .filter(|c| !c.is_empty())?;
    let key = PrivateKeyDer::from_pem_file(std::path::Path::new(&key_path)).ok()?;
    let mut cfg = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .ok()?;
    // hyper HTTP/1 only this window (≠ h2 / ≠ WSS claim).
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    Some(tokio_rustls::TlsAcceptor::from(Arc::new(cfg)))
}

#[cfg(target_arch = "wasm32")]
fn load_http_serve_tls_acceptor() -> Option<()> {
    None
}

async fn http_serve_loop(
    addr: String,
    port: u16,
    handler_name: String,
    max_requests: Option<u64>,
) -> SyxResult<()> {
    let listener = TcpListener::bind(format!("{addr}:{port}"))
        .await
        .map_err(|e| {
            SyxDiagnostic::error(format!("HTTP serve bind failed: {e}")).with_code("R012")
        })?;
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
    register_active_http_shutdown(shutdown_tx);
    let served = Arc::new(AtomicU64::new(0));
    // Capture ProgramCtx on the serve/CallHost thread; worker tasks have empty TLS.
    let program_ctx = with_program_ctx(|ctx| ctx.clone());
    // W1179: optional in-process HTTPS when cert/key env set (≠ WSS claim).
    #[cfg(not(target_arch = "wasm32"))]
    let tls_acceptor = load_http_serve_tls_acceptor();
    // W1178: overlap connections via JoinSet; accept loop no longer awaits each conn.
    let mut tasks = tokio::task::JoinSet::new();
    // Count only after a real request head is parsed (probe connects must not burn MAX_REQUESTS).
    let started = Arc::new(AtomicU64::new(0));

    loop {
        if *shutdown_rx.borrow() {
            break;
        }
        if let Some(max) = max_requests {
            if served.load(Ordering::Relaxed) >= max {
                break;
            }
            // Test hook: do not accept more than N real requests; wait for in-flight.
            if started.load(Ordering::Relaxed) >= max {
                match tasks.join_next().await {
                    Some(_) => continue,
                    None => break,
                }
            }
        }

        let (stream, _) = tokio::select! {
            _ = shutdown_rx.wait_for(|v| *v) => break,
            // Reap finished tasks so JoinSet does not grow unboundedly.
            Some(_) = tasks.join_next(), if !tasks.is_empty() => continue,
            accepted = listener.accept() => {
                match accepted {
                    Ok(v) => v,
                    Err(_) => continue,
                }
            }
        };

        let handler = handler_name.clone();
        let served_c = Arc::clone(&served);
        let started_c = Arc::clone(&started);
        let ctx = program_ctx.clone();

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(ref acceptor) = tls_acceptor {
            let acceptor = acceptor.clone();
            tasks.spawn(async move {
                if let Some(ref ctx) = ctx {
                    set_program_ctx(ctx.clone());
                }
                let Ok(mut tls_stream) = acceptor.accept(stream).await else {
                    clear_program_ctx();
                    return;
                };
                let head_buf = match read_http_head_async(&mut tls_stream).await {
                    Ok(b) => b,
                    Err(_) => {
                        clear_program_ctx();
                        return;
                    }
                };
                let (method, path_and_query, header_map, is_upgrade) =
                    match parse_http_request_head(&head_buf) {
                        Ok(v) => v,
                        Err(_) => {
                            clear_program_ctx();
                            return;
                        }
                    };
                started_c.fetch_add(1, Ordering::Relaxed);
                // W1181: Upgrade-over-TLS — same upgradeUpstream byte tunnel as plain
                // (async copy_bidirectional; ≠ WSS frame parse / ≠ product GA).
                if is_upgrade {
                    let leftover = find_header_end(&head_buf)
                        .map(|pos| head_buf[pos + 4..].to_vec())
                        .unwrap_or_default();
                    handle_upgrade_connection_tls(
                        tls_stream,
                        &handler,
                        &method,
                        &path_and_query,
                        header_map,
                        leftover,
                        ctx.clone(),
                    )
                    .await;
                    served_c.fetch_add(1, Ordering::Relaxed);
                    clear_program_ctx();
                    return;
                }
                let prefixed = PrefixedStream {
                    prefix: std::io::Cursor::new(head_buf),
                    inner: tls_stream,
                };
                let io = TokioIo::new(prefixed);
                let service = service_fn(move |req| {
                    let handler = handler.clone();
                    let served_c = Arc::clone(&served_c);
                    let ctx = ctx.clone();
                    async move {
                        let resp = handle_hyper_request(req, &handler, ctx).await;
                        served_c.fetch_add(1, Ordering::Relaxed);
                        resp
                    }
                });
                let _ = http1::Builder::new().serve_connection(io, service).await;
                clear_program_ctx();
            });
            continue;
        }

        // Plain TCP: peek on accept task so probe/EOF connects do not burn MAX_REQUESTS.
        let mut stream = stream;
        let head_buf = match read_http_head_async(&mut stream).await {
            Ok(b) => b,
            Err(_) => continue,
        };
        let (method, path_and_query, header_map, is_upgrade) =
            match parse_http_request_head(&head_buf) {
                Ok(v) => v,
                Err(_) => continue,
            };
        started.fetch_add(1, Ordering::Relaxed);

        if is_upgrade {
            tasks.spawn_blocking(move || {
                if let Some(ctx) = ctx {
                    set_program_ctx(ctx);
                }
                handle_upgrade_connection(
                    stream,
                    &handler,
                    &method,
                    &path_and_query,
                    header_map,
                );
                served_c.fetch_add(1, Ordering::Relaxed);
                clear_program_ctx();
            });
            continue;
        }

        tasks.spawn(async move {
            if let Some(ref ctx) = ctx {
                set_program_ctx(ctx.clone());
            }
            let prefixed = PrefixedStream {
                prefix: std::io::Cursor::new(head_buf),
                inner: stream,
            };
            let io = TokioIo::new(prefixed);
            let service = service_fn(move |req| {
                let handler = handler.clone();
                let served_c = Arc::clone(&served_c);
                let ctx = ctx.clone();
                async move {
                    let resp = handle_hyper_request(req, &handler, ctx).await;
                    served_c.fetch_add(1, Ordering::Relaxed);
                    resp
                }
            });
            let _ = http1::Builder::new().serve_connection(io, service).await;
            clear_program_ctx();
        });
    }

    // ADR-238 / W1178: drain in-flight connections before clearing shutdown slot.
    while tasks.join_next().await.is_some() {}

    clear_active_http_shutdown();
    Ok(())
}

async fn read_http_head_async<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Vec<u8>, ()> {
    let mut buf = Vec::with_capacity(1024);
    let mut tmp = [0u8; 512];
    loop {
        let n = stream.read(&mut tmp).await.map_err(|_| ())?;
        if n == 0 {
            return Err(());
        }
        buf.extend_from_slice(&tmp[..n]);
        if find_header_end(&buf).is_some() {
            return Ok(buf);
        }
        if buf.len() > 64 * 1024 {
            return Err(());
        }
    }
}

fn parse_http_request_head(
    buf: &[u8],
) -> Result<(String, String, HashMap<String, Value>, bool), ()> {
    let end = find_header_end(buf).ok_or(())?;
    let head = std::str::from_utf8(&buf[..end]).map_err(|_| ())?;
    let mut lines = head.split("\r\n");
    let request_line = lines.next().ok_or(())?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_string();
    let path_and_query = parts.next().unwrap_or("/").to_string();
    let mut header_map = HashMap::new();
    let mut is_upgrade = false;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let Some((k, v)) = line.split_once(':') else {
            continue;
        };
        let name = k.trim().to_string();
        let value = v.trim().to_string();
        if name.eq_ignore_ascii_case("Upgrade")
            || (name.eq_ignore_ascii_case("Connection")
                && value.to_ascii_lowercase().contains("upgrade"))
        {
            is_upgrade = true;
        }
        header_map.insert(name, Value::Str(value));
    }
    Ok((method, path_and_query, header_map, is_upgrade))
}

fn handle_upgrade_connection(
    stream: tokio::net::TcpStream,
    handler_name: &str,
    method: &str,
    path_and_query: &str,
    header_map: HashMap<String, Value>,
) {
    let path_only = path_and_query
        .split('?')
        .next()
        .unwrap_or("/")
        .to_string();
    let query = path_and_query
        .split_once('?')
        .map(|(_, q)| q.to_string())
        .unwrap_or_default();
    let header_map_for_proxy = header_map.clone();
    let req_obj = http_request_object(method, &path_only, &query, header_map, String::new());

    let Ok(std_stream) = stream.into_std() else {
        return;
    };
    let _ = std_stream.set_nonblocking(false);

    let resp_val = match invoke_http_handler_isolated(handler_name, req_obj, None) {
        Ok(v) => v,
        Err(err) => {
            let _ = write_plain_http_response(
                &std_stream,
                502,
                &format!("Bad Gateway: {err}"),
            );
            return;
        }
    };
    // W1182: in-process terminate before upgradeUpstream proxy.
    if extract_upgrade_terminate(&resp_val) {
        handle_upgrade_terminate_std(&std_stream, &header_map_for_proxy);
        return;
    }
    let Some(upstream_url) = extract_upgrade_upstream(&resp_val) else {
        let _ = write_plain_http_response(
            &std_stream,
            502,
            "Bad Gateway: Upgrade handler must return upgradeUpstream",
        );
        return;
    };
    match prepare_upgrade_upstream(method, path_and_query, &header_map_for_proxy, &upstream_url)
    {
        Err(err) => {
            let _ = write_plain_http_response(
                &std_stream,
                502,
                &format!("Bad Gateway: {err}"),
            );
        }
        Ok(UpgradePrep::UpstreamFail {
            status,
            headers,
            body,
        }) => {
            let _ = write_http_response_full(&std_stream, status.max(400), &headers, &body);
        }
        Ok(UpgradePrep::Ready {
            upstream,
            resp_headers,
            preamble,
        }) => {
            if write_101_switching(&std_stream, &resp_headers).is_err() {
                return;
            }
            let mut client = std_stream;
            if !preamble.is_empty() {
                let _ = client.write_all(&preamble);
                let _ = client.flush();
            }
            spawn_bidirectional_std_proxy(client, upstream);
        }
    }
}

/// W1182–W1188: serve-computed Accept + N=3 frag-text/binary → ping/pong → close (≠ WSS product GA).
fn handle_upgrade_terminate_std(stream: &TcpStream, headers: &HashMap<String, Value>) {
    let Some(client_key) = header_value_ci(headers, "Sec-WebSocket-Key") else {
        let _ = write_plain_http_response(
            stream,
            400,
            "Bad Request: Sec-WebSocket-Key required for upgradeTerminate",
        );
        return;
    };
    if client_key.is_empty() {
        let _ = write_plain_http_response(
            stream,
            400,
            "Bad Request: empty Sec-WebSocket-Key",
        );
        return;
    }
    let accept = crate::ws_rfc6455::accept_key(&client_key);
    let resp_headers = vec![
        ("Upgrade".into(), "websocket".into()),
        ("Connection".into(), "Upgrade".into()),
        ("Sec-WebSocket-Accept".into(), accept),
    ];
    if write_101_switching(stream, &resp_headers).is_err() {
        return;
    }
    // `&TcpStream` implements Read+Write — N=3 frag-text → N=3 frag-binary → ping/pong → close.
    let mut client = stream;
    let _ = crate::ws_rfc6455::echo_fragmented_text_fragmented_binary_ping_then_close_exchange(
        &mut client,
    );
}

/// W1181: Upgrade-over-TLS — handler + upstream dial on blocking pool; 101 +
/// byte tunnel via `copy_bidirectional` on the already-terminated rustls stream.
/// `client_leftover` is bytes after `\r\n\r\n` already read with the request head.
#[cfg(not(target_arch = "wasm32"))]
async fn handle_upgrade_connection_tls(
    mut tls_stream: tokio_rustls::server::TlsStream<tokio::net::TcpStream>,
    handler_name: &str,
    method: &str,
    path_and_query: &str,
    header_map: HashMap<String, Value>,
    client_leftover: Vec<u8>,
    ctx: Option<ProgramCtx>,
) {
    use tokio::io::AsyncWriteExt;

    let path_only = path_and_query
        .split('?')
        .next()
        .unwrap_or("/")
        .to_string();
    let query = path_and_query
        .split_once('?')
        .map(|(_, q)| q.to_string())
        .unwrap_or_default();
    let header_map_for_proxy = header_map.clone();
    let req_obj = http_request_object(method, &path_only, &query, header_map, String::new());
    let handler = handler_name.to_string();
    let method_owned = method.to_string();
    let pq = path_and_query.to_string();

    let prep = match tokio::task::spawn_blocking(move || {
        if let Some(ctx) = ctx {
            set_program_ctx(ctx);
        }
        let resp_val = match invoke_http_handler_isolated(&handler, req_obj, None) {
            Ok(v) => v,
            Err(err) => {
                clear_program_ctx();
                return TlsUpgradePrep::PlainErr(502u16, format!("Bad Gateway: {err}"));
            }
        };
        // W1182: terminate before upstream proxy.
        if extract_upgrade_terminate(&resp_val) {
            let accept = match header_value_ci(&header_map_for_proxy, "Sec-WebSocket-Key") {
                Some(k) if !k.is_empty() => crate::ws_rfc6455::accept_key(&k),
                _ => {
                    clear_program_ctx();
                    return TlsUpgradePrep::PlainErr(
                        400u16,
                        "Bad Request: Sec-WebSocket-Key required for upgradeTerminate".into(),
                    );
                }
            };
            clear_program_ctx();
            return TlsUpgradePrep::Terminate { accept };
        }
        let Some(upstream_url) = extract_upgrade_upstream(&resp_val) else {
            clear_program_ctx();
            return TlsUpgradePrep::PlainErr(
                502u16,
                "Bad Gateway: Upgrade handler must return upgradeUpstream".into(),
            );
        };
        let result =
            prepare_upgrade_upstream(&method_owned, &pq, &header_map_for_proxy, &upstream_url);
        clear_program_ctx();
        match result {
            Err(err) => TlsUpgradePrep::PlainErr(502u16, format!("Bad Gateway: {err}")),
            Ok(UpgradePrep::UpstreamFail {
                status,
                headers,
                body,
            }) => TlsUpgradePrep::UpstreamFail {
                status: status.max(400),
                headers,
                body,
            },
            Ok(UpgradePrep::Ready {
                upstream,
                resp_headers,
                preamble,
            }) => TlsUpgradePrep::Ready {
                upstream,
                resp_headers,
                preamble,
            },
        }
    })
    .await
    {
        Ok(inner) => inner,
        Err(_) => {
            let _ = write_plain_http_response_async(
                &mut tls_stream,
                502,
                "Bad Gateway: upgrade worker join failed",
            )
            .await;
            return;
        }
    };

    match prep {
        TlsUpgradePrep::PlainErr(status, body) => {
            let _ = write_plain_http_response_async(&mut tls_stream, status, &body).await;
        }
        TlsUpgradePrep::UpstreamFail {
            status,
            headers,
            body,
        } => {
            let _ = write_http_response_full_async(&mut tls_stream, status, &headers, &body).await;
        }
        TlsUpgradePrep::Terminate { accept } => {
            let resp_headers = vec![
                ("Upgrade".into(), "websocket".into()),
                ("Connection".into(), "Upgrade".into()),
                ("Sec-WebSocket-Accept".into(), accept),
            ];
            if write_101_switching_async(&mut tls_stream, &resp_headers)
                .await
                .is_err()
            {
                return;
            }
            // Sync echo on rustls StreamOwned after async 101.
            let (tcp, server_conn) = tls_stream.into_inner();
            let Ok(std_tcp) = tcp.into_std() else {
                return;
            };
            let _ = std_tcp.set_nonblocking(false);
            let mut owned = rustls::StreamOwned::new(server_conn, std_tcp);
            let _ = client_leftover; // client waits for 101 before frames
            let _ = crate::ws_rfc6455::echo_fragmented_text_fragmented_binary_ping_then_close_exchange(
                &mut owned,
            );
        }
        TlsUpgradePrep::Ready {
            upstream,
            resp_headers,
            preamble,
        } => {
            if write_101_switching_async(&mut tls_stream, &resp_headers)
                .await
                .is_err()
            {
                return;
            }
            if !preamble.is_empty() {
                let _ = tls_stream.write_all(&preamble).await;
                let _ = tls_stream.flush().await;
            }
            let _ = upstream.set_nonblocking(true);
            let Ok(mut up) = tokio::net::TcpStream::from_std(upstream) else {
                return;
            };
            if !client_leftover.is_empty() {
                let _ = tokio::io::AsyncWriteExt::write_all(&mut up, &client_leftover).await;
            }
            let _ = tokio::io::copy_bidirectional(&mut tls_stream, &mut up).await;
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
enum TlsUpgradePrep {
    Terminate { accept: String },
    Ready {
        upstream: TcpStream,
        resp_headers: Vec<(String, String)>,
        preamble: Vec<u8>,
    },
    UpstreamFail {
        status: u16,
        headers: HashMap<String, String>,
        body: String,
    },
    PlainErr(u16, String),
}

#[cfg(not(target_arch = "wasm32"))]
async fn write_plain_http_response_async<W: tokio::io::AsyncWrite + Unpin>(
    stream: &mut W,
    status: u16,
    body: &str,
) -> std::io::Result<()> {
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".into(),
        "text/plain; charset=utf-8".into(),
    );
    write_http_response_full_async(stream, status, &headers, body).await
}

#[cfg(not(target_arch = "wasm32"))]
async fn write_http_response_full_async<W: tokio::io::AsyncWrite + Unpin>(
    stream: &mut W,
    status: u16,
    headers: &HashMap<String, String>,
    body: &str,
) -> std::io::Result<()> {
    use tokio::io::AsyncWriteExt;
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        502 => "Bad Gateway",
        _ => "Error",
    };
    let mut out = format!("HTTP/1.1 {status} {reason}\r\n");
    for (k, v) in headers {
        out.push_str(&format!("{k}: {v}\r\n"));
    }
    if !headers.keys().any(|k| k.eq_ignore_ascii_case("content-length")) {
        out.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    out.push_str("Connection: close\r\n\r\n");
    out.push_str(body);
    stream.write_all(out.as_bytes()).await?;
    stream.flush().await
}

#[cfg(not(target_arch = "wasm32"))]
async fn write_101_switching_async<W: tokio::io::AsyncWrite + Unpin>(
    stream: &mut W,
    resp_headers: &[(String, String)],
) -> std::io::Result<()> {
    use tokio::io::AsyncWriteExt;
    let mut out = String::from("HTTP/1.1 101 Switching Protocols\r\n");
    for (k, v) in resp_headers {
        let lower = k.to_ascii_lowercase();
        if lower == "transfer-encoding" || lower == "content-length" {
            continue;
        }
        out.push_str(&format!("{k}: {v}\r\n"));
    }
    out.push_str("\r\n");
    stream.write_all(out.as_bytes()).await?;
    stream.flush().await
}

fn write_plain_http_response(stream: &TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".into(),
        "text/plain; charset=utf-8".into(),
    );
    write_http_response_full(stream, status, &headers, body)
}

fn write_http_response_full(
    mut stream: &TcpStream,
    status: u16,
    headers: &HashMap<String, String>,
    body: &str,
) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        502 => "Bad Gateway",
        _ => "Error",
    };
    let mut out = format!("HTTP/1.1 {status} {reason}\r\n");
    for (k, v) in headers {
        out.push_str(&format!("{k}: {v}\r\n"));
    }
    if !headers.keys().any(|k| k.eq_ignore_ascii_case("content-length")) {
        out.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    out.push_str("Connection: close\r\n\r\n");
    out.push_str(body);
    stream.write_all(out.as_bytes())?;
    stream.flush()
}

fn write_101_switching(
    mut stream: &TcpStream,
    resp_headers: &[(String, String)],
) -> std::io::Result<()> {
    let mut out = String::from("HTTP/1.1 101 Switching Protocols\r\n");
    for (k, v) in resp_headers {
        let lower = k.to_ascii_lowercase();
        if lower == "transfer-encoding" || lower == "content-length" {
            continue;
        }
        out.push_str(&format!("{k}: {v}\r\n"));
    }
    out.push_str("\r\n");
    stream.write_all(out.as_bytes())?;
    stream.flush()
}

fn spawn_bidirectional_std_proxy(client: TcpStream, upstream: TcpStream) {
    let mut up_read = match upstream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut up_write = upstream;
    let mut client_read = match client.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut client_write = client;
    let _ = up_read.set_nodelay(true);
    let _ = up_write.set_nodelay(true);
    let _ = client_read.set_nodelay(true);
    let _ = client_write.set_nodelay(true);

    std::thread::spawn(move || {
        let mut buf = [0u8; 16_384];
        loop {
            match client_read.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if up_write.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        let _ = up_write.shutdown(Shutdown::Write);
    });
    std::thread::spawn(move || {
        let mut buf = [0u8; 16_384];
        loop {
            match up_read.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if client_write.write_all(&buf[..n]).is_err() {
                        break;
                    }
                    let _ = client_write.flush();
                }
                Err(_) => break,
            }
        }
        let _ = up_read.shutdown(Shutdown::Read);
    });
}

fn headers_to_map(req: &Request<Incoming>) -> HashMap<String, Value> {
    let mut header_map = HashMap::new();
    for (name, value) in req.headers() {
        let v = value.to_str().unwrap_or("").to_string();
        header_map.insert(name.as_str().to_string(), Value::Str(v));
    }
    header_map
}

fn build_handler_http_response(
    status: u16,
    resp_headers: HashMap<String, String>,
    resp_body: String,
) -> Response<RespBody> {
    let mut builder =
        Response::builder().status(StatusCode::from_u16(status).unwrap_or(StatusCode::OK));
    for (k, v) in resp_headers {
        if let (Ok(name), Ok(val)) = (
            HeaderName::from_bytes(k.as_bytes()),
            HeaderValue::from_str(&v),
        ) {
            builder = builder.header(name, val);
        }
    }
    builder
        .body(Full::new(Bytes::from(resp_body)))
        .unwrap_or_else(|_| Response::new(Full::new(Bytes::new())))
}

async fn handle_hyper_request(
    req: Request<Incoming>,
    handler_name: &str,
    program_ctx: Option<ProgramCtx>,
) -> Result<Response<RespBody>, Infallible> {
    let method = req.method().as_str().to_string();
    let uri = req.uri().clone();
    let path_only = uri.path().to_string();
    let query = uri.query().unwrap_or("").to_string();
    let header_map = headers_to_map(&req);
    let body = match req.collect().await {
        Ok(collected) => String::from_utf8_lossy(&collected.to_bytes()).into_owned(),
        Err(_) => String::new(),
    };
    let req_obj = http_request_object(&method, &path_only, &query, header_map, body);
    let (status, resp_headers, resp_body) =
        match invoke_http_handler_isolated(handler_name, req_obj, program_ctx) {
            Ok(resp_val) => parse_handler_response(&resp_val),
            Err(err) => (
                502,
                HashMap::from([(
                    "Content-Type".to_string(),
                    "text/plain; charset=utf-8".to_string(),
                )]),
                format!("Bad Gateway: {err}"),
            ),
        };
    Ok(build_handler_http_response(status, resp_headers, resp_body))
}

enum UpgradePrep {
    Ready {
        upstream: TcpStream,
        resp_headers: Vec<(String, String)>,
        preamble: Vec<u8>,
    },
    UpstreamFail {
        status: u16,
        headers: HashMap<String, String>,
        body: String,
    },
}

/// Dial upstream and read the Upgrade response head (sync; called before 101 to client).
fn prepare_upgrade_upstream(
    method: &str,
    request_url: &str,
    client_headers: &HashMap<String, Value>,
    upstream_url: &str,
) -> Result<UpgradePrep, String> {
    let (host, port, path) = parse_upgrade_target(upstream_url)?;
    let mut upstream = TcpStream::connect((host.as_str(), port))
        .map_err(|e| format!("upgrade dial {host}:{port}: {e}"))?;
    let _ = upstream.set_nodelay(true);
    let _ = upstream.set_read_timeout(Some(Duration::from_secs(30)));
    let _ = upstream.set_write_timeout(Some(Duration::from_secs(30)));

    let forward_path = if path == "/" {
        if request_url.is_empty() {
            "/".into()
        } else {
            request_url.to_string()
        }
    } else {
        path
    };

    let mut req_buf = format!("{method} {forward_path} HTTP/1.1\r\n");
    let mut saw_host = false;
    for (k, v) in client_headers {
        if k.starts_with("__") {
            continue;
        }
        if k.eq_ignore_ascii_case("host") {
            saw_host = true;
            req_buf.push_str(&format!("Host: {host}:{port}\r\n"));
            continue;
        }
        req_buf.push_str(&format!("{k}: {}\r\n", v.as_str()));
    }
    if !saw_host {
        req_buf.push_str(&format!("Host: {host}:{port}\r\n"));
    }
    req_buf.push_str("\r\n");
    upstream
        .write_all(req_buf.as_bytes())
        .map_err(|e| format!("upgrade write request: {e}"))?;

    let (status_line, resp_headers, preamble) =
        read_http_head(&mut upstream).map_err(|e| format!("upgrade read response: {e}"))?;
    let status = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);
    if status != 101 {
        let body = String::from_utf8_lossy(&preamble).into_owned();
        let body = if body.is_empty() {
            format!("Bad Gateway: upstream returned {status_line}")
        } else {
            body
        };
        let headers: HashMap<String, String> = resp_headers.into_iter().collect();
        return Ok(UpgradePrep::UpstreamFail {
            status,
            headers,
            body,
        });
    }
    Ok(UpgradePrep::Ready {
        upstream,
        resp_headers,
        preamble,
    })
}

fn parse_handler_response(v: &Value) -> (u16, HashMap<String, String>, String) {
    match v {
        Value::Object(map) => {
            let status = map.get("status").map(|s| s.as_i64() as u16).unwrap_or(200);
            let body = map
                .get("body")
                .map(|b| value_to_response_string(b))
                .unwrap_or_default();
            let headers = map
                .get("headers")
                .and_then(|h| match h {
                    Value::Object(m) | Value::Dict(m) => Some(
                        m.iter()
                            // `__object_id` (and other `__*` keys) are VM object metadata
                            // injected by json_to_value — must not leak as HTTP headers.
                            .filter(|(k, _)| !k.starts_with("__"))
                            .map(|(k, v)| (k.clone(), v.as_str()))
                            .collect(),
                    ),
                    _ => None,
                })
                .unwrap_or_default();
            (status, headers, body)
        }
        _ => (200, HashMap::new(), value_to_response_string(v)),
    }
}

/// Handler Object field for in-process Upgrade byte proxy (todo 7 first-slice).
fn extract_upgrade_upstream(v: &Value) -> Option<String> {
    let key = Value::Str("upgradeUpstream".into());
    let got = v.get_by_key(&key);
    match got {
        Value::Nil => None,
        Value::Str(s) if !s.is_empty() => Some(s),
        other => {
            let s = other.as_str();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
    }
}

/// W1182: in-process frame terminate / serve-computed Accept (≠ missing upgradeUpstream).
fn extract_upgrade_terminate(v: &Value) -> bool {
    let key = Value::Str("upgradeTerminate".into());
    let got = v.get_by_key(&key);
    match got {
        Value::Bool(b) => b,
        Value::Nil => false,
        other => {
            let s = other.as_str();
            s == "true" || s == "1"
        }
    }
}

fn header_value_ci(headers: &HashMap<String, Value>, name: &str) -> Option<String> {
    for (k, v) in headers {
        if k.eq_ignore_ascii_case(name) {
            let s = v.as_str();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }
    None
}

fn parse_upgrade_target(raw: &str) -> Result<(String, u16, String), String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("empty upgradeUpstream".into());
    }
    let with_scheme = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("http://{trimmed}")
    };
    let url = url::Url::parse(&with_scheme).map_err(|e| format!("bad upgradeUpstream: {e}"))?;
    let host = url
        .host_str()
        .filter(|h| !h.is_empty())
        .ok_or_else(|| "upgradeUpstream missing host".to_string())?
        .to_string();
    let port = url
        .port_or_known_default()
        .ok_or_else(|| "upgradeUpstream missing port".to_string())?;
    let mut path = url.path().to_string();
    if path.is_empty() {
        path = "/".into();
    }
    if let Some(q) = url.query() {
        path = format!("{path}?{q}");
    }
    Ok((host, port, path))
}

fn read_http_head(
    stream: &mut TcpStream,
) -> Result<(String, Vec<(String, String)>, Vec<u8>), String> {
    let mut buf = Vec::with_capacity(1024);
    let mut tmp = [0u8; 512];
    loop {
        let n = stream
            .read(&mut tmp)
            .map_err(|e| format!("read head: {e}"))?;
        if n == 0 {
            return Err("upstream closed before headers".into());
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = find_header_end(&buf) {
            let head = String::from_utf8_lossy(&buf[..pos]).into_owned();
            let rest = buf[pos + 4..].to_vec();
            let mut lines = head.split("\r\n");
            let status_line = lines.next().unwrap_or("HTTP/1.1 502").to_string();
            let mut headers = Vec::new();
            for line in lines {
                if line.is_empty() {
                    continue;
                }
                if let Some((k, v)) = line.split_once(':') {
                    headers.push((k.trim().to_string(), v.trim().to_string()));
                }
            }
            return Ok((status_line, headers, rest));
        }
        if buf.len() > 64 * 1024 {
            return Err("upstream headers too large".into());
        }
    }
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}


pub fn host_url_parse(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let parsed = url::Url::parse(&text)
        .map_err(|e| SyxDiagnostic::error(format!("URL parse failed: {e}")).with_code("R013"))?;
    let mut obj = HashMap::new();
    obj.insert("href".into(), Value::Str(parsed.as_str().to_string()));
    obj.insert(
        "protocol".into(),
        Value::Str(parsed.scheme().to_string() + ":"),
    );
    obj.insert(
        "host".into(),
        Value::Str(parsed.host_str().unwrap_or("").to_string()),
    );
    obj.insert(
        "port".into(),
        platform_int_value(parsed.port().unwrap_or(0) as i64),
    );
    obj.insert("path".into(), Value::Str(parsed.path().to_string()));
    obj.insert(
        "query".into(),
        Value::Str(parsed.query().unwrap_or("").to_string()),
    );
    stack.push(Value::Object(obj));
    Ok(())
}

pub fn host_url_encode_query(stack: &mut Vec<Value>) -> SyxResult<()> {
    let query = stack.pop().unwrap_or(Value::Nil);
    let encoded = match query {
        Value::Object(map) | Value::Dict(map) => {
            let pairs: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}={}",
                        urlencoding_encode(k),
                        urlencoding_encode(&v.as_str())
                    )
                })
                .collect();
            pairs.join("&")
        }
        _ => String::new(),
    };
    stack.push(Value::Str(encoded));
    Ok(())
}

pub fn host_url_build(stack: &mut Vec<Value>) -> SyxResult<()> {
    let query = stack.pop().unwrap_or(Value::Nil);
    let path = stack.pop().unwrap_or(Value::Str("/".into())).as_str();
    let base = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let mut url = base.trim_end_matches('/').to_string();
    if !path.is_empty() {
        if path.starts_with('/') {
            url.push_str(&path);
        } else {
            url.push('/');
            url.push_str(&path);
        }
    }
    let final_url = append_query(&url, &query)?;
    stack.push(Value::Str(final_url));
    Ok(())
}

fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        v if crate::display_numeric(v).is_some() => serde_json::json!(v.as_i64()),
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) | Value::Set(arr) => {
            serde_json::Value::Array(arr.iter().map(value_to_json).collect())
        }
        Value::Dict(map) | Value::Object(map) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in map {
                obj.insert(k.clone(), value_to_json(val));
            }
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null,
    }
}

fn json_to_value(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                platform_int_value(i)
            } else {
                Value::F64(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::Str(s.clone()),
        serde_json::Value::Array(arr) => Value::Array(arr.iter().map(json_to_value).collect()),
        serde_json::Value::Object(map) => {
            let mut obj = HashMap::new();
            for (k, v) in map {
                obj.insert(k.clone(), json_to_value(v));
            }
            crate::ensure_object_id(&mut obj);
            Value::Object(obj)
        }
    }
}
