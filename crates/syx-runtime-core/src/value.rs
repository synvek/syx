use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

use crate::numeric::display_numeric;

#[derive(Debug, Clone)]
struct SharedCell(Arc<Mutex<HashMap<String, Value>>>);

impl PartialEq for SharedCell {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Clone)]
struct WeakCell(Weak<Mutex<HashMap<String, Value>>>);

impl PartialEq for WeakCell {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IteratorState {
    Static {
        items: Vec<Value>,
        index: usize,
    },
    Witness {
        obj: Box<Value>,
        next_fn: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapSlotKind {
    Array,
    String,
    Dict,
    Object,
    /// ADR-224 / M2-S1-02: Set values need HeapSlot for insert/remove write-back.
    Set,
    /// ADR-248 / M2-S1-03: Deque values need HeapSlot for push/pop write-back.
    Deque,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Simd {
        lanes: u8,
        element: syx_stdlib::NumericKind,
        bits: [u8; 64],
    },
    Bool(bool),
    /// Phase 139 L5: distinct Unicode scalar Character (single grapheme cluster).
    Character(String),
    Str(String),
    Nil,
    Array(Vec<Value>),
    Set(Vec<Value>),
    Dict(HashMap<String, Value>),
    Object(HashMap<String, Value>),
    /// Phase 55/56: class instance reference (`mutable` allows field writes).
    /// Wave A W537 / ADR-881: ObjectRef rc storage stripped; lifetime via SharedCell Arc
    /// + ManagedHeap dual-track (handle-drop Arc death; collect Type_deinit).
    ObjectRef {
        inner: SharedCell,
        mutable: bool,
        deiniting: bool,
    },
    /// Phase 75: unified heap slot backing Array/String/Dict/ObjectRef in ManagedHeap v4.
    HeapSlot {
        idx: usize,
        kind: HeapSlotKind,
    },
    /// Phase 103: weak class handle — does not bump SharedCell Arc or keep it alive.
    WeakRef(WeakCell),
    /// Legacy `&mut` borrow wrapper (non-class paths).
    MutRefCell(SharedCell),
    Iterator(Box<IteratorState>),
    Closure {
        fn_index: u32,
        captures: Vec<Value>,
    },
}

impl Value {
    pub fn as_i32(&self) -> i32 {
        self.as_i64() as i32
    }

    pub fn object_ref_from_map(map: HashMap<String, Value>, mutable: bool) -> Self {
        Value::ObjectRef {
            inner: SharedCell(Arc::new(Mutex::new(map))),
            mutable,
            deiniting: false,
        }
    }

    /// Wave A W527 / ADR-871: true when Type_deinit already marked this ObjectRef dead.
    pub fn object_ref_is_weak_dead(v: &Value) -> bool {
        let resolved = Self::dispatch_receiver_for_method(v.clone());
        if let Value::ObjectRef { inner, .. } = resolved {
            return inner.0.lock().ok().is_some_and(|map| {
                matches!(map.get("__syx_weak_dead"), Some(Value::Bool(true)))
            });
        }
        false
    }

    /// Mark class instance map so weak handles observe death (Arc may linger).
    pub fn mark_object_weak_dead(v: &Value) {
        let resolved = Self::dispatch_receiver_for_method(v.clone());
        if let Value::ObjectRef { inner, .. } = resolved {
            Self::mark_object_map_weak_dead(&inner.0);
        }
    }

    fn mark_object_map_weak_dead(arc: &Arc<Mutex<HashMap<String, Value>>>) {
        if let Ok(mut map) = arc.lock() {
            map.insert("__syx_weak_dead".into(), Value::Bool(true));
        }
    }

    /// Phase 103: create a weak handle from a class `ObjectRef` without retaining.
    /// Accepts bare `ObjectRef` or `MutRefCell` / heap wrappers (via `dispatch_receiver_for_method`).
    pub fn weak_from_object_ref(v: &Value) -> Value {
        let resolved = Self::dispatch_receiver_for_method(v.clone());
        match resolved {
            Value::ObjectRef { inner, .. } => Value::WeakRef(WeakCell(Arc::downgrade(&inner.0))),
            _ => Value::Nil,
        }
    }

    /// Phase 103: upgrade weak to a strong `ObjectRef`, or `Nil` when dead.
    pub fn weak_load(v: &Value) -> Value {
        match v {
            Value::WeakRef(WeakCell(w)) => w.upgrade().map_or(Value::Nil, |arc| {
                let map = arc.lock().ok().map(|g| g.clone()).unwrap_or_default();
                if matches!(map.get("__syx_weak_dead"), Some(Value::Bool(true))) {
                    return Value::Nil;
                }
                Value::object_ref_from_map(map, false)
            }),
            _ => Value::Nil,
        }
    }

    /// Phase 103: true when weak still points at a live (not deinited) object.
    pub fn weak_is_alive(v: &Value) -> bool {
        match v {
            Value::WeakRef(WeakCell(w)) => w.upgrade().is_some_and(|arc| {
                arc.lock().ok().is_some_and(|map| {
                    !matches!(map.get("__syx_weak_dead"), Some(Value::Bool(true)))
                })
            }),
            _ => false,
        }
    }

    /// Stable identity for mark-sweep GC (VM pool tracing).
    pub fn object_ref_identity(v: &Value) -> Option<usize> {
        match v {
            Value::ObjectRef { inner, .. } => Some(Arc::as_ptr(&inner.0) as usize),
            _ => None,
        }
    }

    /// Wave A W502 / ADR-846: stable SharedCell Arc identity for MutRefCell
    /// ManagedHeap + `heap_slots` dual-register dedup (≠ full MutRefCell migrate).
    pub fn mut_ref_cell_identity(v: &Value) -> Option<usize> {
        match v {
            Value::MutRefCell(SharedCell(arc)) => Some(Arc::as_ptr(arc) as usize),
            _ => None,
        }
    }

    /// Canonical identity for Hashable class instances used as Set/Dict keys:
    /// unwrap `MutRefCell` → inner `ObjectRef` Arc, else bare `ObjectRef` / `MutRefCell`.
    pub fn class_instance_identity(v: &Value) -> Option<usize> {
        let v = match v {
            Value::HeapSlot { .. } => Self::resolve_heap_slot(v),
            other => other.clone(),
        };
        if let Some(inner) = Self::mut_ref_cell_inner(&v) {
            if let Some(id) = Self::object_ref_identity(&inner) {
                return Some(id);
            }
            if let Some(id) = Self::class_instance_identity(&inner) {
                return Some(id);
            }
        }
        Self::object_ref_identity(&v).or_else(|| Self::mut_ref_cell_identity(&v))
    }

    /// Field values reachable from a live `ObjectRef` (for GC marking).
    pub fn object_ref_field_values(v: &Value) -> Option<Vec<Value>> {
        match v {
            Value::ObjectRef { inner, .. } => inner
                .0
                .lock()
                .ok()
                .map(|map| map.values().cloned().collect()),
            _ => None,
        }
    }

    /// User-visible fields of an `ObjectRef` (excludes `__*` metadata keys).
    pub fn object_ref_user_fields(v: &Value) -> Option<Vec<(String, Value)>> {
        match v {
            Value::ObjectRef { inner, .. } => inner.0.lock().ok().map(|map| {
                map.iter()
                    .filter(|(k, _)| !k.starts_with("__"))
                    .map(|(k, val)| (k.clone(), val.clone()))
                    .collect()
            }),
            _ => None,
        }
    }

    pub fn mut_ref_cell_inner(v: &Value) -> Option<Value> {
        match v {
            Value::MutRefCell(cell) => cell
                .0
                .lock()
                .ok()
                .and_then(|map| map.get("__syx_ref").cloned()),
            Value::ObjectRef { inner, .. } => {
                let map = inner.0.lock().ok()?;
                if map
                    .get("__type")
                    .and_then(Self::type_name_from_meta_value)
                    .as_deref()
                    == Some("MutRefCell")
                {
                    map.get("__syx_ref").cloned()
                } else {
                    None
                }
            }
            Value::Object(map) | Value::Dict(map) => {
                if map
                    .get("__type")
                    .and_then(Self::type_name_from_meta_value)
                    .as_deref()
                    == Some("MutRefCell")
                {
                    map.get("__syx_ref").cloned()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn mut_ref_cell_from_map(map: HashMap<String, Value>) -> Self {
        Value::MutRefCell(SharedCell(Arc::new(Mutex::new(map))))
    }

    pub fn is_object_ref(&self) -> bool {
        matches!(self, Value::ObjectRef { .. })
    }

    pub fn heap_v4_enabled() -> bool {
        std::env::var("SYX_HEAP_V4")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
    }

    pub fn heap_slot_index(v: &Value) -> Option<usize> {
        match v {
            Value::HeapSlot { idx, .. } => Some(*idx),
            _ => None,
        }
    }

    pub fn rewrite_heap_slot(v: &mut Value, map: &std::collections::HashMap<usize, usize>) {
        if let Value::HeapSlot { idx, .. } = v {
            if let Some(&new_idx) = map.get(idx) {
                *idx = new_idx;
            }
        }
    }

    pub fn resolve_heap_slot(v: &Value) -> Value {
        if let Some(idx) = Self::heap_slot_index(v) {
            if let Ok(heap) = crate::global_heap().lock() {
                if let Some(inner) = heap.get(idx) {
                    return inner.clone();
                }
            }
        }
        v.clone()
    }

    /// Phase 82: unwrap HeapSlot before host/collection operations (four-end parity).
    pub fn materialize_receiver(v: Value) -> Value {
        match v {
            Value::HeapSlot { .. } => Self::resolve_heap_slot(&v),
            other => other,
        }
    }

    /// Phase 82: write mutated value back to HeapSlot (four-end parity).
    pub fn write_back_receiver(outer: Value, inner: Value) -> Value {
        Self::write_back_heap_slot(outer, inner)
    }

    /// Phase 82: String complete offheap (default on with heap v11).
    pub fn heap_v11_string_offheap_full_enabled() -> bool {
        std::env::var("SYX_HEAP_V11_STRING_OFFHEAP_FULL")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(Self::heap_v10_string_offheap_enabled())
    }

    /// Phase 82: long string offheap threshold in bytes (default 64).
    pub fn string_offheap_threshold() -> usize {
        std::env::var("SYX_STRING_OFFHEAP_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(64)
    }

    /// Unwrap inline `Array` or heap-backed `HeapSlot` array (Phase 76).
    pub fn materialize_array(v: Value) -> Option<Vec<Value>> {
        let v = match v {
            Value::HeapSlot { .. } => Self::resolve_heap_slot(&v),
            other => other,
        };
        match v {
            Value::Array(items) => Some(items),
            _ => None,
        }
    }

    /// Unwrap inline `Dict` or heap-backed `HeapSlot` dict (Phase 77).
    pub fn materialize_dict(v: Value) -> Option<std::collections::HashMap<String, Value>> {
        let v = match v {
            Value::HeapSlot { .. } => Self::resolve_heap_slot(&v),
            other => other,
        };
        match v {
            Value::Dict(map) | Value::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Phase 79: unwrap heap-backed short strings (inline in slot when enabled).
    pub fn materialize_str(v: Value) -> Option<String> {
        let v = match v {
            Value::HeapSlot { .. } => Self::resolve_heap_slot(&v),
            other => other,
        };
        match v {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Phase 79: when heap v8 inline strings enabled, small strings stay as Str without extra copy.
    pub fn heap_v8_string_inline_enabled() -> bool {
        std::env::var("SYX_HEAP_V8_STRING_INLINE")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
    }

    /// Phase 80: String full inline GA (extends heap v8).
    pub fn heap_v9_string_full_inline_enabled() -> bool {
        std::env::var("SYX_HEAP_V9_STRING_FULL_INLINE")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(Self::heap_v8_string_inline_enabled())
    }

    /// Phase 81: long String offheap MVP (short strings stay inline).
    pub fn heap_v10_string_offheap_enabled() -> bool {
        std::env::var("SYX_HEAP_V10_STRING_OFFHEAP")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
    }

    /// Phase 81: offload long strings to offheap slot; short strings inline.
    pub fn maybe_offheap_string(value: Value, kind: HeapSlotKind) -> Value {
        if Self::heap_v11_string_offheap_full_enabled() || Self::heap_v10_string_offheap_enabled() {
            if let Value::Str(ref s) = value {
                if s.len() > Self::string_offheap_threshold() {
                    return Self::alloc_heap_slot(value, kind);
                }
            }
        }
        Self::maybe_inline_heap_string(value, kind)
    }

    pub fn maybe_inline_heap_string(value: Value, kind: HeapSlotKind) -> Value {
        if Self::heap_v9_string_full_inline_enabled() {
            if let Value::Str(ref s) = value {
                if s.len() <= 15 {
                    return Self::alloc_heap_slot(value, kind);
                }
            }
        }
        Self::alloc_heap_slot(value, kind)
    }

    pub fn register_actor_id(self, id: i64) -> Self {
        match self {
            Value::Object(mut map) => {
                map.insert("__actor_id".into(), Value::I32(id as i32));
                Value::Object(map)
            }
            Value::ObjectRef {
                inner,
                mutable,
                deiniting,
            } => {
                if let Ok(mut map) = inner.0.lock() {
                    map.insert("__actor_id".into(), Value::I32(id as i32));
                }
                Value::ObjectRef {
                    inner,
                    mutable,
                    deiniting,
                }
            }
            other => other,
        }
    }

    pub fn actor_id(&self) -> Option<i64> {
        match self {
            Value::Object(map) => map.get("__actor_id").and_then(|id| match id {
                Value::I32(n) => Some(*n as i64),
                Value::I64(n) => Some(*n),
                _ => None,
            }),
            Value::HeapSlot { .. } => Self::resolve_heap_slot(self).actor_id(),
            Value::MutRefCell(_) => Self::mut_ref_cell_inner(self).and_then(|inner| inner.actor_id()),
            Value::ObjectRef { inner, .. } => {
                let map = inner.0.lock().ok()?;
                if let Some(id) = map.get("__actor_id").and_then(|id| match id {
                    Value::I32(n) => Some(*n as i64),
                    Value::I64(n) => Some(*n),
                    _ => None,
                }) {
                    return Some(id);
                }
                // Heap-v4 may mis-tag MutRefCell wraps as ObjectRef; follow __syx_ref.
                map.get("__syx_ref").cloned().and_then(|inner| inner.actor_id())
            }
            _ => None,
        }
    }

    /// Write value back to heap slot if `outer` is HeapSlot (Phase 77).
    pub fn write_back_heap_slot(outer: Value, inner: Value) -> Value {
        if let Value::HeapSlot { idx, kind } = outer {
            if let Ok(mut heap) = crate::global_heap().lock() {
                if let Some(slot) = heap.get_mut(idx) {
                    *slot = inner;
                }
            }
            Value::HeapSlot { idx, kind }
        } else {
            inner
        }
    }

    pub fn alloc_heap_slot(value: Value, kind: HeapSlotKind) -> Value {
        if !Self::heap_v4_enabled() {
            return value;
        }
        if let Ok(mut heap) = crate::global_heap().lock() {
            let idx = heap.alloc(value);
            Value::HeapSlot { idx, kind }
        } else {
            value
        }
    }

    pub fn object_ref_deep_equal(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Value::ObjectRef { inner: a, .. }, Value::ObjectRef { inner: b, .. }) => {
                if Arc::ptr_eq(&a.0, &b.0) {
                    Some(true)
                } else {
                    match (a.0.lock(), b.0.lock()) {
                        (Ok(x), Ok(y)) => Some(crate::protocols::dict_equals_maps(&x, &y)),
                        _ => Some(false),
                    }
                }
            }
            _ => None,
        }
    }

    fn type_name_from_meta_value(v: &Value) -> Option<String> {
        match v {
            Value::Str(s) => Some(s.clone()),
            Value::HeapSlot { .. } => match Self::resolve_heap_slot(v) {
                Value::Str(s) => Some(s),
                _ => None,
            },
            _ => None,
        }
    }

    /// Unwrap `MutRefCell` handles (`Value::MutRefCell` or `ObjectRef` with `__type` MutRefCell).
    pub fn dispatch_receiver_for_method(value: Value) -> Value {
        let mut current = Value::materialize_receiver(value);
        for _ in 0..3 {
            let is_mut_cell_wrapper = match &current {
                Value::MutRefCell(_) => true,
                Value::ObjectRef { inner, .. } => inner.0.lock().ok().and_then(|map| {
                    map.get("__type")
                        .and_then(Self::type_name_from_meta_value)
                }).as_deref() == Some("MutRefCell"),
                Value::Object(map) | Value::Dict(map) => map
                    .get("__type")
                    .and_then(Self::type_name_from_meta_value)
                    .as_deref()
                    == Some("MutRefCell"),
                _ => false,
            };
            if !is_mut_cell_wrapper {
                break;
            }
            let inner = current.get_by_key(&Value::Str("__syx_ref".into()));
            if matches!(inner, Value::Nil) {
                break;
            }
            current = Value::materialize_receiver(inner);
        }
        current
    }

    pub fn object_ref_type_name(&self) -> Option<String> {
        match self {
            Value::HeapSlot { .. } => {
                Self::resolve_heap_slot(self).object_ref_type_name()
            }
            Value::ObjectRef { inner, .. } => inner.0.lock().ok().and_then(|map| {
                if let Some(inner_val) = map.get("__syx_ref") {
                    if map
                        .get("__type")
                        .and_then(Self::type_name_from_meta_value)
                        .as_deref()
                        == Some("MutRefCell")
                    {
                        if let Some(name) = inner_val.object_ref_type_name() {
                            return Some(name);
                        }
                    }
                }
                map.get("__type").and_then(Self::type_name_from_meta_value)
            }),
            Value::MutRefCell(cell) => cell.0.lock().ok().and_then(|map| {
                if let Some(inner) = map.get("__syx_ref") {
                    if let Some(name) = inner.object_ref_type_name() {
                        return Some(name);
                    }
                }
                map.get("__type").and_then(|v| {
                    Self::type_name_from_meta_value(v).filter(|s| s.as_str() != "MutRefCell")
                })
            }),
            Value::Object(map) | Value::Dict(map) => {
                if map
                    .get("__type")
                    .and_then(Self::type_name_from_meta_value)
                    .as_deref()
                    == Some("MutRefCell")
                {
                    if let Some(inner) = map.get("__syx_ref") {
                        if let Some(name) = inner.object_ref_type_name() {
                            return Some(name);
                        }
                    }
                }
                map.get("__type").and_then(Self::type_name_from_meta_value)
            }
            _ => None,
        }
    }

    pub fn get_by_key(&self, key: &Value) -> Value {
        match self {
            Value::HeapSlot { .. } => Self::resolve_heap_slot(self).get_by_key(key),
            Value::Dict(map) => {
                // Align with Object: missing `__syx_ref` means already unwrapped
                // (MemberAssign may ObjectGet that key before ObjectSet).
                let k = key.as_str();
                if k == "__syx_ref" && !map.contains_key("__syx_ref") {
                    return self.clone();
                }
                crate::dict::dict_get(map, key)
                    .cloned()
                    .unwrap_or(Value::Nil)
            }
            _ => self.mut_ref_cell_get_field(&key.as_str()),
        }
    }

    pub fn mut_ref_cell_get_field(&self, key: &str) -> Value {
        match self {
            Value::HeapSlot { .. } => {
                // Wave A W547 / ADR-891: HeapSlot-wrapped ObjectRef (DictBuild
                // heap-primary) and HeapSlot `__syx_ref` inside MutRefCell.
                Self::resolve_heap_slot(self).mut_ref_cell_get_field(key)
            }
            Value::ObjectRef { inner, .. } => {
                if let Ok(map) = inner.0.lock() {
                    if map
                        .get("__type")
                        .and_then(Self::type_name_from_meta_value)
                        .as_deref()
                        == Some("MutRefCell")
                    {
                        if let Some(inner_val) = map.get("__syx_ref") {
                            if key != "__syx_ref" && key != "__type" {
                                return inner_val.mut_ref_cell_get_field(key);
                            }
                        }
                    }
                }
                let got = inner
                    .0
                    .lock()
                    .ok()
                    .and_then(|map| map.get(key).cloned());
                // Call sites often unwrap MutRefCell before Call, while method bodies
                // also unwrap `self`; treat missing `__syx_ref` as already-unwrapped.
                if key == "__syx_ref" && got.is_none() {
                    return self.clone();
                }
                got.unwrap_or(Value::Nil)
            }
            Value::MutRefCell(cell) => {
                let inner = cell
                    .0
                    .lock()
                    .ok()
                    .and_then(|map| map.get("__syx_ref").cloned());
                if key == "__syx_ref" || key == "__type" {
                    return cell
                        .0
                        .lock()
                        .ok()
                        .and_then(|map| map.get(key).cloned())
                        .unwrap_or(Value::Nil);
                }
                if let Some(inner) = inner {
                    return inner.mut_ref_cell_get_field(key);
                }
                Value::Nil
            }
            Value::Object(map) | Value::Dict(map) => {
                if key == "__syx_ref" && !map.contains_key(key) {
                    return self.clone();
                }
                map.get(key).cloned().unwrap_or(Value::Nil)
            }
            _ => Value::Nil,
        }
    }

    pub fn mut_ref_cell_set_field(obj: Value, key: &str, val: Value) -> Value {
        match obj {
            Value::HeapSlot { .. } => {
                // Wave A W547 / ADR-891: HeapSlot-wrapped ObjectRef (DictBuild
                // heap-primary birth) must mutate through write-back.
                let inner = Self::resolve_heap_slot(&obj);
                let updated = Self::mut_ref_cell_set_field(inner, key, val);
                Self::write_back_heap_slot(obj, updated)
            }
            Value::ObjectRef {
                inner,
                mutable: _,
                deiniting,
            } => {
                // Always allow field writes on live ObjectRef handles (call-site unwrap
                // and DI boot assign rely on mutating the shared map).
                if let Ok(mut map) = inner.0.lock() {
                    map.insert(key.to_string(), val);
                }
                Value::ObjectRef {
                    inner,
                    mutable: true,
                    deiniting,
                }
            }
            Value::MutRefCell(cell) => {
                if key == "__syx_ref" || key == "__type" {
                    if let Ok(mut map) = cell.0.lock() {
                        map.insert(key.to_string(), val);
                    }
                    return Value::MutRefCell(cell);
                }
                if let Ok(mut map) = cell.0.lock() {
                    if let Some(inner) = map.get("__syx_ref").cloned() {
                        let updated = Value::mut_ref_cell_set_field(inner, key, val);
                        map.insert("__syx_ref".to_string(), updated);
                    }
                }
                Value::MutRefCell(cell)
            }
            Value::Object(mut map) => {
                map.insert(key.to_string(), val);
                Value::Object(map)
            }
            // DictBuild without `__type` births `Value::Dict`; Object field
            // writes (ObjectSet) must mutate it the same way as `Object`.
            Value::Dict(mut map) => {
                map.insert(key.to_string(), val);
                Value::Dict(map)
            }
            other => other,
        }
    }

    pub fn is_mut_ref_cell(&self) -> bool {
        matches!(self, Value::MutRefCell(_))
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            Value::Nil => false,
            _ => true,
        }
    }

    pub fn as_str(&self) -> String {
        let v = match self {
            Value::HeapSlot { .. } => Self::resolve_heap_slot(self),
            other => other.clone(),
        };
        match &v {
            Value::Str(s) => s.clone(),
            Value::Character(c) => c.clone(),
            v if display_numeric(v).is_some() => display_numeric(v).unwrap(),
            _ => String::new(),
        }
    }
}

pub fn cooperative_yield() {
    std::thread::yield_now();
}

#[cfg(test)]
mod phase125_mutref_field_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn mut_ref_cell_field_set_persists_across_clone() {
        let mut inner = HashMap::new();
        inner.insert("__type".into(), Value::Str("App".into()));
        inner.insert("logger".into(), Value::Nil);
        let obj = Value::object_ref_from_map(inner, true);
        let mut cell_map = HashMap::new();
        cell_map.insert("__type".into(), Value::Str("MutRefCell".into()));
        cell_map.insert("__syx_ref".into(), obj);
        let cell = Value::mut_ref_cell_from_map(cell_map);
        let caller = cell.clone();
        let unwrapped = cell.mut_ref_cell_get_field("__syx_ref");
        let updated = Value::mut_ref_cell_set_field(
            unwrapped,
            "logger",
            Value::Str("fast".into()),
        );
        let _ = updated;
        let got = caller.mut_ref_cell_get_field("logger");
        assert_eq!(got.as_str(), "fast", "got={got:?}");
    }
}


#[cfg(test)]
mod weak_ref_tests {
    use super::*;

    #[test]
    fn weak_from_object_ref_alive() {
        let o = Value::object_ref_from_map(HashMap::from([("n".into(), Value::I64(1))]), true);
        let w = Value::weak_from_object_ref(&o);
        assert!(Value::weak_is_alive(&w), "alive while strong exists");
        assert!(!matches!(Value::weak_load(&w), Value::Nil));
        drop(o);
        assert!(!Value::weak_is_alive(&w), "dead after drop");
    }

    #[test]
    fn weak_from_mut_ref_cell_wrapper() {
        let o = Value::object_ref_from_map(HashMap::from([("n".into(), Value::I64(1))]), true);
        let mut cell_map = HashMap::new();
        cell_map.insert("__type".into(), Value::Str("MutRefCell".into()));
        cell_map.insert("__syx_ref".into(), o);
        let cell = Value::mut_ref_cell_from_map(cell_map);
        let w = Value::weak_from_object_ref(&cell);
        assert!(Value::weak_is_alive(&w), "alive via MutRefCell wrapper");
        drop(cell);
        assert!(!Value::weak_is_alive(&w), "dead after MutRefCell drop");
    }
}
