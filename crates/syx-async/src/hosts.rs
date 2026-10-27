use std::collections::HashMap;

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_vm::{
    platform_int_value, register_actor_shared_pool_hosts, register_concurrency_control_hosts,
    register_task_group_cancel_host, register_task_hosts, register_task_live_count_host,
    register_task_local_hosts, register_actor_reentrancy_policy_host,
    register_blocking_isolation_hosts, register_blocking_pool_sizing_hosts,
    register_blocking_execute, register_io_adapter_policy_host, register_task_pool_sizing_hosts,
    register_task_set_deadline_ms_host, register_trace_context_providers, Value,
};

use crate::runtime::{
    actor_reentrancy_policy, actor_shared_pool_enabled, configured_actor_shared_pool_size,
    configured_blocking_pool_size, configured_core_pool_size, configured_keep_alive_ms,
    configured_live_worker_count, configured_max_pool_size, configured_task_queue_capacity,
    configured_task_reject_policy, configured_worker_count, current_processing_actor,
    current_runtime, isolation_policy, io_adapter_policy, mailbox_capacity, max_actors,
    set_actor_shared_pool_enabled, set_actor_shared_pool_size, set_configured_blocking_pool_size,
    set_configured_core_pool_size, set_configured_keep_alive_ms, set_configured_max_pool_size,
    set_configured_task_queue_capacity, set_configured_task_reject_policy,
    set_configured_worker_count, set_mailbox_capacity, set_max_actors, TaskRejectPolicy,
    SyxRuntime,
};
use crate::task::task_id_from_value;

pub fn install_hosts() {
    syx_vm::register_mixed_await_cancel_probe(|| crate::task::task_should_stop());
    register_trace_context_providers(
        crate::task::current_task_id,
        current_processing_actor,
    );
    register_blocking_execute(|job| {
        if let Some(rt) = current_runtime() {
            rt.execute_on_blocking_pool(job);
        } else {
            std::thread::spawn(move || job());
        }
    });
    register_task_hosts(
        host_task_detach,
        host_task_join,
        host_task_cancel,
        host_task_sleep,
        host_main_actor_run,
        host_await_task,
        host_task_group_create,
        host_task_group_add,
        host_task_group_wait,
    );
    register_task_group_cancel_host(host_task_group_cancel);
    register_task_live_count_host(host_task_live_count);
    register_task_set_deadline_ms_host(host_task_set_deadline_ms);
    register_task_local_hosts(host_task_local_set, host_task_local_get);
    register_actor_reentrancy_policy_host(host_actor_reentrancy_policy);
    register_blocking_isolation_hosts(host_blocking_sleep, host_isolation_policy);
    register_blocking_pool_sizing_hosts(host_blocking_pool_set_size, host_blocking_pool_size);
    register_io_adapter_policy_host(host_io_adapter_policy);
    register_concurrency_control_hosts(
        host_task_set_worker_count,
        host_task_worker_count,
        host_task_set_queue_capacity,
        host_task_queue_capacity,
        host_task_set_reject_policy,
        host_task_reject_policy,
        host_actor_set_max_actors,
        host_actor_max_actors,
        host_actor_set_mailbox_capacity,
        host_actor_mailbox_capacity,
    );
    register_task_pool_sizing_hosts(
        host_task_set_core_pool_size,
        host_task_core_pool_size,
        host_task_set_max_pool_size,
        host_task_max_pool_size,
        host_task_set_keep_alive_ms,
        host_task_keep_alive_ms,
        host_task_live_worker_count,
    );
    register_actor_shared_pool_hosts(
        host_actor_set_shared_pool_enabled,
        host_actor_shared_pool_enabled,
        host_actor_set_shared_pool_size,
        host_actor_shared_pool_size,
    );
}

