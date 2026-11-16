//! U-22: `let` subscript assign → S060 (same family as mutating `append`; ≠ MainActor S020).

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
fn u22_let_array_subscript_is_s060() {
    assert!(
        has_error(
            r#"
public func main() {
    let a: Array<Int> = [1]
    a[0] = 2
}
"#,
            "S060"
        ),
        "let Array subscript assign must be S060"
    );
}

#[test]
fn u22_let_dict_subscript_is_s060() {
    assert!(
        has_error(
            r#"
public func main() {
    let d: Dictionary<String, Int> = {"a": 1}
    d["b"] = 2
}
"#,
            "S060"
        ),
        "let Dictionary subscript assign must be S060"
    );
}

#[test]
fn u22_var_array_subscript_ok() {
    assert!(
        !has_error(
            r#"
public func main() {
    var a: Array<Int> = [1]
    a[0] = 2
}
"#,
            "S060"
        ),
        "var Array subscript assign must not be S060"
    );
}

#[test]
fn u22_let_append_still_s060() {
    assert!(
        has_error(
            r#"
public func main() {
    let a: Array<Int> = []
    a.append(1)
}
"#,
            "S060"
        ),
        "let append must still be S060"
    );
}

#[test]
fn u22_var_subscript_runs() {
    let path = write_temp(
        r#"
public func main() {
    var a: Array<Int> = [1]
    a[0] = 9
    print(a[0])
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("var subscript must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["9"]);
    let _ = std::fs::remove_file(&path);
}
