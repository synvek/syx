//! build.syb LSP / analyze diagnostics (Phase 66).
// was: build_syx_lsp

use syx_diagnostics::SourceFile;
use syx_driver::build_syx::diagnose_build_syx;
use syx_lsp::semantic_recheck_after_change;
use syx_driver::DriverOptions;

fn diag_codes(text: &str) -> Vec<String> {
    let source = SourceFile::new("build.syb", text);
    diagnose_build_syx(&source)
        .into_iter()
        .filter_map(|d| d.code)
        .collect()
}

fn lsp_codes(text: &str) -> Vec<String> {
    semantic_recheck_after_change("file:///proj/build.syb", text, &DriverOptions::default())
        .into_iter()
        .filter_map(|v| v.get("code").and_then(|c| c.as_str()).map(str::to_string))
        .collect()
}

#[test]
fn build_syx_unknown_target_s403() {
    let text = r#"task bad {
    target = "fly"
}"#;
    assert!(diag_codes(text).contains(&"S403".into()));
}

#[test]
fn build_syx_unknown_dep_s401() {
    let text = r#"task a {
    dependsOn = ["missing"]
    target = "mir"
}"#;
    assert!(diag_codes(text).contains(&"S401".into()));
}

#[test]
fn build_syx_cycle_s402() {
    let text = r#"task a {
    dependsOn = ["b"]
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "mir"
}"#;
    assert!(diag_codes(text).contains(&"S402".into()));
}

#[test]
fn build_syx_valid_mir_ok() {
    let text = r#"task build {
    target = "mir"
}"#;
    assert!(diag_codes(text).is_empty());
}

#[test]
fn build_syx_valid_wasm_ok() {
    let text = r#"task wasm {
    target = "wasm"
}"#;
    assert!(diag_codes(text).is_empty());
}

#[test]
fn build_syx_valid_native_ok() {
    let text = r#"task native {
    target = "native"
}"#;
    assert!(diag_codes(text).is_empty());
}

#[test]
fn build_syx_valid_link_ok() {
    let text = r#"task link {
    target = "link"
}"#;
    assert!(diag_codes(text).is_empty());
}

#[test]
fn build_syx_valid_test_ok() {
    let text = r#"task tests {
    target = "test"
}"#;
    assert!(diag_codes(text).is_empty());
}

#[test]
fn build_syx_dag_chain_ok() {
    let text = r#"task a {
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "wasm"
}"#;
    assert!(diag_codes(text).is_empty());
}

#[test]
fn lsp_build_syx_unknown_target() {
    let text = r#"task x {
    target = "docker"
}"#;
    assert!(lsp_codes(text).contains(&"S403".into()));
}

#[test]
fn lsp_build_syx_cycle() {
    let text = r#"task a {
    dependsOn = ["b"]
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "mir"
}"#;
    assert!(lsp_codes(text).contains(&"S402".into()));
}

#[test]
fn lsp_build_syx_unknown_dep() {
    let text = r#"task solo {
    dependsOn = ["nope"]
    target = "mir"
}"#;
    assert!(lsp_codes(text).contains(&"S401".into()));
}

#[test]
fn build_syx_multiple_unknown_targets() {
    let text = r#"task a {
    target = "x"
}
task b {
    target = "y"
}"#;
    let codes = diag_codes(text);
    assert_eq!(codes.iter().filter(|c| *c == "S403").count(), 2);
}

#[test]
fn build_syx_unknown_target_span_message() {
    let source = SourceFile::new(
        "build.syb",
        "task t {\n    target = \"bogus\"\n}",
    );
    let diags = diagnose_build_syx(&source);
    assert!(diags.iter().any(|d| d.code.as_deref() == Some("S403")));
    assert!(diags[0].message.contains("bogus"));
}

#[test]
fn build_syx_compile_options_no_diag() {
    let text = "compileOptions {\n    optLevel = release\n}";
    assert!(diag_codes(text).is_empty());
}

#[test]
fn build_syx_empty_tasks_ok() {
    assert!(diag_codes("features {\n    wasm_aot = true\n}").is_empty());
}

#[test]
fn build_syx_self_cycle_single_task_ignored() {
    let text = r#"task a {
    dependsOn = ["a"]
    target = "mir"
}"#;
    assert!(diag_codes(text).contains(&"S402".into()));
}

#[test]
fn build_syx_known_targets_list() {
    for target in ["mir", "wasm", "native", "link", "test"] {
        let text = format!("task t {{ target = \"{target}\" }}");
        assert!(diag_codes(&text).is_empty(), "target {target}");
    }
}

#[test]
fn build_syx_goto_def_task_name() {
    use syx_driver::build_syx::definition_for_build_syx;
    // line 4 = `    dependsOn = ["alpha"]` — cursor on "alpha"
    let text = "task alpha {\n    target = \"mir\"\n}\ntask beta {\n    dependsOn = [\"alpha\"]\n    target = \"wasm\"\n}";
    let loc = definition_for_build_syx(text, 4, 20);
    assert!(loc.is_some());
    let (line, _col) = loc.unwrap();
    assert_eq!(line, 0);
}

#[test]
fn build_syx_goto_def_depends_on() {
    use syx_driver::build_syx::definition_for_build_syx;
    let text = "task alpha {\n    target = \"mir\"\n}\ntask beta {\n    dependsOn = [\"alpha\"]\n    target = \"wasm\"\n}";
    let loc = definition_for_build_syx(text, 4, 20);
    assert!(loc.is_some());
}
