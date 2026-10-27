//! Phase 83: stdlib GA host functions (Fmt, Encoding, Math float, Process, File+, UDP/DNS, Crypto, Mutex, WsRecv, Log, Testing).

use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{ToSocketAddrs, UdpSocket};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::Instant;

use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_runtime_core::{platform_int_value, value_debug_description, value_description, Value};

/// ADR-282: held flag + Condvar; Arc so waiters release the global table before parking.
struct SyxMutexEntry {
    held: Mutex<bool>,
    cv: Condvar,
}

static MUTEX_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static MUTEX_TABLE: OnceLock<Mutex<HashMap<i64, Arc<SyxMutexEntry>>>> = OnceLock::new();

/// ADR-284 / M3-C02: counting permits + Condvar (independent of Mutex table).
struct SyxSemaphoreEntry {
    permits: Mutex<i64>,
    cv: Condvar,
}

static SEMAPHORE_NEXT_ID: AtomicI64 = AtomicI64::new(1);
static SEMAPHORE_TABLE: OnceLock<Mutex<HashMap<i64, Arc<SyxSemaphoreEntry>>>> = OnceLock::new();
static LAST_EXIT_CODE: AtomicI64 = AtomicI64::new(0);
static UDP_HANDLES: OnceLock<Mutex<HashMap<i64, UdpSocket>>> = OnceLock::new();
static UDP_NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);

fn mutex_table() -> &'static Mutex<HashMap<i64, Arc<SyxMutexEntry>>> {
    MUTEX_TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn semaphore_table() -> &'static Mutex<HashMap<i64, Arc<SyxSemaphoreEntry>>> {
    SEMAPHORE_TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn udp_handles() -> &'static Mutex<HashMap<i64, UdpSocket>> {
    UDP_HANDLES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    let normalized = if s.len() % 2 != 0 {
        format!("0{s}")
    } else {
        s.to_string()
    };
    (0..normalized.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&normalized[i..i + 2], 16).ok())
        .collect()
}

fn value_as_f64(v: &Value) -> f64 {
    match v {
        Value::F64(n) => *n,
        Value::F32(n) => f64::from(*n),
        _ => v.as_i64() as f64,
    }
}

fn value_as_f32(v: &Value) -> f32 {
    match v {
        Value::F32(n) => *n,
        Value::F64(n) => *n as f32,
        _ => v.as_i64() as f32,
    }
}

