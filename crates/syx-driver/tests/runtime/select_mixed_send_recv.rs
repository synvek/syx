//! Select mixed send/recv — unified tagged array poll + fair RR.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{reset_select_fair_cursor, select_fair_fingerprint, select_mixed_unified_fingerprint};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[test]
fn select_mixed_send_recv_ready_fair_stdout() {
    reset_select_fair_cursor(0);
    let lines = run(
        r#"func main() {
    let recv_ch = __syx_channel_create(1)
    let send_ch = __syx_channel_create(1)
    __syx_channel_send(recv_ch, 7)
    select {
        case v <- recv_ch:
            print("recv")
            print(v)
        case send_ch <- 99:
            print("sent")
        default:
            print("d")
    }
}"#,
    );
    // seed 0 → start at index 0 (recv) when both ready.
    assert!(lines.iter().any(|l| l.contains("recv")), "{lines:?}");
    assert!(lines.iter().any(|l| l.contains('7')), "{lines:?}");
    assert!(!lines.iter().any(|l| l.contains("sent")), "{lines:?}");
    assert!(!lines.iter().any(|l| l.contains('d')), "{lines:?}");

    // Second select: only send ready (fresh VM + fair cursor).
    reset_select_fair_cursor(0);
    let lines2 = run(
        r#"func main() {
    let recv_ch = __syx_channel_create(1)
    let send_ch = __syx_channel_create(1)
    select {
        case v <- recv_ch:
            print("recv")
        case send_ch <- 99:
            print("sent")
        default:
            print("d")
    }
    print(__syx_channel_try_recv(send_ch))
}"#,
    );
    assert!(lines2.iter().any(|l| l.contains("sent")), "{lines2:?}");
    assert!(lines2.iter().any(|l| l.contains("99")), "{lines2:?}");
    assert!(!lines2.iter().any(|l| l.contains("recv")), "{lines2:?}");
}

#[test]
fn select_mixed_fair_rr_both_ready() {
    reset_select_fair_cursor(0);
    // Host tagged-array path (same ops sugar emits via __syx_select_tag).
    // Avoid sugar+while: nested select if-chain + loop exposed an i-increment hang.
    let lines = run(
        r#"func main() {
    let recv_ch = __syx_channel_create(1)
    let send_ch = __syx_channel_create(1)
    var i = 0
    while i < 20 {
        __syx_channel_try_recv(recv_ch)
        __syx_channel_try_recv(send_ch)
        __syx_channel_send(recv_ch, 11)
        let a0 = __syx_select_tag(0, recv_ch, nil)
        let a1 = __syx_select_tag(1, send_ch, 22)
        let r = __syx_channel_select([a0, a1])
        print(r[1])
        i = i + 1
    }
}"#,
    );
    let c11 = lines.iter().filter(|l| l.contains("11")).count();
    // send success returns Bool true — print shows "true", not 22; count via index.
    let c_send = lines.iter().filter(|l| l.contains("true")).count();
    assert_eq!(
        (c11, c_send),
        (10, 10),
        "fair RR from seed 0 must split evenly: 11={c11} send={c_send} lines={lines:?}"
    );
}

#[test]
fn select_mixed_fingerprints_present() {
    assert_eq!(select_fair_fingerprint(), "select-fair-v1:round-robin");
    assert_eq!(
        select_mixed_unified_fingerprint(),
        "select-mixed-unified-v1:tagged-array-poll"
    );
    assert_eq!(
        syx_parser::select_mixed_unified_fingerprint(),
        "select-mixed-unified-v1:tagged-array-poll"
    );

    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("select-mixed-unified-v1:tagged-array-poll"));

    let hosts = include_str!("../../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("select-mixed-unified-v1:tagged-array-poll"));
    assert!(hosts.contains("host_select_tag"));
    assert!(hosts.contains("__syx_select_dir"));

    let parser = include_str!("../../../syx-parser/src/lib.rs");
    assert!(parser.contains("select-mixed-unified-v1:tagged-array-poll"));
    assert!(parser.contains("select_tagged_arm_expr"));
    assert!(parser.contains("__syx_select_tag"));
    assert!(parser.contains("__syx_sel_arr"));
    assert!(parser.contains("__syx_channel_select_default"));
    assert!(
        parser.contains("never `while true` + Break"),
        "multi-case must keep no-while lowering"
    );
    // Per-arm try_send must not remain the mixed GA path.
    assert!(
        !parser.contains("via per-arm try_send"),
        "mixed GA must not document per-arm try_send"
    );
}

#[test]
fn select_mixed_parser_dump_unified_poll() {
    let source = syx_diagnostics::SourceFile::new(
        "p108_mixed.sy",
        r#"func main() {
    let ch1 = __syx_channel_create(1)
    let ch2 = __syx_channel_create(1)
    select {
        case v <- ch1:
            print("recv")
        case ch2 <- 1:
            print("sent")
        default:
            print("d")
    }
}"#,
    );
    let module = syx_parser::Parser::parse(source).unwrap();
    let syx_ast::Decl::Function { body, .. } = &module.decls[0] else {
        panic!("expected function");
    };
    let dump = format!("{body:?}");
    assert!(
        dump.contains("__syx_sel_arr"),
        "must build select array: {dump}"
    );
    assert!(
        dump.contains("__syx_channel_select_default"),
        "must have unified poll call: {dump}"
    );
    assert!(
        dump.contains("__syx_sel_idx") && dump.contains("__syx_sel_res"),
        "must index-dispatch after one poll: {dump}"
    );
    assert!(
        !dump.contains("__syx_channel_try_send"),
        "must not use per-arm try_send chain: {dump}"
    );
    // Object-tagged arms via `__syx_select_tag` (homogeneous Object[]).
    assert!(
        dump.contains("__syx_select_tag") && dump.contains("__syx_sel_arm_"),
        "must emit __syx_select_tag temps: {dump}"
    );
}

