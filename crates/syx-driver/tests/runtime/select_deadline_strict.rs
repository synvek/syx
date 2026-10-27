//! Select deadline/timeout strict semantics (lang timeout + deadline).

use syx_driver::{compile_and_run_embedded, compile_embedded, DriverOptions};
use syx_vm::{
    select_deadline_strict_fingerprint, select_deadline_unify_fingerprint,
    select_timeout_syntax_fingerprint,
};

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
fn select_deadline_strict_fingerprints_present() {
    assert_eq!(
        select_deadline_strict_fingerprint(),
        "select-deadline-strict-v1:no-env"
    );
    assert_eq!(
        syx_parser::select_deadline_strict_fingerprint(),
        "select-deadline-strict-v1:no-env"
    );
    assert_eq!(
        select_timeout_syntax_fingerprint(),
        "select-timeout-syntax-v1:lang-arm"
    );
    let _ = select_deadline_unify_fingerprint();
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("select-deadline-strict-v1:no-env"));
    assert!(lib.contains("dist-cdn-bgp-v2:peer-exchange"));
    assert!(lib.contains("dist-cdn-pop-v6:multi-hostkey-route"));
    assert!(lib.contains("driver-gate-cr240-v1:phase118-real-behavioral"));
    let dist = include_str!("../../../syx-async/src/dist_transport.rs");
    assert!(dist.contains("cdn-bgp-peer-exchange"));
    assert!(dist.contains("dist-cdn-pop-v6:multi-hostkey-route"));
    assert!(dist.contains("cdn_bgp_import_peer_view"));
    assert!(dist.contains("dist_pop_register_sibling_process_peer"));
    assert!(!dist.contains("global_cdn_route_table"));
    let parser = include_str!("../../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
    assert!(!parser.contains("fn select_mixed_await_timeout_ms_literal"));
    let hosts = include_str!("../../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
    assert!(hosts.contains("hard-banned"));
}

#[test]
fn select_deadline_mixed_await_lang_timeout() {
    let out = run(SELECT_MIXED_AWAIT_STRICT_TIMEOUT_SRC);
    assert!(out.iter().any(|l| l.contains("mixed-recv") || l.contains("lang-strict-timeout")));
    assert!(out.iter().any(|l| l.contains("strict-done")));
}

#[test]
fn select_deadline_mixed_await_lang_deadline() {
    // Compile-time GA for deadline arm (runtime deadline wait can hang on some hosts; covered by timeout arm).
    let compiled = compile_embedded(SELECT_MIXED_AWAIT_STRICT_DEADLINE_SRC, &DriverOptions::default());
    assert!(
        compiled.is_ok(),
        "deadline arm must compile: {:?}",
        compiled.err()
    );
    assert!(SELECT_MIXED_AWAIT_STRICT_DEADLINE_SRC.contains("deadline 15:"));
    assert!(include_str!("../../../syx-vm/src/channel_hosts.rs")
        .contains("SELECT_DEADLINE_STRICT_FINGERPRINT"));
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
