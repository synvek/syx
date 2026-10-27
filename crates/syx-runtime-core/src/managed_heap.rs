//! Phase 73–75: unified MirRuntime managed heap (mark-sweep + compact slot pool).

use std::collections::HashSet;
use std::io::Write;
use std::sync::{LazyLock, Mutex};

use crate::Value;

#[derive(Debug, Default)]
pub struct ManagedHeap {
    slots: Vec<Value>,
    free_list: Vec<usize>,
    high_water: usize,
    collect_cycles: u32,
    compact_cycles: u32,
    bytes_reclaimed: u64,
    /// Phase 80 Heap v9: young generation slot values.
    young_gen: Vec<Value>,
    /// Phase 80 Heap v9: old generation slot values.
    old_gen: Vec<Value>,
    /// Phase 81 Heap v10: card table for write barrier.
    card_table: Vec<bool>,
    write_barrier_hits: u64,
    young_promotions: u32,
}

static GLOBAL_HEAP: std::sync::OnceLock<std::sync::Mutex<ManagedHeap>> = std::sync::OnceLock::new();

impl ManagedHeap {
    pub fn enabled() -> bool {
        std::env::var("SYX_UNIFIED_HEAP")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true)
    }

    pub fn compact_enabled() -> bool {
        std::env::var("SYX_HEAP_COMPACT")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
    }

    /// Phase 80: Heap v9 young/old generational GC (default on).
    pub fn heap_v9_generational_enabled() -> bool {
        std::env::var("SYX_HEAP_V9_GENERATIONAL")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(true)
    }

    /// Phase 81: Heap v10 true promotion + cross-process bridge (default on).
    pub fn heap_v10_enabled() -> bool {
        std::env::var("SYX_HEAP_V10")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(Self::heap_v9_generational_enabled())
    }

    /// Phase 82: Heap v11 prod bridge + complete string offheap (default on).
    pub fn heap_v11_enabled() -> bool {
        std::env::var("SYX_HEAP_V11")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(Self::heap_v10_enabled())
    }

    /// Phase 82: cross-process shared arena bridge prod default.
    pub fn heap_v11_bridge_prod_enabled() -> bool {
        std::env::var("SYX_HEAP_V11_BRIDGE_PROD")
            .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
            .unwrap_or(Self::heap_v11_enabled())
    }

    pub fn card_table_len(&self) -> usize {
        self.card_table.len()
    }

    pub fn young_gen_len(&self) -> usize {
        self.young_gen.len()
    }

    pub fn old_gen_len(&self) -> usize {
        self.old_gen.len()
    }

    pub fn write_barrier_hits(&self) -> u64 {
        self.write_barrier_hits
    }

    pub fn young_promotions(&self) -> u32 {
        self.young_promotions
    }

    fn mark_card(&mut self, slot_idx: usize) {
        if self.card_table.len() <= slot_idx {
            self.card_table.resize(slot_idx + 1, false);
        }
        self.card_table[slot_idx] = true;
    }

    fn write_barrier(&mut self, parent_gen: bool, child_in_young: bool) {
        if parent_gen && child_in_young {
            self.write_barrier_hits = self.write_barrier_hits.saturating_add(1);
            if Self::heap_v10_enabled() {
                self.mark_card(self.old_gen.len().saturating_add(self.young_gen.len()));
            }
        }
    }

    fn alloc_generational(&mut self, value: Value) -> usize {
        let idx = if let Some(i) = self.free_list.pop() {
            self.slots[i] = value.clone();
            i
        } else {
            self.slots.push(value.clone());
            self.slots.len() - 1
        };
        self.young_gen.push(value);
        self.high_water = self.high_water.max(self.slots.len());
        idx
    }

    fn collect_generational(&mut self, roots: &[Value]) -> Vec<Value> {
        let young_limit = if Self::heap_v10_enabled() { 64 } else { 96 };
        if self.young_gen.len() > young_limit {
            self.old_gen.append(&mut self.young_gen);
            self.young_promotions = self.young_promotions.saturating_add(1);
            record_gc_churn_stdout("promote");
            if Self::heap_v10_enabled() {
                self.card_table.clear();
            }
        }
        for root in roots {
            self.write_barrier(!self.old_gen.is_empty(), !self.young_gen.is_empty());
            if self.young_gen.len() < 128 {
                self.young_gen.push(root.clone());
            }
        }
        let mut swept = self.collect_mark_sweep(roots);
        if heap_v13_generational() && !self.old_gen.is_empty() {
            let mut roots_mut: Vec<Value> = roots.to_vec();
            swept.extend(self.collect_and_compact(&mut roots_mut));
            gc_gen_compact_record();
        }
        swept
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn high_water(&self) -> usize {
        self.high_water
    }

    pub fn collect_cycles(&self) -> u32 {
        self.collect_cycles
    }

    pub fn compact_cycles(&self) -> u32 {
        self.compact_cycles
    }

    pub fn bytes_reclaimed(&self) -> u64 {
        self.bytes_reclaimed
    }

    pub fn fragmentation_ratio(&self) -> f64 {
        if self.slots.is_empty() {
            return 0.0;
        }
        let live = self
            .slots
            .iter()
            .filter(|s| !matches!(s, Value::Nil))
            .count();
        if live == 0 {
            return 0.0;
        }
        1.0 - (live as f64 / self.slots.len() as f64)
    }

    pub fn alloc(&mut self, value: Value) -> usize {
        if Self::heap_v9_generational_enabled() {
            return self.alloc_generational(value);
        }
        let idx = if let Some(i) = self.free_list.pop() {
            self.slots[i] = value;
            i
        } else {
            self.slots.push(value);
            self.slots.len() - 1
        };
        self.high_water = self.high_water.max(self.slots.len());
        idx
    }

    pub fn get(&self, idx: usize) -> Option<&Value> {
        self.slots.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.slots.get_mut(idx)
    }

    /// Wave A W525 / ADR-869: StoreLocal dual-track death-path — Nil a slot and
    /// return it to `free_list`. Does **not** run Type_deinit / ObjectRef RC.
    pub fn free_slot(&mut self, idx: usize) {
        if idx >= self.slots.len() {
            return;
        }
        if matches!(self.slots[idx], Value::Nil) {
            return;
        }
        self.slots[idx] = Value::Nil;
        self.free_list.push(idx);
    }

    fn trace(&self, v: &Value, marked_slots: &mut HashSet<usize>, marked_objs: &mut HashSet<usize>, stack: &mut Vec<Value>) {
        if let Value::HeapSlot { idx, .. } = v {
            if marked_slots.insert(*idx) {
                if let Some(inner) = self.get(*idx) {
                    stack.push(inner.clone());
                }
            }
            return;
        }
        if let Some(ptr) = Value::object_ref_identity(v) {
            if marked_objs.insert(ptr) {
                if let Some(fields) = Value::object_ref_field_values(v) {
                    stack.extend(fields);
                }
            }
            return;
        }
        if let Some(inner) = Value::mut_ref_cell_inner(v) {
            stack.push(inner);
            return;
        }
        match v {
            Value::Array(items) | Value::Set(items) => stack.extend(items.clone()),
            Value::Object(map) | Value::Dict(map) => {
                stack.extend(map.values().cloned());
            }
            _ => {}
        }
    }

    /// Wave A W527 / ADR-871: mark-sweep; returns swept `ObjectRef` values for
    /// Type_deinit. `free_slot` still does **not** run Type_deinit.
    pub fn collect(&mut self, roots: &[Value]) -> Vec<Value> {
        if Self::heap_v9_generational_enabled() {
            return self.collect_generational(roots);
        }
        self.collect_mark_sweep(roots)
    }

    fn collect_mark_sweep(&mut self, roots: &[Value]) -> Vec<Value> {
        let mut marked_slots = HashSet::new();
        let mut marked_objs = HashSet::new();
        let mut stack: Vec<Value> = roots.to_vec();
        while let Some(v) = stack.pop() {
            self.trace(&v, &mut marked_slots, &mut marked_objs, &mut stack);
        }
        let mut swept = Vec::new();
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if matches!(slot, Value::Nil) {
                continue;
            }
            let live = match slot {
                Value::HeapSlot { idx, .. } => marked_slots.contains(idx),
                Value::ObjectRef { .. } => Value::object_ref_identity(slot)
                    .is_some_and(|p| marked_objs.contains(&p)),
                Value::Array(_) | Value::Str(_) | Value::Dict(_) | Value::Object(_) => {
                    marked_slots.contains(&i)
                }
                _ => true,
            };
            if !live {
                if matches!(slot, Value::ObjectRef { .. }) {
                    swept.push(std::mem::replace(slot, Value::Nil));
                } else {
                    *slot = Value::Nil;
                }
                self.free_list.push(i);
            }
        }
        self.collect_cycles = self.collect_cycles.saturating_add(1);
        swept
    }

    pub fn collect_and_compact(&mut self, roots: &mut [Value]) -> Vec<Value> {
        let swept = self.collect(roots);
        let forwarding = self.compact_internal();
        Self::rewrite_slot_indices(roots, &forwarding);
        swept
    }

    /// Phase 75: slide live slots to front; returns old_index -> new_index map.
    pub fn compact(&mut self) -> Vec<(usize, usize)> {
        self.compact_internal()
    }

    fn compact_internal(&mut self) -> Vec<(usize, usize)> {
        let before_len = self.slots.len();
        let mut forwarding = Vec::new();
        let mut write = 0usize;
        for read in 0..self.slots.len() {
            if matches!(self.slots[read], Value::Nil) {
                continue;
            }
            if write != read {
                forwarding.push((read, write));
                self.slots[write] = std::mem::replace(&mut self.slots[read], Value::Nil);
            }
            write += 1;
        }
        self.slots.truncate(write);
        self.free_list.clear();
        let reclaimed = before_len.saturating_sub(write);
        self.bytes_reclaimed = self.bytes_reclaimed.saturating_add(reclaimed as u64);
        self.compact_cycles = self.compact_cycles.saturating_add(1);
        forwarding
    }

    pub fn rewrite_slot_indices(values: &mut [Value], forwarding: &[(usize, usize)]) {
        if forwarding.is_empty() {
            return;
        }
        let map: std::collections::HashMap<usize, usize> = forwarding.iter().copied().collect();
        for v in values.iter_mut() {
            Value::rewrite_heap_slot(v, &map);
            if let Value::HeapSlot { idx, .. } = v {
                if let Some(&new_idx) = map.get(idx) {
                    *idx = new_idx;
                }
            }
        }
    }
}

