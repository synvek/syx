//! Dictionary subscript read/write check key and value types (facade stays Object).
//! Todo 5 slice 24: instance bag APIs on dictionary.sy extension (→ `__syx_dict_*`).

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
fn dict_unknown_append_is_s002_not_codegen() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Double> = {}
    d.append(2.0)
}
"#
        )
        .as_deref(),
        Some("S002")
    );
}

#[test]
fn dict_empty_brace_under_annotation_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var d: Dictionary<String, Double> = {}
    print(d.count)
    d["A"] = 1
    print(d.count)
    print(d["A"])
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
fn dict_empty_colon_bracket_under_annotation_runs() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var d: Dictionary<String, Int> = [:]
    print(d.count)
    d["k"] = 3
    print(d["k"])
}
"#,
        &opts(),
    )
    .expect("run");
    let lines: Vec<_> = result.printed.iter().map(|l| l.trim()).collect();
    assert!(lines.contains(&"0"), "printed={lines:?}");
    assert!(lines.contains(&"3"), "printed={lines:?}");
}

#[test]
fn dict_string_int_assign_and_read() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var d: Dictionary<String, Int> = {"a": 1}
    print(d.count)
    d["b"] = 2
    print(d.count)
    print(d["a"])
}
"#,
        &opts(),
    )
    .expect("run");
    let lines: Vec<_> = result.printed.iter().map(|l| l.trim()).collect();
    assert!(
        lines.windows(2).any(|w| w == ["1", "2"]),
        "count should grow 1→2, printed={lines:?}"
    );
    assert!(
        lines.iter().any(|l| *l == "1"),
        "d[\"a\"] should print 1, printed={lines:?}"
    );
}

#[test]
fn dict_int_double_unsuffixed_int_assign() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var d: Dictionary<Int, Double> = [0: 0.0]
    d[1] = 2
    print(d[1])
}
"#,
        &opts(),
    )
    .expect("run");
    let printed = result.printed.join("\n");
    assert!(
        printed.lines().any(|l| {
            l.trim().parse::<f64>().ok().is_some_and(|n| (n - 2.0).abs() < 1e-9)
        }),
        "printed={printed:?}"
    );
}

#[test]
fn dict_wrong_key_type_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = {"z": 0}
    d[1] = 1
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn dict_wrong_value_type_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = {"z": 0}
    d["a"] = "x"
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn dict_float_value_int_suffix_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Float> = {"z": 0.0}
    d["a"] = 1i8
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn dict_instance_bag_ops_run() {
    syx_host_plugin::register_echo_plugin_in_process();
    let result = compile_and_run_embedded(
        r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1, "b": 2]
    print(d.containsKey("a"))
    print(d.containsKey("z"))
    print(d.keys().count)
    print(d.values().count)
    print(d.remove("a"))
    print(d.containsKey("a"))
    print(d.count)
}
"#,
        &opts(),
    )
    .expect("run");
    assert_eq!(
        result.printed,
        vec![
            "true".to_string(),
            "false".to_string(),
            "2".to_string(),
            "2".to_string(),
            "true".to_string(),
            "false".to_string(),
            "1".to_string(),
        ]
    );
}

#[test]
fn dict_instance_contains_key_wrong_type_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    print(d.containsKey(1))
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}

#[test]
fn dict_instance_remove_wrong_type_is_s004() {
    assert_eq!(
        compile_fail_code(
            r#"
func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    print(d.remove(1))
}
"#
        )
        .as_deref(),
        Some("S004")
    );
}
