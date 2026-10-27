//! Bounded Task queue + Abort reject MVP (`runtime-concurrency-v2:bounded-queue-abort-mvp`).

use std::sync::Mutex;

use syx_driver::{compile_and_run_embedded, DriverOptions};

static P134_QUEUE_LOCK: Mutex<()> = Mutex::new(());

fn run(src: &str) -> Result<Vec<String>, String> {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => Ok(r.printed),
        Err(e) => Err(format!("{e}")),
    }
}

#[test]
fn bounded_task_queue_task_queue_capacity_roundtrip() {
    let _g = P134_QUEUE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(
        r#"
func main() {
    print(__syx_task_set_queue_capacity(4))
    print(__syx_task_queue_capacity())
    print(__syx_task_set_queue_capacity(0))
    print(__syx_task_queue_capacity())
}
"#,
    )
    .expect("queue capacity roundtrip");
    assert_eq!(printed[0].trim(), "4");
    assert_eq!(printed[1].trim(), "4");
    assert_eq!(printed[2].trim(), "0");
    assert_eq!(printed[3].trim(), "0");
}

#[test]
fn bounded_task_queue_flood_abort_c008() {
    let _g = P134_QUEUE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let err = syx_async::probe_bounded_queue_abort_mvp().expect_err("third spawn should abort");
    assert!(
        err.to_string().contains("C008")
            || err.to_string().contains("Task queue full")
            || err.to_string().contains("Abort"),
        "expected C008 / Abort error, got: {err}"
    );
}

#[test]
fn bounded_task_queue_package_and_runtime_fingerprints() {
    let concurrency = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        concurrency.contains("setTaskQueueCapacity") && concurrency.contains("taskQueueCapacity")
    );
    let runtime_rs = include_str!("../../../syx-async/src/runtime.rs");
    assert!(runtime_rs.contains("runtime-concurrency-v2:bounded-queue-abort-mvp"));
}
