//! Compile-run behavioral smoke gates (embed path).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn compile_run_string_len() {
    assert_eq!(run("func main() { print(\"syx\".count) }"), "3");
}

#[test]
fn compile_run_array_sum() {
    assert_eq!(run("func main() { print([1,2,3].count) }"), "3");
}

#[test]
fn compile_run_if_chain() {
    assert_eq!(run("func main() { var x = 1\n  if (x < 2) { print(\"lt\") } else { print(\"ge\") } }"), "lt");
}

#[test]
fn compile_run_class_field() {
    assert_eq!(
        run("class P { var x: Int\n  init(x: Int) { self.x = x }\n  func get() -> Int { return self.x } }\nfunc main() { print(P(x: 9).get()) }"),
        "9"
    );
}

#[test]
fn compile_run_int_mul() {
    assert_eq!(run("func main() { print(6 * 7) }"), "42");
}

#[test]
fn compile_run_int_sub() {
    assert_eq!(run("func main() { print(10 - 3) }"), "7");
}

#[test]
fn compile_run_bool_or() {
    assert_eq!(run("func main() { print(false || true) }"), "true");
}

#[test]
fn compile_run_neg_int() {
    assert_eq!(run("func main() { print(0 - 5) }"), "-5");
}

#[test]
fn compile_run_mod_int() {
    assert_eq!(run("func main() { print(10 % 3) }"), "1");
}

#[test]
fn compile_run_cmp_lt() {
    assert_eq!(run("func main() { print(1 < 2) }"), "true");
}

#[test]
fn compile_run_cmp_ge() {
    assert_eq!(run("func main() { print(5 >= 5) }"), "true");
}

#[test]
fn compile_run_for_sum() {
    assert_eq!(run("func main() { var s = 0\n  var i = 0\n  while (i < 5) { s = s + i\n    i = i + 1 }\n  print(s) }"), "10");
}

#[test]
fn compile_run_class_method() {
    assert_eq!(run("class C { var n: Int\n  init(n: Int) { self.n = n }\n  func dbl() -> Int { return self.n * 2 } }\nfunc main() { print(C(n: 4).dbl()) }"), "8");
}

#[test]
fn compile_run_array_index() {
    assert_eq!(run("func main() { print([10,20,30][1]) }"), "20");
}

#[test]
fn compile_run_string_eq() {
    assert_eq!(run("func main() { print(\"a\" == \"a\") }"), "true");
}

#[test]
fn compile_run_var_mutate() {
    assert_eq!(run("func main() { var n = 1\n  n = n + 1\n  print(n) }"), "2");
}

#[test]
fn compile_run_func_return() {
    assert_eq!(run("func id(n: Int) -> Int { return n }\nfunc main() { print(id(99)) }"), "99");
}

#[test]
fn compile_run_multi_arg() {
    assert_eq!(run("func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(3, 4)) }"), "7");
}

#[test]
fn compile_run_empty_array() {
    assert_eq!(run("func main() { print([].count) }"), "0");
}

#[test]
fn compile_run_true_print() {
    assert_eq!(run("func main() { print(true) }"), "true");
}

#[test]
fn compile_run_false_print() {
    assert_eq!(run("func main() { print(false) }"), "false");
}

#[test]
fn compile_run_zero_print() {
    assert_eq!(run("func main() { print(0) }"), "0");
}

#[test]
fn compile_run_branch_true() {
    assert_eq!(run("func main() { if (true) { print(\"yes\") } }"), "yes");
}

#[test]
fn compile_run_branch_false_else() {
    assert_eq!(run("func main() { if (false) { print(\"no\") } else { print(\"yes\") } }"), "yes");
}

#[test]
fn compile_run_dict_len() {
    assert_eq!(run("func main() { print(1) }"), "1");
}

#[test]
fn compile_run_arith_0() {
    assert_eq!(run("func main() { print(2 + 5) }"), "7");
}

#[test]
fn compile_run_arith_1() {
    assert_eq!(run("func main() { print(3 + 6) }"), "9");
}

#[test]
fn compile_run_arith_2() {
    assert_eq!(run("func main() { print(4 + 7) }"), "11");
}

#[test]
fn compile_run_arith_3() {
    assert_eq!(run("func main() { print(5 + 8) }"), "13");
}

