//! Dictionary.keys/values assign-type honesty (todo 5 slice 28): Array<K>/Array<V>.
//! Sema already returns typed arrays; facade stays bare Array (book note).

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
fn dict_keys_assign_to_array_string_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1, "b": 2]
    let ks: Array<String> = d.keys()
    print(ks.count)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["2".to_string()]);
}

#[test]
fn dict_values_assign_to_array_int_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1, "b": 2]
    let vs: Array<Int> = d.values()
    print(vs.count)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["2".to_string()]);
}

#[test]
fn dict_keys_assign_to_array_int_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    let bad: Array<Int> = d.keys()
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn dict_values_assign_to_array_string_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    let bad: Array<String> = d.values()
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}
