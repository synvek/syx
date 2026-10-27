use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, Instant as MonoInstant, SystemTime, UNIX_EPOCH};

use chrono::Offset;
use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::{platform_int_value, Value};

/// Process-local monotonic origin for Instant.now() (ADR-220 / M2-S5-01).
static INSTANT_ORIGIN: OnceLock<MonoInstant> = OnceLock::new();

pub fn host_env_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    match std::env::var(&key) {
        Ok(v) => stack.push(Value::Str(v)),
        Err(_) => stack.push(Value::Nil),
    }
    Ok(())
}

pub fn host_env_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let key = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    std::env::set_var(&key, &value);
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_path_join(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let a = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let joined = Path::new(&a).join(b);
    stack.push(Value::Str(joined.to_string_lossy().into_owned()));
    Ok(())
}

fn normalize_path_str(path: &str) -> String {
    let p = Path::new(path);
    let mut out = PathBuf::new();
    for comp in p.components() {
        match comp {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    out.to_string_lossy().into_owned()
}

pub fn host_path_normalize(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    stack.push(Value::Str(normalize_path_str(&path)));
    Ok(())
}

pub fn host_path_exists(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    stack.push(Value::Bool(Path::new(&path).exists()));
    Ok(())
}

pub fn host_path_extension(stack: &mut Vec<Value>) -> SyxResult<()> {
    let path = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let ext = Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    stack.push(Value::Str(ext));
    Ok(())
}

pub fn host_time_now_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| SyxDiagnostic::error(format!("Time error: {e}")))?
        .as_millis() as i64;
    stack.push(platform_int_value(ms));
    Ok(())
}

pub fn host_time_sleep_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64().max(0) as u64;
    std::thread::sleep(Duration::from_millis(ms));
    stack.push(Value::Nil);
    Ok(())
}

/// M2-S5-01 / ADR-220: monotonic ms since process-local Instant origin (not wall/epoch).
pub fn host_instant_now_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    let origin = INSTANT_ORIGIN.get_or_init(MonoInstant::now);
    let ms = origin.elapsed().as_millis() as i64;
    stack.push(platform_int_value(ms));
    Ok(())
}

/// M2-S5-04 / ADR-222: wall-clock DateTime object (replaces parse("unsupported") preview hack).
pub fn host_datetime_now(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    stack.push(datetime_object_from_ms(ms));
    Ok(())
}

/// M0-B01 / ADR-133: narrow Gregorian + RFC3339 UTC/fixed-offset MVP.
/// ADR-233 / M2-S5-02: named timezone APIs via chrono-tz.
pub fn host_datetime_parse(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let ms = parse_rfc3339_ms(&text)?;
    stack.push(datetime_object_from_ms(ms));
    Ok(())
}

/// Format epoch ms. Pattern `"iso"` / `"rfc3339"` / empty → RFC3339 UTC; else chrono strftime.
pub fn host_datetime_format(stack: &mut Vec<Value>) -> SyxResult<()> {
    let pattern = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let dt = ms_to_datetime(ms)?;
    let out = if pattern.is_empty() || pattern == "iso" || pattern == "rfc3339" {
        dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    } else {
        dt.format(&pattern).to_string()
    };
    stack.push(Value::Str(out));
    Ok(())
}

/// RFC3339 / ISO-8601 subset parse → DateTime object.
pub fn host_datetime_parse_iso8601(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let ms = parse_rfc3339_ms(&text)?;
    stack.push(datetime_object_from_ms(ms));
    Ok(())
}

/// ADR-233 / M2-S5-02: format epoch ms in IANA zone. Pattern `"iso"`/`"rfc3339"`/empty → RFC3339 local.
pub fn host_datetime_format_with_timezone(stack: &mut Vec<Value>) -> SyxResult<()> {
    let timezone = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let pattern = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let tz = parse_iana_tz(&timezone)?;
    let utc = ms_to_datetime(ms)?;
    let local = utc.with_timezone(&tz);
    let out = if pattern.is_empty() || pattern == "iso" || pattern == "rfc3339" {
        local.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    } else {
        local.format(&pattern).to_string()
    };
    stack.push(Value::Str(out));
    Ok(())
}

/// ADR-233 / M2-S5-02: offset minutes east of UTC at epoch `ms` in IANA zone (EST=-300, EDT=-240).
pub fn host_datetime_timezone_offset(stack: &mut Vec<Value>) -> SyxResult<()> {
    let timezone = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let tz = parse_iana_tz(&timezone)?;
    let utc = ms_to_datetime(ms)?;
    let local = utc.with_timezone(&tz);
    let mins = local.offset().fix().local_minus_utc() / 60;
    stack.push(platform_int_value(i64::from(mins)));
    Ok(())
}

fn parse_iana_tz(name: &str) -> SyxResult<chrono_tz::Tz> {
    name.parse::<chrono_tz::Tz>().map_err(|_| {
        SyxDiagnostic::error(format!(
            "DateTime: unknown IANA timezone {name:?} (no tzdb match)"
        ))
        .with_code("U001")
    })
}

