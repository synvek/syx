use std::collections::HashMap;

use crate::hasher::hash_value;
use crate::protocols::value_equals;
use crate::Value;

const PREFIX: &str = "\x1fsyx:";

fn slot_hash(key: &Value) -> u64 {
    hash_value(key)
}

/// One bucket per hash: `Array` of `[k0, v0, k1, v1, …]`.
fn pairs_slot(h: u64) -> String {
    format!("{PREFIX}h:{h:016x}:pairs")
}

fn pair_index(pairs: &[Value], key: &Value) -> Option<usize> {
    let mut i = 0;
    while i + 1 < pairs.len() {
        if value_equals(&pairs[i], key) {
            return Some(i);
        }
        i += 2;
    }
    None
}

fn dict_insert_with_hash(map: &mut HashMap<String, Value>, key: Value, value: Value, h: u64) {
    let slot = pairs_slot(h);
    let mut pairs = match map.remove(&slot) {
        Some(Value::Array(items)) => items,
        Some(other) => {
            map.insert(slot.clone(), other);
            return;
        }
        None => Vec::new(),
    };
    if let Some(idx) = pair_index(&pairs, &key) {
        pairs[idx + 1] = value;
    } else {
        pairs.push(key);
        pairs.push(value);
    }
    map.insert(slot, Value::Array(pairs));
}

fn dict_get_with_hash<'a>(
    map: &'a HashMap<String, Value>,
    key: &Value,
    h: u64,
) -> Option<&'a Value> {
    let Value::Array(pairs) = map.get(&pairs_slot(h))? else {
        return None;
    };
    let idx = pair_index(pairs, key)?;
    pairs.get(idx + 1)
}

fn dict_remove_with_hash(map: &mut HashMap<String, Value>, key: &Value, h: u64) -> bool {
    let slot = pairs_slot(h);
    let mut pairs = match map.remove(&slot) {
        Some(Value::Array(items)) => items,
        Some(other) => {
            map.insert(slot, other);
            return false;
        }
        None => return false,
    };
    let Some(idx) = pair_index(&pairs, key) else {
        map.insert(slot, Value::Array(pairs));
        return false;
    };
    pairs.remove(idx);
    pairs.remove(idx);
    if !pairs.is_empty() {
        map.insert(slot, Value::Array(pairs));
    }
    true
}

/// Insert `key` → `value` using witness hash + eq (SYX 39.0 Hashable dict keys).
/// Non-String keys that share a hash chain in one bucket; equal keys replace the value.
pub fn dict_insert(map: &mut HashMap<String, Value>, key: Value, value: Value) {
    let key = Value::materialize_receiver(key);
    if let Value::Str(s) = &key {
        map.insert(s.clone(), value);
        return;
    }
    let h = slot_hash(&key);
    dict_insert_with_hash(map, key, value, h);
}

pub fn dict_get<'a>(map: &'a HashMap<String, Value>, key: &Value) -> Option<&'a Value> {
    let key = Value::materialize_receiver(key.clone());
    if let Value::Str(s) = &key {
        return map.get(s.as_str());
    }
    let h = slot_hash(&key);
    dict_get_with_hash(map, &key, h)
}

pub fn dict_contains(map: &HashMap<String, Value>, key: &Value) -> bool {
    dict_get(map, key).is_some()
}

pub fn dict_remove(map: &mut HashMap<String, Value>, key: &Value) -> bool {
    let key = Value::materialize_receiver(key.clone());
    if let Value::Str(s) = &key {
        return map.remove(s.as_str()).is_some();
    }
    let h = slot_hash(&key);
    dict_remove_with_hash(map, &key, h)
}

pub fn dict_len(map: &HashMap<String, Value>) -> usize {
    let mut n = 0;
    for (k, v) in map {
        if k.starts_with(PREFIX) {
            if let Value::Array(pairs) = v {
                n += pairs.len() / 2;
            }
        } else {
            n += 1;
        }
    }
    n
}

