//! R-26 Swift-style closure statement-body contract.
//!
//! `{ (x: Int) -> Int in return x * 2 }` must parse (Block body). Previously
//! `parse_swift_closure` never consulted `looks_like_closure_stmt_body` → P001.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn r26_parse_swift_closure_uses_stmt_body_helper() {
    let src = std::fs::read_to_string(repo_root().join("crates/syx-parser/src/lib.rs"))
        .expect("parser lib.rs");
    let swift = src
        .split("fn parse_swift_closure(")
        .nth(1)
        .and_then(|s| s.split("fn parse_closure_params(").next())
        .expect("parse_swift_closure body");
    assert!(
        swift.contains("looks_like_closure_stmt_body()"),
        "parse_swift_closure must call looks_like_closure_stmt_body (R-26)"
    );
    assert!(
        src.contains("r26_swift_closure_stmt_body_return")
            && src.contains("r26_swift_closure_expr_body_preserved"),
        "parser unit tests for stmt vs expr body must exist"
    );
}

#[test]
fn r26_tutorial_08_closures_checks_clean() {
    let path = repo_root().join("tutorial/08-closures.sy");
    assert!(path.is_file(), "tutorial/08-closures.sy must exist");
    let syx = repo_root().join("target/debug/syx");
    let bin = if syx.is_file() {
        syx
    } else {
        // Fall back to PATH after `cargo build -p syx-cli` in CI.
        PathBuf::from("syx")
    };
    let out = Command::new(&bin)
        .arg("check")
        .arg(&path)
        .output()
        .expect("syx check");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let combined = format!("{stdout}{stderr}");
    assert!(
        !combined.contains("P001"),
        "tutorial/08 must have zero P001 after R-26; got:\n{combined}"
    );
    assert!(
        out.status.success(),
        "syx check tutorial/08-closures.sy must succeed; got:\n{combined}"
    );
}

#[test]
fn r26_no_ignore_on_hof_closure_path() {
    let hof = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/runtime/hof_closure.rs"),
    )
    .expect("hof_closure.rs");
    assert!(
        !hof.contains("#[ignore]"),
        "fail-closed: do not #[ignore] hof_closure tests (L006 native may fail honestly)"
    );
}
