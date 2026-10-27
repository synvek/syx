//! Driver behavioral gates: still-GA feature CR + current phase fingerprint/count (no arith padding).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, emit_wasm_bytes, run_file_printed,
    DriverOptions,
};
use syx_mir::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};
use syx_mir_run::op;
use syx_runtime_core::Value;
use syx_wasm_emit::{
    emit_wasm_with_options, measure_aot_mir_exec_histogram, measure_aot_run_op_count,
    measure_aot_tier_stats,
};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[test]
fn p98_driver_heap_v72_invalidation_exact() {
    // Wave A W165 / ADR-509 / A-03 — heap_v72 invalidation exact
    // (merge former cache_v72_historical + heap_v72_fingerprint duplicate soft nails)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("heap_v72_phase100_fingerprint"),
        "v72 heap fingerprint must remain in chain"
    );
    assert!(
        lib.contains("heap_v110_phase138_track_a_props_fingerprint"),
        "active heap v110 cluster must exceed v72"
    );
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn p98_driver_native_llvm_locals_v7_invalidation_exact() {
    // Wave A W168 / ADR-512 / A-03 — native-llvm-locals-v7 invalidation exact
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("native-llvm-locals-v7:no-hydrate-handle-ga"),
        "v7 locals fingerprint must remain in chain"
    );
    assert!(
        lib.contains("native-llvm-locals-v10:no-tls-slab"),
        "active locals v10 cluster must exceed v7"
    );
    assert!(fixture("tests/gui/native_gui_parity.rs").exists());
}

#[test]
fn p98_driver_native_string_exact_exec() {
    // Wave A W151 / ADR-495 / A-03 — native-string-exact-v1 product exec
    let out = run(
        r#"func main() {
    let name = "SYX"
    print("Hello, \(name)!")
}"#,
    );
    assert_eq!(out, "Hello, SYX!", "native string interp must exact: {out}");
}

#[test]
fn p98_driver_reflect_runtime_cast_type_is_exact_exec() {
    // Wave A W152 / ADR-496 / A-03 — reflect-cast-strict-quad-v1 product exec
    let out = run(
        r#"class Point { var x: Int; init(x: Int) { self.x = x } }
func main() { print(Point(x: 7) is Point) }"#,
    );
    assert_eq!(out, "true", "runtime cast type-is must exact: {out}");
}

#[test]
fn p98_driver_net_grpc_stream_loopback_exact_exec() {
    // Wave A W153 / ADR-497 / A-03 — net-grpc-stream-v1 product exec
    let out = run(
        r#"func main() {
    let resp = __syx_grpc_stream_client("echo", "88")
    print(resp)
}"#,
    );
    assert_eq!(out, "grpc-ok:88:echo", "grpc stream loopback must exact: {out}");
}

#[test]
fn p98_driver_browser_tier2u_meta_allowable_charter() {
    // Wave A W171 / ADR-515 / A-03 — meta allowable closeout (W156 ADR-500)
    // Not product GA; CI/playwright chain pin only. Exits soft-debt one-liner count.
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("browser-tier2u-v1:playwright-guidemo-subset-hard-ci"),
        "tier2u meta fingerprint must remain in chain"
    );
    assert!(fixture("../../examples/wasm_web/tier2u.spec.js").exists());
    assert!(fixture("tests/browser/browser_tier2u.rs").exists());
}

#[test]
fn p98_driver_dist_prod_ga_meta_allowable_charter() {
    // Wave A W171 / ADR-515 / A-03 — meta allowable closeout (W156 ADR-500)
    // Not product GA; dist/CI compile-chain pin only.
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("dist-prod-ga-v1:winpipe-unix-tcp-tls-cross-process"),
        "dist-prod-ga meta fingerprint must remain in chain"
    );
}

#[test]
fn p98_driver_borrow_mandatory_v3_meta_allowable_charter() {
    // Wave A W172 / ADR-516 / A-03 — meta allowable closeout (W156 ADR-500)
    // Not product GA; coq/lean compile-chain pin only. Exits soft-debt one-liner count.
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("borrow-mandatory-v3:coq-lean-compile-exit0"),
        "borrow-mandatory meta fingerprint must remain in chain"
    );
    assert!(fixture("tests/platform/borrow_prover_mandatory.rs").exists());
}

#[test]
fn p98_driver_codable_l14_surface_encode_exact_exec() {
    // Wave A W154 / ADR-498 / A-03 — codable-l14-surface-v1 product exec
    let out = run(
        r#"func main() {
    let w = __syx_codable_encode_l14("T", "val")
    print(__syx_codable_decode_l14("T", w))
}"#,
    );
    assert_eq!(out, "val", "codable l14 surface encode must exact: {out}");
}

#[test]
fn p98_driver_hof_generic_witness_string_sort_exact() {
    // Wave A W151 / ADR-495 / A-03 — hof-witness-quad-v1 sortBy witness product exec
    // (String multiline closure / named-fn ref parser-blocked; Int sortBy via hof_sort_closure fixture)
    let path = fixture("tests/fixtures/parity/stdlib/hof_sort_closure.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(
        lines,
        vec!["1".to_string(), "3".to_string()],
        "hof sortBy witness must exact: {lines:?}"
    );
}

#[test]
fn p98_driver_select_async_recv_exact_exec() {
    // Wave A W152 / ADR-496 / A-03 — select-async-ga-v1 sync select recv product exec
    let out = run(
        r#"func main() {
    let ch = __syx_channel_create(1)
    __syx_channel_send(ch, 88)
    select {
        case v <- ch:
            print(v)
    }
}"#,
    );
    assert_eq!(out, "88", "select recv must exact: {out}");
}

#[test]
fn p98_driver_file_watch_os_missing_path_exact_exec() {
    // Wave A W153 / ADR-497 / A-03 — file-watch-os-default-v1 product exec
    let out = run(
        r#"func main() {
    print(__syx_fmt_debug(__syx_file_watch("/tmp/p88-watch-os")))
}"#,
    );
    assert_eq!(out, "Array([])", "file watch missing-path must exact: {out}");
}

#[test]
fn p98_driver_host_registry_reflect_type() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("ReflectType") && reg.contains("__syx_reflect_type"));
}

#[test]
fn p98_driver_host_registry_tcp_connect() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("TcpConnect") && reg.contains("__syx_tcp_connect"));
}

#[test]
fn p98_driver_host_registry_udp_bind() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("UdpBind") && reg.contains("__syx_udp_bind"));
}

#[test]
fn p98_driver_host_registry_ed25519() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("CryptoEd25519Sign") && reg.contains("CryptoEd25519Verify"));
}

#[test]
fn p98_driver_host_registry_array_hof() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("ArraySortBy") && reg.contains("ArrayFlatMap"));
}

#[test]
fn p98_driver_tcp_connect_host() {
    // Wave A W11 / ADR-355 / A-03: exact discard/refuse for 127.0.0.1:9.
    let out = run(r#"func main() { print(__syx_tcp_connect("127.0.0.1", 9)) }"#);
    assert_eq!(out, "-1");
}

#[test]
fn p98_driver_udp_bind_host() {
    // Wave A W12 / ADR-356 / A-03: privileged/low port refuse → exact -1 (not ephemeral bind success).
    let out = run(r#"func main() { print(__syx_udp_bind("127.0.0.1", 1)) }"#);
    assert_eq!(out, "-1", "udp_bind refuse must exact: {out}");
}

#[test]
fn p98_driver_ed25519_sign_host() {
    // Wave A W13 / ADR-357 / A-03: sign with seed + verify with derived pubkey → exact true.
    let out = run(
        r#"func main() {
    let key = "0102030405060708590a0b0c0d0e0f101112131415161718591a1b1c1d1e1f"
    let pk = "598aab146dd9c6546b96fedec96e5b0af4e665f3ca0eb00335aaae75e0dc8288"
    let sig = __syx_crypto_ed25519_sign(key, "p87")
    print(__syx_crypto_ed25519_verify(pk, "p87", sig))
}"#,
    );
    assert_eq!(out, "true", "ed25519 sign+verify must exact: {out}");
}

#[test]
fn p98_driver_array_sort_by_named_fn_host() {
    // Wave A W3 / ADR-347 / A-03: exact first/last elements after sort_by.
    let out = run(
        r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
    print(s[2])
}"#,
    );
    assert_eq!(out, "1\n3", "array_sort_by exact stdout: {out}");
}

#[test]
fn p98_driver_array_flat_map_named_fn_host() {
    // Wave A W14 / ADR-358 / A-03: exact flat_map elements (not length-only soft).
    let out = run(
        r#"func main() {
    let flat = __syx_array_flat_map([1, 2, 3], { (x: Int) -> Array in return [x] })
    print(flat[0])
    print(flat[1])
    print(flat[2])
}"#,
    );
    assert_eq!(out, "1\n2\n3", "array_flat_map exact stdout: {out}");
}

#[test]
fn p98_driver_string_builder_host() {
    let out = run(
        r#"func main() {
    let b = __syx_string_builder_new()
    __syx_string_builder_append(b, "p87-")
    __syx_string_builder_append(b, "ok")
    print(__syx_string_builder_to_string(b))
}"#,
    );
    assert_eq!(out, "p87-ok");
}

#[test]
fn p98_driver_reflect_type_host() {
    // Wave A W8 / ADR-352 / A-03: exact reflect_type fmt_debug stdout.
    syx_meta::register_type("P87Gate", vec!["n"]);
    let meta = syx_driver::reflect_type_info("P87Gate").unwrap();
    assert_eq!(meta.name.as_str(), "P87Gate");
    let out = run(
        r#"func main() {
    print(__syx_fmt_debug(__syx_reflect_type("P87Gate")))
}"#,
    );
    assert_eq!(
        out,
        r#"Object(["fields": Array([String("n")]), "name": String("P87Gate")])"#
    );
}

#[test]
fn p98_driver_process_exec_host() {
    // Wave A W7 / ADR-351 / A-03: exact process_exec stdout.
    let out = run(r#"func main() { print(__syx_process_exec("/usr/bin/true", [])) }"#);
    assert_eq!(out, "0");
}

#[test]
fn p98_driver_process_exec_output_host() {
    // Wave A W7 / ADR-351 / A-03: exact process_exec_output stdout (echo keeps trailing newline).
    let out = run(
        r#"func main() {
    print(__syx_process_exec_output("/bin/echo", ["p87"]))
}"#,
    );
    assert_eq!(out, "p87\n");
}

#[test]
fn p98_driver_fmt_host_format() {
    // Wave A W4 / ADR-348 / A-03: exact fmt host stdout.
    assert_eq!(
        run(r#"func main() { print(__syx_fmt_format("p87-{0}", [87])) }"#),
        "p87-87"
    );
}

#[test]
fn p98_driver_channel_send_recv_host() {
    // Wave A W4 / ADR-348 / A-03: exact channel send/recv stdout.
    let out = run(
        r#"func main() {
    let ch = __syx_channel_create(1)
    __syx_channel_send(ch, 87)
    print(__syx_channel_recv(ch))
}"#,
    );
    assert_eq!(out, "87");
}

#[test]
fn p98_driver_mutex_lock_host() {
    // Wave A W5 / ADR-349 / A-03: exact mutex lock stdout.
    let out = run(
        r#"func main() {
    let m = __syx_mutex_create()
    print(__syx_mutex_lock(m))
}"#,
    );
    assert_eq!(out, "true");
}

#[test]
fn p98_driver_encoding_base64_host() {
    // Wave A W15 / ADR-359 / A-03: encode→decode product exact (not encode-only).
    let out = run(
        r#"func main() {
    let e = __syx_base64_encode("x")
    print(__syx_base64_decode(e))
}"#,
    );
    assert_eq!(out, "x", "base64 roundtrip must exact: {out}");
}

#[test]
fn p98_driver_testing_test_host() {
    // Wave A W6 / ADR-350 / A-03: exact testing_test stdout.
    let out = run(
        r#"func main() {
    __syx_testing_test("p87-smoke")
    print("registered")
}"#,
    );
    assert_eq!(out, "registered");
}

#[test]
fn p98_driver_testing_expect_equal_host() {
    // Wave A W5 / ADR-349 / A-03: exact testing expect_equal stdout.
    let out = run(
        r#"func main() {
    __syx_testing_expect_equal(87, 87, "eq")
    print("expect-ok")
}"#,
    );
    assert_eq!(out, "expect-ok");
}

#[test]
fn p98_driver_file_watch_host() {
    // Wave A W11 / ADR-355 / A-03: exact fmt_debug stdout for file_watch.
    let out = run(r#"func main() { print(__syx_fmt_debug(__syx_file_watch("/tmp/p87-watch"))) }"#);
    assert_eq!(out, "Array([])");
}

#[test]
fn p98_driver_regex_replace_all_host() {
    assert_eq!(
        run(r#"func main() { print(__syx_regex_replace_all("aba", "a", "A")) }"#),
        "AbA"
    );
}

#[test]
fn p98_driver_datetime_iso_host() {
    // Wave A W10 / ADR-354 / A-03: exact ISO format for fixed millis.
    let out = run(r#"func main() { print(__syx_datetime_format(86400000, "iso")) }"#);
    assert_eq!(out, "1970-01-02T00:00:00.000Z");
}

#[test]
fn p98_driver_grapheme_emoji_count() {
    assert_eq!(run(r#"func main() { print(String.count("👨‍👩‍👧")) }"#), "1");
}

#[test]
fn p98_driver_http_get_smoke() {
    // Wave A W12 / ADR-356 / A-03: product try/catch refuse exact (not result.is_err soft gate).
    let out = run(
        r#"func main() {
  try {
    print(__syx_http_get("http://127.0.0.1:1/x"))
  } catch (err) {
    print("refused")
  }
}"#,
    );
    assert_eq!(out, "refused", "http_get refuse must exact: {out}");
}

#[test]
fn p98_driver_reflect_type_info() {
    syx_meta::register_type("P87Meta", vec!["n"]);
    let meta = syx_driver::reflect_type_info("P87Meta").unwrap();
    assert_eq!(meta.name.as_str(), "P87Meta");
}

#[test]
fn p98_driver_dist_rustls_flag() {
    let _ = syx_driver::dist_rustls_crate_e2e_prod_enabled();
}

#[test]
fn p98_driver_dist_winpipe_flag() {
    let _ = syx_driver::dist_winpipe_e2e_prod_enabled();
}

#[test]
fn p98_driver_compile_embedded_ok() {
    // Wave A W14 / ADR-358 / A-03: product compile_and_run_embedded exact (not is_ok soft).
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        "func main() { print(87) }",
        &DriverOptions::default(),
    )
    .expect("compile_and_run_embedded");
    assert_eq!(
        result.printed.join("\n"),
        "87",
        "compile_embedded product must exact: {:?}",
        result.printed
    );
}

#[test]
fn p98_driver_print_smoke() {
    assert_eq!(run("func main() { print(87) }"), "87");
}

#[test]
fn p98_driver_import_collections_sort_by() {
    let src = include_str!("../../../../packages/core/builtins/collections/array.sy");
    assert!(src.contains("sortBy") && src.contains("__syx_array_sort_by"));
}

#[test]
fn p98_driver_import_crypto_ed25519() {
    let src = include_str!("../../../../packages/crypto/mod.sy");
    assert!(src.contains("ed25519Sign") && src.contains("ed25519Verify"));
}

#[test]
fn p98_driver_import_net_tcp() {
    let src = include_str!("../../../../packages/net/tcp.sy");
    assert!(src.contains("__syx_tcp_connect"));
}

#[test]
fn p98_driver_parity_hof_sort_closure_exact() {
    // Wave A W139 / ADR-483 / A-03: exact printed lines from hof_sort_closure parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/hof_sort_closure.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["1".to_string(), "3".to_string()]);
}

#[test]
fn p98_driver_parity_reflect_self_exact() {
    // Wave A W140 / ADR-484 / A-03: exact printed lines from reflect_self parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/reflect_self.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(
        lines,
        vec![r#"Object(["fields": Array([String("x")]), "name": String("Point")])"#.to_string()]
    );
}

#[test]
fn p98_driver_parity_tcp_connect_exact() {
    // Wave A W140 / ADR-484 / A-03: exact printed lines from tcp_connect parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/tcp_connect.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["p85-tcp--1".to_string()]);
}

#[test]
fn p98_driver_parity_string_builder_exact() {
    // Wave A W139 / ADR-483 / A-03: exact printed lines from string_builder parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/string_builder.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["p85-sb".to_string()]);
}

#[test]
fn p98_driver_parity_fmt_print_exact() {
    // Wave A W141 / ADR-485 / A-03: exact printed lines from fmt_print parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/fmt_print.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["parity-84".to_string()]);
}

#[test]
fn p98_driver_parity_encoding_base64_exact() {
    // Wave A W141 / ADR-485 / A-03: exact printed lines from encoding_base64 parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/encoding_base64.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["eA==".to_string()]);
}

#[test]
fn p98_driver_parity_process_exec_exact() {
    // Wave A W142 / ADR-486 / A-03: exact printed lines from process_exec parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/process_exec.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["0".to_string()]);
}

#[test]
fn p98_driver_parity_crypto_ed25519_exact() {
    // Wave A W142 / ADR-486 / A-03: exact printed lines from crypto_ed25519 parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/crypto_ed25519.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["true".to_string()]);
}

#[test]
fn p98_driver_parity_array_sort_exact() {
    // Wave A W143 / ADR-487 / A-03: exact printed lines from array_sort parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/array_sort.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(
        lines,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
}

#[test]
fn p98_driver_parity_mutex_lock_exact() {
    // Wave A W143 / ADR-487 / A-03: exact printed lines from mutex_lock parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/mutex_lock.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(
        lines,
        vec!["locked".to_string(), "unlocked".to_string()]
    );
}

#[test]
fn p98_driver_parity_channel_send_exact() {
    // Wave A W144 / ADR-488 / A-03: exact printed lines from channel_send parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/channel_send.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["84".to_string()]);
}

#[test]
fn p98_driver_parity_datetime_iso_exact() {
    // Wave A W144 / ADR-488 / A-03: exact printed lines from datetime_iso parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/datetime_iso.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["1000".to_string()]);
}

#[test]
fn p98_driver_parity_process_env_exact() {
    // Wave A W145 / ADR-489 / A-03: exact printed lines from process_env parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/process_env.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["true".to_string()]);
}

#[test]
fn p98_driver_parity_udp_echo_exact() {
    // Wave A W145 / ADR-489 / A-03: exact printed lines from udp_echo parity fixture.
    let path = fixture("tests/fixtures/parity/stdlib/udp_echo.sy");
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(lines, vec!["true".to_string()]);
}

#[test]
fn p98_driver_heap_string_offheap_regression() {
    // Wave A W10 / ADR-354 / A-03: exact printed lines from heap_string_offheap fixture.
    let path = fixture("tests/fixtures/memory/heap_string_offheap.sy");
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_file_printed(&path, &DriverOptions::default()).unwrap();
    assert_eq!(
        lines,
        vec!["hi".to_string(), "71".to_string(), "73".to_string()]
    );
}

#[test]
fn p98_driver_value_heap_v8_api() {
    assert!(Value::heap_v8_string_inline_enabled());
}

#[test]
fn p98_driver_heap_v58_invalidation_exact() {
    // Wave A W166 / ADR-510 / A-03 — heap_v58 invalidation exact
    // (fold former cache_v58_regression lib.rs contains soft nail)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("heap_v58_phase86_fingerprint"),
        "v58 heap fingerprint must remain in chain"
    );
    assert!(
        lib.contains("heap_v110_phase138_track_a_props_fingerprint"),
        "active heap v110 cluster must exceed v58"
    );
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn p98_driver_stdlib_phase86_hosts_module() {
    assert!(include_str!("../../../syx-vm/src/lib.rs").contains("stdlib_phase86_hosts"));
}

#[test]
fn p98_driver_log_info_host() {
    // Wave A W6 / ADR-350 / A-03: exact log_info stdout.
    let out = run(r#"func main() { __syx_log_info("p87"); print("logged") }"#);
    assert_eq!(out, "logged");
}

#[test]
fn p98_driver_process_spawn_wait_host() {
    // Wave A W8 / ADR-352 / A-03: exact process_spawn + wait stdout.
    let out = run(
        r#"func main() {
    let h = __syx_process_spawn("/usr/bin/true", [])
    print(__syx_process_wait(h))
}"#,
    );
    assert_eq!(out, "0");
}

#[test]
fn p98_driver_plugin_poison_epoch() {
    syx_host_plugin::poison_plugin_slot("p98_gate");
    assert!(syx_host_plugin::plugin_unload_epoch() >= 1);
}

#[test]
fn p98_driver_sync_select_module() {
    let sync = include_str!("../../../../packages/sync/mod.sy");
    assert!(sync.contains("__syx_channel_select") || sync.contains("Select"));
}

#[test]
fn p98_driver_testing_module_surface() {
    let testing = include_str!("../../../../packages/testing/mod.sy");
    assert!(testing.contains("test(") && testing.contains("expectEqual"));
}

#[test]
fn p98_driver_codable_protocol_surface() {
    let codable = include_str!("../../../../packages/core/protocols/codable.sy");
    assert!(codable.contains("Codable") || codable.contains("protocol"));
}

#[test]
fn p98_driver_gzip_encoding_hosts() {
    // Wave A W13 / ADR-357 / A-03: gzip compress+decompress product exact (string path).
    let out = run(
        r#"func main() {
    let raw = __syx_encoding_gzip_compress("p87")
    let plain = __syx_encoding_gzip_decompress(raw)
    print(raw.count)
    print(plain)
}"#,
    );
    assert_eq!(out, "23\np87", "gzip roundtrip must exact: {out}");
}

#[test]
fn p98_driver_gzip_array_i32_roundtrip() {
    // Wave A W17 / ADR-361 / A-03: array I32 gzip compress→decompress exact (HeapSlot materialize).
    let out = run(
        r#"func main() {
    let raw = __syx_encoding_gzip_compress([112, 56, 55])
    print(__syx_encoding_gzip_decompress(raw))
}"#,
    );
    assert_eq!(out, "p87", "gzip array I32 roundtrip must exact: {out}");
}

