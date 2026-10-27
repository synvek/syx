//! Phase 88: KVO observe/fire, Codable L5 length-prefixed wire, gRPC streaming loopback,
//! in-memory SQL MVP, simple XML pull parser.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

#[allow(unused_imports)]
pub use crate::stdlib_phase87_hosts::{host_runtime_cast, runtime_cast_matches};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

static KVO_EVENTS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static KVO_OBSERVED: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn kvo_key(object: &Value, key: &str) -> String {
    format!("{}:{}", object.as_str(), key)
}

fn record_kvo(event: &str) {
    if let Ok(mut events) = KVO_EVENTS.lock() {
        events.push(event.to_string());
    }
}

/// Snapshot KVO event sequence (driver tests / native hooks).
pub fn kvo_event_sequence() -> Vec<String> {
    KVO_EVENTS.lock().map(|g| g.clone()).unwrap_or_default()
}

/// Reset KVO event log between tests.
pub fn reset_kvo_events() {
    if let Ok(mut events) = KVO_EVENTS.lock() {
        events.clear();
    }
    if let Ok(mut observed) = KVO_OBSERVED.lock() {
        observed.clear();
    }
}

/// Phase 88: register KVO observation for `(object, key)`.
pub fn host_kvo_observe(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = pop_str(stack);
    let object = stack.pop().unwrap_or(Value::Nil);
    let id = kvo_key(&object, &key);
    if let Ok(mut observed) = KVO_OBSERVED.lock() {
        observed.insert(id, true);
    }
    record_kvo(&format!("observe:{key}"));
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 88: fire KVO `willSet` → `didSet` → `didChange` for `(object, key)`.
pub fn host_kvo_fire(stack: &mut Vec<Value>) -> SyxResult<()> {
    let new_val = stack.pop().unwrap_or(Value::Nil);
    let old_val = stack.pop().unwrap_or(Value::Nil);
    let key = pop_str(stack);
    let object = stack.pop().unwrap_or(Value::Nil);
    let id = kvo_key(&object, &key);
    let observed = KVO_OBSERVED
        .lock()
        .ok()
        .and_then(|g| g.get(&id).copied())
        .unwrap_or(true);
    if observed {
        record_kvo(&format!(
            "willSet:{key}:{}->{}",
            old_val.as_str(),
            new_val.as_str()
        ));
        record_kvo(&format!(
            "didSet:{key}:{}->{}",
            old_val.as_str(),
            new_val.as_str()
        ));
        record_kvo(&format!("didChange:{key}"));
    }
    let seq = kvo_event_sequence().join(",");
    stack.push(Value::Str(seq));
    Ok(())
}

fn push_u32(buf: &mut Vec<u8>, n: u32) {
    buf.extend_from_slice(&n.to_le_bytes());
}

fn read_u32(data: &[u8], off: &mut usize) -> Option<u32> {
    if *off + 4 > data.len() {
        return None;
    }
    let n = u32::from_le_bytes(data[*off..*off + 4].try_into().ok()?);
    *off += 4;
    Some(n)
}

fn push_bytes(buf: &mut Vec<u8>, bytes: &[u8]) {
    push_u32(buf, bytes.len() as u32);
    buf.extend_from_slice(bytes);
}

fn read_bytes(data: &[u8], off: &mut usize) -> Option<Vec<u8>> {
    let len = read_u32(data, off)? as usize;
    if *off + len > data.len() {
        return None;
    }
    let slice = data[*off..*off + len].to_vec();
    *off += len;
    Some(slice)
}

fn encode_field_value(buf: &mut Vec<u8>, v: &Value) {
    match v {
        Value::Nil => {
            buf.push(0);
        }
        Value::Bool(b) => {
            buf.push(1);
            buf.push(u8::from(*b));
        }
        Value::I32(n) => {
            buf.push(2);
            buf.extend_from_slice(&(*n as i64).to_le_bytes());
        }
        Value::I64(n) => {
            buf.push(2);
            buf.extend_from_slice(&n.to_le_bytes());
        }
        Value::Str(s) => {
            buf.push(3);
            push_bytes(buf, s.as_bytes());
        }
        other => {
            buf.push(3);
            push_bytes(buf, other.as_str().as_bytes());
        }
    }
}

fn decode_field_value(data: &[u8], off: &mut usize) -> Option<Value> {
    if *off >= data.len() {
        return None;
    }
    let tag = data[*off];
    *off += 1;
    match tag {
        0 => Some(Value::Nil),
        1 => {
            let b = *data.get(*off)?;
            *off += 1;
            Some(Value::Bool(b != 0))
        }
        2 => {
            if *off + 8 > data.len() {
                return None;
            }
            let n = i64::from_le_bytes(data[*off..*off + 8].try_into().ok()?);
            *off += 8;
            Some(Value::I64(n))
        }
        3 => {
            let bytes = read_bytes(data, off)?;
            Some(Value::Str(String::from_utf8_lossy(&bytes).into_owned()))
        }
        _ => Some(Value::Nil),
    }
}

fn value_as_map(value: &Value) -> Option<HashMap<String, Value>> {
    match Value::resolve_heap_slot(value) {
        Value::Object(map) | Value::Dict(map) => Some(map),
        _ => None,
    }
}

fn grpc_unary_chunk(method: &str, body: &str) -> String {
    let mut stack = vec![Value::Str(body.to_string()), Value::Str(method.to_string())];
    let _ = crate::stdlib_phase87_hosts::host_grpc_unary(&mut stack);
    stack
        .pop()
        .map(|v| v.as_str())
        .unwrap_or_else(|| format!("grpc-ok:{method}:{body}"))
}

/// Phase 88: gRPC client streaming MVP — unary loopback per message, `|` joined.
pub fn host_grpc_stream_client(stack: &mut Vec<Value>) -> SyxResult<()> {
    let messages = stack.pop().unwrap_or(Value::Array(vec![]));
    let method = pop_str(stack);
    let mut responses = Vec::new();
    if let Value::Array(items) = messages {
        for item in items {
            responses.push(grpc_unary_chunk(&method, &item.as_str()));
        }
    } else {
        responses.push(grpc_unary_chunk(&method, &messages.as_str()));
    }
    stack.push(Value::Str(responses.join("|")));
    Ok(())
}

/// Phase 88: register gRPC streaming loopback handler prefix for `method`.
pub fn host_grpc_stream_server(stack: &mut Vec<Value>) -> SyxResult<()> {
    let prefix = pop_str(stack);
    let method = pop_str(stack);
    crate::stdlib_phase87_hosts::register_grpc_loopback_handler(
        &method,
        &format!("stream:{prefix}"),
    );
    stack.push(Value::Bool(true));
    Ok(())
}

#[derive(Default)]
struct SqlTable {
    columns: Vec<String>,
    rows: Vec<HashMap<String, Value>>,
}

static SQL_STORE: LazyLock<Mutex<HashMap<String, SqlTable>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn sql_store() -> std::sync::MutexGuard<'static, HashMap<String, SqlTable>> {
    SQL_STORE.lock().unwrap_or_else(|e| e.into_inner())
}

