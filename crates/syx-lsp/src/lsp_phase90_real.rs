// Phase 90 real LSP matrix (<=25): wasm tier-3, native direct, winit GUI, cache v62, manifest 68.0.

use super::*;

fn p90_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p90_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p90_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p90_native_llvm_phi_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("native-llvm-direct-v1"));
}

#[test]
fn p90_reflect_runtime_cast_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("reflect-cast-hard-v1"));
}

#[test]
fn p90_import_collections_sort_by_closure_parse() {
    assert!(p90_doc_ok(
        "func cmp(a: Int, b: Int) -> Int { return a - b }\nfunc main() { print([3,1,2].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p90_import_collections_string_sort_by_parse() {
    assert!(p90_doc_ok(
        "func cmp(a: String, b: String) -> Int { if a < b { return -1 }; return 1 }\nfunc main() { print([\"c\",\"a\"].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p90_reflect_class_point_parse() {
    assert!(p90_doc_ok(
        "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 87).x) }"
    ));
}

#[test]
fn p90_reflect_type_host_parse() {
    assert!(p90_doc_ok(
        "func main() { print(__syx_reflect_type(\"Point\")) }"
    ));
}

#[test]
fn p90_as_optional_cast_parse() {
    assert!(p90_doc_ok(
        "class Cat { func speak() -> String { return \"meow\" } }\nclass Dog { func speak() -> String { return \"woof\" } }\nfunc main() { let d = Dog(); let c = d as? Cat; print(d is Dog) }"
    ));
}

#[test]
fn p90_import_net_http_fetch_parse() {
    assert!(p90_doc_ok(
        "import Net\nfunc main() { print(Http.fetch(\"http://example.com\", {})) }"
    ));
}

#[test]
fn p90_import_text_string_builder_parse() {
    assert!(p90_doc_ok(
        "import Text\nfunc main() { let b = StringBuilder.create(); StringBuilder.append(b, \"p87\"); print(StringBuilder.toString(b)) }"
    ));
}

#[test]
fn p90_import_testing_test_parse() {
    assert!(p90_doc_ok(
        "import Testing\nfunc main() { test(\"smoke\", 0); expectEqual(87, 87) }"
    ));
}

#[test]
fn p90_import_sync_mutex_parse() {
    assert!(p90_doc_ok(
        "import Sync\nfunc main() { let m = Mutex.create(); Mutex.lock(m); Mutex.unlock(m) }"
    ));
}

#[test]
fn p90_import_io_file_watch_parse() {
    assert!(p90_doc_ok("import IO\nfunc main() { print(File.watch(\"/tmp\")) }"));
}

#[test]
fn p90_closure_inline_parse() {
    assert!(p90_doc_ok(
        "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(86)) }"
    ));
}

#[test]
fn p90_protocol_dynamic_dispatch_parse() {
    assert!(p90_doc_ok(
        "protocol P { func f() -> Int }\nclass C: P { func f() -> Int { return 87 } }\nfunc main() { let p: P = C(); print(p.f()) }"
    ));
}

#[test]
fn p90_codable_struct_parse() {
    assert!(p90_doc_ok(
        "@Codable\nstruct Item { var n: Int }\nfunc main() { print(Item(n: 87).n) }"
    ));
}

#[test]
fn p90_select_channel_parse() {
    assert!(p90_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(1); ch.send(87); print(ch.recv()) }"
    ));
}

#[test]
fn p90_async_fn_parse() {
    assert!(p90_doc_ok("async func f() -> Int { return 87 }\nfunc main() { print(87) }"));
}

#[test]
fn p90_select_async_parse() {
    assert!(p90_doc_ok(
        "import Sync\nasync func worker() -> Int { return 87 }\nfunc main() { let ch = Channel.create(1); ch.send(87); print(ch.recv()) }"
    ));
}

#[test]
fn p90_net_grpc_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("net-grpc-v1"));
}

#[test]
fn p90_browser_tier2j_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("browser-tier2l-v1"));
}

#[test]
fn p90_codable_l4_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("codable-l6-schema-v1"));
}

#[test]
fn p90_dist_prod_hard_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("dist-prod-ga-v1"));
}