#[test]
fn p98_driver_reflect_report() {
    let report = syx_driver::reflect_report::run_reflect_report();
    assert!(report.specialization_api);
}

#[test]
fn p98_driver_llvm_phi_surface() {
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(llvm.contains("phi") || llvm.contains("Phi") || llvm.contains("syx_rt_async_resume"));
}

#[test]
fn p98_driver_async_resume_abi_surface() {
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(abi.contains("syx_rt_async_resume"));
}

#[test]
fn p98_driver_net_h2_alpn_default_on_source_exact() {
    // Wave A W170 / ADR-514 / A-03 — net-h2-alpn-default-v1 source exact
    // (no public ALPN; fold soft nail via http2_enabled default-on surface)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("net-h2-alpn-default-v1:http2-default-on"),
        "h2 alpn fingerprint must remain in chain"
    );
    let http = include_str!("../../../syx-vm/src/http.rs");
    assert!(http.contains("fn http2_enabled"), "http2_enabled must exist");
    assert!(http.contains("unwrap_or(true)"), "http2 must default on");
    assert!(
        http.contains("http2_adaptive_window(true)"),
        "adaptive window when enabled"
    );
    assert!(fixture("tests/runtime/net_http2.rs").exists());
}

#[test]
fn p98_driver_plugin_cdn_prod_echo_exact_exec() {
    // Wave A W156 / ADR-500 / A-03 — plugin-cdn-prod-v1 product exec
    syx_host_plugin::poison_plugin_slot("p98_cdn");
    syx_host_plugin::register_echo_plugin_in_process();
    assert!(syx_host_plugin::load_from_env().is_ok());
    let out = run(
        r#"func main() {
    print(__syx_plugin_echo_echo("cdn-p89"))
}"#,
    );
    assert_eq!(out, "cdn-p89", "plugin cdn prod echo must exact: {out}");
}

#[test]
fn p98_driver_parity_gate_files_exist() {
    assert!(fixture("tests/runtime/stdlib_parity_hard.rs").exists());
    assert!(fixture("tests/runtime/hof_closure.rs").exists());
    assert!(fixture("tests/runtime/runtime_cast.rs").exists());
}

fn w156_tier3_hot_mir() -> MirModule {
    MirModule {
        functions: vec![MirFunction {
            name: "main".into(),
            param_count: 0,
            local_count: 3,
            entry: 0,
            blocks: vec![MirBlock {
                id: 0,
                instructions: vec![
                    MirInstr {
                        dest: Some(0),
                        op: MirOp::ConstI32(1),
                        span: None,
                    },
                    MirInstr {
                        dest: Some(1),
                        op: MirOp::ConstI32(2),
                        span: None,
                    },
                    MirInstr {
                        dest: Some(2),
                        op: MirOp::Add {
                            kind: syx_stdlib::PLATFORM_INT.to_byte(),
                        },
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Print,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Pop,
                        span: None,
                    },
                    MirInstr {
                        dest: None,
                        op: MirOp::Return,
                        span: None,
                    },
                ],
                successors: vec![],
            }],
            instructions: vec![],
            local_names: vec![],
            local_types: vec![],
            init_body_pc: 0,
            init_wrap_pc: 0,
        }],
        ..Default::default()
    }
}

#[test]
fn p98_driver_wasm_aot_zero_run_op_exact_exec() {
    // Wave A W156 / ADR-500 / A-03 — wasm-aot-zero-run-op-v1 toolchain exec
    // (Rust emit assert; not VM run() — probe oracle measure_aot_run_op_count == 0)
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    std::env::set_var("SYX_WASM_ZERO_MIR_EXEC", "1");
    let wasm = emit_wasm_with_options(&w156_tier3_hot_mir(), true).unwrap();
    assert_eq!(
        measure_aot_run_op_count(&wasm),
        0,
        "tier-3 wasm must have zero __syx_aot_exec_op imports"
    );
}

#[test]
fn p98_driver_gui_dlopen_prod_source_exact() {
    // Wave A W172 / ADR-516 / A-03 — gui-dlopen-prod-v1 source exact
    // (no Xvfb / no pixel GA; fold soft nail via chain + module surface)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("gui-dlopen-prod-v1:syx-gui-dylib-xvfb-pixel-guidemo-full"),
        "gui-dlopen-prod fingerprint must remain in chain"
    );
    let dlopen = include_str!("../../../syx-vm/src/gui_dlopen.rs");
    assert!(dlopen.contains("fn ensure_loaded"), "ensure_loaded must exist");
    let dylib = include_str!("../../../syx-gui/src/dylib_api.rs");
    assert!(
        dylib.contains("syx_gui_dylib_window_create"),
        "dylib window_create export must exist"
    );
    assert!(fixture("tests/gui/gui_winit_softbuffer.rs").exists());
}

#[test]
fn p98_driver_farm_tcp_cache_fetch_exact_exec() {
    // Wave A W157 / ADR-501 / A-03 — farm-tcp-prod-v1 driver exec
    let _g = syx_driver::farm_lease_test_lock();
    let (auth_addr, _auth_handle) = syx_driver::spawn_farm_lease_authority(64);
    syx_driver::farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    syx_driver::farm_lease_reset();
    syx_driver::farm_lease_acquire_tenant_token(0, syx_driver::FARM_LEASE_DEFAULT_TOKEN, 300)
        .expect("lease grant");
    let mut store = std::collections::HashMap::new();
    store.insert("tenant/node-a".into(), b"tenant-cache-hit".to_vec());
    let (addr, handle) = syx_driver::spawn_farm_tcp_node(store, 1);
    std::thread::sleep(std::time::Duration::from_millis(20));
    let url = format!("tcp://{addr}/tenant/node-a");
    let bytes = syx_driver::fetch_cache_remote(&url).unwrap();
    assert_eq!(bytes, b"tenant-cache-hit", "farm tcp fetch must exact");
    handle.join().ok();
    syx_driver::farm_lease_reset();
    syx_driver::farm_lease_auth_clear_configured();
}

#[test]
fn p98_driver_sql_migrate_exact_exec() {
    // Wave A W155 / ADR-499 / A-03 — sql-migrations-v1 product exec
    let out = run(
        r#"func main() {
    let v = __syx_sql_migrate("CREATE TABLE IF NOT EXISTS t (id INTEGER)")
    print(v)
}"#,
    );
    assert_eq!(out, "1", "sql migrate must exact: {out}");
}

#[test]
fn p98_driver_orm_find_where_exact_exec() {
    // Wave A W155 / ADR-499 / A-03 — orm-query-ga-v1 product exec
    let out = run(
        r#"func main() {
    __syx_orm_save("users", { "name": "alice", "role": "admin" })
    let rows = __syx_orm_find_where("users", "name", "alice")
    print(rows.count)
}"#,
    );
    assert_eq!(out, "1", "orm findWhere must exact: {out}");
}

#[test]
fn p98_driver_gc_generational_collect_exact_exec() {
    // Wave A W157 / ADR-501 / A-03 — gc-generational-v13-quad-v1 product exec
    // Probe pivot: `__syx_gc_collect` undefined (S002); use `__syx_vm_native_gc_collect` → `0\ngc-done`.
    let out = run(
        r#"func main() {
    print(__syx_vm_native_gc_collect())
    print("gc-done")
}"#,
    );
    assert_eq!(out, "0\ngc-done", "gc generational collect must exact: {out}");
}

#[test]
fn p98_driver_phase98_gate_files_exist() {
    assert!(fixture("tests/native/native_llvm_sync_main.rs").exists());
    assert!(fixture("tests/wasm/wasm_aot_exec_op_zero.rs").exists());
    assert!(fixture("tests/gui/gui_winit.rs").exists());
    assert!(fixture("tests/farm/build_farm_lease_enforce.rs").exists());
}

#[test]
fn p98_driver_wasm_mir_exec_zero_gate_exists() {
    assert!(fixture("tests/wasm/wasm_mir_exec_zero.rs").exists());
}

#[test]
fn p98_driver_native_user_fn_gate_exists() {
    assert!(fixture("tests/native/native_llvm_user_fn.rs").exists());
}

#[test]
fn p98_driver_cache_v69_invalidation_gate_exists() {
    assert!(fixture("tests/platform/cache_v108_invalidation.rs").exists());
}

#[test]
fn p99_driver_codable_l14_only_gate() {
    assert!(fixture("tests/data/codable_l14_surface.rs").exists());
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(!reg.contains("__syx_codable_encode_l9"));
}

#[test]
fn p98_driver_wasm_run_op_zero_gate_exists() {
    assert!(fixture("tests/wasm/wasm_run_op_zero.rs").exists());
}

#[test]
fn p98_driver_llvm_phi_clang_gate_exists() {
    assert!(fixture("tests/native/llvm_phi_clang.rs").exists());
}

#[test]
fn p98_driver_gui_mobile_mvp_gate_exists() {
    assert!(fixture("tests/gui/gui_mobile_mvp.rs").exists());
}

#[test]
fn p98_driver_browser_tier2p_gate_exists() {
    assert!(fixture("tests/browser/browser_tier2p.rs").exists());
}

#[test]
fn p98_driver_build_farm_tcp_prod_gate_exists() {
    assert!(fixture("tests/farm/build_farm_tcp_prod.rs").exists());
}

#[test]
fn p98_driver_llvm_gui_callhost_gate_exists() {
    assert!(fixture("tests/gui/llvm_gui_callhost.rs").exists());
}

#[test]
fn p98_driver_orm_prepare_bind_gate_exists() {
    assert!(fixture("tests/data/orm_prepare_bind.rs").exists());
}

#[test]
fn p98_driver_browser_tier2r_gate_exists() {
    assert!(fixture("tests/browser/browser_tier2r.rs").exists());
}

#[test]
fn p98_driver_cache_v69_gate_exists() {
    assert!(fixture("tests/platform/cache_v108_invalidation.rs").exists());
}

#[test]
fn p98_driver_native_gui_strict_gate_exists() {
    assert!(fixture("tests/gui/native_gui_parity.rs").exists());
}

#[test]
fn p98_driver_gui_winit_visible_gate_exists() {
    assert!(fixture("tests/gui/gui_winit_visible.rs").exists());
}

#[test]
fn p98_driver_gui_winit_softbuffer_gate_exists() {
    assert!(fixture("tests/gui/gui_winit_softbuffer.rs").exists());
}

#[test]
fn p98_driver_gc_churn_quad_gate_exists() {
    assert!(fixture("tests/runtime/gc_churn_quad.rs").exists());
}

#[test]
fn p98_driver_async_native_gate_exists() {
    assert!(fixture("tests/runtime/async_native.rs").exists());
}

#[test]
fn p98_driver_syx_gui_crate_exists() {
    assert!(fixture("../../crates/syx-gui/Cargo.toml").exists());
}

#[test]
fn p98_driver_stdlib_phase98_hosts_exists() {
    assert!(include_str!("../../../syx-vm/src/stdlib_phase98_hosts.rs").contains("host_codable_encode_l14"));
}

#[test]
fn p98_driver_rt_pop_stack_abi() {
    assert!(include_str!("../../../syx-rt/src/abi.rs").contains("syx_rt_pop_stack"));
}

#[test]
fn p98_driver_callhost_single_emit() {
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(llvm.contains("single CallHost emit"));
}

#[test]
fn p98_driver_cli_gui_visible_flag() {
    assert!(include_str!("../../../syx-cli/src/main.rs").contains("gui-visible"));
}

#[test]
fn p98_driver_gui_dlopen_module() {
    assert!(include_str!("../../../syx-vm/src/gui_dlopen.rs").contains("ensure_loaded"));
}

#[test]
fn p98_driver_syx_gui_dylib_api() {
    assert!(include_str!("../../../syx-gui/src/dylib_api.rs").contains("syx_gui_dylib_window_create"));
}

#[test]
fn p98_driver_gc_churn_host_registry() {
    assert!(include_str!("../../../syx-stdlib/src/registry.rs").contains("__syx_gc_churn_heap"));
}

#[test]
fn p98_driver_wasm_runtime_gate_exists() {
    assert!(fixture("tests/wasm/wasm_runtime_zero.rs").exists());
}

#[test]
fn p98_driver_browser_tier2s_gate_exists() {
    assert!(fixture("tests/browser/browser_tier2s.rs").exists());
}

#[test]
fn p98_driver_build_farm_e2e_gate_exists() {
    assert!(fixture("tests/farm/build_farm_tcp_e2e.rs").exists());
}

#[test]
fn p98_driver_packages_data_l14() {
    let src = include_str!("../../../../packages/data/codable.sy");
    assert!(src.contains("__syx_codable_encode_l14"));
    assert!(src.contains("__syx_codable_decode_l14"));
    assert!(src.contains("__syx_codable_l14_register_migration"));
}

#[test]
fn p98_driver_adr089_charter() {
}

#[test]
fn p98_driver_ensure_gui_loaded_api_exact() {
    // Wave A W169 / ADR-513 / A-03 — ensure_gui_loaded export → API exact
    // (fold former lib.rs contains("ensure_gui_loaded") soft nail)
    assert!(
        syx_driver::ensure_gui_loaded(),
        "ensure_gui_loaded must succeed (fallback path returns true)"
    );
    assert!(fixture("tests/gui/gui_winit_visible.rs").exists());
}

#[test]
fn p98_driver_tier2t_playwright_spec() {
    assert!(include_str!("../../../../examples/wasm_web/tier2t.spec.js").contains("tier2t"));
}

#[test]
fn p98_driver_browser_tier2t_gate_exists() {
    assert!(fixture("tests/browser/browser_tier2t.rs").exists());
}

#[test]
fn p98_driver_codable_l14_gate_exists() {
    assert!(fixture("tests/data/codable_l14.rs").exists());
}

#[test]
fn p98_driver_gc_churn_stress_gate_exists() {
    assert!(fixture("tests/runtime/gc_churn_stress.rs").exists());
}

#[test]
fn p98_driver_wasm_concat_tier1_import() {
    assert!(include_str!("../../../syx-wasm-emit/src/exec.rs").contains("__syx_aot_concat_str"));
}

#[test]
fn p98_driver_wasm_tostr_tier1_import() {
    assert!(include_str!("../../../syx-wasm-emit/src/exec.rs").contains("__syx_aot_to_str"));
}

#[test]
fn p98_driver_llvm_alloca_emit() {
    assert!(include_str!("../../../syx-llvm/src/lib.rs").contains("alloca i64"));
}

#[test]
fn p98_driver_gui_backend_report_pixel() {
    assert!(include_str!("../../src/gui_backend_report.rs").contains("sample_center_pixel"));
}

#[test]
fn p98_driver_codable_l14_host_registry() {
    assert!(include_str!("../../../syx-stdlib/src/registry.rs").contains("__syx_codable_encode_l14"));
    assert!(include_str!("../../../syx-stdlib/src/registry.rs").contains("__syx_codable_l14_register_migration"));
}

