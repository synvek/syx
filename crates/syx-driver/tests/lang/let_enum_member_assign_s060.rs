//! U-28: `let` enum associated MemberAssign → S060 (nested dataclass→enum; var stays green).

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
fn u28_let_enum_payload_is_s060() {
    assert!(
        has_error(
            r#"
enum Box { case wrap(payload: Int) }
public func main() {
    let e = Box.wrap(payload: 1)
    e.payload = 9
}
"#,
            "S060"
        ),
        "let enum associated assign must be S060"
    );
}

#[test]
fn u28_let_nested_enum_is_s060() {
    assert!(
        has_error(
            r#"
enum E { case wrap(payload: Int) }
struct W { var e: E }
public func main() {
    let w = W(e: E.wrap(payload: 1))
    w.e.payload = 9
}
"#,
            "S060"
        ),
        "let nested struct→enum assign must be S060"
    );
}

#[test]
fn u28_var_enum_ok() {
    assert!(
        !has_error(
            r#"
enum Box { case wrap(payload: Int) }
public func main() {
    var e = Box.wrap(payload: 1)
    e.payload = 9
}
"#,
            "S060"
        ),
        "var enum associated assign must not be S060"
    );
}

#[test]
fn u28_let_tuple_still_s060() {
    assert!(
        has_error(
            r#"
public func main() {
    let t = (1, 2)
    t.0 = 3
}
"#,
            "S060"
        ),
        "let tuple .0= must still be S060"
    );
}

#[test]
fn u28_let_struct_still_s060() {
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
        "let Ident struct must still be S060"
    );
}

#[test]
fn u28_var_enum_runs() {
    let path = write_temp(
        r#"
enum Box { case wrap(payload: Int) }
public func main() {
    var e = Box.wrap(payload: 1)
    e.payload = 9
    print(e.payload)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("var enum associated must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["9"]
    );
    let _ = std::fs::remove_file(&path);
}