fn value_to_bytes(v: &Value) -> Vec<u8> {
    if let Some(items) = Value::materialize_array(v.clone()) {
        return items
            .iter()
            .map(|x| match x {
                Value::U8(b) => *b,
                _ => x.as_i64().clamp(0, 255) as u8,
            })
            .collect();
    }
    v.as_str().into_bytes()
}

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(data: &[u8]) -> String {
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64[((n >> 18) & 63) as usize] as char);
        out.push(B64[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(B64[((n >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64[(n & 63) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let mut decode = |c: u8| -> Option<u8> {
        if (b'A'..=b'Z').contains(&c) {
            Some(c - b'A')
        } else if (b'a'..=b'z').contains(&c) {
            Some(c - b'a' + 26)
        } else if (b'0'..=b'9').contains(&c) {
            Some(c - b'0' + 52)
        } else if c == b'+' {
            Some(62)
        } else if c == b'/' {
            Some(63)
        } else {
            None
        }
    };
    let bytes: Vec<u8> = s
        .bytes()
        .filter(|&b| b != b'=' && !b.is_ascii_whitespace())
        .collect();
    let mut out = Vec::new();
    for chunk in bytes.chunks(4) {
        let vals: Vec<u8> = chunk.iter().filter_map(|&b| decode(b)).collect();
        if vals.is_empty() {
            continue;
        }
        let n = (vals[0] as u32) << 18
            | vals.get(1).copied().unwrap_or(0) as u32 * 4096
            | vals.get(2).copied().unwrap_or(0) as u32 * 64
            | vals.get(3).copied().unwrap_or(0) as u32;
        out.push((n >> 16) as u8);
        if vals.len() > 2 {
            out.push((n >> 8) as u8);
        }
        if vals.len() > 3 {
            out.push(n as u8);
        }
    }
    Some(out)
}

fn fmt_replace(template: &str, replacements: &[Value]) -> String {
    let mut out = template.to_string();
    for (i, val) in replacements.iter().enumerate() {
        let needle = format!("{{{i}}}");
        if out.contains(&needle) {
            out = out.replace(&needle, &value_description(val));
        }
    }
    out
}

fn log_with_level(level: &str, stack: &mut Vec<Value>) -> SyxResult<()> {
    let msg = stack.pop().unwrap_or(Value::Nil);
    if !log_level_enabled(level) {
        stack.push(Value::Nil);
        return Ok(());
    }
    let line = format!("[{level}] {}", value_description(&msg));
    println!("{line}");
    set_log_last_emitted(line);
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-292: DEBUG=0 .. ERROR=3; default DEBUG so existing Log.* keep emitting.
static LOG_MIN_LEVEL: AtomicI64 = AtomicI64::new(0);
static LOG_LAST_EMITTED: OnceLock<Mutex<String>> = OnceLock::new();

fn log_last_slot() -> &'static Mutex<String> {
    LOG_LAST_EMITTED.get_or_init(|| Mutex::new(String::new()))
}

fn set_log_last_emitted(line: String) {
    if let Ok(mut g) = log_last_slot().lock() {
        *g = line;
    }
}

fn log_level_rank(level: &str) -> Option<i64> {
    match level.to_ascii_uppercase().as_str() {
        "DEBUG" => Some(0),
        "INFO" => Some(1),
        "WARN" => Some(2),
        "ERROR" => Some(3),
        _ => None,
    }
}

fn log_level_name(rank: i64) -> &'static str {
    match rank {
        0 => "DEBUG",
        1 => "INFO",
        2 => "WARN",
        _ => "ERROR",
    }
}

fn log_level_enabled(level: &str) -> bool {
    let Some(rank) = log_level_rank(level) else {
        return false;
    };
    rank >= LOG_MIN_LEVEL.load(Ordering::Relaxed)
}

// --- ADR-295 / ADR-296 / M3-D02: Trace span stack (shared with Log.*Fields inject) ---

thread_local! {
    static TRACE_SPAN_STACK: std::cell::RefCell<Vec<(String, String)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

static TRACE_SPAN_SEQ: AtomicI64 = AtomicI64::new(1);

fn format_log_fields_line(level: &str, message: &str, fields: &HashMap<String, Value>) -> String {
    let mut keys: Vec<&String> = fields.keys().collect();
    keys.sort();
    let mut out = format!("[{level}] {message}");
    for k in keys {
        if let Some(v) = fields.get(k) {
            out.push(' ');
            out.push_str(k);
            out.push('=');
            out.push_str(&value_description(v));
        }
    }
    out
}

/// ADR-296 / M3-D02: correlation values shared with Trace.contextLine.
fn trace_correlation_parts() -> (String, String, String) {
    let span = TRACE_SPAN_STACK.with(|s| {
        s.borrow()
            .last()
            .map(|(id, _)| id.clone())
            .unwrap_or_else(|| "-".into())
    });
    let task = crate::task_hosts::trace_current_task_id()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "-".into());
    let actor = crate::task_hosts::trace_current_actor_id()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "-".into());
    (span, task, actor)
}

/// Merge span/task_id/actor_id when the caller dict does not already define them.
fn inject_trace_correlation(fields: &mut HashMap<String, Value>) {
    let (span, task, actor) = trace_correlation_parts();
    fields
        .entry("span".into())
        .or_insert_with(|| Value::Str(span));
    fields
        .entry("task_id".into())
        .or_insert_with(|| Value::Str(task));
    fields
        .entry("actor_id".into())
        .or_insert_with(|| Value::Str(actor));
}

fn emit_log_fields(level: &str, stack: &mut Vec<Value>) -> SyxResult<()> {
    let fields_raw = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let message = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    if !log_level_enabled(level) {
        stack.push(Value::Str(String::new()));
        return Ok(());
    }
    let mut fields = Value::materialize_dict(fields_raw).unwrap_or_default();
    inject_trace_correlation(&mut fields);
    let line = format_log_fields_line(level, &message, &fields);
    println!("{line}");
    set_log_last_emitted(line.clone());
    stack.push(Value::Str(line));
    Ok(())
}

pub fn host_log_set_min_level(stack: &mut Vec<Value>) -> SyxResult<()> {
    let level = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    match log_level_rank(&level) {
        Some(rank) => {
            LOG_MIN_LEVEL.store(rank, Ordering::Relaxed);
            stack.push(Value::Bool(true));
        }
        None => stack.push(Value::Bool(false)),
    }
    Ok(())
}

pub fn host_log_min_level(stack: &mut Vec<Value>) -> SyxResult<()> {
    let rank = LOG_MIN_LEVEL.load(Ordering::Relaxed);
    stack.push(Value::Str(log_level_name(rank).into()));
    Ok(())
}

pub fn host_log_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str("log-v1-fields".into()));
    Ok(())
}

pub fn host_log_info_fields(stack: &mut Vec<Value>) -> SyxResult<()> {
    emit_log_fields("INFO", stack)
}

pub fn host_log_debug_fields(stack: &mut Vec<Value>) -> SyxResult<()> {
    emit_log_fields("DEBUG", stack)
}

pub fn host_log_warn_fields(stack: &mut Vec<Value>) -> SyxResult<()> {
    emit_log_fields("WARN", stack)
}

pub fn host_log_error_fields(stack: &mut Vec<Value>) -> SyxResult<()> {
    emit_log_fields("ERROR", stack)
}

pub fn host_log_take_last(stack: &mut Vec<Value>) -> SyxResult<()> {
    let line = if let Ok(mut g) = log_last_slot().lock() {
        std::mem::take(&mut *g)
    } else {
        String::new()
    };
    stack.push(Value::Str(line));
    Ok(())
}

// --- ADR-295 / ADR-296 / M3-D02: Trace HostFn ---

pub fn host_trace_start(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let id = format!("s{}", TRACE_SPAN_SEQ.fetch_add(1, Ordering::Relaxed));
    TRACE_SPAN_STACK.with(|s| {
        s.borrow_mut().push((id.clone(), name));
    });
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_trace_end(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ok = TRACE_SPAN_STACK.with(|s| s.borrow_mut().pop().is_some());
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_trace_current(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = TRACE_SPAN_STACK.with(|s| {
        s.borrow()
            .last()
            .map(|(id, _)| id.clone())
            .unwrap_or_default()
    });
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_trace_context_line(stack: &mut Vec<Value>) -> SyxResult<()> {
    let (span, task, actor) = trace_correlation_parts();
    stack.push(Value::Str(format!(
        "span={span} task_id={task} actor_id={actor}"
    )));
    Ok(())
}

pub fn host_trace_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str("trace-v1-logctx".into()));
    Ok(())
}

// --- ADR-298 / ADR-299 / M3-D03: Metrics Counter + Gauge + labels ---

static METRICS_COUNTERS: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();
static METRICS_GAUGES: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();

fn metrics_counters() -> &'static Mutex<HashMap<String, i64>> {
    METRICS_COUNTERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn metrics_gauges() -> &'static Mutex<HashMap<String, i64>> {
    METRICS_GAUGES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Labeled series key: `name{k1=v1,k2=v2}` (label keys sorted). Empty labels → plain `name`.
fn metrics_labels_series_key(name: &str, labels: &HashMap<String, Value>) -> String {
    if labels.is_empty() {
        return name.to_string();
    }
    let mut keys: Vec<&String> = labels.keys().collect();
    keys.sort();
    let pairs: Vec<String> = keys
        .iter()
        .filter_map(|k| {
            labels
                .get(*k)
                .map(|v| format!("{k}={}", value_description(v)))
        })
        .collect();
    format!("{name}{{{}}}", pairs.join(","))
}

pub fn host_metrics_counter_inc(stack: &mut Vec<Value>) -> SyxResult<()> {
    let delta = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let new_val = if let Ok(mut g) = metrics_counters().lock() {
        let e = g.entry(name).or_insert(0);
        *e = e.saturating_add(delta);
        *e
    } else {
        0
    };
    stack.push(platform_int_value(new_val));
    Ok(())
}

pub fn host_metrics_counter_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let val = if let Ok(g) = metrics_counters().lock() {
        g.get(&name).copied().unwrap_or(0)
    } else {
        0
    };
    stack.push(platform_int_value(val));
    Ok(())
}

pub fn host_metrics_counter_reset(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let ok = if let Ok(mut g) = metrics_counters().lock() {
        g.remove(&name);
        true
    } else {
        false
    };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_metrics_snapshot(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut entries: Vec<(String, i64)> = Vec::new();
    if let Ok(g) = metrics_counters().lock() {
        for (k, v) in g.iter() {
            entries.push((k.clone(), *v));
        }
    }
    if let Ok(g) = metrics_gauges().lock() {
        for (k, v) in g.iter() {
            entries.push((format!("g:{k}"), *v));
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let line = entries
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(" ");
    stack.push(Value::Str(line));
    Ok(())
}

pub fn host_metrics_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str("metrics-v1-gauge-labels".into()));
    Ok(())
}

pub fn host_metrics_gauge_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let written = if let Ok(mut g) = metrics_gauges().lock() {
        g.insert(name, value);
        value
    } else {
        0
    };
    stack.push(platform_int_value(written));
    Ok(())
}

pub fn host_metrics_gauge_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let val = if let Ok(g) = metrics_gauges().lock() {
        g.get(&name).copied().unwrap_or(0)
    } else {
        0
    };
    stack.push(platform_int_value(val));
    Ok(())
}

pub fn host_metrics_counter_inc_labels(stack: &mut Vec<Value>) -> SyxResult<()> {
    let labels_raw = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let delta = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let labels = Value::materialize_dict(labels_raw).unwrap_or_default();
    let key = metrics_labels_series_key(&name, &labels);
    let new_val = if let Ok(mut g) = metrics_counters().lock() {
        let e = g.entry(key).or_insert(0);
        *e = e.saturating_add(delta);
        *e
    } else {
        0
    };
    stack.push(platform_int_value(new_val));
    Ok(())
}

pub fn host_metrics_counter_get_labels(stack: &mut Vec<Value>) -> SyxResult<()> {
    let labels_raw = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let labels = Value::materialize_dict(labels_raw).unwrap_or_default();
    let key = metrics_labels_series_key(&name, &labels);
    let val = if let Ok(g) = metrics_counters().lock() {
        g.get(&key).copied().unwrap_or(0)
    } else {
        0
    };
    stack.push(platform_int_value(val));
    Ok(())
}

// --- ADR-301 / ADR-302 / M3-D04: Profiler wall-clock + process CPU sections ---

static PROFILER_ACTIVE: OnceLock<Mutex<HashMap<String, Instant>>> = OnceLock::new();
static PROFILER_TOTALS: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();
static PROFILER_CPU_ACTIVE: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();
static PROFILER_CPU_TOTALS: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();

fn profiler_active() -> &'static Mutex<HashMap<String, Instant>> {
    PROFILER_ACTIVE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn profiler_totals() -> &'static Mutex<HashMap<String, i64>> {
    PROFILER_TOTALS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn profiler_cpu_active() -> &'static Mutex<HashMap<String, i64>> {
    PROFILER_CPU_ACTIVE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn profiler_cpu_totals() -> &'static Mutex<HashMap<String, i64>> {
    PROFILER_CPU_TOTALS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Process user+system CPU time in ms. Unix: getrusage; non-Unix: 0.
fn process_cpu_ms() -> i64 {
    #[cfg(unix)]
    {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        let rc = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) };
        if rc != 0 {
            return 0;
        }
        let usage = unsafe { usage.assume_init() };
        let user = usage.ru_utime.tv_sec as i64 * 1000 + usage.ru_utime.tv_usec as i64 / 1000;
        let sys = usage.ru_stime.tv_sec as i64 * 1000 + usage.ru_stime.tv_usec as i64 / 1000;
        user.saturating_add(sys)
    }
    #[cfg(not(unix))]
    {
        0
    }
}

