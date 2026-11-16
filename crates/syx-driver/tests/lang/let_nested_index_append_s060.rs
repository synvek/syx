//! U-25: nested let value-type IndexAssign / append → S060 (class hop stays legal).

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
fn u25_let_nested_array_index_is_s060() {
    assert!(
        has_error(
            r#"
struct Bag { var arr: Array<Int> }
public func main() {
    let s = Bag(arr: [1])
    s.arr[0] = 9
}
"#,
            "S060"
        ),
        "let nested Array subscript must be S060"
    );
}

#[test]
fn u25_let_nested_array_append_is_s060() {
    assert!(
        has_error(
            r#"
struct Bag { var arr: Array<Int> }
public func main() {
    let s = Bag(arr: [1])
    s.arr.append(2)
}
"#,
            "S060"
        ),
        "let nested Array append must be S060"
    );
}

#[test]
fn u25_let_nested_dict_index_is_s060() {
    assert!(
        has_error(
            r#"
struct Bag { var d: Dictionary<String, Int> }
public func main() {
    let s = Bag(d: {"a": 1})
    s.d["b"] = 2
}
"#,
            "S060"
        ),
        "let nested Dictionary subscript must be S060"
    );
}

#[test]
fn u25_let_class_array_ok() {
    assert!(
        !has_error(
            r#"
class Box {
    var arr: Array<Int>
    init(arr: Array<Int>) { self.arr = arr }
}
public func main() {
    let b = Box(arr: [1])
    b.arr[0] = 9
    b.arr.append(2)
}
"#,
            "S060"
        ),
        "let class-held Array mutate must not be S060"
    );
}

#[test]
fn u25_let_ident_index_still_s060() {
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
        "let Ident Array subscript must still be S060"
    );
}

#[test]
fn u25_let_nested_member_still_s060() {
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
        "let nested MemberAssign must still be S060"
    );
}

#[test]
fn u25_var_nested_array_ok() {
    assert!(
        !has_error(
            r#"
struct Bag { var arr: Array<Int> }
public func main() {
    var s = Bag(arr: [1])
    s.arr[0] = 9
    s.arr.append(2)
}
"#,
            "S060"
        ),
        "var nested Array mutate must not be S060"
    );
}

#[test]
fn u25_let_class_array_runs() {
    let path = write_temp(
        r#"
class Box {
    var arr: Array<Int>
    init(arr: Array<Int>) { self.arr = arr }
}
public func main() {
    let b = Box(arr: [1])
    b.arr.append(7)
    print(b.arr.count)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("let class Array append must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["2"]);
    let _ = std::fs::remove_file(&path);
}
