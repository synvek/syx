//! SYX 38.0 driver integration — language and compiler E2E.
// was: integration

use syx_driver::{
    compile_file_with_options, compile_project_with_options, cranelift_backend_retired,
    emit_mir_text, resolve_native_backend, run_file_printed, DriverOptions, NativeBackend,
};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn run(rel: &str) -> (i32, Vec<String>) {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile");
    run_file_printed(&path, &options).expect("run")
}

#[test]
fn manifest_v41() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    assert_eq!(m["phase"], 63);
    assert_eq!(m["version"], "41.0");
}

#[test]
fn deps_function_call_cross_module() {
    syx_host_plugin::register_echo_plugin_in_process();
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/deps");
    let options = DriverOptions::default();
    let project = compile_project_with_options(&root, &options).expect("compile examples/deps");
    let main = root.join(project.manifest.main.as_str());
    let (code, lines) = run_file_printed(&main, &options).expect("run examples/deps");
    assert_eq!(code, 0);
    assert!(
        lines.iter().any(|l| l.trim() == "42"),
        "expected local_math.add(10, 32) → 42, got: {lines:?}"
    );
}

#[test]
fn operator_assign_host() {
    let (code, lines) = run("operators/assign_host.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "15"));
    assert!(lines.iter().any(|l| l.trim() == "12"));
}

#[test]
fn operator_host_matrix() {
    let (code, lines) = run("operators/host_matrix.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "42"));
}

#[test]
fn operator_mir_compare() {
    let mir = emit_mir_text(&fixture("operators/host_matrix.sy"), &DriverOptions::default())
        .unwrap();
    assert!(mir.contains("__syx_int_lt") || mir.contains("CallHost"));
}

#[test]
fn assign_mir_hosts() {
    let mir = emit_mir_text(&fixture("operators/assign_host.sy"), &DriverOptions::default())
        .unwrap();
    assert!(mir.contains("__syx_int_add_assign") || mir.contains("CallHost"));
}

#[test]
fn generic_witness_constraint() {
    let (code, lines) = run("macros/generic_witness_constraint.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "7"));
}

#[test]
fn generic_witness_missing_rejected() {
    assert!(compile_file_with_options(
        &fixture("macros/generic_witness_missing.sy"),
        &DriverOptions::default()
    )
    .is_err());
}

#[test]
fn gc_object_ref() {
    let (code, lines) = run("gc/gc_object_ref.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "dropped"));
    assert!(lines.iter().any(|l| l.trim() == "done"));
}

#[test]
fn attached_conformance_dispatch() {
    let (code, lines) = run("macros/attached_conformance_dispatch.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "true"));
}

#[test]
fn codable_l2_external() {
    let (code, lines) = run("macros/codable_l2_external.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "syx"));
}

#[test]
fn macro_matrix() {
    let (code, lines) = run("macros/macro_matrix.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "7"));
}

#[test]
fn fileprivate_helper_public_api() {
    syx_host_plugin::register_echo_plugin_in_process();
    let root = fixture("access/fileprivate_cross_module/app");
    let options = DriverOptions::default();
    let project = compile_project_with_options(&root, &options).expect("compile fileprivate project");
    let main = root.join(project.manifest.main.as_str());
    let (code, lines) = run_file_printed(&main, &options).expect("run fileprivate project");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "42"));
}

#[test]
fn conformance_cross_module() {
    syx_host_plugin::register_echo_plugin_in_process();
    let root = fixture("projects/conformance_cross_module/app");
    let options = DriverOptions::default();
    let project = compile_project_with_options(&root, &options).expect("compile project");
    let main = root.join(project.manifest.main.as_str());
    let (code, lines) = run_file_printed(&main, &options).expect("run");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "true"));
}

#[test]
fn attached_wrapper_regression() {
    let (code, lines) = run("macros/attached_wrapper.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "3"));
}

#[test]
fn hygiene_escape_regression() {
    let (code, lines) = run("macros/hygiene_escape_ok.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "outer"));
}

#[test]
fn macro_engine_regression() {
    let (code, lines) = run("macros/macro_engine.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "done"));
}

#[test]
fn operator_host_matrix_regression() {
    let (code, lines) = run("operators/host_matrix.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "42"));
}

#[test]
fn gc_object_ref_regression() {
    let (code, lines) = run("gc/gc_object_ref.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "dropped"));
}

#[test]
fn cranelift_removed() {
    assert!(cranelift_backend_retired());
    assert!(NativeBackend::parse("cranelift").is_err());
    assert_eq!(resolve_native_backend(None).unwrap(), NativeBackend::Llvm);
}

#[test]
fn llvm_backend_default() {
    assert_eq!(DriverOptions::default().native_backend, NativeBackend::Llvm);
}

#[test]
fn register_imported_macros() {
    syx_macro::register_imported_macros(&[]);
}

#[test]
fn h002_validate() {
    assert!(syx_host_plugin::validate_compile_plugin_lock().is_ok());
}

#[test]
fn control_flow_no_parens() {
    let (code, lines) = run("syntax/control_flow_no_parens.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "zero"));
    assert!(lines.iter().any(|l| l.trim() == "2"));
}

#[test]
fn as_is_cast() {
    let (code, lines) = run("types/as_is_cast.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "dog"));
    assert!(lines.iter().any(|l| l.trim() == "woof"));
    assert!(lines.iter().any(|l| l.trim() == "not-cat"));
    assert!(lines.iter().any(|l| l.trim() == "switch-cat"));
}

#[test]
fn dict_hashable_key() {
    let (code, lines) = run("types/dict_hashable_key.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "one"));
    assert!(lines.iter().any(|l| l.trim() == "answer"));
    assert!(lines.iter().any(|l| l.trim() == "two"));
    assert!(lines.iter().any(|l| l.trim() == "3"));
}

#[test]
fn nested_types() {
    let (code, lines) = run("types/nested_types.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "7"));
}

#[test]
fn nested_types_cross_module() {
    syx_host_plugin::register_echo_plugin_in_process();
    let root = fixture("nested_cross_module");
    let options = DriverOptions::default();
    let project = compile_project_with_options(&root, &options).expect("compile nested_cross_module");
    let main = root.join(project.manifest.main.as_str());
    let (code, lines) = run_file_printed(&main, &options).expect("run nested_cross_module");
    assert_eq!(code, 0);
    assert!(
        lines.iter().any(|l| l.trim() == "7"),
        "expected nested cross-module value 7, got: {lines:?}"
    );
}

#[test]
fn top_level_stmts_main() {
    let (code, lines) = run("syntax/top_level_stmts.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "top"));
    assert!(lines.iter().any(|l| l.trim() == "main"));
}

#[test]
fn top_level_stmts_rejected_in_lib() {
    assert!(compile_project_with_options(
        &fixture("syntax/top_level_rejected"),
        &DriverOptions::default()
    )
    .is_err());
}

#[test]
fn fileprivate_same_file() {
    let (code, lines) = run("access/fileprivate_local.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "42"));
}

#[test]
fn array_and_dict_description_in_vm() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("collection/array_dict_props.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile");
    let (code, lines) = run_file_printed(&path, &options).expect("run");
    assert_eq!(code, 0);
    assert!(
        lines.first().is_some_and(|l| l.contains('a') && l.contains('b')),
        "array description should list elements, got: {lines:?}"
    );
    assert_eq!(lines.get(1).map(String::as_str), Some("2"));
    assert!(
        lines.get(2).is_some_and(|l| l.contains('1') && l.contains("10")),
        "dict description should list entries, got: {lines:?}"
    );
}