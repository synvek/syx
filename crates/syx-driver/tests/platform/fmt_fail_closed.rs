//! Formatter fail-closed — unsupported AST must not rewrite files.

use std::fs;
use std::path::PathBuf;

use syx_diagnostics::SourceFile;
use syx_driver::format_project;
use syx_fmt::format_source;

/// IfCase KeyPath pattern → F001 (`if case pattern`; lazy computed now formats).
const CASE_PATTERN_NEG: &str =
    "func main() {\n    if case \\Foo.bar = x {\n        print(1)\n    }\n}\n";

#[test]
fn fmt_fail_closed_format_source_rejects_lazy() {
    let source = SourceFile::new("t.sy", CASE_PATTERN_NEG);
    let err = format_source(&source).expect_err("must fail closed");
    assert!(
        err.message.contains("unsupported") || err.code.as_deref() == Some("F001"),
        "{err:?}"
    );
    assert!(
        err.message.contains("if case pattern"),
        "expected if case pattern tip; got {err:?}"
    );
}

#[test]
fn fmt_fail_closed_format_project_does_not_write_on_failure() {
    let dir = tempfile_dir();
    let main = dir.join("main.sy");
    let original = CASE_PATTERN_NEG;
    fs::write(&main, original).expect("write");
    fs::write(
        dir.join("syx.json"),
        r#"{"name":"m0-fmt","version":"0.0.1","main":"main.sy"}"#,
    )
    .expect("manifest");
    let err = format_project(&dir).expect_err("format must fail");
    assert!(
        err.message.contains("unsupported") || err.code.as_deref() == Some("F001"),
        "{err:?}"
    );
    let after = fs::read_to_string(&main).expect("read back");
    assert_eq!(after, original, "must not rewrite on F001");
}

fn tempfile_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("syx-m0-fmt-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("mkdir");
    dir
}
