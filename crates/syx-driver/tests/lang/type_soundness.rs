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

#[test]
fn c0a_found_object_to_int_still_ok() {
    compile_ok(
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