pub fn host_profiler_start(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let ok = if let Ok(mut g) = profiler_active().lock() {
        if g.contains_key(&name) {
            false
        } else {
            g.insert(name, Instant::now());
            true
        }
    } else {
        false
    };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_profiler_stop(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let elapsed_ms = if let Ok(mut active) = profiler_active().lock() {
        if let Some(started) = active.remove(&name) {
            let ms = started.elapsed().as_millis() as i64;
            if let Ok(mut totals) = profiler_totals().lock() {
                let e = totals.entry(name).or_insert(0);
                *e = e.saturating_add(ms);
            }
            ms
        } else {
            0
        }
    } else {
        0
    };
    stack.push(platform_int_value(elapsed_ms));
    Ok(())
}

pub fn host_profiler_elapsed(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let val = if let Ok(g) = profiler_totals().lock() {
        g.get(&name).copied().unwrap_or(0)
    } else {
        0
    };
    stack.push(platform_int_value(val));
    Ok(())
}

pub fn host_profiler_reset(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let ok = {
        let a = if let Ok(mut g) = profiler_active().lock() {
            g.remove(&name);
            true
        } else {
            false
        };
        let t = if let Ok(mut g) = profiler_totals().lock() {
            g.remove(&name);
            true
        } else {
            false
        };
        let ca = if let Ok(mut g) = profiler_cpu_active().lock() {
            g.remove(&name);
            true
        } else {
            false
        };
        let ct = if let Ok(mut g) = profiler_cpu_totals().lock() {
            g.remove(&name);
            true
        } else {
            false
        };
        a && t && ca && ct
    };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_profiler_snapshot(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut entries: Vec<(String, i64)> = Vec::new();
    if let Ok(g) = profiler_totals().lock() {
        for (k, v) in g.iter() {
            entries.push((k.clone(), *v));
        }
    }
    if let Ok(g) = profiler_cpu_totals().lock() {
        for (k, v) in g.iter() {
            entries.push((format!("c:{k}"), *v));
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let line = entries
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(" ");
    stack.push(Value::Str(line));
    Ok(())
}

pub fn host_profiler_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str("profiler-v1-cpu".into()));
    Ok(())
}

pub fn host_profiler_cpu_start(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let ok = if let Ok(mut g) = profiler_cpu_active().lock() {
        if g.contains_key(&name) {
            false
        } else {
            g.insert(name, process_cpu_ms());
            true
        }
    } else {
        false
    };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_profiler_cpu_stop(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let elapsed_ms = if let Ok(mut active) = profiler_cpu_active().lock() {
        if let Some(started) = active.remove(&name) {
            let now = process_cpu_ms();
            let ms = now.saturating_sub(started).max(0);
            if let Ok(mut totals) = profiler_cpu_totals().lock() {
                let e = totals.entry(name).or_insert(0);
                *e = e.saturating_add(ms);
            }
            ms
        } else {
            0
        }
    } else {
        0
    };
    stack.push(platform_int_value(elapsed_ms));
    Ok(())
}

pub fn host_profiler_cpu_elapsed(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Nil).as_str().to_string();
    let val = if let Ok(g) = profiler_cpu_totals().lock() {
        g.get(&name).copied().unwrap_or(0)
    } else {
        0
    };
    stack.push(platform_int_value(val));
    Ok(())
}

fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, dk_len: usize) -> Vec<u8> {
    let mut dk = vec![0u8; dk_len];
    let blocks = (dk_len + 31) / 32;
    for block in 1..=blocks {
        let mut u = {
            let mut mac =
                Hmac::<Sha256>::new_from_slice(password).expect("HMAC accepts any key length");
            mac.update(salt);
            mac.update(&(block as u32).to_be_bytes());
            mac.finalize().into_bytes().to_vec()
        };
        let mut block_out = u.clone();
        for _ in 1..iterations {
            let mut mac =
                Hmac::<Sha256>::new_from_slice(password).expect("HMAC accepts any key length");
            mac.update(&u);
            u = mac.finalize().into_bytes().to_vec();
            for (i, b) in u.iter().enumerate() {
                block_out[i] ^= b;
            }
        }
        let start = (block - 1) * 32;
        let end = (start + 32).min(dk_len);
        dk[start..end].copy_from_slice(&block_out[..end - start]);
    }
    dk
}

pub fn set_last_exit_code(code: i32) {
    LAST_EXIT_CODE.store(code as i64, Ordering::Relaxed);
}

fn udp_mvp_enabled() -> bool {
    std::env::var("SYX_UDP_MVP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true)
}

fn dns_mock_enabled() -> bool {
    std::env::var("SYX_DNS_MOCK")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or_else(|_| {
            std::env::var("CI")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        })
}

fn ws_url_from_handle(value: &Value) -> Option<String> {
    match value {
        Value::Object(map) | Value::Dict(map) => map.get("url").map(|v| v.as_str()),
        _ => None,
    }
}

// --- Fmt ---

pub fn host_fmt_print(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    print!("{}", value_description(&v));
    // Wave A W241 / ADR-585: flush so SYX_NATIVE_STRICT AOT captures no-newline print.
    let _ = io::stdout().flush();
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_fmt_print_line(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    println!("{}", value_description(&v));
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_fmt_format(stack: &mut Vec<Value>) -> SyxResult<()> {
    let replacements = stack.pop().unwrap_or(Value::Array(vec![]));
    let template = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let args = Value::materialize_array(replacements).unwrap_or_default();
    stack.push(Value::Str(fmt_replace(&template, &args)));
    Ok(())
}

pub fn host_fmt_debug(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    stack.push(Value::Str(value_debug_description(&v)));
    Ok(())
}

// --- Encoding ---

pub fn host_base64_encode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let input = stack.pop().unwrap_or(Value::Str(String::new()));
    let data = value_to_bytes(&input);
    stack.push(Value::Str(base64_encode(&data)));
    Ok(())
}

pub fn host_base64_decode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let bytes = base64_decode(&s).ok_or_else(|| SyxDiagnostic::error("invalid base64 input"))?;
    stack.push(Value::Str(String::from_utf8_lossy(&bytes).into_owned()));
    Ok(())
}

pub fn host_hex_encode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let input = stack.pop().unwrap_or(Value::Str(String::new()));
    stack.push(Value::Str(bytes_to_hex(&value_to_bytes(&input))));
    Ok(())
}

pub fn host_hex_decode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let bytes = hex_to_bytes(&s).ok_or_else(|| SyxDiagnostic::error("invalid hex input"))?;
    stack.push(Value::Str(String::from_utf8_lossy(&bytes).into_owned()));
    Ok(())
}

