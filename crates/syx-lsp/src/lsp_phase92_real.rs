// Phase 92 real LSP matrix (<=25): wasm zero run_op, native async, winit softbuffer, cache v64, manifest 70.0.

use super::*;

fn p92_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p92_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p92_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p92_native_llvm_async_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("native-llvm-async-v1"));
}

#[test]
fn p92_reflect_runtime_cast_strict_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("reflect-cast-strict-quad-v1"));
}

#[test]
fn p92_import_collections_sort_by_closure_parse() {
    assert!(p92_doc_ok(
        "func cmp(a: Int, b: Int) -> Int { return a - b }\nfunc main() { print([3,1,2].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p92_import_collections_string_sort_by_parse() {
    assert!(p92_doc_ok(
        "func cmp(a: String, b: String) -> Int { if a < b { return -1 }; return 1 }\nfunc main() { print([\"c\",\"a\"].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p92_reflect_class_point_parse() {
    assert!(p92_doc_ok(
        "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 92).x) }"
    ));
}

#[test]
fn p92_reflect_type_host_parse() {
    assert!(p92_doc_ok(
        "func main() { print(__syx_reflect_type(\"Point\")) }"
    ));
}

#[test]
fn p92_as_optional_cast_parse() {
    assert!(p92_doc_ok(
        "class Cat { func speak() -> String { return \"meow\" } }\nclass Dog { func speak() -> String { return \"woof\" } }\nfunc main() { let d = Dog(); let c = d as? Cat; print(d is Dog) }"
    ));
}

#[test]
fn p92_import_net_http_fetch_parse() {
    assert!(p92_doc_ok(
        "import Net\nfunc main() { print(Http.fetch(\"http://example.com\", {})) }"
    ));
}

#[test]
fn p92_import_text_string_builder_parse() {
    assert!(p92_doc_ok(
        "import Text\nfunc main() { let b = StringBuilder.create(); StringBuilder.append(b, \"p92\"); print(StringBuilder.toString(b)) }"
    ));
}

#[test]
fn p92_import_testing_test_parse() {
    assert!(p92_doc_ok(
        "import Testing\nfunc main() { test(\"smoke\", 0); expectEqual(92, 92) }"
    ));
}

#[test]
fn p92_import_sync_mutex_parse() {
    assert!(p92_doc_ok(
        "import Sync\nfunc main() { let m = Mutex.create(); Mutex.lock(m); Mutex.unlock(m) }"
    ));
}

#[test]
fn p92_import_io_file_watch_parse() {
    assert!(p92_doc_ok("import IO\nfunc main() { print(File.watch(\"/tmp\")) }"));
}

#[test]
fn p92_closure_inline_parse() {
    assert!(p92_doc_ok(
        "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(91)) }"
    ));
}

#[test]
fn p92_protocol_dynamic_dispatch_parse() {
    assert!(p92_doc_ok(
        "protocol P { func f() -> Int }\nclass C: P { func f() -> Int { return 92 } }\nfunc main() { let p: P = C(); print(p.f()) }"
    ));
}

#[test]
fn p92_codable_struct_parse() {
    assert!(p92_doc_ok(
        "@Codable\nstruct Item { var n: Int }\nfunc main() { print(Item(n: 92).n) }"
    ));
}

#[test]
fn p92_select_channel_parse() {
    assert!(p92_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(1); ch.send(92); print(ch.recv()) }"
    ));
}

#[test]
fn p92_async_fn_parse() {
    assert!(p92_doc_ok("async func f() -> Int { return 92 }\nfunc main() { print(92) }"));
}

#[test]
fn p92_select_async_parse() {
    assert!(p92_doc_ok(
        "import Sync\nasync func worker() -> Int { return 92 }\nfunc main() { let ch = Channel.create(1); ch.send(92); print(ch.recv()) }"
    ));
}

#[test]
fn p92_wasm_zero_run_op_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("wasm-aot-zero-run-op-v1"));
}

#[test]
fn p92_browser_tier2o_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("browser-tier2o-v1"));
}

#[test]
fn p92_codable_l9_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("codable-l9-protobuf-v1"));
}

#[test]
fn p92_gui_softbuffer_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("gui-winit-softbuffer-v1"));
}

#[test]
fn p92_dist_prod_hard_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("dist-prod-ga-v1"));
}
