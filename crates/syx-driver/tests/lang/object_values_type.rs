//! Object.values return-type honesty (todo 5 slice 27): Array<Object>.

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
fn object_values_assign_to_array_object_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let o: Object = { "a": 1, "b": 2 }
    let vs: Array<Object> = o.values()
    print(vs.count)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["2".to_string()]);
}

#[test]
fn object_values_assign_to_array_int_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let o: Object = { "a": 1 }
    let bad: Array<Int> = o.values()
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}
