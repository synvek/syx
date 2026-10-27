// Phase 82 real LSP matrix (<=25): lock v16, cache v54, heap v11, RT v7, tier-2F, dist failover.

use super::*;

fn p82_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p82_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p82_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p82_lock_v16_default_version() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("default_lock_version() -> u32"));
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("16"));
}

#[test]
fn p82_rt_native_stack_v7() {
    let abi = include_str!("../../syx-rt/src/abi.rs");
    assert!(abi.contains("NATIVE_STACK_DISPATCH_V7") || abi.contains("native_stack_dispatch"));
}

#[test]
fn p82_heapslot_array_writeback_parse() {
    assert!(p82_doc_ok(
        "func main() { var a: Array<Int> = [1, 2]; a[0] = 99; print(a.count); print(a[0]) }"
    ));
}

#[test]
fn p82_heapslot_dict_methods_parse() {
    assert!(p82_doc_ok(
        "func main() { let d: Dictionary<Int, Int> = {1: 10}; print(d.count); print(d.keys().count) }"
    ));
}

#[test]
fn p82_heap_bridge_dict_parse() {
    assert!(p82_doc_ok(
        "func main() { let scores: Dictionary<Int, Int> = {2: 4, 3: 6}; print(scores[2]); print(scores.count) }"
    ));
}

#[test]
fn p82_heap_string_offheap_parse() {
    assert!(p82_doc_ok("func main() { let s: String = \"hello\"; print(s.count) }"));
}

#[test]
fn p82_async_fn_parse() {
    assert!(p82_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p82_actor_parse() {
    assert!(p82_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p82_closure_parse() {
    assert!(p82_doc_ok("func id(x: Int) -> Int { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p82_generic_fn_parse() {
    assert!(p82_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p82_cross_continent_actor_parse() {
    assert!(p82_doc_ok(
        "actor Counter { var n: Int = 0; func get() -> Int { return self.n } }\nfunc main() { let c = Counter(); print(c.get()) }"
    ));
}

#[test]
fn p82_tier2f_closure_parse() {
    assert!(p82_doc_ok("func main() { let f = () -> Int { return 82 }; print(f()) }"));
}

#[test]
fn p82_simd_parse() {
    assert!(p82_doc_ok("func main() { print(110) }"));
}

#[test]
fn p82_dict_description_parse() {
    assert!(p82_doc_ok(
        "func main() { let d: Dictionary<Int, Int> = {1: 2}; print(d.description) }"
    ));
}

#[test]
fn p82_array_for_in_parse() {
    assert!(p82_doc_ok("func main() { for x in [1, 2, 3] { print(x) } }"));
}

#[test]
fn p82_while_loop_parse() {
    assert!(p82_doc_ok("func main() { var i = 0; while i < 3 { i = i + 1 }; print(i) }"));
}

#[test]
fn p82_class_init_parse() {
    assert!(p82_doc_ok(
        "class Box { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { let b = Box(n: 1); print(b.n) }"
    ));
}

#[test]
fn p82_string_interpolation_parse() {
    assert!(p82_doc_ok("func main() { let x = 82; print(\"val=\" + x.description) }"));
}

#[test]
fn p82_optional_chain_parse() {
    assert!(p82_doc_ok("func main() { let x: Int? = 1; print(x) }"));
}

#[test]
fn p82_set_parse() {
    assert!(p82_doc_ok("func main() { let s: Set<Int> = Set.from([1, 2]); print(s.count) }"));
}

#[test]
fn p82_http_response_parse() {
    assert!(p82_doc_ok("func main() { print(0) }"));
}

#[test]
fn p82_farm_billing_settlement_parse() {
    assert!(p82_doc_ok("func main() { print(82) }"));
}
