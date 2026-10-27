use std::collections::HashMap;

use syx_stdlib::type_name_with_alias;

use crate::hasher::{combine_value, hash_value};
use crate::numeric::{display_numeric, value_kind};
use crate::Value;

/// Max length for `description` / `debugDescription` / `print` collection output.
pub const DESCRIPTION_MAX_LEN: usize = 2048;

pub const OBJECT_ID_KEY: &str = "__object_id";

pub fn truncate_description(s: String) -> String {
    if s.len() <= DESCRIPTION_MAX_LEN {
        return s;
    }
    let mut end = DESCRIPTION_MAX_LEN;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &s[..end])
}

fn format_separated_descriptions(
    parts: impl IntoIterator<Item = String>,
    open: &str,
    close: &str,
) -> String {
    let mut out = String::from(open);
    let mut first = true;
    for inner in parts {
        let sep = if first { "" } else { ", " };
        first = false;
        let addition = format!("{sep}{inner}");
        if out.len() + addition.len() + close.len() + 3 > DESCRIPTION_MAX_LEN {
            out.push_str(", ...");
            break;
        }
        out.push_str(&addition);
    }
    if !out.ends_with("...") {
        out.push_str(close);
    } else {
        out.push(']');
    }
    truncate_description(out)
}

static NEXT_OBJECT_ID: std::sync::atomic::AtomicI64 =
    std::sync::atomic::AtomicI64::new(1);

pub fn allocate_object_id() -> i64 {
    NEXT_OBJECT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub fn ensure_object_id(map: &mut HashMap<String, Value>) {
    if !map.contains_key(OBJECT_ID_KEY) {
        map.insert(
            OBJECT_ID_KEY.into(),
            Value::I64(allocate_object_id()),
        );
    }
}

pub fn value_object_id(v: &Value) -> Option<i64> {
    match v {
        Value::Object(map) | Value::Dict(map) => map.get(OBJECT_ID_KEY).and_then(|id| match id {
            Value::I64(n) => Some(*n),
            Value::I32(n) => Some(*n as i64),
            _ => None,
        }),
        Value::Closure { .. } => Some(closure_id(v)),
        _ => None,
    }
}

fn closure_id(v: &Value) -> i64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    format!("{v:?}").hash(&mut h);
    h.finish() as i64
}

pub fn value_description(v: &Value) -> String {
    if let Value::HeapSlot { .. } = v {
        return value_description(&Value::resolve_heap_slot(v));
    }
    if let Some(inner) = Value::mut_ref_cell_inner(v) {
        return value_description(&inner);
    }
    if let Some(s) = display_numeric(v) {
        return truncate_description(s);
    }
    let out = match v {
        Value::Bool(b) => b.to_string(),
        Value::Str(s) => s.clone(),
        Value::Character(c) => c.clone(),
        Value::Nil => "nil".into(),
        Value::Array(items) => format_separated_descriptions(
            items.iter().map(value_description),
            "[",
            "]",
        ),
        Value::Set(items) => format_separated_descriptions(
            items.iter().map(value_description),
            "[",
            "]",
        ),
        Value::Dict(map) => {
            let mut pairs: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{}: {}", k, value_description(v)))
                .collect();
            pairs.sort();
            format_separated_descriptions(pairs, "[", "]")
        }
        Value::Object(map) if is_object_identifier(map) => {
            format!("ObjectIdentifier({})", object_identifier_id(map))
        }
        Value::Object(map) if is_hasher(map) => "Hasher".into(),
        Value::Object(map)
            if map
                .get("__type")
                .map(|t| t.as_str())
                .as_deref()
                == Some(crate::bigint::BIGINT_TYPE) =>
        {
            crate::bigint::bigint_description(v)
        }
        Value::Object(map) => object_fields_description(map),
        Value::ObjectRef { .. } => {
            if let Some(fields) = Value::object_ref_user_fields(v) {
                let mut pairs: Vec<String> = fields
                    .iter()
                    .map(|(k, val)| format!("{}: {}", k, value_description(val)))
                    .collect();
                pairs.sort();
                if pairs.is_empty() {
                    v.object_ref_type_name()
                        .map(|n| format!("{n} {{}}"))
                        .unwrap_or_else(|| "{...}".into())
                } else {
                    format_separated_descriptions(pairs, "{", "}")
                }
            } else {
                "{...}".into()
            }
        }
        Value::Closure { .. } => "<closure>".into(),
        Value::Iterator(_) => "<iterator>".into(),
        _ => "?".into(),
    };
    truncate_description(out)
}

