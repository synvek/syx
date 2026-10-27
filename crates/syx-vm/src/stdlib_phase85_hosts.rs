//! Phase 85: gzip/zstd encoding hosts + reflect(T.self) type metadata host.

use std::collections::HashMap;
use std::io::{Read, Write};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

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
    if let Some(meta) = syx_meta::reflect_type(&name) {
        for f in &meta.fields {
            fields.push(Value::Str(f.to_string()));
        }
    }
    let mut map = HashMap::new();
    map.insert("name".into(), Value::Str(name));
    map.insert("fields".into(), Value::Array(fields));
    stack.push(Value::Object(map));
    Ok(())
}
