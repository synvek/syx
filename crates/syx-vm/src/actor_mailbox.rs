//! Wave A W489 / ADR-833: actor mailbox Value queue + GC roots.
//!
//! Retains depth-only nesting tracking while also holding in-flight call
//! argument snapshots so `GcCollect` / MirRuntime collect can see them.
//! Does **not** claim async multi-worker mailbox GA or unified-heap enable.

use std::collections::{HashMap, VecDeque};
use std::sync::{LazyLock, Mutex};

use syx_runtime_core::Value;

static MAILBOX_DEPTH: LazyLock<Mutex<HashMap<i64, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Per-actor in-flight message snapshots (argument vectors) for GC roots.
static MAILBOX_QUEUES: LazyLock<Mutex<HashMap<i64, VecDeque<Vec<Value>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Enter actor mailbox (serializes work per `actor_id`).
pub fn mailbox_enter(actor_id: i64) -> usize {
    let mut map = MAILBOX_DEPTH.lock().unwrap_or_else(|e| e.into_inner());
    let depth = map.entry(actor_id).or_insert(0);
    *depth += 1;
    *depth
}

pub fn mailbox_leave(actor_id: i64) {
    let mut map = MAILBOX_DEPTH.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(depth) = map.get_mut(&actor_id) {
        *depth = depth.saturating_sub(1);
    }
}

pub fn mailbox_depth(actor_id: i64) -> usize {
    MAILBOX_DEPTH
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&actor_id)
        .copied()
        .unwrap_or(0)
}

/// Enqueue a call-argument snapshot for GC-visible mailbox roots (W489 / ADR-833).
pub fn mailbox_enqueue(actor_id: i64, args: &[Value]) {
    let mut map = MAILBOX_QUEUES.lock().unwrap_or_else(|e| e.into_inner());
    map.entry(actor_id).or_default().push_back(args.to_vec());
}

/// Dequeue the oldest in-flight snapshot for `actor_id` (paired with enqueue).
pub fn mailbox_dequeue(actor_id: i64) -> Option<Vec<Value>> {
    let mut map = MAILBOX_QUEUES.lock().unwrap_or_else(|e| e.into_inner());
    map.get_mut(&actor_id).and_then(|q| q.pop_front())
}

/// Collect all Values currently held in mailbox queues (GC roots).
pub fn mailbox_collect_roots() -> Vec<Value> {
    let map = MAILBOX_QUEUES.lock().unwrap_or_else(|e| e.into_inner());
    let mut roots = Vec::new();
    for queue in map.values() {
        for msg in queue {
            roots.extend(msg.iter().cloned());
        }
    }
    roots
}

/// Rewrite HeapSlot indices inside mailbox queues after compact (W489 / ADR-833).
pub fn mailbox_rewrite_slots(forwarding: &[(usize, usize)]) {
    let mut map = MAILBOX_QUEUES.lock().unwrap_or_else(|e| e.into_inner());
    for queue in map.values_mut() {
        for msg in queue.iter_mut() {
            syx_runtime_core::ManagedHeap::rewrite_slot_indices(msg, forwarding);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mailbox_tracks_nested_depth() {
        assert_eq!(mailbox_enter(99), 1);
        assert_eq!(mailbox_enter(99), 2);
        mailbox_leave(99);
        assert_eq!(mailbox_depth(99), 1);
        mailbox_leave(99);
        assert_eq!(mailbox_depth(99), 0);
    }

    #[test]
    fn mailbox_value_queue_roots_include_enqueued_args() {
        // Wave A W489 / ADR-833: enqueued Values must appear in collect roots.
        let id = 42_001;
        // Drain any leftover from prior tests on this id.
        while mailbox_dequeue(id).is_some() {}
        mailbox_enqueue(id, &[Value::I64(77), Value::Str("mbox".into())]);
        let roots = mailbox_collect_roots();
        assert!(
            roots.iter().any(|v| matches!(v, Value::I64(77))),
            "enqueued I64 must be a root: {roots:?}"
        );
        assert!(
            roots
                .iter()
                .any(|v| matches!(v, Value::Str(s) if s == "mbox")),
            "enqueued Str must be a root: {roots:?}"
        );
        let _ = mailbox_dequeue(id);
        let after = mailbox_collect_roots();
        assert!(
            !after.iter().any(|v| matches!(v, Value::I64(77))),
            "dequeued Values must leave roots: {after:?}"
        );
    }
}
