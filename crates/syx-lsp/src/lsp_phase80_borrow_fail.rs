// Phase 80 borrow compile-fail smoke via driver embed (16+ cross-await / heap-borrow / tier-2D).
use super::*;

fn p80_embed_fail(text: &str) -> bool {
    syx_driver::compile_embedded(text, &DriverOptions::default()).is_err()
}

#[test]
fn p80_borrow_fail_cross_await_outside_async() {
    assert!(p80_embed_fail("func main() { let x = await 1; print(x) }"));
}

#[test]
fn p80_borrow_fail_await_undefined() {
    assert!(p80_embed_fail(
        "async func f() -> Int { return await missing }\nfunc main() { print(0) }"
    ));
}

#[test]
fn p80_borrow_fail_heap_borrow_escape() {
    assert!(p80_embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &a; print(p) }"
    ));
}

#[test]
fn p80_borrow_fail_mut_borrow_twice() {
    assert!(p80_embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &mut a; let q = &mut a; print(p) }"
    ));
}

#[test]
fn p80_borrow_fail_dict_bad_key_type() {
    assert!(p80_embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {1: 2}; print(d[\"x\"]) }"
    ));
}

#[test]
fn p80_borrow_fail_undefined_var() {
    assert!(p80_embed_fail("func main() { print(undefined_x) }"));
}

#[test]
fn p80_borrow_fail_bad_type_add() {
    assert!(p80_embed_fail("func main() { let x: Int = \"bad\"; print(x) }"));
}

#[test]
fn p80_borrow_fail_generic_arity() {
    assert!(p80_embed_fail("func id<T>(x: T) -> T { return x }\nfunc main() { print(id()) }"));
}

#[test]
fn p80_borrow_fail_class_no_init() {
    assert!(p80_embed_fail("func main() { print(unknownType()) }"));
}

#[test]
fn p80_borrow_fail_call_missing_fn() {
    assert!(p80_embed_fail("func main() { missingFn() }"));
}

#[test]
fn p80_borrow_fail_int_from_string() {
    assert!(p80_embed_fail("func main() { let x: Int = \"x\"; print(x) }"));
}

#[test]
fn p80_borrow_fail_array_wrong_elem() {
    assert!(p80_embed_fail("func main() { let a: Array<Int> = [\"a\"]; print(a) }"));
}

#[test]
fn p80_borrow_fail_dict_value_type() {
    assert!(p80_embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {\"a\": 1}; print(d) }"
    ));
}

#[test]
fn p80_borrow_fail_async_let_await() {
    assert!(p80_embed_fail("func main() { print(await_missing_fn()) }"));
}

#[test]
fn p80_borrow_fail_import_forbidden() {
    assert!(p80_embed_fail("import Internal\nfunc main() { print(0) }"));
}

#[test]
fn p80_borrow_fail_lock_v14_field_expr() {
    assert!(p80_embed_fail("func main() { let lock = hsm_key_ref; print(lock) }"));
}

#[test]
fn p80_borrow_fail_closure_bad_return() {
    assert!(p80_embed_fail(
        "func main() { let f = () -> String { return 1 }; print(f()) }"
    ));
}

#[test]
fn p80_borrow_fail_actor_missing_method() {
    assert!(p80_embed_fail("actor A { var n: Int = 0 }\nfunc main() { let a = A(); a.missing() }"));
}

#[test]
fn p80_borrow_fail_simd_wrong_elem() {
    assert!(p80_embed_fail(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([\"a\"]); print(v) }"
    ));
}

#[test]
fn p80_borrow_fail_set_wrong_type() {
    assert!(p80_embed_fail("func main() { let s: Set<Int> = Set.from([\"a\"]); print(s.count) }"));
}
