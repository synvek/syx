//! `syx fmt --check` MVP corpus nails (F001 + idempotent slices).

use std::fs;
use std::path::PathBuf;

use syx_diagnostics::SourceFile;
use syx_driver::{format_check_path, format_check_path_cst};
use syx_lexer::{Lexer, TokenKind};

#[test]
fn fmt_check_reports_f001() {
    let dir = tempfile_dir();
    let main = dir.join("main.sy");
    // W874: lazy computed formats; if-case KeyPath pattern remains F001.
    let original = "func main() {\n    if case \\Foo.bar = x {\n        print(1)\n    }\n}\n";
    fs::write(&main, original).expect("write");
    let ok_file = dir.join("ok.sy");
    fs::write(&ok_file, "func main() {\n    print(1)\n}\n").expect("write ok");

    let report = format_check_path(&dir).expect("check must return report");
    assert!(report.checked >= 2, "checked={}", report.checked);
    assert!(
        report.failed >= 1,
        "expected F001 failure; failures={:?}",
        report.failures
    );
    assert!(
        report
            .failures
            .iter()
            .any(|(_, m)| m.contains("F001") || m.contains("unsupported")),
        "failures={:?}",
        report.failures
    );
    let after = fs::read_to_string(&main).expect("read");
    assert_eq!(after, original, "format --check must not write on F001");
    let after_ok = fs::read_to_string(&ok_file).expect("read ok");
    assert!(
        after_ok.contains("func main"),
        "ok file must remain untouched"
    );
}

#[test]
fn fmt_check_idempotent_narrow_ok() {
    let dir = tempfile_dir();
    let main = dir.join("main.sy");
    let src = "func main() {\n    print(1)\n}\n";
    fs::write(&main, src).expect("write");
    let report = format_check_path(&main).expect("check");
    assert_eq!(report.checked, 1);
    assert_eq!(report.failed, 0, "failures={:?}", report.failures);
    if report.would_change > 0 {
        use syx_diagnostics::SourceFile;
        use syx_fmt::format_dry_run;
        let dry = format_dry_run(&SourceFile::new("main.sy", src)).unwrap();
        fs::write(&main, &dry.formatted).unwrap();
        let report2 = format_check_path(&main).expect("recheck");
        assert!(report2.ok(), "after normalize: {:?}", report2.failures);
    } else {
        assert!(report.ok());
    }
}

