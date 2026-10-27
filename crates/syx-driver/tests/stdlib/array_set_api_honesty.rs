//! Todo 5 slice 17: Array/Set method-table honesty — ghost APIs → S002.

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
fn array_is_empty_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Array<Int> = [1]
    print(a.isEmpty)
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn array_for_each_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Array<Int> = [1]
    a.forEach({ (x: Int) -> Void in print(x) })
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn array_all_satisfy_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Array<Int> = [1]
    let _ = a.allSatisfy({ (x: Int) -> Bool in return x > 0 })
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn array_reversed_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Array<Int> = [1, 2]
    let _ = a.reversed()
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn set_is_empty_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let s: Set<Int> = [1]
    print(s.isEmpty)
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn set_subtraction_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let s: Set<Int> = [1, 2]
    let _ = s.subtraction([1])
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn set_for_each_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let s: Set<Int> = [1]
    s.forEach({ (x: Int) -> Void in print(x) })
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}
