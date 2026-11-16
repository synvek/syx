//! U-19: `throw` only legal inside `throws` → S028 (≠ MainActor S020; ≠ missing-try S018).

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

#[test]
fn u19_throw_outside_throws_is_s028() {
    assert!(
        has_error(
            r#"
public func main() {
    throw "x"
}
"#,
            "S028"
        ),
        "throw in non-throws must be S028"
    );
}

#[test]
fn u19_throw_outside_throws_not_s020() {
    assert!(
        !has_error(
            r#"
public func main() {
    throw "x"
}
"#,
            "S020"
        ),
        "throw in non-throws must not be MainActor S020"
    );
}

#[test]
fn u19_throw_inside_throws_ok() {
    assert!(
        !has_error(
            r#"
func f() throws {
    throw "x"
}
public func main() {
    do {
        try f()
    } catch {
        print("c")
    }
}
"#,
            "S028"
        ),
        "throw inside throws must not be S028"
    );
}

#[test]
fn u19_throws_do_catch_runs() {
    let path = write_temp(
        r#"
func f() throws {
    throw "x"
}
public func main() {
    do {
        try f()
    } catch {
        print("caught")
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("throws + do/catch must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["caught"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u19_catch_let_path_still_ok() {
    assert!(
        !has_error(
            r#"
func f() throws {
    throw "boom"
}
public func main() {
    do {
        try f()
    } catch let e {
        print(e)
    }
}
"#,
            "S028"
        ),
        "catch let + throws throw must stay green"
    );
}
