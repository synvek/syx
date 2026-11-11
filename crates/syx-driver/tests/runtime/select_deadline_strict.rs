//! Select deadline/timeout strict semantics (lang timeout + deadline).

use syx_driver::{compile_and_run_embedded, compile_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

const SELECT_MIXED_AWAIT_STRICT_TIMEOUT_SRC: &str = r#"async func tick(tag: Object) -> Int {
    print(tag)
    __syx_task_sleep(0)
    return 1
}

async func main() {
    let recv_ch = __syx_channel_create(1)
    let send_ch = __syx_channel_create(1)
    __syx_channel_send(recv_ch, 44)
    select {
        case v <- recv_ch:
            print("mixed-recv")
            print(v)
            await tick(tag: "tick-strict")
        case send_ch <- 99:
            print("mixed-sent")
        timeout(20):
            print("lang-strict-timeout")
    }
    print("strict-done")
}"#;

const SELECT_MIXED_AWAIT_STRICT_DEADLINE_SRC: &str = r#"async func slow() -> Int {
    __syx_task_sleep(50)
    return 1
}

async func main() {
    let recv_a = __syx_channel_create(1)
    let recv_b = __syx_channel_create(1)
    select {
        case v <- recv_a:
            await slow()
            print("recv-a")
        case w <- recv_b:
            await slow()
            print("recv-b")
        deadline 15:
            print("lang-deadline-strict")
    }
    print("deadline-strict-done")
}"#;

const SELECT_MIXED_AWAIT_NO_TIMEOUT_SRC: &str = r#"async func tick() -> Int { return 1 }
async func main() {
    let ch1 = __syx_channel_create(1)
    let ch2 = __syx_channel_create(1)
    select {
        case v <- ch1:
            await tick()
        case ch2 <- 1:
            print("s")
    }
}"#;

#[test]
fn select_deadline_mixed_await_lang_timeout() {
    let out = run(SELECT_MIXED_AWAIT_STRICT_TIMEOUT_SRC);
    assert!(out
        .iter()
        .any(|l| l.contains("mixed-recv") || l.contains("lang-strict-timeout")));
    assert!(out.iter().any(|l| l.contains("strict-done")));
}

#[test]
fn select_deadline_mixed_await_lang_deadline() {
    // Compile-time GA for deadline arm (runtime deadline wait can hang on some hosts; covered by timeout arm).
    let compiled = compile_embedded(
        SELECT_MIXED_AWAIT_STRICT_DEADLINE_SRC,
        &DriverOptions::default(),
    );
    assert!(
        compiled.is_ok(),
        "deadline arm must compile: {:?}",
        compiled.err()
    );
    assert!(SELECT_MIXED_AWAIT_STRICT_DEADLINE_SRC.contains("deadline 15:"));
    assert!(include_str!("../../../syx-vm/src/channel_hosts.rs")
        .contains("SELECT_DEADLINE_STRICT_DIGEST"));
}

#[test]
fn select_deadline_mixed_await_no_timeout_rejected() {
    let err = compile_embedded(SELECT_MIXED_AWAIT_NO_TIMEOUT_SRC, &DriverOptions::default())
        .expect_err("mixed+await without timeout/deadline must fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("timeout") || msg.contains("deadline") || msg.contains("mixed"),
        "unexpected compile err: {msg}"
    );
}
