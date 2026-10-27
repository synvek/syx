use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError};
use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::{ensure_object_id, platform_int_value, Value};

struct ChannelPair {
    /// ADR-266 / M3-A04: `None` after close (sender dropped).
    tx: Mutex<Option<Sender<Value>>>,
    rx: Receiver<Value>,
    /// Create-time capacity; `0` means unbounded.
    capacity: i32,
}

impl ChannelPair {
    fn try_send(&self, value: Value) -> Result<(), TrySendError<Value>> {
        let guard = match self.tx.lock() {
            Ok(g) => g,
            Err(_) => return Err(TrySendError::Disconnected(value)),
        };
        match guard.as_ref() {
            Some(tx) => tx.try_send(value),
            None => Err(TrySendError::Disconnected(value)),
        }
    }

    fn send(&self, value: Value) -> Result<(), Value> {
        let guard = match self.tx.lock() {
            Ok(g) => g,
            Err(_) => return Err(value),
        };
        match guard.as_ref() {
            Some(tx) => tx.send(value).map_err(|e| e.into_inner()),
            None => Err(value),
        }
    }

    /// ADR-266: drop sender so receivers observe Disconnected after drain.
    fn close(&self) {
        if let Ok(mut guard) = self.tx.lock() {
            *guard = None;
        }
    }
}

static GLOBAL_CHANNELS: std::sync::LazyLock<Arc<Mutex<HashMap<i64, Arc<ChannelPair>>>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));
static NEXT_CHANNEL_ID: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1);

/// Phase 107: fair round-robin cursor for multi-ready channel select.
/// Thread-local so tests/seeds stay deterministic without cross-thread interference.
/// Each successful select advances the cursor so simultaneous ready ops share wins
/// deterministically (start = cursor % n, then scan).
thread_local! {
    static SELECT_FAIR_CURSOR: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Phase 107 fingerprint: fair multi-ready select.
pub fn select_fair_fingerprint() -> &'static str {
    "select-fair-v1:round-robin"
}

/// Reset fair cursor (tests / deterministic seeds).
pub fn reset_select_fair_cursor(seed: usize) {
    SELECT_FAIR_CURSOR.with(|c| c.set(seed));
}

pub fn select_fair_cursor() -> usize {
    SELECT_FAIR_CURSOR.with(|c| c.get())
}

enum SelectOp {
    Recv { channel: Value },
    Send { channel: Value, value: Value },
}

const SELECT_SEND_VALUE_KEY: &str = "__syx_select_send_value";
const SELECT_DIR_KEY: &str = "__syx_select_dir";

/// Phase 108: tagged select arm convention for mixed send/recv fair poll.
///
/// Host accepts (in one fair poll):
/// 1. bare channel object → recv (legacy all-recv)
/// 2. `[dir, channel, payload?]` where `dir` 0 = recv, 1 = send (literal tagged array)
/// 3. channel object with `__syx_select_dir` (0|1) and optional `__syx_select_send_value`
///    (sugar path: homogeneous `Object[]` so semantic S014 accepts the array)
/// 4. legacy send-marked channel with only `__syx_select_send_value`
pub const SELECT_MIXED_UNIFIED_FINGERPRINT: &str = "select-mixed-unified-v1:tagged-array-poll";

pub fn select_mixed_unified_fingerprint() -> &'static str {
    SELECT_MIXED_UNIFIED_FINGERPRINT
}

/// Phase 109: mixed select + await interleave hard gate.
pub const SELECT_MIXED_AWAIT_FINGERPRINT: &str = "select-mixed-await-v1:interleave";

pub fn select_mixed_await_fingerprint() -> &'static str {
    SELECT_MIXED_AWAIT_FINGERPRINT
}

/// Phase 110: mixed select + await requires cancel or deadline hard gate.
pub const SELECT_MIXED_TIMEOUT_FINGERPRINT: &str = "select-mixed-timeout-v1:cancel-or-deadline";

/// Phase 111: language-level `timeout(ms):` / `deadline ms:` select arms (not env-only GA).
pub const SELECT_TIMEOUT_SYNTAX_FINGERPRINT: &str = "select-timeout-syntax-v1:lang-arm";

