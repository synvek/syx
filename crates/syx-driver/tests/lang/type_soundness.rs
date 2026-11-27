//! C-0b: return type assignability (S004).
//! C-0c: free-function arity (S061).
//! C-0d: `&mut` bare-value args rejected (S004).
//! C-0a: Unknown→ground sinks from untyped params / missing members (S004).
//! C-0f: call-arg Unknown→ground concrete params (S004; ≠ close soft arm).

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

fn compile_fail_message(src: &str) -> Option<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = match syx_parser::Parser::parse(file) {
        Ok(m) => m,
        Err(d) => return Some(d.message.clone()),
    };
    match syx_semantic::SemanticAnalyzer::analyze(&module) {
        Err(d) => Some(d.message.clone()),
        Ok(a) => a.errors.first().map(|e| e.message.clone()),
    }
}

fn compile_ok(src: &str) {
    syx_host_plugin::register_echo_plugin_in_process();
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = syx_parser::Parser::parse(file).expect("parse");
    match syx_semantic::SemanticAnalyzer::analyze(&module) {
        Ok(a) => assert!(
            a.errors.is_empty(),
            "expected Ok, got errors={:?}",
            a.errors
                .iter()
                .map(|e| format!("{:?}:{}", e.code, e.message))
                .collect::<Vec<_>>()
        ),
        Err(d) => panic!("expected Ok, got Err {}:{}", d.code.as_deref().unwrap_or("?"), d.message),
    }
}