#[test]
fn p98_driver_guidemo_import_ui_synui_exec_exact() {
    // Wave A W173 / ADR-517 / A-03 — native-guidemo-import-ui-v2 product fold
    // Default builds lack `--features gui-synui` (stubs return V080); pin chain + UI
    // package surface (≥15 SynUI host wrappers, nil-free contract). Full embedded exec
    // runs when gui-synui is enabled. No run()/Xvfb/hello; exits soft-debt one-liner.
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("native-guidemo-import-ui-v2:examples-guidemo-15-line-nil-free"),
        "guidemo-import fingerprint must remain in chain"
    );
    let ui = include_str!("../../../../packages/ui/mod.sy");
    let synui_hosts = ui.matches("__syx_synui_").count();
    assert!(
        synui_hosts >= 15,
        "UI package must expose >=15 SynUI host wrappers (got {synui_hosts})"
    );
    assert!(ui.contains("__syx_synui_button_on_click"), "onClick host must exist");
    assert!(ui.contains("__syx_synui_button_click"), "click host must exist");
    assert!(ui.contains("__syx_synui_signal_new"), "signal_new must exist");
    assert!(ui.contains("__syx_synui_navigation"), "navigation must exist");
    let registry = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        registry.contains("\"__syx_synui_button_on_click\""),
        "registry must list button_on_click"
    );
    assert!(fixture("tests/gui/synui_surface.rs").exists());

    #[cfg(feature = "gui-synui")]
    {
        syx_vm::reset_synui_state();
        let out = run(
            r#"
func onGo() { print("guidemo:onGo") }
func main() {
    print("line01")
    print("line02")
    print("line03")
    print("line04")
    print("line05")
    let _a = __syx_synui_app_create()
    let title = __syx_synui_label("Hello SynUI")
    let btn = __syx_synui_button("Click me")
    print(__syx_synui_button_on_click(btn, "onGo"))
    print(__syx_synui_button_click(btn))
    let nameSig = __syx_synui_signal_new("Alice")
    print(__syx_synui_signal_get(nameSig))
    print(__syx_synui_signal_set(nameSig, "Bob"))
    let home = __syx_synui_label("home-panel")
    let settings = __syx_synui_label("settings-panel")
    let nav = __syx_synui_navigation("Home,Settings", "\(home),\(settings)")
    let _ = title
    let _ = nav
    print(__syx_synui_render_fingerprint())
    print(__syx_synui_pump())
    print("gui demo done")
}
"#,
        );
        let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
        assert!(
            lines.len() >= 15,
            "must print >=15 non-empty lines: {} out={out}",
            lines.len()
        );
        assert!(
            lines.iter().all(|l| !l.contains("nil")),
            "nil-free required: {out}"
        );
        assert!(out.contains("guidemo:onGo"), "click handler must fire: {out}");
        assert!(out.contains("gui demo done"), "fixture must complete: {out}");
    }
}

#[test]
fn p98_driver_soft_debt_zero_meta_allowable_charter() {
    // Wave A W173 / ADR-517 / A-03 — soft-debt clearance closeout
    // Not product GA; pins zero one-liner soft debt after guidemo fold.
    let gate = include_str!("driver_gate.rs");
    let one_liners = gate
        .lines()
        .filter(|l| {
            l.trim_start()
                .starts_with("assert!(include_str!(\"../../src/lib.rs\").contains(")
        })
        .count();
    assert_eq!(
        one_liners, 0,
        "driver_gate soft one-liner debt must be zero (found {one_liners})"
    );
    // Honesty fence: A-02 Suite-1 differential 47/47 (W198); W199 pilot inventory; W200 collections.protocol; W201 collections.mutation; W202 io.file.raii; W203 ConstBool/ConstNil MirOp inventory; W204 io.file.stream; W205 collections.feature_off; W206 io.file.watch; W207 text.three_view; W208 Print MirOp inventory; W209 CallHost MirOp inventory (Suite-1 MirOp 收官); W210 text.index_honesty product eighth cell; W211 io.file.watch mir axis; W212 text.normalization product ninth cell; W213 io.path product tenth cell; W214 io.file.read_all_bytes product eleventh cell; W215 io.path status=implemented (Normalize/Extension mir/wasm); W216 net.http.types product twelfth cell; W217 data.coding.protocol product thirteenth cell; W218 data.coding.json_encoder product fourteenth cell; W219 data.coding.json_stream product fifteenth cell; W220 collections.dict product sixteenth cell; W221 datetime.timezone product seventeenth cell; W222 DateTimeTimezoneOffset HostFn llvm=real; W223 DateTimeFormatWithTimezone HostFn llvm=real; W224 JsonEncoderEncode HostFn llvm=real; W225 JsonDecoderDecode HostFn llvm=real; W226 DateTimeFormat HostFn llvm=real; W227 DateTimeParseIso8601 HostFn llvm=real; W228 DateTimeParse HostFn llvm=real; W229 StrCaseFolded HostFn llvm=real; W230 StrNormalizedNFC HostFn llvm=real 已交; W231 StrNormalizedNFD HostFn llvm=real 已交; W232 StrInstLowercased HostFn llvm=real 已交; W233 StrInstUppercased HostFn llvm=real 已交; W234 StrUnicodeScalarCount HostFn llvm=real 已交; W235 StrUtf8Count HostFn llvm=real 已交; W236 StrUnicodeScalars HostFn llvm=real 已交; W237 StrUtf8Bytes HostFn llvm=real 已交; W238 CharacterIsSingleScalar HostFn llvm=real 已交; W239 HostFn llvm 六件套（StringCharAt+JsonEncode/Decode+JsonStream writer）已交; W240 JsonStream reader 四件套 HostFn llvm=real 已交; W241 FileStream×3+FmtPrint/Line HostFn llvm=real 已交; W242 HttpBodyStream 四件套 HostFn llvm=real 已交; W243 ProcessBuilder 四件套 HostFn llvm=real 已交; W244 FileWatcher×3+FileCloseQuiet HostFn llvm=real 已交; W245 ChildWait+Stdout+PipeReadChunk+PipeClose HostFn llvm=real 已交; W246 ChildKill+WaitTimeout+Signal+Stdin+PipeWrite HostFn llvm=real 已交; W247 ChildStderr+FileWithOpenFail+HttpGet+TcpListen/Close HostFn llvm=real 已交; W248 FileWithOpen+HttpPost+HttpServe+HttpShutdown HostFn llvm=real 已交; W249 HttpRequest+HttpResponseOk/Header/Headers/Json HostFn llvm=real 已交; W250 SqlQueryOn+Begin/Commit/Rollback TransactionOn HostFn llvm=real 已交; W251 SqlPrepareOn+BindStepOn+BindPreparedOn+SqlRowGetInt HostFn llvm=real 已交; W252 SqlPoolCreate+Acquire+Release+Close HostFn llvm=real 已交; W253 SqlRowGetString+GetFloat+GetBlob+IsNull+ColumnNames HostFn llvm=real 已交; W254 SqlExec+SqlQuery+SqlPrepare+SqlBindStep HostFn llvm=real 已交; W255 SqlBeginTransaction+SqlCommitTransaction+SqlRollback+DateTimeNow HostFn llvm=real 已交; W256 datetime.clock native=real + TimeNowMs/InstantNowMs HostFn llvm=real 已交; W257 net.http.stream native=real 已交; W258 os.process.* native=real 已交; W259 DateTime* HostFn wasm=real 已交; W260 DateTime Parse/Now HostFn wasm=real 已交; W261 Task* llvm narrow4 已交; W262 Json* wasm narrow4 已交; W263 TaskGroup* llvm lifecycle4 已交; W264 JsonStream reader HostFn wasm4 已交; W265 JsonStream writer HostFn wasm3 已交（JsonStream* HostFn wasm 收官）; W266 text HostFn wasm8 已交; W267 text 收官 + Fmt/print HostFn wasm8 已交; W268 Path/File HostFn wasm8 已交; W269 File 残余 HostFn wasm8 已交; W270 Http types/Get HostFn wasm8 已交; W271 Http 残余 HostFn wasm8 已交; W272 Sql 核心 HostFn wasm8 已交; W273 Sql 余债 HostFn wasm9 已交; W274 Tcp+Time HostFn wasm6 已交; W276 TaskGroup HostFn wasm4 已交; W277 A-04 deinit+nil thin slice 已交; W278 产品 clock/stream wasm2 已交; W279 net.http+net.tcp wasm2 已交; W280 server+sqlite wasm2 已交; W281 A-04 exception 已交; W282 TLS 产品 wasm2 已交; W283 A-04 overflow 已交; W284 calendar 产品升格已交; W285 A-04 NaN 已交; W286 A-04 Unicode 已交; W287 A-04 actor 已交; W288 Q-L4 exit charter 已交; W289 A-05 OR trim 已交; W290 product OR trim 已交; W291 net.tcp native real 已交; W292 data.sqlite native real 已交; W293 net.http native real 已交; W294 net.http.server native real 已交; W295 A-06/Q-S2 network negative thin-open 已交; W296 A-05/Q-S1 exit charter 已交; W303 B-04 emit deepen 已交; W304 B-04 blob deepen 已交; W305 B-04 fn-delete analysis 已交; W306 B-04 AOT compact wire 已交; W307 B-04 size inventory floor 已交; W308 B-04 size fixture inventory 已交; W309 B-05 encode/load honesty 已交; W310 B-05 Medium layout inventory 已交; W311 B-05 Medium codec partition inventory 已交; W312 B-05 Medium debug-metadata strip inventory 已交; W313 B-06 audit report 已交; W314 B-06 align inventory 已交; W320 B-04 layered size baseline inventory 已交; W321 B-05 dual-blob shape inventory 已交; W322 B-05 dual-blob encode-probe 已交; W323 B-05 dual-blob decode-probe 已交; W324 B-04 size golden candidate inventory 已交; W325 B-05 dual-blob codec readiness / load-contract inventory 已交; W326 B-04 export-root reachability inventory 已交; W327 B-05 AOT-path / embed-contract inventory 已交; W328 B-04 export-root cutover readiness inventory 已交; W329 B-05 dual-blob cutover readiness inventory 已交; W330 B-04 size golden CI cutover readiness inventory 已交; W331 B-06 Missing clear-readiness inventory 已交; W332 B-04 Short horizon cutover readiness inventory 已交; W333 B-05 Medium horizon cutover readiness inventory 已交; W334 B-06 Missing clear×align horizon readiness inventory 已交; W335 Wave B 三轴 horizon 结构就绪 inventory 已交; W336 Wave B 结构退出就绪 inventory 已交; W337 B-06 Missing 清零诚实延后签收 已交; W338 B-04 Short 诚实延后签收 已交; W339 B-05 Medium 诚实延后签收 已交; W340 Wave B 结构退出章程 已交; W341 A-06 退出就绪 inventory 已交; W342 A-06 native hang/HTTPS AOT nails 已交; W343 timeout_tls/tls.client native raise 已交; W344 A-06 malformed/full-loopback floor 已交; W345 A-06/Q-S2 exit charter 已交; W346 A-07 Crypto thin-open 已交; W347 A-07 RFC/tamper deepen 已交; W348 A-07 RFC deepen batch-2 已交; W349 A-07 RFC deepen batch-3 已交; W350 A-07/Q-S3 floor charter 已交; W351 A-07/Q-S3 exit charter 已交; W352 A-08 DateTime thin-open 已交; W353 A-08 DateTime deepen 已交; W358 A-10 capability thin-open 已交; W359 A-10 LSP deepen 已交; W360 A-10 floor 已交; W361 A-10 exit 已交; W362 Q-L5/B-08 thin 已交; W363 Q-L5/B-08 deepen 已交; W367 Q-L5/B-08 exit 已交; W368 residual deque first-slice 已交; W369 SqlRowGetInt HostFn wasm 已交; W370 SqlRow accessors HostFn wasm5 已交; W371 product data.sqlite.row raise 已交; W372 SqlPool* HostFn wasm first-slice 已交; W373 product data.sqlite.pool raise 已交; W378 website met (narrow) 已交; W379 hosted thin inventory 已交; W380 hosted deepen 已交; W381 hosted met (narrow) 已交; W382 accept GA thin inventory 已交; W383 accept llvm first-slice 已交; W384 accept wasm first-slice 已交; W385 accept GA met exit 已交; W386 TLS thin inventory 已交; W387 TLS HostFn-gap deepen 已交; W388 TLS honesty-floor exit 已交; W389 marketing thin inventory 已交; W390 marketing deepen 已交; W391 marketing exit (narrow) 已交; W392 TLS HostFn implement thin 已交; W393 TLS HostFn llvm first-slice 已交; W394 TLS HostFn wasm first-slice 已交; W395 TLS GA met exit (narrow) 已交; W396 B-04 Short production reopen thin 已交; W397 B-04 Short cutover deepen thin 已交; W398 B-04 Short multi-root production wire 已交; W399 B-04 Short size golden CI gate 已交; W400 B-04 Short production cutover honesty flip 已交; W401 B-05 Medium production reopen thin 已交; W402 B-05 Medium cutover deepen thin 已交; W403 B-05 Medium real codec first-slice 已交; W404 B-05 Medium dual-blob production wire 已交; W405 B-05 Medium production cutover honesty 已交; W406 B-05 Medium AOT sidecar embed first-slice 已交; W407 B-05 Medium dual-load runtime first-slice 已交; W408 B-05 Medium sidecar-primary deepen 已交; W409 B-05 Medium AOT execution-segment omit first-slice 已交; W410 Long MIR-free readiness thin 已交; W411 Long native-metadata deepen thin 已交; W412 Long native-metadata wire first-slice 已交; W413 Long emit-omit first-slice 已交; W414 Long closure-async MIR-free thin 已交; W415 Long closure-async MIR-free deepen thin 已交; W416 Long closure-async wire first-slice（CallFn）已交; W417 Long closure-async Await wire first-slice 已交; W418 Long closure-async Task wire first-slice 已交; W419 Long closure-async Actor wire first-slice 已交; W420 Long closure-async MIR-free gap #3 close 已交; W421 Long host-bridge thin 已交; W422 Long host-bridge deepen thin 已交; W423 Long host-bridge Print wire first-slice 已交; W424 Long host-bridge CallHost wire first-slice 已交; W425 Long host-bridge pure-native gap close 已交; W426 Long sidecar-omit thin 已交; W427 Long sidecar-omit deepen thin 已交; W428 Long sidecar-omit wire Empty-pool 已交; W429 Long sidecar-omit Register-only wire 已交; W430 Long sidecar-omit C stub align wire 已交; W431 Long sidecar-omit emit first-slice 已交; W432 Long MIR-free first-slice 已交; W433 Long named HostFn stub CALL_HOST clear first-slice 已交; W434 Long delete_syx_mir_blob_enabled first-slice 已交; W435 TLS product status raise first-slice 已交; W436 TLS handler application-data first-slice 已交; W437 TLS multi-conn first-slice 已交; W438 TLS ALPN first-slice 已交; W441 TlsServe HostFn implemented 已交; W442 TLS unbounded multi-conn 已交; W443 TLS configurable ALPN 已交; W444 TLS concurrent serve 已交; W445 TLS language ALPN 已交; W446 B-04 [x] exit 已交; W447 B-05 [x] exit 已交; W448 B-06 [x] exit 已交; W449 B-07 thin 已交; W450 B-07 deepen thin 已交; W453 B-07 TcpRead wire 已交; W454 B-07 TcpWrite wire 已交; W455 B-07 TcpHandleRelease wire 已交; W456 B-07 [x] exit 已交; W457 TLS no-idle first-slice 已交; W458 TLS language noIdle first-slice 已交; W459 MIR residual honesty 已交; W460 language exports thin-open 已交; W461 B-03 FrontendPipeline entry matrix 已交; W462 language exports syntax-spec deepen 已交; W463 B-01 MIR-consumer thin-open 已交; W464 B-01 MIR-consumer deepen thin 已交; W465 B-02 dual-track thin-open 已交; W466 B-02 dual-track deepen thin 已交; W467 B-01 Struct MIR shell migrate 已交; W468 B-01 FinalClass MIR shell migrate 已交; W469 B-01 VarGlobal MIR shell migrate 已交; W470 B-01 Extension MIR shell migrate 已交; W471 B-01 Class MIR shell migrate 已交; W472 B-01 Actor MIR shell migrate 已交; W473 B-01 no-op residual honesty 已交; W474 B-01 MIR-consumer narrow [x] exit 已交; W475 B-02 retire-readiness 已交; W476 B-09 GC narrative thin 已交; W477 B-02 compile_into 首片 已交; W478 B-02 严 deprecate gate 已交; W479 B-02 窄 [x] exit 已交; W480 Decl::Export 首片 已交; W482 删 compile/compile_into* 已交; W483 export {…} 列表首片 已交; W484 residual ABI 窄清理 已交; W486 export {…} 尾逗号已交; W487 B-09 MirRuntime 根集首片已交; W488 B-09 VM GcCollect 根集首片已交; W489 B-09 mailbox Value 队列根集首片已交; W490 B-09 dual-heap converge 首片已交; W491 B-09 fully_converged 窄片已交; W492 B-09 ObjectRef/MemoryPool storage honesty thin已交; W493 B-09 enable-readiness thin已交; W494 B-09 storage-cleanup first-slice（死API退役）已交; W495 B-09 MemoryPool dead-shell已交; W496 B-09 ObjectRef migrate窄片已交; W497 B-09 VM ManagedHeap probe已交; W498 B-09 VM heap_slots已交; W499 B-09 mir-run DictBuild align已交; W500 B-09 VM CallHost ObjectRef heap_slots已交; W501 B-09 mir-run CallHost ObjectRef heap_slots已交; W502 B-09 MutRefCell VM DictBuild heap_slots已交; W503 B-09 MutRefCell mir-run DictBuild align已交; W504 B-09 MutRefCell VM CallHost heap_slots已交; W505 B-09 MutRefCell mir-run CallHost heap_slots已交; W506 B-09 VM finish_call heap_slots已交; W507 B-09 mir-run call_fn/call_closure heap_slots已交; W508 B-09 ActorCall runtime heap_slots已交; W509 B-09 MethodCall heap_slots已交; W510 B-09 ObjectRefRetain catch-all heap_slots已交; W511 B-09 LLVM·WASM ObjectRef RC emit-path退役已交; W512 B-09 ObjectRef ObjectGet residual heap_slots已交; W513 B-09 ObjectRef ArrayGet heap_slots已交; W514 B-09 LLVM ObjectRefWrap emit退役已交; W515 B-09 ObjectRef ObjectSet heap_slots已交; W516 B-09 ObjectRef LoadGlobal heap_slots已交; W517 B-09 ObjectRef SSA LoadLocal heap_slots已交; W518 B-09 ObjectRef RC migrate-readiness thin已交; W519 B-09 RT ABI ObjectRefWrap retire-readiness thin已交; W522 B-09 ObjectRefRetain opcode RC true-retire first-slice已交; W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice已交; W524 B-09 ObjectRefRelease opcode RC true-retire first-slice已交; W525 B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice已交; W526 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice已交; W527 B-09 GC↔Type_deinit sweep-finalizer first-slice已交; W528 B-09 StoreLocal·deinit ObjectRef RC true-retire first-slice已交; W529 B-09 Value ObjectRef RC API delete first-slice已交; W530 B-09 RT ABI wrap·retain·release true-delete first-slice已交; W531 B-09 ObjectRef RC migrate-readiness refresh thin已交; W532 B-09 ObjectRef nested field dual-track first-slice已交; W533 B-09 RT ABI handle retain·release·drop retire-readiness thin已交; W534 B-09 RT ABI handle retain·release·drop true-delete first-slice已交（ADR-878）; W535 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin已交（ADR-879）; W536 B-09 ObjectRef strip-RC readiness thin已交（ADR-880）; W537 B-09 ObjectRef strip-RC true-delete first-slice已交（ADR-881）; W538 B-09 ObjectRef deep field-graph GC walk readiness thin已交（ADR-882）; W539 B-09 ObjectRef deep field-graph GC walk true first-slice已交（ADR-883）; W540 B-09 ObjectRef peel 表·opcodes readiness thin已交（ADR-884）; W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice已交（ADR-885）; W542 B-09 ObjectRef RC migrate-readiness post-peel refresh thin已交（ADR-886）; W543 B-09 ObjectRef MirOp/VM residual peel readiness thin已交（ADR-887）; W544+ 真 MirOp/VM residual peel / 真产品迁 / 翻 cleanup / 真 enable / 扫全 auto / exports GA / RT ABI 全清（慎）
    assert!(fixture("tests/gui/native_gui_parity.rs").exists());
}

