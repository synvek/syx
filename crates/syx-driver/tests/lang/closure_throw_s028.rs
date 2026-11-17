//! U-32: closure body clears `current_function_is_throws` → `throw` is always S028.
//! ≠ closure `throws` syntax; ≠ field-default nil; ≠ R-50.

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
fn u32_throws_parent_closure_throw_is_s028() {
    assert!(
        has_error(
            r#"
func boom() throws {
    let c = { throw "x" }
    c()
}
public func main() {
    do { try boom() } catch { print("c") }
}
"#,
            "S028"
        ),
        "throw in closure under throws parent must be S028"
    );
}

#[test]
fn u32_non_throws_parent_closure_throw_is_s028() {
    assert!(
        has_error(
            r#"
public func main() {
    let c = { throw "x" }
    c()
}
"#,
            "S028"
        ),
        "throw in closure under non-throws parent must be S028"
    );
}

#[test]
fn u32_toplevel_throws_direct_throw_ok() {
    assert!(
        !has_error(
            r#"
func f() throws {
    throw "x"
}
public func main() {
    do { try f() } catch { print("c") }
}
"#,
            "S028"
        ),
        "direct throw inside throws must not be S028"
    );
}

#[test]
fn u32_throws_do_catch_still_runs() {
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
            .expect("throws + do/catch must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["caught"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u32_method_throws_still_ok() {
    assert!(
        !has_error(
            r#"
class C {
    func f() throws {
        throw "x"
    }
}
public func main() {
    do { try C().f() } catch { print("c") }
}
"#,
            "S028"
        ),
        "throws method throw must not be S028 (U-20 regression)"
    );
}