#[test]
fn c0b_wrong_return_string_to_int_s004() {
    let code = compile_fail_code(
        r#"
func f() -> Int { return "wrong" }
func main() { }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func f() -> Int { return "wrong" }
func main() { }
"#,
    )
    .expect("message");
    assert!(
        msg.contains("expected Int") && msg.contains("String"),
        "msg={msg}"
    );
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0b_correct_return_int_ok() {
    compile_ok(
        r#"
func f() -> Int { return 1 }
func main() { }
"#,
    );
}

#[test]
fn c0b_bare_return_non_void_s004() {
    let code = compile_fail_code(
        r#"
func f() -> Int { return }
func main() { }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
}

#[test]
fn c0b_method_wrong_return_s004() {
    let code = compile_fail_code(
        r#"
struct Box {
    func value() -> Int { return "no" }
}
func main() { }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
}

#[test]
fn c0b_method_correct_return_ok() {
    compile_ok(
        r#"
struct Box {
    func value() -> Int { return 7 }
}
func main() {
    let b = Box()
    print(b.value())
}
"#,
    );
}

#[test]
fn c0c_too_few_args_s061() {
    let code = compile_fail_code(
        r#"
func sum2(_ a: Int, _ b: Int) -> Int { return a }
func main() { sum2(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S061"), "got={code:?}");
}

#[test]
fn c0c_too_many_args_s061() {
    let code = compile_fail_code(
        r#"
func sum2(_ a: Int, _ b: Int) -> Int { return a }
func main() { sum2(1, 2, 3) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S061"), "got={code:?}");
}

#[test]
fn c0c_exact_arity_ok() {
    compile_ok(
        r#"
func sum2(_ a: Int, _ b: Int) -> Int { return a }
func main() { let _ = sum2(1, 2) }
"#,
    );
}

#[test]
fn c0c_trailing_default_omit_ok() {
    compile_ok(
        r#"
func f(_ x: Int = 1) -> Int { return x }
func main() {
    print(f())
    print(f(2))
}
"#,
    );
}

#[test]
fn c0c_wrong_arg_type_still_s004() {
    let code = compile_fail_code(
        r#"
func sum2(_ a: Int, _ b: Int) -> Int { return a }
func main() { sum2("x", 1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
}

#[test]
fn c0c_missing_label_still_s042() {
    let code = compile_fail_code(
        r#"
func sum2(a: Int, b: Int) -> Int { return a }
func main() { sum2(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S042"), "got={code:?}");
}

#[test]
fn c0c_untyped_param_id_still_ok() {
    // Untyped params stay Unknown (no S004); `_` keeps labels out of the way.
    compile_ok(
        r#"
func id(_ x) -> Int { return 1 }
func main() { let _ = id(1) }
"#,
    );
}

#[test]
fn c0d_bare_value_to_mut_ref_s004() {
    let code = compile_fail_code(
        r#"
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump(n)
}
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump(n)
}
"#,
    )
    .expect("message");
    assert!(
        msg.contains("expected &mut Int") && msg.contains("Int"),
        "msg={msg}"
    );
}

#[test]
fn c0d_explicit_mut_borrow_ok() {
    compile_ok(
        r#"
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump(&mut n)
}
"#,
    );
}

#[test]
fn c0d_shared_borrow_to_mut_ref_s004() {
    let code = compile_fail_code(
        r#"
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump(&n)
}
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
}

#[test]
fn c0d_shared_ref_bare_value_still_ok() {
    compile_ok(
        r#"
func id(_ x: &Int) -> Int { return 1 }
func main() {
    var n = 0
    let _ = id(n)
}
"#,
    );
}

#[test]
fn c0d_grouped_mut_borrow_ok() {
    compile_ok(
        r#"
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump((&mut n))
}
"#,
    );
}

#[test]
fn c0a_untyped_param_return_concrete_s004() {
    let code = compile_fail_code(
        r#"
func id(_ x) -> Int { return x }
func main() { }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func id(_ x) -> Int { return x }
func main() { }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0a_object_missing_member_to_int_s004() {
    let code = compile_fail_code(
        r#"
func main() {
    let o: Object = [:]
    let n: Int = o.missing
}
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
}

#[test]
fn c0a_untyped_param_return_literal_still_ok() {
    compile_ok(
        r#"
func id(_ x) -> Int { return 1 }
func main() { }
"#,
    );
}

#[test]
fn c0f_untyped_arg_to_int_param_s004() {
    // C-0f: retired C-0a "still_ok" green nail — call-arg Unknown→Int must S004.
    let code = compile_fail_code(
        r#"
func take(_ n: Int) { }
func wrap(_ x) { take(x) }
func main() { wrap(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func take(_ n: Int) { }
func wrap(_ x) { take(x) }
func main() { wrap(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0f_untyped_arg_to_any_still_ok() {
    compile_ok(
        r#"
func take(_ n: Any) { }
func wrap(_ x) { take(x) }
func main() { wrap(1) }
"#,
    );
}

#[test]
fn c0f_literal_arg_to_int_still_ok() {
    compile_ok(
        r#"
func take(_ n: Int) { }
func main() { take(1) }
"#,
    );
}

#[test]
fn c0g_untyped_compound_assign_int_s004() {
    let code = compile_fail_code(
        r#"
func bump(_ x) {
    var n: Int = 0
    n += x
}
func main() { bump(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func bump(_ x) {
    var n: Int = 0
    n += x
}
func main() { bump(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0g_literal_compound_still_ok() {
    compile_ok(
        r#"
func main() {
    var n: Int = 0
    n += 1
}
"#,
    );
}

#[test]
fn c0g_untyped_compound_assign_any_still_ok() {
    compile_ok(
        r#"
func bump(_ x) {
    var a: Any = 0
    a += x
}
func main() { bump(1) }
"#,
    );
}

#[test]
fn c0h_untyped_member_assign_int_s004() {
    let code = compile_fail_code(
        r#"
struct Cell {
    var n: Int = 0
}
func put(_ x) {
    var c = Cell()
    c.n = x
}
func main() { put(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
struct Cell {
    var n: Int = 0
}
func put(_ x) {
    var c = Cell()
    c.n = x
}
func main() { put(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0h_literal_member_assign_still_ok() {
    compile_ok(
        r#"
struct Cell {
    var n: Int = 0
}
func main() {
    var c = Cell()
    c.n = 1
}
"#,
    );
}

#[test]
fn c0h_any_member_assign_still_ok() {
    compile_ok(
        r#"
struct Bag {
    var a: Any = 0
}
func put(_ x) {
    var b = Bag()
    b.a = x
}
func main() { put(1) }
"#,
    );
}

#[test]
fn c0i_untyped_index_assign_array_int_s004() {
    let code = compile_fail_code(
        r#"
func put(_ x) {
    var a: Array<Int> = [0]
    a[0] = x
}
func main() { put(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func put(_ x) {
    var a: Array<Int> = [0]
    a[0] = x
}
func main() { put(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0i_untyped_index_assign_dict_int_s004() {
    let code = compile_fail_code(
        r#"
func put(_ x) {
    var d: Dictionary<String, Int> = [:]
    d["k"] = x
}
func main() { put(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
}

#[test]
fn c0i_literal_index_assign_still_ok() {
    compile_ok(
        r#"
func main() {
    var a: Array<Int> = [0]
    a[0] = 1
}
"#,
    );
}

#[test]
fn c0i_any_index_assign_still_ok() {
    compile_ok(
        r#"
func put(_ x) {
    var a: Array<Any> = [0]
    a[0] = x
}
func main() { put(1) }
"#,
    );
}

#[test]
fn c0p_untyped_array_index_int_s004() {
    let code = compile_fail_code(
        r#"
func put(_ i) {
    var a: Array<Int> = [0]
    a[i] = 1
}
func main() { put(0) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func put(_ i) {
    var a: Array<Int> = [0]
    a[i] = 1
}
func main() { put(0) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0p_untyped_dict_key_string_s004() {
    let code = compile_fail_code(
        r#"
func put(_ k) {
    var d: Dictionary<String, Int> = [:]
    d[k] = 1
}
func main() { put("k") }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func put(_ k) {
    var d: Dictionary<String, Int> = [:]
    d[k] = 1
}
func main() { put("k") }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected String"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0p_literal_index_key_still_ok() {
    compile_ok(
        r#"
func main() {
    var a: Array<Int> = [0]
    a[0] = 1
    var d: Dictionary<String, Int> = [:]
    d["k"] = 1
}
"#,
    );
}

#[test]
fn c0p_any_index_key_still_ok() {
    compile_ok(
        r#"
func put(_ i: Any, _ k: Any) {
    var a: Array<Int> = [0]
    a[0] = 1
    var d: Dictionary<String, Int> = [:]
    d["k"] = 1
}
func main() { put(0, "k") }
"#,
    );
}

#[test]
fn c0q_untyped_array_index_read_int_s004() {
    let code = compile_fail_code(
        r#"
func get(_ i) {
    var a: Array<Int> = [0]
    let _ = a[i]
}
func main() { get(0) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func get(_ i) {
    var a: Array<Int> = [0]
    let _ = a[i]
}
func main() { get(0) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0q_untyped_dict_key_read_string_s004() {
    let code = compile_fail_code(
        r#"
func get(_ k) {
    var d: Dictionary<String, Int> = ["a": 1]
    let _ = d[k]
}
func main() { get("a") }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func get(_ k) {
    var d: Dictionary<String, Int> = ["a": 1]
    let _ = d[k]
}
func main() { get("a") }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected String"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0q_literal_index_key_read_still_ok() {
    compile_ok(
        r#"
func main() {
    var a: Array<Int> = [0]
    let x = a[0]
    var d: Dictionary<String, Int> = ["k": 1]
    let y = d["k"]
}
"#,
    );
}

#[test]
fn c0q_any_index_key_read_still_ok() {
    compile_ok(
        r#"
func get(_ i: Any, _ k: Any) {
    var a: Array<Int> = [0]
    let x = a[0]
    var d: Dictionary<String, Int> = ["k": 1]
    let y = d["k"]
}
func main() { get(0, "k") }
"#,
    );
}

#[test]
fn c0r_untyped_string_index_int_s004() {
    let code = compile_fail_code(
        r#"
func get(_ i) {
    let s = "ab"
    let c = s[i]
}
func main() { get(0) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
func get(_ i) {
    let s = "ab"
    let c = s[i]
}
func main() { get(0) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0r_literal_string_index_still_ok() {
    compile_ok(
        r#"
func main() {
    let s = "ab"
    let c = s[0]
}
"#,
    );
}

#[test]
fn c0r_any_string_index_still_ok() {
    compile_ok(
        r#"
func get(_ i: Any) {
    let s = "ab"
    let c = s[0]
}
func main() { get(0) }
"#,
    );
}

#[test]
fn c0s_untyped_subscript_get_index_int_s004() {
    let code = compile_fail_code(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func get(_ i) {
    var b = Box()
    let x = b[i]
}
func main() { get(0) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func get(_ i) {
    var b = Box()
    let x = b[i]
}
func main() { get(0) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0s_untyped_subscript_set_index_int_s004() {
    let code = compile_fail_code(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func put(_ i) {
    var b = Box()
    b[i] = 1
}
func main() { put(0) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func put(_ i) {
    var b = Box()
    b[i] = 1
}
func main() { put(0) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0s_literal_subscript_index_still_ok() {
    compile_ok(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func main() {
    var b = Box()
    let x = b[0]
    b[0] = 1
}
"#,
    );
}

#[test]
fn c0s_any_subscript_index_still_ok() {
    compile_ok(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func get(_ i: Any) {
    var b = Box()
    let x = b[0]
    b[0] = 1
}
func main() { get(0) }
"#,
    );
}

#[test]
fn c0t_untyped_enum_case_member_arg_int_s004() {
    let code = compile_fail_code(
        r#"
enum E { wrap(n: Int) }
func take(_ x) { E.wrap(n: x) }
func main() { take(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
enum E { wrap(n: Int) }
func take(_ x) { E.wrap(n: x) }
func main() { take(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0t_literal_enum_case_arg_still_ok() {
    compile_ok(
        r#"
enum E { wrap(n: Int) }
func main() { E.wrap(n: 1) }
"#,
    );
}

#[test]
fn c0t_any_enum_case_arg_still_ok() {
    compile_ok(
        r#"
enum E { wrap(n: Any) }
func take(_ x) { E.wrap(n: x) }
func main() { take(1) }
"#,
    );
}

#[test]
fn c0j_untyped_method_arg_int_s004() {
    let code = compile_fail_code(
        r#"
struct S {
    func take(_ n: Int) { }
    func wrap(_ x) { self.take(x) }
}
func main() { S().wrap(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
struct S {
    func take(_ n: Int) { }
    func wrap(_ x) { self.take(x) }
}
func main() { S().wrap(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0j_literal_method_arg_still_ok() {
    compile_ok(
        r#"
struct S {
    func take(_ n: Int) { }
}
func main() { S().take(1) }
"#,
    );
}

#[test]
fn c0j_any_method_arg_still_ok() {
    compile_ok(
        r#"
struct S {
    func take(_ n: Any) { }
    func wrap(_ x) { self.take(x) }
}
func main() { S().wrap(1) }
"#,
    );
}

#[test]
fn c0k_untyped_subscript_set_int_s004() {
    // Desugared accessors: type_soundness helpers skip frontend subscript desugar.
    let code = compile_fail_code(
        r#"
class Bag {
}
extension Bag {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func put(_ v) {
    var b = Bag()
    b[0] = v
}
func main() { put(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
class Bag {
}
extension Bag {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func put(_ v) {
    var b = Bag()
    b[0] = v
}
func main() { put(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0k_literal_subscript_set_still_ok() {
    compile_ok(
        r#"
class Bag {
}
extension Bag {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func main() {
    var b = Bag()
    b[0] = 1
}
"#,
    );
}

#[test]
fn c0k_any_subscript_set_still_ok() {
    compile_ok(
        r#"
class Bag {
}
extension Bag {
    func __subscript_get(i: Int) -> Any { return 0 }
    func __subscript_set(i: Int, newValue: Any) { }
}
func put(_ v) {
    var b = Bag()
    b[0] = v
}
func main() { put(1) }
"#,
    );
}

#[test]
fn c0l_untyped_init_arg_int_s004() {
    let code = compile_fail_code(
        r#"
struct S {
    init(_ n: Int) { }
}
func wrap(_ x) { S(x) }
func main() { wrap(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
struct S {
    init(_ n: Int) { }
}
func wrap(_ x) { S(x) }
func main() { wrap(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0l_literal_init_arg_still_ok() {
    compile_ok(
        r#"
struct S {
    init(_ n: Int) { }
}
func main() { S(1) }
"#,
    );
}

#[test]
fn c0l_any_init_arg_still_ok() {
    compile_ok(
        r#"
struct S {
    init(_ n: Any) { }
}
func wrap(_ x) { S(x) }
func main() { wrap(1) }
"#,
    );
}

#[test]
fn c0m_untyped_class_subscript_set_int_s004() {
    let code = compile_fail_code(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func put(_ v) {
    var b = Box()
    b[0] = v
}
func main() { put(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func put(_ v) {
    var b = Box()
    b[0] = v
}
func main() { put(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0m_literal_class_subscript_set_still_ok() {
    compile_ok(
        r#"
class Box {
    func __subscript_get(i: Int) -> Int { return 0 }
    func __subscript_set(i: Int, newValue: Int) { }
}
func main() {
    var b = Box()
    b[0] = 1
}
"#,
    );
}

#[test]
fn c0m_any_class_subscript_set_still_ok() {
    compile_ok(
        r#"
class Box {
    func __subscript_get(i: Int) -> Any { return 0 }
    func __subscript_set(i: Int, newValue: Any) { }
}
func put(_ v) {
    var b = Box()
    b[0] = v
}
func main() { put(1) }
"#,
    );
}

#[test]
fn c0n_untyped_actor_init_arg_int_s004() {
    let code = compile_fail_code(
        r#"
actor Counter {
    init(_ n: Int) { }
}
func wrap(_ x) { Counter(x) }
func main() { wrap(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
actor Counter {
    init(_ n: Int) { }
}
func wrap(_ x) { Counter(x) }
func main() { wrap(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0n_literal_actor_init_arg_still_ok() {
    compile_ok(
        r#"
actor Counter {
    init(_ n: Int) { }
}
func main() { Counter(1) }
"#,
    );
}

#[test]
fn c0n_any_actor_init_arg_still_ok() {
    compile_ok(
        r#"
actor Counter {
    init(_ n: Any) { }
}
func wrap(_ x) { Counter(x) }
func main() { wrap(1) }
"#,
    );
}

#[test]
fn c0o_untyped_dataclass_init_arg_int_s004() {
    let code = compile_fail_code(
        r#"
data class P(x: Int)
func wrap(_ v) { P(v) }
func main() { wrap(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
data class P(x: Int)
func wrap(_ v) { P(v) }
func main() { wrap(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0o_literal_dataclass_init_arg_still_ok() {
    compile_ok(
        r#"
data class P(x: Int)
func main() { P(1) }
"#,
    );
}

#[test]
fn c0o_any_dataclass_init_arg_still_ok() {
    compile_ok(
        r#"
data class P(x: Any)
func wrap(_ v) { P(v) }
func main() { wrap(1) }
"#,
    );
}

#[test]
fn c0a_any_accepts_missing_member_ok() {
    compile_ok(
        r#"
func main() {
    let o: Object = [:]
    let a: Any = o.missing
}
"#,
    );
}

#[test]
fn c0a_object_bag_accepts_missing_member_ok() {
    compile_ok(
        r#"
func main() {
    let o: Object = [:]
    let b: Object = o.missing
}
"#,
    );
}

#[test]
fn c0a_empty_array_still_ok() {
    compile_ok(
        r#"
func main() {
    let xs: Array<Int> = []
}
"#,
    );
}

/// C-0a+3: invalid `super` yields ErrorType — must not soft-assign to ground Int.
#[test]
fn c0ap_error_type_super_not_assignable_to_int() {
    let code = compile_fail_code(
        r#"
func main() {
    let n: Int = super
}
"#,
    );
    // S055 (invalid super) and/or S004 (ErrorType → Int); must fail.
    assert!(
        matches!(code.as_deref(), Some("S055") | Some("S004")),
        "got={code:?}"
    );
}

/// C-0v3: found-Object must not soft-assign to ground Int.
#[test]
fn c0v_object_to_int_s004() {
    let code = compile_fail_code(
        r#"
func façade() -> Object {
    let o: Object = [:]
    return o
}
func main() {
    let n: Int = façade()
}
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
}

#[test]
fn c0a_any_param_missing_member_still_ok() {
    compile_ok(
        r#"
func jStr(v: Any) -> String { return "" }
func main() {
    let o: Object = [:]
    let _ = jStr(v: o.missing)
}
"#,
    );
}

#[test]
fn c0u_untyped_module_type_init_arg_int_s004() {
    let code = compile_fail_code(
        r#"
class Outer {
    class Inner {
        init(_ n: Int) { }
    }
}
func wrap(_ x) { Outer.Inner(x) }
func main() { wrap(1) }
"#,
    );
    assert_eq!(code.as_deref(), Some("S004"), "got={code:?}");
    let msg = compile_fail_message(
        r#"
class Outer {
    class Inner {
        init(_ n: Int) { }
    }
}
func wrap(_ x) { Outer.Inner(x) }
func main() { wrap(1) }
"#,
    )
    .expect("message");
    assert!(msg.contains("expected Int"), "msg={msg}");
    assert!(
        !msg.contains("Unknown"),
        "diag must not print literal Unknown: {msg}"
    );
}

#[test]
fn c0u_literal_module_type_init_still_ok() {
    compile_ok(
        r#"
class Outer {
    class Inner {
        init(_ n: Int) { }
    }
}
func main() { Outer.Inner(1) }
"#,
    );
}

#[test]
fn c0u_any_module_type_init_still_ok() {
    compile_ok(
        r#"
class Outer {
    class Inner {
        init(_ n: Any) { }
    }
}
func wrap(_ x) { Outer.Inner(x) }
func main() { wrap(1) }
"#,
    );
}