pub fn host_url_encode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    stack.push(Value::Str(
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect(),
    ));
    Ok(())
}

pub fn host_url_decode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let s = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let decoded: String = url::form_urlencoded::parse(s.as_bytes())
        .map(|(k, v)| {
            if v.is_empty() {
                k.into_owned()
            } else {
                format!("{}={}", k, v)
            }
        })
        .collect::<Vec<_>>()
        .join("&");
    stack.push(Value::Str(decoded));
    Ok(())
}

// --- Math float (stack: last arg popped first) ---

pub fn host_math_float_sin(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.sin()));
    Ok(())
}

pub fn host_math_float_cos(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.cos()));
    Ok(())
}

pub fn host_math_float_tan(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.tan()));
    Ok(())
}

pub fn host_math_float_sqrt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.sqrt()));
    Ok(())
}

pub fn host_math_float_pow(stack: &mut Vec<Value>) -> SyxResult<()> {
    let exp = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let base = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(base.powf(exp)));
    Ok(())
}

pub fn host_math_float_floor(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.floor()));
    Ok(())
}

pub fn host_math_float_ceil(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.ceil()));
    Ok(())
}

pub fn host_math_float_round(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.round()));
    Ok(())
}

pub fn host_math_float_log(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.ln()));
    Ok(())
}

pub fn host_math_float_asin(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.asin()));
    Ok(())
}

pub fn host_math_float_acos(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.acos()));
    Ok(())
}

pub fn host_math_float_atan(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.atan()));
    Ok(())
}

pub fn host_math_float_atan2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let y = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(y.atan2(x)));
    Ok(())
}

pub fn host_math_float_exp(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.exp()));
    Ok(())
}

pub fn host_math_float_exp2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.exp2()));
    Ok(())
}

pub fn host_math_float_log2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.log2()));
    Ok(())
}

pub fn host_math_float_log10(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.log10()));
    Ok(())
}

pub fn host_math_float_hypot(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.hypot(y)));
    Ok(())
}

pub fn host_math_float_cbrt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.cbrt()));
    Ok(())
}

pub fn host_math_float_trunc(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.trunc()));
    Ok(())
}

pub fn host_math_float_abs(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.abs()));
    Ok(())
}

pub fn host_math_float_sinh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.sinh()));
    Ok(())
}

pub fn host_math_float_cosh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.cosh()));
    Ok(())
}

pub fn host_math_float_tanh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.tanh()));
    Ok(())
}

pub fn host_math_float_fmod(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x % y));
    Ok(())
}

pub fn host_math_float_copysign(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sign = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let mag = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(mag.copysign(sign)));
    Ok(())
}

pub fn host_math_float_isnan(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::Bool(x.is_nan()));
    Ok(())
}

pub fn host_math_float_isinf(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::Bool(x.is_infinite()));
    Ok(())
}

pub fn host_math_float_isfinite(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::Bool(x.is_finite()));
    Ok(())
}

pub fn host_math_float_expm1(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.exp_m1()));
    Ok(())
}

pub fn host_math_float_log1p(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.ln_1p()));
    Ok(())
}

pub fn host_math_float_asinh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.asinh()));
    Ok(())
}

pub fn host_math_float_acosh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.acosh()));
    Ok(())
}

pub fn host_math_float_atanh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.atanh()));
    Ok(())
}

pub fn host_math_float_remainder(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    // IEEE-style remainder: x - y * round(x/y); ≠ fmod
    let r = if y == 0.0 || x.is_nan() || y.is_nan() {
        f64::NAN
    } else {
        x - y * (x / y).round()
    };
    stack.push(Value::F64(r));
    Ok(())
}

