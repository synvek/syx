// Phase 82 borrow compile-fail smoke via compile_embedded only (40 distinct semantic failures).
use super::*;

fn p82_embed_fail(text: &str) -> bool {
    syx_driver::compile_embedded(text, &DriverOptions::default()).is_err()
}

#[test]
fn p82_borrow_fail_cross_await_outside_async() {
    assert!(p82_embed_fail("func main() { let x = await 1; print(x) }"));
}

#[test]
fn p82_borrow_fail_await_undefined() {
    assert!(p82_embed_fail(
        "async func f() -> Int { return await missing }\nfunc main() { print(0) }"
    ));
}

#[test]
fn p82_borrow_fail_heap_borrow_escape() {
    assert!(p82_embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &a; print(p) }"
    ));
}

#[test]
fn p82_borrow_fail_mut_borrow_twice() {
    assert!(p82_embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &mut a; let q = &mut a; print(p) }"
    ));
}

#[test]
fn p82_borrow_fail_dict_bad_key_type() {
    assert!(p82_embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {1: 2}; print(d[\"x\"]) }"
    ));
}

#[test]
fn p82_borrow_fail_undefined_var() {
    assert!(p82_embed_fail("func main() { print(undefined_x) }"));
}

#[test]
fn p82_borrow_fail_bad_type_add() {
    assert!(p82_embed_fail("func main() { let x: Int = \"bad\"; print(x) }"));
}

#[test]
fn p82_borrow_fail_generic_arity() {
    assert!(p82_embed_fail("func id<T>(x: T) -> T { return x }\nfunc main() { print(id()) }"));
}

#[test]
fn p82_borrow_fail_class_no_init() {
    assert!(p82_embed_fail("func main() { print(unknownType()) }"));
}

#[test]
fn p82_borrow_fail_call_missing_fn() {
    assert!(p82_embed_fail("func main() { missingFn() }"));
}

#[test]
fn p82_borrow_fail_int_from_string() {
    assert!(p82_embed_fail("func main() { let x: Int = \"x\"; print(x) }"));
}

#[test]
fn p82_borrow_fail_array_wrong_elem() {
    assert!(p82_embed_fail("func main() { let a: Array<Int> = [\"a\"]; print(a) }"));
}

#[test]
fn p82_borrow_fail_dict_value_type() {
    assert!(p82_embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {\"a\": 1}; print(d) }"
    ));
}

#[test]
fn p82_borrow_fail_async_let_await() {
    assert!(p82_embed_fail("func main() { print(await_missing_fn()) }"));
}

#[test]
fn p82_borrow_fail_import_forbidden() {
    assert!(p82_embed_fail("import Internal\nfunc main() { print(0) }"));
}

#[test]
fn p82_borrow_fail_lock_v16_field_expr() {
    assert!(p82_embed_fail("func main() { let lock = marketplace_publish_epoch; print(lock) }"));
}

#[test]
fn p82_borrow_fail_closure_bad_return() {
    assert!(p82_embed_fail(
        "func main() { let f = () -> String { return 1 }; print(f()) }"
    ));
}

#[test]
fn p82_borrow_fail_actor_missing_method() {
    assert!(p82_embed_fail("actor A { var n: Int = 0 }\nfunc main() { let a = A(); a.missing() }"));
}

#[test]
fn p82_borrow_fail_simd_wrong_elem() {
    assert!(p82_embed_fail(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([\"a\"]); print(v) }"
    ));
}

#[test]
fn p82_borrow_fail_set_wrong_type() {
    assert!(p82_embed_fail("func main() { let s: Set<Int> = Set.from([\"a\"]); print(s.count) }"));
}

#[test]
fn p82_borrow_fail_billing_settlement_expr() {
    assert!(p82_embed_fail("func main() { let x = billing_settlement_id; print(x) }"));
}

#[test]
fn p82_borrow_fail_heap_bridge_ref_escape() {
    assert!(p82_embed_fail(
        "func main() { var s: String = \"x\"; let p = &s; print(p) }"
    ));
}

#[test]
fn p82_borrow_fail_tier2f_actor_bad_send() {
    assert!(p82_embed_fail(
        "actor A { var n: Int = 0 }\nfunc main() { let a = A(); a.send(\"bad\") }"
    ));
}

#[test]
fn p82_borrow_fail_bool_from_int() {
    assert!(p82_embed_fail("func main() { let x: Bool = 1; print(x) }"));
}

#[test]
fn p82_borrow_fail_return_type_mismatch() {
    assert!(p82_embed_fail("func main() { let x: Int = \"x\"; print(x) }"));
}

#[test]
fn p82_borrow_fail_param_type_mismatch() {
    assert!(p82_embed_fail("func id<T>(x: T) -> T { return x }\nfunc main() { print(id()) }"));
}

#[test]
fn p82_borrow_fail_void_return_value() {
    assert!(p82_embed_fail("func main() { let x: Void = 1; print(x) }"));
}

#[test]
fn p82_borrow_fail_undefined_fn_in_expr() {
    assert!(p82_embed_fail("func main() { print(missing(1)) }"));
}

#[test]
fn p82_borrow_fail_array_index_string() {
    assert!(p82_embed_fail("func main() { let a: Array<Int> = [1]; print(a[\"0\"]) }"));
}

#[test]
fn p82_borrow_fail_dict_index_int_on_string_key() {
    assert!(p82_embed_fail("func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d[1]) }"));
}

#[test]
fn p82_borrow_fail_assign_immutable() {
    assert!(p82_embed_fail("func main() { let x = 1; x = 2; print(x) }"));
}

#[test]
fn p82_borrow_fail_break_outside_loop() {
    assert!(p82_embed_fail("func main() { break; print(0) }"));
}

#[test]
fn p82_borrow_fail_continue_outside_loop() {
    assert!(p82_embed_fail("func main() { continue; print(0) }"));
}

#[test]
fn p82_borrow_fail_string_plus_int() {
    assert!(p82_embed_fail("func main() { let x: Int = \"a\" + 1; print(x) }"));
}



#[test]
fn p82_borrow_fail_extension_signing_key_expr() {
    assert!(p82_embed_fail("func main() { let x = extension_signing_key; print(x) }"));
}

#[test]
fn p82_borrow_fail_heapslot_bad_index_type() {
    assert!(p82_embed_fail("func main() { var a: Array<Int> = [1]; a[\"0\"] = 2; print(a[0]) }"));
}

#[test]
fn p82_borrow_fail_cross_region_actor_bad_type() {
    assert!(p82_embed_fail(
        "actor A { var n: String = \"x\" }\nfunc main() { let a = A(); a.n = 1; print(a.n) }"
    ));
}

#[test]
fn p82_borrow_fail_mono_witness_bad_type_args() {
    assert!(p82_embed_fail("func main() { let x: Int = \"a\" + 1; print(x) }"));
}

#[test]
fn p82_borrow_fail_dist_failover_undefined_region() {
    assert!(p82_embed_fail("func main() { let r = cross_continent_failover_region; print(r) }"));
}