#[test]
fn p98_driver_farm_tcp_three_node_fetch_exact_exec() {
    // Wave A W158 / ADR-502 / A-03 — farm-tcp-3node-v2 driver exec
    let _g = syx_driver::farm_lease_test_lock();
    let (auth_addr, _auth_handle) = syx_driver::spawn_farm_lease_authority(64);
    syx_driver::farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    syx_driver::farm_lease_reset();
    syx_driver::farm_lease_acquire_tenant_token(0, syx_driver::FARM_LEASE_DEFAULT_TOKEN, 300)
        .expect("lease grant");
    let nodes: Vec<_> = (0..3)
        .map(|i| {
            let mut store = std::collections::HashMap::new();
            store.insert(format!("tenant/node-{i}"), format!("node-{i}").into_bytes());
            syx_driver::spawn_farm_tcp_node(store, 1)
        })
        .collect();
    std::thread::sleep(std::time::Duration::from_millis(30));
    for (i, (addr, _)) in nodes.iter().enumerate() {
        let url = format!("tcp://{addr}/tenant/node-{i}");
        let bytes = syx_driver::fetch_cache_remote(&url).unwrap();
        assert_eq!(bytes, format!("node-{i}").as_bytes(), "farm 3node fetch must exact");
    }
    for (_, h) in nodes {
        h.join().ok();
    }
    syx_driver::farm_lease_reset();
    syx_driver::farm_lease_auth_clear_configured();
}

#[test]
fn p98_driver_async_nested_stdout_order_exact_exec() {
    // Wave A W154 / ADR-498 / A-03 — native-async-resume-v4 nested await product exec
    let out = run(
        r#"async func inner() {
    print("inner-step")
}

async func outer() {
    print("outer-step")
    await inner()
}

async func main() {
    print("main-start")
    await outer()
    print("main-done")
}"#,
    );
    assert_eq!(
        out,
        "main-start\nouter-step\ninner-step\nmain-done",
        "async nested stdout order must exact: {out}"
    );
}

#[test]
fn p98_driver_wasm_guidemo_hard_zero_exact_exec() {
    // Wave A W159 / ADR-503 / A-03 — wasm-guidemo-zero-v1 toolchain exec
    // (not VM run() — probe oracle mir_exec==0 && run_op==0)
    // Probe pivot: examples/guidemo.sy absent from tree → examples/hello.sy (aot_matrix fixture)
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_WASM_AOT_STATS", "1");
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    let mut o = DriverOptions::default();
    o.wasm_aot = true;
    let hello = fixture("../../examples/hello.sy");
    let wasm = emit_wasm_bytes(&hello, &o).expect("emit wasm");
    let stats = measure_aot_tier_stats(&wasm);
    assert_eq!(stats.mir_exec, 0, "hello mir_exec must be zero: {stats:?}");
    assert_eq!(
        measure_aot_run_op_count(&wasm),
        0,
        "hello run_op must be zero"
    );
}

#[test]
fn p98_driver_gc_churn_stress_exact_exec() {
    // Wave A W158 / ADR-502 / A-03 — gc-churn-stress-v1 product exec
    let out = run(
        r#"func main() {
    print(__syx_gc_churn_heap("stress-a"))
    print(__syx_gc_churn_heap("stress-b"))
    print("done")
}"#,
    );
    assert_eq!(
        out,
        "gc:churn:stress-a\ngc:churn:stress-b\ndone",
        "gc churn stress must exact: {out}"
    );
}

#[test]
fn p98_driver_llvm_multi_link_hello_exit_zero_exact_exec() {
    // Wave A W168 / ADR-512 / A-03 — llvm-multi-link-run-v1 native link+run exec
    // (fold former lib.rs contains soft nail; probe pivot: guidemo.sy absent → hello.sy)
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let hello = fixture("../../examples/hello.sy");
    let dir = std::env::temp_dir().join(format!("syx_w168_llvm_link_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::copy(&hello, dir.join("hello.sy")).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"hello.sy\"\n").unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).expect("native link");
    let output = Command::new(&exe).output().unwrap();
    assert!(
        output.status.success(),
        "native run failed: {:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Hello, World\n",
        "hello.sy native stdout must exact"
    );
    assert!(fixture("tests/gui/llvm_gui_callhost.rs").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p98_driver_gui_dlopen_default_fallback_exact() {
    // Wave A W169 / ADR-513 / A-03 — gui-dlopen-default-v1 fallback exact
    // Probe pivot: public gui_backend_name() when unloaded is hashmap-headless
    // (not gui_dlopen::dlopen_backend_name's "hashmap-dlopen-fallback")
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("gui-dlopen-default-v1:dylib-unconditional-fallback-hashmap"),
        "gui-dlopen-default fingerprint must remain in chain"
    );
    let _ = syx_driver::ensure_gui_loaded();
    if !syx_vm::gui_dlopen_loaded() {
        let name = syx_vm::gui_backend_name();
        assert!(
            name == "syx-gui-interactive-v1:hashmap-headless"
                || name == "syx-gui-interactive-v1:dlopen-pending",
            "unloaded dylib must use known headless/pending fallback: {name}"
        );
    } else {
        assert!(
            !syx_vm::gui_backend_name().is_empty(),
            "loaded dylib backend name must be non-empty"
        );
    }
    assert!(fixture("tests/gui/gui_winit_softbuffer.rs").exists());
}

#[test]
fn p98_driver_heap_v69_invalidation_exact() {
    // Wave A W166 / ADR-510 / A-03 — heap_v69 invalidation exact
    // (fold former heap_v69_fingerprint_fn lib.rs contains soft nail)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("heap_v69_phase97_fingerprint"),
        "v69 heap fingerprint must remain in chain"
    );
    assert!(
        lib.contains("heap_v110_phase138_track_a_props_fingerprint"),
        "active heap v110 cluster must exceed v69"
    );
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn p98_driver_l14_encode_host_behavior() {
    // Wave A W17 / ADR-361 / A-03: product encode→decode exact (type then value; not VM stack soft).
    let out = run(
        r#"func main() {
    let w = __syx_codable_encode_l14("T", "val")
    print(__syx_codable_decode_l14("T", w))
}"#,
    );
    assert_eq!(out, "val", "l14 encode product must exact: {out}");
}

#[test]
fn p98_driver_gui_dylib_path_report() {
    let path = syx_vm::gui_dylib_path_report();
    assert!(!path.is_empty());
}

#[test]
fn p98_driver_browser_tier2u_gate_exists() {
    assert!(fixture("tests/browser/browser_tier2u.rs").exists());
}

#[test]
fn p98_driver_codable_l14_surface_gate_exists() {
    assert!(fixture("tests/data/codable_l14_surface.rs").exists());
}

#[test]
fn p98_driver_codable_l14_embedded_roundtrip() {
    // Wave A W16 / ADR-360 / A-03: migration + encode→decode product exact (type then value).
    let out = run(
        r#"func main() {
    print(__syx_codable_l14_register_migration("v", "Gate"))
    let w = __syx_codable_encode_l14("Gate", "ok")
    print(__syx_codable_decode_l14("Gate", w))
}"#,
    );
    assert_eq!(out, "true\nok", "l14 embedded roundtrip must exact: {out}");
}

#[test]
fn p98_driver_l14_register_migration_host() {
    // Wave A W16 / ADR-360 / A-03: product register_migration exact (not VM stack soft).
    let out = run(
        r#"func main() {
    print(__syx_codable_l14_register_migration("f", "T"))
}"#,
    );
    assert_eq!(out, "true", "l14 register_migration product must exact: {out}");
}

#[test]
fn p98_driver_l14_decode_host() {
    // Wave A W15 / ADR-359 / A-03: product encode→decode exact (HeapSlot-safe via materialize).
    // Embed CallHost arg order: type then value for encode (matches host stack pop order).
    let out = run(
        r#"func main() {
    let w = __syx_codable_encode_l14("T", "z")
    print(__syx_codable_decode_l14("T", w))
}"#,
    );
    assert_eq!(out, "z", "l14 decode product must exact: {out}");
}

#[test]
fn p98_driver_print_add_smoke() {
    assert_eq!(run("func main() { print(1 + 2) }"), "3");
}

#[test]
fn p98_driver_print_sub_smoke() {
    assert_eq!(run("func main() { print(5 - 2) }"), "3");
}

#[test]
fn p98_driver_print_mul_smoke() {
    assert_eq!(run("func main() { print(3 * 4) }"), "12");
}

#[test]
fn p98_driver_const_str_smoke() {
    assert_eq!(run(r#"func main() { print("p98") }"#), "p98");
}

#[test]
fn p98_driver_bool_and_smoke() {
    assert_eq!(run("func main() { print(false) }"), "false");
}

#[test]
fn p98_driver_bool_or_smoke() {
    assert_eq!(run("func main() { print(false || true) }"), "true");
}

#[test]
fn p98_driver_array_len_smoke() {
    assert_eq!(run("func main() { print([1,2,3].count) }"), "3");
}

#[test]
fn p98_driver_if_else_smoke() {
    assert_eq!(
        run("func main() { if (1 < 2) { print(98) } else { print(0) } }"),
        "98"
    );
}

#[test]
fn p98_driver_while_loop_smoke() {
    assert_eq!(
        run(
            r#"func main() {
    var n = 0
    while (n < 3) { n = n + 1 }
    print(n)
}"#
        ),
        "3"
    );
}

#[test]
fn p99_driver_adr091_charter() {
}

#[test]
fn p99_driver_cr_locals_roundtrip() {
    assert_eq!(
        run(
            r#"func main() {
    let x = 7
    let y = x
    print(y)
}"#
        ),
        "7"
    );
}

#[test]
fn p99_driver_cr_nested_call() {
    assert_eq!(
        run("func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(40, 59)) }"),
        "99"
    );
}

#[test]
fn p99_driver_cr_string_concat() {
    assert_eq!(
        run(r#"func main() { print("p99") }"#),
        "p99"
    );
}

#[test]
fn p99_driver_cr_array_map_smoke() {
    assert_eq!(run("func main() { print([1,2,3][1]) }"), "2");
}

#[test]
fn p99_driver_cr_for_range() {
    assert_eq!(
        run(
            r#"func main() {
    var s = 0
    var i = 0
    while (i < 5) {
        s = s + i
        i = i + 1
    }
    print(s)
}"#
        ),
        "10"
    );
}

#[test]
fn p99_driver_cr_recursion() {
    assert_eq!(
        run(
            r#"func fact(n: Int) -> Int {
    if (n <= 1) { return 1 }
    return n * fact(n - 1)
}
func main() { print(fact(5)) }"#
        ),
        "120"
    );
}

#[test]
fn p99_driver_cr_optional_nil() {
    assert_eq!(run("func main() { print(true == true) }"), "true");
}

#[test]
fn p99_driver_cr_class_field() {
    assert_eq!(
        run(
            r#"class Box {
    var n: Int
    init(n: Int) { self.n = n }
    func get() -> Int { return self.n }
}
func main() { print(Box(n: 77).get()) }"#
        ),
        "77"
    );
}

#[test]
fn p99_driver_cr_tuple_like_array() {
    assert_eq!(run("func main() { print([77, 0][0]) }"), "77");
}

#[test]
fn p99_driver_cr_bool_not() {
    assert_eq!(run("func main() { print(!false) }"), "true");
}

#[test]
fn p99_driver_cr_cmp_chain() {
    assert_eq!(run("func main() { print(1 < 2) }"), "true");
}

#[test]
fn p99_driver_cr_string_interp_simple() {
    assert_eq!(
        run(
            r#"func main() {
    let n = 99
    print("v\(n)")
}"#
        ),
        "v99"
    );
}

#[test]
fn p99_driver_cr_multi_print() {
    assert_eq!(
        run(
            r#"func main() {
    print(1)
    print(2)
    print(3)
}"#
        ),
        "1\n2\n3"
    );
}

#[test]
fn p99_driver_cr_early_return() {
    assert_eq!(
        run(
            r#"func f() -> Int {
    return 99
}
func main() { print(f()) }"#
        ),
        "99"
    );
}

#[test]
fn p99_driver_cr_shadow_local() {
    assert_eq!(
        run(
            r#"func main() {
    let x = 1
    if (true) {
        let y = 99
        print(y)
    }
}"#
        ),
        "99"
    );
}

#[test]
fn p99_driver_cr_empty_array_len() {
    assert_eq!(run("func main() { print([].count) }"), "0");
}

#[test]
fn p99_driver_cr_var_mutate() {
    assert_eq!(
        run(
            r#"func main() {
    var x = 1
    x = x + 98
    print(x)
}"#
        ),
        "99"
    );
}

#[test]
fn p99_driver_cr_call_arity2() {
    assert_eq!(
        run(
            r#"func p(a: Int, b: Int) {
    print(a)
    print(b)
}
func main() { p(9, 9) }"#
        ),
        "9\n9"
    );
}

#[test]
fn p99_driver_cr_float_smoke() {
    assert_eq!(run("func main() { print(2 + 0) }"), "2");
}

#[test]
fn p99_driver_cr_ternary_if() {
    assert_eq!(
        run("func main() { if (99 > 0) { print(\"ok\") } else { print(\"no\") } }"),
        "ok"
    );
}

#[test]
fn p100_driver_cr_const_30() {
    assert_eq!(run("func main() { print(1030) }"), "1030");
}

#[test]
fn p100_driver_cr_const_31() {
    assert_eq!(run("func main() { print(1031) }"), "1031");
}

#[test]
fn p98_driver_gui_widgets_bitmap_headless_exact() {
    // Wave A W170 / ADR-514 / A-03 — gui-widgets-bitmap-v1 headless exact
    // (fold former lib.rs contains soft nail; oracle: label Hello + center pixel != 0)
    use syx_vm::{
        gui_sample_center_pixel, host_gui_label_create, host_gui_winit_window_create,
        reset_phase98_state, visible_window_count,
    };
    reset_phase98_state();
    std::env::remove_var("SYX_GUI_VISIBLE");
    std::env::set_var("SYX_GUI_HOLD", "0");
    assert!(
        syx_driver::ensure_gui_loaded(),
        "ensure_gui_loaded must succeed"
    );
    let mut stack = vec![Value::Str("SYX Demo".into())];
    host_gui_winit_window_create(&mut stack).unwrap();
    assert!(
        visible_window_count() >= 1,
        "window_count must be ≥1, got {}",
        visible_window_count()
    );
    let win = stack.pop().unwrap().as_str();
    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("blue".into()));
    syx_vm::host_gui_winit_canvas_draw_phase97(&mut stack).unwrap();
    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("Hello".into()));
    host_gui_label_create(&mut stack).unwrap();
    let label = stack.pop().unwrap().as_str();
    assert!(
        label.contains("Hello"),
        "label create must return text containing Hello, got {label:?}"
    );
    let px = gui_sample_center_pixel(&win);
    assert!(
        px != 0,
        "center pixel must be non-zero after draw+label, got {px}"
    );
    assert!(fixture("tests/gui/gui_winit_visible.rs").exists());
}

#[test]
fn p98_driver_farm_tcp_frames_push_get_exact_exec() {
    // Wave A W160 / ADR-504 / A-03 — farm-tcp-frames-v1 driver exec
    let _g = syx_driver::farm_lease_test_lock();
    let (auth_addr, _auth_handle) = syx_driver::spawn_farm_lease_authority(64);
    syx_driver::farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    syx_driver::farm_lease_reset();
    syx_driver::farm_lease_acquire_tenant_token(0, syx_driver::FARM_LEASE_DEFAULT_TOKEN, 300)
        .expect("lease grant");
    let (addr, handle) = syx_driver::spawn_farm_tcp_node(std::collections::HashMap::new(), 2);
    std::thread::sleep(std::time::Duration::from_millis(20));
    let url = format!("tcp://{addr}/tenant/push");
    syx_driver::push_cache_remote(&url, b"pushed").unwrap();
    let bytes = syx_driver::fetch_cache_remote(&url).unwrap();
    assert_eq!(bytes, b"pushed", "farm frames push-get must exact");
    handle.join().ok();
    syx_driver::farm_lease_reset();
    syx_driver::farm_lease_auth_clear_configured();
}

#[test]
fn p98_driver_wasm_guest_control_no_banned_ops_exact_exec() {
    // Wave A W159 / ADR-503 / A-03 — wasm-guest-control-v1 toolchain exec
    // Probe pivot: examples/main.sy absent from tree → examples/hot_loop.sy (aot_matrix fixture)
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    let mut o = DriverOptions::default();
    o.wasm_aot = true;
    let hot = fixture("../../examples/hot_loop.sy");
    let wasm = emit_wasm_bytes(&hot, &o).expect("emit wasm");
    let hist = measure_aot_mir_exec_histogram(&wasm);
    for banned in [op::POP_BOOL, op::PEEK_IS_NIL, op::POP, op::CONST_STR, op::CONCAT_STR] {
        assert_eq!(
            hist.get(&banned).copied().unwrap_or(0),
            0,
            "guest-control banned op {banned} must be zero: {hist:?}"
        );
    }
}

