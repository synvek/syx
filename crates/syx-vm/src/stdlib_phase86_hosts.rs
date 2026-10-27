//! Phase 86: zstd encoding + @Codable L3 JSON encode/decode MVP + testing run-all.

use std::collections::HashMap;
use std::io::{Read, Write};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn pop_bytes(stack: &mut Vec<Value>) -> Vec<u8> {
    match stack.pop() {
        Some(Value::Str(s)) => s.into_bytes(),
        Some(Value::Array(items)) => items
            .into_iter()
            .filter_map(|v| v.as_i64().try_into().ok())
            .collect(),
        Some(v) => v.as_str().into_bytes(),
        None => vec![],
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_encoding_zstd_compress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let input = pop_bytes(stack);
    let out = zstd::stream::encode_all(input.as_slice(), 3).unwrap_or(input);
    stack.push(Value::Array(
        out.into_iter().map(|b| Value::I32(b as i32)).collect(),
    ));
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn host_encoding_zstd_compress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_bytes(stack);
    Err(syx_diagnostics::SyxDiagnostic::error(
        "zstd unavailable on wasm32",
    )
    .with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_encoding_zstd_decompress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let input = pop_bytes(stack);
    let out = zstd::stream::decode_all(input.as_slice()).unwrap_or(input);
    stack.push(Value::Str(String::from_utf8_lossy(&out).into_owned()));
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn host_encoding_zstd_decompress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_bytes(stack);
    Err(syx_diagnostics::SyxDiagnostic::error(
        "zstd unavailable on wasm32",
    )
    .with_code("V081"))
}

pub fn host_codable_encode(stack: &mut Vec<Value>) -> SyxResult<()> {
    if stack.len() >= 2 {
        return crate::stdlib_phase87_hosts::host_codable_encode_l4(stack);
    }
    let value = stack.pop().unwrap_or(Value::Nil);
    let json = value_to_json(&value);
    stack.push(Value::Str(
        serde_json::to_string(&json).unwrap_or_else(|_| "null".into()),
    ));
    Ok(())
}

pub fn host_codable_decode(stack: &mut Vec<Value>) -> SyxResult<()> {
    if stack.len() >= 2 {
        return crate::stdlib_phase87_hosts::host_codable_decode_l4(stack);
    }
    let json = pop_str(stack);
    let value: serde_json::Value = serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
    stack.push(json_to_value(&value));
    Ok(())
}

fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::I32(n) => serde_json::json!(*n),
        Value::I64(n) => serde_json::json!(*n),
        Value::U8(n) => serde_json::json!(*n),
        Value::F64(n) => serde_json::json!(*n),
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Array(items) => serde_json::Value::Array(items.iter().map(value_to_json).collect()),
        Value::Object(map) | Value::Dict(map) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in map {
                obj.insert(k.to_string(), value_to_json(val));
            }
            serde_json::Value::Object(obj)
        }
        other => serde_json::Value::String(other.as_str()),
    }
}

fn json_to_value(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::I64(i)
            } else if let Some(u) = n.as_u64() {
                Value::I64(u as i64)
            } else {
                Value::F64(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::Str(s.clone()),
        serde_json::Value::Array(items) => Value::Array(items.iter().map(json_to_value).collect()),
        serde_json::Value::Object(map) => {
            let mut out = HashMap::new();
            for (k, val) in map {
                out.insert(k.clone(), json_to_value(val));
            }
            Value::Object(out)
        }
    }
}

pub fn host_testing_run_all(stack: &mut Vec<Value>) -> SyxResult<()> {
    let failed = crate::stdlib_phase84_hosts::run_all_registered_tests();
    stack.push(Value::I32(failed));
    Ok(())
}
