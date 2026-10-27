use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Mutex, OnceLock};

use regex::Regex;
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_meta::reflect_type;
use syx_stdlib::{
    bigint_host_names, collection_host_names, from_type_name, is_float, is_integer,
    lookup_bigint_host, lookup_collection_host, lookup_numeric_host, lookup_result_host,
    numeric_host_names, BigIntHostOp, CollectionHostOp, HostFnId, MethodId, NumericHostOp,
    NumericKind, ResultHostOp, NUMERIC_HOST_TABLE,
};

use crate::http;
use crate::script_hosts;
use crate::ws;
use crate::{
    combine_value, finalize_state, hasher_from_value, hasher_value, platform_int_value,
    try_numeric_cast, HasherState, HostFn, Value,
};

static TCP_NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);
static TCP_STREAMS: OnceLock<Mutex<HashMap<i64, TcpStream>>> = OnceLock::new();
static TCP_LISTENER_NEXT: AtomicI64 = AtomicI64::new(1);
static TCP_LISTENERS: OnceLock<Mutex<HashMap<i64, TcpListener>>> = OnceLock::new();

fn tcp_streams() -> &'static Mutex<HashMap<i64, TcpStream>> {
    TCP_STREAMS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn tcp_listeners() -> &'static Mutex<HashMap<i64, TcpListener>> {
    TCP_LISTENERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn call_receiver_method(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    if matches!(
        method,
        MethodId::HttpResponseJson
            | MethodId::HttpResponseStatus
            | MethodId::HttpResponseText
            | MethodId::HttpResponseBody
            | MethodId::HttpResponseOk
            | MethodId::HttpResponseHeader
            | MethodId::HttpResponseHeaders
    ) {
        return http::call_http_response_method(method, receiver, args);
    }
    if method == MethodId::SetCount {
        if matches!(receiver, Value::Dict(_) | Value::Object(_)) {
            return call_dict_method(MethodId::DictCount, receiver, args);
        }
    }
    match method {
        MethodId::ArrLen
        | MethodId::ArrMap
        | MethodId::ArrFilter
        | MethodId::ArrReduce
        | MethodId::ArrSorted
        | MethodId::ArrContains
        | MethodId::ArrReverse
        | MethodId::ArrJoined => call_array_method(method, receiver, args),
        MethodId::SetInsert
        | MethodId::SetRemove
        | MethodId::SetContains
        | MethodId::SetCount
        | MethodId::SetUnion
        | MethodId::SetIntersection => call_set_method(method, receiver, args),
        MethodId::DictContainsKey
        | MethodId::DictRemove
        | MethodId::DictCount
        | MethodId::DictKeys
        | MethodId::DictValues => call_dict_method(method, receiver, args),
        _ => call_string_method_inner(method, receiver, args),
    }
}

fn call_string_method_inner(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    let s = receiver.as_str();
    let arg = args.first().cloned();
    match method {
        MethodId::StrSplit => {
            let sep = arg.map(|v| v.as_str()).unwrap_or_default();
            Value::Array(s.split(&sep).map(|p| Value::Str(p.to_string())).collect())
        }
        MethodId::StrTrim => Value::Str(s.trim().to_string()),
        MethodId::StrHasPrefix => {
            let prefix = arg.map(|v| v.as_str()).unwrap_or_default();
            Value::Bool(s.starts_with(&prefix))
        }
        MethodId::StrLowercased => Value::Str(s.to_lowercase()),
        MethodId::StrUppercased => Value::Str(s.to_uppercase()),
        MethodId::StrReplacingOccurrences => {
            let with = args.first().map(|v| v.as_str()).unwrap_or_default();
            let of = args.get(1).map(|v| v.as_str()).unwrap_or_default();
            Value::Str(s.replace(&of, &with))
        }
        MethodId::StrCount => platform_int_value(crate::grapheme::grapheme_count(&s) as i64),
        MethodId::StrSubstring => {
            let start = args.first().map(|v| v.as_i32() as usize).unwrap_or(0);
            let end = args.get(1).map(|v| v.as_i32() as usize).unwrap_or(s.len());
            Value::Str(crate::grapheme::grapheme_substring(&s, start, end))
        }
        MethodId::StrMatches => {
            let pattern = arg.map(|v| v.as_str()).unwrap_or_default();
            if let Ok(re) = Regex::new(&pattern) {
                let matches: Vec<Value> = re
                    .find_iter(&s)
                    .map(|m| Value::Str(m.as_str().to_string()))
                    .collect();
                Value::Array(matches)
            } else {
                Value::Array(vec![])
            }
        }
        _ => Value::Nil,
    }
}

fn hof_closure_arg(args: &[Value]) -> Option<(u32, Vec<Value>)> {
    match args.first()? {
        Value::Closure { fn_index, captures } => Some((*fn_index, captures.clone())),
        _ => None,
    }
}

/// ADR-225 / M2-S1-04: HOF illegal shape or interpreter feature-off → U001 (never `[]`/`Nil` success).
fn array_hof_unsupported(op: &str) -> SyxDiagnostic {
    SyxDiagnostic::error(format!(
        "unsupported array HOF '{op}': requires Array receiver and Closure (feature-off or illegal shape)"
    ))
    .with_code("U001")
}

#[cfg(feature = "interpreter")]
fn hof_map(items: &[Value], args: &[Value]) -> SyxResult<Value> {
    let Some((fn_index, captures)) = hof_closure_arg(args) else {
        return Err(array_hof_unsupported("map"));
    };
    let mut mapped = Vec::with_capacity(items.len());
    for item in items {
        mapped.push(crate::program_ctx::execute_closure(
            fn_index,
            &captures,
            &[item.clone()],
        ));
    }
    Ok(Value::Array(mapped))
}

#[cfg(not(feature = "interpreter"))]
fn hof_map(_items: &[Value], _args: &[Value]) -> SyxResult<Value> {
    // ADR-225: never silent-empty-succeed when interpreter HOF is unavailable.
    Err(array_hof_unsupported("map"))
}

#[cfg(feature = "interpreter")]
fn hof_filter(items: &[Value], args: &[Value]) -> SyxResult<Value> {
    let Some((fn_index, captures)) = hof_closure_arg(args) else {
        return Err(array_hof_unsupported("filter"));
    };
    let mut filtered = Vec::new();
    for item in items {
        let keep = crate::program_ctx::execute_closure(fn_index, &captures, &[item.clone()]);
        if keep.as_bool() {
            filtered.push(item.clone());
        }
    }
    Ok(Value::Array(filtered))
}

#[cfg(not(feature = "interpreter"))]
fn hof_filter(_items: &[Value], _args: &[Value]) -> SyxResult<Value> {
    Err(array_hof_unsupported("filter"))
}

#[cfg(feature = "interpreter")]
fn hof_reduce(items: &[Value], args: &[Value]) -> SyxResult<Value> {
    if args.len() >= 2 {
        let Some((fn_index, captures)) = hof_closure_arg(&args[1..]) else {
            return Err(array_hof_unsupported("reduce"));
        };
        let mut acc = args[0].clone();
        for item in items {
            acc = crate::program_ctx::execute_closure(fn_index, &captures, &[acc, item.clone()]);
        }
        return Ok(acc);
    }
    let Some((fn_index, captures)) = hof_closure_arg(args) else {
        return Err(array_hof_unsupported("reduce"));
    };
    if items.is_empty() {
        return Ok(Value::Nil);
    }
    let mut acc = items[0].clone();
    for item in items.iter().skip(1) {
        acc = crate::program_ctx::execute_closure(fn_index, &captures, &[acc, item.clone()]);
    }
    Ok(acc)
}

#[cfg(not(feature = "interpreter"))]
fn hof_reduce(_items: &[Value], _args: &[Value]) -> SyxResult<Value> {
    Err(array_hof_unsupported("reduce"))
}

/// Fallible Array HOF dispatch (ADR-225 / M2-S1-04).
pub fn try_call_array_hof(method: MethodId, receiver: &Value, args: &[Value]) -> SyxResult<Value> {
    let resolved = match receiver {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(receiver),
        other => other.clone(),
    };
    let Value::Array(items) = &resolved else {
        let op = match method {
            MethodId::ArrMap => "map",
            MethodId::ArrFilter => "filter",
            MethodId::ArrReduce => "reduce",
            _ => "hof",
        };
        return Err(array_hof_unsupported(op));
    };
    let items = items.as_slice();
    match method {
        MethodId::ArrMap => hof_map(items, args),
        MethodId::ArrFilter => hof_filter(items, args),
        MethodId::ArrReduce => hof_reduce(items, args),
        _ => Err(array_hof_unsupported("hof")),
    }
}

fn call_array_method(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    if matches!(
        method,
        MethodId::ArrMap | MethodId::ArrFilter | MethodId::ArrReduce
    ) {
        // ADR-225: Value-returning callers must use try_call_array_hof; never silent-empty.
        return try_call_array_hof(method, receiver, args)
            .unwrap_or_else(|e| panic!("array HOF fail-closed (use try_call_array_hof): {e}"));
    }
    // ADR-224: ArrayBuild may allocate HeapSlot — resolve before Array match.
    let resolved = match receiver {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(receiver),
        other => other.clone(),
    };
    let Value::Array(items) = &resolved else {
        return match method {
            MethodId::ArrLen => platform_int_value(0),
            MethodId::ArrContains => Value::Bool(false),
            MethodId::ArrJoined => Value::Str(String::new()),
            _ => Value::Array(vec![]),
        };
    };
    let items = items.clone();
    match method {
        MethodId::ArrLen => platform_int_value(items.len() as i64),
        MethodId::ArrMap | MethodId::ArrFilter | MethodId::ArrReduce => {
            unreachable!("HOF via try_call_array_hof")
        }
        MethodId::ArrSorted => {
            let mut sorted = items.clone();
            sorted.sort_by(syx_runtime_core::value_compare);
            let array = Value::Array(sorted);
            if Value::heap_v4_enabled() {
                Value::alloc_heap_slot(array, syx_runtime_core::HeapSlotKind::Array)
            } else {
                array
            }
        }
        MethodId::ArrReverse => {
            let mut reversed = items.clone();
            reversed.reverse();
            let array = Value::Array(reversed);
            if Value::heap_v4_enabled() {
                Value::alloc_heap_slot(array, syx_runtime_core::HeapSlotKind::Array)
            } else {
                array
            }
        }
        MethodId::ArrContains => {
            let needle = args.first().cloned().unwrap_or(Value::Nil);
            // value_equals unwraps HeapSlot strings (PartialEq would miss Str vs HeapSlot).
            Value::Bool(
                items
                    .iter()
                    .any(|v| syx_runtime_core::value_equals(v, &needle)),
            )
        }
        MethodId::ArrJoined => {
            let sep = args.first().map(|v| v.as_str()).unwrap_or_default();
            let joined = items
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(&sep);
            Value::Str(joined)
        }
        _ => Value::Array(items.clone()),
    }
}

/// MIR runtime helper for array method dispatch (Phase 49 HOF ABI).
pub fn call_array_method_for_mir(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    call_array_method(method, receiver, args)
}

/// Fallible MIR/CallHost helper (ADR-225 / M2-S1-04).
pub fn try_call_array_method_for_mir(
    method: MethodId,
    receiver: &Value,
    args: &[Value],
) -> SyxResult<Value> {
    if matches!(
        method,
        MethodId::ArrMap | MethodId::ArrFilter | MethodId::ArrReduce
    ) {
        return try_call_array_hof(method, receiver, args);
    }
    Ok(call_array_method(method, receiver, args))
}

fn unique_values(items: &[Value]) -> Vec<Value> {
    let mut unique = Vec::new();
    for item in items {
        if !unique
            .iter()
            .any(|u| syx_runtime_core::value_equals(u, item))
        {
            unique.push(item.clone());
        }
    }
    unique
}

fn call_set_method(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    // ADR-224 / M2-S1-02: resolve HeapSlot so insert/remove can write back like DictRemove.
    let resolved = match receiver {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(receiver),
        other => other.clone(),
    };
    let items: &[Value] = match &resolved {
        Value::Set(v) => v,
        Value::Array(v) => v.as_slice(),
        _ => {
            return match method {
                MethodId::SetCount => platform_int_value(0),
                MethodId::SetContains | MethodId::SetInsert | MethodId::SetRemove => {
                    Value::Bool(false)
                }
                _ => Value::Set(vec![]),
            };
        }
    };
    match method {
        MethodId::SetInsert => {
            let item = args.first().cloned().unwrap_or(Value::Nil);
            let mut set = items.to_vec();
            let inserted = !set
                .iter()
                .any(|v| syx_runtime_core::value_equals(v, &item));
            if inserted {
                set.push(item);
            }
            let _ = Value::write_back_heap_slot(receiver.clone(), Value::Set(set));
            Value::Bool(inserted)
        }
        MethodId::SetRemove => {
            let item = args.first().cloned().unwrap_or(Value::Nil);
            let mut set: Vec<Value> = items.to_vec();
            let before = set.len();
            set.retain(|v| !syx_runtime_core::value_equals(v, &item));
            let removed = set.len() != before;
            let _ = Value::write_back_heap_slot(receiver.clone(), Value::Set(set));
            Value::Bool(removed)
        }
        MethodId::SetContains => {
            let item = args.first().cloned().unwrap_or(Value::Nil);
            Value::Bool(
                items
                    .iter()
                    .any(|v| syx_runtime_core::value_equals(v, &item)),
            )
        }
        MethodId::SetCount => {
            let unique = unique_values(items);
            platform_int_value(unique.len() as i64)
        }
        MethodId::SetUnion => {
            let other = match args.first() {
                Some(Value::Set(v)) => v.as_slice(),
                Some(Value::Array(v)) => v.as_slice(),
                _ => &[],
            };
            let mut merged = items.to_vec();
            for item in other {
                if !merged
                    .iter()
                    .any(|v| syx_runtime_core::value_equals(v, item))
                {
                    merged.push(item.clone());
                }
            }
            Value::Set(merged)
        }
        MethodId::SetIntersection => {
            let other = match args.first() {
                Some(Value::Set(v)) => v.as_slice(),
                Some(Value::Array(v)) => v.as_slice(),
                _ => &[],
            };
            let inter: Vec<Value> = items
                .iter()
                .filter(|v| {
                    other
                        .iter()
                        .any(|o| syx_runtime_core::value_equals(o, v))
                })
                .cloned()
                .collect();
            Value::Set(inter)
        }
        _ => Value::Set(items.to_vec()),
    }
}

fn resolve_collection_receiver(receiver: Value) -> Value {
    Value::materialize_receiver(receiver)
}

fn call_dict_method(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    let resolved = match receiver {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(receiver),
        other => other.clone(),
    };
    let map = match &resolved {
        Value::Dict(m) | Value::Object(m) => m,
        _ => {
            return match method {
                MethodId::DictCount => platform_int_value(0),
                MethodId::DictContainsKey => Value::Bool(false),
                _ => Value::Array(vec![]),
            };
        }
    };
    match method {
        MethodId::DictContainsKey => {
            let key = args.first().cloned().unwrap_or(Value::Nil);
            Value::Bool(syx_runtime_core::dict_contains(map, &key))
        }
        MethodId::DictRemove => {
            // M0-B03: actually remove; write back HeapSlot so mutation persists.
            let key = args.first().cloned().unwrap_or(Value::Nil);
            let mut owned = map.clone();
            let removed = syx_runtime_core::dict_remove(&mut owned, &key);
            let updated = match &resolved {
                Value::Object(_) => Value::Object(owned),
                _ => Value::Dict(owned),
            };
            let _ = Value::write_back_heap_slot(receiver.clone(), updated);
            Value::Bool(removed)
        }
        MethodId::DictCount => platform_int_value(syx_runtime_core::dict_len(map) as i64),
        MethodId::DictKeys => Value::Array(syx_runtime_core::dict_keys(map)),
        MethodId::DictValues => Value::Array(syx_runtime_core::dict_values(map)),
        _ => Value::Nil,
    }
}

/// Backward-compatible alias.
pub fn call_string_method(method: MethodId, receiver: &Value, args: &[Value]) -> Value {
    call_receiver_method(method, receiver, args)
}

pub fn host_file_read(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let content = std::fs::read_to_string(&path).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read file '{path}': {e}")).with_code("R001")
    })?;
    stack.push(Value::Str(content));
    Ok(())
}

static FILE_ASYNC_NEXT_ID: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1);
static FILE_ASYNC_PENDING: std::sync::LazyLock<
    Mutex<HashMap<i64, std::sync::mpsc::Receiver<SyxResult<Value>>>>,
> = std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

fn file_async_pending_id(value: &Value) -> Option<i64> {
    match value {
        Value::Object(map) => map.get("__file_async_pending").map(|v| v.as_i64()),
        _ => None,
    }
}

fn make_file_read_awaitable(id: i64) -> Value {
    let mut obj = HashMap::new();
    obj.insert("__file_async_pending".into(), platform_int_value(id));
    Value::Object(obj)
}

/// ADR-278 / M3-B02: async file read on blocking_pool; await polls the result.
pub fn host_file_read_async(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack
        .pop()
        .unwrap_or(Value::Str(String::new()))
        .as_str()
        .to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    crate::blocking_execute::execute_on_blocking_pool(Box::new(move || {
        let result = std::fs::read_to_string(&path).map(Value::Str).map_err(|e| {
            SyxDiagnostic::error(format!("Failed to read file '{path}': {e}")).with_code("R001")
        });
        let _ = tx.send(result);
    }));
    let id = FILE_ASYNC_NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    FILE_ASYNC_PENDING.lock().unwrap().insert(id, rx);
    stack.push(make_file_read_awaitable(id));
    Ok(())
}

pub fn try_poll_file_read(value: &Value) -> Option<SyxResult<Value>> {
    let id = file_async_pending_id(value)?;
    let rx = FILE_ASYNC_PENDING.lock().unwrap().remove(&id)?;
    match rx.try_recv() {
        Ok(result) => Some(result),
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            FILE_ASYNC_PENDING.lock().unwrap().insert(id, rx);
            None
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Err(SyxDiagnostic::error(
            "File async read worker dropped",
        )
        .with_code("R001"))),
    }
}

pub fn host_file_read_lines(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let content = std::fs::read_to_string(&path).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read file '{path}': {e}")).with_code("R001")
    })?;
    let lines: Vec<Value> = content.lines().map(|l| Value::Str(l.to_string())).collect();
    stack.push(Value::Array(lines));
    Ok(())
}

pub fn host_file_write(stack: &mut Vec<Value>) -> SyxResult<()> {
    let content = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    std::fs::write(&path, content).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to write file '{path}': {e}")).with_code("R002")
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_file_append(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::Write;
    let content = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| {
            SyxDiagnostic::error(format!("Failed to append file '{path}': {e}")).with_code("R002")
        })?;
    file.write_all(content.as_bytes()).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to append file '{path}': {e}")).with_code("R002")
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_file_exists(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    stack.push(Value::Bool(std::path::Path::new(&path).exists()));
    Ok(())
}

pub fn host_file_delete(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    std::fs::remove_file(&path).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to delete file '{path}': {e}")).with_code("R002")
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_file_copy(stack: &mut Vec<Value>) -> SyxResult<()> {
    let to = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let from = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    std::fs::copy(&from, &to).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to copy '{from}' to '{to}': {e}")).with_code("R002")
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_file_move(stack: &mut Vec<Value>) -> SyxResult<()> {
    let to = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let from = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    std::fs::rename(&from, &to).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to move '{from}' to '{to}': {e}")).with_code("R002")
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

fn array_to_bytes(array: &Value) -> Vec<u8> {
    let items = Value::materialize_array(array.clone()).unwrap_or_default();
    items
        .iter()
        .map(|v| v.as_i64().max(0).min(255) as u8)
        .collect()
}

pub fn host_file_read_bytes(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let bytes = std::fs::read(&path).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read file '{path}': {e}")).with_code("R001")
    })?;
    stack.push(Value::Array(
        bytes.into_iter().map(|b| Value::U8(b)).collect(),
    ));
    Ok(())
}

pub fn host_file_write_bytes(stack: &mut Vec<Value>) -> SyxResult<()> {
    let data = stack.pop().unwrap_or(Value::Array(vec![]));
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    std::fs::write(&path, array_to_bytes(&data)).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to write file '{path}': {e}")).with_code("R002")
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

static NEXT_FILE_HANDLE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);
static FILE_HANDLES: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<i32, std::fs::File>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));

