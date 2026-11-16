//! U-21: S020 bag honesty — mutating→S060, await-outside-async→A002 (≠ MainActor S020).

use syx_diagnostics::SourceFile;
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

fn semantic_has(src: &str, code: &str) -> bool {
    let file = SourceFile::new("test.sy", src);
    let module = Parser::parse(file).unwrap();
    match SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

fn check_err_contains(src: &str, code: &str) -> bool {
    let path = write_temp(src);
    let hit =
        match syx_driver::check_file_with_options(&path, &syx_driver::DriverOptions::default()) {
            Ok(_) => false,
            Err(diags) => diags
                .iter()
                .any(|d| d.code.as_deref() == Some(code) || format!("{d:?}").contains(code)),
        };
    let _ = std::fs::remove_file(&path);
    hit
}

#[test]
fn u21_mutating_let_is_s060() {
    let src = r#"
public func main() {
    let a: Array<Int> = []
    a.append(1)
}
"#;
    assert!(semantic_has(src, "S060"), "mutating on let must be S060");
    assert!(
        !semantic_has(src, "S020"),
        "mutating on let must not be S020"
    );
}

#[test]
fn u21_await_outside_async_is_a002() {
    let src = r#"
func f() -> Int {
    return await g()
}
async func g() -> Int { return 1 }
public func main() { print(1) }
"#;
    assert!(
        check_err_contains(src, "A002"),
        "await outside async must be A002"
    );
    assert!(
        !check_err_contains(src, "S020"),
        "await outside async must not be S020"
    );
}

#[test]
fn u21_mutating_not_s020_via_check() {
    let src = r#"
public func main() {
    let a: Array<Int> = []
    a.append(1)
}
"#;
    assert!(
        check_err_contains(src, "S060"),
        "check path must report S060"
    );
    assert!(
        !check_err_contains(src, "S020"),
        "check path must not report S020 for mutating"
    );
}