pub fn resolve_actor_method(
    rt: &SyxRuntime,
    method_name: &str,
    actor_value: &Value,
) -> SyxResult<u32> {
    let type_name = actor_type_name(actor_value);
    let qualified = if method_name.contains('_') {
        method_name.to_string()
    } else {
        format!("{type_name}_{method_name}")
    };
    rt.function_index(&qualified)
}

fn actor_type_name(actor_value: &Value) -> String {
    if let Value::Object(map) = actor_value {
        if let Some(Value::Str(name)) = map.get("__type") {
            return name.clone();
        }
    }
    "Actor".into()
}

pub fn wrap_await_value(value: Value) -> Value {
    let mut map = HashMap::new();
    map.insert("__await_value".into(), value);
    Value::Object(map)
}

fn host_task_detach(stack: &mut Vec<Value>) -> SyxResult<()> {
    let top = stack.pop().unwrap_or(Value::Nil);
    let bottom = stack.pop();
    // ConstStr may be HeapSlot under string-offheap — use as_str(), not `Value::Str` only.
    let top_str = {
        let s = top.as_str();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    };
    let bottom_str = bottom.as_ref().and_then(|v| {
        let s = v.as_str();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    });
    let top_arr = match &top {
        Value::Array(items) => Some(items.clone()),
        Value::HeapSlot { .. } => match Value::resolve_heap_slot(&top) {
            Value::Array(items) => Some(items),
            _ => None,
        },
        _ => None,
    };
    let bottom_arr = bottom.as_ref().and_then(|v| match v {
        Value::Array(items) => Some(items.clone()),
        Value::HeapSlot { .. } => match Value::resolve_heap_slot(v) {
            Value::Array(items) => Some(items),
            _ => None,
        },
        _ => None,
    });
    let (name, args) = match (top_str, bottom_str, top_arr, bottom_arr) {
        (Some(n), _, _, Some(items)) => (n, items),
        (_, Some(n), Some(items), _) => (n, items),
        (Some(n), _, _, _) => (n, vec![]),
        (_, Some(n), _, _) => (n, vec![]),
        _ => {
            return Err(SyxDiagnostic::error(format!(
                "TaskDetach expected function name, top={top:?} bottom={bottom:?}"
            )));
        }
    };
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let handle = rt.spawn_detached_by_name(&name, args)?;
    stack.push(handle.to_value());
    Ok(())
}

fn host_task_join(stack: &mut Vec<Value>) -> SyxResult<()> {
    let task = stack.pop().unwrap_or(Value::Nil);
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let value = rt.vm_runtime().join_task_value(&task)?;
    let mut map = HashMap::new();
    map.insert("__done".into(), Value::Bool(true));
    map.insert("__value".into(), value);
    stack.push(Value::Object(map));
    Ok(())
}

fn host_task_cancel(stack: &mut Vec<Value>) -> SyxResult<()> {
    let task = stack.pop().unwrap_or(Value::Nil);
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    rt.vm_runtime().cancel_task_value(&task)?;
    stack.push(Value::Bool(true));
    Ok(())
}

fn host_task_sleep(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i32() as i64;
    if let Some(rt) = current_runtime() {
        rt.vm_runtime().task_sleep_ms(ms)?;
    } else {
        crate::runtime::await_sleep_ms(ms.max(0) as u64);
        if crate::task::task_should_stop() {
            return Err(SyxDiagnostic::error("Task was cancelled").with_code("C004"));
        }
    }
    stack.push(Value::Nil);
    Ok(())
}

fn host_main_actor_run(stack: &mut Vec<Value>) -> SyxResult<()> {
    let name = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let value = rt.vm_runtime().main_actor_run_fn(&name, vec![])?;
    stack.push(value);
    Ok(())
}

fn host_await_task(stack: &mut Vec<Value>) -> SyxResult<()> {
    let task = stack.pop().unwrap_or(Value::Nil);
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let value = rt.vm_runtime().await_task_value(&task)?;
    stack.push(value);
    Ok(())
}