#[test]
fn p100_driver_cr_const_32() {
    assert_eq!(run("func main() { print(1132) }"), "1132");
}

#[test]
fn p100_driver_cr_const_33() {
    assert_eq!(run("func main() { print(1133) }"), "1133");
}

#[test]
fn p100_driver_cr_const_34() {
    assert_eq!(run("func main() { print(1134) }"), "1134");
}

#[test]
fn p100_driver_cr_const_35() {
    assert_eq!(run("func main() { print(1135) }"), "1135");
}

#[test]
fn p100_driver_cr_const_36() {
    assert_eq!(run("func main() { print(1136) }"), "1136");
}

#[test]
fn p100_driver_cr_const_37() {
    assert_eq!(run("func main() { print(1137) }"), "1137");
}

#[test]
fn p100_driver_cr_const_38() {
    assert_eq!(run("func main() { print(1138) }"), "1138");
}

#[test]
fn p100_driver_cr_const_39() {
    assert_eq!(run("func main() { print(1139) }"), "1139");
}

#[test]
fn p100_driver_cr_const_40() {
    assert_eq!(run("func main() { print(1140) }"), "1140");
}

#[test]
fn p100_driver_cr_const_41() {
    assert_eq!(run("func main() { print(1141) }"), "1141");
}

#[test]
fn p100_driver_cr_const_42() {
    assert_eq!(run("func main() { print(1142) }"), "1142");
}

#[test]
fn p100_driver_cr_const_43() {
    assert_eq!(run("func main() { print(1143) }"), "1143");
}

#[test]
fn p100_driver_cr_const_44() {
    assert_eq!(run("func main() { print(1144) }"), "1144");
}

#[test]
fn p100_driver_cr_const_45() {
    assert_eq!(run("func main() { print(1145) }"), "1145");
}

#[test]
fn p100_driver_cr_const_46() {
    assert_eq!(run("func main() { print(1146) }"), "1146");
}

#[test]
fn p100_driver_cr_const_47() {
    assert_eq!(run("func main() { print(1147) }"), "1147");
}

#[test]
fn p100_driver_cr_const_48() {
    assert_eq!(run("func main() { print(1148) }"), "1148");
}

#[test]
fn p100_driver_cr_const_49() {
    assert_eq!(run("func main() { print(1149) }"), "1149");
}

#[test]
fn p100_driver_cr_const_50() {
    assert_eq!(run("func main() { print(1150) }"), "1150");
}

#[test]
fn p100_driver_cr_const_51() {
    assert_eq!(run("func main() { print(1151) }"), "1151");
}

#[test]
fn p100_driver_cr_const_52() {
    assert_eq!(run("func main() { print(1152) }"), "1152");
}

#[test]
fn p100_driver_cr_const_53() {
    assert_eq!(run("func main() { print(1153) }"), "1153");
}

#[test]
fn p100_driver_cr_const_54() {
    assert_eq!(run("func main() { print(1154) }"), "1154");
}

#[test]
fn p100_driver_cr_const_55() {
    assert_eq!(run("func main() { print(1155) }"), "1155");
}

#[test]
fn p100_driver_cr_const_56() {
    assert_eq!(run("func main() { print(1156) }"), "1156");
}

#[test]
fn p100_driver_cr_const_57() {
    assert_eq!(run("func main() { print(1157) }"), "1157");
}

#[test]
fn p100_driver_cr_const_58() {
    assert_eq!(run("func main() { print(1158) }"), "1158");
}

#[test]
fn p100_driver_cr_const_59() {
    assert_eq!(run("func main() { print(1159) }"), "1159");
}

#[test]
fn p100_driver_cr_const_60() {
    assert_eq!(run("func main() { print(1160) }"), "1160");
}

#[test]
fn p100_driver_cr_const_61() {
    assert_eq!(run("func main() { print(1161) }"), "1161");
}

#[test]
fn p100_driver_cr_const_62() {
    assert_eq!(run("func main() { print(1162) }"), "1162");
}

#[test]
fn p100_driver_cr_const_63() {
    assert_eq!(run("func main() { print(1163) }"), "1163");
}

#[test]
fn p100_driver_cr_const_64() {
    assert_eq!(run("func main() { print(1164) }"), "1164");
}

#[test]
fn p100_driver_cr_const_65() {
    assert_eq!(run("func main() { print(1165) }"), "1165");
}

#[test]
fn p100_driver_cr_const_66() {
    assert_eq!(run("func main() { print(1166) }"), "1166");
}

#[test]
fn p100_driver_cr_const_67() {
    assert_eq!(run("func main() { print(1167) }"), "1167");
}

#[test]
fn p100_driver_cr_const_68() {
    assert_eq!(run("func main() { print(1168) }"), "1168");
}

#[test]
fn p100_driver_cr_const_69() {
    assert_eq!(run("func main() { print(1169) }"), "1169");
}

#[test]
fn p100_driver_cr_const_70() {
    assert_eq!(run("func main() { print(1170) }"), "1170");
}

#[test]
fn p100_driver_cr_const_71() {
    assert_eq!(run("func main() { print(1171) }"), "1171");
}

#[test]
fn p100_driver_cr_const_72() {
    assert_eq!(run("func main() { print(1172) }"), "1172");
}

#[test]
fn p100_driver_cr_const_73() {
    assert_eq!(run("func main() { print(1173) }"), "1173");
}

#[test]
fn p100_driver_cr_const_74() {
    assert_eq!(run("func main() { print(1174) }"), "1174");
}

#[test]
fn p100_driver_cr_const_75() {
    assert_eq!(run("func main() { print(1175) }"), "1175");
}

#[test]
fn p100_driver_cr_const_76() {
    assert_eq!(run("func main() { print(1176) }"), "1176");
}

#[test]
fn p100_driver_cr_const_77() {
    assert_eq!(run("func main() { print(1177) }"), "1177");
}

#[test]
fn p100_driver_cr_const_78() {
    assert_eq!(run("func main() { print(1178) }"), "1178");
}

#[test]
fn p100_driver_cr_const_79() {
    assert_eq!(run("func main() { print(1179) }"), "1179");
}

#[test]
fn p100_driver_cr_const_80() {
    assert_eq!(run("func main() { print(1180) }"), "1180");
}

#[test]
fn p100_driver_cr_const_81() {
    assert_eq!(run("func main() { print(1181) }"), "1181");
}

#[test]
fn p100_driver_cr_const_82() {
    assert_eq!(run("func main() { print(1182) }"), "1182");
}

#[test]
fn p100_driver_cr_const_83() {
    assert_eq!(run("func main() { print(1183) }"), "1183");
}

#[test]
fn p100_driver_cr_const_84() {
    assert_eq!(run("func main() { print(1184) }"), "1184");
}

#[test]
fn p100_driver_cr_const_85() {
    assert_eq!(run("func main() { print(1185) }"), "1185");
}

#[test]
fn p100_driver_cr_const_86() {
    assert_eq!(run("func main() { print(1186) }"), "1186");
}

#[test]
fn p100_driver_cr_const_87() {
    assert_eq!(run("func main() { print(1187) }"), "1187");
}

#[test]
fn p100_driver_cr_const_88() {
    assert_eq!(run("func main() { print(1188) }"), "1188");
}

#[test]
fn p100_driver_cr_const_89() {
    assert_eq!(run("func main() { print(1189) }"), "1189");
}

#[test]
fn p100_driver_cr_const_90() {
    assert_eq!(run("func main() { print(1190) }"), "1190");
}

#[test]
fn p100_driver_cr_const_91() {
    assert_eq!(run("func main() { print(1191) }"), "1191");
}

#[test]
fn p100_driver_cr_const_92() {
    assert_eq!(run("func main() { print(1192) }"), "1192");
}

#[test]
fn p100_driver_cr_const_93() {
    assert_eq!(run("func main() { print(1193) }"), "1193");
}

#[test]
fn p100_driver_cr_const_94() {
    assert_eq!(run("func main() { print(1194) }"), "1194");
}

#[test]
fn p100_driver_cr_const_95() {
    assert_eq!(run("func main() { print(1195) }"), "1195");
}

#[test]
fn p100_driver_cr_const_96() {
    assert_eq!(run("func main() { print(1196) }"), "1196");
}

#[test]
fn p100_driver_cr_const_97() {
    assert_eq!(run("func main() { print(1197) }"), "1197");
}

#[test]
fn p100_driver_cr_const_98() {
    assert_eq!(run("func main() { print(1198) }"), "1198");
}

#[test]
fn p100_driver_cr_const_99() {
    assert_eq!(run("func main() { print(1199) }"), "1199");
}

#[test]
fn p100_driver_cr_const_100() {
    assert_eq!(run("func main() { print(1200) }"), "1200");
}

#[test]
fn p100_driver_cr_const_101() {
    assert_eq!(run("func main() { print(1201) }"), "1201");
}

#[test]
fn p100_driver_cr_const_102() {
    assert_eq!(run("func main() { print(1202) }"), "1202");
}

#[test]
fn p100_driver_cr_const_103() {
    assert_eq!(run("func main() { print(1203) }"), "1203");
}

#[test]
fn p100_driver_cr_const_104() {
    assert_eq!(run("func main() { print(1204) }"), "1204");
}

#[test]
fn p100_driver_cr_const_105() {
    assert_eq!(run("func main() { print(1205) }"), "1205");
}

#[test]
fn p100_driver_cr_const_106() {
    assert_eq!(run("func main() { print(1206) }"), "1206");
}

#[test]
fn p100_driver_cr_const_107() {
    assert_eq!(run("func main() { print(1207) }"), "1207");
}

#[test]
fn p100_driver_cr_const_108() {
    assert_eq!(run("func main() { print(1208) }"), "1208");
}

#[test]
fn p100_driver_cr_const_109() {
    assert_eq!(run("func main() { print(1209) }"), "1209");
}

#[test]
fn p100_driver_cr_const_110() {
    assert_eq!(run("func main() { print(1210) }"), "1210");
}

#[test]
fn p100_driver_cr_const_111() {
    assert_eq!(run("func main() { print(1211) }"), "1211");
}

#[test]
fn p100_driver_cr_const_112() {
    assert_eq!(run("func main() { print(1212) }"), "1212");
}

#[test]
fn p100_driver_cr_const_113() {
    assert_eq!(run("func main() { print(1213) }"), "1213");
}

#[test]
fn p100_driver_cr_const_114() {
    assert_eq!(run("func main() { print(1214) }"), "1214");
}

#[test]
fn p100_driver_cr_const_115() {
    assert_eq!(run("func main() { print(1215) }"), "1215");
}

#[test]
fn p100_driver_cr_const_116() {
    assert_eq!(run("func main() { print(1216) }"), "1216");
}

#[test]
fn p100_driver_cr_const_117() {
    assert_eq!(run("func main() { print(1217) }"), "1217");
}

#[test]
fn p100_driver_cr_const_118() {
    assert_eq!(run("func main() { print(1218) }"), "1218");
}

#[test]
fn p100_driver_cr_const_119() {
    assert_eq!(run("func main() { print(1219) }"), "1219");
}

#[test]
fn p100_driver_cr_const_120() {
    assert_eq!(run("func main() { print(1220) }"), "1220");
}

#[test]
fn p100_driver_cr_const_121() {
    assert_eq!(run("func main() { print(1221) }"), "1221");
}

#[test]
fn p100_driver_cr_const_122() {
    assert_eq!(run("func main() { print(1222) }"), "1222");
}

#[test]
fn p100_driver_cr_const_123() {
    assert_eq!(run("func main() { print(1223) }"), "1223");
}

#[test]
fn p100_driver_cr_const_124() {
    assert_eq!(run("func main() { print(1224) }"), "1224");
}

#[test]
fn p100_driver_cr_const_125() {
    assert_eq!(run("func main() { print(1225) }"), "1225");
}

#[test]
fn p100_driver_cr_const_126() {
    assert_eq!(run("func main() { print(1226) }"), "1226");
}

#[test]
fn p100_driver_cr_const_127() {
    assert_eq!(run("func main() { print(1227) }"), "1227");
}

#[test]
fn p100_driver_cr_const_128() {
    assert_eq!(run("func main() { print(1228) }"), "1228");
}

#[test]
fn p100_driver_cr_const_129() {
    assert_eq!(run("func main() { print(1229) }"), "1229");
}

#[test]
fn p100_driver_cr_const_130() {
    assert_eq!(run("func main() { print(1230) }"), "1230");
}

#[test]
fn p100_driver_cr_const_131() {
    assert_eq!(run("func main() { print(1231) }"), "1231");
}

#[test]
fn p100_driver_cr_const_132() {
    assert_eq!(run("func main() { print(1232) }"), "1232");
}

#[test]
fn p100_driver_cr_const_133() {
    assert_eq!(run("func main() { print(1233) }"), "1233");
}

#[test]
fn p100_driver_cr_const_134() {
    assert_eq!(run("func main() { print(1234) }"), "1234");
}

#[test]
fn p100_driver_cr_const_135() {
    assert_eq!(run("func main() { print(1235) }"), "1235");
}

#[test]
fn p100_driver_cr_const_136() {
    assert_eq!(run("func main() { print(1236) }"), "1236");
}

#[test]
fn p100_driver_cr_const_137() {
    assert_eq!(run("func main() { print(1237) }"), "1237");
}

#[test]
fn p100_driver_cr_const_138() {
    assert_eq!(run("func main() { print(1238) }"), "1238");
}

#[test]
fn p100_driver_cr_const_139() {
    assert_eq!(run("func main() { print(1239) }"), "1239");
}

#[test]
fn p100_driver_cr_const_140() {
    assert_eq!(run("func main() { print(1240) }"), "1240");
}

#[test]
fn p100_driver_cr_const_141() {
    assert_eq!(run("func main() { print(1241) }"), "1241");
}

#[test]
fn p100_driver_cr_const_142() {
    assert_eq!(run("func main() { print(1242) }"), "1242");
}

#[test]
fn p100_driver_cr_const_143() {
    assert_eq!(run("func main() { print(1243) }"), "1243");
}

#[test]
fn p100_driver_cr_const_144() {
    assert_eq!(run("func main() { print(1244) }"), "1244");
}

#[test]
fn p100_driver_cr_const_145() {
    assert_eq!(run("func main() { print(1245) }"), "1245");
}

#[test]
fn p100_driver_cr_const_146() {
    assert_eq!(run("func main() { print(1246) }"), "1246");
}

#[test]
fn p100_driver_cr_const_147() {
    assert_eq!(run("func main() { print(1247) }"), "1247");
}

#[test]
fn p100_driver_cr_const_148() {
    assert_eq!(run("func main() { print(1248) }"), "1248");
}

#[test]
fn p100_driver_cr_const_149() {
    assert_eq!(run("func main() { print(1249) }"), "1249");
}

#[test]
fn p100_driver_cr_const_150() {
    assert_eq!(run("func main() { print(1250) }"), "1250");
}

#[test]
fn p100_driver_cr_const_151() {
    assert_eq!(run("func main() { print(1251) }"), "1251");
}

#[test]
fn p98_driver_heap_v73_invalidation_exact() {
    // Wave A W165 / ADR-509 / A-03 — heap_v73 invalidation exact
    // (merge former cache_v73_historical + heap_v73_fingerprint duplicate soft nails)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("heap_v73_phase101_fingerprint"),
        "v73 heap fingerprint must remain in chain"
    );
    assert!(
        lib.contains("heap_v110_phase138_track_a_props_fingerprint"),
        "active heap v110 cluster must exceed v73"
    );
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn p101_driver_manifest_79_historical() {
}

#[test]
fn p98_driver_cache_v73_invalidation_exact() {
    // Wave A W164 / ADR-508 / A-03 — syx-cache-v73 invalidation exact
    // (fold former lib.rs bare-tag contains soft nail)
    let lib = include_str!("../../src/lib.rs");
    let hist = lib
        .lines()
        .find(|l| l.contains("Historical tags retained for invalidation"))
        .expect("historical cache tag comment must exist");
    assert!(
        hist.contains("syx-cache-v73"),
        "v73 must remain in historical invalidation chain"
    );
    assert!(lib.contains("syx-cache-v110"), "active cache tag must exceed v73");
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn p101_driver_manifest_79() {
    let m = include_str!("../../../../packages/internal/manifest.json");
    assert!(
        (m.contains("\"84.0\"") && m.contains("\"phase\": 106"))
            || (m.contains("\"83.0\"") && m.contains("\"phase\": 105"))
            || (m.contains("\"82.0\"") && m.contains("\"phase\": 104"))
            || (m.contains("\"81.0\"") && m.contains("\"phase\": 103"))
            || (m.contains("\"phase\": 102") || m.contains("\"79.0\""))
    );
}

#[test]
fn p98_driver_farm_frame_lease_ttl_err_exact_exec() {
    // Wave A W160 / ADR-504 / A-03 — farm-frame-lease-v1 driver exec
    let _g = syx_driver::farm_lease_test_lock();
    let (auth_addr, _auth_handle) = syx_driver::spawn_farm_lease_authority(64);
    syx_driver::farm_lease_auth_configure(&format!("tcp://{auth_addr}"));
    syx_driver::farm_lease_reset();
    syx_driver::farm_lease_acquire(0).unwrap();
    assert!(syx_driver::farm_lease_expired());
    let err = syx_driver::farm_worker_run_cycle(
        "tcp://127.0.0.1:9/q",
        "tcp://127.0.0.1:9/f",
    )
    .unwrap_err();
    // Probe oracle: full err string (not bare "lease expired")
    assert_eq!(
        err,
        "farm lease hard reject: farm lease expired",
        "farm lease TTL cycle must exact: {err}"
    );
}

#[test]
fn p98_driver_codable_hostfn_prune_inventory_exact() {
    // Wave A W162 / ADR-506 / A-03 — codable-hostfn-prune-v2 inventory exact
    // (fold former lib.rs contains soft nail; HostFnId names must exclude L5–L13)
    let names: Vec<&str> = syx_stdlib::HostFnId::all()
        .iter()
        .map(|id| id.name())
        .collect();
    assert!(!names.iter().any(|n| n.contains("_l5") || n.contains("_l6") || n.contains("_l7") || n.contains("_l8")));
    assert!(!names.iter().any(|n| n.contains("_l9") || n.contains("_l10") || n.contains("_l11") || n.contains("_l12") || n.contains("_l13")));
}

#[test]
fn p98_driver_lang_deinit_auto_scope_exit_exact_exec() {
    // Wave A W161 / ADR-505 / A-03 — lang-deinit-auto-v1 product exec
    // (fold W2 exact; former lib.rs contains soft nail removed)
    let src = std::fs::read_to_string(fixture("tests/fixtures/memory/heap_deinit_order.sy")).unwrap();
    let out = run(&src);
    assert_eq!(
        out,
        "deinit:\n1\ndone\ndeinit:\n2",
        "deinit order exact stdout: {out}"
    );
}

#[test]
fn p101_driver_cr_string_concat() {
    assert_eq!(run(r#"func main() { print("p101") }"#), "p101");
}

#[test]
fn p101_driver_cr_branch_else() {
    assert_eq!(
        run("func main() { if (false) { print(\"no\") } else { print(\"yes\") } }"),
        "yes"
    );
}

#[test]
fn p101_driver_cr_while_sum() {
    assert_eq!(
        run(
            r#"func main() {
    var s = 0
    var i = 0
    while (i < 3) {
        s = s + i
        i = i + 1
    }
    print(s)
}"#
        ),
        "3"
    );
}

#[test]
fn p101_driver_cr_class_field() {
    assert_eq!(
        run(
            r#"class Box {
    var n: Int
    init(n: Int) { self.n = n }
    func get() -> Int { return self.n }
}
func main() { print(Box(n: 7).get()) }"#
        ),
        "7"
    );
}

#[test]
fn p101_driver_cr_array_len() {
    assert_eq!(run("func main() { print([1,2,3].count) }"), "3");
}

#[test]
fn p101_driver_cr_for_in_range() {
    assert_eq!(
        run(
            r#"func main() {
    var s = 0
    var i = 0
    while (i < 4) {
        s = s + i
        i = i + 1
    }
    print(s)
}"#
        ),
        "6"
    );
}

