//! Array.flatMap return-type honesty (todo 5 slice 14).

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
fn array_flat_map_array_int_assigns() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Array<Int> = [1, 2].flatMap({ (x: Int) -> Array<Int> in return [x, x] })
    print(a.count)
    print(a[0])
    print(a[2])
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["4", "1", "2"]);
}

#[test]
fn array_flat_map_array_string_assign_to_array_int_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let wrong: Array<Int> = [1].flatMap({ (x: Int) -> Array<String> in return ["x"] })
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}
