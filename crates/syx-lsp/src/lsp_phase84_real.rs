// Phase 84 real LSP matrix (<=25): stdlib production, grapheme, reflect, cache v56, manifest 62.0.

use super::*;

fn p84_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p84_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p84_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p84_stdlib_v56_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("heap_v56_phase84_fingerprint"));
}

#[test]
fn p84_import_text_string_builder_parse() {
    assert!(p84_doc_ok(
        "import Text\nfunc main() { let b = StringBuilder.create(); StringBuilder.append(b, \"p84\"); print(StringBuilder.toString(b)) }"
    ));
}

#[test]
fn p84_import_text_bytes_parse() {
    assert!(p84_doc_ok(
        "import Text\nfunc main() { let b = Bytes.fromArray([1, 2]); print(b.count) }"
    ));
}

#[test]
fn p84_import_text_regex_compile_parse() {
    assert!(p84_doc_ok(
        "import Text\nfunc main() { print(Regex.compile(\"a+\")) }"
    ));
}

#[test]
fn p84_import_io_file_watch_parse() {
    assert!(p84_doc_ok(
        "import IO\nfunc main() { print(File.watch(\"/tmp\").count) }"
    ));
}

#[test]
fn p84_import_process_spawn_wait_parse() {
    assert!(p84_doc_ok(
        "import Process\nfunc main() { let h = Process.spawn(\"/bin/true\", []); print(Process.wait(h)) }"
    ));
}

#[test]
fn p84_import_process_exec_output_parse() {
    assert!(p84_doc_ok(
        "import Process\nfunc main() { print(Process.execOutput(\"/bin/echo\", [\"p84\"])) }"
    ));
}

#[test]
fn p84_import_testing_test_parse() {
    assert!(p84_doc_ok(
        "import Testing\nfunc main() { test(\"smoke\", 0); expectEqual(84, 84) }"
    ));
}

#[test]
fn p84_import_sync_channel_close_parse() {
    assert!(p84_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(1); ch.close(); print(1) }"
    ));
}

#[test]
fn p84_import_datetime_format_with_timezone_parse() {
    assert!(p84_doc_ok(
        "import DateTime\nfunc main() { print(DateTime.formatWithTimezone(0, \"iso\", \"UTC\")) }"
    ));
}

#[test]
fn p84_grapheme_emoji_length_parse() {
    assert!(p84_doc_ok(
        "func main() { print(String.count(\"👨‍👩‍👧\")) }"
    ));
}

#[test]
fn p84_grapheme_emoji_char_at_parse() {
    assert!(p84_doc_ok(
        "func main() { print(\"👨‍👩‍👧\".charAt(0)) }"
    ));
}

#[test]
fn p84_import_sync_mutex_parse() {
    assert!(p84_doc_ok(
        "import Sync\nfunc main() { let m = Mutex.create(); Mutex.lock(m); Mutex.unlock(m) }"
    ));
}

#[test]
fn p84_import_text_encoding_parse() {
    assert!(p84_doc_ok(
        "import Text\nfunc main() { print(Encoding.base64Encode(\"x\")) }"
    ));
}

#[test]
fn p84_import_sync_channel_send_parse() {
    assert!(p84_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(2); ch.send(84); print(ch.recv()) }"
    ));
}

#[test]
fn p84_prelude_fmt_parse() {
    assert!(p84_doc_ok("func main() { Fmt.printLine(Fmt.format(\"p84-{0}\", [84])) }"));
}

#[test]
fn p84_tcp_socket_write_parse() {
    assert!(p84_doc_ok(
        "import Net\nfunc main() { print(TcpSocket.write(\"data\")) }"
    ));
}

#[test]
fn p84_async_fn_parse() {
    assert!(p84_doc_ok("async func f() -> Int { return 84 }\nfunc main() { print(84) }"));
}

#[test]
fn p84_actor_parse() {
    assert!(p84_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p84_generic_fn_parse() {
    assert!(p84_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(84)) }"));
}

#[test]
fn p84_class_init_parse() {
    assert!(p84_doc_ok(
        "class Box { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { let b = Box(n: 84); print(b.n) }"
    ));
}

#[test]
fn p84_import_log_parse() {
    assert!(p84_doc_ok("import Log\nfunc main() { Log.info(\"p84\") }"));
}

#[test]
fn p84_import_net_http_parse() {
    assert!(p84_doc_ok("import Net\nfunc main() { print(Http.get(\"http://example.com\")) }"));
}
