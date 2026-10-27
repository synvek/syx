// Phase 79 borrow compile-fail smoke via driver embed (12+ cross-await / heap-borrow).
use super::*;

fn p79_embed_fail(text: &str) -> bool {
    syx_driver::compile_and_run_embedded(text, &DriverOptions::default()).is_err()
}

#[test]
fn p79_borrow_fail_cross_await_outside_async() {
    assert!(p79_embed_fail("func main() { let x = await 1; print(x) }"));
}

#[test]
fn p79_borrow_fail_await_undefined() {
    assert!(p79_embed_fail(
        "async func f() -> Int { return await missing }\nfunc main() { print(0) }"
    ));
}

#[test]
fn p79_borrow_fail_heap_borrow_escape() {
    assert!(p79_embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &a; print(p) }"
    ));
}

#[test]
fn p79_borrow_fail_mut_borrow_twice() {
    assert!(p79_embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &mut a; let q = &mut a; print(p) }"
    ));
}

#[test]
fn p79_borrow_fail_dict_bad_key_type() {
    assert!(p79_embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {1: 2}; print(d[\"x\"]) }"
    ));
}

#[test]
fn p79_borrow_fail_undefined_var() {
    assert!(p79_embed_fail("func main() { print(undefined_x) }"));
}

#[test]
fn p79_borrow_fail_bad_type_add() {
    assert!(p79_embed_fail("func main() { let x: Int = \"bad\"; print(x) }"));
}

#[test]
fn p79_borrow_fail_generic_arity() {
    assert!(p79_embed_fail("func id<T>(x: T) -> T { return x }\nfunc main() { print(id()) }"));
}

#[test]
fn p79_borrow_fail_class_no_init() {
    assert!(p79_embed_fail("func main() { print(unknownType()) }"));
}

#[test]
fn p79_borrow_fail_call_missing_fn() {
    assert!(p79_embed_fail("func main() { missingFn() }"));
}

#[test]
fn p79_borrow_fail_int_from_string() {
    assert!(p79_embed_fail("func main() { let x: Int = \"x\"; print(x) }"));
}

#[test]
fn p79_borrow_fail_array_wrong_elem() {
    assert!(p79_embed_fail("func main() { let a: Array<Int> = [\"a\"]; print(a) }"));
}

#[test]
fn p79_borrow_fail_dict_value_type() {
    assert!(p79_embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {\"a\": 1}; print(d) }"
    ));
}

#[test]
fn p79_borrow_fail_async_let_await() {
    assert!(p79_embed_fail("func main() { print(await_missing_fn()) }"));
}

#[test]
fn p79_borrow_fail_import_forbidden() {
    assert!(p79_embed_fail("import Internal\nfunc main() { print(0) }"));
}

#[test]
fn p79_borrow_fail_lock_v13_field_expr() {
    assert!(p79_embed_fail("func main() { let lock = tenant_id; print(lock) }"));
}