pub fn dict_keys(map: &HashMap<String, Value>) -> Vec<Value> {
    let mut out = Vec::new();
    for (k, v) in map {
        if k.starts_with(PREFIX) {
            if let Value::Array(pairs) = v {
                let mut i = 0;
                while i < pairs.len() {
                    out.push(pairs[i].clone());
                    i += 2;
                }
            }
        } else {
            out.push(Value::Str(k.clone()));
        }
    }
    out
}

pub fn dict_values(map: &HashMap<String, Value>) -> Vec<Value> {
    let mut out = Vec::new();
    for (k, v) in map {
        if k.starts_with(PREFIX) {
            if let Value::Array(pairs) = v {
                let mut i = 1;
                while i < pairs.len() {
                    out.push(pairs[i].clone());
                    i += 2;
                }
            }
        } else {
            out.push(v.clone());
        }
    }
    out
}

/// Build a dict from stack pairs (key, value) in reverse pop order.
pub fn dict_build_from_pairs(pairs: impl IntoIterator<Item = (Value, Value)>) -> Value {
    let mut map = HashMap::new();
    for (key, value) in pairs {
        dict_insert(&mut map, key, value);
    }
    Value::Dict(map)
}

#[cfg(test)]
mod field_storage_smoke {
    use super::*;
    use crate::Value;
    use crate::hasher::hash_value;

    #[test]
    fn object_ref_keys_have_distinct_hashes_and_dict_slots() {
        let a = Value::object_ref_from_map(
            HashMap::from([("__type".into(), Value::Str("Key".into()))]),
            true,
        );
        let b = Value::object_ref_from_map(
            HashMap::from([("__type".into(), Value::Str("Key".into()))]),
            true,
        );
        assert_ne!(
            hash_value(&a),
            hash_value(&b),
            "distinct ObjectRef instances must not share a hash"
        );
        let mut map = HashMap::new();
        dict_insert(&mut map, a.clone(), Value::I32(1));
        dict_insert(&mut map, b.clone(), Value::I32(2));
        assert_eq!(dict_len(&map), 2);
        assert_eq!(dict_get(&map, &a).map(|v| v.as_i32()), Some(1));
        assert_eq!(dict_get(&map, &b).map(|v| v.as_i32()), Some(2));
        dict_insert(&mut map, a.clone(), Value::I32(9));
        assert_eq!(dict_len(&map), 2);
        assert_eq!(dict_get(&map, &a).map(|v| v.as_i32()), Some(9));
    }