pub fn host_math_float_next_up(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.next_up()));
    Ok(())
}

pub fn host_math_float_next_down(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(x.next_down()));
    Ok(())
}

pub fn host_math_float_mul_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let c = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let b = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    let a = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(a.mul_add(b, c)));
    Ok(())
}

pub fn host_math_float_erf(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(libm::erf(x)));
    Ok(())
}

pub fn host_math_float_erfc(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(libm::erfc(x)));
    Ok(())
}

pub fn host_math_float_tgamma(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    stack.push(Value::F64(libm::tgamma(x)));
    Ok(())
}

pub fn host_math_float_lgamma(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f64(&stack.pop().unwrap_or(Value::F64(0.0)));
    // libm::lgamma returns ln|Γ(x)| (magnitude); sign via lgamma_r if needed later
    stack.push(Value::F64(libm::lgamma(x)));
    Ok(())
}

pub fn host_math_float32_sqrt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.sqrt()));
    Ok(())
}

pub fn host_math_float32_sin(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.sin()));
    Ok(())
}

pub fn host_math_float32_cos(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.cos()));
    Ok(())
}

pub fn host_math_float32_tan(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.tan()));
    Ok(())
}

pub fn host_math_float32_exp(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.exp()));
    Ok(())
}

pub fn host_math_float32_log(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.ln()));
    Ok(())
}

pub fn host_math_float32_abs(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.abs()));
    Ok(())
}

pub fn host_math_float32_floor(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.floor()));
    Ok(())
}

pub fn host_math_float32_ceil(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.ceil()));
    Ok(())
}

pub fn host_math_float32_round(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.round()));
    Ok(())
}

pub fn host_math_float32_trunc(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.trunc()));
    Ok(())
}

pub fn host_math_float32_pow(stack: &mut Vec<Value>) -> SyxResult<()> {
    let exp = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let base = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(base.powf(exp)));
    Ok(())
}

pub fn host_math_float32_asin(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.asin()));
    Ok(())
}

pub fn host_math_float32_acos(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.acos()));
    Ok(())
}

pub fn host_math_float32_atan(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.atan()));
    Ok(())
}

pub fn host_math_float32_hypot(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.hypot(y)));
    Ok(())
}

pub fn host_math_float32_atan2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let y = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(y.atan2(x)));
    Ok(())
}

pub fn host_math_float32_log2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.log2()));
    Ok(())
}

pub fn host_math_float32_log10(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.log10()));
    Ok(())
}

pub fn host_math_float32_exp2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.exp2()));
    Ok(())
}

pub fn host_math_float32_cbrt(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.cbrt()));
    Ok(())
}

pub fn host_math_float32_sinh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.sinh()));
    Ok(())
}

pub fn host_math_float32_cosh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.cosh()));
    Ok(())
}

pub fn host_math_float32_tanh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.tanh()));
    Ok(())
}

pub fn host_math_float32_fmod(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x % y));
    Ok(())
}

pub fn host_math_float32_copysign(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sign = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let mag = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(mag.copysign(sign)));
    Ok(())
}

pub fn host_math_float32_expm1(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.exp_m1()));
    Ok(())
}

pub fn host_math_float32_log1p(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.ln_1p()));
    Ok(())
}

pub fn host_math_float32_asinh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.asinh()));
    Ok(())
}

pub fn host_math_float32_acosh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.acosh()));
    Ok(())
}

pub fn host_math_float32_atanh(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.atanh()));
    Ok(())
}

pub fn host_math_float32_erf(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(libm::erff(x)));
    Ok(())
}

pub fn host_math_float32_erfc(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(libm::erfcf(x)));
    Ok(())
}

pub fn host_math_float32_tgamma(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(libm::tgammaf(x)));
    Ok(())
}

pub fn host_math_float32_lgamma(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(libm::lgammaf(x)));
    Ok(())
}

pub fn host_math_float32_remainder(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let r = if y == 0.0 || x.is_nan() || y.is_nan() {
        f32::NAN
    } else {
        x - y * (x / y).round()
    };
    stack.push(Value::F32(r));
    Ok(())
}

pub fn host_math_float32_isnan(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::Bool(x.is_nan()));
    Ok(())
}

pub fn host_math_float32_isinf(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::Bool(x.is_infinite()));
    Ok(())
}

pub fn host_math_float32_isfinite(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::Bool(x.is_finite()));
    Ok(())
}

pub fn host_math_float32_next_up(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.next_up()));
    Ok(())
}

pub fn host_math_float32_next_down(stack: &mut Vec<Value>) -> SyxResult<()> {
    let x = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(x.next_down()));
    Ok(())
}

pub fn host_math_float32_mul_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let c = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let b = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    let a = value_as_f32(&stack.pop().unwrap_or(Value::F32(0.0)));
    stack.push(Value::F32(a.mul_add(b, c)));
    Ok(())
}

// --- Process ---

pub fn host_process_args(stack: &mut Vec<Value>) -> SyxResult<()> {
    let args: Vec<Value> = std::env::args().map(|s| Value::Str(s)).collect();
    stack.push(Value::Array(args));
    Ok(())
}

pub fn host_process_env(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let val = std::env::var(&key).unwrap_or_default();
    stack.push(Value::Str(val));
    Ok(())
}

pub fn host_process_exec(stack: &mut Vec<Value>) -> SyxResult<()> {
    let args_val = stack.pop().unwrap_or(Value::Array(vec![]));
    let command = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let args = Value::materialize_array(args_val).unwrap_or_default();
    let cmd_args: Vec<String> = args.iter().map(|v| v.as_str()).collect();
    let output = Command::new(&command).args(&cmd_args).output();
    match output {
        Ok(out) => {
            let code = out.status.code().unwrap_or(-1);
            set_last_exit_code(code);
            stack.push(platform_int_value(code as i64));
        }
        Err(_) => {
            set_last_exit_code(127);
            stack.push(platform_int_value(127));
        }
    }
    Ok(())
}

pub fn host_process_exit_code(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(LAST_EXIT_CODE.load(Ordering::Relaxed)));
    Ok(())
}

// --- File ---

#[cfg(feature = "file")]
pub fn host_file_stat(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let meta = std::fs::metadata(&path).map_err(|e| SyxDiagnostic::error(format!("stat: {e}")))?;
    let mut obj = HashMap::new();
    obj.insert("size".into(), platform_int_value(meta.len() as i64));
    obj.insert("isFile".into(), Value::Bool(meta.is_file()));
    obj.insert("isDir".into(), Value::Bool(meta.is_dir()));
    obj.insert(
        "modifiedMs".into(),
        platform_int_value(
            meta.modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as i64)
                .unwrap_or(0),
        ),
    );
    stack.push(Value::Object(obj));
    Ok(())
}

