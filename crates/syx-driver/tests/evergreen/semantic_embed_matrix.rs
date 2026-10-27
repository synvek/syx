//! Phase 64 semantic embed matrix — one minimal repro per language rule.
// was: semantic_embed_matrix
use syx_driver::{compile_and_run_embedded, DriverOptions};
#[test]
fn sem_int_add() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(1 + 2) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "3"), "{:?}", r.printed);
}

#[test]
fn sem_int_sub() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(10 - 3) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "7"), "{:?}", r.printed);
}

#[test]
fn sem_int_mul() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(4 * 5) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "20"), "{:?}", r.printed);
}

#[test]
fn sem_int_div() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(20 / 4) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "5"), "{:?}", r.printed);
}

#[test]
fn sem_int_mod() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(10 % 3) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "1"), "{:?}", r.printed);
}

#[test]
fn sem_bool_and() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(true && false) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "false"), "{:?}", r.printed);
}

#[test]
fn sem_bool_or() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(true || false) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_bool_not() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(!false) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_cmp_lt() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(1 < 2) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_cmp_gt() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(3 > 2) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_cmp_eq() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(2 == 2) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_cmp_ne() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(2 != 3) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_str_lit() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(\"hello\") }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "hello"), "{:?}", r.printed);
}

#[test]
fn sem_nil_print() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(nil) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "nil"), "{:?}", r.printed);
}

#[test]
fn sem_if_true() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { if true { print(\"yes\") } }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "yes"), "{:?}", r.printed);
}

#[test]
fn sem_if_false() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { if false { print(\"no\") } else { print(\"else\") } }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "else"), "{:?}", r.printed);
}

#[test]
fn sem_while_once() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { var i = 0; while i < 1 { print(i); i = i + 1 } }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "0"), "{:?}", r.printed);
}

#[test]
fn sem_for_range() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { for i in 0..2 { print(i) } }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "0"), "{:?}", r.printed);
}

#[test]
fn sem_array_len() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded(
        "func main() {\n  let a = [1,2,3]\n  print(a.count)\n}",
        &DriverOptions::default(),
    )
    .unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "3"), "{:?}", r.printed);
}

#[test]
fn sem_dict_count() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded(
        "func main() {\n  let d = [\"a\": 1]\n  print(d.count)\n}",
        &DriverOptions::default(),
    )
    .unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "1"), "{:?}", r.printed);
}

#[test]
fn sem_func_call() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func f() -> Int { return 42 } func main() { print(f()) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "42"), "{:?}", r.printed);
}

#[test]
fn sem_nested_call() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func double(x: Int) -> Int { return x * 2 } func main() { print(double(21)) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "42"), "{:?}", r.printed);
}

#[test]
fn sem_var_mut() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { var x = 1; x = 2; print(x) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "2"), "{:?}", r.printed);
}

#[test]
fn sem_string_eq() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { print(\"a\" == \"a\") }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_array_get() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { let a = [10,20]; print(a[0]) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "10"), "{:?}", r.printed);
}

#[test]
fn sem_optional_some() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { let o: Int? = 5; print(o!) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "5"), "{:?}", r.printed);
}

#[test]
fn sem_guard_early() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { let x = 1; guard x > 0 else { return }; print(\"ok\") }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "ok"), "{:?}", r.printed);
}

#[test]
fn sem_switch_int() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { switch 1 { case 1: print(\"one\") default: print(\"other\") } }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "one"), "{:?}", r.printed);
}

#[test]
fn sem_break_loop() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { var i = 0; while true { if i == 1 { break }; i = i + 1 }; print(\"done\") }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "done"), "{:?}", r.printed);
}

#[test]
fn sem_continue_loop() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { for i in 0..3 { if i == 1 { continue }; print(i) } }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "0"), "{:?}", r.printed);
}

#[test]
fn sem_tuple_destructure() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { let t = (1, 2); print(t.0) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "1"), "{:?}", r.printed);
}

#[test]
fn sem_closure_capture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { var x = 1; let f = { print(x) }; f() }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "1"), "{:?}", r.printed);
}

#[test]
fn sem_class_field() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("class C { var n: Int init(n: Int) { self.n = n } } func main() { print(C(n: 7).n) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "7"), "{:?}", r.printed);
}

#[test]
fn sem_struct_init() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("struct P { var x: Int var y: Int } func main() { print(P(x: 1, y: 2).x) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "1"), "{:?}", r.printed);
}

#[test]
fn sem_enum_case() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("enum E { case a case b } func main() { print(E.a) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "a"), "{:?}", r.printed);
}

#[test]
fn sem_protocol_ext() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("protocol P { func id() -> Int } extension Int: P { func id() -> Int { return self } } func main() { print(5.id()) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "5"), "{:?}", r.printed);
}

#[test]
fn sem_try_catch() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { do { print(\"try\") } catch { print(\"catch\") } }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "try"), "{:?}", r.printed);
}

#[test]
fn sem_result_ok() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("func main() { let r = Result.ok(1); print(r.is_ok()) }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "true"), "{:?}", r.printed);
}

#[test]
fn sem_actor_basic() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("actor A { var n: Int init() { self.n = 0 } func inc() { self.n = self.n + 1 } } func main() { let a = A(); a.inc(); print(\"ok\") }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "ok"), "{:?}", r.printed);
}

#[test]
fn sem_inheritance_call() {
    syx_host_plugin::register_echo_plugin_in_process();
    let r = compile_and_run_embedded("class B { func f() { print(\"b\") } } class D: B { override func f() { print(\"d\") } } func main() { D().f() }", &DriverOptions::default()).unwrap();
    assert!(r.printed.iter().any(|l| l.trim() == "d"), "{:?}", r.printed);
}

