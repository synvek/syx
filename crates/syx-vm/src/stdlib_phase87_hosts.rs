//! Phase 87: runtime cast (as? downcast), Codable L4 reflect-driven JSON, gRPC unary loopback MVP.
//! ADR-231 / M2-S7-02: recursive JSONEncoder/JSONDecoder + codingPath (R010).

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{LazyLock, Mutex};
use std::thread;
use std::time::Duration;

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_runtime_core::Value;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn object_type_name(value: &Value) -> Option<String> {
    if let Some(name) = value.object_ref_type_name() {
        return Some(name);
    }
    match Value::resolve_heap_slot(value) {
        Value::Object(map) | Value::Dict(map) => map.get("__type").and_then(|v| match v {
            Value::Str(s) => Some(s.clone()),
            _ => None,
        }),
        _ => None,
    }
}

pub fn runtime_cast_matches(type_name: &str, value: &Value) -> bool {
    let mut resolved = Value::resolve_heap_slot(value);
    if let Some(inner) = Value::mut_ref_cell_inner(&resolved) {
        resolved = Value::resolve_heap_slot(&inner);
    }
    if let Some(actual) = resolved.object_ref_type_name() {
        return type_name_matches(actual.as_str(), type_name);
    }
    // Builtin / collection values before reflect_type: Array/Set/Dict are registered
    // in meta, but runtime values are not ObjectRef and have no __type field.
    match (type_base(type_name), &resolved) {
        ("String", Value::Str(_)) => return true,
        ("Bool", Value::Bool(_)) => return true,
        ("Int" | "Int32" | "Int64", Value::I32(_) | Value::I64(_)) => return true,
        ("Float" | "Float32" | "Float64" | "Double", Value::F32(_) | Value::F64(_)) => return true,
        ("Array", Value::Array(_)) => return true,
        ("Set", Value::Set(_)) => return true,
        ("Dictionary" | "Dict", Value::Dict(_)) => return true,
        ("Nil" | "Optional", Value::Nil) => return true,
        _ => {}
    }
    if let Some(meta) = syx_meta::reflect_type(type_base(type_name)) {
        let Some(obj_type) = object_type_name(&resolved) else {
            return false;
        };
        if !type_name_matches(obj_type.as_str(), type_name) {
            return false;
        }
        let (Value::Object(map) | Value::Dict(map)) = &resolved else {
            return false;
        };
        return meta.fields.iter().all(|f| map.contains_key(f.as_str()));
    }
    matches!(
        (&resolved, type_base(type_name)),
        (Value::Object(_) | Value::Dict(_), name)
            if object_type_name(&resolved).as_deref() == Some(name)
    )
}

fn type_base(type_name: &str) -> &str {
    type_name.split('<').next().unwrap_or(type_name).trim()
}

fn type_name_matches(actual: &str, expected: &str) -> bool {
    actual == expected || type_base(actual) == type_base(expected)
}

/// Phase 87: `as?` downcast using `syx_meta` type registry + structural field check.
pub fn host_runtime_cast(stack: &mut Vec<Value>) -> SyxResult<()> {
    let target = pop_str(stack);
    let value = stack.pop().unwrap_or(Value::Nil);
    stack.push(if runtime_cast_matches(&target, &value) {
        value
    } else {
        Value::Nil
    });
    Ok(())
}

fn field_to_json(v: &Value) -> serde_json::Value {
    let resolved = Value::resolve_heap_slot(v);
    if let Some(fields) = Value::object_ref_user_fields(&resolved) {
        let mut obj = serde_json::Map::new();
        for (k, val) in fields {
            obj.insert(k, field_to_json(&val));
        }
        return serde_json::Value::Object(obj);
    }
    match resolved {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(b),
        Value::I32(n) => serde_json::json!(n),
        Value::I64(n) => serde_json::json!(n),
        Value::U8(n) => serde_json::json!(n),
        Value::F64(n) => serde_json::json!(n),
        Value::Str(s) => serde_json::Value::String(s),
        Value::Array(items) => serde_json::Value::Array(items.iter().map(field_to_json).collect()),
        Value::Object(map) | Value::Dict(map) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in map {
                if k.as_str() == "__type" || k.starts_with("__") {
                    continue;
                }
                obj.insert(k.to_string(), field_to_json(&val));
            }
            serde_json::Value::Object(obj)
        }
        other => serde_json::Value::String(other.as_str()),
    }
}

