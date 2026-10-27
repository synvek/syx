//! Array.map/filter return-type honesty (todo 5 slice 11).

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
fn array_map_int_to_int_assigns() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Array<Int> = [1, 2].map({ (x: Int) -> Int in return x * 2 })
    print(a)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result
            .printed
            .iter()
            .any(|l| l.contains('2') && l.contains('4')),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn array_map_int_to_string_assign_to_array_int_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let wrong: Array<Int> = [1].map({ (x: Int) -> String in return "x" })
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_filter_preserves_elem_type() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let f: Array<Int> = [1, 2, 3].filter({ (x: Int) -> Bool in return x > 1 })
    print(f)
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result
            .printed
            .iter()
            .any(|l| l.contains('2') && l.contains('3')),
        "printed={:?}",
        result.printed
    );
}
