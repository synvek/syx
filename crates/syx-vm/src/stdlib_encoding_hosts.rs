//! Encoding hosts: gzip/zstd, reflect(T.self), @Codable L3 JSON, testing run-all.

use std::collections::HashMap;
use std::io::{Read, Write};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use syx_diagnostics::{Span, SyxResult};
use syx_runtime_core::Value;

use crate::vm_error_at;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn pop_bytes(stack: &mut Vec<Value>) -> Vec<u8> {
    let v = stack.pop().unwrap_or(Value::Nil);
    // HeapSlot arrays (literal `[…]`) must materialize before byte extract.
    if let Some(items) = Value::materialize_array(v.clone()) {
        return items
            .into_iter()
            .filter_map(|item| {
                let n = item.as_i64();
                if (0..=255).contains(&n) {
                    Some(n as u8)
                } else {
                    None
                }
            })
            .collect();
    }
    match Value::materialize_receiver(v) {
        Value::Str(s) => s.into_bytes(),
        other => other.as_str().into_bytes(),
    }
}

pub fn host_encoding_gzip_compress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let input = pop_bytes(stack);
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&input).ok();
    let out = encoder.finish().unwrap_or_default();
    stack.push(Value::Array(
        out.into_iter().map(|b| Value::I32(b as i32)).collect(),
    ));
    Ok(())
}

pub fn host_encoding_gzip_decompress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let input = pop_bytes(stack);
    let mut decoder = GzDecoder::new(input.as_slice());
    let mut out = Vec::new();
    decoder.read_to_end(&mut out).ok();
    stack.push(Value::Str(String::from_utf8_lossy(&out).into_owned()));
    Ok(())
}

pub fn host_reflect_specialization(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut type_args = Vec::new();
    while stack.len() > 1 {
        type_args.push(pop_str(stack));
    }
    type_args.reverse();
    let fn_name = pop_str(stack);
    let arg_refs: Vec<&str> = type_args.iter().map(String::as_str).collect();
    let sig = syx_meta::reflect_specialization(&fn_name, &arg_refs)?;
    let mut map = HashMap::new();
    map.insert("fn".into(), Value::Str(sig.fn_name.to_string()));
    map.insert(
        "typeArgs".into(),
        Value::Array(
            sig.type_args
                .iter()
                .map(|t| Value::Str(t.to_string()))
                .collect(),
        ),
    );
    map.insert("paramCount".into(), Value::I64(sig.param_count as i64));
    map.insert("hasWitness".into(), Value::Bool(sig.has_witness));
    stack.push(Value::Object(map));
    Ok(())
}

pub fn host_reflect_type(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = pop_str(stack);
    let mut fields = Vec::new();
    let mut field_types = Vec::new();
    if let Some(meta) = syx_meta::reflect_type(&name) {
        for f in &meta.fields {
            fields.push(Value::Str(f.to_string()));
        }
        // S3-I46: user-facing demangle for reflection display; registry identity unchanged.
        for t in meta.user_facing_field_types() {
            field_types.push(Value::Str(t.to_string()));
        }
    }
    let display_name = syx_meta::user_facing_meta_type_name(&name).to_string();
    let mut map = HashMap::new();
    map.insert("name".into(), Value::Str(display_name));
    map.insert("fields".into(), Value::Array(fields));
    map.insert("fieldTypes".into(), Value::Array(field_types));
    stack.push(Value::Object(map));
    Ok(())
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
    Err(vm_error_at(
        "zstd unavailable on wasm32",
        None,
        Span::DUMMY,
        "V081",
    ))
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
    Err(vm_error_at(
        "zstd unavailable on wasm32",
        None,
        Span::DUMMY,
        "V081",
    ))
}

pub fn host_codable_encode(stack: &mut Vec<Value>) -> SyxResult<()> {
    if stack.len() >= 2 {
        return crate::stdlib_json_grpc_hosts::host_codable_encode_l4(stack);
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
        return crate::stdlib_json_grpc_hosts::host_codable_decode_l4(stack);
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
    let failed = crate::stdlib_process_hosts::run_all_registered_tests();
    stack.push(Value::I32(failed));
    Ok(())
}