fn json_to_field(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::I64(i)
            } else {
                Value::F64(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::Str(s.clone()),
        serde_json::Value::Array(items) => Value::Array(items.iter().map(json_to_field).collect()),
        serde_json::Value::Object(map) => {
            let mut out = HashMap::new();
            for (k, val) in map {
                out.insert(k.clone(), json_to_field(val));
            }
            Value::Object(out)
        }
    }
}

fn value_as_map(value: &Value) -> Option<HashMap<String, Value>> {
    let resolved = Value::resolve_heap_slot(value);
    if let Some(fields) = Value::object_ref_user_fields(&resolved) {
        return Some(fields.into_iter().collect());
    }
    match resolved {
        Value::Object(map) | Value::Dict(map) => Some(map),
        _ => None,
    }
}

fn strip_optional(type_name: &str) -> &str {
    type_name.strip_suffix('?').unwrap_or(type_name).trim()
}

fn is_optional_type(type_name: &str) -> bool {
    type_name.trim_end().ends_with('?')
}

fn is_json_leaf_type(type_name: &str) -> bool {
    matches!(
        strip_optional(type_name),
        "" | "String"
            | "Bool"
            | "Int"
            | "Int8"
            | "Int16"
            | "Int32"
            | "Int64"
            | "Int128"
            | "UInt"
            | "UInt8"
            | "UInt16"
            | "UInt32"
            | "UInt64"
            | "UInt128"
            | "Float"
            | "Float32"
            | "Float64"
            | "Double"
            | "Character"
            | "Object"
            | "Any"
            | "Nil"
    ) || strip_optional(type_name).starts_with("Array<")
        || strip_optional(type_name).starts_with("Dictionary<")
        || strip_optional(type_name).starts_with("Set<")
}

fn field_type_at(meta: &syx_meta::TypeMetadata, idx: usize) -> &str {
    meta.field_types.get(idx).map(|s| s.as_str()).unwrap_or("")
}

fn coding_path_join(path: &[String], field: &str) -> String {
    if path.is_empty() {
        field.to_string()
    } else {
        format!("{}.{}", path.join("."), field)
    }
}

fn encode_value_recursive(type_name: &str, value: &Value) -> serde_json::Value {
    let base = strip_optional(type_name);
    if let Some(meta) = syx_meta::reflect_type(base) {
        let Some(map) = value_as_map(value) else {
            return field_to_json(value);
        };
        let mut obj = serde_json::Map::new();
        for (idx, field) in meta.fields.iter().enumerate() {
            let ft = field_type_at(&meta, idx);
            match map.get(field.as_str()) {
                Some(Value::Nil) if is_optional_type(ft) => {}
                Some(val) => {
                    if !is_json_leaf_type(ft)
                        && syx_meta::reflect_type(strip_optional(ft)).is_some()
                    {
                        obj.insert(field.to_string(), encode_value_recursive(ft, val));
                    } else {
                        obj.insert(field.to_string(), field_to_json(val));
                    }
                }
                None => {}
            }
        }
        return serde_json::Value::Object(obj);
    }
    field_to_json(value)
}

