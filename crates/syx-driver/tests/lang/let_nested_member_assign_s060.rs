//! U-24: nested let value-type MemberAssign → S060 (class hop stays legal; Ident U-23 intact).

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
fn u24_let_nested_struct_member_is_s060() {
    assert!(
        has_error(
            r#"
struct Inner { var x: Int }
struct Outer { var inner: Inner }
public func main() {
    let o = Outer(inner: Inner(x: 1))
    o.inner.x = 2
}
"#,
            "S060"
        ),
        "let nested struct member assign must be S060"
    );
}

#[test]
fn u24_let_struct_holding_class_ok() {
    assert!(
        !has_error(
            r#"
class Cell {
    var x: Int
    init(x: Int) { self.x = x }
}
struct Wrap { var cell: Cell }
public func main() {
    let w = Wrap(cell: Cell(x: 1))
    w.cell.x = 2
}
"#,
            "S060"
        ),
        "let struct→class field write must not be S060"
    );
}

#[test]
fn u24_var_nested_struct_ok() {
    assert!(
        !has_error(
            r#"
struct Inner { var x: Int }
struct Outer { var inner: Inner }
public func main() {
    var o = Outer(inner: Inner(x: 1))
    o.inner.x = 2
}
"#,
            "S060"
        ),
        "var nested struct member assign must not be S060"
    );
}

#[test]
fn u24_let_ident_struct_still_s060() {
    assert!(
        has_error(
            r#"
struct Point { var x: Int }
public func main() {
    let p = Point(x: 1)
    p.x = 3
}
"#,
            "S060"
        ),
        "let Ident struct member assign must still be S060"
    );
}

#[test]
fn u24_let_class_hop_runs() {
    let path = write_temp(
        r#"
class Cell {
    var x: Int
    init(x: Int) { self.x = x }
}
struct Wrap { var cell: Cell }
public func main() {
    let w = Wrap(cell: Cell(x: 1))
    w.cell.x = 2
    print(w.cell.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("let struct→class member must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["2"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u24_var_nested_struct_runs() {
    let path = write_temp(
        r#"
struct Inner { var x: Int }
struct Outer { var inner: Inner }
public func main() {
    var o = Outer(inner: Inner(x: 1))
    o.inner.x = 2
    print(o.inner.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("var nested struct member must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["2"]
    );
    let _ = std::fs::remove_file(&path);
}
