//! C-0b: return type assignability (S004). C-0c: free-function arity (S061).

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