fn decode_value_recursive(
    type_name: &str,
    parsed: &serde_json::Value,
    path: &[String],
) -> SyxResult<Value> {
    let base = strip_optional(type_name);
    if let Some(meta) = syx_meta::reflect_type(base) {
        let serde_json::Value::Object(obj) = parsed else {
            return Err(SyxDiagnostic::error(format!(
                "JSONDecoder expected object for type '{base}' at codingPath={}",
                if path.is_empty() {
                    "<root>".to_string()
                } else {
                    path.join(".")
                }
            ))
            .with_code("R010"));
        };
        let mut map = HashMap::new();
        map.insert("__type".into(), Value::Str(base.to_string()));
        for (idx, field) in meta.fields.iter().enumerate() {
            let ft = field_type_at(&meta, idx);
            let optional = is_optional_type(ft);
            match obj.get(field.as_str()) {
                Some(val) => {
                    let nested_path = {
                        let mut p = path.to_vec();
                        p.push(field.to_string());
                        p
                    };
                    let decoded = if !is_json_leaf_type(ft)
                        && syx_meta::reflect_type(strip_optional(ft)).is_some()
                    {
                        decode_value_recursive(ft, val, &nested_path)?
                    } else {
                        json_to_field(val)
                    };
                    map.insert(field.to_string(), decoded);
                }
                None if optional => {}
                None => {
                    let coding_path = coding_path_join(path, field.as_str());
                    return Err(SyxDiagnostic::error(format!(
                        "JSONDecoder missing required key '{field}' codingPath={coding_path}"
                    ))
                    .with_code("R010"));
                }
            }
        }
        return Ok(Value::Object(map));
    }
    Ok(json_to_field(parsed))
}

/// Phase 87: reflect-driven L4 encode — walk registered struct fields into JSON.
pub fn host_codable_encode_l4(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Nil);
    let type_name = pop_str(stack);
    let json = serde_json::to_string(&encode_value_recursive(&type_name, &value))
        .unwrap_or_else(|_| "null".into());
    stack.push(Value::Str(json));
    Ok(())
}

/// Phase 87: reflect-driven L4 decode — rebuild struct object from JSON + metadata.
/// Missing keys remain silent (compat); path-strict decode → `host_json_decoder_decode`.
pub fn host_codable_decode_l4(stack: &mut Vec<Value>) -> SyxResult<()> {
    let json = pop_str(stack);
    let type_name = pop_str(stack);
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
    let mut map = HashMap::new();
    map.insert("__type".into(), Value::Str(type_name.clone()));
    if let Some(meta) = syx_meta::reflect_type(&type_name) {
        if let serde_json::Value::Object(obj) = parsed {
            for (idx, field) in meta.fields.iter().enumerate() {
                if let Some(val) = obj.get(field.as_str()) {
                    let ft = field_type_at(&meta, idx);
                    let decoded = if !is_json_leaf_type(ft)
                        && syx_meta::reflect_type(strip_optional(ft)).is_some()
                    {
                        decode_value_recursive(ft, val, &[field.to_string()])
                            .unwrap_or_else(|_| json_to_field(val))
                    } else {
                        json_to_field(val)
                    };
                    map.insert(field.to_string(), decoded);
                }
            }
        }
    } else if let serde_json::Value::Object(obj) = parsed {
        for (k, v) in obj {
            map.insert(k, json_to_field(&v));
        }
    }
    stack.push(Value::Object(map));
    Ok(())
}

/// ADR-231: `JSONEncoder.encode(value:typeName:)` — recursive reflect encode.
pub fn host_json_encoder_encode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let type_name = pop_str(stack);
    let value = stack.pop().unwrap_or(Value::Nil);
    let json = serde_json::to_string(&encode_value_recursive(&type_name, &value))
        .unwrap_or_else(|_| "null".into());
    stack.push(Value::Str(json));
    Ok(())
}

/// ADR-231: `JSONDecoder.decode(text:typeName:)` — recursive reflect decode; missing → R010.
pub fn host_json_decoder_decode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let type_name = pop_str(stack);
    let json = pop_str(stack);
    let parsed: serde_json::Value = serde_json::from_str(&json).map_err(|e| {
        SyxDiagnostic::error(format!("JSONDecoder invalid JSON codingPath=<root>: {e}"))
            .with_code("R010")
    })?;
    let value = decode_value_recursive(&type_name, &parsed, &[])?;
    stack.push(value);
    Ok(())
}

// --- ADR-232 / M2-S7-03: JSONStream NDJSON session + chunk feed ---

enum JsonStreamSession {
    Writer { buf: String },
    Reader { buf: String, next_line: usize },
}