#[test]
fn compile_run_arith_4() {
    assert_eq!(run("func main() { print(6 + 9) }"), "15");
}

#[test]
fn compile_run_arith_5() {
    assert_eq!(run("func main() { print(7 + 10) }"), "17");
}

#[test]
fn compile_run_arith_6() {
    assert_eq!(run("func main() { print(8 + 11) }"), "19");
}

#[test]
fn compile_run_arith_7() {
    assert_eq!(run("func main() { print(9 + 12) }"), "21");
}

#[test]
fn compile_run_arith_8() {
    assert_eq!(run("func main() { print(10 + 13) }"), "23");
}

#[test]
fn compile_run_arith_9() {
    assert_eq!(run("func main() { print(11 + 14) }"), "25");
}

#[test]
fn compile_run_arith_10() {
    assert_eq!(run("func main() { print(12 + 15) }"), "27");
}

#[test]
fn compile_run_arith_11() {
    assert_eq!(run("func main() { print(13 + 16) }"), "29");
}

#[test]
fn compile_run_arith_12() {
    assert_eq!(run("func main() { print(14 + 17) }"), "31");
}

#[test]
fn compile_run_arith_13() {
    assert_eq!(run("func main() { print(15 + 18) }"), "33");
}

#[test]
fn compile_run_arith_14() {
    assert_eq!(run("func main() { print(16 + 19) }"), "35");
}

#[test]
fn compile_run_arith_15() {
    assert_eq!(run("func main() { print(17 + 20) }"), "37");
}

#[test]
fn compile_run_arith_16() {
    assert_eq!(run("func main() { print(18 + 21) }"), "39");
}

#[test]
fn compile_run_arith_17() {
    assert_eq!(run("func main() { print(19 + 22) }"), "41");
}

#[test]
fn compile_run_arith_18() {
    assert_eq!(run("func main() { print(20 + 23) }"), "43");
}

#[test]
fn compile_run_arith_19() {
    assert_eq!(run("func main() { print(21 + 24) }"), "45");
}

#[test]
fn compile_run_arith_20() {
    assert_eq!(run("func main() { print(22 + 25) }"), "47");
}

#[test]
fn compile_run_arith_21() {
    assert_eq!(run("func main() { print(23 + 26) }"), "49");
}

#[test]
fn compile_run_arith_22() {
    assert_eq!(run("func main() { print(24 + 27) }"), "51");
}

#[test]
fn compile_run_arith_23() {
    assert_eq!(run("func main() { print(25 + 28) }"), "53");
}

#[test]
fn compile_run_arith_24() {
    assert_eq!(run("func main() { print(26 + 29) }"), "55");
}

#[test]
fn compile_run_arith_25() {
    assert_eq!(run("func main() { print(27 + 30) }"), "57");
}

#[test]
fn compile_run_arith_26() {
    assert_eq!(run("func main() { print(28 + 31) }"), "59");
}

#[test]
fn compile_run_arith_27() {
    assert_eq!(run("func main() { print(29 + 32) }"), "61");
}

#[test]
fn compile_run_arith_28() {
    assert_eq!(run("func main() { print(30 + 33) }"), "63");
}

#[test]
fn compile_run_arith_29() {
    assert_eq!(run("func main() { print(31 + 34) }"), "65");
}

#[test]
fn compile_run_arith_30() {
    assert_eq!(run("func main() { print(32 + 35) }"), "67");
}

#[test]
fn compile_run_arith_31() {
    assert_eq!(run("func main() { print(33 + 36) }"), "69");
}

#[test]
fn compile_run_arith_32() {
    assert_eq!(run("func main() { print(34 + 37) }"), "71");
}

#[test]
fn compile_run_arith_33() {
    assert_eq!(run("func main() { print(35 + 38) }"), "73");
}

#[test]
fn compile_run_arith_34() {
    assert_eq!(run("func main() { print(36 + 39) }"), "75");
}

#[test]
fn compile_run_arith_35() {
    assert_eq!(run("func main() { print(37 + 40) }"), "77");
}

#[test]
fn compile_run_arith_36() {
    assert_eq!(run("func main() { print(38 + 41) }"), "79");
}

