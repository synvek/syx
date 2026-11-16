//! U-23: `let` struct member assign → S060 (class/actor field write stays legal; ≠ IndexAssign).

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
fn u23_let_struct_member_is_s060() {
    assert!(
        has_error(
            r#"
struct Point {
    var x: Int
    var y: Int
}
public func main() {
    let p = Point(x: 1, y: 2)
    p.x = 3
}
"#,
            "S060"
        ),
        "let struct member assign must be S060"
    );
}

#[test]
fn u23_let_class_member_ok() {
    assert!(
        !has_error(
            r#"
class Box {
    var n: Int
    init(n: Int) { self.n = n }
}
public func main() {
    let b = Box(n: 1)
    b.n = 9
}
"#,
            "S060"
        ),
        "let class member assign must not be S060"
    );
}

#[test]
fn u23_var_struct_member_ok() {
    assert!(
        !has_error(
            r#"
struct Point {
    var x: Int
    var y: Int
}
public func main() {
    var p = Point(x: 1, y: 2)
    p.x = 3
}
"#,
            "S060"
        ),
        "var struct member assign must not be S060"
    );
}

#[test]
fn u23_let_subscript_still_s060() {
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
        "let Array subscript assign must still be S060"
    );
}

#[test]
fn u23_let_append_still_s060() {
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
fn u23_let_class_member_runs() {
    let path = write_temp(
        r#"
class Box {
    var n: Int
    init(n: Int) { self.n = n }
}
public func main() {
    let b = Box(n: 1)
    b.n = 9
    print(b.n)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("let class member must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["9"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u23_var_struct_member_runs() {
    let path = write_temp(
        r#"
struct Point {
    var x: Int
    var y: Int
}
public func main() {
    var p = Point(x: 1, y: 2)
    p.x = 3
    print(p.x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("var struct member must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["3"]);
    let _ = std::fs::remove_file(&path);
}