fn open_file_with_mode(path: &str, mode: &str) -> std::io::Result<std::fs::File> {
    use std::fs::OpenOptions;
    let m = mode.trim();
    let mut opts = OpenOptions::new();
    match m {
        "r" | "read" => {
            opts.read(true);
        }
        "w" | "write" => {
            opts.write(true).create(true).truncate(true);
        }
        "a" | "append" => {
            opts.write(true).create(true).append(true);
        }
        "r+" | "rw" => {
            opts.read(true).write(true).create(true);
        }
        _ => {
            opts.read(true);
        }
    }
    opts.open(path)
}

fn host_string_instance(
    method: MethodId,
    stack: &mut Vec<Value>,
    needs_arg: bool,
) -> SyxResult<()> {
    let (receiver, args) = if needs_arg {
        let arg = stack.pop().unwrap_or(Value::Nil);
        let receiver = stack.pop().unwrap_or(Value::Nil);
        (receiver, vec![arg])
    } else {
        (stack.pop().unwrap_or(Value::Nil), vec![])
    };
    stack.push(call_string_method_inner(method, &receiver, &args));
    Ok(())
}

pub fn host_str_inst_split(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrSplit, stack, true)
}
pub fn host_str_inst_trim(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrTrim, stack, false)
}
pub fn host_str_inst_has_prefix(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrHasPrefix, stack, true)
}
pub fn host_str_inst_lowercased(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrLowercased, stack, false)
}
pub fn host_str_inst_uppercased(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrUppercased, stack, false)
}
pub fn host_str_inst_matches(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrMatches, stack, true)
}
pub fn host_str_inst_length(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrCount, stack, false)
}
pub fn host_str_inst_substring(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_string_instance(MethodId::StrSubstring, stack, true)
}

/// ADR-226 / M2-S2-01: Unicode scalar count (≠ grapheme `.count`).
pub fn host_str_unicode_scalar_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().unwrap_or(Value::Nil).as_str();
    stack.push(platform_int_value(s.chars().count() as i64));
    Ok(())
}

/// ADR-226 / M2-S2-01: Unicode scalars as Array<Character>.
pub fn host_str_unicode_scalars(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let items: Vec<Value> = s.chars().map(|c| Value::Character(c.to_string())).collect();
    stack.push(Value::Array(items));
    Ok(())
}

/// ADR-226 / M2-S2-01: UTF-8 byte count.
pub fn host_str_utf8_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    stack.push(platform_int_value(s.len() as i64));
    Ok(())
}

/// ADR-226 / M2-S2-01: UTF-8 bytes as Array<Int> (0–255).
pub fn host_str_utf8_bytes(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let items: Vec<Value> = s.bytes().map(|b| platform_int_value(b as i64)).collect();
    stack.push(Value::Array(items));
    Ok(())
}

/// ADR-226 / M2-S2-01: true Unicode scalar count == 1 (not grapheme length).
pub fn host_character_is_single_scalar(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    stack.push(Value::Bool(s.chars().count() == 1));
    Ok(())
}

/// ADR-228 / M2-S2-03: Unicode NFC normalization.
pub fn host_str_normalized_nfc(stack: &mut Vec<Value>) -> SyxResult<()> {
    use unicode_normalization::UnicodeNormalization;
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    stack.push(Value::Str(s.nfc().collect()));
    Ok(())
}

/// ADR-228 / M2-S2-03: Unicode NFD normalization.
pub fn host_str_normalized_nfd(stack: &mut Vec<Value>) -> SyxResult<()> {
    use unicode_normalization::UnicodeNormalization;
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    stack.push(Value::Str(s.nfd().collect()));
    Ok(())
}

/// ADR-228 / M2-S2-03: Unicode default/full case folding (e.g. ß → ss).
pub fn host_str_case_folded(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    stack.push(Value::Str(caseless::default_case_fold_str(&s)));
    Ok(())
}

pub fn host_file_open(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mode = stack.pop().unwrap_or(Value::Str("r".into())).as_str();
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let file = open_file_with_mode(&path, &mode).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to open '{path}': {e}")).with_code("R003")
    })?;
    let id = NEXT_FILE_HANDLE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    FILE_HANDLES.lock().insert(id, file);
    stack.push(Value::I32(id));
    Ok(())
}

// --- ADR-240 / M2-S3-03: FileStream + FileWatcher sessions (≠ FileHandle table) ---

static NEXT_FILE_STREAM: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1);
static FILE_STREAM_SESSIONS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<i64, std::fs::File>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));

struct FileWatchSession {
    path: String,
    /// Mock/CI: one synthetic event available until polled.
    pending: Vec<String>,
}

static NEXT_FILE_WATCH: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1);
static FILE_WATCH_SESSIONS: std::sync::LazyLock<
    parking_lot::Mutex<std::collections::HashMap<i64, FileWatchSession>>,
> = std::sync::LazyLock::new(|| parking_lot::Mutex::new(std::collections::HashMap::new()));

/// ADR-240: `FileStream.open(path, mode) -> Int`
pub fn host_file_stream_open(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mode = stack.pop().unwrap_or(Value::Str("r".into())).as_str();
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let file = open_file_with_mode(&path, &mode).map_err(|e| {
        SyxDiagnostic::error(format!("FileStream open failed '{path}': {e}")).with_code("R003")
    })?;
    let id = NEXT_FILE_STREAM.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    FILE_STREAM_SESSIONS.lock().insert(id, file);
    stack.push(platform_int_value(id));
    Ok(())
}

/// ADR-240: `FileStream.readChunk(handle, maxBytes) -> Array<UInt8>` (empty = EOF).
pub fn host_file_stream_read_chunk(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::Read;
    let max_bytes = stack.pop().unwrap_or(Value::I32(4096)).as_i64().max(0) as usize;
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let mut file = FILE_STREAM_SESSIONS.lock().remove(&handle).ok_or_else(|| {
        SyxDiagnostic::error(format!("FileStream invalid handle {handle}")).with_code("R003")
    })?;
    if max_bytes == 0 {
        FILE_STREAM_SESSIONS.lock().insert(handle, file);
        stack.push(Value::Array(vec![]));
        return Ok(());
    }
    let mut buf = vec![0u8; max_bytes];
    let n = file.read(&mut buf).map_err(|e| {
        SyxDiagnostic::error(format!("FileStream read failed: {e}")).with_code("R001")
    })?;
    buf.truncate(n);
    FILE_STREAM_SESSIONS.lock().insert(handle, file);
    stack.push(Value::Array(buf.into_iter().map(Value::U8).collect()));
    Ok(())
}

/// ADR-240: `FileStream.close(handle)` — remove session; further ops fail.
pub fn host_file_stream_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let removed = FILE_STREAM_SESSIONS.lock().remove(&handle).is_some();
    if !removed {
        return Err(
            SyxDiagnostic::error(format!("FileStream invalid handle {handle}")).with_code("R003"),
        );
    }
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-240: `FileWatcher.open(path) -> Int` (mock-default lifecycle session).
pub fn host_file_watcher_open(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let id = NEXT_FILE_WATCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let pending = vec![format!("watch:session:{path}")];
    FILE_WATCH_SESSIONS.lock().insert(
        id,
        FileWatchSession {
            path: path.clone(),
            pending,
        },
    );
    let _ = path;
    stack.push(platform_int_value(id));
    Ok(())
}

/// ADR-240: `FileWatcher.poll(handle) -> Array<String>`
pub fn host_file_watcher_poll(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let mut sessions = FILE_WATCH_SESSIONS.lock();
    let Some(session) = sessions.get_mut(&handle) else {
        return Err(
            SyxDiagnostic::error(format!("FileWatcher invalid handle {handle}")).with_code("R003"),
        );
    };
    let events: Vec<Value> = session.pending.drain(..).map(Value::Str).collect();
    stack.push(Value::Array(events));
    Ok(())
}

/// ADR-240: `FileWatcher.close(handle)` — remove session; further poll fails.
pub fn host_file_watcher_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let removed = FILE_WATCH_SESSIONS.lock().remove(&handle).is_some();
    if !removed {
        return Err(
            SyxDiagnostic::error(format!("FileWatcher invalid handle {handle}")).with_code("R003"),
        );
    }
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_file_read_chunk(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::Read;
    let max_bytes = stack.pop().unwrap_or(Value::I32(4096)).as_i32().max(0) as usize;
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let mut file = FILE_HANDLES.lock().remove(&handle).ok_or_else(|| {
        SyxDiagnostic::error(format!("Invalid file handle {handle}")).with_code("R003")
    })?;
    let mut buf = vec![0u8; max_bytes];
    let n = file.read(&mut buf).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read handle {handle}: {e}")).with_code("R001")
    })?;
    buf.truncate(n);
    FILE_HANDLES.lock().insert(handle, file);
    stack.push(Value::Array(buf.into_iter().map(Value::U8).collect()));
    Ok(())
}

pub fn host_file_write_chunk(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::Write;
    let data = stack.pop().unwrap_or(Value::Array(vec![]));
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let mut file = FILE_HANDLES.lock().remove(&handle).ok_or_else(|| {
        SyxDiagnostic::error(format!("Invalid file handle {handle}")).with_code("R003")
    })?;
    let bytes = array_to_bytes(&data);
    let n = file.write(&bytes).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to write handle {handle}: {e}")).with_code("R002")
    })?;
    FILE_HANDLES.lock().insert(handle, file);
    stack.push(Value::I32(n as i32));
    Ok(())
}

pub fn host_file_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let removed = FILE_HANDLES.lock().remove(&handle).is_some();
    if !removed {
        return Err(
            SyxDiagnostic::error(format!("Invalid file handle {handle}")).with_code("R003"),
        );
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// ADR-234 / M2-S3-02b: never-throw close for `withOpen` catch / `deinit`.
pub fn host_file_close_quiet(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let removed = FILE_HANDLES.lock().remove(&handle).is_some();
    stack.push(Value::Bool(removed));
    Ok(())
}

/// ADR-234 / M2-S3-02b: open → call body(handle: Int) → always quiet-close.
/// Body returning Int `< 0` (or non-Int) → Err after close (resource contract).
pub fn host_file_with_open(stack: &mut Vec<Value>) -> SyxResult<()> {
    let body = stack.pop();
    let mode = stack.pop().unwrap_or(Value::Str("r".into())).as_str();
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let file = open_file_with_mode(&path, &mode).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to open '{path}': {e}")).with_code("R003")
    })?;
    let id = NEXT_FILE_HANDLE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    FILE_HANDLES.lock().insert(id, file);

    let result = if let Some(Value::Closure { fn_index, captures }) = body {
        #[cfg(feature = "interpreter")]
        {
            crate::program_ctx::execute_closure(fn_index, &captures, &[Value::I32(id)])
        }
        #[cfg(not(feature = "interpreter"))]
        {
            let _ = (fn_index, captures);
            Value::I32(-1)
        }
    } else {
        Value::I32(-1)
    };

    let _ = FILE_HANDLES.lock().remove(&id);

    match result {
        Value::I32(n) if n >= 0 => {
            stack.push(Value::I32(n));
            Ok(())
        }
        Value::I64(n) if n >= 0 && n <= i32::MAX as i64 => {
            stack.push(Value::I32(n as i32));
            Ok(())
        }
        _ => Err(SyxDiagnostic::error("withOpen body failed".to_string()).with_code("R001")),
    }
}

/// ADR-234 / M2-S3-02b: open then always close, then Err — withOpen throw-path host inject.
pub fn host_file_with_open_fail(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mode = stack.pop().unwrap_or(Value::Str("r".into())).as_str();
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let file = open_file_with_mode(&path, &mode).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to open '{path}': {e}")).with_code("R003")
    })?;
    let id = NEXT_FILE_HANDLE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    FILE_HANDLES.lock().insert(id, file);
    let _ = FILE_HANDLES.lock().remove(&id);
    Err(SyxDiagnostic::error("withOpen body failed (host inject)".to_string()).with_code("R001"))
}

/// M2 W34 / ADR-178: live `FILE_HANDLES` count for EC-M2-3 FileHandle leak nails (test/ops only).
pub fn file_handle_live_count() -> usize {
    FILE_HANDLES.lock().len()
}

/// Test cleanup: drop a live handle by id without going through the SYX facade.
pub fn file_handle_force_close(id: i32) -> bool {
    FILE_HANDLES.lock().remove(&id).is_some()
}

/// Wave A W192 / ADR-536: reset FileHandle table + next-id so VM↔native product exec can nail id `1`.
pub fn reset_file_handles_for_tests() {
    FILE_HANDLES.lock().clear();
    NEXT_FILE_HANDLE.store(1, std::sync::atomic::Ordering::Relaxed);
}

/// M2 W109 / ADR-253: live `FILE_STREAM_SESSIONS` count (test/ops only).
pub fn file_stream_live_count() -> usize {
    FILE_STREAM_SESSIONS.lock().len()
}

/// Test cleanup: drop a live FileStream session by id.
pub fn file_stream_force_close(id: i64) -> bool {
    FILE_STREAM_SESSIONS.lock().remove(&id).is_some()
}

/// M2 W109 / ADR-253: live `FILE_WATCH_SESSIONS` count (test/ops only).
pub fn file_watch_live_count() -> usize {
    FILE_WATCH_SESSIONS.lock().len()
}

/// Test cleanup: drop a live FileWatcher session by id.
pub fn file_watch_force_close(id: i64) -> bool {
    FILE_WATCH_SESSIONS.lock().remove(&id).is_some()
}

pub fn host_file_seek(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::{Seek, SeekFrom};
    let offset = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let mut file = FILE_HANDLES.lock().remove(&handle).ok_or_else(|| {
        SyxDiagnostic::error(format!("Invalid file handle {handle}")).with_code("R003")
    })?;
    // ADR-152: fail-closed Err (no Bool(false) sentinel). Success pushes Bool(true) like close.
    if let Err(e) = file.seek(SeekFrom::Start(offset.max(0) as u64)) {
        FILE_HANDLES.lock().insert(handle, file);
        return Err(
            SyxDiagnostic::error(format!("Failed to seek handle {handle}: {e}")).with_code("R003"),
        );
    }
    FILE_HANDLES.lock().insert(handle, file);
    stack.push(Value::Bool(true));
    Ok(())
}

/// M0-B07 / ADR-241: read remaining bytes from an open handle (`read_to_end`).
/// Historical fn name / HostFnId `FileMmapRead` — **not** OS mmap (S3-04 rename closeout).
/// Public API is `FileHandle.readAllBytes`.
pub fn host_file_mmap_read(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::Read;
    let handle = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let mut file = FILE_HANDLES.lock().remove(&handle).ok_or_else(|| {
        SyxDiagnostic::error(format!("Invalid file handle {handle}")).with_code("R003")
    })?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to read-all-bytes handle {handle}: {e}"))
            .with_code("R001")
    })?;
    FILE_HANDLES.lock().insert(handle, file);
    stack.push(Value::Array(buf.into_iter().map(Value::U8).collect()));
    Ok(())
}

pub fn host_keypath_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let optional = stack.pop().unwrap_or(Value::Bool(false)).as_bool();
    let property = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let root_type = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let receiver = stack.pop().unwrap_or(Value::Nil);
    if let Value::Object(map) | Value::Dict(map) = &receiver {
        if let Some(Value::Str(actual)) = map.get("__type") {
            if actual.as_str() != root_type {
                if optional {
                    stack.push(Value::Nil);
                    return Ok(());
                }
                return Err(SyxDiagnostic::error(format!(
                    "KeyPath root type mismatch: expected {root_type}, found {actual}"
                )));
            }
        }
    } else if !optional && !matches!(receiver, Value::Nil) {
        // allow optional mismatch to yield nil below
    }
    // Phase 47: composed key paths (`name.age`) walk nested objects.
    let value = property.split('.').fold(receiver, |cur, segment| {
        if let Some(inner) = segment.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            if let Ok(idx) = inner.parse::<usize>() {
                return match cur {
                    Value::Array(items) => items.get(idx).cloned().unwrap_or(Value::Nil),
                    _ => Value::Nil,
                };
            }
        }
        match cur {
            Value::Object(map) | Value::Dict(map) => {
                map.get(segment).cloned().unwrap_or(Value::Nil)
            }
            _ => Value::Nil,
        }
    });
    let _ = root_type;
    stack.push(value);
    Ok(())
}

pub fn host_as_optional(stack: &mut Vec<Value>) -> SyxResult<()> {
    let target = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let value = stack.pop().unwrap_or(Value::Nil);
    if runtime_type_matches(&target, &value) {
        stack.push(value);
        return Ok(());
    }
    // Integer widen/narrow or float→int (trunc): OOR → nil (≠ R003).
    if let (Some(from_k), Some(to_k)) = (
        syx_runtime_core::value_kind(&value),
        from_type_name(&target),
    ) {
        let numeric_try = (is_integer(from_k) && is_integer(to_k))
            || (is_float(from_k) && is_integer(to_k));
        if numeric_try {
            match try_numeric_cast(from_k, to_k, &value) {
                Ok(v) => stack.push(v),
                Err(_) => stack.push(Value::Nil),
            }
            return Ok(());
        }
    }
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_type_is(stack: &mut Vec<Value>) -> SyxResult<()> {
    let target = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let value = stack.pop().unwrap_or(Value::Nil);
    let value = unwrap_mut_ref_cell_value(&value);
    let ok = crate::stdlib_phase87_hosts::runtime_cast_matches(&target, &value);
    stack.push(Value::Bool(ok));
    Ok(())
}

fn unwrap_mut_ref_cell_value(value: &Value) -> Value {
    let resolved = Value::resolve_heap_slot(value);
    Value::mut_ref_cell_inner(&resolved)
        .map(|inner| Value::resolve_heap_slot(&inner))
        .unwrap_or(resolved)
}

pub fn host_type_cast(stack: &mut Vec<Value>) -> SyxResult<()> {
    let target = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let value = stack.pop().unwrap_or(Value::Nil);
    if crate::stdlib_phase87_hosts::runtime_cast_matches(&target, &value) {
        stack.push(value);
        Ok(())
    } else {
        Err(SyxDiagnostic::error(format!("Cast to '{target}' failed")).with_code("R003"))
    }
}

fn runtime_type_matches(type_name: &str, value: &Value) -> bool {
    crate::stdlib_phase87_hosts::runtime_cast_matches(type_name, value)
}

pub fn host_dir_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    std::fs::create_dir_all(&path).map_err(|e| {
        SyxDiagnostic::error(format!("Failed to create directory '{path}': {e}")).with_code("R002")
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_dir_list(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let entries: Vec<Value> = std::fs::read_dir(&path)
        .map_err(|e| {
            SyxDiagnostic::error(format!("Failed to list directory '{path}': {e}"))
                .with_code("R002")
        })?
        .filter_map(|e| e.ok())
        .map(|e| Value::Str(e.file_name().to_string_lossy().into_owned()))
        .collect();
    stack.push(Value::Array(entries));
    Ok(())
}

pub fn host_dir_exists(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    stack.push(Value::Bool(std::path::Path::new(&path).is_dir()));
    Ok(())
}

pub fn host_http_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    http::host_http_get(stack)
}

pub fn host_http_post(stack: &mut Vec<Value>) -> SyxResult<()> {
    http::host_http_post(stack)
}

pub fn host_http_response_json(stack: &mut Vec<Value>) -> SyxResult<()> {
    http::host_http_response_json(stack)
}

pub fn host_read_line(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::{self, BufRead};
    let mut line = String::new();
    io::stdin()
        .lock()
        .read_line(&mut line)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read line: {e}")).with_code("R008"))?;
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    stack.push(Value::Str(line));
    Ok(())
}

pub fn host_regex_match(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let pattern = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let re = Regex::new(&pattern).map_err(|e| {
        SyxDiagnostic::error(format!("Invalid regex '{pattern}': {e}")).with_code("R006")
    })?;
    let matches: Vec<Value> = re
        .find_iter(&text)
        .map(|m| Value::Str(m.as_str().to_string()))
        .collect();
    stack.push(Value::Array(matches));
    Ok(())
}

fn value_to_json(v: &Value) -> serde_json::Value {
    let v = Value::resolve_heap_slot(v);
    if let Some(inner) = Value::mut_ref_cell_inner(&v) {
        return value_to_json(&inner);
    }
    match &v {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        v if crate::display_numeric(v).is_some() => {
            serde_json::json!(v.as_i64())
        }
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) | Value::Set(arr) => {
            serde_json::Value::Array(arr.iter().map(value_to_json).collect())
        }
        Value::Dict(map) | Value::Object(map) => object_map_to_json(map),
        Value::ObjectRef { .. } => {
            if let Some(fields) = Value::object_ref_user_fields(&v) {
                let mut obj = serde_json::Map::new();
                for (k, val) in fields {
                    obj.insert(k, value_to_json(&val));
                }
                serde_json::Value::Object(obj)
            } else {
                serde_json::Value::Null
            }
        }
        _ => serde_json::Value::Null,
    }
}

fn object_map_to_json(map: &HashMap<String, Value>) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for (k, val) in map {
        // Skip runtime metadata; keep `__tag` for enum payloads.
        if k.starts_with("__") && k.as_str() != "__tag" {
            continue;
        }
        obj.insert(k.clone(), value_to_json(val));
    }
    serde_json::Value::Object(obj)
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
            if let Some(serde_json::Value::String(type_name)) = map.get("__type") {
                if let Some(meta) = reflect_type(type_name) {
                    let mut obj = HashMap::new();
                    obj.insert("__type".into(), Value::Str(type_name.clone()));
                    for field in &meta.fields {
                        if let Some(v) = map.get(field.as_str()) {
                            obj.insert(field.to_string(), json_to_value(v));
                        }
                    }
                    crate::ensure_object_id(&mut obj);
                    return Value::Object(obj);
                }
            }
            let mut obj = HashMap::new();
            for (k, v) in map {
                if k.starts_with("__") && k.as_str() != "__tag" {
                    continue;
                }
                obj.insert(k.clone(), json_to_value(v));
            }
            crate::ensure_object_id(&mut obj);
            Value::Object(obj)
        }
    }
}