/// Phase 112 (historical): mixed+await unified with language timeout/deadline ms.
pub const SELECT_DEADLINE_UNIFY_FINGERPRINT: &str = "select-deadline-unify-v1:lang-over-env";

/// Phase 113: mixed+await requires syntax timeout/deadline; env fallback hard-banned.
pub const SELECT_DEADLINE_STRICT_FINGERPRINT: &str = "select-deadline-strict-v1:no-env";

pub fn select_mixed_timeout_fingerprint() -> &'static str {
    SELECT_MIXED_TIMEOUT_FINGERPRINT
}

pub fn select_timeout_syntax_fingerprint() -> &'static str {
    SELECT_TIMEOUT_SYNTAX_FINGERPRINT
}

pub fn select_deadline_unify_fingerprint() -> &'static str {
    SELECT_DEADLINE_UNIFY_FINGERPRINT
}

pub fn select_deadline_strict_fingerprint() -> &'static str {
    SELECT_DEADLINE_STRICT_FINGERPRINT
}

static MIXED_AWAIT_CANCEL_PROBE: OnceLock<fn() -> bool> = OnceLock::new();

/// Register cooperative cancel probe for mixed+await select (from async runtime).
pub fn register_mixed_await_cancel_probe(probe: fn() -> bool) {
    let _ = MIXED_AWAIT_CANCEL_PROBE.set(probe);
}

fn mixed_await_is_cancelled() -> bool {
    MIXED_AWAIT_CANCEL_PROBE.get().copied().unwrap_or(|| false)()
}

/// Phase 113: env fallback removed (`select-deadline-strict-v1:no-env`).
/// Kept as a hard-ban probe — calling this is not a GA timeout source.
pub fn mixed_await_select_timeout_ms() -> u64 {
    let _ = SELECT_DEADLINE_STRICT_FINGERPRINT;
    // Hard-ban: SYX_SELECT_MIXED_TIMEOUT_MS must not supply GA deadlines.
    if std::env::var("SYX_SELECT_MIXED_TIMEOUT_MS").is_ok() {
        panic!(
            "SYX_SELECT_MIXED_TIMEOUT_MS is hard-banned (select-deadline-strict-v1:no-env); use timeout(ms): or deadline ms:"
        );
    }
    panic!(
        "mixed_await_select_timeout_ms removed; mixed+await requires syntax timeout/deadline (select-deadline-strict-v1:no-env)"
    );
}

fn parse_select_op(v: &Value) -> SyxResult<SelectOp> {
    let v = Value::resolve_heap_slot(v);
    // Phase 108: tagged arm `[dir, channel, payload?]` (dir 0=recv, 1=send).
    if let Some(parts) = Value::materialize_array(v.clone()) {
        if parts.is_empty() {
            return Err(
                SyxDiagnostic::error("Select tagged arm must be [dir, channel, payload?]")
                    .with_code("C003"),
            );
        }
        let dir = parts[0].as_i64();
        let channel = parts.get(1).cloned().unwrap_or(Value::Nil);
        match dir {
            0 => return Ok(SelectOp::Recv { channel }),
            1 => {
                let value = parts.get(2).cloned().unwrap_or(Value::Nil);
                return Ok(SelectOp::Send { channel, value });
            }
            _ => {
                return Err(SyxDiagnostic::error(format!(
                    "Select arm dir must be 0 (recv) or 1 (send), got {dir}"
                ))
                .with_code("C003"));
            }
        }
    }
    if let Value::Object(map) = &v {
        // Phase 108 sugar: Object-tagged arm with `__syx_select_dir`.
        if let Some(dir_v) = map.get(SELECT_DIR_KEY) {
            match dir_v.as_i64() {
                0 => return Ok(SelectOp::Recv { channel: v.clone() }),
                1 => {
                    let value = map
                        .get(SELECT_SEND_VALUE_KEY)
                        .cloned()
                        .unwrap_or(Value::Nil);
                    return Ok(SelectOp::Send {
                        channel: v.clone(),
                        value,
                    });
                }
                other => {
                    return Err(SyxDiagnostic::error(format!(
                        "Select arm __syx_select_dir must be 0 or 1, got {other}"
                    ))
                    .with_code("C003"));
                }
            }
        }
        // Legacy Phase 107: send ops marked with `__syx_select_send_value` only.
        if let Some(send_val) = map.get(SELECT_SEND_VALUE_KEY) {
            return Ok(SelectOp::Send {
                channel: v.clone(),
                value: send_val.clone(),
            });
        }
    }
    Ok(SelectOp::Recv { channel: v })
}