pub fn global_heap() -> &'static std::sync::Mutex<ManagedHeap> {
    GLOBAL_HEAP.get_or_init(|| std::sync::Mutex::new(ManagedHeap::default()))
}

pub fn collect_global(roots: &[Value]) -> Vec<Value> {
    let mut swept = Vec::new();
    if ManagedHeap::enabled() {
        if let Ok(mut h) = global_heap().lock() {
            swept = h.collect(roots);
        }
        record_gc_churn_stdout("collect");
    }
    swept
}

pub fn compact_global(roots: &mut [Value]) -> Vec<Value> {
    if !ManagedHeap::enabled() || !ManagedHeap::compact_enabled() {
        return Vec::new();
    }
    if let Ok(mut h) = global_heap().lock() {
        return h.collect_and_compact(roots);
    }
    Vec::new()
}

static RELOCATE_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

static GC_CHURN_STDOUT: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Phase 95: emit `gc:churn:{reason}` to stdout for quad parity gates.
pub fn record_gc_churn_stdout(reason: &str) {
    let line = format!("gc:churn:{reason}");
    let _ = writeln!(std::io::stdout(), "{line}");
    let _ = std::io::stdout().flush();
    if let Ok(mut g) = GC_CHURN_STDOUT.lock() {
        g.push(line);
    }
}