fn parse_create_table(sql: &str) -> Option<String> {
    let upper = sql.to_uppercase();
    if !upper.starts_with("CREATE TABLE") {
        return None;
    }
    let rest = sql.split_whitespace().nth(2)?;
    Some(rest.trim_end_matches('(').to_string())
}

fn parse_insert(sql: &str) -> Option<(String, Vec<Value>)> {
    let upper = sql.to_uppercase();
    if !upper.starts_with("INSERT INTO") {
        return None;
    }
    let table = sql.split_whitespace().nth(2)?.to_string();
    let values_idx = upper.find("VALUES")?;
    let values_part = sql[values_idx + 6..].trim();
    let inner = values_part.trim_start_matches('(').trim_end_matches(')');
    let values: Vec<Value> = inner
        .split(',')
        .map(|v| {
            let v = v.trim();
            if v.starts_with('\'') && v.ends_with('\'') {
                Value::Str(v[1..v.len() - 1].to_string())
            } else if let Ok(n) = v.parse::<i64>() {
                Value::I64(n)
            } else {
                Value::Str(v.to_string())
            }
        })
        .collect();
    Some((table, values))
}

fn parse_select(sql: &str) -> Option<String> {
    let upper = sql.to_uppercase();
    if !upper.starts_with("SELECT") {
        return None;
    }
    sql.split_whitespace().nth(3).map(|s| s.to_string())
}

fn row_from_values(table: &SqlTable, values: &[Value]) -> HashMap<String, Value> {
    let mut row = HashMap::new();
    for (i, col) in table.columns.iter().enumerate() {
        let val = values.get(i).cloned().unwrap_or(Value::Nil);
        row.insert(col.clone(), val);
    }
    if table.columns.is_empty() {
        for (i, val) in values.iter().enumerate() {
            row.insert(format!("col{i}"), val.clone());
        }
    }
    row
}