fn host_task_group_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    // Wave A W276 / ADR-620: prefer wasm/native MIR bridge when active.
    if let Some(ops) = syx_vm::bridge_task_group_ops() {
        if (ops.active)() {
            return (ops.create)(stack);
        }
    }
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let parent = crate::task::current_task_cancel().unwrap_or_default();
    let (id, group) = rt.create_task_group(parent);
    stack.push(group.to_value_with_id(id));
    Ok(())
}

fn host_task_group_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    if let Some(ops) = syx_vm::bridge_task_group_ops() {
        if (ops.active)() {
            return (ops.add)(stack);
        }
    }
    let task = stack.pop().unwrap_or(Value::Nil);
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let task_id = task_id_from_value(&task).ok_or_else(|| SyxDiagnostic::error("Expected task"))?;
    let group_id = crate::task::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    let handle = rt
        .tasks
        .get(task_id)
        .ok_or_else(|| SyxDiagnostic::error("Unknown task"))?;
    let group = rt
        .groups
        .lock()
        .get(&group_id)
        .cloned()
        .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
    group.add(handle);
    stack.push(Value::Bool(true));
    Ok(())
}

fn host_task_group_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    if let Some(ops) = syx_vm::bridge_task_group_ops() {
        if (ops.active)() {
            return (ops.wait)(stack);
        }
    }
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let group_id = crate::task::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    let group = rt
        .groups
        .lock()
        .get(&group_id)
        .cloned()
        .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
    group.wait_all()?;
    stack.push(Value::Nil);
    Ok(())
}

fn host_task_group_cancel(stack: &mut Vec<Value>) -> SyxResult<()> {
    if let Some(ops) = syx_vm::bridge_task_group_ops() {
        if (ops.active)() {
            return (ops.cancel)(stack);
        }
    }
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let rt = current_runtime().ok_or_else(|| SyxDiagnostic::error("No runtime"))?;
    let group_id = crate::task::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    let group = rt
        .groups
        .lock()
        .get(&group_id)
        .cloned()
        .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
    group.cancel_all();
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-261 / M3-A01: unreaped TaskRegistry live count (silent-orphan diagnostic).
/// Wave A W275 / ADR-619: prefer bridge provider (wasm/native MIR ctx) when set.
fn host_task_live_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = syx_vm::bridge_task_live_count().unwrap_or_else(crate::runtime::task_live_count);
    stack.push(platform_int_value(n as i64));
    Ok(())
}

/// ADR-262 / M3-A02: set task-scoped deadline (ms from now); children inherit.
fn host_task_set_deadline_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64().max(0) as u64;
    crate::task::set_deadline_ms(ms);
    crate::task::apply_deadline_to_cancel();
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-263 / M3-A02: set task-local key → value (children inherit a clone).
fn host_task_local_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Nil);
    let key = stack.pop().unwrap_or(Value::Nil).as_str();
    crate::task::task_local_set(key, value);
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-263 / M3-A02: get task-local value (Nil if missing).
fn host_task_local_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = stack.pop().unwrap_or(Value::Nil).as_str();
    stack.push(crate::task::task_local_get(&key));
    Ok(())
}

/// ADR-264 / M3-A03: readonly reentrancy policy fingerprint.
fn host_actor_reentrancy_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str(actor_reentrancy_policy().into()));
    Ok(())
}

/// ADR-275 / M3-B01: sleep on dedicated blocking pool (help-while-wait on Task pool).
fn host_blocking_sleep(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i32() as i64;
    if let Some(rt) = current_runtime() {
        rt.blocking_sleep_ms(ms);
    } else {
        crate::runtime::await_sleep_ms(ms.max(0) as u64);
    }
    if crate::task::task_should_stop() {
        return Err(SyxDiagnostic::error("Task was cancelled").with_code("C004"));
    }
    stack.push(Value::Nil);
    Ok(())
}

/// ADR-275 / M3-B01: readonly Task vs blocking pool isolation fingerprint.
fn host_isolation_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str(isolation_policy().into()));
    Ok(())
}