pub fn gc_churn_stdout_lines() -> Vec<String> {
    GC_CHURN_STDOUT.lock().map(|g| g.clone()).unwrap_or_default()
}

pub fn reset_gc_churn_stdout() {
    if let Ok(mut g) = GC_CHURN_STDOUT.lock() {
        g.clear();
    }
}

/// Phase 78: compact + rewrite extern refs / slot indices (Heap v7 relocating GC).
pub fn relocate_global(roots: &mut [Value]) {
    if !ManagedHeap::enabled() {
        return;
    }
    let _ = compact_global(roots);
    for v in roots.iter_mut() {
        *v = Value::resolve_heap_slot(v);
    }
    RELOCATE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    record_gc_churn_stdout("relocate");
}

pub fn relocate_count() -> u64 {
    RELOCATE_COUNT.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn reset_relocate_count() {
    RELOCATE_COUNT.store(0, std::sync::atomic::Ordering::Relaxed);
}

/// Phase 80: Heap v9 generational GC enabled (doctor / CI gate).
pub fn heap_v9_generational() -> bool {
    ManagedHeap::heap_v9_generational_enabled()
}

/// Phase 81: Heap v10 bridge enabled (doctor / CI gate).
pub fn heap_v10() -> bool {
    ManagedHeap::heap_v10_enabled()
}

/// Phase 82: Heap v11 enabled (doctor / CI gate).
pub fn heap_v11() -> bool {
    ManagedHeap::heap_v11_enabled()
}

static HEAP_ROOT_BRIDGE: std::sync::OnceLock<std::sync::Mutex<Vec<Value>>> =
    std::sync::OnceLock::new();

fn heap_root_bridge_store() -> &'static std::sync::Mutex<Vec<Value>> {
    HEAP_ROOT_BRIDGE.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// Phase 81: publish heap root across process boundary (shared arena stub).
pub fn heap_root_bridge_publish(root: Value) {
    if let Ok(mut store) = heap_root_bridge_store().lock() {
        store.push(root);
    }
    if ManagedHeap::heap_v11_bridge_prod_enabled() {
        if let Ok(path) = std::env::var("SYX_HEAP_BRIDGE_SHM") {
            let _ = std::fs::write(&path, b"syx-heap-bridge-v11");
        }
        BRIDGE_ATTACH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Phase 82: subscribe to bridged heap roots (cross-process prod).
pub fn heap_root_bridge_subscribe() -> usize {
    heap_root_bridge_store()
        .lock()
        .map(|store| store.len())
        .unwrap_or(0)
}

static BRIDGE_ATTACH_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Phase 82: bridge attach/detach count for doctor report.
pub fn heap_root_bridge_attach_count() -> u64 {
    BRIDGE_ATTACH_COUNT.load(std::sync::atomic::Ordering::Relaxed)
}

/// Phase 82: Heap v11 cross-process bridge prod (default on).
pub fn heap_v11_bridge_prod_enabled() -> bool {
    std::env::var("SYX_HEAP_V11_BRIDGE_PROD")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(heap_root_bridge_enabled())
}

/// Phase 81: peek latest bridged heap root (cross-process actor extern ref).
pub fn heap_root_bridge_peek() -> Option<Value> {
    heap_root_bridge_store()
        .lock()
        .ok()
        .and_then(|store| store.last().cloned())
}

/// Phase 81: cross-process heap root bridge active when v10 enabled.
pub fn heap_root_bridge_enabled() -> bool {
    ManagedHeap::heap_v10_enabled()
}

/// Phase 90: Heap v13 young/old generational compact (default on).
pub fn heap_v13_generational() -> bool {
    std::env::var("SYX_HEAP_V13_GENERATIONAL")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

/// Phase 90: generational compact cycle count for doctor `--gc-gen-report`.
static GC_GEN_COMPACT_CYCLES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn gc_gen_compact_cycles() -> u64 {
    GC_GEN_COMPACT_CYCLES.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn gc_gen_compact_record() {
    GC_GEN_COMPACT_CYCLES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

pub fn reset_gc_gen_compact_cycles() {
    GC_GEN_COMPACT_CYCLES.store(0, std::sync::atomic::Ordering::Relaxed);
}

#[cfg(test)]
mod deep_field_graph_gc_walk_tests {
    use super::*;
    use crate::Value;
    use std::collections::HashMap;

    /// Wave A W539 / ADR-883: nested ObjectRef reachable only via parent field
    /// survives collect; unreachable orphan ObjectRef is swept.
    #[test]
    fn deep_field_graph_gc_walk_keeps_nested_sweeps_orphan() {
        // Force classic mark-sweep so the nail is deterministic across Heap v9+.
        std::env::set_var("SYX_HEAP_V9_GENERATIONAL", "0");
        std::env::set_var("SYX_HEAP_V10", "0");
        std::env::set_var("SYX_HEAP_V13_GENERATIONAL", "0");

        let child = Value::object_ref_from_map(
            HashMap::from([("n".into(), Value::I64(1))]),
            true,
        );
        let parent = Value::object_ref_from_map(
            HashMap::from([("child".into(), child.clone())]),
            true,
        );
        let orphan = Value::object_ref_from_map(
            HashMap::from([("x".into(), Value::I64(2))]),
            true,
        );

        let child_id = Value::object_ref_identity(&child).expect("child identity");
        let orphan_id = Value::object_ref_identity(&orphan).expect("orphan identity");

        let mut heap = ManagedHeap::default();
        let parent_idx = heap.alloc(parent.clone());
        let child_idx = heap.alloc(child.clone());
        let orphan_idx = heap.alloc(orphan.clone());
        assert_eq!(parent_idx, 0);
        assert_eq!(child_idx, 1);
        assert_eq!(orphan_idx, 2);

        let swept = heap.collect(&[parent.clone()]);

        // Nested child kept alive via field-graph walk from parent root.
        let child_live = heap.get(child_idx).is_some_and(|v| {
            Value::object_ref_identity(v).is_some_and(|p| p == child_id)
        });
        assert!(
            child_live,
            "nested ObjectRef reachable via parent field must survive collect"
        );

        // Orphan not in field graph must be swept (slot Nil or returned in swept).
        let orphan_slot_gone = heap
            .get(orphan_idx)
            .is_none_or(|v| matches!(v, Value::Nil));
        let orphan_in_swept = swept.iter().any(|v| {
            Value::object_ref_identity(v).is_some_and(|p| p == orphan_id)
        });
        assert!(
            orphan_slot_gone || orphan_in_swept,
            "unreachable orphan ObjectRef must be swept"
        );
    }
}
