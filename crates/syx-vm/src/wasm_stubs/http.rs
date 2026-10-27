//! Stub HTTP hosts when the `http` feature is off (e.g. wasm32 compiler / mir-run slim).
#![allow(unused_variables)]

use std::collections::HashMap;

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_runtime_core::Value;
use syx_stdlib::MethodId;

pub type NativeHttpHandlerFn = fn(&str, &Value) -> SyxResult<Value>;

pub fn register_native_http_handler(_f: NativeHttpHandlerFn) {}

pub fn http_response(_status: i32, _body: String, _headers: HashMap<String, Value>) -> Value {
    Value::Nil
}

pub fn is_http_response(_v: &Value) -> bool {
    false
}

pub fn execute_http_request(_url: &str, _options: &Value) -> SyxResult<Value> {
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_body_stream_open(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_body_stream_open_opts(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_body_stream_read_chunk(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_body_stream_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn http_body_stream_live_count() -> usize {
    0
}

pub fn http_body_stream_force_close(_id: i64) -> bool {
    false
}

pub fn parse_response_json(_v: &Value) -> SyxResult<Value> {
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn call_http_response_method(
    _method: MethodId,
    _receiver: &Value,
    _args: &[Value],
) -> Value {
    Value::Nil
}

pub fn host_http_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_post(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_request(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn make_http_request_awaitable(_id: i64) -> Value {
    Value::Nil
}

pub fn host_http_request_async(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn try_poll_http_request(_value: &Value) -> Option<SyxResult<Value>> {
    None
}

pub fn host_http_response_json(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_response_status(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_response_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_response_ok(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_response_header(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_response_headers(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn http_make_response(
    _status: i32,
    _headers: HashMap<String, Value>,
    _body: String,
) -> Value {
    Value::Nil
}

pub fn host_http_make_response(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn http_request_object(
    _method: &str,
    _url: &str,
    _headers: HashMap<String, Value>,
    _body: Option<String>,
) -> Value {
    Value::Nil
}

pub fn host_http_serve_shutdown(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_http_serve(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_url_parse(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_url_encode_query(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}

pub fn host_url_build(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(SyxDiagnostic::error("http feature disabled").with_code("V081"))
}