fn datetime_object_from_ms(ms: i64) -> Value {
    let mut map = std::collections::HashMap::new();
    map.insert("__type".into(), Value::Str("DateTime".into()));
    map.insert("ms".into(), platform_int_value(ms));
    if let Ok(dt) = ms_to_datetime(ms) {
        map.insert(
            "iso".into(),
            Value::Str(dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)),
        );
    }
    Value::Object(map)
}

fn ms_to_datetime(ms: i64) -> SyxResult<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::from_timestamp_millis(ms).ok_or_else(|| {
        SyxDiagnostic::error(format!("DateTime: invalid epoch ms {ms}")).with_code("U002")
    })
}

fn parse_rfc3339_ms(text: &str) -> SyxResult<i64> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(text) {
        return Ok(dt.timestamp_millis());
    }
    // Accept date-only YYYY-MM-DD as UTC midnight.
    if let Ok(d) = chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d") {
        let dt = d
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| SyxDiagnostic::error("DateTime: invalid date").with_code("U002"))?
            .and_utc();
        return Ok(dt.timestamp_millis());
    }
    Err(SyxDiagnostic::error(format!(
        "DateTime.parse: expected RFC3339 or YYYY-MM-DD, got {text:?}"
    ))
    .with_code("U002"))
}

pub fn host_ordered_set_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut map = std::collections::HashMap::new();
    map.insert("__type".into(), Value::Str("OrderedSet".into()));
    map.insert("items".into(), Value::Array(Vec::new()));
    stack.push(Value::mut_ref_cell_from_map(map));
    Ok(())
}

pub fn host_ordered_set_insert(stack: &mut Vec<Value>) -> SyxResult<()> {
    let item = stack.pop().unwrap_or(Value::Nil);
    let set = stack.pop().unwrap_or(Value::Nil);
    if set.is_mut_ref_cell() {
        let items = set.mut_ref_cell_get_field("items");
        if let Value::Array(mut items) = items {
            if !items.iter().any(|v| v == &item) {
                items.push(item);
            }
            stack.push(Value::mut_ref_cell_set_field(
                set,
                "items",
                Value::Array(items),
            ));
            return Ok(());
        }
    }
    stack.push(Value::Bool(false));
    Ok(())
}

pub fn host_ordered_set_contains(stack: &mut Vec<Value>) -> SyxResult<()> {
    let item = stack.pop().unwrap_or(Value::Nil);
    let set = stack.pop().unwrap_or(Value::Nil);
    let found = match set.mut_ref_cell_get_field("items") {
        Value::Array(items) => items.iter().any(|v| v == &item),
        _ => false,
    };
    stack.push(Value::Bool(found));
    Ok(())
}

/// ADR-248 / M2-S1-03: Deque&lt;Int&gt; MVP — HeapSlotKind::Deque + Array backing.
fn deque_items(receiver: &Value) -> Vec<Value> {
    let resolved = match receiver {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(receiver),
        other => other.clone(),
    };
    match resolved {
        Value::Array(v) => v,
        _ => Vec::new(),
    }
}

fn deque_write_back(receiver: Value, items: Vec<Value>) {
    let _ = Value::write_back_heap_slot(receiver, Value::Array(items));
}

pub fn host_deque_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    let empty = Value::Array(Vec::new());
    stack.push(Value::alloc_heap_slot(
        empty,
        syx_runtime_core::HeapSlotKind::Deque,
    ));
    Ok(())
}

pub fn host_deque_push_back(stack: &mut Vec<Value>) -> SyxResult<()> {
    let item = stack.pop().unwrap_or(Value::Nil);
    let receiver = stack.pop().unwrap_or(Value::Nil);
    let mut items = deque_items(&receiver);
    items.push(item);
    deque_write_back(receiver, items);
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_deque_push_front(stack: &mut Vec<Value>) -> SyxResult<()> {
    let item = stack.pop().unwrap_or(Value::Nil);
    let receiver = stack.pop().unwrap_or(Value::Nil);
    let mut items = deque_items(&receiver);
    items.insert(0, item);
    deque_write_back(receiver, items);
    stack.push(Value::Nil);
    Ok(())
}

pub fn host_deque_pop_back(stack: &mut Vec<Value>) -> SyxResult<()> {
    let receiver = stack.pop().unwrap_or(Value::Nil);
    let mut items = deque_items(&receiver);
    let out = items.pop().unwrap_or(Value::Nil);
    deque_write_back(receiver, items);
    stack.push(out);
    Ok(())
}

pub fn host_deque_pop_front(stack: &mut Vec<Value>) -> SyxResult<()> {
    let receiver = stack.pop().unwrap_or(Value::Nil);
    let mut items = deque_items(&receiver);
    let out = if items.is_empty() {
        Value::Nil
    } else {
        items.remove(0)
    };
    deque_write_back(receiver, items);
    stack.push(out);
    Ok(())
}

pub fn host_deque_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let receiver = stack.pop().unwrap_or(Value::Nil);
    let items = deque_items(&receiver);
    stack.push(platform_int_value(items.len() as i64));
    Ok(())
}

pub fn host_deque_is_empty(stack: &mut Vec<Value>) -> SyxResult<()> {
    let receiver = stack.pop().unwrap_or(Value::Nil);
    let items = deque_items(&receiver);
    stack.push(Value::Bool(items.is_empty()));
    Ok(())
}
