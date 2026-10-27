use syx_stdlib::{is_integer, NumericKind, PLATFORM_INT};

use crate::Value;

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

#[derive(Debug, Clone)]
pub struct HasherState {
    hash: u64,
}

impl HasherState {
    pub fn new() -> Self {
        Self { hash: FNV_OFFSET }
    }

    pub fn combine_u64(&mut self, v: u64) {
        self.hash ^= v;
        self.hash = self.hash.wrapping_mul(FNV_PRIME);
    }

    pub fn finalize(&self) -> u64 {
        self.hash
    }
}

impl Default for HasherState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn hasher_value(state: &HasherState) -> Value {
    let mut map = std::collections::HashMap::new();
    map.insert("__hasher".into(), Value::Bool(true));
    map.insert(
        "__hasher_state".into(),
        Value::I64(state.hash as i64),
    );
    Value::Object(map)
}

pub fn hasher_from_value(v: &Value) -> HasherState {
    if let Value::Object(map) = v {
        if let Some(Value::I64(h)) = map.get("__hasher_state") {
            return HasherState { hash: *h as u64 };
        }
    }
    HasherState::new()
}

pub fn combine_value(state: &mut HasherState, v: &Value) {
    let v = match v {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(v),
        other => other.clone(),
    };
    match &v {
        Value::Nil => state.combine_u64(0),
        Value::Bool(b) => state.combine_u64(if *b { 1 } else { 0 }),
        Value::Str(s) => {
            state.combine_u64(2);
            for b in s.bytes() {
                state.combine_u64(b as u64);
            }
        }
        Value::Array(items) => {
            state.combine_u64(10);
            state.combine_u64(items.len() as u64);
            for item in items {
                combine_value(state, item);
            }
        }
        Value::Set(items) => {
            state.combine_u64(11);
            state.combine_u64(items.len() as u64);
            for item in items {
                combine_value(state, item);
            }
        }
        Value::Dict(map) => {
            state.combine_u64(12);
            state.combine_u64(map.len() as u64);
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            for k in keys {
                state.combine_u64(3);
                for b in k.bytes() {
                    state.combine_u64(b as u64);
                }
                if let Some(val) = map.get(k) {
                    combine_value(state, val);
                }
            }
        }
        Value::Object(map) => {
            state.combine_u64(13);
            if let Some(Value::I64(id)) = map.get(crate::protocols::OBJECT_ID_KEY) {
                state.combine_u64(*id as u64);
            }
            let mut keys: Vec<_> = map
                .keys()
                .filter(|k| *k != crate::protocols::OBJECT_ID_KEY)
                .collect();
            keys.sort();
            for k in keys {
                if let Some(val) = map.get(k) {
                    combine_value(state, val);
                }
            }
        }
        // Class instances: identity hash so Dictionary keys do not all collide.
        // MutRefCell wrappers and bare ObjectRef share the same canonical identity.
        Value::ObjectRef { .. } => {
            state.combine_u64(14);
            state.combine_u64(Value::class_instance_identity(&v).unwrap_or(0) as u64);
        }
        _ => {
            if let Some(id) = Value::class_instance_identity(&v) {
                state.combine_u64(14);
                state.combine_u64(id as u64);
            } else if let Some((kind, bits)) = crate::numeric::numeric_bits(&v) {
                state.combine_u64(20);
                if is_integer(kind) {
                    state.combine_u64(PLATFORM_INT.to_byte() as u64);
                    state.combine_u64(v.as_i64() as u64);
                } else {
                    state.combine_u64(kind.to_byte() as u64);
                    state.combine_u64(bits);
                }
            }
        }
    }
}

pub fn finalize_state(state: &HasherState) -> u64 {
    state.finalize()
}

pub fn hash_value(v: &Value) -> u64 {
    let mut state = HasherState::new();
    combine_value(&mut state, v);
    state.finalize()
}

pub fn hash_kind_tag(kind: NumericKind) -> u64 {
    kind.to_byte() as u64
}
