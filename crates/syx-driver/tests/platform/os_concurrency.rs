//! Os queries + Task/Actor/Channel concurrency control.

use std::sync::Mutex;

use syx_driver::{compile_and_run_embedded, DriverOptions};

/// Serializes tests that mutate process-wide Task/Actor knobs (`MAX_ACTORS`, workers, mailbox).
static P133_CONCURRENCY_LOCK: Mutex<()> = Mutex::new(());

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn with_os(body: &str) -> String {
    let os = include_str!("../../../../packages/os/mod.sy");
    format!("{os}\n{body}")
}

fn run(src: &str) -> Result<Vec<String>, String> {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &opts()) {
        Ok(r) => Ok(r.printed),
        Err(e) => Err(format!("{e}")),
    }
}

#[test]
fn os_concurrency_cpu_and_user_positive() {
    let printed = run(&with_os(
        r#"
func main() {
    print(Os.cpuCount())
    try {
        print(Os.userName())
    } catch (e) {
        print("userName-unavailable")
    }
}
"#,
    ))
    .expect("os queries should run");
    let cpu: i64 = printed[0].trim().parse().expect("cpuCount int");
    assert!(cpu > 0, "cpuCount should be positive, got {cpu}");
    assert!(
        !printed[1].trim().is_empty(),
        "userName should be non-empty or unavailable marker"
    );
}

#[test]
fn os_concurrency_set_worker_count_roundtrip() {
    let _g = P133_CONCURRENCY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(
        r#"
func main() {
    print(__syx_task_set_worker_count(3))
    print(__syx_task_worker_count())
}
"#,
    )
    .expect("workerCount roundtrip");
    assert_eq!(printed[0].trim(), "3");
    assert_eq!(printed[1].trim(), "3");
}

#[test]
fn os_concurrency_set_max_actors_overflow_fails() {
    let _g = P133_CONCURRENCY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    syx_async::set_max_actors(0);
    let err = run(
        r#"
actor Counter {
    var value: Int
    init(value: Int) {
        self.value = value
    }
    func increment() {
        self.value = self.value + 1
    }
}
func main() {
    __syx_actor_set_max_actors(1)
    let a = Counter(value: 0)
    a.increment()
    let b = Counter(value: 0)
    b.increment()
    print("unreachable")
}
"#,
    )
    .expect_err("second actor executor should fail");
    assert!(
        err.contains("C007") || err.contains("Actor limit") || err.contains("maxActors"),
        "expected C007 / maxActors error, got: {err}"
    );
    syx_async::set_max_actors(0);
}

#[test]
fn os_concurrency_max_actors_host_roundtrip() {
    let _g = P133_CONCURRENCY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(
        r#"
func main() {
    print(__syx_actor_set_max_actors(5))
    print(__syx_actor_max_actors())
    print(__syx_actor_set_max_actors(0))
}
"#,
    )
    .expect("maxActors host roundtrip");
    assert_eq!(printed[0].trim(), "5");
    assert_eq!(printed[1].trim(), "5");
    assert_eq!(printed[2].trim(), "0");
}

#[test]
fn os_concurrency_channel_length_capacity() {
    let printed = run(
        r#"
func main() {
    let ch = __syx_channel_create(2)
    print(__syx_channel_capacity(ch))
    print(__syx_channel_length(ch))
    __syx_channel_try_send(ch, 7)
    print(__syx_channel_length(ch))
}
"#,
    )
    .expect("channel length/capacity");
    assert_eq!(printed[0].trim(), "2");
    assert_eq!(printed[1].trim(), "0");
    assert_eq!(printed[2].trim(), "1");
}

#[test]
fn os_concurrency_mailbox_capacity_roundtrip() {
    let _g = P133_CONCURRENCY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let printed = run(
        r#"
func main() {
    print(__syx_actor_set_mailbox_capacity(4))
    print(__syx_actor_mailbox_capacity())
    print(__syx_actor_set_mailbox_capacity(0))
    print(__syx_actor_mailbox_capacity())
}
"#,
    )
    .expect("mailbox capacity roundtrip");
    assert_eq!(printed[0].trim(), "4");
    assert_eq!(printed[1].trim(), "4");
    assert_eq!(printed[2].trim(), "0");
    assert_eq!(printed[3].trim(), "0");
}

#[test]
fn os_concurrency_package_surfaces() {
    let os = include_str!("../../../../packages/os/mod.sy");
    assert!(os.contains("cpuCount") && os.contains("memoryTotal") && os.contains("userName"));
    let concurrency = include_str!("../../../../packages/core/concurrency.sy");
    assert!(concurrency.contains("setWorkerCount") && concurrency.contains("setMaxActors"));
    assert!(concurrency.contains("setMailboxCapacity"));
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(sync.contains("func length") && sync.contains("func capacity"));
}