#[test]
fn fmt_check_differential_suite1_ok() {
    let suite1 = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/differential/suite1");
    let report = format_check_path(&suite1).expect("suite1 check");
    assert_eq!(
        report.checked, 8,
        "suite1 has 8 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "suite1 must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_class_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/class_mvp");
    let report = format_check_path(&dir).expect("class_mvp check");
    assert!(report.checked >= 3, "got {}", report.checked);
    assert!(
        report.ok(),
        "class_mvp must be green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_ext_proto_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/ext_proto_mvp");
    let report = format_check_path(&dir).expect("ext_proto_mvp check");
    assert!(report.checked >= 3, "got {}", report.checked);
    assert!(
        report.ok(),
        "ext_proto_mvp must be green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_actor_deinit_mvp_ok() {
    // W687 / ADR-1031: narrow Actor/Deinit MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/actor_deinit_mvp");
    let report = format_check_path(&dir).expect("actor_deinit_mvp check");
    assert!(
        report.checked >= 3,
        "actor_deinit_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W687: actor_deinit_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_enum_typealias_mvp_ok() {
    // W688 / ADR-1032: narrow Enum/TypeAlias MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/enum_typealias_mvp");
    let report = format_check_path(&dir).expect("enum_typealias_mvp check");
    assert!(
        report.checked >= 4,
        "enum_typealias_mvp has >=4 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W688: enum_typealias_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_dataclass_mvp_ok() {
    // W689 / ADR-1033: narrow DataClass MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/dataclass_mvp");
    let report = format_check_path(&dir).expect("dataclass_mvp check");
    assert!(
        report.checked >= 3,
        "dataclass_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W689: dataclass_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_finalclass_mvp_ok() {
    // W690 / ADR-1034: narrow FinalClass MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/finalclass_mvp");
    let report = format_check_path(&dir).expect("finalclass_mvp check");
    assert!(
        report.checked >= 3,
        "finalclass_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W690: finalclass_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_globalactor_mvp_ok() {
    // W691 / ADR-1035: narrow GlobalActor MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/globalactor_mvp");
    let report = format_check_path(&dir).expect("globalactor_mvp check");
    assert!(
        report.checked >= 3,
        "globalactor_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W691: globalactor_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_subscript_mvp_ok() {
    // W692 / ADR-1036: narrow Subscript get-only MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/subscript_mvp");
    let report = format_check_path(&dir).expect("subscript_mvp check");
    assert!(
        report.checked >= 3,
        "subscript_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W692: subscript_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_observers_mvp_ok() {
    // W693 / ADR-1037: narrow observers MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/observers_mvp");
    let report = format_check_path(&dir).expect("observers_mvp check");
    assert!(
        report.checked >= 3,
        "observers_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W693: observers_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_subscript_setter_mvp_ok() {
    // W694 / ADR-1038: narrow Subscript get+set MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/subscript_setter_mvp");
    let report = format_check_path(&dir).expect("subscript_setter_mvp check");
    assert!(
        report.checked >= 3,
        "subscript_setter_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W694: subscript_setter_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_computed_mvp_ok() {
    // W695 / ADR-1039: narrow computed get-only MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/computed_mvp");
    let report = format_check_path(&dir).expect("computed_mvp check");
    assert!(
        report.checked >= 3,
        "computed_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W695: computed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_lazy_mvp_ok() {
    // W696 / ADR-1040: narrow lazy var MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/lazy_mvp");
    let report = format_check_path(&dir).expect("lazy_mvp check");
    assert!(
        report.checked >= 3,
        "lazy_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W696: lazy_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_lazy_computed_bare_mvp_ok() {
    // W874 / ADR-1218: lazy computed get-only MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/lazy_computed_bare_mvp");
    let report = format_check_path(&dir).expect("lazy_computed_bare_mvp check");
    assert!(
        report.checked >= 5,
        "lazy_computed_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W874: lazy_computed_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_lazy_computed_getset_mvp_ok() {
    // W874 / ADR-1218: lazy computed get+set MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/lazy_computed_getset_mvp");
    let report = format_check_path(&dir).expect("lazy_computed_getset_mvp check");
    assert!(
        report.checked >= 5,
        "lazy_computed_getset_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W874: lazy_computed_getset_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_lazy_computed_mixed_mvp_ok() {
    // W874 / ADR-1218: lazy computed mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/lazy_computed_mixed_mvp");
    let report = format_check_path(&dir).expect("lazy_computed_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "lazy_computed_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W874: lazy_computed_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_computed_setter_mvp_ok() {
    // W697 / ADR-1041: narrow computed get+set MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/computed_setter_mvp");
    let report = format_check_path(&dir).expect("computed_setter_mvp check");
    assert!(
        report.checked >= 3,
        "computed_setter_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W697: computed_setter_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_override_var_mvp_ok() {
    // W698 / ADR-1042: narrow override var MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/override_var_mvp");
    let report = format_check_path(&dir).expect("override_var_mvp check");
    assert!(
        report.checked >= 3,
        "override_var_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W698: override_var_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_operator_func_mvp_ok() {
    // W699 / ADR-1043: narrow OperatorFunc MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/operator_func_mvp");
    let report = format_check_path(&dir).expect("operator_func_mvp check");
    assert!(
        report.checked >= 3,
        "operator_func_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W699: operator_func_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_macro_decl_mvp_ok() {
    // W700 / ADR-1044: narrow MacroDecl User MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/macro_decl_mvp");
    let report = format_check_path(&dir).expect("macro_decl_mvp check");
    assert!(
        report.checked >= 3,
        "macro_decl_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W700: macro_decl_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_macro_site_invoke_mvp_ok() {
    // W701 / ADR-1045: narrow MacroSiteInvoke MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/macro_site_invoke_mvp");
    let report = format_check_path(&dir).expect("macro_site_invoke_mvp check");
    assert!(
        report.checked >= 3,
        "macro_site_invoke_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W701: macro_site_invoke_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_macro_invoke_mvp_ok() {
    // W702 / ADR-1046: narrow Expr/Stmt MacroInvoke MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/macro_invoke_mvp");
    let report = format_check_path(&dir).expect("macro_invoke_mvp check");
    assert!(
        report.checked >= 3,
        "macro_invoke_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W702: macro_invoke_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_toplevel_stmt_mvp_ok() {
    // W703 / ADR-1047: narrow TopLevelStmt MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/toplevel_stmt_mvp");
    let report = format_check_path(&dir).expect("toplevel_stmt_mvp check");
    assert!(
        report.checked >= 3,
        "toplevel_stmt_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W703: toplevel_stmt_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_macro_builtin_mvp_ok() {
    // W704 / ADR-1048: narrow BuiltIn MacroDecl MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/macro_builtin_mvp");
    let report = format_check_path(&dir).expect("macro_builtin_mvp check");
    assert!(
        report.checked >= 3,
        "macro_builtin_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W704: macro_builtin_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_macro_external_mvp_ok() {
    // W705 / ADR-1049: narrow External MacroDecl MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/macro_external_mvp");
    let report = format_check_path(&dir).expect("macro_external_mvp check");
    assert!(
        report.checked >= 3,
        "macro_external_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W705: macro_external_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_macro_decl_body_mvp_ok() {
    // W706 / ADR-1050: narrow nonempty decl_body MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/macro_decl_body_mvp");
    let report = format_check_path(&dir).expect("macro_decl_body_mvp check");
    assert!(
        report.checked >= 3,
        "macro_decl_body_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W706: macro_decl_body_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_throw_mvp_ok() {
    // W707 / ADR-1051: narrow Throw MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/throw_mvp");
    let report = format_check_path(&dir).expect("throw_mvp check");
    assert!(
        report.checked >= 3,
        "throw_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W707: throw_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_try_mvp_ok() {
    // W708 / ADR-1052: narrow Try MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/try_mvp");
    let report = format_check_path(&dir).expect("try_mvp check");
    assert!(
        report.checked >= 3,
        "try_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W708: try_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_closure_mvp_ok() {
    // W709 / ADR-1053: narrow Closure MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/closure_mvp");
    let report = format_check_path(&dir).expect("closure_mvp check");
    assert!(
        report.checked >= 3,
        "closure_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W709: closure_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_await_mvp_ok() {
    // W710 / ADR-1054: narrow Await MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/await_mvp");
    let report = format_check_path(&dir).expect("await_mvp check");
    assert!(
        report.checked >= 3,
        "await_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W710: await_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_force_unwrap_mvp_ok() {
    // W711 / ADR-1055: narrow ForceUnwrap MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/force_unwrap_mvp");
    let report = format_check_path(&dir).expect("force_unwrap_mvp check");
    assert!(
        report.checked >= 3,
        "force_unwrap_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W711: force_unwrap_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cast_mvp_ok() {
    // W712 / ADR-1056: narrow Cast MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cast_mvp");
    let report = format_check_path(&dir).expect("cast_mvp check");
    assert!(
        report.checked >= 3,
        "cast_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W712: cast_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_optional_cast_mvp_ok() {
    // W713 / ADR-1057: narrow OptionalCast MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/optional_cast_mvp");
    let report = format_check_path(&dir).expect("optional_cast_mvp check");
    assert!(
        report.checked >= 3,
        "optional_cast_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W713: optional_cast_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_is_mvp_ok() {
    // W714 / ADR-1058: narrow Is MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/is_mvp");
    let report = format_check_path(&dir).expect("is_mvp check");
    assert!(
        report.checked >= 3,
        "is_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W714: is_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_type_self_mvp_ok() {
    // W715 / ADR-1059: narrow TypeSelf MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/type_self_mvp");
    let report = format_check_path(&dir).expect("type_self_mvp check");
    assert!(
        report.checked >= 3,
        "type_self_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W715: type_self_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_optional_chain_mvp_ok() {
    // W716 / ADR-1060: narrow OptionalChain MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/optional_chain_mvp");
    let report = format_check_path(&dir).expect("optional_chain_mvp check");
    assert!(
        report.checked >= 3,
        "optional_chain_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W716: optional_chain_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_half_open_range_mvp_ok() {
    // W717 / ADR-1061: narrow HalfOpenRange MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/half_open_range_mvp");
    let report = format_check_path(&dir).expect("half_open_range_mvp check");
    assert!(
        report.checked >= 3,
        "half_open_range_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W717: half_open_range_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_closed_range_mvp_ok() {
    // W718 / ADR-1062: narrow ClosedRange MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/closed_range_mvp");
    let report = format_check_path(&dir).expect("closed_range_mvp check");
    assert!(
        report.checked >= 3,
        "closed_range_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W718: closed_range_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_mvp_ok() {
    // W719 / ADR-1063: narrow Switch MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_mvp");
    let report = format_check_path(&dir).expect("switch_mvp check");
    assert!(
        report.checked >= 3,
        "switch_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W719: switch_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_if_let_mvp_ok() {
    // W720 / ADR-1064: narrow IfLet MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/if_let_mvp");
    let report = format_check_path(&dir).expect("if_let_mvp check");
    assert!(
        report.checked >= 3,
        "if_let_mvp has >=3 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W720: if_let_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_guard_let_mvp_ok() {
    // W721 / ADR-1065: narrow GuardLet MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/guard_let_mvp");
    let report = format_check_path(&dir).expect("guard_let_mvp check");
    assert!(
        report.checked >= 5,
        "guard_let_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W721: guard_let_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_repeat_while_mvp_ok() {
    // W722 / ADR-1066: narrow RepeatWhile MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/repeat_while_mvp");
    let report = format_check_path(&dir).expect("repeat_while_mvp check");
    assert!(
        report.checked >= 5,
        "repeat_while_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W722: repeat_while_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_if_case_mvp_ok() {
    // W723 / ADR-1067: narrow IfCase MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/if_case_mvp");
    let report = format_check_path(&dir).expect("if_case_mvp check");
    assert!(
        report.checked >= 5,
        "if_case_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W723: if_case_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_tuple_pattern_mvp_ok() {
    // W724 / ADR-1068: narrow TuplePattern MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/tuple_pattern_mvp");
    let report = format_check_path(&dir).expect("tuple_pattern_mvp check");
    assert!(
        report.checked >= 5,
        "tuple_pattern_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W724: tuple_pattern_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_task_block_mvp_ok() {
    // W725 / ADR-1069: narrow TaskBlock MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/task_block_mvp");
    let report = format_check_path(&dir).expect("task_block_mvp check");
    assert!(
        report.checked >= 5,
        "task_block_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W725: task_block_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_borrow_mvp_ok() {
    // W726 / ADR-1070: narrow Borrow MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/borrow_mvp");
    let report = format_check_path(&dir).expect("borrow_mvp check");
    assert!(
        report.checked >= 5,
        "borrow_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W726: borrow_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_keypath_mvp_ok() {
    // W727 / ADR-1071: narrow KeyPath MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/keypath_mvp");
    let report = format_check_path(&dir).expect("keypath_mvp check");
    assert!(
        report.checked >= 5,
        "keypath_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W727: keypath_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_compound_assign_mvp_ok() {
    // W728 / ADR-1072: narrow CompoundAssign MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/compound_assign_mvp");
    let report = format_check_path(&dir).expect("compound_assign_mvp check");
    assert!(
        report.checked >= 5,
        "compound_assign_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W728: compound_assign_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_index_assign_mvp_ok() {
    // W729 / ADR-1073: narrow IndexAssign MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/index_assign_mvp");
    let report = format_check_path(&dir).expect("index_assign_mvp check");
    assert!(
        report.checked >= 5,
        "index_assign_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W729: index_assign_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_tuple_mvp_ok() {
    // W730 / ADR-1074: narrow Tuple MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/tuple_mvp");
    let report = format_check_path(&dir).expect("tuple_mvp check");
    assert!(
        report.checked >= 5,
        "tuple_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W730: tuple_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_super_mvp_ok() {
    // W731 / ADR-1075: narrow Super MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/super_mvp");
    let report = format_check_path(&dir).expect("super_mvp check");
    assert!(
        report.checked >= 5,
        "super_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W731: super_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_seq_mvp_ok() {
    // W732 / ADR-1076: narrow Seq MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/seq_mvp");
    let report = format_check_path(&dir).expect("seq_mvp check");
    assert!(
        report.checked >= 5,
        "seq_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W732: seq_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_where_mvp_ok() {
    // W733 / ADR-1077: narrow Switch-where MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_where_mvp");
    let report = format_check_path(&dir).expect("switch_where_mvp check");
    assert!(
        report.checked >= 5,
        "switch_where_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W733: switch_where_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_range_mvp_ok() {
    // W734 / ADR-1078: narrow Switch-range MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_range_mvp");
    let report = format_check_path(&dir).expect("switch_range_mvp check");
    assert!(
        report.checked >= 5,
        "switch_range_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W734: switch_range_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_variant_mvp_ok() {
    // W735 / ADR-1079: narrow Switch-variant MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_variant_mvp");
    let report = format_check_path(&dir).expect("switch_variant_mvp check");
    assert!(
        report.checked >= 5,
        "switch_variant_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W735: switch_variant_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_tuple_mvp_ok() {
    // W736 / ADR-1080: narrow Switch-tuple MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_tuple_mvp");
    let report = format_check_path(&dir).expect("switch_tuple_mvp check");
    assert!(
        report.checked >= 5,
        "switch_tuple_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W736: switch_tuple_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_is_mvp_ok() {
    // W737 / ADR-1081: narrow Switch-is MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_is_mvp");
    let report = format_check_path(&dir).expect("switch_is_mvp check");
    assert!(
        report.checked >= 5,
        "switch_is_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W737: switch_is_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_ident_mvp_ok() {
    // W738 / ADR-1082: narrow Switch-Ident MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_ident_mvp");
    let report = format_check_path(&dir).expect("switch_ident_mvp check");
    assert!(
        report.checked >= 5,
        "switch_ident_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W738: switch_ident_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_binary_mvp_ok() {
    // W739 / ADR-1083: narrow Switch-Binary MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_binary_mvp");
    let report = format_check_path(&dir).expect("switch_binary_mvp check");
    assert!(
        report.checked >= 5,
        "switch_binary_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W739: switch_binary_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_call_mvp_ok() {
    // W740 / ADR-1084: narrow Switch-Call MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_call_mvp");
    let report = format_check_path(&dir).expect("switch_call_mvp check");
    assert!(
        report.checked >= 5,
        "switch_call_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W740: switch_call_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_member_mvp_ok() {
    // W741 / ADR-1085: narrow Switch-Member MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_member_mvp");
    let report = format_check_path(&dir).expect("switch_member_mvp check");
    assert!(
        report.checked >= 5,
        "switch_member_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W741: switch_member_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_index_mvp_ok() {
    // W742 / ADR-1086: narrow Switch-Index MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_index_mvp");
    let report = format_check_path(&dir).expect("switch_index_mvp check");
    assert!(
        report.checked >= 5,
        "switch_index_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W742: switch_index_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_optional_chain_mvp_ok() {
    // W743 / ADR-1087: narrow Switch-OptionalChain MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_optional_chain_mvp");
    let report = format_check_path(&dir).expect("switch_optional_chain_mvp check");
    assert!(
        report.checked >= 5,
        "switch_optional_chain_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W743: switch_optional_chain_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_force_unwrap_mvp_ok() {
    // W744 / ADR-1088: narrow Switch-ForceUnwrap MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_force_unwrap_mvp");
    let report = format_check_path(&dir).expect("switch_force_unwrap_mvp check");
    assert!(
        report.checked >= 5,
        "switch_force_unwrap_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W744: switch_force_unwrap_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_keypath_mvp_ok() {
    // W745 / ADR-1089: narrow Switch-KeyPath MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_keypath_mvp");
    let report = format_check_path(&dir).expect("switch_keypath_mvp check");
    assert!(
        report.checked >= 5,
        "switch_keypath_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W745: switch_keypath_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_array_mvp_ok() {
    // W746 / ADR-1090: narrow Switch-Array MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_array_mvp");
    let report = format_check_path(&dir).expect("switch_array_mvp check");
    assert!(
        report.checked >= 5,
        "switch_array_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W746: switch_array_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_unary_mvp_ok() {
    // W747 / ADR-1091: narrow Switch-Unary MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_unary_mvp");
    let report = format_check_path(&dir).expect("switch_unary_mvp check");
    assert!(
        report.checked >= 5,
        "switch_unary_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W747: switch_unary_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_await_mvp_ok() {
    // W748 / ADR-1092: narrow Switch-Await MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_await_mvp");
    let report = format_check_path(&dir).expect("switch_await_mvp check");
    assert!(
        report.checked >= 5,
        "switch_await_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W748: switch_await_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_cast_mvp_ok() {
    // W749 / ADR-1093: narrow Switch-Cast MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_cast_mvp");
    let report = format_check_path(&dir).expect("switch_cast_mvp check");
    assert!(
        report.checked >= 5,
        "switch_cast_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W749: switch_cast_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_optional_cast_mvp_ok() {
    // W750 / ADR-1094: narrow Switch-OptionalCast MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_optional_cast_mvp");
    let report = format_check_path(&dir).expect("switch_optional_cast_mvp check");
    assert!(
        report.checked >= 5,
        "switch_optional_cast_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W750: switch_optional_cast_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_type_self_mvp_ok() {
    // W751 / ADR-1095: narrow Switch-TypeSelf MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_type_self_mvp");
    let report = format_check_path(&dir).expect("switch_type_self_mvp check");
    assert!(
        report.checked >= 5,
        "switch_type_self_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W751: switch_type_self_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_super_mvp_ok() {
    // W752 / ADR-1096: narrow Switch-Super MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_super_mvp");
    let report = format_check_path(&dir).expect("switch_super_mvp check");
    assert!(
        report.checked >= 5,
        "switch_super_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W752: switch_super_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_borrow_mvp_ok() {
    // W753 / ADR-1097: narrow Switch-Borrow MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_borrow_mvp");
    let report = format_check_path(&dir).expect("switch_borrow_mvp check");
    assert!(
        report.checked >= 5,
        "switch_borrow_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W753: switch_borrow_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_macro_invoke_mvp_ok() {
    // W754 / ADR-1098: narrow Switch-MacroInvoke MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_macro_invoke_mvp");
    let report = format_check_path(&dir).expect("switch_macro_invoke_mvp check");
    assert!(
        report.checked >= 5,
        "switch_macro_invoke_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W754: switch_macro_invoke_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_closure_mvp_ok() {
    // W755 / ADR-1099: narrow Switch-Closure MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_closure_mvp");
    let report = format_check_path(&dir).expect("switch_closure_mvp check");
    assert!(
        report.checked >= 5,
        "switch_closure_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W755: switch_closure_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_task_block_mvp_ok() {
    // W756 / ADR-1100: narrow Switch-TaskBlock MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_task_block_mvp");
    let report = format_check_path(&dir).expect("switch_task_block_mvp check");
    assert!(
        report.checked >= 5,
        "switch_task_block_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W756: switch_task_block_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_assign_mvp_ok() {
    // W757 / ADR-1101: narrow Switch-Assign MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_assign_mvp");
    let report = format_check_path(&dir).expect("switch_assign_mvp check");
    assert!(
        report.checked >= 5,
        "switch_assign_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W757: switch_assign_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_compound_assign_mvp_ok() {
    // W758 / ADR-1102: narrow Switch-CompoundAssign MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_compound_assign_mvp");
    let report = format_check_path(&dir).expect("switch_compound_assign_mvp check");
    assert!(
        report.checked >= 5,
        "switch_compound_assign_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W758: switch_compound_assign_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_member_assign_mvp_ok() {
    // W759 / ADR-1103: narrow Switch-MemberAssign MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_member_assign_mvp");
    let report = format_check_path(&dir).expect("switch_member_assign_mvp check");
    assert!(
        report.checked >= 5,
        "switch_member_assign_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W759: switch_member_assign_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_index_assign_mvp_ok() {
    // W760 / ADR-1104: narrow Switch-IndexAssign MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_index_assign_mvp");
    let report = format_check_path(&dir).expect("switch_index_assign_mvp check");
    assert!(
        report.checked >= 5,
        "switch_index_assign_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W760: switch_index_assign_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_switch_is_expr_mvp_ok() {
    // W761 / ADR-1105: narrow Switch non-desugar-Is MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/switch_is_expr_mvp");
    let report = format_check_path(&dir).expect("switch_is_expr_mvp check");
    assert!(
        report.checked >= 5,
        "switch_is_expr_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W761: switch_is_expr_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_if_case_literal_mvp_ok() {
    // W762 / ADR-1106: narrow IfCase Literal MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/if_case_literal_mvp");
    let report = format_check_path(&dir).expect("if_case_literal_mvp check");
    assert!(
        report.checked >= 5,
        "if_case_literal_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W762: if_case_literal_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_if_case_range_mvp_ok() {
    // W763 / ADR-1107: narrow IfCase Range MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/if_case_range_mvp");
    let report = format_check_path(&dir).expect("if_case_range_mvp check");
    assert!(
        report.checked >= 5,
        "if_case_range_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W763: if_case_range_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_if_case_is_mvp_ok() {
    // W764 / ADR-1108: narrow IfCase desugar-Is MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/if_case_is_mvp");
    let report = format_check_path(&dir).expect("if_case_is_mvp check");
    assert!(
        report.checked >= 5,
        "if_case_is_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W764: if_case_is_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_protocol_assoc_mvp_ok() {
    // W765 / ADR-1109: narrow protocol associatedtype MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/protocol_assoc_mvp");
    let report = format_check_path(&dir).expect("protocol_assoc_mvp check");
    assert!(
        report.checked >= 5,
        "protocol_assoc_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W765: protocol_assoc_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_extension_where_mvp_ok() {
    // W766 / ADR-1110: narrow extension where MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/extension_where_mvp");
    let report = format_check_path(&dir).expect("extension_where_mvp check");
    assert!(
        report.checked >= 5,
        "extension_where_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W766: extension_where_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_enum_type_params_mvp_ok() {
    // W767 / ADR-1111: narrow enum type_params MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/enum_type_params_mvp");
    let report = format_check_path(&dir).expect("enum_type_params_mvp check");
    assert!(
        report.checked >= 5,
        "enum_type_params_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W767: enum_type_params_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_data_class_type_params_mvp_ok() {
    // W768 / ADR-1112: narrow data class type_params MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/data_class_type_params_mvp");
    let report = format_check_path(&dir).expect("data_class_type_params_mvp check");
    assert!(
        report.checked >= 5,
        "data_class_type_params_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W768: data_class_type_params_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_data_class_where_mvp_ok() {
    // W769 / ADR-1113: narrow data class where MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/data_class_where_mvp");
    let report = format_check_path(&dir).expect("data_class_where_mvp check");
    assert!(
        report.checked >= 5,
        "data_class_where_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W769: data_class_where_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_final_class_type_params_mvp_ok() {
    // W770 / ADR-1114: narrow final class type_params MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/final_class_type_params_mvp");
    let report = format_check_path(&dir).expect("final_class_type_params_mvp check");
    assert!(
        report.checked >= 5,
        "final_class_type_params_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W770: final_class_type_params_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_func_type_params_mvp_ok() {
    // W771 / ADR-1115: narrow function type_params MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/func_type_params_mvp");
    let report = format_check_path(&dir).expect("func_type_params_mvp check");
    assert!(
        report.checked >= 5,
        "func_type_params_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W771: func_type_params_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_func_throws_mvp_ok() {
    // W772 / ADR-1116: narrow function throws MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/func_throws_mvp");
    let report = format_check_path(&dir).expect("func_throws_mvp check");
    assert!(
        report.checked >= 5,
        "func_throws_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W772: func_throws_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_func_effect_mvp_ok() {
    // W773 / ADR-1117: narrow function effect MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/func_effect_mvp");
    let report = format_check_path(&dir).expect("func_effect_mvp check");
    assert!(
        report.checked >= 5,
        "func_effect_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W773: func_effect_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_func_where_mvp_ok() {
    // W774 / ADR-1118: narrow function where MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/func_where_mvp");
    let report = format_check_path(&dir).expect("func_where_mvp check");
    assert!(
        report.checked >= 5,
        "func_where_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W774: func_where_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_parameter_attrs_mvp_ok() {
    // W775 / ADR-1119: narrow parameter attributes MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/parameter_attrs_mvp");
    let report = format_check_path(&dir).expect("parameter_attrs_mvp check");
    assert!(
        report.checked >= 5,
        "parameter_attrs_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W775: parameter_attrs_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_isolated_sending_mvp_ok() {
    // W776 / ADR-1120: narrow isolated/sending MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/isolated_sending_mvp");
    let report = format_check_path(&dir).expect("isolated_sending_mvp check");
    assert!(
        report.checked >= 5,
        "isolated_sending_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W776: isolated_sending_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_binop_mod_mvp_ok() {
    // W777 / ADR-1121: narrow binop Mod MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/binop_mod_mvp");
    let report = format_check_path(&dir).expect("binop_mod_mvp check");
    assert!(
        report.checked >= 5,
        "binop_mod_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W777: binop_mod_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_binop_lteq_mvp_ok() {
    // W778 / ADR-1122: narrow binop LtEq MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/binop_lteq_mvp");
    let report = format_check_path(&dir).expect("binop_lteq_mvp check");
    assert!(
        report.checked >= 5,
        "binop_lteq_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W778: binop_lteq_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_binop_gteq_mvp_ok() {
    // W779 / ADR-1123: narrow binop GtEq MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/binop_gteq_mvp");
    let report = format_check_path(&dir).expect("binop_gteq_mvp check");
    assert!(
        report.checked >= 5,
        "binop_gteq_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W779: binop_gteq_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_binop_nilcoalesce_mvp_ok() {
    // W780 / ADR-1124: narrow binop NilCoalesce MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/binop_nilcoalesce_mvp");
    let report = format_check_path(&dir).expect("binop_nilcoalesce_mvp check");
    assert!(
        report.checked >= 5,
        "binop_nilcoalesce_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W780: binop_nilcoalesce_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_unary_try_mvp_ok() {
    // W781 / ADR-1125: narrow unary Try MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/unary_try_mvp");
    let report = format_check_path(&dir).expect("unary_try_mvp check");
    assert!(
        report.checked >= 5,
        "unary_try_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W781: unary_try_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_type_optional_mvp_ok() {
    // W782 / ADR-1126: narrow type Optional MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/type_optional_mvp");
    let report = format_check_path(&dir).expect("type_optional_mvp check");
    assert!(
        report.checked >= 5,
        "type_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W782: type_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_opaque_some_mvp_ok() {
    // W783 / ADR-1127: narrow OpaqueSome MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/opaque_some_mvp");
    let report = format_check_path(&dir).expect("opaque_some_mvp check");
    assert!(
        report.checked >= 5,
        "opaque_some_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W783: opaque_some_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_opaque_any_mvp_ok() {
    // W784 / ADR-1128: narrow OpaqueAny MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/opaque_any_mvp");
    let report = format_check_path(&dir).expect("opaque_any_mvp check");
    assert!(
        report.checked >= 5,
        "opaque_any_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W784: opaque_any_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_func_type_mvp_ok() {
    // W785 / ADR-1129: narrow Function type MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/func_type_mvp");
    let report = format_check_path(&dir).expect("func_type_mvp check");
    assert!(
        report.checked >= 5,
        "func_type_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W785: func_type_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_type_reference_mvp_ok() {
    // W786 / ADR-1130: narrow Reference MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/type_reference_mvp");
    let report = format_check_path(&dir).expect("type_reference_mvp check");
    assert!(
        report.checked >= 5,
        "type_reference_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W786: type_reference_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_literal_bigint_mvp_ok() {
    // W787 / ADR-1131: narrow BigInt MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/literal_bigint_mvp");
    let report = format_check_path(&dir).expect("literal_bigint_mvp check");
    assert!(
        report.checked >= 5,
        "literal_bigint_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W787: literal_bigint_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_literal_interpolated_mvp_ok() {
    // W788 / ADR-1132: narrow Interpolated MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/literal_interpolated_mvp");
    let report = format_check_path(&dir).expect("literal_interpolated_mvp check");
    assert!(
        report.checked >= 5,
        "literal_interpolated_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W788: literal_interpolated_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_literal_regex_mvp_ok() {
    // W789 / ADR-1133: narrow Regex MVP directory must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/literal_regex_mvp");
    let report = format_check_path(&dir).expect("literal_regex_mvp check");
    assert!(
        report.checked >= 5,
        "literal_regex_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W789: literal_regex_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_print_mvp_ok() {
    // W790 / ADR-1134: narrow CST print MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_print_mvp");
    let report = format_check_path(&dir).expect("cst_print_mvp check");
    assert!(
        report.checked >= 5,
        "cst_print_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W790: cst_print_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_let_mvp_ok() {
    // W791 / ADR-1135: narrow CST let MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_let_mvp");
    let report = format_check_path(&dir).expect("cst_let_mvp check");
    assert!(
        report.checked >= 5,
        "cst_let_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W791: cst_let_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_return_mvp_ok() {
    // W792 / ADR-1136: narrow CST return MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_return_mvp");
    let report = format_check_path(&dir).expect("cst_return_mvp check");
    assert!(
        report.checked >= 5,
        "cst_return_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W792: cst_return_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_double_mvp_ok() {
    // W793 / ADR-1137: narrow CST Double MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_double_mvp");
    let report = format_check_path(&dir).expect("cst_double_mvp check");
    assert!(
        report.checked >= 5,
        "cst_double_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W793: cst_double_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_bool_mvp_ok() {
    // W794 / ADR-1138: narrow CST Bool MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_bool_mvp");
    let report = format_check_path(&dir).expect("cst_bool_mvp check");
    assert!(
        report.checked >= 5,
        "cst_bool_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W794: cst_bool_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_nil_mvp_ok() {
    // W795 / ADR-1139: narrow CST Nil MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_nil_mvp");
    let report = format_check_path(&dir).expect("cst_nil_mvp check");
    assert!(
        report.checked >= 5,
        "cst_nil_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W795: cst_nil_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_var_mvp_ok() {
    // W796 / ADR-1140: narrow CST var MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_var_mvp");
    let report = format_check_path(&dir).expect("cst_var_mvp check");
    assert!(
        report.checked >= 5,
        "cst_var_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W796: cst_var_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_bare_return_mvp_ok() {
    // W797 / ADR-1141: narrow CST bare-return MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_bare_return_mvp");
    let report = format_check_path(&dir).expect("cst_bare_return_mvp check");
    assert!(
        report.checked >= 5,
        "cst_bare_return_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W797: cst_bare_return_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_multi_func_mvp_ok() {
    // W798 / ADR-1142: narrow CST multi-func MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_multi_func_mvp");
    let report = format_check_path(&dir).expect("cst_multi_func_mvp check");
    assert!(
        report.checked >= 5,
        "cst_multi_func_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W798: cst_multi_func_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_bind_mvp_ok() {
    // W799 / ADR-1143: narrow CST typed-bind MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_bind_mvp");
    let report = format_check_path(&dir).expect("cst_typed_bind_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_bind_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W799: cst_typed_bind_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_optional_mvp_ok() {
    // W800 / ADR-1144: narrow CST typed-optional MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_optional_mvp");
    let report = format_check_path(&dir).expect("cst_typed_optional_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W800: cst_typed_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_reference_mvp_ok() {
    // W801 / ADR-1145: narrow CST typed-reference MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_reference_mvp");
    let report = format_check_path(&dir).expect("cst_typed_reference_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_reference_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W801: cst_typed_reference_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_mut_reference_mvp_ok() {
    // W802 / ADR-1146: narrow CST typed-mut-reference MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_mut_reference_mvp");
    let report = format_check_path(&dir).expect("cst_typed_mut_reference_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_mut_reference_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W802: cst_typed_mut_reference_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_lifetime_reference_mvp_ok() {
    // W803 / ADR-1147: narrow CST typed-lifetime-reference MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_lifetime_reference_mvp");
    let report = format_check_path(&dir).expect("cst_typed_lifetime_reference_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_lifetime_reference_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W803: cst_typed_lifetime_reference_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_function_type_mvp_ok() {
    // W804 / ADR-1148: narrow CST typed-function-type MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_function_type_mvp");
    let report = format_check_path(&dir).expect("cst_typed_function_type_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_function_type_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W804: cst_typed_function_type_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_function_param_reference_mvp_ok() {
    // W805 / ADR-1149: narrow CST typed-function-param-reference MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_function_param_reference_mvp");
    let report = format_check_path(&dir).expect("cst_typed_function_param_reference_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_function_param_reference_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W805: cst_typed_function_param_reference_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_function_param_optional_mvp_ok() {
    // W806 / ADR-1150: narrow CST typed-function-param-optional MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_function_param_optional_mvp");
    let report = format_check_path(&dir).expect("cst_typed_function_param_optional_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_function_param_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W806: cst_typed_function_param_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_function_return_reference_mvp_ok() {
    // W807 / ADR-1151: narrow CST typed-function-return-reference MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_function_return_reference_mvp");
    let report = format_check_path(&dir).expect("cst_typed_function_return_reference_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_function_return_reference_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W807: cst_typed_function_return_reference_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_function_return_optional_mvp_ok() {
    // W808 / ADR-1152: narrow CST typed-function-return-optional MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_function_return_optional_mvp");
    let report = format_check_path(&dir).expect("cst_typed_function_return_optional_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_function_return_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W808: cst_typed_function_return_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_function_param_nested_mvp_ok() {
    // W809 / ADR-1153: narrow CST typed-function-param-nested MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_function_param_nested_mvp");
    let report = format_check_path(&dir).expect("cst_typed_function_param_nested_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_function_param_nested_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W809: cst_typed_function_param_nested_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_function_return_nested_mvp_ok() {
    // W810 / ADR-1154: narrow CST typed-function-return-nested MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_function_return_nested_mvp");
    let report = format_check_path(&dir).expect("cst_typed_function_return_nested_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_function_return_nested_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W810: cst_typed_function_return_nested_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_amp_optional_mvp_ok() {
    // W811 / ADR-1155: narrow CST typed-amp-optional MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_amp_optional_mvp");
    let report = format_check_path(&dir).expect("cst_typed_amp_optional_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_amp_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W811: cst_typed_amp_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_generic_mvp_ok() {
    // W812 / ADR-1156: narrow CST typed-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_generic_mvp");
    let report = format_check_path(&dir).expect("cst_typed_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W812: cst_typed_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_nested_generic_mvp_ok() {
    // W813 / ADR-1157: narrow CST typed-nested-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_nested_generic_mvp");
    let report = format_check_path(&dir).expect("cst_typed_nested_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_nested_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W813: cst_typed_nested_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_multi_arg_generic_mvp_ok() {
    // W814 / ADR-1158: narrow CST typed-multi-arg-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_multi_arg_generic_mvp");
    let report = format_check_path(&dir).expect("cst_typed_multi_arg_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_multi_arg_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W814: cst_typed_multi_arg_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_toplevel_simple_mvp_ok() {
    // W815 / ADR-1159: narrow CST toplevel-simple MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_toplevel_simple_mvp");
    let report = format_check_path(&dir).expect("cst_toplevel_simple_mvp check");
    assert!(
        report.checked >= 5,
        "cst_toplevel_simple_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W815: cst_toplevel_simple_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_param_func_mvp_ok() {
    // W816 / ADR-1160: narrow CST param-func MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_param_func_mvp");
    let report = format_check_path(&dir).expect("cst_param_func_mvp check");
    assert!(
        report.checked >= 5,
        "cst_param_func_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W816: cst_param_func_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_mvp_ok() {
    // W817 / ADR-1161: narrow CST typed-param MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W817: cst_typed_param_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_optional_mvp_ok() {
    // W818 / ADR-1162: narrow CST typed-param-optional MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_optional_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_optional_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W818: cst_typed_param_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_amp_mvp_ok() {
    // W819 / ADR-1163: narrow CST typed-param-amp MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_amp_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_amp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_amp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W819: cst_typed_param_amp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_mut_amp_mvp_ok() {
    // W820 / ADR-1164: narrow CST typed-param-mut-amp MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_mut_amp_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_mut_amp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_mut_amp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W820: cst_typed_param_mut_amp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_lifetime_amp_mvp_ok() {
    // W821 / ADR-1165: narrow CST typed-param-lifetime-amp MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_lifetime_amp_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_lifetime_amp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_lifetime_amp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W821: cst_typed_param_lifetime_amp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_amp_optional_mvp_ok() {
    // W822 / ADR-1166: narrow CST typed-param-amp-optional MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_amp_optional_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_amp_optional_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_amp_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W822: cst_typed_param_amp_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_generic_mvp_ok() {
    // W823 / ADR-1167: narrow CST typed-param-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_generic_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W823: cst_typed_param_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_nested_generic_mvp_ok() {
    // W824 / ADR-1168: narrow CST typed-param-nested-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_nested_generic_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_nested_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_nested_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W824: cst_typed_param_nested_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_typed_param_multi_arg_generic_mvp_ok() {
    // W825 / ADR-1169: narrow CST typed-param-multi-arg-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_typed_param_multi_arg_generic_mvp");
    let report = format_check_path(&dir).expect("cst_typed_param_multi_arg_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_typed_param_multi_arg_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W825: cst_typed_param_multi_arg_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_mvp_ok() {
    // W826 / ADR-1170: narrow CST func-return MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W826: cst_func_return_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_amp_mvp_ok() {
    // W827 / ADR-1171: narrow CST func-return-amp MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_amp_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_amp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_amp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W827: cst_func_return_amp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_mut_amp_mvp_ok() {
    // W828 / ADR-1172: narrow CST func-return-mut-amp MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_mut_amp_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_mut_amp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_mut_amp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W828: cst_func_return_mut_amp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_lifetime_amp_mvp_ok() {
    // W829 / ADR-1173: narrow CST func-return-lifetime-amp MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_lifetime_amp_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_lifetime_amp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_lifetime_amp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W829: cst_func_return_lifetime_amp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_amp_optional_mvp_ok() {
    // W830 / ADR-1174: narrow CST func-return-amp-optional MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_amp_optional_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_amp_optional_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_amp_optional_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W830: cst_func_return_amp_optional_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_generic_mvp_ok() {
    // W831 / ADR-1175: narrow CST func-return-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_generic_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W831: cst_func_return_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_nested_generic_mvp_ok() {
    // W832 / ADR-1176: narrow CST func-return-nested-generic MVP directory must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_nested_generic_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_nested_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_nested_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W832: cst_func_return_nested_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_multi_arg_generic_mvp_ok() {
    // W833 / ADR-1177 face1: cst_func_return_multi_arg_generic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_multi_arg_generic_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_multi_arg_generic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_multi_arg_generic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W833: cst_func_return_multi_arg_generic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_return_function_mvp_ok() {
    // W833 / ADR-1177 face2: cst_func_return_function MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_return_function_mvp");
    let report = format_check_path(&dir).expect("cst_func_return_function_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_return_function_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W833: cst_func_return_function_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_amp_as_generic_arg_mvp_ok() {
    // W833 / ADR-1177 face3: cst_amp_as_generic_arg MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_amp_as_generic_arg_mvp");
    let report = format_check_path(&dir).expect("cst_amp_as_generic_arg_mvp check");
    assert!(
        report.checked >= 5,
        "cst_amp_as_generic_arg_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W833: cst_amp_as_generic_arg_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_function_as_generic_arg_mvp_ok() {
    // W834 / ADR-1178 face1: cst_function_as_generic_arg MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_function_as_generic_arg_mvp");
    let report = format_check_path(&dir).expect("cst_function_as_generic_arg_mvp check");
    assert!(
        report.checked >= 5,
        "cst_function_as_generic_arg_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W834: cst_function_as_generic_arg_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_print_call_mvp_ok() {
    // W834 / ADR-1178 face2: cst_print_call MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_print_call_mvp");
    let report = format_check_path(&dir).expect("cst_print_call_mvp check");
    assert!(
        report.checked >= 5,
        "cst_print_call_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W834: cst_print_call_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_call_rhs_mvp_ok() {
    // W834 / ADR-1178 face3: cst_call_rhs MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_call_rhs_mvp");
    let report = format_check_path(&dir).expect("cst_call_rhs_mvp check");
    assert!(
        report.checked >= 5,
        "cst_call_rhs_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W834: cst_call_rhs_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_ident_cond_mvp_ok() {
    // W835 / ADR-1179 face1: cst_while_ident_cond MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_ident_cond_mvp");
    let report = format_check_path(&dir).expect("cst_while_ident_cond_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_ident_cond_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W835: cst_while_ident_cond_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_call_cond_mvp_ok() {
    // W835 / ADR-1179 face2: cst_while_call_cond MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_call_cond_mvp");
    let report = format_check_path(&dir).expect("cst_while_call_cond_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_call_cond_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W835: cst_while_call_cond_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_mixed_mvp_ok() {
    // W835 / ADR-1179 face3: cst_while_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W835: cst_while_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_nested_mvp_ok() {
    // W836 / ADR-1180 face1: cst_while_nested MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_nested_mvp");
    let report = format_check_path(&dir).expect("cst_while_nested_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_nested_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W836: cst_while_nested_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_compare_cond_mvp_ok() {
    // W836 / ADR-1180 face2: cst_while_compare_cond MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_compare_cond_mvp");
    let report = format_check_path(&dir).expect("cst_while_compare_cond_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_compare_cond_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W836: cst_while_compare_cond_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_deeper_mixed_mvp_ok() {
    // W836 / ADR-1180 face3: cst_while_deeper_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_deeper_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_deeper_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_deeper_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W836: cst_while_deeper_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_effect_pure_mvp_ok() {
    // W837 / ADR-1181 face1: cst_effect_pure MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_effect_pure_mvp");
    let report = format_check_path(&dir).expect("cst_effect_pure_mvp check");
    assert!(
        report.checked >= 5,
        "cst_effect_pure_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W837: cst_effect_pure_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_effect_io_mvp_ok() {
    // W837 / ADR-1181 face2: cst_effect_io MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_effect_io_mvp");
    let report = format_check_path(&dir).expect("cst_effect_io_mvp check");
    assert!(
        report.checked >= 5,
        "cst_effect_io_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W837: cst_effect_io_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_effect_mixed_mvp_ok() {
    // W837 / ADR-1181 face3: cst_effect_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_effect_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_effect_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_effect_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W837: cst_effect_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_nested_call_print_mvp_ok() {
    // W838 / ADR-1182 face1: cst_nested_call_print MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_nested_call_print_mvp");
    let report = format_check_path(&dir).expect("cst_nested_call_print_mvp check");
    assert!(
        report.checked >= 5,
        "cst_nested_call_print_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W838: cst_nested_call_print_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_nested_call_rhs_mvp_ok() {
    // W838 / ADR-1182 face2: cst_nested_call_rhs MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_nested_call_rhs_mvp");
    let report = format_check_path(&dir).expect("cst_nested_call_rhs_mvp check");
    assert!(
        report.checked >= 5,
        "cst_nested_call_rhs_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W838: cst_nested_call_rhs_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_nested_call_mixed_mvp_ok() {
    // W838 / ADR-1182 face3: cst_nested_call_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_nested_call_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_nested_call_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_nested_call_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W838: cst_nested_call_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_throws_bare_mvp_ok() {
    // W839 / ADR-1183 face1: cst_throws_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_throws_bare_mvp");
    let report = format_check_path(&dir).expect("cst_throws_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_throws_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W839: cst_throws_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_throws_clause_mvp_ok() {
    // W839 / ADR-1183 face2: cst_throws_clause MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_throws_clause_mvp");
    let report = format_check_path(&dir).expect("cst_throws_clause_mvp check");
    assert!(
        report.checked >= 5,
        "cst_throws_clause_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W839: cst_throws_clause_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_throws_mixed_mvp_ok() {
    // W839 / ADR-1183 face3: cst_throws_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_throws_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_throws_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_throws_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W839: cst_throws_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_type_params_single_mvp_ok() {
    // W840 / ADR-1184 face1: cst_type_params_single MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_type_params_single_mvp");
    let report = format_check_path(&dir).expect("cst_type_params_single_mvp check");
    assert!(
        report.checked >= 5,
        "cst_type_params_single_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W840: cst_type_params_single_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_type_params_multi_mvp_ok() {
    // W840 / ADR-1184 face2: cst_type_params_multi MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_type_params_multi_mvp");
    let report = format_check_path(&dir).expect("cst_type_params_multi_mvp check");
    assert!(
        report.checked >= 5,
        "cst_type_params_multi_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W840: cst_type_params_multi_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_type_params_mixed_mvp_ok() {
    // W840 / ADR-1184 face3: cst_type_params_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_type_params_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_type_params_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_type_params_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W840: cst_type_params_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_where_bare_mvp_ok() {
    // W841 / ADR-1185 face1: cst_where_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_where_bare_mvp");
    let report = format_check_path(&dir).expect("cst_where_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_where_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W841: cst_where_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_where_clause_mvp_ok() {
    // W841 / ADR-1185 face2: cst_where_clause MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_where_clause_mvp");
    let report = format_check_path(&dir).expect("cst_where_clause_mvp check");
    assert!(
        report.checked >= 5,
        "cst_where_clause_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W841: cst_where_clause_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_where_mixed_mvp_ok() {
    // W841 / ADR-1185 face3: cst_where_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_where_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_where_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_where_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W841: cst_where_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_depth2_call_print_mvp_ok() {
    // W842 / ADR-1186 face1: cst_depth2_call_print MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_depth2_call_print_mvp");
    let report = format_check_path(&dir).expect("cst_depth2_call_print_mvp check");
    assert!(
        report.checked >= 5,
        "cst_depth2_call_print_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W842: cst_depth2_call_print_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_depth2_call_rhs_mvp_ok() {
    // W842 / ADR-1186 face2: cst_depth2_call_rhs MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_depth2_call_rhs_mvp");
    let report = format_check_path(&dir).expect("cst_depth2_call_rhs_mvp check");
    assert!(
        report.checked >= 5,
        "cst_depth2_call_rhs_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W842: cst_depth2_call_rhs_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_depth2_call_mixed_mvp_ok() {
    // W842 / ADR-1186 face3: cst_depth2_call_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_depth2_call_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_depth2_call_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_depth2_call_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W842: cst_depth2_call_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_braceless_bare_mvp_ok() {
    // W843 / ADR-1187 face1: cst_while_braceless_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_braceless_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_braceless_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_braceless_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W843: cst_while_braceless_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_braceless_cond_mvp_ok() {
    // W843 / ADR-1187 face2: cst_while_braceless_cond MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_braceless_cond_mvp");
    let report = format_check_path(&dir).expect("cst_while_braceless_cond_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_braceless_cond_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W843: cst_while_braceless_cond_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_braceless_mixed_mvp_ok() {
    // W843 / ADR-1187 face3: cst_while_braceless_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_braceless_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_braceless_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_braceless_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W843: cst_while_braceless_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_and_mvp_ok() {
    // W844 / ADR-1188 face1: cst_while_logic_and MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_and_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_and_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_and_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W844: cst_while_logic_and_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_or_mvp_ok() {
    // W844 / ADR-1188 face2: cst_while_logic_or MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_or_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_or_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_or_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W844: cst_while_logic_or_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_mixed_mvp_ok() {
    // W844 / ADR-1188 face3: cst_while_logic_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W844: cst_while_logic_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_chain_mvp_ok() {
    // W845 / ADR-1189 face1: cst_while_logic_chain MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_chain_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_chain_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_chain_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W845: cst_while_logic_chain_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_cmp_mvp_ok() {
    // W845 / ADR-1189 face2: cst_while_logic_cmp MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_cmp_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_cmp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_cmp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W845: cst_while_logic_cmp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_deep_mixed_mvp_ok() {
    // W845 / ADR-1189 face3: cst_while_logic_deep_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_deep_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_deep_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_deep_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W845: cst_while_logic_deep_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_mixedops_mvp_ok() {
    // W846 / ADR-1190 face1: cst_while_logic_mixedops MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_mixedops_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_mixedops_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_mixedops_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W846: cst_while_logic_mixedops_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_call_mvp_ok() {
    // W846 / ADR-1190 face2: cst_while_logic_call MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_call_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_call_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_call_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W846: cst_while_logic_call_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_logic_final_mixed_mvp_ok() {
    // W846 / ADR-1190 face3: cst_while_logic_final_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_logic_final_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_logic_final_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_logic_final_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W846: cst_while_logic_final_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_unary_bare_mvp_ok() {
    // W847 / ADR-1191 face1: cst_while_unary_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_unary_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_unary_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_unary_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W847: cst_while_unary_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_unary_logic_mvp_ok() {
    // W847 / ADR-1191 face2: cst_while_unary_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_unary_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_unary_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_unary_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W847: cst_while_unary_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_unary_mixed_mvp_ok() {
    // W847 / ADR-1191 face3: cst_while_unary_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_unary_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_unary_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_unary_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W847: cst_while_unary_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren_bare_mvp_ok() {
    // W848 / ADR-1192 face1: cst_while_paren_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W848: cst_while_paren_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren_unary_mvp_ok() {
    // W848 / ADR-1192 face2: cst_while_paren_unary MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren_unary_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren_unary_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren_unary_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W848: cst_while_paren_unary_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren_mixed_mvp_ok() {
    // W848 / ADR-1192 face3: cst_while_paren_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W848: cst_while_paren_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg_bare_mvp_ok() {
    // W849 / ADR-1193 face1: cst_while_neg_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W849: cst_while_neg_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg_logic_mvp_ok() {
    // W849 / ADR-1193 face2: cst_while_neg_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W849: cst_while_neg_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg_mixed_mvp_ok() {
    // W849 / ADR-1193 face3: cst_while_neg_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W849: cst_while_neg_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang2_bare_mvp_ok() {
    // W850 / ADR-1194 face1: cst_while_bang2_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang2_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang2_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang2_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W850: cst_while_bang2_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang2_logic_mvp_ok() {
    // W850 / ADR-1194 face2: cst_while_bang2_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang2_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang2_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang2_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W850: cst_while_bang2_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang2_mixed_mvp_ok() {
    // W850 / ADR-1194 face3: cst_while_bang2_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang2_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang2_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang2_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W850: cst_while_bang2_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren2_bare_mvp_ok() {
    // W851 / ADR-1195 face1: cst_while_paren2_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren2_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren2_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren2_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W851: cst_while_paren2_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren2_logic_mvp_ok() {
    // W851 / ADR-1195 face2: cst_while_paren2_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren2_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren2_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren2_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W851: cst_while_paren2_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren2_mixed_mvp_ok() {
    // W851 / ADR-1195 face3: cst_while_paren2_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren2_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren2_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren2_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W851: cst_while_paren2_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang3_bare_mvp_ok() {
    // W852 / ADR-1196 face1: cst_while_bang3_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang3_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang3_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang3_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W852: cst_while_bang3_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang3_logic_mvp_ok() {
    // W852 / ADR-1196 face2: cst_while_bang3_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang3_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang3_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang3_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W852: cst_while_bang3_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang3_mixed_mvp_ok() {
    // W852 / ADR-1196 face3: cst_while_bang3_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang3_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang3_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang3_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W852: cst_while_bang3_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg2_bare_mvp_ok() {
    // W853 / ADR-1197 face1: cst_while_neg2_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg2_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg2_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg2_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W853: cst_while_neg2_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg2_logic_mvp_ok() {
    // W853 / ADR-1197 face2: cst_while_neg2_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg2_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg2_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg2_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W853: cst_while_neg2_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg2_mixed_mvp_ok() {
    // W853 / ADR-1197 face3: cst_while_neg2_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg2_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg2_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg2_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W853: cst_while_neg2_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg3_bare_mvp_ok() {
    // W854 / ADR-1198 face1: cst_while_neg3_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg3_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg3_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg3_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W854: cst_while_neg3_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg3_logic_mvp_ok() {
    // W854 / ADR-1198 face2: cst_while_neg3_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg3_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg3_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg3_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W854: cst_while_neg3_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg3_mixed_mvp_ok() {
    // W854 / ADR-1198 face3: cst_while_neg3_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg3_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg3_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg3_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W854: cst_while_neg3_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg4_bare_mvp_ok() {
    // W855 / ADR-1199 face1: cst_while_neg4_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg4_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg4_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg4_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W855: cst_while_neg4_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg4_logic_mvp_ok() {
    // W855 / ADR-1199 face2: cst_while_neg4_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg4_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg4_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg4_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W855: cst_while_neg4_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg4_mixed_mvp_ok() {
    // W855 / ADR-1199 face3: cst_while_neg4_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg4_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg4_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg4_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W855: cst_while_neg4_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg5_bare_mvp_ok() {
    // W856 / ADR-1200 face1: cst_while_neg5_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg5_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg5_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg5_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W856: cst_while_neg5_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg5_logic_mvp_ok() {
    // W856 / ADR-1200 face2: cst_while_neg5_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg5_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg5_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg5_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W856: cst_while_neg5_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg5_mixed_mvp_ok() {
    // W856 / ADR-1200 face3: cst_while_neg5_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg5_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg5_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg5_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W856: cst_while_neg5_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg6_bare_mvp_ok() {
    // W857 / ADR-1201 face1: cst_while_neg6_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg6_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg6_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg6_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W857: cst_while_neg6_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg6_logic_mvp_ok() {
    // W857 / ADR-1201 face2: cst_while_neg6_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg6_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg6_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg6_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W857: cst_while_neg6_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg6_mixed_mvp_ok() {
    // W857 / ADR-1201 face3: cst_while_neg6_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg6_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg6_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg6_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W857: cst_while_neg6_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg7_bare_mvp_ok() {
    // W858 / ADR-1202 face1: cst_while_neg7_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg7_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg7_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg7_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W858: cst_while_neg7_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg7_logic_mvp_ok() {
    // W858 / ADR-1202 face2: cst_while_neg7_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg7_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg7_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg7_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W858: cst_while_neg7_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg7_mixed_mvp_ok() {
    // W858 / ADR-1202 face3: cst_while_neg7_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg7_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg7_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg7_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W858: cst_while_neg7_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg8_bare_mvp_ok() {
    // W859 / ADR-1203 face1: cst_while_neg8_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg8_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg8_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg8_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W859: cst_while_neg8_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg8_logic_mvp_ok() {
    // W859 / ADR-1203 face2: cst_while_neg8_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg8_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg8_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg8_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W859: cst_while_neg8_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg8_mixed_mvp_ok() {
    // W859 / ADR-1203 face3: cst_while_neg8_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg8_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg8_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg8_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W859: cst_while_neg8_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg9_bare_mvp_ok() {
    // W860 / ADR-1204 face1: cst_while_neg9_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg9_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg9_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg9_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W860: cst_while_neg9_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg9_logic_mvp_ok() {
    // W860 / ADR-1204 face2: cst_while_neg9_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg9_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg9_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg9_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W860: cst_while_neg9_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg9_mixed_mvp_ok() {
    // W860 / ADR-1204 face3: cst_while_neg9_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg9_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg9_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg9_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W860: cst_while_neg9_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg10_bare_mvp_ok() {
    // W861 / ADR-1205 face1: cst_while_neg10_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg10_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg10_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg10_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W861: cst_while_neg10_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg10_logic_mvp_ok() {
    // W861 / ADR-1205 face2: cst_while_neg10_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg10_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg10_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg10_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W861: cst_while_neg10_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg10_mixed_mvp_ok() {
    // W861 / ADR-1205 face3: cst_while_neg10_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg10_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg10_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg10_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W861: cst_while_neg10_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg11_bare_mvp_ok() {
    // W862 / ADR-1206 face1: cst_while_neg11_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg11_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg11_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg11_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W862: cst_while_neg11_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg11_logic_mvp_ok() {
    // W862 / ADR-1206 face2: cst_while_neg11_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg11_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg11_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg11_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W862: cst_while_neg11_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg11_mixed_mvp_ok() {
    // W862 / ADR-1206 face3: cst_while_neg11_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg11_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg11_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg11_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W862: cst_while_neg11_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_neg12_bare_mvp_ok() {
    // W863 / ADR-1207 face1: cst_while_neg12_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg12_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg12_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg12_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W863: cst_while_neg12_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg12_logic_mvp_ok() {
    // W863 / ADR-1207 face2: cst_while_neg12_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg12_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg12_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg12_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W863: cst_while_neg12_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg12_mixed_mvp_ok() {
    // W863 / ADR-1207 face3: cst_while_neg12_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg12_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg12_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg12_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W863: cst_while_neg12_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_neg13_bare_mvp_ok() {
    // W864 / ADR-1208 face1: cst_while_neg13_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg13_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg13_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg13_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W864: cst_while_neg13_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg13_logic_mvp_ok() {
    // W864 / ADR-1208 face2: cst_while_neg13_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg13_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg13_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg13_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W864: cst_while_neg13_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg13_mixed_mvp_ok() {
    // W864 / ADR-1208 face3: cst_while_neg13_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg13_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg13_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg13_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W864: cst_while_neg13_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_neg14_bare_mvp_ok() {
    // W865 / ADR-1209 face1: cst_while_neg14_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg14_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg14_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg14_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W865: cst_while_neg14_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg14_logic_mvp_ok() {
    // W865 / ADR-1209 face2: cst_while_neg14_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg14_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg14_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg14_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W865: cst_while_neg14_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg14_mixed_mvp_ok() {
    // W865 / ADR-1209 face3: cst_while_neg14_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg14_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg14_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg14_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W865: cst_while_neg14_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_neg15_bare_mvp_ok() {
    // W866 / ADR-1210 face1: cst_while_neg15_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg15_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg15_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg15_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W866: cst_while_neg15_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg15_logic_mvp_ok() {
    // W866 / ADR-1210 face2: cst_while_neg15_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg15_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg15_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg15_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W866: cst_while_neg15_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg15_mixed_mvp_ok() {
    // W866 / ADR-1210 face3: cst_while_neg15_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg15_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg15_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg15_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W866: cst_while_neg15_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_neg16_bare_mvp_ok() {
    // W867 / ADR-1211 face1: cst_while_neg16_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg16_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg16_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg16_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W867: cst_while_neg16_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg16_logic_mvp_ok() {
    // W867 / ADR-1211 face2: cst_while_neg16_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg16_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg16_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg16_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W867: cst_while_neg16_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg16_mixed_mvp_ok() {
    // W867 / ADR-1211 face3: cst_while_neg16_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg16_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg16_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg16_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W867: cst_while_neg16_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_neg17_bare_mvp_ok() {
    // W868 / ADR-1212 face1: cst_while_neg17_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg17_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg17_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg17_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W868: cst_while_neg17_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg17_logic_mvp_ok() {
    // W868 / ADR-1212 face2: cst_while_neg17_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg17_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg17_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg17_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W868: cst_while_neg17_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg17_mixed_mvp_ok() {
    // W868 / ADR-1212 face3: cst_while_neg17_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg17_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg17_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg17_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W868: cst_while_neg17_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_neg18_bare_mvp_ok() {
    // W869 / ADR-1213 face1: cst_while_neg18_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg18_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg18_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg18_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W869: cst_while_neg18_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg18_logic_mvp_ok() {
    // W869 / ADR-1213 face2: cst_while_neg18_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg18_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg18_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg18_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W869: cst_while_neg18_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_neg18_mixed_mvp_ok() {
    // W869 / ADR-1213 face3: cst_while_neg18_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_neg18_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_neg18_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_neg18_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W869: cst_while_neg18_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}


#[test]
fn fmt_check_cst_while_bang4_bare_mvp_ok() {
    // W870 / ADR-1214 face1: cst_while_bang4_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang4_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang4_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang4_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W870: cst_while_bang4_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang4_logic_mvp_ok() {
    // W870 / ADR-1214 face2: cst_while_bang4_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang4_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang4_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang4_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W870: cst_while_bang4_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_bang4_mixed_mvp_ok() {
    // W870 / ADR-1214 face3: cst_while_bang4_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_bang4_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_bang4_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_bang4_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W870: cst_while_bang4_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren3_bare_mvp_ok() {
    // W871 / ADR-1215 face1: cst_while_paren3_bare MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren3_bare_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren3_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren3_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W871: cst_while_paren3_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren3_logic_mvp_ok() {
    // W871 / ADR-1215 face2: cst_while_paren3_logic MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren3_logic_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren3_logic_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren3_logic_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W871: cst_while_paren3_logic_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_while_paren3_mixed_mvp_ok() {
    // W871 / ADR-1215 face3: cst_while_paren3_mixed MVP must --check green (default AST).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_while_paren3_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_while_paren3_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_while_paren3_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W871: cst_while_paren3_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_comment_line_mvp_ok() {
    // W873 / ADR-1217: default check preserves comments (CST-first).
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_comment_line_mvp");
    let report = format_check_path(&dir).expect("cst_comment_line_mvp default check");
    assert!(
        report.checked >= 5,
        "cst_comment_line_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W873: cst_comment_line_mvp must be default --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_comment_stmt_mvp_ok() {
    // W873 face2: default check.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_comment_stmt_mvp");
    let report = format_check_path(&dir).expect("cst_comment_stmt_mvp default check");
    assert!(
        report.checked >= 5,
        "cst_comment_stmt_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W873: cst_comment_stmt_mvp must be default --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_comment_mixed_mvp_ok() {
    // W873 face3: default check.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_comment_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_comment_mixed_mvp default check");
    assert!(
        report.checked >= 5,
        "cst_comment_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W873: cst_comment_mixed_mvp must be default --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_access_public_mvp_ok() {
    // W875 / ADR-1219 face1: public func + comments MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_access_public_mvp");
    let report = format_check_path(&dir).expect("cst_func_access_public_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_access_public_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W875: cst_func_access_public_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_access_private_mvp_ok() {
    // W875 / ADR-1219 face2: private family MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_access_private_mvp");
    let report = format_check_path(&dir).expect("cst_func_access_private_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_access_private_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W875: cst_func_access_private_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_access_mixed_mvp_ok() {
    // W875 / ADR-1219 face3: access mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_access_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_func_access_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_access_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W875: cst_func_access_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_static_mvp_ok() {
    // W876 / ADR-1220 face1: static func + comments MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_static_mvp");
    let report = format_check_path(&dir).expect("cst_func_static_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_static_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W876: cst_func_static_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_async_mvp_ok() {
    // W876 / ADR-1220 face2: async func MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_async_mvp");
    let report = format_check_path(&dir).expect("cst_func_async_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_async_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W876: cst_func_async_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_static_async_mixed_mvp_ok() {
    // W876 / ADR-1220 face3: static/async mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_static_async_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_func_static_async_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_static_async_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W876: cst_func_static_async_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_if_bare_mvp_ok() {
    // W877 / ADR-1221 face1: if bare + comments MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_if_bare_mvp");
    let report = format_check_path(&dir).expect("cst_if_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_if_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W877: cst_if_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_if_else_mvp_ok() {
    // W877 / ADR-1221 face2: if/else MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_if_else_mvp");
    let report = format_check_path(&dir).expect("cst_if_else_mvp check");
    assert!(
        report.checked >= 5,
        "cst_if_else_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W877: cst_if_else_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_if_mixed_mvp_ok() {
    // W877 / ADR-1221 face3: if mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_if_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_if_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_if_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W877: cst_if_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_export_bare_mvp_ok() {
    // W878 / ADR-1222 face1: export func bare + comments MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_export_bare_mvp");
    let report = format_check_path(&dir).expect("cst_func_export_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_export_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W878: cst_func_export_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_export_mod_mvp_ok() {
    // W878 / ADR-1222 face2: export static/async MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_export_mod_mvp");
    let report = format_check_path(&dir).expect("cst_func_export_mod_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_export_mod_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W878: cst_func_export_mod_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_func_export_mixed_mvp_ok() {
    // W878 / ADR-1222 face3: export mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_func_export_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_func_export_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_func_export_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W878: cst_func_export_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_bare_mvp_ok() {
    // W879 / ADR-1223 face1: class bare + comments MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_bare_mvp");
    let report = format_check_path(&dir).expect("cst_class_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W879: cst_class_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_members_mvp_ok() {
    // W879 / ADR-1223 face2: class members MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_members_mvp");
    let report = format_check_path(&dir).expect("cst_class_members_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_members_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W879: cst_class_members_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_mixed_mvp_ok() {
    // W879 / ADR-1223 face3: class mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_class_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W879: cst_class_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_inherit_mvp_ok() {
    // W880 / ADR-1224 face1: class inherit MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_inherit_mvp");
    let report = format_check_path(&dir).expect("cst_class_inherit_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_inherit_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W880: cst_class_inherit_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_init_mvp_ok() {
    // W880 / ADR-1224 face2: class init MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_init_mvp");
    let report = format_check_path(&dir).expect("cst_class_init_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_init_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W880: cst_class_init_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_deep_mixed_mvp_ok() {
    // W880 / ADR-1224 face3: class deep mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_deep_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_class_deep_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_deep_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W880: cst_class_deep_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_data_class_mvp_ok() {
    // W881 / ADR-1225 face1: data class MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_data_class_mvp");
    let report = format_check_path(&dir).expect("cst_data_class_mvp check");
    assert!(
        report.checked >= 5,
        "cst_data_class_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W881: cst_data_class_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_final_class_mvp_ok() {
    // W881 / ADR-1225 face2: final class MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_final_class_mvp");
    let report = format_check_path(&dir).expect("cst_final_class_mvp check");
    assert!(
        report.checked >= 5,
        "cst_final_class_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W881: cst_final_class_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_data_final_mixed_mvp_ok() {
    // W881 / ADR-1225 face3: data/final mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_data_final_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_data_final_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_data_final_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W881: cst_data_final_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_data_class_tp_mvp_ok() {
    // W882 / ADR-1226 face1: data class tp MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_data_class_tp_mvp");
    let report = format_check_path(&dir).expect("cst_data_class_tp_mvp check");
    assert!(
        report.checked >= 5,
        "cst_data_class_tp_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W882: cst_data_class_tp_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_data_class_where_mvp_ok() {
    // W882 / ADR-1226 face2: data class where MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_data_class_where_mvp");
    let report = format_check_path(&dir).expect("cst_data_class_where_mvp check");
    assert!(
        report.checked >= 5,
        "cst_data_class_where_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W882: cst_data_class_where_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_data_final_gw_mixed_mvp_ok() {
    // W882 / ADR-1226 face3: data/final gw mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_data_final_gw_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_data_final_gw_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_data_final_gw_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W882: cst_data_final_gw_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_export_list_bare_mvp_ok() {
    // W883 / ADR-1227 face1: export list bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_export_list_bare_mvp");
    let report = format_check_path(&dir).expect("cst_export_list_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_export_list_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W883: cst_export_list_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_export_list_multi_mvp_ok() {
    // W883 / ADR-1227 face2: export list multi MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_export_list_multi_mvp");
    let report = format_check_path(&dir).expect("cst_export_list_multi_mvp check");
    assert!(
        report.checked >= 5,
        "cst_export_list_multi_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W883: cst_export_list_multi_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_export_list_mixed_mvp_ok() {
    // W883 / ADR-1227 face3: export list mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_export_list_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_export_list_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_export_list_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W883: cst_export_list_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_final_where_bare_mvp_ok() {
    // W884 / ADR-1228 face1: final+where bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_final_where_bare_mvp");
    let report = format_check_path(&dir).expect("cst_final_where_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_final_where_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W884: cst_final_where_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_final_where_multi_mvp_ok() {
    // W884 / ADR-1228 face2: final+where multi MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_final_where_multi_mvp");
    let report = format_check_path(&dir).expect("cst_final_where_multi_mvp check");
    assert!(
        report.checked >= 5,
        "cst_final_where_multi_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W884: cst_final_where_multi_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_final_where_mixed_mvp_ok() {
    // W884 / ADR-1228 face3: final+where mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_final_where_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_final_where_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_final_where_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W884: cst_final_where_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_export_as_bare_mvp_ok() {
    // W885 / ADR-1229 face1: export as-rename bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_export_as_bare_mvp");
    let report = format_check_path(&dir).expect("cst_export_as_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_export_as_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W885: cst_export_as_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_export_empty_mvp_ok() {
    // W885 / ADR-1229 face2: empty export MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_export_empty_mvp");
    let report = format_check_path(&dir).expect("cst_export_empty_mvp check");
    assert!(
        report.checked >= 5,
        "cst_export_empty_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W885: cst_export_empty_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_export_as_empty_mixed_mvp_ok() {
    // W885 / ADR-1229 face3: as × empty mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_export_as_empty_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_export_as_empty_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_export_as_empty_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W885: cst_export_as_empty_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_multi_conf_mvp_ok() {
    // W886 / ADR-1230 face1: multi-conformance MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_multi_conf_mvp");
    let report = format_check_path(&dir).expect("cst_class_multi_conf_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_multi_conf_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W886: cst_class_multi_conf_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_deinit_mvp_ok() {
    // W886 / ADR-1230 face2: deinit MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_deinit_mvp");
    let report = format_check_path(&dir).expect("cst_class_deinit_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_deinit_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W886: cst_class_deinit_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_class_multi_deinit_mixed_mvp_ok() {
    // W886 / ADR-1230 face3: multi × deinit mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_class_multi_deinit_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_class_multi_deinit_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_class_multi_deinit_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W886: cst_class_multi_deinit_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_self_field_bare_mvp_ok() {
    // W887 / ADR-1231 face1: init self.field= MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_self_field_bare_mvp");
    let report = format_check_path(&dir).expect("cst_self_field_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_self_field_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W887: cst_self_field_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_self_field_method_mvp_ok() {
    // W887 / ADR-1231 face2: method self.field= MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_self_field_method_mvp");
    let report = format_check_path(&dir).expect("cst_self_field_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_self_field_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W887: cst_self_field_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_self_field_mixed_mvp_ok() {
    // W887 / ADR-1231 face3: self.field= mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_self_field_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_self_field_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_self_field_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W887: cst_self_field_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_nested_self_field_bare_mvp_ok() {
    // W888 / ADR-1232 face1: nested self.a.b= bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_nested_self_field_bare_mvp");
    let report = format_check_path(&dir).expect("cst_nested_self_field_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_nested_self_field_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W888: cst_nested_self_field_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_nested_self_field_method_mvp_ok() {
    // W888 / ADR-1232 face2: nested self.a.b= method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_nested_self_field_method_mvp");
    let report = format_check_path(&dir).expect("cst_nested_self_field_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_nested_self_field_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W888: cst_nested_self_field_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_nested_self_field_mixed_mvp_ok() {
    // W888 / ADR-1232 face3: nested self.a.b= mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_nested_self_field_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_nested_self_field_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_nested_self_field_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W888: cst_nested_self_field_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_bare_mvp_ok() {
    // W889 / ADR-1233 face1: non-self member assign bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_bare_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W889: cst_member_field_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_method_mvp_ok() {
    // W889 / ADR-1233 face2: non-self member assign method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_method_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W889: cst_member_field_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_mixed_mvp_ok() {
    // W889 / ADR-1233 face3: non-self member assign mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W889: cst_member_field_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_plus_bare_mvp_ok() {
    // W890 / ADR-1234 face1: member += bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_plus_bare_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_plus_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_plus_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W890: cst_member_field_plus_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_plus_method_mvp_ok() {
    // W890 / ADR-1234 face2: member += method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_plus_method_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_plus_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_plus_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W890: cst_member_field_plus_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_plus_mixed_mvp_ok() {
    // W890 / ADR-1234 face3: member += mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_plus_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_plus_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_plus_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W890: cst_member_field_plus_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_minus_bare_mvp_ok() {
    // W891 / ADR-1235 face1: member -= bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_minus_bare_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_minus_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_minus_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W891: cst_member_field_minus_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_minus_method_mvp_ok() {
    // W891 / ADR-1235 face2: member -= method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_minus_method_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_minus_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_minus_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W891: cst_member_field_minus_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_minus_mixed_mvp_ok() {
    // W891 / ADR-1235 face3: member -= mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_minus_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_minus_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_minus_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W891: cst_member_field_minus_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_star_bare_mvp_ok() {
    // W892 / ADR-1236 face1: member *= bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_star_bare_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_star_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_star_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W892: cst_member_field_star_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_star_method_mvp_ok() {
    // W892 / ADR-1236 face2: member *=/= method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_star_method_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_star_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_star_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W892: cst_member_field_star_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_field_star_mixed_mvp_ok() {
    // W892 / ADR-1236 face3: member *=/%= mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_field_star_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_member_field_star_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_field_star_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W892: cst_member_field_star_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_ident_compound_bare_mvp_ok() {
    // W893 / ADR-1237 face1: bare ident compound MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_ident_compound_bare_mvp");
    let report = format_check_path(&dir).expect("cst_ident_compound_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_ident_compound_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W893: cst_ident_compound_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_ident_compound_method_mvp_ok() {
    // W893 / ADR-1237 face2: ident *=/= method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_ident_compound_method_mvp");
    let report = format_check_path(&dir).expect("cst_ident_compound_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_ident_compound_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W893: cst_ident_compound_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_ident_compound_mixed_mvp_ok() {
    // W893 / ADR-1237 face3: ident×member compound mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_ident_compound_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_ident_compound_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_ident_compound_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W893: cst_ident_compound_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_assign_bare_mvp_ok() {
    // W894 / ADR-1238 face1: IndexAssign bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_assign_bare_mvp");
    let report = format_check_path(&dir).expect("cst_index_assign_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_assign_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W894: cst_index_assign_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_assign_method_mvp_ok() {
    // W894 / ADR-1238 face2: IndexAssign method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_assign_method_mvp");
    let report = format_check_path(&dir).expect("cst_index_assign_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_assign_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W894: cst_index_assign_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_assign_mixed_mvp_ok() {
    // W894 / ADR-1238 face3: IndexAssign mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_assign_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_index_assign_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_assign_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W894: cst_index_assign_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_compound_bare_mvp_ok() {
    // W895 / ADR-1239 face1: IndexAssign compound bare MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_compound_bare_mvp");
    let report = format_check_path(&dir).expect("cst_index_compound_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_compound_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W895: cst_index_compound_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_compound_method_mvp_ok() {
    // W895 / ADR-1239 face2: IndexAssign compound method MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_compound_method_mvp");
    let report = format_check_path(&dir).expect("cst_index_compound_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_compound_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W895: cst_index_compound_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_compound_mixed_mvp_ok() {
    // W895 / ADR-1239 face3: IndexAssign compound mixed MVP must --check green.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_compound_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_index_compound_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_compound_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.ok(),
        "W895: cst_index_compound_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_nested_bare_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_nested_bare_mvp");
    let report = format_check_path(&dir).expect("cst_index_nested_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_nested_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W896: cst_index_nested_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_nested_method_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_nested_method_mvp");
    let report = format_check_path(&dir).expect("cst_index_nested_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_nested_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W896: cst_index_nested_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_nested_mixed_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_nested_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_index_nested_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_nested_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W896: cst_index_nested_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_index_assign_bare_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_index_assign_bare_mvp");
    let report = format_check_path(&dir).expect("cst_member_index_assign_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_index_assign_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W897: cst_member_index_assign_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_index_assign_method_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_index_assign_method_mvp");
    let report = format_check_path(&dir).expect("cst_member_index_assign_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_index_assign_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W897: cst_member_index_assign_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_member_index_assign_mixed_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_member_index_assign_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_member_index_assign_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_member_index_assign_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W897: cst_member_index_assign_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_nested_compound_bare_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_nested_compound_bare_mvp");
    let report = format_check_path(&dir).expect("cst_index_nested_compound_bare_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_nested_compound_bare_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W898: cst_index_nested_compound_bare_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_nested_compound_method_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_nested_compound_method_mvp");
    let report = format_check_path(&dir).expect("cst_index_nested_compound_method_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_nested_compound_method_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W898: cst_index_nested_compound_method_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_cst_index_nested_compound_mixed_mvp_ok() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/format/cst_index_nested_compound_mixed_mvp");
    let report = format_check_path(&dir).expect("cst_index_nested_compound_mixed_mvp check");
    assert!(
        report.checked >= 5,
        "cst_index_nested_compound_mixed_mvp has >=5 .sy files; got {}",
        report.checked
    );
    assert!(
        report.would_change == 0 && report.failed == 0,
        "W898: cst_index_nested_compound_mixed_mvp must be format --check green; would_change={} failed={} failures={:?}",
        report.would_change,
        report.failed,
        report.failures
    );
}

#[test]
fn fmt_check_production_scope_honesty() {
    // W901 / ADR-1245: Production Scope = packages + crates + examples + tools,
    // excluding fixtures / sgate / syx-compiler-wasm (plus tutorial/ still out).
    // Checklist (a)(c): failed=0 + would_change=0.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut checked = 0usize;
    let mut would_change = 0usize;
    let mut failed = 0usize;
    let mut failures: Vec<(PathBuf, String)> = Vec::new();
    for sub in ["packages", "crates", "examples", "tools"] {
        let dir = root.join(sub);
        if !dir.is_dir() {
            continue;
        }
        let files = collect_production_scope_sy_files(&dir);
        for file in files {
            let report = format_check_path(&file).unwrap_or_else(|e| {
                panic!("production scope check {}: {e}", file.display());
            });
            checked += report.checked;
            would_change += report.would_change;
            failed += report.failed;
            failures.extend(report.failures);
        }
    }
    assert!(
        checked >= 100,
        "production scope should cover product .sy; got checked={checked}"
    );
    assert_eq!(
        would_change, 0,
        "production scope would_change must be 0; got {would_change}"
    );
    assert_eq!(
        failed, 0,
        "W901: production scope failed must be 0; failed={failed} checked={checked} sample={:?}",
        failures.iter().take(8).collect::<Vec<_>>()
    );
    let green = checked - failed - would_change;
    eprintln!(
        "W901 production_scope_honesty: checked={checked} failed={failed} would_change={would_change} green={green}"
    );
    assert!(
        green >= 100,
        "production scope green floor for report; green={green}"
    );
}

/// W900: walk `.sy` under `dir`, skipping `tests/fixtures` segments and `sgate`.
fn collect_production_scope_sy_files(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    fn walk(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name == ".sy" || name == "sgate" {
                    continue;
                }
                // W901: compiler-wasm prelude is toolchain corpus, not product packages.
                if name == "syx-compiler-wasm" {
                    continue;
                }
                // Skip `tests/fixtures` corpus (non-product).
                if name == "fixtures" {
                    let parent = p.parent().and_then(|x| x.file_name()).and_then(|n| n.to_str());
                    if parent == Some("tests") {
                        continue;
                    }
                }
                walk(&p, out);
            } else if p.extension().and_then(|e| e.to_str()) == Some("sy") {
                out.push(p);
            }
        }
    }
    walk(dir, &mut out);
    out.sort();
    out
}

#[test]
fn fmt_check_production_scope_comment_samples() {
    // W901 / ADR-1245: comment GA evidence samples (narrow charter) —
    // file_handle + deque + ≥3 additional Scope facades with ///.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let samples = [
        root.join("packages/io/file_handle.sy"),
        root.join("packages/core/builtins/collections/deque.sy"),
        root.join("packages/net/http_body_stream.sy"),
        root.join("packages/os/mod.sy"),
        root.join("packages/core/json.sy"),
    ];
    for path in &samples {
        let report = format_check_path(path).unwrap_or_else(|e| {
            panic!("comment sample {}: {e}", path.display());
        });
        assert_eq!(report.checked, 1, "{}", path.display());
        assert_eq!(
            report.failed, 0,
            "comment sample must parse/format: {} failures={:?}",
            path.display(),
            report.failures
        );
        assert_eq!(
            report.would_change, 0,
            "comment sample must be byte-stable (CST trivia): {}",
            path.display()
        );
        let text = fs::read_to_string(path).expect("read sample");
        assert!(
            text.contains("///"),
            "comment sample must retain /// docs: {}",
            path.display()
        );
        let cst = format_check_path_cst(path).unwrap_or_else(|e| {
            panic!("comment sample CST-only {}: {e}", path.display());
        });
        assert_eq!(
            cst.failed, 0,
            "comment sample must be CST-green (no AST fallback): {} failures={:?}",
            path.display(),
            cst.failures
        );
        assert_eq!(
            cst.would_change, 0,
            "comment sample CST dry-run stable: {}",
            path.display()
        );
        let source = SourceFile::new(path.display().to_string(), text);
        syx_cst::parse_cst_subset(&source).unwrap_or_else(|e| {
            panic!(
                "comment sample parse_cst_subset must succeed: {} err={e}",
                path.display()
            );
        });
    }
}

/// W901: Production Scope files with real LineComment/BlockComment tokens must be
/// format --check green + parse_cst_subset OK (narrow comment-preserving GA charter).
#[test]
fn fmt_check_production_scope_comment_scan() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut scanned = 0usize;
    let mut failures: Vec<(PathBuf, String)> = Vec::new();
    for sub in ["packages", "crates", "examples", "tools"] {
        let dir = root.join(sub);
        if !dir.is_dir() {
            continue;
        }
        for file in collect_production_scope_sy_files(&dir) {
            let text = match fs::read_to_string(&file) {
                Ok(t) => t,
                Err(e) => {
                    failures.push((file, format!("read: {e}")));
                    continue;
                }
            };
            let source = SourceFile::new(file.display().to_string(), text.clone());
            let tokens = match Lexer::new(source.clone()).scan_all() {
                Ok(t) => t,
                Err(e) => {
                    failures.push((file, format!("lex: {e}")));
                    continue;
                }
            };
            let has_comment = tokens.iter().any(|t| {
                matches!(
                    t.kind,
                    TokenKind::LineComment | TokenKind::BlockComment
                )
            });
            if !has_comment {
                continue;
            }
            scanned += 1;
            let report = format_check_path(&file).unwrap_or_else(|e| {
                panic!("comment scan check {}: {e}", file.display());
            });
            if report.failed != 0 || report.would_change != 0 {
                failures.push((
                    file.clone(),
                    format!(
                        "format --check failed={} would_change={} {:?}",
                        report.failed, report.would_change, report.failures
                    ),
                ));
                continue;
            }
            if let Err(e) = syx_cst::parse_cst_subset(&source) {
                failures.push((
                    file,
                    format!("parse_cst_subset failed (AST fallback not GA): {e}"),
                ));
            }
        }
    }
    assert!(
        scanned >= 5,
        "comment scan should cover ≥5 Scope files with real comments; got {scanned}"
    );
    assert!(
        failures.is_empty(),
        "W901 comment scan (narrow GA): scanned={scanned} failures={failures:?}"
    );
    eprintln!("W901 production_scope_comment_scan: scanned={scanned} failures=0");
}

#[test]
fn fmt_check_baseline_honesty() {
    // Honesty nail (W901 / ADR-1245): would_change=0; green≥1915; failed still high (tutorial etc.).
    // Full-repo (incl. tutorial/) contrast — C-06 [x] is Production Scope + narrow comment GA, not full-repo.
    // Production Scope flip path is c06_fmt_check_production_scope_honesty (ADR-1243/1244).
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let report = format_check_path(&root).expect("repo root check");
    assert!(
        report.checked >= 300 && report.checked <= 3510,
        "checked≈380–3510; got {}",
        report.checked
    );
    assert_eq!(
        report.would_change, 0,
        "W834 corpus normalize requires would_change=0; got {}",
        report.would_change
    );
    // W900: builtins/@static/F002 cut full-repo failed (~110→~48); still >0 (tutorial etc.).
    assert!(
        report.failed >= 40 && report.failed < 380,
        "failed still high (tutorial / residual packages etc.); failed={} checked={}",
        report.failed,
        report.checked
    );
    let green = report.checked - report.failed - report.would_change;
    assert!(
        green >= 1915,
        "green files >=1915 (W898 floor retained; W900 does not pretend full-repo green); green={green}"
    );
    // W687–W703 honesty: restored protocols must keep public + inheritance after normalize.
    let codable = root.join("packages/core/protocols/codable.sy");
    let text = fs::read_to_string(&codable).expect("codable.sy");
    assert!(
        text.contains("public protocol Codable") && text.contains("Encodable"),
        "codable.sy must retain public + inheritance; got {text:?}"
    );
}


fn tempfile_dir() -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "syx-c06-fmt-{}-{}-{}",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}