static JSON_STREAM_SESSIONS: LazyLock<Mutex<HashMap<i64, JsonStreamSession>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static JSON_STREAM_NEXT_ID: AtomicI64 = AtomicI64::new(1);

fn json_stream_r010(msg: impl Into<String>) -> SyxDiagnostic {
    SyxDiagnostic::error(msg.into()).with_code("R010")
}

fn pop_handle(stack: &mut Vec<Value>) -> i64 {
    stack.pop().unwrap_or(Value::I64(0)).as_i64()
}

/// ADR-232: `JSONStream.openWriter() -> Int`
pub fn host_json_stream_open_writer(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = JSON_STREAM_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    JSON_STREAM_SESSIONS
        .lock()
        .expect("JSON_STREAM_SESSIONS poisoned")
        .insert(id, JsonStreamSession::Writer { buf: String::new() });
    stack.push(Value::I64(id));
    Ok(())
}

/// ADR-232: `JSONStream.write(handle:value:typeName:) throws`
pub fn host_json_stream_write(stack: &mut Vec<Value>) -> SyxResult<()> {
    let type_name = pop_str(stack);
    let value = stack.pop().unwrap_or(Value::Nil);
    let handle = pop_handle(stack);
    let json = serde_json::to_string(&encode_value_recursive(&type_name, &value)).map_err(|e| {
        json_stream_r010(format!(
            "JSONStream write encode failed codingPath=<root>: {e}"
        ))
    })?;
    let mut sessions = JSON_STREAM_SESSIONS
        .lock()
        .expect("JSON_STREAM_SESSIONS poisoned");
    match sessions.get_mut(&handle) {
        Some(JsonStreamSession::Writer { buf }) => {
            buf.push_str(&json);
            buf.push('\n');
            stack.push(Value::Nil);
            Ok(())
        }
        Some(JsonStreamSession::Reader { .. }) => Err(json_stream_r010(format!(
            "JSONStream write on reader handle {handle} codingPath=<root>"
        ))),
        None => Err(json_stream_r010(format!(
            "JSONStream invalid writer handle {handle} codingPath=<root>"
        ))),
    }
}

/// ADR-232: `JSONStream.finishWriter(handle:) -> String`
pub fn host_json_stream_finish_writer(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = pop_handle(stack);
    let mut sessions = JSON_STREAM_SESSIONS
        .lock()
        .expect("JSON_STREAM_SESSIONS poisoned");
    match sessions.remove(&handle) {
        Some(JsonStreamSession::Writer { buf }) => {
            stack.push(Value::Str(buf));
            Ok(())
        }
        Some(JsonStreamSession::Reader { .. }) => Err(json_stream_r010(format!(
            "JSONStream finishWriter on reader handle {handle} codingPath=<root>"
        ))),
        None => Err(json_stream_r010(format!(
            "JSONStream invalid writer handle {handle} codingPath=<root>"
        ))),
    }
}

/// ADR-232: `JSONStream.openReader() -> Int`
pub fn host_json_stream_open_reader(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = JSON_STREAM_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    JSON_STREAM_SESSIONS
        .lock()
        .expect("JSON_STREAM_SESSIONS poisoned")
        .insert(
            id,
            JsonStreamSession::Reader {
                buf: String::new(),
                next_line: 0,
            },
        );
    stack.push(Value::I64(id));
    Ok(())
}

/// ADR-232: `JSONStream.feed(handle:chunk:) throws`
pub fn host_json_stream_feed(stack: &mut Vec<Value>) -> SyxResult<()> {
    let chunk = pop_str(stack);
    let handle = pop_handle(stack);
    let mut sessions = JSON_STREAM_SESSIONS
        .lock()
        .expect("JSON_STREAM_SESSIONS poisoned");
    match sessions.get_mut(&handle) {
        Some(JsonStreamSession::Reader { buf, .. }) => {
            buf.push_str(&chunk);
            stack.push(Value::Nil);
            Ok(())
        }
        Some(JsonStreamSession::Writer { .. }) => Err(json_stream_r010(format!(
            "JSONStream feed on writer handle {handle} codingPath=<root>"
        ))),
        None => Err(json_stream_r010(format!(
            "JSONStream invalid reader handle {handle} codingPath=<root>"
        ))),
    }
}

