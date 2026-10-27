//! Two top-level functions with the same name are S001 (the first must not silently win).

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
fn duplicate_top_level_func_is_s001() {
    assert_eq!(
        compile_fail_code(
            r#"
func test004() {
    print(1)
}
func test004() {
    print(2)
}
func main() {
    test004()
}
"#
        )
        .as_deref(),
        Some("S001")
    );
}