fn try_select_op(op: &SelectOp) -> SyxResult<Option<Value>> {
    match op {
        SelectOp::Recv { channel } => {
            let id = channel_id_from_value(channel)?;
            let pair = get_channel_pair(id)?;
            match pair.rx.try_recv() {
                Ok(value) => Ok(Some(value)),
                Err(TryRecvError::Empty) => Ok(None),
                // ADR-266: closed + drained → Nil (ready), not C003 hang/err.
                Err(TryRecvError::Disconnected) => Ok(Some(Value::Nil)),
            }
        }
        SelectOp::Send { channel, value } => {
            let id = channel_id_from_value(channel)?;
            let pair = get_channel_pair(id)?;
            match pair.try_send(value.clone()) {
                Ok(()) => Ok(Some(Value::Bool(true))),
                Err(TrySendError::Full(_)) => Ok(None),
                Err(TrySendError::Disconnected(_)) => {
                    Err(SyxDiagnostic::error("Channel closed").with_code("C010"))
                }
            }
        }
    }
}

fn wrap_indexed_select_result(index: usize, value: Value) -> Value {
    // String index: sugar compares labels at block scope (avoids nested if+local VM bug).
    Value::Array(vec![Value::Str(index.to_string()), value])
}

fn wrap_indexed_select_miss(fallback: Value) -> Value {
    Value::Array(vec![Value::Str("-1".into()), fallback])
}

/// Fair round-robin scan: start at `cursor % n`, pick first ready op.
/// Cursor advances only on a successful selection (not on empty polls / timeout spins).
fn fair_select_ready(ops: &[SelectOp], indexed: bool) -> SyxResult<Option<Value>> {
    let n = ops.len();
    if n == 0 {
        return Ok(None);
    }
    let cursor = SELECT_FAIR_CURSOR.with(|c| c.get());
    let start = cursor % n;
    for offset in 0..n {
        let idx = (start + offset) % n;
        if let Some(value) = try_select_op(&ops[idx])? {
            SELECT_FAIR_CURSOR.with(|c| c.set(cursor.wrapping_add(1)));
            return Ok(Some(if indexed {
                wrap_indexed_select_result(idx, value)
            } else {
                value
            }));
        }
    }
    Ok(None)
}

fn channels_to_ops(channels: &[Value]) -> SyxResult<Vec<SelectOp>> {
    channels.iter().map(parse_select_op).collect()
}

fn channel_id_from_value(v: &Value) -> SyxResult<i64> {
    channel_id_from_value_depth(v, 0)
}

fn channel_id_from_value_depth(v: &Value, depth: u8) -> SyxResult<i64> {
    let v = Value::resolve_heap_slot(v);
    if let Value::Object(map) = &v {
        if let Some(id) = map.get("__channel_id").and_then(|id| match id {
            Value::I64(n) => Some(*n),
            Value::I32(n) => Some(*n as i64),
            _ => None,
        }) {
            return Ok(id);
        }
        // Sync.Channel facade stores the host handle in `handle`.
        if depth < 2 {
            if let Some(inner) = map.get("handle") {
                return channel_id_from_value_depth(inner, depth + 1);
            }
        }
        Err(
            SyxDiagnostic::error("Expected channel object with __channel_id").with_code("C003"),
        )
    } else {
        Err(SyxDiagnostic::error("Expected channel object").with_code("C003"))
    }
}

