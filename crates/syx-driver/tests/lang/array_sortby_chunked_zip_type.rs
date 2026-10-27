//! Array.sortBy / chunked / zip return-type honesty (todo 5 slice 21–22).

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
fn array_sort_by_preserves_elem_type() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Array<Int> = [3, 1, 2].sortBy({ (x: Int, y: Int) -> Int in return x - y })
    print(a[0])
    print(a[2])
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["1".to_string(), "3".to_string()]);
}

#[test]
fn array_sort_by_assign_to_array_string_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let wrong: Array<String> = [1, 2].sortBy({ (x: Int, y: Int) -> Int in return x - y })
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_chunked_preserves_elem_type() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let c: Array<Array<Int>> = [1, 2, 3, 4].chunked(2)
    print(c.count)
    print(c[0][0])
    print(c[1][1])
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(
        result.printed,
        vec!["2".to_string(), "1".to_string(), "4".to_string()]
    );
}

#[test]
fn array_chunked_assign_to_array_int_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let wrong: Array<Int> = [1, 2, 3, 4].chunked(2)
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_zip_same_elem_preserves_nested_type() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let z: Array<Array<Int>> = [1, 2].zip([3, 4])
    print(z.count)
    print(z[0][0])
    print(z[0][1])
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(
        result.printed,
        vec!["2".to_string(), "1".to_string(), "3".to_string()]
    );
}

#[test]
fn array_zip_hetero_assign_to_array_array_int_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let wrong: Array<Array<Int>> = [1, 2].zip(["a", "b"])
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_zip_hetero_assigns_to_array_array_object() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let z: Array<Array<Object>> = [1, 2].zip(["a", "b"])
    print(z.count)
    print(z[0][0])
    print(z[0][1])
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(
        result.printed,
        vec!["2".to_string(), "1".to_string(), "a".to_string()]
    );
}