/// ADR-232: `JSONStream.decodeNext(handle:typeName:) -> Object throws`
pub fn host_json_stream_decode_next(stack: &mut Vec<Value>) -> SyxResult<()> {
    let type_name = pop_str(stack);
    let handle = pop_handle(stack);
    let mut sessions = JSON_STREAM_SESSIONS
        .lock()
        .expect("JSON_STREAM_SESSIONS poisoned");
    let (line, line_idx) = match sessions.get_mut(&handle) {
        Some(JsonStreamSession::Reader { buf, next_line }) => {
            let Some(nl) = buf.find('\n') else {
                return Err(json_stream_r010(format!(
                    "JSONStream decodeNext incomplete line codingPath=line[{next_line}]"
                )));
            };
            let line = buf[..nl].to_string();
            *buf = buf[nl + 1..].to_string();
            let idx = *next_line;
            *next_line += 1;
            (line, idx)
        }
        Some(JsonStreamSession::Writer { .. }) => {
            return Err(json_stream_r010(format!(
                "JSONStream decodeNext on writer handle {handle} codingPath=<root>"
            )));
        }
        None => {
            return Err(json_stream_r010(format!(
                "JSONStream invalid reader handle {handle} codingPath=<root>"
            )));
        }
    };
    drop(sessions);
    let line_path = format!("line[{line_idx}]");
    if line.trim().is_empty() {
        return Err(json_stream_r010(format!(
            "JSONStream decodeNext empty line codingPath={line_path}"
        )));
    }
    let parsed: serde_json::Value = serde_json::from_str(&line).map_err(|e| {
        json_stream_r010(format!(
            "JSONStream decodeNext invalid JSON codingPath={line_path}: {e}"
        ))
    })?;
    let value = decode_value_recursive(&type_name, &parsed, &[line_path])?;
    stack.push(value);
    Ok(())
}

/// ADR-232: `JSONStream.closeReader(handle:)`
pub fn host_json_stream_close_reader(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = pop_handle(stack);
    let mut sessions = JSON_STREAM_SESSIONS
        .lock()
        .expect("JSON_STREAM_SESSIONS poisoned");
    match sessions.remove(&handle) {
        Some(JsonStreamSession::Reader { .. }) => {
            stack.push(Value::Nil);
            Ok(())
        }
        Some(JsonStreamSession::Writer { .. }) => Err(json_stream_r010(format!(
            "JSONStream closeReader on writer handle {handle} codingPath=<root>"
        ))),
        None => Err(json_stream_r010(format!(
            "JSONStream invalid reader handle {handle} codingPath=<root>"
        ))),
    }
}

static GRPC_PORT: AtomicI64 = AtomicI64::new(0);
static GRPC_HANDLERS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn grpc_loopback_handler(method: &str, body: &str) -> String {
    if let Ok(handlers) = GRPC_HANDLERS.lock() {
        if let Some(resp) = handlers.get(method) {
            return resp.clone();
        }
    }
    format!("grpc-ok:{method}:{body}")
}

fn ensure_grpc_loopback_server() {
    if GRPC_PORT.load(Ordering::Relaxed) != 0 {
        return;
    }
    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(_) => return,
    };
    let port = listener.local_addr().map(|a| a.port() as i64).unwrap_or(0);
    if port == 0 {
        return;
    }
    GRPC_PORT.store(port, Ordering::Relaxed);
    thread::spawn(move || {
        listener.set_nonblocking(true).ok();
        for _ in 0..200 {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 4096];
                if let Ok(n) = stream.read(&mut buf) {
                    let req = String::from_utf8_lossy(&buf[..n]);
                    let (method, body) = parse_grpc_http_request(&req);
                    let resp_body = grpc_loopback_handler(&method, &body);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/grpc\r\nContent-Length: {}\r\n\r\n{}",
                        resp_body.len(),
                        resp_body
                    );
                    let _ = stream.write_all(response.as_bytes());
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
    });
}

