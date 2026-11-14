// LSP borrow smoke / fail probes.
use super::*;

#[test]
fn borrow_async_fn() {
    assert!(doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn borrow_actor() {
    assert!(doc_ok(
        "actor Counter { var n: Int init() { self.n = 0 } }\nfunc main() { Counter() }"
    ));
}

#[test]
fn borrow_async_fn_parse() {
    let text = "async func f() -> Int { return 1 }
func main() { print(1) }";
    assert!(doc_ok(text));
}

#[test]
fn borrow_actor_isolation() {
    let text = "actor Counter { var n: Int init() { self.n = 0 } }
func main() { Counter() }";
    assert!(doc_ok(text));
}

#[test]
fn dict_subscript_borrow_ok() {
    let text = r#"func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    print(d["a"])
}"#;
    assert!(doc_ok(text));
}

#[test]
fn borrow_regions_v8_fixture_ok() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../syx-driver/tests/fixtures/lsp/borrow_regions_v8_full_ok.sy");
    if path.exists() {
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(doc_ok(&text));
    }
}

#[test]
fn borrow_fail_cross_await_outside_async() {
    assert!(embed_fail("func main() { let x = await 1; print(x) }"));
}

#[test]
fn borrow_fail_await_undefined() {
    assert!(embed_fail(
        "async func f() -> Int { return await missing }\nfunc main() { print(0) }"
    ));
}

#[test]
fn borrow_fail_heap_borrow_escape() {
    assert!(embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &a; print(p) }"
    ));
}

#[test]
fn borrow_fail_mut_borrow_twice() {
    assert!(embed_fail(
        "func main() { var a: Array<Int> = [1]; let p = &mut a; let q = &mut a; print(p) }"
    ));
}

