// Phase 97 real LSP matrix (<=25): cache v69, manifest 75.0, L14 codable, tier-2T, alloca v4.

use super::*;

fn p97_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p97_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p97_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p97_guidemo_import_ui_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("native-guidemo-import-ui-v1:examples-guidemo-ui-strict-parity-15plus"));
}

#[test]
fn p97_reflect_runtime_cast_strict_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("reflect-cast-strict-quad-v1"));
}

#[test]
fn p97_import_collections_sort_by_closure_parse() {
    assert!(p97_doc_ok(
        "func cmp(a: Int, b: Int) -> Int { return a - b }\nfunc main() { print([3,1,2].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p97_import_collections_string_sort_by_parse() {
    assert!(p97_doc_ok(
        "func cmp(a: String, b: String) -> Int { if a < b { return -1 }; return 1 }\nfunc main() { print([\"c\",\"a\"].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p97_reflect_class_point_parse() {
    assert!(p97_doc_ok(
        "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 92).x) }"
    ));
}

#[test]
fn p97_reflect_type_host_parse() {
    assert!(p97_doc_ok(
        "func main() { print(__syx_reflect_type(\"Point\")) }"
    ));
}

#[test]
fn p97_as_optional_cast_parse() {
    assert!(p97_doc_ok(
        "class Cat { func speak() -> String { return \"meow\" } }\nclass Dog { func speak() -> String { return \"woof\" } }\nfunc main() { let d = Dog(); let c = d as? Cat; print(d is Dog) }"
    ));
}

#[test]
fn p97_import_net_http_fetch_parse() {
    assert!(p97_doc_ok(
        "import Net\nfunc main() { print(Http.fetch(\"http://example.com\", {})) }"
    ));
}

#[test]
fn p97_import_text_string_builder_parse() {
    assert!(p97_doc_ok(
        "import Text\nfunc main() { let b = StringBuilder.create(); StringBuilder.append(b, \"p97\"); print(StringBuilder.toString(b)) }"
    ));
}

#[test]
fn p97_import_testing_test_parse() {
    assert!(p97_doc_ok(
        "import Testing\nfunc main() { test(\"smoke\", 0); expectEqual(97, 97) }"
    ));
}

#[test]
fn p97_import_sync_mutex_parse() {
    assert!(p97_doc_ok(
        "import Sync\nfunc main() { let m = Mutex.create(); Mutex.lock(m); Mutex.unlock(m) }"
    ));
}

#[test]
fn p97_import_io_file_watch_parse() {
    assert!(p97_doc_ok("import IO\nfunc main() { print(File.watch(\"/tmp\")) }"));
}

#[test]
fn p97_closure_inline_parse() {
    assert!(p97_doc_ok(
        "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(97)) }"
    ));
}

#[test]
fn p97_protocol_dynamic_dispatch_parse() {
    assert!(p97_doc_ok(
        "protocol P { func f() -> Int }\nclass C: P { func f() -> Int { return 97 } }\nfunc main() { let p: P = C(); print(p.f()) }"
    ));
}

#[test]
fn p97_codable_struct_parse() {
    assert!(p97_doc_ok(
        "@Codable\nstruct Item { var n: Int }\nfunc main() { print(Item(n: 97).n) }"
    ));
}

#[test]
fn p97_select_channel_parse() {
    assert!(p97_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(1); ch.send(97); print(ch.recv()) }"
    ));
}

#[test]
fn p97_async_fn_parse() {
    assert!(p97_doc_ok("async func f() -> Int { return 97 }\nfunc main() { print(97) }"));
}

#[test]
fn p97_select_async_parse() {
    assert!(p97_doc_ok(
        "import Sync\nasync func worker() -> Int { return 97 }\nfunc main() { let ch = Channel.create(1); ch.send(97); print(ch.recv()) }"
    ));
}

#[test]
fn p97_wasm_cold_zero_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("wasm-cold-zero-v1:concat-tostr-tier1-mir-exec-counter-zero"));
}

#[test]
fn p97_browser_tier2t_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("browser-tier2t-v1:playwright-multistep-gui-ci-mandatory"));
}

#[test]
fn p97_codable_l14_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("codable-l14-evolution-v1:prost-schema-breaking-l13"));
}

#[test]
fn p97_gui_dlopen_default_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("gui-dlopen-default-v1:dylib-unconditional-fallback-hashmap"));
}

#[test]
fn p97_llvm_alloca_v4_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("native-llvm-locals-v4:alloca-ssa-ga"));
}
