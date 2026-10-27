// Phase 78 borrow compile-fail smoke via driver embed (8+ behavioral).
use super::*;

fn p78_embed_fail(text: &str) -> bool {
    syx_driver::compile_and_run_embedded(text, &DriverOptions::default()).is_err()
}

#[test]
fn p78_borrow_fail_undefined_var() {
    assert!(p78_embed_fail("func main() { print(undefined_x) }"));
}

#[test]
fn p78_borrow_fail_undefined_fn() {
    assert!(p78_embed_fail("func main() { noSuchFn() }"));
}

#[test]
fn p78_borrow_fail_bad_type_add() {
    assert!(p78_embed_fail("func main() { let x: Int = \"bad\"; print(x) }"));
}

#[test]
fn p78_borrow_fail_missing_return_type() {
    assert!(p78_embed_fail("func main() { print(missing_global) }"));
}

#[test]
fn p78_borrow_fail_class_no_init() {
    assert!(p78_embed_fail("func main() { print(unknownType()) }"));
}

#[test]
fn p78_borrow_fail_async_outside() {
    assert!(p78_embed_fail("func main() { print(await_not_defined) }"));
}

#[test]
fn p78_borrow_fail_dict_bad_subscript() {
    assert!(p78_embed_fail("func main() { let d: Dictionary<Int, Int> = {1: 2}; print(d[\"x\"]) }"));
}

#[test]
fn p78_borrow_fail_generic_arity() {
    assert!(p78_embed_fail("func id<T>(x: T) -> T { return x }\nfunc main() { print(id()) }"));
}