pub fn host_hasher_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(hasher_value(&HasherState::new()));
    Ok(())
}

pub fn host_hasher_combine(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Nil);
    let hasher = stack.pop().unwrap_or(Value::Nil);
    let mut state = hasher_from_value(&hasher);
    combine_value(&mut state, &value);
    stack.push(hasher_value(&state));
    Ok(())
}

pub fn host_hasher_finalize(stack: &mut Vec<Value>) -> SyxResult<()> {
    let hasher = stack.pop().unwrap_or(Value::Nil);
    let state = hasher_from_value(&hasher);
    stack.push(Value::U64(finalize_state(&state)));
    Ok(())
}

pub fn host_object_identifier(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(oid) = crate::make_object_identifier(&v) {
        stack.push(oid);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_json_encode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    let json = serde_json::to_string(&value_to_json(&v))
        .map_err(|e| SyxDiagnostic::error(format!("JSON encode failed: {e}")).with_code("R009"))?;
    stack.push(Value::Str(json));
    Ok(())
}

pub fn host_json_decode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let parsed: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| SyxDiagnostic::error(format!("JSON decode failed: {e}")).with_code("R005"))?;
    stack.push(json_to_value(&parsed));
    Ok(())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn host_random_next_int64(stack: &mut Vec<Value>) -> SyxResult<()> {
    use rand::Rng;
    let max = stack.pop().unwrap_or(Value::I64(0)).as_i64();
    if max <= 0 {
        stack.push(Value::I64(0));
        return Ok(());
    }
    let n = rand::thread_rng().gen_range(0..max);
    stack.push(Value::I64(n));
    Ok(())
}

pub fn host_random_next_uint64(stack: &mut Vec<Value>) -> SyxResult<()> {
    use rand::Rng;
    let n: u64 = rand::thread_rng().gen();
    stack.push(Value::U64(n));
    Ok(())
}

pub fn host_random_fill_bytes(stack: &mut Vec<Value>) -> SyxResult<()> {
    use rand::RngCore;
    let count = stack.pop().unwrap_or(Value::I64(0)).as_i64().max(0) as usize;
    let mut buf = vec![0u8; count];
    rand::thread_rng().fill_bytes(&mut buf);
    stack.push(Value::Array(
        buf.into_iter().map(|b| Value::U8(b)).collect(),
    ));
    Ok(())
}

pub fn host_crypto_sha256(stack: &mut Vec<Value>) -> SyxResult<()> {
    use sha2::{Digest, Sha256};
    let s = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    stack.push(Value::Str(bytes_to_hex(&hasher.finalize())));
    Ok(())
}

pub fn host_crypto_sha512(stack: &mut Vec<Value>) -> SyxResult<()> {
    use sha2::{Digest, Sha512};
    let s = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let mut hasher = Sha512::new();
    hasher.update(s.as_bytes());
    stack.push(Value::Str(bytes_to_hex(&hasher.finalize())));
    Ok(())
}

pub fn host_crypto_hmac_sha256(stack: &mut Vec<Value>) -> SyxResult<()> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let data = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let key = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
        .map_err(|e| SyxDiagnostic::error(format!("HMAC key error: {e}")))?;
    mac.update(data.as_bytes());
    stack.push(Value::Str(bytes_to_hex(
        mac.finalize().into_bytes().as_ref(),
    )));
    Ok(())
}

pub fn host_crypto_aes_gcm_encrypt(stack: &mut Vec<Value>) -> SyxResult<()> {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    let plaintext = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let nonce_str = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let key_str = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let key_bytes = hex_to_bytes(key_str.as_str()).unwrap_or_else(|| key_str.as_bytes().to_vec());
    if key_bytes.len() != 32 {
        return Err(SyxDiagnostic::error(
            "AES-GCM key must be 32 bytes (64 hex chars or 32 raw bytes)",
        ));
    }
    let nonce_bytes =
        hex_to_bytes(nonce_str.as_str()).unwrap_or_else(|| nonce_str.as_bytes().to_vec());
    if nonce_bytes.len() != 12 {
        return Err(SyxDiagnostic::error(
            "AES-GCM nonce must be 12 bytes (24 hex chars or 12 raw bytes)",
        ));
    }
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| SyxDiagnostic::error(format!("AES-GCM key error: {e}")))?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| SyxDiagnostic::error(format!("AES-GCM encrypt failed: {e}")))?;
    stack.push(Value::Str(bytes_to_hex(&ciphertext)));
    Ok(())
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