fn parse_grpc_http_request(req: &str) -> (String, String) {
    let mut method = "unknown".to_string();
    let mut body = String::new();
    if let Some(first) = req.lines().next() {
        let parts: Vec<&str> = first.split_whitespace().collect();
        if parts.len() >= 2 {
            method = parts[1].trim_start_matches('/').to_string();
        }
    }
    if let Some(idx) = req.find("\r\n\r\n") {
        body = req[idx + 4..].to_string();
    }
    (method, body)
}

fn grpc_unary_via_loopback(method: &str, body: &str) -> String {
    ensure_grpc_loopback_server();
    let port = GRPC_PORT.load(Ordering::Relaxed);
    if port == 0 {
        return grpc_loopback_handler(method, body);
    }
    let request = format!(
        "POST /{method} HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/grpc\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    if let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port as u16)) {
        stream
            .set_read_timeout(Some(Duration::from_millis(500)))
            .ok();
        stream
            .set_write_timeout(Some(Duration::from_millis(500)))
            .ok();
        if stream.write_all(request.as_bytes()).is_ok() {
            let mut buf = vec![0u8; 4096];
            if let Ok(n) = stream.read(&mut buf) {
                let resp = String::from_utf8_lossy(&buf[..n]);
                if let Some(idx) = resp.find("\r\n\r\n") {
                    return resp[idx + 4..].to_string();
                }
            }
        }
    }
    grpc_loopback_handler(method, body)
}

/// Phase 87: gRPC unary.
/// Default: in-process HTTP/1.1 facade (not wire gRPC).
/// Opt-in `SYX_GRPC_REAL=1` (W1152 / ADR-1498): narrow h2c + gRPC framing loopback.
pub fn host_grpc_unary(stack: &mut Vec<Value>) -> SyxResult<()> {
    let body = pop_str(stack);
    let method = pop_str(stack);
    let response = if crate::grpc_wire::syx_grpc_real_enabled() {
        crate::grpc_wire::grpc_wire_unary_via_loopback(&method, &body)
            .unwrap_or_else(|| grpc_unary_via_loopback(&method, &body))
    } else {
        grpc_unary_via_loopback(&method, &body)
    };
    stack.push(Value::Str(response));
    Ok(())
}

/// Register a loopback unary handler (used by native tests / GrpcServer MVP wiring).
pub fn register_grpc_loopback_handler(method: &str, response: &str) {
    if let Ok(mut handlers) = GRPC_HANDLERS.lock() {
        handlers.insert(method.to_string(), response.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn runtime_cast_matches_mut_ref_cell_point() {
        let mut inner = HashMap::new();
        inner.insert("__type".into(), Value::Str("Point".into()));
        let mut wrap = HashMap::new();
        wrap.insert("__syx_ref".into(), Value::object_ref_from_map(inner, true));
        wrap.insert("__type".into(), Value::Str("MutRefCell".into()));
        let v = Value::mut_ref_cell_from_map(wrap);
        assert_eq!(v.object_ref_type_name().as_deref(), Some("Point"));
        assert!(runtime_cast_matches("Point", &v));
    }

    #[test]
    fn runtime_cast_matches_array_and_dict() {
        assert!(runtime_cast_matches(
            "Array",
            &Value::Array(vec![Value::I32(1), Value::I32(2)])
        ));
        assert!(runtime_cast_matches(
            "Array<Int>",
            &Value::Array(vec![Value::I32(1)])
        ));
        assert!(!runtime_cast_matches("Array", &Value::Str("x".into())));
        let mut map = HashMap::new();
        map.insert("k".into(), Value::I32(1));
        assert!(runtime_cast_matches("Dictionary", &Value::Dict(map)));
        assert!(runtime_cast_matches(
            "Set",
            &Value::Set(vec![Value::I32(1)])
        ));
        assert!(runtime_cast_matches("String", &Value::Str("hi".into())));
    }
}