fn get_channel_pair(id: i64) -> SyxResult<Arc<ChannelPair>> {
    let channels = GLOBAL_CHANNELS
        .lock()
        .map_err(|_| SyxDiagnostic::error("Channel table lock poisoned").with_code("C003"))?;
    channels
        .get(&id)
        .cloned()
        .ok_or_else(|| SyxDiagnostic::error(format!("Unknown channel id {id}")).with_code("C003"))
}

fn with_channel<F, R>(id: i64, f: F) -> SyxResult<R>
where
    F: FnOnce(&ChannelPair) -> SyxResult<R>,
{
    f(get_channel_pair(id)?.as_ref())
}

pub fn make_channel_recv_awaitable(channel: Value, id: i64) -> Value {
    let mut map = HashMap::new();
    map.insert("__channel_recv_pending".into(), Value::Bool(true));
    map.insert("__channel_id".into(), platform_int_value(id));
    map.insert("__channel_ref".into(), channel);
    ensure_object_id(&mut map);
    Value::Object(map)
}

pub fn wrap_channel_recv_value(value: Value) -> Value {
    let mut map = HashMap::new();
    map.insert("__await_value".into(), value);
    ensure_object_id(&mut map);
    Value::Object(map)
}

pub fn channel_recv_pending_id(value: &Value) -> Option<i64> {
    let Value::Object(map) = value else {
        return None;
    };
    if map.get("__channel_recv_pending").and_then(|v| match v {
        Value::Bool(b) => Some(*b),
        _ => None,
    }) != Some(true)
    {
        return None;
    }
    map.get("__channel_id").and_then(|v| match v {
        Value::I64(n) => Some(*n),
        Value::I32(n) => Some(*n as i64),
        _ => None,
    })
}

pub fn host_channel_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let capacity = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    let (tx, rx) = if capacity > 0 {
        crossbeam_channel::bounded(capacity as usize)
    } else {
        crossbeam_channel::unbounded()
    };
    let id = NEXT_CHANNEL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    {
        let mut channels = GLOBAL_CHANNELS
            .lock()
            .map_err(|_| SyxDiagnostic::error("Channel table lock poisoned").with_code("C003"))?;
        channels.insert(
            id,
            Arc::new(ChannelPair {
                tx: Mutex::new(Some(tx)),
                rx,
                capacity,
            }),
        );
    }
    let mut map = HashMap::new();
    map.insert("__channel_id".into(), platform_int_value(id));
    ensure_object_id(&mut map);
    stack.push(Value::Object(map));
    Ok(())
}

pub fn host_channel_send(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Nil);
    let channel = stack.pop().unwrap_or(Value::Nil);
    let id = channel_id_from_value(&channel)?;
    let pair = get_channel_pair(id)?;
    pair.send(value)
        .map_err(|_| SyxDiagnostic::error("Channel closed").with_code("C010"))?;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_channel_try_send(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Nil);
    let channel = stack.pop().unwrap_or(Value::Nil);
    let id = channel_id_from_value(&channel)?;
    let pair = get_channel_pair(id)?;
    match pair.try_send(value) {
        Ok(()) => stack.push(Value::Bool(true)),
        Err(TrySendError::Full(_)) => stack.push(Value::Bool(false)),
        Err(TrySendError::Disconnected(_)) => {
            return Err(SyxDiagnostic::error("Channel closed").with_code("C010"));
        }
    }
    Ok(())
}

pub fn host_channel_try_recv(stack: &mut Vec<Value>) -> SyxResult<()> {
    let channel = stack.pop().unwrap_or(Value::Nil);
    let id = channel_id_from_value(&channel)?;
    let pair = get_channel_pair(id)?;
    match pair.rx.try_recv() {
        Ok(value) => stack.push(value),
        Err(TryRecvError::Empty) => stack.push(Value::Nil),
        // ADR-266: closed + empty → Nil (book contract).
        Err(TryRecvError::Disconnected) => stack.push(Value::Nil),
    }
    Ok(())
}

