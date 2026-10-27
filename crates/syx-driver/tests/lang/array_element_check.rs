//! Array append/contains check the element type (facade stays Object).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn compile_fail_message(src: &str) -> Option<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = match syx_parser::Parser::parse(file) {
        Ok(m) => m,
        Err(d) => return Some(d.message.clone()),
    };
    match syx_semantic::SemanticAnalyzer::analyze(&module) {
        Err(d) => Some(d.message.clone()),
        Ok(a) => a.errors.first().map(|e| e.message.clone()),
    }
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
fn array_float_append_unsuffixed_int_grows() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Array<Float> = []
    print(a.count)
    a.append(2)
    print(a.count)
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
fn array_int_append_grows() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Array<Int> = []
    print(a.count)
    a.append(1)
    print(a.count)
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
fn array_int_contains() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    let a: Array<Int> = [1]
    print(a.contains(1))
}
"#,
        &opts(),
    )
    .expect("run");
    assert!(
        result.printed.iter().any(|l| l.trim() == "true" || l.trim() == "1"),
        "printed={:?}",
        result.printed
    );
}

#[test]
fn array_float_append_string_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Array<Float> = []
    a.append("2")
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn array_double_append_int_suffix_names_double() {
    let message = compile_fail_message(
        r#"
func main() {
    let a: Array<Double> = []
    a.append(2i8)
}
"#,
    )
    .unwrap_or_default();
    assert!(
        message.contains("expected Double"),
        "diagnostic should keep the written alias, got: {message}"
    );
    assert!(
        !message.contains("Float64"),
        "diagnostic should not rename Double to Float64, got: {message}"
    );
}

#[test]
fn array_float64_append_int_suffix_names_float64() {
    let message = compile_fail_message(
        r#"
func main() {
    let a: Array<Float64> = []
    a.append(2i8)
}
"#,
    )
    .unwrap_or_default();
    assert!(
        message.contains("expected Float64"),
        "explicit Float64 should stay Float64, got: {message}"
    );
}

#[test]
fn array_float_append_int_suffix_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    let a: Array<Float> = []
    a.append(2i8)
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}