#[cfg(feature = "file")]
pub fn host_file_symlink(stack: &mut Vec<Value>) -> SyxResult<()> {
    let target = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let link = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&target, &link)
            .map_err(|e| SyxDiagnostic::error(format!("symlink: {e}")))?;
    }
    #[cfg(not(unix))]
    {
        let _ = (target, link);
        return Err(SyxDiagnostic::error(
            "symlink not supported on this platform",
        ));
    }
    stack.push(Value::Bool(true));
    Ok(())
}

#[cfg(feature = "file")]
pub fn host_file_read_link(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let target =
        std::fs::read_link(&path).map_err(|e| SyxDiagnostic::error(format!("readLink: {e}")))?;
    stack.push(Value::Str(target.to_string_lossy().into_owned()));
    Ok(())
}

#[cfg(feature = "file")]
pub fn host_file_chmod(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mode = stack.pop().unwrap_or(Value::I64(0o644)).as_i64() as u32;
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(mode);
        std::fs::set_permissions(&path, perms)
            .map_err(|e| SyxDiagnostic::error(format!("chmod: {e}")))?;
    }
    #[cfg(not(unix))]
    {
        let _ = (mode, path);
        return Err(SyxDiagnostic::error("chmod not supported on this platform"));
    }
    stack.push(Value::Bool(true));
    Ok(())
}

#[cfg(not(feature = "file"))]
pub fn host_file_stat(_stack: &mut Vec<Value>) -> SyxResult<()> {
    Err(SyxDiagnostic::error(
        "file host disabled (enable `file` feature)",
    ))
}

#[cfg(not(feature = "file"))]
pub fn host_file_symlink(_stack: &mut Vec<Value>) -> SyxResult<()> {
    Err(SyxDiagnostic::error(
        "file host disabled (enable `file` feature)",
    ))
}

#[cfg(not(feature = "file"))]
pub fn host_file_read_link(_stack: &mut Vec<Value>) -> SyxResult<()> {
    Err(SyxDiagnostic::error(
        "file host disabled (enable `file` feature)",
    ))
}

#[cfg(not(feature = "file"))]
pub fn host_file_chmod(_stack: &mut Vec<Value>) -> SyxResult<()> {
    Err(SyxDiagnostic::error(
        "file host disabled (enable `file` feature)",
    ))
}

// --- UDP / DNS ---

pub fn host_udp_bind(stack: &mut Vec<Value>) -> SyxResult<()> {
    let port = stack.pop().unwrap_or(Value::I64(0)).as_i64() as u16;
    let host = stack
        .pop()
        .unwrap_or(Value::Str("127.0.0.1".into()))
        .as_str();
    let addr = format!("{host}:{port}");
    if udp_mvp_enabled() {
        if let Ok(sock) = UdpSocket::bind(&addr) {
            let id = UDP_NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
            udp_handles().lock().unwrap().insert(id, sock);
            stack.push(platform_int_value(id));
            return Ok(());
        }
    }
    stack.push(platform_int_value(-1));
    Ok(())
}

pub fn host_udp_send_to(stack: &mut Vec<Value>) -> SyxResult<()> {
    let data = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let port = stack.pop().unwrap_or(Value::I64(0)).as_i64() as u16;
    let host = stack
        .pop()
        .unwrap_or(Value::Str("127.0.0.1".into()))
        .as_str();
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let sent = if udp_mvp_enabled() {
        udp_handles()
            .lock()
            .unwrap()
            .get(&handle)
            .and_then(|sock| sock.send_to(data.as_bytes(), format!("{host}:{port}")).ok())
            .unwrap_or(0)
    } else {
        0
    };
    stack.push(platform_int_value(sent as i64));
    Ok(())
}

pub fn host_udp_recv_from(stack: &mut Vec<Value>) -> SyxResult<()> {
    let max_bytes = stack.pop().unwrap_or(Value::I64(4096)).as_i64().max(1) as usize;
    let handle = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let (data, addr, port) = if udp_mvp_enabled() {
        udp_handles()
            .lock()
            .unwrap()
            .get(&handle)
            .and_then(|sock| {
                let mut buf = vec![0u8; max_bytes];
                sock.recv_from(&mut buf)
                    .ok()
                    .map(|(n, peer)| (buf[..n].to_vec(), peer.ip().to_string(), peer.port() as i64))
            })
            .unwrap_or_else(|| (b"mock-udp-recv".to_vec(), "127.0.0.1".into(), 0i64))
    } else {
        (b"mock-udp-recv".to_vec(), "127.0.0.1".into(), 0i64)
    };
    let mut obj = HashMap::new();
    obj.insert(
        "data".into(),
        Value::Array(data.iter().map(|&b| Value::U8(b)).collect()),
    );
    obj.insert("addr".into(), Value::Str(addr));
    obj.insert("port".into(), platform_int_value(port));
    stack.push(Value::Object(obj));
    Ok(())
}