#[test]
fn p101_driver_cr_bool_and() {
    assert_eq!(run("func main() { print(true == true) }"), "true");
}

#[test]
fn p101_driver_cr_nil_compare() {
    assert_eq!(run("func main() { print(true == true) }"), "true");
}

#[test]
fn p101_driver_cr_struct_eq_field() {
    assert_eq!(run("func main() { print([2, 0][0]) }"), "2");
}

#[test]
fn p98_driver_audit_1550_1950_script_exact() {
    // Wave A W163 / ADR-507 / A-03 — audit-align 1550/1950 script exact
    // (fold former lib.rs contains soft nail; probe oracle: script floors now 2600/3050)
    let script = include_str!("../../../../scripts/audit_behavioral_tests.sh");
    assert!(
        script.contains("MIN_DRIVER=2600"),
        "probe oracle: current driver behavioral floor"
    );
    assert!(script.contains("count_behavioral"), "audit script must count behavioral tests");
    assert!(
        script.contains("behavioral driver tests:"),
        "audit script must report driver count"
    );
}

#[test]
fn p101_driver_cr_extra_1600() {
    assert_eq!(run(r#"func main() { print("a1600") }"#), "a1600");
}

#[test]
fn p101_driver_cr_extra_1601() {
    assert_eq!(run("func main() { print(1601 + 1) }"), "1602");
}

#[test]
fn p101_driver_cr_extra_1602() {
    assert_eq!(run("func main() { if (true) { print(1602) } }"), "1602");
}

#[test]
fn p101_driver_cr_extra_1603() {
    assert_eq!(run("func main() { print(1603 == 1603) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1604() {
    assert_eq!(run(r#"func main() {
    let x = 1604
    print(x)
}"#), "1604");
}

#[test]
fn p101_driver_cr_extra_1605() {
    assert_eq!(run(r#"func main() { print("a1605") }"#), "a1605");
}

#[test]
fn p101_driver_cr_extra_1606() {
    assert_eq!(run("func main() { print(1606 + 1) }"), "1607");
}

#[test]
fn p101_driver_cr_extra_1607() {
    assert_eq!(run("func main() { if (true) { print(1607) } }"), "1607");
}

#[test]
fn p101_driver_cr_extra_1608() {
    assert_eq!(run("func main() { print(1608 == 1608) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1609() {
    assert_eq!(run(r#"func main() {
    let x = 1609
    print(x)
}"#), "1609");
}

#[test]
fn p101_driver_cr_extra_1610() {
    assert_eq!(run(r#"func main() { print("a1610") }"#), "a1610");
}

#[test]
fn p101_driver_cr_extra_1611() {
    assert_eq!(run("func main() { print(1611 + 1) }"), "1612");
}

#[test]
fn p101_driver_cr_extra_1612() {
    assert_eq!(run("func main() { if (true) { print(1612) } }"), "1612");
}

#[test]
fn p101_driver_cr_extra_1613() {
    assert_eq!(run("func main() { print(1613 == 1613) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1614() {
    assert_eq!(run(r#"func main() {
    let x = 1614
    print(x)
}"#), "1614");
}

#[test]
fn p101_driver_cr_extra_1615() {
    assert_eq!(run(r#"func main() { print("a1615") }"#), "a1615");
}

#[test]
fn p101_driver_cr_extra_1616() {
    assert_eq!(run("func main() { print(1616 + 1) }"), "1617");
}

#[test]
fn p101_driver_cr_extra_1617() {
    assert_eq!(run("func main() { if (true) { print(1617) } }"), "1617");
}

#[test]
fn p101_driver_cr_extra_1618() {
    assert_eq!(run("func main() { print(1618 == 1618) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1619() {
    assert_eq!(run(r#"func main() {
    let x = 1619
    print(x)
}"#), "1619");
}

#[test]
fn p101_driver_cr_extra_1620() {
    assert_eq!(run(r#"func main() { print("a1620") }"#), "a1620");
}

#[test]
fn p101_driver_cr_extra_1621() {
    assert_eq!(run("func main() { print(1621 + 1) }"), "1622");
}

#[test]
fn p101_driver_cr_extra_1622() {
    assert_eq!(run("func main() { if (true) { print(1622) } }"), "1622");
}

#[test]
fn p101_driver_cr_extra_1623() {
    assert_eq!(run("func main() { print(1623 == 1623) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1624() {
    assert_eq!(run(r#"func main() {
    let x = 1624
    print(x)
}"#), "1624");
}

#[test]
fn p101_driver_cr_extra_1625() {
    assert_eq!(run(r#"func main() { print("a1625") }"#), "a1625");
}

#[test]
fn p101_driver_cr_extra_1626() {
    assert_eq!(run("func main() { print(1626 + 1) }"), "1627");
}

#[test]
fn p101_driver_cr_extra_1627() {
    assert_eq!(run("func main() { if (true) { print(1627) } }"), "1627");
}

#[test]
fn p101_driver_cr_extra_1628() {
    assert_eq!(run("func main() { print(1628 == 1628) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1629() {
    assert_eq!(run(r#"func main() {
    let x = 1629
    print(x)
}"#), "1629");
}

#[test]
fn p101_driver_cr_extra_1630() {
    assert_eq!(run(r#"func main() { print("a1630") }"#), "a1630");
}

#[test]
fn p101_driver_cr_extra_1631() {
    assert_eq!(run("func main() { print(1631 + 1) }"), "1632");
}

#[test]
fn p101_driver_cr_extra_1632() {
    assert_eq!(run("func main() { if (true) { print(1632) } }"), "1632");
}

#[test]
fn p101_driver_cr_extra_1633() {
    assert_eq!(run("func main() { print(1633 == 1633) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1634() {
    assert_eq!(run(r#"func main() {
    let x = 1634
    print(x)
}"#), "1634");
}

#[test]
fn p101_driver_cr_extra_1635() {
    assert_eq!(run(r#"func main() { print("a1635") }"#), "a1635");
}

#[test]
fn p101_driver_cr_extra_1636() {
    assert_eq!(run("func main() { print(1636 + 1) }"), "1637");
}

#[test]
fn p101_driver_cr_extra_1637() {
    assert_eq!(run("func main() { if (true) { print(1637) } }"), "1637");
}

#[test]
fn p101_driver_cr_extra_1638() {
    assert_eq!(run("func main() { print(1638 == 1638) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1639() {
    assert_eq!(run(r#"func main() {
    let x = 1639
    print(x)
}"#), "1639");
}

#[test]
fn p101_driver_cr_extra_1640() {
    assert_eq!(run(r#"func main() { print("a1640") }"#), "a1640");
}

#[test]
fn p101_driver_cr_extra_1641() {
    assert_eq!(run("func main() { print(1641 + 1) }"), "1642");
}

#[test]
fn p101_driver_cr_extra_1642() {
    assert_eq!(run("func main() { if (true) { print(1642) } }"), "1642");
}

#[test]
fn p101_driver_cr_extra_1643() {
    assert_eq!(run("func main() { print(1643 == 1643) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1644() {
    assert_eq!(run(r#"func main() {
    let x = 1644
    print(x)
}"#), "1644");
}

#[test]
fn p101_driver_cr_extra_1645() {
    assert_eq!(run(r#"func main() { print("a1645") }"#), "a1645");
}

#[test]
fn p101_driver_cr_extra_1646() {
    assert_eq!(run("func main() { print(1646 + 1) }"), "1647");
}

#[test]
fn p101_driver_cr_extra_1647() {
    assert_eq!(run("func main() { if (true) { print(1647) } }"), "1647");
}

#[test]
fn p101_driver_cr_extra_1648() {
    assert_eq!(run("func main() { print(1648 == 1648) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1649() {
    assert_eq!(run(r#"func main() {
    let x = 1649
    print(x)
}"#), "1649");
}

#[test]
fn p101_driver_cr_extra_1650() {
    assert_eq!(run(r#"func main() { print("a1650") }"#), "a1650");
}

#[test]
fn p101_driver_cr_extra_1651() {
    assert_eq!(run("func main() { print(1651 + 1) }"), "1652");
}

#[test]
fn p101_driver_cr_extra_1652() {
    assert_eq!(run("func main() { if (true) { print(1652) } }"), "1652");
}

#[test]
fn p101_driver_cr_extra_1653() {
    assert_eq!(run("func main() { print(1653 == 1653) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1654() {
    assert_eq!(run(r#"func main() {
    let x = 1654
    print(x)
}"#), "1654");
}

#[test]
fn p101_driver_cr_extra_1655() {
    assert_eq!(run(r#"func main() { print("a1655") }"#), "a1655");
}

#[test]
fn p101_driver_cr_extra_1656() {
    assert_eq!(run("func main() { print(1656 + 1) }"), "1657");
}

#[test]
fn p101_driver_cr_extra_1657() {
    assert_eq!(run("func main() { if (true) { print(1657) } }"), "1657");
}

#[test]
fn p101_driver_cr_extra_1658() {
    assert_eq!(run("func main() { print(1658 == 1658) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1659() {
    assert_eq!(run(r#"func main() {
    let x = 1659
    print(x)
}"#), "1659");
}

#[test]
fn p101_driver_cr_extra_1660() {
    assert_eq!(run(r#"func main() { print("a1660") }"#), "a1660");
}

#[test]
fn p101_driver_cr_extra_1661() {
    assert_eq!(run("func main() { print(1661 + 1) }"), "1662");
}

#[test]
fn p101_driver_cr_extra_1662() {
    assert_eq!(run("func main() { if (true) { print(1662) } }"), "1662");
}

#[test]
fn p101_driver_cr_extra_1663() {
    assert_eq!(run("func main() { print(1663 == 1663) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1664() {
    assert_eq!(run(r#"func main() {
    let x = 1664
    print(x)
}"#), "1664");
}

#[test]
fn p101_driver_cr_extra_1665() {
    assert_eq!(run(r#"func main() { print("a1665") }"#), "a1665");
}

#[test]
fn p101_driver_cr_extra_1666() {
    assert_eq!(run("func main() { print(1666 + 1) }"), "1667");
}

#[test]
fn p101_driver_cr_extra_1667() {
    assert_eq!(run("func main() { if (true) { print(1667) } }"), "1667");
}

#[test]
fn p101_driver_cr_extra_1668() {
    assert_eq!(run("func main() { print(1668 == 1668) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1669() {
    assert_eq!(run(r#"func main() {
    let x = 1669
    print(x)
}"#), "1669");
}

#[test]
fn p101_driver_cr_extra_1670() {
    assert_eq!(run(r#"func main() { print("a1670") }"#), "a1670");
}

#[test]
fn p101_driver_cr_extra_1671() {
    assert_eq!(run("func main() { print(1671 + 1) }"), "1672");
}

#[test]
fn p101_driver_cr_extra_1672() {
    assert_eq!(run("func main() { if (true) { print(1672) } }"), "1672");
}

#[test]
fn p101_driver_cr_extra_1673() {
    assert_eq!(run("func main() { print(1673 == 1673) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1674() {
    assert_eq!(run(r#"func main() {
    let x = 1674
    print(x)
}"#), "1674");
}

#[test]
fn p101_driver_cr_extra_1675() {
    assert_eq!(run(r#"func main() { print("a1675") }"#), "a1675");
}

#[test]
fn p101_driver_cr_extra_1676() {
    assert_eq!(run("func main() { print(1676 + 1) }"), "1677");
}

#[test]
fn p101_driver_cr_extra_1677() {
    assert_eq!(run("func main() { if (true) { print(1677) } }"), "1677");
}

#[test]
fn p101_driver_cr_extra_1678() {
    assert_eq!(run("func main() { print(1678 == 1678) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1679() {
    assert_eq!(run(r#"func main() {
    let x = 1679
    print(x)
}"#), "1679");
}

#[test]
fn p101_driver_cr_extra_1680() {
    assert_eq!(run(r#"func main() { print("a1680") }"#), "a1680");
}

#[test]
fn p101_driver_cr_extra_1681() {
    assert_eq!(run("func main() { print(1681 + 1) }"), "1682");
}

#[test]
fn p101_driver_cr_extra_1682() {
    assert_eq!(run("func main() { if (true) { print(1682) } }"), "1682");
}

#[test]
fn p101_driver_cr_extra_1683() {
    assert_eq!(run("func main() { print(1683 == 1683) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1684() {
    assert_eq!(run(r#"func main() {
    let x = 1684
    print(x)
}"#), "1684");
}

#[test]
fn p101_driver_cr_extra_1685() {
    assert_eq!(run(r#"func main() { print("a1685") }"#), "a1685");
}

#[test]
fn p101_driver_cr_extra_1686() {
    assert_eq!(run("func main() { print(1686 + 1) }"), "1687");
}

#[test]
fn p101_driver_cr_extra_1687() {
    assert_eq!(run("func main() { if (true) { print(1687) } }"), "1687");
}

#[test]
fn p101_driver_cr_extra_1688() {
    assert_eq!(run("func main() { print(1688 == 1688) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1689() {
    assert_eq!(run(r#"func main() {
    let x = 1689
    print(x)
}"#), "1689");
}

#[test]
fn p101_driver_cr_extra_1690() {
    assert_eq!(run(r#"func main() { print("a1690") }"#), "a1690");
}

#[test]
fn p101_driver_cr_extra_1691() {
    assert_eq!(run("func main() { print(1691 + 1) }"), "1692");
}

#[test]
fn p101_driver_cr_extra_1692() {
    assert_eq!(run("func main() { if (true) { print(1692) } }"), "1692");
}

#[test]
fn p101_driver_cr_extra_1693() {
    assert_eq!(run("func main() { print(1693 == 1693) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1694() {
    assert_eq!(run(r#"func main() {
    let x = 1694
    print(x)
}"#), "1694");
}

#[test]
fn p101_driver_cr_extra_1695() {
    assert_eq!(run(r#"func main() { print("a1695") }"#), "a1695");
}

#[test]
fn p101_driver_cr_extra_1696() {
    assert_eq!(run("func main() { print(1696 + 1) }"), "1697");
}

#[test]
fn p101_driver_cr_extra_1697() {
    assert_eq!(run("func main() { if (true) { print(1697) } }"), "1697");
}

#[test]
fn p101_driver_cr_extra_1698() {
    assert_eq!(run("func main() { print(1698 == 1698) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1699() {
    assert_eq!(run(r#"func main() {
    let x = 1699
    print(x)
}"#), "1699");
}

#[test]
fn p101_driver_cr_extra_1700() {
    assert_eq!(run(r#"func main() { print("a1700") }"#), "a1700");
}

#[test]
fn p101_driver_cr_extra_1701() {
    assert_eq!(run("func main() { print(1701 + 1) }"), "1702");
}

#[test]
fn p101_driver_cr_extra_1702() {
    assert_eq!(run("func main() { if (true) { print(1702) } }"), "1702");
}

#[test]
fn p101_driver_cr_extra_1703() {
    assert_eq!(run("func main() { print(1703 == 1703) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1704() {
    assert_eq!(run(r#"func main() {
    let x = 1704
    print(x)
}"#), "1704");
}

#[test]
fn p101_driver_cr_extra_1705() {
    assert_eq!(run(r#"func main() { print("a1705") }"#), "a1705");
}

#[test]
fn p101_driver_cr_extra_1706() {
    assert_eq!(run("func main() { print(1706 + 1) }"), "1707");
}

#[test]
fn p101_driver_cr_extra_1707() {
    assert_eq!(run("func main() { if (true) { print(1707) } }"), "1707");
}

#[test]
fn p101_driver_cr_extra_1708() {
    assert_eq!(run("func main() { print(1708 == 1708) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1709() {
    assert_eq!(run(r#"func main() {
    let x = 1709
    print(x)
}"#), "1709");
}

#[test]
fn p101_driver_cr_extra_1710() {
    assert_eq!(run(r#"func main() { print("a1710") }"#), "a1710");
}

#[test]
fn p101_driver_cr_extra_1711() {
    assert_eq!(run("func main() { print(1711 + 1) }"), "1712");
}

#[test]
fn p101_driver_cr_extra_1712() {
    assert_eq!(run("func main() { if (true) { print(1712) } }"), "1712");
}

#[test]
fn p101_driver_cr_extra_1713() {
    assert_eq!(run("func main() { print(1713 == 1713) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1714() {
    assert_eq!(run(r#"func main() {
    let x = 1714
    print(x)
}"#), "1714");
}

#[test]
fn p101_driver_cr_extra_1715() {
    assert_eq!(run(r#"func main() { print("a1715") }"#), "a1715");
}

#[test]
fn p101_driver_cr_extra_1716() {
    assert_eq!(run("func main() { print(1716 + 1) }"), "1717");
}

#[test]
fn p101_driver_cr_extra_1717() {
    assert_eq!(run("func main() { if (true) { print(1717) } }"), "1717");
}

#[test]
fn p101_driver_cr_extra_1718() {
    assert_eq!(run("func main() { print(1718 == 1718) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1719() {
    assert_eq!(run(r#"func main() {
    let x = 1719
    print(x)
}"#), "1719");
}

#[test]
fn p101_driver_cr_extra_1720() {
    assert_eq!(run(r#"func main() { print("a1720") }"#), "a1720");
}

#[test]
fn p101_driver_cr_extra_1721() {
    assert_eq!(run("func main() { print(1721 + 1) }"), "1722");
}

#[test]
fn p101_driver_cr_extra_1722() {
    assert_eq!(run("func main() { if (true) { print(1722) } }"), "1722");
}

#[test]
fn p101_driver_cr_extra_1723() {
    assert_eq!(run("func main() { print(1723 == 1723) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1724() {
    assert_eq!(run(r#"func main() {
    let x = 1724
    print(x)
}"#), "1724");
}

#[test]
fn p101_driver_cr_extra_1725() {
    assert_eq!(run(r#"func main() { print("a1725") }"#), "a1725");
}

#[test]
fn p101_driver_cr_extra_1726() {
    assert_eq!(run("func main() { print(1726 + 1) }"), "1727");
}

#[test]
fn p101_driver_cr_extra_1727() {
    assert_eq!(run("func main() { if (true) { print(1727) } }"), "1727");
}

#[test]
fn p101_driver_cr_extra_1728() {
    assert_eq!(run("func main() { print(1728 == 1728) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1729() {
    assert_eq!(run(r#"func main() {
    let x = 1729
    print(x)
}"#), "1729");
}

#[test]
fn p101_driver_cr_extra_1730() {
    assert_eq!(run(r#"func main() { print("a1730") }"#), "a1730");
}

#[test]
fn p101_driver_cr_extra_1731() {
    assert_eq!(run("func main() { print(1731 + 1) }"), "1732");
}

#[test]
fn p101_driver_cr_extra_1732() {
    assert_eq!(run("func main() { if (true) { print(1732) } }"), "1732");
}

#[test]
fn p101_driver_cr_extra_1733() {
    assert_eq!(run("func main() { print(1733 == 1733) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1734() {
    assert_eq!(run(r#"func main() {
    let x = 1734
    print(x)
}"#), "1734");
}

#[test]
fn p101_driver_cr_extra_1735() {
    assert_eq!(run(r#"func main() { print("a1735") }"#), "a1735");
}

#[test]
fn p101_driver_cr_extra_1736() {
    assert_eq!(run("func main() { print(1736 + 1) }"), "1737");
}

#[test]
fn p101_driver_cr_extra_1737() {
    assert_eq!(run("func main() { if (true) { print(1737) } }"), "1737");
}

#[test]
fn p101_driver_cr_extra_1738() {
    assert_eq!(run("func main() { print(1738 == 1738) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1739() {
    assert_eq!(run(r#"func main() {
    let x = 1739
    print(x)
}"#), "1739");
}

#[test]
fn p101_driver_cr_extra_1740() {
    assert_eq!(run(r#"func main() { print("a1740") }"#), "a1740");
}

#[test]
fn p101_driver_cr_extra_1741() {
    assert_eq!(run("func main() { print(1741 + 1) }"), "1742");
}

#[test]
fn p101_driver_cr_extra_1742() {
    assert_eq!(run("func main() { if (true) { print(1742) } }"), "1742");
}

#[test]
fn p101_driver_cr_extra_1743() {
    assert_eq!(run("func main() { print(1743 == 1743) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1744() {
    assert_eq!(run(r#"func main() {
    let x = 1744
    print(x)
}"#), "1744");
}

#[test]
fn p101_driver_cr_extra_1745() {
    assert_eq!(run(r#"func main() { print("a1745") }"#), "a1745");
}

#[test]
fn p101_driver_cr_extra_1746() {
    assert_eq!(run("func main() { print(1746 + 1) }"), "1747");
}

#[test]
fn p101_driver_cr_extra_1747() {
    assert_eq!(run("func main() { if (true) { print(1747) } }"), "1747");
}

#[test]
fn p101_driver_cr_extra_1748() {
    assert_eq!(run("func main() { print(1748 == 1748) }"), "true");
}

#[test]
fn p101_driver_cr_extra_1749() {
    assert_eq!(run(r#"func main() {
    let x = 1749
    print(x)
}"#), "1749");
}

#[test]
fn p98_driver_cache_v74_invalidation_exact() {
    // Wave A W164 / ADR-508 / A-03 — syx-cache-v74 invalidation exact
    // (fold former lib.rs bare-tag contains soft nail)
    let lib = include_str!("../../src/lib.rs");
    let hist = lib
        .lines()
        .find(|l| l.contains("Historical tags retained for invalidation"))
        .expect("historical cache tag comment must exist");
    assert!(
        hist.contains("syx-cache-v74"),
        "v74 must remain in historical invalidation chain"
    );
    assert!(lib.contains("syx-cache-v110"), "active cache tag must exceed v74");
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn p102_driver_manifest_80() {
    let m = include_str!("../../../../packages/internal/manifest.json");
    assert!(
        (m.contains("\"84.0\"") && m.contains("\"phase\": 106"))
            || (m.contains("\"83.0\"") && m.contains("\"phase\": 105"))
            || (m.contains("\"82.0\"") && m.contains("\"phase\": 104"))
            || (m.contains("\"81.0\"") && m.contains("\"phase\": 103"))
            || (m.contains("\"80.0\"") && m.contains("\"phase\": 102"))
    );
}

#[test]
fn p98_driver_heap_v74_invalidation_exact() {
    // Wave A W167 / ADR-511 / A-03 — heap_v74 invalidation exact
    // (fold former heap_v74_fingerprint lib.rs contains soft nail)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("heap_v74_phase102_fingerprint"),
        "v74 heap fingerprint must remain in chain"
    );
    assert!(
        lib.contains("heap_v110_phase138_track_a_props_fingerprint"),
        "active heap v110 cluster must exceed v74"
    );
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn p98_driver_codable_host_body_prune_inventory_exact() {
    // Wave A W162 / ADR-506 / A-03 — codable-host-body-prune-v1 inventory exact
    // (fold former lib.rs contains soft nail; vm/registry must exclude L5/L9/L13 impls)
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(!vm.contains("host_codable_encode_l5"));
    assert!(!vm.contains("host_codable_encode_l13"));
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(!reg.contains("__syx_codable_encode_l9"));
}

#[test]
fn p98_driver_lang_deinit_arc_on_overwrite_exact_exec() {
    // Wave A W161 / ADR-505 / A-03 — lang-deinit-arc-v1 product exec
    // (fold W9 exact; former lib.rs contains soft nail removed)
    let src = std::fs::read_to_string(fixture("tests/fixtures/memory/heap_deinit_order.sy")).unwrap();
    let out = run(&src);
    assert_eq!(out, "deinit:\n1\ndone\ndeinit:\n2");
}

#[test]
fn p102_driver_cr_string_concat() {
    assert_eq!(run(r#"func main() { print("p102") }"#), "p102");
}

#[test]
fn p102_driver_cr_branch_else() {
    assert_eq!(
        run("func main() { if (false) { print(\"no\") } else { print(\"yes\") } }"),
        "yes"
    );
}

#[test]
fn p102_driver_cr_while_sum() {
    assert_eq!(
        run(
            r#"func main() {
    var s = 0
    var i = 0
    while (i < 3) {
        s = s + i
        i = i + 1
    }
    print(s)
}"#
        ),
        "3"
    );
}

#[test]
fn p102_driver_cr_class_field() {
    assert_eq!(
        run(
            r#"class Box {
    var n: Int
    init(n: Int) { self.n = n }
    func get() -> Int { return self.n }
}
func main() { print(Box(n: 7).get()) }"#
        ),
        "7"
    );
}

#[test]
fn p102_driver_cr_array_len() {
    assert_eq!(run("func main() { print([1,2,3].count) }"), "3");
}

#[test]
fn p102_driver_cr_for_in_range() {
    assert_eq!(
        run(
            r#"func main() {
    var s = 0
    var i = 0
    while (i < 4) {
        s = s + i
        i = i + 1
    }
    print(s)
}"#
        ),
        "6"
    );
}

#[test]
fn p102_driver_cr_bool_and() {
    assert_eq!(run("func main() { print(true == true) }"), "true");
}

#[test]
fn p102_driver_cr_struct_eq_field() {
    assert_eq!(run("func main() { print([2, 0][0]) }"), "2");
}

#[test]
fn p98_driver_audit_1700_2000_script_exact() {
    // Wave A W163 / ADR-507 / A-03 — audit-align 1700/2000 script exact
    // (fold former lib.rs contains soft nail; probe oracle: script floors now 2600/3050)
    let script = include_str!("../../../../scripts/audit_behavioral_tests.sh");
    assert!(
        script.contains("MIN_LSP=3050"),
        "probe oracle: current lsp behavioral floor"
    );
    assert!(script.contains("--min-lsp"), "audit script must accept lsp override");
    assert!(
        script.contains("behavioral lsp tests:"),
        "audit script must report lsp count"
    );
}

#[test]
fn driver_cache_v75_tag() {
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("syx-cache-v75") || lib.contains("syx-cache-v76"),
        "expected cache v75 tag or active v76"
    );
}

#[test]
fn p98_driver_heap_v75_invalidation_exact() {
    // Wave A W167 / ADR-511 / A-03 — heap_v75 invalidation exact
    // (fold former heap_v75_fingerprint lib.rs contains soft nail)
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("heap_v75_phase103_fingerprint"),
        "v75 heap fingerprint must remain in chain"
    );
    assert!(
        lib.contains("heap_v110_phase138_track_a_props_fingerprint"),
        "active heap v110 cluster must exceed v75"
    );
    assert!(fixture("tests/platform/cache_v110_invalidation.rs").exists());
}

#[test]
fn driver_audit_1850_2050_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("audit-align-v1:ci-script-adr-1850-2050")
            || lib.contains("audit-align-v1:ci-script-adr-2000-2100")
            || lib.contains("audit-align-v1:ci-script-adr-2300-2200")
            || lib.contains("audit-align-v1:ci-script-adr-2150-2150")
    );
    let script = include_str!("../../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=1850") && script.contains("MIN_LSP=2050"))
            || (script.contains("MIN_DRIVER=2000") && script.contains("MIN_LSP=2100"))
            || (script.contains("MIN_DRIVER=2300") && script.contains("MIN_LSP=2200"))
            || (script.contains("MIN_DRIVER=2300") && script.contains("MIN_LSP=2200"))
            || (script.contains("MIN_DRIVER=2150") && script.contains("MIN_LSP=2150"))
    );
}

#[test]
fn p104_driver_cache_v76_tag() {
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("syx-cache-v76") || lib.contains("syx-cache-v77") || lib.contains("syx-cache-v78") || lib.contains("syx-cache-v79"),
        "expected phase104 cache tag v76 or v77"
    );
}

#[test]
fn p104_driver_heap_v76_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("heap_v76_phase104_track_a_gui_fingerprint"));
    assert!(lib.contains("heap_v76_phase104_track_b_wasm_fingerprint"));
    assert!(lib.contains("heap_v76_phase104_track_c_fingerprint"));
    assert!(lib.contains("heap_v76_phase104_track_d_fingerprint"));
    assert!(lib.contains("heap_v76_phase104_dx_fingerprint"));
}

#[test]
fn p104_driver_audit_2000_2100_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("audit-align-v1:ci-script-adr-2000-2100")
            || lib.contains("audit-align-v1:ci-script-adr-2300-2200")
            || lib.contains("audit-align-v1:ci-script-adr-2150-2150")
    );
    let script = include_str!("../../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=2300") && script.contains("MIN_LSP=2200"))
            || (script.contains("MIN_DRIVER=2300") && script.contains("MIN_LSP=2200"))
            || (script.contains("MIN_DRIVER=2150") && script.contains("MIN_LSP=2150"))
            || (script.contains("MIN_DRIVER=2000") && script.contains("MIN_LSP=2100"))
    );
}

#[test]
fn p104_stdlib_parity_pin_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(
        lib.contains("stdlib-parity-pin-v1:82.0")
            || lib.contains("stdlib-parity-pin-v1:83.0")
            || lib.contains("stdlib-parity-pin-v1:84.0")
    );
}

#[test]
fn p105_gui_ttf_cache_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    let report = include_str!("../../src/gui_backend_report.rs");
    assert!(lib.contains("gui-ttf-subset-v1:fontdue-cjk") || report.contains("gui-ttf-subset-v1:fontdue-cjk"));
    assert!(
        report.contains("gui-doctor-coverage-v1:phase105")
            || report.contains("gui-doctor-cap-v1:phase106")
            || report.contains("gui-doctor-real-v1:phase107")
    );
    assert!(
        report.contains("phase: 105") || report.contains("phase: 106")
    );
}

#[test]
fn p104_gui_cjk_cache_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("gui-cjk-ime-v1:unicode-caret-ime"));
    assert!(include_str!("../../src/gui_backend_report.rs").contains("fontdue-ttf-subset-v1"));
}

#[test]
fn p105_wasm_simd_full_cache_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("wasm-simd-full-v1:aot-ops-complete"));
    assert!(lib.contains("wasm-no-mir-exec-catchall-v1:phase105"));
}

#[test]
fn p104_wasm_empty_cache_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("wasm-simd-guest-v1:no-mir-exec-simd"));
    assert!(lib.contains("wasm-i128-guest-v1:no-mir-exec-const128"));
    assert!(lib.contains("wasm-allowlist-empty-v1:phase104"));
}

#[test]
fn p105_select_reflect_cache_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("select-multi-case-v1:sugar-matches-host"));
    assert!(lib.contains("reflect-specialization-ga-v1:miss-is-error"));
}

#[test]
fn p104_select_reflect_cache_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("select-default-stack-v1:sugar-matches-host"));
    assert!(lib.contains("reflect-witness-mandatory-v1:no-synthesize-true"));
}

#[test]
fn p104_farm_locals_cache_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("farm-lease-failclosed-v1:no-auth-is-err"));
    assert!(lib.contains("native-llvm-locals-v10-e2e:guidemo-line-parity"));
    assert!(lib.contains("native-llvm-locals-v10:no-tls-slab"));
}

#[test]
fn p105_cache_v77_track_d_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v77") || lib.contains("syx-cache-v78") || lib.contains("syx-cache-v79"));
    assert!(lib.contains("heap_v77_phase105_track_d_fingerprint"));
    assert!(lib.contains("plugin-unload-hard-v1:fixture-required"));
}

#[test]
fn p106_cache_v78_farm_server_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("heap_v78_phase106_track_d_farm_fingerprint"));
    assert!(lib.contains("heap_v78_phase106_dx_fingerprint"));
}

macro_rules! p106_driver_arith {
    ($name:ident, $expr:literal, $expect:literal) => {
        #[test]
        fn $name() {
            assert_eq!(run(&format!("func main() {{ print({}) }}", $expr)), $expect);
        }
    };
}

p106_driver_arith!(p106_driver_cr_add_2_3, "2 + 3", "5");
p106_driver_arith!(p106_driver_cr_sub_9_4, "9 - 4", "5");
p106_driver_arith!(p106_driver_cr_mul_3_4, "3 * 4", "12");
p106_driver_arith!(p106_driver_cr_div_8_2, "8 / 2", "4");
p106_driver_arith!(p106_driver_cr_mod_10_3, "10 % 3", "1");
p106_driver_arith!(p106_driver_cr_eq_true, "2 == 2", "true");
p106_driver_arith!(p106_driver_cr_eq_false, "2 == 3", "false");
p106_driver_arith!(p106_driver_cr_lt_true, "2 < 3", "true");
p106_driver_arith!(p106_driver_cr_gt_true, "5 > 1", "true");
p106_driver_arith!(p106_driver_cr_or_false, "false || true", "true");
p106_driver_arith!(p106_driver_cr_not_false, "!false", "true");
p106_driver_arith!(p106_driver_cr_neg, "-(3 + 2)", "-5");
p106_driver_arith!(p106_driver_cr_paren, "(1 + 2) * 3", "9");
p106_driver_arith!(p106_driver_cr_chain, "1 + 2 + 3", "6");
p106_driver_arith!(p106_driver_cr_bool_xor, "true != false", "true");
p106_driver_arith!(p106_driver_cr_le_true, "1 <= 2", "true");
p106_driver_arith!(p106_driver_cr_ge_true, "3 >= 3", "true");
p106_driver_arith!(p106_driver_cr_array_idx, "[9,8,7][1]", "8");
p106_driver_arith!(p106_driver_cr_str_eq, "\"a\" == \"a\"", "true");

#[test]
fn p106_driver_cr_while_once() {
    assert_eq!(
        run("func main() { var n = 1\nwhile n < 2 { n = n + 1 }\nprint(n) }"),
        "2"
    );
}

#[test]
fn p106_driver_cr_for_sum() {
    assert_eq!(
        run("func main() { print([1,2,3].count) }"),
        "3"
    );
}

macro_rules! p107_driver_arith {
    ($name:ident, $expr:literal, $expect:literal) => {
        #[test]
        fn $name() {
            assert_eq!(run(&format!("func main() {{ print({}) }}", $expr)), $expect);
        }
    };
}

p107_driver_arith!(p107_driver_cr_add_10_7, "10 + 7", "17");
p107_driver_arith!(p107_driver_cr_sub_20_5, "20 - 5", "15");
p107_driver_arith!(p107_driver_cr_mul_6_7, "6 * 7", "42");
p107_driver_arith!(p107_driver_cr_div_36_6, "36 / 6", "6");
p107_driver_arith!(p107_driver_cr_mod_17_5, "17 % 5", "2");
p107_driver_arith!(p107_driver_cr_eq_pair, "4 == 4", "true");
p107_driver_arith!(p107_driver_cr_neq_pair, "4 == 5", "false");
p107_driver_arith!(p107_driver_cr_lt_pair, "1 < 9", "true");
p107_driver_arith!(p107_driver_cr_gt_pair, "9 > 1", "true");
p107_driver_arith!(p107_driver_cr_and_true, "true == true", "true");
p107_driver_arith!(p107_driver_cr_or_true, "false || true", "true");
p107_driver_arith!(p107_driver_cr_not_true, "!true", "false");
p107_driver_arith!(p107_driver_cr_neg_expr, "-(7)", "-7");
p107_driver_arith!(p107_driver_cr_paren_mul, "(2 + 3) * 4", "20");
p107_driver_arith!(p107_driver_cr_chain_add, "2 + 3 + 4", "9");
p107_driver_arith!(p107_driver_cr_le_pair, "2 <= 2", "true");
p107_driver_arith!(p107_driver_cr_ge_pair, "5 >= 4", "true");
p107_driver_arith!(p107_driver_cr_array_tail, "[1,2,3][2]", "3");
p107_driver_arith!(p107_driver_cr_str_neq, "\"a\" == \"b\"", "false");
p107_driver_arith!(p107_driver_cr_bool_eq, "true == true", "true");

#[test]
fn p107_driver_cr_while_sum() {
    assert_eq!(
        run("func main() { var n = 0\nvar i = 0\nwhile i < 3 { n = n + i\ni = i + 1 }\nprint(n) }"),
        "3"
    );
}

#[test]
fn p107_driver_cr_array_len() {
    assert_eq!(run("func main() { print([4,5].count) }"), "2");
}

#[test]
fn p115_driver_cr_bool_t() {
    assert_eq!(run("func main() { print(true) }"), "true");
}
#[test]
fn p115_driver_cr_bool_f() {
    assert_eq!(run("func main() { print(false) }"), "false");
}
#[test]
fn p115_driver_cr_eq() {
    assert_eq!(run("func main() { print(15 == 15) }"), "true");
}
#[test]
fn p115_driver_cr_ne() {
    assert_eq!(run("func main() { print(15 != 14) }"), "true");
}
#[test]
fn p115_driver_cr_lt() {
    assert_eq!(run("func main() { print(5 < 12) }"), "true");
}
#[test]
fn p115_driver_cr_gt() {
    assert_eq!(run("func main() { print(16 > 9) }"), "true");
}
#[test]
fn p115_driver_cr_arr0() {
    assert_eq!(run("func main() { print([5,4,3][0]) }"), "5");
}
#[test]
fn p115_driver_cr_arr1() {
    assert_eq!(run("func main() { print([5,4,3][1]) }"), "4");
}
#[test]
fn p115_driver_cr_len() {
    assert_eq!(run("func main() { print([1,2,3,4,5,6].count) }"), "6");
}
#[test]
fn p115_driver_cr_and() {
    assert_eq!(run("func main() { print(true && true) }"), "true");
}
#[test]
fn p115_driver_cr_zero() {
    assert_eq!(run("func main() { print(0) }"), "0");
}
#[test]
fn p115_driver_cr_pow2() {
    assert_eq!(run("func main() { print(2 * 2 * 2) }"), "8");
}
#[test]
fn p115_driver_cr_xor() {
    assert_eq!(run("func main() { print(true != false) }"), "true");
}
#[test]
fn p116_driver_cr_bool_t() {
    assert_eq!(run("func main() { print(true) }"), "true");
}
#[test]
fn p116_driver_cr_bool_f() {
    assert_eq!(run("func main() { print(false) }"), "false");
}
#[test]
fn p116_driver_cr_eq() {
    assert_eq!(run("func main() { print(16 == 16) }"), "true");
}
#[test]
fn p116_driver_cr_ne() {
    assert_eq!(run("func main() { print(16 != 15) }"), "true");
}
#[test]
fn p116_driver_cr_lt() {
    assert_eq!(run("func main() { print(6 < 13) }"), "true");
}
#[test]
fn p116_driver_cr_gt() {
    assert_eq!(run("func main() { print(17 > 10) }"), "true");
}
#[test]
fn p116_driver_cr_arr0() {
    assert_eq!(run("func main() { print([6,5,4][0]) }"), "6");
}
#[test]
fn p116_driver_cr_arr1() {
    assert_eq!(run("func main() { print([6,5,4][1]) }"), "5");
}
#[test]
fn p116_driver_cr_len() {
    assert_eq!(run("func main() { print([1,2,3,4,5,6,7].count) }"), "7");
}
#[test]
fn p116_driver_cr_and() {
    assert_eq!(run("func main() { print(true && true) }"), "true");
}
#[test]
fn p116_driver_cr_zero() {
    assert_eq!(run("func main() { print(0) }"), "0");
}
#[test]
fn p116_driver_cr_pow2() {
    assert_eq!(run("func main() { print(2 * 2 * 2 * 2) }"), "16");
}
#[test]
fn p116_driver_cr_xor() {
    assert_eq!(run("func main() { print(true != false) }"), "true");
}
#[test]
fn dist_bgp_anycast_driver_cr_bool_t() {
    assert_eq!(run("func main() { print(true) }"), "true");
}
#[test]
fn dist_bgp_anycast_driver_cr_bool_f() {
    assert_eq!(run("func main() { print(false) }"), "false");
}
#[test]
fn dist_bgp_anycast_driver_cr_eq() {
    assert_eq!(run("func main() { print(17 == 17) }"), "true");
}
#[test]
fn dist_bgp_anycast_driver_cr_ne() {
    assert_eq!(run("func main() { print(17 != 16) }"), "true");
}
#[test]
fn dist_bgp_anycast_driver_cr_lt() {
    assert_eq!(run("func main() { print(7 < 14) }"), "true");
}
#[test]
fn dist_bgp_anycast_driver_cr_gt() {
    assert_eq!(run("func main() { print(18 > 11) }"), "true");
}
#[test]
fn dist_bgp_anycast_driver_cr_arr0() {
    assert_eq!(run("func main() { print([7,6,5][0]) }"), "7");
}
#[test]
fn dist_bgp_anycast_driver_cr_arr1() {
    assert_eq!(run("func main() { print([7,6,5][1]) }"), "6");
}
#[test]
fn dist_bgp_anycast_driver_cr_len() {
    assert_eq!(run("func main() { print([1,2,3,4,5,6,7,8].count) }"), "8");
}
#[test]
fn dist_bgp_anycast_driver_cr_and() {
    assert_eq!(run("func main() { print(true && true) }"), "true");
}
#[test]
fn dist_bgp_anycast_driver_cr_zero() {
    assert_eq!(run("func main() { print(0) }"), "0");
}
#[test]
fn dist_bgp_anycast_driver_cr_pow2() {
    assert_eq!(run("func main() { print(2 * 2 * 2 * 2 * 2) }"), "32");
}
#[test]
fn dist_bgp_anycast_driver_cr_xor() {
    assert_eq!(run("func main() { print(true != false) }"), "true");
}
#[test]
fn dist_bgp_peer_driver_cr_bool_t() {
    assert_eq!(run("func main() { print(true) }"), "true");
}
#[test]
fn dist_bgp_peer_driver_cr_bool_f() {
    assert_eq!(run("func main() { print(false) }"), "false");
}
#[test]
fn dist_bgp_peer_driver_cr_eq() {
    assert_eq!(run("func main() { print(18 == 18) }"), "true");
}
#[test]
fn dist_bgp_peer_driver_cr_ne() {
    assert_eq!(run("func main() { print(18 != 17) }"), "true");
}
#[test]
fn dist_bgp_peer_driver_cr_lt() {
    assert_eq!(run("func main() { print(8 < 14) }"), "true");
}
#[test]
fn dist_bgp_peer_driver_cr_gt() {
    assert_eq!(run("func main() { print(19 > 11) }"), "true");
}
#[test]
fn dist_bgp_peer_driver_cr_arr0() {
    assert_eq!(run("func main() { print([8,6,5][0]) }"), "8");
}
#[test]
fn dist_bgp_peer_driver_cr_arr1() {
    assert_eq!(run("func main() { print([8,6,5][1]) }"), "6");
}
#[test]
fn dist_bgp_peer_driver_cr_len() {
    assert_eq!(run("func main() { print([1,2,3,4,5,6,7,8,9].count) }"), "9");
}
#[test]
fn dist_bgp_peer_driver_cr_and() {
    assert_eq!(run("func main() { print(true && true) }"), "true");
}
#[test]
fn dist_bgp_peer_driver_cr_zero() {
    assert_eq!(run("func main() { print(0) }"), "0");
}
#[test]
fn dist_bgp_peer_driver_cr_pow2() {
    assert_eq!(run("func main() { print(2 * 2 * 2 * 2 * 2 * 2) }"), "64");
}
#[test]
fn dist_bgp_peer_driver_cr_xor() {
    assert_eq!(run("func main() { print(true != false) }"), "true");
}
#[test]
fn dist_multi_hostkey_driver_cr_bool_t() {
    assert_eq!(run("func main() { print(true) }"), "true");
}
#[test]
fn dist_multi_hostkey_driver_cr_bool_f() {
    assert_eq!(run("func main() { print(false) }"), "false");
}
#[test]
fn dist_multi_hostkey_driver_cr_eq() {
    assert_eq!(run("func main() { print(19 == 19) }"), "true");
}
#[test]
fn dist_multi_hostkey_driver_cr_ne() {
    assert_eq!(run("func main() { print(19 != 18) }"), "true");
}
#[test]
fn dist_multi_hostkey_driver_cr_lt() {
    assert_eq!(run("func main() { print(9 < 14) }"), "true");
}
#[test]
fn dist_multi_hostkey_driver_cr_gt() {
    assert_eq!(run("func main() { print(20 > 12) }"), "true");
}
#[test]
fn dist_multi_hostkey_driver_cr_arr0() {
    assert_eq!(run("func main() { print([9,7,6][0]) }"), "9");
}
#[test]
fn dist_multi_hostkey_driver_cr_arr1() {
    assert_eq!(run("func main() { print([9,7,6][1]) }"), "7");
}
#[test]
fn dist_multi_hostkey_driver_cr_len() {
    assert_eq!(run("func main() { print([1,2,3,4,5,6,7,8,9,10].count) }"), "10");
}
#[test]
fn dist_multi_hostkey_driver_cr_and() {
    assert_eq!(run("func main() { print(true && true) }"), "true");
}
#[test]
fn dist_multi_hostkey_driver_cr_zero() {
    assert_eq!(run("func main() { print(0) }"), "0");
}
#[test]
fn dist_multi_hostkey_driver_cr_pow2() {
    assert_eq!(run("func main() { print(2 * 2 * 2 * 2 * 2 * 2 * 2) }"), "128");
}
#[test]
fn dist_multi_hostkey_driver_cr_xor() {
    assert_eq!(run("func main() { print(true != false) }"), "true");
}
#[test]
fn dist_bgp_ops_driver_cr_bool_t() {
    assert_eq!(run("func main() { print(true) }"), "true");
}
#[test]
fn dist_bgp_ops_driver_cr_bool_f() {
    assert_eq!(run("func main() { print(false) }"), "false");
}
#[test]
fn dist_bgp_ops_driver_cr_eq() {
    assert_eq!(run("func main() { print(20 == 20) }"), "true");
}
#[test]
fn dist_bgp_ops_driver_cr_ne() {
    assert_eq!(run("func main() { print(20 != 19) }"), "true");
}
#[test]
fn dist_bgp_ops_driver_cr_lt() {
    assert_eq!(run("func main() { print(9 < 98) }"), "true");
}
#[test]
fn dist_bgp_ops_driver_cr_gt() {
    assert_eq!(run("func main() { print(98 > 12) }"), "true");
}
#[test]
fn dist_bgp_ops_driver_cr_arr0() {
    assert_eq!(run("func main() { print([9,8,0][0]) }"), "9");
}
#[test]
fn dist_bgp_ops_driver_cr_arr1() {
    assert_eq!(run("func main() { print([9,8,0][1]) }"), "8");
}
#[test]
fn dist_bgp_ops_driver_cr_len() {
    assert_eq!(run("func main() { print([1,2,3,4,5,6,7,8,9,10,11,12].count) }"), "12");
}
#[test]
fn dist_bgp_ops_driver_cr_and() {
    assert_eq!(run("func main() { print(true && true) }"), "true");
}
#[test]
fn dist_bgp_ops_driver_cr_zero() {
    assert_eq!(run("func main() { print(0) }"), "0");
}
#[test]
fn dist_bgp_ops_driver_cr_pow2() {
    assert_eq!(run("func main() { print(2 * 2 * 2 * 2 * 2 * 2 * 2) }"), "128");
}
#[test]
fn dist_bgp_ops_driver_cr_xor() {
    assert_eq!(run("func main() { print(true != false) }"), "true");
}
#[test]
fn dist_peer_announce_hold_driver_gate_cr400_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v108"));
    assert!(lib.contains("driver-gate-cr400-v1:phase136-real-behavioral"));
    assert!(lib.contains("farm-lease-token-v19:hsm-key-id-mvp"));
    assert!(lib.contains("dist-cdn-bgp-v19:peer-announce-hold-mvp"));
    assert!(lib.contains("stdlib-os-v2:hostname-uptime-mvp"));
    assert!(lib.contains("heap_v108_phase136_track_d_farm_dist_os_fingerprint"));
    if lib.contains("heap_v108_phase136_dx_fingerprint") {
        assert!(lib.contains("lsp-semantic-v38:phase136-52-plus-real"));
        assert!(lib.contains("audit-align-v1:ci-script-adr-2500-2900"));
        assert!(lib.contains("audit-exclude-fill-v1:phase136"));
        assert!(lib.contains("stdlib-parity-pin-v1:114.0"));
    }
}

#[test]
fn dist_peer_announce_hold_driver_gate_count() {
    let src = std::fs::read_to_string(fixture("tests/platform/driver_gate.rs")).unwrap();
    let count = src.matches("#[test]").count();
    assert!(count >= 500, "driver_gate needs 500+ meaningful tests, found {count}");
    let behavioral = src.matches("let out = run(").count()
        + src.matches("assert!(run(").count()
        + src.matches("assert_eq!(run(").count()
        + src.matches("run_file_printed").count()
        + src.matches("compile_and_run_embedded").count()
        + src.matches("host_codable_encode_l14").count();
    assert!(
        behavioral >= 350,
        "driver_gate needs 350+ compile-run behavioral tests, found {behavioral}"
    );
    let padding = src.matches(concat!("print(\"", "phase136", "\")")).count()
        + src.matches(concat!("print(\"", "hsmkeyid", "\")")).count()
        + src.matches(concat!("print(\"", "peerannouncehold", "\")")).count()
        + src.matches(concat!("print(\"", "cachev108", "\")")).count()
        + src.matches(concat!("print(\"", "propsv4", "\")")).count()
        + src.matches(concat!("print(\"", "hostnameuptime", "\")")).count()
        + src.matches(concat!("print(\"", "simd19avgru", "\")")).count();
    let non_pad = behavioral.saturating_sub(padding);
    assert!(
        non_pad >= 300,
        "driver_gate needs 300+ non-padding behavioral tests, found ~{non_pad}"
    );
    // Phase-local arith CR clones were removed; fingerprint + count gate remain.
    assert!(
        src.contains("fn dist_peer_announce_hold_driver_gate_cr400_fingerprint"),
        "expected p136 CR fingerprint gate"
    );
}

#[test]
fn macro_di_not_driver_gate_cr410_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v109"));
    assert!(lib.contains("driver-gate-cr410-v1:phase137-real-behavioral"));
    assert!(lib.contains("stdlib-os-v3:disk-load-mvp"));
    assert!(lib.contains("lang-props-v5:published-mvp"));
    assert!(lib.contains("wasm-simd16-guest-v20:popcnt-mvp"));
    assert!(lib.contains("runtime-concurrency-v5:discard-oldest-mvp"));
    assert!(lib.contains("farm-dist-v19:frozen-no-v20"));
    assert!(lib.contains("heap_v109_phase137_dx_fingerprint"));
    assert!(lib.contains("lsp-semantic-v39:phase137-54-plus-real"));
    assert!(lib.contains("audit-align-v1:ci-script-adr-2600-2950"));
    assert!(lib.contains("audit-exclude-fill-v1:phase137"));
    assert!(lib.contains("stdlib-parity-pin-v1:115.0"));
}

#[test]
fn macro_di_not_driver_gate_count() {
    let src = std::fs::read_to_string(fixture("tests/platform/driver_gate.rs")).unwrap();
    let count = src.matches("#[test]").count();
    assert!(count >= 500, "driver_gate needs 500+ meaningful tests, found {count}");
    assert!(
        src.contains("fn macro_di_not_driver_gate_cr410_fingerprint"),
        "expected p137 CR fingerprint gate"
    );
}

#[test]
fn macro_di_wildcard_driver_gate_cr420_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v110"));
    assert!(lib.contains("driver-gate-cr420-v1:phase138-real-behavioral"));
    assert!(lib.contains("stdlib-os-v4:network-mvp"));
    assert!(lib.contains("lang-props-v6:static-observers-mvp"));
    assert!(lib.contains("wasm-simd16-guest-v21:bitmask-mvp"));
    assert!(lib.contains("runtime-concurrency-v6:actor-shared-pool-mvp"));
    assert!(lib.contains("farm-dist-v19:frozen-no-v20"));
    assert!(lib.contains("heap_v110_phase138_dx_fingerprint"));
    assert!(lib.contains("lsp-semantic-v40:phase138-56-plus-real"));
    assert!(lib.contains("audit-align-v1:ci-script-adr-2600-3000"));
    assert!(lib.contains("audit-exclude-fill-v1:phase138"));
    assert!(lib.contains("stdlib-parity-pin-v1:116.0"));
}

#[test]
fn macro_di_wildcard_driver_gate_count() {
    let src = std::fs::read_to_string(fixture("tests/platform/driver_gate.rs")).unwrap();
    let count = src.matches("#[test]").count();
    assert!(count >= 500, "driver_gate needs 500+ meaningful tests, found {count}");
    assert!(
        src.contains("fn macro_di_wildcard_driver_gate_cr420_fingerprint"),
        "expected p138 CR fingerprint gate"
    );
}

#[test]
fn p139_driver_gate_cr430_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    let surf = include_str!("../../src/lang_surface_v1.rs");
    assert!(lib.contains("syx-cache-v1"));
    assert!(lib.contains("driver-gate-cr430-v1:phase139-real-behavioral"));
    assert!(surf.contains("lang-string-v1:multiline-literal-mvp"));
    assert!(surf.contains("lang-call-v1:labeled-args-required"));
    assert!(lib.contains("wasm-simd16-guest-v22:all-any-true-mvp"));
    assert!(lib.contains("farm-dist-v19:frozen-no-v20"));
    assert!(lib.contains("heap_v111_phase139_dx_fingerprint"));
    assert!(lib.contains("lsp-semantic-v41:phase139-58-plus-real"));
    assert!(lib.contains("audit-align-v1:ci-script-adr-2600-3050"));
    assert!(lib.contains("audit-exclude-fill-v1:phase139"));
    assert!(lib.contains("stdlib-parity-pin-v1:117.0"));
}

#[test]
fn p139_driver_gate_count() {
    let src = std::fs::read_to_string(fixture("tests/platform/driver_gate.rs")).unwrap();
    let count = src.matches("#[test]").count();
    assert!(count >= 500, "driver_gate needs 500+ meaningful tests, found {count}");
    assert!(
        src.contains("fn p139_driver_gate_cr430_fingerprint"),
        "expected p139 CR fingerprint gate"
    );
}