    #[test]
    fn same_hash_unequal_keys_chain() {
        let mut map = HashMap::new();
        let a = Value::I32(1);
        let b = Value::I32(2);
        let h = 0xabcdu64;
        dict_insert_with_hash(&mut map, a.clone(), Value::I32(10), h);
        dict_insert_with_hash(&mut map, b.clone(), Value::I32(20), h);
        assert_eq!(dict_len(&map), 2);
        assert_eq!(dict_get_with_hash(&map, &a, h).map(|v| v.as_i32()), Some(10));
        assert_eq!(dict_get_with_hash(&map, &b, h).map(|v| v.as_i32()), Some(20));
        dict_insert_with_hash(&mut map, a.clone(), Value::I32(11), h);
        assert_eq!(dict_len(&map), 2);
        assert_eq!(dict_get_with_hash(&map, &a, h).map(|v| v.as_i32()), Some(11));
        assert_eq!(dict_get_with_hash(&map, &b, h).map(|v| v.as_i32()), Some(20));
        assert!(dict_remove_with_hash(&mut map, &a, h));
        assert_eq!(dict_len(&map), 1);
        assert!(dict_get_with_hash(&map, &a, h).is_none());
        assert_eq!(dict_get_with_hash(&map, &b, h).map(|v| v.as_i32()), Some(20));
        let keys = dict_keys(&map);
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].as_i32(), 2);
        assert!(
            keys.iter()
                .all(|k| !matches!(k, Value::Str(s) if s.starts_with(PREFIX))),
            "internal slots must not leak as keys"
        );
        let values = dict_values(&map);
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].as_i32(), 20);
    }

    #[test]
    fn dict_build_pop_order_matches_init_prologue() {
        let mut stack = vec![
            Value::Str("fastlogger".into()), // LoadLocal logger
            Value::Str("logger".into()),     // ConstStr key
            Value::Str("App".into()),        // ConstStr __type val
            Value::Str("__type".into()),     // ConstStr __type key
        ];
        let count = 2u32;
        let mut map = HashMap::new();
        for _ in 0..count {
            let key_val = stack.pop().unwrap();
            let val = stack.pop().unwrap();
            dict_insert(&mut map, key_val, val);
        }
        assert_eq!(map.get("logger").map(|v| v.as_str()), Some("fastlogger".to_string()));
        assert_eq!(map.get("__type").map(|v| v.as_str()), Some("App".to_string()));
    }

    #[test]
    fn mut_ref_cell_field_roundtrip_after_class_init_pattern() {
        let mut inner = HashMap::new();
        dict_insert(
            &mut inner,
            Value::Str("logger".into()),
            Value::Str("child".into()),
        );
        dict_insert(
            &mut inner,
            Value::Str("__type".into()),
            Value::Str("App".into()),
        );
        let inner_ref = Value::object_ref_from_map(inner, true);
        let mut wrap = HashMap::new();
        dict_insert(
            &mut wrap,
            Value::Str("__syx_ref".into()),
            inner_ref.clone(),
        );
        dict_insert(
            &mut wrap,
            Value::Str("__type".into()),
            Value::Str("MutRefCell".into()),
        );
        let app = Value::mut_ref_cell_from_map(wrap);
        assert_eq!(app.mut_ref_cell_get_field("logger").as_str(), "child");
    }

    #[test]
    fn object_ref_stores_mut_ref_cell_field() {
        let child = Value::mut_ref_cell_from_map(HashMap::from([
            ("__type".into(), Value::Str("FastLogger".into())),
            (
                "__syx_ref".into(),
                Value::object_ref_from_map(
                    HashMap::from([("__type".into(), Value::Str("FastLogger".into()))]),
                    true,
                ),
            ),
        ]));
        let mut inner = HashMap::new();
        dict_insert(&mut inner, Value::Str("logger".into()), child.clone());
        dict_insert(
            &mut inner,
            Value::Str("__type".into()),
            Value::Str("App".into()),
        );
        let inner_ref = Value::object_ref_from_map(inner, true);
        let mut wrap = HashMap::new();
        dict_insert(
            &mut wrap,
            Value::Str("__syx_ref".into()),
            inner_ref,
        );
        dict_insert(
            &mut wrap,
            Value::Str("__type".into()),
            Value::Str("MutRefCell".into()),
        );
        let app = Value::mut_ref_cell_from_map(wrap);
        assert!(!matches!(app.get_by_key(&Value::Str("logger".into())), Value::Nil));
        let logger = app.get_by_key(&Value::Str("logger".into()));
        assert_eq!(
            logger.object_ref_type_name().as_deref(),
            Some("FastLogger"),
            "field-stored MutRefCell must expose runtime type for virtual dispatch"
        );
    }

    #[test]
    fn vm_dict_build_wrap_matches_class_init() {
        let mut inner = HashMap::new();
        dict_insert(
            &mut inner,
            Value::Str("logger".into()),
            Value::Str("child".into()),
        );
        dict_insert(
            &mut inner,
            Value::Str("__type".into()),
            Value::Str("App".into()),
        );
        crate::protocols::ensure_object_id(&mut inner);
        let inner_ref = Value::object_ref_from_map(inner, true);
        let mut stack = vec![
            inner_ref,
            Value::Str("__syx_ref".into()),
            Value::Str("MutRefCell".into()),
            Value::Str("__type".into()),
        ];
        let mut map = HashMap::new();
        for _ in 0..2 {
            let key_val = stack.pop().unwrap();
            let val = stack.pop().unwrap();
            dict_insert(&mut map, key_val, val);
        }
        let app = Value::mut_ref_cell_from_map(map);
        assert_eq!(app.get_by_key(&Value::Str("logger".into())).as_str(), "child");
    }

    #[test]
    fn object_ref_type_name_reads_str_type_meta() {
        let named = Value::object_ref_from_map(
            HashMap::from([("__type".into(), Value::Str("FastLogger".into()))]),
            true,
        );
        assert_eq!(
            named.object_ref_type_name().as_deref(),
            Some("FastLogger")
        );
    }

    #[test]
    fn mut_ref_cell_delegates_vtable_slot_lookup() {
        let mut inner_map = HashMap::from([
            ("__type".into(), Value::Str("FastLogger".into())),
            ("__vtable_tag".into(), Value::I32(314)),
        ]);
        crate::protocols::ensure_object_id(&mut inner_map);
        let inner = Value::object_ref_from_map(inner_map, true);
        let wrapped = Value::mut_ref_cell_from_map(HashMap::from([
            ("__syx_ref".into(), inner),
            ("__type".into(), Value::Str("MutRefCell".into())),
        ]));
        assert!(matches!(
            wrapped.get_by_key(&Value::Str("__vtable_tag".into())),
            Value::I32(314)
        ));
    }
}