fn object_fields_description(map: &std::collections::HashMap<String, Value>) -> String {
    let mut pairs: Vec<String> = map
        .iter()
        .filter(|(k, _)| !k.starts_with("__"))
        .map(|(k, v)| format!("{}: {}", k, value_description(v)))
        .collect();
    pairs.sort();
    if pairs.is_empty() {
        if let Some(ty) = map.get("__type").map(|t| t.as_str()) {
            if ty != "MutRefCell" {
                return format!("{ty} {{}}");
            }
        }
        "{...}".into()
    } else {
        format_separated_descriptions(pairs, "{", "}")
    }
}

pub fn value_debug_description(v: &Value) -> String {
    if let Value::HeapSlot { .. } = v {
        return value_debug_description(&Value::resolve_heap_slot(v));
    }
    if let Some(kind) = value_kind(v) {
        let name = type_name_with_alias(kind);
        return truncate_description(format!("{name}({})", value_description(v)));
    }
    let out = match v {
        Value::Bool(b) => format!("Bool({b})"),
        Value::Str(s) => format!("String(\"{s}\")"),
        Value::Character(c) => format!("Character(\"{c}\")"),
        Value::Nil => "Optional(nil)".into(),
        Value::Array(items) => {
            let inner: Vec<String> = items.iter().map(value_debug_description).collect();
            format!("Array([{}])", inner.join(", "))
        }
        Value::Set(items) => {
            let inner: Vec<String> = items.iter().map(value_debug_description).collect();
            format!("Set([{}])", inner.join(", "))
        }
        Value::Dict(map) => {
            let mut pairs: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("\"{k}\": {}", value_debug_description(v)))
                .collect();
            pairs.sort();
            format!("Dictionary([{}])", pairs.join(", "))
        }
        Value::Object(map) if is_object_identifier(map) => {
            format!(
                "ObjectIdentifier({})",
                object_identifier_id(map)
            )
        }
        Value::Object(map) if is_hasher(map) => "Hasher(...)".into(),
        Value::Object(map) => {
            if let Some(id) = map.get(OBJECT_ID_KEY) {
                format!("Object(id: {id:?}, ...)")
            } else if map.len() <= 16 {
                let mut pairs: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{k}\": {}", value_debug_description(v)))
                    .collect();
                pairs.sort();
                format!("Object([{}])", pairs.join(", "))
            } else {
                "Object(...)".into()
            }
        }
        Value::Closure { fn_index, .. } => format!("Closure(fn: {fn_index})"),
        Value::Iterator(_) => "<iterator>".into(),
        _ => format!("{:?}", value_description(v)),
    };
    truncate_description(out)
}

pub fn opaque_witness_name(v: &Value) -> Option<&str> {
    let map = match v {
        Value::Object(m) | Value::Dict(m) => m,
        _ => return None,
    };
    if !map.contains_key("__opaque_payload") {
        return None;
    }
    map.get("__opaque_witness").and_then(|w| match w {
        Value::Str(s) => Some(s.as_str()),
        _ => None,
    })
}

pub fn opaque_payload(v: &Value) -> Option<&Value> {
    let map = match v {
        Value::Object(m) | Value::Dict(m) => m,
        _ => return None,
    };
    map.get("__opaque_payload")
}

pub fn value_equals(a: &Value, b: &Value) -> bool {
    let a = match a {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(a),
        other => other.clone(),
    };
    let b = match b {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(b),
        other => other.clone(),
    };
    if let (Some(wa), Some(wb)) = (opaque_witness_name(&a), opaque_witness_name(&b)) {
        if wa != wb {
            return false;
        }
        if let (Some(pa), Some(pb)) = (opaque_payload(&a), opaque_payload(&b)) {
            return value_equals(pa, pb);
        }
        return false;
    }
    match (&a, &b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Character(x), Value::Character(y)) => x == y,
        (Value::Character(x), Value::Str(y)) | (Value::Str(y), Value::Character(x)) => x == y,
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| value_equals(a, b))
        }
        (Value::Set(x), Value::Set(y)) => {
            x.len() == y.len() && x.iter().all(|v| y.iter().any(|u| value_equals(v, u)))
        }
        (Value::Dict(x), Value::Dict(y)) => dict_equals_maps(x, y),
        // Class instances as Set/Dictionary keys use identity (same ObjectRef Arc),
        // including MutRefCell wrappers (via class_instance_identity).
        (Value::ObjectRef { .. }, Value::ObjectRef { .. }) => {
            let ia = Value::class_instance_identity(&a);
            let ib = Value::class_instance_identity(&b);
            ia.is_some() && ia == ib
        }
        (Value::Simd { .. }, Value::Simd { .. }) => crate::simd::simd_equals(&a, &b),
        (Value::Object(x), Value::Object(y)) => {
            if is_object_identifier(x) && is_object_identifier(y) {
                return object_identifier_id(x) == object_identifier_id(y);
            }
            dict_equals(x, y)
        }
        _ => {
            if let (Some(ia), Some(ib)) = (
                Value::class_instance_identity(&a),
                Value::class_instance_identity(&b),
            ) {
                return ia == ib;
            }
            if value_kind(&a).is_some() && value_kind(&b).is_some() {
                return Value::numeric_eq(&a, &b);
            }
            false
        }
    }
}

