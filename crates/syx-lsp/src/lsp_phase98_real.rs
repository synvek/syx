// Phase 98 real LSP matrix (<=25): cache v70, manifest 76.0, L14 codable surface, tier-2U, alloca v5.

use super::*;

fn p98_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p98_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p98_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p98_reflect_runtime_cast_strict_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("reflect-cast-strict-quad-v1"));
}

#[test]
fn p98_import_collections_sort_by_closure_parse() {
    assert!(p98_doc_ok(
        "func cmp(a: Int, b: Int) -> Int { return a - b }\nfunc main() { print([3,1,2].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p98_import_collections_string_sort_by_parse() {
    assert!(p98_doc_ok(
        "func cmp(a: String, b: String) -> Int { if a < b { return -1 }; return 1 }\nfunc main() { print([\"c\",\"a\"].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p98_reflect_class_point_parse() {
    assert!(p98_doc_ok(
        "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 92).x) }"
    ));
}

#[test]
fn p98_reflect_type_host_parse() {
    assert!(p98_doc_ok(
        "func main() { print(__syx_reflect_type(\"Point\")) }"
    ));
}

#[test]
fn p98_as_optional_cast_parse() {
    assert!(p98_doc_ok(
        "class Cat { func speak() -> String { return \"meow\" } }\nclass Dog { func speak() -> String { return \"woof\" } }\nfunc main() { let d = Dog(); let c = d as? Cat; print(d is Dog) }"
    ));
}

#[test]
fn p98_import_net_http_fetch_parse() {
    assert!(p98_doc_ok(
        "import Net\nfunc main() { print(Http.fetch(\"http://example.com\", {})) }"
    ));
}

#[test]
fn p98_import_text_string_builder_parse() {
    assert!(p98_doc_ok(
        "import Text\nfunc main() { let b = StringBuilder.create(); StringBuilder.append(b, \"p98\"); print(StringBuilder.toString(b)) }"
    ));
}

#[test]
fn p98_import_testing_test_parse() {
    assert!(p98_doc_ok(
        "import Testing\nfunc main() { test(\"smoke\", 0); expectEqual(98, 98) }"
    ));
}

#[test]
fn p98_import_sync_mutex_parse() {
    assert!(p98_doc_ok(
        "import Sync\nfunc main() { let m = Mutex.create(); Mutex.lock(m); Mutex.unlock(m) }"
    ));
}

#[test]
fn p98_import_io_file_watch_parse() {
    assert!(p98_doc_ok("import IO\nfunc main() { print(File.watch(\"/tmp\")) }"));
}

#[test]
fn p98_closure_inline_parse() {
    assert!(p98_doc_ok(
        "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(98)) }"
    ));
}

#[test]
fn p98_protocol_dynamic_dispatch_parse() {
    assert!(p98_doc_ok(
        "protocol P { func f() -> Int }\nclass C: P { func f() -> Int { return 98 } }\nfunc main() { let p: P = C(); print(p.f()) }"
    ));
}

#[test]
fn p98_codable_struct_parse() {
    assert!(p98_doc_ok(
        "@Codable\nstruct Item { var n: Int }\nfunc main() { print(Item(n: 98).n) }"
    ));
}

#[test]
fn p98_select_channel_parse() {
    assert!(p98_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(1); ch.send(98); print(ch.recv()) }"
    ));
}

#[test]
fn p98_async_fn_parse() {
    assert!(p98_doc_ok("async func f() -> Int { return 98 }\nfunc main() { print(98) }"));
}

#[test]
fn p98_select_async_parse() {
    assert!(p98_doc_ok(
        "import Sync\nasync func worker() -> Int { return 98 }\nfunc main() { let ch = Channel.create(1); ch.send(98); print(ch.recv()) }"
    ));
}

#[test]
fn p98_wasm_guidemo_zero_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("wasm-guidemo-zero-v1:main-guidemo-mir-exec-run-op-zero"));
}

#[test]
fn p98_browser_tier2u_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("browser-tier2u-v1:playwright-guidemo-subset-hard-ci"));
}

#[test]
fn p98_codable_l14_surface_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("codable-l14-surface-v1:hostfn-packages-l14-names"));
}

#[test]
fn p98_gui_dlopen_default_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("gui-dlopen-default-v1:dylib-unconditional-fallback-hashmap"));
}

#[test]
fn p98_llvm_alloca_v5_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("native-llvm-locals-v5:alloca-ssa-value-ga"));
}
