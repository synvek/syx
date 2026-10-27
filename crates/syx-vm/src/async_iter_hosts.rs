use std::cell::RefCell;
use std::collections::HashMap;

use syx_diagnostics::{SyxDiagnostic, SyxResult};

use crate::{ensure_object_id, platform_int_value, Value};

struct AsyncIterState {
    idx: usize,
    items: Vec<Value>,
}

thread_local! {
    static ASYNC_ITERS: RefCell<HashMap<i64, AsyncIterState>> = RefCell::new(HashMap::new());
    static NEXT_ASYNC_ITER_ID: RefCell<i64> = RefCell::new(1);
}

fn iter_step(idx: usize, items: &[Value]) -> Value {
    if idx >= items.len() {
        let mut map = HashMap::new();
        map.insert("__async_iter_done".into(), Value::Bool(true));
        map.insert("__async_iter_value".into(), Value::Nil);
        ensure_object_id(&mut map);
        return Value::Object(map);
    }
    let mut map = HashMap::new();
    map.insert("__async_iter_done".into(), Value::Bool(false));
    map.insert("__async_iter_value".into(), items[idx].clone());
    ensure_object_id(&mut map);
    Value::Object(map)
}

fn wrap_awaitable(step: Value) -> Value {
    let mut map = HashMap::new();
    map.insert("__await_value".into(), step);
    ensure_object_id(&mut map);
    Value::Object(map)
}

pub fn host_async_iter_from(stack: &mut Vec<Value>) -> SyxResult<()> {
    let iterable = stack.pop().unwrap_or(Value::Nil);
    let items = match iterable {
        Value::Array(items) => items,
        _ => {
            return Err(SyxDiagnostic::error("Expected array for async iteration").with_code("A003"))
        }
    };
    let id = NEXT_ASYNC_ITER_ID.with(|next| {
        let mut next = next.borrow_mut();
        let id = *next;
        *next += 1;
        id
    });
    ASYNC_ITERS.with(|iters| {
        iters
            .borrow_mut()
            .insert(id, AsyncIterState { idx: 0, items });
    });
    let mut map = HashMap::new();
    map.insert("__async_iter_id".into(), platform_int_value(id));
    ensure_object_id(&mut map);
    stack.push(Value::Object(map));
    Ok(())
}

fn iter_id_from_value(v: &Value) -> SyxResult<i64> {
    if let Value::Object(map) = v {
        map.get("__async_iter_id")
            .and_then(|id| match id {
                Value::I64(n) => Some(*n),
                Value::I32(n) => Some(*n as i64),
                _ => None,
            })
            .ok_or_else(|| SyxDiagnostic::error("Expected async iterator handle").with_code("A003"))
    } else {
        Err(SyxDiagnostic::error("Expected async iterator handle").with_code("A003"))
    }
}

pub fn host_async_iter_next(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = stack.pop().unwrap_or(Value::Nil);
    let id = iter_id_from_value(&handle)?;
    let step = ASYNC_ITERS.with(|iters| {
        let mut iters = iters.borrow_mut();
        let state = iters.get_mut(&id).ok_or_else(|| {
            SyxDiagnostic::error(format!("Unknown async iterator id {id}")).with_code("A003")
        })?;
        let step = iter_step(state.idx, &state.items);
        state.idx += 1;
        Ok(step)
    })?;
    stack.push(wrap_awaitable(step));
    Ok(())
}
