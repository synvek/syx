//! Contextual unsuffixed integer literals under fixed-width annotations.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn compile_fail_code(src: &str) -> Option<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = match syx_parser::Parser::parse(file) {
        Ok(m) => m,
        Err(d) => return d.code.clone(),
    };
    match syx_semantic::SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.clone(),
        Ok(a) => a.errors.first().and_then(|e| e.code.clone()),
    }
}

#[test]
fn contextual_int8_unsuffixed_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let x: Int8 = 8
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "8"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn contextual_uint16_unsuffixed_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let x: UInt16 = 1
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "1"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn contextual_int8_out_of_range_s011() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let x: Int8 = 128
}
"#
        )
        .as_deref(),
        Some("S011")
    );
}

#[test]
fn contextual_int8_wrong_suffix_still_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let x: Int8 = 8i16
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn contextual_int8_suffix_i8_still_ok() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let x: Int8 = 8i8
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "8"),
        "printed={:?}",
        result.printed
    );
}

fn printed_number(lines: &[String]) -> Option<f64> {
    lines.iter().find_map(|l| l.trim().parse::<f64>().ok())
}

#[test]
fn contextual_double_unsuffixed_int_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let x: Double = 1
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(
        printed_number(&result.printed),
        Some(1.0),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn contextual_float_unsuffixed_int_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let y: Float = 2
    print(y)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(
        printed_number(&result.printed),
        Some(2.0),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn contextual_double_decimal_still_ok() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let x: Double = 1.5
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(
        printed_number(&result.printed),
        Some(1.5),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn contextual_float_wrong_suffix_still_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let x: Float = 1i16
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn integer_narrow_int_to_int8_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int = 8
    let b: Int8 = a
    print(b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "8"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn integer_narrow_int_to_int8_oor_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    let a: Int = 128
    let b: Int8 = a
    print(b)
}
"#,
        &opts(),
    )
    .expect_err("narrow OOR");
    assert_eq!(err.code.as_deref(), Some("R003"));
    assert!(
        err.message.contains("out of range"),
        "message={}",
        err.message
    );
}

#[test]
fn integer_narrow_assign_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int = 7
    var b: Int8 = 0
    b = a
    print(b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "7"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn integer_widen_int8_to_int_still_ok() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = 8
    let b: Int = a
    print(b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "8"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn integer_as_int8_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let n: Int = 8
    print(n as Int8)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "8"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn integer_as_int8_oor_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    let n: Int = 128
    let _x = n as Int8
}
"#,
        &opts(),
    )
    .expect_err("as OOR");
    assert_eq!(err.code.as_deref(), Some("R003"));
    assert!(
        err.message.contains("out of range"),
        "message={}",
        err.message
    );
}

#[test]
fn integer_as_optional_int8_oor_is_nil() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let n: Int = 128
    print(n as? Int8)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "nil"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn integer_as_optional_int8_in_range_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let n: Int = 8
    print(n as? Int8)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "8"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn integer_as_widen_int8_to_int_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = 8
    print(a as Int)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "8"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn float_as_int_truncates_toward_zero() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let d: Double = 1.5
    print(d as Int)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "1"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn float_as_int8_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let d: Double = 1.5
    print(d as Int8)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "1"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn float_as_optional_int8_in_range_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let d: Double = 1.5
    print(d as? Int8)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "1"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn float_as_int8_oor_is_r003() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
    let big: Double = 100000000000000000000.0
    let _x = big as Int8
}
"#,
        &opts(),
    )
    .expect_err("float as OOR");
    assert_eq!(err.code.as_deref(), Some("R003"));
    assert!(
        err.message.contains("out of range"),
        "message={}",
        err.message
    );
}

#[test]
fn float_as_optional_int8_oor_is_nil() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let big: Double = 100000000000000000000.0
    print(big as? Int8)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "nil"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn float_assign_to_int8_still_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let x: Int8 = 1.5
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn binary_int8_plus_int8_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = 100
    let b: Int8 = 100
    print(a + b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "-56"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn binary_int8_plus_int_widens() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = 1
    let b: Int = 100
    print(a + b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "101"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn binary_int_plus_double_still_ok() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int = 1
    print(a + 1.5)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "2.5"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn binary_int_plus_uint_is_s014() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Int = 1
    let b: UInt = 2
    let _x = a + b
}
"#
        )
        .as_deref(),
        Some("S014")
    );
}

#[test]
fn binary_int8_shl_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = 64
    print(a << 1)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "-128"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn binary_int8_bitand_keeps_width() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = -1
    let b: Int8 = 15
    print(a & b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "15"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn binary_int_bitand_uint_is_s014() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Int = 1
    let b: UInt = 2
    let _x = a & b
}
"#
        )
        .as_deref(),
        Some("S014")
    );
}

#[test]
fn compare_uint8_gt_uses_unsigned_width() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: UInt8 = 200
    let b: UInt8 = 100
    print(a > b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["true".to_string()]);
}

#[test]
fn compare_int8_lt_negative() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = -1
    let b: Int8 = 0
    print(a < b)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["true".to_string()]);
}

#[test]
fn unary_neg_int8_min_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Int8 = -128
    print(-a)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "-128"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn compound_assign_int8_plus_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var x: Int8 = 120
    x += 10
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "-126"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn compound_assign_int8_bitand_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var x: Int8 = -1
    x &= 15
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "15"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn compound_assign_int8_bitor_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var x: Int8 = 1
    x |= 15
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "15"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn compound_assign_int8_bitxor_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var x: Int8 = 15
    x ^= 1
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "14"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn compound_assign_int8_shl_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var x: Int8 = 64
    x <<= 1
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "-128"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn compound_assign_int8_shr_wraps() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var x: Int8 = -128
    x >>= 1
    print(x)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "-64"),
        "printed={:?}",
        result.printed
    );
}
