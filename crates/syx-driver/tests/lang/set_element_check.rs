//! Set insert/remove/contains check the element type (facade stays Object).
//! Todo 5 slice 23: instance `contains` is declared on set.sy extension (→ `__syx_set_contains`).

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
fn set_float_insert_unsuffixed_int_grows() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let s: Set<Float> = []
    print(s.count)
    s.insert(2)
    print(s.count)
}
"#,
        &opts(),
    )
    .expect("run");
    let lines: Vec<_> = result.printed.iter().map(|l| l.trim()).collect();
    assert!(
        lines.windows(2).any(|w| w == ["0", "1"]),
        "printed={lines:?}"
    );
}

#[test]
fn set_int_insert_grows() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let s: Set<Int> = []
    print(s.count)
    s.insert(1)
    print(s.count)
}
"#,
        &opts(),
    )
    .expect("run");
    let lines: Vec<_> = result.printed.iter().map(|l| l.trim()).collect();
    assert!(
        lines.windows(2).any(|w| w == ["0", "1"]),
        "printed={lines:?}"
    );
}

#[test]
fn set_int_remove_and_contains() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let s: Set<Int> = [1]
    print(s.contains(1))
    s.remove(1)
    print(s.count)
}
"#,
        &opts(),
    )
    .expect("run");
    let lines: Vec<_> = result.printed.iter().map(|l| l.trim().to_string()).collect();
    assert!(
        lines.iter().any(|l| l == "true" || l == "1"),
        "contains printed={lines:?}"
    );
    assert!(lines.iter().any(|l| l == "0"), "count after remove={lines:?}");
}

#[test]
fn set_instance_contains_true() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let s: Set<Int> = [1, 2]
    print(s.contains(2))
    print(s.contains(9))
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(result.printed, vec!["true".to_string(), "false".to_string()]);
}

#[test]
fn set_instance_contains_wrong_elem_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let s: Set<Int> = [1]
    print(s.contains("x"))
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn set_float_insert_string_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let s: Set<Float> = []
    s.insert("2")
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn set_float_insert_int_suffix_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let s: Set<Float> = []
    s.insert(2i8)
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}