pub fn host_dns_resolve(stack: &mut Vec<Value>) -> SyxResult<()> {
    let host = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    if dns_mock_enabled() {
        stack.push(Value::Array(vec![Value::Str(format!("mock.{host}"))]));
        return Ok(());
    }
    let addrs: Vec<Value> = format!("{host}:0")
        .to_socket_addrs()
        .map(|iter| {
            iter.map(|a| Value::Str(a.ip().to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    stack.push(Value::Array(addrs));
    Ok(())
}

pub fn host_dns_resolve_ip(stack: &mut Vec<Value>) -> SyxResult<()> {
    let host = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    if dns_mock_enabled() {
        stack.push(Value::Str("127.0.0.1".into()));
        return Ok(());
    }
    let ip = format!("{host}:0")
        .to_socket_addrs()
        .ok()
        .and_then(|mut iter| iter.next())
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|| "127.0.0.1".into());
    stack.push(Value::Str(ip));
    Ok(())
}

// --- Crypto ---

pub fn host_crypto_ed25519_sign(stack: &mut Vec<Value>) -> SyxResult<()> {
    let message = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let key_hex = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let key_bytes = hex_to_bytes(&key_hex).unwrap_or_else(|| value_to_bytes(&Value::Str(key_hex)));
    let mut seed = [0u8; 32];
    let take = key_bytes.len().min(32);
    if take == 0 {
        return Err(SyxDiagnostic::error(
            "ed25519 signing key must be non-empty",
        ));
    }
    seed[..take].copy_from_slice(&key_bytes[..take]);
    let signing_key = SigningKey::from_bytes(&seed);
    let sig = signing_key.sign(message.as_bytes());
    stack.push(Value::Str(bytes_to_hex(sig.to_bytes().as_ref())));
    Ok(())
}

pub fn host_crypto_ed25519_verify(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_hex = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let message = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let pubkey_hex = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let ok = match (hex_to_bytes(&pubkey_hex), hex_to_bytes(&sig_hex)) {
        (Some(pk), Some(sig)) if pk.len() >= 32 && sig.len() >= 64 => {
            let pk_arr: [u8; 32] = pk[..32].try_into().unwrap();
            let sig_arr: [u8; 64] = sig[..64].try_into().unwrap();
            VerifyingKey::from_bytes(&pk_arr)
                .ok()
                .and_then(|vk| {
                    vk.verify(
                        message.as_bytes(),
                        &ed25519_dalek::Signature::from_bytes(&sig_arr),
                    )
                    .ok()
                })
                .is_some()
        }
        _ => false,
    };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_crypto_pbkdf2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dk_len = stack.pop().unwrap_or(Value::I64(32)).as_i64().max(1) as usize;
    let iterations = stack.pop().unwrap_or(Value::I64(10_000)).as_i64().max(1) as u32;
    let salt = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let password = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dk = pbkdf2_hmac_sha256(password.as_bytes(), salt.as_bytes(), iterations, dk_len);
    stack.push(Value::Str(bytes_to_hex(&dk)));
    Ok(())
}

// --- Mutex ---

pub fn host_mutex_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = MUTEX_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    mutex_table().lock().unwrap().insert(
        id,
        Arc::new(SyxMutexEntry {
            held: Mutex::new(false),
            cv: Condvar::new(),
        }),
    );
    stack.push(platform_int_value(id));
    Ok(())
}

pub fn host_mutex_lock(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let entry = mutex_table().lock().unwrap().get(&id).cloned();
    let ok = entry
        .map(|entry| {
            let mut guard = entry.held.lock().unwrap();
            if !*guard {
                *guard = true;
                true
            } else {
                false
            }
        })
        .unwrap_or(false);
    stack.push(Value::Bool(ok));
    Ok(())
}

/// ADR-282 / M3-C01: blocking wait until acquired (≠ try-lock `__syx_mutex_lock`).
pub fn host_mutex_lock_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let Some(entry) = mutex_table().lock().unwrap().get(&id).cloned() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    // Park only after releasing the global table lock (via Arc clone above).
    let mut guard = entry.held.lock().unwrap();
    while *guard {
        guard = entry.cv.wait(guard).unwrap();
    }
    *guard = true;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_mutex_unlock(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let entry = mutex_table().lock().unwrap().get(&id).cloned();
    let ok = entry
        .map(|entry| {
            let mut guard = entry.held.lock().unwrap();
            if *guard {
                *guard = false;
                entry.cv.notify_one();
                true
            } else {
                false
            }
        })
        .unwrap_or(false);
    stack.push(Value::Bool(ok));
    Ok(())
}

/// ADR-281 / M3-C01: never-throw quiet unlock for MutexGuard.deinit (unknown/unlocked → success).
pub fn host_mutex_unlock_quiet(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    if let Some(entry) = mutex_table().lock().unwrap().get(&id).cloned() {
        let mut guard = entry.held.lock().unwrap();
        *guard = false;
        entry.cv.notify_one();
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// ADR-282 / M3-C01: Mutex RAII + blocking deepen fingerprint.
pub fn host_sync_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str("mutex-raii-guard-v1-blocking".into()));
    Ok(())
}

// --- Semaphore (ADR-284 / M3-C02) ---

pub fn host_semaphore_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let permits = stack.pop().unwrap_or(Value::I64(0)).as_i64().max(0);
    let id = SEMAPHORE_NEXT_ID.fetch_add(1, Ordering::Relaxed);
    semaphore_table().lock().unwrap().insert(
        id,
        Arc::new(SyxSemaphoreEntry {
            permits: Mutex::new(permits),
            cv: Condvar::new(),
        }),
    );
    stack.push(platform_int_value(id));
    Ok(())
}

pub fn host_semaphore_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let Some(entry) = semaphore_table().lock().unwrap().get(&id).cloned() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let mut guard = entry.permits.lock().unwrap();
    while *guard <= 0 {
        guard = entry.cv.wait(guard).unwrap();
    }
    *guard -= 1;
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_semaphore_try_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let entry = semaphore_table().lock().unwrap().get(&id).cloned();
    let ok = entry
        .map(|entry| {
            let mut guard = entry.permits.lock().unwrap();
            if *guard > 0 {
                *guard -= 1;
                true
            } else {
                false
            }
        })
        .unwrap_or(false);
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_semaphore_signal(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    let entry = semaphore_table().lock().unwrap().get(&id).cloned();
    let ok = entry
        .map(|entry| {
            let mut guard = entry.permits.lock().unwrap();
            *guard += 1;
            entry.cv.notify_one();
            true
        })
        .unwrap_or(false);
    stack.push(Value::Bool(ok));
    Ok(())
}

/// ADR-285 / M3-C02: never-throw quiet signal for SemaphoreGuard.deinit (unknown id → success).
pub fn host_semaphore_signal_quiet(stack: &mut Vec<Value>) -> SyxResult<()> {
    let id = stack.pop().unwrap_or(Value::I64(-1)).as_i64();
    if let Some(entry) = semaphore_table().lock().unwrap().get(&id).cloned() {
        let mut guard = entry.permits.lock().unwrap();
        *guard += 1;
        entry.cv.notify_one();
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// ADR-285 / M3-C02: Semaphore RAII deepen fingerprint (≠ C01 SyncPolicy).
pub fn host_semaphore_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str("semaphore-v1-raii".into()));
    Ok(())
}

// --- WebSocket recv ---

pub fn host_ws_recv(stack: &mut Vec<Value>) -> SyxResult<()> {
    let socket = stack.pop().unwrap_or(Value::Nil);
    if let Value::Object(ref map) = socket {
        if matches!(map.get("rfc6455"), Some(Value::Bool(true))) {
            if let Some(id) = map.get("__ws_conn_id").map(|v| v.as_i64()) {
                if let Some(msg) = crate::ws_rfc6455::rfc6455_recv(id) {
                    stack.push(Value::Str(msg));
                    return Ok(());
                }
            }
        }
    }
    let url = ws_url_from_handle(&socket).unwrap_or_default();
    let wss = url.starts_with("wss://");
    let prefix = if wss { "wss-recv" } else { "recv" };
    let mock = crate::ws::ws_mock_enabled()
        || std::env::var("SYX_WS_MOCK")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
    let msg = if mock || url.is_empty() {
        format!("{prefix}:mock-message")
    } else {
        format!("{prefix}:{url}")
    };
    stack.push(Value::Str(msg));
    Ok(())
}

pub fn host_ws_close(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut socket = stack.pop().unwrap_or(Value::Nil);
    if let Value::Object(ref mut map) = socket {
        if matches!(map.get("rfc6455"), Some(Value::Bool(true))) {
            if let Some(id) = map.get("__ws_conn_id").map(|v| v.as_i64()) {
                crate::ws_rfc6455::rfc6455_close(id);
            }
        }
        map.insert("closed".into(), Value::Bool(true));
    }
    stack.push(Value::Nil);
    Ok(())
}

// --- Array algorithm extensions (Phase 83) ---

fn pop_array(stack: &mut Vec<Value>) -> Vec<Value> {
    match stack.pop() {
        Some(v) => Value::materialize_array(v).unwrap_or_default(),
        None => vec![],
    }
}

pub fn host_array_binary_search(stack: &mut Vec<Value>) -> SyxResult<()> {
    let item = stack.pop().unwrap_or(Value::Nil);
    let items = pop_array(&mut *stack);
    let idx = items
        .iter()
        .position(|v| v == &item || Value::numeric_eq(v, &item))
        .unwrap_or(usize::MAX);
    stack.push(platform_int_value(if idx == usize::MAX {
        -1
    } else {
        idx as i64
    }));
    Ok(())
}

pub fn host_array_chunked(stack: &mut Vec<Value>) -> SyxResult<()> {
    let size = stack.pop().unwrap_or(Value::I32(1)).as_i64().max(1) as usize;
    let items = pop_array(&mut *stack);
    let chunks: Vec<Value> = items
        .chunks(size)
        .map(|c| Value::Array(c.to_vec()))
        .collect();
    stack.push(Value::Array(chunks));
    Ok(())
}

pub fn host_array_zip(stack: &mut Vec<Value>) -> SyxResult<()> {
    let other = pop_array(&mut *stack);
    let items = pop_array(&mut *stack);
    let len = items.len().min(other.len());
    let pairs: Vec<Value> = (0..len)
        .map(|i| Value::Array(vec![items[i].clone(), other[i].clone()]))
        .collect();
    stack.push(Value::Array(pairs));
    Ok(())
}

pub fn host_array_sort_by(stack: &mut Vec<Value>) -> SyxResult<()> {
    let comparator = stack.pop();
    let mut items = pop_array(&mut *stack);
    if let Some(Value::Closure { fn_index, captures }) = comparator {
        #[cfg(feature = "interpreter")]
        {
            items.sort_by(|a, b| {
                let cmp = crate::program_ctx::execute_closure(
                    fn_index,
                    &captures,
                    &[a.clone(), b.clone()],
                );
                match cmp {
                    Value::I32(v) => v.cmp(&0),
                    Value::I64(v) => v.cmp(&0),
                    Value::Bool(b) => b.cmp(&false),
                    _ => syx_runtime_core::value_compare(a, b),
                }
            });
        }
        #[cfg(not(feature = "interpreter"))]
        {
            items.sort_by(syx_runtime_core::value_compare);
        }
    } else {
        items.sort_by(syx_runtime_core::value_compare);
    }
    stack.push(Value::Array(items));
    Ok(())
}

pub fn host_array_flat_map(stack: &mut Vec<Value>) -> SyxResult<()> {
    let transform = stack.pop();
    let items = pop_array(&mut *stack);
    let mut flat = Vec::new();
    if let Some(Value::Closure { fn_index, captures }) = transform {
        #[cfg(feature = "interpreter")]
        {
            for item in items {
                let mapped = crate::program_ctx::execute_closure(fn_index, &captures, &[item]);
                let mapped = Value::resolve_heap_slot(&mapped);
                if let Value::Array(nested) = mapped {
                    flat.extend(nested);
                } else {
                    flat.push(mapped);
                }
            }
        }
        #[cfg(not(feature = "interpreter"))]
        {
            for item in items {
                if let Value::Array(nested) = item {
                    flat.extend(nested);
                } else {
                    flat.push(item);
                }
            }
        }
    } else {
        for item in items {
            if let Value::Array(nested) = item {
                flat.extend(nested);
            } else {
                flat.push(item);
            }
        }
    }
    stack.push(Value::Array(flat));
    Ok(())
}

// --- Log ---

pub fn host_log_debug(stack: &mut Vec<Value>) -> SyxResult<()> {
    log_with_level("DEBUG", stack)
}

pub fn host_log_info(stack: &mut Vec<Value>) -> SyxResult<()> {
    log_with_level("INFO", stack)
}

pub fn host_log_warn(stack: &mut Vec<Value>) -> SyxResult<()> {
    log_with_level("WARN", stack)
}

pub fn host_log_error(stack: &mut Vec<Value>) -> SyxResult<()> {
    log_with_level("ERROR", stack)
}

// --- Testing ---

pub fn host_testing_assert(stack: &mut Vec<Value>) -> SyxResult<()> {
    let msg = stack
        .pop()
        .unwrap_or(Value::Str("assertion failed".into()))
        .as_str();
    let cond = stack.pop().unwrap_or(Value::Bool(false)).as_bool();
    if !cond {
        return Err(SyxDiagnostic::error(format!("assertion failed: {msg}")));
    }
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_testing_assert_eq(stack: &mut Vec<Value>) -> SyxResult<()> {
    let msg = stack
        .pop()
        .unwrap_or(Value::Str("assertEq failed".into()))
        .as_str();
    let expected = stack.pop().unwrap_or(Value::Nil);
    let actual = stack.pop().unwrap_or(Value::Nil);
    if !syx_runtime_core::value_equals(&actual, &expected) {
        return Err(SyxDiagnostic::error(format!(
            "assertEq failed: {msg} — expected {}, got {}",
            value_debug_description(&expected),
            value_debug_description(&actual)
        )));
    }
    stack.push(Value::Nil);
    Ok(())
}

#[cfg(test)]
mod phase85_host_tests {
    use super::*;

    #[test]
    fn ed25519_sign_hex_length() {
        let mut stack = vec![
            Value::Str("p85".into()),
            Value::Str("0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f".into()),
        ];
        host_crypto_ed25519_sign(&mut stack).unwrap();
        let sig = stack.pop().unwrap();
        assert!(sig.as_str().len() >= 64, "sig len {}", sig.as_str().len());
    }
}