/// ADR-276 / M3-B01: set blocking pool size (core=max=n).
fn host_blocking_pool_set_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(1)).as_i64().max(1) as usize;
    let set = set_configured_blocking_pool_size(n);
    stack.push(platform_int_value(set as i64));
    Ok(())
}

/// ADR-276 / M3-B01: get blocking pool size.
fn host_blocking_pool_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_blocking_pool_size() as i64));
    Ok(())
}

/// ADR-278 / M3-B02: readonly I/O adapter policy fingerprint.
fn host_io_adapter_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str(io_adapter_policy().into()));
    Ok(())
}

fn host_task_set_worker_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(1)).as_i64().max(1) as usize;
    let set = set_configured_worker_count(n);
    stack.push(platform_int_value(set as i64));
    Ok(())
}

fn host_task_worker_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_worker_count() as i64));
    Ok(())
}

fn host_task_set_queue_capacity(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let set = set_configured_task_queue_capacity(n);
    stack.push(platform_int_value(set));
    Ok(())
}

fn host_task_queue_capacity(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_task_queue_capacity()));
    Ok(())
}

fn host_task_set_reject_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    let raw = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let policy = match raw.trim().to_ascii_lowercase().as_str() {
        "abort" => TaskRejectPolicy::Abort,
        "callerruns" | "caller_runs" | "caller-runs" => TaskRejectPolicy::CallerRuns,
        "discardoldest" | "discard_oldest" | "discard-oldest" => {
            TaskRejectPolicy::DiscardOldest
        }
        _ => TaskRejectPolicy::Abort,
    };
    let set = set_configured_task_reject_policy(policy);
    stack.push(Value::Str(set.as_str().into()));
    Ok(())
}

fn host_task_reject_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str(
        configured_task_reject_policy().as_str().into(),
    ));
    Ok(())
}

fn host_task_set_core_pool_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(1)).as_i64().max(1) as usize;
    let set = set_configured_core_pool_size(n);
    stack.push(platform_int_value(set as i64));
    Ok(())
}

fn host_task_core_pool_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_core_pool_size() as i64));
    Ok(())
}

fn host_task_set_max_pool_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(1)).as_i64().max(1) as usize;
    let set = set_configured_max_pool_size(n);
    stack.push(platform_int_value(set as i64));
    Ok(())
}

fn host_task_max_pool_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_max_pool_size() as i64));
    Ok(())
}

fn host_task_set_keep_alive_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let set = set_configured_keep_alive_ms(n);
    stack.push(platform_int_value(set));
    Ok(())
}

fn host_task_keep_alive_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_keep_alive_ms()));
    Ok(())
}

fn host_task_live_worker_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_live_worker_count() as i64));
    Ok(())
}

fn host_actor_set_max_actors(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    stack.push(platform_int_value(set_max_actors(n)));
    Ok(())
}

fn host_actor_max_actors(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(max_actors()));
    Ok(())
}

fn host_actor_set_mailbox_capacity(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    stack.push(platform_int_value(set_mailbox_capacity(n)));
    Ok(())
}

fn host_actor_mailbox_capacity(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(mailbox_capacity()));
    Ok(())
}

fn host_actor_set_shared_pool_enabled(stack: &mut Vec<Value>) -> SyxResult<()> {
    let enabled = stack.pop().unwrap_or(Value::Bool(false)).as_bool();
    stack.push(Value::Bool(set_actor_shared_pool_enabled(enabled)));
    Ok(())
}

fn host_actor_shared_pool_enabled(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Bool(actor_shared_pool_enabled()));
    Ok(())
}

fn host_actor_set_shared_pool_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = stack.pop().unwrap_or(Value::I32(1)).as_i64().max(1) as usize;
    stack.push(platform_int_value(set_actor_shared_pool_size(n) as i64));
    Ok(())
}

fn host_actor_shared_pool_size(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(platform_int_value(configured_actor_shared_pool_size() as i64));
    Ok(())
}
