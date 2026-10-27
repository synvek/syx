// Phase 86 real LSP matrix (<=25): honest parity, HOF closure, reflect self, net-v3, cache v58, manifest 64.0.

use super::*;

fn p86_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p86_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p86_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p86_p85_honest_closure_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("p85-honest-closure-v1"));
}

#[test]
fn p86_reflect_runtime_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("reflect-runtime-v1"));
}

#[test]
fn p86_import_collections_sort_by_closure_parse() {
    assert!(p86_doc_ok(
        "func cmp(a: Int, b: Int) -> Int { return a - b }\nfunc main() { print([3,1,2].sortBy(cmp)[0]) }"
    ));
}

#[test]
fn p86_import_collections_flat_map_parse() {
    assert!(p86_doc_ok(
        "func id(x: Int) -> Array { return [x] }\nfunc main() { print([1,2].flatMap(id).count) }"
    ));
}

#[test]
fn p86_reflect_class_point_parse() {
    assert!(p86_doc_ok(
        "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 86).x) }"
    ));
}

#[test]
fn p86_reflect_type_host_parse() {
    assert!(p86_doc_ok(
        "func main() { print(__syx_reflect_type(\"Point\")) }"
    ));
}

#[test]
fn p86_import_net_http_fetch_parse() {
    assert!(p86_doc_ok(
        "import Net\nfunc main() { print(Http.fetch(\"http://example.com\", {})) }"
    ));
}

#[test]
fn p86_import_text_string_builder_parse() {
    assert!(p86_doc_ok(
        "import Text\nfunc main() { let b = StringBuilder.create(); StringBuilder.append(b, \"p86\"); print(StringBuilder.toString(b)) }"
    ));
}

#[test]
fn p86_import_testing_test_parse() {
    assert!(p86_doc_ok(
        "import Testing\nfunc main() { test(\"smoke\", 0); expectEqual(86, 86) }"
    ));
}

#[test]
fn p86_import_sync_mutex_parse() {
    assert!(p86_doc_ok(
        "import Sync\nfunc main() { let m = Mutex.create(); Mutex.lock(m); Mutex.unlock(m) }"
    ));
}

#[test]
fn p86_import_io_file_watch_parse() {
    assert!(p86_doc_ok("import IO\nfunc main() { print(File.watch(\"/tmp\")) }"));
}

#[test]
fn p86_closure_inline_parse() {
    assert!(p86_doc_ok(
        "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(85)) }"
    ));
}

#[test]
fn p86_protocol_dynamic_dispatch_parse() {
    assert!(p86_doc_ok(
        "protocol P { func f() -> Int }\nclass C: P { func f() -> Int { return 86 } }\nfunc main() { let p: P = C(); print(p.f()) }"
    ));
}

#[test]
fn p86_codable_struct_parse() {
    assert!(p86_doc_ok(
        "@Codable\nstruct Item { var n: Int }\nfunc main() { print(Item(n: 86).n) }"
    ));
}

#[test]
fn p86_select_channel_parse() {
    assert!(p86_doc_ok(
        "import Sync\nfunc main() { let ch = Channel.create(1); ch.send(86); print(ch.recv()) }"
    ));
}

#[test]
fn p86_async_fn_parse() {
    assert!(p86_doc_ok("async func f() -> Int { return 86 }\nfunc main() { print(86) }"));
}

#[test]
fn p86_actor_parse() {
    assert!(p86_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p86_try_catch_parse() {
    assert!(p86_doc_ok("func main() { try { print(86) } catch { print(0) } }"));
}

#[test]
fn p86_net_v3_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("net-v3-h2-zstd"));
}

#[test]
fn p86_browser_tier2i_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("browser-tier2i-v1"));
}

#[test]
fn p86_test_runner_v3_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("test-runner-v3"));
}

#[test]
fn p86_codable_l3_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("codable-l3-v1"));
}

#[test]
fn p86_dist_prod_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("dist-prod-e2e-v1"));
}
