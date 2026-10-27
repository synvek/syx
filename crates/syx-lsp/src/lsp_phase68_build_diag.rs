// Phase 69 build.syb diagnostic gate (≤5 real S401/S402/S403 + cycle).

use super::*;

fn p69_build_has_code(text: &str, code: &str) {
    use syx_diagnostics::SourceFile;
    use syx_driver::build_syx::diagnose_build_syx;
    let codes: Vec<_> = diagnose_build_syx(&SourceFile::new("build.syb", text))
        .into_iter()
        .filter_map(|d| d.code)
        .collect();
    assert!(codes.iter().any(|c| c == code), "expected {code}, got {codes:?}");
}

#[test]
fn p69_build_s401_missing_dep() {
    p69_build_has_code(
        r#"task a {
  dependsOn = ["missing_x"]
  target = "mir"
}"#,
        "S401",
    );
}

#[test]
fn p69_build_s402_cycle() {
    p69_build_has_code(
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
fn p69_build_s403_unknown_target() {
    p69_build_has_code(
        r#"task t {
  target = "unknown_target_xyz"
}"#,
        "S403",
    );
}

#[test]
fn p69_build_s401_named_task() {
    p69_build_has_code(
        r#"task compile {
  dependsOn = ["no_such_task"]
  target = "mir"
}"#,
        "S401",
    );
}

#[test]
fn p69_build_s401_multi_dep() {
    p69_build_has_code(
        r#"task t {
  dependsOn = ["a", "missing_b"]
  target = "mir"
}"#,
        "S401",
    );
}