/// Phase 88: in-memory SQL exec (`CREATE TABLE` / `INSERT INTO`).
pub fn host_sql_exec(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let mut store = sql_store();
    if let Some(table) = parse_create_table(&sql) {
        store.entry(table).or_default();
        stack.push(Value::I32(0));
        return Ok(());
    }
    if let Some((table, values)) = parse_insert(&sql) {
        let entry = store.entry(table).or_default();
        if entry.columns.is_empty() && !values.is_empty() {
            entry.columns = (0..values.len()).map(|i| format!("col{i}")).collect();
        }
        entry.rows.push(row_from_values(entry, &values));
        stack.push(Value::I32(1));
        return Ok(());
    }
    stack.push(Value::I32(0));
    Ok(())
}

/// Phase 88: in-memory SQL query (`SELECT * FROM table`).
pub fn host_sql_query(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let table_name = parse_select(&sql).unwrap_or_default();
    let store = sql_store();
    let rows = store
        .get(&table_name)
        .map(|t| {
            t.rows
                .iter()
                .map(|row| {
                    let mut obj = row.clone();
                    obj.insert("__type".into(), Value::Str("SqlRow".into()));
                    Value::Object(obj)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    stack.push(Value::Array(rows));
    Ok(())
}

/// Reset in-memory SQL store (tests).
pub fn reset_sql_store() {
    if let Ok(mut store) = SQL_STORE.lock() {
        store.clear();
    }
}

fn parse_xml_element(input: &str) -> Option<(String, String, String)> {
    let s = input.trim();
    if !s.starts_with('<') || s.starts_with("<?") {
        return None;
    }
    let close_gt = s.find('>')?;
    let open = &s[1..close_gt];
    let tag = open.split_whitespace().next()?.to_string();
    let rest = s[close_gt + 1..].trim();
    let close = format!("</{tag}>");
    if let Some(idx) = rest.find(&close) {
        let text = rest[..idx].trim().to_string();
        let tail = rest[idx + close.len()..].trim().to_string();
        return Some((tag, text, tail));
    }
    None
}

fn xml_element_value(tag: &str, text: &str, tail: &str) -> Value {
    let mut map = HashMap::new();
    map.insert("__type".into(), Value::Str("XmlElement".into()));
    map.insert("tag".into(), Value::Str(tag.to_string()));
    map.insert("text".into(), Value::Str(text.to_string()));
    let mut children = Vec::new();
    let mut rest = tail.to_string();
    while let Some((child_tag, child_text, child_tail)) = parse_xml_element(&rest) {
        children.push(xml_element_value(&child_tag, &child_text, &child_tail));
        rest = child_tail;
        if rest.is_empty() {
            break;
        }
    }
    map.insert("children".into(), Value::Array(children));
    Value::Object(map)
}

/// Phase 88: simple pull XML parser for `<tag>text</tag>` (nested subset).
pub fn host_xml_parse(stack: &mut Vec<Value>) -> SyxResult<()> {
    let xml = pop_str(stack);
    if let Some((tag, text, tail)) = parse_xml_element(&xml) {
        stack.push(xml_element_value(&tag, &text, &tail));
    } else {
        stack.push(Value::Nil);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kvo_fire_records_will_did_sequence() {
        reset_kvo_events();
        let mut stack = vec![
            Value::Str("count".into()),
            Value::Str("obj1".into()),
            Value::I32(1),
            Value::I32(2),
        ];
        host_kvo_observe(&mut vec![
            Value::Str("count".into()),
            Value::Str("obj1".into()),
        ])
        .unwrap();
        host_kvo_fire(&mut stack).unwrap();
        let seq = kvo_event_sequence();
        assert!(seq.iter().any(|e| e.starts_with("willSet:")));
        assert!(seq.iter().any(|e| e.starts_with("didSet:")));
        assert!(seq.iter().any(|e| e.starts_with("didChange:")));
    }

    #[test]
    fn sql_insert_select_smoke() {
        reset_sql_store();
        host_sql_exec(&mut vec![Value::Str("CREATE TABLE users".into())]).unwrap();
        host_sql_exec(&mut vec![Value::Str(
            "INSERT INTO users VALUES ('alice', 30)".into(),
        )])
        .unwrap();
        let mut q = vec![Value::Str("SELECT * FROM users".into())];
        host_sql_query(&mut q).unwrap();
        let rows = q.pop().unwrap();
        assert!(matches!(rows, Value::Array(v) if !v.is_empty()));
    }

    #[test]
    fn xml_parse_simple_tag() {
        let mut stack = vec![Value::Str("<name>syx</name>".into())];
        host_xml_parse(&mut stack).unwrap();
        let el = stack.pop().unwrap();
        if let Value::Object(map) = el {
            assert_eq!(map.get("tag").map(|v| v.as_str()), Some("name".to_string()));
            assert_eq!(map.get("text").map(|v| v.as_str()), Some("syx".to_string()));
        } else {
            panic!("expected XmlElement");
        }
    }
}
