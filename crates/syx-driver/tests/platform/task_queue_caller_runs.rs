//! Task queue CallerRuns reject policy MVP (`runtime-concurrency-v3:caller-runs-mvp`).

use std::sync::Mutex;

use syx_driver::{compile_and_run_embedded, DriverOptions};

static P135_QUEUE_LOCK: Mutex<()> = Mutex::new(());

fn run(src: &str) -> Result<Vec<String>, String> {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => Ok(r.printed),
        Err(e) => Err(format!("{e}")),
    }
}

#[test]
fn caller_runs_task_reject_policy_roundtrip() {
    let _g = P135_QUEUE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(
        r#"
func main() {
    print(__syx_task_set_reject_policy("callerRuns"))
    print(__syx_task_reject_policy())
    print(__syx_task_set_reject_policy("abort"))
    print(__syx_task_reject_policy())
}
"#,
    )
    .expect("reject policy roundtrip");
    assert_eq!(printed[0].trim(), "callerRuns");
    assert_eq!(printed[1].trim(), "callerRuns");
    assert_eq!(printed[2].trim(), "abort");
    assert_eq!(printed[3].trim(), "abort");
}

#[test]
fn caller_runs_bounded_queue_flood_caller_runs_observable() {
    let _g = P135_QUEUE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    syx_async::probe_bounded_queue_caller_runs_mvp().expect("CallerRuns overflow should succeed");
}

#[test]
fn caller_runs_package_and_runtime_fingerprints() {
    let concurrency = include_str!("../../../../packages/core/concurrency.sy");
    assert!(
        concurrency.contains("setTaskRejectPolicy") && concurrency.contains("taskRejectPolicy")
    );
    let runtime_rs = include_str!("../../../syx-async/src/runtime.rs");
    assert!(runtime_rs.contains("runtime-concurrency-v3:caller-runs-mvp"));
}
