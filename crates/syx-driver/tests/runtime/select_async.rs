//! Phase 103: select case body with `await` + ordered stdout.

use syx_driver::{compile_and_run_embedded, compile_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

const SELECT_AWAIT_CASE_SRC: &str = r#"async func tick() -> Int {
    print("tick-before")
    __syx_task_sleep(0)
    print("tick-after")
    return 7
}

async func main() {
    let ch = __syx_channel_create(1)
    __syx_channel_send(ch, 88)
    select {
        case v <- ch:
            print("recv")
            print(v)
            await tick()
    }
    print("case-order-done")
}"#;

#[test]
fn p103_sync_select_recv_only_smoke() {
    let lines = run(
        r#"func main() {
    let ch = __syx_channel_create(1)
    __syx_channel_send(ch, 88)
    select {
        case v <- ch:
            print(v)
    }
}"#,
    );
    assert!(lines.iter().any(|l| l.contains("88")), "{lines:?}");
}

#[test]
fn p103_async_channel_recv_smoke() {
    let lines = run(
        r#"async func main() {
    let ch = __syx_channel_create(1)
    __syx_channel_send(ch, 88)
    print(__syx_channel_recv(ch))
}"#,
    );
    assert!(lines.iter().any(|l| l.contains("88")), "{lines:?}");
}

#[test]
fn p103_select_await_interleave_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("select-await-interleave-v2:case-await-order"));
}

#[test]
fn p103_select_await_case_compile() {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_embedded(SELECT_AWAIT_CASE_SRC, &DriverOptions::default())
        .expect("select case await must compile");
}

#[test]
fn p103_select_await_case_stdout_order() {
    let lines = run(SELECT_AWAIT_CASE_SRC);
    let recv = lines.iter().position(|l| l.contains("recv")).expect("recv");
    let val = lines.iter().position(|l| l.contains("88")).expect("88");
    let before = lines.iter().position(|l| l.contains("tick-before")).expect("tick-before");
    let after = lines.iter().position(|l| l.contains("tick-after")).expect("tick-after");
    assert!(recv < val && val < before && before < after, "{lines:?}");
}

#[test]
fn p103_select_await_fixture_file() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/runtime/select_await_case_order.sy");
    let src = std::fs::read_to_string(&path).unwrap();
    assert!(src.contains("await"));
    assert!(src.contains("select"));
    let lines = run(&src);
    assert!(lines.iter().any(|l| l.contains("case-order-done")), "{lines:?}");
}