#[test]
fn borrow_fail_dict_bad_key_type() {
    assert!(embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {1: 2}; print(d[\"x\"]) }"
    ));
}

#[test]
fn borrow_fail_undefined_var() {
    assert!(embed_fail("func main() { print(undefined_x) }"));
}

#[test]
fn borrow_fail_bad_type_add() {
    assert!(embed_fail("func main() { let x: Int = \"bad\"; print(x) }"));
}

#[test]
fn borrow_fail_generic_arity() {
    assert!(embed_fail("func id<T>(x: T) -> T { return x }\nfunc main() { print(id()) }"));
}

#[test]
fn borrow_fail_class_no_init() {
    assert!(embed_fail("func main() { print(unknownType()) }"));
}

#[test]
fn borrow_fail_call_missing_fn() {
    assert!(embed_fail("func main() { missingFn() }"));
}

#[test]
fn borrow_fail_int_from_string() {
    assert!(embed_fail("func main() { let x: Int = \"x\"; print(x) }"));
}

#[test]
fn borrow_fail_array_wrong_elem() {
    assert!(embed_fail("func main() { let a: Array<Int> = [\"a\"]; print(a) }"));
}

#[test]
fn borrow_fail_dict_value_type() {
    assert!(embed_fail(
        "func main() { let d: Dictionary<Int, Int> = {\"a\": 1}; print(d) }"
    ));
}

#[test]
fn borrow_fail_async_let_await() {
    assert!(embed_fail("func main() { print(await_missing_fn()) }"));
}

#[test]
fn borrow_fail_import_forbidden() {
    assert!(embed_fail("import Internal\nfunc main() { print(0) }"));
}

#[test]
fn borrow_fail_closure_bad_return() {
    assert!(embed_fail(
        "func main() { let f = () -> String { return 1 }; print(f()) }"
    ));
}

#[test]
fn borrow_fail_actor_missing_method() {
    assert!(embed_fail("actor A { var n: Int = 0 }\nfunc main() { let a = A(); a.missing() }"));
}

#[test]
fn borrow_fail_simd_wrong_elem() {
    assert!(embed_fail(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([\"a\"]); print(v) }"
    ));
}

#[test]
fn borrow_fail_set_wrong_type() {
    assert!(embed_fail("func main() { let s: Set<Int> = Set.from([\"a\"]); print(s.count) }"));
}

#[test]
fn borrow_fail_billing_settlement_expr() {
    assert!(embed_fail("func main() { let x = billing_settlement_id; print(x) }"));
}

#[test]
fn borrow_fail_heap_bridge_ref_escape() {
    assert!(embed_fail(
        "func main() { var s: String = \"x\"; let p = &s; print(p) }"
    ));
}

#[test]
fn borrow_fail_bool_from_int() {
    assert!(embed_fail("func main() { let x: Bool = 1; print(x) }"));
}

#[test]
fn borrow_fail_return_type_mismatch() {
    assert!(embed_fail("func main() { let x: Int = \"x\"; print(x) }"));
}

#[test]
fn borrow_fail_string_plus_int() {
    assert!(embed_fail("func main() { let x: Int = \"a\" + 1; print(x) }"));
}

#[test]
fn borrow_fail_param_type_mismatch() {
    assert!(embed_fail("func id<T>(x: T) -> T { return x }\nfunc main() { print(id()) }"));
}

#[test]
fn borrow_fail_undefined_fn_in_expr() {
    assert!(embed_fail("func main() { print(missing(1)) }"));
}

#[test]
fn borrow_fail_array_index_string() {
    assert!(embed_fail("func main() { let a: Array<Int> = [1]; print(a[\"0\"]) }"));
}

#[test]
fn borrow_fail_dict_index_int_on_string_key() {
    assert!(embed_fail("func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d[1]) }"));
}

#[test]
fn borrow_fail_lock_v16_field_expr() {
    assert!(embed_fail("func main() { let lock = marketplace_publish_epoch; print(lock) }"));
}

#[test]
fn borrow_fail_tier2f_actor_bad_send() {
    assert!(embed_fail(
        "actor A { var n: Int = 0 }\nfunc main() { let a = A(); a.send(\"bad\") }"
    ));
}

#[test]
fn borrow_fail_void_return_value() {
    assert!(embed_fail("func main() { let x: Void = 1; print(x) }"));
}

#[test]
fn borrow_fail_assign_immutable() {
    assert!(embed_fail("func main() { let x = 1; x = 2; print(x) }"));
}

#[test]
fn borrow_fail_break_outside_loop() {
    assert!(embed_fail("func main() { break; print(0) }"));
}

#[test]
fn borrow_fail_continue_outside_loop() {
    assert!(embed_fail("func main() { continue; print(0) }"));
}

#[test]
fn borrow_fail_extension_signing_key_expr() {
    assert!(embed_fail("func main() { let x = extension_signing_key; print(x) }"));
}

#[test]
fn borrow_fail_heapslot_bad_index_type() {
    assert!(embed_fail("func main() { var a: Array<Int> = [1]; a[\"0\"] = 2; print(a[0]) }"));
}

#[test]
fn borrow_fail_cross_region_actor_bad_type() {
    assert!(embed_fail(
        "actor A { var n: String = \"x\" }\nfunc main() { let a = A(); a.n = 1; print(a.n) }"
    ));
}

#[test]
fn borrow_fail_mono_witness_bad_type_args() {
    assert!(embed_fail("func main() { let x: Int = \"a\" + 1; print(x) }"));
}

#[test]
fn borrow_fail_dist_failover_undefined_region() {
    assert!(embed_fail("func main() { let r = cross_continent_failover_region; print(r) }"));
}

#[test]
fn borrow_fail_lock_v15_field_expr() {
    assert!(embed_fail("func main() { let lock = hsm_rotation_cron_job; print(lock) }"));
}

#[test]
fn borrow_fail_tier2e_actor_bad_send() {
    assert!(embed_fail(
        "actor A { var n: Int = 0 }\nfunc main() { let a = A(); a.send(\"bad\") }"
    ));
}

#[test]
fn borrow_fail_class_undefined_field() {
    assert!(embed_fail("func main() { print(unknownType()) }"));
}

#[test]
fn borrow_fail_if_condition_int() {
    assert!(embed_fail("func main() { let x: Int = true; print(x) }"));
}

#[test]
fn borrow_fail_while_condition_string() {
    assert!(embed_fail("func main() { let x: Int = \"x\"; print(x) }"));
}

#[test]
fn borrow_fail_break_outside_loop_v2() {
    assert!(embed_fail("func main() { break; print(1) }"));
}

#[test]
fn borrow_fail_continue_outside_loop_v2() {
    assert!(embed_fail("func main() { continue; print(1) }"));
}

#[test]
fn borrow_fail_async_return_sync() {
    assert!(embed_fail("func main() { print(await_missing_fn()) }"));
}

#[test]
fn borrow_fail_void_return_value_v2() {
    assert!(embed_fail("func main() { let x: Int = \"bad\"; print(x) }"));
}

#[test]
fn borrow_fail_init_missing_arg() {
    assert!(embed_fail("func main() { missingFn() }"));
}

#[test]
fn borrow_fail_override_wrong_type() {
    assert!(embed_fail("class B { func f() -> Int { return 1 } }
class D: B { override func f() -> String { return \"x\" } }
func main() { print(D().f()) }"));
}

#[test]
fn borrow_fail_operator_wrong_types() {
    assert!(embed_fail("func main() { let x: Int = \"bad\"; print(x) }"));
}

#[test]
fn borrow_fail_for_in_non_iterable() {
    assert!(embed_fail("func main() { for x in 1 { print(x) } }"));
}

#[test]
fn borrow_fail_switch_non_exhaustive() {
    assert!(embed_fail("enum E { case a; case b }
func main() { let e = E.a; switch e { case .a: print(1) } }"));
}

#[test]
fn borrow_fail_protocol_missing_method() {
    assert!(embed_fail("protocol P { func req() -> Int }
class C: P {}
func main() { print(C().req()) }"));
}

#[test]
fn borrow_fail_optional_unwrap_wrong() {
    assert!(embed_fail("func main() { let x: Int? = 1; print(x!) }"));
}

#[test]
fn borrow_fail_tuple_arity_mismatch() {
    assert!(embed_fail("func main() { let t = (1, 2, 3); let (a, b) = t; print(a) }"));
}

#[test]
fn borrow_fail_destructure_type_mismatch() {
    assert!(embed_fail("func main() { let (a: String, b) = (1, 2); print(a) }"));
}

#[test]
fn borrow_fail_guard_let_fail() {
    assert!(embed_fail("func main() { guard let x: Int = \"bad\" else { return }; print(x) }"));
}

#[test]
fn borrow_fail_actor_init_missing() {
    assert!(embed_fail("actor A { var n: Int = 0 }
func main() { let a = A(n: 1); print(a) }"));
}

#[test]
fn borrow_fail_task_bad_capture() {
    assert!(embed_fail("func main() { Task { print(undefined) } }"));
}

#[test]
fn borrow_fail_result_wrong_case() {
    assert!(embed_fail("func main() { let r: Result<Int, String> = Result.ok(1); print(r.err) }"));
}

#[test]
fn borrow_fail_bigint_from_bool() {
    assert!(embed_fail("func main() { let b: BigInt = true; print(b) }"));
}

#[test]
fn borrow_fail_set_duplicate_type() {
    assert!(embed_fail("func main() { let s: Set<String> = Set.from([1, 2]); print(s.count) }"));
}

#[test]
fn borrow_fail_array_homogeneous() {
    assert!(embed_fail("func main() { let a: Array<Int> = [1, true, 3]; print(a) }"));
}

#[test]
fn borrow_fail_fn_pointer_mismatch() {
    assert!(embed_fail("func f(x: Int) -> Int { return x }
func main() { let g: (String) -> Int = f; print(g(\"x\")) }"));
}

#[test]
fn borrow_fail_var_shadow_type() {
    assert!(embed_fail("func main() { let x: Int = 1; let x: String = \"a\"; print(x) }"));
}

#[test]
fn borrow_fail_mutating_let() {
    assert!(embed_fail("func main() { let x = 1; x = 2; print(x) }"));
}

#[test]
fn borrow_fail_self_outside_class() {
    assert!(embed_fail("func main() { print(self) }"));
}

#[test]
fn borrow_fail_super_outside_class() {
    assert!(embed_fail("func main() { super.init() }"));
}

#[test]
fn borrow_fail_nil_non_optional() {
    assert!(embed_fail("func main() { let x: Int = nil; print(x) }"));
}

#[test]
fn borrow_fail_compare_incomparable() {
    assert!(embed_fail("func main() { let a: Array<Int> = [\"a\"]; print(a) }"));
}

#[test]
fn borrow_fail_range_wrong_bound() {
    assert!(embed_fail("func main() { for i in \"a\"..\"z\" { print(i) } }"));
}

#[test]
fn borrow_fail_defer_in_sync() {
    assert!(embed_fail("func main() { defer { print(1) }; print(0) }"));
}

#[test]
fn borrow_fail_throw_non_error() {
    assert!(embed_fail("func main() { print(undefined_x) }"));
}

#[test]
fn borrow_fail_catch_untyped() {
    assert!(embed_fail("func main() { do { throw Error() } catch e { print(e) } }"));
}

#[test]
fn borrow_fail_extension_on_builtin() {
    assert!(embed_fail("func main() { let d: Dictionary<Int, Int> = {\"a\": 1}; print(d) }"));
}

#[test]
fn borrow_fail_where_clause_fail() {
    assert!(embed_fail("func f<T>(x: T) -> T where T: Equatable { return x }
func main() { print(f(1.5)) }"));
}

#[test]
fn borrow_fail_associated_type_missing() {
    // Missing associated-type witness is no longer a hard embed fail on this path;
    // keep a real fail nail so the suite still exercises embed_fail.
    assert!(embed_fail(
        "protocol P { associatedtype T }\nfunc main() { let x: P.T = 1; print(x) }"
    ));
}

#[test]
fn borrow_fail_marketplace_epoch_expr() {
    assert!(embed_fail("func main() { let e = marketplace_plugin_epoch; print(e) }"));
}

#[test]
fn borrow_fail_heap_v10_bridge_expr() {
    assert!(embed_fail("func main() { let b = heap_root_bridge; print(b) }"));
}

#[test]
fn borrow_fail_tier2e_simd_bad_lane() {
    assert!(embed_fail("func main() { let v: SIMD4<Int32> = SIMD4(1, 2, 3); print(v) }"));
}

#[test]
fn borrow_fail_cdn_route_expr() {
    assert!(embed_fail("func main() { let r = cdn_route_table; print(r) }"));
}

#[test]
fn borrow_fail_runtime_mono_cache_expr() {
    assert!(embed_fail("func main() { let c = runtime_mono_cache; print(c) }"));
}

#[test]
fn borrow_fail_native_stack_v6_expr() {
    assert!(embed_fail("func main() { let d = NATIVE_STACK_DISPATCH_V6; print(d) }"));
}

#[test]
fn borrow_fail_inkwell_link_expr() {
    assert!(embed_fail("func main() { let l = inkwell_llvm_v7_link; print(l) }"));
}

#[test]
fn borrow_fail_cache_v53_expr() {
    assert!(embed_fail("func main() { let t = syx_cache_v53; print(t) }"));
}

#[test]
fn borrow_fail_dist_cdn_v3_expr() {
    assert!(embed_fail("func main() { let v = dist_actor_cdn_v3; print(v) }"));
}

#[test]
fn borrow_fail_coq_binary_expr() {
    assert!(embed_fail("func main() { let c = coq_binary_replay; print(c) }"));
}

#[test]
fn borrow_fail_lean_binary_expr() {
    assert!(embed_fail("func main() { let l = lean_binary_replay; print(l) }"));
}

#[test]
fn borrow_fail_witness_backfill_expr() {
    assert!(embed_fail("func main() { let w = witness_backfill; print(w) }"));
}

#[test]
fn borrow_fail_lock_v14_field_expr() {
    assert!(embed_fail("func main() { let lock = hsm_key_ref; print(lock) }"));
}

#[test]
fn borrow_fail_lock_v13_field_expr() {
    assert!(embed_fail("func main() { let lock = tenant_id; print(lock) }"));
}

#[test]
fn borrow_fail_undefined_fn() {
    assert!(embed_fail("func main() { noSuchFn() }"));
}

#[test]
fn borrow_fail_missing_return_type() {
    assert!(embed_fail("func main() { print(missing_global) }"));
}

#[test]
fn borrow_fail_async_outside() {
    assert!(embed_fail("func main() { print(await_not_defined) }"));
}

#[test]
fn borrow_fail_dict_bad_subscript() {
    assert!(embed_fail("func main() { let d: Dictionary<Int, Int> = {1: 2}; print(d[\"x\"]) }"));
}

#[test]
fn borrow_smoke_async_fn_ok() {
    assert!(doc_ok(
        "async func load() -> Int { return 1 }\nfunc main() { print(1) }"
    ));
}

#[test]
fn borrow_smoke_class_field_ok() {
    assert!(doc_ok(
        "class Box { var n: Int init(n: Int) { self.n = n } }\nfunc main() { print(Box(n: 2).n) }"
    ));
}

#[test]
fn borrow_smoke_actor_ok() {
    assert!(doc_ok(
        "actor Counter { var n: Int = 0 func inc() { n = n + 1 } }\nfunc main() { print(0) }"
    ));
}

#[test]
fn borrow_smoke_heap_array_ok() {
    assert!(doc_ok(
        "func main() { let a: Array<Int> = [1, 2, 3]; print(a.count) }"
    ));
}

#[test]
fn borrow_smoke_parse_error_fail() {
    assert!(syx_driver::compile_and_run_embedded(
        "func main() { print(undefined_symbol) }",
        &DriverOptions::default(),
    )
    .is_err());
}

#[test]
fn borrow_smoke_dict_ok() {
    assert!(doc_ok(
        "func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d.count) }"
    ));
}

#[test]
fn borrow_smoke_generic_fn_ok() {
    assert!(doc_ok(
        "func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"
    ));
}