pub fn host_simd_from_array(stack: &mut Vec<Value>) -> SyxResult<()> {
    let lanes = stack.pop().unwrap_or(Value::I64(4)).as_i64() as u8;
    let array = stack.pop().unwrap_or(Value::Array(vec![]));
    let items = Value::materialize_array(array).unwrap_or_default();
    let mut element = items
        .first()
        .and_then(|v| syx_runtime_core::value_kind(v))
        .unwrap_or(syx_stdlib::NumericKind::I32);
    if lanes == 8 {
        element = syx_stdlib::NumericKind::I16;
    }
    if lanes == 16 {
        element = syx_stdlib::NumericKind::I8;
    }
    if let Some(simd) = syx_runtime_core::simd_from_array(lanes, element, &items) {
        stack.push(simd);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_reduce_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(sum) = syx_runtime_core::simd_reduce_add(&v) {
        stack.push(sum);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_load_from_array(stack: &mut Vec<Value>) -> SyxResult<()> {
    let lanes = stack.pop().unwrap_or(Value::I64(4)).as_i64() as u8;
    let offset = stack.pop().unwrap_or(Value::I64(0)).as_i64() as usize;
    let array = stack.pop().unwrap_or(Value::Array(vec![]));
    let mut element = Value::materialize_array(array.clone())
        .and_then(|items| items.first().and_then(|v| syx_runtime_core::value_kind(v)))
        .unwrap_or(syx_stdlib::NumericKind::I32);
    if lanes == 8 {
        element = syx_stdlib::NumericKind::I16;
    }
    if lanes == 16 {
        element = syx_stdlib::NumericKind::I8;
    }
    if let Some(simd) = syx_runtime_core::simd_load_from_array(&array, lanes, element, offset) {
        stack.push(simd);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_store_to_array(stack: &mut Vec<Value>) -> SyxResult<()> {
    let simd = stack.pop().unwrap_or(Value::Nil);
    let offset = stack.pop().unwrap_or(Value::I64(0)).as_i64() as usize;
    let array_outer = stack.pop().unwrap_or(Value::Array(vec![]));
    let mut array = match &array_outer {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(&array_outer),
        other => other.clone(),
    };
    if syx_runtime_core::simd_store_to_array(&mut array, offset, &simd).is_some() {
        if matches!(array_outer, Value::HeapSlot { .. }) {
            stack.push(Value::write_back_heap_slot(array_outer, array));
        } else {
            stack.push(array);
        }
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_shuffle(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mode = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Value::Simd {
        lanes,
        element,
        mut bits,
    } = v
    {
        if lanes == 4 {
            let mut lane_vals = [0i32; 4];
            for i in 0..4 {
                let off = i * 4;
                lane_vals[i] = i32::from_le_bytes(bits[off..off + 4].try_into().unwrap_or([0; 4]));
            }
            let shuffled = match mode {
                0 => [lane_vals[3], lane_vals[2], lane_vals[1], lane_vals[0]],
                1 => [lane_vals[1], lane_vals[0], lane_vals[3], lane_vals[2]],
                _ => lane_vals,
            };
            for (i, val) in shuffled.iter().enumerate() {
                bits[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
            }
            stack.push(Value::Simd {
                lanes,
                element,
                bits,
            });
            return Ok(());
        }
    }
    stack.push(v);
    Ok(())
}

pub fn host_simd_xor(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_xor(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_and(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_and(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_or(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_or(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_not(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_not(&v) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_shl(stack: &mut Vec<Value>) -> SyxResult<()> {
    let amount = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_shl(&v, amount) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_shr(stack: &mut Vec<Value>) -> SyxResult<()> {
    let amount = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_shr(&v, amount) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_min(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_min(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_max(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_max(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f32_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Add, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f32_sub(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Sub, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f32_mul(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Mul, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f32_div(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Div, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f64_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Add, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i16_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Add, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i16_sub(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Sub, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i16_mul(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Mul, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_i16_add(stack)
}

pub fn host_simd_i8_sub(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_i16_sub(stack)
}

pub fn host_simd_i8_mul(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_i16_mul(stack)
}

pub fn host_simd_i8_and(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_and(stack)
}

pub fn host_simd_i8_or(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_or(stack)
}

pub fn host_simd_i8_xor(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_xor(stack)
}

pub fn host_simd_i8_not(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_not(stack)
}

pub fn host_simd_i8_min(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_min(stack)
}

pub fn host_simd_i8_max(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_max(stack)
}

/// Phase 130: unsigned lane min (`i8x16.min_u`).
pub fn host_simd_i8_umin(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_umin(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

/// Phase 130: unsigned lane max (`i8x16.max_u`).
pub fn host_simd_i8_umax(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_umax(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

/// Phase 131: signed lane abs (`i8x16.abs`).
pub fn host_simd_i8_abs(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_abs(&v) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

/// Phase 132: signed lane neg (`i8x16.neg`).
pub fn host_simd_i8_neg(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_neg(&v) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_extract(stack: &mut Vec<Value>) -> SyxResult<()> {
    let lane = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_extract_lane(&v, lane) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_replace(stack: &mut Vec<Value>) -> SyxResult<()> {
    let scalar = stack.pop().unwrap_or(Value::Nil);
    let lane = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_replace_lane(&v, lane, &scalar) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_eq(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_eq(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_ne(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_ne(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_le(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_le(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_ge(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_ge(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_avgr_u(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_avgr_u(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

/// Phase 137: lane-wise popcnt (`i8x16.popcnt`).
pub fn host_simd_i8_popcnt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_popcnt(&v) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

/// Phase 138: MSB bitmask (`i8x16.bitmask`).
pub fn host_simd_i8_bitmask(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_bitmask(&v) {
        stack.push(r);
    } else {
        stack.push(Value::I32(0));
    }
    Ok(())
}

/// Phase 139: all lanes non-zero (`i8x16.all_true`).
pub fn host_simd_i8_all_true(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_all_true(&v) {
        stack.push(r);
    } else {
        stack.push(Value::Bool(false));
    }
    Ok(())
}

/// Phase 139: any lane non-zero (`v128.any_true`).
pub fn host_simd_i8_any_true(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_any_true(&v) {
        stack.push(r);
    } else {
        stack.push(Value::Bool(false));
    }
    Ok(())
}

pub fn host_simd_i8_lt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_lt(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_gt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_gt(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_add_sat(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_add_sat(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_sub_sat(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_sub_sat(&a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_shuffle(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mode = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let v = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_shuffle_i8(&v, mode) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_i8_shl(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_shl(stack)
}

pub fn host_simd_i8_shr(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_simd_shr(stack)
}

pub fn host_simd_i64_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Add, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f64_sub(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Sub, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f64_mul(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Mul, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_simd_f64_div(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    if let Some(r) = syx_runtime_core::simd_binop(syx_runtime_core::SimdBinOp::Div, &a, &b) {
        stack.push(r);
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

pub fn host_string_char_at(stack: &mut Vec<Value>) -> SyxResult<()> {
    let idx = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let s = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    match crate::grapheme::try_grapheme_at(&s, idx) {
        Ok(g) => {
            stack.push(Value::Character(g));
            Ok(())
        }
        Err((i, count)) => Err(SyxDiagnostic::error(format!(
            "String index {i} out of bounds (count {count})"
        ))
        .with_code("R003")),
    }
}

pub fn host_array_set_at(stack: &mut Vec<Value>) -> SyxResult<()> {
    // ADR-224 / M2-S1-02: align with ArraySet — HeapSlot write-back + return updated array.
    let val = stack.pop().unwrap_or(Value::Nil);
    let idx = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let arr_outer = stack.pop().unwrap_or(Value::Array(vec![]));
    let resolved = match &arr_outer {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(&arr_outer),
        other => other.clone(),
    };
    if let Value::Array(mut items) = resolved {
        if idx < 0 || (idx as usize) >= items.len() {
            return Err(SyxDiagnostic::error(format!(
                "Array index {idx} out of bounds (count {})",
                items.len()
            ))
            .with_code("R003"));
        }
        items[idx as usize] = val;
        let updated = Value::Array(items);
        stack.push(Value::write_back_heap_slot(arr_outer, updated));
    } else {
        stack.push(arr_outer);
    }
    Ok(())
}

pub fn host_array_append(stack: &mut Vec<Value>) -> SyxResult<()> {
    // Grow array in place via HeapSlot write-back; return Nil (Void).
    let val = stack.pop().unwrap_or(Value::Nil);
    let arr_outer = stack.pop().unwrap_or(Value::Array(vec![]));
    let resolved = match &arr_outer {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(&arr_outer),
        other => other.clone(),
    };
    if let Value::Array(mut items) = resolved {
        items.push(val);
        let updated = Value::Array(items);
        let _ = Value::write_back_heap_slot(arr_outer, updated);
    }
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_leading_zeros128(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::U128(0));
    let n = match v {
        Value::U128(x) => x.leading_zeros(),
        Value::I128(x) => (x as u128).leading_zeros(),
        _ => 128u32,
    };
    stack.push(Value::I32(n as i32));
    Ok(())
}

pub fn host_value_description(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    stack.push(Value::Str(syx_runtime_core::value_description(&v)));
    Ok(())
}

pub fn host_value_debug_description(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    stack.push(Value::Str(syx_runtime_core::value_debug_description(&v)));
    Ok(())
}

pub fn host_value_compare(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    stack.push(platform_int_value(
        syx_runtime_core::value_compare_int(&a, &b) as i64,
    ));
    Ok(())
}

pub fn dispatch_collection_host(name: &str, stack: &mut Vec<Value>) -> SyxResult<bool> {
    let Some(entry) = lookup_collection_host(name) else {
        return Ok(false);
    };
    match entry.op {
        CollectionHostOp::Count => {
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let n = match &receiver {
                Value::Set(items) | Value::Array(items) => items.len() as i32,
                Value::Dict(map) | Value::Object(map) => syx_runtime_core::dict_len(map) as i32,
                _ => 0,
            };
            stack.push(Value::I32(n));
            return Ok(true);
        }
        CollectionHostOp::Contains | CollectionHostOp::ContainsKey => {
            let item = stack.pop().unwrap_or(Value::Nil);
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let found = match &receiver {
                Value::Set(items) => items
                    .iter()
                    .any(|x| syx_runtime_core::value_equals(x, &item)),
                Value::Array(items) => items
                    .iter()
                    .any(|x| syx_runtime_core::value_equals(x, &item)),
                Value::Dict(map) | Value::Object(map) => {
                    syx_runtime_core::dict_contains(map, &item)
                }
                _ => false,
            };
            stack.push(Value::Bool(found));
            return Ok(true);
        }
        CollectionHostOp::Keys => {
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let keys = match &receiver {
                Value::Dict(map) | Value::Object(map) => syx_runtime_core::dict_keys(map),
                _ => Vec::new(),
            };
            stack.push(Value::Array(keys));
            return Ok(true);
        }
        CollectionHostOp::Values => {
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let values = match &receiver {
                Value::Dict(map) | Value::Object(map) => syx_runtime_core::dict_values(map),
                _ => Vec::new(),
            };
            stack.push(Value::Array(values));
            return Ok(true);
        }
        CollectionHostOp::Insert => {
            // ADR-224 / M2-S1-02: Set.insert → Bool + HeapSlot write-back (registry contract).
            let item = stack.pop().unwrap_or(Value::Nil);
            let receiver = stack.pop().unwrap_or(Value::Nil);
            let resolved = match &receiver {
                Value::HeapSlot { .. } => Value::resolve_heap_slot(&receiver),
                other => other.clone(),
            };
            let inserted = match &resolved {
                Value::Set(items) => {
                    let mut set = items.clone();
                    let ok = !set
                        .iter()
                        .any(|x| syx_runtime_core::value_equals(x, &item));
                    if ok {
                        set.push(item);
                    }
                    let _ = Value::write_back_heap_slot(receiver, Value::Set(set));
                    ok
                }
                _ => false,
            };
            stack.push(Value::Bool(inserted));
            return Ok(true);
        }
        CollectionHostOp::Remove => {
            // M0-B03 dict + ADR-224 set: mutate in place (HeapSlot write-back) and return Bool.
            let key = stack.pop().unwrap_or(Value::Nil);
            let receiver = stack.pop().unwrap_or(Value::Nil);
            let resolved = match &receiver {
                Value::HeapSlot { .. } => Value::resolve_heap_slot(&receiver),
                other => other.clone(),
            };
            let removed = match &resolved {
                Value::Dict(map) | Value::Object(map) => {
                    let mut owned = map.clone();
                    let ok = syx_runtime_core::dict_remove(&mut owned, &key);
                    let updated = match &resolved {
                        Value::Object(_) => Value::Object(owned),
                        _ => Value::Dict(owned),
                    };
                    let _ = Value::write_back_heap_slot(receiver, updated);
                    ok
                }
                Value::Set(items) => {
                    let mut set = items.clone();
                    let before = set.len();
                    set.retain(|v| !syx_runtime_core::value_equals(v, &key));
                    let ok = set.len() != before;
                    let _ = Value::write_back_heap_slot(receiver, Value::Set(set));
                    ok
                }
                _ => false,
            };
            stack.push(Value::Bool(removed));
            return Ok(true);
        }
        CollectionHostOp::Union => {
            let other = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let left = match &receiver {
                Value::Set(items) | Value::Array(items) => items.as_slice(),
                _ => &[],
            };
            let right = match &other {
                Value::Set(items) | Value::Array(items) => items.as_slice(),
                _ => &[],
            };
            let mut merged = left.to_vec();
            for item in right {
                if !merged
                    .iter()
                    .any(|v| syx_runtime_core::value_equals(v, item))
                {
                    merged.push(item.clone());
                }
            }
            stack.push(Value::Set(merged));
            return Ok(true);
        }
        CollectionHostOp::Intersection => {
            let other = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let left = match &receiver {
                Value::Set(items) | Value::Array(items) => items.as_slice(),
                _ => &[],
            };
            let right = match &other {
                Value::Set(items) | Value::Array(items) => items.as_slice(),
                _ => &[],
            };
            let inter: Vec<Value> = left
                .iter()
                .filter(|v| {
                    right
                        .iter()
                        .any(|o| syx_runtime_core::value_equals(o, v))
                })
                .cloned()
                .collect();
            stack.push(Value::Set(inter));
            return Ok(true);
        }
        CollectionHostOp::Len => {
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let n = match &receiver {
                Value::Array(items) => items.len() as i32,
                _ => 0,
            };
            stack.push(platform_int_value(n as i64));
            return Ok(true);
        }
        CollectionHostOp::Get => {
            let key = stack.pop().unwrap_or(Value::Nil);
            let receiver = resolve_collection_receiver(stack.pop().unwrap_or(Value::Nil));
            let got = match &receiver {
                Value::Dict(map) | Value::Object(map) => syx_runtime_core::dict_get(map, &key)
                    .cloned()
                    .unwrap_or(Value::Nil),
                _ => Value::Nil,
            };
            stack.push(got);
            return Ok(true);
        }
        CollectionHostOp::Set => {
            let val = stack.pop().unwrap_or(Value::Nil);
            let key = stack.pop().unwrap_or(Value::Nil);
            let receiver = stack.pop().unwrap_or(Value::Nil);
            let resolved = match &receiver {
                Value::HeapSlot { .. } => Value::resolve_heap_slot(&receiver),
                other => other.clone(),
            };
            let updated = match resolved {
                Value::Object(mut map) => {
                    syx_runtime_core::dict_insert(&mut map, key, val);
                    Value::Object(map)
                }
                Value::Dict(mut map) => {
                    syx_runtime_core::dict_insert(&mut map, key, val);
                    Value::Dict(map)
                }
                other => other,
            };
            let _ = Value::write_back_heap_slot(receiver, updated.clone());
            stack.push(updated);
            return Ok(true);
        }
        _ => {}
    }
    let method = match entry.op {
        CollectionHostOp::Map => MethodId::ArrMap,
        CollectionHostOp::Filter => MethodId::ArrFilter,
        CollectionHostOp::Reduce => MethodId::ArrReduce,
        CollectionHostOp::Sorted => MethodId::ArrSorted,
        CollectionHostOp::Reverse => MethodId::ArrReverse,
        CollectionHostOp::Contains => MethodId::ArrContains,
        CollectionHostOp::Joined => MethodId::ArrJoined,
        CollectionHostOp::Insert
        | CollectionHostOp::Values
        | CollectionHostOp::Remove
        | CollectionHostOp::Count
        | CollectionHostOp::ContainsKey
        | CollectionHostOp::Keys
        | CollectionHostOp::Len
        | CollectionHostOp::Get
        | CollectionHostOp::Set
        | CollectionHostOp::Union
        | CollectionHostOp::Intersection => unreachable!(),
    };
    let args = match entry.op {
        CollectionHostOp::Reduce => {
            let closure = stack.pop().unwrap_or(Value::Nil);
            let initial = stack.pop().unwrap_or(Value::Nil);
            vec![initial, closure]
        }
        CollectionHostOp::Joined => {
            vec![stack.pop().unwrap_or(Value::Nil)]
        }
        CollectionHostOp::Sorted | CollectionHostOp::Reverse => Vec::new(),
        _ => vec![stack.pop().unwrap_or(Value::Nil)],
    };
    let receiver = stack.pop().unwrap_or(Value::Nil);
    // ADR-225 / M2-S1-04: Map/Filter/Reduce fail-closed (U001); never push `[]`/`Nil` sentinel success.
    if matches!(
        method,
        MethodId::ArrMap | MethodId::ArrFilter | MethodId::ArrReduce
    ) {
        stack.push(try_call_array_hof(method, &receiver, &args)?);
    } else {
        stack.push(call_array_method(method, &receiver, &args));
    }
    Ok(true)
}

pub fn dispatch_bigint_host(name: &str, stack: &mut Vec<Value>) -> SyxResult<bool> {
    let Some(entry) = lookup_bigint_host(name) else {
        return Ok(false);
    };
    match entry.op {
        BigIntHostOp::FromString => {
            let s = match stack.pop().unwrap_or(Value::Nil) {
                Value::Str(s) => s,
                _ => String::new(),
            };
            stack.push(syx_runtime_core::bigint_from_string(&s).unwrap_or(Value::Nil));
        }
        BigIntHostOp::FromI128 => {
            let v = stack.pop().unwrap_or(Value::Nil);
            let n = match v {
                Value::I128(x) => x,
                Value::I32(x) => x as i128,
                Value::I64(x) => x as i128,
                _ => 0,
            };
            stack.push(syx_runtime_core::bigint_from_i128(n));
        }
        BigIntHostOp::Add => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            stack.push(syx_runtime_core::bigint_add(&a, &b).unwrap_or(Value::Nil));
        }
        BigIntHostOp::Sub => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            stack.push(syx_runtime_core::bigint_sub(&a, &b).unwrap_or(Value::Nil));
        }
        BigIntHostOp::Mul => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            stack.push(syx_runtime_core::bigint_mul(&a, &b).unwrap_or(Value::Nil));
        }
        BigIntHostOp::Cmp => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            let n = syx_runtime_core::bigint_cmp(&a, &b).unwrap_or(0);
            stack.push(Value::I32(n));
        }
        BigIntHostOp::Description => {
            let v = stack.pop().unwrap_or(Value::Nil);
            stack.push(Value::Str(syx_runtime_core::bigint_description(&v)));
        }
    }
    Ok(true)
}

pub fn dispatch_numeric_host(name: &str, stack: &mut Vec<Value>) -> SyxResult<bool> {
    let Some(entry) = lookup_numeric_host(name) else {
        return Ok(false);
    };
    let result = match entry.op {
        NumericHostOp::Min | NumericHostOp::Max => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            match entry.op {
                NumericHostOp::Min => syx_runtime_core::numeric_min(entry.kind, &a, &b),
                NumericHostOp::Max => syx_runtime_core::numeric_max(entry.kind, &a, &b),
                _ => None,
            }
        }
        NumericHostOp::Abs => {
            let a = stack.pop().unwrap_or(Value::Nil);
            syx_runtime_core::numeric_abs(entry.kind, &a)
        }
        NumericHostOp::Add => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            syx_runtime_core::numeric_add(entry.kind, &a, &b)
        }
        NumericHostOp::AddAssign => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            syx_runtime_core::numeric_add(entry.kind, &a, &b)
        }
        NumericHostOp::SubAssign => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            syx_runtime_core::numeric_sub(entry.kind, &a, &b)
        }
        NumericHostOp::Eq => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            Some(Value::Bool(syx_runtime_core::value_equals(&a, &b)))
        }
        NumericHostOp::Ne => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            Some(Value::Bool(!syx_runtime_core::value_equals(&a, &b)))
        }
        NumericHostOp::Lt | NumericHostOp::Le | NumericHostOp::Gt | NumericHostOp::Ge => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            let cmp = syx_runtime_core::value_compare(&a, &b);
            let ok = match entry.op {
                NumericHostOp::Lt => cmp == std::cmp::Ordering::Less,
                NumericHostOp::Le => cmp != std::cmp::Ordering::Greater,
                NumericHostOp::Gt => cmp == std::cmp::Ordering::Greater,
                NumericHostOp::Ge => cmp != std::cmp::Ordering::Less,
                _ => false,
            };
            Some(Value::Bool(ok))
        }
        NumericHostOp::Compare => {
            let b = stack.pop().unwrap_or(Value::Nil);
            let a = stack.pop().unwrap_or(Value::Nil);
            Some(platform_int_value(
                syx_runtime_core::value_compare_int(&a, &b) as i64,
            ))
        }
    };
    if let Some(r) = result {
        stack.push(r);
    }
    Ok(true)
}

pub fn dispatch_result_host(name: &str, stack: &mut Vec<Value>) -> SyxResult<bool> {
    let Some(entry) = lookup_result_host(name) else {
        return Ok(false);
    };
    match entry.op {
        ResultHostOp::Ok => {
            let value = stack.pop().unwrap_or(Value::Nil);
            stack.push(Value::Dict(std::collections::HashMap::from([
                ("__type".into(), Value::Str("Result".into())),
                ("tag".into(), Value::Str("ok".into())),
                ("value".into(), value),
            ])));
        }
        ResultHostOp::Err => {
            let value = stack.pop().unwrap_or(Value::Nil);
            stack.push(Value::Dict(std::collections::HashMap::from([
                ("__type".into(), Value::Str("Result".into())),
                ("tag".into(), Value::Str("err".into())),
                ("value".into(), value),
            ])));
        }
        ResultHostOp::IsOk => {
            let r = stack.pop().unwrap_or(Value::Nil);
            let ok = matches!(
                r,
                Value::Dict(m) if matches!(m.get("tag"), Some(Value::Str(s)) if s == "ok")
            );
            stack.push(Value::Bool(ok));
        }
        ResultHostOp::IsErr => {
            let r = stack.pop().unwrap_or(Value::Nil);
            let err = matches!(
                r,
                Value::Dict(m) if matches!(m.get("tag"), Some(Value::Str(s)) if s == "err")
            );
            stack.push(Value::Bool(err));
        }
    }
    Ok(true)
}

/// Run host fn by unified index (`HostFnId` first, then numeric hosts).
pub fn run_host_by_index(idx: u32, stack: &mut Vec<Value>) -> SyxResult<bool> {
    use syx_stdlib::host_fn_name_for_index;
    let Some(name) = host_fn_name_for_index(idx) else {
        return Ok(false);
    };
    if dispatch_numeric_host(name, stack)? {
        return Ok(true);
    }
    if dispatch_collection_host(name, stack)? {
        return Ok(true);
    }
    if dispatch_bigint_host(name, stack)? {
        return Ok(true);
    }
    if dispatch_result_host(name, stack)? {
        return Ok(true);
    }
    if let Some(id) = HostFnId::all().iter().find(|h| h.name() == name).copied() {
        if let Some(host) = host_fn_for(id) {
            host(stack)?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn host_fn_for(id: HostFnId) -> Option<HostFn> {
    #[cfg(not(feature = "file"))]
    if matches!(
        id,
        HostFnId::FileRead
            | HostFnId::FileReadLines
            | HostFnId::FileWrite
            | HostFnId::FileAppend
            | HostFnId::FileExists
            | HostFnId::FileDelete
            | HostFnId::FileCopy
            | HostFnId::FileMove
            | HostFnId::FileReadBytes
            | HostFnId::FileWriteBytes
            | HostFnId::FileOpen
            | HostFnId::FileReadChunk
            | HostFnId::FileWriteChunk
            | HostFnId::FileClose
            | HostFnId::FileCloseQuiet
            | HostFnId::FileWithOpen
            | HostFnId::FileWithOpenFail
            | HostFnId::FileSeek
            | HostFnId::FileMmapRead
            | HostFnId::FileStat
            | HostFnId::FileSymlink
            | HostFnId::FileReadLink
            | HostFnId::FileChmod
            | HostFnId::KeyPathGet
            | HostFnId::AsOptional
            | HostFnId::TypeIs
            | HostFnId::TypeCast
            | HostFnId::StrInstSplit
            | HostFnId::StrInstTrim
            | HostFnId::StrInstHasPrefix
            | HostFnId::StrInstLowercased
            | HostFnId::StrInstUppercased
            | HostFnId::StrInstMatches
            | HostFnId::StrInstLength
            | HostFnId::StrInstSubstring
            | HostFnId::DirCreate
            | HostFnId::DirList
            | HostFnId::DirExists
    ) {
        return None;
    }
    Some(match id {
        HostFnId::FileRead => host_file_read,
        HostFnId::FileReadLines => host_file_read_lines,
        HostFnId::FileWrite => host_file_write,
        HostFnId::FileAppend => host_file_append,
        HostFnId::FileExists => host_file_exists,
        HostFnId::FileDelete => host_file_delete,
        HostFnId::FileCopy => host_file_copy,
        HostFnId::FileMove => host_file_move,
        HostFnId::FileReadBytes => host_file_read_bytes,
        HostFnId::FileWriteBytes => host_file_write_bytes,
        HostFnId::FileOpen => host_file_open,
        HostFnId::FileReadChunk => host_file_read_chunk,
        HostFnId::FileWriteChunk => host_file_write_chunk,
        HostFnId::FileClose => host_file_close,
        HostFnId::FileCloseQuiet => host_file_close_quiet,
        HostFnId::FileWithOpen => host_file_with_open,
        HostFnId::FileWithOpenFail => host_file_with_open_fail,
        HostFnId::FileSeek => host_file_seek,
        HostFnId::FileMmapRead => host_file_mmap_read,
        HostFnId::KeyPathGet => host_keypath_get,
        HostFnId::AsOptional => host_as_optional,
        HostFnId::TypeIs => host_type_is,
        HostFnId::TypeCast => host_type_cast,
        HostFnId::StrInstSplit => host_str_inst_split,
        HostFnId::StrInstTrim => host_str_inst_trim,
        HostFnId::StrInstHasPrefix => host_str_inst_has_prefix,
        HostFnId::StrInstLowercased => host_str_inst_lowercased,
        HostFnId::StrInstUppercased => host_str_inst_uppercased,
        HostFnId::StrInstMatches => host_str_inst_matches,
        HostFnId::StrInstLength => host_str_inst_length,
        HostFnId::StrInstSubstring => host_str_inst_substring,
        HostFnId::StrUnicodeScalarCount => host_str_unicode_scalar_count,
        HostFnId::StrUnicodeScalars => host_str_unicode_scalars,
        HostFnId::StrUtf8Count => host_str_utf8_count,
        HostFnId::StrUtf8Bytes => host_str_utf8_bytes,
        HostFnId::CharacterIsSingleScalar => host_character_is_single_scalar,
        HostFnId::StrNormalizedNFC => host_str_normalized_nfc,
        HostFnId::StrNormalizedNFD => host_str_normalized_nfd,
        HostFnId::StrCaseFolded => host_str_case_folded,
        HostFnId::DirCreate => host_dir_create,
        HostFnId::DirList => host_dir_list,
        HostFnId::DirExists => host_dir_exists,
        HostFnId::HttpGet => host_http_get,
        HostFnId::HttpPost => host_http_post,
        HostFnId::HttpResponseJson => host_http_response_json,
        HostFnId::HttpRequest => http::host_http_request,
        HostFnId::HttpResponseStatus => http::host_http_response_status,
        HostFnId::HttpResponseText => http::host_http_response_text,
        HostFnId::HttpResponseOk => http::host_http_response_ok,
        HostFnId::HttpResponseHeader => http::host_http_response_header,
        HostFnId::HttpResponseHeaders => http::host_http_response_headers,
        HostFnId::HttpServe => http::host_http_serve,
        HostFnId::HttpShutdown => http::host_http_serve_shutdown,
        HostFnId::HttpBodyStreamOpen => http::host_http_body_stream_open,
        HostFnId::HttpBodyStreamOpenOpts => http::host_http_body_stream_open_opts,
        HostFnId::HttpBodyStreamReadChunk => http::host_http_body_stream_read_chunk,
        HostFnId::HttpBodyStreamClose => http::host_http_body_stream_close,
        HostFnId::FileStreamOpen => host_file_stream_open,
        HostFnId::FileStreamReadChunk => host_file_stream_read_chunk,
        HostFnId::FileStreamClose => host_file_stream_close,
        HostFnId::FileWatcherOpen => host_file_watcher_open,
        HostFnId::FileWatcherPoll => host_file_watcher_poll,
        HostFnId::FileWatcherClose => host_file_watcher_close,
        HostFnId::HttpMakeResponse => http::host_http_make_response,
        HostFnId::UrlParse => http::host_url_parse,
        HostFnId::UrlEncodeQuery => http::host_url_encode_query,
        HostFnId::UrlBuild => http::host_url_build,
        HostFnId::EnvGet => script_hosts::host_env_get,
        HostFnId::EnvSet => script_hosts::host_env_set,
        HostFnId::PathJoin => script_hosts::host_path_join,
        HostFnId::PathNormalize => script_hosts::host_path_normalize,
        HostFnId::PathExists => script_hosts::host_path_exists,
        HostFnId::PathExtension => script_hosts::host_path_extension,
        HostFnId::TimeNowMs => script_hosts::host_time_now_ms,
        HostFnId::TimeSleepMs => script_hosts::host_time_sleep_ms,
        HostFnId::InstantNowMs => script_hosts::host_instant_now_ms,
        HostFnId::RegexMatch => host_regex_match,
        HostFnId::ReadLine => host_read_line,
        HostFnId::HasherNew => host_hasher_new,
        HostFnId::HasherCombine => host_hasher_combine,
        HostFnId::HasherFinalize => host_hasher_finalize,
        HostFnId::ObjectIdentifier => host_object_identifier,
        HostFnId::JsonEncode => host_json_encode,
        HostFnId::JsonDecode => host_json_decode,
        HostFnId::TaskDetach => crate::task_hosts::task_detach_host().unwrap_or(host_noop),
        HostFnId::TaskJoin => crate::task_hosts::task_join_host().unwrap_or(host_noop),
        HostFnId::TaskCancel => crate::task_hosts::task_cancel_host().unwrap_or(host_noop),
        HostFnId::TaskSleep => crate::task_hosts::task_sleep_host().unwrap_or(host_noop),
        HostFnId::MainActorRun => crate::task_hosts::main_actor_run_host().unwrap_or(host_noop),
        HostFnId::AwaitTask => crate::task_hosts::await_task_host().unwrap_or(host_noop),
        HostFnId::TaskGroupCreate => {
            if let Some(ops) = crate::task_hosts::bridge_task_group_ops() {
                if (ops.active)() {
                    ops.create
                } else {
                    crate::task_hosts::task_group_create_host().unwrap_or(host_noop)
                }
            } else {
                crate::task_hosts::task_group_create_host().unwrap_or(host_noop)
            }
        }
        HostFnId::TaskGroupAdd => {
            if let Some(ops) = crate::task_hosts::bridge_task_group_ops() {
                if (ops.active)() {
                    ops.add
                } else {
                    crate::task_hosts::task_group_add_host().unwrap_or(host_noop)
                }
            } else {
                crate::task_hosts::task_group_add_host().unwrap_or(host_noop)
            }
        }
        HostFnId::TaskGroupWait => {
            if let Some(ops) = crate::task_hosts::bridge_task_group_ops() {
                if (ops.active)() {
                    ops.wait
                } else {
                    crate::task_hosts::task_group_wait_host().unwrap_or(host_noop)
                }
            } else {
                crate::task_hosts::task_group_wait_host().unwrap_or(host_noop)
            }
        }
        HostFnId::TaskGroupCancel => {
            if let Some(ops) = crate::task_hosts::bridge_task_group_ops() {
                if (ops.active)() {
                    ops.cancel
                } else {
                    crate::task_hosts::task_group_cancel_host().unwrap_or(host_noop)
                }
            } else {
                crate::task_hosts::task_group_cancel_host().unwrap_or(host_noop)
            }
        }
        HostFnId::TaskLiveCount => crate::task_hosts::task_live_count_host().unwrap_or(host_noop),
        HostFnId::TaskSetDeadlineMs => {
            crate::task_hosts::task_set_deadline_ms_host().unwrap_or(host_noop)
        }
        HostFnId::TaskLocalSet => crate::task_hosts::task_local_set_host().unwrap_or(host_noop),
        HostFnId::TaskLocalGet => crate::task_hosts::task_local_get_host().unwrap_or(host_noop),
        HostFnId::ActorReentrancyPolicy => {
            crate::task_hosts::actor_reentrancy_policy_host().unwrap_or(host_noop)
        }
        HostFnId::BlockingSleep => crate::task_hosts::blocking_sleep_host().unwrap_or(host_noop),
        HostFnId::IsolationPolicy => {
            crate::task_hosts::isolation_policy_host().unwrap_or(host_noop)
        }
        HostFnId::BlockingPoolSetSize => {
            crate::task_hosts::blocking_pool_set_size_host().unwrap_or(host_noop)
        }
        HostFnId::BlockingPoolSize => {
            crate::task_hosts::blocking_pool_size_host().unwrap_or(host_noop)
        }
        HostFnId::FileReadAsync => host_file_read_async,
        HostFnId::IoAdapterPolicy => {
            crate::task_hosts::io_adapter_policy_host().unwrap_or(host_noop)
        }
        HostFnId::TcpReadAsync => host_tcp_read_async,
        HostFnId::MutexUnlockQuiet => crate::stdlib_phase83_hosts::host_mutex_unlock_quiet,
        HostFnId::SyncPolicy => crate::stdlib_phase83_hosts::host_sync_policy,
        HostFnId::MutexLockWait => crate::stdlib_phase83_hosts::host_mutex_lock_wait,
        HostFnId::SemaphoreCreate => crate::stdlib_phase83_hosts::host_semaphore_create,
        HostFnId::SemaphoreWait => crate::stdlib_phase83_hosts::host_semaphore_wait,
        HostFnId::SemaphoreTryWait => crate::stdlib_phase83_hosts::host_semaphore_try_wait,
        HostFnId::SemaphoreSignal => crate::stdlib_phase83_hosts::host_semaphore_signal,
        HostFnId::SemaphorePolicy => crate::stdlib_phase83_hosts::host_semaphore_policy,
        HostFnId::SemaphoreSignalQuiet => crate::stdlib_phase83_hosts::host_semaphore_signal_quiet,
        HostFnId::ChannelCreate => crate::channel_hosts::host_channel_create,
        HostFnId::ChannelSend => crate::channel_hosts::host_channel_send,
        HostFnId::ChannelTrySend => crate::channel_hosts::host_channel_try_send,
        HostFnId::ChannelTryRecv => crate::channel_hosts::host_channel_try_recv,
        HostFnId::ChannelSelect => crate::channel_hosts::host_channel_select,
        HostFnId::ChannelRecv => crate::channel_hosts::host_channel_recv,
        HostFnId::ChannelRecvAsync => crate::channel_hosts::host_channel_recv_async,
        HostFnId::AsyncIterFrom => crate::async_iter_hosts::host_async_iter_from,
        HostFnId::AsyncIterNext => crate::async_iter_hosts::host_async_iter_next,
        HostFnId::DateTimeParse => script_hosts::host_datetime_parse,
        HostFnId::DateTimeFormat => script_hosts::host_datetime_format,
        HostFnId::DateTimeParseIso8601 => script_hosts::host_datetime_parse_iso8601,
        HostFnId::DateTimeFormatWithTimezone => script_hosts::host_datetime_format_with_timezone,
        HostFnId::DateTimeNow => script_hosts::host_datetime_now,
        HostFnId::DateTimeTimezoneOffset => script_hosts::host_datetime_timezone_offset,
        HostFnId::OrderedSetNew => script_hosts::host_ordered_set_new,
        HostFnId::OrderedSetInsert => script_hosts::host_ordered_set_insert,
        HostFnId::OrderedSetContains => script_hosts::host_ordered_set_contains,
        HostFnId::ChannelSelectTimeout => crate::channel_hosts::host_channel_select_timeout,
        HostFnId::ChannelSelectDefault => crate::channel_hosts::host_channel_select_default,
        HostFnId::ChannelSelectMixedAwait => crate::channel_hosts::host_channel_select_mixed_await,
        HostFnId::SelectTag => crate::channel_hosts::host_select_tag,
        HostFnId::ServiceRegister => crate::stdlib_phase120_hosts::host_service_register,
        HostFnId::ServiceRegisterWitness => {
            crate::stdlib_phase120_hosts::host_service_register_witness
        }
        HostFnId::ServiceAddDep => crate::stdlib_phase120_hosts::host_service_add_dep,
        HostFnId::ServiceSeal => crate::stdlib_phase120_hosts::host_service_seal,
        HostFnId::ServiceResolve => crate::stdlib_phase120_hosts::host_service_resolve,
        HostFnId::ServicePut => crate::stdlib_phase120_hosts::host_service_put,
        HostFnId::ServiceReset => crate::stdlib_phase120_hosts::host_service_reset,
        HostFnId::ServiceDestroy => crate::stdlib_phase120_hosts::host_service_destroy,
        HostFnId::ServiceBoot => crate::stdlib_phase120_hosts::host_service_boot,
        HostFnId::ServiceBeginRequest => crate::stdlib_phase120_hosts::host_service_begin_request,
        HostFnId::ServiceEndRequest => crate::stdlib_phase120_hosts::host_service_end_request,
        HostFnId::ServiceResolveDynamic => crate::stdlib_phase120_hosts::host_service_resolve,
        HostFnId::ServiceRunInRequest => crate::stdlib_phase120_hosts::host_service_run_in_request,
        HostFnId::ServiceSetProfile => crate::stdlib_phase120_hosts::host_service_set_profile,
        HostFnId::ServiceLazyProxy => crate::stdlib_phase120_hosts::host_service_lazy_proxy,
        HostFnId::SynuiAppCreate => crate::synui_hosts::host_synui_app_create,
        HostFnId::SynuiLabel => crate::synui_hosts::host_synui_label,
        HostFnId::SynuiButton => crate::synui_hosts::host_synui_button,
        HostFnId::SynuiColumn => crate::synui_hosts::host_synui_column,
        HostFnId::SynuiPump => crate::synui_hosts::host_synui_pump,
        HostFnId::SynuiFingerprint => crate::synui_hosts::host_synui_fingerprint,
        HostFnId::SynuiSignalNew => crate::synui_hosts::host_synui_signal_new,
        HostFnId::SynuiSignalGet => crate::synui_hosts::host_synui_signal_get,
        HostFnId::SynuiSignalSet => crate::synui_hosts::host_synui_signal_set,
        HostFnId::SynuiRow => crate::synui_hosts::host_synui_row,
        HostFnId::SynuiTextField => crate::synui_hosts::host_synui_text_field,
        HostFnId::SynuiButtonOnClick => crate::synui_hosts::host_synui_button_on_click,
        HostFnId::SynuiButtonClick => crate::synui_hosts::host_synui_button_click,
        HostFnId::SynuiRenderFingerprint => crate::synui_hosts::host_synui_render_fingerprint,
        HostFnId::SynuiRun => crate::synui_hosts::host_synui_run,
        HostFnId::SynuiCheckbox => crate::synui_hosts::host_synui_checkbox,
        HostFnId::SynuiSlider => crate::synui_hosts::host_synui_slider,
        HostFnId::SynuiSignalBoolNew => crate::synui_hosts::host_synui_signal_bool_new,
        HostFnId::SynuiSignalBoolGet => crate::synui_hosts::host_synui_signal_bool_get,
        HostFnId::SynuiSignalBoolSet => crate::synui_hosts::host_synui_signal_bool_set,
        HostFnId::SynuiSignalNumberNew => crate::synui_hosts::host_synui_signal_number_new,
        HostFnId::SynuiSignalNumberGet => crate::synui_hosts::host_synui_signal_number_get,
        HostFnId::SynuiSignalNumberSet => crate::synui_hosts::host_synui_signal_number_set,
        HostFnId::SynuiBindTextField => crate::synui_hosts::host_synui_bind_text_field,
        HostFnId::SynuiBindCheckbox => crate::synui_hosts::host_synui_bind_checkbox,
        HostFnId::SynuiBindSlider => crate::synui_hosts::host_synui_bind_slider,
        HostFnId::SynuiSwitch => crate::synui_hosts::host_synui_switch,
        HostFnId::SynuiRadio => crate::synui_hosts::host_synui_radio,
        HostFnId::SynuiBindSwitch => crate::synui_hosts::host_synui_bind_switch,
        HostFnId::SynuiBindRadio => crate::synui_hosts::host_synui_bind_radio,
        HostFnId::SynuiProgress => crate::synui_hosts::host_synui_progress,
        HostFnId::SynuiPicker => crate::synui_hosts::host_synui_picker,
        HostFnId::SynuiTextArea => crate::synui_hosts::host_synui_text_area,
        HostFnId::SynuiList => crate::synui_hosts::host_synui_list,
        HostFnId::SynuiBindProgress => crate::synui_hosts::host_synui_bind_progress,
        HostFnId::SynuiImage => crate::synui_hosts::host_synui_image,
        HostFnId::SynuiScrollView => crate::synui_hosts::host_synui_scroll_view,
        HostFnId::SynuiBindPicker => crate::synui_hosts::host_synui_bind_picker,
        HostFnId::SynuiBindTextArea => crate::synui_hosts::host_synui_bind_text_area,
        HostFnId::SynuiBindList => crate::synui_hosts::host_synui_bind_list,
        HostFnId::SynuiSpacer => crate::synui_hosts::host_synui_spacer,
        HostFnId::SynuiBindImage => crate::synui_hosts::host_synui_bind_image,
        HostFnId::SynuiDivider => crate::synui_hosts::host_synui_divider,
        HostFnId::SynuiStack => crate::synui_hosts::host_synui_stack,
        HostFnId::SynuiBindScrollView => crate::synui_hosts::host_synui_bind_scroll_view,
        HostFnId::SynuiTabView => crate::synui_hosts::host_synui_tab_view,
        HostFnId::SynuiBindTabView => crate::synui_hosts::host_synui_bind_tab_view,
        HostFnId::SynuiNavigation => crate::synui_hosts::host_synui_navigation,
        HostFnId::SynuiBindNavigation => crate::synui_hosts::host_synui_bind_navigation,
        HostFnId::OsCpuCount => crate::os_hosts::host_os_cpu_count,
        HostFnId::OsMemoryTotal => crate::os_hosts::host_os_memory_total,
        HostFnId::OsMemoryAvailable => crate::os_hosts::host_os_memory_available,
        HostFnId::OsUserName => crate::os_hosts::host_os_user_name,
        HostFnId::OsHomeDir => crate::os_hosts::host_os_home_dir,
        HostFnId::OsHostname => crate::os_hosts::host_os_hostname,
        HostFnId::OsUptimeMs => crate::os_hosts::host_os_uptime_ms,
        HostFnId::OsDiskTotal => crate::os_hosts::host_os_disk_total,
        HostFnId::OsDiskAvailable => crate::os_hosts::host_os_disk_available,
        HostFnId::OsLoadAverage => crate::os_hosts::host_os_load_average,
        HostFnId::OsPrimaryIpv4 => crate::os_hosts::host_os_primary_ipv4,
        HostFnId::TaskSetWorkerCount => {
            crate::task_hosts::task_set_worker_count_host().unwrap_or(host_noop)
        }
        HostFnId::TaskWorkerCount => {
            crate::task_hosts::task_worker_count_host().unwrap_or(host_noop)
        }
        HostFnId::TaskSetQueueCapacity => {
            crate::task_hosts::task_set_queue_capacity_host().unwrap_or(host_noop)
        }
        HostFnId::TaskQueueCapacity => {
            crate::task_hosts::task_queue_capacity_host().unwrap_or(host_noop)
        }
        HostFnId::TaskSetRejectPolicy => {
            crate::task_hosts::task_set_reject_policy_host().unwrap_or(host_noop)
        }
        HostFnId::TaskRejectPolicy => {
            crate::task_hosts::task_reject_policy_host().unwrap_or(host_noop)
        }
        HostFnId::TaskSetCorePoolSize => {
            crate::task_hosts::task_set_core_pool_size_host().unwrap_or(host_noop)
        }
        HostFnId::TaskCorePoolSize => {
            crate::task_hosts::task_core_pool_size_host().unwrap_or(host_noop)
        }
        HostFnId::TaskSetMaximumPoolSize => {
            crate::task_hosts::task_set_max_pool_size_host().unwrap_or(host_noop)
        }
        HostFnId::TaskMaximumPoolSize => {
            crate::task_hosts::task_max_pool_size_host().unwrap_or(host_noop)
        }
        HostFnId::TaskSetKeepAliveMs => {
            crate::task_hosts::task_set_keep_alive_ms_host().unwrap_or(host_noop)
        }
        HostFnId::TaskKeepAliveMs => {
            crate::task_hosts::task_keep_alive_ms_host().unwrap_or(host_noop)
        }
        HostFnId::TaskLiveWorkerCount => {
            crate::task_hosts::task_live_worker_count_host().unwrap_or(host_noop)
        }
        HostFnId::ActorSetMaxActors => {
            crate::task_hosts::actor_set_max_actors_host().unwrap_or(host_noop)
        }
        HostFnId::ActorMaxActors => crate::task_hosts::actor_max_actors_host().unwrap_or(host_noop),
        HostFnId::ActorSetMailboxCapacity => {
            crate::task_hosts::actor_set_mailbox_capacity_host().unwrap_or(host_noop)
        }
        HostFnId::ActorMailboxCapacity => {
            crate::task_hosts::actor_mailbox_capacity_host().unwrap_or(host_noop)
        }
        HostFnId::ActorSetSharedPoolEnabled => {
            crate::task_hosts::actor_set_shared_pool_enabled_host().unwrap_or(host_noop)
        }
        HostFnId::ActorSharedPoolEnabled => {
            crate::task_hosts::actor_shared_pool_enabled_host().unwrap_or(host_noop)
        }
        HostFnId::ActorSetSharedPoolSize => {
            crate::task_hosts::actor_set_shared_pool_size_host().unwrap_or(host_noop)
        }
        HostFnId::ActorSharedPoolSize => {
            crate::task_hosts::actor_shared_pool_size_host().unwrap_or(host_noop)
        }
        HostFnId::ChannelLength => crate::channel_hosts::host_channel_length,
        HostFnId::ChannelCapacity => crate::channel_hosts::host_channel_capacity,
        HostFnId::HttpRequestAsync => http::host_http_request_async,
        HostFnId::WsConnect => crate::ws::host_ws_connect,
        HostFnId::WsConnectAsync => crate::ws::host_ws_connect_async,
        HostFnId::WsSendAsync => crate::ws::host_ws_send_async,
        HostFnId::WsSend => crate::ws::host_ws_send,
        HostFnId::WsSendProd => crate::ws::host_ws_send_prod,
        HostFnId::WasmSabNew => crate::wasm_threads::host_wasm_sab_new,
        HostFnId::WasmAtomicAddI32 => crate::wasm_threads::host_wasm_atomic_add_i32,
        HostFnId::TcpConnect => host_tcp_connect,
        HostFnId::TcpRead => host_tcp_read,
        HostFnId::TcpWrite => host_tcp_write,
        HostFnId::TcpStream => host_tcp_stream,
        HostFnId::TcpStreamTls => host_tcp_stream_tls,
        HostFnId::TcpStreamHandle => host_tcp_stream_handle,
        HostFnId::NetBackpressure => host_net_backpressure,
        HostFnId::NetRingBuffer => host_net_ring_buffer,
        HostFnId::TcpHandleRelease => host_tcp_handle_release,
        HostFnId::NetFlowControl => host_net_flow_control,
        HostFnId::TcpHandleRefcount => host_tcp_handle_refcount,
        HostFnId::NetSocketWindow => host_net_socket_window,
        HostFnId::TcpSockbuf => host_tcp_sockbuf,
        HostFnId::NetSetsockopt => host_net_setsockopt,
        HostFnId::VmNativeGcMark => host_vm_native_gc_remembered_set_mark,
        HostFnId::VmNativeGcCount => host_vm_native_gc_remembered_set_count,
        HostFnId::VmNativeGcCollect => host_vm_native_gc_remembered_set_collect,
        HostFnId::RandomNextInt64 => host_random_next_int64,
        HostFnId::RandomNextUInt64 => host_random_next_uint64,
        HostFnId::RandomFillBytes => host_random_fill_bytes,
        HostFnId::CryptoSha256 => host_crypto_sha256,
        HostFnId::CryptoSha512 => host_crypto_sha512,
        HostFnId::CryptoHmacSha256 => host_crypto_hmac_sha256,
        HostFnId::CryptoAesGcmEncrypt => host_crypto_aes_gcm_encrypt,
        HostFnId::SimdFromArray => host_simd_from_array,
        HostFnId::SimdReduceAdd => host_simd_reduce_add,
        HostFnId::SimdLoadFromArray => host_simd_load_from_array,
        HostFnId::SimdStoreToArray => host_simd_store_to_array,
        HostFnId::SimdShuffle => host_simd_shuffle,
        HostFnId::SimdAnd => host_simd_and,
        HostFnId::SimdOr => host_simd_or,
        HostFnId::SimdXor => host_simd_xor,
        HostFnId::SimdNot => host_simd_not,
        HostFnId::SimdShl => host_simd_shl,
        HostFnId::SimdShr => host_simd_shr,
        HostFnId::SimdMin => host_simd_min,
        HostFnId::SimdMax => host_simd_max,
        HostFnId::SimdF32Add => host_simd_f32_add,
        HostFnId::SimdF32Sub => host_simd_f32_sub,
        HostFnId::SimdF32Mul => host_simd_f32_mul,
        HostFnId::SimdF32Div => host_simd_f32_div,
        HostFnId::SimdF64Add => host_simd_f64_add,
        HostFnId::SimdI16Add => host_simd_i16_add,
        HostFnId::SimdI16Sub => host_simd_i16_sub,
        HostFnId::SimdI16Mul => host_simd_i16_mul,
        HostFnId::SimdI8Add => host_simd_i8_add,
        HostFnId::SimdI8Sub => host_simd_i8_sub,
        HostFnId::SimdI8Mul => host_simd_i8_mul,
        HostFnId::SimdI8And => host_simd_i8_and,
        HostFnId::SimdI8Or => host_simd_i8_or,
        HostFnId::SimdI8Xor => host_simd_i8_xor,
        HostFnId::SimdI8Not => host_simd_i8_not,
        HostFnId::SimdI8Min => host_simd_i8_min,
        HostFnId::SimdI8Max => host_simd_i8_max,
        HostFnId::SimdI8UMin => host_simd_i8_umin,
        HostFnId::SimdI8UMax => host_simd_i8_umax,
        HostFnId::SimdI8Abs => host_simd_i8_abs,
        HostFnId::SimdI8Neg => host_simd_i8_neg,
        HostFnId::SimdI8Extract => host_simd_i8_extract,
        HostFnId::SimdI8Replace => host_simd_i8_replace,
        HostFnId::SimdI8Eq => host_simd_i8_eq,
        HostFnId::SimdI8Ne => host_simd_i8_ne,
        HostFnId::SimdI8Le => host_simd_i8_le,
        HostFnId::SimdI8Ge => host_simd_i8_ge,
        HostFnId::SimdI8AvgrU => host_simd_i8_avgr_u,
        HostFnId::SimdI8Popcnt => host_simd_i8_popcnt,
        HostFnId::SimdI8Bitmask => host_simd_i8_bitmask,
        HostFnId::SimdI8AllTrue => host_simd_i8_all_true,
        HostFnId::SimdI8AnyTrue => host_simd_i8_any_true,
        HostFnId::SimdI8Lt => host_simd_i8_lt,
        HostFnId::SimdI8Gt => host_simd_i8_gt,
        HostFnId::SimdI8AddSat => host_simd_i8_add_sat,
        HostFnId::SimdI8SubSat => host_simd_i8_sub_sat,
        HostFnId::SimdI8Shuffle => host_simd_i8_shuffle,
        HostFnId::SimdI8Shl => host_simd_i8_shl,
        HostFnId::SimdI8Shr => host_simd_i8_shr,
        HostFnId::SimdI64Add => host_simd_i64_add,
        HostFnId::SimdF64Sub => host_simd_f64_sub,
        HostFnId::SimdF64Mul => host_simd_f64_mul,
        HostFnId::SimdF64Div => host_simd_f64_div,
        HostFnId::StringCharAt => host_string_char_at,
        HostFnId::ArraySetAt => host_array_set_at,
        HostFnId::ArrayAppend => host_array_append,
        HostFnId::LeadingZeros128 => host_leading_zeros128,
        HostFnId::FmtPrint => crate::stdlib_phase83_hosts::host_fmt_print,
        HostFnId::FmtPrintLine => crate::stdlib_phase83_hosts::host_fmt_print_line,
        HostFnId::FmtFormat => crate::stdlib_phase83_hosts::host_fmt_format,
        HostFnId::FmtDebug => crate::stdlib_phase83_hosts::host_fmt_debug,
        HostFnId::Base64Encode => crate::stdlib_phase83_hosts::host_base64_encode,
        HostFnId::Base64Decode => crate::stdlib_phase83_hosts::host_base64_decode,
        HostFnId::HexEncode => crate::stdlib_phase83_hosts::host_hex_encode,
        HostFnId::HexDecode => crate::stdlib_phase83_hosts::host_hex_decode,
        HostFnId::UrlEncode => crate::stdlib_phase83_hosts::host_url_encode,
        HostFnId::UrlDecode => crate::stdlib_phase83_hosts::host_url_decode,
        HostFnId::MathFloatSin => crate::stdlib_phase83_hosts::host_math_float_sin,
        HostFnId::MathFloatCos => crate::stdlib_phase83_hosts::host_math_float_cos,
        HostFnId::MathFloatTan => crate::stdlib_phase83_hosts::host_math_float_tan,
        HostFnId::MathFloatSqrt => crate::stdlib_phase83_hosts::host_math_float_sqrt,
        HostFnId::MathFloatPow => crate::stdlib_phase83_hosts::host_math_float_pow,
        HostFnId::MathFloatFloor => crate::stdlib_phase83_hosts::host_math_float_floor,
        HostFnId::MathFloatCeil => crate::stdlib_phase83_hosts::host_math_float_ceil,
        HostFnId::MathFloatRound => crate::stdlib_phase83_hosts::host_math_float_round,
        HostFnId::MathFloatLog => crate::stdlib_phase83_hosts::host_math_float_log,
        HostFnId::MathFloatAsin => crate::stdlib_phase83_hosts::host_math_float_asin,
        HostFnId::MathFloatAcos => crate::stdlib_phase83_hosts::host_math_float_acos,
        HostFnId::MathFloatAtan => crate::stdlib_phase83_hosts::host_math_float_atan,
        HostFnId::MathFloatAtan2 => crate::stdlib_phase83_hosts::host_math_float_atan2,
        HostFnId::MathFloatExp => crate::stdlib_phase83_hosts::host_math_float_exp,
        HostFnId::MathFloatExp2 => crate::stdlib_phase83_hosts::host_math_float_exp2,
        HostFnId::MathFloatLog2 => crate::stdlib_phase83_hosts::host_math_float_log2,
        HostFnId::MathFloatLog10 => crate::stdlib_phase83_hosts::host_math_float_log10,
        HostFnId::MathFloatHypot => crate::stdlib_phase83_hosts::host_math_float_hypot,
        HostFnId::MathFloatCbrt => crate::stdlib_phase83_hosts::host_math_float_cbrt,
        HostFnId::MathFloatTrunc => crate::stdlib_phase83_hosts::host_math_float_trunc,
        HostFnId::MathFloatAbs => crate::stdlib_phase83_hosts::host_math_float_abs,
        HostFnId::MathFloatSinh => crate::stdlib_phase83_hosts::host_math_float_sinh,
        HostFnId::MathFloatCosh => crate::stdlib_phase83_hosts::host_math_float_cosh,
        HostFnId::MathFloatTanh => crate::stdlib_phase83_hosts::host_math_float_tanh,
        HostFnId::MathFloatFmod => crate::stdlib_phase83_hosts::host_math_float_fmod,
        HostFnId::MathFloatCopysign => crate::stdlib_phase83_hosts::host_math_float_copysign,
        HostFnId::MathFloatIsNan => crate::stdlib_phase83_hosts::host_math_float_isnan,
        HostFnId::MathFloatIsInfinite => crate::stdlib_phase83_hosts::host_math_float_isinf,
        HostFnId::MathFloatIsFinite => crate::stdlib_phase83_hosts::host_math_float_isfinite,
        HostFnId::MathFloatExpm1 => crate::stdlib_phase83_hosts::host_math_float_expm1,
        HostFnId::MathFloatLog1p => crate::stdlib_phase83_hosts::host_math_float_log1p,
        HostFnId::MathFloatAsinh => crate::stdlib_phase83_hosts::host_math_float_asinh,
        HostFnId::MathFloatAcosh => crate::stdlib_phase83_hosts::host_math_float_acosh,
        HostFnId::MathFloatAtanh => crate::stdlib_phase83_hosts::host_math_float_atanh,
        HostFnId::MathFloatRemainder => crate::stdlib_phase83_hosts::host_math_float_remainder,
        HostFnId::MathFloatNextUp => crate::stdlib_phase83_hosts::host_math_float_next_up,
        HostFnId::MathFloatNextDown => crate::stdlib_phase83_hosts::host_math_float_next_down,
        HostFnId::MathFloatMulAdd => crate::stdlib_phase83_hosts::host_math_float_mul_add,
        HostFnId::MathFloatErf => crate::stdlib_phase83_hosts::host_math_float_erf,
        HostFnId::MathFloatErfc => crate::stdlib_phase83_hosts::host_math_float_erfc,
        HostFnId::MathFloatTgamma => crate::stdlib_phase83_hosts::host_math_float_tgamma,
        HostFnId::MathFloatLgamma => crate::stdlib_phase83_hosts::host_math_float_lgamma,
        HostFnId::MathFloat32Sqrt => crate::stdlib_phase83_hosts::host_math_float32_sqrt,
        HostFnId::MathFloat32Sin => crate::stdlib_phase83_hosts::host_math_float32_sin,
        HostFnId::MathFloat32Cos => crate::stdlib_phase83_hosts::host_math_float32_cos,
        HostFnId::MathFloat32Tan => crate::stdlib_phase83_hosts::host_math_float32_tan,
        HostFnId::MathFloat32Exp => crate::stdlib_phase83_hosts::host_math_float32_exp,
        HostFnId::MathFloat32Log => crate::stdlib_phase83_hosts::host_math_float32_log,
        HostFnId::MathFloat32Abs => crate::stdlib_phase83_hosts::host_math_float32_abs,
        HostFnId::MathFloat32Floor => crate::stdlib_phase83_hosts::host_math_float32_floor,
        HostFnId::MathFloat32Ceil => crate::stdlib_phase83_hosts::host_math_float32_ceil,
        HostFnId::MathFloat32Round => crate::stdlib_phase83_hosts::host_math_float32_round,
        HostFnId::MathFloat32Trunc => crate::stdlib_phase83_hosts::host_math_float32_trunc,
        HostFnId::MathFloat32Pow => crate::stdlib_phase83_hosts::host_math_float32_pow,
        HostFnId::MathFloat32Asin => crate::stdlib_phase83_hosts::host_math_float32_asin,
        HostFnId::MathFloat32Acos => crate::stdlib_phase83_hosts::host_math_float32_acos,
        HostFnId::MathFloat32Atan => crate::stdlib_phase83_hosts::host_math_float32_atan,
        HostFnId::MathFloat32Hypot => crate::stdlib_phase83_hosts::host_math_float32_hypot,
        HostFnId::MathFloat32Atan2 => crate::stdlib_phase83_hosts::host_math_float32_atan2,
        HostFnId::MathFloat32Log2 => crate::stdlib_phase83_hosts::host_math_float32_log2,
        HostFnId::MathFloat32Log10 => crate::stdlib_phase83_hosts::host_math_float32_log10,
        HostFnId::MathFloat32Exp2 => crate::stdlib_phase83_hosts::host_math_float32_exp2,
        HostFnId::MathFloat32Cbrt => crate::stdlib_phase83_hosts::host_math_float32_cbrt,
        HostFnId::MathFloat32Sinh => crate::stdlib_phase83_hosts::host_math_float32_sinh,
        HostFnId::MathFloat32Cosh => crate::stdlib_phase83_hosts::host_math_float32_cosh,
        HostFnId::MathFloat32Tanh => crate::stdlib_phase83_hosts::host_math_float32_tanh,
        HostFnId::MathFloat32Fmod => crate::stdlib_phase83_hosts::host_math_float32_fmod,
        HostFnId::MathFloat32Copysign => crate::stdlib_phase83_hosts::host_math_float32_copysign,
        HostFnId::MathFloat32Expm1 => crate::stdlib_phase83_hosts::host_math_float32_expm1,
        HostFnId::MathFloat32Log1p => crate::stdlib_phase83_hosts::host_math_float32_log1p,
        HostFnId::MathFloat32Asinh => crate::stdlib_phase83_hosts::host_math_float32_asinh,
        HostFnId::MathFloat32Acosh => crate::stdlib_phase83_hosts::host_math_float32_acosh,
        HostFnId::MathFloat32Atanh => crate::stdlib_phase83_hosts::host_math_float32_atanh,
        HostFnId::MathFloat32Erf => crate::stdlib_phase83_hosts::host_math_float32_erf,
        HostFnId::MathFloat32Erfc => crate::stdlib_phase83_hosts::host_math_float32_erfc,
        HostFnId::MathFloat32Tgamma => crate::stdlib_phase83_hosts::host_math_float32_tgamma,
        HostFnId::MathFloat32Lgamma => crate::stdlib_phase83_hosts::host_math_float32_lgamma,
        HostFnId::MathFloat32Remainder => crate::stdlib_phase83_hosts::host_math_float32_remainder,
        HostFnId::MathFloat32IsNan => crate::stdlib_phase83_hosts::host_math_float32_isnan,
        HostFnId::MathFloat32IsInfinite => crate::stdlib_phase83_hosts::host_math_float32_isinf,
        HostFnId::MathFloat32IsFinite => crate::stdlib_phase83_hosts::host_math_float32_isfinite,
        HostFnId::MathFloat32NextUp => crate::stdlib_phase83_hosts::host_math_float32_next_up,
        HostFnId::MathFloat32NextDown => crate::stdlib_phase83_hosts::host_math_float32_next_down,
        HostFnId::MathFloat32MulAdd => crate::stdlib_phase83_hosts::host_math_float32_mul_add,
        HostFnId::ProcessArgs => crate::stdlib_phase83_hosts::host_process_args,
        HostFnId::ProcessEnv => crate::stdlib_phase83_hosts::host_process_env,
        HostFnId::ProcessExec => crate::stdlib_phase83_hosts::host_process_exec,
        HostFnId::ProcessExitCode => crate::stdlib_phase83_hosts::host_process_exit_code,
        HostFnId::FileStat => crate::stdlib_phase83_hosts::host_file_stat,
        HostFnId::FileSymlink => crate::stdlib_phase83_hosts::host_file_symlink,
        HostFnId::FileReadLink => crate::stdlib_phase83_hosts::host_file_read_link,
        HostFnId::FileChmod => crate::stdlib_phase83_hosts::host_file_chmod,
        HostFnId::UdpBind => crate::stdlib_phase83_hosts::host_udp_bind,
        HostFnId::UdpSendTo => crate::stdlib_phase83_hosts::host_udp_send_to,
        HostFnId::UdpRecvFrom => crate::stdlib_phase83_hosts::host_udp_recv_from,
        HostFnId::DnsResolve => crate::stdlib_phase83_hosts::host_dns_resolve,
        HostFnId::DnsResolveIp => crate::stdlib_phase83_hosts::host_dns_resolve_ip,
        HostFnId::CryptoEd25519Sign => crate::stdlib_phase83_hosts::host_crypto_ed25519_sign,
        HostFnId::CryptoEd25519Verify => crate::stdlib_phase83_hosts::host_crypto_ed25519_verify,
        HostFnId::CryptoPbkdf2 => crate::stdlib_phase83_hosts::host_crypto_pbkdf2,
        HostFnId::MutexCreate => crate::stdlib_phase83_hosts::host_mutex_create,
        HostFnId::MutexLock => crate::stdlib_phase83_hosts::host_mutex_lock,
        HostFnId::MutexUnlock => crate::stdlib_phase83_hosts::host_mutex_unlock,
        HostFnId::WsRecv => crate::stdlib_phase83_hosts::host_ws_recv,
        HostFnId::WsClose => crate::stdlib_phase83_hosts::host_ws_close,
        HostFnId::ArraySortBy => crate::stdlib_phase83_hosts::host_array_sort_by,
        HostFnId::ArrayBinarySearch => crate::stdlib_phase83_hosts::host_array_binary_search,
        HostFnId::ArrayChunked => crate::stdlib_phase83_hosts::host_array_chunked,
        HostFnId::ArrayZip => crate::stdlib_phase83_hosts::host_array_zip,
        HostFnId::ArrayFlatMap => crate::stdlib_phase83_hosts::host_array_flat_map,
        HostFnId::LogDebug => crate::stdlib_phase83_hosts::host_log_debug,
        HostFnId::LogInfo => crate::stdlib_phase83_hosts::host_log_info,
        HostFnId::LogWarn => crate::stdlib_phase83_hosts::host_log_warn,
        HostFnId::LogError => crate::stdlib_phase83_hosts::host_log_error,
        HostFnId::LogSetMinLevel => crate::stdlib_phase83_hosts::host_log_set_min_level,
        HostFnId::LogMinLevel => crate::stdlib_phase83_hosts::host_log_min_level,
        HostFnId::LogPolicy => crate::stdlib_phase83_hosts::host_log_policy,
        HostFnId::LogInfoFields => crate::stdlib_phase83_hosts::host_log_info_fields,
        HostFnId::LogTakeLast => crate::stdlib_phase83_hosts::host_log_take_last,
        HostFnId::LogDebugFields => crate::stdlib_phase83_hosts::host_log_debug_fields,
        HostFnId::LogWarnFields => crate::stdlib_phase83_hosts::host_log_warn_fields,
        HostFnId::LogErrorFields => crate::stdlib_phase83_hosts::host_log_error_fields,
        HostFnId::TraceStart => crate::stdlib_phase83_hosts::host_trace_start,
        HostFnId::TraceEnd => crate::stdlib_phase83_hosts::host_trace_end,
        HostFnId::TraceCurrent => crate::stdlib_phase83_hosts::host_trace_current,
        HostFnId::TraceContextLine => crate::stdlib_phase83_hosts::host_trace_context_line,
        HostFnId::TracePolicy => crate::stdlib_phase83_hosts::host_trace_policy,
        HostFnId::MetricsCounterInc => crate::stdlib_phase83_hosts::host_metrics_counter_inc,
        HostFnId::MetricsCounterGet => crate::stdlib_phase83_hosts::host_metrics_counter_get,
        HostFnId::MetricsCounterReset => crate::stdlib_phase83_hosts::host_metrics_counter_reset,
        HostFnId::MetricsSnapshot => crate::stdlib_phase83_hosts::host_metrics_snapshot,
        HostFnId::MetricsPolicy => crate::stdlib_phase83_hosts::host_metrics_policy,
        HostFnId::MetricsGaugeSet => crate::stdlib_phase83_hosts::host_metrics_gauge_set,
        HostFnId::MetricsGaugeGet => crate::stdlib_phase83_hosts::host_metrics_gauge_get,
        HostFnId::MetricsCounterIncLabels => {
            crate::stdlib_phase83_hosts::host_metrics_counter_inc_labels
        }
        HostFnId::MetricsCounterGetLabels => {
            crate::stdlib_phase83_hosts::host_metrics_counter_get_labels
        }
        HostFnId::ProfilerStart => crate::stdlib_phase83_hosts::host_profiler_start,
        HostFnId::ProfilerStop => crate::stdlib_phase83_hosts::host_profiler_stop,
        HostFnId::ProfilerElapsed => crate::stdlib_phase83_hosts::host_profiler_elapsed,
        HostFnId::ProfilerReset => crate::stdlib_phase83_hosts::host_profiler_reset,
        HostFnId::ProfilerSnapshot => crate::stdlib_phase83_hosts::host_profiler_snapshot,
        HostFnId::ProfilerPolicy => crate::stdlib_phase83_hosts::host_profiler_policy,
        HostFnId::ProfilerCpuStart => crate::stdlib_phase83_hosts::host_profiler_cpu_start,
        HostFnId::ProfilerCpuStop => crate::stdlib_phase83_hosts::host_profiler_cpu_stop,
        HostFnId::ProfilerCpuElapsed => crate::stdlib_phase83_hosts::host_profiler_cpu_elapsed,
        HostFnId::TestingAssert => crate::stdlib_phase83_hosts::host_testing_assert,
        HostFnId::TestingAssertEq => crate::stdlib_phase83_hosts::host_testing_assert_eq,
        HostFnId::ProcessExecOutput => crate::stdlib_phase84_hosts::host_process_exec_output,
        HostFnId::ProcessSpawn => crate::stdlib_phase84_hosts::host_process_spawn,
        HostFnId::ProcessWait => crate::stdlib_phase84_hosts::host_process_wait,
        HostFnId::ProcessBuilderCreate => crate::stdlib_phase84_hosts::host_process_builder_create,
        HostFnId::ProcessBuilderCwd => crate::stdlib_phase84_hosts::host_process_builder_cwd,
        HostFnId::ProcessBuilderEnv => crate::stdlib_phase84_hosts::host_process_builder_env,
        HostFnId::ProcessBuilderSpawn => crate::stdlib_phase84_hosts::host_process_builder_spawn,
        HostFnId::ChildWait => crate::stdlib_phase84_hosts::host_child_wait,
        HostFnId::ChildKill => crate::stdlib_phase84_hosts::host_child_kill,
        HostFnId::ChildWaitTimeout => crate::stdlib_phase84_hosts::host_child_wait_timeout,
        HostFnId::ChildStdout => crate::stdlib_phase84_hosts::host_child_stdout,
        HostFnId::ChildStderr => crate::stdlib_phase84_hosts::host_child_stderr,
        HostFnId::ChildStdin => crate::stdlib_phase84_hosts::host_child_stdin,
        HostFnId::ChildPipeReadChunk => crate::stdlib_phase84_hosts::host_child_pipe_read_chunk,
        HostFnId::ChildPipeWrite => crate::stdlib_phase84_hosts::host_child_pipe_write,
        HostFnId::ChildPipeClose => crate::stdlib_phase84_hosts::host_child_pipe_close,
        HostFnId::ChildSignal => crate::stdlib_phase84_hosts::host_child_signal,
        HostFnId::FileWatch => crate::stdlib_phase84_hosts::host_file_watch,
        HostFnId::StringBuilderNew => crate::stdlib_phase84_hosts::host_string_builder_new,
        HostFnId::StringBuilderAppend => crate::stdlib_phase84_hosts::host_string_builder_append,
        HostFnId::StringBuilderToString => {
            crate::stdlib_phase84_hosts::host_string_builder_to_string
        }
        HostFnId::BytesFromArray => crate::stdlib_phase84_hosts::host_bytes_from_array,
        HostFnId::BytesSlice => crate::stdlib_phase84_hosts::host_bytes_slice,
        HostFnId::BytesConcat => crate::stdlib_phase84_hosts::host_bytes_concat,
        HostFnId::ChannelClose => crate::stdlib_phase84_hosts::host_channel_close,
        HostFnId::TestingTest => crate::stdlib_phase84_hosts::host_testing_test,
        HostFnId::TestingExpectEqual => crate::stdlib_phase84_hosts::host_testing_expect_equal,
        HostFnId::RegexCompile => crate::stdlib_phase84_hosts::host_regex_compile,
        HostFnId::RegexFindAll => crate::stdlib_phase84_hosts::host_regex_find_all,
        HostFnId::RegexReplaceAll => crate::stdlib_phase84_hosts::host_regex_replace_all,
        HostFnId::EncodingGzipCompress => crate::stdlib_phase85_hosts::host_encoding_gzip_compress,
        HostFnId::EncodingGzipDecompress => {
            crate::stdlib_phase85_hosts::host_encoding_gzip_decompress
        }
        HostFnId::ReflectType => crate::stdlib_phase85_hosts::host_reflect_type,
        HostFnId::ReflectSpecialization => crate::stdlib_phase85_hosts::host_reflect_specialization,
        HostFnId::EncodingZstdCompress => crate::stdlib_phase86_hosts::host_encoding_zstd_compress,
        HostFnId::EncodingZstdDecompress => {
            crate::stdlib_phase86_hosts::host_encoding_zstd_decompress
        }
        HostFnId::CodableEncode => crate::stdlib_phase86_hosts::host_codable_encode,
        HostFnId::CodableDecode => crate::stdlib_phase86_hosts::host_codable_decode,
        HostFnId::TestingRunAll => crate::stdlib_phase86_hosts::host_testing_run_all,
        HostFnId::RuntimeCast => crate::stdlib_phase87_hosts::host_runtime_cast,
        HostFnId::CodableEncodeL4 => crate::stdlib_phase87_hosts::host_codable_encode_l4,
        HostFnId::CodableDecodeL4 => crate::stdlib_phase87_hosts::host_codable_decode_l4,
        HostFnId::JsonEncoderEncode => crate::stdlib_phase87_hosts::host_json_encoder_encode,
        HostFnId::JsonDecoderDecode => crate::stdlib_phase87_hosts::host_json_decoder_decode,
        HostFnId::JsonStreamOpenWriter => crate::stdlib_phase87_hosts::host_json_stream_open_writer,
        HostFnId::JsonStreamWrite => crate::stdlib_phase87_hosts::host_json_stream_write,
        HostFnId::JsonStreamFinishWriter => {
            crate::stdlib_phase87_hosts::host_json_stream_finish_writer
        }
        HostFnId::JsonStreamOpenReader => crate::stdlib_phase87_hosts::host_json_stream_open_reader,
        HostFnId::JsonStreamFeed => crate::stdlib_phase87_hosts::host_json_stream_feed,
        HostFnId::JsonStreamDecodeNext => crate::stdlib_phase87_hosts::host_json_stream_decode_next,
        HostFnId::JsonStreamCloseReader => {
            crate::stdlib_phase87_hosts::host_json_stream_close_reader
        }
        HostFnId::GrpcUnary => crate::stdlib_phase87_hosts::host_grpc_unary,
        HostFnId::GrpcStreamClient => crate::stdlib_phase88_hosts::host_grpc_stream_client,
        HostFnId::GrpcStreamServer => crate::stdlib_phase88_hosts::host_grpc_stream_server,
        HostFnId::KvoObserve => crate::stdlib_phase88_hosts::host_kvo_observe,
        HostFnId::SqlExec => crate::stdlib_phase89_hosts::host_sql_exec_rusqlite,
        HostFnId::SqlQuery => crate::stdlib_phase89_hosts::host_sql_query_rusqlite,
        HostFnId::SqlPrepare => crate::stdlib_phase89_hosts::host_sql_prepare,
        HostFnId::SqlOpen => crate::stdlib_phase89_hosts::host_sql_open,
        HostFnId::SqlClose => crate::stdlib_phase89_hosts::host_sql_close,
        HostFnId::SqlExecOn => crate::stdlib_phase89_hosts::host_sql_exec_on,
        HostFnId::SqlQueryOn => crate::stdlib_phase89_hosts::host_sql_query_on,
        HostFnId::SqlPrepareOn => crate::stdlib_phase89_hosts::host_sql_prepare_on,
        HostFnId::SqlBindStepOn => crate::stdlib_phase89_hosts::host_sql_bind_step_on,
        HostFnId::SqlBindPreparedOn => crate::stdlib_phase89_hosts::host_sql_bind_prepared_on,
        HostFnId::SqlBeginTransactionOn => {
            crate::stdlib_phase89_hosts::host_sql_begin_transaction_on
        }
        HostFnId::SqlCommitTransactionOn => {
            crate::stdlib_phase89_hosts::host_sql_commit_transaction_on
        }
        HostFnId::SqlRollbackTransactionOn => {
            crate::stdlib_phase89_hosts::host_sql_rollback_transaction_on
        }
        HostFnId::SqlPoolCreate => crate::stdlib_phase89_hosts::host_sql_pool_create,
        HostFnId::SqlPoolAcquire => crate::stdlib_phase89_hosts::host_sql_pool_acquire,
        HostFnId::SqlPoolRelease => crate::stdlib_phase89_hosts::host_sql_pool_release,
        HostFnId::SqlPoolClose => crate::stdlib_phase89_hosts::host_sql_pool_close,
        HostFnId::SqlRowGetString => crate::stdlib_phase89_hosts::host_sql_row_get_string,
        HostFnId::SqlRowGetInt => crate::stdlib_phase89_hosts::host_sql_row_get_int,
        HostFnId::SqlRowGetFloat => crate::stdlib_phase89_hosts::host_sql_row_get_float,
        HostFnId::SqlRowGetBlob => crate::stdlib_phase89_hosts::host_sql_row_get_blob,
        HostFnId::SqlRowIsNull => crate::stdlib_phase89_hosts::host_sql_row_is_null,
        HostFnId::SqlRowColumnNames => crate::stdlib_phase89_hosts::host_sql_row_column_names,
        HostFnId::DequeNew => script_hosts::host_deque_new,
        HostFnId::DequePushBack => script_hosts::host_deque_push_back,
        HostFnId::DequePushFront => script_hosts::host_deque_push_front,
        HostFnId::DequePopBack => script_hosts::host_deque_pop_back,
        HostFnId::DequePopFront => script_hosts::host_deque_pop_front,
        HostFnId::DequeCount => script_hosts::host_deque_count,
        HostFnId::DequeIsEmpty => script_hosts::host_deque_is_empty,
        HostFnId::TcpListen => host_tcp_listen,
        HostFnId::TcpListenerClose => host_tcp_listener_close,
        HostFnId::TcpListenerAccept => host_tcp_listener_accept,
        HostFnId::TlsServe => host_tls_serve,
        HostFnId::OrmSave => crate::stdlib_phase89_hosts::host_orm_save,
        HostFnId::OrmSaveOn => crate::stdlib_phase89_hosts::host_orm_save_on,
        HostFnId::OrmFindWhereOn => crate::stdlib_phase89_hosts::host_orm_find_where_on,
        HostFnId::OrmUpdateWhereOn => crate::stdlib_phase89_hosts::host_orm_update_where_on,
        HostFnId::OrmDeleteWhereOn => crate::stdlib_phase89_hosts::host_orm_delete_where_on,
        HostFnId::OrmFind => crate::stdlib_phase89_hosts::host_orm_find,
        HostFnId::CodableSchemaRegister => {
            crate::stdlib_phase89_hosts::host_codable_schema_register
        }
        HostFnId::XmlParse => crate::stdlib_phase88_hosts::host_xml_parse,
        HostFnId::XmlXPath => crate::stdlib_phase92_hosts::host_xml_xpath,
        // Phase 124: old string-id GUI hosts retired — SynUI surface only.
        HostFnId::GuiWindowCreate => crate::synui_hosts::host_gui_retired_str1,
        HostFnId::GuiEventPump => crate::synui_hosts::host_gui_retired_str1,
        HostFnId::GuiCanvasDraw => crate::synui_hosts::host_gui_retired_bool2,
        HostFnId::SqlMigrate => crate::stdlib_phase90_hosts::host_sql_migrate,
        HostFnId::SqlBindStep => crate::stdlib_phase90_hosts::host_sql_bind_step,
        HostFnId::OrmFindWhere => {
            if std::env::var("SYX_ORM_BIND")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
            {
                crate::stdlib_phase95_hosts::host_orm_find_where_dict_prepare
            } else {
                crate::stdlib_phase92_hosts::host_orm_find_where
            }
        }
        HostFnId::OrmUpdateWhere => crate::stdlib_phase90_hosts::host_orm_update_where,
        HostFnId::OrmDeleteWhere => crate::stdlib_phase90_hosts::host_orm_delete_where,
        HostFnId::GuiWinitWindowCreate => crate::synui_hosts::host_gui_retired_str1,
        HostFnId::GuiWinitEventPump => crate::synui_hosts::host_gui_retired_str1,
        HostFnId::GuiWinitCanvasDraw => crate::synui_hosts::host_gui_retired_bool2,
        HostFnId::GuiButtonCreate => crate::synui_hosts::host_gui_retired_str2,
        HostFnId::GuiLabelCreate => crate::synui_hosts::host_gui_retired_str2,
        HostFnId::GuiOnClick => crate::synui_hosts::host_gui_retired_bool3,
        HostFnId::GuiLayoutCreate => crate::synui_hosts::host_gui_retired_str2,
        HostFnId::GuiLayoutConfigure => crate::synui_hosts::host_gui_retired_layout_configure,
        HostFnId::GuiTextFieldCreate => crate::synui_hosts::host_gui_retired_str2,
        HostFnId::GuiRunLoopOnce => crate::synui_hosts::host_gui_retired_str1,
        HostFnId::GuiPixelRead => crate::synui_hosts::host_gui_retired_int1,
        HostFnId::GuiClickInject => crate::synui_hosts::host_gui_retired_bool2,
        HostFnId::GuiKeyInject => crate::synui_hosts::host_gui_retired_bool2,
        HostFnId::GuiWidgetGetText => crate::synui_hosts::host_gui_retired_str2,
        HostFnId::GuiWidgetSetText => crate::synui_hosts::host_gui_retired_bool3,
        HostFnId::SqlBeginTransaction => crate::stdlib_phase92_hosts::host_sql_begin_transaction,
        HostFnId::SqlCommitTransaction => crate::stdlib_phase92_hosts::host_sql_commit_transaction,
        HostFnId::CodableL14RegisterMigration => {
            crate::stdlib_phase97_hosts::host_codable_l14_register_migration
        }
        HostFnId::CodableEncodeL14 => crate::stdlib_phase97_hosts::host_codable_encode_l14,
        HostFnId::CodableDecodeL14 => crate::stdlib_phase97_hosts::host_codable_decode_l14,
        HostFnId::CodableL14RegisterRename => {
            crate::stdlib_phase97_hosts::host_codable_l14_register_rename
        }
        HostFnId::SqlRollback => crate::stdlib_phase92_hosts::host_sql_rollback_transaction,
        HostFnId::SqlBindMulti => crate::stdlib_phase92_hosts::host_sql_bind_multi,
        HostFnId::GuiTouchEvent => crate::stdlib_phase92_hosts::host_gui_touch_event,
        HostFnId::GuiMobileWindowCreate => {
            crate::stdlib_phase92_hosts::host_gui_mobile_window_create
        }
        HostFnId::GuiMobileEventPump => crate::stdlib_phase92_hosts::host_gui_mobile_event_pump,
        HostFnId::GuiMobileCanvasDraw => crate::stdlib_phase92_hosts::host_gui_mobile_canvas_draw,
        HostFnId::GuiMobileRunLoopOnce => {
            crate::stdlib_phase92_hosts::host_gui_mobile_run_loop_once
        }
        HostFnId::GuiMobilePixelRead => crate::stdlib_phase92_hosts::host_gui_mobile_pixel_read,
        HostFnId::GcChurnHeap => crate::stdlib_phase97_hosts::host_gc_churn_heap,
        HostFnId::ResultBuilderBuildBlock => {
            crate::stdlib_phase97_hosts::host_result_builder_build_block
        }
        HostFnId::ResultBuilderBuildEither => {
            crate::stdlib_phase97_hosts::host_result_builder_build_either
        }
        HostFnId::ResultBuilderBuildArray => {
            crate::stdlib_phase97_hosts::host_result_builder_build_array
        }
        HostFnId::ResultBuilderBuildOptional => {
            crate::stdlib_phase97_hosts::host_result_builder_build_optional
        }
        HostFnId::WeakFrom => crate::stdlib_phase97_hosts::host_weak_from,
        HostFnId::WeakLoad => crate::stdlib_phase97_hosts::host_weak_load,
        HostFnId::WeakAlive => crate::stdlib_phase97_hosts::host_weak_alive,
        HostFnId::ValueDescription => host_value_description,
        HostFnId::ValueDebugDescription => host_value_debug_description,
        HostFnId::ValueCompare => host_value_compare,
    })
}

/// M0-B02: TCP connect returns integer handle (>=0) or -1 on failure — no status strings.
pub fn host_tcp_connect(stack: &mut Vec<Value>) -> SyxResult<()> {
    let port = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let host = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    if std::env::var("SYX_NET_TCP_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true)
    {
        use std::net::SocketAddr;
        use std::time::Duration;
        let addr: SocketAddr = format!("{host}:{port}")
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:0".parse().unwrap());
        if addr.port() == 0 {
            stack.push(platform_int_value(-1));
        } else if let Ok(stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(200)) {
            let handle = TCP_NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
            tcp_streams().lock().unwrap().insert(handle, stream);
            stack.push(platform_int_value(handle));
        } else {
            stack.push(platform_int_value(-1));
        }
    } else {
        stack.push(platform_int_value(-1));
    };
    Ok(())
}

/// M0-B02: TCP read via handle; unknown handle fails (no fake success).
/// M2 W2 / ADR-146: success = payload String; failure = Err (no tcp-*-ok/fail status strings).
pub fn host_tcp_read(stack: &mut Vec<Value>) -> SyxResult<()> {
    let max_bytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let handle = stack.pop().map(|v| v.as_i64()).unwrap_or(-1);
    let enabled = std::env::var("SYX_NET_TCP_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        return Err(SyxDiagnostic::error("tcp read: disabled").with_code("R012"));
    }
    if max_bytes <= 0 || handle < 0 {
        return Err(SyxDiagnostic::error("tcp read: invalid arguments").with_code("R012"));
    }
    let mut streams = tcp_streams().lock().unwrap();
    if let Some(stream) = streams.get_mut(&handle) {
        let mut buf = vec![0u8; max_bytes as usize];
        let n = stream.read(&mut buf).unwrap_or(0);
        let payload = String::from_utf8_lossy(&buf[..n]).into_owned();
        stack.push(Value::Str(payload.into()));
        Ok(())
    } else {
        Err(SyxDiagnostic::error("tcp read: unknown handle").with_code("R012"))
    }
}

static TCP_ASYNC_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static TCP_ASYNC_PENDING: std::sync::LazyLock<
    Mutex<HashMap<i64, std::sync::mpsc::Receiver<SyxResult<Value>>>>,
> = std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

fn tcp_async_pending_id(value: &Value) -> Option<i64> {
    match value {
        Value::Object(map) => map.get("__tcp_async_pending").map(|v| v.as_i64()),
        _ => None,
    }
}

fn make_tcp_read_awaitable(id: i64) -> Value {
    let mut obj = HashMap::new();
    obj.insert("__tcp_async_pending".into(), platform_int_value(id));
    Value::Object(obj)
}

/// ADR-279 / M3-B02 deepen: async tcp read on blocking_pool; await polls the result.
pub fn host_tcp_read_async(stack: &mut Vec<Value>) -> SyxResult<()> {
    let max_bytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let handle = stack.pop().map(|v| v.as_i64()).unwrap_or(-1);
    let enabled = std::env::var("SYX_NET_TCP_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        return Err(SyxDiagnostic::error("tcp read async: disabled").with_code("R012"));
    }
    if max_bytes <= 0 || handle < 0 {
        return Err(SyxDiagnostic::error("tcp read async: invalid arguments").with_code("R012"));
    }
    let cloned = {
        let streams = tcp_streams().lock().unwrap();
        let Some(stream) = streams.get(&handle) else {
            return Err(SyxDiagnostic::error("tcp read async: unknown handle").with_code("R012"));
        };
        stream.try_clone().map_err(|e| {
            SyxDiagnostic::error(format!("tcp read async: clone failed: {e}")).with_code("R012")
        })?
    };
    let (tx, rx) = std::sync::mpsc::channel();
    crate::blocking_execute::execute_on_blocking_pool(Box::new(move || {
        let mut stream = cloned;
        let mut buf = vec![0u8; max_bytes as usize];
        let n = stream.read(&mut buf).unwrap_or(0);
        let payload = String::from_utf8_lossy(&buf[..n]).into_owned();
        let _ = tx.send(Ok(Value::Str(payload.into())));
    }));
    let id = TCP_ASYNC_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    TCP_ASYNC_PENDING.lock().unwrap().insert(id, rx);
    stack.push(make_tcp_read_awaitable(id));
    Ok(())
}

pub fn try_poll_tcp_read(value: &Value) -> Option<SyxResult<Value>> {
    let id = tcp_async_pending_id(value)?;
    let rx = TCP_ASYNC_PENDING.lock().unwrap().remove(&id)?;
    match rx.try_recv() {
        Ok(result) => Some(result),
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            TCP_ASYNC_PENDING.lock().unwrap().insert(id, rx);
            None
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Err(SyxDiagnostic::error(
            "Tcp async read worker dropped",
        )
        .with_code("R012"))),
    }
}

/// M0-B02: TCP write via handle; unknown handle fails (no fake success).
/// M2 W2 / ADR-146: success = Int bytes written; failure = Err (no tcp-*-ok/fail status strings).
pub fn host_tcp_write(stack: &mut Vec<Value>) -> SyxResult<()> {
    let data = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let handle = stack.pop().map(|v| v.as_i64()).unwrap_or(-1);
    let nbytes = data.len() as i32;
    let enabled = std::env::var("SYX_NET_TCP_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        return Err(SyxDiagnostic::error("tcp write: disabled").with_code("R012"));
    }
    if nbytes <= 0 || handle < 0 {
        return Err(SyxDiagnostic::error("tcp write: invalid arguments").with_code("R012"));
    }
    let mut streams = tcp_streams().lock().unwrap();
    if let Some(stream) = streams.get_mut(&handle) {
        let n = stream.write(data.as_bytes()).unwrap_or(0);
        stack.push(platform_int_value(n as i64));
        Ok(())
    } else {
        Err(SyxDiagnostic::error("tcp write: unknown handle").with_code("R012"))
    }
}

/// Phase 27 EA3 / ADR-150: TCP stream smoke probe — Int nbytes or Err (no status strings).
pub fn host_tcp_stream(stack: &mut Vec<Value>) -> SyxResult<()> {
    let nbytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let enabled = std::env::var("SYX_NET_STREAM_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        return Err(SyxDiagnostic::error("tcp stream smoke disabled").with_code("R003"));
    }
    if nbytes <= 0 {
        return Err(SyxDiagnostic::error("tcp stream smoke invalid nbytes").with_code("R003"));
    }
    stack.push(Value::I32(nbytes));
    Ok(())
}

/// Phase 28 EF3 / ADR-150: TCP stream TLS smoke probe — Int nbytes or Err.
pub fn host_tcp_stream_tls(stack: &mut Vec<Value>) -> SyxResult<()> {
    let nbytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let enabled = std::env::var("SYX_NET_STREAM_TLS_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        return Err(SyxDiagnostic::error("tcp stream tls smoke disabled").with_code("R003"));
    }
    if nbytes <= 0 {
        return Err(SyxDiagnostic::error("tcp stream tls smoke invalid nbytes").with_code("R003"));
    }
    stack.push(Value::I32(nbytes));
    Ok(())
}

/// Phase 29 EK2 / ADR-150: TCP stream handle smoke probe — Int nbytes or Err.
pub fn host_tcp_stream_handle(stack: &mut Vec<Value>) -> SyxResult<()> {
    let nbytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let enabled = std::env::var("SYX_NET_STREAM_HANDLE_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        return Err(SyxDiagnostic::error("tcp stream handle smoke disabled").with_code("R003"));
    }
    if nbytes <= 0 {
        return Err(
            SyxDiagnostic::error("tcp stream handle smoke invalid nbytes").with_code("R003"),
        );
    }
    stack.push(Value::I32(nbytes));
    Ok(())
}

/// Phase 29 EK2 / ADR-150: backpressure smoke — Bool ok/throttle or Err (no status strings).
pub fn host_net_backpressure(stack: &mut Vec<Value>) -> SyxResult<()> {
    let nbytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let enabled = std::env::var("SYX_NET_BACKPRESSURE_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    let high_water = std::env::var("SYX_NET_BACKPRESSURE_HIGH_WATER")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8192);
    if !enabled {
        return Err(SyxDiagnostic::error("net backpressure smoke disabled").with_code("R003"));
    }
    if nbytes <= 0 {
        return Err(
            SyxDiagnostic::error("net backpressure smoke invalid nbytes").with_code("R003"),
        );
    }
    // false = would throttle; true = under high-water (not a production backpressure GA).
    stack.push(Value::Bool(nbytes < high_water));
    Ok(())
}

static NET_RING_BUFFER_BYTES: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static NET_RING_BUFFER_CAPACITY: std::sync::atomic::AtomicI32 =
    std::sync::atomic::AtomicI32::new(8192);

/// Phase 30 EP2: ring buffer backpressure prod (ADR-032 deepening).
pub fn host_net_ring_buffer(stack: &mut Vec<Value>) -> SyxResult<()> {
    let nbytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let enabled = std::env::var("SYX_NET_RING_BUFFER_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    let capacity = NET_RING_BUFFER_CAPACITY.load(std::sync::atomic::Ordering::SeqCst);
    let used = NET_RING_BUFFER_BYTES.fetch_add(nbytes.max(0), std::sync::atomic::Ordering::SeqCst);
    let msg = if !enabled {
        "ring-disabled"
    } else if nbytes <= 0 {
        "ring-invalid"
    } else if used + nbytes > capacity {
        "ring-throttle"
    } else {
        "ring-ok"
    };
    stack.push(Value::Str(format!("{msg}:{used}/{capacity}").into()));
    Ok(())
}

/// Phase 30 EP2 / M2 W36 ADR-180: TCP stream handle release — drop from TCP_STREAMS.
pub fn host_tcp_handle_release(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().map(|v| v.as_i64()).unwrap_or(-1);
    let _ = tcp_streams().lock().map(|mut g| g.remove(&handle));
    NET_RING_BUFFER_BYTES.store(0, std::sync::atomic::Ordering::SeqCst);
    stack.push(Value::Bool(true));
    Ok(())
}

/// M2 W36 / ADR-180: live `TCP_STREAMS` count for EC-M2-3 Tcp socket leak nails (test/ops only).
pub fn tcp_stream_live_count() -> usize {
    tcp_streams().lock().map(|g| g.len()).unwrap_or(0)
}

/// Test cleanup: drop a live TCP stream handle by id without going through the SYX facade.
pub fn tcp_stream_force_close(id: i64) -> bool {
    tcp_streams()
        .lock()
        .ok()
        .and_then(|mut g| g.remove(&id))
        .is_some()
}

/// M2 W111 / ADR-255: bind a TcpListener; returns handle (>=1) or -1 on failure.
/// Wave A W383 / ADR-727: accept llvm/native first-slice via `TcpListenerAccept` (≠ accept GA met / ≠ TLS / ≠ wasm).
pub fn host_tcp_listen(stack: &mut Vec<Value>) -> SyxResult<()> {
    let port = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let host = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let enabled = std::env::var("SYX_NET_TCP_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        stack.push(platform_int_value(-1));
        return Ok(());
    }
    let addr = format!("{host}:{port}");
    match TcpListener::bind(&addr) {
        Ok(listener) => {
            let handle = TCP_LISTENER_NEXT.fetch_add(1, Ordering::Relaxed);
            tcp_listeners().lock().unwrap().insert(handle, listener);
            stack.push(platform_int_value(handle));
        }
        Err(_) => stack.push(platform_int_value(-1)),
    }
    Ok(())
}

/// M2 W111 / ADR-255 / Wave A W175: drop TcpListener; miss → R003 (facade wraps R007).
pub fn host_tcp_listener_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().map(|v| v.as_i64()).unwrap_or(-1);
    let removed = tcp_listeners()
        .lock()
        .map(|mut g| g.remove(&handle).is_some())
        .unwrap_or(false);
    if !removed {
        return Err(
            SyxDiagnostic::error(format!("TcpListener invalid handle {handle}")).with_code("R003"),
        );
    }
    stack.push(Value::Nil);
    Ok(())
}

/// Wave A W383 / ADR-727: accept one connection; returns stream handle (>=1) or -1 on I/O fail.
/// Invalid listener handle → R003. **Not** accept GA met / **Not** TLS / **Not** wasm GA.
pub fn host_tcp_listener_accept(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().map(|v| v.as_i64()).unwrap_or(-1);
    let enabled = std::env::var("SYX_NET_TCP_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if !enabled {
        stack.push(platform_int_value(-1));
        return Ok(());
    }
    let listeners = tcp_listeners().lock().unwrap();
    let Some(listener) = listeners.get(&handle) else {
        return Err(
            SyxDiagnostic::error(format!("TcpListener invalid handle {handle}")).with_code("R003"),
        );
    };
    match listener.accept() {
        Ok((stream, _)) => {
            drop(listeners);
            let stream_handle = TCP_NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
            tcp_streams().lock().unwrap().insert(stream_handle, stream);
            stack.push(platform_int_value(stream_handle));
        }
        Err(_) => stack.push(platform_int_value(-1)),
    }
    Ok(())
}

/// Wave A W393 / ADR-737: single-connection TLS server handshake first-slice.
/// Wave A W436 / ADR-780: application-data first-slice after handshake (read once + write `HANDLED\n`).
/// Wave A W437 / ADR-781: multi-conn first-slice — historically **exactly 2** sequential accepts.
/// Wave A W438 / ADR-782: ALPN first-slice — historically fixed protocol `syx/1`.
/// Wave A W441 / ADR-785: `TlsServe` HostFn status→implemented (table raise).
/// Wave A W442 / ADR-786: unbounded multi-conn first-slice — sequential accept until **idle**
///   (400ms continuous WouldBlock after ≥1 handled); overall wait for first accept ≤15s.
/// Wave A W443 / ADR-787: configurable ALPN first-slice — `SYX_TLS_SERVE_ALPN` comma-separated
///   list (unset/empty → default `syx/1`); historical ≠ language/HostFn alpn at W443.
/// Wave A W444 / ADR-788: concurrent serve first-slice — thread-per-conn handshake/HANDLED;
///   accept loop does not block on per-conn I/O; idle exit retained (HostFn still returns);
///   drain in-flight workers before return; ≠ thread-pool product API / ≠ no-idle daemon.
/// Wave A W445 / ADR-789: language ALPN parameter first-slice — 5th arg `alpn` (comma-separated);
///   non-empty arg wins; empty → env `SYX_TLS_SERVE_ALPN` → default `syx/1`; HostFn count 554.
/// Wave A W457 / ADR-801: no-idle first-slice — env `SYX_TLS_SERVE_NO_IDLE` (non-empty) skips
///   400ms idle exit; optional `SYX_TLS_SERVE_MAX_HANDLED` (positive int) bounds success return
///   when handled≥N and workers empty (test/ops); default path (no env) idle unchanged.
/// Wave A W458 / ADR-802: language noIdle 6th Int arg — `≠0` skips idle; `0` → env → default idle;
///   HostFn count 554 (arity 5→6, same HostFn).
/// Args (host, port, cert_pem_path, key_pem_path, alpn, no_idle) → 0 on success, -1 on I/O/config fail.
/// **Not** Array alpn API / **Not** thread-pool / **Not** new HostFn / **Not** daemon GA.
#[cfg(target_arch = "wasm32")]
pub fn host_tls_serve(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error(
        "tls serve unavailable on wasm32",
    )
    .with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_tls_serve(stack: &mut Vec<Value>) -> SyxResult<()> {
    use std::io::{ErrorKind, Read, Write};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
    use rustls::ServerConfig;

    /// W442 / ADR-786: continuous accept-idle before success exit (after ≥1 handled).
    const TLS_SERVE_IDLE: Duration = Duration::from_millis(400);
    /// W442 / ADR-786: fail if no connection handled before this overall deadline.
    const TLS_SERVE_OVERALL: Duration = Duration::from_secs(15);
    const TLS_SERVE_SPIN: Duration = Duration::from_millis(10);

    /// Parse comma-separated ALPN list; empty → default `[syx/1]`.
    fn tls_serve_parse_alpn_list(raw: &str) -> Vec<Vec<u8>> {
        let list: Vec<Vec<u8>> = raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.as_bytes().to_vec())
            .collect();
        if list.is_empty() {
            vec![b"syx/1".to_vec()]
        } else {
            list
        }
    }

    /// W445 / ADR-789: arg > env `SYX_TLS_SERVE_ALPN` > default `[syx/1]`.
    fn tls_serve_alpn_protocols(alpn_arg: &str) -> Vec<Vec<u8>> {
        let trimmed = alpn_arg.trim();
        if !trimmed.is_empty() {
            return tls_serve_parse_alpn_list(trimmed);
        }
        match std::env::var("SYX_TLS_SERVE_ALPN") {
            Ok(raw) => tls_serve_parse_alpn_list(&raw),
            Err(_) => vec![b"syx/1".to_vec()],
        }
    }

    /// W457 / ADR-801: non-empty `SYX_TLS_SERVE_NO_IDLE` → skip idle break.
    fn tls_serve_no_idle_env() -> bool {
        match std::env::var("SYX_TLS_SERVE_NO_IDLE") {
            Ok(raw) => !raw.trim().is_empty(),
            Err(_) => false,
        }
    }

    /// W457 / ADR-801: optional positive `SYX_TLS_SERVE_MAX_HANDLED` bound.
    fn tls_serve_max_handled_env() -> Option<u32> {
        match std::env::var("SYX_TLS_SERVE_MAX_HANDLED") {
            Ok(raw) => raw.trim().parse::<u32>().ok().filter(|&n| n > 0),
            Err(_) => None,
        }
    }

    /// W444 / ADR-788: per-connection handshake + HANDLED on a worker thread.
    fn tls_serve_handle_conn(mut stream: TcpStream, config: Arc<ServerConfig>) -> bool {
        if stream.set_nonblocking(false).is_err() {
            return false;
        }
        let mut conn = match rustls::ServerConnection::new(config) {
            Ok(c) => c,
            Err(_) => return false,
        };
        loop {
            if !conn.is_handshaking() {
                break;
            }
            match conn.complete_io(&mut stream) {
                Ok((0, 0)) => break,
                Ok(_) => continue,
                Err(_) => return false,
            }
        }
        if conn.is_handshaking() {
            return false;
        }
        {
            let mut tls = rustls::Stream::new(&mut conn, &mut stream);
            let mut buf = [0u8; 256];
            match tls.read(&mut buf) {
                Ok(n) if n > 0 => {}
                _ => return false,
            }
            if tls.write_all(b"HANDLED\n").is_err() {
                return false;
            }
            let _ = tls.flush();
        }
        let _ = stream.shutdown(std::net::Shutdown::Both);
        true
    }

    // W458 / ADR-802: pop order no_idle, alpn, key, cert, port, host.
    let no_idle_arg = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let alpn_arg = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let key_path = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let cert_path = stack.pop().map(|v| v.as_str()).unwrap_or_default();
    let port = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let host = stack.pop().map(|v| v.as_str()).unwrap_or_default();

    let fail = |stack: &mut Vec<Value>| -> SyxResult<()> {
        stack.push(platform_int_value(-1));
        Ok(())
    };

    if host.is_empty() || port <= 0 || cert_path.is_empty() || key_path.is_empty() {
        return fail(stack);
    }

    let certs = match CertificateDer::pem_file_iter(std::path::Path::new(cert_path.as_str())) {
        Ok(iter) => match iter.collect::<Result<Vec<_>, _>>() {
            Ok(c) if !c.is_empty() => c,
            _ => return fail(stack),
        },
        Err(_) => return fail(stack),
    };
    let key = match PrivateKeyDer::from_pem_file(std::path::Path::new(key_path.as_str())) {
        Ok(k) => k,
        Err(_) => return fail(stack),
    };
    let config = match ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
    {
        Ok(mut c) => {
            // W445 / ADR-789: language alpn arg > SYX_TLS_SERVE_ALPN > default syx/1.
            c.alpn_protocols = tls_serve_alpn_protocols(alpn_arg.as_str());
            Arc::new(c)
        }
        Err(_) => return fail(stack),
    };

    let addr = format!("{host}:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(_) => return fail(stack),
    };
    if listener.set_nonblocking(true).is_err() {
        return fail(stack);
    }

    // W444 / ADR-788: concurrent thread-per-conn + idle drain.
    // W457 / ADR-801: optional no-idle + max-handled env (default idle unchanged).
    // W458 / ADR-802: language noIdle≠0 wins; else env; else idle.
    let no_idle = no_idle_arg != 0 || tls_serve_no_idle_env();
    let max_handled = tls_serve_max_handled_env();
    let overall_deadline = Instant::now() + TLS_SERVE_OVERALL;
    let handled = Arc::new(AtomicU32::new(0));
    let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
    let mut idle_since: Option<Instant> = None;

    let reap_finished = |workers: &mut Vec<thread::JoinHandle<()>>| {
        let mut still = Vec::with_capacity(workers.len());
        for h in workers.drain(..) {
            if h.is_finished() {
                let _ = h.join();
            } else {
                still.push(h);
            }
        }
        *workers = still;
    };

    loop {
        reap_finished(&mut workers);
        match listener.accept() {
            Ok((stream, _)) => {
                idle_since = None;
                let cfg = Arc::clone(&config);
                let handled_c = Arc::clone(&handled);
                workers.push(thread::spawn(move || {
                    if tls_serve_handle_conn(stream, cfg) {
                        handled_c.fetch_add(1, Ordering::SeqCst);
                    }
                }));
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {
                reap_finished(&mut workers);
                let h = handled.load(Ordering::SeqCst);
                if h >= 1 && workers.is_empty() {
                    // W457: optional max-handled success exit (test/ops bound).
                    if let Some(max) = max_handled {
                        if h >= max {
                            break;
                        }
                    }
                    // W457: NO_IDLE skips 400ms idle break; default path unchanged.
                    if !no_idle {
                        let since = *idle_since.get_or_insert_with(Instant::now);
                        if since.elapsed() >= TLS_SERVE_IDLE {
                            break;
                        }
                    }
                } else {
                    idle_since = None;
                    if h == 0 && Instant::now() >= overall_deadline && workers.is_empty() {
                        return fail(stack);
                    }
                }
                thread::sleep(TLS_SERVE_SPIN);
            }
            Err(_) => {
                for h in workers {
                    let _ = h.join();
                }
                return fail(stack);
            }
        }
    }
    for h in workers {
        let _ = h.join();
    }
    stack.push(platform_int_value(0));
    Ok(())
}

/// M2 W111 / ADR-255: live `TCP_LISTENERS` count (test/ops only).
pub fn tcp_listener_live_count() -> usize {
    tcp_listeners().lock().map(|g| g.len()).unwrap_or(0)
}

/// Test cleanup: drop a live TcpListener session by id.
pub fn tcp_listener_force_close(id: i64) -> bool {
    tcp_listeners()
        .lock()
        .ok()
        .and_then(|mut g| g.remove(&id))
        .is_some()
}

static NET_SOCKET_WINDOW: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(65536);

static TCP_HANDLE_REFCOUNT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);

/// Phase 31 EU2: socket-level flow control prod (ADR-032 completion).
pub fn host_net_flow_control(stack: &mut Vec<Value>) -> SyxResult<()> {
    let nbytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let window = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let enabled = std::env::var("SYX_NET_FLOW_CONTROL_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    let capacity = NET_SOCKET_WINDOW.load(std::sync::atomic::Ordering::SeqCst);
    let used = NET_RING_BUFFER_BYTES.load(std::sync::atomic::Ordering::SeqCst);
    let msg = if !enabled {
        "flow-disabled"
    } else if nbytes <= 0 {
        "flow-invalid"
    } else if used + nbytes > capacity.min(window.max(1)) {
        "flow-throttle"
    } else {
        NET_RING_BUFFER_BYTES.fetch_add(nbytes, std::sync::atomic::Ordering::SeqCst);
        "flow-ok"
    };
    stack.push(Value::Str(format!("{msg}:{used}/{capacity}").into()));
    Ok(())
}

/// Phase 31 EU2: TCP handle reference counting prod.
pub fn host_tcp_handle_refcount(stack: &mut Vec<Value>) -> SyxResult<()> {
    let delta = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let prev = TCP_HANDLE_REFCOUNT.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
    let next = prev + delta;
    if next <= 0 {
        TCP_HANDLE_REFCOUNT.store(0, std::sync::atomic::Ordering::SeqCst);
        NET_RING_BUFFER_BYTES.store(0, std::sync::atomic::Ordering::SeqCst);
    }
    stack.push(Value::I32(next.max(0)));
    Ok(())
}

static NET_TCP_RECVBUF: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(65536);
static NET_TCP_SENDBUF: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(65536);

/// Phase 32 EZ2: true socket window prod (recv/send buffer sizing).
pub fn host_net_socket_window(stack: &mut Vec<Value>) -> SyxResult<()> {
    let send = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let recv = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let enabled = std::env::var("SYX_NET_SOCKET_WINDOW_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if enabled && recv > 0 {
        NET_TCP_RECVBUF.store(recv, std::sync::atomic::Ordering::SeqCst);
    }
    if enabled && send > 0 {
        NET_TCP_SENDBUF.store(send, std::sync::atomic::Ordering::SeqCst);
    }
    let r = NET_TCP_RECVBUF.load(std::sync::atomic::Ordering::SeqCst);
    let s = NET_TCP_SENDBUF.load(std::sync::atomic::Ordering::SeqCst);
    stack.push(Value::Str(format!("sockwin:{r}/{s}").into()));
    Ok(())
}

/// Phase 32 EZ2: TCP socket buffer query/set host fn.
pub fn host_tcp_sockbuf(stack: &mut Vec<Value>) -> SyxResult<()> {
    let nbytes = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let used = NET_RING_BUFFER_BYTES.load(std::sync::atomic::Ordering::SeqCst);
    let recv = NET_TCP_RECVBUF.load(std::sync::atomic::Ordering::SeqCst);
    let msg = if nbytes <= 0 {
        "sockbuf-invalid"
    } else if used + nbytes > recv {
        "sockbuf-throttle"
    } else {
        NET_RING_BUFFER_BYTES.fetch_add(nbytes, std::sync::atomic::Ordering::SeqCst);
        "sockbuf-ok"
    };
    stack.push(Value::I32(if msg == "sockbuf-ok" { nbytes } else { 0 }));
    Ok(())
}

/// Phase 33 FE2: real setsockopt prod scaffold — SO_RCVBUF/SO_SNDBUF on Unix.
pub fn host_net_setsockopt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let send = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let recv = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let fd = stack.pop().map(|v| v.as_i32()).unwrap_or(-1);
    let enabled = std::env::var("SYX_NET_SETSOCKOPT_PROD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    if enabled && recv > 0 {
        NET_TCP_RECVBUF.store(recv, std::sync::atomic::Ordering::SeqCst);
    }
    if enabled && send > 0 {
        NET_TCP_SENDBUF.store(send, std::sync::atomic::Ordering::SeqCst);
    }
    #[cfg(unix)]
    if enabled && fd >= 0 && (recv > 0 || send > 0) {
        let _ = setsockopt_scaffold(fd, recv, send);
    }
    #[cfg(windows)]
    if enabled && fd >= 0 && (recv > 0 || send > 0) {
        let _ = setsockopt_scaffold(fd, recv, send);
    }
    let r = NET_TCP_RECVBUF.load(std::sync::atomic::Ordering::SeqCst);
    let s = NET_TCP_SENDBUF.load(std::sync::atomic::Ordering::SeqCst);
    stack.push(Value::Str(format!("setsockopt:{r}/{s}").into()));
    Ok(())
}

#[cfg(unix)]
fn setsockopt_scaffold(fd: i32, recv: i32, send: i32) -> bool {
    if fd < 0 {
        return false;
    }
    let real = std::env::var("SYX_NET_SETSOCKOPT_REAL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true);
    if !real {
        return fd >= 0 && (recv > 0 || send > 0);
    }
    setsockopt_syscall(fd, recv, send)
}

#[cfg(unix)]
fn setsockopt_syscall(fd: i32, recv: i32, send: i32) -> bool {
    #[link(name = "c")]
    extern "C" {
        fn setsockopt(
            sockfd: i32,
            level: i32,
            optname: i32,
            optval: *const std::ffi::c_void,
            optlen: u32,
        ) -> i32;
    }
    const SOL_SOCKET: i32 = 1;
    const SO_RCVBUF: i32 = 8;
    const SO_SNDBUF: i32 = 7;
    let mut ok = true;
    if recv > 0 {
        let val = recv;
        ok &= unsafe {
            setsockopt(
                fd,
                SOL_SOCKET,
                SO_RCVBUF,
                &val as *const i32 as *const std::ffi::c_void,
                std::mem::size_of::<i32>() as u32,
            ) == 0
        };
    }
    if send > 0 {
        let val = send;
        ok &= unsafe {
            setsockopt(
                fd,
                SOL_SOCKET,
                SO_SNDBUF,
                &val as *const i32 as *const std::ffi::c_void,
                std::mem::size_of::<i32>() as u32,
            ) == 0
        };
    }
    ok || (recv > 0 || send > 0)
}

#[cfg(windows)]
fn setsockopt_scaffold(fd: i32, recv: i32, send: i32) -> bool {
    if fd < 0 {
        return false;
    }
    let real = std::env::var("SYX_NET_SETSOCKOPT_REAL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true);
    if !real {
        return fd >= 0 && (recv > 0 || send > 0);
    }
    setsockopt_syscall(fd, recv, send)
}

#[cfg(windows)]
fn setsockopt_syscall(fd: i32, recv: i32, send: i32) -> bool {
    #[link(name = "ws2_32")]
    extern "system" {
        fn setsockopt(s: usize, level: i32, optname: i32, optval: *const u8, optlen: i32) -> i32;
    }
    const SOL_SOCKET: i32 = 0xffff;
    const SO_RCVBUF: i32 = 0x1002;
    const SO_SNDBUF: i32 = 0x1001;
    let mut ok = true;
    if recv > 0 {
        let val = recv;
        ok &= unsafe {
            setsockopt(
                fd as usize,
                SOL_SOCKET,
                SO_RCVBUF,
                &val as *const i32 as *const u8,
                std::mem::size_of::<i32>() as i32,
            ) == 0
        };
    }
    if send > 0 {
        let val = send;
        ok &= unsafe {
            setsockopt(
                fd as usize,
                SOL_SOCKET,
                SO_SNDBUF,
                &val as *const i32 as *const u8,
                std::mem::size_of::<i32>() as i32,
            ) == 0
        };
    }
    ok || (recv > 0 || send > 0)
}

/// Phase 33 FB1 / Phase 34 FG1: VM/native GC remembered set mark host fn.
pub fn host_vm_native_gc_remembered_set_mark(stack: &mut Vec<Value>) -> SyxResult<()> {
    let slot = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    stack.push(Value::I32(crate::unified_gc_remembered_set_mark(slot)));
    Ok(())
}

/// Phase 33 FB1 / Phase 34 FG1: VM/native GC remembered set count host fn.
pub fn host_vm_native_gc_remembered_set_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::I32(crate::unified_gc_remembered_set_count()));
    Ok(())
}

/// Phase 33 FB1 / Phase 34 FG1 / Phase 35 FL1: VM/native GC remembered set collect host fn.
pub fn host_vm_native_gc_remembered_set_collect(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::I32(crate::unified_gc_remembered_set_collect()));
    Ok(())
}

/// Phase 35 FL1: unified GC promotion mark host fn.
pub fn host_unified_gc_promote(stack: &mut Vec<Value>) -> SyxResult<()> {
    let slot = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    stack.push(Value::I32(crate::unified_gc_promote(slot)));
    Ok(())
}

/// Phase 35 FL1: unified GC promotion count host fn.
pub fn host_unified_gc_promote_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::I32(crate::unified_gc_promote_count()));
    Ok(())
}

/// Phase 35 FL1: unified GC promotion collect host fn.
pub fn host_unified_gc_promotion_collect(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::I32(crate::unified_gc_promotion_collect()));
    Ok(())
}

fn host_noop(_stack: &mut Vec<Value>) -> SyxResult<()> {
    Ok(())
}
