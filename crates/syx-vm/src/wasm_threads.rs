use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;

use crate::{platform_int_value, Value};

static NEXT_SAB_ID: AtomicI64 = AtomicI64::new(1);

struct SharedAtomicsBuffer {
    cells: Vec<AtomicI32>,
}

static SAB_TABLE: LazyLock<Mutex<HashMap<i64, SharedAtomicsBuffer>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn sab_from_value(value: &Value) -> Option<i64> {
    match value {
        Value::Object(map) => map.get("__wasm_sab").map(|v| v.as_i64()),
        _ => None,
    }
}

fn make_sab_handle(id: i64, len: i32) -> Value {
    let mut obj = HashMap::new();
    obj.insert("__wasm_sab".into(), platform_int_value(id));
    obj.insert("length".into(), platform_int_value(len as i64));
    Value::Object(obj)
}

pub fn host_wasm_sab_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    let len = stack.pop().unwrap_or(Value::I32(0)).as_i32().max(0) as usize;
    let id = NEXT_SAB_ID.fetch_add(1, Ordering::Relaxed);
    let cells = (0..len).map(|_| AtomicI32::new(0)).collect();
    SAB_TABLE
        .lock()
        .unwrap()
        .insert(id, SharedAtomicsBuffer { cells });
    stack.push(make_sab_handle(id, len as i32));
    Ok(())
}

pub fn host_wasm_atomic_add_i32(stack: &mut Vec<Value>) -> SyxResult<()> {
    let delta = stack.pop().unwrap_or(Value::I32(0)).as_i32();
    let index = stack.pop().unwrap_or(Value::I32(0)).as_i32() as usize;
    let sab = stack.pop().unwrap_or(Value::Nil);
    let id = sab_from_value(&sab).unwrap_or(0);
    let table = SAB_TABLE.lock().unwrap();
    let Some(buf) = table.get(&id) else {
        stack.push(Value::I32(0));
        return Ok(());
    };
    let old = if index < buf.cells.len() {
        buf.cells[index].fetch_add(delta, Ordering::SeqCst)
    } else {
        0
    };
    stack.push(Value::I32(old + delta));
    Ok(())
}

pub fn wasm_threads_mvp_enabled() -> bool {
    std::env::var("SYX_WASM_THREADS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}