/// ADR-266 / M3-A04: disconnect sender; buffered values remain receivable.
pub fn host_channel_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut ch = stack.pop().unwrap_or(Value::Nil);
    if let Ok(id) = channel_id_from_value(&ch) {
        if let Ok(pair) = get_channel_pair(id) {
            pair.close();
        }
    }
    if let Value::Object(ref mut map) = ch {
        map.insert("closed".into(), Value::Bool(true));
    }
    stack.push(Value::Nil);
    Ok(())
}

/// Phase 108: `__syx_select_tag(dir, channel, payload) -> Object`
/// Builds a homogeneous Object-tagged select arm for mixed send/recv array polls.
/// `dir` 0 = recv, 1 = send (payload used only for send).
pub fn host_select_tag(stack: &mut Vec<Value>) -> SyxResult<()> {
    let payload = stack.pop().unwrap_or(Value::Nil);
    let channel = stack.pop().unwrap_or(Value::Nil);
    let dir = stack.pop().unwrap_or(Value::I64(0)).as_i64();
    let channel = Value::resolve_heap_slot(&channel);
    let mut map = match channel {
        Value::Object(m) => m,
        other => {
            return Err(SyxDiagnostic::error(format!(
                "__syx_select_tag expected channel object, got {other:?}"
            ))
            .with_code("C003"));
        }
    };
    map.insert(SELECT_DIR_KEY.into(), Value::I64(dir));
    if dir == 1 {
        map.insert(SELECT_SEND_VALUE_KEY.into(), payload);
    } else {
        map.remove(SELECT_SEND_VALUE_KEY);
    }
    ensure_object_id(&mut map);
    stack.push(Value::Object(map));
    Ok(())
}

/// Phase 16/17/107/108: non-blocking N-way channel select.
/// Multi-ready ops use fair round-robin (`select-fair-v1:round-robin`).
/// Array form returns `[index, value]` so select sugar can dispatch arms
/// (`select-mixed-unified-v1:tagged-array-poll`). Arms may be bare channels
/// (recv), `[dir, channel, payload?]` tagged entries, or legacy send-marked
/// channel objects.
pub fn host_channel_select(stack: &mut Vec<Value>) -> SyxResult<()> {
    let top = stack.pop().unwrap_or(Value::Nil);
    let (channels, indexed) = if let Some(items) = Value::materialize_array(top.clone()) {
        let indexed = !items.is_empty();
        (items, indexed)
    } else {
        let ch1 = stack.pop().unwrap_or(Value::Nil);
        // Legacy 2-arg form: return plain value (no index wrap).
        (vec![ch1, top], false)
    };
    let ops = channels_to_ops(&channels)?;
    match fair_select_ready(&ops, indexed)? {
        Some(value) => stack.push(value),
        None => {
            if indexed {
                stack.push(wrap_indexed_select_miss(Value::Nil));
            } else {
                stack.push(Value::Nil);
            }
        }
    }
    Ok(())
}

// Phase 19/107: channel select with default when no channel is ready.
// Stack (bottom → top at host entry): channels… or `[ch…]`, then fallback — matches
// `packages/sync/mod.sy` `orDefault(channels, fallback)`.
// Fair round-robin when multiple ops are ready. Array form returns `[index, value]`.
pub fn host_channel_select_default(stack: &mut Vec<Value>) -> SyxResult<()> {
    let default_value = stack.pop().unwrap_or(Value::Nil);
    let top = stack.pop().unwrap_or(Value::Nil);
    let (channels, indexed) = if let Some(items) = Value::materialize_array(top.clone()) {
        let indexed = !items.is_empty();
        (items, indexed)
    } else {
        // Multi-arg sugar / single channel: plain value return.
        let mut channels = vec![top];
        while stack
            .last()
            .is_some_and(|v| channel_id_from_value(v).is_ok())
        {
            channels.insert(0, stack.pop().unwrap());
        }
        (channels, false)
    };
    let ops = channels_to_ops(&channels)?;
    match fair_select_ready(&ops, indexed)? {
        Some(value) => stack.push(value),
        None => {
            if indexed {
                stack.push(wrap_indexed_select_miss(default_value));
            } else {
                stack.push(default_value);
            }
        }
    }
    Ok(())
}

