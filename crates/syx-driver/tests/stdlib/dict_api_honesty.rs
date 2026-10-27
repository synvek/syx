//! Todo 5 slice 15: Dictionary method-table honesty — ghost APIs → S002.

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
fn dict_map_values_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    let _ = d.mapValues({ (v: Int) -> Int in return v })
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn dict_merge_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    d.merge(["b": 2])
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn dict_for_each_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    d.forEach({ (k: String, v: Int) -> Void in print(k) })
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn dict_is_empty_is_s002() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    print(d.isEmpty)
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}
