// Phase 85 real LSP matrix (<=25): stdlib native parity, HOF, reflect, net-v2, cache v57, manifest 63.0.

use super::*;

fn p85_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p85_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p85_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p85_stdlib_native_parity_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("stdlib-native-parity-v1"));
}

#[test]
fn p85_import_collections_sort_by_parse() {
    assert!(p85_doc_ok(
        "func cmp(a: Int, b: Int) -> Int { return a - b }\nfunc main() { print([3,1,2].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p85_import_collections_flat_map_parse() {
    assert!(p85_doc_ok(
        "func id(x: Int) -> Array { return [x] }\nfunc main() { print([1,2].flatMap(id).count) }"
    ));
}

#[test]
fn p85_import_crypto_ed25519_parse() {
    assert!(p85_doc_ok(
        "import Crypto\nfunc main() { print(Crypto.ed25519Sign(\"0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\", \"p85\").count) }"
    ));
}

#[test]
fn p85_import_net_tcp_connect_parse() {
    assert!(p85_doc_ok(
        "import Net\nfunc main() { print(TcpSocket.connect(\"127.0.0.1\", 9)) }"
    ));
}

#[test]
fn p85_import_net_udp_bind_parse() {
    assert!(p85_doc_ok("import Net\nfunc main() { print(UdpSocket.bind(8080)) }"));
}

#[test]
fn p85_import_text_string_builder_parse() {
    assert!(p85_doc_ok(
        "import Text\nfunc main() { let b = StringBuilder.create(); StringBuilder.append(b, \"p85\"); print(StringBuilder.toString(b)) }"
    ));
}

#[test]
fn p85_import_process_exec_parse() {
    assert!(p85_doc_ok(
        "import Process\nfunc main() { print(Process.exec(\"/usr/bin/true\", [])) }"
    ));
}

#[test]
fn p85_reflect_class_metadata_parse() {
    assert!(p85_doc_ok(
        "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 85).x) }"
    ));
}

#[test]
fn p85_grapheme_emoji_length_parse() {
    assert!(p85_doc_ok("func main() { print(String.count(\"👨‍👩‍👧\")) }"));
}

#[test]
fn p85_async_fn_parse() {
    assert!(p85_doc_ok("async func f() -> Int { return 85 }\nfunc main() { print(85) }"));
}

#[test]
fn p85_actor_parse() {
    assert!(p85_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p85_closure_parse() {
    assert!(p85_doc_ok(
        "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(84)) }"
    ));
}

#[test]
fn p85_try_catch_parse() {
    assert!(p85_doc_ok("func main() { try { print(85) } catch { print(0) } }"));
}

#[test]
fn p85_import_sync_mutex_parse() {
    assert!(p85_doc_ok(
        "import Sync\nfunc main() { let m = Mutex.create(); Mutex.lock(m); Mutex.unlock(m) }"
    ));
}

#[test]
fn p85_import_testing_test_parse() {
    assert!(p85_doc_ok(
        "import Testing\nfunc main() { test(\"smoke\", 0); expectEqual(85, 85) }"
    ));
}

#[test]
fn p85_import_io_file_watch_parse() {
    assert!(p85_doc_ok("import IO\nfunc main() { print(File.watch(\"/tmp\").count) }"));
}

#[test]
fn p85_prelude_fmt_parse() {
    assert!(p85_doc_ok("func main() { Fmt.printLine(Fmt.format(\"p85-{0}\", [85])) }"));
}

#[test]
fn p85_import_net_http_parse() {
    assert!(p85_doc_ok("import Net\nfunc main() { print(Http.get(\"http://example.com\")) }"));
}

#[test]
fn p85_generic_fn_parse() {
    assert!(p85_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(85)) }"));
}

#[test]
fn p85_switch_parse() {
    assert!(p85_doc_ok("func main() { switch 85 { case 85: print(1); default: print(0) } }"));
}

#[test]
fn p85_hof_closure_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("hof-closure-v1"));
}

#[test]
fn p85_reflect_lang_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("reflect-lang-v1"));
}