/// Phase 18/107: channel select with millisecond timeout (stack: ms, ch1, ch2, … or ms, array).
/// Fair round-robin when multiple ops are ready. Array form returns `[index, value]`.
pub fn host_channel_select_timeout(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = SELECT_TIMEOUT_SYNTAX_FINGERPRINT;
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64().max(0) as u64;
    let top = stack.pop().unwrap_or(Value::Nil);
    let (channels, indexed) = if let Some(items) = Value::materialize_array(top.clone()) {
        let indexed = !items.is_empty();
        (items, indexed)
    } else {
        (vec![top], false)
    };
    let ops = channels_to_ops(&channels)?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(ms);
    loop {
        if let Some(value) = fair_select_ready(&ops, indexed)? {
            stack.push(value);
            return Ok(());
        }
        if std::time::Instant::now() >= deadline {
            stack.push(if indexed {
                wrap_indexed_select_miss(Value::Nil)
            } else {
                Value::Nil
            });
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
}

/// Phase 110: mixed select + await poll with cooperative cancel **or** deadline timeout.
/// Stack: `channels` array, `fallback`, `timeout_ms`. Returns same shape as
/// [`host_channel_select_default`] (indexed `[index, value]` or miss as `["-1", fallback]`).
pub fn host_channel_select_mixed_await(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = SELECT_MIXED_TIMEOUT_FINGERPRINT;
    let _ = SELECT_DEADLINE_UNIFY_FINGERPRINT;
    let _ = SELECT_DEADLINE_STRICT_FINGERPRINT;
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64().max(0) as u64;
    let default_value = stack.pop().unwrap_or(Value::Nil);
    let top = stack.pop().unwrap_or(Value::Nil);
    let (channels, indexed) = if let Some(items) = Value::materialize_array(top.clone()) {
        let indexed = !items.is_empty();
        (items, indexed)
    } else {
        (vec![top], false)
    };
    let ops = channels_to_ops(&channels)?;
    if mixed_await_is_cancelled() {
        stack.push(if indexed {
            wrap_indexed_select_miss(default_value)
        } else {
            default_value
        });
        return Ok(());
    }
    let deadline = Instant::now() + Duration::from_millis(ms.max(1));
    loop {
        if mixed_await_is_cancelled() {
            stack.push(if indexed {
                wrap_indexed_select_miss(default_value)
            } else {
                default_value
            });
            return Ok(());
        }
        if let Some(value) = fair_select_ready(&ops, indexed)? {
            stack.push(value);
            return Ok(());
        }
        if Instant::now() >= deadline {
            stack.push(if indexed {
                wrap_indexed_select_miss(default_value)
            } else {
                default_value
            });
            return Ok(());
        }
        std::thread::sleep(Duration::from_micros(100));
    }
}

pub fn host_channel_recv(stack: &mut Vec<Value>) -> SyxResult<()> {
    let channel = stack.pop().unwrap_or(Value::Nil);
    let id = channel_id_from_value(&channel)?;
    let pair = get_channel_pair(id)?;
    match pair.rx.recv() {
        Ok(value) => stack.push(value),
        // ADR-266: closed + drained → Nil.
        Err(_) => stack.push(Value::Nil),
    }
    Ok(())
}

pub fn host_channel_recv_async(stack: &mut Vec<Value>) -> SyxResult<()> {
    let channel = stack.pop().unwrap_or(Value::Nil);
    let id = channel_id_from_value(&channel)?;
    let pair = get_channel_pair(id)?;
    let value = match pair.rx.try_recv() {
        Ok(value) => Ok(Some(value)),
        Err(TryRecvError::Empty) => Ok(None),
        Err(TryRecvError::Disconnected) => Ok(Some(Value::Nil)),
    }?;
    stack.push(match value {
        Some(v) => wrap_channel_recv_value(v),
        None => make_channel_recv_awaitable(channel, id),
    });
    Ok(())
}

/// Phase 133: `__syx_channel_length(channel) -> Int`
pub fn host_channel_length(stack: &mut Vec<Value>) -> SyxResult<()> {
    let channel = stack.pop().unwrap_or(Value::Nil);
    let id = channel_id_from_value(&channel)?;
    let pair = get_channel_pair(id)?;
    stack.push(platform_int_value(pair.rx.len() as i64));
    Ok(())
}

/// Phase 133: `__syx_channel_capacity(channel) -> Int` (`0` = unbounded).
pub fn host_channel_capacity(stack: &mut Vec<Value>) -> SyxResult<()> {
    let channel = stack.pop().unwrap_or(Value::Nil);
    let id = channel_id_from_value(&channel)?;
    let pair = get_channel_pair(id)?;
    stack.push(platform_int_value(pair.capacity as i64));
    Ok(())
}

pub fn try_poll_channel_recv(value: &Value) -> Option<SyxResult<Value>> {
    let id = channel_recv_pending_id(value)?;
    with_channel(id, |pair| {
        Ok(match pair.rx.try_recv() {
            Ok(v) => Some(Ok(v)),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Ok(Value::Nil)),
        })
    })
    .ok()
    .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_select_default_returns_fallback() {
        let mut stack = Vec::new();
        host_channel_create(&mut stack).unwrap();
        let ch1 = stack.pop().unwrap();
        host_channel_create(&mut stack).unwrap();
        let ch2 = stack.pop().unwrap();
        stack.push(ch1);
        stack.push(ch2);
        stack.push(Value::Str("default".into()));
        host_channel_select_default(&mut stack).unwrap();
        assert_eq!(stack.pop(), Some(Value::Str("default".into())));
    }

    #[test]
    fn channel_select_default_single_channel_and_fallback() {
        let mut stack = Vec::new();
        host_channel_create(&mut stack).unwrap();
        let ch = stack.pop().unwrap();
        stack.push(ch);
        stack.push(Value::Str("fallback".into()));
        host_channel_select_default(&mut stack).unwrap();
        assert_eq!(stack.pop(), Some(Value::Str("fallback".into())));
    }

    #[test]
    fn global_channel_table_is_cross_thread_accessible() {
        let mut stack = Vec::new();
        host_channel_create(&mut stack).unwrap();
        let channel = stack.pop().unwrap();
        let id = channel_id_from_value(&channel).unwrap();
        let handle = std::thread::spawn(move || {
            let mut stack = vec![channel, Value::I64(7)];
            host_channel_send(&mut stack).unwrap();
        });
        handle.join().unwrap();
        let mut stack = vec![Value::Object({
            let mut m = HashMap::new();
            m.insert("__channel_id".into(), Value::I64(id));
            m
        })];
        host_channel_recv(&mut stack).unwrap();
        assert_eq!(stack.pop(), Some(Value::I64(7)));
    }

    #[test]
    fn select_fair_round_robin_alternates_when_both_ready() {
        reset_select_fair_cursor(0);
        assert_eq!(select_fair_fingerprint(), "select-fair-v1:round-robin");

        let mut stack = Vec::new();
        stack.push(Value::I32(1));
        host_channel_create(&mut stack).unwrap();
        let ch1 = stack.pop().unwrap();
        stack.push(Value::I32(1));
        host_channel_create(&mut stack).unwrap();
        let ch2 = stack.pop().unwrap();

        let mut counts = [0usize; 2];
        for _ in 0..20 {
            // Drain any leftover so both sends succeed (capacity 1).
            for ch in [&ch1, &ch2] {
                let mut drain = vec![ch.clone()];
                let _ = host_channel_try_recv(&mut drain);
            }
            stack.push(ch1.clone());
            stack.push(Value::I64(11));
            host_channel_send(&mut stack).unwrap();
            let _ = stack.pop();
            stack.push(ch2.clone());
            stack.push(Value::I64(22));
            host_channel_send(&mut stack).unwrap();
            let _ = stack.pop();

            stack.push(Value::Array(vec![ch1.clone(), ch2.clone()]));
            host_channel_select(&mut stack).unwrap();
            match stack.pop() {
                Some(Value::Array(parts)) if parts.len() == 2 => {
                    let idx = parts[0].as_str();
                    match (idx.as_str(), &parts[1]) {
                        ("0", Value::I64(11)) => counts[0] += 1,
                        ("1", Value::I64(22)) => counts[1] += 1,
                        other => panic!("unexpected select value: {other:?}"),
                    }
                }
                other => panic!("unexpected select result: {other:?}"),
            }
        }
        // Deterministic RR from seed 0: perfect alternation.
        assert_eq!(
            counts,
            [10, 10],
            "fair RR must split 20/20 evenly: {counts:?}"
        );
    }

    #[test]
    fn select_mixed_send_marked_channel_returns_index() {
        reset_select_fair_cursor(0);
        let mut stack = Vec::new();
        stack.push(Value::I32(1));
        host_channel_create(&mut stack).unwrap();
        let recv_ch = stack.pop().unwrap();
        stack.push(Value::I32(1));
        host_channel_create(&mut stack).unwrap();
        let mut send_ch = stack.pop().unwrap();

        stack.push(recv_ch.clone());
        stack.push(Value::I64(7));
        host_channel_send(&mut stack).unwrap();
        let _ = stack.pop();

        // Mark send channel with ephemeral select-send payload.
        if let Value::Object(ref mut map) = send_ch {
            map.insert(SELECT_SEND_VALUE_KEY.into(), Value::I64(99));
        }

        let ops = Value::Array(vec![recv_ch, send_ch]);
        stack.push(ops);
        host_channel_select(&mut stack).unwrap();
        let result = stack.pop().unwrap();
        let Value::Array(parts) = result else {
            panic!("array select must return [index, value], got {result:?}");
        };
        assert_eq!(parts[0].as_str(), "0");
        assert_eq!(parts[1], Value::I64(7));
    }

    #[test]
    fn select_mixed_tagged_array_fair_poll() {
        reset_select_fair_cursor(0);
        assert_eq!(
            select_mixed_unified_fingerprint(),
            "select-mixed-unified-v1:tagged-array-poll"
        );

        let mut stack = Vec::new();
        stack.push(Value::I32(1));
        host_channel_create(&mut stack).unwrap();
        let recv_ch = stack.pop().unwrap();
        stack.push(Value::I32(1));
        host_channel_create(&mut stack).unwrap();
        let send_ch = stack.pop().unwrap();

        // Both ready: recv has data, send has capacity.
        stack.push(recv_ch.clone());
        stack.push(Value::I64(7));
        host_channel_send(&mut stack).unwrap();
        let _ = stack.pop();

        // Tagged arms: [0, ch] recv, [1, ch, payload] send.
        let ops = Value::Array(vec![
            Value::Array(vec![Value::I64(0), recv_ch.clone()]),
            Value::Array(vec![Value::I64(1), send_ch.clone(), Value::I64(99)]),
        ]);
        stack.push(ops);
        host_channel_select(&mut stack).unwrap();
        let result = stack.pop().unwrap();
        let Value::Array(parts) = result else {
            panic!("array select must return [index, value], got {result:?}");
        };
        // seed 0 → start at index 0 (recv).
        assert_eq!(parts[0].as_str(), "0");
        assert_eq!(parts[1], Value::I64(7));

        // Only send ready on next poll.
        let ops2 = Value::Array(vec![
            Value::Array(vec![Value::I64(0), recv_ch]),
            Value::Array(vec![Value::I64(1), send_ch.clone(), Value::I64(99)]),
        ]);
        stack.push(ops2);
        host_channel_select(&mut stack).unwrap();
        let result2 = stack.pop().unwrap();
        let Value::Array(parts2) = result2 else {
            panic!("expected [index, value], got {result2:?}");
        };
        assert_eq!(parts2[0].as_str(), "1");
        assert_eq!(parts2[1], Value::Bool(true));

        let mut drain = vec![send_ch];
        host_channel_try_recv(&mut drain).unwrap();
        assert_eq!(drain.pop(), Some(Value::I64(99)));
    }
}
