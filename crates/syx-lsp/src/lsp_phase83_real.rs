// Phase 83 real LSP matrix (<=25): stdlib GA modules, cache v55, manifest 61.0.

use super::*;

fn p83_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p83_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p83_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p83_stdlib_v1_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("heap_v55_phase83_fingerprint"));
}

#[test]
fn p83_import_text_parse() {
    assert!(p83_doc_ok(
        "import Text\nfunc main() { let s = Fmt.format(\"x {0}\", [1]); print(s) }"
    ));
}

#[test]
fn p83_import_datetime_parse() {
    assert!(p83_doc_ok(
        "import DateTime\nfunc main() { let dt = DateTime.parse(\"2024-01-01\"); print(dt) }"
    ));
}

#[test]
fn p83_import_sync_parse() {
    assert!(p83_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(1); ch.send(1); print(ch.recv()) }"
    ));
}

#[test]
fn p83_import_testing_parse() {
    assert!(p83_doc_ok(
        "import Testing\nfunc main() { assert(true); assertEq(1, 1) }"
    ));
}

#[test]
fn p83_import_log_parse() {
    assert!(p83_doc_ok("import Log\nfunc main() { Log.info(\"p83\") }"));
}

#[test]
fn p83_import_process_parse() {
    assert!(p83_doc_ok(
        "import Process\nfunc main() { print(Process.env(\"PATH\").count > 0) }"
    ));
}

#[test]
fn p83_prelude_fmt_parse() {
    assert!(p83_doc_ok("func main() { Fmt.printLine(Fmt.format(\"p83-{0}\", [83])) }"));
}

#[test]
fn p83_string_substring_parse() {
    assert!(p83_doc_ok(
        "func main() { let s = \"hello\"; print(s.substring(0, 2)) }"
    ));
}

#[test]
fn p83_math_float_sin_parse() {
    assert!(p83_doc_ok("func main() { print(Math.sin(1.0)) }"));
}

#[test]
fn p83_encoding_base64_parse() {
    assert!(p83_doc_ok(
        "import Text\nfunc main() { print(Encoding.base64Encode(\"syx\")) }"
    ));
}

#[test]
fn p83_tcp_socket_parse() {
    assert!(p83_doc_ok(
        "import Net\nfunc main() { print(TcpSocket.connect(\"127.0.0.1\", 9)) }"
    ));
}

#[test]
fn p83_channel_select_parse() {
    assert!(p83_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(0); print(Select.select([ch])) }"
    ));
}

#[test]
fn p83_datetime_iso8601_parse() {
    assert!(p83_doc_ok(
        "import DateTime\nfunc main() { print(DateTime.parseIso8601(\"2024-01-01T00:00:00Z\")) }"
    ));
}

#[test]
fn p83_set_insert_parse() {
    assert!(p83_doc_ok(
        "func main() { let s: Set<Int> = Set.from([1]); print(s.insert(2).count) }"
    ));
}

#[test]
fn p83_dict_values_parse() {
    assert!(p83_doc_ok(
        "func main() { let d: Dictionary<String, Int> = {\"a\": 1}; print(d.values().count) }"
    ));
}

#[test]
fn p83_time_parse_timestamp_parse() {
    assert!(p83_doc_ok(
        "func main() { print(Time.parseTimestampMs(\"2024-06-01\")) }"
    ));
}

#[test]
fn p83_async_fn_parse() {
    assert!(p83_doc_ok("async func f() -> Int { return 83 }\nfunc main() { print(83) }"));
}

#[test]
fn p83_actor_parse() {
    assert!(p83_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p83_generic_fn_parse() {
    assert!(p83_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(83)) }"));
}

#[test]
fn p83_class_init_parse() {
    assert!(p83_doc_ok(
        "class Box { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { let b = Box(n: 83); print(b.n) }"
    ));
}

#[test]
fn p83_optional_chain_parse() {
    assert!(p83_doc_ok("func main() { let x: Int? = 83; print(x) }"));
}

#[test]
fn p83_import_net_http_parse() {
    assert!(p83_doc_ok("import Net\nfunc main() { print(Http.get(\"http://example.com\")) }"));
}