/// Phase 58: total order for numeric/compare Host ops.
pub fn value_compare(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::I32(x), Value::I32(y)) => x.cmp(y),
        (Value::I64(x), Value::I64(y)) => x.cmp(y),
        (Value::F32(x), Value::F32(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (Value::F64(x), Value::F64(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (Value::Str(x), Value::Str(y)) => x.cmp(y),
        (Value::Character(x), Value::Character(y)) => x.cmp(y),
        _ if value_kind(a).is_some() && value_kind(b).is_some() => {
            if Value::numeric_eq(a, b) {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        }
        _ => std::cmp::Ordering::Equal,
    }
}

/// Phase 65: Comparable witness total order as signed Int (-1, 0, 1).
pub fn value_compare_int(a: &Value, b: &Value) -> i32 {
    match value_compare(a, b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn dict_equals(x: &HashMap<String, Value>, y: &HashMap<String, Value>) -> bool {
    dict_equals_maps(x, y)
}

pub(crate) fn dict_equals_maps(x: &HashMap<String, Value>, y: &HashMap<String, Value>) -> bool {
    x.iter()
        .filter(|(k, _)| *k != OBJECT_ID_KEY)
        .all(|(k, v)| y.get(k).is_some_and(|yv| value_equals(v, yv)))
        && y
            .iter()
            .filter(|(k, _)| *k != OBJECT_ID_KEY)
            .all(|(k, v)| x.get(k).is_some_and(|xv| value_equals(v, xv)))
}

pub fn value_identical(a: &Value, b: &Value) -> bool {
    if let (Some(ia), Some(ib)) = (
        Value::class_instance_identity(a),
        Value::class_instance_identity(b),
    ) {
        return ia == ib;
    }
    match (a, b) {
        (Value::Object(_), Value::Object(_)) | (Value::Dict(_), Value::Dict(_)) => {
            if let (Some(ix), Some(iy)) = (value_object_id(a), value_object_id(b)) {
                return ix == iy;
            }
            value_equals(a, b)
        }
        (Value::Closure { .. }, Value::Closure { .. }) => closure_id(a) == closure_id(b),
        _ => value_equals(a, b),
    }
}

pub fn make_object_identifier(v: &Value) -> Option<Value> {
    let id = value_object_id(v).unwrap_or_else(|| {
        if value_kind(v).is_some() || matches!(v, Value::Bool(_) | Value::Str(_) | Value::Nil) {
            hash_value(v) as i64
        } else {
            allocate_object_id()
        }
    });
    let mut map = HashMap::new();
    map.insert("__type".into(), Value::Str("ObjectIdentifier".into()));
    map.insert("id".into(), Value::I64(id));
    Some(Value::Object(map))
}

fn is_object_identifier(map: &HashMap<String, Value>) -> bool {
    map.get("__type")
        .and_then(|v| match v {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        })
        == Some("ObjectIdentifier")
}

fn object_identifier_id(map: &HashMap<String, Value>) -> i64 {
    map.get("id").map(|v| v.as_i64()).unwrap_or(0)
}

pub fn is_hasher(map: &std::collections::HashMap<String, Value>) -> bool {
    map.contains_key("__hasher")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    #[test]
    fn array_description_shows_elements() {
        let v = Value::Array(vec![Value::I32(1), Value::I32(2), Value::I32(3)]);
        assert_eq!(value_description(&v), "[1, 2, 3]");
    }

    #[test]
    fn array_description_truncates_long_output() {
        let items: Vec<Value> = (0..500).map(|n| Value::I32(n)).collect();
        let v = Value::Array(items);
        let s = value_description(&v);
        assert!(s.len() <= DESCRIPTION_MAX_LEN + 3);
        assert!(s.ends_with("..."));
    }
}
