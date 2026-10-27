//! Operator overload semantic + parity tests .
// was: operators

use syx_driver::{
    compile_and_run_embedded, compile_file_with_options, run_file_printed, DriverOptions,
};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts()).unwrap().printed
}

fn compile_fail(src: &str, code: &str) -> bool {
    syx_host_plugin::register_echo_plugin_in_process();
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = syx_parser::Parser::parse(file).unwrap();
    let sem = syx_semantic::SemanticAnalyzer::analyze(&module);
    match sem {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

#[test]
fn user_defined_plus_operator() {
    let path = fixture("operators/user_vec.sy");
    compile_file_with_options(&path, &opts()).expect("compile");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "10"));
}

#[test]
fn parity_user_matrix_fixture() {
    let path = fixture("parity/operators/user_matrix.sy");
    compile_file_with_options(&path, &opts()).expect("compile");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "10"));
}

#[test]
fn operator_on_int_rejected_s043() {
    assert!(compile_fail(
        r#"class Bad { operator func +(a: Int, b: Int) -> Int { return 0 } } func main() {}"#,
        "S043"
    ));
}

#[test]
fn operator_on_string_rejected_s044() {
    assert!(compile_fail(
        r#"class Bad { operator func +(a: String, b: String) -> String { return "" } } func main() {}"#,
        "S044"
    ));
}

#[test]
fn operator_on_array_rejected_s047() {
    assert!(compile_fail(
        r#"class Bad { operator func +(a: Array<Int>, b: Array<Int>) -> Array<Int> { return a } } func main() {}"#,
        "S047"
    ));
}

#[test]
fn operator_on_bool_rejected_s043() {
    assert!(compile_fail(
        r#"class Bad { operator func +(a: Bool, b: Bool) -> Bool { return true } } func main() {}"#,
        "S043"
    ));
}

#[test]
fn user_vec_addition_runs() {
    assert!(run(include_str!("../fixtures/operators/user_vec.sy"))
        .iter()
        .any(|l| l.trim() == "10"));
}