#[cfg(test)]
mod class_key_identity {
    use super::*;
    use crate::hasher::hash_value;
    use crate::protocols::value_equals;
    use crate::Value;

    fn key_instance(id: i32) -> Value {
        let inner = Value::object_ref_from_map(
            HashMap::from([
                ("__type".into(), Value::Str("Key".into())),
                ("id".into(), Value::I32(id)),
            ]),
            true,
        );
        Value::mut_ref_cell_from_map(HashMap::from([
            ("__type".into(), Value::Str("MutRefCell".into())),
            ("__syx_ref".into(), inner),
        ]))
    }

    #[test]
    fn mut_ref_cell_dict_roundtrip_and_identity() {
        let a = key_instance(1);
        let b = key_instance(2);
        assert!(value_equals(&a, &a));
        assert!(!value_equals(&a, &b));
        assert_ne!(hash_value(&a), hash_value(&b));
        let mut map = HashMap::new();
        dict_insert(&mut map, a.clone(), Value::I32(10));
        dict_insert(&mut map, b.clone(), Value::I32(20));
        assert_eq!(dict_len(&map), 2);
        assert_eq!(dict_get(&map, &a).map(|v| v.as_i32()), Some(10));
        assert_eq!(dict_get(&map, &b).map(|v| v.as_i32()), Some(20));
        for k in dict_keys(&map) {
            assert!(dict_get(&map, &k).is_some());
        }
        // MutRefCell wrapper and bare inner ObjectRef share canonical identity.
        let inner_a = Value::mut_ref_cell_inner(&a).expect("inner");
        assert!(value_equals(&a, &inner_a));
        assert_eq!(dict_get(&map, &inner_a).map(|v| v.as_i32()), Some(10));
    }

    #[test]
    fn mut_ref_cell_set_identity_dedup() {
        let a = key_instance(1);
        let b = key_instance(1);
        let mut set = Vec::new();
        for item in [a.clone(), a.clone(), b.clone()] {
            if !set.iter().any(|x| value_equals(x, &item)) {
                set.push(item);
            }
        }
        assert_eq!(set.len(), 2, "same fields, distinct instances");
        assert!(set.iter().any(|x| value_equals(x, &a)));
        assert!(set.iter().any(|x| value_equals(x, &b)));
    }
}
