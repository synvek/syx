//! U-30: non-`mutating` struct/enum method writing `self` → S060.

use syx_diagnostics::SourceFile;
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

fn has_error(src: &str, code: &str) -> bool {
    let file = SourceFile::new("test.sy", src);
    let module = Parser::parse(file).unwrap();
    match SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

fn check_ok(src: &str) -> bool {
    let file = SourceFile::new("test.sy", src);
    let module = Parser::parse(file).unwrap();
    match SemanticAnalyzer::analyze(&module) {
        Ok(a) => a.errors.is_empty(),
        Err(_) => false,
    }
}

#[test]
fn u30_non_mutating_self_member_is_s060() {
    assert!(
        has_error(
            r#"
struct S {
    var x: Int = 0
    func f() { self.x = 1 }
}
"#,
            "S060"
        ),
        "non-mutating self.x= must be S060"
    );
}

#[test]
fn u30_non_mutating_implicit_field_is_s060() {
    assert!(
        has_error(
            r#"
struct S {
    var x: Int = 0
    func f() { x = 1 }
}
"#,
            "S060"
        ),
        "non-mutating implicit field assign must be S060"
    );
}

#[test]
fn u30_mutating_self_member_ok() {
    assert!(
        check_ok(
            r#"
struct S {
    var x: Int = 0
    mutating func f() { self.x = 1 }
}
"#
        ),
        "mutating self.x= must check-ok"
    );
}

#[test]
fn u30_class_self_member_ok() {
    assert!(
        check_ok(
            r#"
class C {
    var x: Int = 0
    func f() { self.x = 1 }
}
"#
        ),
        "class self.x= without mutating must check-ok"
    );
}

#[test]
fn u30_init_self_member_ok() {
    assert!(
        check_ok(
            r#"
struct S {
    var x: Int
    init(x: Int) { self.x = x }
}
"#
        ),
        "init self.x= must check-ok"
    );
}

#[test]
fn u30_let_mutating_call_still_s060() {
    assert!(
        has_error(
            r#"
struct S {
    var x: Int = 0
    mutating func f() { self.x = 1 }
}
public func main() {
    let s = S()
    s.f()
}
"#,
            "S060"
        ),
        "let + mutating call must still be S060"
    );
}

#[test]
fn u30_var_mutating_runs() {
    let path = write_temp(
        r#"
struct S {
    var x: Int = 0
    mutating func f() { self.x = 1 }
}
public func main() {
    var s = S()
    s.f()
    print(s.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("var mutating must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1"]
    );
    let _ = std::fs::remove_file(&path);
}
