//! Array.reduce return-type honesty (todo 5 slice 16).

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
fn array_reduce_int_assigns() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let sum: Int = [1, 2, 3].reduce(0, { (acc: Int, x: Int) -> Int in return acc + x })
    print(sum)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["6"]);
}

#[test]
fn array_reduce_int_assign_to_string_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let wrong: String = [1, 2].reduce(0, { (acc: Int, x: Int) -> Int in return acc + x })
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

/// Regression: per-function MIR `__mir_closure_0` collided across HOF sites, so reduce
/// after map/filter ran the wrong body (often returning 0 / false).
#[test]
fn array_reduce_after_map_in_sibling_funcs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func mapped() {
    let a: Array<Int> = [1, 2].map({ (x: Int) -> Int in return x * 2 })
    print(a)
}
func reduced() {
    let sum: Int = [1, 2, 3].reduce(0, { (acc: Int, x: Int) -> Int in return acc + x })
    print(sum)
}
func main() {
    mapped()
    reduced()
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["[2, 4]", "6"]);
}
