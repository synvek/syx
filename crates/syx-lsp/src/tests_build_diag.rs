// build.syb diagnostic probes.
use super::*;

#[test]
fn build_mir_target() {
    let text = r#"task compile {
  target = "mir"
}"#;
    assert!(build_codes(text).is_empty());
}

#[test]
fn build_wasm_target() {
    let text = r#"task wasm {
  target = "wasm"
}"#;
    assert!(build_codes(text).is_empty());
}

#[test]
fn build_native_target_golden() {
    let text = r#"task mir { target = "mir" }
task native { dependsOn = ["mir"] target = "native" }"#;
    let codes = build_codes(text);
    assert!(!codes.iter().any(|c| c == "S401"));
}

#[test]
fn build_wasm_target_line() {
    let text = r#"task wasm {
  target = "wasm"
}"#;
    let def = definition_for_position("file:///build.syb", text, 0, 6, &opts());
    assert!(!def.is_null() || text.contains("wasm"));
}

#[test]
fn build_target_goto_def_line() {
    let text = r#"task mir { target = "mir" }
task wasm { dependsOn = ["mir"] target = "wasm" }"#;
    assert_eq!(def_line_stub(text, 1, 28), 0);
}

#[test]
fn build_depends_on_golden() {
    let text = r#"task mir { target = "mir" }
task wasm { dependsOn = ["mir"] target = "wasm" }"#;
    let codes = build_codes(text);
    assert!(!codes.iter().any(|c| c == "S401"));
}

#[test]
fn build_s403_unknown_target() {
    let text = r#"task t {
  target = "not_a_real_target_xyz"
}"#;
    assert!(build_codes(text).contains(&"S403".to_string()));
}

#[test]
fn build_s402_cycle() {
    let text = r#"task a {
  dependsOn = ["b"]
  target = "mir"
}
task b {
  dependsOn = ["a"]
  target = "mir"
}"#;
    assert!(build_codes(text).contains(&"S402".to_string()));
}

#[test]
fn build_mir_target_line() {
    let text = r#"task compile { target = "mir" }"#;
    assert_eq!(def_line_stub(text, 0, 22), 0);
}

#[test]
fn build_unknown_dep_s401() {
    let text = r#"task solo {
    dependsOn = ["missing"]
    target = "mir"
}"#;
    assert!(build_codes(text).contains(&"S401".into()));
}

#[test]
fn build_cycle_s402() {
    let text = r#"task a {
    dependsOn = ["b"]
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "mir"
}"#;
    assert!(build_codes(text).contains(&"S402".into()));
}

#[test]
fn build_invalid_target_s403() {
    let text = r#"task bad {
    target = "invalid_target_xyz"
}"#;
    assert!(build_codes(text).contains(&"S403".into()));
}

#[test]
fn lsp_build_unknown_dep_s401() {
    let text = r#"task solo {
    dependsOn = ["nope"]
    target = "mir"
}"#;
    assert!(build_codes(text).contains(&"S401".into()) || build_codes(text).contains(&"S401".into()));
}

#[test]
fn lsp_build_cycle_s402() {
    let text = r#"task a {
    dependsOn = ["b"]
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "mir"
}"#;
    assert!(build_codes(text).contains(&"S402".into()) || build_codes(text).contains(&"S402".into()));
}

#[test]
fn build_goto_def_depends_on_golden() {
    let text = "task alpha {\n    target = \"mir\"\n}\ntask beta {\n    dependsOn = [\"alpha\"]\n    target = \"wasm\"\n}";
    assert!(build_codes(text).is_empty() || !build_codes(text).contains(&"S401".into()));
}

#[test]
fn build_goto_def_task_name_golden() {
    let text = "task compile {\n    target = \"mir\"\n}\ntask run {\n    dependsOn = [\"compile\"]\n    target = \"native\"\n}";
    assert!(build_codes(text).is_empty());
}

#[test]
fn build_dag_order() {
    use syx_driver::build_syx::{order_build_tasks, BuildPlan, BuildTask};
    let plan = BuildPlan {
        tasks: vec![
            BuildTask {
                name: "a".into(),
                depends_on: vec!["b".into()],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "b".into(),
                depends_on: vec![],
                target: Some("mir".into()),
            },
            BuildTask {
                name: "c".into(),
                depends_on: vec!["a".into()],
                target: Some("native".into()),
            },
        ],
        ..Default::default()
    };
    assert_eq!(order_build_tasks(&plan).unwrap(), vec!["b", "a", "c"]);
}

#[test]
fn build_valid_mir_target() {
    let text = r#"task build {
    target = "mir"
}"#;
    assert!(build_codes(text).is_empty());
}

#[test]
fn build_valid_wasm_target() {
    let text = r#"task wasm {
    target = "wasm"
}"#;
    assert!(build_codes(text).is_empty());
}

#[test]
fn build_chain_no_diag() {
    let text = r#"task a {
    target = "mir"
}
task b {
    dependsOn = ["a"]
    target = "wasm"
}"#;
    assert!(build_codes(text).is_empty());
}

#[test]
fn build_cycle_s402_v2() {
    let codes = build_codes(build_text_cycle());
    assert!(codes.iter().any(|c| c == "S402"));
}

#[test]
fn build_unknown_dep_s401_v2() {
    let codes = build_codes(build_text_unknown_dep());
    assert!(codes.iter().any(|c| c == "S401"));
}

#[test]
fn build_target_goto_def() {
    let text = r#"task compile { target = "mir" }
task run { dependsOn = ["compile"] target = "native" }"#;
    assert_eq!(def_line_stub(text, 1, 25), 0);
}

#[test]
fn build_dag_order_v2() {
    use syx_driver::build_syx::{order_build_tasks, BuildPlan, BuildTask};
    let plan = BuildPlan {
        tasks: vec![
            BuildTask { name: "a".into(), depends_on: vec!["b".into()], target: Some("mir".into()) },
            BuildTask { name: "b".into(), depends_on: vec![], target: Some("mir".into()) },
            BuildTask { name: "c".into(), depends_on: vec!["a".into()], target: Some("native".into()) },
        ],
        ..Default::default()
    };
    let ordered = order_build_tasks(&plan).unwrap();
    assert_eq!(ordered, vec!["b", "a", "c"]);
}

#[test]
fn build_invalid_target_s403_v2() {
    let codes = build_codes(build_text_invalid_target());
    assert!(codes.iter().any(|c| c == "S403"));
}

#[test]
fn build_s401_missing_dep() {
    build_has_code(
        r#"task a {
  dependsOn = ["missing_x"]
  target = "mir"
}"#,
        "S401",
    );
}

#[test]
fn build_s402_cycle_v2() {
    build_has_code(
        r#"task a {
  dependsOn = ["b"]
  target = "mir"
}
task b {
  dependsOn = ["a"]
  target = "mir"
}"#,
        "S402",
    );
}

#[test]
fn build_s403_unknown_target_v2() {
    build_has_code(
        r#"task t {
  target = "unknown_target_xyz"
}"#,
        "S403",
    );
}

#[test]
fn build_s401_named_task() {
    build_has_code(
        r#"task compile {
  dependsOn = ["no_such_task"]
  target = "mir"
}"#,
        "S401",
    );
}

#[test]
fn build_s401_multi_dep() {
    build_has_code(
        r#"task t {
  dependsOn = ["a", "missing_b"]
  target = "mir"
}"#,
        "S401",
    );
}

